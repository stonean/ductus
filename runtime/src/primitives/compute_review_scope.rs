//! `compute-review-scope` — deterministic scope resolution for `/ductus:review`. What
//! it resolves is `schema::descriptions::COMPUTE_REVIEW_SCOPE`.
//!
//! A first run's base is the **parent** of the commit the spec moved to `in-progress` at, found
//! by the lookup shared with `check-stuck`, because the reopen flow commits the
//! back-edge flip together with the work it authorises — see
//! `specs/066-audit-records-and-the-completion-gate/scenarios/review-base-includes-the-transition-commit.md`.
//! A finished review moves it and one that stopped early does not, so stopping early leaves nothing
//! permanently unread (spec 079, AC5).
//!
//! The review reads the working tree, so the files modified since the base are the
//! diff from the base to the working tree, and an uncommitted transition starts the
//! window at `HEAD`. Answered from commits alone, a review run before its work was
//! committed — the pipeline's natural order — was scoped away from every file the work
//! touched
//! (`specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`).
//!
//! The project need not sit at its git repository's root: history is read by each path
//! from the work tree, and the files modified since the base are reported from the
//! project root, as the plan names them.
//!
//! Defined by
//! `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use git2::{DiffOptions, Oid, Repository, Sort, Tree};
use regex::Regex;

use crate::primitives::analyze_subjects::{is_review_contract, review_freshness};
use crate::primitives::check_review_gate::holds_done_until_reviewed;
use crate::primitives::check_stuck::find_in_progress_commit;
use crate::primitives::commit_attribution::Attributor;
use crate::primitives::{
    PrimitiveError, ProjectRepository, REVIEW_RECORD_FILE, Result, blob_text, frontmatter_status,
    load_review_record, parse_affected_files, parse_feature_dir, split_frontmatter,
};
use crate::schema::paths;
use crate::schema::primitives::{
    BaseSource, ComputeReviewScopeArgs, ComputeReviewScopeResult, ListsForm, RecordFreshness,
    ReviewBlock,
};

/// Execute the `compute-review-scope` primitive: [`compute`], returned inline
/// while it fits `[artifacts] read-size-bytes`, and past it with its lists in
/// the file it names and only their counts inline (`bounded_lists::place`).
///
/// # Errors
///
/// Every error [`compute`] returns, and those `bounded_lists::place` returns
/// for a malformed read size or a list file that cannot be written.
pub fn run(args: &ComputeReviewScopeArgs, repo: &Path) -> Result<ComputeReviewScopeResult> {
    let scope = compute(args, repo)?;
    let excluded: Vec<String> = scope.excluded.keys().cloned().collect();
    let mut result = ComputeReviewScopeResult {
        diff_base: scope.diff_base.clone(),
        base_source: scope.base_source,
        review_owed: scope.review_owed,
        form: ListsForm::Inline,
        scope_count: scope.scope.len(),
        modified_since_count: scope.modified_since.len(),
        plan_affected_count: scope.plan_affected.len(),
        deleted_count: scope.deleted.len(),
        excluded_count: excluded.len(),
        scope: Some(scope.scope.clone()),
        modified_since: Some(scope.modified_since.clone()),
        plan_affected: Some(scope.plan_affected.clone()),
        deleted: Some(scope.deleted.clone()),
        excluded: Some(excluded.clone()),
        lists_file: None,
    };
    let lists = serde_json::json!({
        "diff-base": scope.diff_base,
        "scope": scope.scope,
        "modified-since": scope.modified_since,
        "plan-affected": scope.plan_affected,
        "deleted": scope.deleted,
        "excluded": excluded,
    });
    if let Some(file) =
        super::bounded_lists::place(repo, "compute-review-scope", &args.feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.scope = None;
        result.modified_since = None;
        result.plan_affected = None;
        result.deleted = None;
        result.excluded = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// A review's diff base and its path lists, whole: what
/// `compute-review-scope` resolves, before its result is bounded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ReviewScope {
    /// See [`ComputeReviewScopeResult::diff_base`].
    pub(crate) diff_base: String,
    /// See [`ComputeReviewScopeResult::base_source`].
    pub(crate) base_source: BaseSource,
    /// See [`ComputeReviewScopeResult::review_owed`].
    pub(crate) review_owed: bool,
    /// See [`ComputeReviewScopeResult::scope`].
    pub(crate) scope: Vec<String>,
    /// See [`ComputeReviewScopeResult::modified_since`].
    pub(crate) modified_since: Vec<String>,
    /// See [`ComputeReviewScopeResult::plan_affected`].
    pub(crate) plan_affected: Vec<String>,
    /// See [`ComputeReviewScopeResult::deleted`].
    pub(crate) deleted: Vec<String>,
    /// Each path `modified_since` holds that the scope left out, with the specs
    /// that claimed it.
    pub(crate) excluded: BTreeMap<String, Vec<String>>,
}

/// Resolve the review's diff base and scope, every list whole. `write-review`
/// derives its denominator from this, never from the bounded result.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` would leave the spec
/// root, or when the feature directory, or a link inside it, resolves outside
/// the project, [`PrimitiveError::FeatureNotFound`] when the feature directory is
/// absent, [`PrimitiveError::Io`] when `plan.md` exists but cannot be read,
/// or [`PrimitiveError::Git`] for any libgit2 failure (repo discovery,
/// including a project no work tree contains, revparse of `--since`,
/// tree/diff lookup).
pub(crate) fn compute(args: &ComputeReviewScopeArgs, repo: &Path) -> Result<ReviewScope> {
    let layout = paths::Paths::load(repo);
    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;
    let feature_dir = repo.join(&layout.specs_root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: args.feature.clone(),
        });
    }
    // History names every path from the git work tree, and a project need not
    // sit at its repository's root. Asked by the path from the project root, a
    // project in a subdirectory found no transition and reviewed nothing.
    let project = ProjectRepository::discover(repo)?;
    let repository = &project.repository;
    let record_rel = format!(
        "{}/{}/{REVIEW_RECORD_FILE}",
        layout.specs_root, args.feature
    );

    let (diff_base, base_source) = resolve_base(args, &project, &layout.specs_root, &feature_dir)?;

    let modified_since = if diff_base.is_empty() {
        Vec::new()
    } else {
        diff_since(&project, &diff_base)?
    };

    let plan_affected = read_plan_affected(&feature_dir)?;

    // A path the window deleted cannot be read, so it leaves the scope a
    // review's `examined` is measured against, and stays named in
    // `modified-since` and `deleted` (066 scenario
    // `review-scope-reads-every-cell-path-and-sets-deleted-files-apart`).
    // The working tree decides, as the review reads it: a file deleted and
    // recreated, or removed from the index alone, is present.
    let deleted: BTreeSet<String> = modified_since
        .iter()
        .filter(|path| std::fs::symlink_metadata(repo.join(path)).is_err())
        .cloned()
        .collect();

    // Uncommitted work cannot be attributed, so it joins whole, even on a path
    // another spec's commit also changed. An untracked or staged file is part
    // of it, as `modified_since` takes them.
    let window = if diff_base.is_empty() {
        WindowClaims::default()
    } else {
        attribute_window(
            &project,
            repo,
            &layout.specs_root,
            &args.feature,
            &diff_base,
            &plan_affected,
        )?
    };
    let uncommitted: BTreeSet<String> = if diff_base.is_empty() {
        BTreeSet::new()
    } else {
        let head = repository.head()?.peel_to_commit()?.id().to_string();
        diff_since(&project, &head)?.into_iter().collect()
    };

    let (mut scope, excluded) = split_changes(&modified_since, &deleted, &window, &uncommitted);

    // A plan entry unchanged in the window may mean missing work on a spec's
    // first review, and on a later one only restates what was read. The record
    // at `HEAD` decides, so a run's own uncommitted record does not change the
    // answer between its two writes, and one naming no run (`last-run: null`)
    // records no review.
    let head_tree = repository.head()?.peel_to_commit()?.tree()?;
    if recorded_run(repository, &head_tree, &project.to_git(&record_rel))?.is_none() {
        scope.extend(
            plan_affected
                .iter()
                .filter(|path| !deleted.contains(*path) && !excluded.contains_key(*path))
                .cloned(),
        );
    }

    let own_prefix = format!("{}/{}/", layout.specs_root, args.feature);
    let record = load_review_record(&feature_dir);
    let run = record
        .as_present()
        .filter(|record| record.last_run.is_some());
    let rel_dir = format!("{}/{}", layout.specs_root, args.feature);
    let record_is_held = |record: &ReviewBlock| {
        holds_done_until_reviewed(record)
            || matches!(
                review_freshness(repo, &rel_dir, Some(record)),
                RecordFreshness::Stale { .. }
            )
    };
    // A deletion left the scope because it cannot be read, and is still a change:
    // this spec's unless only another spec's commit made it, as the scope's own rule.
    let its_own_deletions = deleted.iter().filter(|path| {
        window.own.contains(*path)
            || uncommitted.contains(*path)
            || !window.others.contains_key(*path)
    });
    let review_owed = run.is_none_or(record_is_held)
        || scope.iter().chain(its_own_deletions).any(|path| {
            path.strip_prefix(&own_prefix)
                .is_none_or(is_review_contract)
        });

    Ok(ReviewScope {
        diff_base,
        base_source,
        review_owed,
        scope: scope.into_iter().collect(),
        modified_since,
        plan_affected,
        deleted: deleted.into_iter().collect(),
        excluded,
    })
}

/// Where the review's window starts, and why: a `--since` override used
/// verbatim, `HEAD` while the spec's move into `in-progress` is uncommitted, the
/// last review run finished since that move, or the move's parent.
fn resolve_base(
    args: &ComputeReviewScopeArgs,
    project: &ProjectRepository,
    specs_root: &str,
    feature_dir: &Path,
) -> Result<(String, BaseSource)> {
    let repository = &project.repository;
    let spec_rel = project.to_git(&format!("{specs_root}/{}/spec.md", args.feature));
    let record_rel = format!("{specs_root}/{}/{REVIEW_RECORD_FILE}", args.feature);
    Ok(match &args.since {
        // An explicit base is used verbatim. No parent walk: an operator naming
        // a commit means that commit, and quietly reviewing one more than they
        // asked for would be its own scope surprise.
        Some(reference) => (
            repository
                .revparse_single(reference)?
                .peel_to_commit()?
                .id()
                .to_string(),
            BaseSource::Since,
        ),
        // The transition commit is the *boundary* of the work window, and a
        // boundary belongs inside the window it bounds — so a first run's base is
        // its parent. `base..HEAD` excludes the base's own changes, which is
        // harmless when `/ductus:implement` flips `planned -> in-progress` in a
        // commit holding only the status line, and silently fatal in the reopen
        // flow: `/ductus:amend`'s `done -> in-progress` back-edge is naturally
        // committed together with the work it authorises, so the window started
        // after the work and contained none of it. Two specs recorded 0 MUST /
        // 0 SHOULD over an empty subject that way on 2026-08-27 before the
        // operator caught it (scenario
        // `review-base-includes-the-transition-commit`).
        None if transition_is_uncommitted(project, specs_root, &args.feature, feature_dir)? => {
            // The transition that opened this window is not in history yet, so
            // the window starts at `HEAD`. History's newest transition is an
            // earlier window's — on a reopened spec, its first, months back.
            (
                repository.head()?.peel_to_commit()?.id().to_string(),
                BaseSource::UncommittedTransition,
            )
        }
        None => match find_in_progress_commit(repository, &spec_rel)? {
            Some(sha) => match last_review_commit(project, &record_rel, &spec_rel, &sha)? {
                Some(review) => (review, BaseSource::LastReview),
                None => (transition_parent(repository, &sha)?, BaseSource::Reopen),
            },
            None => (String::new(), BaseSource::None),
        },
    })
}

/// Divide the paths changed since the base between the scope and `excluded`.
/// Only a path a commit in the walk left to another spec leaves the scope. A
/// path the walk never met, as when `--since` names a commit off to the side of
/// `HEAD`, has no claimant to name, so it stays and no path goes unread.
fn split_changes(
    modified_since: &[String],
    deleted: &BTreeSet<String>,
    window: &WindowClaims,
    uncommitted: &BTreeSet<String>,
) -> (BTreeSet<String>, BTreeMap<String, Vec<String>>) {
    let mut scope = BTreeSet::new();
    let mut excluded = BTreeMap::new();
    for path in modified_since
        .iter()
        .filter(|path| !deleted.contains(*path))
    {
        let claimants = window
            .others
            .get(path)
            .filter(|_| !window.own.contains(path) && !uncommitted.contains(path));
        match claimants {
            Some(claimants) => {
                excluded.insert(path.clone(), claimants.iter().cloned().collect());
            }
            None => {
                scope.insert(path.clone());
            }
        }
    }
    (scope, excluded)
}

/// A review run as a committed record states it.
struct RecordedRun {
    last_run: String,
    /// The run recorded a skipped pass, a budget stop or an open fix-round
    /// loop, so what it did not read stays in the next window.
    stopped_early: bool,
    /// The fix-round loop was still open at this write.
    loop_open: bool,
}

/// The review run the record at `git_path` in `tree` states: `None` when the
/// tree holds no record, or one that does not parse or names no run.
fn recorded_run(
    repository: &Repository,
    tree: &Tree<'_>,
    git_path: &str,
) -> Result<Option<RecordedRun>> {
    let Some(text) = blob_text(repository, tree, git_path)? else {
        return Ok(None);
    };
    let Ok((frontmatter, _)) = split_frontmatter(&text, Path::new(git_path)) else {
        return Ok(None);
    };
    let Ok(record) = serde_norway::from_str::<ReviewBlock>(frontmatter) else {
        return Ok(None);
    };
    let Some(last_run) = record.last_run else {
        return Ok(None);
    };
    let loop_open = record.fix_rounds_open == Some(true);
    Ok(Some(RecordedRun {
        last_run,
        stopped_early: loop_open
            || record.budget_stop == Some(true)
            || !record.skipped_passes.is_empty(),
        loop_open,
    }))
}

/// The `last-run` a pre-057 `spec.md` states in its `review:` frontmatter block,
/// which `relocate-audit-records` moves into `review.md`.
fn run_in_spec_block(
    repository: &Repository,
    tree: &Tree<'_>,
    spec_git_path: &str,
) -> Result<Option<String>> {
    let Some(text) = blob_text(repository, tree, spec_git_path)? else {
        return Ok(None);
    };
    let Ok((frontmatter, _)) = split_frontmatter(&text, Path::new(spec_git_path)) else {
        return Ok(None);
    };
    let Ok(value) = serde_norway::from_str::<serde_norway::Value>(frontmatter) else {
        return Ok(None);
    };
    Ok(value
        .get("review")
        .and_then(|block| block.get("last-run"))
        .and_then(serde_norway::Value::as_str)
        .map(str::to_string))
}

/// The newest commit since the transition `transition_sha` at which a review
/// run finished, one that did not stop early. The transition itself is left
/// out, since work committed with a reopen belongs to the window it opens.
///
/// A commit finishes a run when no parent's record already holds that run with its
/// loop closed: a run's two writes share one `last-run`, and the fixes committed
/// between them are the ones its fix rounds re-read. Every parent, since a merge
/// carrying the record in finishes nothing, or what was committed after the run on
/// that branch would leave the window unread. A commit that only moves a run's record
/// out of `spec.md`'s frontmatter finishes nothing either, or an adopter's upgrade
/// would leave what was committed before it unread.
fn last_review_commit(
    project: &ProjectRepository,
    record_rel: &str,
    spec_git_path: &str,
    transition_sha: &str,
) -> Result<Option<String>> {
    let repository = &project.repository;
    let git_path = project.to_git(record_rel);
    let mut walk = repository.revwalk()?;
    walk.push_head()?;
    walk.hide(Oid::from_str(transition_sha)?)?;
    walk.set_sorting(Sort::TOPOLOGICAL)?;
    for oid in walk {
        let commit = repository.find_commit(oid?)?;
        let Some(run) = recorded_run(repository, &commit.tree()?, &git_path)? else {
            continue;
        };
        if run.stopped_early {
            continue;
        }
        let mut finishes_a_run = true;
        for parent in commit.parents() {
            let tree = parent.tree()?;
            let holds_it = recorded_run(repository, &tree, &git_path)?
                .is_some_and(|prior| prior.last_run == run.last_run && !prior.loop_open);
            let relocated = run_in_spec_block(repository, &tree, spec_git_path)?.as_ref()
                == Some(&run.last_run);
            if holds_it || relocated {
                finishes_a_run = false;
                break;
            }
        }
        if finishes_a_run {
            return Ok(Some(commit.id().to_string()));
        }
    }
    Ok(None)
}

/// What the commits in a window leave one spec's review.
#[derive(Default)]
struct WindowClaims {
    /// Paths a commit gave this spec.
    own: BTreeSet<String>,
    /// Paths a commit left to other specs, each with the specs that claimed it.
    others: BTreeMap<String, BTreeSet<String>>,
}

/// Walk `base..HEAD` and apply AC6's rule to each commit. A feature directory
/// [`parse_feature_dir`] does not recognize can be claimed by no commit, so
/// nothing is left to another spec and every changed path stays in scope.
fn attribute_window(
    project: &ProjectRepository,
    repo: &Path,
    specs_root: &str,
    feature: &str,
    base: &str,
    plan_affected: &[String],
) -> Result<WindowClaims> {
    let mut window = WindowClaims::default();
    let Some(form) = parse_feature_dir(feature) else {
        return Ok(window);
    };
    let repository = &project.repository;
    let mut walk = repository.revwalk()?;
    walk.push_head()?;
    walk.hide(Oid::from_str(base)?)?;
    let attributor = Attributor::new(project, repo, specs_root);
    let own_prefix = format!("{specs_root}/{feature}/");
    let own_plan = PlanEntries::new(plan_affected);
    let mut plans: BTreeMap<String, PlanEntries> = BTreeMap::new();

    for oid in walk {
        let commit = repository.find_commit(oid?)?;
        let claims = attributor.claims(&commit)?;
        if claims.claimants.is_empty() {
            window.own.extend(claims.paths);
            continue;
        }
        let others: Vec<String> = claims
            .claimants
            .iter()
            .filter(|claimant| **claimant != form)
            .map(|claimant| attributor.name_of(claimant))
            .collect();
        if !claims.is_claimed_by(&form) {
            for path in claims.paths {
                window
                    .others
                    .entry(path)
                    .or_default()
                    .extend(others.iter().cloned());
            }
            continue;
        }

        // This spec claims the commit, alone or with others. Alone, every path
        // is its own or unplanned, and the unplanned rule gives it those too.
        for claimant in &claims.claimants {
            if let Some(directory) = attributor.directory_of(claimant)
                && !plans.contains_key(directory)
            {
                let entries = read_plan_affected(&repo.join(specs_root).join(directory))?;
                plans.insert(directory.to_string(), PlanEntries::new(&entries));
            }
        }
        let takes_unplanned = claims
            .first_named
            .as_ref()
            .is_none_or(|first| *first == form);
        for path in claims.paths {
            let unplanned = !claims
                .claimants
                .iter()
                .filter_map(|claimant| attributor.directory_of(claimant))
                .filter_map(|directory| plans.get(directory))
                .any(|plan| plan.names(&path));
            if path.starts_with(&own_prefix)
                || own_plan.names(&path)
                || (unplanned && takes_unplanned)
            {
                window.own.insert(path);
            } else {
                window
                    .others
                    .entry(path)
                    .or_default()
                    .extend(others.iter().cloned());
            }
        }
    }
    Ok(window)
}

/// A plan's `Affected Files` entries, as a test of whether one names a path. An
/// entry may be a literal path, a directory ending in `/`, or a pattern with
/// `*`, `?`, `**` or `{a, b}`, as the corpus's plans write them.
struct PlanEntries {
    literal: BTreeSet<String>,
    patterns: Vec<Regex>,
}

impl PlanEntries {
    fn new(entries: &[String]) -> Self {
        let mut literal = BTreeSet::new();
        let mut patterns = Vec::new();
        for entry in entries {
            match entry_pattern(entry) {
                Some(pattern) => patterns.push(pattern),
                None => {
                    literal.insert(entry.clone());
                }
            }
        }
        Self { literal, patterns }
    }

    fn names(&self, path: &str) -> bool {
        self.literal.contains(path) || self.patterns.iter().any(|pattern| pattern.is_match(path))
    }
}

/// The regular expression an entry stands for: `None` for a literal path, and
/// for a pattern too malformed to compile, such as an unclosed `{`, which is
/// then matched only as the text it is.
fn entry_pattern(entry: &str) -> Option<Regex> {
    if !entry.ends_with('/') && !entry.contains(['*', '?', '{']) {
        return None;
    }
    let mut pattern = String::from("^");
    let mut chars = entry.chars().peekable();
    let mut depth = 0_usize;
    while let Some(c) = chars.next() {
        match c {
            '*' if chars.peek() == Some(&'*') => {
                chars.next();
                pattern.push_str(".*");
            }
            '*' => pattern.push_str("[^/]*"),
            '?' => pattern.push_str("[^/]"),
            '{' => {
                depth += 1;
                pattern.push_str("(?:");
            }
            '}' if depth > 0 => {
                depth -= 1;
                pattern.push(')');
            }
            ',' if depth > 0 => pattern.push('|'),
            ' ' if depth > 0 => {}
            other => pattern.push_str(&regex::escape(&other.to_string())),
        }
    }
    if entry.ends_with('/') {
        pattern.push_str(".*");
    }
    pattern.push('$');
    Regex::new(&pattern).ok()
}

/// Whether the spec's move into `in-progress` is still uncommitted: `spec.md`
/// on disk reads `in-progress` and the spec at `HEAD` does not — `done`,
/// `planned`, any other status, or no spec at all.
///
/// Keyed on the on-disk status being `in-progress`, not on the two statuses
/// differing: an uncommitted `in-progress -> done` flip also differs, and
/// moving that review's base to `HEAD` would drop the committed work from its
/// window. An on-disk spec whose status cannot be read is no evidence of an
/// uncommitted transition, so the base resolves from history; the diff to the
/// working tree keeps the change in scope either way, so the cost is a wider
/// window, never a missed change.
///
/// # Errors
///
/// [`PrimitiveError::Git`] when `HEAD`'s copy of the spec cannot be read —
/// an unborn `HEAD` among them.
pub(crate) fn transition_is_uncommitted(
    project: &ProjectRepository,
    specs_root: &str,
    feature: &str,
    feature_dir: &Path,
) -> Result<bool> {
    let on_disk = feature_dir.join("spec.md");
    let disk_status = std::fs::read_to_string(&on_disk)
        .ok()
        .and_then(|content| frontmatter_status(&content, &on_disk));
    if disk_status.as_deref() != Some("in-progress") {
        return Ok(false);
    }
    let rel = format!("{specs_root}/{feature}/spec.md");
    let head_status = project
        .read_at_head(&rel)?
        .and_then(|content| frontmatter_status(&content, Path::new(&rel)));
    Ok(head_status.as_deref() != Some("in-progress"))
}

/// The first parent of the status-transition commit `sha`, or `sha` itself
/// when it has no parent.
///
/// A root transition commit has nothing earlier to diff from, so it stays as
/// it resolves today rather than erroring — an unusual history is not a defect
/// to halt a review on.
///
/// Deliberately applied here rather than inside `find_in_progress_commit`:
/// `check-stuck` shares that lookup and counts `tasks.md` commits *since* the
/// transition, where the transition commit itself is correctly the origin. A
/// shared helper returning different commits to its callers would be the
/// drift this placement keeps out.
pub(crate) fn transition_parent(repository: &Repository, sha: &str) -> Result<String> {
    let commit = repository.find_commit(Oid::from_str(sha)?)?;
    Ok(match commit.parent(0) {
        Ok(parent) => parent.id().to_string(),
        Err(_) => sha.to_string(),
    })
}

/// Diff `base_sha` to the working tree: the sorted set of changed file paths
/// inside the project, named from the project root as the plan names them.
/// Committed, staged and unstaged changes and untracked files all count, and
/// ignored files do not. A change outside the project directory is another
/// project's work and is left out.
///
/// The working tree, because that is what the review reads. A `..HEAD` diff
/// left out every uncommitted change, which in the pipeline's natural order —
/// review before the one commit at the end — is the whole change.
fn diff_since(project: &ProjectRepository, base_sha: &str) -> Result<Vec<String>> {
    let repo = &project.repository;
    let base_tree = repo.find_commit(Oid::from_str(base_sha)?)?.tree()?;

    // One diff, base tree → index → working tree, so committed, staged and
    // unstaged changes arrive together; untracked files surface too, as
    // `diff-cross-spec` takes them.
    let mut opts = project_diff_options(project);
    opts.include_untracked(true).recurse_untracked_dirs(true);
    let diff = repo.diff_tree_to_workdir_with_index(Some(&base_tree), Some(&mut opts))?;
    project_paths(project, &diff)
}

/// Diff options bounded to the project directory, so a project in a
/// subdirectory does not walk its neighbours' trees. Literal, because that
/// path may hold `*`, `?` or `[`, which as a glob would match no file and
/// report no change (QUAL-CLAIM-001). Shared with `brief-re-read`'s
/// tree-to-tree diff, so the two bound a project alike.
pub(crate) fn project_diff_options(project: &ProjectRepository) -> DiffOptions {
    let mut opts = DiffOptions::new();
    let project_dir = project.to_git("");
    let project_dir = project_dir.trim_end_matches('/');
    if !project_dir.is_empty() {
        opts.pathspec(project_dir).disable_pathspec_match(true);
    }
    opts
}

/// Every path `diff` changes inside the project, named from the project root
/// and sorted. A change outside the project directory is another project's
/// work and is left out.
pub(crate) fn project_paths(
    project: &ProjectRepository,
    diff: &git2::Diff<'_>,
) -> Result<Vec<String>> {
    let mut files: BTreeSet<String> = BTreeSet::new();
    diff.foreach(
        &mut |delta, _| {
            let path = delta.new_file().path().or_else(|| delta.old_file().path());
            if let Some(path) = path
                && let Some(inside) = project.to_project(&path.to_string_lossy())
            {
                files.insert(inside.to_string());
            }
            true
        },
        None,
        None,
        None,
    )?;
    Ok(files.into_iter().collect())
}

/// Parse a feature's `plan.md` `## Affected Files` section into a list of file
/// paths, using the shared canonical-table parser (`parse_affected_files`) so
/// `compute-review-scope` and the writeCode plan reader agree on one format.
/// Returns an empty list when `plan.md` is absent or has no such section.
///
/// # Errors
///
/// Returns [`PrimitiveError::Io`] when `plan.md` exists but cannot be read:
/// the same empty list there would read as a plan naming no files, and the
/// review scope would shrink to the diff without saying why (QUAL-CLAIM-001).
pub(crate) fn read_plan_affected(feature_dir: &Path) -> Result<Vec<String>> {
    let path = feature_dir.join("plan.md");
    match std::fs::read_to_string(&path) {
        Ok(content) => Ok(parse_affected_files(&content)),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(source) => Err(PrimitiveError::Io { path, source }),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{self, write};
    use git2::Repository;
    use std::fs;

    /// Stage everything and commit; returns the new commit's sha.
    fn commit_all(repo: &Repository, message: &str) -> String {
        git_fixture::commit_all(repo, message).to_string()
    }

    fn spec(status: &str) -> String {
        format!("---\nstatus: {status}\ndependencies: []\n---\n\n# X\n")
    }

    fn args(feature: &str, since: Option<&str>) -> ComputeReviewScopeArgs {
        ComputeReviewScopeArgs {
            feature: feature.to_string(),
            since: since.map(str::to_string),
        }
    }

    /// A repo where 001-x goes planned → in-progress, then two source files
    /// are added after the transition. Returns (tempdir, in-progress sha).
    fn repo_with_progress() -> (tempfile::TempDir, String) {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/001-x/spec.md");
        write(&spec_path, &spec("planned"));
        commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        let sha = commit_all(&repo, "chore: begin");
        write(&tmp.path().join("src/a.rs"), "fn a() {}\n");
        write(&tmp.path().join("src/b.rs"), "fn b() {}\n");
        commit_all(&repo, "feat: implement");
        (tmp, sha)
    }

    /// A `plan.md` that exists but cannot be read is an error, not a plan
    /// naming no files: the empty list would shrink the scope to the diff
    /// without saying why (QUAL-CLAIM-001). An absent plan stays empty.
    #[test]
    fn an_unreadable_plan_is_an_error_not_an_empty_plan() {
        let (tmp, _) = repo_with_progress();
        assert!(compute(&args("001-x", None), tmp.path()).is_ok());
        fs::write(
            tmp.path().join("specs/001-x/plan.md"),
            b"\xff\xfe not UTF-8",
        )
        .unwrap();
        let err = compute(&args("001-x", None), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Io { .. }), "{err:?}");
    }

    #[test]
    fn diff_base_is_the_transition_commits_parent_and_scope_lists_modified_files() {
        let (tmp, sha) = repo_with_progress();
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        // The base is the parent of the transition, so the transition commit
        // is inside the window rather than excluded from it.
        let parent = Repository::discover(tmp.path())
            .unwrap()
            .find_commit(Oid::from_str(&sha).unwrap())
            .unwrap()
            .parent(0)
            .unwrap()
            .id()
            .to_string();
        assert_eq!(result.diff_base, parent);
        assert!(result.modified_since.contains(&"src/a.rs".to_string()));
        assert!(result.modified_since.contains(&"src/b.rs".to_string()));
        // Widening by the transition commit adds only its status-line change.
        assert!(
            result
                .modified_since
                .contains(&"specs/001-x/spec.md".to_string()),
            "{:?}",
            result.modified_since
        );
    }

    /// The 017/020 shape: `/ductus:amend`'s back-edge flip committed together
    /// with the work it authorises. A base *at* that commit excludes the work,
    /// so the review examines nothing and still reports clean.
    #[test]
    fn work_committed_with_the_back_edge_flip_is_inside_the_window() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/001-x/spec.md");
        write(&spec_path, &spec("done"));
        write(&tmp.path().join("src/old.rs"), "fn old() {}\n");
        commit_all(&repo, "feat: ship it");

        // One commit: the reopen and the deliverable together.
        write(&spec_path, &spec("in-progress"));
        write(&tmp.path().join("src/new.rs"), "fn new() {}\n");
        commit_all(&repo, "fix: reopen and implement");

        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert!(
            result.modified_since.contains(&"src/new.rs".to_string()),
            "the reviewed deliverable fell outside the window: {:?}",
            result.modified_since
        );
        assert!(
            result.scope.contains(&"src/new.rs".to_string()),
            "{:?}",
            result.scope
        );
    }

    /// A transition commit with no parent has nothing earlier to diff from.
    #[test]
    fn a_root_transition_commit_falls_back_to_itself() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        let sha = commit_all(&repo, "feat: root");
        write(&tmp.path().join("src/a.rs"), "fn a() {}\n");
        commit_all(&repo, "feat: more");

        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, sha, "root commit should be its own base");
    }

    #[test]
    fn since_override_replaces_the_derived_base() {
        let (tmp, _sha) = repo_with_progress();
        // Override to HEAD → no files modified since HEAD.
        let result = compute(&args("001-x", Some("HEAD")), tmp.path()).unwrap();
        let head = Repository::discover(tmp.path())
            .unwrap()
            .head()
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .id()
            .to_string();
        assert_eq!(result.diff_base, head);
        assert_eq!(result.modified_since, [] as [String; 0]);
    }

    #[test]
    fn since_is_used_verbatim_with_no_parent_walk() {
        // An operator naming a commit means that commit. Passing the
        // transition sha explicitly must resolve to it, not to its parent.
        let (tmp, sha) = repo_with_progress();
        let result = compute(&args("001-x", Some(&sha)), tmp.path()).unwrap();
        assert_eq!(result.diff_base, sha);
    }

    #[test]
    fn scope_unions_plan_affected_with_modified_since() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/001-x/spec.md");
        write(&spec_path, &spec("planned"));
        commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: begin");
        // Only one file modified since; plan lists three.
        write(&tmp.path().join("src/only.rs"), "fn only() {}\n");
        write(
            &tmp.path().join("specs/001-x/plan.md"),
            "# Plan\n\n## Affected Files\n\n\
             | File | Action | Purpose |\n\
             | --- | --- | --- |\n\
             | `src/one.rs` | Edit | one |\n\
             | `src/two.rs` | Edit | the second |\n\
             | `src/three.rs` | Edit | three |\n",
        );
        commit_all(&repo, "feat: implement");
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(
            result.plan_affected,
            vec!["src/one.rs", "src/two.rs", "src/three.rs"]
        );
        // The regression this guards: plan_affected (3) outnumbers
        // modified_since (2: only.rs + plan.md), so the old larger-of rule
        // returned the plan set alone and `src/only.rs` — the only source file
        // the work actually touched — was never reviewed.
        assert!(
            result.scope.contains(&"src/only.rs".to_string()),
            "the file the work touched must be in scope, not dropped for being \
             in the smaller set: {:?}",
            result.scope
        );
        for f in &result.plan_affected {
            assert!(result.scope.contains(f), "plan-affected {f} missing");
        }
        for f in &result.modified_since {
            assert!(result.scope.contains(f), "modified-since {f} missing");
        }
        // Union, deduplicated and sorted.
        let mut expected: Vec<String> = result
            .plan_affected
            .iter()
            .chain(result.modified_since.iter())
            .cloned()
            .collect();
        expected.sort();
        expected.dedup();
        assert_eq!(result.scope, expected);
    }

    #[test]
    fn no_in_progress_and_no_since_yields_empty_scope() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("planned"));
        commit_all(&repo, "feat: plan");
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, "");
        assert_eq!(result.base_source, BaseSource::None);
        assert_eq!(result.scope, [] as [String; 0]);
        assert_eq!(result.modified_since, [] as [String; 0]);
    }

    /// A project need not sit at its repository's root. History names its
    /// spec by the path from the work tree, so reading it by the path from the
    /// project root found no transition and returned an empty base and scope.
    /// The files modified since are reported from the project root, as the
    /// plan names them; a change outside the project is not its work.
    #[test]
    fn a_project_in_a_repository_subdirectory_resolves_its_base() {
        use crate::primitives::git_fixture;
        let tmp = tempfile::tempdir().unwrap();
        let (repo, project) = git_fixture::subdirectory_project(tmp.path());
        let spec_path = project.join("specs/001-x/spec.md");
        write(&spec_path, &spec("planned"));
        git_fixture::commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        let sha = git_fixture::commit_all(&repo, "chore: begin");
        write(&project.join("src/a.rs"), "fn a() {}\n");
        write(&tmp.path().join("other/x.rs"), "fn x() {}\n");
        git_fixture::commit_all(&repo, "feat: implement");

        let result = compute(&args("001-x", None), &project).unwrap();
        let parent = repo
            .find_commit(sha)
            .unwrap()
            .parent(0)
            .unwrap()
            .id()
            .to_string();
        assert_eq!(result.diff_base, parent);
        assert_eq!(
            result.modified_since,
            vec!["specs/001-x/spec.md".to_string(), "src/a.rs".to_string()]
        );
    }

    /// A repo where 001-x went planned → in-progress with `src/old.rs`
    /// committed before the transition, so nothing has touched it since the
    /// base. Returns (tempdir, repository, the transition's parent).
    fn repo_in_progress_with_an_untouched_file() -> (tempfile::TempDir, Repository, String) {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/001-x/spec.md");
        write(&spec_path, &spec("planned"));
        write(&tmp.path().join("src/old.rs"), "fn old() {}\n");
        write(&tmp.path().join(".gitignore"), "target/\n");
        let parent = commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: begin");
        (tmp, repo, parent)
    }

    /// A plan cell naming two paths puts both in `plan-affected`. A file
    /// deleted since the base, in a commit or only on disk, stays in
    /// `modified-since`, is named in `deleted` and leaves `scope`, even when
    /// the plan names it; a file deleted and recreated is present, so in
    /// scope (066 scenario
    /// `review-scope-reads-every-cell-path-and-sets-deleted-files-apart`).
    #[test]
    fn a_two_path_cell_and_a_deleted_file_resolve_as_the_scenario_says() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/001-x/spec.md");
        write(&spec_path, &spec("planned"));
        write(
            &tmp.path().join("specs/001-x/plan.md"),
            "# Plan\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n\
             | `src/gone.rs` / `src/kept.rs` | Edit |\n",
        );
        for name in ["gone", "unstaged", "back", "kept"] {
            write(&tmp.path().join(format!("src/{name}.rs")), "fn f() {}\n");
        }
        commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: begin");

        fs::remove_file(tmp.path().join("src/gone.rs")).unwrap();
        let mut index = repo.index().unwrap();
        index.remove_path(Path::new("src/gone.rs")).unwrap();
        index.write().unwrap();
        fs::remove_file(tmp.path().join("src/back.rs")).unwrap();
        commit_all(&repo, "fix: drop gone and back");
        write(&tmp.path().join("src/back.rs"), "fn back() {}\n");
        fs::remove_file(tmp.path().join("src/unstaged.rs")).unwrap();

        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.plan_affected, ["src/gone.rs", "src/kept.rs"]);
        assert_eq!(result.deleted, ["src/gone.rs", "src/unstaged.rs"]);
        for path in &result.deleted {
            assert!(result.modified_since.contains(path), "{path}");
            assert!(!result.scope.contains(path), "{path} in scope");
        }
        for path in ["src/back.rs", "src/kept.rs"] {
            assert!(result.scope.contains(&path.to_string()), "{path}");
        }
    }

    fn head_sha(repo: &Repository) -> String {
        repo.head()
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .id()
            .to_string()
    }

    /// The review reads the working tree, so an edit nobody has committed is
    /// part of what it must read. A `..HEAD` diff left it out.
    #[test]
    fn an_unstaged_edit_is_in_scope() {
        let (tmp, _repo, parent) = repo_in_progress_with_an_untouched_file();
        write(&tmp.path().join("src/old.rs"), "fn old() { edited(); }\n");
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, parent);
        assert!(
            result.scope.contains(&"src/old.rs".to_string()),
            "{:?}",
            result.scope
        );
    }

    #[test]
    fn a_staged_edit_is_in_scope() {
        let (tmp, repo, _) = repo_in_progress_with_an_untouched_file();
        write(&tmp.path().join("src/old.rs"), "fn old() { staged(); }\n");
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("src/old.rs")).unwrap();
        index.write().unwrap();
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert!(
            result.modified_since.contains(&"src/old.rs".to_string()),
            "{:?}",
            result.modified_since
        );
    }

    #[test]
    fn an_untracked_file_is_in_scope_and_an_ignored_one_is_not() {
        let (tmp, _repo, _) = repo_in_progress_with_an_untouched_file();
        write(&tmp.path().join("tests/new_test.rs"), "#[test] fn t() {}\n");
        write(&tmp.path().join("target/debug/out.o"), "build output\n");
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert!(
            result
                .modified_since
                .contains(&"tests/new_test.rs".to_string()),
            "{:?}",
            result.modified_since
        );
        assert!(
            !result.scope.iter().any(|f| f.starts_with("target/")),
            "an ignored file entered the scope: {:?}",
            result.scope
        );
    }

    /// `--since=HEAD` used to name an empty window however much work sat
    /// uncommitted on top of it.
    #[test]
    fn since_head_yields_exactly_the_uncommitted_work() {
        let (tmp, repo, _) = repo_in_progress_with_an_untouched_file();
        write(&tmp.path().join("src/old.rs"), "fn old() { edited(); }\n");
        write(&tmp.path().join("src/new.rs"), "fn new() {}\n");
        let result = compute(&args("001-x", Some("HEAD")), tmp.path()).unwrap();
        assert_eq!(result.diff_base, head_sha(&repo));
        assert_eq!(result.modified_since, vec!["src/new.rs", "src/old.rs"]);
    }

    /// The reported shape: a `done` spec reopened by `/ductus:amend` and
    /// worked on, nothing committed. History's newest transition is the
    /// spec's first, so the base reached back to it and the window spanned
    /// every file since; the uncommitted transition starts the window at
    /// `HEAD` instead.
    #[test]
    fn an_uncommitted_reopen_puts_the_base_at_head() {
        let (tmp, repo, _) = repo_in_progress_with_an_untouched_file();
        write(&tmp.path().join("src/first_window.rs"), "fn w() {}\n");
        commit_all(&repo, "feat: implement");
        let spec_path = tmp.path().join("specs/001-x/spec.md");
        write(&spec_path, &spec("done"));
        commit_all(&repo, "chore: done");

        write(&spec_path, &spec("in-progress"));
        write(&tmp.path().join("src/old.rs"), "fn old() { fixed(); }\n");
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, head_sha(&repo));
        assert_eq!(result.base_source, BaseSource::UncommittedTransition);
        assert_eq!(
            result.modified_since,
            vec!["specs/001-x/spec.md", "src/old.rs"],
            "the window is the reopen's work, not every file since the first transition"
        );
    }

    /// `/ductus:implement` starting a `planned` spec, nothing committed yet.
    /// History holds no transition, so the base was empty and the scope the
    /// plan's alone.
    #[test]
    fn an_uncommitted_start_puts_the_base_at_head() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/001-x/spec.md");
        write(&spec_path, &spec("planned"));
        commit_all(&repo, "feat: plan");

        write(&spec_path, &spec("in-progress"));
        write(&tmp.path().join("src/a.rs"), "fn a() {}\n");
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, head_sha(&repo));
        assert_eq!(
            result.modified_since,
            vec!["specs/001-x/spec.md", "src/a.rs"]
        );
    }

    /// An uncommitted `in-progress → done` flip differs from `HEAD` too, but
    /// it opens no window: the base stays the committed transition's parent,
    /// so the committed work stays in the window.
    #[test]
    fn an_uncommitted_completion_leaves_the_base_alone() {
        let (tmp, repo, parent) = repo_in_progress_with_an_untouched_file();
        write(&tmp.path().join("src/a.rs"), "fn a() {}\n");
        commit_all(&repo, "feat: implement");
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("done"));
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, parent);
        assert!(
            result.modified_since.contains(&"src/a.rs".to_string()),
            "{:?}",
            result.modified_since
        );
    }

    /// The working-tree diff is bounded to the project: an untracked file
    /// beside it is another project's work.
    #[test]
    fn untracked_files_outside_a_subdirectory_project_are_left_out() {
        use crate::primitives::git_fixture;
        let tmp = tempfile::tempdir().unwrap();
        let (repo, project) = git_fixture::subdirectory_project(tmp.path());
        let spec_path = project.join("specs/001-x/spec.md");
        write(&spec_path, &spec("planned"));
        git_fixture::commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        git_fixture::commit_all(&repo, "chore: begin");

        write(&project.join("src/inside.rs"), "fn i() {}\n");
        write(&tmp.path().join("other/outside.rs"), "fn o() {}\n");
        let result = compute(&args("001-x", None), &project).unwrap();
        assert_eq!(
            result.modified_since,
            vec!["specs/001-x/spec.md", "src/inside.rs"]
        );
    }

    /// A project whose path from the work tree holds a glob character. As a
    /// glob the diff's pathspec matches no file, so the window would read as
    /// empty with the work on disk (QUAL-CLAIM-001); it is literal. The test
    /// above cannot tell the two apart, because `proj` matches either way.
    #[test]
    fn a_project_path_with_a_glob_character_is_matched_literally() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let project = tmp.path().join("app[v2]");
        let spec_path = project.join("specs/001-x/spec.md");
        write(&spec_path, &spec("planned"));
        write(&project.join("src/old.rs"), "fn old() {}\n");
        commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: begin");

        write(&project.join("src/old.rs"), "fn old() { edited(); }\n");
        write(&project.join("src/new.rs"), "fn new() {}\n");
        let result = compute(&args("001-x", None), &project).unwrap();
        assert_eq!(
            result.modified_since,
            vec!["specs/001-x/spec.md", "src/new.rs", "src/old.rs"]
        );
    }

    #[test]
    fn missing_feature_is_operational_error() {
        let tmp = tempfile::tempdir().unwrap();
        Repository::init(tmp.path()).unwrap();
        let err = compute(&args("999-nope", None), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    #[test]
    fn plan_absent_falls_back_to_modified_since() {
        let (tmp, _sha) = repo_with_progress();
        // No plan.md written → plan_affected empty → scope = modified_since.
        let result = compute(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.plan_affected, [] as [String; 0]);
        assert_eq!(result.scope, result.modified_since);
        assert_ne!(result.scope, [] as [String; 0]);
    }

    fn read_size(tmp: &tempfile::TempDir, bytes: u64) {
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            format!("[artifacts]\nread-size-bytes = {bytes}\n"),
        )
        .unwrap();
    }

    /// A result that fits is inline, with its counts. Past `[artifacts]
    /// read-size-bytes`, every list moves whole to the file the result names,
    /// none of them inline, and the counts still say how many; the list file
    /// joins no later scope (066 scenario `scope-results-have-a-bounded-size`).
    #[test]
    fn a_result_past_the_read_size_names_a_file_holding_every_list() {
        let (tmp, _) = repo_with_progress();
        read_size(&tmp, 1_000_000);
        let whole = compute(&args("001-x", None), tmp.path()).unwrap();
        let inline = run(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(inline.form, ListsForm::Inline);
        assert_eq!(inline.scope.as_deref(), Some(whole.scope.as_slice()));
        assert_eq!(inline.scope_count, whole.scope.len());
        assert_eq!(inline.lists_file, None);

        read_size(&tmp, 10);
        let bounded = run(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(bounded.form, ListsForm::File);
        assert_eq!(
            (
                &bounded.scope,
                &bounded.modified_since,
                &bounded.plan_affected,
                &bounded.deleted
            ),
            (&None, &None, &None, &None)
        );
        assert_eq!(bounded.excluded, None);
        assert_eq!(bounded.deleted_count, whole.deleted.len());
        assert_eq!(bounded.excluded_count, whole.excluded.len());
        assert_eq!(
            (
                bounded.scope_count,
                bounded.modified_since_count,
                bounded.plan_affected_count
            ),
            (
                whole.scope.len(),
                whole.modified_since.len(),
                whole.plan_affected.len()
            )
        );
        let file: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(tmp.path().join(bounded.lists_file.unwrap())).unwrap(),
        )
        .unwrap();
        assert_eq!(file["diff-base"], serde_json::json!(whole.diff_base));
        assert_eq!(
            file["excluded"],
            serde_json::json!(whole.excluded.keys().collect::<Vec<_>>())
        );
        assert_eq!(file["scope"], serde_json::json!(whole.scope));
        assert_eq!(
            file["modified-since"],
            serde_json::json!(whole.modified_since)
        );
        assert_eq!(
            file["plan-affected"],
            serde_json::json!(whole.plan_affected)
        );
        assert_eq!(file["deleted"], serde_json::json!(whole.deleted));
        assert_eq!(compute(&args("001-x", None), tmp.path()).unwrap(), whole);
    }

    // -- spec 079: the window, the scope, `excluded` and `review-owed` -------

    /// Specs 001-x and 002-y, with 001-x in `in-progress` at a committed
    /// transition and 002-y in `in-progress` since the first commit. `plan_x` and
    /// `plan_y` are the entries each spec's `plan.md` starts with.
    fn two_specs(plan_x: &[&str], plan_y: &[&str]) -> (tempfile::TempDir, Repository, String) {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let plan = |entries: &[&str]| {
            let rows: Vec<String> = entries
                .iter()
                .map(|entry| format!("| `{entry}` | Edit |"))
                .collect();
            format!(
                "# Plan\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n{}\n",
                rows.join("\n")
            )
        };
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("planned"));
        write(&tmp.path().join("specs/001-x/plan.md"), &plan(plan_x));
        write(
            &tmp.path().join("specs/002-y/spec.md"),
            &spec("in-progress"),
        );
        write(&tmp.path().join("specs/002-y/plan.md"), &plan(plan_y));
        write(&tmp.path().join("src/old.rs"), "fn old() {}\n");
        let parent = commit_all(&repo, "chore: seed");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repo, "status(001): planned -> in-progress");
        (tmp, repo, parent)
    }

    fn record(extra: &str, last_run: &str) -> String {
        format!(
            "---\nspec: 001-x\nlast-run: {last_run}\nmust-violations: 0\nblocking: false\n{extra}---\n\n# Review\n"
        )
    }

    /// Commit a review record for 001-x; returns the commit.
    fn commit_record(tmp: &tempfile::TempDir, repo: &Repository, extra: &str, run: &str) -> String {
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &record(extra, run),
        );
        commit_all(repo, "review(001)")
    }

    fn change(tmp: &tempfile::TempDir, repo: &Repository, file: &str, subject: &str) -> String {
        write(&tmp.path().join(file), &format!("{subject}\n"));
        commit_all(repo, subject)
    }

    fn x(tmp: &tempfile::TempDir) -> ReviewScope {
        compute(&args("001-x", None), tmp.path()).unwrap()
    }

    fn y(tmp: &tempfile::TempDir) -> ReviewScope {
        compute(&args("002-y", None), tmp.path()).unwrap()
    }

    fn has(scope: &ReviewScope, path: &str) -> bool {
        scope.scope.iter().any(|entry| entry == path)
    }

    #[test]
    fn with_no_earlier_review_the_window_starts_at_the_reopen_and_a_review_is_owed() {
        let (tmp, _repo, parent) = two_specs(&[], &[]);
        let scope = x(&tmp);
        assert_eq!(scope.diff_base, parent);
        assert_eq!(scope.base_source, BaseSource::Reopen);
        assert!(scope.review_owed);
    }

    #[test]
    fn a_review_in_the_reopening_moves_the_start_to_its_commit() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        change(&tmp, &repo, "src/before.rs", "feat(001): before the review");
        let review = commit_record(&tmp, &repo, "", "2026-11-01T00:00:00Z");
        change(&tmp, &repo, "src/after.rs", "feat(001): after the review");

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, review);
        assert_eq!(scope.base_source, BaseSource::LastReview);
        assert_eq!(scope.modified_since, ["src/after.rs"]);
        assert_eq!(scope.scope, ["src/after.rs"]);
    }

    #[test]
    fn the_result_names_where_the_window_starts_and_whether_a_review_is_owed() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        commit_record(&tmp, &repo, DISPOSITIONS, "2026-11-01T00:00:00Z");
        let result = run(&args("001-x", None), tmp.path()).unwrap();
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["base-source"], "last-review");
        assert_eq!(json["review-owed"], false);
        assert_eq!(json["excluded-count"], 0);
    }

    #[test]
    fn a_review_older_than_the_reopen_leaves_the_start_at_the_reopen() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repo, "status(001): start");
        commit_record(&tmp, &repo, "", "2026-10-01T00:00:00Z");
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("done"));
        let parent = commit_all(&repo, "status(001): done");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repo, "status(001): reopen");
        change(&tmp, &repo, "src/new.rs", "feat(001): after the reopen");

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, parent);
        assert_eq!(scope.base_source, BaseSource::Reopen);
        assert!(has(&scope, "src/new.rs"));
    }

    /// A review recorded in the same commit as the reopen ran before it, so
    /// the work committed with that reopen stays in the window.
    #[test]
    fn a_record_in_the_transition_commit_leaves_the_start_at_the_reopen() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("done"));
        let parent = commit_all(&repo, "chore: seed");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &record("", "2026-11-01T00:00:00Z"),
        );
        write(&tmp.path().join("src/fix.rs"), "fn fix() {}\n");
        commit_all(&repo, "fix(001): reopen with its work");

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, parent);
        assert_eq!(scope.base_source, BaseSource::Reopen);
        assert!(has(&scope, "src/fix.rs"));
    }

    /// A run that stopped early, by any of its three signs, leaves what it did
    /// not read in the next window, and a record that does not parse names no
    /// run at all.
    #[test]
    fn a_run_that_stopped_early_or_does_not_parse_does_not_move_the_start() {
        for extra in [
            "skipped-passes: [quality]\n",
            "budget-stop: true\n",
            "fix-rounds-open: true\n",
            "skipped-passes: {not: a list}\n",
        ] {
            let (tmp, repo, parent) = two_specs(&[], &[]);
            commit_record(&tmp, &repo, extra, "2026-11-01T00:00:00Z");
            let scope = x(&tmp);
            assert_eq!(scope.diff_base, parent, "{extra}");
            assert_eq!(scope.base_source, BaseSource::Reopen, "{extra}");
        }
    }

    #[test]
    fn a_stopped_run_falls_back_to_the_last_finished_one() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        let finished = commit_record(&tmp, &repo, "", "2026-11-01T00:00:00Z");
        change(&tmp, &repo, "src/a.rs", "feat(001): between the runs");
        commit_record(&tmp, &repo, "budget-stop: true\n", "2026-11-02T00:00:00Z");

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, finished);
        assert!(has(&scope, "src/a.rs"));
    }

    #[test]
    fn an_uncommitted_record_does_not_move_the_start() {
        let (tmp, _repo, parent) = two_specs(&[], &[]);
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &record("", "2026-11-01T00:00:00Z"),
        );
        let scope = x(&tmp);
        assert_eq!(scope.diff_base, parent);
        assert_eq!(scope.base_source, BaseSource::Reopen);
    }

    /// A run's two writes carry one `last-run`. The one clearing the open mark
    /// finishes it, and the fixes committed between them leave the window.
    #[test]
    fn the_write_clearing_the_open_mark_finishes_the_run() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        let run_at = "2026-11-01T00:00:00Z";
        commit_record(&tmp, &repo, "fix-rounds-open: true\n", run_at);
        change(&tmp, &repo, "src/fix.rs", "fix(001): a round's fix");
        let closed = commit_record(&tmp, &repo, "", run_at);

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, closed);
        assert_eq!(scope.base_source, BaseSource::LastReview);
    }

    /// An upgrade moving a pre-057 record out of `spec.md` is no review, so what
    /// was committed between that run and the upgrade stays in the window.
    #[test]
    fn a_record_relocated_out_of_spec_md_does_not_move_the_start() {
        let (tmp, repo, parent) = two_specs(&[], &[]);
        let run_at = "2026-10-01T00:00:00Z";
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &format!(
                "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: {run_at}\n---\n\n# X\n"
            ),
        );
        commit_all(&repo, "review(001)");
        change(&tmp, &repo, "src/after.rs", "feat(001): after that run");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        commit_record(&tmp, &repo, "", run_at);

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, parent);
        assert_eq!(scope.base_source, BaseSource::Reopen);
        assert!(has(&scope, "src/after.rs"), "{:?}", scope.scope);
    }

    /// A run recorded in the shape a relocation leaves is still a run once its
    /// `last-run` is not the one `spec.md` stated.
    #[test]
    fn a_new_run_after_a_relocation_moves_the_start() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-10-01T00:00:00Z\n---\n\n# X\n",
        );
        commit_all(&repo, "review(001)");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        let review = commit_record(&tmp, &repo, "", "2026-10-02T00:00:00Z");

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, review);
        assert_eq!(scope.base_source, BaseSource::LastReview);
    }

    /// A merge carrying a finished run in is no review, so what was committed after
    /// that run on the branch stays in the window.
    #[test]
    fn a_merge_carrying_a_finished_run_does_not_move_the_start() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        let main_tip = repo.head().unwrap().peel_to_commit().unwrap();
        change(&tmp, &repo, "src/a.rs", "feat(001): the work");
        let review = commit_record(&tmp, &repo, "", "2026-10-01T00:00:00Z");
        change(&tmp, &repo, "src/b.rs", "fix(001): after the review");
        let branch_tip = repo.head().unwrap().peel_to_commit().unwrap();
        let signature = git2::Signature::now("Test", "test@example.com").unwrap();
        let merge = repo
            .commit(
                None,
                &signature,
                &signature,
                "Merge branch 'story'",
                &branch_tip.tree().unwrap(),
                &[&main_tip, &branch_tip],
            )
            .unwrap();
        repo.set_head_detached(merge).unwrap();

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, review);
        assert_eq!(scope.base_source, BaseSource::LastReview);
        assert!(has(&scope, "src/b.rs"), "{:?}", scope.scope);
    }

    /// A later commit carrying the same finished run, such as a re-commit of its
    /// record, finishes nothing, so the window still starts at the run's commit.
    #[test]
    fn a_later_commit_of_a_finished_run_does_not_move_the_start_again() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        let run_at = "2026-11-01T00:00:00Z";
        let finished = commit_record(&tmp, &repo, "", run_at);
        change(&tmp, &repo, "src/a.rs", "feat(001): after the run");
        commit_record(&tmp, &repo, "dispositions: {fixed: 1}\n", run_at);

        let scope = x(&tmp);
        assert_eq!(scope.diff_base, finished);
        assert!(has(&scope, "src/a.rs"));
    }

    #[test]
    fn another_specs_commit_leaves_the_scope_and_is_named_by_its_claimant() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        change(&tmp, &repo, "src/other.rs", "feat(002): its work");
        change(&tmp, &repo, "specs/002-y/tasks.md", "tasks(002): its tasks");
        // Claimed by its directory alone, with a subject naming no spec, so it
        // is no several-specs commit whose unplanned path could come here.
        write(&tmp.path().join("specs/002-y/notes.md"), "n\n");
        write(&tmp.path().join("src/by_directory.rs"), "fn d() {}\n");
        commit_all(&repo, "chore: touches only 002's directory");

        let scope = x(&tmp);
        assert!(!has(&scope, "src/other.rs"), "{:?}", scope.scope);
        assert!(!has(&scope, "specs/002-y/tasks.md"), "{:?}", scope.scope);
        assert_eq!(scope.excluded["src/other.rs"], ["002-y"]);
        assert_eq!(scope.excluded["specs/002-y/tasks.md"], ["002-y"]);
        assert_eq!(scope.excluded["src/by_directory.rs"], ["002-y"]);
        // `modified-since` keeps its meaning: every path changed since the base.
        assert!(scope.modified_since.contains(&"src/other.rs".to_string()));
    }

    #[test]
    fn a_claimant_no_longer_on_disk_is_named_by_its_number() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        write(&tmp.path().join("specs/003-gone/spec.md"), &spec("done"));
        write(&tmp.path().join("src/gone.rs"), "fn gone() {}\n");
        commit_all(&repo, "feat(003): its work");
        fs::remove_dir_all(tmp.path().join("specs/003-gone")).unwrap();

        let scope = x(&tmp);
        assert_eq!(scope.excluded["src/gone.rs"], ["003"]);
    }

    #[test]
    fn a_path_another_specs_commit_also_changed_stays_in_scope() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        change(&tmp, &repo, "src/shared.rs", "feat(002): its change");
        change(
            &tmp,
            &repo,
            "src/shared.rs",
            "feat(001): this spec's change",
        );

        let scope = x(&tmp);
        assert!(has(&scope, "src/shared.rs"));
        assert!(!scope.excluded.contains_key("src/shared.rs"));
    }

    #[test]
    fn an_unclaimed_commit_counts_whole_in_every_window() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        change(&tmp, &repo, "src/unclaimed.rs", "chore: lint cleanup");
        change(&tmp, &repo, "src/nine.rs", "fix(999): a number no spec has");

        for scope in [x(&tmp), y(&tmp)] {
            assert!(has(&scope, "src/unclaimed.rs"), "{:?}", scope.scope);
            assert!(has(&scope, "src/nine.rs"), "{:?}", scope.scope);
        }
    }

    #[test]
    fn an_unplanned_path_of_a_multi_spec_commit_goes_to_the_spec_named_first() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        change(&tmp, &repo, "src/first.rs", "feat(001, 002): both");
        change(&tmp, &repo, "src/second.rs", "feat(002, 001): both");

        let (scope_x, scope_y) = (x(&tmp), y(&tmp));
        assert!(has(&scope_x, "src/first.rs"), "{:?}", scope_x.scope);
        assert_eq!(scope_y.excluded["src/first.rs"], ["001-x"]);
        assert!(has(&scope_y, "src/second.rs"), "{:?}", scope_y.scope);
        assert_eq!(scope_x.excluded["src/second.rs"], ["002-y"]);
    }

    #[test]
    fn an_unplanned_path_of_a_multi_spec_commit_naming_none_goes_to_every_claimant() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        write(&tmp.path().join("specs/001-x/notes.md"), "x\n");
        write(&tmp.path().join("specs/002-y/notes.md"), "y\n");
        write(&tmp.path().join("src/unplanned.rs"), "fn u() {}\n");
        commit_all(&repo, "chore: touches both specs and names neither");

        for scope in [x(&tmp), y(&tmp)] {
            assert!(has(&scope, "src/unplanned.rs"), "{:?}", scope.scope);
            assert!(!scope.excluded.contains_key("src/unplanned.rs"));
        }
    }

    #[test]
    fn a_planned_path_of_a_multi_spec_commit_goes_to_the_plan_naming_it() {
        let (tmp, repo, _) = two_specs(&["src/x.rs"], &["src/y.rs"]);
        write(&tmp.path().join("src/x.rs"), "fn x() {}\n");
        write(&tmp.path().join("src/y.rs"), "fn y() {}\n");
        commit_all(&repo, "feat(001, 002): both");

        let (scope_x, scope_y) = (x(&tmp), y(&tmp));
        assert!(has(&scope_x, "src/x.rs"));
        // Named first, and still not this spec's: its sibling's plan names it.
        assert_eq!(scope_x.excluded["src/y.rs"], ["002-y"]);
        assert!(has(&scope_y, "src/y.rs"));
        assert_eq!(scope_y.excluded["src/x.rs"], ["001-x"]);
    }

    /// The spec's own directory is its own whatever the subject names first.
    #[test]
    fn a_multi_spec_commit_gives_each_spec_its_own_directory() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        write(&tmp.path().join("specs/001-x/tasks.md"), "x\n");
        write(&tmp.path().join("specs/002-y/tasks.md"), "y\n");
        commit_all(&repo, "tasks(002, 001): both");

        let scope = x(&tmp);
        assert!(has(&scope, "specs/001-x/tasks.md"), "{:?}", scope.scope);
        assert_eq!(scope.excluded["specs/002-y/tasks.md"], ["002-y"]);
    }

    #[test]
    fn since_moves_only_the_start_and_the_scope_rule_still_applies() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        let start = change(&tmp, &repo, "src/before.rs", "feat(001): before");
        change(&tmp, &repo, "src/other.rs", "feat(002): its work");
        change(&tmp, &repo, "src/mine.rs", "feat(001): this spec");

        let scope = compute(&args("001-x", Some(&start)), tmp.path()).unwrap();
        assert_eq!(scope.diff_base, start);
        assert_eq!(scope.base_source, BaseSource::Since);
        assert!(has(&scope, "src/mine.rs"));
        assert_eq!(scope.excluded["src/other.rs"], ["002-y"]);
        assert!(!has(&scope, "src/before.rs"));
    }

    /// `--since` naming a commit off to the side of `HEAD`: a path only that
    /// side changed differs from the base, and no walked commit names it, so it
    /// has no claimant to leave it to and stays in scope.
    #[test]
    fn a_changed_path_no_walked_commit_names_stays_in_scope() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        let main_tip = repo.head().unwrap().target().unwrap();
        let main_ref = repo.head().unwrap().name().unwrap().to_string();
        let side = change(&tmp, &repo, "src/old.rs", "feat(002): on a side branch");

        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.force().remove_untracked(true);
        repo.set_head_detached(main_tip).unwrap();
        repo.checkout_head(Some(&mut checkout)).unwrap();
        repo.reference(&main_ref, main_tip, true, "rewind").unwrap();
        repo.set_head(&main_ref).unwrap();

        let scope = compute(&args("001-x", Some(&side)), tmp.path()).unwrap();
        assert_eq!(scope.modified_since, ["src/old.rs"]);
        assert!(has(&scope, "src/old.rs"), "{:?}", scope.scope);
        assert!(scope.excluded.is_empty(), "{:?}", scope.excluded);
    }

    #[test]
    fn uncommitted_work_joins_whole_even_on_another_specs_path() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        change(&tmp, &repo, "src/other.rs", "feat(002): its work");
        write(&tmp.path().join("src/other.rs"), "fn edited() {}\n");
        write(&tmp.path().join("src/untracked.rs"), "fn new() {}\n");

        let scope = x(&tmp);
        assert!(has(&scope, "src/other.rs"), "{:?}", scope.scope);
        assert!(!scope.excluded.contains_key("src/other.rs"));
        assert!(has(&scope, "src/untracked.rs"), "{:?}", scope.scope);
    }

    #[test]
    fn an_unchanged_plan_entry_stays_only_on_the_first_review() {
        let (tmp, repo, _) = two_specs(&["src/planned.rs", "src/touched.rs"], &[]);
        assert!(has(&x(&tmp), "src/planned.rs"));

        commit_record(&tmp, &repo, "", "2026-11-01T00:00:00Z");
        change(&tmp, &repo, "src/touched.rs", "feat(001): the second pass");
        let scope = x(&tmp);
        assert!(!has(&scope, "src/planned.rs"), "{:?}", scope.scope);
        assert!(has(&scope, "src/touched.rs"), "{:?}", scope.scope);
        assert_eq!(
            scope.plan_affected,
            ["src/planned.rs", "src/touched.rs"],
            "plan-affected keeps its meaning"
        );
    }

    /// A record naming no run records no review, as the gate reads it.
    #[test]
    fn a_record_with_no_run_is_a_first_review_and_owes_one() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        write(
            &tmp.path().join("specs/001-x/plan.md"),
            "# Plan\n\n## Affected Files\n\n| File | Action |\n| --- | --- |\n\
             | `src/planned.rs` | Edit |\n",
        );
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &format!(
                "---\nspec: 001-x\nlast-run: null\nmust-violations: 0\nblocking: false\n{DISPOSITIONS}---\n"
            ),
        );
        commit_all(&repo, "chore: seed");

        let scope = x(&tmp);
        assert!(has(&scope, "src/planned.rs"), "{:?}", scope.scope);
        assert!(scope.review_owed);

        // With nothing outside the spec's directory, the record is all that
        // could say no review is owed, and it names no run.
        write(&tmp.path().join("specs/001-x/plan.md"), "# Plan\n");
        commit_all(&repo, "docs(001): a plan with no files");
        let scope = x(&tmp);
        assert_eq!(scope.scope, ["specs/001-x/plan.md"]);
        assert!(scope.review_owed);
    }

    /// A run's own uncommitted record does not make its second write a later
    /// review, or a run's two denominators would disagree.
    #[test]
    fn an_uncommitted_record_keeps_a_first_review_a_first_review() {
        let (tmp, _repo, _) = two_specs(&["src/planned.rs"], &[]);
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &record("", "2026-11-01T00:00:00Z"),
        );
        assert!(has(&x(&tmp), "src/planned.rs"));
    }

    #[test]
    fn a_plan_file_only_another_specs_commit_changed_moves_to_that_spec() {
        let (tmp, repo, _) = two_specs(&["src/planned.rs"], &[]);
        change(&tmp, &repo, "src/planned.rs", "feat(002): its change");

        let scope = x(&tmp);
        assert!(!has(&scope, "src/planned.rs"), "{:?}", scope.scope);
        assert_eq!(scope.excluded["src/planned.rs"], ["002-y"]);
    }

    const DISPOSITIONS: &str =
        "dispositions:\n  fixed: 0\n  deferred: 0\n  waived: 0\n  undispositioned: 0\n";

    /// A spec reopened by `edit`, with a clean review record from before the reopen.
    fn reopened_with_a_record(edit: &str) -> ReviewScope {
        reopened_with(DISPOSITIONS, edit)
    }

    /// A spec reopened by `edit`, with a review record carrying `fields`.
    fn reopened_with(fields: &str, edit: &str) -> ReviewScope {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("done"));
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &record(fields, "2026-10-01T00:00:00Z"),
        );
        commit_all(&repo, "chore: seed");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        write(&tmp.path().join(edit), "edited\n");
        commit_all(&repo, "docs(001): the reopening's edit");
        x(&tmp)
    }

    #[test]
    fn a_reopening_that_changed_only_text_owes_no_review() {
        for edit in ["specs/001-x/plan.md", "specs/001-x/tasks.md"] {
            assert!(!reopened_with_a_record(edit).review_owed, "{edit}");
        }
    }

    /// The gate holds these until a review runs, so calling none owed would leave
    /// `/review` stopping on what the gate asks for.
    #[test]
    fn a_record_the_gate_holds_owes_a_review_whatever_the_window_holds() {
        let held = [
            String::new(),
            format!("{DISPOSITIONS}fix-rounds-open: true\n"),
            format!("{DISPOSITIONS}not-re-read: 1\n"),
            format!("{DISPOSITIONS}should-violations: 1\n"),
        ];
        for fields in &held {
            assert!(
                reopened_with(fields, "specs/001-x/plan.md").review_owed,
                "{fields}"
            );
        }
    }

    /// A spec with a clean review record, reopened by a commit deleting `gone`.
    fn reopened_deleting(gone: &str) -> ReviewScope {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("done"));
        write(&tmp.path().join(gone), "there\n");
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &record(DISPOSITIONS, "2026-10-01T00:00:00Z"),
        );
        commit_all(&repo, "chore: seed");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        std::fs::remove_file(tmp.path().join(gone)).unwrap();
        commit_all(&repo, "fix(001): the reopening deletes it");
        x(&tmp)
    }

    #[test]
    fn a_reopening_deleting_code_or_a_contract_owes_a_review() {
        for gone in ["src/old.rs", "specs/001-x/scenarios/a.md"] {
            let scope = reopened_deleting(gone);
            assert_eq!(scope.deleted, [gone]);
            assert!(scope.review_owed, "{gone}");
        }
    }

    #[test]
    fn a_reopening_deleting_only_text_of_its_own_owes_no_review() {
        assert!(!reopened_deleting("specs/001-x/research.md").review_owed);
    }

    /// Another spec's deletion is that spec's change, as the scope's rule has it.
    #[test]
    fn another_specs_deletion_owes_this_spec_no_review() {
        let (tmp, repo, _) = two_specs(&[], &[]);
        write(&tmp.path().join("src/theirs.rs"), "theirs\n");
        commit_all(&repo, "chore: seed theirs");
        commit_record(&tmp, &repo, DISPOSITIONS, "2026-10-01T00:00:00Z");
        std::fs::remove_file(tmp.path().join("src/theirs.rs")).unwrap();
        write(&tmp.path().join("specs/002-y/notes.md"), "notes\n");
        commit_all(&repo, "fix(002): it deletes its file");
        let scope = x(&tmp);
        assert!(!scope.review_owed, "{:?}", scope.scope);
    }

    /// A contract changed before the flip lies outside the window, and the gate still
    /// reads the digest as stale, so a review is owed.
    #[test]
    fn a_stale_digest_owes_a_review_though_the_window_holds_no_contract() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("done"));
        write(&tmp.path().join("specs/001-x/scenarios/a.md"), "first\n");
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &record(
                &format!("{DISPOSITIONS}reviewed-digest:\n  scenarios/a.md: 0000\n"),
                "2026-10-01T00:00:00Z",
            ),
        );
        commit_all(&repo, "chore: seed");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repo, "status(001): reopen");
        let scope = x(&tmp);
        assert_eq!(scope.scope, ["specs/001-x/spec.md"]);
        assert!(scope.review_owed);
    }

    #[test]
    fn a_reopening_that_changed_code_a_scenario_or_the_data_model_owes_a_review() {
        for edit in [
            "src/code.rs",
            "specs/001-x/scenarios/a.md",
            "specs/001-x/data-model.md",
        ] {
            assert!(reopened_with_a_record(edit).review_owed, "{edit}");
        }
    }

    #[test]
    fn a_review_is_owed_with_no_record_whatever_the_window_holds() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("done"));
        commit_all(&repo, "chore: seed");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repo, "status(001): reopen");
        assert!(x(&tmp).review_owed);
    }

    #[test]
    fn a_project_in_a_subdirectory_excludes_by_project_relative_paths() {
        let tmp = tempfile::tempdir().unwrap();
        let (repo, project) = git_fixture::subdirectory_project(tmp.path());
        write(&project.join("specs/001-x/spec.md"), &spec("planned"));
        write(&project.join("specs/002-y/spec.md"), &spec("planned"));
        commit_all(&repo, "chore: seed");
        write(&project.join("specs/001-x/spec.md"), &spec("in-progress"));
        commit_all(&repo, "status(001): start");
        write(&project.join("specs/002-y/plan.md"), "plan\n");
        write(&project.join("src/theirs.rs"), "fn t() {}\n");
        write(&tmp.path().join("other/x.rs"), "fn x() {}\n");
        commit_all(&repo, "chore: touches 002 and the neighbour");

        let scope = compute(&args("001-x", None), &project).unwrap();
        assert_eq!(scope.excluded["specs/002-y/plan.md"], ["002-y"]);
        assert_eq!(scope.excluded["src/theirs.rs"], ["002-y"]);
        assert!(!scope.modified_since.iter().any(|p| p.contains("other/")));
    }

    #[test]
    fn a_plan_entry_names_a_path_as_the_corpus_writes_them() {
        let plan = PlanEntries::new(&[
            "src/exact.rs".to_string(),
            "runtime/tests/".to_string(),
            "framework/commands/*.md".to_string(),
            "docs/**".to_string(),
            "specs/065-x/{plan.md, tasks.md}".to_string(),
            "scripts/{a,b}-*.sh".to_string(),
            "file?.txt".to_string(),
            "src/{unclosed".to_string(),
        ]);
        for hit in [
            "src/exact.rs",
            "runtime/tests/deep/er.rs",
            "framework/commands/review.md",
            "docs/a/b.md",
            "specs/065-x/tasks.md",
            "scripts/b-lint.sh",
            "file1.txt",
            "src/{unclosed",
        ] {
            assert!(plan.names(hit), "{hit}");
        }
        for miss in [
            "src/exact.rs.bak",
            "src/exactXrs",
            "runtime/testsuite.rs",
            "framework/commands/sub/review.md",
            "specs/065-x/spec.md",
            "scripts/c-lint.sh",
            "file12.txt",
            "dir/file1.txt",
        ] {
            assert!(!plan.names(miss), "{miss}");
        }
    }
}
