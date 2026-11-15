"""Mutation list for spec 066 task 23: the pre-`done` gate reads an affected
spec of a `cross-spec-impact:` entry only through paths contained in the
project (`BE-INPUT-004`). Each mutation drops one containment: the affected
spec's `spec.md` (which also contains the directory above it), its
`scenarios/` directory, and each scenario. The tests link the file or
directory to a place outside the project that holds a back-link, so a read
through the link would discharge the obligation. M4 drops the part of
`check-artifacts`' cross-spec-impact-drift message that tells a link out from
a missing signpost.

Run with `python3 scripts/mutation/run.py scripts/mutation/gate-cross-spec-containment.py
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
        'M1 the gate reads a spec.md, or a spec directory, linked outside the project',
        'src/primitives/check_review_gate.rs',
        '    if let Err(err) = super::validate_contained(repo, &spec_rel) {\n        return outside(err);\n    }\n',
        '',
        'check_review_gate::tests::a_spec_md_linked_outside_the_project_is_reported_and_never_read check_review_gate::tests::a_spec_directory_linked_outside_the_project_is_reported_and_never_read',
    ),
    (
        'M2 the gate lists a scenarios directory linked outside the project',
        'src/primitives/check_review_gate.rs',
        '    if let Err(err) = super::validate_contained(repo, &scenarios_rel) {\n        return outside(err);\n    }\n',
        '',
        'check_review_gate::tests::a_scenarios_directory_linked_outside_the_project_is_reported_and_never_read',
    ),
    (
        'M3 the gate reads a scenario linked outside the project',
        'src/primitives/check_review_gate.rs',
        '    for (rel, _) in &scenarios {\n        if let Err(err) = super::validate_contained(repo, rel) {\n            return outside(err);\n        }\n    }\n',
        '',
        'check_review_gate::tests::a_scenario_linked_outside_the_project_is_reported_and_never_read',
    ),
    (
        'M4 the drift family reports a link out as a missing signpost',
        'src/primitives/check_artifacts.rs',
        '            entry.state != CrossSpecImpactState::Discharged\n                && entry.state != CrossSpecImpactState::OutsideProject\n',
        '            entry.state != CrossSpecImpactState::Discharged\n',
        'check_artifacts::tests::an_impact_read_through_an_outward_link_is_drift_naming_the_link',
    ),
    (
        'M5 a scenario name that is not UTF-8 is contained under a lossy spelling',
        'src/primitives/check_review_gate.rs',
        '    name.to_str().map(|name| format!("{scenarios_rel}/{name}"))',
        '    Some(format!("{scenarios_rel}/{}", name.to_string_lossy()))',
        'check_review_gate::tests::a_scenario_name_that_is_not_utf8_has_no_contained_spelling',
    ),
]
