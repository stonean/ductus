//! A review's scope must not move under it (066 scenario
//! `a-commit-after-the-scope-was-resolved-refuses-the-write`).
//!
//! `write-review` re-derives the scope at write time, so a commit that lands
//! after the run's `compute-review-scope` joins a scope no pass read. The review
//! therefore records as `reviewed-against` the `head` its scope was resolved
//! against, and this guard refuses the write while a commit after that sha
//! changed a path the write-time scope holds.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
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
/// The refusal's size is bounded whatever moved (`BE-PAYLOAD-001`): it counts
/// the moved paths, each once however many commits changed it, names the first
/// [`NAMED_PATHS`], each with up to [`NAMED_COMMITS`] of its commits and their
/// subjects cut at [`SUBJECT_CHARS`] characters, says how many more, and gives
/// the `git log --relative --name-only` command that lists each path every
/// later commit changed, project-relative, a superset of the moved paths it counts. The remedy
/// is the run's: re-run `compute-review-scope`, read each moved path and each
/// path it adds, or list it in `unread`, and write again with its `head`. A
/// moved path is usually one the scope already held, which the re-run does not
/// add, so reading only the added paths would credit content no pass read.
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
    // Each moved path once, in the order a commit first moved it, with the
    // first commits that moved it, as many as a refusal names, and how many
    // moved it in all.
    let mut moved: Vec<(String, Vec<String>, usize)> = Vec::new();
    let mut position: BTreeMap<String, usize> = BTreeMap::new();
    for oid in walk {
        let commit = repository.find_commit(oid?)?;
        let named = format!(
            "{} {}",
            short(commit.id()),
            cut(&flatten_line(
                commit.summary().ok().flatten().unwrap_or_default()
            ))
        );
        for path in attributor
            .claims(&commit)?
            .paths
            .into_iter()
            .filter(|path| entries.contains(path))
        {
            let at = *position.entry(path.clone()).or_insert_with(|| {
                moved.push((path, Vec::new(), 0));
                moved.len() - 1
            });
            let (_, commits, total) = &mut moved[at];
            if commits.len() < NAMED_COMMITS {
                commits.push(named.clone());
            }
            *total += 1;
        }
    }
    if moved.is_empty() {
        return Ok(());
    }
    let named = moved
        .iter()
        .take(NAMED_PATHS)
        .map(|(path, commits, total)| {
            let mut shown = commits.join("; ");
            if *total > NAMED_COMMITS {
                let _ = write!(shown, "; {} more", total - NAMED_COMMITS);
            }
            format!("{} ({shown})", flatten_line(path))
        })
        .collect::<Vec<_>>()
        .join(", ");
    let more = if moved.len() > NAMED_PATHS {
        format!(", and {} more", moved.len() - NAMED_PATHS)
    } else {
        String::new()
    };
    Err(refuse(format!(
        "{} in this run's scope changed in commits after {}, which the passes did not \
         read: {named}{more}; `git log --relative --name-only --format= {}..HEAD` lists \
         each path every later commit changed, read against the scope; re-run \
         compute-review-scope, read each moved path and \
         each path it adds, or list it in unread, and write again with its head as \
         reviewed-against",
        if moved.len() == 1 {
            "1 path".to_string()
        } else {
            format!("{} paths", moved.len())
        },
        short(against.id()),
        against.id()
    )))
}

/// How many moved paths a refusal names; it counts the rest (`BE-PAYLOAD-001`).
const NAMED_PATHS: usize = 3;

/// How many of a moved path's commits a refusal names; it counts the rest.
const NAMED_COMMITS: usize = 3;

/// The characters of a commit subject a refusal keeps.
const SUBJECT_CHARS: usize = 72;

/// `subject` cut to [`SUBJECT_CHARS`] characters, with an ellipsis when cut.
fn cut(subject: &str) -> String {
    if subject.chars().count() <= SUBJECT_CHARS {
        return subject.to_string();
    }
    let mut kept: String = subject.chars().take(SUBJECT_CHARS).collect();
    kept.push('…');
    kept
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
        assert!(reason.contains("read each moved path"), "{reason}");
    }

    /// Four moved paths are counted, three named and the rest counted, with
    /// the command that lists them all, so the refusal's size does not grow
    /// with what moved (`BE-PAYLOAD-001`).
    #[test]
    fn a_refusal_names_three_moved_paths_and_counts_the_rest() {
        let (tmp, repository, head) = repo();
        let paths = ["src/a.rs", "src/b.rs", "src/c.rs", "src/d.rs"];
        for path in paths {
            write(&tmp.path().join(path), "fn x() { 3; }\n");
        }
        commit_all(&repository, "chore: a sweep");
        let err = refuse_a_moved_scope(&head, tmp.path(), &scope(&paths)).unwrap_err();
        let reason = reason(&err);
        assert!(
            reason.starts_with("4 paths in this run's scope"),
            "{reason}"
        );
        let named = paths.iter().filter(|path| reason.contains(*path)).count();
        assert_eq!(named, 3, "{reason}");
        assert!(reason.contains(", and 1 more;"), "{reason}");
        assert!(
            reason.contains(&format!(
                "`git log --relative --name-only --format= {head}..HEAD`"
            )),
            "{reason}"
        );
    }

    /// A path four later commits changed is named with three of them and a
    /// count of the fourth.
    #[test]
    fn a_moved_path_names_three_of_its_commits_and_counts_the_rest() {
        let (tmp, repository, head) = repo();
        let commits: Vec<String> = (1..=4)
            .map(|n| {
                write(
                    &tmp.path().join("src/a.rs"),
                    &format!("fn a() {{ {n}; }}\n"),
                );
                commit_all(&repository, &format!("chore: tidy a {n}")).to_string()
            })
            .collect();
        let err = refuse_a_moved_scope(&head, tmp.path(), &scope(&["src/a.rs"])).unwrap_err();
        let reason = reason(&err);
        assert!(reason.starts_with("1 path in this run's scope"), "{reason}");
        for named in &commits[..3] {
            assert!(reason.contains(&named[..8]), "{reason}");
        }
        assert!(!reason.contains(&commits[3][..8]), "{reason}");
        assert!(reason.contains("; 1 more)"), "{reason}");
    }

    /// A commit subject past 72 characters is cut, so one long subject does
    /// not lengthen the refusal without bound.
    #[test]
    fn a_long_commit_subject_is_cut() {
        let (tmp, repository, head) = repo();
        write(&tmp.path().join("src/a.rs"), "fn a() { 5; }\n");
        let kept = "k".repeat(72);
        commit_all(&repository, &format!("{kept}{}", "x".repeat(40)));
        let err = refuse_a_moved_scope(&head, tmp.path(), &scope(&["src/a.rs"])).unwrap_err();
        let reason = reason(&err);
        assert!(reason.contains(&format!("{kept}…")), "{reason}");
        assert!(!reason.contains(&format!("{kept}x")), "{reason}");
    }

    /// At each bound nothing is counted as more: three moved paths are all
    /// named, a path three commits changed is named with all three, and a
    /// 72-character subject is kept whole.
    #[test]
    fn at_each_bound_nothing_is_counted_as_more() {
        let (tmp, repository, head) = repo();
        let paths = ["src/a.rs", "src/b.rs", "src/c.rs"];
        let subject = "s".repeat(72);
        for n in 1..=3 {
            for path in paths {
                write(&tmp.path().join(path), &format!("fn x() {{ {n}; }}\n"));
            }
            commit_all(&repository, &subject);
        }
        let err = refuse_a_moved_scope(&head, tmp.path(), &scope(&paths)).unwrap_err();
        let reason = reason(&err);
        assert!(
            reason.starts_with("3 paths in this run's scope"),
            "{reason}"
        );
        assert!(!reason.contains("more"), "{reason}");
        assert!(!reason.contains('…'), "{reason}");
        assert!(reason.contains(&subject), "{reason}");
    }

    /// One scope path two later commits changed is one path, named with both
    /// commits.
    #[test]
    fn a_path_two_later_commits_changed_counts_once_naming_both() {
        let (tmp, repository, head) = repo();
        write(&tmp.path().join("src/a.rs"), "fn a() { 1; }\n");
        let first = commit_all(&repository, "chore: tidy a").to_string();
        write(&tmp.path().join("src/a.rs"), "fn a() { 2; }\n");
        let second = commit_all(&repository, "chore: tidy a again").to_string();
        let err = refuse_a_moved_scope(&head, tmp.path(), &scope(&["src/a.rs"])).unwrap_err();
        let reason = reason(&err);
        assert!(reason.starts_with("1 path in this run's scope"), "{reason}");
        assert!(reason.contains(&first[..8]), "{reason}");
        assert!(reason.contains(&second[..8]), "{reason}");
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

    /// Through `write-review` itself: a commit of the spec's own lands after the
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
        // Another session's commit for this spec lands mid-review.
        write(&tmp.path().join("src/c.rs"), "fn c() {}\n");
        let landed = commit_all(&repository, "feat(001): c").to_string();
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
