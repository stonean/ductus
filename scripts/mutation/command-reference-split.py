"""Mutation list for spec 071's review fixes: how `resolve-anchor` judges a
pointer into a shipped document (`runtime/src/primitives/resolve_anchor.rs`,
067 scenario `pointers-into-shipped-files-are-judged`) and how
`check-step-references` qualifies a step of a named section
(`runtime/src/primitives/check_step_references.rs`, 026's Family 34).

Run with `python3 scripts/mutation/run.py scripts/mutation/command-reference-split.py
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
        'M1 a pair is prefix-matched instead of compared by equality',
        'src/primitives/resolve_anchor.rs',
        'pairs.insert(whole.start() + 2, headings.iter().any(|h| h == name));',
        'pairs.insert(whole.start() + 2, headings.iter().any(|h| name.starts_with(h.as_str())));',
        'resolve_anchor',
    ),
    (
        'M2 headings inside fenced blocks and comments are collected',
        'src/primitives/resolve_anchor.rs',
        '        .filter(|line| !skip.skip(line))\n',
        '        .filter(|line| { let _ = &mut skip; !line.is_empty() })\n',
        'resolve_anchor',
    ),
    (
        'M3 the line rule is tried before a pair, so a pair takes its fallbacks',
        'src/primitives/resolve_anchor.rs',
        '            let cross = pairs.get(&whole).copied().or_else(|| {',
        '            let cross = judged_against.as_ref().map(|h| names_local_heading(h, rest) || names_local_heading(&own_headings, rest) || markers.contains(&anchor)).or_else(|| pairs.get(&whole).copied()).or_else(|| {',
        'resolve_anchor',
    ),
    (
        'M4 a step of a named section is resolved against the command',
        'src/primitives/check_step_references.rs',
        '        if !(after.starts_with(" of §") || after.starts_with(" of **§")) {\n            out.extend(numbers);\n        }',
        '        let _ = after;\n        out.extend(numbers);',
        'check_step_references',
    ),
    (
        'M5 the line rule accepts a heading that ends inside a word',
        'src/primitives/resolve_anchor.rs',
        '            !tail\n                .chars()',
        '            true || !tail\n                .chars()',
        'resolve_anchor',
    ),
]
