//! A subcommand whose stderr reader has gone keeps its exit status and the
//! result it wrote to stdout, and does not panic (022's
//! `a-closed-stdout-is-a-quiet-exit`): `eprintln!` panics on a pipe nobody
//! reads, so every exit-path diagnostic in `main.rs` goes through
//! `ductus::io::warn`. The proof pipe has its reader dropped
//! (`std::io::pipe()`), as `config_unknown_keys.rs` does, not a closed
//! descriptor.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;
use std::process::Output;

mod common;
use common::ductus_command;

/// A spec holding `body` under its frontmatter, under `specs/{name}/`.
fn write_spec(root: &Path, name: &str, body: &str) {
    let dir = root.join("specs").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("spec.md"),
        format!("---\nstatus: draft\ndependencies: []\n---\n\n# {name}\n\n{body}\n"),
    )
    .unwrap();
}

/// Run `args` in `dir` with a stderr whose reader has been dropped.
fn run_with_stderr_gone(dir: &Path, args: &[&str]) -> Output {
    let (reader, writer) = std::io::pipe().unwrap();
    drop(reader);
    ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(args)
        .current_dir(dir)
        .stderr(writer)
        .output()
        .expect("binary runs")
}

fn stdout_json(output: &Output) -> serde_json::Value {
    serde_json::from_slice(&output.stdout).expect("a JSON result on stdout")
}

/// `exec` of a command that does not exist exits 1 with its terminal `error`
/// envelope on stdout, whatever becomes of the stderr line.
#[cfg(unix)]
#[test]
fn exec_keeps_its_exit_and_error_envelope() {
    let dir = tempfile::tempdir().unwrap();
    let output = run_with_stderr_gone(dir.path(), &["exec", "nonexistent"]);
    assert_eq!(output.status.code(), Some(1), "{:?}", output.status);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("command-not-found"), "{stdout}");
}

/// `check-corpus-links` exits 1 and prints its JSON result, which names the
/// broken link, although its stderr report cannot be written.
#[cfg(unix)]
#[test]
fn check_corpus_links_keeps_its_exit_and_result() {
    let dir = tempfile::tempdir().unwrap();
    write_spec(dir.path(), "001-a", "See [gone](../009-gone/spec.md).");
    let output = run_with_stderr_gone(dir.path(), &["check-corpus-links"]);
    assert_eq!(output.status.code(), Some(1), "{:?}", output.status);
    let result = stdout_json(&output);
    assert_eq!(result["broken-count"], 1, "{result}");
}

/// `derive-dependencies` exits 1 and prints its JSON result, which names the
/// cycle, although its stderr report cannot be written.
#[cfg(unix)]
#[test]
fn derive_dependencies_keeps_its_exit_and_result() {
    let dir = tempfile::tempdir().unwrap();
    write_spec(dir.path(), "001-a", "See [b](../002-b/spec.md).");
    write_spec(dir.path(), "002-b", "See [a](../001-a/spec.md).");
    let output = run_with_stderr_gone(dir.path(), &["derive-dependencies"]);
    assert_eq!(output.status.code(), Some(1), "{:?}", output.status);
    let result = stdout_json(&output);
    assert!(
        result["cycles"].as_array().is_some_and(|c| !c.is_empty()),
        "{result}"
    );
}
