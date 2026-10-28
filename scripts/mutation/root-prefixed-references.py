"""Mutation list for spec 051 task 30: a managed path behind a prefix that
names the project root from its referrer is a reference, for
`check-orphaned-references` and `rewrite-spec-links` alike —
`./` or a leading `/` in a root markdown file, `./` in `.githooks/pre-commit`,
`../` or a leading `/` in the spec root's `system.md` — and any other prefix
is not, short of a leading `/` that ends another directory's name
(`root_prefixes`, `root_prefix_before`, `managed_path_spans`,
`ends_another_directory` and `closes_a_name` in
runtime/src/primitives/check_orphaned_references.rs).

Run with `python3 scripts/mutation/run.py scripts/mutation/root-prefixed-references.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The rule lives in the shared scanner, so most mutations of it run twice:
M1-M9 and M14 filtered to the check's tests and M15-M24 to the rewrite's,
so a kill shows that each primitive's own tests hold it, as do M25/M26 (a
leading / that ends another directory's name is no reference), M34/M35 (a
leading / after markup or a shell operator is one), M37/M38 (the exception
is the leading /'s alone), M42/M43 (a prefix at a line's start counts) and
M44/M45 (a bare path at a line's start counts). The check's alone are M10
and M11 (the token-start rules read before the prefix), M12 (the check
passes its referrers no prefixes), M27-M31, M36, M39-M41, M47 and M48 (what
ends a directory's name) and M46 (an existence test read from the
repository's root); the rewrite's alone is M13. M32 and M33 tested a rule the
re-read of 3fc5755a reverted.

The killing tests are, among others, `a_path_behind_a_prefix_naming_the_project_root_is_checked`,
`a_path_behind_any_other_prefix_is_not_a_reference`,
`a_prefix_counts_wherever_the_bare_path_does`,
`a_leading_slash_is_read_from_the_project_root_below_the_repository_root` and
`the_import_and_revision_rules_apply_where_the_prefix_begins` in
check_orphaned_references.rs, and
`a_reference_behind_a_project_root_prefix_is_re_pointed_with_its_prefix`,
`a_reference_behind_any_other_prefix_is_left_as_written`,
`a_prefixed_reference_is_re_pointed_wherever_the_bare_one_is` and
`a_leading_slash_below_the_repository_root_is_re_pointed` in
rewrite_spec_links.rs.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
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
        '        .max_by_key(|prefix| prefix.len())\n',
        '        .min_by_key(|prefix| prefix.len())\n',
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
        '        .max_by_key(|prefix| prefix.len())\n',
        '        .min_by_key(|prefix| prefix.len())\n',
        'rewrite_spec_links::',
    ),
    (
        'M24 a span starts at the prefix, not at the root (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '                found.push(start..start + candidate.len());\n',
        '                found.push(token..start + candidate.len());\n',
        'rewrite_spec_links::',
    ),
    (
        'M25 a leading / that ends another directory starts a reference (check)',
        'src/primitives/check_orphaned_references.rs',
        '                || (prefix == Some("/") && ends_another_directory(&line[..token]))\n',
        '',
        'check_orphaned_references::',
    ),
    (
        'M26 a leading / that ends another directory starts a reference (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '                || (prefix == Some("/") && ends_another_directory(&line[..token]))\n',
        '',
        'rewrite_spec_links::',
    ),
    (
        'M27 a leading / after ~ starts a reference',
        'src/primitives/check_orphaned_references.rs',
        "        Some('~') => back.next() != Some('~'),\n",
        "        Some('~') => false,\n",
        'check_orphaned_references::',
    ),
    (
        'M28 a leading / after } starts a reference',
        'src/primitives/check_orphaned_references.rs',
        "        Some('}' | '%' | ')') => true,\n",
        "        Some('%' | ')') => true,\n",
        'check_orphaned_references::',
    ),
    (
        'M29 a leading / after % starts a reference',
        'src/primitives/check_orphaned_references.rs',
        "        Some('}' | '%' | ')') => true,\n",
        "        Some('}' | ')') => true,\n",
        'check_orphaned_references::',
    ),
    (
        'M30 a leading / after ) starts a reference',
        'src/primitives/check_orphaned_references.rs',
        "        Some('}' | '%' | ')') => true,\n",
        "        Some('}' | '%') => true,\n",
        'check_orphaned_references::',
    ),
    (
        'M31 a quote closing a variable before a leading / is read as opening a token',
        'src/primitives/check_orphaned_references.rs',
        '        Some(\'"\' | \'\\\'\' | \'`\') => back.next().is_some_and(closes_a_name),\n',
        '        Some(\'"\' | \'\\\'\' | \'`\') => false,\n',
        'check_orphaned_references::',
    ),
    (
        'M34 a leading / after markup or a shell operator is refused (check)',
        'src/primitives/check_orphaned_references.rs',
        '        _ => false,\n',
        '        _ => true,\n',
        'check_orphaned_references::',
    ),
    (
        'M35 a leading / after markup or a shell operator is refused (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '        _ => false,\n',
        '        _ => true,\n',
        'rewrite_spec_links::',
    ),
    (
        'M36 a leading / after strikethrough ~~ is refused',
        'src/primitives/check_orphaned_references.rs',
        "        Some('~') => back.next() != Some('~'),\n",
        "        Some('~') => true,\n",
        'check_orphaned_references::',
    ),
    (
        'M37 the directory-name exception applies to ./ and ../ too (check)',
        'src/primitives/check_orphaned_references.rs',
        'prefix == Some("/") && ends_another_directory',
        'prefix.is_some() && ends_another_directory',
        'check_orphaned_references::',
    ),
    (
        'M38 the directory-name exception applies to ./ and ../ too (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        'prefix == Some("/") && ends_another_directory',
        'prefix.is_some() && ends_another_directory',
        'rewrite_spec_links::',
    ),
    (
        'M39 any character before a closing quote ends a directory name',
        'src/primitives/check_orphaned_references.rs',
        "    c.is_ascii_alphanumeric() || matches!(c, '}' | ')' | '%')\n",
        '    true\n',
        'check_orphaned_references::',
    ),
    (
        'M40 a leading / at the start of a line is refused',
        'src/primitives/check_orphaned_references.rs',
        '        _ => false,\n',
        '        None => true,\n        _ => false,\n',
        'check_orphaned_references::',
    ),
    (
        'M41 a quote at the start of a line closes a directory name',
        'src/primitives/check_orphaned_references.rs',
        '=> back.next().is_some_and(closes_a_name),',
        '=> back.next().is_none_or(closes_a_name),',
        'check_orphaned_references::',
    ),
    (
        'M42 a prefix at the start of a line is refused (check)',
        'src/primitives/check_orphaned_references.rs',
        '            let not_a_reference = previous.is_some_and(is_path_char)\n',
        '            let not_a_reference = previous.is_none_or(is_path_char)\n',
        'check_orphaned_references::',
    ),
    (
        'M43 a prefix at the start of a line is refused (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '            let not_a_reference = previous.is_some_and(is_path_char)\n',
        '            let not_a_reference = previous.is_none_or(is_path_char)\n',
        'rewrite_spec_links::',
    ),
    (
        'M44 a bare path at the start of a line is refused (check)',
        'src/primitives/check_orphaned_references.rs',
        '            let not_a_reference = previous.is_some_and(is_path_char)\n',
        '            let not_a_reference = (if prefix.is_none() { previous.is_none_or(is_path_char) } else { previous.is_some_and(is_path_char) })\n',
        'check_orphaned_references::',
    ),
    (
        'M45 a bare path at the start of a line is refused (rewrite)',
        'src/primitives/check_orphaned_references.rs',
        '            let not_a_reference = previous.is_some_and(is_path_char)\n',
        '            let not_a_reference = (if prefix.is_none() { previous.is_none_or(is_path_char) } else { previous.is_some_and(is_path_char) })\n',
        'rewrite_spec_links::',
    ),
    (
        "M46 a managed path is tested for existence from the repository's root",
        'src/primitives/check_orphaned_references.rs',
        "                if repo.join(target.trim_end_matches('/')).exists() {\n",
        "                if super::ProjectRepository::discover(repo).ok().and_then(|p| p.repository.workdir().map(|w| w.join(target.trim_end_matches('/')))).unwrap_or_else(|| repo.join(target.trim_end_matches('/'))).exists() {\n",
        'check_orphaned_references::',
    ),
    (
        'M47 a quote after % does not close a directory name',
        'src/primitives/check_orphaned_references.rs',
        "    c.is_ascii_alphanumeric() || matches!(c, '}' | ')' | '%')\n",
        "    c.is_ascii_alphanumeric() || matches!(c, '}' | ')')\n",
        'check_orphaned_references::',
    ),
    (
        'M48 a quote after a digit does not close a directory name',
        'src/primitives/check_orphaned_references.rs',
        "    c.is_ascii_alphanumeric() || matches!(c, '}' | ')' | '%')\n",
        "    c.is_ascii_alphabetic() || matches!(c, '}' | ')' | '%')\n",
        'check_orphaned_references::',
    ),
]
