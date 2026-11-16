"""Mutation list for spec 067's review fix round, part A: every path a 067
primitive draws from committed text is contained before it is probed, listed
or opened (`BE-INPUT-004`), and five results are bounded (`BE-PAYLOAD-001`).
Each containment mutation drops one guard and is killed by the test that
builds the outward link beyond it; each bound mutation keeps a list inline,
leaves one out of the list file, or never moves to the file, and is killed by
that primitive's `the_lists_move_to_a_file_past_the_read_size` test. M32 to
M36 hold the review's test findings: a key named and not only counted, an
unreadable document that reads as unexamined, and the JSON of
`check-rule-ids`' retired state.

Run with `python3 scripts/mutation/run.py scripts/mutation/containment-and-bounds-067.py
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
        'M1 derive-dependencies reads and rewrites a tracked spec that is a link out of the project',
        'src/primitives/derive_dependencies.rs',
        '        if super::leaves_project(repo, spec) {',
        '        if false && super::leaves_project(repo, spec) {',
        'derive_dependencies::tests::a_tracked_spec_that_leaves_the_project_is_named_and_never_rewritten',
    ),
    (
        'M2 derive-references reads and rewrites a tracked spec that is a link out of the project',
        'src/primitives/derive_references.rs',
        '        if super::leaves_project(repo, spec) {',
        '        if false && super::leaves_project(repo, spec) {',
        'derive_references::tests::a_tracked_spec_that_leaves_the_project_is_named_and_never_rewritten',
    ),
    (
        'M3 a spec with sibling links and no dependencies: key reads in sync',
        'src/primitives/derive_dependencies.rs',
        '            if !deps.is_empty() && !has_dependencies_key(content) {',
        '            if false && !deps.is_empty() && !has_dependencies_key(content) {',
        'derive_dependencies::tests::a_spec_with_sibling_links_and_no_key_is_unwritable',
    ),
    (
        'M4 a spec that links nothing is called unwritable for lacking the key',
        'src/primitives/derive_dependencies.rs',
        '            if !deps.is_empty() && !has_dependencies_key(content) {',
        '            if !has_dependencies_key(content) {',
        'derive_dependencies::tests::a_spec_with_sibling_links_and_no_key_is_unwritable',
    ),
    (
        'M5 check-command-flags reads a command source that is a link out of the project',
        'src/primitives/check_command_flags.rs',
        '        if super::leaves_project(repo, &rel) {',
        '        if false && super::leaves_project(repo, &rel) {',
        'check_command_flags::tests::a_command_source_that_leaves_the_project_is_skipped_and_not_read',
    ),
    (
        'M6 check-command-flags lists a command directory that is a link out of the project',
        'src/primitives/check_command_flags.rs',
        '    let listing = if super::leaves_project(repo, COMMANDS_DIR) {',
        '    let listing = if false && super::leaves_project(repo, COMMANDS_DIR) {',
        'check_command_flags::tests::a_command_directory_that_leaves_the_project_is_skipped_and_not_listed',
    ),
    (
        'M7 check-command-flags keeps its lists inline past the read size',
        'src/primitives/check_command_flags.rs',
        '        super::bounded_lists::place(repo, "check-command-flags", "repository", &result, &lists)?',
        '        None::<String>',
        'check_command_flags::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M8 check-command-flags leaves skipped inline in the file form',
        'src/primitives/check_command_flags.rs',
        '        result.skipped = None;\n',
        '',
        'check_command_flags::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M9 check-command-flags leaves a list out of the list file',
        'src/primitives/check_command_flags.rs',
        '        "with-argument-hint": checked.with_argument_hint,\n',
        '',
        'check_command_flags::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M10 check-orphaned-references opens a referrer that is a link out of the project',
        'src/primitives/check_orphaned_references.rs',
        '        if super::leaves_project(repo, &referrer) {',
        '        if false && super::leaves_project(repo, &referrer) {',
        'check_orphaned_references::tests::a_referrer_that_leaves_the_project_is_skipped_and_not_read',
    ),
    (
        'M11 check-orphaned-references probes a managed path through a link out of the project',
        'src/primitives/check_orphaned_references.rs',
        '                if super::leaves_project(repo, probed) {',
        '                if false && super::leaves_project(repo, probed) {',
        'check_orphaned_references::tests::a_managed_path_through_a_link_out_is_skipped_and_not_probed',
    ),
    (
        'M12 check-orphaned-references keeps its lists inline past the read size',
        'src/primitives/check_orphaned_references.rs',
        '    if let Some(file) = super::bounded_lists::place(\n        repo,\n        "check-orphaned-references",\n        "repository",\n        &result,\n        &lists,\n    )? {',
        '    if let Some(file) = None::<String> {',
        'check_orphaned_references::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M13 check-orphaned-references leaves examined inline in the file form',
        'src/primitives/check_orphaned_references.rs',
        '        result.examined = None;\n',
        '',
        'check_orphaned_references::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M14 validate-frontmatter lists a scenarios/ that is a link out of the project',
        'src/primitives/validate_frontmatter.rs',
        '    if super::leaves_project(repo, &format!("{rel}scenarios")) {',
        '    if false && super::leaves_project(repo, &format!("{rel}scenarios")) {',
        'validate_frontmatter::tests::a_scenarios_directory_that_leaves_the_project_is_not_listed',
    ),
    (
        'M15 validate-frontmatter opens a scenario that is a link out of the project',
        'src/primitives/validate_frontmatter.rs',
        '        if super::leaves_project(repo, &format!("{rel}{field}")) {',
        '        if false && super::leaves_project(repo, &format!("{rel}{field}")) {',
        'validate_frontmatter::tests::a_scenario_that_leaves_the_project_is_not_read',
    ),
    (
        'M16 validate-frontmatter opens a review.md that is a link out of the project',
        'src/primitives/validate_frontmatter.rs',
        '        if super::leaves_project(repo, &format!("{rel}{file}")) {',
        '        if false && super::leaves_project(repo, &format!("{rel}{file}")) {',
        'validate_frontmatter::tests::a_record_that_leaves_the_project_is_not_read',
    ),
    (
        'M17 validate-frontmatter reads clean over a scenarios/ it cannot list',
        'src/primitives/validate_frontmatter.rs',
        '        && source.kind() != std::io::ErrorKind::NotFound',
        '        && false',
        'validate_frontmatter::tests::an_unlistable_scenarios_directory_is_a_hard_fail',
    ),
    (
        'M18 validate-frontmatter keeps its findings inline past the read size',
        'src/primitives/validate_frontmatter.rs',
        '    if let Some(file) = super::bounded_lists::place(\n        repo,\n        "validate-frontmatter",\n        super::bounded_lists::path_key(path),\n        &result,\n        &lists,\n    )? {',
        '    if let Some(file) = None::<String> {',
        'validate_frontmatter::tests::the_findings_move_to_a_file_past_the_read_size',
    ),
    (
        'M19 validate-frontmatter keeps findings inline in the file form',
        'src/primitives/validate_frontmatter.rs',
        '        result.findings = None;\n',
        '',
        'validate_frontmatter::tests::the_findings_move_to_a_file_past_the_read_size',
    ),
    (
        'M20 traverse-deps probes a dependency name that climbs out of the spec root or is a link out',
        'src/primitives/traverse_deps.rs',
        '        let exists = !refused(repo, &root, dep_name) && dep_dir',
        '        let exists = true && dep_dir',
        'traverse_deps::tests::a_dependency_name_that_leaves_the_project_is_not_read',
    ),
    (
        'M21 traverse-deps walks a dependency that is a link out of the project',
        'src/primitives/traverse_deps.rs',
        '    let deps = if refused(repo, root, node) {',
        '    let deps = if false && refused(repo, root, node) {',
        'traverse_deps::tests::a_dependency_that_is_a_link_out_is_neither_read_nor_walked',
    ),
    (
        'M37 traverse-deps probes and walks an absolute dependency name',
        'src/primitives/traverse_deps.rs',
        '    super::validate_no_traversal(name).is_err()\n        ||',
        '    false\n        ||',
        'traverse_deps::tests::an_absolute_dependency_name_is_neither_read_nor_walked',
    ),
    (
        'M22 check-corpus-links lists a spec root that is a link out of the project',
        'src/primitives/check_corpus_links.rs',
        '        LinkScope::SpecCorpus if containment.leaves(&specs_dir) => {',
        '        LinkScope::SpecCorpus if false && containment.leaves(&specs_dir) => {',
        'check_corpus_links::tests::a_spec_root_that_is_a_link_out_of_the_project_is_skipped_and_not_listed',
    ),
    (
        'M23 check-corpus-links probes the type of a link out of the base before judging it',
        'src/primitives/check_corpus_links.rs',
        '        let linked = entry.file_type().is_ok_and(|kind| kind.is_symlink());',
        '        let linked = false;',
        'check_corpus_links::tests::a_directory_that_is_a_link_out_of_the_project_is_skipped_and_not_listed',
    ),
    (
        'M24 check-unfolded-specs lists a spec root that is a link out of the project',
        'src/primitives/check_unfolded_specs.rs',
        '    super::validate_contained(repo, &layout.specs_root)?;\n',
        '',
        'check_unfolded_specs::tests::a_spec_root_that_leaves_the_project_halts_unlisted',
    ),
    (
        'M25 check-unfolded-specs opens the spec.md of a directory that is a link out of the project',
        'src/primitives/check_unfolded_specs.rs',
        '        super::validate_contained(repo, &format!("{}/{slug}/spec.md", layout.specs_root))?;\n',
        '',
        'check_unfolded_specs::tests::a_branch_scoped_directory_that_leaves_the_project_halts_unread',
    ),
    (
        'M26 check-departures reads a rule file that is under a link out of the project',
        'src/primitives/check_departures.rs',
        'read_text(&super::contained_path(repo, file)?)?',
        'read_text(&repo.join(file))?',
        'check_departures::tests::a_rule_file_that_leaves_the_project_is_refused_unread',
    ),
    (
        'M27 check-rule-ids keeps its lists inline past the read size',
        'src/primitives/check_rule_ids.rs',
        '    if let Some(file) = super::bounded_lists::place(\n        repo,\n        "check-rule-ids",\n        super::bounded_lists::path_key(path),\n        &result,\n        &lists,\n    )? {',
        '    if let Some(file) = None::<String> {',
        'check_rule_ids::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M28 check-rule-ids leaves retired inline in the file form',
        'src/primitives/check_rule_ids.rs',
        '        result.retired = None;\n',
        '',
        'check_rule_ids::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M29 check-rule-ids leaves not-installed out of the list file',
        'src/primitives/check_rule_ids.rs',
        '        "not-installed": checked.not_installed,\n',
        '',
        'check_rule_ids::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M30 check-promotion-coverage keeps its lists inline past the read size',
        'src/primitives/check_promotion_coverage.rs',
        '    if let Some(file) = super::bounded_lists::place(\n        repo,\n        "check-promotion-coverage",\n        "repository",\n        &result,\n        &lists,\n    )? {',
        '    if let Some(file) = None::<String> {',
        'check_promotion_coverage::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M31 check-promotion-coverage leaves missing-sections inline in the file form',
        'src/primitives/check_promotion_coverage.rs',
        '        result.missing_sections = None;\n',
        '',
        'check_promotion_coverage::tests::the_lists_move_to_a_file_past_the_read_size',
    ),
    (
        'M32 check-promotion-coverage names an unmatched key by something other than the key',
        'src/primitives/check_promotion_coverage.rs',
        '                None => unmatched_keys.push(key),',
        '                None => unmatched_keys.push(String::from("key")),',
        'check_promotion_coverage::tests::a_key_matching_no_entry_is_reported_loudly',
    ),
    (
        'M33 resolve-anchor reads a document it cannot read as absent',
        'src/primitives/resolve_anchor.rs',
        '            return read_text(&path).map_or(Named::Unreadable, |text| {',
        '            return read_text(&path).map_or(Named::Absent, |text| {',
        'resolve_anchor::tests::a_reference_naming_an_unreadable_document_is_unexamined_as_unreadable',
    ),
    (
        'M34 check-rule-ids drops the retired list from its JSON',
        'src/schema/primitives.rs',
        '    #[serde(default, skip_serializing_if = "Option::is_none")]\n    pub retired: Option<Vec<String>>,',
        '    #[serde(skip)]\n    pub retired: Option<Vec<String>>,',
        'schema::primitives::tests::check_rule_ids_round_trip',
    ),
    (
        "M35 check-rule-ids serializes a citation's retired flag when it is false",
        'src/schema/primitives.rs',
        '    #[serde(default, skip_serializing_if = "std::ops::Not::not")]\n    pub retired: bool,',
        '    #[serde(default)]\n    pub retired: bool,',
        'schema::primitives::tests::check_rule_ids_round_trip',
    ),
    (
        'M36 check-rule-ids serializes retired-count at zero',
        'src/schema/primitives.rs',
        '    #[serde(default, skip_serializing_if = "is_zero")]\n    pub retired_count: usize,',
        '    #[serde(default)]\n    pub retired_count: usize,',
        'schema::primitives::tests::check_rule_ids_round_trip',
    ),
]
