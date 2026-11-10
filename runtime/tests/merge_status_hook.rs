//! `merge-status-hook` through the binary, where its probe finds `claude` on
//! the `PATH` the process inherits, as the MCP server Claude Code spawns
//! inherits Claude Code's (spec 080, AC20). The gate's every verdict and the
//! merge are unit-tested in `src/primitives/merge_status_hook.rs`; this pins
//! the lookup itself. Unix only, since the stand-in `claude` is a shell script.

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

mod common;
use common::ductus_command;

const SETTINGS: &str = ".claude/settings.local.json";

/// A directory holding a `claude` that prints `version`, for a `PATH`.
fn claude_printing(version: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let claude = dir.path().join("claude");
    fs::write(
        &claude,
        format!("#!/bin/sh\necho \"{version} (Claude Code)\"\n"),
    )
    .unwrap();
    fs::set_permissions(&claude, fs::Permissions::from_mode(0o755)).unwrap();
    // Run it once with no deadline, so macOS's scan of a new executable is
    // spent before the probe's 10 s timeout starts (task 52's flake).
    std::process::Command::new(&claude).output().unwrap();
    dir
}

/// Run `merge-status-hook` in `repo` with `PATH` set to `path`.
fn merge(repo: &Path, path: &Path) -> serde_json::Value {
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(["merge-status-hook", "--path", SETTINGS])
        .env("PATH", path)
        .current_dir(repo)
        .output()
        .expect("binary runs");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn the_probe_reads_the_claude_on_the_inherited_path() {
    let repo = tempfile::tempdir().unwrap();

    let old = claude_printing("2.1.50");
    let skipped = merge(repo.path(), old.path());
    assert_eq!(skipped["action"], "skipped");
    assert_eq!(skipped["claude-version"], "2.1.50");
    assert!(!repo.path().join(SETTINGS).exists());

    let current = claude_printing("2.1.319");
    let created = merge(repo.path(), current.path());
    assert_eq!(created["action"], "created", "{created}");
    assert_eq!(created["claude-version"], "2.1.319");
    let settings: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(repo.path().join(SETTINGS)).unwrap()).unwrap();
    assert_eq!(
        settings["hooks"]["UserPromptExpansion"][0]["hooks"][0]["command"],
        "\"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus\" hook status"
    );
}

/// With no `claude` on its `PATH`, the probe says so, and nothing is written.
#[test]
fn no_claude_on_the_path_is_skipped_as_unreadable() {
    let repo = tempfile::tempdir().unwrap();
    let empty = tempfile::tempdir().unwrap();
    let result = merge(repo.path(), empty.path());
    assert_eq!(result["action"], "skipped");
    assert_eq!(result["claude-version"], serde_json::Value::Null);
    assert!(
        result["reason"].as_str().unwrap().contains("not on PATH"),
        "{result}"
    );
    assert!(!repo.path().join(SETTINGS).exists());
}
