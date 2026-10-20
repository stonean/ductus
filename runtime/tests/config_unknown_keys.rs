//! Every runtime reader of a `ductus`-owned config section whose result has
//! no notices writes an unknown key in that section to stderr by name, and
//! the run still completes (spec 019; 022's
//! `config-readers-name-unknown-keys`). Driven through the binary, since
//! stderr is the channel under test. The readers with a notice channel,
//! `discover-rule-files` and `check-artifact-size`, and `dashboard`'s
//! callout are tested in their own modules.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

mod common;
use common::ductus_command;

/// One plausible typo in each section a stderr reader reads.
const CONFIG: &str = "\
[host]
projct = \"p\"

[paths]
spec-root = \"specs\"

[migrations]
last-applied = \"x\"

[constitutions.acme]
repo = \"https://github.com/acme/governance\"
path = \"../governance\"
url = \"https://github.com/acme/governance\"

[services.api]
repo = \"https://github.com/acme/api\"
path = \"../api\"
branch = \"main\"
";

fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join(".ductus")).unwrap();
    fs::write(dir.path().join(".ductus/config.toml"), CONFIG).unwrap();
    fs::create_dir_all(dir.path().join("specs/001-a")).unwrap();
    fs::write(
        dir.path().join("specs/001-a/spec.md"),
        "---\nstatus: draft\ndependencies: []\n---\n\n# 001 — A\n",
    )
    .unwrap();
    dir
}

/// The primitive's stderr, after asserting it completed with a JSON result.
fn stderr_of(dir: &Path, args: &[&str]) -> String {
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("binary runs");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(
        serde_json::from_str::<serde_json::Value>(&stdout).is_ok(),
        "{args:?} returned no result\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}"
    );
    stderr
}

#[test]
fn each_reader_names_the_unknown_keys_of_the_section_it_reads() {
    let dir = project();
    let cases: [(&[&str], &[&str]); 4] = [
        (
            &["dashboard"],
            &[
                "[host] projct",
                "[paths] spec-root",
                "[services.api] branch",
            ],
        ),
        (&["resolve-constitutions"], &["[constitutions.acme] url"]),
        (
            &["resolve-references", "--feature", "001-a"],
            &["[services.api] branch"],
        ),
        (
            &["check-orphaned-references"],
            &["[migrations] last-applied"],
        ),
    ];
    for (args, keys) in cases {
        let stderr = stderr_of(dir.path(), args);
        for key in keys {
            let line = format!("ductus: unknown config key: {key} (");
            assert_eq!(
                stderr.matches(&line).count(),
                1,
                "{args:?} should name {key} once: {stderr}"
            );
        }
    }
}

/// A warning never fails the run it warns about: with stderr's reader gone,
/// writing the warning fails, and the primitive still returns its result.
/// Unix only, where the runtime ignores `SIGPIPE`, so the write reports a
/// broken pipe that `eprintln!` would have turned into a panic.
#[cfg(unix)]
#[test]
fn a_warning_stderr_cannot_take_does_not_fail_the_run() {
    let dir = project();
    let (reader, writer) = std::io::pipe().unwrap();
    drop(reader);
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(["read-spec", "--feature", "001-a"])
        .current_dir(dir.path())
        .stderr(writer)
        .output()
        .expect("binary runs");
    assert!(output.status.success(), "{:?}", output.status);
    serde_json::from_slice::<serde_json::Value>(&output.stdout).expect("a JSON result");
}
