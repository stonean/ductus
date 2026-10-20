//! `run-generator` — invoke a bash generator with `--dry-run` and report drift.
//!
//! The primitive distinguishes two outcome categories per §partial-failure
//! semantics: a non-zero exit from the script is **drift** (a domain finding,
//! procedure continues), while a failure to spawn `bash` at all is an
//! operational error that halts the procedure.

use std::path::Path;
use std::process::Command;
use std::time::Duration;

use crate::primitives::{PrimitiveError, Result, output_within, resolve_path};
use crate::schema::primitives::{RunGeneratorArgs, RunGeneratorResult};

/// How long a generator may run before it is stopped (`BE-TIMEOUT-001`). A
/// generator finishes in seconds; five minutes is room for a slow machine,
/// and a hung script no longer holds its caller forever.
pub(crate) const GENERATOR_TIMEOUT: Duration = Duration::from_secs(300);

/// Execute the `run-generator` primitive.
///
/// Spawns `bash <repo>/<script> --dry-run` from `repo` as the working
/// directory. The script's stdout and stderr are captured into the result.
/// A non-zero exit is recorded as `drift: true` rather than an error.
///
/// # Errors
///
/// Returns [`PrimitiveError::Io`] when `bash` cannot be spawned or the
/// script path does not exist, and [`PrimitiveError::ToolTimedOut`] when the
/// script outlives [`GENERATOR_TIMEOUT`] and is stopped.
pub fn run(args: &RunGeneratorArgs, repo: &Path) -> Result<RunGeneratorResult> {
    run_within(args, repo, GENERATOR_TIMEOUT)
}

/// [`run`] under a caller-chosen timeout — the seam a test uses to prove a
/// hung script is stopped without waiting five minutes.
pub(crate) fn run_within(
    args: &RunGeneratorArgs,
    repo: &Path,
    timeout: Duration,
) -> Result<RunGeneratorResult> {
    // The script is passed straight to `bash`, so a caller who allowlists
    // this primitive (mental model: "runs the repo's generator scripts")
    // would otherwise be granting "run any bash script at any path". Bound
    // it to the repo — absolute paths and `..` escapes are refused — so the
    // permission means what its name implies.
    super::validate_contained(repo, &args.script)?;
    let script_path = resolve_path(repo, &args.script);
    if !script_path.exists() {
        return Err(PrimitiveError::Io {
            path: script_path,
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "script not found"),
        });
    }

    let output = output_within(
        Command::new("bash")
            .arg(&script_path)
            .arg("--dry-run")
            .current_dir(repo),
        timeout,
    )
    .map_err(|source| PrimitiveError::Io {
        path: script_path.clone(),
        source,
    })?
    .ok_or_else(|| PrimitiveError::ToolTimedOut {
        program: format!("bash {}", args.script),
        seconds: timeout.as_secs(),
    })?;

    let exit_code = output.status.code().unwrap_or(-1);
    Ok(RunGeneratorResult {
        drift: exit_code != 0,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        exit_code,
    })
}

#[cfg(all(test, unix))]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    fn write_script(path: &Path, body: &str) {
        fs::write(path, body).unwrap();
        let mut perms = fs::metadata(path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).unwrap();
    }

    #[test]
    fn a_script_that_outlives_its_timeout_is_stopped() {
        let tmp = tempdir().unwrap();
        write_script(&tmp.path().join("gen.sh"), "#!/usr/bin/env bash\nsleep 5\n");
        let started = std::time::Instant::now();
        let err = run_within(
            &RunGeneratorArgs {
                script: "gen.sh".into(),
            },
            tmp.path(),
            Duration::from_millis(200),
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::ToolTimedOut { .. }), "{err}");
        assert!(
            started.elapsed() < Duration::from_secs(4),
            "the script was stopped, not waited for"
        );
    }

    #[test]
    fn clean_script_reports_no_drift() {
        let tmp = tempdir().unwrap();
        let script = tmp.path().join("gen.sh");
        write_script(&script, "#!/usr/bin/env bash\necho clean\nexit 0\n");
        let result = run(
            &RunGeneratorArgs {
                script: "gen.sh".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.drift);
        assert_eq!(result.exit_code, 0);
        assert!(result.stdout.contains("clean"));
        assert_eq!(result.stderr, "");
    }

    #[test]
    fn non_zero_exit_is_drift_not_error() {
        let tmp = tempdir().unwrap();
        let script = tmp.path().join("gen.sh");
        write_script(
            &script,
            "#!/usr/bin/env bash\necho 'spec out of sync' >&2\nexit 1\n",
        );
        let result = run(
            &RunGeneratorArgs {
                script: "gen.sh".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(result.drift);
        assert_eq!(result.exit_code, 1);
        assert!(result.stderr.contains("spec out of sync"));
    }

    #[test]
    fn passes_dry_run_flag_to_script() {
        let tmp = tempdir().unwrap();
        let script = tmp.path().join("gen.sh");
        write_script(
            &script,
            "#!/usr/bin/env bash\nif [[ \"$1\" == \"--dry-run\" ]]; then exit 0; else exit 7; fi\n",
        );
        let result = run(
            &RunGeneratorArgs {
                script: "gen.sh".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.drift, "expected --dry-run to be passed");
    }

    #[test]
    fn missing_script_is_operational_error() {
        let tmp = tempdir().unwrap();
        let err = run(
            &RunGeneratorArgs {
                script: "no-such-script.sh".into(),
            },
            tmp.path(),
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::Io { .. }));
    }
}
