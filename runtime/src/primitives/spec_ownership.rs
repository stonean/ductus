//! Which files belong to a spec (spec 086): each one its plan's Affected Files
//! name, by the match a review's scope applies to a plan entry
//! ([`PlanEntries::names`]), and each one a commit attributed to it changed, by
//! the per-path rule a review's window applies ([`SpecShare::split`]). This is
//! the only answer to that question: a review weighing a stored entry that did
//! not fire, `prune-review` and a first review's scope all read it, so no two of
//! them can disagree.
//!
//! One walk of `HEAD`'s history answers for any number of specs. A commit is
//! diffed only when [`Attributor::may_claim`] names a spec asked about. That test
//! names every spec [`Attributor::claims`] would, without a diff, so a walk over
//! one spec gives it what a walk over every spec does. [`Attributor::belongs_to`]
//! would not: it compares the spec directory's tree id, which a commit that only
//! renames the directory leaves unchanged.
//!
//! A history the walk cannot read whole, as in a shallow clone, and a directory
//! [`parse_feature_dir`] does not recognize are not examined, and never read as
//! owning nothing (`QUAL-CLAIM-001`).
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "spec 086's tasks 3 to 5 call it; once they do the expectation fails, and \
                  this attribute goes in that change"
    )
)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use git2::Oid;

use crate::primitives::check_stuck::{TreeListings, history_of};
use crate::primitives::commit_attribution::{Attributor, SpecShare};
use crate::primitives::compute_review_scope::PlanEntries;
use crate::primitives::{FeatureForm, ProjectRepository, Result, parse_feature_dir};

/// The files one spec owns.
pub(crate) struct Owned {
    /// The spec's plan's Affected Files entries, read from disk.
    plan: PlanEntries,
    /// Each path a commit attributed to the spec gave it, from the project root,
    /// whether or not it is still on disk.
    changed: BTreeSet<String>,
}

impl Owned {
    /// Whether `path`, from the project root, belongs to the spec: a plan entry
    /// names it, or a commit attributed to the spec changed it.
    pub(crate) fn belongs(&self, path: &str) -> bool {
        self.plan.names(path) || self.changed.contains(path)
    }

    /// Each path a commit attributed to the spec changed, sorted, whether or not
    /// it is still on disk.
    pub(crate) fn changed(&self) -> &BTreeSet<String> {
        &self.changed
    }
}

/// Whether a spec's files could be told, and which they are.
pub(crate) enum Ownership {
    /// The plan and the whole of `HEAD`'s history were read.
    Examined(Owned),
    /// Nothing is known to belong, so no file may be judged not to.
    NotExamined {
        /// Why, worded for the operator.
        reason: String,
    },
}

/// What the spec directory `feature`, under `specs_root`, owns: [`of_specs`] for
/// one spec.
///
/// # Errors
///
/// As [`of_specs`].
pub(crate) fn of_spec(
    project: &ProjectRepository,
    repo: &Path,
    specs_root: &str,
    feature: &str,
) -> Result<Ownership> {
    let Some(form) = parse_feature_dir(feature) else {
        return Ok(unrecognized(feature));
    };
    let _listings = TreeListings::open();
    let attributor = Attributor::new(project, repo, specs_root)?;
    let asked = [(feature, form, attributor.plan_of(feature)?)];
    let mut changed = [BTreeSet::new()];
    if let Some(reason) = walk(project, &attributor, &asked, &mut changed)? {
        return Ok(Ownership::NotExamined { reason });
    }
    let [(_, _, plan)] = asked;
    let [changed] = changed;
    Ok(Ownership::Examined(Owned { plan, changed }))
}

/// What each spec directory in `features`, under `specs_root`, owns, keyed by its
/// name, from one walk of `HEAD`'s history. A spec with no commit yet owns its
/// plan's files alone, and so does every spec in a repository with no commit.
///
/// # Errors
///
/// [`crate::primitives::PrimitiveError::InvalidPath`] when the spec root runs
/// through a symbolic link leaving the project, as [`Attributor::new`] refuses it;
/// [`crate::primitives::PrimitiveError::Io`] when a plan exists and cannot be
/// read; and [`crate::primitives::PrimitiveError::Git`] for any libgit2 failure,
/// so a commit that could not be attributed never reads as one that gave nothing.
pub(crate) fn of_specs(
    project: &ProjectRepository,
    repo: &Path,
    specs_root: &str,
    features: &[&str],
) -> Result<BTreeMap<String, Ownership>> {
    let _listings = TreeListings::open();
    let attributor = Attributor::new(project, repo, specs_root)?;
    let mut owners = BTreeMap::new();
    let mut asked: Vec<(&str, FeatureForm, PlanEntries)> = Vec::new();
    for &feature in features {
        match parse_feature_dir(feature) {
            Some(form) => asked.push((feature, form, attributor.plan_of(feature)?)),
            None => {
                owners.insert(feature.to_string(), unrecognized(feature));
            }
        }
    }
    if asked.is_empty() {
        return Ok(owners);
    }
    let mut changed = vec![BTreeSet::new(); asked.len()];
    let cut = walk(project, &attributor, &asked, &mut changed)?;
    for ((feature, _, plan), changed) in asked.into_iter().zip(changed) {
        let ownership = match &cut {
            Some(reason) => Ownership::NotExamined {
                reason: reason.clone(),
            },
            None => Ownership::Examined(Owned { plan, changed }),
        };
        owners.insert(feature.to_string(), ownership);
    }
    Ok(owners)
}

/// A directory [`parse_feature_dir`] does not recognize, which no commit can be
/// attributed to.
fn unrecognized(feature: &str) -> Ownership {
    Ownership::NotExamined {
        reason: format!(
            "`{feature}` is no feature directory's name, so no commit can be attributed to it"
        ),
    }
}

/// Walk every commit `HEAD` reaches once, adding to `changed`, one set per asked
/// spec in the order asked, the paths [`SpecShare::split`] gives that spec of each
/// commit it claims. Returns why the history could not be read whole, when it could
/// not: a repository with no commit has nothing to read and is whole.
fn walk(
    project: &ProjectRepository,
    attributor: &Attributor<'_>,
    asked: &[(&str, FeatureForm, PlanEntries)],
    changed: &mut [BTreeSet<String>],
) -> Result<Option<String>> {
    let repository = &project.repository;
    let head = match repository.head() {
        Ok(head) => head.peel_to_commit()?.id(),
        Err(err) if err.code() == git2::ErrorCode::UnbornBranch => return Ok(None),
        Err(err) => return Err(err.into()),
    };
    let commits = history_of(repository, head)?;
    // A shallow clone's boundary commits read as parentless, the cut `check-stuck`
    // reads, so what lay before one is unknown.
    if repository.is_shallow() {
        for &oid in commits.as_slice() {
            if repository.find_commit(oid)?.parent_count() == 0 {
                return Ok(Some(format!(
                    "the history stops at `{}`, where a shallow clone is cut: \
                     `git fetch --unshallow` reads it whole",
                    short(oid)
                )));
            }
        }
    }
    let mut shares: Vec<SpecShare<'_>> = asked
        .iter()
        .map(|(feature, form, plan)| SpecShare::new(attributor, form, feature, plan))
        .collect();
    for &oid in commits.as_slice() {
        let commit = repository.find_commit(oid)?;
        let may = attributor.may_claim(&commit)?;
        if !asked.iter().any(|(_, form, _)| may.contains(form)) {
            continue;
        }
        let claims = attributor.claims(&commit)?;
        for (index, (_, form, _)) in asked.iter().enumerate() {
            if claims.is_claimed_by(form) {
                let (own, _) = shares[index].split(claims.clone())?;
                changed[index].extend(own);
            }
        }
    }
    Ok(None)
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
    use git2::{Repository, Signature};

    const PLAN: &str = "## Affected Files\n\n| File | Action | Purpose |\n| --- | --- | --- |\n\
                        | `src/planned.rs` | Modify | p |\n| `docs/*.md` | Modify | p |\n";

    /// A repository at the project root with specs 012 and 034 on disk, 034's plan
    /// naming `src/theirs.rs`.
    fn two_specs(root: &Path) -> Repository {
        let repository = Repository::init(root).unwrap();
        write(&root.join("specs/012-a/spec.md"), "a\n");
        write(&root.join("specs/034-b/spec.md"), "b\n");
        write(
            &root.join("specs/034-b/plan.md"),
            "## Affected Files\n\n| File | Action | Purpose |\n| --- | --- | --- |\n\
             | `src/theirs.rs` | Modify | p |\n",
        );
        commit_all(&repository, "chore: seed");
        repository
    }

    fn change(root: &Path, repository: &Repository, file: &str, subject: &str) -> Oid {
        write(&root.join(file), &format!("{subject}\n"));
        commit_all(repository, subject)
    }

    fn owned(root: &Path, feature: &str) -> Owned {
        let project = ProjectRepository::discover(root).unwrap();
        match of_spec(&project, root, "specs", feature).unwrap() {
            Ownership::Examined(owned) => owned,
            Ownership::NotExamined { reason } => panic!("not examined: {reason}"),
        }
    }

    fn not_examined(root: &Path, feature: &str) -> String {
        let project = ProjectRepository::discover(root).unwrap();
        match of_spec(&project, root, "specs", feature).unwrap() {
            Ownership::Examined(owned) => panic!("examined: {:?}", owned.changed()),
            Ownership::NotExamined { reason } => reason,
        }
    }

    /// A file a plan entry names belongs, as a literal path and by a pattern, in
    /// a repository with no commit, where the plan's files are all the spec owns.
    #[test]
    fn a_file_a_plan_entry_names_belongs() {
        let tmp = tempfile::tempdir().unwrap();
        Repository::init(tmp.path()).unwrap();
        write(&tmp.path().join("specs/012-a/spec.md"), "a\n");
        write(&tmp.path().join("specs/012-a/plan.md"), PLAN);
        let owned = owned(tmp.path(), "012-a");
        assert!(owned.belongs("src/planned.rs"));
        assert!(owned.belongs("docs/guide.md"));
        assert!(!owned.belongs("src/other.rs"));
        assert!(owned.changed().is_empty());
    }

    /// A file a commit attributed to the spec changed belongs: by its subject's
    /// scope, and by touching its directory alone.
    #[test]
    fn a_file_a_commit_attributed_to_the_spec_changed_belongs() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = two_specs(tmp.path());
        change(tmp.path(), &repository, "src/named.rs", "fix(012): named");
        write(&tmp.path().join("specs/012-a/notes.md"), "n\n");
        change(
            tmp.path(),
            &repository,
            "src/touched.rs",
            "touches 012 alone",
        );
        let owned = owned(tmp.path(), "012-a");
        assert!(owned.belongs("src/named.rs"), "{:?}", owned.changed());
        assert!(owned.belongs("src/touched.rs"), "{:?}", owned.changed());
        assert!(owned.belongs("specs/012-a/notes.md"));
    }

    /// A file only an unclaimed commit changed does not belong, nor one only
    /// another spec's share of a commit both specs claim.
    #[test]
    fn a_file_only_an_unclaimed_commit_or_another_specs_share_changed_does_not_belong() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = two_specs(tmp.path());
        change(tmp.path(), &repository, "src/tidy.rs", "chore: tidy");
        change(tmp.path(), &repository, "src/other.rs", "fix(034): theirs");
        write(&tmp.path().join("specs/012-a/notes.md"), "n\n");
        write(&tmp.path().join("specs/034-b/notes.md"), "n\n");
        write(&tmp.path().join("src/unplanned.rs"), "u\n");
        change(tmp.path(), &repository, "src/theirs.rs", "touches both");
        let owned = owned(tmp.path(), "012-a");
        assert!(!owned.belongs("src/tidy.rs"), "an unclaimed commit's");
        assert!(!owned.belongs("src/other.rs"), "a commit naming 034");
        assert!(!owned.belongs("src/theirs.rs"), "034's plan's share");
        assert!(
            !owned.belongs("specs/034-b/notes.md"),
            "034's directory's share"
        );
        assert!(owned.belongs("src/unplanned.rs"), "{:?}", owned.changed());
        assert!(owned.belongs("specs/012-a/notes.md"));
    }

    /// A merge changing the spec's directory and a file itself gives the spec
    /// neither, since a merge claims nothing; the branch's own commit still gives
    /// it its paths.
    #[test]
    fn a_merge_touching_the_specs_directory_gives_it_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = two_specs(tmp.path());
        let main_tip = repository.head().unwrap().target().unwrap();
        let main_ref = repository.head().unwrap().name().unwrap().to_string();
        let branch = change(
            tmp.path(),
            &repository,
            "src/branch.rs",
            "feat(012): branch",
        );
        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.force().remove_untracked(true);
        repository.set_head_detached(main_tip).unwrap();
        repository.checkout_head(Some(&mut checkout)).unwrap();
        repository
            .reference(&main_ref, main_tip, true, "rewind main")
            .unwrap();
        repository.set_head(&main_ref).unwrap();
        change(tmp.path(), &repository, "src/main.rs", "feat(034): main");
        write(&tmp.path().join("src/branch.rs"), "feat(012): branch\n");
        write(&tmp.path().join("specs/012-a/merged.md"), "m\n");
        write(&tmp.path().join("src/merged.rs"), "m\n");
        let mut index = repository.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let head = repository.head().unwrap().peel_to_commit().unwrap();
        let other = repository.find_commit(branch).unwrap();
        let signature = Signature::now("Test", "test@example.com").unwrap();
        repository
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                "merge(012): the branch",
                &tree,
                &[&head, &other],
            )
            .unwrap();
        let owned = owned(tmp.path(), "012-a");
        assert!(!owned.belongs("src/merged.rs"), "{:?}", owned.changed());
        assert!(!owned.belongs("specs/012-a/merged.md"));
        assert!(owned.belongs("src/branch.rs"), "{:?}", owned.changed());
    }

    /// A commit that only renames the spec's directory gives the spec the paths it
    /// changed, as a walk over every spec does: the directory's tree id is the
    /// same on both sides, which is why the walk asks `may_claim`.
    #[test]
    fn a_commit_only_renaming_the_specs_directory_gives_it_the_new_paths() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = two_specs(tmp.path());
        std::fs::rename(
            tmp.path().join("specs/012-a"),
            tmp.path().join("specs/012-renamed"),
        )
        .unwrap();
        let mut index = repository.index().unwrap();
        index.remove_dir(Path::new("specs/012-a"), 0).unwrap();
        index.write().unwrap();
        commit_all(&repository, "renames a spec");
        let owned = owned(tmp.path(), "012-renamed");
        assert!(
            owned.belongs("specs/012-renamed/spec.md"),
            "{:?}",
            owned.changed()
        );
    }

    /// A shallow history and a directory that is no feature's are not examined,
    /// each with its reason, never read as owning nothing.
    #[test]
    fn a_shallow_history_and_an_unrecognized_directory_are_not_examined() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = two_specs(tmp.path());
        let cut = change(tmp.path(), &repository, "src/a.rs", "fix(012): a");
        change(tmp.path(), &repository, "src/b.rs", "fix(012): b");
        assert!(owned(tmp.path(), "012-a").belongs("src/b.rs"));
        std::fs::write(tmp.path().join(".git/shallow"), format!("{cut}\n")).unwrap();
        let reason = not_examined(tmp.path(), "012-a");
        assert!(reason.contains("--unshallow"), "{reason}");
        assert!(reason.contains(&short(cut)), "{reason}");
        std::fs::remove_file(tmp.path().join(".git/shallow")).unwrap();
        write(&tmp.path().join("specs/notes/spec.md"), "n\n");
        let reason = not_examined(tmp.path(), "notes");
        assert!(reason.contains("`notes`"), "{reason}");
    }

    /// One walk over two specs gives each the answer a walk over it alone gives,
    /// with a commit both claim divided between them.
    #[test]
    fn one_walk_over_two_specs_gives_each_its_own_answer() {
        let tmp = tempfile::tempdir().unwrap();
        let repository = two_specs(tmp.path());
        change(tmp.path(), &repository, "src/mine.rs", "fix(012): mine");
        change(tmp.path(), &repository, "src/other.rs", "fix(034): theirs");
        change(
            tmp.path(),
            &repository,
            "src/both.rs",
            "fix(012, 034): both",
        );
        write(&tmp.path().join("specs/012-a/notes.md"), "n\n");
        write(&tmp.path().join("specs/034-b/notes.md"), "n\n");
        change(tmp.path(), &repository, "src/theirs.rs", "touches both");
        let project = ProjectRepository::discover(tmp.path()).unwrap();
        let both = of_specs(&project, tmp.path(), "specs", &["012-a", "034-b"]).unwrap();
        assert_eq!(both.len(), 2);
        for (feature, ownership) in both {
            let Ownership::Examined(together) = ownership else {
                panic!("{feature} not examined");
            };
            let alone = owned(tmp.path(), &feature);
            assert_eq!(together.changed(), alone.changed(), "{feature}");
            assert!(together.belongs("src/both.rs"), "{feature}");
        }
        assert!(!owned(tmp.path(), "012-a").belongs("src/theirs.rs"));
        assert!(owned(tmp.path(), "034-b").belongs("src/theirs.rs"));
        assert!(!owned(tmp.path(), "034-b").belongs("src/mine.rs"));
    }
}
