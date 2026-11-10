//! `ductus hook` — what Claude Code's `UserPromptExpansion` hooks run to answer
//! a typed `/{project}:status` or `/{project}:help` with no model turn (spec
//! 080). Their input, output and exit statuses are spec 080's `data-model.md`,
//! `ductus hook status` and `ductus hook help`.
//!
//! A host surface, not a primitive: it adapts one host's hook protocol, and the
//! capabilities it adapts are served on MCP, the pipeline view by
//! `render-status` and the guide by `render-help` (§runtime-boundary principle
//! 5). The view is built from the same read `dashboard` makes
//! ([`dashboard::view_model`]) and rendered as fixed-width text, since the hook's reason is
//! shown in a terminal that renders no markdown (AC17). The guide is shown as
//! the help copy writes it ([`render_help::guide`]), since the scenario asks
//! for the guide the model path prints.

use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::Serialize;

use crate::primitives::check_artifact_size::resolve_threshold;
use crate::primitives::{Result, dashboard, pipeline_view, render_help};
use crate::session;

/// The variable Claude Code sets for each command hook it runs
/// (code.claude.com/docs/en/hooks), to the directory the session started in.
/// The hook reads it as the project root, since sessions start at the project
/// root (spec 080). Optional: unset or empty, the working directory is the
/// root.
pub const PROJECT_DIR_VAR: &str = "CLAUDE_PROJECT_DIR";

/// The Claude Code hook event the hooks answer, under which
/// `merge-status-hook` installs their entries.
pub const HOOK_EVENT: &str = "UserPromptExpansion";

/// The project root the hook acts on: `project_dir`, the startup value of
/// [`PROJECT_DIR_VAR`], when it is set and non-empty, and `cwd` otherwise.
#[must_use]
pub fn project_root(project_dir: Option<std::ffi::OsString>, cwd: PathBuf) -> PathBuf {
    project_dir
        .filter(|dir| !dir.is_empty())
        .map_or(cwd, PathBuf::from)
}

/// The response that blocks the typed command and shows the answer as its
/// reason. Fields serialize in declaration order.
#[derive(Debug, Serialize)]
pub struct BlockResponse {
    /// Always `block`: the prompt never reaches the model.
    pub decision: &'static str,
    /// The answer: the view as fixed-width text, or the guide.
    pub reason: String,
    /// The event-specific half of the response.
    #[serde(rename = "hookSpecificOutput")]
    pub hook_specific_output: HookSpecificOutput,
}

/// `hookSpecificOutput` for a `UserPromptExpansion` block.
#[derive(Debug, Serialize)]
pub struct HookSpecificOutput {
    /// Always `UserPromptExpansion`.
    #[serde(rename = "hookEventName")]
    pub hook_event_name: &'static str,
    /// Always `true`, so the shown view does not end with an
    /// "Original prompt:" line (AC16).
    #[serde(rename = "suppressOriginalPrompt")]
    pub suppress_original_prompt: bool,
}

impl BlockResponse {
    /// The block response showing `reason`.
    fn showing(reason: String) -> Self {
        Self {
            decision: "block",
            reason,
            hook_specific_output: HookSpecificOutput {
                hook_event_name: HOOK_EVENT,
                suppress_original_prompt: true,
            },
        }
    }
}

/// Read the hook input Claude Code writes to stdin to its end, so the host
/// never writes into a closed pipe. No field of it is used: arguments typed
/// after the command are ignored, as the model path ignores them. A failure to
/// read it changes nothing the answer shows, so it is not an error.
fn drain(input: &mut impl Read) {
    let _ = io::copy(input, &mut io::sink());
}

/// Answer status's hook for the project at `root`, `input` being the hook
/// input Claude Code writes to stdin, which is drained and not used.
///
/// The CLI prints the response and exits 0, or prints the error on stderr,
/// nothing on stdout, and exits 1 (AC25), as for [`help`]. Exit 1 is a
/// non-blocking hook error, after which Claude Code runs the command on the
/// model path. Exit 2 would block the expansion and show nothing, so the hook
/// never uses it.
///
/// The view is held to `[artifacts] read-size-bytes` measured on the response
/// as printed, so a long view is cut as the model path's is (AC24). No file
/// holds the whole view here: the response is shown to a person, and never
/// reaches the model.
///
/// # Errors
///
/// An invalid `DUCTUS_SESSION`, and every error building the view returns:
/// those of `dashboard`, and [`crate::primitives::PrimitiveError::InvalidReadSize`]
/// for a malformed `[artifacts] read-size-bytes`.
pub fn status(root: &Path, input: &mut impl Read) -> Result<BlockResponse> {
    drain(input);
    let identity = session::process_identity()?;
    let (model, _project) = dashboard::view_model(root, identity.as_ref(), SystemTime::now())?;
    let (threshold, _) = resolve_threshold(root)?;
    let fits = |view: &str| {
        serde_json::to_vec(&BlockResponse::showing(view.to_string()))
            .is_ok_and(|bytes| u64::try_from(bytes.len()).unwrap_or(u64::MAX) <= threshold.bytes)
    };
    Ok(BlockResponse::showing(pipeline_view::render(
        &model,
        pipeline_view::Format::Text,
        &fits,
        None,
    )))
}

/// Answer help's hook for the project at `root`, `input` being drained and not
/// used, with the guide the installed help copy holds. The exits are
/// [`status`]'s.
///
/// # Errors
///
/// Every error [`render_help::guide`] returns, among them no installed help
/// copy.
pub fn help(root: &Path, input: &mut impl Read) -> Result<BlockResponse> {
    drain(input);
    Ok(BlockResponse::showing(render_help::guide(root)?))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::ffi::OsString;

    use super::*;

    #[test]
    fn the_project_dir_is_the_root_when_set() {
        let root = project_root(Some(OsString::from("/project")), PathBuf::from("/cwd"));
        assert_eq!(root, PathBuf::from("/project"));
    }

    #[test]
    fn an_unset_or_empty_project_dir_leaves_the_working_directory() {
        assert_eq!(
            project_root(None, PathBuf::from("/cwd")),
            PathBuf::from("/cwd")
        );
        assert_eq!(
            project_root(Some(OsString::new()), PathBuf::from("/cwd")),
            PathBuf::from("/cwd")
        );
    }

    /// A malformed read size halts the view, as it halts every reader of the
    /// setting, rather than rendering at the default.
    #[test]
    fn a_malformed_read_size_halts_the_view() {
        let tmp = tempfile::tempdir().unwrap();
        for (rel, body) in [
            (".ductus/config.toml", "[artifacts]\nread-size-bytes = 0\n"),
            (
                "specs/001-alpha/spec.md",
                "---\nstatus: draft\ndependencies: []\n---\n\n# A\n",
            ),
        ] {
            let path = tmp.path().join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, body).unwrap();
        }
        let err = status(tmp.path(), &mut io::empty()).unwrap_err();
        assert!(
            matches!(
                err,
                crate::primitives::PrimitiveError::InvalidReadSize { .. }
            ),
            "{err:?}"
        );
    }
}
