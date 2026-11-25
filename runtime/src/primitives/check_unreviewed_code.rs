//! `check-unreviewed-code` — the code no review is going to read (spec 084,
//! AC4): code a `done` spec claims, or that no spec claims, changed by a commit
//! no committed review covers. Code an `in-progress` spec claims passes, since
//! that spec's gate reviews it before `done`.
//!
//! Everything is read from a commit and its history, never from the disk or
//! `HEAD`: a push carries commits, and the hook passes a pushed commit that need
//! not be `HEAD`. So the spec directories, each spec's status, each plan and
//! each review record come from that commit's tree, and uncommitted work no
//! spec claims stays with the open specs' reviews
//! (`compute_review_scope::split_changes`). Only `[review] unreviewed-push`,
//! which says what the caller does with the report, is read from the working
//! tree's configuration.
//!
//! A `done` spec's window starts where `compute-review-scope` would start it at
//! that commit (`compute_review_scope::base_at`), and its share of each commit
//! is `commit_attribution::SpecShare`'s, so a review of what the check reports
//! clears it (`QUAL-DELEG-001`). The window for code no spec claims is the
//! project record's ([`project_window`]), and `write-review`'s project scope
//! reads the same functions.
//!
//! A subject whose window cannot be placed is named in `not-examined` with its
//! reason, never counted as holding nothing (`QUAL-CLAIM-001`).

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use git2::{Commit, Oid, Repository, Sort, Tree};

use crate::primitives::check_stuck::{TreeListings, commits_between, tree_entry_id};
use crate::primitives::commit_attribution::{Attributor, CommitClaims, SpecShare, project_share};
use crate::primitives::compute_review_scope::{
    FoldTest, PlanEntries, ReviewEnd, WindowCommits, base_at, last_review_commit, record_at,
};
use crate::primitives::review_subject::ReviewSubject;
use crate::primitives::{
    FeatureForm, PrimitiveError, ProjectRepository, REVIEW_RECORD_FILE, Result, blob_text,
    frontmatter_status, is_project_artifact, read_text,
};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckUnreviewedCodeArgs, CheckUnreviewedCodeResult, ListsForm, UnreviewedCodeCounts,
    UnreviewedDoneSpec, UnreviewedNotExamined, UnreviewedPushMode, UnreviewedState,
    UnreviewedUnclaimed,
};

const PRIMITIVE: &str = "check-unreviewed-code";

/// The `[review]` key saying what unreviewed code does to a push.
pub(crate) const UNREVIEWED_PUSH: &str = "unreviewed-push";

/// The `not-examined` subject naming the code no spec claims, the project
/// record's `spec:` label.
const PROJECT: &str = "project";

/// Where the window for code no spec claims starts, and what the record it
/// starts from hands on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ProjectWindow {
    /// The last commit the window leaves out.
    pub(crate) start: Oid,
    /// The paths the record the window starts from leaves open or left unread,
    /// which the next review reads wherever its window starts (spec 079).
    pub(crate) carried: Vec<String>,
}

/// Why the window for code no spec claims cannot be placed at a commit. Each
/// is a subject that could not be examined, never one with nothing owed
/// (`QUAL-CLAIM-001`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NotPlaced {
    /// The commit's tree holds no project record.
    NoRecord,
    /// A shallow clone's history stops at a commit already holding the record,
    /// so where its window starts may lie before the cut.
    Shallow,
}

impl NotPlaced {
    /// Why the project subject was not examined, for `not-examined`.
    fn reason(&self, specs_root: &str) -> String {
        let record = ReviewSubject::Project.record_rel(specs_root);
        match self {
            Self::NoRecord => format!(
                "the commit holds no `{record}`, so no window for the code no spec claims can \
                 be placed: seed it with `ductus write-review --seed`, commit it, and bring that \
                 commit into this branch"
            ),
            Self::Shallow => format!(
                "the history is shallow and stops at or before the start of `{record}`'s \
                 window: run `git fetch --unshallow`"
            ),
        }
    }
}

/// Where the project record at the commit `at` starts its window.
///
/// As a spec's window starts where its last review ended
/// (`compute_review_scope::last_review_commit`): after the commit finishing the
/// newest review that read its scope, which holds that run's fixes, or at the
/// `reviewed-against` of one whose record is held. With no such review the
/// window starts after the commit that added the record to history. That is a
/// seed's start, since the install or update that wrote it is committed with
/// it, hooks and command files included, and those are code naming no spec.
///
/// # Errors
///
/// [`PrimitiveError::Git`] for any libgit2 failure. A window that cannot be
/// placed is the `Err` of the inner result, not an error.
pub(crate) fn project_window(
    project: &ProjectRepository,
    specs_root: &str,
    at: &Commit<'_>,
) -> Result<std::result::Result<ProjectWindow, NotPlaced>> {
    let repository = &project.repository;
    let record_rel = ReviewSubject::Project.record_rel(specs_root);
    let record = project.to_git(&record_rel);
    let last = match last_review_commit(project, at, &record_rel, None) {
        Ok(last) => last,
        Err(PrimitiveError::ShallowHistory { .. }) => return Ok(Err(NotPlaced::Shallow)),
        Err(error) => return Err(error),
    };
    Ok(match last.end {
        Some(ReviewEnd::Finished { commit, unread }) => Ok(ProjectWindow {
            start: Oid::from_str(&commit)?,
            carried: unread,
        }),
        Some(ReviewEnd::Held(held)) => Ok(ProjectWindow {
            start: held.against,
            carried: held.open_findings,
        }),
        None => match record_added_commit(repository, &record, at.id())? {
            None => Err(NotPlaced::NoRecord),
            Some(start)
                if repository.is_shallow()
                    && repository.find_commit(start)?.parent_count() == 0 =>
            {
                Err(NotPlaced::Shallow)
            }
            Some(start) => Ok(ProjectWindow {
                start,
                carried: Vec::new(),
            }),
        },
    })
}

/// The commit that added the record `at` holds to history, or `None` when `at`
/// holds none.
///
/// Judged as `check_stuck::find_done_commit` judges a `done` pass: a commit
/// holding the record carries the start of a parent holding it, a merge taking
/// its first such parent's, and otherwise starts one itself, so a record removed
/// and added again starts afresh.
fn record_added_commit(repository: &Repository, record: &str, at: Oid) -> Result<Option<Oid>> {
    let mut walk = repository.revwalk()?;
    walk.push(at)?;
    walk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

    let mut starts: HashMap<Oid, Oid> = HashMap::new();
    for oid in walk {
        let oid = oid?;
        let commit = repository.find_commit(oid)?;
        if commit.tree()?.get_path(Path::new(record)).is_err() {
            continue;
        }
        let carried = commit
            .parent_ids()
            .find_map(|parent| starts.get(&parent).copied());
        starts.insert(oid, carried.unwrap_or(oid));
    }
    Ok(starts.get(&at).copied())
}

/// The code no spec claims in a window, as the commit the window ends at holds it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct UnclaimedCode {
    /// Each code path a commit no spec claims changed, that the end commit's
    /// tree holds: what a project review reads.
    pub(crate) paths: BTreeSet<String>,
    /// Each such path the end commit's tree no longer holds: a deletion is a
    /// change, as `compute_review_scope::paths_owing_a_review` counts a spec's
    /// own, with nothing left to read.
    pub(crate) deleted: BTreeSet<String>,
    /// The commits no spec claims that changed code.
    pub(crate) code_commits: usize,
    /// The walk's commit counts.
    pub(crate) commits: WindowCommits,
}

/// The code no spec claims that the commits after `start`, up to `at`, changed.
///
/// A commit belongs to the project by [`project_share`], the rule a review's
/// scope leaves it out of an open spec by, and the specs that may claim one are
/// those `at`'s tree holds. Code is every path that is not one of the project's
/// artifacts (`is_project_artifact`), so the record itself is never among them.
/// A path `at`'s tree no longer holds has nothing to read, and is set apart
/// in `deleted`.
///
/// # Errors
///
/// [`PrimitiveError::Git`] when the walk, a tree or a diff cannot be read, so
/// an unread history never reads as one holding no code.
pub(crate) fn unclaimed_code(
    project: &ProjectRepository,
    repo: &Path,
    specs_root: &str,
    start: Oid,
    at: &Commit<'_>,
) -> Result<UnclaimedCode> {
    let repository = &project.repository;
    let tree = at.tree()?;
    let attributor = Attributor::at_tree(project, repo, specs_root, &tree)?;
    let mut walk = repository.revwalk()?;
    walk.push(at.id())?;
    walk.hide(start)?;
    let commits: Vec<(Oid, CommitClaims)> = walk
        .map(|oid| -> Result<(Oid, CommitClaims)> {
            let oid = oid?;
            Ok((oid, attributor.claims(&repository.find_commit(oid)?)?))
        })
        .collect::<Result<_>>()?;
    let mut code = UnclaimedCode {
        commits: WindowCommits::of(&commits),
        ..UnclaimedCode::default()
    };
    for (_, claims) in commits {
        let changed: Vec<String> = project_share(claims)
            .0
            .into_iter()
            .filter(|path| !is_project_artifact(path, specs_root))
            .collect();
        code.code_commits += usize::from(!changed.is_empty());
        for path in changed {
            if holds(project, &tree, &path) {
                code.paths.insert(path);
            } else {
                code.deleted.insert(path);
            }
        }
    }
    Ok(code)
}

/// Whether `tree` holds the project path `path`.
fn holds(project: &ProjectRepository, tree: &Tree<'_>, path: &str) -> bool {
    tree.get_path(Path::new(&project.to_git(path))).is_ok()
}

/// A project review's scope: what it reads, and the window it comes from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ProjectScope {
    /// The commit the window starts after.
    pub(crate) diff_base: String,
    /// `HEAD`, the commit the window ends at.
    pub(crate) head: String,
    /// The code no spec claims in the window, and each path a project record
    /// leaves open or left unread, as `HEAD`'s tree holds them.
    pub(crate) scope: BTreeSet<String>,
    /// Each code path a commit no spec claims changed in the window that
    /// `HEAD`'s tree no longer holds, which the scope leaves out.
    pub(crate) deleted: BTreeSet<String>,
    /// The commits no spec claims that changed code in the window.
    pub(crate) code_commits: usize,
    /// The window's commit counts.
    pub(crate) commits: WindowCommits,
}

/// The scope of a project review at `HEAD`: the window [`project_window`]
/// places, or the one starting after `since` when the run names a commit, used
/// verbatim as a spec's `--since` is. The committed tree alone is read, so
/// uncommitted work joins no project scope.
///
/// What the record the window starts from hands on, and what the record at
/// `HEAD` leaves open or left unread, is held in scope wherever the window
/// starts, as a spec's is (spec 079), while `HEAD`'s tree still holds the path.
///
/// # Errors
///
/// [`PrimitiveError::Git`] for a project no work tree contains, a `since`
/// naming no commit, or any libgit2 failure, and whatever loading the project's
/// configuration returns. A window that cannot be placed is the `Err` of the
/// inner result.
pub(crate) fn project_scope(
    repo: &Path,
    since: Option<&str>,
) -> Result<std::result::Result<ProjectScope, NotPlaced>> {
    let specs_root = paths::Paths::load(repo)?.specs_root;
    let project = ProjectRepository::discover(repo)?;
    let head = project.repository.head()?.peel_to_commit()?;
    project_scope_at(&project, repo, &specs_root, &head, since)
}

/// [`project_scope`] as the commit `at` holds it, for `check-unreviewed-code`,
/// which reports what a project review of that commit would read.
fn project_scope_at(
    project: &ProjectRepository,
    repo: &Path,
    specs_root: &str,
    head: &Commit<'_>,
    since: Option<&str>,
) -> Result<std::result::Result<ProjectScope, NotPlaced>> {
    let repository = &project.repository;
    let window = match since {
        Some(reference) => ProjectWindow {
            start: repository
                .revparse_single(reference)?
                .peel_to_commit()?
                .id(),
            carried: Vec::new(),
        },
        None => match project_window(project, specs_root, head)? {
            Ok(window) => window,
            Err(not_placed) => return Ok(Err(not_placed)),
        },
    };
    let code = unclaimed_code(project, repo, specs_root, window.start, head)?;
    let tree = head.tree()?;
    let record = project.to_git(&ReviewSubject::Project.record_rel(specs_root));
    let at_head = record_at(repository, &tree, &record)?;
    let handed_on = at_head
        .into_iter()
        .flat_map(|record| {
            record
                .open_findings
                .into_iter()
                .chain(record.unread.unwrap_or_default())
        })
        .chain(window.carried)
        .filter(|path| holds(project, &tree, path));
    let mut scope = code.paths;
    scope.extend(handed_on);
    Ok(Ok(ProjectScope {
        diff_base: window.start.to_string(),
        head: head.id().to_string(),
        scope,
        deleted: code.deleted,
        code_commits: code.code_commits,
        commits: code.commits,
    }))
}

/// Read `[review] unreviewed-push` from the project's configuration: `block`
/// when the key, the `review` table or the file is absent (`CFG-ENV-001`).
///
/// # Errors
///
/// [`PrimitiveError::InvalidUnreviewedPush`] for any value but `"block"` or
/// `"warn"`, and for a `review` that is not a table, each naming the key and the
/// file, never falling back to a mode (`CFG-ENV-008`); [`PrimitiveError::Toml`]
/// when the file does not parse; [`PrimitiveError::Io`] when it cannot be read;
/// [`PrimitiveError::InvalidPath`] when it is a link leaving the project.
pub(crate) fn unreviewed_push(repo: &Path) -> Result<UnreviewedPushMode> {
    let (path, _) = paths::resolve_config(repo);
    super::contained_file(repo, &path)?;
    if !path.is_file() {
        return Ok(UnreviewedPushMode::Block);
    }
    let content = read_text(&path)?;
    let config: toml::Table = toml::from_str(&content).map_err(|source| PrimitiveError::Toml {
        path: path.clone(),
        source,
    })?;
    let invalid = |problem: String| PrimitiveError::InvalidUnreviewedPush {
        path: path.clone(),
        problem,
    };
    let review = match config.get("review") {
        None => return Ok(UnreviewedPushMode::Block),
        Some(toml::Value::Table(review)) => review,
        Some(other) => {
            return Err(invalid(format!(
                "`review` is {}, not a table",
                other.type_str()
            )));
        }
    };
    match review.get(UNREVIEWED_PUSH) {
        None => Ok(UnreviewedPushMode::Block),
        Some(toml::Value::String(mode)) if mode == "block" => Ok(UnreviewedPushMode::Block),
        Some(toml::Value::String(mode)) if mode == "warn" => Ok(UnreviewedPushMode::Warn),
        Some(other) => Err(invalid(format!("the value is {}", one_line(other)))),
    }
}

/// A TOML value on one line, cut to a length a message can carry.
fn one_line(value: &toml::Value) -> String {
    let text: String = value
        .to_string()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if text.chars().count() > 80 {
        format!("{}…", text.chars().take(80).collect::<String>())
    } else {
        text
    }
}

/// What the check found at one commit, every list whole.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Checked {
    /// Each `done` spec holding unreviewed code, in corpus order.
    pub done_specs: Vec<UnreviewedDoneSpec>,
    /// The code no spec claims, when there is any.
    pub unclaimed: Option<UnreviewedUnclaimed>,
    /// Each subject that could not be examined.
    pub not_examined: Vec<UnreviewedNotExamined>,
    /// The subjects examined.
    pub examined: usize,
}

impl Checked {
    /// The state the lists amount to: a subject not examined decides, whatever
    /// else was found.
    #[must_use]
    pub fn state(&self) -> UnreviewedState {
        if !self.not_examined.is_empty() {
            UnreviewedState::NotExamined
        } else if self.done_specs.is_empty() && self.unclaimed.is_none() {
            UnreviewedState::None
        } else {
            UnreviewedState::Found
        }
    }

    /// Every path reported, `deleted` ones among them.
    fn reported_paths(&self) -> usize {
        self.done_specs
            .iter()
            .map(|spec| spec.paths.len() + spec.deleted.len())
            .sum::<usize>()
            + self.unclaimed_paths()
    }

    fn unclaimed_paths(&self) -> usize {
        self.unclaimed
            .as_ref()
            .map_or(0, |code| code.paths.len() + code.deleted.len())
    }
}

/// The first-parent commits a `since` walk ran the check at.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SinceCount {
    /// The commits walked, from `at` back to `since`, which is left out.
    pub walked: usize,
    /// Those at which the check's state is `found`.
    pub reporting: usize,
    /// Those at which it is `not-examined`.
    pub not_examined: usize,
}

/// `check-unreviewed-code`'s whole answer, before its result is bounded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unreviewed {
    /// The commit read.
    pub at: String,
    /// `[review] unreviewed-push`.
    pub mode: UnreviewedPushMode,
    /// What the check found at `at`.
    pub checked: Checked,
    /// The `since` count, when the call asked for one.
    pub since: Option<SinceCount>,
}

/// Execute the `check-unreviewed-code` primitive, its result returned inline
/// while it fits `[artifacts] read-size-bytes` and past it with its lists in
/// the file `lists-file` names (`BE-PAYLOAD-001`).
///
/// # Errors
///
/// As [`compute`] and [`bound`].
pub fn run(args: &CheckUnreviewedCodeArgs, repo: &Path) -> Result<CheckUnreviewedCodeResult> {
    bound(repo, compute(args, repo)?)
}

/// The check at `HEAD` as `dashboard` carries it for `/{project}:status` (spec
/// 084): its counts, and on `not-examined` each subject not examined with why.
/// A check that failed is `not-examined` with its error, so it never reads as
/// no code owed.
pub(crate) fn counts(repo: &Path) -> UnreviewedCodeCounts {
    let args = CheckUnreviewedCodeArgs {
        at: None,
        since: None,
    };
    match compute(&args, repo) {
        Ok(found) => {
            let checked = &found.checked;
            let reason = (!checked.not_examined.is_empty()).then(|| {
                checked
                    .not_examined
                    .iter()
                    .map(|entry| format!("{}: {}", entry.subject, entry.reason))
                    .collect::<Vec<_>>()
                    .join("; ")
            });
            UnreviewedCodeCounts {
                state: checked.state(),
                done_spec_count: checked.done_specs.len(),
                path_count: checked.reported_paths(),
                unclaimed_path_count: checked.unclaimed_paths(),
                reason: reason.map(|text| super::flatten_line(&text)),
            }
        }
        Err(error) => UnreviewedCodeCounts {
            state: UnreviewedState::NotExamined,
            reason: Some(super::flatten_line(&error.to_string())),
            ..UnreviewedCodeCounts::default()
        },
    }
}

/// Run the check at `at`, and with `since` at each first-parent commit from
/// `at` back to it, every list whole. The CLI reports and gates on this, so a
/// list the bound moves to a file is still named and still blocks.
///
/// # Errors
///
/// [`unreviewed_push`]'s, before anything else is read; [`PrimitiveError::Git`]
/// for a project no work tree contains, an `at` or a `since` naming no commit, or
/// a tree, a walk or a diff that cannot be read; and whatever loading the
/// project's configuration returns. A subject whose window cannot be placed is
/// no error: it is named in `not-examined`.
pub fn compute(args: &CheckUnreviewedCodeArgs, repo: &Path) -> Result<Unreviewed> {
    let mode = unreviewed_push(repo)?;
    let specs_root = paths::Paths::load(repo)?.specs_root;
    let project = ProjectRepository::discover(repo)?;
    let repository = &project.repository;
    // Every `done` spec's window is placed by a walk of all of history.
    let _listings = TreeListings::open();
    let at = repository
        .revparse_single(args.at.as_deref().unwrap_or("HEAD"))?
        .peel_to_commit()?;
    let mut memo = ClaimsMemo::default();
    let checked = check_at(&project, repo, &specs_root, &at, &mut memo)?;
    let since = match &args.since {
        None => None,
        Some(reference) => {
            let since = repository.revparse_single(reference)?.peel_to_commit()?;
            let mut walk = repository.revwalk()?;
            walk.push(at.id())?;
            walk.hide(since.id())?;
            walk.simplify_first_parent()?;
            let mut count = SinceCount::default();
            for oid in walk {
                let commit = repository.find_commit(oid?)?;
                let state = if commit.id() == at.id() {
                    checked.state()
                } else {
                    check_at(&project, repo, &specs_root, &commit, &mut memo)?.state()
                };
                count.walked += 1;
                count.reporting += usize::from(state == UnreviewedState::Found);
                count.not_examined += usize::from(state == UnreviewedState::NotExamined);
            }
            Some(count)
        }
    };
    Ok(Unreviewed {
        at: at.id().to_string(),
        mode,
        checked,
        since,
    })
}

/// Hold `found` to the read size: past it `done-specs`, `unclaimed` and
/// `not-examined` go to the file the result names, with their counts inline.
///
/// # Errors
///
/// Those `bounded_lists::place` returns.
pub fn bound(repo: &Path, found: Unreviewed) -> Result<CheckUnreviewedCodeResult> {
    let checked = found.checked;
    let lists = serde_json::json!({
        "done-specs": checked.done_specs,
        "unclaimed": checked.unclaimed,
        "not-examined": checked.not_examined,
    });
    let mut result = CheckUnreviewedCodeResult {
        at: found.at,
        mode: found.mode,
        state: checked.state(),
        done_specs_count: checked.done_specs.len(),
        unclaimed_path_count: checked.unclaimed_paths(),
        not_examined_count: checked.not_examined.len(),
        examined_count: checked.examined,
        reported_path_count: checked.reported_paths(),
        done_specs: Some(checked.done_specs),
        unclaimed: checked.unclaimed,
        not_examined: Some(checked.not_examined),
        commits_walked: found.since.map(|count| count.walked),
        commits_reporting: found.since.map(|count| count.reporting),
        commits_not_examined: found.since.map(|count| count.not_examined),
        form: ListsForm::Inline,
        lists_file: None,
    };
    if let Some(file) = super::bounded_lists::place(repo, PRIMITIVE, PROJECT, &result, &lists)? {
        result.done_specs = None;
        result.unclaimed = None;
        result.not_examined = None;
        result.form = ListsForm::File;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// Each commit's claims, attributed once per walk of `since`: a `done` spec's
/// window and the project's overlap, and so do the windows of neighbouring
/// commits. Claims depend on which specs the tree holds, so the memo is kept
/// only while that list is the same.
#[derive(Default)]
struct ClaimsMemo {
    specs: Vec<String>,
    claims: HashMap<Oid, CommitClaims>,
    /// [`Attributor::may_claim`] by commit, which reads no spec list.
    may_claim: HashMap<Oid, std::rc::Rc<Vec<FeatureForm>>>,
}

impl ClaimsMemo {
    /// Forget claims made under another list of specs.
    fn for_specs(&mut self, attributor: &Attributor<'_>) {
        let specs: Vec<String> = attributor
            .specs()
            .iter()
            .map(|(_, directory)| directory.clone())
            .collect();
        if specs != self.specs {
            self.specs = specs;
            self.claims.clear();
        }
    }

    fn may_claim(
        &mut self,
        attributor: &Attributor<'_>,
        commit: &Commit<'_>,
    ) -> Result<std::rc::Rc<Vec<FeatureForm>>> {
        if let Some(forms) = self.may_claim.get(&commit.id()) {
            return Ok(std::rc::Rc::clone(forms));
        }
        let forms = std::rc::Rc::new(attributor.may_claim(commit)?);
        self.may_claim
            .insert(commit.id(), std::rc::Rc::clone(&forms));
        Ok(forms)
    }

    fn of(&mut self, attributor: &Attributor<'_>, commit: &Commit<'_>) -> Result<CommitClaims> {
        if let Some(claims) = self.claims.get(&commit.id()) {
            return Ok(claims.clone());
        }
        let claims = attributor.claims(commit)?;
        self.claims.insert(commit.id(), claims.clone());
        Ok(claims)
    }
}

/// One spec the commit's tree holds.
struct TreeSpec {
    form: FeatureForm,
    directory: String,
    /// `None` when the tree holds no `spec.md` for it, or one stating no status.
    status: Option<String>,
    plan: PlanEntries,
}

/// Run the check at the commit `at`.
fn check_at(
    project: &ProjectRepository,
    repo: &Path,
    specs_root: &str,
    at: &Commit<'_>,
    memo: &mut ClaimsMemo,
) -> Result<Checked> {
    let repository = &project.repository;
    let tree = at.tree()?;
    let attributor = Attributor::at_tree(project, repo, specs_root, &tree)?;
    memo.for_specs(&attributor);
    let mut specs = Vec::new();
    for (form, directory) in attributor.specs() {
        let spec = project.to_git(&format!("{specs_root}/{directory}/spec.md"));
        let status = blob_text(repository, &tree, &spec)?
            .and_then(|text| frontmatter_status(&text, Path::new(&spec)));
        specs.push(TreeSpec {
            form: form.clone(),
            directory: directory.clone(),
            status,
            plan: attributor.plan_of(directory)?,
        });
    }
    let mut shares: Vec<SpecShare<'_>> = specs
        .iter()
        .map(|spec| SpecShare::new(&attributor, &spec.form, &spec.directory, &spec.plan))
        .collect();
    let mut checked = Checked::default();
    let subject = DoneSubject {
        project,
        specs_root,
        attributor: &attributor,
        at,
        tree: &tree,
        specs: &specs,
    };
    for (index, spec) in specs.iter().enumerate() {
        match spec.status.as_deref() {
            Some("done") => {}
            Some(_) => continue,
            None => {
                checked.not_examined.push(UnreviewedNotExamined {
                    subject: spec.directory.clone(),
                    reason: "the commit holds no `spec.md` for it, or one stating no status, so \
                             whether it is `done` is unknown"
                        .into(),
                });
                continue;
            }
        }
        match subject.code(index, &mut shares, memo)? {
            Ok(found) => {
                checked.examined += 1;
                checked.done_specs.extend(found);
            }
            Err(reason) => checked.not_examined.push(UnreviewedNotExamined {
                subject: spec.directory.clone(),
                reason,
            }),
        }
    }
    let record = project.to_git(&ReviewSubject::Project.record_rel(specs_root));
    let unparsed =
        tree_entry_id(&tree, &record).is_some() && record_at(repository, &tree, &record)?.is_none();
    let placed = if unparsed {
        Err(format!(
            "`{}` as the commit holds it does not parse, so where its window starts is unknown",
            ReviewSubject::Project.record_rel(specs_root)
        ))
    } else {
        project_scope_at(project, repo, specs_root, at, None)?
            .map_err(|not_placed| not_placed.reason(specs_root))
    };
    match placed {
        Ok(scope) => {
            checked.examined += 1;
            if !scope.scope.is_empty() || !scope.deleted.is_empty() {
                checked.unclaimed = Some(UnreviewedUnclaimed {
                    window_start: scope.diff_base,
                    paths: scope.scope.into_iter().collect(),
                    deleted: scope.deleted.into_iter().collect(),
                    commits: scope.code_commits,
                });
            }
        }
        Err(reason) => checked.not_examined.push(UnreviewedNotExamined {
            subject: PROJECT.into(),
            reason,
        }),
    }
    Ok(checked)
}

/// What a `done` spec's code is read from: the commit, its tree and its specs.
struct DoneSubject<'a> {
    project: &'a ProjectRepository,
    specs_root: &'a str,
    attributor: &'a Attributor<'a>,
    at: &'a Commit<'a>,
    tree: &'a Tree<'a>,
    specs: &'a [TreeSpec],
}

/// Why a `done` spec whose history is cut was not examined.
const SHALLOW: &str = "the history is shallow and stops at or before its window's start: run `git \
                       fetch --unshallow`";

impl DoneSubject<'_> {
    /// The commit the spec's window starts after, as `compute-review-scope`
    /// would place it at this commit, or why it cannot be placed.
    fn window_start(&self, spec: &TreeSpec) -> Result<std::result::Result<String, String>> {
        let repository = &self.project.repository;
        let record = self.project.to_git(&format!(
            "{}/{}/{REVIEW_RECORD_FILE}",
            self.specs_root, spec.directory
        ));
        if tree_entry_id(self.tree, &record).is_some()
            && record_at(repository, self.tree, &record)?.is_none()
        {
            return Ok(Err(format!(
                "its `{REVIEW_RECORD_FILE}` as the commit holds it does not parse, so where its \
                 window starts is unknown"
            )));
        }
        let start = match base_at(
            self.project,
            self.specs_root,
            &spec.directory,
            self.at,
            None,
        ) {
            Ok((start, _, _)) => start,
            Err(PrimitiveError::ShallowHistory { .. }) => return Ok(Err(SHALLOW.into())),
            Err(error) => return Err(error),
        };
        if start.is_empty() {
            return Ok(Err(
                "no commit sets it `done` or records a review of it, so its window has no start"
                    .into(),
            ));
        }
        Ok(Ok(start))
    }

    /// The unreviewed code of the `done` spec at `index`: `None` when it holds
    /// none, and the reason as the inner `Err` when its window cannot be placed.
    ///
    /// The window is `compute-review-scope`'s at this commit, and each commit's
    /// share [`SpecShare`]'s, less any path an `in-progress` spec's share of the
    /// same commit holds, whose gate reviews it. What a staging spec folded into
    /// the spec below `done` brought joins wherever the window starts, as a
    /// review's scope takes it.
    fn code(
        &self,
        index: usize,
        shares: &mut [SpecShare<'_>],
        memo: &mut ClaimsMemo,
    ) -> Result<std::result::Result<Option<UnreviewedDoneSpec>, String>> {
        let repository = &self.project.repository;
        let spec = &self.specs[index];
        let start = match self.window_start(spec)? {
            Ok(start) => start,
            Err(reason) => return Ok(Err(reason)),
        };
        let fold = FoldTest {
            project: self.project,
            specs_root: self.specs_root,
            attributor: self.attributor,
            target: &spec.form,
        };
        let window = commits_between(repository, self.at.id(), Oid::from_str(&start)?)?;
        let mut changed: BTreeSet<String> = BTreeSet::new();
        let mut folded = Vec::new();
        let mut commits = 0;
        for oid in window {
            let commit = repository.find_commit(oid)?;
            // The diff is read only for a commit that may be this spec's, or
            // may have folded a staging spec: most of a long window is neither.
            let may_fold = fold.may_have_folded(&commit)?;
            if !may_fold
                && !memo
                    .may_claim(self.attributor, &commit)?
                    .contains(&spec.form)
            {
                continue;
            }
            let claims = memo.of(self.attributor, &commit)?;
            if may_fold {
                folded.extend(fold.committed(&commit, &claims.paths)?);
            }
            if !claims.is_claimed_by(&spec.form) {
                continue;
            }
            let mut in_hand: BTreeSet<String> = BTreeSet::new();
            for (other, share) in shares.iter_mut().enumerate() {
                let candidate = &self.specs[other];
                if other != index
                    && candidate.status.as_deref() == Some("in-progress")
                    && claims.is_claimed_by(&candidate.form)
                {
                    in_hand.extend(share.split(claims.clone())?.0);
                }
            }
            let own: Vec<String> = shares[index]
                .split(claims)?
                .0
                .into_iter()
                .filter(|path| !is_project_artifact(path, self.specs_root))
                .filter(|path| !in_hand.contains(path))
                .collect();
            commits += usize::from(!own.is_empty());
            changed.extend(own);
        }
        let in_tree = |path: &str| holds(self.project, self.tree, path);
        let brought = match fold.brought(self.at.id(), &in_tree, &folded) {
            Ok(brought) => brought,
            Err(PrimitiveError::ShallowHistory { .. }) => return Ok(Err(SHALLOW.into())),
            Err(error) => return Err(error),
        };
        changed.extend(
            brought
                .into_iter()
                .filter(|path| !is_project_artifact(path, self.specs_root)),
        );
        if changed.is_empty() {
            return Ok(Ok(None));
        }
        let (paths, deleted) = changed.into_iter().partition(|path| in_tree(path));
        Ok(Ok(Some(UnreviewedDoneSpec {
            feature: spec.directory.clone(),
            window_start: start,
            paths,
            deleted,
            commits,
        })))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{commit_all, subdirectory_project, write};
    use crate::primitives::write_review;
    use crate::schema::primitives::{ReviewFinding, SeedOutcome, WriteReviewArgs};
    use crate::schema::severity::ReviewSeverity;

    /// A project with spec 001 on disk and in history.
    fn project_with_a_spec(root: &Path) -> Repository {
        let repository = Repository::init(root).unwrap();
        write(
            &root.join("specs/001-a/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# A\n",
        );
        commit_all(&repository, "spec(001): add");
        repository
    }

    /// Write `file` and commit it under `subject`.
    fn change(root: &Path, repository: &Repository, file: &str, subject: &str) -> Oid {
        write(&root.join(file), &format!("{subject}\n"));
        commit_all(repository, subject)
    }

    /// Seed the project record and commit it with `hooks/pre-push`, as an
    /// install commits its seed with the files it brought.
    fn install(root: &Path, repository: &Repository) -> Oid {
        let seeded = write_review::run(
            &WriteReviewArgs {
                seed: true,
                ..WriteReviewArgs::default()
            },
            root,
        )
        .unwrap();
        assert_eq!(seeded.seed, Some(SeedOutcome::Written));
        change(root, repository, "hooks/pre-push", "chore: install")
    }

    fn window_at_head(
        root: &Path,
        repository: &Repository,
    ) -> std::result::Result<ProjectWindow, NotPlaced> {
        let project = ProjectRepository::discover(root).unwrap();
        let head = repository.head().unwrap().peel_to_commit().unwrap();
        project_window(&project, "specs", &head).unwrap()
    }

    fn scope_paths(root: &Path) -> Vec<String> {
        project_scope(root, None)
            .unwrap()
            .unwrap()
            .scope
            .into_iter()
            .collect()
    }

    /// Record a project review of the window after `base`, with `findings`.
    fn review(root: &Path, repository: &Repository, base: Oid, findings: Vec<ReviewFinding>) {
        let head = repository.head().unwrap().peel_to_commit().unwrap().id();
        write_review::run(
            &WriteReviewArgs {
                reviewed_at: "2026-11-24T00:00:00Z".into(),
                reviewed_against: head.to_string(),
                diff_base: base.to_string(),
                unread: Some(Vec::new()),
                findings,
                ..WriteReviewArgs::default()
            },
            root,
        )
        .unwrap();
    }

    #[test]
    fn a_commit_with_no_project_record_places_no_window() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        change(tmp.path(), &repository, "src/a.rs", "chore: tidy");
        assert_eq!(
            window_at_head(tmp.path(), &repository),
            Err(NotPlaced::NoRecord)
        );
        assert_eq!(
            project_scope(tmp.path(), None).unwrap(),
            Err(NotPlaced::NoRecord)
        );
        // An uncommitted seed is not in the commit either.
        write_review::run(
            &WriteReviewArgs {
                seed: true,
                ..WriteReviewArgs::default()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            window_at_head(tmp.path(), &repository),
            Err(NotPlaced::NoRecord)
        );
    }

    /// A seed's window starts after the commit that added it, so the files that
    /// commit brought are outside it, and the code committed later is inside.
    #[test]
    fn a_seeds_window_starts_after_the_commit_that_added_it() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        change(
            tmp.path(),
            &repository,
            "src/before.rs",
            "chore: before the seed",
        );
        let added = install(tmp.path(), &repository);
        change(tmp.path(), &repository, "src/a.rs", "chore: tidy");
        let window = window_at_head(tmp.path(), &repository).unwrap();
        assert_eq!(
            window.start, added,
            "not the seed's reviewed-against, its parent"
        );
        assert_eq!(scope_paths(tmp.path()), ["src/a.rs"]);
    }

    /// Only committed code no spec claims is the project's: a claimed commit's
    /// code, an artifact, a deleted file and uncommitted work are not.
    #[test]
    fn the_scope_holds_only_committed_code_no_spec_claims() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        change(tmp.path(), &repository, "src/unclaimed.rs", "chore: tidy");
        change(tmp.path(), &repository, "src/claimed.rs", "fix(001): a bug");
        change(
            tmp.path(),
            &repository,
            "specs/001-a/plan.md",
            "plan the spec",
        );
        change(tmp.path(), &repository, "AGENTS.md", "docs: agents");
        change(
            tmp.path(),
            &repository,
            "specs/inbox.md",
            "chore: log an item",
        );
        change(tmp.path(), &repository, "src/gone.rs", "chore: add a file");
        std::fs::remove_file(tmp.path().join("src/gone.rs")).unwrap();
        commit_all(&repository, "chore: remove it");
        write(&tmp.path().join("src/uncommitted.rs"), "fn u() {}\n");
        let scope = project_scope(tmp.path(), None).unwrap().unwrap();
        assert_eq!(
            scope.scope.iter().map(String::as_str).collect::<Vec<_>>(),
            ["src/unclaimed.rs"]
        );
        assert_eq!(scope.commits.examined, 7);
        assert_eq!(
            scope.commits.unclaimed, 5,
            "tidy, agents, inbox, add and remove"
        );
    }

    /// The specs that may claim a commit are those the commit's tree holds: a
    /// spec directory on disk and not committed claims nothing.
    #[test]
    fn an_uncommitted_spec_directory_claims_no_commit() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        change(
            tmp.path(),
            &repository,
            "src/a.rs",
            "fix(002): named before its spec",
        );
        write(
            &tmp.path().join("specs/002-b/spec.md"),
            "---\nstatus: draft\n---\n",
        );
        assert_eq!(scope_paths(tmp.path()), ["src/a.rs"]);
    }

    /// A finished review's next window starts after the commit that committed
    /// its record, which holds its fixes, so they are not owed a second review.
    #[test]
    fn a_finished_reviews_window_starts_after_the_commit_finishing_it() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        let added = install(tmp.path(), &repository);
        change(tmp.path(), &repository, "src/a.rs", "chore: tidy");
        review(tmp.path(), &repository, added, Vec::new());
        // The review's fix rides in the commit that commits its record.
        let finishing = change(
            tmp.path(),
            &repository,
            "src/a.rs",
            "chore: review, with a fix",
        );
        let window = window_at_head(tmp.path(), &repository).unwrap();
        assert_eq!(window.start, finishing);
        assert_eq!(scope_paths(tmp.path()), [] as [&str; 0]);
        change(tmp.path(), &repository, "src/b.rs", "chore: later");
        assert_eq!(scope_paths(tmp.path()), ["src/b.rs"]);
    }

    /// A review whose record is held starts the next window at the
    /// `reviewed-against` it read up to, and hands on what it leaves open.
    #[test]
    fn a_held_reviews_window_starts_at_its_reviewed_against() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        let added = install(tmp.path(), &repository);
        let read_up_to = change(tmp.path(), &repository, "src/a.rs", "chore: tidy");
        let must = ReviewFinding {
            site: Some("walk".into()),
            rule: "BE-INPUT-001".into(),
            severity: ReviewSeverity::Must,
            file: "src/a.rs".into(),
            line_range: "1-1".into(),
            confidence: "high".into(),
            summary: "a summary".into(),
            finding: "a finding".into(),
            rule_text: "the rule".into(),
            auto_fixable: false,
            suggested_fix: String::new(),
            disposition: crate::schema::primitives::Disposition::default(),
            decision_key: None,
        };
        review(tmp.path(), &repository, added, vec![must]);
        change(
            tmp.path(),
            &repository,
            "src/fix.rs",
            "chore: review, with a fix",
        );
        let window = window_at_head(tmp.path(), &repository).unwrap();
        assert_eq!(window.start, read_up_to);
        assert_eq!(window.carried, ["src/a.rs"]);
        // The fix is after `reviewed-against`, and the open finding's file is
        // held although no commit in the window changed it.
        assert_eq!(scope_paths(tmp.path()), ["src/a.rs", "src/fix.rs"]);
    }

    /// `--since` is used verbatim, with no record needed to place it.
    #[test]
    fn a_named_start_is_used_verbatim() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        let first = change(tmp.path(), &repository, "src/a.rs", "chore: first");
        change(tmp.path(), &repository, "src/b.rs", "chore: second");
        let scope = project_scope(tmp.path(), Some(&first.to_string()))
            .unwrap()
            .unwrap();
        assert_eq!(scope.diff_base, first.to_string());
        assert_eq!(scope.scope.into_iter().collect::<Vec<_>>(), ["src/b.rs"]);
        assert!(project_scope(tmp.path(), Some("no-such-ref")).is_err());
    }

    /// The record is found by its path from the git work tree, so a project in
    /// a subdirectory places its window as one at the root does.
    #[test]
    fn a_project_in_a_subdirectory_places_its_window() {
        let tmp = tempfile::tempdir().unwrap();
        let (repository, project) = subdirectory_project(tmp.path());
        write(
            &project.join("specs/001-a/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# A\n",
        );
        commit_all(&repository, "spec(001): add");
        let added = install(&project, &repository);
        change(&project, &repository, "src/a.rs", "chore: tidy");
        change(
            tmp.path(),
            &repository,
            "elsewhere/b.rs",
            "chore: another project",
        );
        assert_eq!(window_at_head(&project, &repository).unwrap().start, added);
        assert_eq!(scope_paths(&project), ["src/a.rs"]);
    }

    /// A shallow clone whose history stops at the commit holding the seed
    /// cannot say that commit added it.
    #[test]
    fn a_shallow_clone_cut_at_the_seed_places_no_window() {
        let tmp = tempfile::tempdir().unwrap();
        let origin = tmp.path().join("origin");
        let repository = project_with_a_spec(&origin);
        install(&origin, &repository);
        change(&origin, &repository, "src/a.rs", "chore: tidy");
        let clone = tmp.path().join("clone");
        let status = std::process::Command::new("git")
            .args(["clone", "--quiet", "--depth", "1"])
            .arg(format!("file://{}", origin.display()))
            .arg(&clone)
            .status()
            .unwrap();
        assert!(status.success());
        let cloned = Repository::open(&clone).unwrap();
        assert!(cloned.is_shallow());
        assert_eq!(window_at_head(&clone, &cloned), Err(NotPlaced::Shallow));
    }

    // -- `check-unreviewed-code` (spec 084, AC4 and AC17) --------------------

    fn spec(status: &str) -> String {
        format!("---\nstatus: {status}\ndependencies: []\n---\n\n# S\n")
    }

    /// A clean review record of a run at `stamp` that read its scope.
    fn clean_record(feature: &str, stamp: &str) -> String {
        format!(
            "---\nspec: {feature}\nlast-run: {stamp}\nmust-violations: 0\nshould-violations: 0\n\
             low-confidence: 0\nunread: []\nblocking: false\ndispositions:\n  fixed: 0\n  \
             deferred: 0\n  waived: 0\n  undispositioned: 0\n---\n\n# Review\n"
        )
    }

    /// Write `body` to `file` and commit it under `subject`.
    fn put(root: &Path, repository: &Repository, file: &str, body: &str, subject: &str) -> Oid {
        write(&root.join(file), body);
        commit_all(repository, subject)
    }

    /// Spec 002-b beside 001-a: started, its code committed, reviewed, and set
    /// `done`. Returns the commit finishing its review.
    fn a_done_spec(root: &Path, repository: &Repository) -> Oid {
        put(
            root,
            repository,
            "specs/002-b/spec.md",
            &spec("in-progress"),
            "status(002): start",
        );
        change(root, repository, "src/b.rs", "feat(002): its code");
        let reviewed = put(
            root,
            repository,
            "specs/002-b/review.md",
            &clean_record("002-b", "2026-11-01T00:00:00Z"),
            "review(002): clean",
        );
        put(
            root,
            repository,
            "specs/002-b/spec.md",
            &spec("done"),
            "status(002): done",
        );
        reviewed
    }

    fn check(root: &Path) -> Checked {
        compute(&CheckUnreviewedCodeArgs::default(), root)
            .unwrap()
            .checked
    }

    fn check_at_rev(root: &Path, at: Oid) -> Checked {
        let args = CheckUnreviewedCodeArgs {
            at: Some(at.to_string()),
            since: None,
        };
        compute(&args, root).unwrap().checked
    }

    fn strings(list: &[&str]) -> Vec<String> {
        list.iter().map(ToString::to_string).collect()
    }

    /// AC4's three directions: a `done` spec's code and the code no spec claims
    /// are reported, each with its window, and an `in-progress` spec's is not.
    #[test]
    fn a_done_specs_code_and_unclaimed_code_are_reported_and_an_open_specs_is_not() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        let added = install(tmp.path(), &repository);
        let reviewed = a_done_spec(tmp.path(), &repository);
        assert_eq!(check(tmp.path()).state(), UnreviewedState::None);
        assert_eq!(check(tmp.path()).examined, 2, "002-b and the project");

        change(tmp.path(), &repository, "src/b2.rs", "fix(002): while done");
        change(tmp.path(), &repository, "src/a.rs", "fix(001): in hand");
        change(tmp.path(), &repository, "src/u.rs", "chore: nobody's");
        let found = check(tmp.path());
        assert_eq!(found.state(), UnreviewedState::Found);
        assert_eq!(
            found.done_specs,
            [UnreviewedDoneSpec {
                feature: "002-b".into(),
                window_start: reviewed.to_string(),
                paths: strings(&["src/b2.rs"]),
                deleted: Vec::new(),
                commits: 1,
            }]
        );
        assert_eq!(
            found.unclaimed,
            Some(UnreviewedUnclaimed {
                window_start: added.to_string(),
                paths: strings(&["src/u.rs"]),
                deleted: Vec::new(),
                commits: 1,
            })
        );
        assert_eq!(found.not_examined, []);
        assert_eq!(found.reported_paths(), 2);
    }

    /// What the check reports, a review clears: a record committed for the
    /// `done` spec, and one for the project, leave nothing reported.
    #[test]
    fn a_committed_review_of_what_is_reported_clears_it() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        let added = install(tmp.path(), &repository);
        a_done_spec(tmp.path(), &repository);
        change(tmp.path(), &repository, "src/b2.rs", "fix(002): while done");
        change(tmp.path(), &repository, "src/u.rs", "chore: nobody's");
        std::fs::remove_file(tmp.path().join("src/u.rs")).unwrap();
        commit_all(&repository, "chore: remove it");
        let found = check(tmp.path());
        // A window holding only a deletion is still reported, with nothing to read.
        let unclaimed = found.unclaimed.unwrap();
        assert_eq!(unclaimed.paths, [] as [&str; 0]);
        assert_eq!(unclaimed.deleted, ["src/u.rs"]);
        assert_eq!(unclaimed.commits, 2);

        put(
            tmp.path(),
            &repository,
            "specs/002-b/review.md",
            &clean_record("002-b", "2026-11-02T00:00:00Z"),
            "review(002): the code changed while done",
        );
        let after = check(tmp.path());
        assert!(after.done_specs.is_empty(), "{:?}", after.done_specs);
        assert!(after.unclaimed.is_some());

        review(tmp.path(), &repository, added, Vec::new());
        commit_all(&repository, "review: the project");
        assert_eq!(check(tmp.path()).state(), UnreviewedState::None);
    }

    /// AC7: a `done` spec no review has recorded starts its window at the commit
    /// that set it `done`, so its code changed after that commit is reported and
    /// none before.
    #[test]
    fn an_unreviewed_done_spec_reports_its_code_after_the_commit_that_set_it_done() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        put(
            tmp.path(),
            &repository,
            "specs/002-b/spec.md",
            &spec("in-progress"),
            "status(002): start",
        );
        change(
            tmp.path(),
            &repository,
            "src/before.rs",
            "feat(002): before",
        );
        let done = put(
            tmp.path(),
            &repository,
            "specs/002-b/spec.md",
            &spec("done"),
            "status(002): done",
        );
        assert_eq!(check(tmp.path()).state(), UnreviewedState::None);
        change(tmp.path(), &repository, "src/after.rs", "fix(002): after");
        let found = check(tmp.path());
        assert_eq!(found.done_specs.len(), 1);
        assert_eq!(found.done_specs[0].window_start, done.to_string());
        assert_eq!(found.done_specs[0].paths, ["src/after.rs"]);
    }

    /// A `done` spec whose only run stopped early keeps its reopening's start, so
    /// what that run left unread stays reported, and the start is read at the
    /// commit asked, whatever reopened the spec after it.
    #[test]
    fn a_done_spec_whose_only_run_stopped_early_reports_from_its_reopenings_start() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        let before = repository.head().unwrap().target().unwrap();
        put(
            tmp.path(),
            &repository,
            "specs/002-b/spec.md",
            &spec("in-progress"),
            "status(002): start",
        );
        change(tmp.path(), &repository, "src/b.rs", "feat(002): its code");
        put(
            tmp.path(),
            &repository,
            "specs/002-b/review.md",
            &clean_record("002-b", "2026-11-01T00:00:00Z")
                .replace("unread: []\n", "unread: []\nbudget-stop: true\n"),
            "review(002): stopped early",
        );
        let done = put(
            tmp.path(),
            &repository,
            "specs/002-b/spec.md",
            &spec("done"),
            "status(002): done",
        );
        let at_done = check(tmp.path());
        assert_eq!(at_done.done_specs.len(), 1);
        assert_eq!(at_done.done_specs[0].window_start, before.to_string());
        assert_eq!(at_done.done_specs[0].paths, ["src/b.rs"]);
        put(
            tmp.path(),
            &repository,
            "specs/002-b/spec.md",
            &spec("in-progress"),
            "status(002): reopen",
        );
        change(tmp.path(), &repository, "src/b3.rs", "feat(002): more");
        put(
            tmp.path(),
            &repository,
            "specs/002-b/spec.md",
            &spec("done"),
            "status(002): done again",
        );
        assert_eq!(check_at_rev(tmp.path(), done), at_done);
    }

    /// A path both a `done` and an `in-progress` spec claim passes while that spec
    /// is `in-progress`, since its gate reviews it; one a `done` spec shares with a
    /// spec at any other status is reported, and so is one two `done` specs share,
    /// under each whose window holds the commit. The pass defers and discharges
    /// nothing: once the open spec is `done`, the path is the other's to review.
    #[test]
    fn a_path_an_in_progress_spec_also_claims_passes() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        a_done_spec(tmp.path(), &repository);
        put(
            tmp.path(),
            &repository,
            "specs/003-c/spec.md",
            &spec("planned"),
            "spec(003): add",
        );
        change(
            tmp.path(),
            &repository,
            "src/open.rs",
            "fix(001, 002): both",
        );
        assert_eq!(check(tmp.path()).state(), UnreviewedState::None);
        change(
            tmp.path(),
            &repository,
            "src/planned.rs",
            "fix(002, 003): both",
        );
        let found = check(tmp.path());
        assert_eq!(found.done_specs.len(), 1);
        assert_eq!(found.done_specs[0].paths, ["src/planned.rs"]);

        put(
            tmp.path(),
            &repository,
            "specs/001-a/spec.md",
            &spec("done"),
            "status(001): done",
        );
        change(
            tmp.path(),
            &repository,
            "src/two.rs",
            "fix(001, 002): two done",
        );
        let found = check(tmp.path());
        let reported: Vec<(&str, &[String])> = found
            .done_specs
            .iter()
            .map(|spec| (spec.feature.as_str(), spec.paths.as_slice()))
            .collect();
        assert_eq!(
            reported,
            [
                ("001-a", &strings(&["src/two.rs"])[..]),
                (
                    "002-b",
                    &strings(&["src/open.rs", "src/planned.rs", "src/two.rs"])[..]
                ),
            ]
        );
    }

    /// A `done` spec's share of a commit naming no spec is its plan's, read from
    /// the commit's tree, and a deletion is reported apart.
    #[test]
    fn a_done_specs_share_reads_the_plans_the_commit_holds_and_sets_a_deletion_apart() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        a_done_spec(tmp.path(), &repository);
        put(
            tmp.path(),
            &repository,
            "specs/002-b/plan.md",
            "# Plan\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n| `src/planned.rs` | Edit |\n",
            "plan(002): affected files",
        );
        // One commit naming no spec, touching both specs' directories: 002's
        // plan names `src/planned.rs`, so it is 002's alone, and the unplanned
        // path is every claimant's, the `in-progress` 001 among them.
        write(&tmp.path().join("specs/001-a/notes.md"), "a\n");
        write(&tmp.path().join("specs/002-b/notes.md"), "b\n");
        write(&tmp.path().join("src/planned.rs"), "fn p() {}\n");
        write(&tmp.path().join("src/unplanned.rs"), "fn u() {}\n");
        commit_all(&repository, "both specs' notes and code");
        let found = check(tmp.path());
        assert_eq!(found.done_specs.len(), 1);
        assert_eq!(found.done_specs[0].paths, ["src/planned.rs"]);

        std::fs::remove_file(tmp.path().join("src/planned.rs")).unwrap();
        commit_all(&repository, "fix(002): remove it");
        let found = check(tmp.path());
        assert_eq!(found.done_specs[0].paths, [] as [&str; 0]);
        assert_eq!(found.done_specs[0].deleted, ["src/planned.rs"]);
        assert_eq!(found.done_specs[0].commits, 2);
    }

    /// The check at a commit reads that commit alone: moving `HEAD` and the disk
    /// on every source it reads changes nothing it says there.
    #[test]
    fn the_check_at_a_commit_reads_nothing_from_head_or_the_disk() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        a_done_spec(tmp.path(), &repository);
        put(
            tmp.path(),
            &repository,
            "specs/002-b/plan.md",
            "# Plan\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n| `src/planned.rs` | Edit |\n",
            "plan(002): affected files",
        );
        write(&tmp.path().join("specs/001-a/notes.md"), "a\n");
        write(&tmp.path().join("specs/002-b/notes.md"), "b\n");
        write(&tmp.path().join("src/planned.rs"), "fn p() {}\n");
        commit_all(&repository, "both specs' notes and code");
        change(tmp.path(), &repository, "src/b2.rs", "fix(002): while done");
        let x = change(tmp.path(), &repository, "src/u.rs", "chore: nobody's");
        let at_x = check(tmp.path());
        assert_eq!(at_x.done_specs.len(), 1);
        assert_eq!(at_x.done_specs[0].paths, ["src/b2.rs", "src/planned.rs"]);
        assert!(at_x.unclaimed.is_some());

        // Each source moves after `x`: the spec's status, its plan and the other
        // claimant's, its record, the spec list, the project record, the code.
        put(
            tmp.path(),
            &repository,
            "specs/002-b/spec.md",
            &spec("in-progress"),
            "status(002): reopen",
        );
        put(
            tmp.path(),
            &repository,
            "specs/002-b/plan.md",
            "# Plan\n",
            "plan(002): names nothing",
        );
        put(
            tmp.path(),
            &repository,
            "specs/001-a/plan.md",
            "# Plan\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n| `src/planned.rs` | Edit |\n",
            "plan(001): takes the path",
        );
        put(
            tmp.path(),
            &repository,
            "specs/002-b/review.md",
            &clean_record("002-b", "2026-11-03T00:00:00Z"),
            "review(002): again",
        );
        put(
            tmp.path(),
            &repository,
            "specs/004-d/spec.md",
            &spec("done"),
            "spec(004): add",
        );
        std::fs::remove_file(tmp.path().join("specs/review.md")).unwrap();
        std::fs::remove_file(tmp.path().join("src/b2.rs")).unwrap();
        commit_all(&repository, "chore: drop the project record and a file");
        assert_ne!(check(tmp.path()), at_x, "HEAD now reads differently");
        // And the disk moves past `HEAD`.
        std::fs::remove_dir_all(tmp.path().join("specs/002-b")).unwrap();
        write(&tmp.path().join("specs/001-a/spec.md"), &spec("done"));
        write(&tmp.path().join("src/u.rs"), "changed\n");

        assert_eq!(check_at_rev(tmp.path(), x), at_x);
    }

    /// A commit whose tree holds no project record places no window for the code
    /// no spec claims: the project is named as not examined, with the seed, and
    /// the state says so whatever else was found.
    #[test]
    fn a_tree_holding_no_project_record_is_not_examined_for_unclaimed_code() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        a_done_spec(tmp.path(), &repository);
        change(tmp.path(), &repository, "src/b2.rs", "fix(002): while done");
        change(tmp.path(), &repository, "src/u.rs", "chore: nobody's");
        let found = check(tmp.path());
        assert_eq!(found.state(), UnreviewedState::NotExamined);
        assert_eq!(found.done_specs.len(), 1, "the spec is still examined");
        assert_eq!(found.examined, 1);
        assert!(found.unclaimed.is_none());
        assert_eq!(found.not_examined.len(), 1);
        assert_eq!(found.not_examined[0].subject, "project");
        assert!(
            found.not_examined[0]
                .reason
                .contains("holds no `specs/review.md`")
                && found.not_examined[0].reason.contains("write-review --seed"),
            "{}",
            found.not_examined[0].reason
        );
    }

    /// A seed's adding commit may hold code, the hooks an install brings: the
    /// window starts after it, so none of it is reported.
    #[test]
    fn the_code_a_seeds_adding_commit_holds_is_not_reported() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        change(
            tmp.path(),
            &repository,
            "src/old.rs",
            "chore: before the seed",
        );
        install(tmp.path(), &repository);
        let found = check(tmp.path());
        assert_eq!(found.state(), UnreviewedState::None);
        assert_eq!(found.examined, 1);
    }

    /// A shallow history that stops before a window's start names each subject it
    /// cannot place, never as holding nothing.
    #[test]
    fn a_shallow_history_names_each_subject_it_cannot_place() {
        let tmp = tempfile::tempdir().unwrap();
        let origin = tmp.path().join("origin");
        let repository = project_with_a_spec(&origin);
        install(&origin, &repository);
        a_done_spec(&origin, &repository);
        change(&origin, &repository, "src/b2.rs", "fix(002): while done");
        let clone = tmp.path().join("clone");
        let status = std::process::Command::new("git")
            .args(["clone", "--quiet", "--depth", "1"])
            .arg(format!("file://{}", origin.display()))
            .arg(&clone)
            .status()
            .unwrap();
        assert!(status.success());
        let found = check(&clone);
        assert_eq!(found.state(), UnreviewedState::NotExamined);
        assert_eq!(found.examined, 0);
        assert_eq!(found.done_specs, []);
        let named: Vec<(&str, bool)> = found
            .not_examined
            .iter()
            .map(|skip| (skip.subject.as_str(), skip.reason.contains("shallow")))
            .collect();
        assert_eq!(named, [("002-b", true), ("project", true)]);
    }

    /// A spec whose status cannot be read, and a record that does not parse, are
    /// each named as not examined: neither is known to hold nothing.
    #[test]
    fn an_unreadable_status_or_record_is_not_examined() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        a_done_spec(tmp.path(), &repository);
        put(
            tmp.path(),
            &repository,
            "specs/003-c/spec.md",
            "# no frontmatter\n",
            "spec(003): damaged",
        );
        put(
            tmp.path(),
            &repository,
            "specs/004-d/notes.md",
            "no spec.md\n",
            "spec(004): notes alone",
        );
        put(
            tmp.path(),
            &repository,
            "specs/002-b/review.md",
            "---\nspec: 002-b\nreason: a: b: [\n---\n",
            "review(002): damaged",
        );
        put(
            tmp.path(),
            &repository,
            "specs/review.md",
            "---\nspec: project\nreason: a: b: [\n---\n",
            "review: damaged",
        );
        let found = check(tmp.path());
        assert_eq!(found.state(), UnreviewedState::NotExamined);
        assert_eq!(found.examined, 0);
        let named: Vec<(&str, &str)> = found
            .not_examined
            .iter()
            .map(|skip| {
                let why = if skip.reason.contains("does not parse") {
                    "record"
                } else if skip.reason.contains("status") {
                    "status"
                } else {
                    "other"
                };
                (skip.subject.as_str(), why)
            })
            .collect();
        assert_eq!(
            named,
            [
                ("002-b", "record"),
                ("003-c", "status"),
                ("004-d", "status"),
                ("project", "record"),
            ]
        );
    }

    /// A staging spec's frontmatter, folding into 002-b.
    fn staging(status: &str) -> String {
        format!("---\nstatus: {status}\ndependencies: []\nfolds-into: 002-b\n---\n\n# Staged\n")
    }

    /// AC19: code a staging spec folded into a `done` spec below `done` brought is
    /// reported under that spec, which stays `done`; a fold from `done` brings
    /// nothing, its own review having read its code.
    #[test]
    fn code_a_staging_spec_folded_below_done_brought_is_reported_under_its_upstream() {
        for (status, brought) in [("in-progress", true), ("done", false)] {
            let tmp = tempfile::tempdir().unwrap();
            let repository = project_with_a_spec(tmp.path());
            a_done_spec(tmp.path(), &repository);
            put(
                tmp.path(),
                &repository,
                "specs/1234.1-foo/spec.md",
                &staging("in-progress"),
                "spec: staging",
            );
            change(
                tmp.path(),
                &repository,
                "src/t.rs",
                "feat(1234.1): its code",
            );
            if status != "in-progress" {
                put(
                    tmp.path(),
                    &repository,
                    "specs/1234.1-foo/spec.md",
                    &staging(status),
                    "status(1234.1)",
                );
            }
            install(tmp.path(), &repository);
            assert_eq!(check(tmp.path()).state(), UnreviewedState::None);
            // A body-edit fold: the staging directory goes, and 002-b stays `done`.
            std::fs::remove_dir_all(tmp.path().join("specs/1234.1-foo")).unwrap();
            let fold = commit_all(&repository, "fold 1234.1 into 002");
            let found = check(tmp.path());
            let paths: Vec<&[String]> = found
                .done_specs
                .iter()
                .map(|spec| spec.paths.as_slice())
                .collect();
            if brought {
                assert_eq!(paths, [&strings(&["src/t.rs"])[..]], "{status}");
                assert_eq!(found.done_specs[0].feature, "002-b");
            } else {
                assert!(paths.is_empty(), "{status}: {paths:?}");
            }
            // The review after the fold ends what it brought.
            put(
                tmp.path(),
                &repository,
                "specs/002-b/review.md",
                &clean_record("002-b", "2026-11-05T00:00:00Z"),
                "review(002): after the fold",
            );
            assert!(check(tmp.path()).done_specs.is_empty(), "{status}");
            assert_eq!(
                check_at_rev(tmp.path(), fold).done_specs.is_empty(),
                !brought,
                "the fold's own commit still reads as it did"
            );
        }
    }

    /// Who reports which path, `done` specs first and then the project's entry.
    fn reporters(found: &Checked) -> Vec<(String, Vec<String>)> {
        let mut by: Vec<(String, Vec<String>)> = found
            .done_specs
            .iter()
            .map(|spec| (spec.feature.clone(), spec.paths.clone()))
            .collect();
        if let Some(code) = &found.unclaimed {
            by.push(("unclaimed".into(), code.paths.clone()));
        }
        by
    }

    /// The seed, then a staging spec folding into the `done` 002-b at `status`
    /// with a commit naming it alone (`src/t.rs`), and from `done` one after its
    /// review (`src/while_done.rs`); then the fold, 002's review after it, and a
    /// commit naming the staging spec (`src/late.rs`). Who reports what before the
    /// fold, after it, and after that last commit.
    fn fold_with_named_only_commits(status: &str) -> [Vec<(String, Vec<String>)>; 3] {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let repository = project_with_a_spec(root);
        a_done_spec(root, &repository);
        install(root, &repository);
        let spec = "specs/1234.1-foo/spec.md";
        put(
            root,
            &repository,
            spec,
            &staging("in-progress"),
            "spec: staging",
        );
        change(root, &repository, "src/t.rs", "feat(1234.1): its code");
        if status == "done" {
            let record = clean_record("1234.1-foo", "2026-11-04T00:00:00Z");
            put(
                root,
                &repository,
                "specs/1234.1-foo/review.md",
                &record,
                "review(1234.1)",
            );
            put(root, &repository, spec, &staging(status), "status(1234.1)");
            change(
                root,
                &repository,
                "src/while_done.rs",
                "fix(1234.1): after review",
            );
        }
        let before = reporters(&check(root));
        std::fs::remove_dir_all(root.join("specs/1234.1-foo")).unwrap();
        commit_all(&repository, "fold 1234.1 into 002");
        let after = reporters(&check(root));
        let record = clean_record("002-b", "2026-11-05T00:00:00Z");
        put(
            root,
            &repository,
            "specs/002-b/review.md",
            &record,
            "review(002): after",
        );
        change(
            root,
            &repository,
            "src/late.rs",
            "fix(1234.1): after the fold",
        );
        [before, after, reporters(&check(root))]
    }

    /// A staging spec's commits that name it without touching its directory are
    /// the project's to review once a fold removes it, the seed before them: no
    /// spec the tree holds claims them. So the project's entry reports, beside
    /// what a fold below `done` brought its upstream, the code the staging spec's
    /// own review read, and is the only one reporting what it changed after that
    /// review and a commit naming it after the fold.
    #[test]
    fn a_folded_staging_specs_named_only_commits_are_the_projects_to_review() {
        let entry = |who: &str, paths: &[&str]| (who.to_string(), strings(paths));
        let [before, after, late] = fold_with_named_only_commits("in-progress");
        assert_eq!(before, [], "before a fold below `done`");
        assert_eq!(
            after,
            [
                entry("002-b", &["src/t.rs"]),
                entry("unclaimed", &["src/t.rs"])
            ],
            "after a fold below `done`"
        );
        assert_eq!(
            late,
            [entry("unclaimed", &["src/late.rs", "src/t.rs"])],
            "after 002's review and a commit naming the folded spec"
        );
        let [before, after, late] = fold_with_named_only_commits("done");
        assert_eq!(before, [entry("1234.1-foo", &["src/while_done.rs"])]);
        assert_eq!(
            after,
            [entry("unclaimed", &["src/t.rs", "src/while_done.rs"])],
            "after a fold from `done`"
        );
        assert_eq!(
            late,
            [entry(
                "unclaimed",
                &["src/late.rs", "src/t.rs", "src/while_done.rs"]
            )],
            "after 002's review and a commit naming the folded spec"
        );
    }

    /// A project in a subdirectory of its repository is checked as one at the root.
    #[test]
    fn a_project_in_a_subdirectory_is_checked() {
        let tmp = tempfile::tempdir().unwrap();
        let (repository, project) = subdirectory_project(tmp.path());
        write(&project.join("specs/001-a/spec.md"), &spec("in-progress"));
        commit_all(&repository, "spec(001): add");
        let added = install(&project, &repository);
        let reviewed = a_done_spec(&project, &repository);
        change(&project, &repository, "src/b2.rs", "fix(002): while done");
        change(&project, &repository, "src/u.rs", "chore: nobody's");
        change(
            tmp.path(),
            &repository,
            "elsewhere/x.rs",
            "chore: a neighbour",
        );
        let found = check(&project);
        assert_eq!(found.done_specs[0].window_start, reviewed.to_string());
        assert_eq!(found.done_specs[0].paths, ["src/b2.rs"]);
        let unclaimed = found.unclaimed.unwrap();
        assert_eq!(unclaimed.window_start, added.to_string());
        assert_eq!(unclaimed.paths, ["src/u.rs"]);
    }

    /// What the check reports as unclaimed is what a project review's scope
    /// holds: one function answers both, the paths a record leaves open among them.
    #[test]
    fn the_unclaimed_paths_are_a_project_reviews_scope() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        let added = install(tmp.path(), &repository);
        change(tmp.path(), &repository, "src/a.rs", "chore: tidy");
        let must = ReviewFinding {
            site: Some("walk".into()),
            rule: "BE-INPUT-001".into(),
            severity: ReviewSeverity::Must,
            file: "src/a.rs".into(),
            line_range: "1-1".into(),
            confidence: "high".into(),
            summary: "a summary".into(),
            finding: "a finding".into(),
            rule_text: "the rule".into(),
            auto_fixable: false,
            suggested_fix: String::new(),
            disposition: crate::schema::primitives::Disposition::default(),
            decision_key: None,
        };
        review(tmp.path(), &repository, added, vec![must]);
        change(
            tmp.path(),
            &repository,
            "src/fix.rs",
            "chore: review, with a fix",
        );
        let scope = project_scope(tmp.path(), None).unwrap().unwrap();
        let unclaimed = check(tmp.path()).unclaimed.unwrap();
        assert_eq!(unclaimed.window_start, scope.diff_base);
        assert_eq!(unclaimed.paths, ["src/a.rs", "src/fix.rs"]);
        assert_eq!(unclaimed.paths, scope.scope.into_iter().collect::<Vec<_>>());
        assert_eq!(
            unclaimed.commits, 1,
            "the open finding's file is no commit's"
        );
    }

    /// AC17: with `since`, the check runs at each first-parent commit back to it
    /// and counts those reporting code, a branch's commits behind a merge left out.
    #[test]
    fn since_counts_the_first_parent_commits_reporting_code() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        let root = repository.head().unwrap().target().unwrap();
        change(
            tmp.path(),
            &repository,
            "src/old.rs",
            "chore: before the seed",
        );
        let added = install(tmp.path(), &repository);
        change(tmp.path(), &repository, "src/a.rs", "chore: first");
        review(tmp.path(), &repository, added, Vec::new());
        commit_all(&repository, "review: the project");
        // A branch of two commits, merged: its own commits are not first-parent.
        let main_tip = repository.head().unwrap().target().unwrap();
        let main_ref = repository.head().unwrap().name().unwrap().to_string();
        change(
            tmp.path(),
            &repository,
            "src/branch1.rs",
            "chore: on a branch",
        );
        let branch_tip = change(tmp.path(), &repository, "src/branch2.rs", "chore: again");
        repository
            .reference(&main_ref, main_tip, true, "rewind main")
            .unwrap();
        repository.set_head(&main_ref).unwrap();
        let signature = git2::Signature::now("Test", "test@example.com").unwrap();
        let tree = repository.find_commit(branch_tip).unwrap().tree().unwrap();
        let merged = repository
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                "merge the branch",
                &tree,
                &[
                    &repository.find_commit(main_tip).unwrap(),
                    &repository.find_commit(branch_tip).unwrap(),
                ],
            )
            .unwrap();
        let args = CheckUnreviewedCodeArgs {
            at: Some(merged.to_string()),
            since: Some(added.to_string()),
        };
        let found = compute(&args, tmp.path()).unwrap();
        assert_eq!(found.at, merged.to_string());
        assert_eq!(
            found.since,
            Some(SinceCount {
                walked: 3,
                reporting: 2,
                not_examined: 0,
            }),
            "`first` and the merge report code, and the review's commit none"
        );
        assert_eq!(found.checked.state(), UnreviewedState::Found);
        // `since` at the tip walks nothing, and a commit holding no record counts
        // apart from one reporting code.
        let none = compute(
            &CheckUnreviewedCodeArgs {
                at: Some(merged.to_string()),
                since: Some(merged.to_string()),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(none.since, Some(SinceCount::default()));
        let cut = compute(
            &CheckUnreviewedCodeArgs {
                at: Some(added.to_string()),
                since: Some(root.to_string()),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            cut.since,
            Some(SinceCount {
                walked: 2,
                reporting: 0,
                not_examined: 1,
            })
        );
    }

    fn config(root: &Path, body: &str) {
        write(&root.join(".ductus/config.toml"), body);
    }

    /// `[review] unreviewed-push`: `block` when absent, `warn` when set, and any
    /// other value an error naming the key and the file, never a fallback.
    #[test]
    fn the_mode_is_block_unless_the_key_says_warn_and_any_other_value_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(
            unreviewed_push(tmp.path()).unwrap(),
            UnreviewedPushMode::Block
        );
        for (body, mode) in [
            ("", UnreviewedPushMode::Block),
            (
                "[review]\ntech-stack-verified = true\n",
                UnreviewedPushMode::Block,
            ),
            (
                "[review]\nunreviewed-push = \"block\"\n",
                UnreviewedPushMode::Block,
            ),
            (
                "[review]\nunreviewed-push = \"warn\"\n",
                UnreviewedPushMode::Warn,
            ),
        ] {
            config(tmp.path(), body);
            assert_eq!(unreviewed_push(tmp.path()).unwrap(), mode, "{body}");
        }
        for (body, names) in [
            (
                "[review]\nunreviewed-push = \"Warn\"\n",
                "the value is \"Warn\"",
            ),
            ("[review]\nunreviewed-push = \"\"\n", "the value is \"\""),
            ("[review]\nunreviewed-push = true\n", "the value is true"),
            ("review = \"warn\"\n", "`review` is string, not a table"),
        ] {
            config(tmp.path(), body);
            let error = unreviewed_push(tmp.path()).unwrap_err();
            assert!(
                matches!(error, PrimitiveError::InvalidUnreviewedPush { .. }),
                "{body}: {error}"
            );
            let message = error.to_string();
            assert!(
                message.contains("`[review] unreviewed-push`")
                    && message.contains(".ductus/config.toml")
                    && message.contains(names),
                "{body}: {message}"
            );
        }
        config(tmp.path(), "[review\n");
        let error = unreviewed_push(tmp.path()).unwrap_err();
        assert!(matches!(error, PrimitiveError::Toml { .. }), "{error}");
        assert!(error.to_string().contains(".ductus/config.toml"));
    }

    /// An invalid key halts the check before it reads anything, and a valid one is
    /// the result's `mode`.
    #[test]
    fn the_result_states_the_mode_and_an_invalid_key_halts_the_check() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        config(tmp.path(), "[review]\nunreviewed-push = \"warn\"\n");
        let result = run(&CheckUnreviewedCodeArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.mode, UnreviewedPushMode::Warn);
        assert_eq!(result.state, UnreviewedState::None);
        assert_eq!((result.examined_count, result.reported_path_count), (1, 0));
        assert_eq!(result.commits_reporting, None);
        config(tmp.path(), "[review]\nunreviewed-push = \"off\"\n");
        let error = run(&CheckUnreviewedCodeArgs::default(), tmp.path()).unwrap_err();
        assert!(matches!(
            error,
            PrimitiveError::InvalidUnreviewedPush { .. }
        ));
    }

    /// Past the read size the three lists move to the file the result names, and
    /// the state and counts a gate reads stay inline.
    #[test]
    fn past_the_read_size_the_lists_move_to_a_file() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_a_spec(tmp.path());
        install(tmp.path(), &repository);
        a_done_spec(tmp.path(), &repository);
        put(
            tmp.path(),
            &repository,
            "specs/003-c/spec.md",
            "# no frontmatter\n",
            "spec(003): damaged",
        );
        change(tmp.path(), &repository, "src/b2.rs", "fix(002): while done");
        change(tmp.path(), &repository, "src/u.rs", "chore: nobody's");
        let inline = run(&CheckUnreviewedCodeArgs::default(), tmp.path()).unwrap();
        assert_eq!(inline.form, ListsForm::Inline);
        assert_eq!(inline.done_specs.as_ref().map(Vec::len), Some(1));
        config(tmp.path(), "[artifacts]\nread-size-bytes = 100\n");
        let result = run(&CheckUnreviewedCodeArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        assert!(
            result.done_specs.is_none()
                && result.unclaimed.is_none()
                && result.not_examined.is_none()
        );
        assert_eq!(result.state, UnreviewedState::NotExamined);
        assert_eq!(
            (
                result.done_specs_count,
                result.unclaimed_path_count,
                result.not_examined_count,
                result.examined_count,
                result.reported_path_count,
            ),
            (1, 1, 1, 2, 2)
        );
        let shown = serde_json::to_value(&result).unwrap();
        let file = tmp.path().join(result.lists_file.unwrap());
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
        assert_eq!(lists["done-specs"][0]["feature"], "002-b");
        assert_eq!(lists["unclaimed"]["paths"][0], "src/u.rs");
        assert_eq!(lists["not-examined"][0]["subject"], "003-c");
        let mut counted =
            crate::primitives::bounded_lists::counted_lists(PRIMITIVE, shown.as_object().unwrap());
        counted.sort();
        assert_eq!(counted, ["done-specs", "not-examined", "unclaimed"]);
        for list in counted {
            assert!(lists.get(&list).is_some(), "{list}");
        }
    }
}
