"""Mutation list for the departure reader and its history attribution (spec 073,
task 1): `runtime/src/primitives/departures.rs` parses a spec's `departs:`
entries under `## Applicable Rules`, reports each structural problem with its
line, and attributes an entry to the first commit of the unbroken run ending at
`HEAD`. Each mutation removes one structural problem or one attribution state,
so a kill shows its own test holds it.

Run with `python3 scripts/mutation/run.py scripts/mutation/departures.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    # -- structural problems and the grammar --------------------------------
    (
        'M1 a departs item with no rule ID reports nothing',
        'src/primitives/departures.rs',
        'if ids.is_empty() {',
        'if false {',
        'an_entry_with_no_rule_id',
    ),
    (
        'M2 a second rule ID is accepted',
        'src/primitives/departures.rs',
        'if ids.len() > 1 {',
        'if false {',
        'a_second_rule_id',
    ),
    (
        'M3 a missing reason is accepted',
        'src/primitives/departures.rs',
        'None => entry.problems.push(Problem {\n            line: entry.line,\n            kind: ProblemKind::MissingReason,\n        }),',
        'None => {}',
        'a_missing_reason',
    ),
    (
        'M4 an empty reason is accepted',
        'src/primitives/departures.rs',
        'Some("") => entry.problems.push(Problem {\n            line: entry.line,\n            kind: ProblemKind::EmptyReason,\n        }),',
        'Some("") => {}',
        'an_empty_reason',
    ),
    (
        'M5 another child key is accepted',
        'src/primitives/departures.rs',
        'entry.problems.push(Problem {\n                line,\n                kind: ProblemKind::UnknownKey(other.to_string()),\n            });',
        'let _ = other;',
        'another_child_key',
    ),
    (
        'M6 a repeated child key is accepted',
        'src/primitives/departures.rs',
        'if slot.is_some() {',
        'if false {',
        'a_repeated_child_key',
    ),
    (
        'M7 a rule two entries name is not reported',
        'src/primitives/departures.rs',
        'if parsed.entries.iter().any(|earlier| earlier.rule == rule) {',
        'if false {',
        'a_rule_two_entries_name',
    ),
    (
        'M8 a malformed entry still waives its rule',
        'src/primitives/departures.rs',
        '(well_formed, entry.rule, entry.reason)',
        '(true, entry.rule, entry.reason)',
        'makes_the_entry_malformed a_missing_reason an_empty_reason',
    ),
    (
        'M9 fenced code and HTML comments are read as entries',
        'src/primitives/departures.rs',
        'if skip.skip(raw) || !range.contains(&index) || index == range.start {',
        'if !range.contains(&index) || index == range.start {',
        'an_entry_in_a_comment_or_a_fence',
    ),
    (
        'M10 an entry outside Applicable Rules is read',
        'src/primitives/departures.rs',
        'if skip.skip(raw) || !range.contains(&index) || index == range.start {',
        'if skip.skip(raw) || index == range.start {',
        'an_entry_outside_applicable_rules',
    ),
    (
        'M11 the legacy citation bullet opens an entry',
        'src/primitives/departures.rs',
        'if let Some(rest) = item.and_then(|rest| rest.strip_prefix("departs:")) {',
        'if let Some(rest) = item.map(|rest| rest.strip_prefix("departs:").unwrap_or(rest)) {',
        'the_legacy_citation_bullet',
    ),
    (
        'M12 a wrapped reason line is dropped',
        'src/primitives/departures.rs',
        'value.push_str(text);',
        'let _ = text;',
        'a_wrapped_reason',
    ),
    (
        'M13 an empty cited child reads as a citation',
        'src/primitives/departures.rs',
        'entry.cited.filter(|cited| !cited.is_empty())',
        'entry.cited',
        'a_cited_child_with_no_text',
    ),
    # -- attribution states ------------------------------------------------
    (
        'M14 introduced: the walk stops at HEAD, attributing the newest commit',
        'src/primitives/departures.rs',
        'if open.is_empty() {',
        'if true {',
        'an_entry_is_attributed_to_the_commit_that_introduced_it an_edit_after_the_commit',
    ),
    (
        'M15 a run is not closed by a commit without the entry',
        'src/primitives/departures.rs',
        'let present = held.contains(*rule);',
        'let present = held.contains(*rule) || true;',
        'an_entry_removed_and_added_back an_uncommitted_entry an_entry_removed_from_head',
    ),
    (
        'M16 a spec directory rename is not followed',
        'src/primitives/departures.rs',
        'git_path = earlier;',
        'let _ = earlier;',
        'a_renamed_spec_directory',
    ),
    (
        'M17 a parent without the spec is an unreadable revision',
        'src/primitives/departures.rs',
        'Err(err) if err.code() == git2::ErrorCode::NotFound => return Ok(BTreeSet::new()),',
        'Err(err) if err.code() == git2::ErrorCode::NotFound => return Err(err),',
        'a_file_added_with_no_rename_to_follow an_entry_is_attributed_to_the_commit_that_introduced_it',
    ),
    (
        'M18 the project prefix is not applied to the spec path',
        'src/primitives/departures.rs',
        'let mut git_path = project.to_git(spec_rel);',
        'let mut git_path = spec_rel.to_string();',
        'a_project_in_a_repository_subdirectory',
    ),
    (
        'M19 a shallow repository is attributed',
        'src/primitives/departures.rs',
        'if project.repository.is_shallow() {',
        'if false {',
        'a_shallow_repository',
    ),
    (
        'M20 an unreadable revision keeps the newer part of the run',
        'src/primitives/departures.rs',
        'introduced.remove(*rule);',
        'let _ = rule;',
        'an_unreadable_revision',
    ),
    (
        'M21 a project with no repository is attributed',
        'src/primitives/departures.rs',
        'let Ok(project) = ProjectRepository::discover(project_root) else {\n        return result;\n    };',
        'let Ok(project) = ProjectRepository::discover(project_root) else {\n        return rules\n            .iter()\n            .map(|rule| {\n                (\n                    rule.clone(),\n                    Attribution::Attributed {\n                        decided_by: String::new(),\n                        decided_at: String::new(),\n                    },\n                )\n            })\n            .collect();\n    };',
        'a_project_with_no_repository_or_no_commit',
    ),
    (
        'M22 the cached blob is reused whatever blob the next commit holds',
        'src/primitives/departures.rs',
        '.filter(|(oid, _)| *oid == entry.id())',
        '.filter(|(_oid, _)| true)',
        'an_entry_removed_and_added_back each_rule_is_attributed_by_its_own_run',
    ),
]
