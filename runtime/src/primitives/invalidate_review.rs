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
/// parent-directory component, or when the feature directory, or a link inside it,
/// resolves outside the project, [`PrimitiveError::FeatureNotFound`] when the
/// feature directory is missing, [`PrimitiveError::UnclosedFrontmatter`] when
/// `review.md`'s frontmatter block never closes, [`PrimitiveError::Yaml`] when
/// its frontmatter or `waivers:` list does not parse, or [`PrimitiveError::Io`]
/// for filesystem failures.
///
/// A spec that records no current review is **not** an error — it is
/// `invalidated: false`, already in the state this primitive produces.
pub fn run(args: &InvalidateReviewArgs, repo: &Path) -> Result<InvalidateReviewResult> {
    let root = paths::Paths::load(repo).specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;

    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    // The record is both read and written at `review.md` (spec 057). `spec.md`
    // is not consulted and not touched: reading the decision from one file
    // while writing it to another is how an invalidation stops meaning
    // anything.
    let review_path = feature_dir.join(crate::primitives::REVIEW_RECORD_FILE);
    let path = rel_path(&review_path, repo);
    // An invalidation says the *review* is out of date, not that a recorded
    // judgement was withdrawn, so waivers are carried across the write.
    let recorded_waivers: Vec<crate::primitives::write_review::RawWaiverFull> =
        crate::primitives::read_recorded_waivers(&feature_dir)?;

    // An absent or unreadable record is already in the state this produces, so
    // there is nothing to invalidate. Writing a record of nulls over a file
    // that will not parse would also destroy whatever it still holds.
    let Some(previous) = crate::primitives::load_review_record(&feature_dir)
        .as_present()
        .and_then(|record| record.last_run.clone())
    else {
        return Ok(InvalidateReviewResult {
            invalidated: false,
            path,
            previous_last_run: None,
        });
    };

    invalidate_record_artifact(&feature_dir, &recorded_waivers)?;

    Ok(InvalidateReviewResult {
        invalidated: true,
        path,
        previous_last_run: Some(previous),
    })
}

/// Rewrite `review.md`'s frontmatter with a nulled record, preserving its
/// waivers and its report body.
///
/// An absent artifact is a no-op, on the same reasoning the spec-block path
/// applies to a missing block: there is no record to invalidate, and writing
/// one whose every field is null would add noise rather than state.
fn invalidate_record_artifact(
    feature_dir: &Path,
    waivers: &[crate::primitives::write_review::RawWaiverFull],
) -> Result<()> {
    // `dispositions` goes with the run's other counts: a record reading
    // `last-run: null` that still carried the invalidated run's disposition
    // counts would state outcomes of a run it says never happened. So does
    // `captured-issues`, for the same reason: no writer emits it since spec
    // 058, but a record written before then still carries it until its next
    // review.
    const SCALARS: [&str; 15] = [
        "last-run",
        "reviewed-against",
        "diff-base",
        "must-violations",
        "should-violations",
        "low-confidence",
        "examined",
        "scope",
        "skipped-passes",
        "reviewed-digest",
        "reviewed-unreadable",
        "blocking",
        "dispositions",
        "captured-issues",
        "waivers",
    ];

    let review_path = feature_dir.join(crate::primitives::REVIEW_RECORD_FILE);
    if !review_path.is_file() {
        return Ok(());
    }
    let content = read_text(&review_path)?;
    let (fm_text, body) = split_frontmatter(&content, &review_path)?;

    // Keep every key that is not a review scalar — `spec`, `scenario`, and any
    // adopter addition — so an invalidation narrows the record rather than
    // rewriting it out from under its owner.
    let mut kept = String::new();
    let mut skipping = false;
    for line in fm_text.lines() {
        if !line.starts_with([' ', '\t']) {
            let key = line.split(':').next().unwrap_or("").trim();
            skipping = SCALARS.contains(&key);
        }
        if !skipping {
            kept.push_str(line);
            kept.push('\n');
        }
    }

    let mut fm = kept;
    fm.push_str("last-run: null\n");
    fm.push_str("reviewed-against: null\n");
    fm.push_str("must-violations: 0\n");
    fm.push_str("should-violations: 0\n");
    fm.push_str("low-confidence: 0\n");
    fm.push_str("blocking: false\n");
    crate::primitives::write_review::render_waivers_at(&mut fm, waivers, "");

    let updated = super::with_line_ending(
        &format!("---\n{}\n---\n{body}", fm.trim_end_matches('\n')),
        super::line_ending_of(&content),
    );
    write_atomic(&review_path, &updated)
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

    const SPEC: &str = "status: done\ndependencies: []\nnext-criterion: 4\n";

    /// The recorded review, where it now lives (spec 057).
    const RECORD: &str = "---\nspec: 050-alpha\nlast-run: 2026-08-01T00:00:00Z\nreviewed-against: abc123\nmust-violations: 0\nshould-violations: 2\nlow-confidence: 1\nblocking: false\n---\n\n# Review — 050-alpha\n";

    fn seed_reviewed(repo: &Path) {
        write_spec(repo, "050-alpha", SPEC);
        fs::write(repo.join("specs/050-alpha/review.md"), RECORD).unwrap();
    }

    fn review_md(repo: &Path) -> String {
        fs::read_to_string(repo.join("specs/050-alpha/review.md")).unwrap()
    }

    #[test]
    fn a_recorded_review_is_reset_to_the_un_reviewed_state() {
        let tmp = tempfile::tempdir().unwrap();
        seed_reviewed(tmp.path());
        let spec_before = fs::read_to_string(tmp.path().join("specs/050-alpha/spec.md")).unwrap();

        let result = run(&args("050-alpha"), tmp.path()).unwrap();

        assert!(result.invalidated);
        assert_eq!(
            result.previous_last_run.as_deref(),
            Some("2026-08-01T00:00:00Z")
        );
        let record = review_md(tmp.path());
        assert!(record.contains("last-run: null"), "{record}");
        assert!(record.contains("reviewed-against: null"), "{record}");
        assert!(record.contains("should-violations: 0"), "{record}");
        assert!(record.contains("blocking: false"), "{record}");
        // Keys the record carries that are not review scalars survive, and so
        // does the report body — an invalidation narrows the record rather
        // than rewriting the file.
        assert!(record.contains("spec: 050-alpha"), "{record}");
        assert!(record.contains("# Review — 050-alpha"), "{record}");
        // And the spec is not a party to any of it.
        assert_eq!(
            fs::read_to_string(tmp.path().join("specs/050-alpha/spec.md")).unwrap(),
            spec_before
        );
    }

    /// The invalidated run's disposition counts go with its other counts: a
    /// record reading `last-run: null` cannot state outcomes of a run it says
    /// never happened. Its stored decisions stay — like waivers, they are an
    /// operator's recorded judgements, not the run's output (spec 058).
    #[test]
    fn the_dispositions_map_goes_and_the_decisions_stay() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", SPEC);
        fs::write(
            tmp.path().join("specs/050-alpha/review.md"),
            "---\nspec: 050-alpha\nlast-run: 2026-08-01T00:00:00Z\nmust-violations: 0\nblocking: false\n\
             dispositions:\n  fixed: 1\n  routed: 0\n  discarded: 1\n  undispositioned: 0\n\
             decisions:\n  - key: \"other: noise\"\n    outcome: discarded\n    reason: not this spec\n    \
             decided-at: 2026-08-01T00:00:00Z\n    decided-by: dev@example.com\n---\n\n# Review — 050-alpha\n",
        )
        .unwrap();
        run(&args("050-alpha"), tmp.path()).unwrap();
        let record = review_md(tmp.path());
        assert!(!record.contains("dispositions:"), "{record}");
        assert!(!record.contains("fixed: 1"), "{record}");
        assert!(record.contains("decisions:"), "{record}");
        assert!(record.contains("reason: not this spec"), "{record}");
    }

    /// A record written before spec 058 still carries its run's
    /// `captured-issues` count, and it goes with the other counts: left beside
    /// `last-run: null`, it would state a result of a run the record says
    /// never happened.
    #[test]
    fn a_pre_058_captured_issues_count_goes_with_the_run() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", SPEC);
        fs::write(
            tmp.path().join("specs/050-alpha/review.md"),
            "---\nspec: 050-alpha\nlast-run: 2026-08-01T00:00:00Z\nmust-violations: 0\n\
             captured-issues: 3\nblocking: false\n---\n\n# Review — 050-alpha\n",
        )
        .unwrap();
        run(&args("050-alpha"), tmp.path()).unwrap();
        let record = review_md(tmp.path());
        assert!(!record.contains("captured-issues"), "{record}");
        assert!(record.contains("last-run: null"), "{record}");
    }

    /// A waiver is an operator's recorded judgement about a finding.
    /// Invalidating a review says it is out of date, not that the judgement
    /// was withdrawn — dropping one would re-block a MUST with the reasoning
    /// for accepting it gone.
    #[test]
    fn waivers_survive_the_invalidation() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", SPEC);
        // The record and its waivers both live in `review.md` (spec 057).
        fs::write(
            tmp.path().join("specs/050-alpha/review.md"),
            "---\nspec: 050-alpha\nlast-run: 2026-08-01T00:00:00Z\nreviewed-against: abc123\nmust-violations: 0\nblocking: false\nwaivers:\n  - rule: BE-INPUT-004\n    file: src/x.rs\n    reason: \"internal-only path, reviewed by hand\"\n    waived-at: 2026-08-01T00:00:00Z\n    waived-by: someone@example.com\n    ticket: PROJ-7\n---\n\n# Review — 050-alpha\n",
        )
        .unwrap();

        assert!(run(&args("050-alpha"), tmp.path()).unwrap().invalidated);

        // Both records are nulled, and both keep the judgement.
        let report = fs::read_to_string(tmp.path().join("specs/050-alpha/review.md")).unwrap();
        assert!(report.contains("last-run: null"), "{report}");
        assert!(report.contains("rule: BE-INPUT-004"), "{report}");
        assert!(report.contains("internal-only path"), "{report}");
        // An adopter-authored extra field is open-schema state, kept verbatim.
        assert!(report.contains("PROJ-7"), "{report}");
        // The report body is not collateral damage of a frontmatter rewrite.
        assert!(report.contains("# Review — 050-alpha"), "{report}");
    }

    /// A waiver whose `file` lists several paths (020's `waiver-file-lists`)
    /// comes through the invalidation with every path — it prunes nothing.
    #[test]
    fn a_waiver_listing_several_paths_survives_whole() {
        let tmp = tempfile::tempdir().unwrap();
        write_spec(tmp.path(), "050-alpha", SPEC);
        let waiver = "  - rule: BE-INPUT-004\n    file:\n      - src/x.rs\n      - src/y.rs\n    reason: internal-only paths\n    waived-at: 2026-08-01T00:00:00Z\n    waived-by: someone@example.com\n";
        fs::write(
            tmp.path().join("specs/050-alpha/review.md"),
            format!("---\nspec: 050-alpha\nlast-run: 2026-08-01T00:00:00Z\nmust-violations: 0\nblocking: false\nwaivers:\n{waiver}---\n\n# Review — 050-alpha\n"),
        )
        .unwrap();

        assert!(run(&args("050-alpha"), tmp.path()).unwrap().invalidated);

        let report = fs::read_to_string(tmp.path().join("specs/050-alpha/review.md")).unwrap();
        assert!(report.contains("last-run: null"), "{report}");
        assert!(report.contains(&format!("waivers:\n{waiver}")), "{report}");
    }

    /// Converges: the second call is the domain outcome, not an error, so a
    /// re-run of an interrupted fold does not halt here.
    #[test]
    fn invalidating_twice_converges() {
        let tmp = tempfile::tempdir().unwrap();
        seed_reviewed(tmp.path());

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
