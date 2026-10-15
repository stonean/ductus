//! `compute-review-scope` — deterministic scope resolution for `/ductus:review`.
//!
//! Resolves two things the review command needs, from git history:
//!
//! - **diff-base** — the **parent** of the commit the spec advanced to
//!   `in-progress` at (the transition itself is found by the lookup shared with
//!   `check-stuck`), `HEAD` when that transition is not committed yet, or a
//!   caller-supplied `--since` override used verbatim. The parent, because the
//!   reopen flow commits the back-edge flip together with the work it
//!   authorises — see `scenarios/review-base-includes-the-transition-commit.md`.
//! - **scope** — the union of the plan's `Affected Files` set and the set of
//!   files modified since `diff-base`, in the working tree as well as in
//!   commits. Both, because either alone can omit what the review exists to
//!   look at.
//!
//! The review reads the working tree, so the files modified since the base
//! are the diff from the base to the working tree, and an uncommitted
//! transition starts the window at `HEAD`. Answered from commits alone, a
//! review run before its work was committed — the pipeline's natural order —
//! was scoped away from every file the work touched
//! (`scenarios/review-scope-includes-uncommitted-work.md`).
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

use git2::{DiffOptions, Oid, Repository};

use crate::primitives::check_stuck::find_in_progress_commit;
use crate::primitives::{
    PrimitiveError, ProjectRepository, Result, frontmatter_status, parse_affected_files,
};
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
        None if transition_is_uncommitted(
            &project,
            &layout.specs_root,
            &args.feature,
            &feature_dir,
        )? =>
        {
            // The transition that opened this window is not in history yet, so
            // the window starts at `HEAD`. History's newest transition is an
            // earlier window's — on a reopened spec, its first, months back.
            repository.head()?.peel_to_commit()?.id().to_string()
        }
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
fn transition_is_uncommitted(
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
/// shared helper returning different commits to its two callers would be the
/// drift this placement keeps out.
fn transition_parent(repository: &Repository, sha: &str) -> Result<String> {
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
    // `diff-cross-spec` takes them. Bounded to the project directory, so a
    // project in a subdirectory does not walk its neighbours' untracked
    // trees. Literal, because that path may hold `*`, `?` or `[`, which as a
    // glob would match no file and report no change (QUAL-CLAIM-001).
    let mut opts = DiffOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);
    let project_dir = project.to_git("");
    let project_dir = project_dir.trim_end_matches('/');
    if !project_dir.is_empty() {
        opts.pathspec(project_dir).disable_pathspec_match(true);
    }
    let diff = repo.diff_tree_to_workdir_with_index(Some(&base_tree), Some(&mut opts))?;
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
        let result = run(&args("001-x", None), tmp.path()).unwrap();
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
        let result = run(&args("001-x", None), tmp.path()).unwrap();
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
        let result = run(&args("001-x", None), tmp.path()).unwrap();
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
        let result = run(&args("001-x", Some("HEAD")), tmp.path()).unwrap();
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
        let result = run(&args("001-x", None), tmp.path()).unwrap();
        assert_eq!(result.diff_base, head_sha(&repo));
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
        let result = run(&args("001-x", None), tmp.path()).unwrap();
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
        let result = run(&args("001-x", None), tmp.path()).unwrap();
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
        let result = run(&args("001-x", None), &project).unwrap();
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
        let result = run(&args("001-x", None), &project).unwrap();
        assert_eq!(
            result.modified_since,
            vec!["specs/001-x/spec.md", "src/new.rs", "src/old.rs"]
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
