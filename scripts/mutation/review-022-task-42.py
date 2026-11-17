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
Windows every mutation but M13, M28 and M31-M37 would survive: their tests
plant no link.
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
        'M16 the shipped-destination manifest is read through a link outside the project',
        'src/primitives/check_artifacts.rs',
        '    if super::leaves_project(repo, manifest) {\n        return BTreeSet::new();\n    }\n',
        '',
        'check_artifacts::tests::a_manifest_linked_outside_the_project_derives_no_destinations',
    ),
    (
        'M17 the migrations registry is read through a link outside the project',
        'src/primitives/check_orphaned_references.rs',
        '    if super::leaves_project(repo, "framework/migrations.toml") {\n        return None;\n    }\n',
        '',
        'check_orphaned_references::tests::a_registry_linked_outside_the_project_is_not_read',
    ),
    (
        'M18 the inbox creation template is read through a link outside the project',
        'src/primitives/append_inbox.rs',
        '    if super::leaves_project(repo, PROJECT_TEMPLATE) {\n        return FALLBACK_HEADING.to_string();\n    }\n',
        '',
        'append_inbox::tests::a_template_linked_outside_the_project_is_not_read',
    ),
    (
        'M19 a path through a link outside the project is probed and held in review scope',
        'src/primitives/compute_review_scope.rs',
        '        !super::leaves_project(repo, path) && repo.join(path).symlink_metadata().is_ok()\n',
        '        repo.join(path).symlink_metadata().is_ok()\n',
        'compute_review_scope::tests::a_path_through_a_link_outside_the_project_is_not_held_in_scope',
    ),
    (
        'M20 another claimants plan is read through a link outside the project',
        'src/primitives/compute_review_scope.rs',
        '    if super::leaves_project(repo, &format!("{specs_root}/{directory}/plan.md")) {\n        return Ok(PlanEntries::new(&[]));\n    }\n',
        '',
        'compute_review_scope::tests::another_claimants_plan_linked_outside_the_project_is_not_read',
    ),
    (
        'M21 a routing candidate reads the status of a spec linked outside the project',
        'src/primitives/derive_routing_candidates.rs',
        '    if crate::primitives::leaves_project(repo, &spec_rel) {\n        return String::new();\n    }\n',
        '',
        'derive_routing_candidates::tests::a_spec_linked_outside_the_project_is_not_read',
    ),
    (
        'M22 the runtime home reads a plan linked outside the project',
        'src/primitives/derive_routing_candidates.rs',
        '        !crate::primitives::leaves_project(repo, &format!("{specs_root}/{slug}/plan.md"))\n            && crate::primitives::compute_review_scope::read_plan_affected(&specs_dir.join(slug))',
        '        crate::primitives::compute_review_scope::read_plan_affected(&specs_dir.join(slug))',
        'derive_routing_candidates::tests::a_spec_linked_outside_the_project_is_not_read',
    ),
    (
        'M23 the framework reference sources are stat-ed through a link outside the base',
        'src/primitives/check_corpus_links.rs',
        '    let generates_references =\n        !containment.leaves(&reference_sources) && reference_sources.is_dir();',
        '    let generates_references = reference_sources.is_dir();',
        'check_corpus_links::tests::a_reference_sources_directory_linked_outside_is_not_probed',
    ),
    (
        'M24 a project file linked outside the project is measured',
        'src/primitives/check_artifact_size.rs',
        '            if super::leaves_project(repo, &rel) {\n                records.push(record(ProjectSizeState::Unreadable, None));\n                continue;\n            }\n',
        '',
        'check_artifact_size::tests::a_project_file_linked_outside_the_project_is_unreadable_not_measured',
    ),
    (
        'M25 check-step-references lists a command directory linked outside the project',
        'src/primitives/check_step_references.rs',
        '    let listing = if super::leaves_project(repo, COMMANDS_DIR) {\n        Err(std::io::Error::other("it leaves the project"))\n    } else {\n        std::fs::read_dir(repo.join(COMMANDS_DIR))\n    };\n',
        '    let listing = std::fs::read_dir(repo.join(COMMANDS_DIR));\n',
        'check_step_references::tests::a_source_linked_outside_the_project_is_skipped_not_read',
    ),
    (
        'M26 check-step-references reads a command file linked outside the project',
        'src/primitives/check_step_references.rs',
        '        if super::leaves_project(repo, &rel) {\n            skipped.push(StepReferenceSkip {\n                path: rel,\n                reason: "could not be read: it leaves the project".to_string(),\n            });\n            continue;\n        }\n',
        '',
        'check_step_references::tests::a_source_linked_outside_the_project_is_skipped_not_read',
    ),
    (
        'M28 rewrite-spec-links takes a from that names no feature directory',
        'src/primitives/rewrite_spec_links.rs',
        '    super::validate_session_feature("rewrite-spec-links", "from", &args.from)?;\n',
        '',
        'rewrite_spec_links::tests::a_from_that_names_no_feature_directory_is_refused_with_the_corpus_untouched',
    ),
    (
        'M29 rewrite-spec-links probes a scenario targets spec.md through a link outside the project',
        'src/primitives/rewrite_spec_links.rs',
        '    super::validate_feature_argument(repo, &layout.specs_root, &target.feature)?;\n',
        '',
        'rewrite_spec_links::tests::a_scenario_targets_spec_linked_outside_the_project_is_refused',
    ),
    (
        'M30 rewrite-spec-links stats a link leaving the project below the spec root',
        'src/primitives/rewrite_spec_links.rs',
        '                    || super::leaves_project(self.repo, &repo_relative(self.repo, path))\n',
        '',
        'rewrite_spec_links::tests::a_link_out_of_the_project_below_the_spec_root_is_listed_without_a_stat',
    ),
    (
        'M31 discover-rule-files keeps selected inline past the read size',
        'src/primitives/discover_rule_files.rs',
        '        result.form = ListsForm::File;\n        result.selected = None;\n',
        '        result.form = ListsForm::File;\n',
        'discover_rule_files::tests::past_the_read_size_selected_moves_to_the_file',
    ),
    (
        'M32 check-departures reads the selection from a result bounded to a list file',
        'src/primitives/check_departures.rs',
        '        let (discovered, _) = discover_rule_files::compute(',
        '        let discovered = discover_rule_files::run(',
        'check_departures::tests::the_tiers_are_read_whatever_the_read_size',
    ),
    (
        'M33 dashboard keeps config disabled-rule-files inline past the read size',
        'src/primitives/dashboard.rs',
        '        result.config.disabled_rule_files = None;\n',
        '',
        'dashboard::tests::the_file_form_moves_the_disabled_rule_files',
    ),
    (
        'M34 label-criteria keeps assigned inline past the read size',
        'src/primitives/label_criteria.rs',
        '    bound(repo, &args.feature, &mut labelled)?;\n    guarded::write_guarded(&snapshot, &new_content)?;\n',
        '    guarded::write_guarded(&snapshot, &new_content)?;\n',
        'label_criteria::tests::past_the_read_size_assigned_moves_to_the_file',
    ),
    (
        'M35 label-criteria writes the spec before its result can be bounded',
        'src/primitives/label_criteria.rs',
        '    bound(repo, &args.feature, &mut labelled)?;\n    guarded::write_guarded(&snapshot, &new_content)?;\n',
        '    guarded::write_guarded(&snapshot, &new_content)?;\n    bound(repo, &args.feature, &mut labelled)?;\n',
        'label_criteria::tests::a_malformed_read_size_refuses_the_call_with_the_spec_untouched',
    ),
    (
        'M36 the next label is the body maximum, not one past it, with no stored counter',
        'src/primitives/label_criteria.rs',
        '        (Some(max_label), None) => max_label + 1,\n',
        '        (Some(max_label), None) => max_label,\n',
        'label_criteria::tests::the_next_label_is_one_past_the_body_maximum_when_the_counter_is_absent_or_behind',
    ),
    (
        'M37 the next label is the body maximum, not one past it, under a counter behind it',
        'src/primitives/label_criteria.rs',
        '        (Some(max_label), Some(next)) => next.max(max_label + 1),\n',
        '        (Some(max_label), Some(next)) => next.max(max_label),\n',
        'label_criteria::tests::the_next_label_is_one_past_the_body_maximum_when_the_counter_is_absent_or_behind',
    ),
    (
        'M38 a rule directory linked outside the project fails routing instead of being skipped',
        'src/primitives/derive_routing_candidates.rs',
        '        Err(crate::primitives::PrimitiveError::InvalidPath { reason, .. }) => {',
        '        Err(crate::primitives::PrimitiveError::InvalidPath { reason, .. }) if false => {',
        'derive_routing_candidates::tests::a_rule_directory_linked_outside_the_project_is_skipped_by_name',
    ),
]
