//! The CLI contract of `check-artifact-size --project-files --staged` (spec 063,
//! scenario `project-files-are-measured`, for spec 075's hook step): exit 1
//! when a staged project file is refused, naming its path, size, limit and fix
//! on stderr; exit 0 when none is; exit 1 and no JSON on a call that cannot
//! look, so a hook under `set -e` stops the commit rather than passing over it.
//!
//! Unit tests in `src/primitives/check_artifact_size.rs` pin the refusal rule;
//! only a run of the binary pins that `main` turns it into an exit status.

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

use git2::{IndexAddOption, Repository, Signature};

mod common;
use common::ductus_command;

/// A repository at `dir` holding a 100-byte read size and an `AGENTS.md` of
/// `committed` bytes, committed.
fn fixture(dir: &Path, committed: usize) -> Repository {
    let repository = Repository::init(dir).unwrap();
    fs::create_dir_all(dir.join(".ductus")).unwrap();
    fs::write(
        dir.join(".ductus/config.toml"),
        "[artifacts]\nread-size-bytes = 100\n",
    )
    .unwrap();
    fs::write(dir.join("AGENTS.md"), "x".repeat(committed)).unwrap();
    let mut index = repository.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();
    repository
        .commit(Some("HEAD"), &sig, &sig, "fixture", &tree, &[])
        .unwrap();
    drop(tree);
    repository
}

/// Set the read size in the working tree's config, which the check reads from
/// disk whatever the index holds.
fn read_size(dir: &Path, bytes: usize) {
    fs::write(
        dir.join(".ductus/config.toml"),
        format!("[artifacts]\nread-size-bytes = {bytes}\n"),
    )
    .unwrap();
}

fn stage_agents(repository: &Repository, dir: &Path, bytes: usize) {
    fs::write(dir.join("AGENTS.md"), "x".repeat(bytes)).unwrap();
    let mut index = repository.index().unwrap();
    index.add_path(Path::new("AGENTS.md")).unwrap();
    index.write().unwrap();
}

fn run(dir: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .arg("check-artifact-size")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("binary runs");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn a_refused_file_exits_1_and_names_its_path_size_limit_and_fix() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), 150);
    stage_agents(&repository, tmp.path(), 151);
    let (code, stdout, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(1), "{stderr}");
    for named in ["AGENTS.md", "151", "100-byte", "/prune --project"] {
        assert!(stderr.contains(named), "{named} not in: {stderr}");
    }
    // The result is still printed, so a caller reading JSON sees the same.
    assert!(stdout.contains("\"refused\":true"), "{stdout}");
}

/// The same refusal in the inline form, the one a project at a read size its
/// staged result fits takes: the fixture's 100-byte read size puts every
/// staged result past it, so the test above runs in the file form only
/// (BE-PAYLOAD-001).
#[test]
fn a_refused_file_exits_1_in_the_inline_form() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), 1600);
    read_size(tmp.path(), 1500);
    stage_agents(&repository, tmp.path(), 2000);
    let (code, stdout, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(1), "{stderr}");
    for named in ["AGENTS.md", "2000", "1500-byte", "/prune --project"] {
        assert!(stderr.contains(named), "{named} not in: {stderr}");
    }
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(result["form"], "inline", "{stdout}");
    assert_eq!(result["oversized-count"], 1, "{stdout}");
    assert_eq!(result["oversized"][0]["path"], "AGENTS.md", "{stdout}");
}

/// A staged project file that cannot be measured stops the commit as a refused
/// one does: a hook that passed over a file it could not measure would let an
/// oversized one through. `AGENTS.md` is left in conflict, one index entry per
/// stage and none at stage 0, so there is no one staged copy; it is under the
/// read size, so only the unmeasured state can set the exit status. It does so
/// in both forms: past the read size `skipped` is in the list file, and the
/// exit and the stderr line must not depend on it being inline
/// (BE-PAYLOAD-001).
#[test]
fn an_unmeasurable_staged_file_exits_1_and_names_it_in_either_form() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), 50);
    let mut index = repository.index().unwrap();
    let entry = index.get_path(Path::new("AGENTS.md"), 0).unwrap();
    index.remove_path(Path::new("AGENTS.md")).unwrap();
    for stage in 1..=3_u16 {
        let staged = git2::IndexEntry {
            ctime: entry.ctime,
            mtime: entry.mtime,
            dev: entry.dev,
            ino: entry.ino,
            mode: entry.mode,
            uid: entry.uid,
            gid: entry.gid,
            file_size: entry.file_size,
            id: entry.id,
            flags: (entry.flags & !0x3000) | (stage << 12),
            flags_extended: entry.flags_extended,
            path: entry.path.clone(),
        };
        index.add(&staged).unwrap();
    }
    index.write().unwrap();
    assert!(repository.index().unwrap().has_conflicts());

    for (bytes, form) in [(100, "file"), (100_000, "inline")] {
        read_size(tmp.path(), bytes);
        let (code, stdout, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
        assert_eq!(code, Some(1), "{form}: {stdout}");
        assert!(
            stderr.contains("AGENTS.md could not be measured (artifact-unreadable)"),
            "{form}: {stderr}"
        );
        let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
        assert_eq!(result["form"], form, "{stdout}");
        assert_eq!(result["skipped-count"], 1, "{stdout}");
        assert_eq!(result["skipped"].is_array(), form == "inline", "{stdout}");
        assert!(stdout.contains("\"state\":\"unreadable\""), "{stdout}");
        assert!(!stdout.contains("\"refused\":true"), "{stdout}");
    }
}

#[test]
fn an_unchanged_or_shrunk_file_exits_0() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), 150);
    let (code, _, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(0), "unchanged: {stderr}");
    stage_agents(&repository, tmp.path(), 120);
    let (code, _, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(0), "shrunk: {stderr}");
    assert!(stderr.is_empty(), "{stderr}");
}

/// Past the read size the oversized file is held in the list file the result
/// names, under `project-files` for a call naming no feature, with its count
/// inline (BE-PAYLOAD-001).
#[test]
fn an_oversized_file_without_staged_is_a_finding_never_an_exit_status() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), 150);
    let (code, stdout, _) = run(tmp.path(), &["--project-files"]);
    assert_eq!(code, Some(0));
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(result["form"], "file", "{stdout}");
    assert_eq!(result["oversized-count"], 1, "{stdout}");
    let file = result["lists-file"].as_str().unwrap();
    assert!(
        file.starts_with(".ductus/results/check-artifact-size/project-files/"),
        "{file}"
    );
    let lists: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(tmp.path().join(file)).unwrap()).unwrap();
    assert_eq!(lists["oversized"][0]["kind"], "project-file", "{lists}");
    assert_eq!(lists["oversized"][0]["path"], "AGENTS.md", "{lists}");
}

#[test]
fn a_call_that_cannot_look_exits_1() {
    // No repository: staged has no index to read.
    let tmp = tempfile::tempdir().unwrap();
    let (code, stdout, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(1), "{stdout}");
    assert_ne!(stderr, "");
    // Naming nothing to measure is an argument error.
    let (code, _, stderr) = run(tmp.path(), &[]);
    assert_eq!(code, Some(1), "{stderr}");
    assert!(stderr.contains("project-files"), "{stderr}");
}

// -- parts (spec 087, AC5) ---------------------------------------------------

const PART: &str = ".ductus/docs/system/storage.md";

/// Write `rel` with `bytes` of content and stage it.
fn stage(repository: &Repository, dir: &Path, rel: &str, bytes: usize) {
    let path = dir.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, "x".repeat(bytes)).unwrap();
    let mut index = repository.index().unwrap();
    index.add_path(Path::new(rel)).unwrap();
    index.write().unwrap();
}

/// Commit what is staged on top of `HEAD`.
fn commit(repository: &Repository) {
    let mut index = repository.index().unwrap();
    let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
    let parent = repository.head().unwrap().peel_to_commit().unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();
    repository
        .commit(Some("HEAD"), &sig, &sig, "parts", &tree, &[&parent])
        .unwrap();
}

/// A staged part over the read size with no `HEAD` copy, or larger than its
/// `HEAD` copy, is refused, named on stderr with its path, staged size, limit
/// and fix, in the file form (the fixture's 100-byte read size) and the
/// inline one.
#[test]
fn a_refused_part_exits_1_and_names_its_path_size_limit_and_fix_in_either_form() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), 10);
    stage(&repository, tmp.path(), PART, 151);
    let (code, stdout, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(1), "{stderr}");
    for named in [
        PART,
        "151 bytes staged",
        "no HEAD copy",
        "100-byte",
        "sibling parts",
    ] {
        assert!(stderr.contains(named), "{named} not in: {stderr}");
    }
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(result["form"], "file", "{stdout}");
    assert_eq!(result["refused-count"], 1, "{stdout}");

    // Inline: a part committed over the limit, then grown.
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), 10);
    stage(&repository, tmp.path(), PART, 1600);
    commit(&repository);
    read_size(tmp.path(), 1500);
    stage(&repository, tmp.path(), PART, 2000);
    let (code, stdout, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(1), "{stderr}");
    for named in [
        PART,
        "2000 bytes staged",
        "HEAD copy 1600 bytes",
        "1500-byte",
    ] {
        assert!(stderr.contains(named), "{named} not in: {stderr}");
    }
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(result["form"], "inline", "{stdout}");
    assert_eq!(result["refused"], serde_json::json!([PART]), "{stdout}");
}

/// An over-limit part that shrinks or is left unchanged passes, and so does
/// one deleted in the commit, whose index link is a finding of the next
/// unstaged call, never a refusal.
#[test]
fn a_shrunk_unchanged_or_deleted_part_exits_0() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), 10);
    stage(&repository, tmp.path(), PART, 150);
    commit(&repository);
    let (code, _, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(0), "unchanged: {stderr}");
    stage(&repository, tmp.path(), PART, 120);
    let (code, _, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(0), "shrunk: {stderr}");
    assert!(stderr.is_empty(), "{stderr}");
    let mut index = repository.index().unwrap();
    index.remove_path(Path::new(PART)).unwrap();
    index.write().unwrap();
    let (code, stdout, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
    assert_eq!(code, Some(0), "deleted: {stderr}");
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(result["parts-count"], 0, "{stdout}");
}

/// A staged part that cannot be measured, left in conflict with no stage-0
/// entry, stops the commit and is named, in either form.
#[test]
fn an_unmeasurable_staged_part_exits_1_and_names_it_in_either_form() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), 10);
    stage(&repository, tmp.path(), PART, 20);
    let mut index = repository.index().unwrap();
    let entry = index.get_path(Path::new(PART), 0).unwrap();
    index.remove_path(Path::new(PART)).unwrap();
    for stage in 1..=3_u16 {
        let staged = git2::IndexEntry {
            ctime: entry.ctime,
            mtime: entry.mtime,
            dev: entry.dev,
            ino: entry.ino,
            mode: entry.mode,
            uid: entry.uid,
            gid: entry.gid,
            file_size: entry.file_size,
            id: entry.id,
            flags: (entry.flags & !0x3000) | (stage << 12),
            flags_extended: entry.flags_extended,
            path: entry.path.clone(),
        };
        index.add(&staged).unwrap();
    }
    index.write().unwrap();
    for (bytes, form) in [(100, "file"), (100_000, "inline")] {
        read_size(tmp.path(), bytes);
        let (code, stdout, stderr) = run(tmp.path(), &["--project-files", "--staged"]);
        assert_eq!(code, Some(1), "{form}: {stdout}");
        assert!(
            stderr.contains(&format!(
                "{PART} could not be measured (artifact-unreadable)"
            )),
            "{form}: {stderr}"
        );
        let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
        assert_eq!(result["form"], form, "{stdout}");
        assert_eq!(result["refused-count"], 0, "{stdout}");
    }
}
