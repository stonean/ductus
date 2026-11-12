//! Integration test for the MCP server surface.
//!
//! Starts a `GovRuntimeServer` in-process on one half of a `tokio::io::duplex`
//! pair and connects an `rmcp` client on the other half. The test:
//!
//! 1. Lists tools and asserts the served set equals `tool_names()` (the
//!    canonical primitive registry less `CLI_ONLY_PRIMITIVES`).
//! 2. Asserts `framework/runtime-tools.txt` (the shipped manifest) is
//!    set-equal to the same set.
//! 3. Invokes read-only primitives against the shared fixture repo and
//!    asserts each response is a structured JSON object — a sample of the
//!    surface, not every tool.
//! 4. Invokes the write primitives against per-test scratch copies of the
//!    fixture so each test is hermetic.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use rmcp::ServiceExt;
use rmcp::model::CallToolRequestParams;
use serde_json::{Value, json};

use ductus::mcp::server::{ALWAYS_LOAD_META, GovRuntimeServer, tool_names};
use ductus::primitives::tool_loading::text_step_tools;
use ductus::schema::descriptions;
use ductus::schema::registry::CLI_ONLY_PRIMITIVES;

mod common;

fn fixture_repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/primitives/sample-repo")
}

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is the runtime crate; the workspace root is its parent.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn copy_recursively(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let ty = entry.file_type().unwrap();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_recursively(&entry.path(), &dst_path);
        } else {
            fs::copy(entry.path(), dst_path).unwrap();
        }
    }
}

/// Vendor a `markdownlint-cli2` that reports nothing and exits 0, which
/// `lint-markdown` prefers over `npx` — so a test whose subject is not the lint
/// never waits on a download.
fn vendor_a_clean_markdownlint(root: &Path) {
    let bin = root.join("node_modules").join(".bin");
    fs::create_dir_all(&bin).unwrap();
    #[cfg(windows)]
    fs::write(bin.join("markdownlint-cli2.cmd"), "@exit /b 0\r\n").unwrap();
    #[cfg(not(windows))]
    {
        let stub = bin.join("markdownlint-cli2");
        fs::write(&stub, "#!/bin/sh\nexit 0\n").unwrap();
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&stub).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&stub, perms).unwrap();
        }
    }
}

async fn start_pair(repo: PathBuf) -> rmcp::service::RunningService<rmcp::service::RoleClient, ()> {
    let (server_side, client_side) = tokio::io::duplex(64 * 1024);
    let server = GovRuntimeServer::new(repo);
    tokio::spawn(async move {
        match server.serve(server_side).await {
            Ok(service) => {
                let _ = service.waiting().await;
            }
            Err(err) => eprintln!("server failed: {err}"),
        }
    });
    ().serve(client_side).await.unwrap()
}

async fn call_tool(
    client: &rmcp::service::RunningService<rmcp::service::RoleClient, ()>,
    name: &str,
    arguments: Value,
) -> rmcp::model::CallToolResult {
    let args_obj = arguments
        .as_object()
        .cloned()
        .expect("arguments must be a JSON object");
    client
        .call_tool(CallToolRequestParams::new(name.to_string()).with_arguments(args_obj))
        .await
        .unwrap_or_else(|err| panic!("call_tool({name}) failed: {err}"))
}

fn structured_object(result: &rmcp::model::CallToolResult) -> &serde_json::Map<String, Value> {
    result
        .structured_content
        .as_ref()
        .unwrap_or_else(|| panic!("tool returned no structured content: {result:?}"))
        .as_object()
        .expect("structured content is a JSON object")
}

#[tokio::test]
async fn lists_every_manifest_tool_and_canonical_set() {
    let client = start_pair(fixture_repo()).await;
    let tools = client.list_tools(Option::default()).await.unwrap();
    let names: std::collections::BTreeSet<&str> =
        tools.tools.iter().map(|t| t.name.as_ref()).collect();
    let expected: std::collections::BTreeSet<&str> = tool_names().collect();

    // The served tools must be set-EQUAL to `tool_names` (the registry less
    // `CLI_ONLY_PRIMITIVES`): a name with no tool is unreachable over MCP,
    // and a served tool outside the set is one the registry, or the
    // CLI-only list, says must not be served.
    assert_eq!(names, expected, "list_tools diverged from tool_names()");

    // The shipped manifest must be set-EQUAL to the same set: a manifest
    // entry with no tool is a phantom, and a tool missing from the manifest
    // escapes the host-integration pointer lint (scripts/lint-tool-coverage.sh).
    let manifest = fs::read_to_string(workspace_root().join("framework/runtime-tools.txt"))
        .expect("framework/runtime-tools.txt must exist (canonical manifest)");
    let manifest_names: std::collections::BTreeSet<&str> = manifest
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    assert_eq!(
        manifest_names, expected,
        "framework/runtime-tools.txt diverged from the served tools"
    );
}

/// The names of the tools whose `_meta` holds [`ALWAYS_LOAD_META`] as
/// `true`, and of every tool carrying a `_meta` at all.
fn flagged_and_with_meta(
    tools: &[rmcp::model::Tool],
) -> (
    std::collections::BTreeSet<String>,
    std::collections::BTreeSet<String>,
) {
    let flagged = tools
        .iter()
        .filter(|tool| {
            tool.meta
                .as_ref()
                .is_some_and(|meta| meta.0.get(ALWAYS_LOAD_META) == Some(&Value::Bool(true)))
        })
        .map(|tool| tool.name.to_string())
        .collect();
    let with_meta = tools
        .iter()
        .filter(|tool| tool.meta.is_some())
        .map(|tool| tool.name.to_string())
        .collect();
    (flagged, with_meta)
}

/// AC21: with the generated `target.md` installed, the tools its source's
/// steps name carry `_meta["anthropic/alwaysLoad"]`, and no other tool
/// carries a `_meta`. The expected set is derived from
/// `framework/commands/target.md`, so a tool named there without the flag,
/// or a flag on one not named there, fails. Each flagged description is at
/// most 4,096 characters, Claude Code's cut for an always-loaded tool.
#[tokio::test]
async fn the_tools_targets_steps_name_are_always_loaded_and_no_other_is() {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
    fs::write(
        tmp.path().join(".ductus/config.toml"),
        "[host]\nproject = \"ductus\"\n",
    )
    .unwrap();
    let installed = tmp.path().join(".claude/commands/ductus");
    fs::create_dir_all(&installed).unwrap();
    fs::copy(
        workspace_root().join(".claude/commands/ductus/target.md"),
        installed.join("target.md"),
    )
    .unwrap();

    let source = fs::read_to_string(workspace_root().join("framework/commands/target.md")).unwrap();
    let expected: std::collections::BTreeSet<String> = text_step_tools(&source)
        .unwrap()
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert!(
        !expected.is_empty(),
        "target.md's steps name no tool, so the flag has no subject"
    );

    let client = start_pair(tmp.path().to_path_buf()).await;
    let tools = client.list_tools(Option::default()).await.unwrap().tools;
    let (flagged, with_meta) = flagged_and_with_meta(&tools);
    assert_eq!(
        flagged, expected,
        "the flagged set is target.md's step tools"
    );
    assert_eq!(with_meta, expected, "no other tool carries a _meta");
    for tool in tools
        .iter()
        .filter(|tool| flagged.contains(tool.name.as_ref()))
    {
        let chars = tool
            .description
            .as_deref()
            .unwrap_or_default()
            .chars()
            .count();
        assert!(
            chars <= 4_096,
            "{}'s description is {chars} characters, past the 4,096 an always-loaded tool keeps",
            tool.name
        );
    }
}

/// With no installed `target.md`, no tool carries a `_meta`, as before spec 080.
#[tokio::test]
async fn with_no_installed_target_no_tool_carries_meta() {
    let client = start_pair(fixture_repo()).await;
    let tools = client.list_tools(Option::default()).await.unwrap().tools;
    let (_, with_meta) = flagged_and_with_meta(&tools);
    assert!(with_meta.is_empty(), "{with_meta:?}");
}

/// With no installed `target.md`, the server says so on stderr when it starts,
/// naming each path it tried, so flagging no tool is never silent (AC21). The
/// server is the binary, run with its stdin closed, so it ends once it has
/// started; a deadline fails the test rather than hang it.
#[test]
fn with_no_installed_target_the_server_says_so_on_stderr() {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
    fs::write(
        tmp.path().join(".ductus/config.toml"),
        "[host]\nproject = \"acme\"\n",
    )
    .unwrap();
    let mut child = common::ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .arg("mcp")
        .current_dir(tmp.path())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while child.try_wait().unwrap().is_none() {
        if std::time::Instant::now() > deadline {
            let _ = child.kill();
            panic!("`ductus mcp` did not end within 30 s of its stdin closing");
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ductus: no installed `target` command at "),
        "{stderr}"
    );
    assert!(
        stderr.contains(".claude/commands/acme/target.md"),
        "{stderr}"
    );
}

/// What `ductus <subcommand> --help` prints above its usage: its `long_about`.
fn long_about(subcommand: &str) -> String {
    let output = common::ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args([subcommand, "--help"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{subcommand} --help failed");
    let help = String::from_utf8(output.stdout).unwrap();
    let (long_about, _) = help
        .split_once("\n\nUsage:")
        .unwrap_or_else(|| panic!("no usage in {subcommand} --help: {help}"));
    long_about.to_string()
}

/// One description per primitive (spec 079, AC9): each served tool's text and its
/// subcommand's `--help` are the same text, so neither can drift from the other.
#[tokio::test]
async fn each_served_tools_description_is_its_subcommands_long_about() {
    let client = start_pair(fixture_repo()).await;
    let tools = client.list_tools(Option::default()).await.unwrap();
    assert_eq!(tools.tools.len(), tool_names().count());
    for tool in &tools.tools {
        let description = tool.description.as_deref().unwrap_or_default();
        assert!(!description.is_empty(), "{} has no description", tool.name);
        assert_eq!(long_about(&tool.name), description, "{}", tool.name);
    }
    client.cancel().await.unwrap();
}

/// The one primitive the server does not serve still takes its text from the same
/// place.
#[test]
fn the_cli_only_primitive_takes_its_description_from_the_same_place() {
    assert_eq!(CLI_ONLY_PRIMITIVES, ["run-generator"]);
    assert_eq!(long_about("run-generator"), descriptions::RUN_GENERATOR);
}

/// A literal would pass the test above while a second copy of the text sat beside
/// the constant it should have named.
#[test]
fn no_tool_spells_its_description_as_a_literal() {
    let server = include_str!("../src/mcp/server.rs");
    assert_eq!(server.matches("description = \"").count(), 0);
    assert_eq!(
        server.matches("description = descriptions::").count(),
        tool_names().count()
    );
}

/// A spec's data model or scenario points at a primitive's description for what it
/// states (079 task 36), so each detail those passages used to carry, and that a
/// description once lacked, must be there to point at.
#[test]
fn descriptions_hold_the_detail_specs_point_at() {
    let held = [
        (descriptions::RESOLVE_CONSTITUTIONS, "`examined`"),
        (descriptions::RELOCATE_AUDIT_RECORDS, "`disagreements`"),
        (
            descriptions::WRITE_SESSION,
            "`clear` removes the target block",
        ),
        (descriptions::WRITE_SESSION, "`last-used`"),
        (descriptions::RESOLVE_SESSION, "`consolidated`"),
        (descriptions::RETARGET_SESSIONS, "`cause`"),
        (descriptions::DASHBOARD, "`fold-target-missing`"),
        (
            descriptions::CHECK_PRESCRIPTIONS,
            "no round.toml was written",
        ),
        (descriptions::CHECK_ARTIFACT_SIZE, "named on stderr"),
        (descriptions::READ_DEPRECATION_LABELS, "labels both rules"),
        (descriptions::CLASSIFY_SPEC_EDIT, "absent on either side"),
        (descriptions::APPEND_TASK, "keyed on that whole pointer"),
        (descriptions::APPEND_TASK, "`dedup-title`, which is opt-in"),
        (descriptions::WRITE_SESSION, "cannot be proven idle"),
        (descriptions::WRITE_REVIEW, "this run's own earlier write"),
        (
            descriptions::REWRITE_SPEC_LINKS,
            "a skipped referrer is not counted",
        ),
        (descriptions::START_FIX_ROUNDS, "a snapshot of nothing"),
        (descriptions::BRIEF_RE_READ, "data model and scenarios"),
        (descriptions::BRIEF_RE_READ, "never read for a newer brief"),
        (descriptions::BRIEF_RE_READ, "excluded-count stay inline"),
        (descriptions::CHECK_ARTIFACT_SIZE, "each unmeasurable one"),
        (descriptions::CREATE_PLAN_ARTIFACTS, "with no `template`"),
        (descriptions::DASHBOARD, "`scenario-open-question-count`"),
        (descriptions::RESOLVE_SESSION, "`identity` is null"),
        (descriptions::RESOLVE_SESSION, "`co-target`"),
        (
            descriptions::PRUNE_TASKS,
            "writes the counter when it removes a task section",
        ),
    ];
    for (description, detail) in held {
        assert!(description.contains(detail), "missing {detail}");
    }
    assert!(!descriptions::RELOCATE_AUDIT_RECORDS.contains("never overwrites"));
    assert!(!descriptions::CREATE_PLAN_ARTIFACTS.contains("action, template"));
    assert!(!descriptions::PRUNE_TASKS.contains("so a reset file with no counter yields task 1"));
}

/// The same, for the details 079 task 36's second sweep moved from the 051, 063,
/// 067, 078 and 079 data models and the descriptions' own truth-checks added.
#[test]
fn descriptions_hold_the_detail_the_second_sweep_points_at() {
    let held = [
        (
            descriptions::WRITE_SESSION,
            "passed through unchanged and does not exclude the peer",
        ),
        (descriptions::PROCESS_WAIVERS, "a deleted file's included"),
        (
            descriptions::PROCESS_WAIVERS,
            "a file the plan names counts as read",
        ),
        (
            descriptions::REWRITE_SPEC_LINKS,
            "no scenario tail is listed on that account",
        ),
        (
            descriptions::RESOLVE_ANCHOR,
            "takes precedence over anchor-target-unjudgeable",
        ),
        (descriptions::BRIEF_RE_READ, "`empty` returns none"),
        (
            descriptions::BRIEF_RE_READ,
            ".ductus/results/brief-re-read/{feature}/",
        ),
        (descriptions::READ_RE_READ, "as `unprescribed`"),
        (
            descriptions::READ_RE_READ,
            ".ductus/results/read-re-read/{feature}/",
        ),
        (
            descriptions::CHECK_PRESCRIPTIONS,
            "a checkout that converts line endings on disk compares alike",
        ),
        (
            descriptions::CHECK_PRESCRIPTIONS,
            "`{finding, edit, path, occurrences}`",
        ),
        (
            descriptions::CHECK_PRESCRIPTIONS,
            "a file the snapshot does not hold as text has none",
        ),
        (
            descriptions::CHECK_PRESCRIPTIONS,
            "by the tree-to-tree diff brief-re-read uses",
        ),
        (
            descriptions::CHECK_PRESCRIPTIONS,
            ".ductus/results/check-prescriptions/{feature}/",
        ),
        (
            descriptions::WRITE_REVIEW,
            "`ductus exec`, with no loop, never does",
        ),
        (
            descriptions::WRITE_REVIEW,
            "a waiver is then recorded `decided-by: rule`",
        ),
        (
            descriptions::WRITE_REVIEW,
            "a `ductus exec` review pass that judges an observation a broken contract",
        ),
        (
            descriptions::DERIVE_DEPENDENCIES,
            "that `staged` left unwritten",
        ),
        (
            descriptions::DERIVE_DEPENDENCIES,
            "the cycles sorted by least member",
        ),
        (
            descriptions::DERIVE_DEPENDENCIES,
            "a file with no frontmatter at all is not unparseable",
        ),
        (
            descriptions::DERIVE_REFERENCES,
            "a file with no frontmatter at all is not unparseable",
        ),
        (
            descriptions::DERIVE_REFERENCES,
            "only when `unparseable` and `unwritable` are also empty",
        ),
        (
            descriptions::CHECK_ORPHANED_REFERENCES,
            "the latest in registry order winning",
        ),
        (
            descriptions::CHECK_ORPHANED_REFERENCES,
            "which the result does not distinguish",
        ),
        (
            descriptions::CHECK_COMMAND_FLAGS,
            "command files were examined and none holds a Flags table",
        ),
    ];
    for (description, detail) in held {
        assert!(description.contains(detail), "missing {detail}");
    }
}

/// 078's and 079's data models point at these for what a fix round's lists carry.
#[test]
fn descriptions_hold_the_fix_round_details_the_data_models_point_at() {
    let held = [
        (descriptions::READ_RE_READ, "stays in `findings` as one"),
        (
            descriptions::WRITE_REVIEW,
            "is the `excluded` list of that round's brief-re-read result",
        ),
    ];
    for (description, detail) in held {
        assert!(description.contains(detail), "missing {detail}");
    }
}

/// The manifest names each primitive the server does not serve on a
/// `# cli-only:` line, which `/audit`'s Family 9 reads, and the set must be
/// `CLI_ONLY_PRIMITIVES` exactly.
#[test]
fn the_manifest_names_the_cli_only_primitives() {
    let manifest = fs::read_to_string(workspace_root().join("framework/runtime-tools.txt"))
        .expect("framework/runtime-tools.txt must exist (canonical manifest)");
    let named: std::collections::BTreeSet<&str> = manifest
        .lines()
        .filter_map(|l| l.strip_prefix("# cli-only:"))
        .map(str::trim)
        .collect();
    let expected: std::collections::BTreeSet<&str> = ductus::schema::registry::CLI_ONLY_PRIMITIVES
        .iter()
        .copied()
        .collect();
    assert_eq!(
        named, expected,
        "`# cli-only:` lines diverged from CLI_ONLY_PRIMITIVES"
    );
}

/// The handshake names this runtime, not the SDK it is built on. rmcp fills
/// `serverInfo` from its own build environment unless the server sets it, so
/// a host was told it was talking to `rmcp` at the SDK's version.
#[tokio::test]
async fn handshake_names_the_runtime_and_its_version() {
    let client = start_pair(fixture_repo()).await;
    let info = client
        .peer_info()
        .expect("the handshake recorded the server's info");
    let server = info
        .server_info
        .as_ref()
        .expect("the server reported its implementation");
    assert_eq!(server.name, "ductus");
    assert_eq!(server.version, env!("CARGO_PKG_VERSION"));
}

/// `schemars` stamps OpenAPI-style numeric `format` hints (`uint32`,
/// `uint64`, `uint8`, …) onto Rust integer/float fields. JSON Schema
/// defines no `format` for numeric types, so strict MCP clients (Ajv-based
/// validators) log `unknown format "uint32" ignored` for each one. The server strips
/// them at construction; assert none survive on any served tool schema.
#[tokio::test]
async fn no_tool_schema_carries_a_nonstandard_numeric_format() {
    fn collect_formats(value: &Value, out: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                if let Some(Value::String(fmt)) = map.get("format") {
                    out.push(fmt.clone());
                }
                for child in map.values() {
                    collect_formats(child, out);
                }
            }
            Value::Array(items) => items.iter().for_each(|v| collect_formats(v, out)),
            _ => {}
        }
    }

    let is_numeric_format =
        |f: &str| f.starts_with("int") || f.starts_with("uint") || matches!(f, "float" | "double");

    let client = start_pair(fixture_repo()).await;
    let tools = client.list_tools(Option::default()).await.unwrap();

    for tool in &tools.tools {
        let mut formats = Vec::new();
        collect_formats(&Value::Object((*tool.input_schema).clone()), &mut formats);
        if let Some(output) = &tool.output_schema {
            collect_formats(&Value::Object((**output).clone()), &mut formats);
        }
        let offenders: Vec<&String> = formats.iter().filter(|f| is_numeric_format(f)).collect();
        assert!(
            offenders.is_empty(),
            "tool {} exposes non-standard numeric format(s): {offenders:?}",
            tool.name
        );
    }
}

#[tokio::test]
async fn read_spec_returns_structured_frontmatter() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "read-spec",
        json!({"feature": "001-basic", "include-body": false}),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["frontmatter"]["status"], "clarified");
    assert_eq!(obj["path"], "specs/001-basic/spec.md");
}

/// Task 65 (scenarios/mcp-arg-unknown-field-strictness.md), MCP surface: an
/// argument key outside the primitive's known field set — here the
/// `snake_case` misspelling `include_body` for the kebab `include-body` — is
/// rejected with an error naming the unknown field, rather than silently
/// dropped and run with that field's default. The exec-surface half of the
/// contract (the subprocess interpreter's superset-context binding stays
/// lenient) is asserted by
/// `interpreter::tests::exec_path_ignores_unknown_argument_key`.
#[tokio::test]
async fn mcp_surface_rejects_unknown_argument_field() {
    let client = start_pair(fixture_repo()).await;
    let args = json!({"feature": "001-basic", "include_body": false})
        .as_object()
        .cloned()
        .unwrap();
    let err = client
        .call_tool(CallToolRequestParams::new("read-spec".to_string()).with_arguments(args))
        .await
        .expect_err("a misspelled kebab field must be rejected, not silently dropped");
    let rendered = format!("{err:?}");
    assert!(
        rendered.contains("include_body"),
        "rejection should name the unknown field, got: {rendered}"
    );
}

#[tokio::test]
async fn read_tasks_returns_task_list() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(&client, "read-tasks", json!({"feature": "001-basic"})).await;
    let obj = structured_object(&result);
    let tasks = obj["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0]["number"], "1");
}

#[tokio::test]
async fn validate_frontmatter_reports_clean_fixture() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "validate-frontmatter",
        json!({"path": "specs/001-basic/spec.md"}),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["clean"], true);
    assert_eq!(obj["findings"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn resolve_anchor_returns_references() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "resolve-anchor",
        json!({"path": "framework/constitution.md"}),
    )
    .await;
    let obj = structured_object(&result);
    assert!(obj["references"].is_array());
}

#[tokio::test]
async fn traverse_deps_returns_compatibility() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "traverse-deps",
        json!({"feature": "002-dependent"}),
    )
    .await;
    let obj = structured_object(&result);
    assert!(obj["compatible"].is_boolean());
    // Acyclic fixture must carry an empty `cycles` array — confirms the
    // schema field is wired through the MCP surface and stays empty when
    // the reachable subgraph has no SCCs.
    assert!(obj["cycles"].is_array());
    assert_eq!(obj["cycles"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn traverse_deps_surfaces_two_cycle_via_mcp() {
    // Parity coverage for spec 067's `traverse-deps-cycle-check` scenario:
    // the MCP surface (called by every slash command host) returns the same `cycles` payload the
    // primitive's unit tests produce. Asserts the end-to-end wiring from
    // the rmcp tool handler through to the response envelope.
    let tmp = tempfile::tempdir().unwrap();
    let specs = tmp.path().join("specs");
    fs::create_dir_all(&specs).unwrap();
    for (slug, dep) in [("200-a", "201-b"), ("201-b", "200-a")] {
        let dir = specs.join(slug);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\nstatus: planned\ndependencies: [{dep}]\n---\n\n# {slug}\n"),
        )
        .unwrap();
    }

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(&client, "traverse-deps", json!({"feature": "200-a"})).await;
    let obj = structured_object(&result);
    assert_eq!(obj["compatible"], false, "cycle flips compatible to false");
    let cycles = obj["cycles"].as_array().expect("cycles array").clone();
    assert_eq!(cycles.len(), 1, "exactly one SCC for the two-node cycle");
    let slugs: Vec<String> = cycles[0]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert!(slugs.contains(&"200-a".to_string()));
    assert!(slugs.contains(&"201-b".to_string()));
}

#[tokio::test]
async fn check_rule_ids_returns_citations() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "check-rule-ids",
        json!({
            "path": "specs/001-basic/spec.md",
            "rule-files": ["framework/rules/security-backend.md"],
        }),
    )
    .await;
    let obj = structured_object(&result);
    assert!(obj["citations"].is_array());
}

#[tokio::test]
async fn resolve_feature_resolves_number_against_fixture() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(&client, "resolve-feature", json!({"identifier": "1"})).await;
    let obj = structured_object(&result);
    assert_eq!(obj["outcome"], "resolved");
    assert_eq!(obj["feature"], "001-basic");
    assert_eq!(obj["path"], "specs/001-basic");
    assert_eq!(obj["status"], "clarified");
}

#[tokio::test]
async fn resolve_feature_reports_not_found_as_domain_outcome() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "resolve-feature",
        json!({"identifier": "no-such-slug"}),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["outcome"], "not-found");
    assert_eq!(obj["candidates"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn check_artifacts_reports_the_fixture_s_one_open_question() {
    // 001-basic is `clarified` with a well-formed tasks.md and no scenarios,
    // and keeps one open question on purpose, for the parser tests that read
    // it: the one finding is spec-integrity's, a clarified spec holding a
    // question, and every other family passes.
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(&client, "check-artifacts", json!({"feature": "001-basic"})).await;
    let obj = structured_object(&result);
    assert_eq!(obj["clean"], false);
    assert_eq!(obj["status"], "clarified");
    let findings = obj["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0]["family"], "spec-integrity");
    assert_eq!(findings[0]["severity"], "blocking");
    assert_eq!(obj["path"], "specs/001-basic/spec.md");
}

#[tokio::test]
async fn check_artifacts_flags_planned_spec_missing_artifacts() {
    // 002-dependent is `planned` with no plan.md / tasks.md → two
    // blocking artifact-completeness findings.
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "check-artifacts",
        json!({"feature": "002-dependent"}),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["clean"], false);
    let findings = obj["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 2, "{findings:?}");
    for finding in findings {
        assert_eq!(finding["family"], "artifact-completeness");
        assert_eq!(finding["severity"], "blocking");
    }
}

#[tokio::test]
async fn gate_confirm_returns_prompt_payload_without_blocking() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "gate-confirm",
        json!({"gate": "plan-finalize-status", "prompt": "Advance status?"}),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["gate"], "plan-finalize-status");
    assert_eq!(obj["prompt"], "Advance status?");
    assert!(
        obj["request-id"].as_str().unwrap().starts_with("gate-"),
        "request-id present and prefixed"
    );
}

#[tokio::test]
async fn mark_task_against_scratch_copy_flips_checkbox() {
    let tmp = tempfile::tempdir().unwrap();
    copy_recursively(&fixture_repo(), tmp.path());
    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(
        &client,
        "mark-task",
        json!({
            "feature": "001-basic",
            "task-number": "1",
            "subtask-index": 1,
            "checked": true,
        }),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["current"], true);
    let body = fs::read_to_string(tmp.path().join("specs/001-basic/tasks.md")).unwrap();
    assert!(body.contains("- [x] Subtask two — pending."));
}

#[tokio::test]
async fn mark_criterion_against_scratch_copy_flips_checkbox() {
    let tmp = tempfile::tempdir().unwrap();
    copy_recursively(&fixture_repo(), tmp.path());
    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(
        &client,
        "mark-criterion",
        json!({
            "feature": "001-basic",
            "criterion-index": 0,
            "checked": true,
        }),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["current"], true);
    assert_eq!(obj["previous"], false);
}

/// Build a tempdir copy of the fixture and `git init` + `git commit -m init`
/// against it so primitives that walk git history (`check-stuck`,
/// `derive-boundary`) have a real repo to operate on.
fn init_git_fixture() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    copy_recursively(&fixture_repo(), tmp.path());
    let repo = git2::Repository::init(tmp.path()).unwrap();
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[])
        .unwrap();
    tmp
}

#[tokio::test]
async fn check_stuck_returns_commit_count() {
    let tmp = init_git_fixture();
    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(
        &client,
        "check-stuck",
        json!({"feature": "001-basic", "threshold": 3}),
    )
    .await;
    let obj = structured_object(&result);
    assert!(obj["commit-count"].is_number());
    assert_eq!(obj["threshold"], 3);
}

#[tokio::test]
async fn derive_boundary_returns_diff_paths() {
    let tmp = init_git_fixture();
    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(&client, "derive-boundary", json!({"feature": "001-basic"})).await;
    let obj = structured_object(&result);
    assert!(obj["boundary"].is_array());
    assert!(obj["first-commit"].is_string());
}

/// `run-generator` hands any project file to `bash`, so the server does not
/// serve it (022 task 14): it is absent from `list_tools`, and a call to it
/// is refused rather than run.
#[cfg(unix)]
#[tokio::test]
async fn run_generator_is_not_an_mcp_tool() {
    let tmp = tempfile::tempdir().unwrap();
    let marker = tmp.path().join("ran");
    fs::write(
        tmp.path().join("gen.sh"),
        format!("touch '{}'\n", marker.display()),
    )
    .unwrap();
    let client = start_pair(tmp.path().to_path_buf()).await;
    let tools = client.list_tools(Option::default()).await.unwrap();
    assert!(
        !tools.tools.iter().any(|t| t.name == "run-generator"),
        "run-generator is served"
    );
    let args = json!({"script": "gen.sh"}).as_object().cloned().unwrap();
    let outcome = client
        .call_tool(CallToolRequestParams::new("run-generator".to_string()).with_arguments(args))
        .await;
    assert!(
        outcome.is_err() || outcome.as_ref().is_ok_and(|r| r.is_error == Some(true)),
        "a call to run-generator was not refused: {outcome:?}"
    );
    assert!(!marker.exists(), "the script ran");
}

#[tokio::test]
async fn lint_markdown_returns_violations_array() {
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(
        &client,
        "lint-markdown",
        json!({"paths": ["specs/001-basic/spec.md"], "fix": false}),
    )
    .await;
    let obj = structured_object(&result);
    // The fixture spec may or may not be clean depending on the local
    // markdownlint config; assert only the response shape is valid.
    assert!(obj["violations"].is_array());
    assert!(obj["clean"].is_boolean());
    assert!(obj["exit-code"].is_number());
}

#[tokio::test]
async fn set_status_against_scratch_copy_updates_field() {
    let tmp = tempfile::tempdir().unwrap();
    copy_recursively(&fixture_repo(), tmp.path());
    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(
        &client,
        "set-status",
        json!({
            "feature": "001-basic",
            "from": "clarified",
            "to": "planned",
        }),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["previous"], "clarified");
    assert_eq!(obj["current"], "planned");
}

// -- review-runtime-acceleration primitives (spec 022 scenario) --------------

#[tokio::test]
async fn discover_rule_files_selects_via_mcp() {
    // The shared fixture repo ships framework/rules/security-backend.md.
    let client = start_pair(fixture_repo()).await;
    let result = call_tool(&client, "discover-rule-files", json!({})).await;
    let obj = structured_object(&result);
    let selected: Vec<&str> = obj["selected"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(
        selected.iter().any(|s| s.contains("security-backend")),
        "expected security-backend.md in {selected:?}"
    );
    assert!(obj["notices"].is_array());
}

#[tokio::test]
async fn process_waivers_applies_via_mcp() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("specs/001-x");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
    )
    .unwrap();
    // Waivers live in `review.md`'s frontmatter (spec 057 task 5).
    fs::write(
        dir.join("review.md"),
        "---\nspec: 001-x\nlast-run: 2026-01-01T00:00:00Z\nblocking: false\nwaivers:\n  \
         - rule: SEC-BE-014\n    file: src/x.ts\n    reason: internal-only endpoint behind mTLS\n    \
         waived-at: 2026-01-01T00:00:00Z\n    waived-by: dev@example.com\n---\n\n# Review — 001-x\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::write(tmp.path().join("src/x.ts"), "code\n").unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(
        &client,
        "process-waivers",
        json!({
            "feature": "001-x",
            "fired": [{"rule": "SEC-BE-014", "file": "src/x.ts"}],
        }),
    )
    .await;
    let obj = structured_object(&result);
    let applied = obj["applied"].as_array().unwrap();
    assert_eq!(applied.len(), 1);
    assert_eq!(applied[0]["rule"], "SEC-BE-014");
    assert_eq!(
        obj["expired"].as_array().unwrap().as_slice(),
        [] as [Value; 0]
    );
}

/// `process-decisions` answers over MCP with the kebab-case wire names the
/// commands read: `record`, `fired`, `restricted` in, and `matched` /
/// `lapsed` / `expired` / `retained` / `notices` out, each decision carrying
/// its `key`, `outcome`, and companion (specs 058 and 076).
#[tokio::test]
async fn process_decisions_classifies_via_mcp() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("specs/001-x");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
    )
    .unwrap();
    fs::write(
        dir.join("analysis.md"),
        "---\nspec: 001-x\nlast-run: 2026-09-25T00:00:00Z\ndecisions:\n  \
         - key: \"grounding — cites a missing path\"\n    outcome: deferred\n    \
         target: specs/001-x/tasks.md\n    decided-at: 2026-09-25T00:00:00Z\n    \
         decided-by: dev@example.com\n  \
         - key: \"applicable-rules — BE-AUTHN-001 does not fire\"\n    outcome: waived\n    \
         reason: cited for a future endpoint\n    decided-at: 2026-09-25T00:00:00Z\n    \
         decided-by: dev@example.com\n---\n\n# Analysis — 001-x\n",
    )
    .unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(
        &client,
        "process-decisions",
        json!({
            "feature": "001-x",
            "record": "analysis",
            "fired": ["grounding — cites a missing path"],
            "restricted": false,
        }),
    )
    .await;
    let obj = structured_object(&result);
    let matched = obj["matched"].as_array().unwrap();
    assert_eq!(matched.len(), 1);
    assert_eq!(matched[0]["key"], "grounding — cites a missing path");
    assert_eq!(matched[0]["outcome"], "deferred");
    assert_eq!(matched[0]["target"], "specs/001-x/tasks.md");
    assert_eq!(
        obj["lapsed"].as_array().unwrap().as_slice(),
        [] as [Value; 0],
        "a deferral to work still in progress is matched, not lapsed"
    );
    let expired = obj["expired"].as_array().unwrap();
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0]["outcome"], "waived");
    assert_eq!(expired[0]["reason"], "cited for a future endpoint");
    assert_eq!(
        obj["retained"].as_array().unwrap().as_slice(),
        [] as [Value; 0]
    );
    assert_ne!(
        obj["notices"].as_array().unwrap().as_slice(),
        [] as [Value; 0]
    );
}

#[tokio::test]
async fn compute_review_scope_returns_structured_scope_via_mcp() {
    use git2::{IndexAddOption, Repository, Signature};
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::init(tmp.path()).unwrap();
    let dir = tmp.path().join("specs/001-x");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("spec.md"),
        "---\nstatus: planned\ndependencies: []\n---\n\n# x\n",
    )
    .unwrap();
    let mut index = repo.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[])
        .unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(&client, "compute-review-scope", json!({"feature": "001-x"})).await;
    let obj = structured_object(&result);
    // Never reached in-progress → empty diff-base and scope, but the wire
    // returns the structured shape.
    assert!(obj["diff-base"].is_string());
    assert!(obj["scope"].is_array());
    assert_eq!(obj["form"], "inline");
    assert!(
        obj.get("captured-issues").is_none(),
        "the inbox window left this result (spec 058)"
    );
}

/// The `[artifacts] read-size-bytes` both bounded tests set: small enough
/// that twelve paths pass it inline, large enough for the file form.
const SMALL_READ_SIZE: usize = 400;

/// A repository whose `001-x` went in progress and then gained `n` source
/// files, and whose sibling `002-y` gained `n` scenarios, with `[artifacts]
/// read-size-bytes` set to [`SMALL_READ_SIZE`] (066 scenario
/// `scope-results-have-a-bounded-size`).
fn over_the_read_size(n: usize) -> tempfile::TempDir {
    use git2::{IndexAddOption, Repository, Signature};
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::init(tmp.path()).unwrap();
    let commit = |message: &str| {
        let mut index = repo.index().unwrap();
        index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
        index.write().unwrap();
        let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
        let sig = Signature::now("Test", "test@example.com").unwrap();
        let parent = repo.head().ok().map(|head| head.peel_to_commit().unwrap());
        let parents: Vec<&git2::Commit> = parent.iter().collect();
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
            .unwrap();
    };
    let write = |rel: &str, body: &str| {
        let path = tmp.path().join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    };
    let spec = |status: &str| format!("---\nstatus: {status}\ndependencies: []\n---\n\n# X\n");
    write("specs/001-x/spec.md", &spec("planned"));
    write("specs/002-y/spec.md", &spec("planned"));
    commit("plan");
    write("specs/001-x/spec.md", &spec("in-progress"));
    commit("begin");
    for i in 0..n {
        write(&format!("src/f{i}.rs"), &format!("fn f{i}() {{}}\n"));
        write(&format!("specs/002-y/scenarios/s{i}.md"), "x\n");
    }
    commit("feat(001): work");
    write(
        ".ductus/config.toml",
        &format!("[artifacts]\nread-size-bytes = {SMALL_READ_SIZE}\n"),
    );
    tmp
}

/// The lists in the file a file-form result names, after checking the
/// result itself is within the read size and carries `inline_keys` nowhere.
fn lists_in_file(
    tmp: &tempfile::TempDir,
    obj: &serde_json::Map<String, Value>,
    inline_keys: &[&str],
) -> Value {
    assert_eq!(obj["form"], "file", "the result stayed inline: {obj:?}");
    for key in inline_keys {
        assert!(obj.get(*key).is_none(), "`{key}` was inline: {obj:?}");
    }
    let size = serde_json::to_vec(obj).unwrap().len();
    assert!(size <= SMALL_READ_SIZE, "{size} bytes: {obj:?}");
    let file = obj["lists-file"].as_str().expect("a lists-file path");
    serde_json::from_str(&fs::read_to_string(tmp.path().join(file)).unwrap()).unwrap()
}

/// On the MCP path a scope past the read size reaches the host whole: the
/// result is within the bound, and the file it names holds every in-scope
/// path (066 scenario `scope-results-have-a-bounded-size`).
#[tokio::test]
async fn a_scope_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp() {
    let tmp = over_the_read_size(12);
    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(&client, "compute-review-scope", json!({"feature": "001-x"})).await;
    let obj = structured_object(&result);
    let lists = lists_in_file(&tmp, obj, &["scope", "modified-since", "plan-affected"]);
    let scope = lists["scope"].as_array().unwrap();
    assert_eq!(obj["scope-count"], scope.len());
    assert_eq!(
        obj["modified-since-count"],
        lists["modified-since"].as_array().unwrap().len()
    );
    for i in 0..12 {
        let path = json!(format!("src/f{i}.rs"));
        assert!(scope.contains(&path), "{path} missing: {scope:?}");
    }
}

/// On the MCP path a cross-spec surface past the read size reaches the host
/// whole: the result is within the bound, and the file it names holds every
/// sibling path (066 scenario `scope-results-have-a-bounded-size`).
#[tokio::test]
async fn a_cross_spec_surface_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp() {
    let tmp = over_the_read_size(12);
    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(&client, "diff-cross-spec", json!({"feature": "001-x"})).await;
    let obj = structured_object(&result);
    let lists = lists_in_file(&tmp, obj, &["cross-spec-paths"]);
    // `002-y/spec.md` is in 001-x's first commit, so it is not a change.
    let mut expected: Vec<String> = (0..12)
        .map(|i| format!("specs/002-y/scenarios/s{i}.md"))
        .collect();
    expected.sort();
    assert_eq!(lists["cross-spec-paths"], json!(expected));
    assert_eq!(obj["cross-spec-count"], expected.len());
}

/// On the MCP path a classification past the read size reaches the host
/// whole: the result is within the bound, its counts are inline, and the file
/// it names holds every expired decision the host passes to the writer and
/// every notice it emits (066 scenario
/// `process-decisions-results-have-a-bounded-size`).
#[tokio::test]
async fn a_decisions_result_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("specs/001-x");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
    )
    .unwrap();
    let decisions = (0..12)
        .map(|i| {
            format!(
                "  - key: \"grounding — finding {i}\"\n    outcome: waived\n    \
                 reason: noise\n    decided-at: 2026-09-25T00:00:00Z\n    \
                 decided-by: dev@example.com\n"
            )
        })
        .collect::<Vec<_>>()
        .concat();
    fs::write(
        dir.join("analysis.md"),
        format!("---\nspec: 001-x\ndecisions:\n{decisions}---\n\n# Analysis — 001-x\n"),
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
    fs::write(
        tmp.path().join(".ductus/config.toml"),
        format!("[artifacts]\nread-size-bytes = {SMALL_READ_SIZE}\n"),
    )
    .unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(
        &client,
        "process-decisions",
        json!({"feature": "001-x", "record": "analysis", "fired": [], "restricted": false}),
    )
    .await;
    let obj = structured_object(&result);
    let lists = lists_in_file(
        &tmp,
        obj,
        &["matched", "expired", "retained", "lapsed", "notices"],
    );
    let expired: Vec<&str> = lists["expired"]
        .as_array()
        .unwrap()
        .iter()
        .map(|decision| decision["key"].as_str().unwrap())
        .collect();
    let expected: Vec<String> = (0..12)
        .map(|i| format!("grounding — finding {i}"))
        .collect();
    assert_eq!(expired, expected);
    assert_eq!(obj["expired-count"], 12);
    assert_eq!(obj["notices-count"], 12);
    assert_eq!(lists["notices"].as_array().unwrap().len(), 12);
    for key in ["matched", "retained", "lapsed"] {
        assert_eq!(lists[key], json!([]), "{key}");
        assert_eq!(obj[format!("{key}-count").as_str()], 0, "{key}");
    }
}

/// Vendor a `markdownlint-cli2` that reports three violations and exits 1, so
/// a lint, and the review gate's lint check, return a list to move.
fn vendor_a_violating_markdownlint(root: &Path) {
    use common::corpus::slug;
    let bin = root.join("node_modules").join(".bin");
    fs::create_dir_all(&bin).unwrap();
    let lines = [
        format!(
            "specs/{}/spec.md:1 MD041/first-line-heading First line in a file should be a top-level heading",
            slug(1)
        ),
        format!(
            "specs/{}/tasks.md:2 MD012/no-multiple-blanks Multiple consecutive blank lines",
            slug(1)
        ),
        format!(
            "specs/{}/spec.md:3 MD009/no-trailing-spaces Trailing spaces",
            slug(2)
        ),
    ];
    #[cfg(windows)]
    {
        let mut body = String::new();
        for line in &lines {
            body.push_str("@echo ");
            body.push_str(line);
            body.push_str("\r\n");
        }
        fs::write(
            bin.join("markdownlint-cli2.cmd"),
            format!("{body}@exit /b 1\r\n"),
        )
        .unwrap();
    }
    #[cfg(not(windows))]
    {
        let mut body = String::new();
        for line in &lines {
            body.push_str("echo '");
            body.push_str(line);
            body.push_str("'\n");
        }
        let stub = bin.join("markdownlint-cli2");
        fs::write(&stub, format!("#!/bin/sh\n{body}exit 1\n")).unwrap();
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&stub).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&stub, perms).unwrap();
        }
    }
}

/// AC7's population (spec 080): each primitive `research.md` measures as
/// growing with the corpus, with the arguments it is called with on the
/// generated corpus and the lists its file form moves. A primitive with an
/// argument mode that grows is a row per mode.
fn bounded_members() -> Vec<(&'static str, Value, &'static [&'static str])> {
    use common::corpus::{SUBJECT, slug};
    let feature = json!({ "feature": SUBJECT });
    vec![
        (
            "read-spec",
            json!({ "feature": SUBJECT, "include-body": true }),
            &[
                "sections",
                "acceptance-criteria",
                "open-questions",
                "scenario-open-questions",
                "scenario-files-unreadable",
            ],
        ),
        ("read-tasks", feature.clone(), &["tasks"]),
        (
            "read-section",
            json!({ "path": format!("specs/{SUBJECT}/spec.md"), "headings": ["Acceptance Criteria"] }),
            &["sections"],
        ),
        ("derive-boundary", feature.clone(), &["boundary"]),
        (
            "lint-markdown",
            json!({ "paths": ["specs"] }),
            &["violations"],
        ),
        ("prune-tasks", feature.clone(), &["sections"]),
        (
            "prune-tasks",
            json!({ "all": true }),
            &["features", "skipped"],
        ),
        (
            "prune-plan",
            json!({ "all": true }),
            &["features", "skipped"],
        ),
        ("dashboard", json!({}), &["specs", "tags-union"]),
        (
            "check-review-gate",
            feature.clone(),
            &["violations", "cross-spec-impact"],
        ),
        ("check-artifacts", feature, &["findings", "skipped"]),
        (
            "check-corpus-links",
            json!({}),
            &["broken", "examined", "skipped"],
        ),
        (
            "check-corpus-links",
            json!({ "scope": "repository" }),
            &["broken", "examined", "skipped"],
        ),
        ("check-unfolded-specs", json!({}), &["unfolded"]),
        (
            "rewrite-spec-links",
            json!({ "from": slug(2), "to": SUBJECT }),
            &["rewritten", "skipped", "kept-tails", "mentions"],
        ),
        (
            "derive-dependencies",
            json!({}),
            &[
                "updated",
                "unwritten",
                "untracked-skipped",
                "absent",
                "cycles",
                "unparseable",
            ],
        ),
        (
            "derive-routing-candidates",
            json!({ "description": "handle a corpus case" }),
            &["candidates"],
        ),
        (
            "traverse-deps",
            json!({ "feature": slug(4) }),
            &["dependencies", "cycles"],
        ),
    ]
}

/// Drift spec 3's frontmatter from its body, which still links spec 2, so
/// `derive-dependencies` has a drifted spec to list.
fn drift_one_spec(root: &Path) {
    use common::corpus::slug;
    let path = root.join("specs").join(slug(3)).join("spec.md");
    let text = fs::read_to_string(&path).unwrap();
    let drifted = text.replace(&format!("dependencies: [{}]", slug(2)), "dependencies: []");
    assert_ne!(drifted, text, "spec 3 declared its dependency");
    fs::write(path, drifted).unwrap();
}

/// `name` called with `args` over MCP on a fresh copy of `template`, with
/// `[artifacts] read-size-bytes` set to `read_size` when given: the result,
/// and the copy it ran on, which holds any list file it wrote.
async fn call_on_a_copy(
    template: &Path,
    name: &str,
    args: &Value,
    read_size: Option<usize>,
) -> (serde_json::Map<String, Value>, tempfile::TempDir) {
    let copy = tempfile::tempdir().unwrap();
    common::copy_dir_recursive(template, copy.path());
    if let Some(bytes) = read_size {
        fs::write(
            copy.path().join(".ductus/config.toml"),
            format!("[project]\nname = \"corpus\"\n\n[artifacts]\nread-size-bytes = {bytes}\n"),
        )
        .unwrap();
    }
    let client = start_pair(copy.path().to_path_buf()).await;
    let result = call_tool(&client, name, args.clone()).await;
    assert_ne!(result.is_error, Some(true), "{name} {args}: {result:?}");
    (structured_object(&result).clone(), copy)
}

/// Spec 080, AC7: every primitive whose result grows with the corpus bounds
/// it. Each member is called on a generated corpus twice, on fresh copies:
/// at the default read size its lists are inline; past its bound, at a read
/// size of one byte, they are absent, each count still matches, and the file
/// `lists-file` names holds every list exactly as the inline call returned
/// it. A list the inline form leaves out when empty
/// is an empty list. At least one list a member moves is non-empty, so a
/// member that moved nothing could not pass.
#[tokio::test]
async fn every_result_that_grows_with_the_corpus_moves_its_lists_past_the_read_size_via_mcp() {
    let template = tempfile::tempdir().unwrap();
    common::corpus::generate(template.path(), &common::corpus::SMALL);
    vendor_a_violating_markdownlint(template.path());
    drift_one_spec(template.path());
    for (name, args, lists) in bounded_members() {
        let (inline, _) = call_on_a_copy(template.path(), name, &args, None).await;
        assert_eq!(inline["form"], "inline", "{name} {args}: {inline:?}");
        let (bounded, copy) = call_on_a_copy(template.path(), name, &args, Some(1)).await;
        assert_eq!(bounded["form"], "file", "{name} {args}: {bounded:?}");
        let file = bounded["lists-file"].as_str().expect("a lists-file path");
        assert!(
            file.starts_with(&format!(".ductus/results/{name}/")),
            "{name}: {file}"
        );
        let moved: Value =
            serde_json::from_str(&fs::read_to_string(copy.path().join(file)).unwrap()).unwrap();
        let mut entries = 0;
        for key in lists {
            assert!(bounded.get(*key).is_none(), "{name}: `{key}` stayed inline");
            let whole = inline.get(*key).cloned().unwrap_or_else(|| json!([]));
            assert_eq!(moved[*key], whole, "{name} {args}: `{key}`");
            let len = whole.as_array().map_or(0, Vec::len);
            let count = format!("{key}-count");
            assert_eq!(inline[count.as_str()], len, "{name}: inline `{count}`");
            assert_eq!(bounded[count.as_str()], len, "{name}: file `{count}`");
            entries += len;
        }
        assert!(entries > 0, "{name} {args}: every moved list was empty");
    }
}

/// `check-review-gate`'s per-entry `cross-spec-impact`, a field 0.48.0 added
/// to a primitive **result**, crosses the MCP wire.
///
/// It was verified through the CLI and through unit tests, both of which
/// share the serde path this exercises — so this is not a second
/// serialization. What it pins is the MCP surface specifically: a result type
/// that fails to serve (a `JsonSchema` derive that will not generate, a field
/// the wrapper drops) would be invisible to every other test, and the
/// CLI and MCP surfaces are supposed to reach the same result.
///
/// 0.48.0 added two more, the `inbox-standing` fields on `diff-cross-spec`
/// and `write-review`; spec 058 removed both, and the standing inbox count now
/// crosses the wire on `dashboard`, asserted below.
///
/// The gate lints the feature directory before it reaches the cross-spec
/// check, so the fixture vendors a linter that exits clean. Without one the
/// gate falls back to `npx`, which downloads `markdownlint-cli2` into a bare
/// tempdir: on Windows that download failed twice in CI (2026-09-15 and
/// 2026-09-30), and the gate answered `markdown-lint` before the check this
/// test is about.
#[tokio::test]
async fn the_cross_spec_impact_gate_reports_per_entry_via_mcp() {
    let tmp = tempfile::tempdir().unwrap();
    vendor_a_clean_markdownlint(tmp.path());
    let dir = tmp.path().join("specs/001-x");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\ncross-spec-impact: [002-y]\n---\n\n# x\n",
    )
    .unwrap();
    let sibling = tmp.path().join("specs/002-y");
    fs::create_dir_all(&sibling).unwrap();
    fs::write(
        sibling.join("spec.md"),
        "---\nstatus: done\ndependencies: []\n---\n\n# y\n",
    )
    .unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(&client, "check-review-gate", json!({ "feature": "001-x" })).await;
    let obj = structured_object(&result);

    assert_eq!(obj["passed"], false);
    assert_eq!(obj["blocked-by"], "undischarged-cross-spec-impact");
    let entries = obj["cross-spec-impact"].as_array().expect("per-entry list");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["spec"], "002-y");
    assert_eq!(entries[0]["state"], "undischarged");
}

#[tokio::test]
async fn dashboard_reports_the_standing_inbox_via_mcp() {
    // A committed fixture, so the inbox written afterwards has no blame.
    let tmp = init_git_fixture();
    fs::write(
        tmp.path().join("specs/inbox.md"),
        "# Inbox\n\n- one outstanding item\n- and another\n",
    )
    .unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(&client, "dashboard", json!({})).await;
    let standing = &structured_object(&result)["inbox-standing"];

    assert_eq!(standing["state"], "outstanding");
    assert_eq!(standing["outstanding"], 2);
    assert_eq!(standing["path"], "specs/inbox.md");
    // The inbox was written after the fixture's only commit, so blame can say
    // nothing about it: the age is absent rather than defaulted to today,
    // which is the distinction the row exists to preserve.
    assert!(standing.get("oldest").is_none(), "{standing:?}");
}

/// A registered source's `description` reaches the record it is reported from,
/// including on a **skipped** entry — the case where the document is missing
/// and the description is the only thing naming what was lost.
#[tokio::test]
async fn resolve_constitutions_carries_the_description_via_mcp() {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
    fs::write(
        tmp.path().join(".ductus/config.toml"),
        "[constitutions.acme]\nrepo = \"https://example.test/g\"\npath = \"nowhere\"\n\
         description = \"Acme platform engineering rules\"\n",
    )
    .unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(&client, "resolve-constitutions", json!({})).await;
    let obj = structured_object(&result);

    let skipped = obj["skipped"].as_array().expect("skipped list");
    assert_eq!(skipped.len(), 1);
    assert_eq!(skipped[0]["alias"], "acme");
    assert_eq!(skipped[0]["description"], "Acme platform engineering rules");
    assert_eq!(skipped[0]["outcome"], "not-checked-out");
}

/// `examined` and `scope` cross the MCP wire, and an unstated claim stays
/// **absent** rather than arriving as a zero.
///
/// Pinned here for the reason the 0.48.0 fields were: the review parity golden
/// captures only a progress line for `write-review`, never its result, so a
/// field the wrapper dropped would be invisible to every other test. The
/// distinction this asserts is the whole point of the field — a record that
/// never made the claim and one that made it and it was empty are different
/// facts, and collapsing them would make an unread scope indistinguishable
/// from a read one.
#[tokio::test]
async fn write_review_reports_examined_against_a_derived_scope_via_mcp() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("specs/001-x");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
    )
    .unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let base = json!({
        "feature": "001-x",
        "reviewed-at": "2026-09-13T00:00:00Z",
        "reviewed-against": "abc1234",
        "diff-base": "",
    });

    // Unstated: absent on the wire and absent from the record.
    let result = call_tool(&client, "write-review", base.clone()).await;
    let obj = structured_object(&result);
    assert!(obj["examined"].is_null(), "{obj:?}");
    assert!(
        obj["scope"].is_number(),
        "scope is always computed: {obj:?}"
    );
    let report = fs::read_to_string(tmp.path().join("specs/001-x/review.md")).unwrap();
    assert!(!report.contains("examined:"), "{report}");

    // Stated: echoed on the wire and written to the record — which has one
    // home now (spec 057), so the spec must not carry a second copy.
    let mut stated = base;
    stated["examined"] = json!(4);
    let result = call_tool(&client, "write-review", stated).await;
    let obj = structured_object(&result);
    assert_eq!(obj["examined"], 4);
    let report = fs::read_to_string(tmp.path().join("specs/001-x/review.md")).unwrap();
    assert!(report.contains("examined: 4"), "{report}");
    let spec = fs::read_to_string(tmp.path().join("specs/001-x/spec.md")).unwrap();
    assert!(!spec.contains("examined:"), "{spec}");
}

#[tokio::test]
async fn write_review_renders_report_via_mcp() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("specs/001-x");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
    )
    .unwrap();

    let client = start_pair(tmp.path().to_path_buf()).await;
    let result = call_tool(
        &client,
        "write-review",
        json!({
            "feature": "001-x",
            "reviewed-at": "2026-07-06T00:00:00Z",
            "reviewed-against": "abc1234",
            "diff-base": "",
            "findings": [{
                "rule": "SEC-BE-001", "severity": "must", "file": "src/a.rs",
                "line-range": "1-5", "confidence": "high"
            }],
        }),
    )
    .await;
    let obj = structured_object(&result);
    assert_eq!(obj["must-violations"], 1);
    assert_eq!(obj["blocking"], true);
    assert!(
        tmp.path().join("specs/001-x/review.md").exists(),
        "write-review must render review.md"
    );
}
