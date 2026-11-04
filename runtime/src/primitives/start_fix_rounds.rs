//! `start-fix-rounds` — the base of a `/{project}:review` run's first fix
//! round (spec 078).
//!
//! Called once per run, when the passes end and before any fix: it clears the
//! feature's earlier rounds and snapshots the working tree, so the first
//! round's changed paths are exactly the fixes made after the passes.
//!
//! Defined by `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`.

use std::path::Path;

#[cfg(doc)]
use crate::primitives::PrimitiveError;
use crate::primitives::fix_rounds;
use crate::primitives::{ProjectRepository, Result};
use crate::schema::paths;
use crate::schema::primitives::{StartFixRoundsArgs, StartFixRoundsResult};

/// Execute the `start-fix-rounds` primitive.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] when `feature` is not a feature
/// directory name, [`PrimitiveError::InvalidPath`] when it would leave the
/// spec root or the round directory resolves outside the project,
/// [`PrimitiveError::FeatureNotFound`] when the feature directory is absent,
/// [`PrimitiveError::InvalidReReadsOff`] or [`PrimitiveError::Toml`] for a
/// malformed `[review] re-reads-off` or config file,
/// [`PrimitiveError::Io`] when the round directory cannot be cleared, and
/// [`PrimitiveError::Git`] for a project no work tree contains or any libgit2
/// failure. An error is never a base: the host records the run's first round
/// as not re-read.
pub fn run(args: &StartFixRoundsArgs, repo: &Path) -> Result<StartFixRoundsResult> {
    let layout = paths::Paths::load(repo);
    fix_rounds::validate_feature("start-fix-rounds", repo, &layout.specs_root, &args.feature)?;
    // Read before the run writes any fix, so a malformed switch halts the run
    // here rather than at the first round, after its fixes have landed.
    fix_rounds::re_reads_off(repo)?;
    // Discovered before anything is cleared, so a project outside any work
    // tree errors with its earlier rounds intact.
    let project = ProjectRepository::discover(repo)?;
    let dir = fix_rounds::feature_rounds_dir(repo, &args.feature)?;
    fix_rounds::clear_rounds(&dir)?;
    let (base, files) = fix_rounds::snapshot(&project)?;
    Ok(StartFixRoundsResult {
        base,
        files,
        budget: fix_rounds::RE_READ_BUDGET,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::collections::BTreeMap;
    use std::fs;

    use git2::{Oid, Repository};

    use super::*;
    use crate::primitives::PrimitiveError;
    use crate::primitives::git_fixture::{commit_all, write};

    fn args(feature: &str) -> StartFixRoundsArgs {
        StartFixRoundsArgs {
            feature: feature.to_string(),
        }
    }

    /// A committed repository holding a feature, two tracked files and an
    /// ignore rule.
    fn fixture() -> (tempfile::TempDir, Repository) {
        let tmp = tempfile::tempdir().unwrap();
        let repository = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# X\n",
        );
        write(&tmp.path().join("a.txt"), "a before\n");
        write(&tmp.path().join("b.txt"), "b\n");
        write(&tmp.path().join(".gitignore"), "ignored.txt\n");
        commit_all(&repository, "init");
        (tmp, repository)
    }

    /// Every path in `sha`'s tree, with its blob's text.
    fn tree_files(repository: &Repository, sha: &str) -> BTreeMap<String, String> {
        let commit = repository.find_commit(Oid::from_str(sha).unwrap()).unwrap();
        let mut files = BTreeMap::new();
        commit
            .tree()
            .unwrap()
            .walk(git2::TreeWalkMode::PreOrder, |dir, entry| {
                if entry.kind() == Some(git2::ObjectType::Blob) {
                    let blob = repository.find_blob(entry.id()).unwrap();
                    files.insert(
                        format!("{dir}{}", entry.name().unwrap()),
                        String::from_utf8_lossy(blob.content()).into_owned(),
                    );
                }
                git2::TreeWalkResult::Ok
            })
            .unwrap();
        files
    }

    /// Every ref and what it points at, `HEAD` included.
    fn refs(repository: &Repository) -> BTreeMap<String, Option<Oid>> {
        let mut all: BTreeMap<String, Option<Oid>> = repository
            .references()
            .unwrap()
            .map(|reference| {
                let reference = reference.unwrap();
                (reference.name().unwrap().to_string(), reference.target())
            })
            .collect();
        all.insert(
            "HEAD".to_string(),
            repository.head().ok().and_then(|head| head.target()),
        );
        all
    }

    /// The snapshot is the working tree as it is on disk: a modified file's
    /// new text, an untracked file, no deleted file, no ignored file, and
    /// nothing under the round directory.
    #[test]
    fn the_snapshot_is_the_working_tree() {
        let (tmp, repository) = fixture();
        write(&tmp.path().join("a.txt"), "a after\n");
        fs::remove_file(tmp.path().join("b.txt")).unwrap();
        write(&tmp.path().join("c.txt"), "c, untracked\n");
        write(&tmp.path().join("ignored.txt"), "ignored\n");

        let result = run(&args("001-x"), tmp.path()).unwrap();
        let files = tree_files(&repository, &result.base);

        assert_eq!(files.get("a.txt").map(String::as_str), Some("a after\n"));
        assert_eq!(
            files.get("c.txt").map(String::as_str),
            Some("c, untracked\n")
        );
        assert!(!files.contains_key("b.txt"), "{files:?}");
        assert!(!files.contains_key("ignored.txt"), "{files:?}");
        assert!(
            files.keys().all(|path| !path.starts_with(".ductus/")),
            "{files:?}"
        );
        assert_eq!(result.files, files.len());
        assert_eq!(result.budget, fix_rounds::RE_READ_BUDGET);
    }

    /// Taking a snapshot leaves the index file's bytes, `HEAD` and every ref
    /// as they were, and the snapshot's parent is `HEAD`.
    #[test]
    fn the_snapshot_moves_nothing() {
        let (tmp, repository) = fixture();
        write(&tmp.path().join("a.txt"), "a after\n");
        write(&tmp.path().join("c.txt"), "c\n");
        let index_path = tmp.path().join(".git/index");
        let index_before = fs::read(&index_path).unwrap();
        let refs_before = refs(&repository);

        let result = run(&args("001-x"), tmp.path()).unwrap();

        assert_eq!(fs::read(&index_path).unwrap(), index_before);
        assert_eq!(refs(&repository), refs_before);
        let commit = repository
            .find_commit(Oid::from_str(&result.base).unwrap())
            .unwrap();
        assert_eq!(
            commit.parent_ids().collect::<Vec<_>>(),
            vec![repository.head().unwrap().target().unwrap()]
        );
    }

    /// An unborn `HEAD` has no commit to be a parent, so the snapshot has none.
    #[test]
    fn an_unborn_head_gives_a_parentless_snapshot() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n",
        );
        let result = run(&args("001-x"), tmp.path()).unwrap();
        let commit = repository
            .find_commit(Oid::from_str(&result.base).unwrap())
            .unwrap();
        assert_eq!(commit.parent_count(), 0);
        assert!(tree_files(&repository, &result.base).contains_key("specs/001-x/spec.md"));
        assert!(repository.head().is_err(), "HEAD stayed unborn");
    }

    /// A run starts from no rounds: the feature's earlier round directories
    /// are removed, another feature's are kept, and the rounds directory
    /// ignores itself.
    #[test]
    fn earlier_rounds_of_the_feature_are_removed() {
        let (tmp, _repository) = fixture();
        let rounds = tmp.path().join(fix_rounds::ROUNDS_DIR);
        write(&rounds.join("001-x/abc/brief.md"), "old brief\n");
        write(&rounds.join("002-y/def/brief.md"), "another feature's\n");

        run(&args("001-x"), tmp.path()).unwrap();

        assert!(rounds.join("001-x").is_dir());
        assert_eq!(fs::read_dir(rounds.join("001-x")).unwrap().count(), 0);
        assert!(rounds.join("002-y/def/brief.md").is_file());
        assert_eq!(
            fs::read_to_string(rounds.join(".gitignore")).unwrap(),
            "*\n"
        );
    }

    /// An existing `.gitignore` in the rounds directory is left as it is.
    #[test]
    fn an_existing_ignore_file_is_kept() {
        let (tmp, _repository) = fixture();
        let ignore = tmp.path().join(fix_rounds::ROUNDS_DIR).join(".gitignore");
        write(&ignore, "*\n!keep.md\n");
        run(&args("001-x"), tmp.path()).unwrap();
        assert_eq!(fs::read_to_string(&ignore).unwrap(), "*\n!keep.md\n");
    }

    #[test]
    fn a_missing_feature_is_an_error() {
        let (tmp, _repository) = fixture();
        let err = run(&args("009-absent"), tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::FeatureNotFound { .. }),
            "{err:?}"
        );
    }

    /// `feature` is a feature directory name by the runtime's allowlist
    /// (`BE-INPUT-002`), refused before anything is cleared: `.` names the
    /// spec root, and the rounds directory itself, so it would empty every
    /// feature's rounds, and a nested path names rounds no feature owns.
    #[test]
    fn a_name_outside_the_allowlist_clears_nothing() {
        let (tmp, _repository) = fixture();
        write(&tmp.path().join("specs/001-x/scenarios/s.md"), "# S\n");
        let rounds = tmp.path().join(fix_rounds::ROUNDS_DIR);
        write(&rounds.join("002-y/def/brief.md"), "another feature's\n");
        for feature in [".", "001-x/scenarios"] {
            let err = run(&args(feature), tmp.path()).unwrap_err();
            assert!(
                matches!(err, PrimitiveError::InvalidArgument { .. }),
                "{feature}: {err:?}"
            );
        }
        assert!(rounds.join("002-y/def/brief.md").is_file());
    }

    /// A malformed `[review] re-reads-off` halts the run before its first
    /// fix, here, with the feature's earlier rounds intact, rather than at
    /// the first round after the fixes have landed.
    #[test]
    fn a_malformed_switch_halts_before_anything_is_cleared() {
        let (tmp, _repository) = fixture();
        let rounds = tmp.path().join(fix_rounds::ROUNDS_DIR);
        write(&rounds.join("001-x/abc/brief.md"), "old brief\n");
        write(
            &tmp.path().join(".ductus/config.toml"),
            "[review]\nre-reads-off = \"short\"\n",
        );
        let err = run(&args("001-x"), tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidReReadsOff { .. }),
            "{err:?}"
        );
        assert!(rounds.join("001-x/abc/brief.md").is_file());
    }
}
