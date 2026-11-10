//! Cross-process session targets (spec 062, task 7).
//!
//! The unit tests in `src/session.rs` pass identities as parameters. These
//! spawn the real binary as separate processes, each with its own
//! environment, because the property the spec promises is about *processes*:
//! two agents in one working tree, each launched with its own identity, each
//! keeping its own target. Every child's environment is scrubbed of the
//! identity variables first, so the shell running the suite — which may itself
//! carry a platform session id — cannot leak into a case.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::Value;

mod common;

fn runtime_binary() -> PathBuf {
    common::release_binary()
}

fn ensure_binary_built() {
    // Always build (once per test binary): an incremental release build is a
    // fast no-op when current, and it guarantees the tested binary matches
    // the working tree.
    static BUILD: std::sync::Once = std::sync::Once::new();
    BUILD.call_once(|| {
        let status = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("cargo build --release must succeed");
        assert!(status.success(), "cargo build failed");
    });
}

/// Which identity a child process is launched with.
#[derive(Clone, Copy)]
enum Who<'a> {
    /// `DUCTUS_SESSION={name}`.
    Named(&'a str),
    /// `CLAUDE_CODE_SESSION_ID={id}` and nothing else.
    Claude(&'a str),
    /// Neither variable.
    Nobody,
}

fn command(repo: &Path, who: Who<'_>, args: &[&str]) -> Command {
    let mut cmd = Command::new(runtime_binary());
    cmd.args(args)
        .current_dir(repo)
        .env_remove("DUCTUS_SESSION")
        .env_remove("CLAUDE_CODE_SESSION_ID")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    match who {
        Who::Named(name) => {
            cmd.env("DUCTUS_SESSION", name);
        }
        Who::Claude(id) => {
            cmd.env("CLAUDE_CODE_SESSION_ID", id);
        }
        Who::Nobody => {}
    }
    cmd
}

fn run(repo: &Path, who: Who<'_>, args: &[&str]) -> Value {
    let out = command(repo, who, args)
        .output()
        .expect("runtime binary must run");
    assert!(
        out.status.success(),
        "{args:?} failed: {}\n{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    serde_json::from_slice(&out.stdout).expect("primitive output is JSON")
}

fn target(repo: &Path, who: Who<'_>, feature: &str) -> Value {
    let path = format!("specs/{feature}");
    run(
        repo,
        who,
        &["write-session", "--feature", feature, "--path", &path],
    )
}

fn resolved(repo: &Path, who: Who<'_>) -> Option<String> {
    let value = run(repo, who, &["resolve-session"]);
    value["target"]["feature"].as_str().map(str::to_owned)
}

fn repo() -> tempfile::TempDir {
    ensure_binary_built();
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
    tmp
}

/// AC1 and AC2: a target write in one process — plain, or in any of the
/// shapes the commands write as a side effect — leaves the other process's
/// target alone, and each resolves its own.
#[test]
fn a_write_in_one_process_never_moves_the_other() {
    let tmp = repo();
    let (a, b) = (Who::Named("alpha"), Who::Named("beta"));
    target(tmp.path(), a, "055-a");

    // /target, /specify, /groom's spec-edit route: a plain target write.
    target(tmp.path(), b, "056-b");
    assert_eq!(resolved(tmp.path(), a).as_deref(), Some("055-a"));
    assert_eq!(resolved(tmp.path(), b).as_deref(), Some("056-b"));

    // /amend's scenario route and /groom's scenario route: a scenario target.
    run(
        tmp.path(),
        b,
        &[
            "write-session",
            "--feature",
            "057-c",
            "--path",
            "specs/057-c",
            "--scenario",
            "edge",
            "--scenario-path",
            "specs/057-c/scenarios/edge.md",
        ],
    );
    assert_eq!(resolved(tmp.path(), a).as_deref(), Some("055-a"));

    // /target --clear.
    run(tmp.path(), b, &["write-session", "--clear"]);
    assert_eq!(resolved(tmp.path(), a).as_deref(), Some("055-a"));
    assert_eq!(resolved(tmp.path(), b), None, "beta cleared its own");

    // /fold and /consolidate of a spec alpha does not target.
    run(
        tmp.path(),
        b,
        &[
            "retarget-sessions",
            "--from",
            "1234.1-x",
            "--cause",
            "fold",
            "--feature",
            "058-d",
            "--path",
            "specs/058-d",
        ],
    );
    run(
        tmp.path(),
        b,
        &[
            "retarget-sessions",
            "--from",
            "059-e",
            "--cause",
            "consolidate",
            "--clear",
        ],
    );
    assert_eq!(resolved(tmp.path(), a).as_deref(), Some("055-a"));

    // The pipeline view agrees with resolve-session, per process (AC8): its
    // target line names this process's own target, labelled with its session.
    let view = run(tmp.path(), a, &["render-status"]);
    let first = view["rendered-markdown"]
        .as_str()
        .and_then(|md| md.lines().next())
        .unwrap_or_default();
    assert!(
        first.starts_with("Target: 055-a ") && first.ends_with("— session alpha"),
        "the view's target line: {first:?}"
    );
    let dash = run(tmp.path(), a, &["dashboard"]);
    assert_eq!(dash["session-target"]["feature"], "055-a");
    assert_eq!(dash["session-identity"], "alpha");
}

/// AC4: many processes writing at the same moment each keep what they wrote.
#[test]
fn concurrent_writers_each_keep_their_own_target() {
    const WRITERS: usize = 16;
    let tmp = repo();
    let names: Vec<String> = (0..WRITERS).map(|i| format!("w{i}")).collect();
    let children: Vec<_> = names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let feature = format!("{:03}-f{i}", 100 + i);
            let path = format!("specs/{feature}");
            command(
                tmp.path(),
                Who::Named(name),
                &["write-session", "--feature", &feature, "--path", &path],
            )
            .spawn()
            .expect("spawn writer")
        })
        .collect();
    for child in children {
        let out = child.wait_with_output().unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    for (i, name) in names.iter().enumerate() {
        assert_eq!(
            resolved(tmp.path(), Who::Named(name)),
            Some(format!("{:03}-f{i}", 100 + i)),
            "writer {name} lost its target"
        );
    }
    // And the default holds one of them, whole — never a torn mixture.
    let default = std::fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
    let parsed: toml::Value = toml::from_str(&default).unwrap();
    let feature = parsed["feature"].as_str().unwrap();
    assert_eq!(
        parsed["path"].as_str().unwrap(),
        format!("specs/{feature}"),
        "{default}"
    );
}

/// AC13: on Claude Code, with no `DUCTUS_SESSION`, two agents are told apart
/// by the session id Claude Code passes each of them.
#[test]
fn two_claude_code_sessions_hold_separate_targets() {
    let tmp = repo();
    let first = Who::Claude("3f2a9c1d-0000-4000-8000-000000000001");
    let second = Who::Claude("7b1e0d44-0000-4000-8000-000000000002");
    let written = target(tmp.path(), first, "055-a");
    assert_eq!(written["identity"], "claude-code:3f2a9c1d");
    target(tmp.path(), second, "056-b");
    assert_eq!(resolved(tmp.path(), first).as_deref(), Some("055-a"));
    assert_eq!(resolved(tmp.path(), second).as_deref(), Some("056-b"));
}

/// AC15: processes with no identity share the default, exactly as before
/// spec 062 — the bound the spec states rather than closes.
#[test]
fn two_unidentified_processes_share_the_default() {
    let tmp = repo();
    target(tmp.path(), Who::Nobody, "055-a");
    assert_eq!(resolved(tmp.path(), Who::Nobody).as_deref(), Some("055-a"));
    target(tmp.path(), Who::Nobody, "056-b");
    assert_eq!(resolved(tmp.path(), Who::Nobody).as_deref(), Some("056-b"));
    // Their writes hold the session lock, so the directory holds the lock and
    // its `.gitignore` — dotfiles, never a per-process target.
    let targets: Vec<_> = std::fs::read_dir(tmp.path().join(".ductus/sessions"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .filter(|name| !name.to_string_lossy().starts_with('.'))
        .collect();
    assert!(
        targets.is_empty(),
        "unidentified processes create no per-process state: {targets:?}"
    );
}

/// AC14 at the process edge: a name that sanitizes to nothing is refused,
/// naming the variable, rather than silently running unidentified.
#[test]
fn an_unusable_session_name_is_refused_at_the_edge() {
    let tmp = repo();
    let out = command(tmp.path(), Who::Named("!!!"), &["resolve-session"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("DUCTUS_SESSION"), "{stderr}");
}
