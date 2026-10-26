"""Mutation list for the owner-approved review fixes to spec 067's runtime
primitives: `validate-frontmatter`'s severities, the two frontmatter splices
(`derive-dependencies`, `derive-references`) and the parse check before their
write, `check-command-flags`' whole pass, `check-rule-ids`' line-scoped
section scan, `check-corpus-links`' reference sources, `resolve-anchor`'s
markers basename, and `check-promotion-coverage`'s required section set.

Each mutation removes the behavior one new or strengthened unit test names,
and its filter selects that test (and, where two tests share the behavior,
both). Behaviors only an integration test under `runtime/tests/` covers
cannot run here, since the harness runs `cargo test --lib`.

Run with `python3 scripts/mutation/run.py scripts/mutation/owner-reviews-067.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    # -- validate-frontmatter: each branch's severity ---------------------
    (
        'M1 frontmatter that is not YAML is blocking, not hard-fail',
        'src/primitives/validate_frontmatter.rs',
        '                severity: AnalyzeSeverity::HardFail,\n                field: String::new(),\n                message: format!("frontmatter is not valid YAML: {e}"),',
        '                severity: AnalyzeSeverity::Blocking,\n                field: String::new(),\n                message: format!("frontmatter is not valid YAML: {e}"),',
        'validate_frontmatter::tests::frontmatter_that_is_not_valid_yaml_is_hard_fail',
    ),
    (
        'M2 frontmatter that is not a mapping is blocking, not hard-fail',
        'src/primitives/validate_frontmatter.rs',
        '                severity: AnalyzeSeverity::HardFail,\n                field: String::new(),\n                message: "frontmatter must be a mapping".into(),',
        '                severity: AnalyzeSeverity::Blocking,\n                field: String::new(),\n                message: "frontmatter must be a mapping".into(),',
        'validate_frontmatter::tests::frontmatter_that_is_not_a_mapping_is_hard_fail',
    ),
    (
        'M3 an unknown status is blocking, not hard-fail',
        'src/primitives/validate_frontmatter.rs',
        '                    severity: AnalyzeSeverity::HardFail,\n                    field: "status".into(),\n                    message: format!("status \'{s}\' is not one of',
        '                    severity: AnalyzeSeverity::Blocking,\n                    field: "status".into(),\n                    message: format!("status \'{s}\' is not one of',
        'validate_frontmatter::tests::unknown_status_is_hard_fail',
    ),
    (
        'M4 a non-string status is blocking, not hard-fail',
        'src/primitives/validate_frontmatter.rs',
        '            severity: AnalyzeSeverity::HardFail,\n            field: "status".into(),\n            message: "status must be a string".into(),',
        '            severity: AnalyzeSeverity::Blocking,\n            field: "status".into(),\n            message: "status must be a string".into(),',
        'validate_frontmatter::tests::a_non_string_status_is_hard_fail',
    ),
    (
        'M5 a non-string dependency entry is blocking, not hard-fail',
        'src/primitives/validate_frontmatter.rs',
        '                        severity: AnalyzeSeverity::HardFail,\n                        field: format!("dependencies[{i}]"),',
        '                        severity: AnalyzeSeverity::Blocking,\n                        field: format!("dependencies[{i}]"),',
        'validate_frontmatter::tests::a_non_string_dependency_entry_is_hard_fail',
    ),
    (
        'M6 a non-list dependencies value is blocking, not hard-fail',
        'src/primitives/validate_frontmatter.rs',
        '            severity: AnalyzeSeverity::HardFail,\n            field: "dependencies".into(),\n            message: "dependencies must be a list".into(),',
        '            severity: AnalyzeSeverity::Blocking,\n            field: "dependencies".into(),\n            message: "dependencies must be a list".into(),',
        'validate_frontmatter::tests::dependencies_must_be_a_list',
    ),
    (
        'M7 a non-string cross-spec-impact entry is hard-fail, not blocking',
        'src/primitives/validate_frontmatter.rs',
        '                severity: AnalyzeSeverity::Blocking,\n                field: "cross-spec-impact".into(),\n                message: "cross-spec-impact entry must be a string feature name".into(),',
        '                severity: AnalyzeSeverity::HardFail,\n                field: "cross-spec-impact".into(),\n                message: "cross-spec-impact entry must be a string feature name".into(),',
        'validate_frontmatter::tests::a_non_string_cross_spec_impact_entry_is_blocking',
    ),
    (
        'M8 a non-string fold target is hard-fail, not blocking',
        'src/primitives/validate_frontmatter.rs',
        '            severity: AnalyzeSeverity::Blocking,\n            field: "folds-into".into(),\n            message: "folds-into must be a string feature name".into(),',
        '            severity: AnalyzeSeverity::HardFail,\n            field: "folds-into".into(),\n            message: "folds-into must be a string feature name".into(),',
        'validate_frontmatter::tests::a_malformed_fold_target_is_blocking',
    ),
    (
        'M9 a fold target that is not a sequential name is hard-fail, not blocking',
        'src/primitives/validate_frontmatter.rs',
        '            severity: AnalyzeSeverity::Blocking,\n            field: "folds-into".into(),\n            message: format!(',
        '            severity: AnalyzeSeverity::HardFail,\n            field: "folds-into".into(),\n            message: format!(',
        'validate_frontmatter::tests::a_malformed_fold_target_is_blocking',
    ),
    # -- the shared continuation rule and parse check (spec_links) --------
    (
        'M10 a column-zero `- ` entry no longer continues a key',
        'src/primitives/spec_links.rs',
        ' || line == "-" || line.starts_with("- ")\n',
        ' || line == "-"\n',
        'spec_links::tests::indented_lines_and_column_zero_entries_continue_a_key',
    ),
    (
        'M11 a bare `-` entry no longer continues a key',
        'src/primitives/spec_links.rs',
        ' || line == "-" || line.starts_with("- ")\n',
        ' || line.starts_with("- ")\n',
        'spec_links::tests::indented_lines_and_column_zero_entries_continue_a_key derive_dependencies::tests::splice_replaces_a_column_zero_block_sequence',
    ),
    (
        'M12 the parse check passes every block',
        'src/primitives/spec_links.rs',
        '            return serde_norway::from_str::<serde_norway::Value>(&block).is_ok();',
        '            return !block.is_empty() || block.is_empty();',
        'spec_links::tests::frontmatter_parses_judges_the_block_between_the_fences derive_dependencies::tests::a_splice_that_would_not_parse_is_reported_and_never_written derive_references::tests::a_splice_that_would_not_parse_is_reported_and_never_written',
    ),
    (
        'M13 the parse check ends the block only at an exact `---`, unlike the splices',
        'src/primitives/spec_links.rs',
        '        if is_frontmatter_fence(line) {\n            return serde_norway',
        '        if line == "---" {\n            return serde_norway',
        'spec_links::tests::frontmatter_parses_judges_the_block_between_the_fences',
    ),
    (
        'M14 a file with no frontmatter reads as not parsing',
        'src/primitives/spec_links.rs',
        '        Some(first) if is_frontmatter_fence(first) => {}\n        _ => return true,\n    }\n    let mut block',
        '        Some(first) if is_frontmatter_fence(first) => {}\n        _ => return false,\n    }\n    let mut block',
        'spec_links::tests::frontmatter_parses_judges_the_block_between_the_fences',
    ),
    # -- derive-dependencies: the splice, the guard, `absent` -------------
    (
        'M15 derive-dependencies skips only indented continuation lines again',
        'src/primitives/derive_dependencies.rs',
        '            if continues_key(&lines, idx) {\n                continue;\n            }',
        "            if line.starts_with(' ') || line.starts_with('\\t') {\n                continue;\n            }",
        'derive_dependencies::tests::splice_replaces_a_column_zero_block_sequence derive_dependencies::tests::a_column_zero_dependency_list_is_rewritten_on_disk',
    ),
    (
        'M16 derive-dependencies writes a rewrite that does not parse',
        'src/primitives/derive_dependencies.rs',
        '        if !frontmatter_parses(&rewritten) {',
        '        if !frontmatter_parses(&rewritten) && rewritten.is_empty() {',
        'derive_dependencies::tests::a_splice_that_would_not_parse_is_reported_and_never_written',
    ),
    (
        'M17 a tracked spec absent from disk is not named in absent',
        'src/primitives/derive_dependencies.rs',
        '            absent.push(spec.clone());\n            continue;',
        '            let _ = &absent;\n            continue;',
        'derive_dependencies::tests::a_tracked_but_absent_spec_is_named_not_counted',
    ),
    (
        'M18 a tracked spec absent from disk is counted in examined',
        'src/primitives/derive_dependencies.rs',
        '        let path = repo.join(spec);\n        if !path.is_file() {',
        '        let path = repo.join(spec);\n        read_count += usize::from(!path.is_file());\n        if !path.is_file() {',
        'derive_dependencies::tests::a_tracked_but_absent_spec_is_named_not_counted',
    ),
    # -- derive-references: the splice and the guard ----------------------
    (
        'M19 derive-references skips only indented lines of a stale references block again',
        'src/primitives/derive_references.rs',
        '            if skipping {\n                if continues_key(&lines, idx) {',
        "            if skipping {\n                if line.starts_with(' ') || line.starts_with('\\t') {",
        'derive_references::tests::a_column_zero_references_block_is_replaced_whole',
    ),
    (
        'M20 the references block goes straight after the dependencies key line again',
        'src/primitives/derive_references.rs',
        '                out.push(line.to_string());\n                pending = true;\n                continue;',
        '                out.push(line.to_string());\n                out.extend(block.iter().flatten().cloned());\n                inserted = true;\n                continue;',
        'derive_references::tests::block_is_inserted_after_a_block_form_dependencies_entry',
    ),
    (
        'M21 the dependencies entry ends at the first column-zero item',
        'src/primitives/derive_references.rs',
        '            if pending {\n                if continues_key(&lines, idx) {',
        "            if pending {\n                if line.starts_with(' ') || line.starts_with('\\t') {",
        'derive_references::tests::block_is_inserted_after_a_block_form_dependencies_entry',
    ),
    (
        'M22 derive-references writes a rewrite that does not parse',
        'src/primitives/derive_references.rs',
        '        if !frontmatter_parses(&rewritten) {',
        '        if !frontmatter_parses(&rewritten) && rewritten.is_empty() {',
        'derive_references::tests::a_splice_that_would_not_parse_is_reported_and_never_written',
    ),
    # -- check-command-flags: the whole pass ------------------------------
    (
        'M23 a hint with no substitution token is not reported',
        'src/primitives/check_command_flags.rs',
        '        if !has_substitution_token(body) {',
        '        if !has_substitution_token(body) && body.is_empty() {',
        'check_command_flags::tests::run_reports_each_direction_with_its_denominator',
    ),
    (
        'M24 a Flags table with no argument-hint is not reported',
        'src/primitives/check_command_flags.rs',
        '    let Some(hint) = hint else {\n        findings.push(CommandFlagFinding {',
        '    let Some(hint) = hint else {\n        return;\n    };\n    if flags.is_empty() {\n        findings.push(CommandFlagFinding {',
        'check_command_flags::tests::run_reports_each_direction_with_its_denominator',
    ),
    (
        'M25 a tabled flag the hint omits is not reported',
        'src/primitives/check_command_flags.rs',
        '        if !hint_names(&hint, &flag) {',
        '        if !hint_names(&hint, &flag) && hint.is_empty() {',
        'check_command_flags::tests::run_reports_each_direction_with_its_denominator',
    ),
    (
        'M26 a command with a hint is not counted in with-argument-hint',
        'src/primitives/check_command_flags.rs',
        '        with_argument_hint.push(rel.to_string());',
        '        let _ = &with_argument_hint;',
        'check_command_flags::tests::run_reports_each_direction_with_its_denominator',
    ),
    (
        'M27 a command with a Flags table is not counted in with-flags-table',
        'src/primitives/check_command_flags.rs',
        '    with_flags_table.push(rel.to_string());',
        '    let _ = &with_flags_table;',
        'check_command_flags::tests::run_reports_each_direction_with_its_denominator',
    ),
    (
        'M28 no Flags table anywhere reads as a clean run, with no guidance',
        'src/primitives/check_command_flags.rs',
        '    let guidance = if with_flags_table.is_empty() && !examined.is_empty() {',
        '    let guidance = if with_flags_table.is_empty() && examined.is_empty() {',
        'check_command_flags::tests::run_with_no_flags_table_anywhere_reports_an_extraction_failure',
    ),
    # -- check-rule-ids: the line-scoped section scan ---------------------
    (
        'M29 the deprecation scan is cut to a 256-byte window again',
        'src/primitives/check_rule_ids.rs',
        '    let mut lines = content[heading_start..].lines();',
        '    let mut lines = content[heading_start..(heading_start + 256).min(content.len())].lines();',
        'check_rule_ids::tests::multibyte_rule_sections_are_scanned_line_by_line',
    ),
    # -- check-corpus-links: the reference sources are checked ------------
    (
        'M30 the framework/reference/ sources are excluded with their copies',
        'src/primitives/check_corpus_links.rs',
        '|| (generates_references && relative.starts_with(".ductus/reference/"))',
        '|| (generates_references && (relative.starts_with(".ductus/reference/") || relative.starts_with("framework/reference/")))',
        'check_corpus_links::tests::generated_reference_copies_are_excluded_and_the_rest_of_ductus_dir_is_not',
    ),
    # -- resolve-anchor: the markers file's basename ----------------------
    (
        'M31 the markers file is identified by its markers-path alone, not its basename',
        'src/primitives/resolve_anchor.rs',
        '    if let Some(base) = Path::new(path).file_name().and_then(|n| n.to_str()) {\n        names.push(base.to_string());\n    }',
        '    let _ = Path::new(path);',
        'resolve_anchor::tests::a_line_citing_a_nested_markers_file_by_a_relative_path_is_not_qualified',
    ),
    # -- check-promotion-coverage: the required section set ---------------
    (
        'M32 an empty section set is measured instead of refused',
        'src/primitives/check_promotion_coverage.rs',
        '    if args.sections.is_empty() {',
        '    if args.sections.is_empty() && args.rules_file.is_empty() {',
        'check_promotion_coverage::tests::an_empty_section_set_is_refused',
    ),
    # -- a blank line or column-zero comment inside a key's block -------
    (
        'M33 a blank line never continues a key',
        'src/primitives/spec_links.rs',
        '        .is_some_and(|next| is_key_continuation(next))',
        '        .is_some_and(|next| next.is_empty())',
        'spec_links::tests::a_blank_line_continues_a_key_only_when_an_entry_follows_it derive_dependencies::tests::splice_replaces_a_block_sequence_with_blank_lines_inside derive_references::tests::blank_lines_inside_a_block_stay_with_their_key',
    ),
    (
        'M34 a blank line before the next key continues the key above it',
        'src/primitives/spec_links.rs',
        '        .is_some_and(|next| is_key_continuation(next))',
        '        .is_some_and(|next| !next.is_empty())',
        'spec_links::tests::a_blank_line_continues_a_key_only_when_an_entry_follows_it derive_dependencies::tests::splice_replaces_a_block_sequence_with_blank_lines_inside derive_references::tests::blank_lines_inside_a_block_stay_with_their_key',
    ),
    (
        'M35 a whitespace-only line counts as indented content, not blank',
        'src/primitives/spec_links.rs',
        "    let carries_content = |line: &str| !line.trim().is_empty() && !line.starts_with('#');",
        "    let carries_content = |line: &str| !line.is_empty() && !line.starts_with('#');",
        'spec_links::tests::a_blank_line_continues_a_key_only_when_an_entry_follows_it',
    ),
    (
        'M36 a column-zero comment always ends the value',
        'src/primitives/spec_links.rs',
        "    let carries_content = |line: &str| !line.trim().is_empty() && !line.starts_with('#');",
        "    let carries_content = |line: &str| !line.trim().is_empty();",
        'spec_links::tests::a_blank_line_continues_a_key_only_when_an_entry_follows_it derive_dependencies::tests::splice_replaces_a_block_sequence_with_blank_lines_inside derive_references::tests::blank_lines_inside_a_block_stay_with_their_key',
    ),
    (
        'M37 derive-dependencies judges each line alone, with no lookahead past a blank line',
        'src/primitives/derive_dependencies.rs',
        '            if continues_key(&lines, idx) {\n                continue;\n            }',
        '            if crate::primitives::spec_links::is_key_continuation(line) {\n                continue;\n            }',
        'derive_dependencies::tests::splice_replaces_a_block_sequence_with_blank_lines_inside',
    ),
    (
        'M38 derive-references ends a stale references block at a blank line inside it',
        'src/primitives/derive_references.rs',
        '            if skipping {\n                if continues_key(&lines, idx) {',
        '            if skipping {\n                if crate::primitives::spec_links::is_key_continuation(line) {',
        'derive_references::tests::blank_lines_inside_a_block_stay_with_their_key',
    ),
    (
        'M39 derive-references ends the dependencies entry at a blank line inside it',
        'src/primitives/derive_references.rs',
        '            if pending {\n                if continues_key(&lines, idx) {',
        '            if pending {\n                if crate::primitives::spec_links::is_key_continuation(line) {',
        'derive_references::tests::blank_lines_inside_a_block_stay_with_their_key',
    ),
]
