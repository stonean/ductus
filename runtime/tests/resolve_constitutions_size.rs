//! Every surface that dispatches `resolve-constitutions` as a primitive — the
//! CLI subcommand, the MCP tool and the exec walker — reaches the measured
//! entry, so `/{project}:target` sees each loaded shared constitution's size
//! (spec 055, scenario `target-reports-each-constitutions-size`). The
//! measurement itself, and the two callers that keep the unmeasured
//! resolution, are tested in `resolve_constitutions`' own module; these tests
//! prove only the wiring, which a unit test of the module cannot reach.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use rmcp::ServiceExt;
use rmcp::model::CallToolRequestParams;
use serde_json::{Value, json};

use ductus::mcp::server::GovRuntimeServer;

mod common;
use common::ductus_command;

/// A project registering `acme`, whose checkout's `constitution.md` is
/// `bytes` long, under `artifacts`, the `[artifacts]` table's body.
fn project(artifacts: &str, bytes: usize) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join(".ductus")).unwrap();
    fs::write(
        dir.path().join(".ductus/config.toml"),
        format!(
            "[artifacts]\n{artifacts}\n\
             [constitutions.acme]\nrepo = \"https://example.test/acme\"\npath = \"gov\"\n"
        ),
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("gov")).unwrap();
    fs::write(dir.path().join("gov/constitution.md"), "x".repeat(bytes)).unwrap();
    dir
}

/// The one loaded record and the threshold, asserted as measured: one byte
/// over a 100-byte limit, with the owning repository named as the fix.
fn assert_measured(result: &Value) {
    assert_eq!(
        result["threshold"],
        json!({"bytes": 100, "source": "config"})
    );
    let record = &result["loaded"][0];
    assert_eq!(record["alias"], "acme", "{result}");
    assert_eq!(record["bytes"], 101, "{result}");
    assert_eq!(record["over-limit"], true, "{result}");
    let warning = record["warning"].as_str().expect("a warning when over");
    assert!(
        warning.contains("https://example.test/acme"),
        "the fix names the owning repository: {warning}"
    );
}

fn cli(dir: &Path) -> std::process::Output {
    ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .arg("resolve-constitutions")
        .current_dir(dir)
        .output()
        .expect("binary runs")
}

#[test]
fn the_cli_subcommand_measures_each_loaded_document() {
    let dir = project("read-size-bytes = 100", 101);
    let output = cli(dir.path());
    assert!(output.status.success(), "{output:?}");
    assert_measured(&serde_json::from_slice(&output.stdout).expect("a JSON result"));
}

/// The measured entry reads `[artifacts]`, so it names an unknown key there,
/// as every reader of a section names the unknown keys of the section it
/// reads (spec 019) — and only once it measures, since with nothing
/// registered nothing new is read.
#[test]
fn the_cli_subcommand_names_an_unknown_artifacts_key_once_it_measures() {
    let line = "ductus: unknown config key: [artifacts] read-size-byte (";
    let dir = project("read-size-byte = 100", 10);
    let output = cli(dir.path());
    assert!(output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr.matches(line).count(), 1, "{stderr}");

    let unregistered = tempfile::tempdir().unwrap();
    fs::create_dir_all(unregistered.path().join(".ductus")).unwrap();
    fs::write(
        unregistered.path().join(".ductus/config.toml"),
        "[artifacts]\nread-size-byte = 100\n",
    )
    .unwrap();
    let output = cli(unregistered.path());
    assert!(output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains(line), "nothing registered: {stderr}");
}

#[tokio::test]
async fn the_mcp_tool_measures_each_loaded_document() {
    let dir = project("read-size-bytes = 100", 101);
    let (server_side, client_side) = tokio::io::duplex(64 * 1024);
    let server = GovRuntimeServer::new(PathBuf::from(dir.path()));
    tokio::spawn(async move {
        if let Ok(service) = server.serve(server_side).await {
            let _ = service.waiting().await;
        }
    });
    let client = ().serve(client_side).await.unwrap();
    let result = client
        .call_tool(
            CallToolRequestParams::new("resolve-constitutions".to_string())
                .with_arguments(serde_json::Map::new()),
        )
        .await
        .expect("the tool answers");
    let structured = result.structured_content.expect("a structured result");
    assert_measured(&structured);
}

/// The exec walker dispatches the measured entry too, observed through the
/// one effect a walk exposes: a malformed read size halts it once a source is
/// registered, as it halts `/{project}:target`.
#[test]
fn an_exec_walk_halts_on_a_malformed_read_size_once_a_source_is_registered() {
    let dir = project("read-size-bytes = \"lots\"", 10);
    let commands = dir.path().join("framework/commands");
    fs::create_dir_all(&commands).unwrap();
    fs::write(
        commands.join("probe.md"),
        "# /ductus:probe\n\n## Instructions\n\n\
         1. Invoke `resolve-constitutions` to resolve the registry.\n",
    )
    .unwrap();

    let output = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(["exec", "probe"])
        .current_dir(dir.path())
        .stdin(Stdio::null())
        .output()
        .expect("binary runs");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success(), "the walk halts: {stdout}");
    let last = stdout
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .expect("a terminal envelope");
    let envelope: Value = serde_json::from_str(last).expect("a JSON envelope");
    assert_eq!(envelope["type"], "error", "{envelope}");
    let message = envelope["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("read-size-bytes"),
        "names the setting: {message}"
    );
}
