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

    /// The view names the target of the process that runs it, labelled with
    /// its session, not the shared default's (spec 062, AC8). The identity is
    /// captured once at startup, so the test re-runs itself as a child test
    /// process with `DUCTUS_SESSION` set and captures it there, as the binary's
    /// `main` does; the parent asserts that the child passed.
    #[test]
    fn the_view_names_this_process_target_and_session() {
        const NAME: &str =
            "primitives::render_status::tests::the_view_names_this_process_target_and_session";
        const CHILD: &str = "DUCTUS_TEST_RENDER_STATUS_CHILD";
        if std::env::var_os(CHILD).is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", NAME, "--test-threads=1"])
                .env(CHILD, "1")
                .env("DUCTUS_SESSION", "alpha")
                .env_remove("CLAUDE_CODE_SESSION_ID")
                .output()
                .unwrap();
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(
                output.status.success() && stdout.contains("1 passed"),
                "the child run, with an identity set, did not pass: {stdout}"
            );
            return;
        }
        session::init_process_identity();
        let alpha = session::process_identity().unwrap().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let target = |feature: &str| crate::schema::primitives::SessionTarget {
            feature: feature.into(),
            path: format!("specs/{feature}"),
            scenario: None,
            scenario_path: None,
        };
        let now = SystemTime::now();
        session::write(
            tmp.path(),
            Some(&alpha),
            &session::WriteShape::Target(target("055-a")),
            None,
            now,
        )
        .unwrap();
        // A later unidentified write moves the shared default elsewhere, so a
        // view rendered for no process would show that target, unlabelled.
        session::write(
            tmp.path(),
            None,
            &session::WriteShape::Target(target("056-b")),
            None,
            now,
        )
        .unwrap();
        let result = run(&RenderStatusArgs::default(), tmp.path()).unwrap();
        let first = result.rendered_markdown.lines().next().unwrap_or_default();
        assert!(
            first.starts_with("Target: 055-a ") && first.ends_with("— session alpha"),
            "the target line names this process's own target and session: {first:?}"
        );
    }
}
