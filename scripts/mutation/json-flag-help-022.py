"""Mutation list for spec 022 task 53 (scenario `every-mcp-argument-has-a-cli-form`):
each `--{name}-json` flag's `--help` states the JSON its file holds, rendered
from the schema the MCP tool publishes with every nested type's fields, and no
subcommand's help prints a developer note (a Rust path or a rustdoc link).

Run with `python3 scripts/mutation/run.py scripts/mutation/json-flag-help-022.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing tests are, in runtime/src/main.rs,
`every_json_flag_help_states_its_arguments_shape` (M1, M2) and
`no_help_prints_a_developer_note` (M3, M4).
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a json flag keeps its own help and states no shape',
        'src/main.rs',
        '                        Some(shape) => arg.long_help(format!("{own}\\n\\n{shape}")),\n',
        '                        Some(_) => arg,\n',
        '--bin ductus tests::every_json_flag_help_states_its_arguments_shape',
    ),
    (
        'M2 a nested type\'s fields are not expanded under their field',
        'src/mcp/flag_help.rs',
        '            expand(root, field, depth + 1, lines);\n',
        '',
        '--bin ductus tests::every_json_flag_help_states_its_arguments_shape',
    ),
    (
        'M3 write-session\'s help names a Rust path again',
        'src/schema/primitives.rs',
        '    /// agent choice never lands in committed config. Read back when the\n    /// runtime locates an installed command file. On a target write it is\n    /// preserved from the existing file unless supplied here.\n',
        '    /// agent choice never lands in committed config. Read back by\n    /// `crate::host::Host`. On a target write it is preserved from the\n    /// existing file unless supplied here.\n',
        '--bin ductus tests::no_help_prints_a_developer_note',
    ),
    (
        'M4 append-task\'s help names an error variant again',
        'src/schema/primitives.rs',
        '    /// a heading-not-found error rather than silently\n',
        '    /// `PrimitiveError::ParentHeadingNotFound` rather than silently\n',
        '--bin ductus tests::no_help_prints_a_developer_note',
    ),
]
