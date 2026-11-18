//! A review's verdicts on a spec's acceptance criteria (066 task 27): the one
//! definition of a criterion's text, its digest and its name, which
//! `write-review` records and `compute-review-scope` compares, and which
//! `classify-spec-edit` reads to compare criteria across an edit.

use sha2::{Digest, Sha256};

use crate::schema::primitives::AcceptanceCriterion;

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

#[cfg(test)]
mod tests {
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
}
