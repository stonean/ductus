"""Mutation list for 079's one description per primitive: what the `ductus` list and
`-h` show of a description (`runtime/src/schema/descriptions.rs`). The tests that hold
each tool's description to its subcommand's `long_about` are integration tests, which
run.py cannot run.

Run with `python3 scripts/mutation/run.py scripts/mutation/one-description.py
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
        'M1 a summary runs past the first sentence',
        'src/schema/descriptions.rs',
        'let sentence = description.find(". ").map(|end| end + 1);',
        'let sentence = description.rfind(". ").map(|end| end + 1);',
        'descriptions::tests::a_summary_ends_at_the_first_sentence',
    ),
    (
        'M2 a summary drops its full stop',
        'src/schema/descriptions.rs',
        'let sentence = description.find(". ").map(|end| end + 1);',
        'let sentence = description.find(". ");',
        'descriptions::tests::a_summary_ends_at_the_first_sentence',
    ),
    (
        'M3 a colon does not end a summary',
        'src/schema/descriptions.rs',
        'let clause = description.find(": ");',
        'let clause: Option<usize> = None;',
        'descriptions::tests::a_summary_ends_at_a_colon_that_comes_first',
    ),
    (
        'M4 the later of the two ends a summary',
        'src/schema/descriptions.rs',
        '(Some(sentence), Some(clause)) => sentence.min(clause),',
        '(Some(sentence), Some(clause)) => sentence.max(clause),',
        'descriptions::tests::a_summary_ends_at_a_colon_that_comes_first',
    ),
    (
        'M5 a text with neither ends empty',
        'src/schema/descriptions.rs',
        '(None, None) => description.len(),',
        '(None, None) => 0,',
        'descriptions::tests::a_dot_or_colon_inside_a_word_ends_nothing',
    ),
    (
        'M6 a colon wins with no sentence end only',
        'src/schema/descriptions.rs',
        '(Some(end), None) | (None, Some(end)) => end,',
        '(Some(end), None) => end,\n        (None, Some(_)) => description.len(),',
        'descriptions::tests::a_summary_ends_at_a_colon_with_no_sentence_end',
    ),
]
