//! The CLI's exit status for `check-step-references`: non-zero when a
//! reference section is reached by no step, as the audit and any hook that
//! calls it read it (spec 026).

#![cfg(unix)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

mod common;
use common::ductus_command;

fn tree(dir: &Path, reference: &str) {
    for sub in ["framework/commands", "framework/reference"] {
        fs::create_dir_all(dir.join(sub)).unwrap();
    }
    fs::write(
        dir.join("framework/commands/x.md"),
        "# X\n\n## Instructions\n\n1. One, per **§Top** in `.ductus/reference/x.md`.\n2. Two.\n3. Three.\n",
    )
    .unwrap();
    fs::write(dir.join("framework/reference/x.md"), reference).unwrap();
}

fn run(dir: &Path) -> (Option<i32>, String) {
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .arg("check-step-references")
        .current_dir(dir)
        .output()
        .expect("binary runs");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

#[test]
fn an_unreached_section_exits_1_and_a_reached_one_exits_0() {
    let tmp = tempfile::tempdir().unwrap();
    tree(tmp.path(), "## Top\n\nText.\n");
    let (code, stdout) = run(tmp.path());
    assert_eq!(code, Some(0), "{stdout}");

    tree(tmp.path(), "## Top\n\nText.\n\n## Orphan\n\nText.\n");
    let (code, stdout) = run(tmp.path());
    assert_eq!(code, Some(1), "{stdout}");
    assert!(stdout.contains("unreached-section"), "{stdout}");
    assert!(stdout.contains("Orphan"), "{stdout}");
}
