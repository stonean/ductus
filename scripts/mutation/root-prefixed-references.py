"""Mutation list for spec 051 task 30: a managed path behind a prefix that
names the project root from its referrer is a reference, for
`check-orphaned-references` and `rewrite-spec-links` alike —
`./` or a leading `/` in a root markdown file, `./` in `.githooks/pre-commit`,
`../` or a leading `/` in the spec root's `system.md` — and any other prefix
is not (`root_prefixes`, `root_prefix_before` and `managed_path_spans` in
runtime/src/primitives/check_orphaned_references.rs).

Run with `python3 scripts/mutation/run.py scripts/mutation/root-prefixed-references.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The rule lives in the shared scanner, so each mutation of it runs twice:
M1-M9 and M14 filtered to the check's tests, and M15-M24 the same mutations
filtered to the rewrite's, so a kill shows that each primitive's own tests
hold it. M10 and M11 (the token-start rules read before the prefix) and M12
are the check's alone, M13 the rewrite's.

The killing tests are `a_path_behind_a_prefix_naming_the_project_root_is_checked`,
`a_path_behind_any_other_prefix_is_not_a_reference` and
`the_import_and_revision_rules_apply_where_the_prefix_begins` in
check_orphaned_references.rs, and
`a_reference_behind_a_project_root_prefix_is_re_pointed_with_its_prefix` and
`a_reference_behind_any_other_prefix_is_left_as_written` in
rewrite_spec_links.rs.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    # -- the check's tests -----------------------------------------------------
    (
        'M1 a root markdown file does not take ./ (check)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 0) => &["./", "/"],\n',
        '        (true, 0) => &["/"],\n',
        'check_orphaned_references::',
    ),
    (
        'M2 a root markdown file does not take a leading / (check)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 0) => &["./", "/"],\n',
        '        (true, 0) => &["./"],\n',
        'check_orphaned_references::',
    ),
    (
        'M3 system.md does not take ../ (check)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 1) => &["../", "/"],\n',
        '        (true, 1) => &["/"],\n',
        'check_orphaned_references::',
    ),
    (
        'M4 system.md does not take a leading / (check)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 1) => &["../", "/"],\n',
        '        (true, 1) => &["../"],\n',
        'check_orphaned_references::',
    ),
    (
        'M5 the hook does not take ./ (check)',
        'src/primitives/check_orphaned_references.rs',
        '        _ => &["./"],\n',
        '        _ => &[],\n',
        'check_orphaned_references::',
    ),
    (
        'M6 a root markdown file takes ../, which leaves the project (check)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 0) => &["./", "/"],\n',
        '        (true, 0) => &["../", "./", "/"],\n',
        'check_orphaned_references::',
    ),
    (
        'M7 system.md takes ./, which names the spec root (check)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 1) => &["../", "/"],\n',
        '        (true, 1) => &["../", "./", "/"],\n',
        'check_orphaned_references::',
    ),
    (
        'M8 the hook takes a leading /, an absolute path (check)',
        'src/primitives/check_orphaned_references.rs',
        '        _ => &["./"],\n',
        '        _ => &["./", "/"],\n',
        'check_orphaned_references::',
    ),
    (
        'M9 the shortest matching prefix is taken, not the longest (check)',
        'src/primitives/check_orphaned_references.rs',
        '        .map(|prefix| prefix.len())\n        .max()\n',
        '        .map(|prefix| prefix.len())\n        .min()\n',
        'check_orphaned_references::',
    ),
    (
        'M10 the import rule reads before the root, not before the prefix',
        'src/primitives/check_orphaned_references.rs',
        '                    && line[..token - 1]\n',
        '                    && line[..start - 1]\n',
        'check_orphaned_references::',
    ),
    (
        'M11 the revision rule reads before the root, not before the prefix',
        'src/primitives/check_orphaned_references.rs',
        'names_a_git_object(&line[..token - 1])',
        'names_a_git_object(&line[..start - 1])',
        'check_orphaned_references::',
    ),
    (
        'M12 the check passes its referrers no prefixes',
        'src/primitives/check_orphaned_references.rs',
        '        let prefixes = root_prefixes(&referrer);\n',
        '        let prefixes: &[&str] = &[];\n',
        'check_orphaned_references::',
    ),
    (
        'M13 the rewrite passes its referrers no prefixes',
        'src/primitives/rewrite_spec_links.rs',
        '                prefixes: check_orphaned_references::root_prefixes(shown),\n',
        '                prefixes: &[],\n',
        'rewrite_spec_links::',
    ),
    (
        'M14 a span starts at the prefix, not at the root (check)',
        'src/primitives/check_orphaned_references.rs',
        '                found.push(start..start + candidate.len());\n',
        '                found.push(token..start + candidate.len());\n',
        'check_orphaned_references::',
    ),
    # -- the rewrite's tests ---------------------------------------------------
    (
        'M15 a root markdown file does not take ./ (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 0) => &["./", "/"],\n',
        '        (true, 0) => &["/"],\n',
        'rewrite_spec_links::',
    ),
    (
        'M16 a root markdown file does not take a leading / (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 0) => &["./", "/"],\n',
        '        (true, 0) => &["./"],\n',
        'rewrite_spec_links::',
    ),
    (
        'M17 system.md does not take ../ (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 1) => &["../", "/"],\n',
        '        (true, 1) => &["/"],\n',
        'rewrite_spec_links::',
    ),
    (
        'M18 system.md does not take a leading / (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 1) => &["../", "/"],\n',
        '        (true, 1) => &["../"],\n',
        'rewrite_spec_links::',
    ),
    (
        'M19 the hook does not take ./ (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        _ => &["./"],\n',
        '        _ => &[],\n',
        'rewrite_spec_links::',
    ),
    (
        'M20 a root markdown file takes ../, which leaves the project (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 0) => &["./", "/"],\n',
        '        (true, 0) => &["../", "./", "/"],\n',
        'rewrite_spec_links::',
    ),
    (
        'M21 system.md takes ./, which names the spec root (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        (true, 1) => &["../", "/"],\n',
        '        (true, 1) => &["../", "./", "/"],\n',
        'rewrite_spec_links::',
    ),
    (
        'M22 the hook takes a leading /, an absolute path (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        _ => &["./"],\n',
        '        _ => &["./", "/"],\n',
        'rewrite_spec_links::',
    ),
    (
        'M23 the shortest matching prefix is taken, not the longest (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        .map(|prefix| prefix.len())\n        .max()\n',
        '        .map(|prefix| prefix.len())\n        .min()\n',
        'rewrite_spec_links::',
    ),
    (
        'M24 a span starts at the prefix, not at the root (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '                found.push(start..start + candidate.len());\n',
        '                found.push(token..start + candidate.len());\n',
        'rewrite_spec_links::',
    ),
]
