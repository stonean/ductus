"""Mutation list for the owner-approved review fixes to spec 067's runtime
primitives: `validate-frontmatter`'s severities, the two frontmatter splices
(`derive-dependencies`, `derive-references`) and the parse check before their
write, `check-command-flags`' whole pass, `check-rule-ids`' line-scoped
section scan, `check-corpus-links`' reference sources, `resolve-anchor`'s
markers basename, `check-promotion-coverage`'s required section set, the
bound that moves `derive-dependencies`' `unwritable` past the read size, the
exec analyze tally's step 6, `check-artifacts`' status-gated path families
and `gitignored` skip, and `validate-frontmatter`'s row chosen by path, and `resolve-anchor`'s judgment of
qualified, registered and span references with the exec tally's unjudged
counts, `check-artifacts`' unreadable `plan.md` under `plan-consistency`,
`check-rule-ids`' loaded files deciding before its shipped index, and
`derive-references`' bounded result (task 32: M93 to M95).

Task 32's two CLI behaviors live in `src/main.rs`, which `cargo test --lib`
does not build: the gate reading `unwritable-count`, and the stderr lines
reported from the whole derivation rather than the bounded result. Each was
proven by hand on 2026-11-14 to fail
`past_the_read_size_an_unwritable_rewrite_still_exits_1_and_is_named` in
`runtime/tests/derive_unwritable_cli.rs` (the gate's mutant also fails the
binary's `an_unwritable_spec_moved_to_the_lists_file_still_blocks`): run
`cargo test --locked --test derive_unwritable_cli` with either applied.
M93 also fails `tests/mcp.rs`'s
`every_result_that_grows_with_the_corpus_moves_its_lists_past_the_read_size_via_mcp`,
run by hand with `cargo test --locked --test mcp -- past_the_read_size`.

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
        'spec_links::tests::frontmatter_parses_judges_the_block_between_the_fences derive_dependencies::tests::a_splice_that_would_not_parse_is_unwritable_and_never_written derive_references::tests::a_splice_that_would_not_parse_is_unwritable_and_never_written',
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
        '            if idx <= flow_end || continues_key(&lines, idx) {\n                continue;\n            }',
        "            if idx <= flow_end || line.starts_with(' ') || line.starts_with('\\t') {\n                continue;\n            }",
        'derive_dependencies::tests::splice_replaces_a_column_zero_block_sequence derive_dependencies::tests::a_column_zero_dependency_list_is_rewritten_on_disk',
    ),
    (
        'M16 derive-dependencies writes a rewrite that does not parse',
        'src/primitives/derive_dependencies.rs',
        '        if !frontmatter_parses(&rewritten) {',
        '        if !frontmatter_parses(&rewritten) && rewritten.is_empty() {',
        'derive_dependencies::tests::a_splice_that_would_not_parse_is_unwritable_and_never_written',
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
        '            if skipping {\n                if idx <= flow_end || continues_key(&lines, idx) {',
        "            if skipping {\n                if idx <= flow_end || line.starts_with(' ') || line.starts_with('\\t') {",
        'derive_references::tests::a_column_zero_references_block_is_replaced_whole',
    ),
    (
        'M20 the references block goes straight after the dependencies key line again',
        'src/primitives/derive_references.rs',
        '                out.push(line.to_string());\n                pending = true;\n                flow_end = flow_value_end(&lines, idx);\n                continue;',
        '                out.push(line.to_string());\n                out.extend(block.iter().flatten().cloned());\n                inserted = true;\n                flow_end = flow_value_end(&lines, idx);\n                continue;',
        'derive_references::tests::block_is_inserted_after_a_block_form_dependencies_entry',
    ),
    (
        'M21 the dependencies entry ends at the first column-zero item',
        'src/primitives/derive_references.rs',
        '            if pending {\n                if idx <= flow_end || continues_key(&lines, idx) {',
        "            if pending {\n                if idx <= flow_end || line.starts_with(' ') || line.starts_with('\\t') {",
        'derive_references::tests::block_is_inserted_after_a_block_form_dependencies_entry',
    ),
    (
        'M22 derive-references writes a rewrite that does not parse',
        'src/primitives/derive_references.rs',
        '        if !frontmatter_parses(&rewritten) {',
        '        if !frontmatter_parses(&rewritten) && rewritten.is_empty() {',
        'derive_references::tests::a_splice_that_would_not_parse_is_unwritable_and_never_written',
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
        'src/primitives/rule_sections.rs',
        '    let mut lines = content[heading_start..].lines().enumerate();',
        '    let mut lines = content[heading_start..(heading_start + 256).min(content.len())].lines().enumerate();',
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
        'M31 a document token resolves from the root alone, so a nested markers file cited by a relative path is another document',
        'src/primitives/resolve_anchor.rs',
        '        let bases = [self.citing_dir.clone(), PathBuf::new()];',
        '        let bases = [PathBuf::new(), PathBuf::new()];',
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
        '            if idx <= flow_end || continues_key(&lines, idx) {\n                continue;\n            }',
        '            if idx <= flow_end || crate::primitives::spec_links::is_key_continuation(line) {\n                continue;\n            }',
        'derive_dependencies::tests::splice_replaces_a_block_sequence_with_blank_lines_inside',
    ),
    (
        'M38 derive-references ends a stale references block at a blank line inside it',
        'src/primitives/derive_references.rs',
        '            if skipping {\n                if idx <= flow_end || continues_key(&lines, idx) {',
        '            if skipping {\n                if idx <= flow_end || crate::primitives::spec_links::is_key_continuation(line) {',
        'derive_references::tests::blank_lines_inside_a_block_stay_with_their_key',
    ),
    (
        'M39 derive-references ends the dependencies entry at a blank line inside it',
        'src/primitives/derive_references.rs',
        '            if pending {\n                if idx <= flow_end || continues_key(&lines, idx) {',
        '            if pending {\n                if idx <= flow_end || crate::primitives::spec_links::is_key_continuation(line) {',
        'derive_references::tests::blank_lines_inside_a_block_stay_with_their_key',
    ),
    # -- a multi-line flow value, and the unwritable list ---------------
    (
        'M40 derive-dependencies stops a replaced flow value at a column-zero bracket',
        'src/primitives/derive_dependencies.rs',
        '            if idx <= flow_end || continues_key(&lines, idx) {',
        '            if continues_key(&lines, idx) {',
        'derive_dependencies::tests::a_flow_list_closing_at_column_zero_is_rewritten',
    ),
    (
        'M41 derive-references ends a stale references flow list at a column-zero bracket',
        'src/primitives/derive_references.rs',
        '            if skipping {\n                if idx <= flow_end || continues_key(&lines, idx) {',
        '            if skipping {\n                if continues_key(&lines, idx) {',
        'derive_references::tests::a_flow_list_closing_at_column_zero_is_rewritten',
    ),
    (
        'M42 derive-references ends the dependencies flow list at a column-zero bracket',
        'src/primitives/derive_references.rs',
        '            if pending {\n                if idx <= flow_end || continues_key(&lines, idx) {',
        '            if pending {\n                if continues_key(&lines, idx) {',
        'derive_references::tests::a_flow_list_closing_at_column_zero_is_rewritten',
    ),
    (
        'M43 derive-dependencies reports a refused rewrite as unparseable, not unwritable',
        'src/primitives/derive_dependencies.rs',
        '            unwritable.push(spec.clone());',
        '            unparseable.push(spec.clone());',
        'derive_dependencies::tests::a_splice_that_would_not_parse_is_unwritable_and_never_written',
    ),
    (
        'M44 derive-references reports a refused rewrite as unparseable, not unwritable',
        'src/primitives/derive_references.rs',
        '            unwritable.push(spec.clone());',
        '            unparseable.push(spec.clone());',
        'derive_references::tests::a_splice_that_would_not_parse_is_unwritable_and_never_written',
    ),
    (
        'M45 derive-dependencies reports an unterminated fence as unwritable',
        'src/primitives/derive_dependencies.rs',
        '            // No closing fence means the splice below has no anchor, so the\n            // spec is left alone. Say so rather than letting it read as clean.\n            unparseable.push(spec.clone());',
        '            // No closing fence means the splice below has no anchor, so the\n            // spec is left alone. Say so rather than letting it read as clean.\n            unwritable.push(spec.clone());',
        'derive_dependencies::tests::an_unterminated_fence_is_unparseable_not_unwritable',
    ),
    (
        'M46 derive-references reports an unterminated fence as unwritable',
        'src/primitives/derive_references.rs',
        '        if has_unterminated_frontmatter(&content) {\n            unparseable.push(spec.clone());',
        '        if has_unterminated_frontmatter(&content) {\n            unwritable.push(spec.clone());',
        'derive_references::tests::an_unterminated_fence_is_unparseable_not_unwritable',
    ),
    (
        'M47 the bounded derive-dependencies result drops unwritable-count',
        'src/primitives/derive_dependencies.rs',
        '        unwritable_count: derived.unwritable.len(),',
        '        unwritable_count: 0,',
        'derive_dependencies::tests::a_splice_that_would_not_parse_is_unwritable_and_never_written',
    ),
    (
        'M48 a flow value is never found to balance',
        'src/primitives/spec_links.rs',
        '        if depth <= 0 {\n            return offset;',
        '        if depth <= 0 {\n            return key_idx;',
        'spec_links::tests::a_flow_value_ends_at_its_balancing_bracket derive_dependencies::tests::a_flow_list_closing_at_column_zero_is_rewritten',
    ),
    (
        'M49 a bracket in a comment counts toward a flow value',
        'src/primitives/spec_links.rs',
        "                '#' if previous.is_whitespace() => break,\n",
        '',
        'spec_links::tests::a_flow_value_ends_at_its_balancing_bracket',
    ),
    (
        'M50 a bracket in a quoted scalar counts toward a flow value',
        'src/primitives/spec_links.rs',
        "                '\"' | '\\'' => quote = Some(c),\n",
        '',
        'spec_links::tests::a_flow_value_ends_at_its_balancing_bracket',
    ),
    # -- derive-dependencies' bound, and the exec tally's step 6 (task 25) --
    (
        'M51 the file-form derive-dependencies result keeps unwritable inline',
        'src/primitives/derive_dependencies.rs',
        '        result.unparseable = None;\n        result.unwritable = None;\n',
        '        result.unparseable = None;\n',
        'derive_dependencies::tests::past_the_read_size_unwritable_moves_to_the_list_file',
    ),
    (
        'M52 the derive-dependencies list file leaves out unwritable',
        'src/primitives/derive_dependencies.rs',
        '        "unwritable": derived.unwritable,\n',
        '',
        'derive_dependencies::tests::past_the_read_size_unwritable_moves_to_the_list_file',
    ),
    (
        'M53 the exec analyze tally ignores unwritable',
        'src/interpreter/analyze_tally.rs',
        '                for counted in ["unwritable-count", "unparseable-count"] {',
        '                for counted in ["unparseable-count"] {',
        'analyze_tally::tests::unwritable_and_unparseable_dependencies_are_advisory',
    ),
    (
        'M54 the exec analyze tally ignores unparseable',
        'src/interpreter/analyze_tally.rs',
        '                for counted in ["unwritable-count", "unparseable-count"] {',
        '                for counted in ["unwritable-count"] {',
        'analyze_tally::tests::unwritable_and_unparseable_dependencies_are_advisory',
    ),
    # -- the guard ahead of the staged filter (task 14's fourth item) ------
    (
        'M55 derive-dependencies names an unwritable spec only when it is staged',
        'src/primitives/derive_dependencies.rs',
        '        if !frontmatter_parses(&rewritten) {',
        '        if staged.as_ref().is_none_or(|set| set.contains(spec)) && !frontmatter_parses(&rewritten) {',
        'derive_dependencies::tests::an_unstaged_unwritable_spec_is_named_under_staged',
    ),
    (
        'M56 derive-references names an unwritable spec only when it is staged',
        'src/primitives/derive_references.rs',
        '        if !frontmatter_parses(&rewritten) {',
        '        if staged.as_ref().is_none_or(|set| set.contains(spec)) && !frontmatter_parses(&rewritten) {',
        'derive_references::tests::an_unstaged_unwritable_spec_is_named_under_staged',
    ),
    # -- check-artifacts: the status-gated path families (task 17) --------
    (
        'M57 the path families examine a spec at done only',
        'src/primitives/check_artifacts.rs',
        'const PATH_FAMILY_STATUSES: [&str; 2] = ["in-progress", "done"];',
        'const PATH_FAMILY_STATUSES: [&str; 2] = ["done", "done"];',
        'an_in_progress_criterion_naming_a_deleted_path_is_a_finding an_in_progress_body_naming_a_deleted_path_is_a_finding',
    ),
    (
        'M58 a status-gated path family records no skip',
        'src/primitives/check_artifacts.rs',
        '    record_skip(skipped, family, "status-gated", spec_md);\n    false',
        '    let _ = (&skipped, family, spec_md);\n    false',
        'a_criterion_below_in_progress_is_status_gated a_body_below_in_progress_is_status_gated',
    ),
    (
        'M59 the criterion family ignores its status gate',
        'src/primitives/check_artifacts.rs',
        '    if !path_family_examines(skipped, "criterion-path-existence", status, &citing) {\n        return;\n    }',
        '    let _ = status;',
        'a_criterion_below_in_progress_is_status_gated',
    ),
    (
        'M60 the body family ignores its status gate',
        'src/primitives/check_artifacts.rs',
        '    if !path_family_examines(skipped, BODY_PATH_FAMILY, status, &spec_md) {\n        return;\n    }',
        '    let _ = (status, &spec_md);',
        'a_body_below_in_progress_is_status_gated',
    ),
    # -- check-artifacts: a criterion path git ignores (task 17) -----------
    (
        'M61 a criterion path git ignores is a finding, not a gitignored skip',
        'src/primitives/check_artifacts.rs',
        '            if ignore_rules.ignores(&candidate) {',
        '            if false && ignore_rules.ignores(&candidate) {',
        'a_criterion_path_git_ignores_is_a_gitignored_skip a_subdirectory_project_reads_its_ignore_rules_through_the_work_tree',
    ),
    (
        'M62 every unresolved criterion path reads as ignored',
        'src/primitives/check_artifacts.rs',
        '            if ignore_rules.ignores(&candidate) {',
        '            if true || ignore_rules.ignores(&candidate) {',
        'an_unignored_absent_criterion_path_is_still_a_finding',
    ),
    (
        'M63 the ignore lookup skips the work-tree conversion',
        'src/primitives/check_artifacts.rs',
        '                .is_path_ignored(project.to_git(rel))',
        '                .is_path_ignored(rel)',
        'a_subdirectory_project_reads_its_ignore_rules_through_the_work_tree',
    ),
    (
        'M64 outside a repository every path reads as ignored',
        'src/primitives/check_artifacts.rs',
        '        project.as_ref().is_some_and(|project| {',
        '        project.as_ref().is_none_or(|project| {',
        'check_artifacts::tests::the_originating_case_is_reproduced',
    ),
    # -- validate-frontmatter: the path chooses the row (task 18) --------
    (
        'M65 a file under scenarios/ takes the spec row',
        'src/primitives/validate_frontmatter.rs',
        '        return Some(Artifact::Scenario);',
        '        return Some(Artifact::Spec);',
        'validate_frontmatter::tests::a_section_only_scenario_is_clean',
    ),
    (
        'M66 a scenario with no section passes',
        'src/primitives/validate_frontmatter.rs',
        '    if names_a_section("section") || names_a_section("spec-ref") {',
        '    if true {',
        'validate_frontmatter::tests::a_scenario_missing_section_is_a_hard_fail',
    ),
    (
        'M67 the legacy spec-ref no longer names a section',
        'src/primitives/validate_frontmatter.rs',
        'names_a_section("section") || names_a_section("spec-ref")',
        'names_a_section("section")',
        'validate_frontmatter::tests::a_legacy_spec_ref_scenario_is_clean',
    ),
    (
        'M68 a spec.md run skips the scenarios beside it',
        'src/primitives/validate_frontmatter.rs',
        '                validate_scenarios_beside(dir, &mut findings);\n',
        '',
        'validate_frontmatter::tests::a_spec_run_reports_a_malformed_scenario_by_its_path',
    ),
    (
        'M69 a fenceless scenario beside a spec passes silently',
        'src/primitives/validate_frontmatter.rs',
        '                report(format!(\n                    "{field} has no frontmatter (no leading `---` block)"\n                ));\n',
        '',
        'validate_frontmatter::tests::an_unparseable_scenario_in_a_spec_run_is_a_hard_fail_not_an_error',
    ),
    (
        'M70 a scenario beside a spec that cannot be read passes silently',
        'src/primitives/validate_frontmatter.rs',
        '                report(format!("{field} could not be read: {source}"));\n',
        '',
        'validate_frontmatter::tests::a_scenario_a_spec_run_cannot_read_is_a_hard_fail',
    ),
    (
        'M71 a path of no row takes the spec row',
        'src/primitives/validate_frontmatter.rs',
        '        ANALYSIS_RECORD_FILE => Some(Artifact::Record(ANALYSIS_RECORD_FILE)),\n        _ => None,',
        '        ANALYSIS_RECORD_FILE => Some(Artifact::Record(ANALYSIS_RECORD_FILE)),\n        _ => Some(Artifact::Spec),',
        'validate_frontmatter::tests::a_path_of_no_schema_row_is_refused',
    ),
    (
        'M72 a direct record path is not checked',
        'src/primitives/validate_frontmatter.rs',
        '                validate_record(dir, file, &mut findings);\n',
        '',
        'validate_frontmatter::tests::a_direct_record_path_takes_its_record_checks',
    ),
    (
        'M73 an absent direct record passes as never-run',
        'src/primitives/validate_frontmatter.rs',
        '            read_text(&path)?;\n',
        '',
        'validate_frontmatter::tests::an_absent_direct_record_path_is_an_error',
    ),
    (
        'M74 a spec or record name under scenarios/ is not a scenario',
        'src/primitives/validate_frontmatter.rs',
        '    if under_scenarios && markdown {',
        '    if under_scenarios && markdown && name != "spec.md" && name != REVIEW_RECORD_FILE {',
        'validate_frontmatter::tests::a_file_under_scenarios_takes_the_scenario_row_whatever_its_name',
    ),
    (
        'M75 a spec-row finding stops the rest of a spec.md run',
        'src/primitives/validate_frontmatter.rs',
        '            validate_spec_row(&path, &mut findings)?;\n            if let Some(dir) = path.parent() {',
        '            validate_spec_row(&path, &mut findings)?;\n            if findings.is_empty() && let Some(dir) = path.parent() {',
        'validate_frontmatter::tests::a_spec_that_will_not_parse_still_has_its_scenarios_checked',
    ),
    # -- resolve-anchor: qualified, registered and span references judged (task 19)
    (
        'M76 the markers file is recognized by a basename substring again',
        'src/primitives/resolve_anchor.rs',
        '            .is_some_and(|markers| relatives.contains(markers))',
        '            .is_some_and(|markers| relatives.contains(markers) || token.contains("constitution"))',
        'a_lookalike_of_the_markers_file_is_another_document',
    ),
    (
        'M77 no registered constitution is read',
        'src/primitives/resolve_anchor.rs',
        '        for record in registry.loaded.iter().chain(&registry.skipped) {',
        '        for record in registry.loaded.iter().chain(&registry.skipped).take(0) {',
        'same_form_references_into_a_registered_constitution_classify_alike a_reference_into_a_registered_constitution_is_judged_against_its_markers a_reference_naming_a_skipped_registered_entry_is_unexamined_as_unreadable',
    ),
    (
        'M78 a skipped registered entry reads as absent, not unreadable',
        'src/primitives/resolve_anchor.rs',
        '            return markers.clone().map_or(Named::Unreadable, |markers| {',
        '            return markers.clone().map_or(Named::Absent, |markers| {',
        'a_reference_naming_a_skipped_registered_entry_is_unexamined_as_unreadable',
    ),
    (
        'M79 a qualified reference no document carries is unjudged, never unresolved',
        'src/primitives/resolve_anchor.rs',
        '    } else {\n        Verdict::Unresolved\n    }\n}',
        '    } else {\n        Verdict::Unjudged(UNJUDGEABLE)\n    }\n}',
        'a_reference_on_a_line_naming_a_readable_document_is_judged_against_it a_line_naming_two_documents_is_judged_against_both',
    ),
    (
        "M80 a named document's headings are not read",
        'src/primitives/resolve_anchor.rs',
        '                if names_local_heading(&sections.headings, rest)\n                    || sections.markers.contains(anchor)\n                {',
        '                if sections.markers.contains(anchor) {',
        'a_reference_on_a_line_naming_a_readable_document_is_judged_against_it a_line_naming_two_documents_is_judged_against_both',
    ),
    (
        'M81 a line naming only absent documents reads as unresolved',
        'src/primitives/resolve_anchor.rs',
        '    } else if non_markdown || all_absent {',
        '    } else if non_markdown {',
        'a_reference_naming_only_absent_documents_is_unexamined',
    ),
    (
        'M82 a readable document beside an absent one no longer counts against the absent excuse',
        'src/primitives/resolve_anchor.rs',
        '                    return Verdict::Resolved;\n                }\n                all_absent = false;\n            }',
        '                    return Verdict::Resolved;\n                }\n            }',
        'a_reference_naming_only_absent_documents_is_unexamined',
    ),
    (
        'M83 an unreadable document reads as unjudgeable',
        'src/primitives/resolve_anchor.rs',
        '    if unreadable {\n        Verdict::Unjudged(UNREADABLE)',
        '    if unreadable {\n        Verdict::Unjudged(UNJUDGEABLE)',
        'a_mapped_source_that_cannot_be_read_is_unexamined_as_unreadable a_reference_naming_an_unreadable_document_is_unexamined_as_unreadable',
    ),
    (
        'M84 a backticked path that is not markdown names no document',
        'src/primitives/resolve_anchor.rs',
        "            if !is_markdown(word) && word.contains('/') && has_extension(word) && !is_url(word) {",
        '            if false {',
        'a_reference_beside_a_non_markdown_file_is_unexamined',
    ),
    (
        'M85 a non-markdown file beside a readable document no longer excuses an unmatched reference',
        'src/primitives/resolve_anchor.rs',
        '    } else if non_markdown || all_absent {',
        '    } else if all_absent {',
        'a_reference_beside_a_non_markdown_file_is_unexamined',
    ),
    (
        'M86 a code span holding a path and a reference is not read',
        'src/primitives/resolve_anchor.rs',
        '            let Some((token, offset)) = span_citation(&line[span.clone()]) else {',
        '            let Some((token, offset)) = span_citation(&line[span.clone()]).filter(|_| false) else {',
        'a_span_holding_a_path_and_one_reference_is_judged',
    ),
    (
        'M87 a code span holding two references reads as a citation',
        'src/primitives/resolve_anchor.rs',
        "    if content.matches('§').count() != 1 {",
        "    if content.matches('§').count() == 0 {",
        'a_span_holding_a_path_and_one_reference_is_judged',
    ),
    (
        'M88 a shipped destination of the markers file is judged as another document',
        'src/primitives/resolve_anchor.rs',
        '            if self.markers_rel.as_ref() == normalize_rel(Path::new(""), &source).as_ref() {',
        '            if false {',
        'a_shipped_destination_of_the_markers_file_is_the_markers_file',
    ),
    (
        'M89 the exec tally drops the unjudged anchor counts',
        'src/interpreter/analyze_tally.rs',
        '                    self.record_unexamined(reason, unjudged);',
        '                    let _ = (reason, unjudged);',
        'an_unjudged_anchor_reference_is_recorded_unexamined',
    ),
    (
        'M90 a link to a markdown document by URL names no document',
        'src/primitives/resolve_anchor.rs',
        '        let names = if is_url(path) {\n            is_markdown(path)\n        } else {',
        '        let names = if is_url(path) {\n            false\n        } else {',
        'a_url_to_a_markdown_document_names_an_absent_document',
    ),
    (
        'M91 plan-consistency returns silently over a plan.md it cannot read (task 31)',
        'src/primitives/check_artifacts.rs',
        '        Err(_) => {\n            record_unreadable_artifact(findings, skipped, "plan-consistency", status, path);\n            return;\n        }',
        '        Err(_) => return,',
        'an_unreadable_plan',
    ),
    (
        'M92 the shipped index decides before the loaded rule files (033 review, QUAL-TEST-001)',
        'src/primitives/check_rule_ids.rs',
        '        match known.get(&id).copied() {',
        '        match (if shipped_rule_ids().contains(id.as_str()) { None } else { known.get(&id).copied() }) {',
        'an_installed_shipped_rule_is_found',
    ),
    # -- derive-references: the bounded result (task 32) -----------------
    (
        'M93 derive-references returns its lists inline past the read size',
        'src/primitives/derive_references.rs',
        '    if let Some(file) = super::bounded_lists::place(\n        repo,\n        "derive-references",\n        super::bounded_lists::ALL_SPECS,\n        &result,\n        &lists,\n    )? {',
        '    let _ = &lists;\n    if let Some(file) = None::<String> {',
        'derive_references::tests::past_the_read_size_every_list_moves_to_the_list_file',
    ),
    (
        'M94 derive-references leaves unwritable inline in the file form',
        'src/primitives/derive_references.rs',
        '        result.unwritable = None;\n        result.absent = None;\n        result.lists_file = Some(file);',
        '        result.absent = None;\n        result.lists_file = Some(file);',
        'derive_references::tests::past_the_read_size_every_list_moves_to_the_list_file',
    ),
    (
        'M95 derive-references rewrites specs before it reads the read size',
        'src/primitives/derive_references.rs',
        'pub fn compute(args: &DeriveReferencesArgs, repo: &Path) -> Result<Derived> {\n    super::check_artifact_size::resolve_threshold(repo)?;\n',
        'pub fn compute(args: &DeriveReferencesArgs, repo: &Path) -> Result<Derived> {\n',
        'derive_references::tests::a_malformed_read_size_halts_a_writing_call_before_any_write',
    ),
]
