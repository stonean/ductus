//! `render-status` — the pipeline view `/{project}:status` shows on the model path,
//! without `dashboard`'s structured payload (spec 080, AC13). What it returns is
//! `schema::descriptions::RENDER_STATUS`.
//!
//! It takes no arguments, because `ductus exec` dispatches a step's primitive with
//! none, so a flag on `dashboard` would never reach an exec-driven status. The view is
//! rendered from the same read `dashboard` makes ([`super::dashboard::render_as`]), so
//! the two cannot disagree about the pipeline state.

use std::path::Path;
use std::time::SystemTime;

use crate::primitives::{Result, dashboard, pipeline_view};
use crate::schema::primitives::{RenderStatusArgs, RenderStatusResult};
use crate::session;

/// Execute the `render-status` primitive.
///
/// # Errors
///
/// Every error `dashboard` returns, [`crate::primitives::PrimitiveError::InvalidReadSize`]
/// for a malformed `[artifacts] read-size-bytes`, and the errors resolving a
/// spec's `references:` index returns.
pub fn run(_args: &RenderStatusArgs, repo: &Path) -> Result<RenderStatusResult> {
    let identity = session::process_identity()?;
    let (rendered_markdown, project) = dashboard::render_as(
        repo,
        identity.as_ref(),
        SystemTime::now(),
        pipeline_view::Format::Markdown,
    )?;
    Ok(RenderStatusResult {
        rendered_markdown,
        project,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn write(repo: &Path, rel: &str, body: &str) {
        let path = repo.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, body).unwrap();
    }

    /// The result carries the view and the namespace, and nothing of
    /// `dashboard`'s structured payload.
    #[test]
    fn the_result_is_the_view_alone() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            ".ductus/config.toml",
            "[host]\nproject = \"myapp\"\n",
        );
        write(
            tmp.path(),
            "specs/001-alpha/spec.md",
            "---\nstatus: planned\ndependencies: []\n---\n\n# A\n",
        );
        let result = run(&RenderStatusArgs::default(), tmp.path()).unwrap();
        let value = serde_json::to_value(&result).unwrap();
        let keys: Vec<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, ["rendered-markdown", "project"]);
        assert_eq!(result.project, "myapp");
        assert!(
            result.rendered_markdown.contains("| 001-alpha | planned |"),
            "{}",
            result.rendered_markdown
        );
        assert!(
            result
                .rendered_markdown
                .ends_with("Run /myapp:target <feature> to select one."),
            "{}",
            result.rendered_markdown
        );
    }
}
