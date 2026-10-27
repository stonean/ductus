"""Mutation list for spec 002's `claude-md-template-imports-load` scenario: how
`check-orphaned-references` reads an `@path` import
(`runtime/src/primitives/check_orphaned_references.rs`, `managed_paths_in`).

Run with `python3 scripts/mutation/run.py scripts/mutation/claude-md-imports.py
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
        'M1 every `@` before a managed root skips the path, as before the fix',
        'src/primitives/check_orphaned_references.rs',
        "                || (previous == Some('@')\n                    && line[..start - 1]\n                        .chars()\n                        .next_back()\n                        .is_some_and(|c| !c.is_whitespace()))\n",
        "                || previous == Some('@')\n",
        'check_orphaned_references',
    ),
    (
        'M2 an `@` glued to a word is read as an import',
        'src/primitives/check_orphaned_references.rs',
        "                || (previous == Some('@')\n                    && line[..start - 1]\n                        .chars()\n                        .next_back()\n                        .is_some_and(|c| !c.is_whitespace()))\n",
        '',
        'check_orphaned_references',
    ),
]
