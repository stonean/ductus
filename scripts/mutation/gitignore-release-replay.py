"""Mutation list for 022's task 39: `merge-managed-block` steps over at most
three consecutive unmatched groups, the agent removal's Auggie, Antigravity
and OpenCode subsections, so every released `.gitignore` template merges into
the shipped one with nothing stranded, and four unmatched groups are still
adopter territory (scenario `merge-managed-block-renamed-subsection`;
`runtime/src/primitives/merge_managed_block.rs` `MAX_SKIPPED_RETIRED_GROUPS`).

Run with `python3 scripts/mutation/run.py scripts/mutation/gitignore-release-replay.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

M1 restores the bound of two, at which 7 of the 11 released templates strand
their old block's tail; M2 widens it to four, which consumes adopter content
past the bound.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the lookahead steps over two unmatched groups, as before the agent removal',
        'src/primitives/merge_managed_block.rs',
        'const MAX_SKIPPED_RETIRED_GROUPS: usize = 3;',
        'const MAX_SKIPPED_RETIRED_GROUPS: usize = 2;',
        'merge_managed_block::tests::every_released_gitignore_template_merges_cleanly_into_the_shipped_one',
    ),
    (
        'M2 the lookahead steps over four unmatched groups',
        'src/primitives/merge_managed_block.rs',
        'const MAX_SKIPPED_RETIRED_GROUPS: usize = 3;',
        'const MAX_SKIPPED_RETIRED_GROUPS: usize = 4;',
        'merge_managed_block::tests::line_prefix_long_unmatched_run_is_treated_as_adopter_territory',
    ),
]
