//! `ductus hook status` and `ductus hook help`, driven through the binary,
//! since their exit status, their two streams and the environment they read
//! are the contract (spec 080's `data-model.md`, `ductus hook status` and
//! `ductus hook help`): AC16, AC17 and AC25 on the runtime's side, and the
//! scenario `help-answers-with-no-model-turn`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Output, Stdio};

mod common;
use common::ductus_command;

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

/// A project named `myapp` holding one planned spec, `spec`.
fn project(spec: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path(),
        ".ductus/config.toml",
        "[host]\nproject = \"myapp\"\n",
    );
    write(
        dir.path(),
        &format!("specs/{spec}/spec.md"),
        "---\nstatus: planned\ndependencies: []\n---\n\n# Spec\n",
    );
    dir
}

/// The input Claude Code writes to the hook's stdin for a typed
/// `/myapp:status {args}` in a session whose working directory is `cwd`.
fn hook_input(args: &str, cwd: &Path) -> String {
    serde_json::json!({
        "session_id": "hook-input-session",
        "cwd": cwd,
        "hook_event_name": "UserPromptExpansion",
        "command_name": "myapp:status",
        "command_args": args,
        "prompt": format!("/myapp:status {args}").trim_end(),
    })
    .to_string()
}

/// Run `ductus hook status` from `cwd` with `env` set, writing `input` to
/// its stdin.
fn run_hook(cwd: &Path, env: &[(&str, &std::ffi::OsStr)], input: &str) -> Output {
    run_member("status", cwd, env, input)
}

/// Run `ductus hook {member}` from `cwd` with `env` set, writing `input` to
/// its stdin.
fn run_member(member: &str, cwd: &Path, env: &[(&str, &std::ffi::OsStr)], input: &str) -> Output {
    let mut command = ductus_command(env!("CARGO_BIN_EXE_ductus"));
    command
        .args(["hook", member])
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in env {
        command.env(name, value);
    }
    let mut child = command.spawn().expect("binary runs");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

/// The response of a run that succeeded: exit 0 and one JSON object on stdout.
fn response(output: &Output) -> serde_json::Value {
    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "stdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(stdout.lines().count(), 1, "one JSON object: {stdout}");
    serde_json::from_str(&stdout).unwrap()
}

fn reason(response: &serde_json::Value) -> String {
    response["reason"].as_str().unwrap().to_owned()
}

/// The response blocks the typed command and suppresses its echo, so the
/// shown view does not end with an "Original prompt:" line (AC16).
#[test]
fn the_view_is_shown_as_a_block_that_suppresses_the_original_prompt() {
    let dir = project("001-alpha");
    let value = response(&run_hook(dir.path(), &[], &hook_input("", dir.path())));
    let keys: Vec<&str> = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(keys, ["decision", "reason", "hookSpecificOutput"]);
    assert_eq!(value["decision"], "block");
    assert_eq!(
        value["hookSpecificOutput"],
        serde_json::json!({
            "hookEventName": "UserPromptExpansion",
            "suppressOriginalPrompt": true,
        })
    );
}

/// The reason is the fixed-width text rendering: columns separated by two
/// spaces, no pipe table or emphasis, and the closing prompt (AC17).
#[test]
fn the_reason_is_the_fixed_width_text_view() {
    let dir = project("001-alpha");
    let view = reason(&response(&run_hook(
        dir.path(),
        &[],
        &hook_input("", dir.path()),
    )));
    assert!(
        view.lines()
            .any(|line| line.contains("001-alpha  planned  ")),
        "{view}"
    );
    assert!(
        !view.lines().any(|line| line.trim_start().starts_with('|')),
        "{view}"
    );
    assert!(!view.contains("**"), "{view}");
    assert!(
        view.ends_with("Run /myapp:target <feature> to select one."),
        "{view}"
    );
}

/// No field of the input is read: arguments typed after the command, a `cwd`
/// naming another project, input that is not JSON and no input at all show
/// the same view, the one at `CLAUDE_PROJECT_DIR`.
#[test]
fn arguments_and_every_other_input_field_are_ignored() {
    let dir = project("001-alpha");
    let other = project("001-other");
    let env = [("CLAUDE_PROJECT_DIR", dir.path().as_os_str())];
    let views: Vec<String> = [
        hook_input("", dir.path()),
        hook_input("foo --bar", other.path()),
        "not json".to_owned(),
        String::new(),
    ]
    .iter()
    .map(|input| reason(&response(&run_hook(other.path(), &env, input))))
    .collect();
    assert!(views[0].contains("001-alpha"), "{}", views[0]);
    assert!(!views[0].contains("001-other"), "{}", views[0]);
    for view in &views[1..] {
        assert_eq!(view, &views[0]);
    }
}

/// The root is `CLAUDE_PROJECT_DIR` when it is set and non-empty, so a hook
/// run from below the project root with the root in it shows the root's view;
/// otherwise it is the working directory.
#[test]
fn the_root_is_claude_project_dir_when_set_and_the_working_directory_otherwise() {
    let dir = project("001-alpha");
    let below = dir.path().join("src/deeper");
    fs::create_dir_all(&below).unwrap();
    let view_from = |cwd: &Path, env: &[(&str, &std::ffi::OsStr)]| {
        reason(&response(&run_hook(cwd, env, &hook_input("", cwd))))
    };

    let set = view_from(&below, &[("CLAUDE_PROJECT_DIR", dir.path().as_os_str())]);
    assert!(set.contains("001-alpha"), "{set}");
    assert!(
        set.ends_with("Run /myapp:target <feature> to select one."),
        "{set}"
    );

    let unset = view_from(&below, &[]);
    assert!(!unset.contains("001-alpha"), "{unset}");
    let empty = view_from(&below, &[("CLAUDE_PROJECT_DIR", std::ffi::OsStr::new(""))]);
    assert_eq!(empty, unset);
    assert!(view_from(dir.path(), &[]).contains("001-alpha"));
}

/// The session identity Claude Code puts in the hook's environment reaches the
/// view, which names that session's own target rather than the shared default.
#[test]
fn the_view_names_the_hook_sessions_own_target() {
    let dir = project("001-alpha");
    write(
        dir.path(),
        "specs/002-beta/spec.md",
        "---\nstatus: planned\ndependencies: []\n---\n\n# Spec\n",
    );
    write(
        dir.path(),
        ".ductus/session.toml",
        "feature = \"001-alpha\"\npath = \"specs/001-alpha\"\n",
    );
    write(
        dir.path(),
        ".ductus/sessions/hooksession.toml",
        "source = \"claude-code\"\nfeature = \"002-beta\"\npath = \"specs/002-beta\"\n",
    );
    let first_line = |env: &[(&str, &std::ffi::OsStr)]| {
        let view = reason(&response(&run_hook(
            dir.path(),
            env,
            &hook_input("", dir.path()),
        )));
        view.lines().next().unwrap().to_owned()
    };

    let own = first_line(&[(
        "CLAUDE_CODE_SESSION_ID",
        std::ffi::OsStr::new("hooksession"),
    )]);
    assert!(own.starts_with("Target: 002-beta "), "{own}");
    assert!(own.ends_with("— session claude-code:hooksess"), "{own}");
    let shared = first_line(&[]);
    assert!(shared.starts_with("Target: 001-alpha "), "{shared}");
}

/// A view that cannot be rendered exits 1, a non-blocking hook error after
/// which Claude Code runs the command on the model path, with the error on
/// stderr and nothing on stdout (AC25). Exit 2 would block the expansion.
#[test]
fn a_view_that_cannot_be_rendered_exits_1_with_the_error_on_stderr_alone() {
    let malformed = project("001-alpha");
    write(
        malformed.path(),
        ".ductus/config.toml",
        "[host]\nproject = \"myapp\"\n\n[artifacts]\nread-size-bytes = \"abc\"\n",
    );
    let fine = project("001-alpha");
    for (dir, env, named) in [
        (malformed.path(), Vec::new(), "read-size-bytes"),
        (
            fine.path(),
            vec![("DUCTUS_SESSION", std::ffi::OsStr::new("!!!"))],
            "DUCTUS_SESSION",
        ),
    ] {
        let output = run_hook(dir, &env, &hook_input("", dir));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
        assert!(
            output.stdout.is_empty(),
            "stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(stderr.contains(named), "stderr: {stderr}");
    }
}

/// Install the help command's copy as Claude Code's layout holds it, with
/// `{project}` substituted as the bootstrap substitutes it.
fn install_help(root: &Path) {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/commands/help.md"),
    )
    .unwrap();
    write(
        root,
        ".claude/commands/myapp/help.md",
        &source.replace("{project}", "myapp"),
    );
}

/// A typed help is answered with the guide as a block that suppresses the
/// typed command, and the guide is the one `render-help` returns, so the hook
/// and Pi's command show one guide. The input, arguments included, is not read.
#[test]
fn help_is_shown_as_a_block_holding_the_installed_guide() {
    let dir = project("001-alpha");
    install_help(dir.path());
    let value = response(&run_member(
        "help",
        dir.path(),
        &[],
        &hook_input("", dir.path()),
    ));
    assert_eq!(value["decision"], "block");
    assert_eq!(
        value["hookSpecificOutput"],
        serde_json::json!({
            "hookEventName": "UserPromptExpansion",
            "suppressOriginalPrompt": true,
        })
    );
    let guide = reason(&value);
    assert!(
        guide.starts_with("## myapp — Spec-Driven Development Pipeline"),
        "{guide}"
    );
    assert!(guide.contains("| `/myapp:specify` |"), "{guide}");
    assert!(
        !guide.contains("{project}") && !guide.contains("<!--"),
        "{guide}"
    );

    let rendered = common::ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .arg("render-help")
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(rendered.status.success(), "{rendered:?}");
    let rendered: serde_json::Value = serde_json::from_slice(&rendered.stdout).unwrap();
    assert_eq!(rendered["guide"].as_str(), Some(guide.as_str()));

    let with_args = reason(&response(&run_member(
        "help",
        dir.path(),
        &[],
        &hook_input("foo --bar", dir.path()),
    )));
    assert_eq!(with_args, guide);
}

/// With no installed help copy the hook exits 1, with the error naming the
/// paths tried on stderr and nothing on stdout, so Claude Code runs help on
/// the model path.
#[test]
fn help_with_no_installed_copy_exits_1_with_the_error_on_stderr_alone() {
    let dir = project("001-alpha");
    let output = run_member("help", dir.path(), &[], &hook_input("", dir.path()));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(
        output.stdout.is_empty(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        stderr.contains("no installed copy of the help command")
            && stderr.contains(".claude/commands/myapp/help.md"),
        "stderr: {stderr}"
    );
}
