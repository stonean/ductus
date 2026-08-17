//! The CLI argument surface for the two manifest primitives.
//!
//! `apply-manifest`'s `entries` / `pinned` / `substitutions` and
//! `enforce-manifest`'s `expected` / `pinned` are arrays and maps of objects,
//! which clap cannot express as flags — they arrive through the JSON context on
//! the MCP and interpreter paths. Spec 048's `state-b-continues-in-session`
//! needs them from the **CLI**, because a State-B `/ductus` run has the binary
//! on disk but no live MCP server, so it drives the rest of the bootstrap
//! through `.ductus/bin/ductus <primitive>`.
//!
//! Without this surface the run would reach Shared Files, hand `apply-manifest`
//! an **empty** manifest, and copy nothing — silently, because an empty
//! manifest is a legal one and the primitive would report success over it. That
//! is the failure these tests exist to keep out, so the loud-failure cases
//! matter more here than the happy path.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn runtime_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/release")
        .join(format!("ductus{}", std::env::consts::EXE_SUFFIX))
}

fn ensure_binary_built() {
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

fn run(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(runtime_binary())
        .args(args)
        .current_dir(repo)
        .output()
        .expect("runtime binary must run")
}

/// A staging tree with one file to copy, plus a destination.
fn staged(tmp: &Path) -> (PathBuf, PathBuf) {
    let src = tmp.join("staging");
    let dst = tmp.join("project");
    fs::create_dir_all(src.join("framework")).unwrap();
    fs::create_dir_all(&dst).unwrap();
    fs::write(src.join("framework/constitution.md"), "# {name}\n").unwrap();
    (src, dst)
}

#[test]
fn entries_json_supplies_the_manifest_the_cli_cannot_express() {
    ensure_binary_built();
    let tmp = tempfile::tempdir().unwrap();
    let (_src, dst) = staged(tmp.path());

    let entries = tmp.path().join("entries.json");
    fs::write(
        &entries,
        r#"[{"source":"framework/constitution.md","dest":".ductus/constitution.md","strategy":"update"}]"#,
    )
    .unwrap();
    let subs = tmp.path().join("subs.json");
    fs::write(&subs, r#"{"name":"Acme"}"#).unwrap();

    let out = run(
        tmp.path(),
        &[
            "apply-manifest",
            "--source-root",
            "staging",
            "--target-root",
            "project",
            "--entries-json",
            "entries.json",
            "--substitutions-json",
            "subs.json",
        ],
    );
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let written = fs::read_to_string(dst.join(".ductus/constitution.md"))
        .expect("the manifest entry must have been applied");
    assert_eq!(
        written, "# Acme\n",
        "substitutions must arrive through --substitutions-json too"
    );
}

#[test]
fn an_unreadable_entries_file_fails_loudly_instead_of_applying_nothing() {
    // The whole point. An empty manifest is a *legal* manifest, so a silent
    // fallback here would make `apply-manifest` report success over a
    // destination it never wrote — `QUAL-CLAIM-001`, at the step that installs
    // every shared file an adopter gets.
    ensure_binary_built();
    let tmp = tempfile::tempdir().unwrap();
    let (_src, dst) = staged(tmp.path());

    let out = run(
        tmp.path(),
        &[
            "apply-manifest",
            "--source-root",
            "staging",
            "--target-root",
            "project",
            "--entries-json",
            "absent.json",
        ],
    );
    assert!(!out.status.success(), "a missing manifest must not succeed");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--entries-json") && stderr.contains("cannot read"),
        "the error must name the flag and the failure: {stderr}"
    );
    assert!(
        !dst.join(".ductus").exists(),
        "nothing may be written when the manifest could not be read"
    );
}

#[test]
fn malformed_json_fails_loudly_rather_than_defaulting_to_empty() {
    ensure_binary_built();
    let tmp = tempfile::tempdir().unwrap();
    let (_src, _dst) = staged(tmp.path());
    let entries = tmp.path().join("entries.json");
    fs::write(&entries, "{ not json").unwrap();

    let out = run(
        tmp.path(),
        &[
            "apply-manifest",
            "--source-root",
            "staging",
            "--target-root",
            "project",
            "--entries-json",
            "entries.json",
        ],
    );
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("not valid JSON"),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn enforce_manifest_expected_json_drives_the_cleanup() {
    ensure_binary_built();
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("commands");
    fs::create_dir_all(&dir).unwrap();
    for name in ["keep.md", "drop.md", "pinned.md"] {
        fs::write(dir.join(name), "x\n").unwrap();
    }
    let expected = tmp.path().join("expected.json");
    fs::write(&expected, r#"["keep.md"]"#).unwrap();
    let pinned = tmp.path().join("pinned.json");
    fs::write(&pinned, r#"["pinned.md"]"#).unwrap();

    let out = run(
        tmp.path(),
        &[
            "enforce-manifest",
            "--directory",
            "commands",
            "--expected-json",
            "expected.json",
            "--pinned-json",
            "pinned.json",
        ],
    );
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dir.join("keep.md").exists(), "expected file must survive");
    assert!(dir.join("pinned.md").exists(), "pinned file must survive");
    assert!(
        !dir.join("drop.md").exists(),
        "an unexpected, unpinned file must be removed"
    );
}

#[test]
fn omitting_the_flags_leaves_the_context_supplied_fields_untouched() {
    // The MCP and interpreter paths pass these fields directly and never set
    // the `--*-json` flags; adding the flags must not change that behaviour.
    // With no entries at all, `apply-manifest` applies nothing and succeeds —
    // which is correct *here* because the caller genuinely supplied none.
    ensure_binary_built();
    let tmp = tempfile::tempdir().unwrap();
    let (_src, dst) = staged(tmp.path());
    let out = run(
        tmp.path(),
        &[
            "apply-manifest",
            "--source-root",
            "staging",
            "--target-root",
            "project",
        ],
    );
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!dst.join(".ductus").exists());
}
