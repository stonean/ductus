//! A review's verdicts on a spec's acceptance criteria (066 task 27): the one
//! definition of a criterion's text, its digest and its name, which
//! `write-review` records and `compute-review-scope` compares.

use std::collections::BTreeSet;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::primitives::Result;
use crate::schema::primitives::{AcceptanceCriterion, ReviewBlock};

/// A criterion's text without its `AC{n}:` label, so assigning a label leaves
/// both an edit's classification (079 AC15) and a verdict where they were.
pub(crate) fn text(criterion: &AcceptanceCriterion) -> String {
    let unlabelled = criterion
        .label
        .as_deref()
        .and_then(|label| criterion.text.strip_prefix(label))
        .and_then(|rest| rest.strip_prefix(':'))
        .map(str::trim_start);
    unlabelled.map_or_else(|| criterion.text.clone(), str::to_string)
}

/// The lowercase-hex sha256 of [`text`]: what a verdict is kept by, so a
/// verdict holds while the criterion's words do, whatever its tick or label.
pub(crate) fn digest(criterion: &AcceptanceCriterion) -> String {
    super::hex(&Sha256::digest(text(criterion).as_bytes()))
}

/// How a record or a message names a criterion: its label, or `#{index}`
/// (0-based, body order) for one the labelling pass has not reached.
pub(crate) fn name(criterion: &AcceptanceCriterion, index: usize) -> String {
    criterion
        .label
        .clone()
        .unwrap_or_else(|| format!("#{index}"))
}

/// Each criterion that keeps a review owed whatever the window holds: one
/// unticked, or one ticked whose digest is not among the record's
/// `criteria-verdicts`, by [`name`], in body order. No record, or a record
/// without the field, holds no verdict, so every ticked criterion owes one.
pub(crate) fn owing(criteria: &[AcceptanceCriterion], record: Option<&ReviewBlock>) -> Vec<String> {
    let judged: BTreeSet<&str> = record
        .and_then(|record| record.criteria_verdicts.as_deref())
        .unwrap_or_default()
        .iter()
        .map(|verdict| verdict.digest.as_str())
        .collect();
    criteria
        .iter()
        .enumerate()
        .filter(|(_, criterion)| !criterion.checked || !judged.contains(digest(criterion).as_str()))
        .map(|(index, criterion)| name(criterion, index))
        .collect()
}

/// [`owing`] over the acceptance criteria of the `spec.md` at `spec_path`, or
/// nothing when there is none. One that cannot be read is an error, never a
/// spec owing nothing.
pub(crate) fn owing_in(spec_path: &Path, record: Option<&ReviewBlock>) -> Result<Vec<String>> {
    if !spec_path.is_file() {
        return Ok(Vec::new());
    }
    let text = super::read_text(spec_path)?;
    Ok(owing(
        &super::read_spec::parse_checkboxes(&text, "Acceptance Criteria"),
        record,
    ))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn criterion(text: &str, label: Option<&str>, checked: bool) -> AcceptanceCriterion {
        AcceptanceCriterion {
            checked,
            text: text.to_string(),
            label: label.map(str::to_string),
        }
    }

    #[test]
    fn a_label_and_a_tick_leave_the_digest_and_a_word_moves_it() {
        let bare = criterion("The gate holds done.", None, false);
        let labelled = criterion("AC3: The gate holds done.", Some("AC3"), true);
        assert_eq!(text(&labelled), "The gate holds done.");
        assert_eq!(digest(&bare), digest(&labelled));
        let reworded = criterion("AC3: The gate holds `done`.", Some("AC3"), true);
        assert_ne!(digest(&labelled), digest(&reworded));
    }

    #[test]
    fn a_criterion_is_named_by_its_label_or_its_position() {
        assert_eq!(name(&criterion("AC7: x", Some("AC7"), false), 4), "AC7");
        assert_eq!(name(&criterion("x", None, false), 4), "#4");
    }

    fn record_judging(criteria: &[&AcceptanceCriterion]) -> ReviewBlock {
        ReviewBlock {
            criteria_verdicts: Some(
                criteria
                    .iter()
                    .map(
                        |criterion| crate::schema::primitives::CriterionVerdictRecord {
                            label: criterion.label.clone(),
                            digest: digest(criterion),
                        },
                    )
                    .collect(),
            ),
            ..ReviewBlock::default()
        }
    }

    /// An unticked criterion owes a review even when the record judged it met
    /// (a person or a later review unticked it), a ticked one owes one unless
    /// the record judged its current text met, and one judged met owes none.
    #[test]
    fn a_criterion_owes_a_review_until_judged_met_with_its_text_and_ticked() {
        let met = criterion("AC1: One holds.", Some("AC1"), true);
        let reworded = criterion("AC2: Two holds now.", Some("AC2"), true);
        let unticked = criterion("AC3: Three holds.", Some("AC3"), false);
        let unlabelled = criterion("Four holds.", None, true);
        let as_judged = criterion("AC2: Two holds.", Some("AC2"), true);
        let record = record_judging(&[&met, &as_judged, &unticked, &unlabelled]);
        let criteria = [met.clone(), reworded, unticked, unlabelled];
        assert_eq!(owing(&criteria, Some(&record)), vec!["AC2", "AC3"]);
    }

    /// A `spec.md` that cannot be read is an error, never a spec owing nothing;
    /// one that is absent owes nothing.
    #[cfg(unix)]
    #[test]
    fn an_unreadable_spec_is_an_error_not_a_spec_owing_nothing() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let spec = tmp.path().join("spec.md");
        std::fs::write(&spec, "## Acceptance Criteria\n\n- [x] One.\n").unwrap();
        std::fs::set_permissions(&spec, std::fs::Permissions::from_mode(0o000)).unwrap();
        // Root reads a file whatever its mode, so there is no unreadable file to test.
        if std::fs::read(&spec).is_err() {
            assert!(owing_in(&spec, None).is_err());
        }
        assert_eq!(
            owing_in(&tmp.path().join("absent.md"), None).unwrap(),
            Vec::<String>::new()
        );
    }

    /// No record, and a record from before the field, hold no verdict: every
    /// criterion owes a review, ticked or not.
    #[test]
    fn a_record_without_verdicts_leaves_every_criterion_owing() {
        let criteria = [
            criterion("AC1: One holds.", Some("AC1"), true),
            criterion("Two holds.", None, true),
        ];
        assert_eq!(owing(&criteria, None), vec!["AC1", "#1"]);
        assert_eq!(
            owing(&criteria, Some(&ReviewBlock::default())),
            vec!["AC1", "#1"]
        );
    }
}
