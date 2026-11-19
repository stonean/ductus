"""Mutation list for spec 057's scenario `a-merge-carries-each-keys-whole-value`:
`relocate-audit-records` moves, keeps, renames and drops a frontmatter key with
the lines of its whole value, and a blank line inside a `review:` / `analyze:`
block does not end it. Separate from `compare-and-set-057.py`, which covers the
write guard. The tests in `relocate_audit_records.rs` kill these.

Run with `python3 scripts/mutation/run.py scripts/mutation/relocate-merge-children-057.py
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
        'M1 an entry never gathers its indented lines',
        'src/primitives/relocate_audit_records.rs',
        '                entry.children.append(&mut blanks);\n                entry.children.push(line);\n',
        '                blanks.clear();\n',
        'relocate_audit_records::tests::a_kept_multi_line_key_carries_its_children',
    ),
    (
        'M2 a key is compared on its key line alone',
        'src/primitives/relocate_audit_records.rs',
        '            value.push(\'\\n\');\n            value.push_str(child.trim());\n',
        '',
        'relocate_audit_records::tests::an_overridden_multi_line_key_is_replaced_whole_and_a_difference_is_named',
    ),
    (
        'M3 a kept key is written without its lines',
        'src/primitives/relocate_audit_records.rs',
        '            for child in &entry.children {\n                out.push_str(child);\n                out.push(\'\\n\');\n            }\n',
        '',
        'relocate_audit_records::tests::a_kept_multi_line_key_carries_its_children',
    ),
    (
        'M4 a blank line between two scalar keys is carried',
        'src/primitives/relocate_audit_records.rs',
        '            Some(entry) if indented || sequence => {',
        '            Some(entry) if indented || sequence || !blanks.is_empty() => {',
        'relocate_audit_records::tests::a_blank_line_between_scalar_keys_is_still_dropped',
    ),
    (
        'M5 a blank line inside a key\'s value is dropped',
        'src/primitives/relocate_audit_records.rs',
        '                entry.children.append(&mut blanks);\n',
        '                blanks.clear();\n                entry.children.push(line);\n                continue;\n',
        'relocate_audit_records::tests::a_kept_multi_line_key_carries_its_children',
    ),
    (
        'M6 a blank line ends a record block',
        'src/primitives/relocate_audit_records.rs',
        '        if current.is_some() && indented {\n            if let Some((_, lines)) = blocks.last_mut() {\n                lines.extend(std::iter::repeat_n(String::new(), blanks));',
        '        if current.is_some() && indented && blanks == 0 {\n            if let Some((_, lines)) = blocks.last_mut() {\n                lines.extend(std::iter::repeat_n(String::new(), blanks));',
        'relocate_audit_records::tests::a_blank_line_inside_a_spec_block_does_not_end_it',
    ),
    (
        'M7 a column-zero sequence item is its own key',
        'src/primitives/relocate_audit_records.rs',
        '        let sequence = line.trim_end() == "-" || line.starts_with("- ");',
        '        let sequence = false;',
        'relocate_audit_records::tests::a_column_zero_sequence_belongs_to_its_key',
    ),
    (
        'M8 a renamed legacy key loses its lines',
        'src/primitives/relocate_audit_records.rs',
        '                let _ = writeln!(out, "{current}: {value}");\n            } else {\n                out.push_str(entry.line);\n                out.push(\'\\n\');\n            }\n            for child in &entry.children {',
        '                let _ = writeln!(out, "{current}: {value}");\n                continue;\n            } else {\n                out.push_str(entry.line);\n                out.push(\'\\n\');\n            }\n            for child in &entry.children {',
        'relocate_audit_records::tests::a_renamed_legacy_key_keeps_its_children',
    ),
]
