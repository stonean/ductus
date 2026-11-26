//! The CLI contract of `check-audit-records` (spec 085), which the adopter CI
//! template's audit-record gate reads as stderr lines and an exit status: a
//! clean corpus exits 0 with its counts reported, a finding exits 1 naming its
//! file, a run that examined no spec exits 1 saying so, and a spec root that
//! cannot be listed exits 1 naming it.
//!
//! Unit tests in `src/main.rs` pin the predicate and the lines; only a run of
//! the binary pins that `main` applies them.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;
use std::process::Output;

mod common;
use common::ductus_command;

const DONE_SPEC: &str = "---\nstatus: done\ndependencies: []\n---\n\n# A\n";
const REVIEW: &str = "---\nspec: x\nlast-run: 2026-11-25T00:00:00Z\nmust-violations: 0\nshould-violations: 0\nlow-confidence: 0\nblocking: false\n---\n\n# Review\n";
const ANALYSIS: &str = "---\nspec: x\nlast-run: 2026-11-25T00:00:00Z\nhard-fail: 0\nblocking-findings: 0\nadvisory: 0\nunexamined: 0\nblocking: false\n---\n\n# Analysis\n";

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

fn check(root: &Path) -> (Output, String) {
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .arg("check-audit-records")
        .current_dir(root)
        .output()
        .expect("binary runs");
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();
    (output, stderr)
}

/// A project with one `done` spec carrying both records, clean.
fn project() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    write(
        tmp.path(),
        ".ductus/config.toml",
        "[host]\nproject = \"acme\"\n",
    );
    write(tmp.path(), "specs/001-a/spec.md", DONE_SPEC);
    write(tmp.path(), "specs/001-a/review.md", REVIEW);
    write(tmp.path(), "specs/001-a/analysis.md", ANALYSIS);
    tmp
}

#[test]
fn a_clean_corpus_exits_zero_with_its_counts_reported() {
    let tmp = project();
    let (output, stderr) = check(tmp.path());
    assert!(output.status.success(), "{stderr}");
    assert_eq!(
        stderr,
        "check-audit-records: examined 1 spec(s) under 'specs'; 1 at done; 0 skipped\n\
         check-audit-records:   review-exempt: 0 (baseline 0)\n\
         check-audit-records:   analyze-exempt: 0 (baseline 0)\n"
    );
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["findings-count"], 0);
}

#[test]
fn a_finding_exits_one_naming_its_file() {
    let tmp = project();
    write(
        tmp.path(),
        "specs/001-a/review.md",
        &REVIEW.replace("blocking: false\n", "blocking: true\n"),
    );
    let (output, stderr) = check(tmp.path());
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(
        stderr.starts_with("check-audit-records: finding: specs/001-a/review.md: done spec has review.blocking=true"),
        "{stderr}"
    );
    assert!(stderr.contains("/acme:review"), "{stderr}");
    assert!(stderr.contains("examined 1 spec(s)"), "{stderr}");
}

#[test]
fn a_run_that_examined_no_spec_exits_one_saying_so() {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join("specs")).unwrap();
    let (output, stderr) = check(tmp.path());
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(
        stderr.ends_with(
            "check-audit-records: nothing examined: no spec under 'specs' was read, which is not the same as passing\n"
        ),
        "{stderr}"
    );
}

#[test]
fn a_spec_root_that_cannot_be_listed_exits_one_naming_it() {
    let tmp = tempfile::tempdir().unwrap();
    let (output, stderr) = check(tmp.path());
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("specs"), "{stderr}");
    assert!(
        !stderr.contains("examined"),
        "an error is no count: {stderr}"
    );
    assert!(output.stdout.is_empty(), "an error is no result");
}
