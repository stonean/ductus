//! Shared helpers for integration test crates under `runtime/tests/`.
//!
//! Each `tests/*.rs` file compiles as its own integration-test binary,
//! so plain top-level helpers are not directly shareable. The
//! `tests/common/mod.rs` shape is the idiomatic Rust workaround: a
//! sub-module path that cargo does NOT auto-build as a test binary
//! (the `tests/foo.rs` shape would). Each integration test that needs
//! a helper does `mod common;` at the top of its file and imports the
//! symbols it uses.

#![allow(dead_code, clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;
use std::process::Command;

/// A runtime command with the session-identity variables removed (spec 062),
/// so a spawned process never depends on the shell running the suite — which
/// may itself carry a platform session id and would otherwise make every run
/// an identified process's (AGENTS.md, Gotchas). A test that spawns the binary
/// builds its command here, so the rule holds by construction, and one that
/// wants an identity sets it on the returned command; `concurrent_sessions.rs`
/// alone builds its own, since it sets each child's identity explicitly.
pub fn ductus_command(bin: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut command = Command::new(bin);
    command
        .env_remove("DUCTUS_SESSION")
        .env_remove("CLAUDE_CODE_SESSION_ID");
    command
}

/// A `git` command with the variables a git hook exports removed, so a test
/// that runs git in a fixture repository acts on the fixture and never on the
/// repository whose hook is running the suite. A commit from a linked
/// worktree hands its pre-commit hook an absolute `GIT_DIR` and
/// `GIT_INDEX_FILE`, and this repository's hook runs `cargo test`: inherited,
/// they make a fixture's `git init` re-initialise the outer repository, its
/// `git config` write the outer config, and its `git add` stage into the
/// outer index. A test that spawns git builds its command here, as one that
/// spawns the runtime builds it with [`ductus_command`];
/// `mechanical_sweep_parity.rs` does not yet, which spec 066's task 8 owns.
pub fn git_command() -> Command {
    let mut command = Command::new("git");
    command
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE");
    command
}

/// Recursively copy `src` into `dst`. Creates `dst` (and any missing
/// parents for nested files) as needed. Used to stage fixtures from
/// `runtime/tests/fixtures/<name>/` into a tempdir for write-side
/// integration tests.
pub fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir_recursive(&from, &to);
        } else {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(&from, &to).unwrap();
        }
    }
}

/// Render a fixture's spec files into the golden-comparison text form.
fn render_specs(specs: &[(String, String)]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    for (slug, body) in specs {
        let _ = write!(out, "=== {slug} ===\n{body}");
    }
    out
}

/// Compare rendered fixture output against `runtime/tests/golden/<dir>/<file>.md`.
///
/// Shared by the two `derive-*` golden suites, which produce the same shape and
/// previously carried byte-identical copies of this helper.
pub fn compare_golden(repo_root: &Path, dir: &str, file: &str, specs: &[(String, String)]) {
    let path = repo_root
        .join("runtime/tests/golden")
        .join(dir)
        .join(format!("{file}.md"));
    let actual = render_specs(specs);
    let expected = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing golden {}; re-bless with BLESS=1", path.display()));
    assert_eq!(
        expected, actual,
        "golden mismatch for {file}\n--- expected ---\n{expected}\n--- actual ---\n{actual}"
    );
}

/// With `BLESS=1`, rewrite the golden from the current output. Deliberate
/// behavior changes only — see each suite's provenance note.
pub fn maybe_bless(repo_root: &Path, dir: &str, file: &str, specs: &[(String, String)]) {
    if std::env::var("BLESS").as_deref() != Ok("1") {
        return;
    }
    let out_dir = repo_root.join("runtime/tests/golden").join(dir);
    fs::create_dir_all(&out_dir).unwrap();
    fs::write(out_dir.join(format!("{file}.md")), render_specs(specs)).unwrap();
}
