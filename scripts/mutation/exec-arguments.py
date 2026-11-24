"""Mutation list for spec 022 task 49: `ductus exec` refuses a forwarded
argument it cannot bind, one with no `=` or with an empty key, rather than
dropping it (data model §JSON-over-stdio protocol, **Exec arguments**).

Run with `python3 scripts/mutation/run.py scripts/mutation/exec-arguments.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused; when the
code a mutation anchors on changes, update the anchor in the same change
(`run.py --check`).

The test runs the release binary, which `cargo build --release` rebuilds from
the mutated tree, so rebuild it after a run (docs/developing-ductus.md,
Proving a test fails).

The behaviours and the tests that hold them:

  M1  `exec_overrides` drops an argument with no `=`, as `run_exec` did before
      task 49. Killed by exec_subprocess's
      `exec_refuses_an_argument_it_cannot_bind`.
  M2  `exec_overrides` binds an empty key. Killed by the same test's
      `=001-basic` case.
  M3  The arguments are screened only after the command file is found and
      parsed, so a bare argument to a command with no file reads as
      command-not-found. Killed by the same test's `absent` case (022's
      review of 2026-11-17).

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 exec_overrides drops an argument with no `=`',
        'src/main.rs',
        '    args.iter()\n        .map(|arg| match arg.split_once(\'=\') {\n',
        '    args.iter()\n        .filter(|arg| arg.contains(\'=\'))\n        .map(|arg| match arg.split_once(\'=\') {\n',
        '--test exec_subprocess exec_refuses_an_argument_it_cannot_bind',
    ),
    (
        'M2 exec_overrides binds an empty key',
        'src/main.rs',
        "            Some((key, value)) if !key.is_empty() => Ok((key, value)),\n",
        "            Some((key, value)) => Ok((key, value)),\n",
        '--test exec_subprocess exec_refuses_an_argument_it_cannot_bind',
    ),
    (
        'M3 the arguments are screened after the command file is read',
        'src/main.rs',
        [
            '    let overrides = match exec_overrides(args) {\n        Ok(overrides) => overrides,\n        Err(message) => {\n            emit_exec_error("invalid-argument", &message);\n            warn(format_args!("ductus exec: {message}"));\n            return ExitCode::from(1);\n        }\n    };\n    let host = match Host::load(repo) {\n',
            '    let procedure = match parser::parse(&source, command) {\n        Ok(p) => p,\n        Err(err) => return emit_exec_parse_error(repo, path, &err),\n    };\n',
        ],
        [
            '    let host = match Host::load(repo) {\n',
            '    let procedure = match parser::parse(&source, command) {\n        Ok(p) => p,\n        Err(err) => return emit_exec_parse_error(repo, path, &err),\n    };\n    let overrides = match exec_overrides(args) {\n        Ok(overrides) => overrides,\n        Err(message) => {\n            emit_exec_error("invalid-argument", &message);\n            warn(format_args!("ductus exec: {message}"));\n            return ExitCode::from(1);\n        }\n    };\n',
        ],
        '--test exec_subprocess exec_refuses_an_argument_it_cannot_bind',
    ),
]
