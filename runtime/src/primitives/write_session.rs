//! `write-session` — atomically rewrite the session state file at
//! `<repo>/.ductus/session.toml`.
//!
//! Mirrors the write-side of [`crate::primitives::dashboard::load_session_target`].
//! The session file is the second of the two durable journals named by
//! spec 022 (markdown + `.ductus/session.toml`, per
//! `specs/022-deterministic-runtime/plan.md` §No data persistence outside
//! session file + markdown); the read path is exposed by `dashboard`, and
//! this primitive is the matching write path.
//!
//! Pre-022 prose left the write to the host's file-writing tool (`Write`
//! on Claude Code), which on Claude Code surfaces a per-invocation
//! permission prompt that documented `Write(...)` allow entries have not
//! reliably suppressed. Routing the write through an MCP tool moves the
//! consent into the MCP tool-permission lane, so a single allow covers
//! every subsequent target/scenario-switch.
//!
//! The previous shape — host-specific JSON at `{cli-config-dir}/{project}-session.json`
//! (e.g., `.claude/gov-session.json`) — coupled the session location to
//! both the AI CLI (`.claude/` vs `.pi/`) and the adopting project's
//! name (`gov-session.json` vs `acme-session.json`). Consolidating onto
//! `.ductus/session.toml` at the repo root makes the path host-agnostic,
//! project-name-agnostic, and uniform across every adopter; the runtime
//! no longer hardcodes any AI CLI's config directory.
//!
//! Since spec 062 the file is the **shared default**, and an identified
//! process — one launched with `DUCTUS_SESSION` or a platform session
//! identity — also writes its own target under `.ductus/sessions/`. The
//! logic lives in [`crate::session`]; this primitive validates the
//! arguments, takes this process's identity (read once at startup) at the
//! edge, and delegates.

use std::path::Path;
use std::time::SystemTime;

use crate::primitives::{
    PrimitiveError, Result, rel_path, validate_no_traversal, validate_session_feature,
    validate_session_scenario,
};
use crate::schema::primitives::{SessionTarget, WriteSessionArgs, WriteSessionResult};
use crate::session::{self, Identity, WriteShape};

/// Execute the `write-session` primitive against `repo`, as the process whose
/// environment this runtime inherited (spec 062).
///
/// Writes the shared default, `.ductus/session.toml`, via tempfile + rename —
/// the same atomic-write pattern every other state-modifying primitive
/// (`mark-task`, `mark-criterion`, `set-status`) uses — and, for an identified
/// process, its own file under `.ductus/sessions/` too.
///
/// # Errors
///
/// Returns [`PrimitiveError::MissingArgument`] when `scenario` and
/// `scenario-path` are not supplied together,
/// [`PrimitiveError::InvalidArgument`] when `clear` is combined with a
/// target field, `feature` is not a feature directory name or `scenario` not
/// a scenario slug, or `DUCTUS_SESSION` sanitizes to nothing,
/// [`PrimitiveError::InvalidPath`] when any caller-supplied path contains a
/// parent-directory component or is absolute, or [`PrimitiveError::Io`] for
/// filesystem failures during the write.
pub fn run(args: &WriteSessionArgs, repo: &Path) -> Result<WriteSessionResult> {
    let identity = session::process_identity()?;
    run_as(args, repo, identity.as_ref(), SystemTime::now())
}

/// Write as `identity`: the one path [`run`] and the tests share.
pub(crate) fn run_as(
    args: &WriteSessionArgs,
    repo: &Path,
    identity: Option<&Identity>,
    now: SystemTime,
) -> Result<WriteSessionResult> {
    validate_args(args)?;

    // Three shapes, in precedence order. A *clear write* removes the target
    // block while preserving the per-contributor `cli-config-dir` (a supplied
    // value overrides the preserved one). A *target write* sets the target and
    // a fresh `set-at`, preserving `cli-config-dir` unless overridden. A
    // *host-config write* sets `cli-config-dir` and preserves the target.
    let shape = if args.clear {
        WriteShape::Clear
    } else if let (Some(feature), Some(path)) = (&args.feature, &args.path) {
        WriteShape::Target(SessionTarget {
            feature: feature.clone(),
            path: path.clone(),
            scenario: args.scenario.clone(),
            scenario_path: args.scenario_path.clone(),
        })
    } else {
        WriteShape::HostConfig
    };
    let outcome = session::write(repo, identity, &shape, args.cli_config_dir.clone(), now)?;

    Ok(WriteSessionResult {
        path: rel_path(&outcome.default_path, repo),
        created: outcome.created,
        identity: outcome.identity,
        own_path: outcome.own_path.map(|path| rel_path(&path, repo)),
        peers: outcome.peers,
        expired: outcome.expired,
        unreadable: outcome
            .unreadable
            .iter()
            .map(|path| rel_path(path, repo))
            .collect(),
        notices: outcome.notices,
    })
}

/// Argument-shape validation for the three write shapes (clear / target /
/// host-config). Split from [`run_as`] so the write path stays
/// readable; the checks run in precedence order.
fn validate_args(args: &WriteSessionArgs) -> Result<()> {
    if let Some(path) = &args.path {
        validate_no_traversal(path)?;
    }
    if let Some(scenario_path) = &args.scenario_path {
        validate_no_traversal(scenario_path)?;
    }
    // Precedence: `clear` is classified before the target / host-config
    // shapes and is mutually exclusive with every target field — a caller
    // cannot clear and set a target in the same write. `cli-config-dir`
    // is NOT a target field and may accompany `clear` (the supplied value
    // overrides the preserved one).
    if args.clear
        && (args.feature.is_some()
            || args.path.is_some()
            || args.scenario.is_some()
            || args.scenario_path.is_some())
    {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "write-session".into(),
            argument: "clear".into(),
            reason: "mutually exclusive with a target write — omit `feature`, `path`, \
                     `scenario`, and `scenario-path` when clearing"
                .into(),
        });
    }
    // `feature` and `path` are a pair — a target needs both.
    match (&args.feature, &args.path) {
        (Some(_), None) => {
            return Err(PrimitiveError::MissingArgument {
                primitive: "write-session".into(),
                argument: "path".into(),
                reason: "must be supplied together with `feature`".into(),
            });
        }
        (None, Some(_)) => {
            return Err(PrimitiveError::MissingArgument {
                primitive: "write-session".into(),
                argument: "feature".into(),
                reason: "must be supplied together with `path`".into(),
            });
        }
        _ => {}
    }
    match (&args.scenario, &args.scenario_path) {
        (Some(_), None) => {
            return Err(PrimitiveError::MissingArgument {
                primitive: "write-session".into(),
                argument: "scenario-path".into(),
                reason: "must be supplied together with `scenario`".into(),
            });
        }
        (None, Some(_)) => {
            return Err(PrimitiveError::MissingArgument {
                primitive: "write-session".into(),
                argument: "scenario".into(),
                reason: "must be supplied together with `scenario-path`".into(),
            });
        }
        _ => {}
    }
    // A scenario only means something inside a target write.
    if args.scenario.is_some() && args.feature.is_none() {
        return Err(PrimitiveError::MissingArgument {
            primitive: "write-session".into(),
            argument: "feature".into(),
            reason: "`scenario` requires a target write (supply `feature` and `path`)".into(),
        });
    }
    // Nothing to do unless this is a clear write, a target write, or a
    // host-config write.
    if !args.clear && args.feature.is_none() && args.cli_config_dir.is_none() {
        return Err(PrimitiveError::MissingArgument {
            primitive: "write-session".into(),
            argument: "feature".into(),
            reason:
                "supply `feature`+`path` (target write), `cli-config-dir` (host-config write), \
                 or `clear` (clear write)"
                    .into(),
        });
    }
    // A target this write stores reaches other sessions through the shared
    // default, so its names are held to the same allowlist as a retarget's.
    if let Some(feature) = &args.feature {
        validate_session_feature("write-session", "feature", feature)?;
    }
    if let Some(scenario) = &args.scenario {
        validate_session_scenario("write-session", "scenario", scenario)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::session::{civil_from_days, iso8601_utc};
    use std::fs;
    use std::time::{Duration, UNIX_EPOCH};
    use tempfile::tempdir;

    /// Seam that injects a stable clock and writes as an **unidentified**
    /// process, so a result never depends on the environment the tests
    /// happen to run in (this shell may carry a platform session id).
    fn run_with_now(
        args: &WriteSessionArgs,
        repo: &Path,
        now: SystemTime,
    ) -> Result<WriteSessionResult> {
        run_as(args, repo, None, now)
    }

    fn fixed_now() -> SystemTime {
        // 2026-05-23T12:34:56Z
        UNIX_EPOCH + Duration::from_secs(1_779_539_696)
    }

    fn base_args() -> WriteSessionArgs {
        WriteSessionArgs {
            feature: Some("022-deterministic-runtime".into()),
            path: Some("specs/022-deterministic-runtime".into()),
            scenario: None,
            scenario_path: None,
            cli_config_dir: None,
            clear: false,
        }
    }

    /// `BE-INPUT-002`: a target this write stores reaches other sessions
    /// through the shared default, so its names are checked before anything
    /// is written.
    #[test]
    fn names_outside_the_grammar_are_refused_and_nothing_is_written() {
        let with_feature = |feature: &str| WriteSessionArgs {
            feature: Some(feature.into()),
            ..base_args()
        };
        let bad_scenario = WriteSessionArgs {
            scenario: Some("Not A Slug".into()),
            scenario_path: Some("specs/022-deterministic-runtime/scenarios/s.md".into()),
            ..base_args()
        };
        let cases = [
            (
                "feature",
                with_feature("022-deterministic-runtime\nNotice: forged"),
            ),
            ("feature", with_feature("022-a/b")),
            ("feature", with_feature("022-a\u{202e}b")),
            ("feature", with_feature("022-a\u{200b}")),
            ("feature", with_feature("")),
            ("feature", with_feature("not-a-feature")),
            ("scenario", bad_scenario),
        ];
        for (argument, args) in cases {
            let tmp = tempdir().unwrap();
            fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
            match run_with_now(&args, tmp.path(), fixed_now()) {
                Err(PrimitiveError::InvalidArgument {
                    argument: named, ..
                }) => {
                    assert_eq!(named, argument, "{args:?}");
                }
                other => panic!("expected InvalidArgument naming {argument}, got {other:?}"),
            }
            assert!(
                !tmp.path().join(".ductus/session.toml").exists(),
                "nothing written for {args:?}"
            );
        }
    }

    fn clear_args() -> WriteSessionArgs {
        WriteSessionArgs {
            feature: None,
            path: None,
            scenario: None,
            scenario_path: None,
            cli_config_dir: None,
            clear: true,
        }
    }

    #[test]
    fn writes_canonical_shape_without_scenario() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let result = run_with_now(&base_args(), tmp.path(), fixed_now()).unwrap();
        assert_eq!(result.path, ".ductus/session.toml");
        assert!(result.created, "fresh file is reported as created");

        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert_eq!(
            body,
            "feature = \"022-deterministic-runtime\"\n\
             path = \"specs/022-deterministic-runtime\"\n\
             set-at = \"2026-05-23T12:34:56Z\"\n"
        );
    }

    #[test]
    fn writes_scenario_pair_when_both_supplied() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let mut args = base_args();
        args.scenario = Some("write-session-primitive".into());
        args.scenario_path =
            Some("specs/022-deterministic-runtime/scenarios/write-session-primitive.md".into());

        let result = run_with_now(&args, tmp.path(), fixed_now()).unwrap();
        assert!(result.created);

        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert!(
            body.contains("scenario = \"write-session-primitive\""),
            "{body}"
        );
        assert!(
            body.contains("scenario-path = \"specs/022-deterministic-runtime/scenarios/write-session-primitive.md\""),
            "{body}"
        );
        // Key order: feature, path, scenario, scenario-path, set-at.
        let feat = body.find("feature =").unwrap();
        let p = body.find("path =").unwrap();
        let scen = body.find("scenario =").unwrap();
        let scen_path = body.find("scenario-path =").unwrap();
        let set_at = body.find("set-at =").unwrap();
        assert!(feat < p && p < scen && scen < scen_path && scen_path < set_at);
    }

    #[test]
    fn overwrites_existing_file_and_reports_not_created() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            "feature = \"old-feature\"\npath = \"specs/old\"\nset-at = \"2026-01-01T00:00:00Z\"\n",
        )
        .unwrap();

        let result = run_with_now(&base_args(), tmp.path(), fixed_now()).unwrap();
        assert!(!result.created, "existing file is reported as overwritten");

        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert!(body.contains("022-deterministic-runtime"));
        assert!(!body.contains("old-feature"));
    }

    #[test]
    fn writes_at_repo_root_regardless_of_project_name() {
        // The point of the consolidation: the path doesn't change with
        // project name, AI CLI, or anything else. It is always
        // `.ductus/session.toml` at the repo root.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let mut args = base_args();
        args.feature = Some("002-observability".into());
        args.path = Some("specs/002-observability".into());
        let result = run_with_now(&args, tmp.path(), fixed_now()).unwrap();
        assert_eq!(result.path, ".ductus/session.toml");
        assert!(tmp.path().join(".ductus/session.toml").is_file());
        // No host-specific or project-specific sibling exists.
        assert!(!tmp.path().join(".claude").exists());
        assert!(!tmp.path().join(".claude/gov-session.json").exists());
        assert!(!tmp.path().join(".claude/acme-session.json").exists());
    }

    #[test]
    fn preserves_cli_config_dir_across_a_target_switch() {
        // `/ductus` records the per-contributor `cli-config-dir` in the
        // session file; a later `/{project}:target` rewrites the file for a
        // new feature and must NOT drop it.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            "feature = \"001-old\"\npath = \"specs/001-old\"\nset-at = \"2026-01-01T00:00:00Z\"\ncli-config-dir = \".pi\"\n",
        )
        .unwrap();

        let mut args = base_args();
        args.feature = Some("002-new".into());
        args.path = Some("specs/002-new".into());
        run_with_now(&args, tmp.path(), fixed_now()).unwrap();

        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert!(body.contains("feature = \"002-new\""), "{body}");
        assert!(
            body.contains("cli-config-dir = \".pi\""),
            "cli-config-dir must survive the target switch: {body}"
        );
        // Serialized after the target block.
        assert!(body.find("set-at =").unwrap() < body.find("cli-config-dir =").unwrap());
    }

    #[test]
    fn omits_cli_config_dir_when_none_recorded() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        run_with_now(&base_args(), tmp.path(), fixed_now()).unwrap();
        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert!(!body.contains("cli-config-dir"), "{body}");
    }

    #[test]
    fn host_config_write_sets_cli_config_dir_on_fresh_repo() {
        // `/ductus` setting the agent identity before any target is selected:
        // a host-config write (no feature) against a fresh repo writes just
        // `cli-config-dir`.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let args = WriteSessionArgs {
            feature: None,
            path: None,
            scenario: None,
            scenario_path: None,
            cli_config_dir: Some(".pi".into()),
            clear: false,
        };
        let result = run_with_now(&args, tmp.path(), fixed_now()).unwrap();
        assert!(result.created);
        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert_eq!(body, "cli-config-dir = \".pi\"\n");
    }

    #[test]
    fn host_config_write_preserves_existing_target() {
        // Setting `cli-config-dir` after a target is already selected must not
        // disturb the target block (feature/path/scenario/set-at).
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            "feature = \"001-x\"\npath = \"specs/001-x\"\nset-at = \"2026-01-01T00:00:00Z\"\n",
        )
        .unwrap();
        let args = WriteSessionArgs {
            feature: None,
            path: None,
            scenario: None,
            scenario_path: None,
            cli_config_dir: Some(".pi".into()),
            clear: false,
        };
        run_with_now(&args, tmp.path(), fixed_now()).unwrap();
        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert!(body.contains("feature = \"001-x\""), "{body}");
        assert!(body.contains("path = \"specs/001-x\""), "{body}");
        assert!(body.contains("set-at = \"2026-01-01T00:00:00Z\""), "{body}");
        assert!(body.contains("cli-config-dir = \".pi\""), "{body}");
    }

    #[test]
    fn rejects_write_with_neither_target_nor_cli_config_dir() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let args = WriteSessionArgs {
            feature: None,
            path: None,
            scenario: None,
            scenario_path: None,
            cli_config_dir: None,
            clear: false,
        };
        let err = run_with_now(&args, tmp.path(), fixed_now()).unwrap_err();
        assert!(matches!(err, PrimitiveError::MissingArgument { .. }));
        assert!(!tmp.path().join(".ductus/session.toml").exists());
    }

    #[test]
    fn rejects_feature_without_path() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let args = WriteSessionArgs {
            feature: Some("001-x".into()),
            path: None,
            scenario: None,
            scenario_path: None,
            cli_config_dir: None,
            clear: false,
        };
        let err = run_with_now(&args, tmp.path(), fixed_now()).unwrap_err();
        match err {
            PrimitiveError::MissingArgument {
                primitive,
                argument,
                ..
            } => {
                assert_eq!(primitive, "write-session");
                assert_eq!(argument, "path");
            }
            other => panic!("expected MissingArgument, got {other:?}"),
        }
    }

    #[test]
    fn rejects_scenario_without_a_target() {
        // A scenario is a sub-selection of the current target, so it requires
        // a target write (feature + path).
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let args = WriteSessionArgs {
            feature: None,
            path: None,
            scenario: Some("x".into()),
            scenario_path: Some("specs/x/scenarios/y.md".into()),
            cli_config_dir: Some(".pi".into()),
            clear: false,
        };
        let err = run_with_now(&args, tmp.path(), fixed_now()).unwrap_err();
        match err {
            PrimitiveError::MissingArgument {
                primitive,
                argument,
                ..
            } => {
                assert_eq!(primitive, "write-session");
                assert_eq!(argument, "feature");
            }
            other => panic!("expected MissingArgument, got {other:?}"),
        }
    }

    #[test]
    fn clearing_scenario_omits_both_keys() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        // First write with a scenario set.
        let mut with_scenario = base_args();
        with_scenario.scenario = Some("write-session-primitive".into());
        with_scenario.scenario_path =
            Some("specs/022-deterministic-runtime/scenarios/write-session-primitive.md".into());
        run_with_now(&with_scenario, tmp.path(), fixed_now()).unwrap();

        // Then overwrite without — both keys must vanish.
        run_with_now(&base_args(), tmp.path(), fixed_now()).unwrap();
        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert!(!body.contains("scenario"), "{body}");
        assert!(!body.contains("scenario-path"), "{body}");
    }

    #[test]
    fn clear_removes_target_and_preserves_cli_config_dir() {
        // target.md's `--clear`: the target block vanishes, the
        // per-contributor `cli-config-dir` survives.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            "feature = \"001-x\"\npath = \"specs/001-x\"\nscenario = \"y\"\nscenario-path = \"specs/001-x/scenarios/y.md\"\nset-at = \"2026-01-01T00:00:00Z\"\ncli-config-dir = \".pi\"\n",
        )
        .unwrap();

        let result = run_with_now(&clear_args(), tmp.path(), fixed_now()).unwrap();
        assert!(!result.created, "existing file is overwritten, not created");
        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert_eq!(
            body, "cli-config-dir = \".pi\"\n",
            "only cli-config-dir survives a clear: {body}"
        );
    }

    #[test]
    fn clear_with_cli_config_dir_override_applies_supplied_value() {
        // `cli-config-dir` is not a target field; supplied alongside
        // `clear`, it overrides the preserved value.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            "feature = \"001-x\"\npath = \"specs/001-x\"\nset-at = \"2026-01-01T00:00:00Z\"\ncli-config-dir = \".claude\"\n",
        )
        .unwrap();
        let mut args = clear_args();
        args.cli_config_dir = Some(".pi".into());
        run_with_now(&args, tmp.path(), fixed_now()).unwrap();
        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert_eq!(body, "cli-config-dir = \".pi\"\n");
    }

    #[test]
    fn clear_on_session_without_cli_config_dir_writes_empty_file() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            "feature = \"001-x\"\npath = \"specs/001-x\"\nset-at = \"2026-01-01T00:00:00Z\"\n",
        )
        .unwrap();
        run_with_now(&clear_args(), tmp.path(), fixed_now()).unwrap();
        let body = fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap();
        assert_eq!(body, "", "nothing to preserve → empty session file");
    }

    #[test]
    fn clear_rejects_target_write_arguments() {
        // Mutual exclusion: `clear` combined with any target field is a
        // supplied-and-rejected InvalidArgument, and nothing is written.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let mut args = base_args();
        args.clear = true;
        let err = run_with_now(&args, tmp.path(), fixed_now()).unwrap_err();
        match err {
            PrimitiveError::InvalidArgument {
                primitive,
                argument,
                ..
            } => {
                assert_eq!(primitive, "write-session");
                assert_eq!(argument, "clear");
            }
            other => panic!("expected InvalidArgument, got {other:?}"),
        }
        assert!(!tmp.path().join(".ductus/session.toml").exists());
    }

    #[test]
    fn rejects_scenario_without_scenario_path() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let mut args = base_args();
        args.scenario = Some("orphan".into());
        let err = run_with_now(&args, tmp.path(), fixed_now()).unwrap_err();
        match err {
            PrimitiveError::MissingArgument {
                primitive,
                argument,
                ..
            } => {
                assert_eq!(primitive, "write-session");
                assert_eq!(argument, "scenario-path");
            }
            other => panic!("expected MissingArgument, got {other:?}"),
        }
        // Disk is unchanged (no file created).
        assert!(!tmp.path().join(".ductus/session.toml").exists());
    }

    #[test]
    fn rejects_scenario_path_without_scenario() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let mut args = base_args();
        args.scenario_path = Some("specs/x/scenarios/y.md".into());
        let err = run_with_now(&args, tmp.path(), fixed_now()).unwrap_err();
        match err {
            PrimitiveError::MissingArgument {
                primitive,
                argument,
                ..
            } => {
                assert_eq!(primitive, "write-session");
                assert_eq!(argument, "scenario");
            }
            other => panic!("expected MissingArgument, got {other:?}"),
        }
    }

    #[test]
    fn rejects_path_with_parent_component() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let mut args = base_args();
        args.path = Some("specs/../escape".into());
        let err = run_with_now(&args, tmp.path(), fixed_now()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }

    #[test]
    fn rejects_absolute_scenario_path() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let mut args = base_args();
        args.scenario = Some("x".into());
        args.scenario_path = Some("/etc/passwd".into());
        let err = run_with_now(&args, tmp.path(), fixed_now()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }

    /// The write replaces the session file by renaming a new one over it,
    /// never by rewriting it in place, so a reader outside the lock meets
    /// the old file or the new one and never a torn one. The path names a
    /// new inode afterwards; an in-place rewrite would keep the old one. The
    /// old file is held open, so its inode cannot be reused meanwhile.
    #[cfg(unix)]
    #[test]
    fn a_write_replaces_the_session_file_rather_than_rewriting_it() {
        use std::os::unix::fs::MetadataExt as _;
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let session_path = tmp.path().join(".ductus/session.toml");
        fs::write(&session_path, "feature = \"before\"\n").unwrap();
        let held = fs::File::open(&session_path).unwrap();
        let before = held.metadata().unwrap().ino();
        run_with_now(&base_args(), tmp.path(), fixed_now()).unwrap();
        assert_ne!(fs::metadata(&session_path).unwrap().ino(), before);
        assert!(
            fs::read_to_string(&session_path)
                .unwrap()
                .contains("022-deterministic-runtime")
        );
    }

    #[test]
    fn iso8601_utc_formats_known_epoch() {
        // 0 → 1970-01-01T00:00:00Z
        assert_eq!(iso8601_utc(UNIX_EPOCH), "1970-01-01T00:00:00Z");
        // 1700000000 → 2023-11-14T22:13:20Z (a well-known epoch).
        assert_eq!(
            iso8601_utc(UNIX_EPOCH + Duration::from_secs(1_700_000_000)),
            "2023-11-14T22:13:20Z"
        );
        // Our fixed test moment.
        assert_eq!(iso8601_utc(fixed_now()), "2026-05-23T12:34:56Z");
    }

    #[test]
    fn civil_from_days_handles_leap_years() {
        // 2024-02-29 — 2024 is a leap year.
        let days = day_count(2024, 2, 29);
        assert_eq!(civil_from_days(days), (2024, 2, 29));
        // 2100-03-01 — 2100 is NOT a leap year (divisible by 100, not 400).
        let days = day_count(2100, 3, 1);
        assert_eq!(civil_from_days(days), (2100, 3, 1));
        // 2000-02-29 — 2000 IS a leap year.
        let days = day_count(2000, 2, 29);
        assert_eq!(civil_from_days(days), (2000, 2, 29));
    }

    // -- spec 062: identified writes ---------------------------------------

    const CANONICAL_DEFAULT: &str = "feature = \"022-deterministic-runtime\"\n\
                                     path = \"specs/022-deterministic-runtime\"\n\
                                     set-at = \"2026-05-23T12:34:56Z\"\n";

    #[test]
    fn a_single_unidentified_agent_writes_todays_bytes_and_nothing_else() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let result = run_with_now(&base_args(), tmp.path(), fixed_now()).unwrap();
        assert_eq!(
            fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap(),
            CANONICAL_DEFAULT
        );
        assert_eq!((result.identity, result.own_path), (None, None));
        // The write holds the session lock (`BE-TXN-002`), so the directory
        // holds the lock and its self-ignoring `.gitignore` — and no target.
        let mut entries: Vec<String> = fs::read_dir(tmp.path().join(".ductus/sessions"))
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        entries.sort();
        assert_eq!(
            entries,
            [".gitignore", ".lock"],
            "no per-process state for a single unidentified agent"
        );
    }

    #[test]
    fn an_identified_target_write_writes_its_own_file_and_the_same_default() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let review = Identity::named("review").unwrap();
        let result = run_as(&base_args(), tmp.path(), Some(&review), fixed_now()).unwrap();

        assert_eq!(result.path, ".ductus/session.toml");
        assert_eq!(result.identity.as_deref(), Some("review"));
        assert_eq!(
            result.own_path.as_deref(),
            Some(".ductus/sessions/review.toml")
        );
        assert_eq!(
            fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap(),
            CANONICAL_DEFAULT,
            "the default is byte-identical to an unidentified write"
        );
        let own = fs::read_to_string(tmp.path().join(".ductus/sessions/review.toml")).unwrap();
        assert!(
            own.contains("feature = \"022-deterministic-runtime\""),
            "{own}"
        );
        assert!(own.contains("used-at = \"2026-05-23T12:34:56Z\""), "{own}");
    }

    #[test]
    fn an_identified_clear_write_clears_its_own_file_and_keeps_cli_config_dir() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/session.toml"),
            "feature = \"old\"\npath = \"specs/old\"\ncli-config-dir = \".claude\"\n",
        )
        .unwrap();
        let review = Identity::named("review").unwrap();
        run_as(&base_args(), tmp.path(), Some(&review), fixed_now()).unwrap();
        run_as(&clear_args(), tmp.path(), Some(&review), fixed_now()).unwrap();

        assert_eq!(
            fs::read_to_string(tmp.path().join(".ductus/session.toml")).unwrap(),
            "cli-config-dir = \".claude\"\n"
        );
        let own = fs::read_to_string(tmp.path().join(".ductus/sessions/review.toml")).unwrap();
        assert!(own.contains("cleared = true"), "{own}");
        assert!(!own.contains("feature"), "{own}");
    }

    #[test]
    fn an_identified_writer_is_told_who_else_targets_the_feature() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let (a, b) = (Identity::named("a").unwrap(), Identity::named("b").unwrap());
        run_as(&base_args(), tmp.path(), Some(&a), fixed_now()).unwrap();
        let result = run_as(&base_args(), tmp.path(), Some(&b), fixed_now()).unwrap();
        assert_eq!(result.peers.len(), 1);
        assert_eq!(result.peers[0].session, "a");
    }

    /// Round-trip helper: count days since 1970-01-01 for a known date.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn day_count(year: i64, month: u32, day: u32) -> i64 {
        let y = year - i64::from(month <= 2);
        let era = if y >= 0 { y / 400 } else { (y - 399) / 400 };
        let yoe = (y - era * 400) as u64;
        let m = u64::from(month);
        let d = u64::from(day);
        let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe as i64 - 719_468
    }
}
