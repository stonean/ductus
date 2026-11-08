"""Mutation list for spec 030's registry and `resolve-references`' result:
`duplicate-repos`, each repo the `[services]` registry holds under two or more
aliases, grouped by `Services::duplicate_repos` (`runtime/src/schema/services.rs`)
under the harvester's own identity, `normalize_repo` (M1, M2); the registry's
validation of an absent or empty `repo` or `path`, which every reader shares
through `resolve_references::load_services` (M3, M4); `derive-references`
halting on a registry it cannot read (M5); and the bound on
`resolve-references`' result (M6). M1 and M2 are task 14's, M3 to M6 task 15's.

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
]
