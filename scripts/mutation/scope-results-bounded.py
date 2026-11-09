"""Mutation list for 066's `scope-results-have-a-bounded-size`: a
`compute-review-scope` or `diff-cross-spec` result is inline while it fits
`[artifacts] read-size-bytes`, and past it names a file holding its lists
whole, which the `ductus exec` walker reads every list back from when the
result arrives, halting on a file that lacks a list the result counts, so a
counts-only result never reads as an empty scope
(`runtime/src/primitives/bounded_lists.rs`, `compute_review_scope.rs`,
`diff_cross_spec.rs`, `runtime/src/interpreter/payload.rs`; spec 080 bounded
the other results that grow with the corpus the same way).

Run with `python3 scripts/mutation/run.py scripts/mutation/scope-results-bounded.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. The integration tests the bound also answers
to, the `past_the_read_size` tests in `runtime/tests/walker.rs` and
`runtime/tests/mcp.rs`, fail under M1, run by hand with
`cargo test --locked --test walker --test mcp -- past_the_read_size`.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the bound removed: every result stays inline',
        'src/primitives/bounded_lists.rs',
        '    if u64::try_from(size).unwrap_or(u64::MAX) <= threshold.bytes {',
        '    if true {',
        'bounded_lists::tests::a_result_past_the_read_size_is_written_whole_to_a_file compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list interpreter::tests::a_cross_spec_surface_past_the_read_size_reaches_the_context_through_its_list_file',
    ),
    (
        'M2 a result exactly at the read size no longer fits',
        'src/primitives/bounded_lists.rs',
        '    if u64::try_from(size).unwrap_or(u64::MAX) <= threshold.bytes {',
        '    if u64::try_from(size).unwrap_or(u64::MAX) < threshold.bytes {',
        'bounded_lists::tests::a_result_past_the_read_size_is_written_whole_to_a_file bounded_lists::tests::the_default_read_size_is_fifty_thousand_bytes',
    ),
    (
        'M3 compute-review-scope keeps its lists inline in the file form',
        'src/primitives/compute_review_scope.rs',
        '        result.scope = None;\n        result.modified_since = None;\n        result.plan_affected = None;\n',
        '',
        'compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M4 diff-cross-spec keeps its list inline in the file form',
        'src/primitives/diff_cross_spec.rs',
        '        result.cross_spec_paths = None;\n',
        '',
        'interpreter::tests::a_cross_spec_surface_past_the_read_size_reaches_the_context_through_its_list_file',
    ),
    (
        'M5 the list file holds no lists',
        'src/primitives/bounded_lists.rs',
        '    write_atomic(&dir.join(&name), &body)?;',
        '    write_atomic(&dir.join(&name), "{}\\n")?;',
        'bounded_lists::tests::a_result_past_the_read_size_is_written_whole_to_a_file compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list interpreter::tests::a_cross_spec_surface_past_the_read_size_reaches_the_context_through_its_list_file',
    ),
    (
        'M6 the walker reads no lists back from a list file',
        'src/interpreter/payload.rs',
        '    if map.get("form").and_then(Value::as_str) != Some("file") {',
        '    if true {',
        'interpreter::payload::tests::a_file_form_result_gets_its_lists_back_from_its_list_file',
    ),
    (
        'M7 a missing list file reads as empty lists',
        'src/interpreter/payload.rs',
        '        Contained::Missing => return Err(unreadable("it does not exist".into())),',
        '        Contained::Missing => return Ok(Vec::new()),',
        'interpreter::payload::tests::an_unreadable_list_file_is_an_error_not_an_empty_list',
    ),
    (
        'M8 a list file outside the project reads as empty lists',
        'src/interpreter/payload.rs',
        '        Contained::Outside => return Err(unreadable("it resolves outside the project".into())),',
        '        Contained::Outside => return Ok(Vec::new()),',
        'interpreter::payload::tests::an_unreadable_list_file_is_an_error_not_an_empty_list',
    ),
    (
        'M9 a list file lacking a counted list reads as an empty list',
        'src/interpreter/payload.rs',
        '        if !lists.get(&list).is_some_and(Value::is_array) {',
        '        if false {',
        'interpreter::payload::tests::a_list_file_lacking_a_counted_list_is_an_error_naming_it',
    ),
    (
        'M10 a write prunes list files a concurrent reader may still need',
        'src/primitives/bounded_lists.rs',
        '                        .is_ok_and(|age| age > STALE_AFTER)',
        '                        .is_ok_and(|age| age >= Duration::ZERO)',
        'bounded_lists::tests::prune_stale_removes_only_stale_files',
    ),
    (
        'M11 a write prunes the file it is about to name',
        'src/primitives/bounded_lists.rs',
        '            && entry.file_name() != keep',
        '            && !keep.is_empty()',
        'bounded_lists::tests::prune_stale_removes_only_stale_files',
    ),
    (
        'M12 the results directory does not ignore itself, so a list file joins the next scope',
        'src/primitives/bounded_lists.rs',
        '    fix_rounds::ignore_everything(&repo.join(RESULTS_DIR))?;',
        '    let _ = fix_rounds::ignore_everything;',
        'bounded_lists::tests::a_result_past_the_read_size_is_written_whole_to_a_file compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M13 two calls with different lists write one file',
        'src/primitives/bounded_lists.rs',
        '    let name = format!("{}.json", super::hex(&Sha256::digest(body.as_bytes())));',
        '    let name = format!("{}.json", super::hex(&Sha256::digest(b"lists")));',
        'bounded_lists::tests::a_file_is_named_by_its_content',
    ),
    (
        'M14 a write no longer prunes the feature\'s stale list files',
        'src/primitives/bounded_lists.rs',
        '    prune_stale(&dir, &name, SystemTime::now());',
        '    let _ = prune_stale;',
        'bounded_lists::tests::a_call_past_the_read_size_prunes_stale_files',
    ),
    (
        'M15 a results directory linked outside the project is written through',
        'src/primitives/bounded_lists.rs',
        '    super::validate_contained(repo, &dir_rel)?;',
        '    let _ = super::validate_contained;',
        'bounded_lists::tests::a_results_directory_linked_outside_the_project_is_refused',
    ),
    (
        'M16 a list file that cannot be written still names its path',
        'src/primitives/bounded_lists.rs',
        [
            '    std::fs::create_dir_all(&dir).map_err(|source| PrimitiveError::Io {\n        path: dir.clone(),\n        source,\n    })?;',
            '    write_atomic(&dir.join(&name), &body)?;',
        ],
        [
            '    let _ = std::fs::create_dir_all(&dir);',
            '    let _ = write_atomic(&dir.join(&name), &body);',
        ],
        'bounded_lists::tests::a_file_that_cannot_be_written_fails_the_call',
    ),
    (
        'M17 a list file the write cannot land still names its path',
        'src/primitives/bounded_lists.rs',
        '    write_atomic(&dir.join(&name), &body)?;',
        '    let _ = write_atomic(&dir.join(&name), &body);',
        'bounded_lists::tests::a_file_that_cannot_be_written_fails_the_call',
    ),
    (
        'M18 diff-cross-spec\'s count names no list, so its paths go unchecked',
        'src/primitives/bounded_lists.rs',
        '        Some("cross-spec-paths"),',
        '        None,',
        'interpreter::payload::tests::a_list_file_lacking_a_counted_list_is_an_error_naming_it bounded_lists::tests::a_result_counts_the_lists_its_counts_name',
    ),
    (
        'M19 prune-tasks\' removed-count names a list, halting a whole file',
        'src/primitives/bounded_lists.rs',
        '    ("prune-tasks", "removed-count", None),\n',
        '',
        'interpreter::payload::tests::a_list_file_lacking_a_counted_list_is_an_error_naming_it prune_tasks::tests::a_summary_past_the_read_size_moves_its_sections_to_a_file bounded_lists::tests::a_result_counts_the_lists_its_counts_name',
    ),
    (
        'M20 measure-reopenings\' closed-count names a list, halting a whole file',
        'src/primitives/bounded_lists.rs',
        '    ("measure-reopenings", "closed-count", None),\n',
        '',
        'measure_reopenings::tests::a_result_past_the_read_size_moves_its_lists_to_a_file',
    ),
    (
        'M21 a counted list the result carries inline is owed by the file too',
        'src/primitives/bounded_lists.rs',
        '        .filter(|list| !result.contains_key(list))\n',
        '',
        'bounded_lists::tests::a_result_counts_the_lists_its_counts_name',
    ),
]
