"""Mutation list for spec 030's registry and `resolve-references`' result:
`duplicate-repos`, each repo the `[services]` registry holds under two or more
aliases, grouped by `Services::duplicate_repos` (`runtime/src/schema/services.rs`)
under the harvester's own identity, `normalize_repo` (M1, M2); the registry's
validation of an absent or empty `repo` or `path`, which every reader shares
through `resolve_references::load_services` (M3, M4); `derive-references`
halting on a registry it cannot read (M5); and the bound on
`resolve-references`' result (M6); and `register-service`
(`runtime/src/primitives/register_service.rs`), `/{project}:link`'s write: its
field and duplicate-alias refusals (M7, M9 to M11), the read-back (M8), the
append's seam and line endings (M12 to M14), escaping (M15, M16), the empty
description (M17), its warnings (M18, M19), the guarded write (M20), the
active file (M21) and `created` (M22). M1 and M2 are task 14's, M3 to M6 task
15's, M7 to M22 task 16's.

Run with `python3 scripts/mutation/run.py
scripts/mutation/cross-service-references.py [ID ...]`; see run.py for the
format. This file is data, read with ast.literal_eval: keep every value a
literal. An id is never reused: a mutation that no longer applies is removed,
and the next one takes the next free number. When the code a mutation anchors
on changes, update the anchor in the same change; `run.py --check` names every
anchor that no longer matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the registry\'s duplicated repos are not reported',
        'src/primitives/resolve_references.rs',
        '    let duplicate_repos = services\n        .duplicate_repos()\n        .into_iter()\n        .map(|(repo, aliases)| DuplicateRepo { repo, aliases })\n        .collect();\n',
        '    let duplicate_repos = Vec::new();\n',
        'resolve_references::tests::a_repo_under_two_aliases_is_reported_with_both',
    ),
    (
        'M2 duplicated repos are grouped by the raw string, not the harvester\'s identity',
        'src/schema/services.rs',
        '                .entry(normalize_repo(&entry.repo))\n',
        '                .entry(entry.repo.clone())\n',
        'resolve_references::tests::a_repo_under_two_aliases_is_reported_with_both',
    ),
    (
        'M3 an entry with an absent or empty repo or path is accepted',
        'src/schema/services.rs',
        '        registry.validate()?;\n',
        '',
        'resolve_references::tests::an_entry_without_a_path_halts_naming_the_alias_the_field_and_the_file',
    ),
    (
        'M4 a whitespace-only repo is not read as empty',
        'src/schema/services.rs',
        '            if entry.repo.trim().is_empty() {',
        '            if entry.repo.is_empty() {',
        'services::tests::an_absent_or_empty_repo_or_path_names_the_alias_and_the_field',
    ),
    (
        'M5 derive-references reads a registry it cannot read as no registry',
        'src/primitives/derive_references.rs',
        '    let services = load_services(repo)?;\n    let mut registry = BTreeMap::new();',
        '    let services = load_services(repo).unwrap_or_default();\n    let mut registry = BTreeMap::new();',
        'derive_references::tests::a_registry_that_cannot_be_read_halts_and_writes_nothing',
    ),
    (
        'M6 resolve-references returns its lists inline past the read size',
        'src/primitives/resolve_references.rs',
        '        super::bounded_lists::place(repo, "resolve-references", &args.feature, &result, &lists)?\n',
        '        None::<String>\n',
        'resolve_references::tests::a_result_past_the_read_size_names_a_file_holding_both_lists',
    ),
    (
        'M7 an alias the registry already holds is written over',
        'src/primitives/register_service.rs',
        '    if before.0.contains_key(&args.alias) {',
        '    if false {',
        'register_service::tests::a_registered_alias_is_refused_before_any_write',
    ),
    (
        'M8 the appended table is not read back before the write',
        'src/primitives/register_service.rs',
        '    match Services::from_toml_str(&content) {',
        '    match Ok::<Services, crate::schema::services::ServicesError>(expected.clone()) {',
        'register_service::tests::an_inline_table_services_is_refused',
    ),
    (
        'M9 an alias that is no bare TOML key is not refused as an argument',
        'src/primitives/register_service.rs',
        '    if !alias\n        .chars()',
        '    if false && !alias\n        .chars()',
        'register_service::tests::a_rejected_field_is_refused_naming_it',
    ),
    (
        'M10 a repo that is not URL-shaped is accepted',
        'src/primitives/register_service.rs',
        '    if !shaped || repo',
        '    if false && repo',
        'register_service::tests::a_rejected_field_is_refused_naming_it',
    ),
    (
        'M11 an empty path is not refused as an argument',
        'src/primitives/register_service.rs',
        '    if path.trim().is_empty() {',
        '    if path.is_empty() && false {',
        'register_service::tests::a_rejected_field_is_refused_naming_it',
    ),
    (
        'M12 no blank line separates the table from the file before it',
        'src/primitives/register_service.rs',
        '    if !blank_already {',
        '    if false && !blank_already {',
        'register_service::tests::appends_the_table_and_keeps_every_other_byte',
    ),
    (
        'M13 an unterminated last line is not closed',
        'src/primitives/register_service.rs',
        '    if !out.is_empty() && !out.ends_with',
        '    if false && !out.ends_with',
        'register_service::tests::an_unterminated_last_line_is_closed_before_the_blank_line',
    ),
    (
        'M14 the table is written in LF whatever the file uses',
        'src/primitives/register_service.rs',
        '    let ending = existing.map_or("\\n", super::line_ending_of);',
        '    let ending = existing.map_or("\\n", |_| "\\n");',
        'register_service::tests::a_crlf_config_gets_a_crlf_table',
    ),
    (
        'M15 a double quote in a value is not escaped',
        'src/primitives/register_service.rs',
        '            \'"\' => out.push_str("\\\\\\""),',
        '            \'"\' => out.push(\'"\'),',
        'register_service::tests::values_are_escaped_and_read_back_as_given',
    ),
    (
        'M16 a control character in a value is written raw',
        'src/primitives/register_service.rs',
        '            c if c.is_control() => {',
        '            c if c.is_control() && false => {',
        'register_service::tests::values_are_escaped_and_read_back_as_given',
    ),
    (
        'M17 an empty description is written',
        'src/primitives/register_service.rs',
        '        .filter(|text| !text.trim().is_empty());',
        '        .filter(|_| true);',
        'register_service::tests::an_empty_description_writes_none',
    ),
    (
        'M18 a duplicate repo is compared as the raw string',
        'src/primitives/register_service.rs',
        '        .filter(|(_, entry)| normalize_repo(&entry.repo) == identity)',
        '        .filter(|(_, entry)| entry.repo == args.repo && !identity.is_empty())',
        'register_service::tests::a_duplicate_repo_and_a_missing_checkout_warn_without_refusing',
    ),
    (
        'M19 an existing checkout is warned of as missing',
        'src/primitives/register_service.rs',
        '    if !resolve_path(repo, &args.path).is_dir() {',
        '    if true {',
        'register_service::tests::an_existing_checkout_and_a_new_repo_warn_nothing',
    ),
    (
        'M20 the write is not guarded against a change since the read',
        'src/primitives/register_service.rs',
        '    guarded::write_guarded(&snapshot, &content)?;',
        '    super::write_atomic(&repo.join(&rel), &content)?;',
        'register_service::tests::a_config_changed_after_the_read_is_refused_and_kept',
    ),
    (
        'M21 the table goes to .ductus/config.toml beside a legacy file',
        'src/primitives/register_service.rs',
        '    let config = paths::config_path_for_write(repo);',
        '    let config = repo.join(".ductus/config.toml");',
        'register_service::tests::writes_the_legacy_file_when_only_it_exists',
    ),
    (
        'M22 a created config is reported as existing',
        'src/primitives/register_service.rs',
        '        created: existing.is_none(),',
        '        created: false,',
        'register_service::tests::creates_the_config_when_none_exists',
    ),
]
