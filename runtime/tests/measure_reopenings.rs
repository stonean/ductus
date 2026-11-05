//! Integration test (spec 079): `measure-reopenings` and the line it gives
//! `/{project}:status`, through the built binary in an adopter-shaped project, with
//! its own spec root and a history made by hand.
//!
//! The primitive's unit tests prove each rule and kill its mutations. This proves
//! each state reaches a caller as the CLI's JSON and as `dashboard`'s rendered view,
//! since AC8 asks for the measurement in an adopter's project and AC19 asks for the
//! counts on every run.

#![allow(clippy::expect_used, clippy::panic)]

mod common;

use std::path::Path;

use common::ductus_command;
use git2::{Repository, Signature};
use serde_json::Value;

const RUNTIME: &str = env!("CARGO_BIN_EXE_ductus");
const SPECS_ROOT: &str = "governance";

fn write(path: &Path, body: &str) {
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("create parents");
    std::fs::write(path, body).expect("write the file");
}

fn commit(repository: &Repository, subject: &str) -> git2::Oid {
    let mut index = repository.index().expect("the index");
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .expect("stage");
    index.write().expect("write the index");
    let tree = repository
        .find_tree(index.write_tree().expect("a tree"))
        .expect("find the tree");
    let signature = Signature::now("Test", "test@example.com").expect("a signature");
    let parent = repository
        .head()
        .ok()
        .and_then(|head| head.target())
        .map(|oid| repository.find_commit(oid).expect("the parent"));
    let parents: Vec<&git2::Commit<'_>> = parent.iter().collect();
    repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            subject,
            &tree,
            &parents,
        )
        .expect("commit")
}

/// A project with its own spec root, as `/ductus` leaves an adopter's.
fn adopter(dir: &Path) -> Repository {
    let repository = Repository::init(dir).expect("init");
    write(
        &dir.join(".ductus/config.toml"),
        &format!("[paths]\nspecs-root = \"{SPECS_ROOT}\"\n"),
    );
    repository
}

fn spec(status: &str) -> String {
    format!("---\nstatus: {status}\ndependencies: []\n---\n\n# Spec\n")
}

fn record(last_run: &str, extra: &str) -> String {
    format!("---\nspec: s\nlast-run: {last_run}\n{extra}---\n\n# Record\n")
}

/// Commit each step as one change under `feature`: a status, or a file's new text.
fn pass(dir: &Path, repository: &Repository, feature: &str, steps: &[(&str, &str)]) {
    for (file, body) in steps {
        write(&dir.join(SPECS_ROOT).join(feature).join(file), body);
        commit(repository, "work: step");
    }
}

fn runtime_json(dir: &Path, args: &[&str]) -> Value {
    let output = ductus_command(RUNTIME)
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run the runtime");
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("JSON on stdout")
}

fn measure(dir: &Path) -> Value {
    runtime_json(dir, &["measure-reopenings"])
}

fn reopenings_line(dir: &Path) -> String {
    let dashboard = runtime_json(dir, &["dashboard"]);
    let rendered = dashboard["rendered-markdown"].as_str().expect("rendered");
    rendered
        .lines()
        .find(|line| line.starts_with("Reopenings"))
        .unwrap_or_else(|| panic!("no Reopenings line: {rendered}"))
        .to_string()
}

fn entry<'a>(list: &'a Value, feature: &str) -> &'a Value {
    list.as_array()
        .expect("a list")
        .iter()
        .find(|item| item["feature"] == feature)
        .unwrap_or_else(|| panic!("no {feature} in {list}"))
}

/// One spec for each kind of reopening, and two still `in-progress`.
fn seed_every_kind(dir: &Path, repository: &Repository) {
    let (in_progress, done) = (spec("in-progress"), spec("done"));
    let first = "2026-01-01T00:00:00Z";
    let second = "2026-01-02T00:00:00Z";
    pass(
        dir,
        repository,
        "001-within",
        &[
            ("spec.md", &spec("planned")),
            ("spec.md", &in_progress),
            ("review.md", &record(first, "")),
            ("analysis.md", &record(first, "")),
            ("spec.md", &done),
        ],
    );
    pass(
        dir,
        repository,
        "002-over",
        &[
            ("spec.md", &in_progress),
            ("review.md", &record(first, "")),
            ("review.md", &record(second, "")),
            ("spec.md", &done),
        ],
    );
    pass(
        dir,
        repository,
        "003-after-a-gating-finding",
        &[
            ("spec.md", &in_progress),
            ("review.md", &record(first, "must-violations: 1\n")),
            ("review.md", &record(second, "")),
            ("spec.md", &done),
        ],
    );
    pass(
        dir,
        repository,
        "004-two-writes",
        &[
            ("spec.md", &in_progress),
            ("review.md", &record(first, "fix-rounds-open: true\n")),
            ("review.md", &record(first, "")),
            ("spec.md", &done),
        ],
    );
    pass(
        dir,
        repository,
        "005-no-run",
        &[("spec.md", &in_progress), ("spec.md", &done)],
    );
    pass(
        dir,
        repository,
        "006-damaged",
        &[
            ("spec.md", &in_progress),
            ("review.md", "no frontmatter\n"),
            ("spec.md", &done),
        ],
    );
    pass(
        dir,
        repository,
        "007-waiting",
        &[
            ("spec.md", &in_progress),
            ("tasks.md", "# Tasks\n\n## 1. A\n\n- [x] a\n"),
        ],
    );
    pass(
        dir,
        repository,
        "008-working",
        &[
            ("spec.md", &in_progress),
            ("tasks.md", "# Tasks\n\n## 1. A\n\n- [ ] a\n"),
        ],
    );
}

#[test]
fn every_kind_of_reopening_is_reported_and_status_shows_the_counts() {
    let dir = tempfile::tempdir().expect("a directory");
    let dir = dir.path();
    let repository = adopter(dir);
    seed_every_kind(dir, &repository);

    let result = measure(dir);
    assert_eq!(result["state"], "measured");
    assert_eq!(result["closed-count"], 6);
    assert_eq!(result["over-bound-count"], 1);
    assert_eq!(result["not-counted-count"], 2);
    assert_eq!(result["waiting-count"], 1);
    assert_eq!(result["runs"], 7);
    assert_eq!(result["form"], "inline");
    let reopenings = &result["reopenings"];
    assert_eq!(entry(reopenings, "001-within")["over-bound"], false);
    assert_eq!(entry(reopenings, "002-over")["over-bound"], true);
    assert_eq!(
        entry(reopenings, "003-after-a-gating-finding")["over-bound"],
        false
    );
    assert_eq!(entry(reopenings, "004-two-writes")["reviews"], 1);
    let not_counted = &result["not-counted"];
    assert_eq!(
        entry(not_counted, "005-no-run")["reason"],
        "no-recorded-run"
    );
    assert_eq!(
        entry(not_counted, "006-damaged")["reason"],
        "record-unparseable"
    );
    assert_eq!(result["waiting"][0], "007-waiting");

    let line = reopenings_line(dir);
    assert!(
        line.contains("6 closed, 1 over the review and analysis bound, 2 not counted"),
        "{line}"
    );
    assert!(
        line.contains("1 in-progress spec(s) with no unchecked task"),
        "{line}"
    );
}

#[test]
fn nothing_closed_is_nothing_to_compare_and_status_says_so() {
    let dir = tempfile::tempdir().expect("a directory");
    let dir = dir.path();
    let repository = adopter(dir);
    pass(
        dir,
        &repository,
        "001-open",
        &[("spec.md", &spec("in-progress"))],
    );
    let result = measure(dir);
    assert_eq!(result["state"], "nothing-to-compare");
    assert_eq!(result["closed-count"], 0);
    let line = reopenings_line(dir);
    assert!(line.contains("none closed, nothing to compare"), "{line}");
    assert!(
        line.contains("1 in-progress spec(s) with no unchecked task"),
        "{line}"
    );
}

/// `.git/shallow` is what a depth-limited clone leaves, and what libgit2 reads.
#[test]
fn a_shallow_history_says_it_cannot_measure_and_status_says_so() {
    let dir = tempfile::tempdir().expect("a directory");
    let dir = dir.path();
    let repository = adopter(dir);
    pass(
        dir,
        &repository,
        "001-closed",
        &[
            ("spec.md", &spec("in-progress")),
            ("review.md", &record("2026-01-01T00:00:00Z", "")),
            ("spec.md", &spec("done")),
        ],
    );
    let head = repository.head().expect("HEAD").target().expect("a commit");
    write(&repository.path().join("shallow"), &format!("{head}\n"));
    let result = measure(dir);
    assert_eq!(result["state"], "shallow-history");
    assert_eq!(result["closed-count"], 0);
    let line = reopenings_line(dir);
    assert!(line.contains("history too shallow to measure"), "{line}");
}

/// A project no repository holds still gets a pipeline view, whose line says the
/// measurement did not run, while the primitive itself reports the error.
#[test]
fn no_repository_is_an_error_and_status_says_not_measured() {
    let dir = tempfile::tempdir().expect("a directory");
    let dir = dir.path();
    write(&dir.join("specs/001-a/spec.md"), &spec("in-progress"));
    let output = ductus_command(RUNTIME)
        .arg("measure-reopenings")
        .current_dir(dir)
        .output()
        .expect("run the runtime");
    assert!(!output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    let line = reopenings_line(dir);
    assert!(line.starts_with("Reopenings: ? not measured — "), "{line}");
}
