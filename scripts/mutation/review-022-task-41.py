"""Mutation list for spec 022 task 41 (from 022's review of 2026-11-17): the
`writeSpecBody` and `routeInboxItem` requests are held to the read size whole
(`BE-PAYLOAD-001`), `writeCode` names why its task was not read
(`QUAL-CLAIM-001`), the rule load propagates a config or feature it cannot
resolve (`CFG-ENV-008`), and every project-file read in
`interpreter/payload.rs` is contained (`BE-INPUT-004`). Each mutation removes
one of those, and the tests in `interpreter::payload::tests` and
`interpreter::tests` kill it.

Run with `python3 scripts/mutation/run.py scripts/mutation/review-022-task-41.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 writeSpecBody past the read size still carries its bodies',
        'src/interpreter/payload.rs',
        '    if !fits(repo, &value)? && (template_read || existing_path.is_some()) {',
        '    if false && (template_read || existing_path.is_some()) {',
        'interpreter::payload::tests::write_spec_body_',
    ),
    (
        'M2 writeSpecBody past the read size still carries the template body',
        'src/interpreter/payload.rs',
        '        if template_read {\n            typed.template_content = None;\n        }\n',
        '',
        'interpreter::payload::tests::write_spec_body_',
    ),
    (
        'M3 writeSpecBody past the read size still carries the existing body',
        'src/interpreter/payload.rs',
        '            typed.existing_content = None;\n            typed.existing_path = existing_path;',
        '            typed.existing_path = existing_path;',
        'interpreter::payload::tests::write_spec_body_',
    ),
    (
        'M4 writeSpecBody past the read size names no existing file',
        'src/interpreter/payload.rs',
        '            typed.existing_content = None;\n            typed.existing_path = existing_path;',
        '            typed.existing_content = None;',
        'interpreter::payload::tests::write_spec_body_',
    ),
    (
        'M5 writeSpecBody bounds a body it did not read',
        'src/interpreter/payload.rs',
        '    if !fits(repo, &value)? && (template_read || existing_path.is_some()) {',
        '    if !fits(repo, &value)? {',
        'interpreter::payload::tests::write_spec_body_',
    ),
    (
        'M6 writeSpecBody moves no legacy tail',
        'src/interpreter/payload.rs',
        '    bound_lists(repo, "writeSpecBody", value, &[], Some(typed_keys(&typed)))',
        '    Ok(value)',
        'interpreter::payload::tests::write_spec_body_',
    ),
    (
        'M7 writeSpecBody swallows a malformed read size',
        'src/interpreter/payload.rs',
        [
            '    if !fits(repo, &value)? && (template_read || existing_path.is_some()) {',
            '    bound_lists(repo, "writeSpecBody", value, &[], Some(typed_keys(&typed)))',
        ],
        [
            '    if !fits(repo, &value).unwrap_or(true) && (template_read || existing_path.is_some()) {',
            '    Ok(value)',
        ],
        'interpreter::payload::tests::a_malformed_read_size_halts_a_write_spec_body_request',
    ),
    (
        'M8 writeSpecBody names the existing file in the inline form',
        'src/interpreter/payload.rs',
        '        // The inline form names no file for the section: the host has it.\n        existing_path: None,',
        '        existing_path: existing_path.clone(),',
        'interpreter::payload::tests::write_spec_body_',
    ),
    (
        'M9 routeInboxItem moves no available specs',
        'src/interpreter/payload.rs',
        '        &["available-specs", "candidates"],',
        '        &["candidates"],',
        'interpreter::payload::tests::route_inbox_item_',
    ),
    (
        'M10 routeInboxItem moves no candidates',
        'src/interpreter/payload.rs',
        '        &["available-specs", "candidates"],',
        '        &["available-specs"],',
        'interpreter::payload::tests::route_inbox_item_',
    ),
    (
        'M11 routeInboxItem is never held to the read size',
        'src/interpreter/payload.rs',
        '    bound_lists(\n        repo,\n        "routeInboxItem",\n        typed_only(&typed),\n        &["available-specs", "candidates"],\n        None,\n    )',
        '    Ok(typed_only(&typed))',
        'interpreter::payload::tests::route_inbox_item_',
    ),
    (
        'M12 writeCode sends no reason for a task it did not read',
        'src/interpreter/payload.rs',
        '        task,\n        task_unexaminable,\n        content_form: None,',
        '        task,\n        task_unexaminable: None,\n        content_form: None,',
        'interpreter::payload::tests::write_code_names_why_its_task_was_not_read',
    ),
    (
        'M13 writeCode names no reason for an unreadable tasks.md',
        'src/interpreter/payload.rs',
        '        return unexamined(format!("tasks-unreadable: {feature}"));',
        '        return (unexamined(String::new()).0, None);',
        'interpreter::payload::tests::write_code_names_why_its_task_was_not_read',
    ),
    (
        'M14 writeCode names no reason for a task number tasks.md lacks',
        'src/interpreter/payload.rs',
        '        None => unexamined(format!("task-not-found: {task_number} in {feature}")),',
        '        None => (unexamined(String::new()).0, None),',
        'interpreter::payload::tests::write_code_names_why_its_task_was_not_read',
    ),
    (
        'M15 writeCode names no reason for a walk with no feature',
        'src/interpreter/payload.rs',
        '        return unexamined("no-feature: the walk names no feature".into());',
        '        return (unexamined(String::new()).0, None);',
        'interpreter::payload::tests::write_code_names_why_its_task_was_not_read',
    ),
    (
        'M16 writeCode names no reason for a tasks.md with no task',
        'src/interpreter/payload.rs',
        '        None if task_number.is_empty() => {\n            unexamined(format!("task-not-found: {feature} holds no task"))\n        }',
        '        None if task_number.is_empty() => (unexamined(String::new()).0, None),',
        'interpreter::payload::tests::write_code_names_why_its_task_was_not_read',
    ),
    (
        'M17 the constitution is read through a link leaving the project',
        'src/interpreter/payload.rs',
        '        crate::primitives::validate_contained(repo, constitution_rel),',
        '        Ok(()),',
        'interpreter::payload::tests::a_constitution_linked_',
    ),
    (
        'M18 the plan is read from a feature directory leaving the project',
        'src/interpreter/payload.rs',
        '        crate::primitives::validate_feature_argument(repo, &specs_root, feature),',
        '        Ok(()),',
        'interpreter::payload::tests::a_plan_outside_the_project_is_not_read',
    ),
    (
        'M19 the rule load reads a malformed config as no feature',
        'src/interpreter/payload.rs',
        '    let root = crate::schema::paths::Paths::load(repo)?.specs_root;\n    refuse_outside(\n        crate::primitives::validate_feature_argument(repo, &root, feature),',
        '    let Ok(paths) = crate::schema::paths::Paths::load(repo) else {\n        return Ok(None);\n    };\n    let root = paths.specs_root;\n    refuse_outside(\n        crate::primitives::validate_feature_argument(repo, &root, feature),',
        'interpreter::payload::tests::load_rules_ interpreter::tests::an_assessment_step_halts_when_the_project_config_stops_parsing',
    ),
    (
        'M20 the rule load reads an outward feature as no feature',
        'src/interpreter/payload.rs',
        '    refuse_outside(\n        crate::primitives::validate_feature_argument(repo, &root, feature),\n        &format!("{root}/{feature}"),\n    )?;',
        '    if crate::primitives::validate_feature_argument(repo, &root, feature).is_err() {\n        return Ok(None);\n    }',
        'interpreter::payload::tests::load_rules_',
    ),
    (
        'M21 the walker carries on when the rule load fails',
        'src/interpreter/mod.rs',
        '            Err(err) => {\n                let code = err.code().to_string();\n                let message = err.to_string();\n                self.emit_error(code.clone(), message.clone(), None)?;\n                return Ok(Some(WalkOutcome::Errored { code, message }));\n            }\n        };\n        if let Some(tally)',
        '            Err(_) => payload::LoadedRules::default(),\n        };\n        if let Some(tally)',
        'interpreter::tests::an_assessment_step_halts_when_the_project_config_stops_parsing',
    ),
    (
        'M22 available-specs lists a spec root leaving the project',
        'src/interpreter/payload.rs',
        '    refuse_outside(\n        crate::primitives::validate_contained(repo, &specs_root),\n        &specs_root,\n    )?;\n    Ok(\n',
        '    Ok(\n',
        'interpreter::payload::tests::available_specs_read_nothing_outside_the_project',
    ),
    (
        'M23 available-specs reads a spec file leaving the project',
        'src/interpreter/payload.rs',
        '    let Some(content) = read_repo_file(repo, rel) else {\n        return String::new();\n    };\n    crate::primitives::frontmatter_status',
        '    let Ok(content) = std::fs::read_to_string(repo.join(rel)) else {\n        return String::new();\n    };\n    crate::primitives::frontmatter_status',
        'interpreter::payload::tests::available_specs_read_nothing_outside_the_project',
    ),
    (
        'M24 the fold lists scenarios from a directory leaving the project',
        'src/interpreter/payload.rs',
        '    refuse_outside(\n        crate::primitives::validate_contained(repo, &scenarios_rel),\n        &scenarios_rel,\n    )?;\n',
        '',
        'interpreter::payload::tests::fold_scenarios_read_nothing_outside_the_project',
    ),
    (
        'M25 the fold reads a scenario file leaving the project',
        'src/interpreter/payload.rs',
        '            let contained =\n                !crate::primitives::leaves_project(repo, &format!("{scenarios_rel}/{name}"));',
        '            let contained = true;',
        'interpreter::payload::tests::fold_scenarios_read_nothing_outside_the_project',
    ),
]
