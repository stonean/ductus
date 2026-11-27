"""Mutation list for spec 066 task 32: a ticked criterion a mechanical sweep alone
changed since the run that judged it keeps its verdict, by the one rule
`mechanical_sweep` applies (`SweepIndex::respells`), and a criterion the sweep
cannot vouch for still owes a review. Each mutation widens or drops one part of
that rule; a test kills each.

Run with `python3 scripts/mutation/run.py scripts/mutation/criterion-sweep-066.py
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
        'M1 an index whose diff was not read vouches for a text',
        'src/primitives/mechanical_sweep.rs',
        '        if !self.read {\n            return false;\n        }\n',
        '',
        'primitives::mechanical_sweep::tests::a_text_is_respelled_only_by_a_repo_wide_rewrite',
    ),
    (
        'M2 a rewrite no other file carries respells a text',
        'src/primitives/mechanical_sweep.rs',
        '                .all(|pair| self.repo_wide.contains(pair) || self.explained_by(pair))\n',
        '                .all(|_| true)\n',
        'primitives::mechanical_sweep::tests::a_text_is_respelled_only_by_a_repo_wide_rewrite primitives::compute_review_scope::tests::a_criterion_a_sweep_alone_changed_keeps_its_verdict',
    ),
    (
        'M3 a swept criterion keeps no verdict',
        'src/primitives/criterion_verdicts.rs',
        '    let respelled = respelled_since_the_run(repo, (spec_rel, record_rel), record, &criteria);\n',
        '    let respelled: BTreeSet<String> = BTreeSet::new();\n',
        'primitives::compute_review_scope::tests::a_criterion_a_sweep_alone_changed_keeps_its_verdict primitives::compute_review_scope::tests::a_history_cut_short_leaves_a_swept_criterion_owing',
    ),
    (
        'M4 an uncommitted edit over a committed sweep keeps the verdict',
        'src/primitives/criterion_verdicts.rs',
        '            (text(&committed) == text(criterion)\n',
        '            (true\n',
        'primitives::compute_review_scope::tests::a_criterion_owes_a_review_where_the_sweep_cannot_vouch_for_it',
    ),
    (
        'M5 a text the record holds no verdict for keeps one once swept',
        'src/primitives/criterion_verdicts.rs',
        '                && judged.contains(digest(&then).as_str())\n',
        '',
        'primitives::compute_review_scope::tests::a_criterion_a_sweep_alone_changed_keeps_its_verdict',
    ),
]
