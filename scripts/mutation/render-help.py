"""Mutation list for 080's `render-help` (task 34; scenario
`help-answers-with-no-model-turn`): the guide is the text between the first
and last `---` lines after the help copy's `## Instructions` heading, never the
frontmatter's, with its whole-line HTML comments removed and no blank line
doubled by a removal, and no installed copy is an error, never an empty guide
(`runtime/src/primitives/render_help.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/render-help.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. The hook that shows the guide is held by
`help_is_shown_as_a_block_holding_the_installed_guide` and
`help_with_no_installed_copy_exits_1_with_the_error_on_stderr_alone` in
`runtime/tests/hook_status.rs`, run by hand with
`cargo test --locked --test hook_status`.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the generator markers stay in the guide',
        'src/primitives/render_help.rs',
        '        if trimmed.starts_with("<!--") && trimmed.ends_with("-->") {\n',
        '        if false {\n',
        'render_help::tests::the_guide_is_the_text_between_the_rules_without_its_comment_lines',
    ),
    (
        'M2 a removed marker leaves its blank line doubled',
        'src/primitives/render_help.rs',
        '        if skip_blank && trimmed.is_empty() {\n',
        '        if false {\n',
        'render_help::tests::the_guide_is_the_text_between_the_rules_without_its_comment_lines',
    ),
    (
        'M3 the rules are counted from the top, the frontmatter\'s among them',
        'src/primitives/render_help.rs',
        '    let rules: Vec<usize> = (heading + 1..lines.len())\n',
        '    let rules: Vec<usize> = (0..lines.len())\n',
        'render_help::tests::a_copy_without_two_rules_after_the_heading_holds_no_guide',
    ),
    (
        'M4 a missing copy reads as an empty guide',
        'src/primitives/render_help.rs',
        '            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,\n',
        '            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(String::new()),\n',
        'render_help::tests::no_installed_copy_is_an_error_naming_the_paths_tried',
    ),
]
