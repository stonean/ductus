//! `invalidate-review` — mark a spec's recorded review as no longer
//! describing the spec.
//!
//! The pre-`done` gate asks two questions of the `review:` block: has a
//! review run at all, and does the recorded one still describe the current
//! code. The second is answered by diffing the spec's **durable contracts**
//! — `scenarios/*.md` and `data-model.md` — between `reviewed-against` and
//! `HEAD`. `spec.md` is deliberately outside that set, because a spec body
//! is not what a review reads.
//!
//! That leaves one case the staleness check cannot see: a fold-back
//! (spec 051) that routes a branch-scoped spec's content into the upstream
//! spec's **body**. Only `spec.md` changes, no durable contract moves, and
//! the upstream spec can return to `done` carrying a review that never saw
//! the code the fold brought with it. The fold knows what the diff cannot,
//! so it says so here rather than leaving the gate to infer it.
//!
//! **Waivers survive.** An invalidation says the review is out of date; it
//! does not withdraw an operator's recorded judgement about a finding, and
//! silently dropping one would make a MUST re-block with the reasoning for
//! accepting it gone.
//!
//! Converges on a re-run: a spec with no current review is
//! `invalidated: false`, a domain outcome rather than an error, so an
//! interrupted fold completes by being run again (spec 051, AC24, AC29).

use std::path::Path;

use crate::primitives::write_review::{SpecReviewFm, render_waivers, splice_review_block};
use crate::primitives::{
    PrimitiveError, Result, read_text, rel_path, split_frontmatter, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{InvalidateReviewArgs, InvalidateReviewResult};

/// Execute the `invalidate-review` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` carries a
/// parent-directory component, [`PrimitiveError::FeatureNotFound`] when the
/// feature directory is missing, [`PrimitiveError::MissingFrontmatter`] when
/// the spec has no `---` fences, [`PrimitiveError::Yaml`] when the
/// frontmatter does not parse, or [`PrimitiveError::Io`] for filesystem
/// failures.
///
/// A spec that records no current review is **not** an error — it is
/// `invalidated: false`, already in the state this primitive produces.
pub fn run(args: &InvalidateReviewArgs, repo: &Path) -> Result<InvalidateReviewResult> {
    super::validate_no_traversal(&args.feature)?;

    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, body) = split_frontmatter(&content, &spec_path)?;
    let existing: SpecReviewFm =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;

    let path = rel_path(&spec_path, repo);
    let Some(review) = existing.review else {
        // No block at all: the gate already reads this as not-reviewed, and
        // writing one whose every field is null would add noise, not state.
        return Ok(InvalidateReviewResult {
            invalidated: false,
            path,
            previous_last_run: None,
        });
    };
    let Some(previous) = review.last_run else {
        return Ok(InvalidateReviewResult {
            invalidated: false,
            path,
            previous_last_run: None,
        });
    };

    let mut block = String::from("review:\n");
    block.push_str("  last-run: null\n");
    block.push_str("  reviewed-against: null\n");
    block.push_str("  must-violations: 0\n");
    block.push_str("  should-violations: 0\n");
    block.push_str("  low-confidence: 0\n");
    block.push_str("  blocking: false\n");
    render_waivers(&mut block, &review.waivers);
    let block = block.trim_end_matches('\n').to_string();

    let new_fm = splice_review_block(fm_text, &block);
    // Same normalization write-review applies, and for the same reason: the
    // splice is LF while `body` is carried through as it was read.
    let updated = super::with_line_ending(
        &format!("---\n{new_fm}\n---\n{body}"),
        super::line_ending_of(&content),
    );
    write_atomic(&spec_path, &updated)?;

    Ok(InvalidateReviewResult {
        invalidated: true,
        path,
        previous_last_run: Some(previous),
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::run;
    use crate::primitives::PrimitiveError;
    use crate::schema::primitives::InvalidateReviewArgs;
    use std::fs;
    use std::path::Path;

    fn args(feature: &str) -> InvalidateReviewArgs {
        InvalidateReviewArgs {
            feature: feature.into(),
        }
    }

    fn write_spec(repo: &Path, feature: &str, frontmatter: &str) {
        let dir = repo.join("specs").join(feature);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\n{frontmatter}---\n\n# {feature}\n\n## Motivation\n\nx\n"),
        )
        .unwrap();
    }

    const REVIEWED: &str = "status: done\ndependencies: []\nreview:\n  last-run: 2026-08-01T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 2\n  low-confidence: 1\n  blocking: false\nnext-criterion: 4\n";

    #[test]
    fn a_recorded_review_is_reset_to_the_un_reviewed_state() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", REVIEWED);

        let result = run(&args("050-alpha"), tmp.path()).unwrap();

        assert!(result.invalidated);
        assert_eq!(
            result.previous_last_run.as_deref(),
            Some("2026-08-01T00:00:00Z")
        );
        let body = fs::read_to_string(tmp.path().join("specs/050-alpha/spec.md")).unwrap();
        assert!(body.contains("last-run: null"), "{body}");
        assert!(body.contains("reviewed-against: null"), "{body}");
        assert!(body.contains("should-violations: 0"), "{body}");
        assert!(body.contains("blocking: false"), "{body}");
        // Neighbouring top-level keys survive the splice.
        assert!(body.contains("status: done"), "{body}");
        assert!(body.contains("next-criterion: 4"), "{body}");
    }

    /// A waiver is an operator's recorded judgement about a finding.
    /// Invalidating a review says it is out of date, not that the judgement
    /// was withdrawn — dropping one would re-block a MUST with the reasoning
    /// for accepting it gone.
    #[test]
    fn waivers_survive_the_invalidation() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(
            tmp.path(),
            "050-alpha",
            "status: done\ndependencies: []\nreview:\n  last-run: 2026-08-01T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\n  waivers:\n    - rule: BE-INPUT-004\n      file: src/x.rs\n      reason: \"internal-only path, reviewed by hand\"\n      waived-at: 2026-08-01T00:00:00Z\n      waived-by: someone@example.com\n      ticket: PROJ-7\n",
        );

        assert!(run(&args("050-alpha"), tmp.path()).unwrap().invalidated);

        let body = fs::read_to_string(tmp.path().join("specs/050-alpha/spec.md")).unwrap();
        assert!(body.contains("last-run: null"), "{body}");
        assert!(body.contains("rule: BE-INPUT-004"), "{body}");
        assert!(body.contains("internal-only path"), "{body}");
        // An adopter-authored extra field is open-schema state, kept verbatim.
        assert!(body.contains("PROJ-7"), "{body}");
    }

    /// Converges: the second call is the domain outcome, not an error, so a
    /// re-run of an interrupted fold does not halt here.
    #[test]
    fn invalidating_twice_converges() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", REVIEWED);

        assert!(run(&args("050-alpha"), tmp.path()).unwrap().invalidated);
        let second = run(&args("050-alpha"), tmp.path()).unwrap();
        assert!(!second.invalidated);
        assert!(second.previous_last_run.is_none());
    }

    /// A spec that was never reviewed is already in the state this produces.
    /// Writing a block of nulls would add noise rather than state, and the
    /// gate already reads an absent block as not-reviewed.
    #[test]
    fn a_spec_with_no_review_block_is_left_alone() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", "status: draft\ndependencies: []\n");
        let before = fs::read_to_string(tmp.path().join("specs/050-alpha/spec.md")).unwrap();

        let result = run(&args("050-alpha"), tmp.path()).unwrap();

        assert!(!result.invalidated);
        assert_eq!(
            fs::read_to_string(tmp.path().join("specs/050-alpha/spec.md")).unwrap(),
            before
        );
    }

    #[test]
    fn an_absent_feature_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run(&args("050-alpha"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    #[test]
    fn a_traversing_feature_name_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run(&args("../../etc"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }
}
