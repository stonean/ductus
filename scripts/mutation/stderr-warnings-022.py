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
the mutated tree, so rebuild it after a run (.ductus/docs/agents/developing-ductus.md,
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
  M8  `run_exec`'s command-not-found line uses `eprintln!` again.
  M9  `report_broken_links`' first line uses `eprintln!` again.
  M10 `report_cycles`' first line uses `eprintln!` again.
  M11 `read_envelope` decodes a line that is not UTF-8 lossily.
  M12 `read_envelope` fails the walk on a line that is not UTF-8.

  M1 to M7 are killed by exec_subprocess's
  `an_ignored_stdin_line_warns_without_failing_the_walk_when_stderr_is_gone`,
  which drives the six ignore-and-continue warnings with stderr's reader
  closed; M8 to M10 by exec_stderr_gone's `exec_keeps_its_exit_and_error_envelope`,
  `check_corpus_links_keeps_its_exit_and_result` and
  `derive_dependencies_keeps_its_exit_and_result`; M11 and M12 by the `io::tests`
  named in their entries.

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
    (
        "M8 run_exec's command-not-found line panics on a broken stderr",
        'src/main.rs',
        '            emit_exec_error(code, &message);\n            warn(format_args!("ductus exec: {message}"));\n',
        '            emit_exec_error(code, &message);\n            eprintln!("ductus exec: {message}");\n',
        '--test exec_stderr_gone exec_keeps_its_exit_and_error_envelope',
    ),
    (
        "M9 report_broken_links' first line panics on a broken stderr",
        'src/main.rs',
        '        warn(format_args!(\n            "broken link: {}:{} -> `{}` — {}",\n            link.path, link.line, link.target, link.guidance\n        ));\n',
        '        eprintln!(\n            "broken link: {}:{} -> `{}` — {}",\n            link.path, link.line, link.target, link.guidance\n        );\n',
        '--test exec_stderr_gone check_corpus_links_keeps_its_exit_and_result',
    ),
    (
        "M10 report_cycles' first line panics on a broken stderr",
        'src/main.rs',
        '            warn(format_args!("cycle: {} -> {first}", cycle.join(" -> ")));\n',
        '            eprintln!("cycle: {} -> {first}", cycle.join(" -> "));\n',
        '--test exec_stderr_gone derive_dependencies_keeps_its_exit_and_result',
    ),
    (
        'M11 read_envelope decodes a non-UTF-8 line lossily',
        'src/io.rs',
        'let Ok(line) = String::from_utf8(raw) else {',
        'let Ok(line) = Ok::<String, ()>(String::from_utf8_lossy(&raw).into_owned()) else {',
        'io::tests::read_envelope_skips_a_line_that_is_not_utf8_until_valid',
    ),
    (
        'M12 read_envelope fails the walk on a non-UTF-8 line',
        'src/io.rs',
        'continue;\n        };\n        let trimmed = line.trim();',
        'return Err(std::io::ErrorKind::InvalidData.into());\n        };\n        let trimmed = line.trim();',
        'io::tests::read_envelope_skips_a_line_that_is_not_utf8_until_valid io::tests::read_envelope_surfaces_eof_after_only_non_utf8_lines',
    ),
]
