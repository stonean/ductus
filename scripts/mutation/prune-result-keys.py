"""Mutation list for spec 041's review: `prune-tasks` and `prune-plan` return
one struct of two flattened `Option` halves, a one-feature summary and a walk,
and `form` and `lists-file` belong to the result, never to a half, because
serde's flatten consumes a shared key even when that half fails to deserialize.
Each mutation reintroduces a loss of `form` or `lists-file` on a round trip, or
a bound judged on less than the result is sent as, and the tests kill it.

Run with `python3 scripts/mutation/run.py scripts/mutation/prune-result-keys.py
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
        'M1 prune-tasks result: `form` is not read back',
        'src/schema/primitives.rs',
        "    pub walk: Option<PruneWalk<PruneTasksLine>>,\n    /// Where the half's lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default)]\n    pub form: ListsForm,\n",
        "    pub walk: Option<PruneWalk<PruneTasksLine>>,\n    /// Where the half's lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default, skip_deserializing)]\n    pub form: ListsForm,\n",
        'schema::primitives::tests::prune_tasks_result_file_form_survives_a_round_trip',
    ),
    (
        'M2 prune-tasks result: `lists-file` is not read back',
        'src/schema/primitives.rs',
        '    pub walk: Option<PruneWalk<PruneTasksLine>>,\n    /// Where the half\'s lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default)]\n    pub form: ListsForm,\n    /// In the file form, the file holding the lists whole, from the project\n    /// root: a JSON object with their keys.\n    #[serde(default, skip_serializing_if = "Option::is_none")]\n    pub lists_file: Option<String>,\n',
        '    pub walk: Option<PruneWalk<PruneTasksLine>>,\n    /// Where the half\'s lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default)]\n    pub form: ListsForm,\n    /// In the file form, the file holding the lists whole, from the project\n    /// root: a JSON object with their keys.\n    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]\n    pub lists_file: Option<String>,\n',
        'schema::primitives::tests::prune_tasks_result_file_form_survives_a_round_trip',
    ),
    (
        'M3 prune-tasks result: `form` is renamed, so a half can consume the key',
        'src/schema/primitives.rs',
        "    pub walk: Option<PruneWalk<PruneTasksLine>>,\n    /// Where the half's lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default)]\n    pub form: ListsForm,\n",
        '    pub walk: Option<PruneWalk<PruneTasksLine>>,\n    /// Where the half\'s lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default, rename = "list-form")]\n    pub form: ListsForm,\n',
        'schema::primitives::tests::prune_tasks_result_file_form_survives_a_round_trip',
    ),
    (
        'M4 prune-plan result: `form` is not read back',
        'src/schema/primitives.rs',
        "    pub walk: Option<PruneWalk<PrunePlanSummary>>,\n    /// Where the half's lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default)]\n    pub form: ListsForm,\n",
        "    pub walk: Option<PruneWalk<PrunePlanSummary>>,\n    /// Where the half's lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default, skip_deserializing)]\n    pub form: ListsForm,\n",
        'schema::primitives::tests::prune_plan_result_file_form_survives_a_round_trip',
    ),
    (
        'M5 prune-plan result: `lists-file` is not read back',
        'src/schema/primitives.rs',
        '    pub walk: Option<PruneWalk<PrunePlanSummary>>,\n    /// Where the half\'s lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default)]\n    pub form: ListsForm,\n    /// In the file form, the file holding the lists whole, from the project\n    /// root: a JSON object with their keys.\n    #[serde(default, skip_serializing_if = "Option::is_none")]\n    pub lists_file: Option<String>,\n',
        '    pub walk: Option<PruneWalk<PrunePlanSummary>>,\n    /// Where the half\'s lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default)]\n    pub form: ListsForm,\n    /// In the file form, the file holding the lists whole, from the project\n    /// root: a JSON object with their keys.\n    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]\n    pub lists_file: Option<String>,\n',
        'schema::primitives::tests::prune_plan_result_file_form_survives_a_round_trip',
    ),
    (
        'M6 prune-plan result: `form` is renamed, so a half can consume the key',
        'src/schema/primitives.rs',
        "    pub walk: Option<PruneWalk<PrunePlanSummary>>,\n    /// Where the half's lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default)]\n    pub form: ListsForm,\n",
        '    pub walk: Option<PruneWalk<PrunePlanSummary>>,\n    /// Where the half\'s lists are (`sections`; under `all`, `features` and\n    /// `skipped`): in this result, or past `[artifacts] read-size-bytes` in\n    /// the file `lists-file` names (spec 080, AC7).\n    #[serde(default, rename = "list-form")]\n    pub form: ListsForm,\n',
        'schema::primitives::tests::prune_plan_result_file_form_survives_a_round_trip',
    ),
    (
        'M7 the bound judges the lists, not the result as it is sent',
        'src/primitives/prune_tasks.rs',
        '    if let Some(file) = super::bounded_lists::place(repo, primitive, key, &*result, &lists)? {',
        '    if let Some(file) = super::bounded_lists::place(repo, primitive, key, &lists, &lists)? {',
        'prune_tasks::tests::a_result_is_held_to_the_read_size_as_it_is_sent prune_plan::tests::a_result_is_held_to_the_read_size_as_it_is_sent',
    ),
    (
        'M8 a prune-tasks walk past the read size keeps its `features` inline',
        'src/primitives/prune_tasks.rs',
        '            walk.features = None;\n',
        '',
        'prune_tasks::tests::a_walk_past_the_read_size_moves_its_lists_to_a_file',
    ),
    (
        'M9 a prune-plan walk past the read size keeps its `features` inline',
        'src/primitives/prune_plan.rs',
        '            walk.features = None;\n',
        '',
        'prune_plan::tests::a_walk_past_the_read_size_moves_its_lists_to_a_file',
    ),
]
