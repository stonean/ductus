"""Mutation list for spec 041's `a-plan-edit-does-not-reopen-a-done-spec`: an
edit confined to a `done` spec's `plan.md` or `tasks.md` that adds no unchecked
task takes no back-edge (§spec-lifecycle), and every surface that
computes a reopen or a record's freshness agrees — `prune-plan`'s
`reopen-required` (`runtime/src/primitives/prune_plan.rs`), analyze freshness
(`runtime/src/primitives/analyze_subjects.rs`, 066), and `check-artifact-size`'s
advice (`runtime/src/primitives/check_artifact_size.rs`, 063).

Run with `python3 scripts/mutation/run.py scripts/mutation/plan-edit-takes-no-back-edge.py
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
        'M1 a plan that differs from HEAD reopens a done spec again',
        'src/primitives/prune_plan.rs',
        '    let project = ProjectRepository::discover(repo)?;\n    if let Some(head) = project.read_at_head(tasks_rel)? {',
        '    let project = ProjectRepository::discover(repo)?;\n    let plan_rel = tasks_rel.replace("tasks.md", "plan.md");\n    if project.read_at_head(&plan_rel)?.is_some_and(|head| head != read_text(&repo.join(&plan_rel)).unwrap_or_default()) {\n        return Ok(true);\n    }\n    if let Some(head) = project.read_at_head(tasks_rel)? {',
        'prune_plan::tests::a_changed_design_record_section_does_not_reopen',
    ),
    (
        'M2 a done spec is judged on every analyze subject',
        'src/primitives/analyze_subjects.rs',
        '    if done {\n        without_plan_edits(repo, rel_dir, freshness)\n    } else {\n        freshness\n    }',
        '    let _ = done;\n    freshness',
        'analyze_subjects::tests::a_plan_or_tasks_edit_does_not_stale_a_done_specs_record',
    ),
    (
        'M3 a deleted plan on a done spec is dropped like an edit in place',
        'src/primitives/analyze_subjects.rs',
        '            .any(|name| *path == format!("{rel_dir}/{name}") && repo.join(path).is_file())',
        '            .any(|name| *path == format!("{rel_dir}/{name}"))',
        'analyze_subjects::tests::a_done_specs_deleted_plan_still_stales_its_record',
    ),
    (
        'M4 trimming a done spec plan reopens it unless the trim changes no claim',
        'src/primitives/check_artifact_size.rs',
        '        (_, FixKind::Prune) | (ArtifactKind::Plan, FixKind::Trim) => OnDone::Never,',
        '        (_, FixKind::Prune) => OnDone::Never,',
        'check_artifact_size::tests::the_effect_on_done_is_named_only_at_done',
    ),
    (
        'M5 a data-model trim on a done spec is exempt like the plan trim',
        'src/primitives/check_artifact_size.rs',
        '        (_, FixKind::Prune) | (ArtifactKind::Plan, FixKind::Trim) => OnDone::Never,',
        '        (_, FixKind::Prune) | (ArtifactKind::Plan | ArtifactKind::DataModel, FixKind::Trim) => OnDone::Never,',
        'check_artifact_size::tests::a_data_model_trim_still_reopens_a_done_spec_unless_no_claim_changes',
    ),
]
