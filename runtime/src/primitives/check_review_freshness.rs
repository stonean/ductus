//! `check-review-freshness` — whether one spec's review record still
//! describes its durable contracts.
//!
//! The comparison `check-review-gate` makes before `in-progress → done`,
//! [`analyze_subjects::review_freshness`](crate::primitives::analyze_subjects::review_freshness),
//! asked of a spec at any status. `/{project}:audit` Family 19
//! (`scripts/audit/review-freshness.sh`) asks it of every `done` spec at
//! release, so the release gate and the completion gate are one
//! implementation of one rule (066 plan, *Family 19 calls the runtime*).
//!
//! Read-only.

use std::path::Path;

use crate::primitives::{PrimitiveError, RecordLoad, Result, load_review_record};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckReviewFreshnessArgs, CheckReviewFreshnessResult, RecordFreshness,
};

/// Execute the `check-review-freshness` primitive.
///
/// An absent `review.md` is `never-run` with `record-present: false`; one
/// whose frontmatter does not parse is `undeterminable`, never `never-run`,
/// since a record that exists and cannot be read is a third state (§Audit
/// records).
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] when `feature` would leave the spec root,
/// or the feature directory resolves outside the project, and
/// [`PrimitiveError::FeatureNotFound`] when the feature directory is absent.
pub fn run(args: &CheckReviewFreshnessArgs, repo: &Path) -> Result<CheckReviewFreshnessResult> {
    let layout = paths::Paths::load(repo);
    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;
    let feature_dir = repo.join(&layout.specs_root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: args.feature.clone(),
        });
    }
    let rel_dir = format!("{}/{}", layout.specs_root, args.feature);
    let (record_present, freshness) = match load_review_record(&feature_dir) {
        RecordLoad::Absent => (false, RecordFreshness::NeverRun),
        RecordLoad::Unreadable(reason) => (
            true,
            RecordFreshness::Undeterminable {
                reason: format!("review.md's frontmatter does not parse: {reason}"),
            },
        ),
        RecordLoad::Present(review) => (
            true,
            crate::primitives::analyze_subjects::review_freshness(repo, &rel_dir, Some(&review)),
        ),
    };
    Ok(CheckReviewFreshnessResult {
        feature: args.feature.clone(),
        record_present,
        freshness,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::analyze_subjects::{is_review_contract, subject_digest};
    use std::fmt::Write as _;
    use std::fs;

    fn args() -> CheckReviewFreshnessArgs {
        CheckReviewFreshnessArgs {
            feature: "001-x".into(),
        }
    }

    /// A done spec with one scenario, and a review record whose digest is
    /// taken now when `digest` is set.
    fn seed(tmp: &Path, digest: bool) {
        let dir = tmp.join("specs/001-x");
        fs::create_dir_all(dir.join("scenarios")).unwrap();
        fs::write(
            dir.join("spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# X\n",
        )
        .unwrap();
        fs::write(dir.join("scenarios/edge.md"), "# Edge\n").unwrap();
        let mut record = String::from(
            "---\nspec: 001-x\nlast-run: 2026-11-04T00:00:00Z\nreviewed-against: abc123\n\
             must-violations: 0\nshould-violations: 0\nlow-confidence: 0\nblocking: false\n",
        );
        if digest {
            record.push_str("reviewed-digest:\n");
            for (path, sha) in subject_digest(&dir, is_review_contract).digests {
                let _ = writeln!(record, "  {path}: {sha}");
            }
        }
        record.push_str("---\n\n# Review\n");
        fs::write(dir.join("review.md"), record).unwrap();
    }

    /// The gate's comparison, at `done`: current until a durable contract
    /// changes, then stale naming it.
    #[test]
    fn a_done_specs_review_is_current_until_a_contract_changes() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), true);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.record_present);
        assert!(
            matches!(result.freshness, RecordFreshness::Current { .. }),
            "{result:?}"
        );

        fs::write(
            tmp.path().join("specs/001-x/scenarios/edge.md"),
            "# Edge, changed\n",
        )
        .unwrap();
        match run(&args(), tmp.path()).unwrap().freshness {
            RecordFreshness::Stale { paths, .. } => {
                assert_eq!(paths, ["specs/001-x/scenarios/edge.md"]);
            }
            other => panic!("expected stale, got {other:?}"),
        }
    }

    /// A record with no digest is undeterminable, an absent one never-run
    /// and not present, and one that does not parse undeterminable: three
    /// states, none of them current.
    #[test]
    fn a_record_that_cannot_be_compared_is_never_current() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), false);
        assert!(matches!(
            run(&args(), tmp.path()).unwrap().freshness,
            RecordFreshness::Undeterminable { .. }
        ));

        fs::write(tmp.path().join("specs/001-x/review.md"), "---\n: [\n---\n").unwrap();
        let unreadable = run(&args(), tmp.path()).unwrap();
        assert!(unreadable.record_present);
        assert!(matches!(
            unreadable.freshness,
            RecordFreshness::Undeterminable { .. }
        ));

        fs::remove_file(tmp.path().join("specs/001-x/review.md")).unwrap();
        let absent = run(&args(), tmp.path()).unwrap();
        assert!(!absent.record_present);
        assert_eq!(absent.freshness, RecordFreshness::NeverRun);
    }

    #[test]
    fn a_missing_or_escaping_feature_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        assert!(matches!(
            run(&args(), tmp.path()),
            Err(PrimitiveError::FeatureNotFound { .. })
        ));
        let escape = CheckReviewFreshnessArgs {
            feature: "../x".into(),
        };
        assert!(matches!(
            run(&escape, tmp.path()),
            Err(PrimitiveError::InvalidPath { .. })
        ));
    }
}
