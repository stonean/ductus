//! `diff-cross-spec` — the cross-spec impact surface for `/ductus:implement`.
//!
//! The deterministic filter implement steps 7 and 13 previously re-derived
//! by hand per task (the gate step's prose self-declared "no primitive owns
//! this filter yet"; spec 022, scenario coverage-expansion-primitives): the
//! diff from the feature's first spec-dir commit — the same base
//! `derive-boundary` computes, through the shared
//! [`first_commit_for_prefix`] walk — scoped to the spec root and filtered
//! to paths outside the feature's own directory.
//!
//! The diff runs against the working tree (index and untracked files
//! included), not `HEAD`: the per-task summary (step 7) fires before the
//! task's commit, when any sibling-spec edits are still uncommitted. On a
//! clean tree the result equals the documented markdown-only form, `git diff
//! --name-only --relative <first-commit>..HEAD -- {specs-root}/` run from the
//! project root (step 13), which names each path from the project root as
//! this primitive does. Read-only.
//!
//! `{specs-root}/inbox.md` is excluded from the result. It holds only what a
//! person logs (spec 058), so editing it is not a cross-spec impact, and no
//! inbox window or standing count is reported here any more — the standing
//! row moved to `dashboard`.

use std::collections::BTreeSet;
use std::path::Path;

use git2::DiffOptions;

use crate::primitives::derive_boundary::first_commit_for_prefix;
use crate::primitives::{PrimitiveError, ProjectRepository, Result};
use crate::schema::paths;
use crate::schema::primitives::{DiffCrossSpecArgs, DiffCrossSpecResult};

/// Execute the `diff-cross-spec` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` would leave the spec
/// root, or when the feature directory, or a link inside it, resolves outside
/// the project, [`PrimitiveError::FeatureNotFound`] when the feature directory
/// is absent, and [`PrimitiveError::Git`] for any libgit2 failure, including
/// a project no work tree contains.
///
/// A spec dir with no commit touching it yields the empty result rather
/// than an error (scenario derive-boundary-uncommitted-spec-dir): there is
/// no window to diff, so there is nothing to report. Erroring here would
/// only relocate the halt `derive-boundary` no longer raises — the same
/// `/{project}:implement` walk hits this primitive at steps 7 and 13.
pub fn run(args: &DiffCrossSpecArgs, repo: &Path) -> Result<DiffCrossSpecResult> {
    let layout = paths::Paths::load(repo);
    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;
    let feature_dir = repo.join(&layout.specs_root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: args.feature.clone(),
        });
    }
    // A diff and history name every path from the git work tree, which is
    // not the project root when the project sits in a subdirectory of its
    // repository. Each path is converted to the project's name before it is
    // matched, as `derive-boundary` does, so another project's work never
    // reaches the filter.
    let project = ProjectRepository::discover(repo)?;
    let repository = &project.repository;
    let root_prefix = format!("{}/", layout.specs_root);
    let spec_prefix = format!("{}/{}/", layout.specs_root, args.feature);
    let inbox_rel = format!("{}/inbox.md", layout.specs_root);

    let Some(first_commit) = first_commit_for_prefix(repository, &project.to_git(&spec_prefix))?
    else {
        return Ok(DiffCrossSpecResult {
            first_commit: String::new(),
            current_head: String::new(),
            cross_spec_paths: Vec::new(),
            // Empty lists alone would read as "no cross-spec impact". They
            // carry guidance instead, so the caller reports "unknowable"
            // rather than a clean bill this primitive cannot vouch for.
            guidance: Some(format!(
                "no commit touches {}/{} yet, so there is no window to diff — \
                 cross-spec impact is unknown, not absent. Commit the spec directory \
                 to get a real answer.",
                layout.specs_root, args.feature
            )),
        });
    };
    let head_oid = repository.head()?.peel_to_commit()?.id();
    let first_tree = repository.find_commit(first_commit)?.tree()?;

    // One diff, first-commit tree → working tree, scoped to the spec root.
    // Untracked files (a brand-new sibling scenario) must surface too. The
    // pathspec is literal: the project's path from the work tree may hold
    // `*`, `?` or `[`, which as a glob would match no file, and the empty
    // result would read as no impact (QUAL-CLAIM-001).
    let mut opts = DiffOptions::new();
    opts.pathspec(project.to_git(&layout.specs_root))
        .disable_pathspec_match(true)
        .include_untracked(true)
        .recurse_untracked_dirs(true);
    let diff = repository.diff_tree_to_workdir_with_index(Some(&first_tree), Some(&mut opts))?;

    let mut cross_spec: BTreeSet<String> = BTreeSet::new();
    diff.foreach(
        &mut |delta, _| {
            for path in [delta.old_file().path(), delta.new_file().path()]
                .into_iter()
                .flatten()
            {
                let s = path.to_string_lossy().replace('\\', "/");
                // Belt and braces over the pathspec: keep spec-root paths
                // only, and drop the feature's own dir and the inbox.
                if let Some(inside) = project.to_project(&s)
                    && inside.starts_with(&root_prefix)
                    && !inside.starts_with(&spec_prefix)
                    && inside != inbox_rel
                {
                    cross_spec.insert(inside.to_string());
                }
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(DiffCrossSpecResult {
        first_commit: first_commit.to_string(),
        current_head: head_oid.to_string(),
        cross_spec_paths: cross_spec.into_iter().collect(),
        guidance: None,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{commit_all, write};
    use git2::Repository;

    fn args(feature: &str) -> DiffCrossSpecArgs {
        DiffCrossSpecArgs {
            feature: feature.into(),
        }
    }

    /// A repo where 020-demo starts, then a sibling spec, the inbox, the
    /// feature's own dir, and non-spec code all change.
    fn seeded_repo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("README.md"), "# repo\n");
        write(
            &tmp.path().join("specs/007-sibling/spec.md"),
            "---\nstatus: done\n---\n\n# 007\n",
        );
        write(&tmp.path().join("specs/inbox.md"), "# Inbox\n");
        commit_all(&repo, "chore: init");
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");
        // Post-first-commit changes: the feature's own dir, a sibling
        // spec, an inbox capture, and non-spec code.
        write(
            &tmp.path().join("specs/020-demo/tasks.md"),
            "# Tasks\n\n- [x] 1\n",
        );
        write(
            &tmp.path().join("specs/007-sibling/spec.md"),
            "---\nstatus: done\n---\n\n# 007\n\nNew criterion.\n",
        );
        write(
            &tmp.path().join("specs/inbox.md"),
            "# Inbox\n\n- security: token logged in plaintext\n",
        );
        write(&tmp.path().join("runtime/src/main.rs"), "fn main() {}\n");
        commit_all(&repo, "feat(020): implement");
        tmp
    }

    #[test]
    fn reports_sibling_changes_only() {
        let tmp = seeded_repo();
        let result = run(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(
            result.cross_spec_paths,
            vec!["specs/007-sibling/spec.md".to_string()],
            "own feature dir, inbox, and non-spec code are all excluded"
        );
        let wire = serde_json::to_value(&result).unwrap();
        assert!(
            wire.get("inbox-additions").is_none() && wire.get("inbox-standing").is_none(),
            "no inbox field is reported any more: {wire}"
        );
        assert_ne!(result.first_commit, "");
        assert_ne!(result.current_head, "");
    }

    /// A project in a subdirectory of its repository, beside another project
    /// whose spec changes in the same window. History and the diff name
    /// every path from the work tree, so asked by the path from the project
    /// root there was no first commit and the impact read as unknown; the
    /// window is found, reported from the project root, and the other
    /// project's spec is not this one's impact (spec 059).
    #[test]
    fn a_subdirectory_project_reports_its_own_siblings_from_its_root() {
        use crate::primitives::git_fixture;
        let tmp = tempfile::tempdir().unwrap();
        let (repo, project) = git_fixture::subdirectory_project(tmp.path());
        let other = tmp.path().join("other");
        write(
            &project.join("specs/007-sibling/spec.md"),
            "---\nstatus: done\n---\n\n# 007\n",
        );
        write(&project.join("specs/inbox.md"), "# Inbox\n");
        write(
            &other.join("specs/007-sibling/spec.md"),
            "---\nstatus: done\n---\n\n# other 007\n",
        );
        git_fixture::commit_all(&repo, "chore: init");
        write(
            &project.join("specs/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        let first = git_fixture::commit_all(&repo, "feat(020): plan");
        write(
            &project.join("specs/007-sibling/spec.md"),
            "---\nstatus: done\n---\n\n# 007\n\nNew criterion.\n",
        );
        write(
            &other.join("specs/007-sibling/spec.md"),
            "---\nstatus: done\n---\n\n# other 007\n\nChanged.\n",
        );
        write(
            &project.join("specs/inbox.md"),
            "# Inbox\n\n- a logged todo\n",
        );
        git_fixture::commit_all(&repo, "feat(020): implement");

        let result = run(&args("020-demo"), &project).unwrap();
        assert_eq!(result.first_commit, first.to_string());
        assert_eq!(result.guidance, None);
        assert_eq!(
            result.cross_spec_paths,
            vec!["specs/007-sibling/spec.md".to_string()],
            "the project's own inbox is excluded, and the other project's spec is not this one's"
        );
    }

    /// A project whose path from the work tree holds a glob character. As a
    /// glob the diff pathspec matched no file, so the window read as clean
    /// while a sibling spec had changed (QUAL-CLAIM-001); it is literal now.
    #[test]
    fn a_project_path_with_a_glob_character_is_matched_literally() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let project = tmp.path().join("app[v2]");
        write(
            &project.join("specs/007-sibling/spec.md"),
            "---\nstatus: done\n---\n\n# 007\n",
        );
        commit_all(&repo, "chore: init");
        write(
            &project.join("specs/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");
        write(
            &project.join("specs/007-sibling/spec.md"),
            "---\nstatus: done\n---\n\n# 007\n\nNew criterion.\n",
        );
        commit_all(&repo, "feat(020): implement");

        let result = run(&args("020-demo"), &project).unwrap();
        assert_eq!(result.guidance, None);
        assert_eq!(
            result.cross_spec_paths,
            vec!["specs/007-sibling/spec.md".to_string()]
        );
    }

    #[test]
    fn clean_window_reports_empty_lists() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");
        let result = run(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.cross_spec_paths, [] as [String; 0]);
    }

    #[test]
    fn uncommitted_working_tree_changes_surface() {
        // Step 7 fires before the task's commit: an untracked sibling
        // scenario must surface, and an uncommitted inbox edit must not.
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            "---\nstatus: in-progress\n---\n\n# 020\n",
        );
        write(&tmp.path().join("specs/inbox.md"), "# Inbox\n");
        commit_all(&repo, "feat(020): begin");
        // Uncommitted: new sibling scenario file + inbox append.
        write(
            &tmp.path().join("specs/007-sibling/scenarios/edge.md"),
            "---\nsection: \"Core\"\n---\n\n# Edge\n",
        );
        write(
            &tmp.path().join("specs/inbox.md"),
            "# Inbox\n\n- leak: connection pool never drained\n",
        );
        let result = run(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(
            result.cross_spec_paths,
            vec!["specs/007-sibling/scenarios/edge.md".to_string()],
            "a logged todo is no spec's impact"
        );
    }

    #[test]
    fn honors_configured_specs_root() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join(".govern.toml"),
            "[paths]\nspecs-root = \"governance\"\n",
        );
        write(
            &tmp.path().join("governance/020-demo/spec.md"),
            "---\nstatus: planned\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");
        write(
            &tmp.path().join("governance/007-sib/spec.md"),
            "---\nstatus: draft\n---\n\n# 007\n",
        );
        write(
            &tmp.path().join("governance/inbox.md"),
            "# Inbox\n\n- captured under the custom root\n",
        );
        commit_all(&repo, "feat(020): work");
        let result = run(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(
            result.cross_spec_paths,
            vec!["governance/007-sib/spec.md".to_string()],
            "the inbox under a configured root is excluded too"
        );
    }

    #[test]
    fn missing_spec_history_is_the_empty_result() {
        // Scenario derive-boundary-uncommitted-spec-dir: erroring here would
        // relocate the halt `derive-boundary` no longer raises, since the
        // same /ductus:implement walk reaches this primitive at steps 7 and 13.
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("README.md"), "# repo\n");
        commit_all(&repo, "chore: init");
        write(
            &tmp.path().join("specs/030-orphan/spec.md"),
            "---\nstatus: planned\n---\n\n# 030\n",
        );
        let result = run(&args("030-orphan"), tmp.path()).unwrap();
        assert_eq!(result.first_commit, "");
        assert_eq!(result.current_head, "");
        assert_eq!(result.cross_spec_paths, [] as [String; 0]);

        // The empty lists must not read as a clean bill of health: guidance
        // is what separates "no impact" from "impact unknowable".
        let guidance = result
            .guidance
            .expect("empty-window result carries guidance");
        assert!(guidance.contains("specs/030-orphan"));
        assert!(
            guidance.contains("unknown, not absent"),
            "names the distinction the empty lists would otherwise hide: {guidance}"
        );
    }

    #[test]
    fn a_real_window_carries_no_guidance() {
        // Guidance presence is the caller's signal, so an ordinary run must
        // never set it — otherwise every report would hedge.
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            "---\nstatus: in-progress\n---\n\n# 020\n",
        );
        commit_all(&repo, "feat(020): plan");

        let result = run(&args("020-demo"), tmp.path()).unwrap();
        assert!(result.guidance.is_none());
        assert_ne!(result.first_commit, "");
    }

    #[test]
    fn missing_feature_and_traversal_error() {
        let tmp = seeded_repo();
        let err = run(&args("099-absent"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
        let err = run(&args("../escape"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }
}
