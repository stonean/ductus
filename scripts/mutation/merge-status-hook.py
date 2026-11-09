"""Mutation list for 080's `merge-status-hook` (task 7; AC2, AC20): the hook
entries are written only on Claude Code 2.1.101 or later, a skip writes
nothing, each of ductus's entries, status's and help's, is recognized by its
command and added beside every other entry, and the version probe reads an exit status and a missing `claude`
(`runtime/src/primitives/merge_status_hook.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/merge-status-hook.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. The integration test the probe's PATH lookup
also answers to, `no_claude_on_the_path_is_skipped_as_unreadable` in
`runtime/tests/merge_status_hook.rs`, fails under M6, run by hand with
`cargo test --locked --test merge_status_hook`. M5, M6 and M7's killing tests
are Unix-only, since they run a stand-in script or a read-only directory.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the bound exclusive: 2.1.101 itself is skipped',
        'src/primitives/merge_status_hook.rs',
        '    if parsed < MIN_CLAUDE_VERSION {\n',
        '    if parsed <= MIN_CLAUDE_VERSION {\n',
        'merge_status_hook::tests::the_version_gate_compares_numerically_and_admits_its_bound',
    ),
    (
        'M2 a skip writes the settings file',
        'src/primitives/merge_status_hook.rs',
        '        Gate::Skip { version, reason } => {\n',
        '        Gate::Skip { version, reason } => {\n            write_atomic(&target_path, "{}")?;\n',
        'merge_status_hook::tests::an_older_version_is_skipped_and_writes_nothing',
    ),
    (
        'M3 an entry recognized by its matcher, not its command',
        'src/primitives/merge_status_hook.rs',
        '                .is_some_and(|inner| inner.iter().any(|hook| hook["command"] == command))\n',
        '                .is_some_and(|_inner| group["matcher"] == hook.0.as_str())\n',
        'merge_status_hook::tests::each_entry_is_recognized_by_its_command_not_its_matcher',
    ),
    (
        'M4 an entry replaces the other UserPromptExpansion entries',
        'src/primitives/merge_status_hook.rs',
        '            groups.push(entry(hook));\n',
        '            *groups = vec![entry(hook)];\n',
        'merge_status_hook::tests::an_existing_file_is_updated_leaving_every_other_key_and_entry',
    ),
    (
        'M5 the probe ignores the exit status',
        'src/primitives/merge_status_hook.rs',
        '    if !output.status.success() {\n',
        '    if false {\n',
        'merge_status_hook::tests::the_probe_reads_a_version_and_names_each_failure',
    ),
    (
        'M6 a missing claude is reported as a spawn failure, and no later program is tried',
        'src/primitives/merge_status_hook.rs',
        '        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return ProbeRun::NotFound,\n',
        '',
        'merge_status_hook::tests::the_probe_reads_a_version_and_names_each_failure',
    ),
    (
        'M7 an unchanged run rewrites the file',
        'src/primitives/merge_status_hook.rs',
        '    if action != "unchanged" {\n',
        '    if true {\n',
        'merge_status_hook::tests::an_unchanged_run_writes_nothing',
    ),
    (
        'M8 a file holding status\'s entry gains no help entry',
        'src/primitives/merge_status_hook.rs',
        '    for hook in wanted {\n',
        '    for hook in wanted.iter().take(1) {\n',
        'merge_status_hook::tests::a_file_holding_only_status_s_entry_gains_help_s',
    ),
    (
        'M9 a created file holds status\'s entry alone',
        'src/primitives/merge_status_hook.rs',
        '        let entries: Vec<Value> = wanted.iter().map(entry).collect();\n',
        '        let entries: Vec<Value> = wanted.iter().take(1).map(entry).collect();\n',
        'merge_status_hook::tests::no_file_is_created_holding_both_entries',
    ),
]
