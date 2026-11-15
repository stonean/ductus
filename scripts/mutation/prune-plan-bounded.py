"""Mutation list for spec 041's analysis (`BE-PAYLOAD-001`): `prune-plan`
bounds one feature's `sections` through `bounded_lists::place`, and halts on a
malformed read size before an applying call writes. Each mutation drops one
half; the three tests kill them.

Run with `python3 scripts/mutation/run.py scripts/mutation/prune-plan-bounded.py
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
        'M1 a single feature past the read size keeps its sections inline',
        'src/primitives/prune_plan.rs',
        '    if let Some(file) = super::bounded_lists::place(repo, "prune-plan", feature, &summary, &lists)?\n    {\n        summary.form = ListsForm::File;\n        summary.sections = None;\n        summary.lists_file = Some(file);\n    }',
        '    let _ = (&lists, feature);',
        'prune_plan::tests::a_summary_past_the_read_size_moves_its_sections_to_a_file',
    ),
    (
        'M2 a single feature under the read size moves its sections to a file',
        'src/primitives/prune_plan.rs',
        '    if let Some(file) = super::bounded_lists::place(repo, "prune-plan", feature, &summary, &lists)?\n    {',
        '    if let Some(file) = super::bounded_lists::place(repo, "prune-plan", feature, &summary, &lists)?\n        .or(Some(String::from(".ductus/results/x")))\n    {',
        'prune_plan::tests::a_summary_under_the_read_size_keeps_its_sections_inline',
    ),
    (
        'M3 an applying call reads the read size only after its write',
        'src/primitives/prune_plan.rs',
        '    super::check_artifact_size::resolve_threshold(repo)?;\n    let Some(feature) = feature else {',
        '    let Some(feature) = feature else {',
        'prune_plan::tests::a_malformed_read_size_halts_an_applying_call_before_any_write',
    ),
]
