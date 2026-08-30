//! `retire-feature` — remove a feature directory whose content now lives
//! elsewhere: a branch-scoped directory once it has been folded into the
//! upstream spec that receives it, or a sequential one an operator has
//! explicitly consolidated away (spec 052).
//!
//! The last step of a fold, and the only irreversible one. Everything before
//! it — applying the content, re-pointing links, reopening the upstream spec —
//! is a write another write can undo; this deletes a directory.
//!
//! **The sequential refusal is gated, never removed** (spec 052). It exists
//! because an irreversible operation must not be reachable from a typo, and
//! deleting it would discard that protection for every caller. Instead the
//! caller opts in: `/{project}:fold` never does, so a mistyped feature name
//! on a fold meets the refusal exactly as before, while consolidation opts in
//! having already named both specs and confirmed the removal. The flag
//! records a second explicit decision rather than weakening the first — and
//! the anti-stranding guard below is not gated at all, because that guarantee
//! is owed to both callers equally.
//!
//! **This is where the fold target's existence is finally enforced** (AC28).
//! Nothing earlier can enforce it and the reason is structural: a
//! branch-scoped spec exists *because* upstream diverged, so its target
//! normally lives on the upstream branch, sometimes created there after this
//! branch forked. `create-feature` therefore checks `fold-into` for shape
//! only, `validate-frontmatter` reports an unresolvable target as no finding
//! at all, and `check-unfolded-specs` calls it out without blocking. Fold-back
//! runs *after* the merge, on the upstream branch — the first tree in which
//! both specs exist, and so the first moment the question has an answer worth
//! refusing on.
//!
//! Defined by `specs/051-branch-scoped-spec-numbering/spec.md` (AC28).

use std::path::Path;

use crate::primitives::{
    FeatureForm, PrimitiveError, Result, parse_feature_dir, validate_no_traversal,
};
use crate::schema::paths;
use crate::schema::primitives::{RetireFeatureArgs, RetireFeatureResult};

/// Execute the `retire-feature` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when either argument is empty,
/// absolute, or carries a parent-directory component;
/// [`PrimitiveError::InvalidArgument`] when `feature` is not the
/// branch-scoped form; [`PrimitiveError::FeatureNotFound`] when
/// `fold-target` names no feature directory holding a `spec.md`; or
/// [`PrimitiveError::Io`] when the removal itself fails.
///
/// A `feature` directory that is already absent is **not** an error — it is
/// `retired: false`, so re-running an interrupted fold converges rather than
/// halting.
pub fn run(args: &RetireFeatureArgs, repo: &Path) -> Result<RetireFeatureResult> {
    validate_no_traversal(&args.feature)?;
    validate_no_traversal(&args.fold_target)?;

    // Refusal 1: the sequential form is permanent *unless the caller says
    // otherwise*. Checked before anything touches the filesystem, so a typo
    // naming a real spec cannot reach the removal below.
    //
    // Gated rather than deleted (spec 052). The refusal exists because an
    // irreversible operation must not be reachable from a typo, and that is
    // still true for every caller that does not opt in: `/{project}:fold`
    // never sets the flag, so a mistyped feature name on a fold meets this
    // refusal exactly as it did before. Consolidation opts in having already
    // named both specs and confirmed the removal, so the flag records a
    // second explicit decision rather than relaxing the first.
    //
    // Refusal 2 below is deliberately *not* gated: it is what stops a
    // retirement stranding content, and that guarantee is owed to both
    // callers equally.
    if !args.allow_sequential
        && !matches!(
            parse_feature_dir(&args.feature),
            Some(FeatureForm::BranchScoped { .. })
        )
    {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "retire-feature".into(),
            argument: "feature".into(),
            reason: format!(
                "{:?} is not a branch-scoped feature (<identifier>.<n>-<slug>); only the \
                 staging form is retired, and a sequential spec is completed rather than \
                 removed",
                args.feature
            ),
        });
    }

    let layout = paths::Paths::load(repo);
    let specs_dir = repo.join(&layout.specs_root);

    // Refusal 2: the content must have somewhere to have landed. A directory
    // with no `spec.md` is not a home — accepting one would satisfy the
    // letter of "the target exists" while defeating the only thing the check
    // is for.
    //
    // The target's *form* is deliberately not re-checked here.
    // `validate-frontmatter` owns the rule that a fold target is sequential,
    // and a second copy of it is the drift this feature is otherwise
    // avoiding.
    if !specs_dir.join(&args.fold_target).join("spec.md").is_file() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: args.fold_target.clone(),
        });
    }

    let feature_dir = specs_dir.join(&args.feature);
    let path = format!("{}/{}", layout.specs_root, args.feature);
    if !feature_dir.is_dir() {
        // Already gone: a re-run of an interrupted fold, not a failure. The
        // per-spec atomicity fold-back promises is what makes this the right
        // reading — each spec's writes complete before its retirement, so a
        // missing directory means that spec's fold already finished.
        return Ok(RetireFeatureResult {
            retired: false,
            path,
        });
    }

    std::fs::remove_dir_all(&feature_dir).map_err(|source| PrimitiveError::Io {
        path: feature_dir,
        source,
    })?;

    Ok(RetireFeatureResult {
        retired: true,
        path,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::run;
    use crate::primitives::PrimitiveError;
    use crate::schema::primitives::RetireFeatureArgs;
    use std::fs;
    use std::path::Path;

    fn args(feature: &str, fold_target: &str) -> RetireFeatureArgs {
        RetireFeatureArgs {
            feature: feature.into(),
            fold_target: fold_target.into(),
            allow_sequential: false,
        }
    }

    /// The consolidation call shape: the same arguments with the opt-in set.
    fn consolidating(feature: &str, into: &str) -> RetireFeatureArgs {
        RetireFeatureArgs {
            feature: feature.into(),
            fold_target: into.into(),
            allow_sequential: true,
        }
    }

    fn write_spec(repo: &Path, feature: &str) {
        let dir = repo.join("specs").join(feature);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\nstatus: draft\ndependencies: []\n---\n\n# {feature}\n"),
        )
        .unwrap();
    }

    #[test]
    fn a_folded_branch_scoped_directory_is_removed() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha");
        write_spec(tmp.path(), "1234.1-staged");
        fs::create_dir_all(tmp.path().join("specs/1234.1-staged/scenarios")).unwrap();
        fs::write(
            tmp.path().join("specs/1234.1-staged/scenarios/inner.md"),
            "---\nsection: X\n---\n",
        )
        .unwrap();

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert!(result.retired);
        assert_eq!(result.path, "specs/1234.1-staged");
        assert!(!tmp.path().join("specs/1234.1-staged").exists());
        // The upstream spec is untouched.
        assert!(tmp.path().join("specs/050-alpha/spec.md").is_file());
    }

    /// AC28: a target that does not exist refuses and leaves the
    /// branch-scoped spec in place, so nothing is ever stranded.
    #[test]
    fn an_absent_fold_target_refuses_and_leaves_the_spec_in_place() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "1234.1-staged");

        let err = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap_err();

        assert!(
            matches!(err, PrimitiveError::FeatureNotFound { ref feature, .. } if feature == "050-alpha"),
            "unexpected error: {err:?}"
        );
        assert!(tmp.path().join("specs/1234.1-staged/spec.md").is_file());
    }

    /// A directory with no `spec.md` is not a home content can have landed
    /// in, so it does not satisfy the check.
    #[test]
    fn a_target_directory_without_a_spec_is_not_a_home() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "1234.1-staged");
        fs::create_dir_all(tmp.path().join("specs/050-alpha")).unwrap();

        let err = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap_err();

        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
        assert!(tmp.path().join("specs/1234.1-staged/spec.md").is_file());
    }

    /// The sequential form is permanent; this primitive can never remove one.
    #[test]
    fn a_sequential_feature_is_refused_outright() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha");
        write_spec(tmp.path(), "060-beta");

        let err = run(&args("060-beta", "050-alpha"), tmp.path()).unwrap_err();

        assert!(
            matches!(
                err,
                PrimitiveError::InvalidArgument { ref argument, .. } if argument == "feature"
            ),
            "unexpected error: {err:?}"
        );
        assert!(tmp.path().join("specs/060-beta/spec.md").is_file());
    }

    /// The refusal is decided before the filesystem is touched, so a valid
    /// fold target cannot make a sequential feature removable.
    #[test]
    fn the_sequential_refusal_outranks_every_other_condition() {
        let tmp = tempfile::tempdir().unwrap();
        // No fold target at all: the sequential refusal still wins, naming
        // the argument the caller actually got wrong.
        let err = run(&args("060-beta", "050-nowhere"), tmp.path()).unwrap_err();

        assert!(matches!(err, PrimitiveError::InvalidArgument { .. }));
    }

    #[test]
    fn an_already_absent_directory_is_a_domain_outcome_not_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha");

        let result = run(&args("1234.1-staged", "050-alpha"), tmp.path()).unwrap();

        assert!(!result.retired);
        assert_eq!(result.path, "specs/1234.1-staged");
    }

    /// A second call after a successful one converges rather than halting,
    /// which is what lets an interrupted fold be re-run.
    #[test]
    fn retiring_twice_converges() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha");
        write_spec(tmp.path(), "1234.1-staged");

        assert!(
            run(&args("1234.1-staged", "050-alpha"), tmp.path())
                .unwrap()
                .retired
        );
        assert!(
            !run(&args("1234.1-staged", "050-alpha"), tmp.path())
                .unwrap()
                .retired
        );
    }

    #[test]
    fn a_traversing_feature_name_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha");

        let err = run(&args("../../etc", "050-alpha"), tmp.path()).unwrap_err();

        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }

    #[test]
    fn a_sequential_feature_is_removed_when_the_caller_opts_in() {
        // Consolidation's call shape. The directory goes, and the target it
        // was consolidated into is untouched.
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "012-overlapping");
        write_spec(tmp.path(), "030-the-survivor");

        let result = run(
            &consolidating("012-overlapping", "030-the-survivor"),
            tmp.path(),
        )
        .unwrap();
        assert!(result.retired);
        assert!(!tmp.path().join("specs/012-overlapping").exists());
        assert!(
            tmp.path().join("specs/030-the-survivor/spec.md").is_file(),
            "consolidation writes nothing to the target"
        );
    }

    #[test]
    fn the_opt_in_does_not_relax_the_anti_stranding_guard() {
        // Refusal 2 is deliberately ungated: opting in to removing a
        // sequential directory is not opting in to stranding its content.
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "012-overlapping");
        fs::create_dir_all(tmp.path().join("specs/030-not-a-home")).unwrap();

        let err = run(
            &consolidating("012-overlapping", "030-not-a-home"),
            tmp.path(),
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
        assert!(
            tmp.path().join("specs/012-overlapping").exists(),
            "a refused consolidation leaves the source in place"
        );
    }

    #[test]
    fn the_fold_call_shape_is_unchanged_by_the_gate() {
        // The load-bearing assertion of the gate: every existing caller
        // behaves exactly as it did. `/{project}:fold` never sets the flag,
        // so a sequential name — the typo case the refusal exists for —
        // still refuses, and a genuine branch-scoped fold still succeeds.
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-upstream");
        write_spec(tmp.path(), "012-sequential");
        write_spec(tmp.path(), "1234.1-staged");

        let err = run(&args("012-sequential", "050-upstream"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidArgument { .. }));
        assert!(tmp.path().join("specs/012-sequential").exists());

        let result = run(&args("1234.1-staged", "050-upstream"), tmp.path()).unwrap();
        assert!(result.retired);
    }
}
