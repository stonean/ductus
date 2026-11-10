//! `check-stuck` — count commits on `tasks.md` since the spec's reopening started,
//! surfacing cycles where the same task is touched repeatedly.
//!
//! History reading is branch-shape tolerant (scenario
//! `primitive-robustness-hardening`): the commit a reopening started at
//! is found even when it landed on a merged side branch, transitions are
//! detected by comparing each commit's status against its *own parents'*
//! blobs (never against walk-order neighbors, which fabricates phantom
//! transitions across topologically adjacent commits of different
//! branches), and the commit count uses `since..HEAD` reachability rather
//! than a first-parent walk (which never reaches a side-branch `since`).
//! On a repo with no merge commits the results are identical to the
//! pre-hardening behavior.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use git2::{Repository, Sort};

use crate::primitives::{
    PrimitiveError, ProjectRepository, Result, SkipScanner, blob_text, checkbox,
    frontmatter_status, parse_feature_dir,
};
use crate::schema::paths;
use crate::schema::primitives::{CheckStuckArgs, CheckStuckResult};

/// Execute the `check-stuck` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::FeatureNotFound`] when the feature directory
/// is absent and [`PrimitiveError::Git`] for any libgit2 failure (repo not
/// found, walk failure, tree lookup, etc.).
pub fn run(args: &CheckStuckArgs, repo: &Path) -> Result<CheckStuckResult> {
    let layout = paths::Paths::load(repo)?;
    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;
    let feature_dir = repo.join(&layout.specs_root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: args.feature.clone(),
        });
    }
    // History names every path from the git work tree; a project in a
    // subdirectory of its repository asked by the path from its own root
    // found no transition, counted nothing, and never reported stuck.
    let project = ProjectRepository::discover(repo)?;
    let repository = &project.repository;
    let spec_rel = project.to_git(&format!("{}/{}/spec.md", layout.specs_root, args.feature));
    let tasks = format!("{}/{}/tasks.md", layout.specs_root, args.feature);
    let tasks_rel = project.to_git(&tasks);

    // No start reads as not stuck, so one that history cannot place has to say so.
    let (since, guidance) = match find_in_progress_commit(repository, &spec_rel) {
        Ok(since) => (since, None),
        Err(error @ PrimitiveError::ShallowHistory { .. }) => (None, Some(error.to_string())),
        Err(error) => return Err(error),
    };
    let count = count_commits_touching(repository, &tasks_rel, since.as_deref())?;

    // Second condition (per scenario check-stuck-tasks-md-advancement):
    // `stuck` only fires when the first incomplete subtask in tasks.md has
    // NOT advanced across the walked commit window. Position-based equality:
    // the linear line index of the first `- [ ]` group at `since-sha` is
    // compared against the same index at HEAD. Vacuous-false when either
    // index is unavailable (no tasks.md at since-sha, or no incomplete
    // subtasks remain at HEAD).
    let first_incomplete_unchanged = match since.as_deref() {
        Some(s) => first_incomplete_index_unchanged(&project, &tasks, s)?,
        None => false,
    };
    let stuck = count >= args.threshold && first_incomplete_unchanged;

    Ok(CheckStuckResult {
        commit_count: count,
        stuck,
        since_sha: since.unwrap_or_default(),
        threshold: args.threshold,
        guidance,
    })
}

/// Compare the linear line-index of the first `- [ ]` group in `tasks`, a
/// path from the project root, at the commit `since_sha` vs. at HEAD. Returns
/// `true` when both indices exist and match (the first incomplete subtask
/// hasn't advanced). Returns `false` when either index is unavailable (no tasks.md at since, or all
/// subtasks complete at HEAD) — vacuous-false matches the scenario's edge
/// cases (no first-incomplete subtask at baseline / completion is the
/// opposite of stuck).
fn first_incomplete_index_unchanged(
    project: &ProjectRepository,
    tasks: &str,
    since_sha: &str,
) -> Result<bool> {
    let head = project
        .repository
        .head()?
        .peel_to_commit()?
        .id()
        .to_string();
    let head_content = read_blob_at_commit(&project.repository, &head, &project.to_git(tasks))?;
    let since_content =
        read_blob_at_commit(&project.repository, since_sha, &project.to_git(tasks))?;
    let head_idx = first_incomplete_subtask_index(head_content.as_deref().unwrap_or(""));
    let since_idx = first_incomplete_subtask_index(since_content.as_deref().unwrap_or(""));
    Ok(match (head_idx, since_idx) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    })
}

/// Read the blob at `path_rel` from the tree of the commit named `sha`.
fn read_blob_at_commit(repo: &Repository, sha: &str, path_rel: &str) -> Result<Option<String>> {
    let oid = git2::Oid::from_str(sha)?;
    let tree = repo.find_commit(oid)?.tree()?;
    match feature_file_in_tree(repo, &tree, path_rel) {
        Some(found) => blob_text(repo, &tree, &found),
        None => Ok(None),
    }
}

/// Return the 0-based line index of the first `- [ ]` group in `content`,
/// or `None` when no incomplete subtask exists. Fenced code blocks and HTML
/// comments are skipped so example `- [ ]` lines inside `` ``` `` blocks or
/// `<!-- … -->` guidance comments do not match.
fn first_incomplete_subtask_index(content: &str) -> Option<usize> {
    let mut skip = SkipScanner::default();
    for (idx, line) in content.lines().enumerate() {
        if skip.skip(line) {
            continue;
        }
        // An unchecked box by the grammar every tasks parser shares, so a
        // nested or tab-separated `- [ ]` counts here as it does in
        // `read-tasks`.
        if checkbox::find_checkbox_line(line)
            .is_some_and(|(_bracket, marker)| line.as_bytes()[marker] == b' ')
        {
            return Some(idx);
        }
    }
    None
}

/// The commit the spec's newest reopening started at, or `None` when it has had none:
/// where it last left `done`, or, on a spec never `done`, where it first entered
/// `in-progress`. A trip back through `draft` stays in the reopening, since it never
/// reaches `done`, so the work before it stays in every window that starts here (spec 079).
///
/// Branch-shape tolerance: the walk covers all parents (not just the
/// first-parent chain), so a start that landed on a merged side branch is still
/// visited. A commit is judged against its own parents, since comparing against
/// walk-order neighbors (the pre-hardening behavior) fabricates phantom transitions
/// when the topological order interleaves commits from different branches. A merge
/// that simply carries a parent's open reopening forward starts nothing; the real
/// start lives in that parent's history and is found there.
///
/// With `TOPOLOGICAL | REVERSE` sorting parents are always visited
/// before children, so "most recent" is the last start seen.
///
/// The status-parse memoization is keyed by the spec's tree-entry **blob
/// OID**, not the commit OID: the `spec.md` blob is byte-identical across
/// nearly every commit (status changes only at transitions), so a
/// blob-keyed cache collapses the parse to one per distinct spec version
/// rather than one per commit — turning tens of thousands of redundant
/// frontmatter parses on a big-history repo into a handful.
///
/// Shared with `compute-review-scope` and `diff-cross-spec`, whose windows start
/// no earlier than that commit's parent. `spec_rel` is looked up in commit trees, so it names
/// the spec from the work tree's root ([`super::ProjectRepository::to_git`]),
/// not from the project root; a project-relative name finds no history in a
/// project that is a subdirectory of its repository (spec 059).
///
/// # Errors
///
/// [`PrimitiveError::ShallowHistory`] when a shallow clone's history stops at a commit
/// holding the spec below `done` and no later commit leaves `done`, so the start may lie
/// before the cut; and any libgit2 failure.
pub(crate) fn find_in_progress_commit(repo: &Repository, spec_rel: &str) -> Result<Option<String>> {
    let walk = walk_reopenings(repo, spec_rel)?;
    if walk.cut && walk.newest.is_some() {
        return Err(PrimitiveError::ShallowHistory {
            spec: spec_rel.to_string(),
        });
    }
    Ok(walk.newest.map(|start| start.to_string()))
}

/// Whether `HEAD` lies inside a reopening, so an uncommitted return to `in-progress` is
/// that reopening's and starts nothing.
pub(crate) fn reopening_is_open(repo: &Repository, spec_rel: &str) -> Result<bool> {
    Ok(walk_reopenings(repo, spec_rel)?.open_at_head)
}

/// What one walk of a spec's status history says about its reopenings.
struct ReopeningWalk {
    newest: Option<git2::Oid>,
    open_at_head: bool,
    /// A shallow clone's history stops inside the newest reopening or a pass before
    /// it, with no later commit leaving `done` to start one afresh.
    cut: bool,
}

fn walk_reopenings(repo: &Repository, spec_rel: &str) -> Result<ReopeningWalk> {
    let mut walk = repo.revwalk()?;
    walk.push_head()?;
    walk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

    let shallow = repo.is_shallow();
    let mut cut = false;
    let mut newest = None;
    let mut inside: HashSet<git2::Oid> = HashSet::new();
    let mut status_by_blob: HashMap<git2::Oid, Option<String>> = HashMap::new();
    for oid in walk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let Some(status) = status_of_commit(repo, &mut status_by_blob, oid, spec_rel)? else {
            // No readable status says nothing about where the reopening stands, so one
            // passing through a damaged or removed `spec.md` keeps its start.
            if commit.parent_ids().any(|parent| inside.contains(&parent)) {
                inside.insert(oid);
            }
            continue;
        };
        if status == "done" {
            continue;
        }
        cut |= shallow && commit.parent_count() == 0;
        let mut carried = false;
        let mut left_done = false;
        for parent in commit.parent_ids() {
            carried |= inside.contains(&parent);
            left_done |= status_of_commit(repo, &mut status_by_blob, parent, spec_rel)?.as_deref()
                == Some("done");
        }
        if carried {
            inside.insert(oid);
        } else if left_done || status == "in-progress" {
            cut &= !left_done;
            inside.insert(oid);
            newest = Some(oid);
        }
    }
    let head = repo.head()?.peel_to_commit()?.id();
    Ok(ReopeningWalk {
        newest,
        open_at_head: inside.contains(&head),
        cut,
    })
}

/// Read (with memoization) the spec's `status:` value at a commit.
///
/// `cache` is keyed by the spec's tree-entry **blob OID** rather than the
/// commit OID: identical spec content shares a blob, so this collapses the
/// parse to one per distinct spec version across the whole walk (see
/// [`find_in_progress_commit`]). A commit whose tree has no `spec.md`
/// (blob absent) has no status and is not cached — that case is cheap and
/// carries no reusable parse.
fn status_of_commit(
    repo: &Repository,
    cache: &mut HashMap<git2::Oid, Option<String>>,
    oid: git2::Oid,
    spec_rel: &str,
) -> Result<Option<String>> {
    let tree = repo.find_commit(oid)?.tree()?;
    let Some(blob_id) =
        feature_file_in_tree(repo, &tree, spec_rel).and_then(|path| tree_entry_id(&tree, &path))
    else {
        return Ok(None);
    };
    if let Some(cached) = cache.get(&blob_id) {
        return Ok(cached.clone());
    }
    // Frontmatter reading reuses the shared [`frontmatter_status`] helper —
    // CRLF-aware splitting plus real YAML parsing; the hand-rolled
    // predecessor missed `\r\n---\r\n` close fences, so a CRLF checkout's
    // transitions were invisible.
    let blob = repo.find_blob(blob_id)?;
    let status = std::str::from_utf8(blob.content())
        .ok()
        .and_then(|content| frontmatter_status(content, Path::new(spec_rel)));
    cache.insert(blob_id, status.clone());
    Ok(status)
}

/// `git_path`, a file directly in a feature's directory, as `tree` names it. A spec
/// renamed later held it under its old slug, and the number is what stays
/// (§numbering), so a directory of the same form answers when the name does not.
pub(crate) fn feature_file_in_tree(
    repo: &Repository,
    tree: &git2::Tree<'_>,
    git_path: &str,
) -> Option<String> {
    if tree_entry_id(tree, git_path).is_some() {
        return Some(git_path.to_string());
    }
    let (directory_path, file) = git_path.rsplit_once('/')?;
    let (root, directory) = directory_path.rsplit_once('/')?;
    let form = parse_feature_dir(directory)?;
    let specs = tree
        .get_path(Path::new(root))
        .ok()?
        .to_object(repo)
        .ok()?
        .into_tree()
        .ok()?;
    let name = specs
        .iter()
        .filter_map(|entry| entry.name().ok().map(str::to_string))
        .find(|name| parse_feature_dir(name).as_ref() == Some(&form))?;
    let renamed = format!("{root}/{name}/{file}");
    tree_entry_id(tree, &renamed).map(|_| renamed)
}

/// Count commits touching `path_rel` in the `since..HEAD` reachability
/// set, plus the `since` commit itself when it touches the file
/// (inclusive per the data-model semantics: the count begins at the
/// transition). Reachability-based selection is branch-shape tolerant: a
/// `since` on a merged side branch is still an ancestor cutoff, and side
/// branch commits merged after the transition are counted exactly once
/// (see [`commit_touches`] for the merge-commit TREESAME rule). The
/// count is a property of the commit *set*, so walk order never affects
/// the result — deterministic without relying on TIME sorting.
fn count_commits_touching(
    repo: &Repository,
    path_rel: &str,
    since_sha: Option<&str>,
) -> Result<u32> {
    let Some(since) = since_sha else {
        return Ok(0);
    };
    let since_oid = git2::Oid::from_str(since)?;
    let mut walk = repo.revwalk()?;
    walk.push_head()?;
    walk.hide(since_oid)?;
    let mut count: u32 = 0;
    for oid in walk {
        if commit_touches(repo, oid?, path_rel)? {
            count = count.saturating_add(1);
        }
    }
    if commit_touches(repo, since_oid, path_rel)? {
        count = count.saturating_add(1);
    }
    Ok(count)
}

/// Whether a commit changed `path_rel`. For a root commit: the path
/// exists in its tree. For a single-parent commit: the blob differs from
/// the parent's (add/remove/modify all differ). For a merge commit: the
/// blob differs from *every* parent — git's TREESAME rule. A merge that
/// merely integrates a side branch's edit is TREESAME to that side
/// parent and does not count; the side branch's own commits are in the
/// walked set and carry the count, so nothing is double-counted. A merge
/// whose conflict resolution produced content unlike any parent counts
/// once, as it should.
fn commit_touches(repo: &Repository, oid: git2::Oid, path_rel: &str) -> Result<bool> {
    let commit = repo.find_commit(oid)?;
    let own = feature_entry_id(repo, &commit.tree()?, path_rel);
    if commit.parent_count() == 0 {
        return Ok(own.is_some());
    }
    for parent in commit.parents() {
        if feature_entry_id(repo, &parent.tree()?, path_rel) == own {
            return Ok(false);
        }
    }
    Ok(true)
}

/// The object id of the feature's file in `tree`, found by its spec's number when its
/// slug differed there, so a rename is no change to it and a reopening keeps its count.
fn feature_entry_id(repo: &Repository, tree: &git2::Tree<'_>, path_rel: &str) -> Option<git2::Oid> {
    tree_entry_id(tree, &feature_file_in_tree(repo, tree, path_rel)?)
}

/// The object id at `path_rel` in `tree`, or `None` when absent.
pub(crate) fn tree_entry_id(tree: &git2::Tree<'_>, path_rel: &str) -> Option<git2::Oid> {
    tree.get_path(Path::new(path_rel)).ok().map(|e| e.id())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{commit_all, write};
    use git2::{IndexAddOption, Repository, Signature};

    /// Stage the full workdir and create a commit with explicit parents.
    /// `update_head: false` leaves HEAD where it is (side-branch commit);
    /// `true` advances HEAD (libgit2 requires the current tip to be the
    /// first parent, which merge fixtures satisfy by listing it first).
    fn commit_with_parents(
        repo: &Repository,
        message: &str,
        parents: &[git2::Oid],
        update_head: bool,
    ) -> git2::Oid {
        let mut index = repo.index().unwrap();
        index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let sig = Signature::now("Test", "test@example.com").unwrap();
        let parent_commits: Vec<git2::Commit> = parents
            .iter()
            .map(|oid| repo.find_commit(*oid).unwrap())
            .collect();
        let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();
        let refname = if update_head { Some("HEAD") } else { None };
        repo.commit(refname, &sig, &sig, message, &tree, &parent_refs)
            .unwrap()
    }

    fn spec(status: &str) -> String {
        format!(
            "---\nstatus: {status}\ndependencies: []\n---\n\n# X\n\n## Acceptance Criteria\n\n- [ ] one\n"
        )
    }

    /// CRLF variant of [`spec`] — close fence is `\r\n---\r\n`, invisible
    /// to a splitter that only knows the LF form.
    fn spec_crlf(status: &str) -> String {
        format!("---\r\nstatus: {status}\r\ndependencies: []\r\n---\r\n\r\n# X\r\n")
    }

    /// An unchecked box is read by the grammar every tasks parser shares, so
    /// a tab after the dash counts, and a `]` run into text does not.
    #[test]
    fn the_first_unchecked_box_uses_the_shared_grammar() {
        assert_eq!(
            first_incomplete_subtask_index("# T\n\n- [x] a\n-\t[ ] tabbed\n"),
            Some(3)
        );
        assert_eq!(
            first_incomplete_subtask_index("# T\n\n  - [ ] nested\n"),
            Some(2)
        );
        assert_eq!(first_incomplete_subtask_index("# T\n\n- [ ]x\n"), None);
    }

    #[test]
    fn counts_commits_under_configured_specs_root() {
        // Spec 040: the git-relative spec/tasks paths track `[paths] specs-root`,
        // so stuck detection works when the spec root is renamed.
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join(".govern.toml"),
            "[paths]\nspecs-root = \"governance\"\n",
        );
        let spec_path = tmp.path().join("governance/010-demo/spec.md");
        let tasks_path = tmp.path().join("governance/010-demo/tasks.md");
        write(&spec_path, &spec("planned"));
        write(&tasks_path, "# Tasks\n\n## 1. Bootstrap\n\n- [ ] start\n");
        commit_all(&repo, "feat(010): plan");

        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore(010): begin");

        for i in 1..=3 {
            write(
                &tasks_path,
                &format!("# Tasks v{i}\n\n## 1. Bootstrap\n\n- [ ] still\n"),
            );
            commit_all(&repo, &format!("wip(010): pass {i}"));
        }

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 3,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.commit_count, 3, "counts commits under governance/");
        assert!(result.stuck);
    }

    #[test]
    fn counts_commits_since_in_progress() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");
        write(&spec_path, &spec("planned"));
        write(&tasks_path, "# Tasks\n\n## 1. Bootstrap\n\n- [ ] start\n");
        commit_all(&repo, "feat(010): plan");

        // Advance to in-progress.
        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore(010): begin");

        // Touch tasks.md three times.
        for i in 1..=3 {
            write(
                &tasks_path,
                &format!("# Tasks v{i}\n\n## 1. Bootstrap\n\n- [ ] still\n"),
            );
            commit_all(&repo, &format!("wip(010): pass {i}"));
        }

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 3,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.threshold, 3);
        assert_eq!(result.commit_count, 3);
        assert!(result.stuck);
        assert_ne!(result.since_sha, "");
    }

    /// The same history as [`counts_commits_since_in_progress`], with the
    /// project in a subdirectory of its repository: history names the spec
    /// from the work tree, so asked by the path from the project root it found
    /// no transition and counted nothing (spec 059).
    #[test]
    fn counts_commits_for_a_project_in_a_repository_subdirectory() {
        use crate::primitives::git_fixture;
        let tmp = tempfile::tempdir().unwrap();
        let (repo, project) = git_fixture::subdirectory_project(tmp.path());
        let spec_path = project.join("specs/010-demo/spec.md");
        let tasks_path = project.join("specs/010-demo/tasks.md");
        write(&spec_path, &spec("planned"));
        write(&tasks_path, "# Tasks\n\n## 1. Bootstrap\n\n- [ ] start\n");
        git_fixture::commit_all(&repo, "feat(010): plan");

        write(&spec_path, &spec("in-progress"));
        let begin = git_fixture::commit_all(&repo, "chore(010): begin");

        for i in 1..=3 {
            write(
                &tasks_path,
                &format!("# Tasks v{i}\n\n## 1. Bootstrap\n\n- [ ] still\n"),
            );
            git_fixture::commit_all(&repo, &format!("wip(010): pass {i}"));
        }

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 3,
            },
            &project,
        )
        .unwrap();
        assert_eq!(result.since_sha, begin.to_string());
        assert_eq!(result.commit_count, 3);
        assert!(result.stuck);
    }

    #[test]
    fn no_in_progress_yields_zero() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");
        write(&spec_path, &spec("planned"));
        write(&tasks_path, "# Tasks\n");
        commit_all(&repo, "feat(010): plan");
        write(&tasks_path, "# Tasks v2\n");
        commit_all(&repo, "wip");
        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 3,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.commit_count, 0);
        assert!(!result.stuck);
        assert_eq!(result.since_sha, "");
    }

    /// Reopen regression test: a spec that went
    /// `planned → in-progress → done → in-progress` must measure
    /// `commit_count` from the SECOND `in-progress` transition, not the
    /// first. The original 022 implementation captured the first
    /// transition, causing 023's first commit attempt on the living-specs
    /// task to fire `stuck: true` with `commit-count: 8` because the
    /// initial implementation window's commits still counted toward the
    /// reopen's count.
    #[test]
    fn reopen_measures_from_most_recent_in_progress_transition() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");

        // Original implementation window: planned → in-progress, plus
        // several tasks.md commits during the work.
        write(&spec_path, &spec("planned"));
        write(&tasks_path, "# Tasks\n\n## 1. Bootstrap\n");
        commit_all(&repo, "feat: plan");

        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: begin original implementation"); // first transition

        for i in 1..=5 {
            write(&tasks_path, &format!("# Tasks v{i}\n\n## 1. Bootstrap\n"));
            commit_all(&repo, &format!("wip: pass {i}"));
        }

        // Close out: in-progress → done.
        write(&spec_path, &spec("done"));
        commit_all(&repo, "feat: ship");

        // Reopen: done → in-progress via /ductus:amend's back-edge. The new
        // task starts fresh.
        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: reopen for follow-on"); // second transition (this is `since`)

        // One tasks.md commit during the reopen window.
        write(
            &tasks_path,
            "# Tasks v6\n\n## 1. Bootstrap\n\n## 2. Follow-on\n",
        );
        commit_all(&repo, "wip: start follow-on");

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 3,
            },
            tmp.path(),
        )
        .unwrap();
        // Count must be 1 (only the reopen-window tasks.md commit), not 6
        // (the original-window commits plus the reopen commit). And not
        // stuck — threshold is 3.
        assert_eq!(
            result.commit_count, 1,
            "expected count from most-recent in-progress, got {} (likely measured from first transition)",
            result.commit_count
        );
        assert!(!result.stuck);
        assert_ne!(result.since_sha, "");

        // Sanity: walking by the captured since_sha should land at the
        // reopen commit, not the original begin commit.
        let repo2 = Repository::open(tmp.path()).unwrap();
        let oid = git2::Oid::from_str(&result.since_sha).unwrap();
        let commit = repo2.find_commit(oid).unwrap();
        assert_eq!(
            commit.message().unwrap(),
            "chore: reopen for follow-on",
            "since_sha should point at the reopen commit, not the original begin"
        );
    }

    /// Counterpart to the reopen test: a spec that has NEVER been
    /// `done` (no reopen has occurred) must still produce the correct
    /// count from its single `in-progress` transition. The fix to the
    /// reopen case must not break this routine path.
    /// Mark `boundary` as where a shallow clone's history stops, and reopen the repository
    /// so its commits read that way.
    fn shallow_at(tmp: &tempfile::TempDir, boundary: git2::Oid) -> Repository {
        std::fs::write(tmp.path().join(".git/shallow"), format!("{boundary}\n")).unwrap();
        Repository::open(tmp.path()).unwrap()
    }

    /// A rename leaves the status under the old slug, and the start where it was.
    #[test]
    fn a_rename_keeps_where_the_reopening_started() {
        let (tmp, repo, commits) = walk_statuses(&["done", "in-progress"]);
        std::fs::rename(
            tmp.path().join("specs/010-demo"),
            tmp.path().join("specs/010-renamed"),
        )
        .unwrap();
        commit_all(&repo, "spec(010): a new slug");

        let found = find_in_progress_commit(&repo, "specs/010-renamed/spec.md").unwrap();
        assert_eq!(found, Some(commits[1].to_string()));
    }

    /// A commit with no readable status says nothing about where the reopening stands,
    /// so a damaged or removed `spec.md` and its repair keep the start.
    #[test]
    fn a_commit_with_no_readable_status_keeps_the_start() {
        for damage in ["---\nstatus: [\n---\n", ""] {
            let (tmp, repo, commits) = walk_statuses(&["done", "in-progress"]);
            let spec_file = tmp.path().join("specs/010-demo/spec.md");
            if damage.is_empty() {
                std::fs::remove_file(&spec_file).unwrap();
            } else {
                std::fs::write(&spec_file, damage).unwrap();
            }
            commit_all(&repo, "damage");
            write(&spec_file, &spec("in-progress"));
            commit_all(&repo, "repair");
            assert_eq!(start_of(&repo), Some(commits[1].to_string()), "{damage:?}");
        }
    }

    /// A rename is no change to `tasks.md`, committed or not, and the count and the first
    /// box follow it, so a first box that never moved still reads stuck.
    #[test]
    fn stuck_follows_a_rename() {
        for committed in [true, false] {
            stuck_after_a_rename(committed);
        }
    }

    fn stuck_after_a_rename(committed: bool) {
        let (tmp, repo, _) = walk_statuses(&["done"]);
        let tasks = tmp.path().join("specs/010-demo/tasks.md");
        write(&tasks, "# T\n\n## 1. A\n\n- [ ] first\n");
        write(
            &tmp.path().join("specs/010-demo/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repo, "status(010): reopen, with its task");
        write(&tasks, "# T\n\n## 1. A\n\n- [ ] first\n- [x] other\n");
        commit_all(&repo, "tasks(010): two");
        std::fs::rename(
            tmp.path().join("specs/010-demo"),
            tmp.path().join("specs/010-renamed"),
        )
        .unwrap();
        if committed {
            commit_all(&repo, "spec(010): a new slug");
        }
        let result = run(
            &CheckStuckArgs {
                feature: "010-renamed".to_string(),
                threshold: 2,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.commit_count, 2, "{result:?}");
        assert!(result.stuck, "{result:?}");
    }

    /// The name still decides when it is there, and a number no directory holds finds
    /// nothing.
    #[test]
    fn a_file_is_found_by_its_name_then_by_its_number() {
        let (tmp, repo, _) = walk_statuses(&["in-progress"]);
        let tree = repo
            .head()
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .tree()
            .unwrap();
        let found = |path: &str| feature_file_in_tree(&repo, &tree, path);
        assert_eq!(
            found("specs/010-demo/spec.md").as_deref(),
            Some("specs/010-demo/spec.md")
        );
        assert_eq!(
            found("specs/010-other/spec.md").as_deref(),
            Some("specs/010-demo/spec.md")
        );
        assert_eq!(found("specs/011-other/spec.md"), None);
        assert_eq!(found("specs/010-other/plan.md"), None);
        drop(tmp);
    }

    /// The start may lie before the cut, and a confident answer would leave work unread.
    #[test]
    fn a_shallow_history_stopping_inside_a_reopening_cannot_place_it() {
        let (tmp, _repo, commits) = walk_statuses(&["done", "in-progress", "in-progress"]);
        let shallow = shallow_at(&tmp, commits[2]);
        let found = find_in_progress_commit(&shallow, "specs/010-demo/spec.md");
        assert!(
            matches!(found, Err(PrimitiveError::ShallowHistory { .. })),
            "{found:?}"
        );
    }

    /// Leaving `done` is visible from here, so the depth does not matter.
    #[test]
    fn a_shallow_history_holding_the_start_still_places_it() {
        let (tmp, _repo, commits) = walk_statuses(&["done", "done", "in-progress"]);
        let shallow = shallow_at(&tmp, commits[1]);
        let found = find_in_progress_commit(&shallow, "specs/010-demo/spec.md").unwrap();
        assert_eq!(found, Some(commits[2].to_string()));
    }

    /// A pass the cut went through is over, and a later reopening starts afresh.
    #[test]
    fn a_reopening_after_the_cut_is_placed() {
        let (tmp, _repo, commits) = walk_statuses(&["in-progress", "done", "in-progress"]);
        let shallow = shallow_at(&tmp, commits[0]);
        let found = find_in_progress_commit(&shallow, "specs/010-demo/spec.md").unwrap();
        assert_eq!(found, Some(commits[2].to_string()));
    }

    /// `is_shallow` says nothing of a complete history.
    #[test]
    fn a_complete_history_is_never_cut() {
        let (_tmp, repo, commits) = walk_statuses(&["in-progress"]);
        let found = find_in_progress_commit(&repo, "specs/010-demo/spec.md").unwrap();
        assert_eq!(found, Some(commits[0].to_string()));
    }

    /// `stuck: false` from a start it could not place is no answer, and says so.
    #[test]
    fn check_stuck_says_when_a_shallow_history_cannot_place_the_start() {
        let (tmp, _repo, commits) = walk_statuses(&["in-progress", "in-progress"]);
        shallow_at(&tmp, commits[1]);
        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".to_string(),
                threshold: 3,
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.stuck);
        assert!(
            result
                .guidance
                .as_deref()
                .is_some_and(|g| g.contains("--unshallow")),
            "{result:?}"
        );
    }

    /// The commit each status in turn lands at, with the last one's sha returned.
    fn walk_statuses(statuses: &[&str]) -> (tempfile::TempDir, Repository, Vec<git2::Oid>) {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let mut commits = Vec::new();
        for status in statuses {
            write(&spec_path, &spec(status));
            commits.push(commit_all(&repo, &format!("status: {status}")));
        }
        (tmp, repo, commits)
    }

    fn start_of(repo: &Repository) -> Option<String> {
        find_in_progress_commit(repo, "specs/010-demo/spec.md").unwrap()
    }

    /// A trip through `draft` never reaches `done`, so it is one reopening, starting
    /// where the spec left `done`.
    #[test]
    fn a_trip_through_draft_stays_in_its_reopening() {
        let (_tmp, repo, commits) = walk_statuses(&[
            "done",
            "in-progress",
            "draft",
            "clarified",
            "planned",
            "in-progress",
        ]);
        assert_eq!(start_of(&repo), Some(commits[1].to_string()));
    }

    /// Never `done`: its first entry into `in-progress`, though a trip follows it.
    #[test]
    fn a_spec_never_done_starts_at_its_first_entry() {
        let (_tmp, repo, commits) =
            walk_statuses(&["planned", "in-progress", "draft", "planned", "in-progress"]);
        assert_eq!(start_of(&repo), Some(commits[1].to_string()));
    }

    /// The back-edge from `done` itself: the reopening starts where it left `done`.
    #[test]
    fn leaving_done_for_draft_starts_the_reopening() {
        let (_tmp, repo, commits) =
            walk_statuses(&["done", "draft", "clarified", "planned", "in-progress"]);
        assert_eq!(start_of(&repo), Some(commits[1].to_string()));
    }

    /// `done` ends a reopening, so the next one starts afresh.
    #[test]
    fn reaching_done_ends_the_reopening() {
        let (_tmp, repo, commits) =
            walk_statuses(&["in-progress", "draft", "in-progress", "done", "in-progress"]);
        assert_eq!(start_of(&repo), Some(commits[4].to_string()));
    }

    /// `HEAD` on the way back through `draft` is inside the reopening; at `done`, or
    /// before the first entry, it is not.
    #[test]
    fn a_reopening_is_open_until_it_reaches_done() {
        let spec_rel = "specs/010-demo/spec.md";
        let (_tmp, repo, _) = walk_statuses(&["done", "in-progress", "draft", "planned"]);
        assert!(reopening_is_open(&repo, spec_rel).unwrap());
        let (_tmp, repo, _) = walk_statuses(&["done", "in-progress", "done"]);
        assert!(!reopening_is_open(&repo, spec_rel).unwrap());
        let (_tmp, repo, _) = walk_statuses(&["draft", "clarified", "planned"]);
        assert!(!reopening_is_open(&repo, spec_rel).unwrap());
    }

    #[test]
    fn first_in_progress_works_when_never_reopened() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");
        write(&spec_path, &spec("planned"));
        write(&tasks_path, "# Tasks\n");
        commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: begin");
        for i in 1..=2 {
            write(&tasks_path, &format!("# Tasks v{i}\n"));
            commit_all(&repo, &format!("wip: pass {i}"));
        }
        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 3,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.commit_count, 2);
        assert!(!result.stuck);
    }

    /// Mechanical sweep commits between the most-recent `in-progress`
    /// transition and HEAD touch `spec.md` but do NOT change the
    /// `status:` line. They must not register as new in-progress
    /// transitions (which would skip past them to the older, wrong
    /// transition).
    #[test]
    fn mechanical_sweeps_do_not_disturb_since_sha() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");

        write(&spec_path, &spec("planned"));
        write(&tasks_path, "# Tasks\n");
        commit_all(&repo, "feat: plan");
        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: begin");

        // Mechanical sweep on spec.md (e.g., rename pass) — same status
        // value, different body content.
        let mut sweeped = spec("in-progress");
        sweeped.push_str("\nMechanical sweep added this line.\n");
        write(&spec_path, &sweeped);
        commit_all(&repo, "chore: rename sweep across specs");

        // One legitimate tasks.md commit after the sweep.
        write(&tasks_path, "# Tasks v2\n");
        commit_all(&repo, "wip: pass 1");

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 3,
            },
            tmp.path(),
        )
        .unwrap();
        // Count is just the tasks.md commit (1). The sweep commit doesn't
        // touch tasks.md so it doesn't count toward stuck.
        assert_eq!(result.commit_count, 1);
        assert!(!result.stuck);
    }

    /// Branch-shape tolerance (scenario primitive-robustness-hardening):
    /// the `in-progress` transition commit landed on a side branch that
    /// was merged into main. The transition must still be found (the
    /// side-branch commit is `since`), and the count must cover the
    /// post-merge tasks.md commits — a first-parent-only walk from HEAD
    /// never reaches a side-branch `since` and would count back to root.
    #[test]
    fn transition_on_merged_side_branch_is_found() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");

        write(&spec_path, &spec("planned"));
        write(&tasks_path, "# Tasks\n\n## 1. Bootstrap\n\n- [ ] start\n");
        let m1 = commit_all(&repo, "feat: plan");

        // Side branch off m1 flips the spec to in-progress. HEAD stays
        // at m1.
        write(&spec_path, &spec("in-progress"));
        let s1 = commit_with_parents(&repo, "side: begin", &[m1], false);

        // Main advances independently (spec restored to planned, an
        // unrelated file added).
        write(&spec_path, &spec("planned"));
        write(&tmp.path().join("README.md"), "readme\n");
        let m2 = commit_all(&repo, "main: unrelated");

        // Merge the side branch: the merged tree carries in-progress.
        write(&spec_path, &spec("in-progress"));
        commit_with_parents(&repo, "merge side", &[m2, s1], true);

        // Two tasks.md commits after the merge; the first incomplete
        // subtask stays at the same line index (genuinely stuck).
        for i in 1..=2 {
            write(
                &tasks_path,
                &format!("# Tasks v{i}\n\n## 1. Bootstrap\n\n- [ ] still\n"),
            );
            commit_all(&repo, &format!("wip: pass {i}"));
        }

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 2,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            result.since_sha,
            s1.to_string(),
            "since must be the side-branch transition commit"
        );
        assert_eq!(
            result.commit_count, 2,
            "only the two post-merge tasks.md commits count"
        );
        assert!(result.stuck);
    }

    /// Phantom-transition regression: a side branch whose spec never left
    /// `planned` merges into a main line that has been `in-progress` for
    /// several commits. Under walk-order neighbor tracking the planned
    /// side commit interleaves before the merge and fabricates a
    /// transition at the merge commit; per-parent comparison keeps the
    /// real (older) transition.
    #[test]
    fn merged_planned_branch_does_not_fabricate_transition() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");
        let base_tasks = "# Tasks\n\n## 1. Bootstrap\n\n- [ ] start\n";

        write(&spec_path, &spec("planned"));
        write(&tasks_path, base_tasks);
        let m1 = commit_all(&repo, "feat: plan");

        write(&spec_path, &spec("in-progress"));
        let m2 = commit_all(&repo, "chore: begin"); // the real transition

        write(
            &tasks_path,
            "# Tasks v1\n\n## 1. Bootstrap\n\n- [ ] still\n",
        );
        commit_all(&repo, "wip: pass 1");
        write(
            &tasks_path,
            "# Tasks v2\n\n## 1. Bootstrap\n\n- [ ] still\n",
        );
        let m4 = commit_all(&repo, "wip: pass 2");

        // Side branch off m1: spec still planned, tasks untouched, one
        // unrelated file. Topologically unordered w.r.t. m2..m4.
        write(&spec_path, &spec("planned"));
        write(&tasks_path, base_tasks);
        write(&tmp.path().join("side.md"), "side\n");
        let s1 = commit_with_parents(&repo, "side: unrelated", &[m1], false);

        // Merge keeps main's in-progress spec and tasks.
        write(&spec_path, &spec("in-progress"));
        write(
            &tasks_path,
            "# Tasks v2\n\n## 1. Bootstrap\n\n- [ ] still\n",
        );
        commit_with_parents(&repo, "merge side", &[m4, s1], true);

        write(
            &tasks_path,
            "# Tasks v3\n\n## 1. Bootstrap\n\n- [ ] still\n",
        );
        commit_all(&repo, "wip: pass 3");

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 99,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            result.since_sha,
            m2.to_string(),
            "since must be the real main-line transition, not the merge"
        );
        // Three tasks.md commits since m2 (side commit restored tasks →
        // TREESAME; merge TREESAME to first parent).
        assert_eq!(result.commit_count, 3);
        assert!(!result.stuck);
    }

    /// CRLF close fence: a spec checked out with CRLF line endings has a
    /// `\r\n---\r\n` close fence, which the hand-rolled splitter missed
    /// (it only knew `\n---\n`), making every transition invisible. The
    /// shared CRLF-aware helper reads it.
    #[test]
    fn crlf_spec_frontmatter_transition_is_detected() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");

        write(&spec_path, &spec_crlf("planned"));
        write(&tasks_path, "# Tasks\n\n## 1. Bootstrap\n\n- [ ] start\n");
        commit_all(&repo, "feat: plan");

        write(&spec_path, &spec_crlf("in-progress"));
        commit_all(&repo, "chore: begin");

        write(
            &tasks_path,
            "# Tasks v1\n\n## 1. Bootstrap\n\n- [ ] still\n",
        );
        commit_all(&repo, "wip: pass 1");

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 1,
            },
            tmp.path(),
        )
        .unwrap();
        assert!(
            !result.since_sha.is_empty(),
            "CRLF close fence must not hide the transition"
        );
        assert_eq!(result.commit_count, 1);
        assert!(result.stuck);
    }

    /// The false-positive case the `check-stuck-tasks-md-advancement`
    /// scenario closes: threshold-count commits land on `tasks.md`, BUT
    /// each one flips a different subtask checkbox. Under the old
    /// implementation (count >= threshold alone), `stuck: true` would fire
    /// despite real progress. With the second condition (first-incomplete
    /// index unchanged), the check correctly reports `stuck: false`.
    #[test]
    fn stuck_false_when_checkboxes_flipped_across_threshold_commits() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec_path = tmp.path().join("specs/010-demo/spec.md");
        let tasks_path = tmp.path().join("specs/010-demo/tasks.md");

        write(&spec_path, &spec("planned"));
        write(
            &tasks_path,
            "# Tasks\n\n## 1. Bootstrap\n\n- [ ] subtask A\n- [ ] subtask B\n- [ ] subtask C\n",
        );
        commit_all(&repo, "feat: plan");

        write(&spec_path, &spec("in-progress"));
        commit_all(&repo, "chore: begin");

        // Three commits, each flipping a subsequent checkbox. The first
        // incomplete subtask's index advances every commit (line 4 → 5 → 6).
        write(
            &tasks_path,
            "# Tasks\n\n## 1. Bootstrap\n\n- [x] subtask A\n- [ ] subtask B\n- [ ] subtask C\n",
        );
        commit_all(&repo, "wip: flip A");
        write(
            &tasks_path,
            "# Tasks\n\n## 1. Bootstrap\n\n- [x] subtask A\n- [x] subtask B\n- [ ] subtask C\n",
        );
        commit_all(&repo, "wip: flip B");
        write(
            &tasks_path,
            "# Tasks\n\n## 1. Bootstrap\n\n- [x] subtask A\n- [x] subtask B\n- [x] subtask C\n",
        );
        commit_all(&repo, "wip: flip C");

        // Add one more commit so count clearly exceeds threshold, again
        // with a different first-incomplete state — re-introduce one.
        write(
            &tasks_path,
            "# Tasks\n\n## 1. Bootstrap\n\n- [x] subtask A\n- [x] subtask B\n- [x] subtask C\n\n## 2. Follow-on\n\n- [ ] D\n",
        );
        commit_all(&repo, "wip: add follow-on task");

        let result = run(
            &CheckStuckArgs {
                feature: "010-demo".into(),
                threshold: 3,
            },
            tmp.path(),
        )
        .unwrap();
        // 4 commits touched tasks.md → count exceeds threshold.
        assert_eq!(result.commit_count, 4);
        // But subtasks advanced across the window: the first incomplete
        // subtask moved from line 4 (subtask A) at since-sha to a later
        // index (line 10, the new Follow-on D) at HEAD. NOT stuck.
        assert!(
            !result.stuck,
            "stuck must not fire when first-incomplete index has advanced"
        );
    }
}
