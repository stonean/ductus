"""Mutation list for spec 022's review of 2026-11-19: a `scenarios/` directory
that exists and refuses a listing is named in `scenario-files-unreadable`,
not read as zero scenarios (`runtime/src/primitives/read_spec.rs`
`collect_scenario_open_questions_unlinked`); and the fold destinations
`routeFold` offers skip a heading-like line inside any fenced block, longer
backtick runs and `~~~` fences included (`runtime/src/interpreter/payload.rs`
`section_headings`).

Run with `python3 scripts/mutation/run.py scripts/mutation/review-022-fixes-2026-11-19.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

M1 and M2 are killed by `read_spec::tests::a_scenarios_directory_that_refuses_a_listing_is_reported_not_read_as_clean`
(unix only, and a no-op under a privileged user, which lists the directory
anyway) and `read_spec::tests::an_absent_scenarios_directory_reports_nothing_unreadable`;
M3 by `interpreter::payload::tests::section_headings_skips_a_longer_or_tilde_fence_holding_a_heading_like_line`.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a scenarios directory that refuses a listing is read as zero scenarios',
        'src/primitives/read_spec.rs',
        '        && err.kind() != std::io::ErrorKind::NotFound\n    {\n        unreadable.push("scenarios/".to_string());\n    }\n',
        '    {\n        let _ = err;\n    }\n',
        'read_spec::tests::a_scenarios_directory_that_refuses_a_listing_is_reported_not_read_as_clean',
    ),
    (
        'M2 an absent scenarios directory is reported unreadable',
        'src/primitives/read_spec.rs',
        '        && err.kind() != std::io::ErrorKind::NotFound\n',
        '        && err.kind() != std::io::ErrorKind::PermissionDenied\n',
        'read_spec::tests::an_absent_scenarios_directory_reports_nothing_unreadable',
    ),
    (
        'M3 the fold destinations toggle a fence on any triple backtick again',
        'src/interpreter/payload.rs',
        '    let mut fences = Fences::default();\n    for line in content.lines() {\n        if fences.holds(line.trim_start()) {\n            continue;\n        }\n        if let Some(text) = line.strip_prefix("## ") {',
        '    let mut fenced = false;\n    for line in content.lines() {\n        if line.trim_start().starts_with("```") {\n            fenced = !fenced;\n            continue;\n        }\n        if fenced {\n            continue;\n        }\n        if let Some(text) = line.strip_prefix("## ") {',
        'interpreter::payload::tests::section_headings_skips_a_longer_or_tilde_fence_holding_a_heading_like_line',
    ),
]
