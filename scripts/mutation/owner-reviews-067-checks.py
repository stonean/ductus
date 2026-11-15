"""Mutation list for the part of spec 067's review fix round that corrects
`check-artifacts`' contract breaks (the placeholder test wholly inside one
bracketed span, a branch-scoped plan's number, the CommonMark list markers,
the completeness family and the scenario listing behind a link that leaves
the project, `resolve_body_path`'s `../` test) and pins the behaviors the
round's tests newly name (`resolve-anchor`'s malformed registry, the segment
a list file sits under, the exec walker reading a list file back before the
tally).

Run with `python3 scripts/mutation/run.py scripts/mutation/owner-reviews-067-checks.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. The ids start at M301, clear of
the lists that came before. Three edits stay unproven by design: the
`!outward.covers(path) && path.is_file()` reorders and the `§` line skip in
`resolve-anchor` change no result, only whether a file is probed or opened,
which no assertion sees.
"""

EXPECTED_SURVIVORS = {}


MUTATIONS = [
    (
        'M301 a criterion that starts and ends with brackets is a placeholder',
        'src/primitives/check_artifacts.rs',
        '            .is_some_and(|inner| !inner.contains(open) && !inner.contains(close))',
        '            .is_some_and(|_| true)',
        'check_artifacts::tests::an_empty_or_placeholder_criterion_is_blocking',
    ),
    (
        'M302 a branch-scoped plan is matched by the part before the first hyphen',
        'src/primitives/check_artifacts.rs',
        '            super::parse_feature_dir(feature)\n        {',
        '            super::parse_feature_dir(feature).filter(|_| false)\n        {',
        'check_artifacts::tests::a_branch_scoped_plan_is_matched_by_its_identifier_and_counter',
    ),
    (
        'M303 a number inside a longer token on its left names the spec',
        'src/primitives/check_artifacts.rs',
        "        !before.is_some_and(|c| c.is_alphanumeric() || matches!(c, '-' | '.'))\n            && ",
        '        ',
        'check_artifacts::tests::a_branch_scoped_plan_is_matched_by_its_identifier_and_counter',
    ),
    (
        'M304 a number inside a longer token on its right names the spec',
        'src/primitives/check_artifacts.rs',
        '            && !after.is_some_and(char::is_alphanumeric)',
        '            && true',
        'check_artifacts::tests::a_branch_scoped_plan_is_matched_by_its_identifier_and_counter',
    ),
    (
        'M305 a plus bullet is no list item',
        'src/primitives/check_artifacts.rs',
        '            .or_else(|| trimmed.strip_prefix("+ "))\n',
        '',
        'check_artifacts::tests::affected_files_in_any_spelling_list_a_file',
    ),
    (
        'M306 a numbered item closed by a parenthesis is no list item',
        'src/primitives/check_artifacts.rs',
        '                            .or_else(|| digits.strip_prefix(") "))\n',
        '',
        'check_artifacts::tests::affected_files_in_any_spelling_list_a_file',
    ),
    (
        'M307 completeness calls a link that leaves the project a missing file',
        'src/primitives/check_artifacts.rs',
        '        if outward.covers(&target) {\n            continue;',
        '        if false {\n            continue;',
        'check_artifacts::tests::a_dangling_link_out_of_the_project_is_no_missing_artifact',
    ),
    (
        'M308 a scenarios directory behind a link that leaves the project is listed',
        'src/primitives/check_artifacts.rs',
        '    if outward.covers(scenarios_dir) {\n        return Vec::new();',
        '    if false {\n        return Vec::new();',
        'check_artifacts::tests::a_scenarios_directory_linked_outside_the_project_is_not_listed',
    ),
    (
        'M318 read-spec lists a scenarios directory its caller covers',
        'src/primitives/read_spec.rs',
        '    if covered(&scenarios_dir) {\n        return ScenarioQuestionScan {',
        '    if false {\n        return ScenarioQuestionScan {',
        'read_spec::tests::a_covered_scenarios_directory_is_never_listed',
    ),
    (
        'M309 a name beginning with two dots resolves from the citing directory',
        'src/primitives/check_artifacts.rs',
        '    let base = if file_part == ".." || file_part.starts_with("../") {',
        '    let base = if file_part.starts_with("..") {',
        'check_artifacts::tests::only_a_dotdot_segment_resolves_from_the_citing_directory',
    ),
    (
        'M310 tbc is no placeholder word',
        'src/primitives/check_artifacts.rs',
        '"tbd" | "todo" | "tbc" |',
        '"tbd" | "todo" |',
        'check_artifacts::tests::an_empty_or_placeholder_criterion_is_blocking',
    ),
    (
        'M311 fixme is no placeholder word',
        'src/primitives/check_artifacts.rs',
        '| "fixme" | "placeholder"',
        '| "placeholder"',
        'check_artifacts::tests::an_empty_or_placeholder_criterion_is_blocking',
    ),
    (
        'M312 placeholder is no placeholder word',
        'src/primitives/check_artifacts.rs',
        '| "placeholder" | "n/a"',
        '| "n/a"',
        'check_artifacts::tests::an_empty_or_placeholder_criterion_is_blocking',
    ),
    (
        'M313 an ellipsis of dots is no placeholder',
        'src/primitives/check_artifacts.rs',
        '"n/a" | "" | "…"',
        '"n/a" | "…"',
        'check_artifacts::tests::an_empty_or_placeholder_criterion_is_blocking',
    ),
    (
        'M314 the ellipsis character is no placeholder',
        'src/primitives/check_artifacts.rs',
        '"n/a" | "" | "…"',
        '"n/a" | ""',
        'check_artifacts::tests::an_empty_or_placeholder_criterion_is_blocking',
    ),
    (
        'M315 resolve-anchor reads a registry that does not parse as an empty one',
        'src/primitives/resolve_anchor.rs',
        '        let registry = super::resolve_constitutions::run(repo)?;',
        '        let Ok(registry) = super::resolve_constitutions::run(repo) else { return Ok(Self { repo, repo_abs, citing_dir, markers_rel, shipped: Shipped::load(repo), registered: Vec::new(), resolved: HashMap::new() }); };',
        'resolve_anchor::tests::a_registry_that_does_not_parse_is_an_error',
    ),
    (
        "M316 a document path's own characters name its list file's directory",
        'src/primitives/bounded_lists.rs',
        "            !stem.is_empty()\n                && stem\n                    .chars()\n                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')",
        '            !stem.is_empty()',
        'bounded_lists::tests::a_document_path_is_keyed_by_its_stem_or_document',
    ),
    (
        'M317 the exec tally reads a result before its lists are read back',
        'src/interpreter/mod.rs',
        [
            '        let restored = match payload::restore_lists(name, &mut result, &self.repo) {',
            '            tally.record_primitive(name, &result, &self.context);',
        ],
        [
            '        let tally_view = result.clone();\n        let restored = match payload::restore_lists(name, &mut result, &self.repo) {',
            '            tally.record_primitive(name, &tally_view, &self.context);',
        ],
        'interpreter::tests::an_exec_analyze_counts_an_unresolved_anchor_held_in_the_file_form',
    ),
]
