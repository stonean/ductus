"""Mutation list for 060's review fix: a fenced block's lines are its content
in the rule-section reader, so a `#`-led line there is no heading and a
`**Field:**`-shaped one no field, the Statement quote and the Verification
marker included, and a fence opens and closes as `CommonMark`'s fenced code
blocks do (`runtime/src/primitives/rule_sections.rs` `FenceRun`, `Fences`,
`parse_rule_sections`, `OpenSection::read`, `other_headings`).

Run with `python3 scripts/mutation/run.py scripts/mutation/rule-sections-fences.py
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
        'M1 the section reader ignores fences',
        'src/primitives/rule_sections.rs',
        '        let in_fence = fences.holds(trimmed);',
        '        let in_fence = {\n            let _ = fences.holds(trimmed);\n            false\n        };',
        'rule_sections::tests::a_fenced_heading_or_field_line_stays_in_the_verification rule_sections::tests::a_fenced_quote_or_verification_marker_opens_neither',
    ),
    (
        'M2 other_headings lists a fenced #-led line',
        'src/primitives/rule_sections.rs',
        '            !fences.holds(lines[i].trim())\n                && (',
        '            ({\n                let _ = fences.holds(lines[i].trim());\n                true\n            }) && (',
        'rule_sections::tests::a_fenced_heading_or_field_line_stays_in_the_verification',
    ),
    (
        'M3 a fenced **Field:** line ends the Verification',
        'src/primitives/rule_sections.rs',
        '                if !in_fence && is_field_line(trimmed) {',
        '                if is_field_line(trimmed) {',
        'rule_sections::tests::a_fenced_heading_or_field_line_stays_in_the_verification',
    ),
    (
        'M4 a fenced > line opens the Statement',
        'src/primitives/rule_sections.rs',
        "        let quoted = if in_fence {\n            None\n        } else {\n            trimmed.strip_prefix('>')\n        };",
        "        let quoted = trimmed.strip_prefix('>');",
        'rule_sections::tests::a_fenced_quote_or_verification_marker_opens_neither',
    ),
    (
        'M5 a fenced Verification marker opens the Verification',
        'src/primitives/rule_sections.rs',
        '                if !in_fence && let Some(rest) = trimmed.strip_prefix("**Verification:**") {',
        '                if let Some(rest) = trimmed.strip_prefix("**Verification:**") {',
        'rule_sections::tests::a_fenced_quote_or_verification_marker_opens_neither',
    ),
    (
        'M6 a fence never closes',
        'src/primitives/rule_sections.rs',
        '            if open.closed_by(trimmed) {\n                self.0 = None;\n            }\n            return true;',
        '            let _ = open;\n            return true;',
        'rule_sections::tests::a_fenced_heading_or_field_line_stays_in_the_verification',
    ),
    (
        'M7 a run carrying an info string closes a fence',
        'src/primitives/rule_sections.rs',
        '        len >= self.len && trimmed[len..].trim().is_empty()',
        '        len >= self.len',
        'rule_sections::tests::a_fence_closes_only_at_a_bare_run_as_long_as_its_opening',
    ),
    (
        'M8 a run shorter than the opening closes a fence',
        'src/primitives/rule_sections.rs',
        '        len >= self.len && trimmed[len..].trim().is_empty()',
        '        len >= 3 && trimmed[len..].trim().is_empty()',
        'rule_sections::tests::a_fence_closes_only_at_a_bare_run_as_long_as_its_opening',
    ),
    (
        'M9 a backtick info string may hold a backtick',
        'src/primitives/rule_sections.rs',
        "        let info_ok = mark == b'~' || !trimmed[len..].contains('`');",
        '        let info_ok = true;',
        'rule_sections::tests::a_backtick_line_holding_another_backtick_opens_no_fence',
    ),
]
