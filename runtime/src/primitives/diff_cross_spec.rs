//! `diff-cross-spec` — the cross-spec impact surface for `/ductus:implement`. What it
//! reports is `schema::descriptions::DIFF_CROSS_SPEC`.
//!
//! The deterministic filter implement steps 7 and 13 previously re-derived by hand per
//! task (spec 022, scenario coverage-expansion-primitives). Its window starts at the
//! spec's last reopen (spec 079, AC7): starting from the feature's first commit, it reported
//! 507 paths for one task of one reopened spec. An uncommitted change counts because the
//! per-task summary fires before the task's commit, when any sibling-spec edits are
//! still uncommitted.

use std::collections::BTreeSet;
use std::path::Path;

use git2::{DiffOptions, Oid};

use crate::primitives::check_stuck::find_in_progress_commit;
use crate::primitives::commit_attribution::Attributor;
use crate::primitives::compute_review_scope::{transition_is_uncommitted, transition_parent};
use crate::primitives::derive_boundary::first_commit_for_prefix;
use crate::primitives::{PrimitiveError, ProjectRepository, Result, parse_feature_dir};
use crate::schema::paths;
use crate::schema::primitives::{DiffCrossSpecArgs, DiffCrossSpecResult, ListsForm};

/// Execute the `diff-cross-spec` primitive: [`compute`], returned inline
/// while it fits `[artifacts] read-size-bytes`, and past it with
/// `cross-spec-paths` in the file it names and only their count inline
/// (`bounded_lists::place`; 066 scenario `scope-results-have-a-bounded-size`).
///
/// # Errors
///
/// Every error [`compute`] returns, and those `bounded_lists::place` returns
/// for a malformed read size or a list file that cannot be written.
pub fn run(args: &DiffCrossSpecArgs, repo: &Path) -> Result<DiffCrossSpecResult> {
    let cross = compute(args, repo)?;
    let mut result = DiffCrossSpecResult {
        window_base: cross.window_base,
        current_head: cross.current_head,
        cross_spec_count: cross.cross_spec_paths.len(),
        cross_spec_paths: Some(cross.cross_spec_paths.clone()),
        form: ListsForm::Inline,
        lists_file: None,
        guidance: cross.guidance,
    };
    let lists = serde_json::json!({ "cross-spec-paths": cross.cross_spec_paths });
    if let Some(file) =
        super::bounded_lists::place(repo, "diff-cross-spec", &args.feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.cross_spec_paths = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// The cross-spec impact surface whole: what `diff-cross-spec` resolves,
/// before its result is bounded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ImpactSurface {
    /// See [`DiffCrossSpecResult::window_base`].
    pub(crate) window_base: String,
    /// See [`DiffCrossSpecResult::current_head`].
    pub(crate) current_head: String,
    /// See [`DiffCrossSpecResult::cross_spec_paths`].
    pub(crate) cross_spec_paths: Vec<String>,
    /// See [`DiffCrossSpecResult::guidance`].
    pub(crate) guidance: Option<String>,
}

/// Resolve the cross-spec impact surface, every path whole.
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
pub(crate) fn compute(args: &DiffCrossSpecArgs, repo: &Path) -> Result<ImpactSurface> {
    let layout = paths::Paths::load(repo)?;
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

    let unknown = |guidance: String| ImpactSurface {
        window_base: String::new(),
        current_head: String::new(),
        cross_spec_paths: Vec::new(),
        guidance: Some(guidance),
    };
    let window_base = match window_base(&project, &layout.specs_root, &args.feature, &feature_dir) {
        Ok(base) => base,
        Err(error @ PrimitiveError::ShallowHistory { .. }) => {
            return Ok(unknown(format!(
                "{error} — cross-spec impact is unknown, not absent."
            )));
        }
        Err(error) => return Err(error),
    };
    let Some(window_base) = window_base else {
        // Empty lists alone would read as "no cross-spec impact". They
        // carry guidance instead, so the caller reports "unknowable"
        // rather than a clean bill this primitive cannot vouch for.
        return Ok(unknown(format!(
            "no commit touches {}/{} yet, so there is no window to diff — \
             cross-spec impact is unknown, not absent. Commit the spec directory \
             to get a real answer.",
            layout.specs_root, args.feature
        )));
    };
    let head = repository.head()?.peel_to_commit()?;
    let form = parse_feature_dir(&args.feature);
    // A directory of this spec's number is this spec under an earlier slug.
    let own_directory = |path: &str| {
        form.is_some()
            && path
                .strip_prefix(&root_prefix)
                .and_then(|rest| rest.split_once('/'))
                .is_some_and(|(directory, _)| parse_feature_dir(directory) == form)
    };
    let is_sibling = |path: &str| {
        path.starts_with(&root_prefix)
            && !path.starts_with(&spec_prefix)
            && !own_directory(path)
            && path != inbox_rel
    };

    // What differs across the window, so a change made and undone inside it is
    // no impact, and what is uncommitted, which nothing can attribute.
    let changed = sibling_changes(&project, &layout.specs_root, window_base, &is_sibling)?;
    let uncommitted = sibling_changes(&project, &layout.specs_root, head.id(), &is_sibling)?;

    // The commits that belong to this spec. A feature directory
    // `parse_feature_dir` does not recognize can be claimed by no commit, so its
    // window keeps every committed sibling change rather than reading as clean.
    let mut own: BTreeSet<String> = BTreeSet::new();
    if form.is_some() && window_base != head.id() {
        let attributor = Attributor::new(&project, repo, &layout.specs_root);
        let mut walk = repository.revwalk()?;
        walk.push(head.id())?;
        walk.hide(window_base)?;
        for oid in walk {
            let claims = attributor.claims(&repository.find_commit(oid?)?)?;
            if form.as_ref().is_some_and(|form| claims.is_claimed_by(form)) {
                own.extend(claims.paths);
            }
        }
    }

    let cross_spec_paths: Vec<String> = changed
        .into_iter()
        .filter(|path| form.is_none() || own.contains(path) || uncommitted.contains(path))
        .collect();

    Ok(ImpactSurface {
        window_base: window_base.to_string(),
        current_head: head.id().to_string(),
        cross_spec_paths,
        guidance: None,
    })
}

/// Where the window starts: `HEAD` while the spec's move into `in-progress` is
/// uncommitted, the parent of the commit its reopening started at, or for a spec
/// never `in-progress` its first spec-directory commit. `None` when no commit
/// touches the spec directory.
fn window_base(
    project: &ProjectRepository,
    specs_root: &str,
    feature: &str,
    feature_dir: &Path,
) -> Result<Option<Oid>> {
    let repository = &project.repository;
    let spec_prefix = format!("{specs_root}/{feature}/");
    let Some(first) = first_commit_for_prefix(repository, &project.to_git(&spec_prefix))? else {
        return Ok(None);
    };
    if transition_is_uncommitted(project, specs_root, feature, feature_dir)? {
        return Ok(Some(repository.head()?.peel_to_commit()?.id()));
    }
    let spec_rel = project.to_git(&format!("{spec_prefix}spec.md"));
    Ok(Some(
        match find_in_progress_commit(repository, &spec_rel)? {
            Some(sha) => Oid::from_str(&transition_parent(repository, &sha)?)?,
            None => first,
        },
    ))
}

/// The sibling paths that differ between `base`'s tree and the working tree,
/// from the project root. Untracked files (a brand-new sibling scenario) surface
/// too. The pathspec is literal: the project's path from the work tree may hold
/// `*`, `?` or `[`, which as a glob would match no file, and the empty result
/// would read as no impact (QUAL-CLAIM-001).
fn sibling_changes(
    project: &ProjectRepository,
    specs_root: &str,
    base: Oid,
    is_sibling: &dyn Fn(&str) -> bool,
) -> Result<BTreeSet<String>> {
    let repository = &project.repository;
    let tree = repository.find_commit(base)?.tree()?;
    let mut opts = DiffOptions::new();
    opts.pathspec(project.to_git(specs_root))
        .disable_pathspec_match(true)
        .include_untracked(true)
        .recurse_untracked_dirs(true);
    let diff = repository.diff_tree_to_workdir_with_index(Some(&tree), Some(&mut opts))?;

    let mut paths = BTreeSet::new();
    diff.foreach(
        &mut |delta, _| {
            for path in [delta.old_file().path(), delta.new_file().path()]
                .into_iter()
                .flatten()
            {
                let s = path.to_string_lossy().replace('\\', "/");
                if let Some(inside) = project.to_project(&s)
                    && is_sibling(inside)
                {
                    paths.insert(inside.to_string());
                }
            }
            true
        },
        None,
        None,
        None,
    )?;
    Ok(paths)
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

    /// 020-demo reopened, its work committed, then its slug changed.
    fn renamed_repo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        let spec = tmp.path().join("specs/020-demo/spec.md");
        write(&spec, "---\nstatus: done\n---\n\n# 020\n");
        write(&tmp.path().join("specs/020-demo/tasks.md"), "# Tasks\n");
        commit_all(&repo, "chore: done");
        write(&spec, "---\nstatus: in-progress\n---\n\n# 020\n");
        commit_all(&repo, "feat(020): reopen");
        std::fs::rename(
            tmp.path().join("specs/020-demo"),
            tmp.path().join("specs/020-renamed"),
        )
        .unwrap();
        commit_all(&repo, "spec(020): a new slug");
        tmp
    }

    /// Its old directory is this spec under another name, not a sibling.
    #[test]
    fn a_rename_is_no_cross_spec_impact() {
        let tmp = renamed_repo();
        let surface = compute(&args("020-renamed"), tmp.path()).unwrap();
        assert_eq!(surface.cross_spec_paths, Vec::<String>::new());
    }

    /// A window that cannot be placed is unknown impact, never none.
    #[test]
    fn a_shallow_history_is_unknown_impact() {
        let tmp = renamed_repo();
        let repo = Repository::open(tmp.path()).unwrap();
        let head = repo.head().unwrap().peel_to_commit().unwrap().id();
        std::fs::write(tmp.path().join(".git/shallow"), format!("{head}\n")).unwrap();

        let surface = compute(&args("020-renamed"), tmp.path()).unwrap();
        assert!(
            surface
                .guidance
                .as_deref()
                .is_some_and(|g| g.contains("--unshallow")),
            "{surface:?}"
        );
        assert_eq!(surface.cross_spec_paths, Vec::<String>::new());
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
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(
            result.cross_spec_paths,
            vec!["specs/007-sibling/spec.md".to_string()],
            "own feature dir, inbox, and non-spec code are all excluded"
        );
        let wire = serde_json::to_value(run(&args("020-demo"), tmp.path()).unwrap()).unwrap();
        assert!(
            wire.get("inbox-additions").is_none() && wire.get("inbox-standing").is_none(),
            "no inbox field is reported any more: {wire}"
        );
        assert_ne!(result.window_base, "");
        assert_ne!(result.current_head, "");
    }

    /// Past the read size the paths move to the list file the result names,
    /// absent inline with their count kept, so a caller reading the result
    /// alone never takes the inline list for the whole (066 scenario
    /// `scope-results-have-a-bounded-size`).
    #[test]
    fn past_the_read_size_the_paths_move_to_a_file() {
        let tmp = seeded_repo();
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 1\n",
        )
        .unwrap();
        let result = run(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        assert_eq!(result.cross_spec_paths, None, "the list is not inline");
        assert_eq!(result.cross_spec_count, 1);
        let file = result.lists_file.clone().expect("a list file");
        let held: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(file)).unwrap()).unwrap();
        assert_eq!(
            held["cross-spec-paths"],
            serde_json::json!(["specs/007-sibling/spec.md"])
        );
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "diff-cross-spec",
            &serde_json::to_value(&result).unwrap(),
            &held,
        );
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

        let result = compute(&args("020-demo"), &project).unwrap();
        assert_eq!(result.window_base, first.to_string());
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

        let result = compute(&args("020-demo"), &project).unwrap();
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
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
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
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
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
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
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
        let result = compute(&args("030-orphan"), tmp.path()).unwrap();
        assert_eq!(result.window_base, "");
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

        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert!(result.guidance.is_none());
        assert_ne!(result.window_base, "");
    }

    #[test]
    fn missing_feature_and_traversal_error() {
        let tmp = seeded_repo();
        let err = compute(&args("099-absent"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
        let err = compute(&args("../escape"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }

    // -- spec 079, AC7 --------------------------------------------------------

    fn spec(status: &str) -> String {
        format!("---\nstatus: {status}\ndependencies: []\n---\n\n# X\n")
    }

    /// 020-demo in `in-progress` since a committed transition, with sibling
    /// 007-sibling. Returns the tree, its repository and the transition's parent.
    fn in_progress() -> (tempfile::TempDir, Repository, String) {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/007-sibling/spec.md"), &spec("done"));
        write(&tmp.path().join("specs/020-demo/spec.md"), &spec("planned"));
        let parent = commit_all(&repo, "chore: seed").to_string();
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repo, "status(020): start");
        (tmp, repo, parent)
    }

    fn touch(tmp: &tempfile::TempDir, repo: &Repository, file: &str, subject: &str) {
        write(&tmp.path().join(file), &format!("{subject}\n"));
        commit_all(repo, subject);
    }

    #[test]
    fn the_window_starts_at_the_transition_parent() {
        let (tmp, _repo, parent) = in_progress();
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.window_base, parent);
    }

    /// The figure that named this: 507 paths from an earlier window's work.
    #[test]
    fn a_reopen_leaves_an_earlier_windows_sibling_changes_out() {
        let (tmp, repo, _) = in_progress();
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/old.md",
            "feat(020): the earlier window",
        );
        write(&tmp.path().join("specs/020-demo/spec.md"), &spec("done"));
        let parent = commit_all(&repo, "status(020): done").to_string();
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repo, "status(020): reopen");
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/new.md",
            "feat(020): this window",
        );

        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.window_base, parent);
        assert_eq!(result.cross_spec_paths, ["specs/007-sibling/new.md"]);
    }

    #[test]
    fn another_specs_commit_is_absent() {
        let (tmp, repo, _) = in_progress();
        touch(&tmp, &repo, "specs/007-sibling/a.md", "feat(007): its own");
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/b.md",
            "chore: claims 007 by its directory",
        );
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.cross_spec_paths, [] as [String; 0]);
    }

    #[test]
    fn a_commit_no_spec_claims_is_absent() {
        let (tmp, repo, _) = in_progress();
        // A subject naming no spec, touching only a file directly under the
        // spec root: nobody's change, so nobody's impact.
        touch(&tmp, &repo, "specs/notes.md", "chore: corpus notes");
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.cross_spec_paths, [] as [String; 0]);
    }

    #[test]
    fn a_commit_belonging_to_the_spec_counts_as_its_impact() {
        let (tmp, repo, _) = in_progress();
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/by_name.md",
            "fix(020): named",
        );
        write(&tmp.path().join("specs/020-demo/notes.md"), "n\n");
        write(&tmp.path().join("specs/007-sibling/by_directory.md"), "d\n");
        commit_all(&repo, "chore: touches both directories");
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/shared.md",
            "fix(007, 020): both",
        );

        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(
            result.cross_spec_paths,
            [
                "specs/007-sibling/by_directory.md",
                "specs/007-sibling/by_name.md",
                "specs/007-sibling/shared.md"
            ]
        );
    }

    #[test]
    fn an_uncommitted_sibling_edit_counts_even_on_another_specs_path() {
        let (tmp, repo, _) = in_progress();
        touch(&tmp, &repo, "specs/007-sibling/a.md", "feat(007): its own");
        write(&tmp.path().join("specs/007-sibling/a.md"), "edited\n");
        write(&tmp.path().join("specs/007-sibling/new.md"), "untracked\n");

        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(
            result.cross_spec_paths,
            ["specs/007-sibling/a.md", "specs/007-sibling/new.md"]
        );
    }

    #[test]
    fn a_change_undone_inside_the_window_is_no_impact() {
        let (tmp, repo, _) = in_progress();
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/spec.md",
            "fix(020): changed",
        );
        write(&tmp.path().join("specs/007-sibling/spec.md"), &spec("done"));
        commit_all(&repo, "fix(020): undone");
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.cross_spec_paths, [] as [String; 0]);
    }

    /// A trip through `draft` is one reopening, so the impact made before it stays.
    #[test]
    fn a_trip_through_draft_keeps_the_impact_made_before_it() {
        let (tmp, repo, parent) = in_progress();
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/before.md",
            "feat(020): before the trip",
        );
        for status in ["draft", "planned", "in-progress"] {
            write(&tmp.path().join("specs/020-demo/spec.md"), &spec(status));
            commit_all(&repo, "status(020): back");
        }
        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.window_base, parent);
        assert_eq!(result.cross_spec_paths, ["specs/007-sibling/before.md"]);
    }

    #[test]
    fn a_spec_never_in_progress_keeps_its_first_commit() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/007-sibling/spec.md"), &spec("done"));
        commit_all(&repo, "chore: seed");
        write(&tmp.path().join("specs/020-demo/spec.md"), &spec("planned"));
        let first = commit_all(&repo, "plan(020): a plan").to_string();
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/a.md",
            "plan(020): a sibling note",
        );

        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.window_base, first);
        assert_eq!(result.cross_spec_paths, ["specs/007-sibling/a.md"]);
    }

    #[test]
    fn an_uncommitted_reopen_starts_at_head() {
        let (tmp, repo, _) = in_progress();
        touch(
            &tmp,
            &repo,
            "specs/007-sibling/old.md",
            "feat(020): earlier",
        );
        write(&tmp.path().join("specs/020-demo/spec.md"), &spec("done"));
        let head = commit_all(&repo, "status(020): done").to_string();
        write(
            &tmp.path().join("specs/020-demo/spec.md"),
            &spec("in-progress"),
        );

        let result = compute(&args("020-demo"), tmp.path()).unwrap();
        assert_eq!(result.window_base, head);
        assert_eq!(result.cross_spec_paths, [] as [String; 0]);
    }

    #[test]
    fn the_wire_form_names_the_window_base() {
        let (tmp, _repo, parent) = in_progress();
        let wire = serde_json::to_value(run(&args("020-demo"), tmp.path()).unwrap()).unwrap();
        assert_eq!(wire["window-base"], parent);
        assert!(wire.get("first-commit").is_none(), "{wire}");
    }
}
