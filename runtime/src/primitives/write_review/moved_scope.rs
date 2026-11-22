//! A review's scope must not move under it (066 scenario
//! `a-commit-after-the-scope-was-resolved-refuses-the-write`).
//!
//! `write-review` re-derives the scope at write time, so a commit that lands
//! after the run's `compute-review-scope` joins a scope no pass read. Since
//! spec 081 another spec's window takes each path of a commit naming no spec
//! as read once a committed record whose window, `diff-base` to
//! `reviewed-against`, holds it leaves the path out of `unread`. The review
//! therefore records as `reviewed-against` the `head` its scope was resolved
//! against, and this guard refuses the write while a commit after that sha
//! changed a path the write-time scope holds.

use std::collections::BTreeSet;
use std::path::Path;

use git2::{Oid, Sort};

use crate::primitives::commit_attribution::Attributor;
use crate::primitives::{PrimitiveError, ProjectRepository, Result, flatten_line};
use crate::schema::paths;

/// Refuse the write when `reviewed_against` names no commit, when `HEAD` does
/// not descend from it, or when a commit after it, up to `HEAD`, changed a
/// path `entries` holds. A commit after it that changed no scope path refuses
/// nothing: nothing the passes should have read moved.
///
/// Each moved path is named with its commit, since the remedy is the run's:
/// re-run `compute-review-scope`, read each path it adds or list it in
/// `unread`, and write again with its `head`.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] naming `reviewed-against` for each
/// refusal above, and [`PrimitiveError::Git`] when history cannot be read, so
/// an unread history never reads as one that did not move.
pub(super) fn refuse_a_moved_scope(
    reviewed_against: &str,
    repo: &Path,
    entries: &BTreeSet<String>,
) -> Result<()> {
    let project = ProjectRepository::discover(repo)?;
    let repository = &project.repository;
    let refuse = |reason: String| PrimitiveError::InvalidArgument {
        primitive: "write-review".into(),
        argument: "reviewed-against".into(),
        reason,
    };
    let against = repository
        .revparse_single(reviewed_against)
        .and_then(|object| object.peel_to_commit())
        .map_err(|_| {
            refuse(format!(
                "{} names no commit; pass the head compute-review-scope returned",
                flatten_line(reviewed_against)
            ))
        })?;
    let head = repository.head()?.peel_to_commit()?;
    if head.id() == against.id() {
        return Ok(());
    }
    if !repository.graph_descendant_of(head.id(), against.id())? {
        return Err(refuse(format!(
            "HEAD {} does not descend from {}; pass the head compute-review-scope returned",
            short(head.id()),
            short(against.id())
        )));
    }
    let specs_root = paths::Paths::load(repo)?.specs_root;
    let attributor = Attributor::new(&project, repo, &specs_root)?;
    let mut walk = repository.revwalk()?;
    walk.push(head.id())?;
    walk.hide(against.id())?;
    walk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;
    let mut moved = Vec::new();
    for oid in walk {
        let commit = repository.find_commit(oid?)?;
        let subject = flatten_line(commit.summary().ok().flatten().unwrap_or_default());
        for path in attributor
            .claims(&commit)?
            .paths
            .into_iter()
            .filter(|path| entries.contains(path))
        {
            moved.push(format!(
                "{} ({} {subject})",
                flatten_line(&path),
                short(commit.id())
            ));
        }
    }
    if moved.is_empty() {
        return Ok(());
    }
    Err(refuse(format!(
        "{} in this run's scope changed in commits after {}, which the passes did not \
         read: {}; re-run compute-review-scope, read each path it adds or list it in \
         unread, and write again with its head as reviewed-against",
        if moved.len() == 1 {
            "1 path".to_string()
        } else {
            format!("{} paths", moved.len())
        },
        short(against.id()),
        moved.join(", ")
    )))
}

/// The first eight hex digits of `oid`, as a commit is named in a message.
fn short(oid: Oid) -> String {
    oid.to_string().chars().take(8).collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{commit_all, write};
    use git2::Repository;

    /// A repository holding `specs/001-x` and `src/a.rs`; returns the root
    /// commit's sha, the scope's head.
    fn repo() -> (tempfile::TempDir, Repository, String) {
        let tmp = tempfile::tempdir().unwrap();
        let repository = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# X\n",
        );
        write(&tmp.path().join("src/a.rs"), "fn a() {}\n");
        let head = commit_all(&repository, "feat: start").to_string();
        (tmp, repository, head)
    }

    fn scope(paths: &[&str]) -> BTreeSet<String> {
        paths.iter().map(ToString::to_string).collect()
    }

    fn reason(err: &PrimitiveError) -> &str {
        match err {
            PrimitiveError::InvalidArgument {
                argument, reason, ..
            } => {
                assert_eq!(argument, "reviewed-against", "{err:?}");
                reason
            }
            other => panic!("expected an argument refusal, got {other:?}"),
        }
    }

    #[test]
    fn an_unmoved_head_writes() {
        let (tmp, _repository, head) = repo();
        refuse_a_moved_scope(&head, tmp.path(), &scope(&["src/a.rs"])).unwrap();
    }

    #[test]
    fn a_commit_after_the_head_that_changed_a_scope_path_refuses_naming_it() {
        let (tmp, repository, head) = repo();
        write(&tmp.path().join("src/a.rs"), "fn a() { 1; }\n");
        let landed = commit_all(&repository, "chore: tidy a").to_string();
        let err = refuse_a_moved_scope(&head, tmp.path(), &scope(&["src/a.rs"])).unwrap_err();
        let reason = reason(&err);
        assert!(reason.starts_with("1 path in this run's scope"), "{reason}");
        assert!(reason.contains("src/a.rs"), "{reason}");
        assert!(reason.contains(&landed[..8]), "{reason}");
        assert!(reason.contains("chore: tidy a"), "{reason}");
        assert!(reason.contains("re-run compute-review-scope"), "{reason}");
    }

    #[test]
    fn a_commit_after_the_head_that_changed_no_scope_path_writes() {
        let (tmp, repository, head) = repo();
        write(&tmp.path().join("specs/002-y/spec.md"), "# Y\n");
        commit_all(&repository, "feat(002): another spec");
        refuse_a_moved_scope(&head, tmp.path(), &scope(&["src/a.rs"])).unwrap();
    }

    #[test]
    fn a_path_the_scope_held_uncommitted_and_another_session_committed_refuses() {
        let (tmp, repository, head) = repo();
        // The passes read `src/b.rs` uncommitted; then it was committed.
        write(&tmp.path().join("src/b.rs"), "fn b() {}\n");
        commit_all(&repository, "feat: b");
        let err =
            refuse_a_moved_scope(&head, tmp.path(), &scope(&["src/a.rs", "src/b.rs"])).unwrap_err();
        assert!(reason(&err).contains("src/b.rs"), "{err:?}");
    }

    #[test]
    fn a_sha_naming_no_commit_is_an_argument_error_not_an_unmoved_scope() {
        let (tmp, _repository, _head) = repo();
        let err = refuse_a_moved_scope("abc1234", tmp.path(), &scope(&["src/a.rs"])).unwrap_err();
        assert!(reason(&err).contains("names no commit"), "{err:?}");
    }

    /// Through `write-review` itself: a commit naming no spec lands after the
    /// review resolved its scope, so the write refuses and writes no record,
    /// and the same write passing the new head goes through.
    #[test]
    fn write_review_refuses_a_write_whose_scope_a_later_commit_moved() {
        use crate::schema::primitives::WriteReviewArgs;
        let tmp = tempfile::tempdir().unwrap();
        let repository = Repository::init(tmp.path()).unwrap();
        let spec = tmp.path().join("specs/001-x/spec.md");
        write(
            &spec,
            "---\nstatus: planned\ndependencies: []\n---\n\n# X\n",
        );
        commit_all(&repository, "plan(001): x");
        write(
            &spec,
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# X\n",
        );
        let base = commit_all(&repository, "status(001): planned -> in-progress").to_string();
        write(&tmp.path().join("src/a.rs"), "fn a() {}\n");
        let head = commit_all(&repository, "feat(001): a").to_string();
        // A commit naming no spec lands mid-review.
        write(&tmp.path().join("src/c.rs"), "fn c() {}\n");
        let landed = commit_all(&repository, "chore: c").to_string();
        let args = |against: &str| WriteReviewArgs {
            feature: "001-x".into(),
            reviewed_at: "2026-11-22T00:00:00Z".into(),
            reviewed_against: against.into(),
            diff_base: base.clone(),
            examined: Some(1),
            unread: Some(Vec::new()),
            ..WriteReviewArgs::default()
        };
        let err = crate::primitives::write_review::run(&args(&head), tmp.path()).unwrap_err();
        assert!(reason(&err).contains("src/c.rs"), "{err:?}");
        assert!(
            !tmp.path().join("specs/001-x/review.md").exists(),
            "a refused write wrote a record"
        );
        crate::primitives::write_review::run(&args(&landed), tmp.path()).unwrap();
        assert!(tmp.path().join("specs/001-x/review.md").exists());
    }

    #[test]
    fn a_head_that_does_not_descend_from_it_is_an_argument_error() {
        let (tmp, repository, root) = repo();
        // A commit on a branch HEAD never reaches.
        let base = repository
            .find_commit(Oid::from_str(&root).unwrap())
            .unwrap();
        let tree = base.tree().unwrap();
        let sig = git2::Signature::now("t", "t@example.com").unwrap();
        let side = repository
            .commit(None, &sig, &sig, "side", &tree, &[&base])
            .unwrap();
        write(&tmp.path().join("src/a.rs"), "fn a() { 2; }\n");
        commit_all(&repository, "feat: main moves on");
        let err =
            refuse_a_moved_scope(&side.to_string(), tmp.path(), &scope(&["src/a.rs"])).unwrap_err();
        assert!(reason(&err).contains("does not descend from"), "{err:?}");
    }
}
