"""Mutation list for 067's AC9: `resolve-anchor` reports each form of a quoted
heading after `§` as a `quoted-heading` reference, counted and unjudgeable,
never left out of every count: double quotes closed on the line, single
quotes, and a double quote the line leaves open
(`runtime/src/primitives/resolve_anchor.rs`, `quoted_regex` and
`quoted_references`; the double-quote form closed on the line is covered by
`a_quoted_heading_is_reported_as_a_form_not_read`).

Run with `python3 scripts/mutation/run.py scripts/mutation/quoted-heading-forms-067.py
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
        "M1 the single-quoted form removed: `§'Heading'` is in no count",
        "src/primitives/resolve_anchor.rs",
        "|['\\u{2018}]([^'\\u{2019}]+)['\\u{2019}]",
        "",
        "primitives::resolve_anchor::tests::a_single_quoted_heading",
    ),
    (
        "M2 the open double quote removed: `§\"Heading` ending its line is in no count",
        "src/primitives/resolve_anchor.rs",
        "|[\\\"\\u{201C}]([^\\\"\\u{201D}]*[^\\\"\\u{201D}\\\\s])\\\\s*$",
        "",
        "primitives::resolve_anchor::tests::a_heading_quoted_across",
    ),
    (
        "M3 the open form's text is never read: its anchor is empty",
        "src/primitives/resolve_anchor.rs",
        "            .or_else(|| cap.get(3))\n",
        "",
        "primitives::resolve_anchor::tests::a_heading_quoted_across",
    ),
    (
        "M4 the single form's text is never read: its anchor is empty",
        "src/primitives/resolve_anchor.rs",
        "            .or_else(|| cap.get(2))\n",
        "",
        "primitives::resolve_anchor::tests::a_single_quoted_heading",
    ),
    (
        "M5 a quote anywhere after `§` opens a heading: an apostrophe in prose is a reference",
        "src/primitives/resolve_anchor.rs",
        "\"§(?:[\\\"\\u{201C}]",
        "\"§[^ ]*(?:[\\\"\\u{201C}]",
        "primitives::resolve_anchor::tests::a_single_quoted_heading",
    ),
]
