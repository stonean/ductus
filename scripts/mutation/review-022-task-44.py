"""Mutation list for spec 022 task 44 (the review round of 2026-11-17): the
tests that named a write's atomicity now drive a failing write through the
primitive (F12, F13 and the class), `ductus exec`'s command-not-found result is
asserted (F22), a project root that cannot be resolved is not labelled an
outward command file (O6), and `append-inbox` refuses marker-only text (O37).

Run with `python3 scripts/mutation/run.py scripts/mutation/review-022-task-44.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused; when the
code a mutation anchors on changes, update the anchor in the same change
(`run.py --check`).

The behaviours and the tests that hold them:

  M1  `write_atomic_bytes` writes the destination in place before it creates
      its tempfile. Killed by `a_failed_write_leaves_the_*_unchanged_and_no_tempfile`
      in mark_criterion, set_status, mark_task and append_task.
  M2  `write_atomic_bytes` swallows a tempfile that cannot be created and
      reports success. Killed by the same four tests and by write_review's
      `a_failed_write_leaves_no_review_md_and_no_tempfile`.
  M3  `write_new_atomic` does the same. Killed by create_scenario's
      `a_failed_write_leaves_no_destination_and_no_tempfile`.
  M4  `locate_exec_command` labels a project root that cannot be resolved as a
      command file outside the project. Killed by main.rs's
      `an_unresolvable_project_root_is_not_reported_as_an_outward_command_file`.
  M5  `ductus exec`'s command-not-found code changed. Killed by
      exec_subprocess's `exec_returns_nonzero_when_command_file_missing`.
  M6  The same test holds the tried-candidates message.
  M7  `append-inbox` does not re-check its text after stripping the marker.
      Killed by append_inbox's `rejects_text_that_is_only_a_marker`.
  M8  `append-inbox` writes text whose bullet opens an unclosed HTML comment.
      Killed by append_inbox's `text_with_an_unclosed_comment_open_is_refused`.

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 write_atomic_bytes writes the destination before its tempfile',
        'src/primitives/mod.rs',
        '    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(|source| PrimitiveError::Io {\n        path: parent.into(),\n        source,\n    })?;\n    tmp.as_file_mut()\n        .write_all(content)\n',
        '    std::fs::write(path, content).map_err(|source| PrimitiveError::Io {\n        path: path.into(),\n        source,\n    })?;\n    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(|source| PrimitiveError::Io {\n        path: parent.into(),\n        source,\n    })?;\n    tmp.as_file_mut()\n        .write_all(content)\n',
        'a_failed_write',
    ),
    (
        'M2 write_atomic_bytes swallows a tempfile that cannot be created',
        'src/primitives/mod.rs',
        '    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(|source| PrimitiveError::Io {\n        path: parent.into(),\n        source,\n    })?;\n    tmp.as_file_mut()\n        .write_all(content)\n',
        '    let Ok(mut tmp) = tempfile::NamedTempFile::new_in(parent) else {\n        return Ok(());\n    };\n    tmp.as_file_mut()\n        .write_all(content)\n',
        'a_failed_write',
    ),
    (
        'M3 write_new_atomic swallows a tempfile that cannot be created',
        'src/primitives/mod.rs',
        '    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(|source| PrimitiveError::Io {\n        path: parent.into(),\n        source,\n    })?;\n    tmp.as_file_mut()\n        .write_all(content.as_bytes())\n',
        '    let Ok(mut tmp) = tempfile::NamedTempFile::new_in(parent) else {\n        return Ok(true);\n    };\n    tmp.as_file_mut()\n        .write_all(content.as_bytes())\n',
        'a_failed_write',
    ),
    (
        'M4 locate_exec_command labels an unresolvable project root as an outward command file',
        'src/main.rs',
        '            Err(err) => return Err(("config-unreadable", err.to_string())),\n',
        '            Err(err) => return Err(("command-file-outside-project", err.to_string())),\n',
        '--bin ductus an_unresolvable_project_root_is_not_reported_as_an_outward_command_file',
    ),
    (
        'M5 ductus exec reports a missing command under another code',
        'src/main.rs',
        '        "command-not-found",\n',
        '        "command-missing",\n',
        '--test exec_subprocess exec_returns_nonzero_when_command_file_missing',
    ),
    (
        'M6 ductus exec drops the tried candidates from its not-found message',
        'src/main.rs',
        '        format!("command file not found (tried {tried})"),\n',
        '        format!("command file not found ({})", tried.len()),\n',
        '--test exec_subprocess exec_returns_nonzero_when_command_file_missing',
    ),
    (
        'M7 append-inbox writes text that is only a marker',
        'src/primitives/append_inbox.rs',
        '    super::validate_inbox_text("append-inbox", "text", &text)?;\n',
        '',
        'rejects_text_that_is_only_a_marker',
    ),
    (
        'M8 append-inbox writes text that opens an unclosed comment',
        'src/primitives/append_inbox.rs',
        '    if SkipScanner::default().skip(&format!("- [ ] {text}")) {\n',
        '    if false {\n',
        'text_with_an_unclosed_comment_open_is_refused',
    ),
]
