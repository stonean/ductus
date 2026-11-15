"""Mutation list for spec 067 task 33: `check-corpus-links` never probes a link
target, and never reads or lists a file, that leaves the repository's work tree
(the project root when there is no repository) lexically or through a
symbolic link (`BE-INPUT-004`). Each mutation removes one guard: the lexical
screen, the shared link check, the file and directory refusals, the choice of
the work tree's root as the base, and the screen on the one-directory-up hint.

Run with `python3 scripts/mutation/run.py scripts/mutation/corpus-link-containment.py
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
        'M1 a target that leaves the base lexically is probed',
        'src/primitives/check_corpus_links.rs',
        '        let Ok(inside) = path.strip_prefix(&self.root) else {\n            return Reach::Leaves;\n        };',
        '        let inside = path.strip_prefix(&self.root).unwrap_or(path);',
        'check_corpus_links::tests::a_target_that_leaves_the_project_lexically_is_broken_and_not_probed',
    ),
    (
        'M2 a target through a link out of the base is followed',
        'src/primitives/check_corpus_links.rs',
        '            Err(super::PrimitiveError::InvalidPath { reason, .. }) => Reach::ThroughLink(reason),\n            Err(other) => Reach::ThroughLink(other.to_string()),',
        '            Err(_) => Reach::Inside,',
        'check_corpus_links::tests::a_target_through_a_link_out_of_the_project_is_broken_and_not_followed',
    ),
    (
        'M3 a markdown file that is a link out of the base is read',
        'src/primitives/check_corpus_links.rs',
        '        if containment.leaves(&path) {\n            result.skipped.push(CorpusLinkSkip {',
        '        if false && containment.leaves(&path) {\n            result.skipped.push(CorpusLinkSkip {',
        'check_corpus_links::tests::a_markdown_file_that_is_a_link_out_of_the_project_is_skipped_and_not_read',
    ),
    (
        'M4 a directory that is a link out of the base is listed',
        'src/primitives/check_corpus_links.rs',
        '        if linked && containment.leaves(&path) {\n            if path.extension()',
        '        if false && linked && containment.leaves(&path) {\n            if path.extension()',
        'check_corpus_links::tests::a_directory_that_is_a_link_out_of_the_project_is_skipped_and_not_listed',
    ),
    (
        'M5 the base is the project root, not the work tree of its repository',
        'src/primitives/check_corpus_links.rs',
        '            Ok(repository) => (repository.work_tree_root(repo), "repository"),',
        '            Ok(_) => (repo.to_path_buf(), "project"),',
        'check_corpus_links::tests::a_project_in_a_repository_subdirectory_resolves_a_link_into_its_repository',
    ),
    (
        'M6 the one-directory-up hint probes outside the base',
        'src/primitives/check_corpus_links.rs',
        '        && matches!(containment.judge(&deeper), Reach::Inside)\n',
        '',
        'check_corpus_links::tests::the_one_directory_up_hint_is_not_probed_outside_the_base',
    ),
]
