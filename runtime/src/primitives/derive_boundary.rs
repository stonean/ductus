//! `derive-boundary` — compute the runtime write boundary from
//! `git diff --name-only <first-commit-on-spec-dir>..HEAD` plus the spec dir.
//!
//! The boundary is emitted as **directory-zone globs**, not exact changed
//! paths (scenario writecode-boundary-derivation): each changed path
//! contributes its parent directory as `{dir}/**`, because the writeCode
//! validator that enforces this boundary must admit *new* files — and a
//! new file can never exact-match a previously-changed path. A root-level
//! changed file stays an exact path (its "zone glob" would be `**`,
//! permitting everything). The spec dir's own `{root}/{feature}/**` glob
//! is always included.

use std::collections::BTreeSet;
use std::path::Path;

use git2::{Repository, Sort};

use crate::primitives::{PrimitiveError, Result};
use crate::schema::paths;
use crate::schema::primitives::{DeriveBoundaryArgs, DeriveBoundaryResult};

/// Execute the `derive-boundary` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::FeatureNotFound`] when the feature directory
/// is absent and [`PrimitiveError::Git`] for any libgit2 failure.
///
/// A spec dir with no commit touching it is **not** an error (scenario
/// derive-boundary-uncommitted-spec-dir): the boundary is unknowable, not
/// broken, so the primitive returns the spec-dir glob alone plus
/// [`uncommitted_guidance`]. Enforcement stays fail-closed on that result —
/// the first out-of-spec edit halts with `out-of-boundary-edit` — while the
/// walk itself reaches a legible next-step instead of dying at step 2.
pub fn run(args: &DeriveBoundaryArgs, repo: &Path) -> Result<DeriveBoundaryResult> {
    super::validate_no_traversal(&args.feature)?;
    let layout = paths::Paths::load(repo);
    let feature_dir = repo.join(&layout.specs_root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: args.feature.clone(),
        });
    }
    let repository = Repository::discover(repo)?;
    let spec_prefix = format!("{}/{}/", layout.specs_root, args.feature);
    // The spec-dir glob needs no history — it is the feature's own zone —
    // so it is the whole boundary in the no-history case below.
    let spec_glob = format!("{}/{}/**", layout.specs_root, args.feature);

    // `zip` collapses two independent no-history shapes into one arm: a
    // spec dir with no commit touching it, and an unborn HEAD (a repo with
    // no commits at all — the fresh-repo case where `/ductus:specify` and
    // `/ductus:plan` both run before the first commit). `head` is resolved
    // once so the no-history arm can still report it when it exists.
    let head = head_oid(&repository)?;
    let Some((first_commit, head_oid)) =
        first_commit_for_prefix(&repository, &spec_prefix)?.zip(head)
    else {
        return Ok(DeriveBoundaryResult {
            boundary: vec![spec_glob],
            first_commit: String::new(),
            current_head: head.map(|oid| oid.to_string()).unwrap_or_default(),
            guidance: Some(uncommitted_guidance(&layout.specs_root, &args.feature)),
        });
    };

    let first = repository.find_commit(first_commit)?;
    let first_tree = first.tree()?;
    let head_tree = repository.find_commit(head_oid)?.tree()?;
    let diff = repository.diff_tree_to_tree(Some(&first_tree), Some(&head_tree), None)?;

    let mut boundary: BTreeSet<String> = BTreeSet::new();
    // Always include the spec dir glob so the boundary covers files inside
    // the feature's own folder even when they aren't on disk yet.
    boundary.insert(spec_glob);

    diff.foreach(
        &mut |delta, _| {
            for path in [delta.old_file().path(), delta.new_file().path()]
                .into_iter()
                .flatten()
            {
                let s = path.to_string_lossy().replace('\\', "/");
                if s.starts_with(&spec_prefix) {
                    continue;
                }
                boundary.insert(zone_glob(&s));
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(DeriveBoundaryResult {
        boundary: boundary.into_iter().collect(),
        first_commit: first_commit.to_string(),
        current_head: head_oid.to_string(),
        guidance: None,
    })
}

/// Next-step text for a spec dir with no commit touching it. Names both
/// escapes so the operator does not have to rediscover the derivation rule:
/// commit the directory (the fix `/{project}:plan`'s gate wants), or seed a
/// `write-boundary` in the session (the deliberate grant the walker unions
/// with the derivation).
fn uncommitted_guidance(root: &str, feature: &str) -> String {
    format!(
        "no commit touches {root}/{feature} yet, so the write boundary cannot be derived \
         from history — commit the spec directory, or seed a write-boundary in the session. \
         Enforcement stays fail-closed until then: only edits inside {root}/{feature}/ are permitted."
    )
}

/// `HEAD`'s commit id, or `None` on an unborn HEAD (a repo with no commits).
/// libgit2 reports the unborn case as an error, but for this primitive it is
/// an ordinary state — a brand-new repo whose spec dir was never committed.
fn head_oid(repo: &Repository) -> Result<Option<git2::Oid>> {
    // `is_empty` is the reliable unborn probe. libgit2 reports a missing
    // HEAD ref through more than one error code (observed: a generic
    // reference error, not `UnbornBranch`), so matching on the code alone
    // is brittle across versions.
    if repo.is_empty()? {
        return Ok(None);
    }
    Ok(Some(repo.head()?.peel_to_commit()?.id()))
}

/// The boundary entry a changed path contributes: its parent directory as
/// a `{dir}/**` zone glob, so writeCode may create new files in
/// directories the feature already touched. A root-level path stays exact
/// — its zone would be `**`, which permits everything.
fn zone_glob(path: &str) -> String {
    match path.rsplit_once('/') {
        Some((dir, _file)) => format!("{dir}/**"),
        None => path.to_string(),
    }
}

/// Earliest commit (topological, from the root) whose first-parent diff
/// touches a path under `prefix` — the feature's spec-history diff base.
/// `pub(crate)`: shared with `diff-cross-spec`, whose window must start
/// at exactly the commit the boundary derivation starts at.
pub(crate) fn first_commit_for_prefix(
    repo: &Repository,
    prefix: &str,
) -> Result<Option<git2::Oid>> {
    // An unborn HEAD (repo with no commits) has nothing to walk — that is
    // "no history", the same outcome as a walk that finds no touching
    // commit, not a failure. Callers treat both as the uncommitted case.
    if repo.is_empty()? {
        return Ok(None);
    }
    let mut walk = repo.revwalk()?;
    walk.push_head()?;
    walk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

    for oid in walk {
        let oid = oid?;
        if commit_touches_prefix(repo, oid, prefix)? {
            return Ok(Some(oid));
        }
    }
    Ok(None)
}

fn commit_touches_prefix(repo: &Repository, oid: git2::Oid, prefix: &str) -> Result<bool> {
    let commit = repo.find_commit(oid)?;
    let tree = commit.tree()?;
    let parent_tree = if commit.parent_count() == 0 {
        None
    } else {
        Some(commit.parent(0)?.tree()?)
    };
    let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
    let mut touched = false;
    diff.foreach(
        &mut |delta, _| {
            for path in [delta.old_file().path(), delta.new_file().path()]
                .into_iter()
                .flatten()
            {
                if path.to_string_lossy().starts_with(prefix) {
                    touched = true;
                }
            }
            true
        },
        None,
        None,
        None,
    )?;
    Ok(touched)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use git2::{IndexAddOption, Repository, Signature};
    use std::fs;
    use std::path::Path;

    fn commit_all(repo: &Repository, message: &str) -> git2::Oid {
        let mut index = repo.index().unwrap();
        index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let sig = Signature::now("Test", "test@example.com").unwrap();
        let parent = repo
            .head()
            .ok()
            .and_then(|h| h.target())
            .and_then(|oid| repo.find_commit(oid).ok());
        let parents: Vec<&git2::Commit> = parent.as_ref().into_iter().collect();
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
            .unwrap()
    }

    fn write(path: &Path, body: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
    }

    #[test]
    fn boundary_uses_configured_specs_root() {
        // Spec 040: a repo that renames its spec root to `governance` derives a
        // boundary glob and git-relative paths under `governance/`, never `specs/`.
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join(".govern.toml"),
            "[paths]\nspecs-root = \"governance\"\n",
        );
        write(&tmp.path().join("README.md"), "# repo\n");
        commit_all(&repo, "chore: init");
        write(
            &tmp.path().join("governance/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");
        write(&tmp.path().join("runtime/src/main.rs"), "fn main() {}\n");
        commit_all(&repo, "feat(020): runtime");

        let result = run(
            &DeriveBoundaryArgs {
                feature: "020-demo".into(),
            },
            tmp.path(),
        )
        .unwrap();
        let boundary: std::collections::HashSet<&str> =
            result.boundary.iter().map(String::as_str).collect();
        assert!(
            boundary.contains("governance/020-demo/**"),
            "boundary glob under configured root: {boundary:?}"
        );
        assert!(
            !boundary.iter().any(|b| b.starts_with("specs/")),
            "no specs/ paths in boundary: {boundary:?}"
        );
        assert!(boundary.contains("runtime/src/**"));
    }

    #[test]
    fn boundary_covers_touched_files_outside_spec_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        // Initial unrelated commit.
        write(&tmp.path().join("README.md"), "# repo\n");
        commit_all(&repo, "chore: init");
        // First spec-touching commit creates the feature.
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");
        // Subsequent commit touches files outside the spec dir.
        write(&tmp.path().join("runtime/src/main.rs"), "fn main() {}\n");
        write(&tmp.path().join("README.md"), "# repo v2\n");
        commit_all(&repo, "feat(020): runtime bootstrap");

        let result = run(
            &DeriveBoundaryArgs {
                feature: "020-demo".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.first_commit.is_empty());
        assert!(!result.current_head.is_empty());
        let boundary: std::collections::HashSet<&str> =
            result.boundary.iter().map(String::as_str).collect();
        assert!(boundary.contains("specs/020-demo/**"));
        assert!(
            boundary.contains("runtime/src/**"),
            "changed path contributes its directory zone: {boundary:?}"
        );
        assert!(
            boundary.contains("README.md"),
            "root-level file stays exact (its zone would be `**`): {boundary:?}"
        );
        assert!(
            !boundary.contains("runtime/src/main.rs"),
            "exact non-root paths are subsumed by their zone glob: {boundary:?}"
        );
    }

    #[test]
    fn files_in_one_directory_collapse_to_one_zone() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");
        write(&tmp.path().join("runtime/src/a.rs"), "fn a() {}\n");
        write(&tmp.path().join("runtime/src/b.rs"), "fn b() {}\n");
        write(&tmp.path().join("scripts/gen.sh"), "#!/bin/sh\n");
        commit_all(&repo, "feat(020): work");

        let result = run(
            &DeriveBoundaryArgs {
                feature: "020-demo".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            result.boundary,
            vec![
                "runtime/src/**".to_string(),
                "scripts/**".to_string(),
                "specs/020-demo/**".to_string(),
            ],
            "one zone per touched directory, sorted"
        );
    }

    #[test]
    fn uncommitted_spec_dir_is_a_domain_outcome_not_an_error() {
        // Scenario derive-boundary-uncommitted-spec-dir: the boundary is
        // unknowable, not broken. The walk must reach a legible next-step
        // instead of dying at /ductus:implement step 2.
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("README.md"), "# repo\n");
        commit_all(&repo, "chore: init");
        write(
            &tmp.path().join("specs/030-orphan/spec.md"),
            "---\nstatus: planned\n---\n\n# 030\n",
        );
        // No commit yet — the feature dir exists on disk but never landed in
        // git history.
        let result = run(
            &DeriveBoundaryArgs {
                feature: "030-orphan".into(),
            },
            tmp.path(),
        )
        .expect("uncommitted spec dir is not an error");

        // Fail-closed: the feature's own zone and nothing else, so the first
        // out-of-spec writeCode edit still halts with out-of-boundary-edit.
        assert_eq!(result.boundary, vec!["specs/030-orphan/**".to_string()]);
        assert_eq!(result.first_commit, "", "no history to point at");
        assert!(
            !result.current_head.is_empty(),
            "HEAD exists and is reported"
        );

        let guidance = result.guidance.expect("guidance names the next step");
        assert!(guidance.contains("specs/030-orphan"));
        assert!(
            guidance.contains("commit the spec directory"),
            "names the fix /ductus:plan's gate wants: {guidance}"
        );
        assert!(
            guidance.contains("seed a write-boundary"),
            "names the session escape hatch: {guidance}"
        );
    }

    #[test]
    fn unborn_head_is_the_same_domain_outcome() {
        // The fresh-repo case: /ductus:init, /ductus:specify and /ductus:plan all run
        // before the first commit exists, so HEAD is unborn. libgit2 reports
        // that as an error; for this primitive it is ordinary no-history.
        let tmp = tempfile::tempdir().unwrap();
        Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/030-orphan/spec.md"),
            "---\nstatus: planned\n---\n\n# 030\n",
        );

        let result = run(
            &DeriveBoundaryArgs {
                feature: "030-orphan".into(),
            },
            tmp.path(),
        )
        .expect("unborn HEAD is not an error");

        assert_eq!(result.boundary, vec!["specs/030-orphan/**".to_string()]);
        assert_eq!(result.first_commit, "");
        assert_eq!(result.current_head, "", "no HEAD to report");
        assert!(result.guidance.is_some());
    }

    #[test]
    fn committed_spec_dir_carries_no_guidance() {
        // The guidance field is the signal /ductus:plan's gate refuses on, so
        // it must be absent on every ordinary derivation.
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");

        let result = run(
            &DeriveBoundaryArgs {
                feature: "020-demo".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(result.guidance.is_none());
        assert!(!result.first_commit.is_empty());
    }
}
