//! `render-status` — the pipeline view `/{project}:status` shows on the model path,
//! without `dashboard`'s structured payload (spec 080, AC13). What it returns is
//! `schema::descriptions::RENDER_STATUS`.
//!
//! It takes no arguments, because `ductus exec` dispatches a step's primitive with
//! none, so a flag on `dashboard` would never reach an exec-driven status. The view is
//! built from the same read `dashboard` makes ([`super::dashboard::view_model`]), so
//! the two cannot disagree about the pipeline state.
//!
//! Past the read size the agent still reaches everything the cut leaves out: the
//! whole view's lines go to a list file through [`bounded_lists::place`], and the
//! cut view names it (spec 080's plan, *What the agent gets past the bound*).

use std::path::Path;
use std::time::SystemTime;

use crate::primitives::check_artifact_size::resolve_threshold;
use crate::primitives::{Result, bounded_lists, dashboard, pipeline_view};
use crate::schema::primitives::{ListsForm, RenderStatusArgs, RenderStatusResult};
use crate::session;

/// Execute the `render-status` primitive.
///
/// # Errors
///
/// Every error `dashboard` returns, the errors resolving a spec's
/// `references:` index returns, and every error [`bounded_lists::place`]
/// returns: among them [`crate::primitives::PrimitiveError::InvalidReadSize`]
/// for a malformed `[artifacts] read-size-bytes`, and a list file that cannot
/// be written, since a cut view never names a file that is not on disk.
pub fn run(_args: &RenderStatusArgs, repo: &Path) -> Result<RenderStatusResult> {
    let identity = session::process_identity()?;
    let (model, project) = dashboard::view_model(repo, identity.as_ref(), SystemTime::now())?;
    let inline = RenderStatusResult {
        rendered_markdown: pipeline_view::whole(&model, pipeline_view::Format::Markdown),
        project,
        form: ListsForm::Inline,
        lists_file: None,
    };
    let lines: Vec<&str> = inline.rendered_markdown.lines().collect();
    let Some(file) = bounded_lists::place(
        repo,
        "render-status",
        bounded_lists::ALL_SPECS,
        &inline,
        &serde_json::json!({ "view": lines }),
    )?
    else {
        return Ok(inline);
    };
    let (threshold, _) = resolve_threshold(repo)?;
    let project = inline.project;
    let fits = |view: &str| {
        let candidate = RenderStatusResult {
            rendered_markdown: view.to_string(),
            project: project.clone(),
            form: ListsForm::File,
            lists_file: Some(file.clone()),
        };
        serde_json::to_vec(&candidate)
            .is_ok_and(|bytes| u64::try_from(bytes.len()).unwrap_or(u64::MAX) <= threshold.bytes)
    };
    let rendered_markdown =
        pipeline_view::render(&model, pipeline_view::Format::Markdown, &fits, Some(&file));
    Ok(RenderStatusResult {
        rendered_markdown,
        project,
        form: ListsForm::File,
        lists_file: Some(file),
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
        assert_eq!(keys, ["rendered-markdown", "project", "form"]);
        assert_eq!(result.form, ListsForm::Inline);
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

    /// The namespace the result carries is the one the view's hints spell,
    /// flattened as they are, so it names the `{project}` of every hint, a
    /// namespace holding a tab or a line break included (`BE-INPUT-011`).
    #[test]
    fn the_namespace_is_spelled_as_the_view_spells_it() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            ".ductus/config.toml",
            "[host]\nproject = \"my\\tapp\"\n",
        );
        write(
            tmp.path(),
            "specs/001-alpha/spec.md",
            "---\nstatus: planned\ndependencies: []\n---\n\n# A\n",
        );
        let result = run(&RenderStatusArgs::default(), tmp.path()).unwrap();
        assert_eq!(result.project, "my app");
        assert!(
            result
                .rendered_markdown
                .ends_with("Run /my app:target <feature> to select one."),
            "{}",
            result.rendered_markdown
        );
    }

    /// A malformed read size halts the view, as it halts every reader of the
    /// setting, rather than rendering at the default.
    #[test]
    fn a_malformed_read_size_halts_the_view() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            ".ductus/config.toml",
            "[artifacts]\nread-size-bytes = 0\n",
        );
        write(
            tmp.path(),
            "specs/001-alpha/spec.md",
            "---\nstatus: draft\ndependencies: []\n---\n\n# A\n",
        );
        let err = run(&RenderStatusArgs::default(), tmp.path()).unwrap_err();
        assert!(
            matches!(
                err,
                crate::primitives::PrimitiveError::InvalidReadSize { .. }
            ),
            "{err:?}"
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
