//! The code no spec claims that no project review has read (spec 084, AC6):
//! where the project record's window starts, and the code committed after it.
//!
//! Everything is read from a commit and its history, never from the disk: a
//! push carries commits, so uncommitted work no spec claims stays with the open
//! specs' reviews (`compute_review_scope::split_changes`). `write-review`'s
//! project scope reads both functions.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use git2::{Commit, Oid, Repository, Sort, Tree};

use crate::primitives::commit_attribution::{Attributor, CommitClaims, project_share};
use crate::primitives::compute_review_scope::{
    ReviewEnd, WindowCommits, last_review_commit, record_at,
};
use crate::primitives::review_subject::ReviewSubject;
use crate::primitives::{PrimitiveError, ProjectRepository, Result, is_project_artifact};
use crate::schema::paths;

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
    /// The walk's commit counts.
    pub(crate) commits: WindowCommits,
}

/// The code no spec claims that the commits after `start`, up to `at`, changed.
///
/// A commit belongs to the project by [`project_share`], the rule a review's
/// scope leaves it out of an open spec by, and the specs that may claim one are
/// those `at`'s tree holds. Code is every path that is not one of the project's
/// artifacts (`is_project_artifact`), so the record itself is never among them.
/// A path `at`'s tree no longer holds has nothing to read and is left out.
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
    let counts = WindowCommits::of(&commits);
    let paths = commits
        .into_iter()
        .flat_map(|(_, claims)| project_share(claims).0)
        .filter(|path| !is_project_artifact(path, specs_root))
        .filter(|path| holds(project, &tree, path))
        .collect();
    Ok(UnclaimedCode {
        paths,
        commits: counts,
    })
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
    let repository = &project.repository;
    let head = repository.head()?.peel_to_commit()?;
    let window = match since {
        Some(reference) => ProjectWindow {
            start: repository
                .revparse_single(reference)?
                .peel_to_commit()?
                .id(),
            carried: Vec::new(),
        },
        None => match project_window(&project, &specs_root, &head)? {
            Ok(window) => window,
            Err(not_placed) => return Ok(Err(not_placed)),
        },
    };
    let code = unclaimed_code(&project, repo, &specs_root, window.start, &head)?;
    let tree = head.tree()?;
    let record = project.to_git(&ReviewSubject::Project.record_rel(&specs_root));
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
        .filter(|path| holds(&project, &tree, path));
    let mut scope = code.paths;
    scope.extend(handed_on);
    Ok(Ok(ProjectScope {
        diff_base: window.start.to_string(),
        head: head.id().to_string(),
        scope,
        commits: code.commits,
    }))
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
}
