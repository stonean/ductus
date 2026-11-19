"""Mutation list for spec 022's review of 2026-11-19: a warning the
`ductus exec` walker writes for a stdin line or envelope it ignores never
fails the walk when stderr's reader has gone (AGENTS.md's gotcha on
`eprintln!`; `warn` in `runtime/src/io.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/stderr-warnings-022.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused; when the
code a mutation anchors on changes, update the anchor in the same change
(`run.py --check`).

The test runs the release binary, which `cargo build --release` rebuilds from
the mutated tree, so rebuild it after a run (docs/developing-ductus.md,
Proving a test fails).

The behaviours and the tests that hold them:

  M1  `warn` writes through `eprintln!` again, which panics on a broken
      stderr. Every one of the six ignore-and-continue warnings goes through
      `warn`.
  M2  `read_envelope`'s blank-line warning uses `eprintln!` again.
  M3  `read_envelope`'s unparseable-line warning uses `eprintln!` again.
  M4  `await_llm_response`'s other-request warning uses `eprintln!` again.
  M5  `await_llm_response`'s other-envelope warning uses `eprintln!` again.
  M6  `await_gate_response`'s other-request warning uses `eprintln!` again.
  M7  `await_gate_response`'s other-envelope warning uses `eprintln!` again.

  Each is killed by exec_subprocess's
  `an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone`,
  which drives all six warnings with stderr's reader closed.

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 warn panics through eprintln! on a broken stderr',
        'src/io.rs',
        '    let _ = writeln!(std::io::stderr().lock(), "{message}");\n',
        '    eprintln!("{message}");\n',
        '--test exec_subprocess an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone',
    ),
    (
        "M2 read_envelope's blank-line warning panics on a broken stderr",
        'src/io.rs',
        '            warn(format_args!(\n                "runtime: ignoring blank stdin line while awaiting an envelope"\n            ));\n',
        '            eprintln!(\n                "runtime: ignoring blank stdin line while awaiting an envelope"\n            );\n',
        '--test exec_subprocess an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone',
    ),
    (
        "M3 read_envelope's unparseable-line warning panics on a broken stderr",
        'src/io.rs',
        '                warn(format_args!(\n                    "runtime: ignoring unparseable stdin line ({err}): {trimmed:?}"\n                ));\n',
        '                eprintln!(\n                    "runtime: ignoring unparseable stdin line ({err}): {trimmed:?}"\n                );\n',
        '--test exec_subprocess an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone',
    ),
    (
        "M4 await_llm_response's other-request warning panics on a broken stderr",
        'src/interpreter/mod.rs',
        '                    warn(format_args!(\n                        "runtime: ignoring llm-response for request-id `{other}` while awaiting `{request_id}`"\n                    ));\n',
        '                    eprintln!(\n                        "runtime: ignoring llm-response for request-id `{other}` while awaiting `{request_id}`"\n                    );\n',
        '--test exec_subprocess an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone',
    ),
    (
        "M5 await_llm_response's other-envelope warning panics on a broken stderr",
        'src/interpreter/mod.rs',
        '                    warn(format_args!(\n                        "runtime: ignoring {} envelope while awaiting llm-response `{request_id}`",\n                        envelope_kind(&other)\n                    ));\n',
        '                    eprintln!(\n                        "runtime: ignoring {} envelope while awaiting llm-response `{request_id}`",\n                        envelope_kind(&other)\n                    );\n',
        '--test exec_subprocess an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone',
    ),
    (
        "M6 await_gate_response's other-request warning panics on a broken stderr",
        'src/interpreter/mod.rs',
        '                    warn(format_args!(\n                        "runtime: ignoring gate-response for request-id `{other}` while awaiting `{request_id}`"\n                    ));\n',
        '                    eprintln!(\n                        "runtime: ignoring gate-response for request-id `{other}` while awaiting `{request_id}`"\n                    );\n',
        '--test exec_subprocess an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone',
    ),
    (
        "M7 await_gate_response's other-envelope warning panics on a broken stderr",
        'src/interpreter/mod.rs',
        '                    warn(format_args!(\n                        "runtime: ignoring {} envelope while awaiting gate-response `{request_id}`",\n                        envelope_kind(&other)\n                    ));\n',
        '                    eprintln!(\n                        "runtime: ignoring {} envelope while awaiting gate-response `{request_id}`",\n                        envelope_kind(&other)\n                    );\n',
        '--test exec_subprocess an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone',
    ),
]
