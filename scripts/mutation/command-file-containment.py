"""Mutation list for spec 022 task 40 (BE-INPUT-004, from 064's analysis):
`Host::load` refuses a `[host] project` or `cli-config-dir` that is not one
path component, naming the file it came from, and each reader of
`Host::command_file_candidates` contains a candidate before it probes or reads
it, so a symbolic link leaving the project is never followed.

Run with `python3 scripts/mutation/run.py scripts/mutation/command-file-containment.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing tests are, in runtime/src/host.rs,
`a_value_that_is_not_one_path_component_halts_naming_its_file` (M1-M5) and
`contain_candidate_refuses_a_link_leaving_the_project` (M6); in
runtime/tests/exec_subprocess.rs
`exec_refuses_a_command_file_linked_outside_the_project` (M7); in
runtime/src/interpreter/payload.rs
`a_command_file_linked_outside_the_project_is_not_read` (M8); in
runtime/src/primitives/render_help.rs
`a_copy_linked_outside_the_project_is_refused_unread` (M9); and in
runtime/src/primitives/tool_loading.rs
`a_copy_linked_outside_the_project_is_reported_unread` (M10). The link tests
are Unix only, so on Windows M6-M10 survive. M7's test rebuilds the release
binary from the mutated tree, so rebuild it after a run.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a [host] project is joined into the path unscreened',
        'src/host.rs',
        '            Some(project) => screen_component(project, "project", &config_path)?,\n',
        '            Some(project) => project,\n',
        'host::tests::a_value_that_is_not_one_path_component_halts_naming_its_file',
    ),
    (
        'M2 the session file\'s cli-config-dir is joined into the path unscreened',
        'src/host.rs',
        '            Some(dir) => screen_component(dir, "cli-config-dir", &paths::session_path(repo))?,\n',
        '            Some(dir) => dir,\n',
        'host::tests::a_value_that_is_not_one_path_component_halts_naming_its_file',
    ),
    (
        'M3 a [host] cli-config-dir is joined into the path unscreened',
        'src/host.rs',
        '                Some(dir) => screen_component(dir, "cli-config-dir", &config_path)?,\n',
        '                Some(dir) => dir,\n',
        'host::tests::a_value_that_is_not_one_path_component_halts_naming_its_file',
    ),
    (
        'M4 the screen lets `..` through',
        'src/host.rs',
        'value == "." || value == ".." || value.contains',
        'value == "." || value.contains',
        'host::tests::a_value_that_is_not_one_path_component_halts_naming_its_file',
    ),
    (
        'M5 the screen lets a backslash through',
        'src/host.rs',
        "value.contains(['/', '\\\\', '\\0'])",
        "value.contains(['/', '\\0'])",
        'host::tests::a_value_that_is_not_one_path_component_halts_naming_its_file',
    ),
    (
        'M6 contain_candidate accepts a link leaving the project',
        'src/host.rs',
        '        crate::primitives::validate_contained(repo, rel)\n',
        '        let _ = (repo, rel);\n        Ok(())\n',
        'host::tests::contain_candidate_refuses_a_link_leaving_the_project',
    ),
    (
        'M7 ductus exec probes and reads a candidate it never contained',
        'src/main.rs',
        '        match ductus::host::Host::contain_candidate(repo, rel) {\n',
        '        match Ok::<(), ductus::primitives::PrimitiveError>(()) {\n',
        '--test exec_subprocess exec_refuses_a_command_file_linked_outside_the_project',
    ),
    (
        'M8 the writeCode excerpt load reads a candidate it never contained',
        'src/interpreter/payload.rs',
        '        match Host::contain_candidate(repo, &rel) {\n',
        '        match Ok::<(), crate::primitives::PrimitiveError>(()) {\n',
        'interpreter::payload::tests::a_command_file_linked_outside_the_project_is_not_read',
    ),
    (
        'M9 render-help reads a candidate it never contained',
        'src/primitives/render_help.rs',
        '        Host::contain_candidate(repo, path)?;\n',
        '',
        'render_help::tests::a_copy_linked_outside_the_project_is_refused_unread',
    ),
    (
        'M10 the always-loaded copy is read without being contained',
        'src/primitives/tool_loading.rs',
        '            if let Err(err) = Host::contain_candidate(repo, path) {\n',
        '            if let Err(err) = Ok::<(), String>(()) {\n',
        'tool_loading::tests::a_copy_linked_outside_the_project_is_reported_unread',
    ),
]
