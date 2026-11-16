"""Mutation list for spec 041's `prune-walks-the-project-files`: the
`prune-project` primitive (`runtime/src/primitives/prune_project.rs`) that
segments the four project files, runs the literal search for catalog
identifiers, and applies digest-claimed removals, replacements and creations.

Run with `python3 scripts/mutation/run.py scripts/mutation/prune-project.py
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
        'M1 a managed region is segmented like any other text',
        'src/primitives/prune_project.rs',
        '        if !skipped && let Some(marker) = begin_marker(trimmed) {',
        '        if false && !skipped && let Some(marker) = begin_marker(trimmed) {',
        'prune_project::tests::a_managed_region_and_a_comment_are_never_entries',
    ),
    (
        'M2 an HTML comment becomes an entry',
        'src/primitives/prune_project.rs',
        '        if (skipped && !fence) || whole_comment {',
        '        if false && ((skipped && !fence) || whole_comment) {',
        'prune_project::tests::a_managed_region_and_a_comment_are_never_entries',
    ),
    (
        'M3 a blank line ends a heading entry',
        'src/primitives/prune_project.rs',
        '            Some((Open::Heading(_), _)) | None => {}',
        '            None => {}',
        'prune_project::tests::a_fenced_block_stays_inside_its_heading_entry',
    ),
    (
        'M4 a catalog heading without a code span names no identifier',
        'src/primitives/prune_project.rs',
        '            .map(|(_, text)| span(&text).unwrap_or_else(|| text.trim().to_string())),',
        '            .and_then(|(_, text)| span(&text)),',
        'prune_project::tests::catalog_entries_carry_their_identifier',
    ),
    (
        'M5 the search reads the spec root',
        'src/primitives/prune_project.rs',
        '        .filter(|path| !path.starts_with(&prefix))\n',
        '',
        'prune_project::tests::the_literal_search_reports_each_tracked_hit_outside_the_spec_root',
    ),
    (
        'M6 the search reads the subjects themselves',
        'src/primitives/prune_project.rs',
        '        .filter(|path| subjects.iter().all(|(_, rel)| rel != path))\n',
        '',
        'prune_project::tests::the_literal_search_reports_each_tracked_hit_outside_the_spec_root',
    ),
    (
        'M7 a search with no repository reads as zero hits',
        'src/primitives/prune_project.rs',
        '    let Some(tracked) = tracked_project_paths(repo)? else {\n        return Ok(ProjectSearch {',
        '    let tracked = tracked_project_paths(repo)?.unwrap_or_default();\n    if false {\n        return Ok(ProjectSearch {',
        'prune_project::tests::with_no_repository_the_search_is_unavailable_never_zero_hits',
    ),
    (
        'M8 a file exactly at the limit is over it',
        'src/primitives/prune_project.rs',
        '        over_limit: bytes.map(|bytes| bytes > limit),',
        '        over_limit: bytes.map(|bytes| bytes >= limit),',
        'prune_project::tests::each_file_reports_its_state_and_the_limit_is_a_maximum',
    ),
    (
        'M9 a stale listing still lets the rest of the apply write',
        'src/primitives/prune_project.rs',
        '    let applied = args.apply && stale_entries.is_empty();',
        '    let applied = args.apply;',
        'prune_project::tests::a_stale_listing_refuses_the_whole_apply',
    ),
    (
        'M10 a removal leaves two blank lines at its seam',
        'src/primitives/prune_project.rs',
        '            if at > 0 && at < lines.len() && blank(&lines[at - 1]) && blank(&lines[at]) {',
        '            if false && at > 0 && at < lines.len() && blank(&lines[at - 1]) && blank(&lines[at]) {',
        'prune_project::tests::an_apply_removes_an_entry_and_collapses_its_seam',
    ),
    (
        'M11 a rewrite drops the file to LF endings',
        'src/primitives/prune_project.rs',
        '    let ending = line_ending_of(content);',
        '    let ending = { let _ = content; "\\n" };',
        'prune_project::tests::an_apply_keeps_crlf_line_endings',
    ),
    (
        'M12 a created home keeps the tempfile mode',
        'src/primitives/prune_project.rs',
        '        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o644)).map_err(',
        '        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).map_err(',
        'prune_project::tests::an_apply_creates_a_missing_home_and_keeps_a_present_one',
    ),
    (
        'M13 a create overwrites a home that is already there',
        'src/primitives/prune_project.rs',
        '    if !write_new_atomic(path, body)? {\n',
        '    if !crate::primitives::write_atomic(path, body).map(|()| true)? {\n',
        'prune_project::tests::an_apply_creates_a_missing_home_and_keeps_a_present_one',
    ),
    (
        'M14 the result is never bounded to the read size',
        'src/primitives/prune_project.rs',
        '    if let Some(file) = bounded_lists::place(repo, PRIMITIVE, PROJECT_FILES_KEY, &result, &lists)? {',
        '    if let Some(file) = None::<String> {',
        'prune_project::tests::entries_are_inline_while_the_result_fits_and_in_a_file_past_it prune_project::tests::an_apply_bounds_its_entries_too',
    ),
    (
        'M15 the file form carries no entry count',
        'src/primitives/prune_project.rs',
        '        entries_count: entries.len(),',
        '        entries_count: 0,',
        'prune_project::tests::entries_are_inline_while_the_result_fits_and_in_a_file_past_it',
    ),
    (
        'M16 the file form keeps the entries inline as well',
        'src/primitives/prune_project.rs',
        '        result.entries = None;\n',
        '',
        'prune_project::tests::entries_are_inline_while_the_result_fits_and_in_a_file_past_it',
    ),
    (
        'M17 an unterminated comment drops the rest of the file unreported',
        'src/primitives/prune_project.rs',
        '        (None, Some(from)) => Some((from, TailCause::OpenComment)),',
        '        (None, Some(_)) => None,',
        'prune_project::tests::an_unterminated_comment_names_where_the_unexamined_tail_begins',
    ),
    (
        'M18 an unterminated managed region drops the rest of the file unreported',
        'src/primitives/prune_project.rs',
        '        (Some((_, from)), _) => Some((*from, TailCause::UnterminatedManagedRegion)),',
        '        (Some(_), _) => None,',
        'prune_project::tests::an_unterminated_managed_region_names_where_the_unexamined_tail_begins',
    ),
    (
        'M19 an open comment is reported at its last line, not where it opened',
        'src/primitives/prune_project.rs',
        '            (true, true) => comment_from,',
        '            (true, true) => Some(i),',
        'prune_project::tests::an_unterminated_comment_names_where_the_unexamined_tail_begins',
    ),
    (
        'M20 a non-UTF-8 tracked file is not searched',
        'src/primitives/prune_project.rs',
        '        None => hay\n            .windows(needle.len().max(1))\n            .any(|window| window == needle.as_bytes()),',
        '        None => false,',
        'prune_project::tests::a_non_utf8_tracked_file_is_searched_bytewise',
    ),
    (
        'M21 the file record does not carry the tail',
        'src/primitives/prune_project.rs',
        '            file.unexamined_tail = tail;',
        '            let _ = tail;',
        'prune_project::tests::a_file_with_an_open_region_reads_present_and_carries_its_tail',
    ),
]
