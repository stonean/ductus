"""Mutation list for review honoring a spec's departure entries (spec 073, task 5):
`write-review` derives a marked waiver from each well-formed entry of any tier and
names each malformed one, and `process-waivers` expires a marked waiver whose rule
has no entry. Each mutation removes one of those behaviors, so a kill shows its own
test holds it.

Run with `python3 scripts/mutation/run.py scripts/mutation/departure-waivers.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a marked waiver is never expired for want of an entry',
        'src/primitives/process_waivers.rs',
        'if waiver.departure != Some(true) {',
        'if true {',
        'departure_waivers::a_removed_entry',
    ),
    (
        'M2 a marked waiver is expired although its rule has an entry',
        'src/primitives/process_waivers.rs',
        '    Ok(!entry_rules\n        .as_deref()\n        .unwrap_or_default()\n        .iter()\n        .any(|held| held == rule))',
        '    Ok(entry_rules\n        .as_deref()\n        .unwrap_or_default()\n        .iter()\n        .any(|held| held == rule))',
        'departure_waivers::',
    ),
    (
        'M3 a derived waiver carries no departure marker',
        'src/primitives/write_review.rs',
        'extra: std::collections::BTreeMap::from([(\n                "departure".to_string(),\n                serde_norway::Value::Bool(true),\n            )]),',
        'extra: std::collections::BTreeMap::new(),',
        'departure_waivers::a_must_departure departure_waivers::a_should_departure',
    ),
    (
        'M4 only a MUST departure derives a waiver',
        'src/primitives/write_review.rs',
        'finding.rule == entry.rule\n                    && waiver_reason(finding, &args.applied_waivers).is_none()',
        'finding.rule == entry.rule\n                    && finding.severity.is_blocking()\n                    && waiver_reason(finding, &args.applied_waivers).is_none()',
        'departure_waivers::a_should_departure',
    ),
    (
        'M5 an entry claims a file a hand waive stored in the same run',
        'src/primitives/write_review.rs',
        '!holds(surviving, &entry.rule, file) && !holds(entries, &entry.rule, file)',
        '!holds(surviving, &entry.rule, file)',
        'departure_waivers::a_hand_waive',
    ),
    (
        'M6 an unattributed entry stores no time of the run',
        'src/primitives/write_review.rs',
        '_ => ("unattributed".to_string(), args.reviewed_at.clone()),',
        '_ => ("unattributed".to_string(), String::new()),',
        'departure_waivers::an_unattributed_entry',
    ),
    (
        'M7 the run names no malformed entry',
        'src/primitives/write_review.rs',
        '        malformed_departures,\n        ..WriteReviewResult::default()',
        '        malformed_departures: Vec::new(),\n        ..WriteReviewResult::default()',
        'departure_waivers::a_malformed_entry departure_waivers::a_rule_two_entries',
    ),
    (
        'M8 a rule two entries name derives one waiver per entry',
        'src/primitives/write_review.rs',
        '.filter(|entry| seen.insert(entry.rule.clone()))',
        '.filter(|entry| seen.insert(entry.rule.clone()) || true)',
        'departure_waivers::a_rule_two_entries',
    ),
    (
        'M9 a derived waiver anchors no finding, so none is counted waived',
        'src/primitives/write_review.rs',
        '        anchors.extend(\n            firing\n                .iter()\n                .map(|file| crate::schema::primitives::WaiverRef {\n                    rule: entry.rule.clone(),\n                    file: (*file).to_string(),\n                    reason: entry.reason.clone(),\n                }),\n        );\n        let fresh',
        '        let fresh',
        'departure_waivers::a_must_departure',
    ),
    (
        'M10 a stored departure waiver is not anchored again on the run that finds it held',
        'src/primitives/write_review.rs',
        'firing\n                .iter()\n                .map(|file| crate::schema::primitives::WaiverRef {\n                    rule: entry.rule.clone(),\n                    file: (*file).to_string(),\n                    reason: entry.reason.clone(),',
        'firing\n                .iter()\n                .filter(|file| !holds(surviving, &entry.rule, file))\n                .map(|file| crate::schema::primitives::WaiverRef {\n                    rule: entry.rule.clone(),\n                    file: (*file).to_string(),\n                    reason: entry.reason.clone(),',
        'departure_waivers::a_run_writing',
    ),
    (
        'M11 the spec is read although no waiver carries the marker',
        'src/primitives/process_waivers.rs',
        '    if waiver.departure != Some(true) {\n        return Ok(false);\n    }',
        '    let _ = super::read_text(spec_path)?;\n    if waiver.departure != Some(true) {\n        return Ok(false);\n    }',
        'departure_waivers::a_spec_that_cannot_be_read',
    ),
    (
        'M12 an entry waives a finding of any rule',
        'src/primitives/write_review.rs',
        'finding.rule == entry.rule\n                    && waiver_reason',
        'true\n                    && waiver_reason',
        'departure_waivers::an_entry_whose_rule_fires_nowhere',
    ),
    (
        'M13 a derived waiver stores a fixed time for the decision',
        'src/primitives/write_review.rs',
        '}) => (decided_by.clone(), decided_at.clone()),',
        '}) => (decided_by.clone(), String::from("2026-01-01T00:00:00Z")),',
        'departure_waivers::a_must_departure',
    ),
    (
        'M14 a non-boolean departure value is accepted and ignored',
        'src/primitives/process_waivers.rs',
        ['departure: Option<bool>,', 'if waiver.departure != Some(true) {'],
        ['departure: Option<serde_norway::Value>,', 'if waiver.departure != Some(serde_norway::Value::Bool(true)) {'],
        'departure_waivers::a_non_boolean_marker',
    ),
]
