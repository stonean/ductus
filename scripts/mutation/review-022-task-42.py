"""Mutation list for spec 022 task 42 (the review of 2026-11-17): every project
file the runtime reads under a fixed name, or through a path a config key or
committed text names, is contained before it is probed or read
(`BE-INPUT-004`); `rewrite-spec-links` holds `from` to the feature-directory
grammar (`BE-INPUT-002`); and `selected`, `config`'s `disabled-rule-files` and
`label-criteria`'s `assigned` are held to the read size (`BE-PAYLOAD-001`),
with the next label's rule pinned (`QUAL-TEST-001`).

Run with `python3 scripts/mutation/run.py scripts/mutation/review-022-task-42.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The containment tests are Unix only (they plant a symbolic link), so on
Windows M1-M30 would survive.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 Host::load reads a config file linked outside the project',
        'src/host.rs',
        '        crate::primitives::contained_file(repo, &toml_path)?;\n',
        '',
        'host::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M2 Paths::load reads a config file linked outside the project',
        'src/schema/paths.rs',
        '        contained_file(base, &toml_path)?;\n',
        '        let _ = contained_file;\n',
        'schema::paths::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M3 discover-rule-files reads a config file linked outside the project',
        'src/primitives/discover_rule_files.rs',
        '    super::contained_file(repo, &path)?;\n',
        '',
        'discover_rule_files::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M4 dashboard reads a config file linked outside the project',
        'src/primitives/dashboard.rs',
        '    super::contained_file(repo, &toml_path)?;\n',
        '',
        'dashboard::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M5 the read-size threshold is read from a config file linked outside the project',
        'src/primitives/check_artifact_size.rs',
        '    super::contained_file(repo, &path)?;\n',
        '',
        'check_artifact_size::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M6 the last-applied migration is read from a config file linked outside the project',
        'src/primitives/check_orphaned_references.rs',
        '    super::contained_file(repo, &path)?;\n',
        '',
        'check_orphaned_references::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M7 the services registry is read from a config file linked outside the project',
        'src/primitives/resolve_references.rs',
        '    super::contained_file(repo, &toml_path)?;\n',
        '',
        'resolve_references::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M8 the re-reads switch is read from a config file linked outside the project',
        'src/primitives/fix_rounds.rs',
        '    super::contained_file(repo, &path)?;\n',
        '',
        'fix_rounds::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M9 the constitutions registry is read from a config file linked outside the project',
        'src/primitives/resolve_constitutions.rs',
        '    super::contained_file(repo, &toml_path)?;\n',
        '',
        'resolve_constitutions::tests::a_config_file_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M10 dashboard reads a spec through a feature directory linked outside the project',
        'src/primitives/dashboard.rs',
        '        super::validate_feature_argument(repo, &layout.specs_root, &slug)?;\n',
        '',
        'dashboard::tests::a_spec_directory_holding_an_outward_link_refuses_the_dashboard',
    ),
    (
        'M11 dashboard reads a scenario file the session names through a link outside the project',
        'src/primitives/dashboard.rs',
        '    if super::leaves_project(repo, rel_path) {\n        return Ok(None);\n    }\n',
        '',
        'dashboard::tests::a_scenario_path_through_a_link_outside_the_project_is_not_read',
    ),
    (
        'M12 the inbox row counts an inbox.md under a spec root linked outside the project',
        'src/primitives/inbox_standing.rs',
        '    super::validate_contained(repo, &rel)?;\n',
        '',
        'inbox_standing::tests::an_inbox_under_a_spec_root_linked_outside_the_project_is_refused',
    ),
    (
        'M13 resolve-references reads a spec value that leaves the checkout',
        'src/primitives/resolve_references.rs',
        '    if super::validate_contained(&specs_dir, &format!("{}/spec.md", entry.spec)).is_err() {\n        return Ok((ReferenceOutcome::StatusUnreadable, None));\n    }\n',
        '',
        'resolve_references::tests::a_spec_value_that_leaves_the_checkouts_spec_root_is_unread',
    ),
    (
        'M14 discover-rule-files lists a specs/rules directory linked outside the project',
        'src/primitives/discover_rule_files.rs',
        '    super::validate_contained(repo, &format!("{}/rules", layout.specs_root))?;\n',
        '',
        'discover_rule_files::tests::a_rules_directory_linked_outside_the_project_is_refused_not_listed',
    ),
    (
        'M15 discover-rule-files lists a framework/rules directory linked outside the project',
        'src/primitives/discover_rule_files.rs',
        '    super::validate_contained(repo, "framework/rules")?;\n',
        '',
        'discover_rule_files::tests::a_rules_directory_linked_outside_the_project_is_refused_not_listed',
    ),
    (
        'M17 the migrations registry is read through a link outside the project',
        'src/primitives/check_orphaned_references.rs',
        '    if super::leaves_project(repo, "framework/migrations.toml") {\n        return None;\n    }\n',
        '',
        'check_orphaned_references::tests::a_registry_linked_outside_the_project_is_not_read',
    ),
    (
        'M24 a project file linked outside the project is measured',
        'src/primitives/check_artifact_size.rs',
        '            if super::leaves_project(repo, &rel) {\n                records.push(record(ProjectSizeState::Unreadable, None));\n                continue;\n            }\n',
        '',
        'check_artifact_size::tests::a_project_file_linked_outside_the_project_is_unreadable_not_measured',
    ),
]
