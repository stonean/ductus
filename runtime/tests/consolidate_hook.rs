//! `/{project}:consolidate`'s documented order, driven through the shipped
//! adopter pre-commit hook (spec 052, scenario
//! `the-content-move-and-the-consolidation-commit-together`).
//!
//! The command migrates no content, so an operator moves the source's
//! scenarios and data model into the target first. Each case builds an
//! adopter-shaped repository, installs `framework/bootstrap/hooks/ductus-pre-commit`
//! as its hook with this build of the runtime behind `.ductus/bin/ductus`, and
//! commits through real `git`. The target links the source, as a target that
//! absorbs one usually does: before spec 051's
//! `a-link-inside-the-target-stays-inside-it`, re-pointing that link made the
//! target link itself, `derive-dependencies` reported the 1-cycle, and the
//! hook refused every consolidation commit.
//!
//! Unix only: the hook is a bash script reached through a symbolic link to the
//! binary, and the rewrite it exercises is platform-independent and covered on
//! every platform by `rewrite_spec_links`' unit tests.
#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::ductus_command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// `git` in `dir`, blind to the user's global and system config so a
/// `commit.gpgsign` or a hook setting there cannot decide a case, and with no
/// session identity for the runtime the hook spawns.
fn git(dir: &Path, args: &[&str]) -> Output {
    Command::new("git")
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

fn ductus(dir: &Path, args: &[&str]) {
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "ductus {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A committed adopter repository: `010-src` with a scenario and a data
/// model, `020-tgt` linking the source and its scenario, and `030-other`
/// linking both too.
fn adopter() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    git_ok(dir, &["init", "-q", "-b", "main", "."]);
    let hook = dir.join(".githooks/pre-commit");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::copy(
        workspace_root().join("framework/bootstrap/hooks/ductus-pre-commit"),
        &hook,
    )
    .unwrap();
    fs::create_dir_all(dir.join(".ductus/bin")).unwrap();
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_ductus"), dir.join(".ductus/bin/ductus"))
        .unwrap();
    git_ok(dir, &["config", "core.hooksPath", ".githooks"]);

    write(
        dir,
        "specs/010-src/spec.md",
        "---\nstatus: done\ndependencies: []\n---\n\n# 010 — Source\n\n\
         See [the eviction scenario](scenarios/eviction.md) and the [data model](data-model.md).\n",
    );
    write(
        dir,
        "specs/010-src/scenarios/eviction.md",
        "---\nsection: \"Cache\"\n---\n\n# Eviction\n\nPer [the parent](../spec.md).\n",
    );
    write(dir, "specs/010-src/data-model.md", "# 010 data model\n");
    write(
        dir,
        "specs/020-tgt/spec.md",
        "---\nstatus: done\ndependencies: [010-src]\n---\n\n# 020 — Target\n\n\
         Absorbs [the source](../010-src/spec.md) and its [eviction](../010-src/scenarios/eviction.md).\n",
    );
    write(
        dir,
        "specs/030-other/spec.md",
        "---\nstatus: done\ndependencies: [010-src]\n---\n\n# 030 — Other\n\n\
         See [the source](../010-src/spec.md) and [eviction](../010-src/scenarios/eviction.md).\n",
    );
    git_ok(dir, &["add", ".githooks", "specs"]);
    let (passed, stderr) = commit(dir, "seed");
    assert!(passed, "the seed corpus must commit cleanly: {stderr}");
    tmp
}

/// Step 4 and step 5 of `/{project}:consolidate`.
fn consolidate(dir: &Path) {
    ductus(
        dir,
        &["rewrite-spec-links", "--from", "010-src", "--to", "020-tgt"],
    );
    ductus(
        dir,
        &[
            "retire-feature",
            "--feature",
            "010-src",
            "--fold-target",
            "020-tgt",
            "--allow-sequential",
        ],
    );
    git_ok(dir, &["add", "-A", "specs"]);
}

/// The order the command documents: move the content, consolidate, then
/// commit the two together. A move committed on its own is refused, which is
/// why the order is forced; the consolidation commit passes, and the target
/// comes out linking its own files locally rather than itself.
#[test]
fn moving_the_content_then_consolidating_commits_through_the_hook() {
    let tmp = adopter();
    let dir = tmp.path();
    fs::create_dir_all(dir.join("specs/020-tgt/scenarios")).unwrap();
    git_ok(
        dir,
        &[
            "mv",
            "specs/010-src/scenarios/eviction.md",
            "specs/020-tgt/scenarios/eviction.md",
        ],
    );
    git_ok(
        dir,
        &[
            "mv",
            "specs/010-src/data-model.md",
            "specs/020-tgt/data-model.md",
        ],
    );

    let (passed, stderr) = commit(dir, "move content");
    assert!(
        !passed,
        "a move alone left links dangling, so the hook must refuse it"
    );
    assert!(stderr.contains("check-corpus-links"), "{stderr}");

    consolidate(dir);
    let (passed, stderr) = commit(dir, "move and consolidate");
    assert!(
        passed,
        "the move and the consolidation must commit together: {stderr}"
    );

    let target = fs::read_to_string(dir.join("specs/020-tgt/spec.md")).unwrap();
    assert!(target.contains("dependencies: []"), "{target}");
    assert!(target.contains("[the source](spec.md)"), "{target}");
    assert!(
        target.contains("[eviction](scenarios/eviction.md)"),
        "{target}"
    );
    let other = fs::read_to_string(dir.join("specs/030-other/spec.md")).unwrap();
    assert!(other.contains("dependencies: [020-tgt]"), "{other}");
    assert!(!dir.join("specs/010-src").exists());
}

/// The reviewed checkpoint: copy rather than move, and every link still
/// resolves, so the merge commits on its own; the consolidation then
/// re-points the inbound links onto the copies and removes the originals.
#[test]
fn copying_the_content_commits_a_checkpoint_before_the_consolidation() {
    let tmp = adopter();
    let dir = tmp.path();
    fs::create_dir_all(dir.join("specs/020-tgt/scenarios")).unwrap();
    fs::copy(
        dir.join("specs/010-src/scenarios/eviction.md"),
        dir.join("specs/020-tgt/scenarios/eviction.md"),
    )
    .unwrap();
    fs::copy(
        dir.join("specs/010-src/data-model.md"),
        dir.join("specs/020-tgt/data-model.md"),
    )
    .unwrap();
    git_ok(dir, &["add", "specs/020-tgt"]);

    let (passed, stderr) = commit(dir, "copy content");
    assert!(passed, "a copy leaves every link resolving: {stderr}");

    consolidate(dir);
    let (passed, stderr) = commit(dir, "consolidate");
    assert!(
        passed,
        "the consolidation after a checkpoint must commit: {stderr}"
    );
    assert!(!dir.join("specs/010-src").exists());
}
