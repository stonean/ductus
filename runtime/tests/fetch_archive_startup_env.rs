//! `fetch-archive` takes its proxy configuration from the environment the
//! process started with, parsed there, so a value it cannot use refuses the
//! fetch naming its variable, with the credentials shown as `***`
//! (`CFG-ENV-001`, `CFG-ENV-008`; spec 048, `fetch-archive-reads-its-proxy-once`).
//! Driven through the binary: the capture is `main`'s call, and no in-process
//! test reaches it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;
use common::ductus_command;

#[test]
fn a_proxy_value_that_does_not_parse_refuses_the_fetch_at_the_binary() {
    let project = tempfile::tempdir().unwrap();
    let staging = tempfile::tempdir().unwrap();
    let archive = staging.path().join("out.tar.gz");
    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(["fetch-archive", "--url", "https://192.0.2.1/main.tar.gz"])
        .arg("--archive")
        .arg(&archive)
        .current_dir(project.path())
        .env_remove("ALL_PROXY")
        .env_remove("all_proxy")
        .env_remove("HTTP_PROXY")
        .env_remove("http_proxy")
        .env_remove("https_proxy")
        .env_remove("REQUEST_METHOD")
        .env("HTTPS_PROXY", "socks5://ops:hunter2@proxy.corp:1080")
        .output()
        .expect("binary runs");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("HTTPS_PROXY"), "{stderr}");
    assert!(stderr.contains("://***@proxy.corp"), "{stderr}");
    assert!(!stderr.contains("hunter2"), "credentials shown: {stderr}");
    assert!(!archive.exists(), "nothing was written");
}
