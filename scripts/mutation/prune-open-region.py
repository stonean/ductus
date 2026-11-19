"""Mutation list for spec 041's scenario `an-open-fenced-block-or-comment-is-not-examined`:
`prune-tasks` and `prune-plan` report `unexamined-tail` when their file ends
inside an open fenced block or HTML comment, and write nothing then. Each
mutation removes one part of that behavior; the tests in `prune_tasks.rs`
and `prune_plan.rs` kill them.

Run with `python3 scripts/mutation/run.py scripts/mutation/prune-open-region.py
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
        'M1 prune-tasks writes a file that ends inside an open region',
        'src/primitives/prune_tasks.rs',
        'let applied = args.apply && !nothing_to_prune && gate_permits && unexamined_tail.is_none();',
        'let applied = args.apply && !nothing_to_prune && gate_permits;',
        'prune_tasks::tests::an_unclosed_fence_is_reported_and_an_apply_writes_nothing',
    ),
    (
        'M2 prune-plan removes sections from a plan that ends inside an open region',
        'src/primitives/prune_plan.rs',
        'let applied = args.apply && stale_sections.is_empty() && unexamined_tail.is_none();',
        'let applied = args.apply && stale_sections.is_empty();',
        'prune_plan::tests::an_unclosed_fence_under_a_non_record_section_cannot_remove_the_design_record',
    ),
    (
        'M3 an open fence is reported as an open comment',
        'src/primitives/prune_tasks.rs',
        '            cause: if self.scanner.in_fence() {',
        '            cause: if !self.scanner.in_fence() {',
        'prune_tasks::tests::an_unclosed_fence_is_reported_and_an_apply_writes_nothing',
    ),
    (
        'M4 the tail never names the line the region opened on',
        'src/primitives/prune_tasks.rs',
        '            self.opened_at = index;',
        '            self.opened_at = 0;',
        'prune_tasks::tests::an_unclosed_fence_is_reported_and_an_apply_writes_nothing',
    ),
    (
        'M5 a prune-tasks walk drops a spec with a tail and nothing to reduce',
        'src/primitives/prune_tasks.rs',
        '                    && reduction.summary.unexamined_tail.is_none() => {}',
        '                    && true => {}',
        'prune_tasks::tests::a_walk_lists_a_spec_with_a_tail_and_writes_the_others',
    ),
    (
        'M6 a prune-plan walk drops a plan with a tail and no section outside the record',
        'src/primitives/prune_plan.rs',
        '            || summary.unexamined_tail.is_some()\n',
        '',
        'prune_plan::tests::a_walk_lists_a_plan_with_a_tail_and_counts_it',
    ),
    (
        'M7 a prune-tasks walk does not count the specs it held back',
        'src/primitives/prune_tasks.rs',
        '        unexamined_count: features\n            .iter()\n            .filter(|entry| entry.summary.unexamined_tail.is_some())\n            .count(),\n',
        '        unexamined_count: 0,\n',
        'prune_tasks::tests::a_walk_lists_a_spec_with_a_tail_and_writes_the_others',
    ),
    (
        'M8 a prune-plan walk does not count the plans it held back',
        'src/primitives/prune_plan.rs',
        '        unexamined_count: features\n            .iter()\n            .filter(|entry| entry.summary.unexamined_tail.is_some())\n            .count(),\n',
        '        unexamined_count: 0,\n',
        'prune_plan::tests::a_walk_lists_a_plan_with_a_tail_and_counts_it',
    ),
    (
        'M9 a prune-tasks line forgets its tail',
        'src/schema/primitives.rs',
        '            unexamined_tail: summary.unexamined_tail,\n',
        '            unexamined_tail: None,\n',
        'schema::primitives::tests::prune_tasks_unexamined_keys_survive_a_round_trip_in_each_shape',
    ),
]
