"""Mutation list for spec 067 task 37 (AC5): `check-artifacts`' line-number-citations
family reports every citation in a spec's examined artifacts as blocking, at every
status, and nothing AC5 excludes. Each mutation drops or widens one part of that;
a test kills each.

Run with `python3 scripts/mutation/run.py scripts/mutation/line-number-citations-067.py
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
        'M1 the family never runs',
        'src/primitives/check_artifacts.rs',
        '    check_line_number_citations(findings, skipped, status, &feature_dir, repo, outward);\n',
        '',
        'primitives::check_artifacts::tests::a_line_number_citation_is_reported_in_each_examined_artifact',
    ),
    (
        'M2 a blockquote line is read',
        'src/primitives/check_artifacts.rs',
        "            if (scanner.skip(line) && scanner.in_fence()) || line.trim_start().starts_with('>') {\n",
        '            if scanner.skip(line) && scanner.in_fence() {\n',
        'primitives::check_artifacts::tests::fenced_quoted_and_port_text_is_no_citation_finding',
    ),
    (
        'M3 a fenced line is read',
        'src/primitives/check_artifacts.rs',
        "            if (scanner.skip(line) && scanner.in_fence()) || line.trim_start().starts_with('>') {\n",
        "            if (scanner.skip(line) && false) || line.trim_start().starts_with('>') {\n",
        'primitives::check_artifacts::tests::fenced_quoted_and_port_text_is_no_citation_finding',
    ),
    (
        'M4 an HTML comment is skipped too',
        'src/primitives/check_artifacts.rs',
        "            if (scanner.skip(line) && scanner.in_fence()) || line.trim_start().starts_with('>') {\n",
        "            if scanner.skip(line) || line.trim_start().starts_with('>') {\n",
        'primitives::check_artifacts::tests::fenced_quoted_and_port_text_is_no_citation_finding',
    ),
    (
        'M5 tasks.md is not examined',
        'src/primitives/check_artifacts.rs',
        '            "tasks.md",\n            "data-model.md",\n            "research.md",\n        ],\n    )\n',
        '            "data-model.md",\n            "research.md",\n        ],\n    )\n',
        'primitives::check_artifacts::tests::a_line_number_citation_is_reported_in_each_examined_artifact',
    ),
    (
        'M6 a run record is examined',
        'src/primitives/check_artifacts.rs',
        '            "tasks.md",\n            "data-model.md",\n            "research.md",\n        ],\n    )\n',
        '            "tasks.md",\n            "data-model.md",\n            "research.md",\n            "review.md",\n        ],\n    )\n',
        'primitives::check_artifacts::tests::a_line_number_citation_is_reported_in_each_examined_artifact',
    ),
    (
        'M7 a citation is an advisory finding',
        'src/primitives/check_artifacts.rs',
        '                    family: LINE_CITATION_FAMILY.into(),\n                    severity: AnalyzeSeverity::Blocking,\n',
        '                    family: LINE_CITATION_FAMILY.into(),\n                    severity: AnalyzeSeverity::Advisory,\n',
        'primitives::check_artifacts::tests::a_line_number_citation_is_reported_in_each_examined_artifact',
    ),
    (
        'M8 the family is status-gated to in-progress and done',
        'src/primitives/check_artifacts.rs',
        '    for artifact in citation_artifacts(feature_dir, outward) {\n',
        '    if !matches!(status, "in-progress" | "done") {\n        return;\n    }\n    for artifact in citation_artifacts(feature_dir, outward) {\n',
        'primitives::check_artifacts::tests::a_line_number_citation_is_reported_at_every_status',
    ),
    (
        'M9 a line number set apart from its file is not reported',
        'src/primitives/check_artifacts.rs',
        '                .chain(line_citations::find_set_apart(line));\n',
        ';\n',
        'primitives::check_artifacts::tests::a_line_number_set_apart_from_its_file_is_reported',
    ),
    (
        'M10 a set-apart span is never found',
        'src/primitives/line_citations.rs',
        '        if whole == Some(content.len()) {\n            found.push(open + 1..close);\n        }\n',
        '        let _ = whole;\n',
        'primitives::line_citations::tests::a_line_number_set_apart_from_its_file_is_found primitives::check_artifacts::tests::a_line_number_set_apart_from_its_file_is_reported',
    ),
]
