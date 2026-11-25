//! The CLI contract of `check-unreviewed-code` (spec 084, AC8), which a pre-push
//! hook and a CI step read as an exit status: under `block`, the default, it
//! exits 1 and names the code on stderr; under `warn` it prints the same report
//! and exits 0; an invalid `[review] unreviewed-push`, or a configuration file
//! that does not parse, exits 1 naming the key or the file; and a subject it
//! cannot examine is treated as reported code is.
//!
//! Unit tests in `src/main.rs` pin the predicate and the lines; only a run of
//! the binary pins that `main` applies them.

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;
use std::process::Output;

use git2::{IndexAddOption, Oid, Repository, Signature};

mod common;
use common::ductus_command;

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

fn commit_all(repository: &Repository, subject: &str) -> Oid {
    let mut index = repository.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.update_all(["*"], None).unwrap();
    index.write().unwrap();
    let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
    let signature = Signature::now("Test", "test@example.com").unwrap();
    let parents: Vec<_> = repository
        .head()
        .ok()
        .and_then(|head| head.peel_to_commit().ok())
        .into_iter()
        .collect();
    let parents: Vec<&git2::Commit<'_>> = parents.iter().collect();
    repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            subject,
            &tree,
            &parents,
        )
        .unwrap()
}

fn check(root: &Path, args: &[&str]) -> Output {
    ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .arg("check-unreviewed-code")
        .args(args)
        .current_dir(root)
        .output()
        .expect("binary runs")
}

/// A project whose record is seeded, with or without a commit no spec claims
/// after it.
fn fixture(root: &Path, unclaimed_code: bool) -> Repository {
    let repository = Repository::init(root).unwrap();
    write(
        root,
        "specs/001-a/spec.md",
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# A\n",
    );
    commit_all(&repository, "spec(001): add");
    let seeded = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(["write-review", "--seed"])
        .current_dir(root)
        .output()
        .unwrap();
    assert!(seeded.status.success(), "{seeded:?}");
    commit_all(&repository, "chore: seed the project record");
    if unclaimed_code {
        write(root, "src/u.rs", "fn u() {}\n");
        commit_all(&repository, "chore: nobody's");
    }
    repository
}

fn result(output: &Output) -> serde_json::Value {
    serde_json::from_slice(&output.stdout).expect("a JSON result on stdout")
}

#[test]
fn nothing_reported_exits_0_and_prints_no_report() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), false);
    let output = check(tmp.path(), &[]);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let result = result(&output);
    assert_eq!(result["state"], "none");
    assert_eq!(result["mode"], "block");
}

#[test]
fn reported_code_exits_1_under_block_and_0_under_warn_with_the_same_report() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), true);
    let blocked = check(tmp.path(), &[]);
    assert_eq!(blocked.status.code(), Some(1), "{blocked:?}");
    let report = String::from_utf8_lossy(&blocked.stderr).to_string();
    assert!(
        report.contains("code no spec claims: 1 path(s)") && report.contains("  src/u.rs\n"),
        "{report}"
    );
    assert_eq!(result(&blocked)["state"], "found");

    write(
        tmp.path(),
        ".ductus/config.toml",
        "[review]\nunreviewed-push = \"warn\"\n",
    );
    let warned = check(tmp.path(), &[]);
    assert_eq!(warned.status.code(), Some(0), "{warned:?}");
    let warned_report = String::from_utf8_lossy(&warned.stderr).to_string();
    let body = |text: &str| -> Vec<String> {
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        lines.pop();
        lines
    };
    assert_eq!(
        body(&warned_report),
        body(&report),
        "all but the closing line"
    );
    assert!(
        warned_report.contains("reported, not blocked"),
        "{warned_report}"
    );
    assert_eq!(result(&warned)["state"], "found");
    assert_eq!(result(&warned)["mode"], "warn");
}

/// The block report names the review command in the project's own namespace,
/// `[host] project`, as `/{project}:status` names every command.
#[test]
fn the_block_report_names_the_projects_own_review_command() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), true);
    write(
        tmp.path(),
        ".ductus/config.toml",
        "[host]\nproject = \"acme\"\n",
    );
    let blocked = check(tmp.path(), &[]);
    assert_eq!(blocked.status.code(), Some(1), "{blocked:?}");
    let report = String::from_utf8_lossy(&blocked.stderr).to_string();
    assert!(
        report.contains("review it (`/acme:review` on each spec"),
        "{report}"
    );
    assert!(!report.contains("{project}"), "{report}");
}

/// A host configuration that cannot be read, here a session file that does
/// not parse, leaves the review command's `{project}` placeholder in the block
/// report, and the report and its exit status stand.
#[test]
fn the_block_report_keeps_the_placeholder_when_the_namespace_cannot_be_read() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), true);
    write(
        tmp.path(),
        ".ductus/config.toml",
        "[host]\nproject = \"acme\"\n",
    );
    write(tmp.path(), ".ductus/session.toml", "feature = [\n");
    let blocked = check(tmp.path(), &[]);
    assert_eq!(blocked.status.code(), Some(1), "{blocked:?}");
    let report = String::from_utf8_lossy(&blocked.stderr).to_string();
    assert!(report.contains("  src/u.rs\n"), "{report}");
    assert!(
        report.contains("review it (`/{project}:review` on each spec"),
        "{report}"
    );
    assert!(!report.contains("/acme:review"), "{report}");
}

#[test]
fn a_subject_not_examined_is_treated_as_reported_code_is() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), false);
    // The commit before the seed holds no project record.
    let head = repository.head().unwrap().peel_to_commit().unwrap();
    let before = head.parent_id(0).unwrap().to_string();
    let blocked = check(tmp.path(), &["--at", &before]);
    assert_eq!(blocked.status.code(), Some(1), "{blocked:?}");
    let report = String::from_utf8_lossy(&blocked.stderr).to_string();
    assert!(
        report.contains("project was not examined") && report.contains("write-review --seed"),
        "{report}"
    );
    assert_eq!(result(&blocked)["state"], "not-examined");
    write(
        tmp.path(),
        ".ductus/config.toml",
        "[review]\nunreviewed-push = \"warn\"\n",
    );
    let warned = check(tmp.path(), &["--at", &before]);
    assert_eq!(warned.status.code(), Some(0), "{warned:?}");
    assert!(
        String::from_utf8_lossy(&warned.stderr).contains("project was not examined"),
        "{warned:?}"
    );
}

#[test]
fn an_invalid_key_or_an_unparsed_file_exits_1_naming_it_and_reports_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), true);
    for (config, names) in [
        (
            "[review]\nunreviewed-push = \"off\"\n",
            "invalid `[review] unreviewed-push` in",
        ),
        ("[review\n", "config.toml"),
    ] {
        write(tmp.path(), ".ductus/config.toml", config);
        let output = check(tmp.path(), &[]);
        assert_eq!(output.status.code(), Some(1), "{config}: {output:?}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(names) && stderr.contains(".ductus/config.toml"),
            "{config}: {stderr}"
        );
        assert!(output.stdout.is_empty(), "no result: {output:?}");
        assert!(
            !stderr.contains("src/u.rs"),
            "nothing is reported: {stderr}"
        );
    }
}

/// A mistyped `[review]` key is not silently dropped: stderr names the key and
/// the file, and the mode stays the default (`CFG-ENV-012`).
#[test]
fn a_mistyped_review_key_is_named_on_stderr() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), true);
    write(
        tmp.path(),
        ".ductus/config.toml",
        "[review]\nunreviewed_push = \"warn\"\n",
    );
    let output = check(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "typo keeps block: {output:?}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unreviewed_push") && stderr.contains(".ductus/config.toml"),
        "{stderr}"
    );
}

#[test]
fn since_counts_on_the_cli() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), true);
    let head = repository.head().unwrap().peel_to_commit().unwrap();
    let seed = head.parent_id(0).unwrap().to_string();
    write(
        tmp.path(),
        ".ductus/config.toml",
        "[review]\nunreviewed-push = \"warn\"\n",
    );
    let output = check(tmp.path(), &["--since", &seed]);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let result = result(&output);
    assert_eq!(result["commits-walked"], 1);
    assert_eq!(result["commits-reporting"], 1);
    assert_eq!(result["commits-not-examined"], 0);
}
