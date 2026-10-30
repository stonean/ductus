"""Mutation list for spec 048's 2026-10-29 analysis: what `fetch-archive` takes
from the environment is captured and parsed at startup, and the CLI reads a
`--*-json` file only from a staging directory
(`runtime/src/primitives/fetch_archive.rs`, `runtime/src/primitives/mod.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/fetch-archive-startup-env.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused; when the
code a mutation anchors on changes, update the anchor in the same change
(`run.py --check`).

  M1  a proxy value that does not parse refuses the fetch (CFG-ENV-008), and
      is not skipped so that scheme's fetches go direct.
  M2  the refusal names the value with its credentials redacted (CFG-ENV-006).
  M3  a proxy URL naming a scheme other than http or https is refused at
      startup, not left to fail at connect naming the scheme.
  M4  the certificate roots load from the paths captured at startup, not from
      a loader that reads the environment itself (CFG-ENV-001).
  M5  the capture splits SSL_CERT_DIR and drops empty entries.
  M6  a `--*-json` path outside a staging directory is refused (BE-INPUT-004).

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a proxy value that does not parse is skipped, not refused',
        'src/primitives/fetch_archive.rs',
        '    let parsed = proxies.parsed.as_deref().map_err(|(variable, reason)| {\n        PrimitiveError::InvalidEnvironmentVariable {\n            variable: (*variable).to_string(),\n            reason: reason.clone(),\n        }\n    })?;\n',
        '    let parsed: &[reqwest::Proxy] = proxies.parsed.as_deref().unwrap_or(&[]);\n',
        'a_proxy_value_that_does_not_parse_refuses_every_fetch',
    ),
    (
        'M2 the refused proxy value is shown with its credentials',
        'src/primitives/fetch_archive.rs',
        '                                redact_proxy(value)\n',
        '                                flatten_line(value)\n',
        'a_proxy_value_that_does_not_parse_refuses_every_fetch',
    ),
    (
        'M3 a proxy scheme this build cannot speak is accepted at startup',
        'src/primitives/fetch_archive.rs',
        '                    _ if !speakable => Err(()),\n',
        '                    _ if !speakable && false => Err(()),\n',
        'a_proxy_value_that_does_not_parse_refuses_every_fetch',
    ),
    (
        'M4 the captured certificate paths are ignored',
        'src/primitives/fetch_archive.rs',
        '        Some(env) if env.is_set() => {\n',
        '        Some(env) if env.is_set() && false => {\n',
        'captured_cert_paths_are_read_instead_of_the_platform_store',
    ),
    (
        'M5 the captured SSL_CERT_DIR keeps its empty entries',
        'src/primitives/fetch_archive.rs',
        '                    .filter(|dir| !dir.as_os_str().is_empty())\n',
        '                    .filter(|_| true)\n',
        'the_cert_capture_splits_the_directory_list',
    ),
    (
        'M6 a --*-json path is read from anywhere',
        'src/primitives/mod.rs',
        '    let resolved = validate_in_temp_dir(path).map_err(|err| format!("--{flag}: {err}"))?;\n',
        '    let resolved = std::path::PathBuf::from(path);\n',
        'a_cli_json_file_is_read_only_from_a_staging_directory',
    ),
]
