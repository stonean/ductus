//! `compute-review-scope` — deterministic scope resolution for `/ductus:review`.
//!
//! Resolves two things the review command needs, from git history:
//!
//! - **diff-base** — the **parent** of the commit the spec advanced to
//!   `in-progress` at (the transition itself is found by the lookup shared with
//!   `check-stuck`), or a caller-supplied `--since` override used verbatim. The
//!   parent, because `base..HEAD` excludes the base's own changes and the
//!   reopen flow commits the back-edge flip together with the work it
//!   authorises — see `scenarios/review-base-includes-the-transition-commit.md`.
//! - **scope** — the union of the plan's `Affected Files` set and the set of
//!   files modified since `diff-base`. Both, because either alone can omit
//!   what the review exists to look at.
//!
//! The project need not sit at its git repository's root: history is read by
//! each path from the work tree, and the files modified since the base are
//! reported from the project root, as the plan names them.
//!
//! It no longer reports an inbox window. Nothing captures findings to the
//! inbox any more (spec 058), so its additions over a review window say
//! nothing about the work being reviewed.
//!
//! Read-only.
//!
//! Defined by
//! `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`.

use std::collections::BTreeSet;
use std::path::Path;

use git2::{Oid, Repository};

use crate::primitives::check_stuck::find_in_progress_commit;
use crate::primitives::{PrimitiveError, ProjectRepository, Result, parse_affected_files};
use crate::schema::paths;
use crate::schema::primitives::{ComputeReviewScopeArgs, ComputeReviewScopeResult};

/// Execute the `compute-review-scope` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` would leave the spec
/// root, [`PrimitiveError::FeatureNotFound`] when the feature directory is
/// absent, [`PrimitiveError::Io`] when `plan.md` exists but cannot be read,
/// or [`PrimitiveError::Git`] for any libgit2 failure (repo discovery,
/// including a project no work tree contains, revparse of `--since`,
/// tree/diff lookup).
pub fn run(args: &ComputeReviewScopeArgs, repo: &Path) -> Result<ComputeReviewScopeResult> {
    super::validate_no_traversal(&args.feature)?;
    let layout = paths::Paths::load(repo);
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
    let spec_rel = project.to_git(&format!("{}/{}/spec.md", layout.specs_root, args.feature));

    let diff_base = match &args.since {
        // An explicit base is used verbatim. No parent walk: an operator naming
        // a commit means that commit, and quietly reviewing one more than they
        // asked for would be its own scope surprise.
        Some(reference) => repository
            .revparse_single(reference)?
            .peel_to_commit()?
            .id()
            .to_string(),
        // The transition commit is the *boundary* of the work window, and a
        // boundary belongs inside the window it bounds — so the base is its
        // parent. `base..HEAD` excludes the base's own changes, which is
        // harmless when `/ductus:implement` flips `planned -> in-progress` in a
        // commit holding only the status line, and silently fatal in the reopen
        // flow: `/ductus:amend`'s `done -> in-progress` back-edge is naturally
        // committed together with the work it authorises, so the window started
        // after the work and contained none of it. Two specs recorded 0 MUST /
        // 0 SHOULD over an empty subject that way on 2026-08-27 before the
        // operator caught it (scenario
        // `review-base-includes-the-transition-commit`).
        None => match find_in_progress_commit(repository, &spec_rel)? {
            Some(sha) => transition_parent(repository, &sha)?,
            None => String::new(),
        },
    };

    let modified_since = if diff_base.is_empty() {
        Vec::new()
    } else {
        diff_since(&project, &diff_base)?
    };

    let plan_affected = read_plan_affected(&feature_dir)?;

    // Union, not "whichever set is larger". Choosing one set can exclude the
    // files the work actually touched: a mature spec's plan lists its whole
    // surface, so on a small follow-on scenario the plan set wins on size and
    // the review is scoped to files the change never went near — the gate
    // reporting on a subject it did not examine. The union is bounded by
    // |plan| + |modified| and the two overlap heavily in practice.
    let scope: Vec<String> = plan_affected
        .iter()
        .chain(modified_since.iter())
        .cloned()
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect();

    Ok(ComputeReviewScopeResult {
        diff_base,
        scope,
        modified_since,
        plan_affected,
    })
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
/// shared helper returning different commits to its two callers would be the
/// drift this placement keeps out.
fn transition_parent(repository: &Repository, sha: &str) -> Result<String> {
    let commit = repository.find_commit(Oid::from_str(sha)?)?;
    Ok(match commit.parent(0) {
        Ok(parent) => parent.id().to_string(),
        Err(_) => sha.to_string(),
    })
}

/// Diff `base_sha..HEAD`: the sorted set of changed file paths inside the
/// project, named from the project root as the plan names them. A change
/// outside the project directory is another project's work and is left out.
fn diff_since(project: &ProjectRepository, base_sha: &str) -> Result<Vec<String>> {
    let repo = &project.repository;
    let base_tree = repo.find_commit(Oid::from_str(base_sha)?)?.tree()?;
    let head_tree = repo.head()?.peel_to_commit()?.tree()?;

    // Every changed file path (added / modified / deleted / renamed).
    let diff = repo.diff_tree_to_tree(Some(&base_tree), Some(&head_tree), None)?;
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
        assert!(run(&args("001-x", None), tmp.path()).is_ok());
        fs::write(
            tmp.path().join("specs/001-x/plan.md"),
            b"\xff\xfe not UTF-8",
        )
        .unwrap();
        let err = run(&args("001-x", None), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Io { .. }), "{err:?}");
    }

    #[test]
    fn diff_base_is_the_transition_commits_parent_and_scope_lists_modified_files() {
        let (tmp, sha) = repo_with_progress();
        let result = run(&args("001-x", None), tmp.path()).unwrap();
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

        let result = run(&args("001-x", None), tmp.path()).unwrap();
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

        let result = run(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, sha, "root commit should be its own base");
    }

    #[test]
    fn since_override_replaces_the_derived_base() {
        let (tmp, _sha) = repo_with_progress();
        // Override to HEAD → no files modified since HEAD.
        let result = run(&args("001-x", Some("HEAD")), tmp.path()).unwrap();
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
        let result = run(&args("001-x", Some(&sha)), tmp.path()).unwrap();
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
        let result = run(&args("001-x", None), tmp.path()).unwrap();
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
        let result = run(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, "");
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

        let result = run(&args("001-x", None), &project).unwrap();
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

    #[test]
    fn missing_feature_is_operational_error() {
        let tmp = tempfile::tempdir().unwrap();
        Repository::init(tmp.path()).unwrap();
        let err = run(&args("999-nope", None), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    #[test]
    fn plan_absent_falls_back_to_modified_since() {
        let (tmp, _sha) = repo_with_progress();
        // No plan.md written → plan_affected empty → scope = modified_since.
        let result = run(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.plan_affected, [] as [String; 0]);
        assert_eq!(result.scope, result.modified_since);
        assert_ne!(result.scope, [] as [String; 0]);
    }
}
