//! The shipped adopter pre-commit hook refuses a commit that grows a project
//! file's part past `[artifacts] read-size-bytes`, and lets one that shrinks
//! it through (spec 087, AC5).
//!
//! Each case builds an adopter-shaped repository, installs
//! `framework/bootstrap/hooks/ductus-pre-commit` as its hook with this build
//! of the runtime behind `.ductus/bin/ductus`, as `consolidate_hook.rs` does,
//! and commits through real `git`, for a project at the repository's root and
//! for one in a subdirectory of it (spec 059), where the hook finds its
//! project from its own location. `check_artifact_size_cli.rs` pins the
//! CLI's exit and stderr; this pins that the hook the adopter runs acts on
//! them.
//!
//! Unix only: the hook is a bash script reached through a symbolic link to the
//! binary.
#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use common::git_command;

const PART: &str = ".ductus/docs/system/storage.md";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// `git` in `dir`, blind to the user's global and system config and to the
/// repository whose hook may be running this suite ([`git_command`]), with
/// no session identity for the runtime the hook spawns.
fn git(dir: &Path, args: &[&str]) -> Output {
    git_command()
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env_remove("DUCTUS_SESSION")
        .env_remove("CLAUDE_CODE_SESSION_ID")
        .args([
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.invalid",
        ])
        .args(args)
        .output()
        .unwrap()
}

fn git_ok(dir: &Path, args: &[&str]) {
    let output = git(dir, args);
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Commit what is staged, returning whether the hook let it through and
/// what it printed.
fn commit(dir: &Path, message: &str) -> (bool, String) {
    let output = git(dir, &["commit", "-q", "-m", message]);
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn write(dir: &Path, rel: &str, content: &str) {
    let path = dir.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn read_size(project: &Path, bytes: usize) {
    write(
        project,
        ".ductus/config.toml",
        &format!("[artifacts]\nread-size-bytes = {bytes}\n"),
    );
}

/// A committed repository at `repo` whose project sits at `repo/{place}` (the
/// root when `place` is empty), holding a `system.md` index and its one part
/// of 150 bytes, committed under a 1,000-byte read size and then left at a
/// 100-byte one on disk, so the part is already over the limit. Returns the
/// project directory.
fn adopter(repo: &Path, place: &str) -> PathBuf {
    git_ok(repo, &["init", "-q", "-b", "main", "."]);
    let project = if place.is_empty() {
        repo.to_path_buf()
    } else {
        repo.join(place)
    };
    let hook = project.join(".githooks/pre-commit");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::copy(
        workspace_root().join("framework/bootstrap/hooks/ductus-pre-commit"),
        &hook,
    )
    .unwrap();
    fs::create_dir_all(project.join(".ductus/bin")).unwrap();
    std::os::unix::fs::symlink(
        env!("CARGO_BIN_EXE_ductus"),
        project.join(".ductus/bin/ductus"),
    )
    .unwrap();
    let hooks_path = if place.is_empty() {
        ".githooks".to_string()
    } else {
        format!("{place}/.githooks")
    };
    git_ok(repo, &["config", "core.hooksPath", &hooks_path]);
    read_size(&project, 1000);
    write(
        &project,
        "specs/system.md",
        "# System\n\n## Storage\n\nSee [storage](../.ductus/docs/system/storage.md).\n",
    );
    write(&project, PART, &"x".repeat(150));
    git_ok(repo, &["add", "."]);
    let (passed, stderr) = commit(repo, "seed");
    assert!(passed, "the seed must commit cleanly: {stderr}");
    read_size(&project, 100);
    project
}

fn grow_then_shrink(place: &str) {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    let project = adopter(repo, place);
    let staged = |bytes: usize| {
        write(&project, PART, &"x".repeat(bytes));
        let rel = if place.is_empty() {
            PART.to_string()
        } else {
            format!("{place}/{PART}")
        };
        git_ok(repo, &["add", &rel]);
    };

    staged(151);
    let (passed, stderr) = commit(repo, "grow the part");
    assert!(
        !passed,
        "{place:?}: growing an over-limit part must be refused"
    );
    for named in [PART, "151 bytes staged", "HEAD copy 150 bytes", "100-byte"] {
        assert!(
            stderr.contains(named),
            "{place:?}: {named} not in: {stderr}"
        );
    }

    staged(120);
    let (passed, stderr) = commit(repo, "shrink the part");
    assert!(
        passed,
        "{place:?}: shrinking an over-limit part must pass: {stderr}"
    );
}

#[test]
fn a_commit_growing_a_part_past_the_limit_is_refused_and_one_shrinking_it_passes() {
    grow_then_shrink("");
}

#[test]
fn the_same_holds_for_a_project_in_a_subdirectory_of_its_repository() {
    grow_then_shrink("proj");
}
