//! The CLI contract of an `unwritable` rewrite (spec 067, task 14): both
//! `derive-dependencies` and `derive-references` exit 1 and name each such
//! spec on stderr, report-only and with `--write` alike, so a hook running
//! under `set -e` stops the commit instead of passing over a stale index.
//!
//! The fixture's frontmatter is invalid YAML (`title: [oops`) while its
//! `dependencies:` and `references:` are stale, so the re-parse guard refuses
//! both rewrites. Unit tests in `src/main.rs` pin the predicates and the line;
//! only a run of the binary pins that `main` applies them.

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

use git2::{IndexAddOption, Repository, Signature};

mod common;
use common::ductus_command;

const SPEC: &str = "specs/001-a/spec.md";
const ORIGINAL: &str = "---\nstatus: done\ntitle: [oops\ndependencies: []\n---\n\n# A\n\n[b](../002-b/spec.md)\n\n[x](https://github.com/other/t/specs/004-new/spec.md)\n";

fn fixture(dir: &Path) {
    let repository = Repository::init(dir).unwrap();
    for (rel, body) in [
        (SPEC, ORIGINAL),
        (
            "specs/002-b/spec.md",
            "---\nstatus: done\ndependencies: []\n---\n\n# B\n",
        ),
    ] {
        let path = dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }
    let mut index = repository.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();
    repository
        .commit(Some("HEAD"), &sig, &sig, "fixture", &tree, &[])
        .unwrap();
}

#[test]
fn an_unwritable_rewrite_exits_1_and_names_the_spec_on_stderr() {
    for primitive in ["derive-dependencies", "derive-references"] {
        for write in [false, true] {
            let tmp = tempfile::tempdir().unwrap();
            fixture(tmp.path());
            let mut command = ductus_command(env!("CARGO_BIN_EXE_ductus"));
            command.arg(primitive).current_dir(tmp.path());
            if write {
                command.arg("--write");
            }
            let output = command.output().expect("binary runs");
            let stderr = String::from_utf8_lossy(&output.stderr);
            let label = format!("{primitive}, write {write}");
            assert_eq!(output.status.code(), Some(1), "{label}: {stderr}");
            assert!(
                stderr
                    .lines()
                    .any(|line| line.starts_with(&format!("{primitive}: {SPEC}:"))
                        && line.contains("edit that key by hand")),
                "{label}: {stderr}"
            );
            let stdout: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(stdout["unwritable"][0], SPEC, "{label}: {stdout}");
            assert_eq!(
                fs::read_to_string(tmp.path().join(SPEC)).unwrap(),
                ORIGINAL,
                "{label}: nothing is written"
            );
        }
    }
}

/// Past `[artifacts] read-size-bytes` the lists are in the file `lists-file`
/// names, and an unwritable spec moved there still exits 1 and is still named
/// on stderr: `main` reports and gates on the whole derivation, never on the
/// bounded result's lists (spec 067, task 32; scenario
/// `derive-references-results-are-bounded`).
#[test]
fn past_the_read_size_an_unwritable_rewrite_still_exits_1_and_is_named() {
    for primitive in ["derive-dependencies", "derive-references"] {
        let tmp = tempfile::tempdir().unwrap();
        fixture(tmp.path());
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 1\n",
        )
        .unwrap();
        let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
            .arg(primitive)
            .current_dir(tmp.path())
            .output()
            .expect("binary runs");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(1), "{primitive}: {stderr}");
        assert!(
            stderr
                .lines()
                .any(|line| line.starts_with(&format!("{primitive}: {SPEC}:"))),
            "{primitive}: {stderr}"
        );
        let stdout: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(stdout["form"], "file", "{primitive}: {stdout}");
        assert!(stdout.get("unwritable").is_none(), "{primitive}: {stdout}");
        assert_eq!(stdout["unwritable-count"], 1, "{primitive}: {stdout}");
    }
}
