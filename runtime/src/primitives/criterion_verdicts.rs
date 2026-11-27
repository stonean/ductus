//! A review's verdicts on a spec's acceptance criteria (066 task 27): the one
//! definition of a criterion's text, its digest and its name, which
//! `write-review` records and `compute-review-scope` compares, a mechanical sweep
//! leaving a verdict standing (066 task 32).

use std::collections::BTreeSet;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::primitives::check_stuck::feature_file_in_tree;
use crate::primitives::compute_review_scope::{ReviewEnd, last_review_commit};
use crate::primitives::mechanical_sweep::SweepIndex;
use crate::primitives::{REVIEW_RECORD_FILE, Result};
use crate::schema::primitives::{AcceptanceCriterion, ReviewBlock};

/// A criterion's text without its `AC{n}:` label, so assigning a label leaves
/// a verdict where it was.
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
    let judged = judged(record);
    criteria
        .iter()
        .enumerate()
        .filter(|(_, criterion)| !criterion.checked || !judged.contains(digest(criterion).as_str()))
        .map(|(index, criterion)| name(criterion, index))
        .collect()
}

/// The digests of the criteria the record's `criteria-verdicts` judged met.
fn judged(record: Option<&ReviewBlock>) -> BTreeSet<&str> {
    record
        .and_then(|record| record.criteria_verdicts.as_deref())
        .unwrap_or_default()
        .iter()
        .map(|verdict| verdict.digest.as_str())
        .collect()
}

/// [`owing`] over the acceptance criteria of the `spec.md` in the feature directory
/// `rel_dir` of the project at `repo`, less each ticked criterion a mechanical sweep
/// alone changed since the run whose record, the directory's `review.md`, judged it
/// ([`respelled_since_the_run`]), or nothing when there is no `spec.md`. One that
/// cannot be read is an error, never a spec owing nothing.
pub(crate) fn owing_in(
    repo: &Path,
    rel_dir: &str,
    record: Option<&ReviewBlock>,
) -> Result<Vec<String>> {
    let spec_rel = format!("{rel_dir}/spec.md");
    let record_rel = format!("{rel_dir}/{REVIEW_RECORD_FILE}");
    let (spec_rel, record_rel) = (spec_rel.as_str(), record_rel.as_str());
    let spec_path = repo.join(spec_rel);
    if !spec_path.is_file() {
        return Ok(Vec::new());
    }
    let text = super::read_text(&spec_path)?;
    let criteria = super::read_spec::parse_checkboxes(&text, "Acceptance Criteria");
    let owing = owing(&criteria, record);
    if owing.is_empty() {
        return Ok(owing);
    }
    let respelled = respelled_since_the_run(repo, (spec_rel, record_rel), record, &criteria);
    Ok(owing
        .into_iter()
        .filter(|name| !respelled.contains(name))
        .collect())
}

/// The labels of the ticked criteria that keep the verdict a review gave them
/// although their text changed since (066 task 32): each whose text at the commit
/// finishing the newest run that read its scope (`last_review_commit`, the commit
/// holding `spec.md` as that run's verdicts judged it) has its digest among the
/// record's verdicts, whose text on disk is its text at `HEAD`, and which differs
/// from that judged text only as the mechanical sweep of the window between the two
/// commits respells it ([`SweepIndex::respells`], the rule the analysis record's
/// freshness and the review record's digest apply; §drift-prevention). A criterion
/// with no label has no earlier text to find, and anything that cannot be read (no
/// such run, a held one, a shallow history, a tree, a `spec.md` or the diff) leaves
/// every criterion owing.
///
/// [`SweepIndex::respells`]: crate::primitives::mechanical_sweep::SweepIndex::respells
fn respelled_since_the_run(
    repo: &Path,
    (spec_rel, record_rel): (&str, &str),
    record: Option<&ReviewBlock>,
    criteria: &[AcceptanceCriterion],
) -> BTreeSet<String> {
    let judged = judged(record);
    let candidates: Vec<&AcceptanceCriterion> = criteria
        .iter()
        .filter(|criterion| {
            criterion.checked
                && criterion.label.is_some()
                && !judged.contains(digest(criterion).as_str())
        })
        .collect();
    if candidates.is_empty() || judged.is_empty() {
        return BTreeSet::new();
    }
    let Some((judged_at, at_head, index)) = sweep_since_the_run(repo, spec_rel, record_rel) else {
        return BTreeSet::new();
    };
    let labelled = |all: &[AcceptanceCriterion], label: &str| -> Option<AcceptanceCriterion> {
        all.iter()
            .find(|criterion| criterion.label.as_deref() == Some(label))
            .cloned()
    };
    candidates
        .into_iter()
        .filter_map(|criterion| {
            let label = criterion.label.as_deref()?;
            let then = labelled(&judged_at, label)?;
            let committed = labelled(&at_head, label)?;
            (text(&committed) == text(criterion)
                && judged.contains(digest(&then).as_str())
                && index.respells(&text(&then), &text(criterion)))
            .then(|| label.to_string())
        })
        .collect()
}

/// The spec's criteria at the commit finishing its newest run that read its scope,
/// its criteria at `HEAD`, and the mechanical-sweep index between the two commits,
/// or `None` when any of them cannot be read.
fn sweep_since_the_run(
    repo: &Path,
    spec_rel: &str,
    record_rel: &str,
) -> Option<(
    Vec<AcceptanceCriterion>,
    Vec<AcceptanceCriterion>,
    SweepIndex,
)> {
    let project = super::ProjectRepository::discover(repo).ok()?;
    let repository = &project.repository;
    let head = repository.head().ok()?.peel_to_commit().ok()?;
    let spec_git = project.to_git(spec_rel);
    let last = last_review_commit(&project, &head, record_rel, Some(&spec_git)).ok()?;
    let Some(ReviewEnd::Finished { commit, .. }) = last.end else {
        return None;
    };
    let base = repository
        .find_commit(git2::Oid::from_str(&commit).ok()?)
        .ok()?
        .tree()
        .ok()?;
    let head_tree = head.tree().ok()?;
    let criteria_in = |tree: &git2::Tree<'_>| -> Option<Vec<AcceptanceCriterion>> {
        let path = feature_file_in_tree(repository, tree, &spec_git)?;
        let spec = super::blob_text(repository, tree, &path).ok()??;
        Some(super::read_spec::parse_checkboxes(
            &spec,
            "Acceptance Criteria",
        ))
    };
    let judged_at = criteria_in(&base)?;
    let at_head = criteria_in(&head_tree)?;
    let index = SweepIndex::build(repository, &base, &head_tree);
    Some((judged_at, at_head, index))
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
        std::fs::create_dir(tmp.path().join("001-x")).unwrap();
        let spec = tmp.path().join("001-x/spec.md");
        std::fs::write(&spec, "## Acceptance Criteria\n\n- [x] One.\n").unwrap();
        std::fs::set_permissions(&spec, std::fs::Permissions::from_mode(0o000)).unwrap();
        // Root reads a file whatever its mode, so there is no unreadable file to test.
        if std::fs::read(&spec).is_err() {
            assert!(owing_in(tmp.path(), "001-x", None).is_err());
        }
        assert_eq!(
            owing_in(tmp.path(), "002-absent", None).unwrap(),
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
