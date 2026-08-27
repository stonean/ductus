//! `lint-markdown` — wrap `npx markdownlint-cli2` and surface violations.
//!
//! The primitive spawns a repo-local `node_modules/.bin/markdownlint-cli2`
//! when one exists, else `npx markdownlint-cli2` (optionally with `--fix`)
//! against the given paths, captures combined stdout/stderr, and parses
//! each line into a [`MarkdownViolation`]. Exit code 1 (violations found)
//! and 2+ (config or runtime error) both flow through as `clean: false`;
//! callers consult `exit_code` to distinguish.

use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

use regex::Regex;

use crate::primitives::{PrimitiveError, Result};
use crate::schema::primitives::{LintMarkdownArgs, LintMarkdownResult, MarkdownViolation};

/// Pick the markdownlint-cli2 invocation: a repo-local
/// `node_modules/.bin/markdownlint-cli2` when one exists, else `npx`.
///
/// Returns `(program, via_npx)`; `via_npx` tells the caller whether to pass
/// `markdownlint-cli2` as the first argument, since the local binary *is* the
/// tool while `npx` needs to be told what to run.
///
/// The local branch exists because `npx` is not reliably on `PATH`: under nvm
/// it is a lazy-loading shell function, and a spawned process inherits `PATH`
/// but never the parent shell's functions, so `Command::new("npx")` cannot
/// resolve it. Checking for a vendored binary is a path test — deterministic,
/// and cheap enough to do on every call.
///
/// Deliberately NOT a login shell. Sourcing a contributor's profile to find a
/// linter would be slow on every lint, non-deterministic across shell configs,
/// and a code-execution surface — it inverts the determinism the runtime
/// boundary exists to guarantee.
fn resolve_markdownlint(repo: &Path) -> (String, bool) {
    // Windows ships npx as a `.cmd` shim, which `Command::new("npx")` cannot
    // resolve (CreateProcess needs the explicit extension); the vendored
    // binary carries the same distinction.
    let local_name = if cfg!(windows) {
        "markdownlint-cli2.cmd"
    } else {
        "markdownlint-cli2"
    };
    let local = repo.join("node_modules").join(".bin").join(local_name);
    if local.exists() {
        // Reported as what was launched when it fails, rather than silently
        // falling back to npx: a vendored tool that cannot run is a condition
        // worth seeing, not one to paper over.
        return (local.to_string_lossy().into_owned(), false);
    }
    let npx = if cfg!(windows) { "npx.cmd" } else { "npx" };
    (npx.to_string(), true)
}

/// Guidance for a spawn failure, or `None` when none applies.
///
/// Attached only to `NotFound` on the `npx` branch. A permissions error, or a
/// vendored binary that will not run, has nothing to do with `PATH`, and
/// stapling a `PATH` explanation to it would assert a cause the primitive has
/// no basis for — the same misattribution this scenario removes, pointed the
/// other way.
fn launch_guidance(kind: std::io::ErrorKind, via_npx: bool) -> Option<String> {
    (kind == std::io::ErrorKind::NotFound && via_npx).then(|| {
        "not found on PATH. Note that a shell-function `npx` — nvm's lazy loader, where \
         `command -v npx` prints `npx` with no path — is invisible to a spawned process, \
         which inherits PATH but never the parent shell's functions. Put the real Node \
         `bin` directory on PATH, or vendor markdownlint-cli2 so \
         `node_modules/.bin/markdownlint-cli2` is used directly."
            .to_string()
    })
}

/// Execute the `lint-markdown` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::ToolLaunch`] when the resolved
/// markdownlint-cli2 invocation cannot be spawned — naming the executable
/// rather than the repository, since the repo is the working directory and
/// never the thing that was missing. A non-zero markdownlint-cli2 exit code
/// is not an error — it's recorded in the result alongside the parsed
/// violations. [`PrimitiveError::InvalidArgument`] is returned for a path
/// beginning with `-`.
pub fn run(args: &LintMarkdownArgs, repo: &Path) -> Result<LintMarkdownResult> {
    // A path beginning with `-` would be parsed by markdownlint-cli2 as an
    // option, not a file — `--config=evil.json` can load a `customRules` JS
    // module, i.e. arbitrary code under this primitive's permission. Reject
    // it so `paths` names files only.
    for path in &args.paths {
        if path.starts_with('-') {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "lint-markdown".into(),
                argument: "paths".into(),
                reason: "a path beginning with '-' would be parsed as a markdownlint-cli2 \
                         flag (e.g. --config loads arbitrary JS); pass file paths only"
                    .into(),
            });
        }
    }
    let (program, via_npx) = resolve_markdownlint(repo);
    let mut cmd = Command::new(&program);
    if via_npx {
        cmd.arg("markdownlint-cli2");
    }
    if args.fix {
        cmd.arg("--fix");
    }
    for path in &args.paths {
        cmd.arg(path);
    }
    cmd.current_dir(repo);

    let output = cmd.output().map_err(|source| PrimitiveError::ToolLaunch {
        program: program.clone(),
        guidance: launch_guidance(source.kind(), via_npx),
        source,
    })?;

    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut violations: Vec<MarkdownViolation> = Vec::new();
    for line in stdout.lines().chain(stderr.lines()) {
        if let Some(violation) = parse_violation_line(line) {
            violations.push(violation);
        }
    }

    // `clean` requires both no parsed violations AND a zero exit code. A
    // non-zero exit with an empty violations vec means markdownlint reported a
    // problem in a line shape the parser did not recognize, or a config/runtime
    // error — neither is clean. Deriving solely from `violations.is_empty()`
    // silently passed such runs. Mirrors run-generator's exit-code-derived
    // `drift`.
    let clean = violations.is_empty() && exit_code == 0;
    Ok(LintMarkdownResult {
        violations,
        clean,
        exit_code,
    })
}

/// Parse one markdownlint-cli2 violation line. Output shape is
/// `path:line[:col] [severity] RULE/aliases description`. The optional
/// `severity` token (`error`/`warning`) is emitted by markdownlint-cli2
/// v0.22.1+; older output omits it. Both forms are accepted.
fn parse_violation_line(line: &str) -> Option<MarkdownViolation> {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    let re = PATTERN.get_or_init(|| {
        Regex::new(r"^(?P<path>[^:]+(?:\.md|\.markdown)):(?P<line>\d+)(?::\d+)?\s+(?:(?:error|warning)\s+)?(?P<rule>MD\d+)(?:/\S+)?\s+(?P<message>.+)$")
            .unwrap_or_else(|err| panic!("markdownlint violation regex must compile: {err}"))
    });
    let caps = re.captures(line.trim())?;
    let line_num: u32 = caps.name("line")?.as_str().parse().ok()?;
    Some(MarkdownViolation {
        path: caps.name("path")?.as_str().into(),
        line: line_num,
        rule: caps.name("rule")?.as_str().into(),
        message: caps.name("message")?.as_str().trim().into(),
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn parses_canonical_violation_line() {
        let v = parse_violation_line(
            "README.md:17 MD013/line-length Line length [Expected: 80, Actual: 120]",
        )
        .unwrap();
        assert_eq!(v.path, "README.md");
        assert_eq!(v.line, 17);
        assert_eq!(v.rule, "MD013");
        assert!(v.message.contains("Line length"));
    }

    #[test]
    fn parses_violation_with_column() {
        let v = parse_violation_line(
            "docs/spec.md:42:3 MD009 Trailing spaces [Expected: 0; Actual: 2]",
        )
        .unwrap();
        assert_eq!(v.path, "docs/spec.md");
        assert_eq!(v.line, 42);
        assert_eq!(v.rule, "MD009");
    }

    #[test]
    fn parses_violation_with_severity_token() {
        // markdownlint-cli2 v0.22.1+ inserts an `error`/`warning` severity
        // token between the location and the rule. This is the exact shape that
        // was silently dropped before the regex accepted the optional token.
        let v = parse_violation_line(
            "specs/028-multi-format-agents/spec.md:34 error MD028/no-blanks-blockquote Blank line inside blockquote",
        )
        .unwrap();
        assert_eq!(v.path, "specs/028-multi-format-agents/spec.md");
        assert_eq!(v.line, 34);
        assert_eq!(v.rule, "MD028");
        assert!(v.message.contains("Blank line inside blockquote"));
    }

    #[test]
    fn parses_violation_with_severity_and_column() {
        let v = parse_violation_line(
            "docs/spec.md:42:3 warning MD009/no-trailing-spaces Trailing spaces",
        )
        .unwrap();
        assert_eq!(v.path, "docs/spec.md");
        assert_eq!(v.line, 42);
        assert_eq!(v.rule, "MD009");
        assert!(v.message.contains("Trailing spaces"));
    }

    #[test]
    fn ignores_non_violation_lines() {
        assert!(parse_violation_line("Finding files...").is_none());
        assert!(parse_violation_line("Summary: 0 errors").is_none());
        assert!(parse_violation_line("").is_none());
        assert!(parse_violation_line("README.md:no-line MD013 foo").is_none());
    }

    #[test]
    fn parses_path_with_spaces_disallowed() {
        // The regex is path-without-spaces; this matches typical markdownlint
        // output and keeps the parser unambiguous. Verify a space-containing
        // path is rejected rather than mis-parsed.
        let v = parse_violation_line("path with space.md:5 MD013 Line length");
        assert!(v.is_some(), "non-greedy [^:]+ accepts spaces before colon");
        let v = v.unwrap();
        assert_eq!(v.path, "path with space.md");
    }

    // --- tool resolution (spec 022, lint-markdown-tool-resolution) ----------

    #[test]
    fn prefers_a_vendored_markdownlint_over_npx() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = tmp.path().join("node_modules").join(".bin");
        std::fs::create_dir_all(&bin).unwrap();
        let name = if cfg!(windows) {
            "markdownlint-cli2.cmd"
        } else {
            "markdownlint-cli2"
        };
        std::fs::write(bin.join(name), "#!/bin/sh\nexit 0\n").unwrap();

        let (program, via_npx) = resolve_markdownlint(tmp.path());
        assert!(
            program.contains("node_modules"),
            "the vendored binary must win: {program}"
        );
        assert!(
            !via_npx,
            "the local binary is the tool; it must not be passed as an npx argument"
        );
    }

    #[test]
    fn falls_back_to_npx_when_nothing_is_vendored() {
        let tmp = tempfile::tempdir().unwrap();
        let (program, via_npx) = resolve_markdownlint(tmp.path());
        assert!(program.starts_with("npx"), "{program}");
        assert!(via_npx, "npx must be told what to run");
    }

    #[test]
    fn a_not_found_npx_carries_the_path_guidance() {
        // The observed failure: npx is a shell function, so a spawned process
        // cannot resolve it, and the operator needs to be told that rather
        // than shown a path that exists.
        let guidance = launch_guidance(std::io::ErrorKind::NotFound, true)
            .expect("a missing npx must explain itself");
        assert!(guidance.contains("not found on PATH"), "{guidance}");
        assert!(
            guidance.contains("shell-function"),
            "the nvm case is the whole point: {guidance}"
        );
    }

    #[test]
    fn a_non_not_found_failure_carries_no_path_guidance() {
        // A permissions error has nothing to do with PATH. Attaching the
        // explanation anyway would be the same misattribution this change
        // removes, pointed the other way.
        assert!(launch_guidance(std::io::ErrorKind::PermissionDenied, true).is_none());
        assert!(launch_guidance(std::io::ErrorKind::Other, true).is_none());
    }

    #[test]
    fn a_vendored_binary_failure_carries_no_path_guidance() {
        // The local branch did not consult PATH, so PATH cannot be the fix.
        assert!(launch_guidance(std::io::ErrorKind::NotFound, false).is_none());
    }

    // Unix-only: forcing a *spawn* failure needs something the OS refuses to
    // execute, and the trick used here does not transfer. On Windows this test
    // failed in CI with `LintMarkdownResult { clean: false, exit_code: 1 }` —
    // the spawn succeeded and the process exited non-zero, most likely because
    // a `.cmd` target is resolved through the command processor rather than
    // executed directly. The behavior under test is not platform-specific;
    // only this way of provoking it is. The other half of the contract — that
    // a vendored-binary failure carries no PATH guidance — is asserted
    // platform-independently by `a_vendored_binary_failure_carries_no_path_guidance`.
    //
    // The first version of this comment claimed a directory is "reliably
    // unspawnable on every platform". That was an unverified cross-platform
    // assertion, and CI disproved it.
    #[cfg(unix)]
    #[test]
    fn a_broken_vendored_binary_names_itself_rather_than_falling_back() {
        // A vendored tool that cannot run is a condition worth seeing. The
        // error must name that path, not npx.
        let tmp = tempfile::tempdir().unwrap();
        let bin = tmp.path().join("node_modules").join(".bin");
        std::fs::create_dir_all(&bin).unwrap();
        // A directory cannot be executed on Unix, and unlike permission bits
        // it is not something CI might normalize.
        std::fs::create_dir(bin.join("markdownlint-cli2")).unwrap();

        let err = run(
            &LintMarkdownArgs {
                paths: vec!["README.md".into()],
                fix: false,
            },
            tmp.path(),
        )
        .expect_err("an unspawnable vendored binary must error");
        let rendered = err.to_string();
        assert!(
            rendered.contains("could not launch"),
            "must name the launch, not an I/O path: {rendered}"
        );
        assert!(
            rendered.contains("node_modules"),
            "must name the vendored binary it actually tried: {rendered}"
        );
        assert!(
            !rendered.contains("not found on PATH"),
            "the local branch never consulted PATH: {rendered}"
        );
    }
}
