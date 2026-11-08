//! Whose change a commit is: the one answer the review window and scope, a fix
//! round's paths and `diff-cross-spec` share (spec 079, AC6).
//!
//! A commit belongs to each spec whose directory it touches and each spec its
//! subject's scope names. Both tests are needed: 35 commits since 2026-10-01
//! name a spec without touching a spec directory, and 30 touch one without
//! naming any. A project whose subjects name no spec loses only
//! [`CommitClaims::first_named`], and AC6 then gives an unplanned path outside
//! the other claimants' directories to every claimant.
//!
//! A merge commit claims nothing, since its branch's commits are in the walk,
//! and its [`CommitClaims::paths`] are only what it changed itself: the paths
//! differing from every parent. So the branch's work does not arrive a second
//! time through the merge, as work no spec claims, which counts whole in every
//! window.
//!
//! Each caller gains the same three facts from one diff, and gives up nothing
//! it enforced: none had an attribution rule of its own before this one.
//! Feature identity is [`FeatureForm`], never the directory's name, so a spec
//! whose slug changed is still one spec, and every form is recognized by
//! [`parse_feature_dir`], the corpus's single membership rule. A number with no
//! spec on disk claims nothing unless the commit touches that spec's directory,
//! which is how a retired spec's commits stay its own.

use std::path::Path;

use git2::{Commit, Tree};

use crate::primitives::compute_review_scope::{project_diff_options, project_paths};
use crate::primitives::{
    FeatureForm, ProjectRepository, Result, list_feature_dirs, parse_feature_dir, spec_feature_slug,
};

/// What one commit contributes to a window, and who it belongs to.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CommitClaims {
    /// Each spec the commit belongs to, subject-named first and in subject
    /// order. Empty for a merge and for a commit no spec claims.
    pub(crate) claimants: Vec<FeatureForm>,
    /// The first spec the subject's scope names, which takes a path in none of
    /// the claimants' plans. `None` when the subject names no spec.
    pub(crate) first_named: Option<FeatureForm>,
    /// The paths the commit changed, from the project root and sorted: against
    /// its parent, against nothing for a root commit, and for a merge only
    /// those differing from every parent. A path outside the project is left
    /// out.
    pub(crate) paths: Vec<String>,
}

impl CommitClaims {
    /// Whether `form` is one of the specs the commit belongs to.
    pub(crate) fn is_claimed_by(&self, form: &FeatureForm) -> bool {
        self.claimants.contains(form)
    }
}

/// Attributes commits for one project. Built once per walk, since it lists the
/// spec root, and [`Self::claims`] then runs once per commit.
pub(crate) struct Attributor<'a> {
    project: &'a ProjectRepository,
    specs_root: &'a str,
    /// The specs on disk with their directories, the only ones a subject alone
    /// can claim for.
    known: Vec<(FeatureForm, String)>,
}

impl<'a> Attributor<'a> {
    /// An attributor for the project at `repo`, whose spec root is `specs_root`.
    pub(crate) fn new(project: &'a ProjectRepository, repo: &Path, specs_root: &'a str) -> Self {
        let known = list_feature_dirs(&repo.join(specs_root))
            .into_iter()
            .filter_map(|name| parse_feature_dir(&name).map(|form| (form, name)))
            .collect();
        Self {
            project,
            specs_root,
            known,
        }
    }

    /// The directory of the spec `form`, when it is on disk.
    pub(crate) fn directory_of(&self, form: &FeatureForm) -> Option<&str> {
        self.known
            .iter()
            .find(|(known, _)| known == form)
            .map(|(_, directory)| directory.as_str())
    }

    /// The spec `form`, as a report names it: its directory, or for a spec no
    /// longer on disk the part of a directory's name before its slug.
    pub(crate) fn name_of(&self, form: &FeatureForm) -> String {
        self.directory_of(form).map_or_else(
            || match form {
                FeatureForm::Sequential { number } => format!("{number:03}"),
                FeatureForm::BranchScoped { identifier, n } => format!("{identifier}.{n}"),
            },
            str::to_string,
        )
    }

    /// Who `commit` belongs to, and the paths it contributes.
    ///
    /// # Errors
    ///
    /// [`crate::primitives::PrimitiveError::Git`] when a tree or a diff cannot
    /// be read, so a commit that could not be attributed never reads as one no
    /// spec claims.
    pub(crate) fn claims(&self, commit: &Commit<'_>) -> Result<CommitClaims> {
        let paths = self.own_paths(commit)?;
        if commit.parent_count() > 1 {
            return Ok(CommitClaims {
                paths,
                ..CommitClaims::default()
            });
        }
        let touched: Vec<FeatureForm> = paths
            .iter()
            .filter_map(|path| spec_feature_slug(path, self.specs_root))
            .filter_map(|slug| parse_feature_dir(&slug))
            .collect();
        // A subject that is not UTF-8 names nothing, so the commit is still
        // claimed by the directories it touches.
        let subject = commit.summary().ok().flatten().unwrap_or_default();
        let named: Vec<FeatureForm> = subject_scope_forms(subject)
            .into_iter()
            .filter(|form| self.directory_of(form).is_some() || touched.contains(form))
            .collect();
        let first_named = named.first().cloned();
        let mut claimants: Vec<FeatureForm> = Vec::new();
        for form in named.into_iter().chain(touched) {
            if !claimants.contains(&form) {
                claimants.push(form);
            }
        }
        Ok(CommitClaims {
            claimants,
            first_named,
            paths,
        })
    }

    /// Whether `commit` belongs to the spec `form`, whose directory may be gone from
    /// disk, or held another slug then: it touched that directory, or its subject's
    /// scope names the spec. [`Self::claims`] answers only for a spec on disk, since a
    /// number in a subject may be anything; a caller naming a spec it read from
    /// history has no such doubt. The tree's entry answers without a diff, as a
    /// caller walks all of history.
    pub(crate) fn belongs_to(&self, commit: &Commit<'_>, form: &FeatureForm) -> Result<bool> {
        let subject = commit.summary().ok().flatten().unwrap_or_default();
        if subject_scope_forms(subject).contains(form) {
            return Ok(true);
        }
        let root = self.project.to_git(self.specs_root);
        // By its number, since a spec whose slug changed is still one spec.
        let entry = |tree: &Tree<'_>| {
            tree.get_path(Path::new(&root))
                .ok()?
                .to_object(&self.project.repository)
                .ok()?
                .into_tree()
                .ok()?
                .iter()
                .find(|entry| entry.name().ok().and_then(parse_feature_dir).as_ref() == Some(form))
                .map(|entry| entry.id())
        };
        let parent = match commit.parent(0) {
            Ok(parent) => entry(&parent.tree()?),
            Err(_) => None,
        };
        Ok(entry(&commit.tree()?) != parent)
    }

    /// The paths `commit` changed itself: for a merge, only those differing
    /// from every parent, so a clean merge contributes none.
    fn own_paths(&self, commit: &Commit<'_>) -> Result<Vec<String>> {
        let tree = commit.tree()?;
        let mut parents = commit.parents();
        let Some(first) = parents.next() else {
            return self.paths_from(None, &tree);
        };
        let mut paths = self.paths_from(Some(&first.tree()?), &tree)?;
        for parent in parents {
            let others = self.paths_from(Some(&parent.tree()?), &tree)?;
            paths.retain(|path| others.binary_search(path).is_ok());
        }
        Ok(paths)
    }

    /// The sorted project paths that differ between `old` and `new`.
    fn paths_from(&self, old: Option<&Tree<'_>>, new: &Tree<'_>) -> Result<Vec<String>> {
        let mut options = project_diff_options(self.project);
        let diff = self
            .project
            .repository
            .diff_tree_to_tree(old, Some(new), Some(&mut options))?;
        project_paths(self.project, &diff)
    }
}

/// The specs a subject's leading parenthesised scope names, in subject order
/// and without regard to whether they exist: `fix(022, 066): …`, and the shapes
/// the history also holds, `docs(agents, 063): …`, `fix(022 analyze): …`,
/// `impl(077) task 8: …` and `review+analyze(059): …`. No colon is required,
/// since at `d62e72f3` that loses 50 commits' claims. Only the scope is read, so
/// a number in the subject's prose claims nothing, and a later clause's scope
/// is not read either: 2 of 822 commits since 2026-10-01 have one naming more.
fn subject_scope_forms(subject: &str) -> Vec<FeatureForm> {
    let Some((kind, rest)) = subject.split_once('(') else {
        return Vec::new();
    };
    let Some((scope, _)) = rest.split_once(')') else {
        return Vec::new();
    };
    if kind.is_empty() || kind.contains(char::is_whitespace) {
        return Vec::new();
    }
    scope
        .split(',')
        .filter_map(|token| token.split_whitespace().next())
        .filter_map(scope_token_form)
        .collect()
}

/// The spec a scope token names: the part of a directory's name before its
/// slug, `079` or `1234.1`, or the whole name. [`parse_feature_dir`] decides,
/// given a placeholder slug, so a scope and a directory cannot disagree about
/// what a number or an identifier may look like.
fn scope_token_form(token: &str) -> Option<FeatureForm> {
    parse_feature_dir(&format!("{token}-x"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{commit_all, subdirectory_project, write};
    use git2::{Oid, Repository, Signature};

    fn seq(number: u32) -> FeatureForm {
        FeatureForm::Sequential { number }
    }

    /// A repository at the project root with specs 012 and 034 on disk.
    fn project_with_two_specs(root: &Path) -> Repository {
        let repository = Repository::init(root).unwrap();
        write(&root.join("specs/012-a/spec.md"), "a\n");
        write(&root.join("specs/034-b/spec.md"), "b\n");
        commit_all(&repository, "chore: seed");
        repository
    }

    /// Change `file`, commit it with `subject`, and attribute the commit.
    fn claims_for(root: &Path, repository: &Repository, file: &str, subject: &str) -> CommitClaims {
        write(&root.join(file), &format!("{subject}\n"));
        let oid = commit_all(repository, subject);
        attribute(root, oid)
    }

    fn attribute(project_root: &Path, oid: Oid) -> CommitClaims {
        let project = ProjectRepository::discover(project_root).unwrap();
        let commit = project.repository.find_commit(oid).unwrap();
        Attributor::new(&project, project_root, "specs")
            .claims(&commit)
            .unwrap()
    }

    #[test]
    fn a_directory_touch_claims_with_no_subject_scope() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let claims = claims_for(
            tmp.path(),
            &repository,
            "specs/012-a/plan.md",
            "chore: tidy",
        );
        assert_eq!(claims.claimants, vec![seq(12)]);
        assert_eq!(claims.first_named, None);
        assert_eq!(claims.paths, vec!["specs/012-a/plan.md".to_string()]);
    }

    #[test]
    fn a_subject_scope_claims_with_no_directory_touch() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let claims = claims_for(tmp.path(), &repository, "src/lib.rs", "fix(012): a bug");
        assert_eq!(claims.claimants, vec![seq(12)]);
        assert_eq!(claims.first_named, Some(seq(12)));
        assert_eq!(claims.paths, vec!["src/lib.rs".to_string()]);
    }

    #[test]
    fn a_scope_and_a_directory_touch_each_claim_and_the_named_spec_stays_first() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let claims = claims_for(
            tmp.path(),
            &repository,
            "specs/034-b/plan.md",
            "fix(012): a bug",
        );
        assert_eq!(claims.claimants, vec![seq(12), seq(34)]);
        assert_eq!(claims.first_named, Some(seq(12)));
        assert!(claims.is_claimed_by(&seq(34)));
        assert!(!claims.is_claimed_by(&seq(56)));
    }

    #[test]
    fn a_spec_both_named_and_touched_is_claimed_once() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let claims = claims_for(
            tmp.path(),
            &repository,
            "specs/012-a/plan.md",
            "fix(012): a bug",
        );
        assert_eq!(claims.claimants, vec![seq(12)]);
    }

    #[test]
    fn a_subject_naming_several_specs_keeps_the_first_named() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let claims = claims_for(tmp.path(), &repository, "src/lib.rs", "fix(034, 012): both");
        assert_eq!(claims.claimants, vec![seq(34), seq(12)]);
        assert_eq!(claims.first_named, Some(seq(34)));
    }

    #[test]
    fn a_number_no_spec_has_claims_nothing_and_the_commit_counts_whole() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let claims = claims_for(tmp.path(), &repository, "src/lib.rs", "fix(999): a bug");
        assert_eq!(claims.claimants, Vec::<FeatureForm>::new());
        assert_eq!(claims.first_named, None);
        assert_eq!(claims.paths, vec!["src/lib.rs".to_string()]);
    }

    fn belongs(root: &Path, oid: Oid, form: &FeatureForm) -> bool {
        let project = ProjectRepository::discover(root).unwrap();
        let commit = project.repository.find_commit(oid).unwrap();
        Attributor::new(&project, root, "specs")
            .belongs_to(&commit, form)
            .unwrap()
    }

    /// A subject's number claims nothing for a spec gone from disk, and a caller naming
    /// that spec from history still gets its answer, by the subject or the directory.
    #[test]
    fn a_commit_belongs_to_a_spec_gone_from_disk_by_its_subject_or_its_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let form = FeatureForm::BranchScoped {
            identifier: "1234".to_string(),
            n: 1,
        };
        write(&tmp.path().join("specs/1234.1-foo/spec.md"), "s\n");
        let touching = commit_all(&repository, "feat: naming nothing");
        write(&tmp.path().join("src/named.rs"), "n\n");
        let named = commit_all(&repository, "feat(1234.1): touching nothing of it");
        write(&tmp.path().join("src/other.rs"), "o\n");
        let neither = commit_all(&repository, "feat(012): another spec's");
        std::fs::remove_dir_all(tmp.path().join("specs/1234.1-foo")).unwrap();
        let removal = commit_all(&repository, "chore: naming nothing either");

        assert!(belongs(tmp.path(), touching, &form));
        assert!(belongs(tmp.path(), named, &form));
        assert!(!belongs(tmp.path(), neither, &form));
        assert!(belongs(tmp.path(), removal, &form));
    }

    #[test]
    fn a_retired_specs_directory_still_claims_though_its_number_alone_would_not() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        write(&tmp.path().join("specs/056-gone/spec.md"), "gone\n");
        let oid = commit_all(&repository, "fix(056): before it was retired");
        std::fs::remove_dir_all(tmp.path().join("specs/056-gone")).unwrap();

        let claims = attribute(tmp.path(), oid);
        assert_eq!(claims.claimants, vec![seq(56)]);
        assert_eq!(claims.first_named, Some(seq(56)));
    }

    #[test]
    fn a_renamed_slug_is_the_same_spec() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let claims = claims_for(
            tmp.path(),
            &repository,
            "specs/012-old-name/notes.md",
            "fix(012): under its old name",
        );
        assert_eq!(claims.claimants, vec![seq(12)]);
    }

    #[test]
    fn a_spec_root_file_or_sibling_directory_claims_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        write(&tmp.path().join("specs/inbox.md"), "- a todo\n");
        write(&tmp.path().join("specs/rules/a-cross.md"), "rule\n");
        write(&tmp.path().join("specs/templates/plan.md"), "plan\n");
        let oid = commit_all(&repository, "chore: corpus housekeeping");
        let claims = attribute(tmp.path(), oid);
        assert_eq!(claims.claimants, Vec::<FeatureForm>::new());
        assert_eq!(
            claims.paths,
            vec![
                "specs/inbox.md".to_string(),
                "specs/rules/a-cross.md".to_string(),
                "specs/templates/plan.md".to_string(),
            ]
        );
    }

    /// Commit the work tree on the current `HEAD` with `extra` as a second
    /// parent.
    fn merge_commit(repository: &Repository, extra: Oid, subject: &str) -> Oid {
        let mut index = repository.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let head = repository.head().unwrap().peel_to_commit().unwrap();
        let other = repository.find_commit(extra).unwrap();
        let signature = Signature::now("Test", "test@example.com").unwrap();
        repository
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                subject,
                &tree,
                &[&head, &other],
            )
            .unwrap()
    }

    /// A branch changing spec 012, and `main` changing spec 034, ready to merge
    /// with the branch's file in the work tree. Returns the branch's commit.
    fn diverged_branch(root: &Path, repository: &Repository) -> Oid {
        let main_tip = repository.head().unwrap().target().unwrap();
        let main_ref = repository.head().unwrap().name().unwrap().to_string();

        write(&root.join("specs/012-a/plan.md"), "branch\n");
        let branch_tip = commit_all(repository, "feat(012): on a branch");

        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.force().remove_untracked(true);
        repository.set_head_detached(main_tip).unwrap();
        repository.checkout_head(Some(&mut checkout)).unwrap();
        repository
            .reference(&main_ref, main_tip, true, "rewind main")
            .unwrap();
        repository.set_head(&main_ref).unwrap();

        write(&root.join("specs/034-b/plan.md"), "main\n");
        commit_all(repository, "feat(034): on main");
        write(&root.join("specs/012-a/plan.md"), "branch\n");
        branch_tip
    }

    #[test]
    fn a_clean_merge_claims_nothing_and_contributes_no_path() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let branch = diverged_branch(tmp.path(), &repository);
        let merge = merge_commit(&repository, branch, "fix(012): merge the branch");

        let claims = attribute(tmp.path(), merge);
        assert_eq!(claims, CommitClaims::default());
    }

    #[test]
    fn a_merge_contributes_only_what_it_changed_itself() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        let branch = diverged_branch(tmp.path(), &repository);
        write(&tmp.path().join("src/evil.rs"), "only in the merge\n");
        let merge = merge_commit(&repository, branch, "Merge branch 'story'");

        let claims = attribute(tmp.path(), merge);
        assert_eq!(claims.claimants, Vec::<FeatureForm>::new());
        assert_eq!(claims.paths, vec!["src/evil.rs".to_string()]);
    }

    #[test]
    fn a_root_commit_diffs_against_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/012-a/spec.md"), "a\n");
        write(&tmp.path().join("README.md"), "r\n");
        let oid = commit_all(&repository, "chore: first");

        let claims = attribute(tmp.path(), oid);
        assert_eq!(claims.claimants, vec![seq(12)]);
        assert_eq!(
            claims.paths,
            vec!["README.md".to_string(), "specs/012-a/spec.md".to_string()]
        );
    }

    #[test]
    fn a_branch_scoped_spec_claims_by_directory_and_by_scope() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = project_with_two_specs(tmp.path());
        write(&tmp.path().join("specs/1234.1-story/spec.md"), "s\n");
        commit_all(&repository, "chore: stage");
        let form = FeatureForm::BranchScoped {
            identifier: "1234".into(),
            n: 1,
        };

        let by_directory = claims_for(
            tmp.path(),
            &repository,
            "specs/1234.1-story/plan.md",
            "chore: tidy",
        );
        assert_eq!(by_directory.claimants, vec![form.clone()]);
        assert_eq!(by_directory.first_named, None);

        let by_scope = claims_for(tmp.path(), &repository, "src/lib.rs", "feat(1234.1): work");
        assert_eq!(by_scope.claimants, vec![form.clone()]);
        assert_eq!(by_scope.first_named, Some(form));
    }

    #[test]
    fn a_project_in_a_subdirectory_claims_only_its_own_spec_directories() {
        let tmp = tempfile::tempdir().unwrap();
        let (repository, project) = subdirectory_project(tmp.path());
        write(&project.join("specs/012-a/spec.md"), "a\n");
        write(&tmp.path().join("other/specs/034-b/spec.md"), "b\n");
        let oid = commit_all(&repository, "chore: both projects");

        let claims = attribute(&project, oid);
        assert_eq!(claims.claimants, vec![seq(12)]);
        assert_eq!(
            claims.paths,
            vec![
                ".ductus/config.toml".to_string(),
                "specs/012-a/spec.md".to_string()
            ]
        );
    }

    #[test]
    fn a_scope_token_is_a_number_a_branch_id_or_a_directory_and_nothing_else() {
        let numbers = subject_scope_forms;
        let none: Vec<FeatureForm> = Vec::new();
        assert_eq!(numbers("fix(022, 066): x"), vec![seq(22), seq(66)]);
        assert_eq!(numbers("docs(agents, 063): x"), vec![seq(63)]);
        assert_eq!(numbers("fix(022 analyze): x"), vec![seq(22)]);
        assert_eq!(numbers("feat(022)!: x"), vec![seq(22)]);
        assert_eq!(numbers("impl(077) task 8 round 6"), vec![seq(77)]);
        assert_eq!(numbers("review+analyze(059): x"), vec![seq(59)]);
        assert_eq!(
            numbers("review(058, 013), analyze(058, 013): x"),
            vec![seq(58), seq(13)]
        );
        assert_eq!(numbers("fix(022-deterministic-runtime): x"), vec![seq(22)]);
        assert_eq!(numbers("fix(1000): x"), vec![seq(1000)]);

        // A scope with no spec in it, or a number outside the scope.
        assert_eq!(numbers("chore: no scope, 022"), none);
        assert_eq!(numbers("docs(agents): x"), none);
        assert_eq!(numbers("fix(runtime): 022"), none);
        // Too short to be a spec number, as parse_feature_dir has it.
        assert_eq!(numbers("fix(22): x"), none);
        assert_eq!(numbers("fix(0079): x"), none);
        // Only a subject's leading word can carry a scope, and a later clause
        // does not add to it.
        assert_eq!(numbers("Merge branch 'story' (022): x"), none);
        assert_eq!(numbers("Revert \"fix(022): x\""), none);
        assert_eq!(numbers("(022): x"), none);
        assert_eq!(numbers("agents: rules; tasks(022): x"), none);
        assert_eq!(
            numbers("impl(013) task 25, impl(058) task 2"),
            vec![seq(13)]
        );
    }
}
