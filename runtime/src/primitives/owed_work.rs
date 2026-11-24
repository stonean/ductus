//! The owed-work test (spec 084): a spec owes work while its `tasks.md` holds an
//! unchecked box or its `spec.md` an unticked acceptance criterion. It is one test,
//! read by every surface that asks the question, so no two of them can disagree
//! about what a spec owes (`QUAL-DELEG-001`): `check-artifacts`' review-state drift,
//! the completion gate, `measure-reopenings`' waiting count and `check-owed-work`.
//!
//! The module does no I/O. Each reader reads its own files under its own rules (a
//! link guard, an absent file, bytes that are not UTF-8) and hands the text here.
//! The two halves are callable apart, since a reader may want one: an `in-progress`
//! spec's criteria stay unticked until its review, so `measure-reopenings`' waiting
//! count reads the boxes alone.

use std::collections::BTreeMap;

use crate::primitives::read_spec::parse_checkboxes;
use crate::primitives::read_tasks::unchecked_lines;
use crate::schema::primitives::AcceptanceCriterion;

/// The `spec.md` section the criteria are read from.
const ACCEPTANCE_CRITERIA: &str = "Acceptance Criteria";

/// What a spec owes: both halves of the test, kept apart.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct OwedWork<'a> {
    /// Each unchecked box in `tasks.md`, trimmed, with how many times it occurs.
    pub(crate) unchecked_boxes: BTreeMap<&'a str, usize>,
    /// Each unticked acceptance criterion in `spec.md`, with its index among all
    /// the spec's criteria, the index `mark-criterion` addresses.
    pub(crate) unticked_criteria: Vec<(usize, AcceptanceCriterion)>,
}

impl OwedWork<'_> {
    /// Whether the spec owes any work: either half is not empty.
    pub(crate) fn owes(&self) -> bool {
        !self.unchecked_boxes.is_empty() || !self.unticked_criteria.is_empty()
    }
}

/// What a spec owes, from its `spec.md` body and its `tasks.md` text, `None` for a
/// spec with no `tasks.md`, which holds no box.
pub(crate) fn owed_work<'a>(spec_body: &str, tasks: Option<&'a str>) -> OwedWork<'a> {
    OwedWork {
        unchecked_boxes: tasks.map(unchecked_boxes).unwrap_or_default(),
        unticked_criteria: unticked_criteria_in(spec_body),
    }
}

/// The boxes half: each unchecked box `tasks` holds, under any heading, outside
/// fenced blocks and HTML comments (spec 079's test, by
/// [`unchecked_lines`]).
pub(crate) fn unchecked_boxes(tasks: &str) -> BTreeMap<&str, usize> {
    unchecked_lines(tasks)
}

/// The unchecked boxes `tasks` holds, for a message: the count, and the first few.
pub(crate) fn describe_unchecked(tasks: &str) -> Option<String> {
    let lines = unchecked_boxes(tasks);
    let count: usize = lines.values().sum();
    if count == 0 {
        return None;
    }
    let named: Vec<&str> = lines.keys().take(3).copied().collect();
    let more = if lines.len() > named.len() {
        ", and more"
    } else {
        ""
    };
    Some(format!(
        "{count} unchecked box(es): {}{more}",
        named.join("; ")
    ))
}

/// The criteria half, over criteria already parsed: each unticked one, with its
/// index among all of `criteria`.
pub(crate) fn unticked_criteria(
    criteria: &[AcceptanceCriterion],
) -> impl Iterator<Item = (usize, &AcceptanceCriterion)> {
    criteria
        .iter()
        .enumerate()
        .filter(|(_, criterion)| !criterion.checked)
}

/// The criteria half, from a `spec.md` body, parsed as `read-spec` parses it.
pub(crate) fn unticked_criteria_in(spec_body: &str) -> Vec<(usize, AcceptanceCriterion)> {
    let criteria = parse_checkboxes(spec_body, ACCEPTANCE_CRITERIA);
    unticked_criteria(&criteria)
        .map(|(index, criterion)| (index, criterion.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: &str = "# S\n\n## Acceptance Criteria\n\n- [x] AC1: one\n- [ ] AC2: two\n- [ ] three\n\n## Open Questions\n\n- [ ] not a criterion\n";

    #[test]
    fn a_spec_owes_its_unchecked_boxes_and_its_unticked_criteria() {
        let tasks = "# T\n\n## 1. One\n\n- [x] done\n- [ ] open\n\n## Notes\n\n- [ ] loose\n";
        let owed = owed_work(SPEC, Some(tasks));
        assert!(owed.owes());
        assert_eq!(
            owed.unchecked_boxes.into_iter().collect::<Vec<_>>(),
            vec![("- [ ] loose", 1), ("- [ ] open", 1)]
        );
        let criteria: Vec<(usize, Option<&str>, &str)> = owed
            .unticked_criteria
            .iter()
            .map(|(index, criterion)| (*index, criterion.label.as_deref(), criterion.text.as_str()))
            .collect();
        assert_eq!(
            criteria,
            vec![(1, Some("AC2"), "AC2: two"), (2, None, "three")]
        );
    }

    #[test]
    fn a_spec_with_every_box_checked_and_every_criterion_ticked_owes_nothing() {
        let spec = "## Acceptance Criteria\n\n- [x] AC1: one\n";
        let tasks =
            "## 1. One\n\n- [x] done\n\n<!--\n- [ ] an example\n-->\n\n```\n- [ ] fenced\n```\n";
        let owed = owed_work(spec, Some(tasks));
        assert!(!owed.owes(), "{owed:?}");
        assert_eq!(owed, OwedWork::default());
    }

    #[test]
    fn each_half_alone_owes_work() {
        let ticked = "## Acceptance Criteria\n\n- [x] AC1: one\n";
        assert!(owed_work(ticked, Some("- [ ] open\n")).owes());
        assert!(owed_work("## Acceptance Criteria\n\n- [ ] AC1: one\n", None).owes());
        assert!(!owed_work(ticked, None).owes());
    }

    #[test]
    fn a_box_repeated_is_counted_and_described_once() {
        let tasks = "- [ ] same\n- [ ] same\n";
        assert_eq!(unchecked_boxes(tasks).get("- [ ] same"), Some(&2));
        assert_eq!(
            describe_unchecked(tasks).as_deref(),
            Some("2 unchecked box(es): - [ ] same")
        );
        assert_eq!(describe_unchecked("- [x] done\n"), None);
    }
}
