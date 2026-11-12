//! `lint-markdown` lints only what it is given, though the project's config names `globs`
//! (022's `lint-markdown-lints-only-what-it-is-given`). Runs the real markdownlint from this
//! checkout's `node_modules`, linked into a two-file project whose config sets `globs` as the
//! shipped one does; a missing install fails the test loudly rather than skipping it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

use serde_json::Value;

mod common;
use common::ductus_command;

/// `a.md` and the feature directory's file are clean; `b.md` has trailing spaces (MD009).
fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::write(
        root.join(".markdownlint-cli2.jsonc"),
        "{ \"globs\": [\"**/*.md\"], \"ignores\": [\"node_modules\"] }\n",
    )
    .unwrap();
    fs::write(root.join(".gitignore"), "node_modules\n").unwrap();
    fs::write(root.join("a.md"), "# A\n\nclean\n").unwrap();
    fs::write(root.join("b.md"), "# B\n\ntrailing   \n").unwrap();
    fs::create_dir_all(root.join("specs/001-a")).unwrap();
    fs::write(root.join("specs/001-a/spec.md"), "# Spec\n\nclean\n").unwrap();
    let modules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../node_modules");
    assert!(
        modules.join(".bin").exists(),
        "run `npm ci` at the checkout root: this test needs the real markdownlint"
    );
    #[cfg(unix)]
    std::os::unix::fs::symlink(&modules, root.join("node_modules")).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&modules, root.join("node_modules")).unwrap();
    dir
}

fn lint(root: &Path, path: &str) -> Value {
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(["lint-markdown", "--path", path])
        .current_dir(root)
        .output()
        .expect("binary runs");
    serde_json::from_slice(&output.stdout).unwrap_or_else(|err| {
        panic!(
            "{err}: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

#[test]
fn a_given_path_reports_violations_only_in_the_files_it_names() {
    let dir = project();
    let clean = lint(dir.path(), "a.md");
    assert_eq!(clean["clean"], true, "{clean}");
    assert_eq!(clean["violations-count"], 0, "{clean}");

    let dirty = lint(dir.path(), "b.md");
    assert_eq!(dirty["clean"], false, "{dirty}");
    let paths: Vec<&str> = dirty["violations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["path"].as_str().unwrap())
        .collect();
    assert_eq!(paths, ["b.md"], "{dirty}");
}

/// The gate's subject is the feature's markdown glob: a violation elsewhere is not its.
#[test]
fn a_feature_glob_ignores_a_violation_outside_the_feature_directory() {
    let dir = project();
    let result = lint(dir.path(), "specs/001-a/**/*.md");
    assert_eq!(result["clean"], true, "{result}");
    assert_eq!(result["violations-count"], 0, "{result}");
}

#[test]
fn a_path_naming_no_file_is_not_clean() {
    let dir = project();
    let result = lint(dir.path(), "missing/**/*.md");
    assert_eq!(result["clean"], false, "{result}");
    assert_eq!(result["violations-count"], 0, "{result}");
    assert_eq!(result["exit-code"], 0, "{result}");
}
