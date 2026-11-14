"""Mutation list for 063's task 17: `check-artifact-size`'s `oversized`,
`skipped` and `notices` are inline while the result fits `[artifacts]
read-size-bytes`, and past it leave the result for the one file `lists-file`
names, keyed by the feature or by `project-files`, with each count inline in
both forms (BE-PAYLOAD-001; `runtime/src/primitives/check_artifact_size.rs`,
through `bounded_lists.rs`, whose own mutations are in
`scope-results-bounded.py`).

Run with `python3 scripts/mutation/run.py scripts/mutation/artifact-size-bounded.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. Three integration tests answer to the bound as
well, run by hand:

- `an_artifact_size_result_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp`
  in `runtime/tests/mcp.rs` fails under M1 (`cargo test --locked --test mcp --
  artifact_size_result_past`).
- `an_unmeasurable_staged_file_exits_1_and_names_it_in_either_form` in
  `runtime/tests/check_artifact_size_cli.rs` fails when `size_check_blocks` in
  `runtime/src/main.rs` reads `skipped` in place of `skipped_count`
  (`!result.skipped.as_ref().is_none_or(Vec::is_empty)`, the hook passing a
  file it could not measure once the lists move), and when
  `report_refused_project_files` names the unreadable files from `skipped`
  in place of `project-files` (`cargo test --locked --test
  check_artifact_size_cli -- unmeasurable`). main.rs is the binary, which no
  unit test run here compiles, so neither is a mutation below.
- `an_oversized_file_without_staged_is_a_finding_never_an_exit_status` in the
  same file fails under M9.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the bound removed: every check-artifact-size result keeps its lists inline',
        'src/primitives/check_artifact_size.rs',
        '    if let Some(file) = bounded_lists::place(repo, PRIMITIVE, key, &result, &lists)? {',
        '    if let Some(file) = bounded_lists::place(repo, PRIMITIVE, key, &result, &lists)?.filter(|_| false) {',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M2 oversized stays inline in the file form',
        'src/primitives/check_artifact_size.rs',
        '        result.oversized = None;\n',
        '',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M3 skipped stays inline in the file form',
        'src/primitives/check_artifact_size.rs',
        '        result.skipped = None;\n',
        '',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M4 notices stay inline in the file form',
        'src/primitives/check_artifact_size.rs',
        '        result.notices = None;\n',
        '',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M5 oversized-count counts nothing',
        'src/primitives/check_artifact_size.rs',
        '        oversized_count: measured.oversized.len(),\n',
        '        oversized_count: 0,\n',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M6 skipped-count counts nothing, so a file-form skip reads as none',
        'src/primitives/check_artifact_size.rs',
        '        skipped_count: measured.skipped.len(),\n',
        '        skipped_count: 0,\n',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M7 notices-count counts nothing',
        'src/primitives/check_artifact_size.rs',
        '        notices_count: measured.notices.len(),\n',
        '        notices_count: 0,\n',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M8 the list file lacks notices',
        'src/primitives/check_artifact_size.rs',
        '        "notices": measured.notices,\n',
        '',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M9 a call naming no feature is keyed as a corpus-wide one',
        'src/primitives/check_artifact_size.rs',
        '    let key = args.feature.as_deref().unwrap_or(PROJECT_FILES_KEY);\n',
        '    let key = args.feature.as_deref().unwrap_or("all-specs");\n',
        'check_artifact_size::tests::a_file_form_call_is_keyed_by_its_feature_or_by_the_project_files',
    ),
    (
        'M10 a call naming a feature is keyed by the project files',
        'src/primitives/check_artifact_size.rs',
        '    let key = args.feature.as_deref().unwrap_or(PROJECT_FILES_KEY);\n',
        '    let key = PROJECT_FILES_KEY;\n',
        'check_artifact_size::tests::a_file_form_call_is_keyed_by_its_feature_or_by_the_project_files',
    ),
    (
        'M11 the file form names no list file',
        'src/primitives/check_artifact_size.rs',
        '        result.lists_file = Some(file);\n',
        '        let _ = file;\n',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M12 the file form still says inline',
        'src/primitives/check_artifact_size.rs',
        '        result.form = ListsForm::File;\n',
        '',
        'check_artifact_size::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M13 an absent oversized is serialized, as null',
        'src/schema/primitives.rs',
        '    #[serde(default, skip_serializing_if = "Option::is_none")]\n    pub oversized: Option<Vec<OversizedArtifact>>,',
        '    #[serde(default)]\n    pub oversized: Option<Vec<OversizedArtifact>>,',
        'schema::primitives::tests::check_artifact_size_round_trip',
    ),
    (
        'M14 the walker does not restore check-artifact-size\'s file form before the tally reads it',
        'src/interpreter/payload.rs',
        '    if map.get("form").and_then(Value::as_str) != Some("file") {\n        return Ok(Vec::new());\n    }\n    let file = map',
        '    if map.get("form").and_then(Value::as_str) != Some("file") || primitive == "check-artifact-size" {\n        return Ok(Vec::new());\n    }\n    let file = map',
        'interpreter::tests::an_exec_analyze_counts_an_oversized_artifact_held_in_the_file_form',
    ),
]
