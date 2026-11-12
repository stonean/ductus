//! A subcommand whose stdout reader has gone exits quietly with a status of its own, never a
//! panic (022's `a-closed-stdout-is-a-quiet-exit`). The proof drops the reader of a
//! `std::io::pipe()`, as `config_unknown_keys.rs` does for stderr, rather than closing a
//! descriptor: the write meets `EPIPE`, which `println!` turns into a panic and exit 101.

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;
use common::ductus_command;

/// `128 + SIGPIPE`: distinct from a failure (1) and from a panic (101).
#[cfg(unix)]
#[test]
fn a_subcommand_with_no_stdout_reader_exits_quietly_with_its_own_status() {
    let dir = tempfile::tempdir().unwrap();
    let cases: [&[&str]; 3] = [
        &["resolve-session"],
        &["parse", "--emit-schema"],
        &["dashboard"],
    ];
    for args in cases {
        let (reader, writer) = std::io::pipe().unwrap();
        drop(reader);
        let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
            .args(args)
            .current_dir(dir.path())
            .stdout(writer)
            .output()
            .expect("binary runs");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("panicked"), "{args:?}: {stderr}");
        assert!(!stderr.contains("Broken pipe"), "{args:?}: {stderr}");
        assert_eq!(output.status.code(), Some(141), "{args:?}: {stderr}");
    }
}
