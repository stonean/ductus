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
  M7  the certificate variables are captured at startup where the verifier
      reads them.
  M8  a redacted proxy value is one line.
  M9  `://` ends a scheme only after scheme characters, so a value with `://`
      in its credentials is redacted whole.
  M10 a host NO_PROXY names is reached directly (the exemption is applied).
  M11 a refused `--*-json` path is shown on one line.
  M12 a `--*-json` file that is not valid JSON is named on one line.

No mutation is an expected survivor. Deleting the certificate half of
`init_proxy_env` is not listed: this runner runs library tests only, and
`runtime/tests/fetch_archive_cert_env_startup.rs`, an integration test (Linux
only, proven red in a Linux container), kills it. Nor is dropping `main`'s `init_proxy_env` call: this runner runs library
tests only, and `runtime/tests/fetch_archive_startup_env.rs` kills it, proven by
hand (2026-11-12).
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
        '    let staged = validate_in_staging(roots, path)\n        .map_err(|err| format!("--{flag}: {}", flatten_line(&err.to_string())))?;\n',
        '    let staged = std::path::PathBuf::from(path);\n    let _ = roots;\n',
        'a_cli_json_file_is_read_only_from_a_staging_directory',
    ),
    (
        'M7 the certificate variables are never captured',
        'src/primitives/fetch_archive.rs',
        '        cell.get_or_init(|| CertEnv::capture(lookup));\n',
        '        let _ = lookup;\n',
        'the_cert_variables_are_captured_only_where_the_verifier_reads_them',
    ),
    (
        'M8 a redacted proxy value keeps its line breaks',
        'src/primitives/fetch_archive.rs',
        '    flatten_line(&shown)\n',
        '    shown\n',
        'a_redacted_proxy_value_is_one_line_without_its_credentials',
    ),
    (
        'M9 the first :// is taken as the scheme wherever it sits',
        'src/primitives/fetch_archive.rs',
        "        .filter(|&at| {\n            value[..at]\n                .chars()\n                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))\n        })\n",
        '',
        'a_redacted_proxy_value_is_one_line_without_its_credentials',
    ),
    (
        'M10 NO_PROXY exempts no host',
        'src/primitives/fetch_archive.rs',
        '                    .map(|proxy| proxy.no_proxy(reqwest::NoProxy::from_string(&self.no)))\n',
        '                    .map(|proxy| proxy)\n',
        'a_host_no_proxy_names_is_fetched_without_the_proxy',
    ),
    (
        'M11 a refused --*-json path is shown as given',
        'src/primitives/mod.rs',
        '        .map_err(|err| format!("--{flag}: {}", flatten_line(&err.to_string())))?;\n',
        '        .map_err(|err| format!("--{flag}: {err}"))?;\n',
        'a_cli_json_file_is_read_only_from_a_staging_directory',
    ),
    (
        'M12 an invalid --*-json file is named as given',
        'src/primitives/mod.rs',
        '            flatten_line(path)\n        )\n    })\n',
        '            path\n        )\n    })\n',
        'an_invalid_cli_json_file_is_named_on_one_line',
    ),
]
