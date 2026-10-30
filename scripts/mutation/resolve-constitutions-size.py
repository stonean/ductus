"""Mutation list for spec 055's `target-reports-each-constitutions-size`: the
measured entry of `resolve-constitutions`
(`runtime/src/primitives/resolve_constitutions.rs`, `run_measured`), which
measures each loaded shared constitution against `[artifacts] read-size-bytes`
while `write-review` and `write-analysis` keep the unmeasured `run`.

Run with `python3 scripts/mutation/run.py
scripts/mutation/resolve-constitutions-size.py [ID ...]`; see run.py for the
format. This file is data, read with ast.literal_eval: keep every value a
literal. An id is never reused: a mutation that no longer applies is removed,
and the next one takes the next free number. When the code a mutation anchors
on changes, update the anchor in the same change; `run.py --check` names every
anchor that no longer matches exactly once.

run.py runs unit tests only. That each dispatch surface (the CLI subcommand,
the MCP tool and the exec walker) reaches `run_measured` is held by
`runtime/tests/resolve_constitutions_size.rs`, an integration test this
harness does not run.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the loaded documents are not measured',
        'src/primitives/resolve_constitutions.rs',
        '    for record in &mut result.loaded {\n        measure(repo, record, threshold.bytes);\n    }\n',
        '',
        'resolve_constitutions::tests::a_constitution_one_byte_over_the_limit_is_named_with_its_repository_as_the_fix',
    ),
    (
        'M2 a document at the limit is over it',
        'src/primitives/resolve_constitutions.rs',
        '    let over = bytes > limit;',
        '    let over = bytes >= limit;',
        'resolve_constitutions::tests::a_constitution_one_byte_over_the_limit_is_named_with_its_repository_as_the_fix',
    ),
    (
        'M3 the documents are measured against the default, not the configured read size',
        'src/primitives/resolve_constitutions.rs',
        '        measure(repo, record, threshold.bytes);',
        '        measure(repo, record, crate::primitives::check_artifact_size::DEFAULT_READ_SIZE_BYTES);',
        'resolve_constitutions::tests::a_constitution_one_byte_over_the_limit_is_named_with_its_repository_as_the_fix',
    ),
    (
        'M4 the warning names no owning repository as the fix',
        'src/primitives/resolve_constitutions.rs',
        ['Fix: cut it in its own repository, {}; ', '        flatten_line(&record.repo),\n'],
        ['Fix: cut it; ', ''],
        'resolve_constitutions::tests::a_constitution_one_byte_over_the_limit_is_named_with_its_repository_as_the_fix',
    ),
    (
        'M5 the result does not carry the threshold it measured against',
        'src/primitives/resolve_constitutions.rs',
        '    result.threshold = Some(threshold);\n',
        '',
        'resolve_constitutions::tests::a_constitution_one_byte_over_the_limit_is_named_with_its_repository_as_the_fix',
    ),
    (
        'M6 a skipped entry is measured, and an unmeasurable document reads as zero bytes',
        'src/primitives/resolve_constitutions.rs',
        [
            '    for record in &mut result.loaded {',
            '    let Some(bytes) = length_on_disk(&document) else {\n        return;\n    };',
        ],
        [
            '    for record in result.loaded.iter_mut().chain(result.skipped.iter_mut()) {',
            '    let bytes = length_on_disk(&document).unwrap_or(0);',
        ],
        'resolve_constitutions::tests::a_skipped_entry_carries_no_size',
    ),
    (
        'M7 a loaded document that cannot be opened reads as zero bytes, under the limit',
        'src/primitives/resolve_constitutions.rs',
        '    let Some(bytes) = length_on_disk(&document) else {\n        return;\n    };',
        '    let bytes = length_on_disk(&document).unwrap_or(0);',
        'resolve_constitutions::tests::a_loaded_document_that_cannot_be_opened_is_not_measured',
    ),
    (
        'M8 a project with nothing registered still resolves the read size',
        'src/primitives/resolve_constitutions.rs',
        '    if args.unmeasured || result.examined == 0 {\n        return Ok(result);\n    }\n',
        '',
        'resolve_constitutions::tests::with_nothing_registered_the_result_carries_no_new_key',
    ),
    (
        'M9 an absent threshold is serialized as a null key',
        'src/schema/primitives.rs',
        '    #[serde(default, skip_serializing_if = "Option::is_none")]\n    pub threshold: Option<ReadSizeThreshold>,',
        '    #[serde(default)]\n    pub threshold: Option<ReadSizeThreshold>,',
        'resolve_constitutions::tests::with_nothing_registered_the_result_carries_no_new_key',
    ),
    (
        'M10 a malformed read size falls back to the default instead of halting',
        'src/primitives/resolve_constitutions.rs',
        '    let (threshold, notices) = resolve_threshold(repo)?;',
        '    let (threshold, notices) = resolve_threshold(repo).unwrap_or((crate::schema::primitives::ReadSizeThreshold { bytes: 50_000, source: crate::schema::primitives::ThresholdSource::Default }, Vec::new()));',
        'resolve_constitutions::tests::a_malformed_read_size_halts_the_measured_entry',
    ),
    (
        'M11 the shared resolution reads the read size, so a malformed one reaches write-review and write-analysis',
        'src/primitives/resolve_constitutions.rs',
        '    let registry = load_constitutions(repo)?;',
        '    resolve_threshold(repo)?;\n    let registry = load_constitutions(repo)?;',
        'resolve_constitutions::tests::write_review_and_write_analysis_keep_the_unmeasured_resolution',
    ),
    (
        'M12 only the first alias on a checkout is measured',
        'src/primitives/resolve_constitutions.rs',
        '    for record in &mut result.loaded {',
        '    for record in result.loaded.iter_mut().take(1) {',
        'resolve_constitutions::tests::two_aliases_on_one_checkout_both_report_its_size',
    ),
    (
        'M13 an unmeasured call still measures',
        'src/primitives/resolve_constitutions.rs',
        '    if args.unmeasured || result.examined == 0 {',
        '    if result.examined == 0 {',
        'resolve_constitutions::tests::an_unmeasured_call_reads_no_read_size',
    ),
]
