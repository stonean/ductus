"""Mutation list for spec 087: the one definition of a project file's parts
(`runtime/src/primitives/project_parts.rs`), which lists the markdown files
directly under each parts directory, reads each part's linkage from its own
project file through `check-corpus-links`' link scan
(`runtime/src/primitives/check_corpus_links.rs`, `link_file_part` and
`link_base`), and reports the five integrity findings; and
`check-artifact-size`'s project half
(`runtime/src/primitives/check_artifact_size.rs`), which measures each part on
disk and staged, reports one over the read size with its fix, and bounds the
parts, their findings and the refused list; and the staged CLI
(`runtime/src/main.rs`), which exits on a refused part and names it, through
the shipped hook (`runtime/tests/size_hook.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/project-parts-087.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a mutation
that no longer applies is removed, and the next one takes the next free number.
When the code a mutation anchors on changes, update the anchor in the same
change; `run.py --check` names every anchor that no longer matches exactly
once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a part its project file does not link is not reported',
        'src/primitives/project_parts.rs',
        '            if linked == Some(false) {',
        '            if false {',
        'project_parts::tests::a_part_its_project_file_does_not_link_is_unlinked',
    ),
    (
        'M2 a link to a part that does not exist is not reported',
        'src/primitives/project_parts.rs',
        '                && !repo.join(link).exists()',
        '                && false',
        'project_parts::tests::a_link_to_a_part_that_does_not_exist_is_a_missing_part',
    ),
    (
        'M3 a link outside every parts directory is reported as a missing part',
        'src/primitives/project_parts.rs',
        '            if KINDS.iter().any(|kind| in_dir(link, parts_dir(*kind)))',
        '            if true',
        'project_parts::tests::a_project_with_no_docs_directory_has_no_part_and_no_finding',
    ),
    (
        'M4 a subdirectory of a parts directory is not walked',
        'src/primitives/project_parts.rs',
        '            nested_files(\n                repo,\n                &subdir,\n                MAX_NESTED_DEPTH,\n                &mut nested,\n                &mut out.unlisted,\n            );\n',
        '            let _ = &subdir;\n',
        'project_parts::tests::a_markdown_file_in_a_subdirectory_of_a_parts_directory_is_nested',
    ),
    (
        'M5 a directory under .ductus/docs/ that is no parts directory is not reported',
        'src/primitives/project_parts.rs',
        '        if KINDS.iter().any(|kind| parts_dir(*kind) == dir) || dir == CONSTITUTION_PARTS_DIR {',
        '        if true {',
        'project_parts::tests::a_directory_under_docs_that_is_no_parts_directory_is_reported',
    ),
    (
        'M6 a shared constitution parts directory is reported as a stray directory',
        'src/primitives/project_parts.rs',
        ' || dir == CONSTITUTION_PARTS_DIR {',
        ' {',
        'project_parts::tests::a_shared_constitution_parts_directory_is_no_stray_directory',
    ),
    (
        'M7 a markdown file directly under .ductus/docs/ is not reported',
        'src/primitives/project_parts.rs',
        '    for stray in docs.parts {',
        '    for stray in Vec::<ListedPart>::new() {',
        'project_parts::tests::a_markdown_file_directly_under_docs_is_not_a_part',
    ),
    (
        'M8 a file in a parts directory that is not markdown is a part',
        'src/primitives/project_parts.rs',
        '        } else if is_part_name(&name) {\n            listing.parts.push(ListedPart {\n                path: rel,\n                outside: false,',
        '        } else {\n            listing.parts.push(ListedPart {\n                path: rel,\n                outside: false,',
        'project_parts::tests::a_non_markdown_file_in_a_parts_directory_is_neither_a_part_nor_a_finding',
    ),
    (
        'M9 a part that is a link leaving the project is read as inside it',
        'src/primitives/project_parts.rs',
        '        if linked && leaves_project(base, &rel) {',
        '        if false {',
        'project_parts::tests::a_part_that_is_a_link_leaving_the_project_is_unreadable',
    ),
    (
        'M10 a parts directory that is a link leaving the project is listed through it',
        'src/primitives/project_parts.rs',
        '    if leaves_project(base, dir_rel) {',
        '    if false {',
        'project_parts::tests::a_parts_directory_that_is_a_link_leaving_the_project_is_unlisted_never_empty',
    ),
    (
        'M11 a project file that is a link leaving the project is read, and its parts read as unlinked',
        'src/primitives/project_parts.rs',
        '    if leaves_project(repo, file) {\n        return None;\n    }',
        '    if false {\n        return None;\n    }',
        'project_parts::tests::a_project_file_that_cannot_be_read_leaves_its_parts_linkage_unknown',
    ),
    (
        'M12 a link from another project file links a part',
        'src/primitives/project_parts.rs',
        '                .map(|file_links| file_links.contains(&listed.path));',
        '                .map(|_| links.iter().flatten().flatten().any(|link| *link == listed.path));',
        'project_parts::tests::a_link_from_another_project_file_does_not_link_a_part',
    ),
    (
        'M13 a link inside an HTML comment links a part',
        'src/primitives/project_parts.rs',
        '        if skip.skip(line) {',
        '        if skip.skip(line) && (skip.in_fence() || line.trim_start().starts_with("```")) {',
        'project_parts::tests::a_link_inside_an_html_comment_or_a_fence_links_no_part',
    ),
    (
        'M14 a directory\'s parts are ordered by bytes, not as scenarios are',
        'src/primitives/project_parts.rs',
        '        .sort_by(|a, b| scenario_name_cmp(&a.path, &b.path));',
        '        .sort_by(|a, b| a.path.cmp(&b.path));',
        'project_parts::tests::a_directory_s_parts_are_in_scenario_order_not_byte_order',
    ),
    (
        'M15 a nested markdown file is classified as a part of the directory above it',
        'src/primitives/project_parts.rs',
        '    KINDS.into_iter().find(|kind| parts_dir(*kind) == dir)',
        '    KINDS.into_iter().find(|kind| dir.starts_with(parts_dir(*kind)))',
        'project_parts::tests::part_of_names_the_project_file_whose_directory_holds_a_markdown_file_directly',
    ),
    (
        'M16 a link\'s fragment is not stripped',
        'src/primitives/check_corpus_links.rs',
        '    let rel = target.split(\'#\').next().unwrap_or("");',
        '    let rel = target;',
        'project_parts::tests::a_link_with_a_fragment_or_rooted_at_the_project_links_its_part',
    ),
    (
        'M17 a root-absolute link resolves against the citing file\'s directory',
        'src/primitives/check_corpus_links.rs',
        "    if rel.starts_with('/') { repo } else { here }",
        '    let _ = repo;\n    here',
        'project_parts::tests::a_link_with_a_fragment_or_rooted_at_the_project_links_its_part',
    ),
    (
        'M18 a part on disk is not measured',
        'src/primitives/check_artifact_size.rs',
        '                measure(&repo.join(&part.path), &part.path, skipped)\n',
        '                None\n',
        'check_artifact_size::tests::a_part_is_listed_under_its_project_file_with_its_state',
    ),
    (
        'M19 a part is neither counted as examined nor judged against the read size',
        'src/primitives/check_artifact_size.rs',
        '        .chain(\n            parts\n                .iter()\n                .map(|part| (&part.path, part.bytes, ArtifactKind::ProjectPart)),\n        );',
        ';',
        'check_artifact_size::tests::a_part_one_byte_over_is_reported_with_its_fix_and_one_at_the_threshold_is_not',
    ),
    (
        'M20 a part over the read size is fixed by pruning the project, not by splitting it',
        'src/primitives/check_artifact_size.rs',
        '            FixKind::SplitPart,',
        '            FixKind::PruneProject,',
        'check_artifact_size::tests::a_part_one_byte_over_is_reported_with_its_fix_and_one_at_the_threshold_is_not',
    ),
    (
        'M21 a part that is a link leaving the project is measured through it',
        'src/primitives/check_artifact_size.rs',
        '            let bytes = if part.outside {',
        '            let bytes = if false {',
        'check_artifact_size::tests::a_part_linked_outside_the_project_is_unreadable_not_measured',
    ),
    (
        'M22 a directory under .ductus/docs/ that could not be listed is not skipped',
        'src/primitives/check_artifact_size.rs',
        '    for dir in found.unlisted {\n        skip(skipped, dir);\n    }\n',
        '    let _ = &found.unlisted;\n',
        'check_artifact_size::tests::a_parts_directory_that_cannot_be_listed_is_skipped_never_empty',
    ),
    (
        'M23 the integrity findings are not carried',
        'src/primitives/check_artifact_size.rs',
        '    (parts, found.findings)',
        '    (parts, Vec::new())',
        'check_artifact_size::tests::each_integrity_finding_is_listed_and_a_clean_project_lists_none',
    ),
    (
        'M24 a part record does not carry its linkage',
        'src/primitives/check_artifact_size.rs',
        '            part_size(part.path, part.project_file, bytes, part.linked)',
        '            part_size(part.path, part.project_file, bytes, None)',
        'check_artifact_size::tests::a_part_is_listed_under_its_project_file_with_its_state',
    ),
    (
        'M25 the file form keeps the parts inline',
        'src/primitives/check_artifact_size.rs',
        '            project.parts = None;\n',
        '',
        'check_artifact_size::tests::a_file_form_project_call_holds_its_parts_and_findings_in_the_file',
    ),
    (
        'M26 the lists file does not hold the parts',
        'src/primitives/check_artifact_size.rs',
        '        lists["parts"] = serde_json::json!(project.parts);\n',
        '',
        'check_artifact_size::tests::a_file_form_project_call_holds_its_parts_and_findings_in_the_file',
    ),
    (
        'M27 the lists file does not hold the part findings',
        'src/primitives/check_artifact_size.rs',
        '            lists["part-findings"] = serde_json::json!(findings);\n',
        '            let _ = findings;\n',
        'check_artifact_size::tests::a_file_form_project_call_holds_its_parts_and_findings_in_the_file',
    ),
    (
        'M28 staged, the parts are read from the disk, not the index',
        'src/primitives/check_artifact_size.rs',
        '        (staged_parts(repo, root, threshold, skipped)?, None)',
        '        (disk_parts(repo, root, skipped).0, None)',
        'check_artifact_size::tests::a_staged_part_is_measured_on_its_index_copy_against_its_head_copy',
    ),
    (
        'M29 staged, a part over the read size is refused though it shrank',
        'src/primitives/check_artifact_size.rs',
        '                Some(bytes > threshold && part.head_bytes.is_none_or(|head| bytes > head));',
        '                Some(bytes > threshold);',
        'check_artifact_size::tests::a_staged_part_is_measured_on_its_index_copy_against_its_head_copy',
    ),
    (
        "M30 staged, a part's HEAD copy is not read",
        'src/primitives/check_artifact_size.rs',
        '            part.head_bytes = head_bytes(&project, &part.path)?;\n',
        '',
        'check_artifact_size::tests::a_staged_part_is_measured_on_its_index_copy_against_its_head_copy',
    ),
    (
        'M31 a feature-only call carries a project half',
        'src/primitives/check_artifact_size.rs',
        '    let project = if args.project_files {',
        '    let project = if true {',
        'check_artifact_size::tests::a_project_with_no_parts_keeps_every_earlier_key_and_value',
    ),
    (
        'M32 a project-files call with no parts omits the findings count',
        'src/primitives/check_artifact_size.rs',
        '        part_findings_count: part_findings.as_ref().map(Vec::len),',
        '        part_findings_count: part_findings.as_ref().and(None),',
        'check_artifact_size::tests::a_project_with_no_parts_keeps_every_earlier_key_and_value',
    ),
    (
        'M33 the staged CLI does not exit on a refused part',
        'src/main.rs',
        '        || result\n            .project\n            .as_ref()\n            .is_some_and(|half| half.refused_count.unwrap_or(0) > 0)',
        '        || false',
        '--test check_artifact_size_cli a_refused_part_exits_1_and_names_its_path_size_limit_and_fix_in_either_form',
    ),
    (
        'M34 the refused list leaves out the parts',
        'src/primitives/check_artifact_size.rs',
        '            .chain(parts.iter().map(|part| (&part.path, part.refused)))\n',
        '',
        'check_artifact_size::tests::a_staged_file_form_result_holds_its_refused_list_in_the_file',
    ),
    (
        'M35 a refused part is not named on stderr',
        'src/main.rs',
        '    for part in half.parts.iter().flatten() {\n        if part.refused == Some(true) {',
        '    for part in half.parts.iter().flatten() {\n        if false && part.refused == Some(true) {',
        '--test check_artifact_size_cli a_refused_part_exits_1_and_names_its_path_size_limit_and_fix_in_either_form',
    ),
    (
        'M36 the lists file does not hold the refused list',
        'src/primitives/check_artifact_size.rs',
        '            lists["refused"] = serde_json::json!(refused);\n',
        '            let _ = refused;\n',
        'check_artifact_size::tests::a_staged_file_form_result_holds_its_refused_list_in_the_file',
    ),
    (
        'M37 the shipped hook lets a grown part through',
        'src/main.rs',
        '        || result\n            .project\n            .as_ref()\n            .is_some_and(|half| half.refused_count.unwrap_or(0) > 0)',
        '        || false',
        '--test size_hook',
    ),
    (
        'M38 an unmeasurable part is not named on stderr',
        'src/main.rs',
        '    for skipped in result.skipped.iter().flatten() {',
        '    for skipped in result.skipped.iter().flatten().filter(|s| !s.path.contains("/docs/")) {',
        '--test check_artifact_size_cli an_unmeasurable_staged_part_exits_1_and_names_it_in_either_form',
    ),
    (
        'M39 an exec /analyze does not count a part finding as advisory',
        'src/interpreter/analyze_tally.rs',
        '                self.advisory += scalar_count(result, "part-findings-count");',
        '                self.advisory += scalar_count(result, "no-such-count");',
        'interpreter::tests::an_exec_analyze_counts_an_unlinked_part_as_advisory',
    ),
]
