//! `merge-status-hook` — install Claude Code's `UserPromptExpansion` hooks for
//! `/{project}:status` and `/{project}:help` into a settings file, gated on the
//! Claude Code version (spec 080, AC2 and AC20, and the scenario
//! `help-answers-with-no-model-turn`). The entries, their recognition and the
//! result are spec 080's `data-model.md`, `The hook entries in {cli-config-dir}/settings.local.json`
//! and `merge-status-hook`; the hooks it installs run `ductus hook status` and
//! `ductus hook help` ([`crate::hook`]).
//!
//! The gate comes first. Before Claude Code 2.1.101 an unrecognized hook event
//! made Claude Code ignore the whole settings file, so writing the entry there
//! would silently cost the user every permission. The primitive therefore runs
//! `claude --version` itself, with a timeout, and on an older version, or one
//! it cannot read, writes nothing and says why. The permissions are a separate
//! `merge-permissions` call, so a skip leaves them as that call left them.
//!
//! The merge then works under `hooks.UserPromptExpansion`. Each of ductus's
//! entries is recognized by its own command string alone, so each is added once,
//! an adopter holding only status's entry gains help's, and every other entry,
//! whatever its event or matcher, and every other key are left as they are. The
//! whole file is written atomically, as `merge-permissions` writes it.

use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use serde_json::{Value, json};

use crate::host::Host;
use crate::primitives::{PrimitiveError, Result, output_within, read_text, write_atomic};
use crate::schema::primitives::{MergeStatusHookArgs, MergeStatusHookResult};

/// The oldest Claude Code release that writes the hook: the first whose
/// settings loader skips an unrecognized hook event rather than ignoring the
/// whole file (its 2.1.101 changelog entry).
pub(crate) const MIN_CLAUDE_VERSION: (u64, u64, u64) = (2, 1, 101);

/// How long `claude --version` may run before the probe gives up and the
/// version counts as unreadable (`BE-TIMEOUT-001`).
pub(crate) const PROBE_TIMEOUT: Duration = Duration::from_secs(10);

/// Each hook entry's own timeout, in seconds, which Claude Code applies to its
/// `ductus hook` command. A timeout takes the hook's failure path, so the
/// command falls back to the model path.
const HOOK_TIMEOUT_SECONDS: u64 = 10;

/// The `ductus hook` members the primitive installs an entry for, in the order
/// the entries are written, each matched to `{project}:{member}`, the slash
/// command it answers.
pub const HOOKED_COMMANDS: &[&str] = &["status", "help"];

/// The hook event the entry sits under.
const EVENT: &str = "UserPromptExpansion";

/// The programs the probe tries, in order, moving on only when one is not
/// found. On Windows `Command` resolves `claude` to `claude.exe` itself but
/// never to the `.cmd` shim an npm install provides.
const PROBE_PROGRAMS: &[&str] = if cfg!(windows) {
    &["claude", "claude.cmd"]
} else {
    &["claude"]
};

/// The command of ductus's hook entry for `member`: the project's runtime
/// pointer, quoted since the project directory may hold a space, running
/// `hook {member}`. It is what recognizes the entry as ductus's.
#[must_use]
pub fn hook_command(member: &str) -> String {
    format!(
        "\"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus{}\" hook {member}",
        std::env::consts::EXE_SUFFIX
    )
}

/// What `claude --version` told the probe.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Probe {
    /// It ran and exited 0, printing this.
    Printed(String),
    /// It could not run, did not answer in time, or failed, for this reason.
    Unreadable(String),
}

/// Execute the `merge-status-hook` primitive.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidPath`] when `path` is absolute, empty, climbs
///   out with `..`, reaches outside the project through a symlink, or names a
///   protected destination: checked before the probe runs or anything is read.
/// - [`PrimitiveError::Io`] on local filesystem failures.
/// - [`PrimitiveError::Json`] when an existing file fails JSON parse.
/// - [`PrimitiveError::JsonSchema`] when the top level, `hooks`, or
///   `hooks.UserPromptExpansion` is present with the wrong JSON type.
pub fn run(args: &MergeStatusHookArgs, repo: &Path) -> Result<MergeStatusHookResult> {
    run_with(args, repo, || probe_version(PROBE_TIMEOUT))
}

/// [`run`] with the version probe supplied: the seam a test uses to gate on a
/// version without the machine's own `claude`. `probe` runs only once the
/// path is accepted.
pub(crate) fn run_with(
    args: &MergeStatusHookArgs,
    repo: &Path,
    probe: impl FnOnce() -> Probe,
) -> Result<MergeStatusHookResult> {
    super::validate_contained(repo, &args.path)?;
    let target_path = repo.join(&args.path);
    super::host_config::check_no_protected(repo, &target_path, &args.path)?;

    let version = match gate(&probe()) {
        Gate::Write(version) => version,
        Gate::Skip { version, reason } => {
            return Ok(MergeStatusHookResult {
                action: "skipped".into(),
                claude_version: version,
                reason: Some(reason),
            });
        }
    };

    let existing = match target_path.try_exists() {
        Ok(true) => Some(read_text(&target_path)?),
        Ok(false) => None,
        Err(source) => {
            return Err(PrimitiveError::Io {
                path: target_path.clone(),
                source,
            });
        }
    };
    let project = Host::load(repo)?.project;
    let hooks: Vec<(String, String)> = HOOKED_COMMANDS
        .iter()
        .map(|member| (format!("{project}:{member}"), hook_command(member)))
        .collect();
    let (merged, action) = merge(existing.as_deref(), &hooks, &target_path)?;
    if action != "unchanged" {
        write_atomic(
            &target_path,
            &super::merge_permissions::serialize_pretty(&merged),
        )?;
    }
    Ok(MergeStatusHookResult {
        action: action.into(),
        claude_version: Some(version),
        reason: None,
    })
}

/// Run `claude --version` through [`PROBE_PROGRAMS`], each under `timeout`.
fn probe_version(timeout: Duration) -> Probe {
    for program in PROBE_PROGRAMS {
        match probe_program(OsStr::new(program), timeout) {
            ProbeRun::NotFound => {}
            ProbeRun::Done(probe) => return probe,
        }
    }
    Probe::Unreadable(
        "`claude` is not on PATH as this process sees it — an MCP server an IDE or a \
         desktop app launched may not inherit the shell's PATH"
            .into(),
    )
}

/// One program's probe: not found, so the next is tried, or its answer.
enum ProbeRun {
    NotFound,
    Done(Probe),
}

/// Run `program --version` under `timeout`, with stdin closed, since an MCP
/// server's stdin carries its protocol.
fn probe_program(program: &OsStr, timeout: Duration) -> ProbeRun {
    let mut command = Command::new(program);
    command.arg("--version");
    let output = match output_within(&mut command, timeout) {
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return ProbeRun::NotFound,
        Err(err) => {
            return ProbeRun::Done(Probe::Unreadable(format!(
                "`claude --version` could not run: {err}"
            )));
        }
        Ok(None) => {
            return ProbeRun::Done(Probe::Unreadable(format!(
                "`claude --version` did not answer within {} s",
                timeout.as_secs()
            )));
        }
        Ok(Some(output)) => output,
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let first = stderr.lines().next().unwrap_or("").trim();
        return ProbeRun::Done(Probe::Unreadable(format!(
            "`claude --version` exited {}{}",
            output
                .status
                .code()
                .map_or_else(|| "on a signal".to_owned(), |code| code.to_string()),
            if first.is_empty() {
                String::new()
            } else {
                format!(": {first}")
            }
        )));
    }
    ProbeRun::Done(Probe::Printed(
        String::from_utf8_lossy(&output.stdout).into_owned(),
    ))
}

/// The gate's verdict on a probe.
#[derive(Debug, PartialEq, Eq)]
enum Gate {
    /// Write the entry; the version read.
    Write(String),
    /// Write nothing, for `reason`; the version read, when one was.
    Skip {
        version: Option<String>,
        reason: String,
    },
}

fn gate(probe: &Probe) -> Gate {
    let printed = match probe {
        Probe::Unreadable(reason) => {
            return Gate::Skip {
                version: None,
                reason: format!("the Claude Code version is unreadable: {reason}"),
            };
        }
        Probe::Printed(printed) => printed,
    };
    let Some((version, parsed)) = parse_version(printed) else {
        return Gate::Skip {
            version: None,
            reason: format!(
                "the Claude Code version is unreadable: `claude --version` printed {:?}, \
                 which starts with no version",
                printed.trim()
            ),
        };
    };
    if parsed < MIN_CLAUDE_VERSION {
        let (major, minor, patch) = MIN_CLAUDE_VERSION;
        return Gate::Skip {
            reason: format!(
                "Claude Code {version} is older than {major}.{minor}.{patch}, before which an \
                 unrecognized hook event made Claude Code ignore the whole settings file, \
                 permissions included; update Claude Code and run this again"
            ),
            version: Some(version),
        };
    }
    Gate::Write(version)
}

/// The version `claude --version` printed, as written and as numbers: its
/// first word, `major.minor.patch`, with a leading `v` and anything after the
/// patch number's digits ignored.
fn parse_version(printed: &str) -> Option<(String, (u64, u64, u64))> {
    let word = printed.split_whitespace().next()?;
    let mut parts = word.strip_prefix('v').unwrap_or(word).splitn(3, '.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch_part = parts.next()?;
    let digits = patch_part
        .find(|c: char| !c.is_ascii_digit())
        .map_or(patch_part, |end| &patch_part[..end]);
    let patch = digits.parse().ok()?;
    Some((word.to_owned(), (major, minor, patch)))
}

/// The settings value with each of ductus's entries present, one per
/// `(matcher, command)` in `wanted`, and the action: `created` with no file,
/// `unchanged` when an inner hook already carries each command, `updated` when
/// any entry was appended.
fn merge(
    existing: Option<&str>,
    wanted: &[(String, String)],
    path: &Path,
) -> Result<(Value, &'static str)> {
    let entry = |(matcher, command): &(String, String)| {
        json!({
            "matcher": matcher,
            "hooks": [{ "type": "command", "command": command, "timeout": HOOK_TIMEOUT_SECONDS }],
        })
    };
    let Some(text) = existing else {
        let entries: Vec<Value> = wanted.iter().map(entry).collect();
        return Ok((json!({ "hooks": { EVENT: entries } }), "created"));
    };
    let mut value: Value = serde_json::from_str(text).map_err(|source| PrimitiveError::Json {
        path: path.into(),
        source,
    })?;
    let schema = |reason: &str| PrimitiveError::JsonSchema {
        path: path.into(),
        reason: reason.into(),
    };
    let root = value
        .as_object_mut()
        .ok_or_else(|| schema("top-level value is not a JSON object"))?;
    let hooks = root
        .entry("hooks")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .ok_or_else(|| schema("`hooks` field exists but is not a JSON object"))?;
    let groups = hooks
        .entry(EVENT)
        .or_insert_with(|| json!([]))
        .as_array_mut()
        .ok_or_else(|| schema("`hooks.UserPromptExpansion` exists but is not a JSON array"))?;
    let mut added = false;
    for hook in wanted {
        let command = hook.1.as_str();
        let present = groups.iter().any(|group| {
            group["hooks"]
                .as_array()
                .is_some_and(|inner| inner.iter().any(|hook| hook["command"] == command))
        });
        if !present {
            groups.push(entry(hook));
            added = true;
        }
    }
    Ok((value, if added { "updated" } else { "unchanged" }))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::fs;

    use super::*;
    use crate::schema::primitives::MergePermissionsArgs;

    const SETTINGS: &str = ".claude/settings.local.json";

    fn args() -> MergeStatusHookArgs {
        MergeStatusHookArgs {
            path: SETTINGS.into(),
        }
    }

    fn printed(text: &str) -> impl FnOnce() -> Probe {
        let text = text.to_owned();
        move || Probe::Printed(text)
    }

    /// A project named `myapp`.
    fn project() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[host]\nproject = \"myapp\"\n",
        )
        .unwrap();
        tmp
    }

    fn settings(repo: &Path) -> Value {
        serde_json::from_str(&fs::read_to_string(repo.join(SETTINGS)).unwrap()).unwrap()
    }

    /// The `UserPromptExpansion` groups whose inner hook runs ductus's command
    /// for `member`.
    fn ductus_entries(value: &Value, member: &str) -> usize {
        value["hooks"][EVENT]
            .as_array()
            .unwrap()
            .iter()
            .filter(|group| group["hooks"][0]["command"] == hook_command(member).as_str())
            .count()
    }

    /// Each command is pinned as each platform's literal, never through
    /// `hook_command`, since the pointer is `ductus.exe` on Windows.
    #[test]
    fn no_file_is_created_holding_both_entries() {
        let tmp = project();
        let result = run_with(&args(), tmp.path(), printed("2.1.319 (Claude Code)\n")).unwrap();
        assert_eq!(result.action, "created");
        assert_eq!(result.claude_version.as_deref(), Some("2.1.319"));
        assert_eq!(result.reason, None);
        let pointer = if cfg!(windows) {
            "\"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus.exe\""
        } else {
            "\"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus\""
        };
        assert_eq!(
            settings(tmp.path()),
            json!({ "hooks": { "UserPromptExpansion": [
                {
                    "matcher": "myapp:status",
                    "hooks": [{
                        "type": "command",
                        "command": format!("{pointer} hook status"),
                        "timeout": 10,
                    }],
                },
                {
                    "matcher": "myapp:help",
                    "hooks": [{
                        "type": "command",
                        "command": format!("{pointer} hook help"),
                        "timeout": 10,
                    }],
                },
            ] } })
        );
    }

    /// The entry is added beside another `UserPromptExpansion` entry and every
    /// other key, all left as they were, in their order.
    #[test]
    fn an_existing_file_is_updated_leaving_every_other_key_and_entry() {
        let tmp = project();
        let before = json!({
            "permissions": { "allow": ["Edit"], "deny": [] },
            "hooks": {
                "PreToolUse": [{ "matcher": "Bash", "hooks": [{ "type": "command", "command": "guard" }] }],
                "UserPromptExpansion": [{
                    "matcher": "other:cmd",
                    "hooks": [{ "type": "command", "command": "other-hook", "timeout": 5 }],
                }],
            },
            "model": "opus",
        });
        fs::create_dir_all(tmp.path().join(".claude")).unwrap();
        fs::write(tmp.path().join(SETTINGS), before.to_string()).unwrap();

        let result = run_with(&args(), tmp.path(), printed("2.1.101 (Claude Code)")).unwrap();
        assert_eq!(result.action, "updated");
        let after = settings(tmp.path());
        let keys: Vec<&String> = after.as_object().unwrap().keys().collect();
        assert_eq!(keys, ["permissions", "hooks", "model"]);
        assert_eq!(after["permissions"], before["permissions"]);
        assert_eq!(after["model"], before["model"]);
        assert_eq!(after["hooks"]["PreToolUse"], before["hooks"]["PreToolUse"]);
        let groups = after["hooks"][EVENT].as_array().unwrap();
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0], before["hooks"][EVENT][0]);
        assert_eq!(groups[1]["matcher"], "myapp:status");
        assert_eq!(groups[2]["matcher"], "myapp:help");
        assert_eq!(ductus_entries(&after, "status"), 1);
        assert_eq!(ductus_entries(&after, "help"), 1);
    }

    /// A settings file holding status's entry alone, as a project bootstrapped
    /// before help's entry shipped does, gains help's beside it, and status's
    /// stays as it was.
    #[test]
    fn a_file_holding_only_status_s_entry_gains_help_s() {
        let tmp = project();
        let status = json!({
            "matcher": "myapp:status",
            "hooks": [{ "type": "command", "command": hook_command("status"), "timeout": 10 }],
        });
        let before = json!({ "hooks": { "UserPromptExpansion": [status] } });
        fs::create_dir_all(tmp.path().join(".claude")).unwrap();
        fs::write(tmp.path().join(SETTINGS), before.to_string()).unwrap();
        let result = run_with(&args(), tmp.path(), printed("2.1.319")).unwrap();
        assert_eq!(result.action, "updated");
        let after = settings(tmp.path());
        let groups = after["hooks"][EVENT].as_array().unwrap();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0], status);
        assert_eq!(groups[1]["matcher"], "myapp:help");
        assert_eq!(
            groups[1]["hooks"][0]["command"],
            hook_command("help").as_str()
        );
    }

    /// A second run finds each entry by its command and writes nothing, so the
    /// file holds exactly one of each.
    #[test]
    fn a_second_run_is_unchanged_and_leaves_one_of_each_entry() {
        let tmp = project();
        run_with(&args(), tmp.path(), printed("2.1.319")).unwrap();
        let first = fs::read(tmp.path().join(SETTINGS)).unwrap();
        let result = run_with(&args(), tmp.path(), printed("2.1.319")).unwrap();
        assert_eq!(result.action, "unchanged");
        assert_eq!(fs::read(tmp.path().join(SETTINGS)).unwrap(), first);
        let after = settings(tmp.path());
        assert_eq!(ductus_entries(&after, "status"), 1);
        assert_eq!(ductus_entries(&after, "help"), 1);
    }

    /// An unchanged run writes nothing: with the settings directory read-only,
    /// any write would fail the run.
    #[cfg(unix)]
    #[test]
    fn an_unchanged_run_writes_nothing() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = project();
        run_with(&args(), tmp.path(), printed("2.1.319")).unwrap();
        let dir = tmp.path().join(".claude");
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o555)).unwrap();
        let result = run_with(&args(), tmp.path(), printed("2.1.319"));
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(result.unwrap().action, "unchanged");
    }

    /// Each of ductus's entries is recognized by its command alone: one under
    /// another matcher is still its own, and no second entry is added.
    #[test]
    fn each_entry_is_recognized_by_its_command_not_its_matcher() {
        let tmp = project();
        let before = json!({ "hooks": { "UserPromptExpansion": [
            {
                "matcher": "renamed:status",
                "hooks": [{ "type": "command", "command": hook_command("status"), "timeout": 10 }],
            },
            {
                "matcher": "renamed:help",
                "hooks": [{ "type": "command", "command": hook_command("help"), "timeout": 10 }],
            },
        ] } });
        fs::create_dir_all(tmp.path().join(".claude")).unwrap();
        fs::write(tmp.path().join(SETTINGS), before.to_string()).unwrap();
        let result = run_with(&args(), tmp.path(), printed("2.1.319")).unwrap();
        assert_eq!(result.action, "unchanged");
        assert_eq!(settings(tmp.path()), before);
    }

    /// An older version writes nothing: no file is created, an existing one
    /// is left byte for byte, and the result names the version and why.
    #[test]
    fn an_older_version_is_skipped_and_writes_nothing() {
        let tmp = project();
        let result = run_with(&args(), tmp.path(), printed("2.1.100 (Claude Code)")).unwrap();
        assert_eq!(result.action, "skipped");
        assert_eq!(result.claude_version.as_deref(), Some("2.1.100"));
        let reason = result.reason.unwrap();
        assert!(
            reason.contains("2.1.100") && reason.contains("2.1.101"),
            "{reason}"
        );
        assert!(!tmp.path().join(SETTINGS).exists());
    }

    /// An unreadable version, whether the probe failed or printed no version,
    /// is skipped with no version and the probe's reason.
    #[test]
    fn an_unreadable_version_is_skipped_with_its_reason() {
        let tmp = project();
        let failed = run_with(&args(), tmp.path(), || {
            Probe::Unreadable("`claude` is not on PATH".into())
        })
        .unwrap();
        assert_eq!(failed.action, "skipped");
        assert_eq!(failed.claude_version, None);
        assert!(
            failed.reason.as_deref().unwrap().contains("not on PATH"),
            "{failed:?}"
        );
        let garbled = run_with(&args(), tmp.path(), printed("Claude Code\n")).unwrap();
        assert_eq!(garbled.action, "skipped");
        assert_eq!(garbled.claude_version, None);
        assert!(
            garbled
                .reason
                .as_deref()
                .unwrap()
                .contains("\"Claude Code\""),
            "{garbled:?}"
        );
        assert!(!tmp.path().join(SETTINGS).exists());
    }

    /// The permissions `merge-permissions` merged survive a skip untouched,
    /// since the skip writes nothing.
    #[test]
    fn a_skip_leaves_the_merged_permissions_intact() {
        let tmp = project();
        crate::primitives::merge_permissions::run(
            &MergePermissionsArgs {
                path: SETTINGS.into(),
                allow: vec!["Edit".into()],
                deny: vec!["Bash(rm -rf *)".into()],
                revoke: Vec::new(),
                canonical: false,
            },
            tmp.path(),
        )
        .unwrap();
        let merged = fs::read(tmp.path().join(SETTINGS)).unwrap();
        for probe in [
            Probe::Printed("2.0.0".into()),
            Probe::Unreadable("timed out".into()),
        ] {
            let result = run_with(&args(), tmp.path(), || probe).unwrap();
            assert_eq!(result.action, "skipped");
            assert_eq!(fs::read(tmp.path().join(SETTINGS)).unwrap(), merged);
        }
    }

    /// A refused path — outside the project, or protected (spec 022 tasks 11
    /// and 20) — never runs the probe.
    #[test]
    fn a_refused_path_is_refused_before_the_probe_runs() {
        let tmp = project();
        for path in [
            "/etc/settings.json",
            "../settings.json",
            ".mcp.json",
            ".git/config",
        ] {
            let result = run_with(
                &MergeStatusHookArgs { path: path.into() },
                tmp.path(),
                || panic!("the probe ran for {path}"),
            );
            assert!(
                matches!(result, Err(PrimitiveError::InvalidPath { .. })),
                "{path}: {result:?}"
            );
        }
    }

    #[test]
    fn a_settings_file_of_the_wrong_shape_is_refused_unwritten() {
        let tmp = project();
        fs::create_dir_all(tmp.path().join(".claude")).unwrap();
        for (text, json_error) in [
            ("{ not json", true),
            ("[]", false),
            (r#"{"hooks": []}"#, false),
            (r#"{"hooks": {"UserPromptExpansion": {}}}"#, false),
        ] {
            fs::write(tmp.path().join(SETTINGS), text).unwrap();
            let result = run_with(&args(), tmp.path(), printed("2.1.319"));
            if json_error {
                assert!(
                    matches!(result, Err(PrimitiveError::Json { .. })),
                    "{text}: {result:?}"
                );
            } else {
                assert!(
                    matches!(result, Err(PrimitiveError::JsonSchema { .. })),
                    "{text}: {result:?}"
                );
            }
            assert_eq!(fs::read_to_string(tmp.path().join(SETTINGS)).unwrap(), text);
        }
    }

    /// The gate compares the numbers, not the text: 2.1.101 itself writes,
    /// and 2.10.0 and 10.0.0 are newer than 2.1.101.
    #[test]
    fn the_version_gate_compares_numerically_and_admits_its_bound() {
        for (printed, writes) in [
            ("2.1.101 (Claude Code)", true),
            ("v2.1.101", true),
            ("2.1.101-beta.1", true),
            ("2.1.100", false),
            ("2.0.999", false),
            ("1.99.200", false),
            ("2.10.0", true),
            ("10.0.0", true),
            ("2.2.0", true),
        ] {
            let verdict = gate(&Probe::Printed(printed.into()));
            assert_eq!(
                matches!(verdict, Gate::Write(_)),
                writes,
                "{printed}: {verdict:?}"
            );
        }
        for unparsed in ["", "Claude Code", "2.1", "two.one.101", "2.1.x"] {
            assert!(
                matches!(
                    gate(&Probe::Printed(unparsed.into())),
                    Gate::Skip { version: None, .. }
                ),
                "{unparsed:?}"
            );
        }
    }

    /// The real probe: a program that prints a version, one that fails, one
    /// that hangs past the timeout, and one that does not exist.
    #[cfg(unix)]
    #[test]
    fn the_probe_reads_a_version_and_names_each_failure() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let script = |name: &str, body: &str| {
            let path = tmp.path().join(name);
            fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
            path
        };
        // A probe that must answer gets a deadline the first run of a freshly
        // written script cannot reach: macOS scans a new executable before it
        // runs it, which took this test past 5 s (two whole-suite failures and
        // one in five lone runs at b70246f7). Only the hang is cut short.
        let timeout = Duration::from_secs(120);
        let prints = script("prints", "echo \"2.1.200 (Claude Code)\"");
        assert!(matches!(
            probe_program(prints.as_os_str(), timeout),
            ProbeRun::Done(Probe::Printed(out)) if out == "2.1.200 (Claude Code)\n"
        ));
        let fails = script("fails", "echo 'no config' >&2; exit 3");
        assert!(matches!(
            probe_program(fails.as_os_str(), timeout),
            ProbeRun::Done(Probe::Unreadable(reason)) if reason == "`claude --version` exited 3: no config"
        ));
        let hangs = script("hangs", "sleep 5");
        assert!(matches!(
            probe_program(hangs.as_os_str(), Duration::from_millis(200)),
            ProbeRun::Done(Probe::Unreadable(reason)) if reason.contains("did not answer")
        ));
        assert!(matches!(
            probe_program(tmp.path().join("absent").as_os_str(), timeout),
            ProbeRun::NotFound
        ));
    }
}
