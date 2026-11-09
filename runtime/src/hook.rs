//! `ductus hook status` — what Claude Code's `UserPromptExpansion` hook runs to
//! answer a typed `/{project}:status` with no model turn (spec 080). Its input,
//! output and exit statuses are spec 080's `data-model.md`, `ductus hook status`.
//!
//! A host surface, not a primitive: it adapts one host's hook protocol, and the
//! capability it adapts, the pipeline view, is served on MCP by `render-status`
//! (§runtime-boundary principle 5). The view is rendered from the same read
//! `dashboard` makes ([`dashboard::render_as`]), as fixed-width text, since the
//! hook's reason is shown in a terminal that renders no markdown (AC17).

use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::Serialize;

use crate::primitives::{Result, dashboard, pipeline_view};
use crate::session;

/// The variable Claude Code sets for each command hook it runs
/// (code.claude.com/docs/en/hooks), to the directory the session started in.
/// The hook reads it as the project root, since sessions start at the project
/// root (spec 080). Optional: unset or empty, the working directory is the
/// root.
pub const PROJECT_DIR_VAR: &str = "CLAUDE_PROJECT_DIR";

/// The project root the hook acts on: `project_dir`, the startup value of
/// [`PROJECT_DIR_VAR`], when it is set and non-empty, and `cwd` otherwise.
#[must_use]
pub fn project_root(project_dir: Option<std::ffi::OsString>, cwd: PathBuf) -> PathBuf {
    project_dir
        .filter(|dir| !dir.is_empty())
        .map_or(cwd, PathBuf::from)
}

/// The response that blocks the typed command and shows the view as its
/// reason. Fields serialize in declaration order.
#[derive(Debug, Serialize)]
pub struct BlockResponse {
    /// Always `block`: the prompt never reaches the model.
    pub decision: &'static str,
    /// The view, as fixed-width text.
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

/// Answer the hook for the project at `root`.
///
/// `input` is the hook input Claude Code writes to stdin. It is read to its
/// end, so the host never writes into a closed pipe, and no field of it is
/// used: arguments typed after the command are ignored, as the model path
/// ignores them. A failure to read it changes nothing the view shows, so it
/// is not an error.
///
/// The CLI prints the response and exits 0, or prints the error on stderr,
/// nothing on stdout, and exits 1 (AC25). Exit 1 is a non-blocking hook
/// error, after which Claude Code runs the command on the model path. Exit 2
/// would block the expansion and show nothing, so the hook never uses it.
///
/// # Errors
///
/// An invalid `DUCTUS_SESSION`, and every error rendering the view returns:
/// those of `dashboard`, and [`crate::primitives::PrimitiveError::InvalidReadSize`]
/// for a malformed `[artifacts] read-size-bytes`.
pub fn status(root: &Path, input: &mut impl Read) -> Result<BlockResponse> {
    let _ = io::copy(input, &mut io::sink());
    let identity = session::process_identity()?;
    let (view, _project) = dashboard::render_as(
        root,
        identity.as_ref(),
        SystemTime::now(),
        pipeline_view::Format::Text,
    )?;
    Ok(BlockResponse {
        decision: "block",
        reason: view,
        hook_specific_output: HookSpecificOutput {
            hook_event_name: "UserPromptExpansion",
            suppress_original_prompt: true,
        },
    })
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
}
