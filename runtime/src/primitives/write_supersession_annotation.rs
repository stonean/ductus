//! `write-supersession-annotation` — record on a superseded spec that a
//! later spec countered it.
//!
//! The `supersedes:` key on the superseding spec is bookkeeping. What stops
//! a reader — human or agent — mistaking a countered spec for a live one is
//! this annotation, written on the spec that was countered.
//!
//! **The frame is compiled in; the substance is authored.** This is the
//! content-ingestion convention `create-scenario` already uses: no primitive
//! writes prose (§runtime-boundary principle 2). A generated banner can name
//! the superseding spec and the date and nothing else, and the sentence a
//! reader actually needs is the one naming what stopped being true.
//!
//! **The blockquote wrapper is structural, not stylistic.**
//! `derive-dependencies` does not harvest links on blockquote-prefixed
//! lines, so the banner may link its superseding spec without the annotated
//! spec acquiring a dependency on its own successor. `specs/005-workflows`
//! demonstrates it: the sunset note links `043-workflows-sunset` while its
//! `dependencies:` names only `004` and `010`. Un-blockquote the frame and
//! every annotation silently inverts the dependency graph.
//!
//! **The frontmatter is never touched.** The write splices a modified body
//! onto the original head, byte for byte, so the spec keeps whatever status
//! it had at any lifecycle state — the annotation is a mechanical edit under
//! §spec-lifecycle and takes no back-edge. That guarantee is structural here
//! rather than a rule this code remembers to follow.
//!
//! Defined by `specs/052-spec-supersession-and-consolidation/spec.md`.

use std::fmt::Write as _;
use std::path::Path;

use crate::primitives::{
    PrimitiveError, Result, read_text, rel_path, split_frontmatter, validate_no_traversal,
    write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    WriteSupersessionAnnotationArgs, WriteSupersessionAnnotationResult,
};

/// Execute the `write-supersession-annotation` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when either feature name is
/// empty, absolute, or carries a parent-directory component;
/// [`PrimitiveError::InvalidArgument`] when a spec would annotate itself or
/// the substance is blank; [`PrimitiveError::FeatureNotFound`] when the
/// superseded feature holds no `spec.md`; and [`PrimitiveError::Io`] when
/// the read or the write fails.
///
/// An annotation already citing this superseding spec is **not** an error —
/// it is `already-present: true` with `written: false`, so a re-run of an
/// interrupted declaration converges instead of stacking a duplicate.
pub fn run(
    args: &WriteSupersessionAnnotationArgs,
    repo: &Path,
) -> Result<WriteSupersessionAnnotationResult> {
    validate_no_traversal(&args.feature)?;
    validate_no_traversal(&args.superseded_by)?;

    if args.feature == args.superseded_by {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "write-supersession-annotation".into(),
            argument: "superseded-by".into(),
            reason: "a spec cannot supersede itself".into(),
        });
    }

    if args.substance.trim().is_empty() {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "write-supersession-annotation".into(),
            argument: "substance".into(),
            reason: "the annotation's substance is authored, never generated: a banner naming \
                     only the superseding spec tells a reader nothing about what stopped being \
                     true"
                .into(),
        });
    }

    let layout = paths::Paths::load(repo);
    let spec_path = repo
        .join(&layout.specs_root)
        .join(&args.feature)
        .join("spec.md");
    if !spec_path.is_file() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: args.feature.clone(),
        });
    }

    let content = read_text(&spec_path)?;
    let (_fm, body) = split_frontmatter(&content, &spec_path)?;

    // `body` is a subslice of `content`, so everything before it is the
    // frontmatter and its fences exactly as written — CRLF included. Keeping
    // the head verbatim is what makes "the status is untouched" structural.
    let head_len = content.len() - body.len();
    let head = &content[..head_len];

    let path = rel_path(&spec_path, repo);
    if cites(body, &args.superseded_by) {
        return Ok(WriteSupersessionAnnotationResult {
            written: false,
            already_present: true,
            path,
        });
    }

    let eol = if body.contains("\r\n") { "\r\n" } else { "\n" };
    let block = render(&args.superseded_by, args.substance.trim(), eol);
    let new_body = insert_at_lead(body, &block, eol);

    write_atomic(&spec_path, &format!("{head}{new_body}"))?;
    Ok(WriteSupersessionAnnotationResult {
        written: true,
        already_present: false,
        path,
    })
}

/// Whether the body already carries a blockquoted annotation citing
/// `superseding`.
///
/// Scoped to blockquote lines on purpose. A sibling link to the same spec in
/// ordinary prose is a dependency-inducing reference and says nothing about
/// whether this spec has been annotated; treating one as an existing
/// annotation would silently suppress a real one.
fn cites(body: &str, superseding: &str) -> bool {
    let target = format!("../{superseding}/spec.md");
    body.lines()
        .filter(|line| line.trim_start().starts_with('>'))
        .any(|line| line.contains(&target))
}

/// Render the annotation block. The frame is everything except `substance`.
///
/// The closing sentence deliberately does not name a status. The annotation
/// is written at whatever lifecycle state the superseded spec is in, so
/// hardcoding `done` would state a falsehood on the very case that is
/// accepted-with-guidance rather than refused.
fn render(superseding: &str, substance: &str, eol: &str) -> String {
    let mut out = String::new();
    let _ = write!(
        out,
        "> **Sunset ([{superseding}](../{superseding}/spec.md)):** {substance}{eol}"
    );
    let _ = write!(out, ">{eol}");
    let _ = write!(
        out,
        "> This spec is retained as the historical record of what shipped; the material named \
         above no longer describes the current system.{eol}"
    );
    out
}

/// Insert `block` after the H1 and its lead paragraph, ahead of any
/// annotation already present.
///
/// Newest-first is the corpus convention and it carries meaning: a later,
/// broader supersession scopes the notes beneath it rather than replacing
/// them, which is how `005-workflows` reads with four stacked annotations.
fn insert_at_lead(body: &str, block: &str, eol: &str) -> String {
    let lines: Vec<&str> = body.split_inclusive('\n').collect();

    let Some(h1) = lines
        .iter()
        .position(|line| line.trim_start().starts_with("# "))
    else {
        // No H1 to anchor to. Prepending is the honest fallback: the
        // annotation must be the first thing read, and inventing a heading
        // would edit a claim the spec makes about itself.
        return format!("{block}{eol}{body}");
    };

    let mut i = h1 + 1;
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    // The lead paragraph: prose directly under the H1. A heading or an
    // existing blockquote means there is none, and the block goes straight
    // after the H1 — ahead of the annotation already there.
    while i < lines.len() {
        let t = lines[i].trim_start();
        if t.is_empty() || t.starts_with('>') || t.starts_with('#') {
            break;
        }
        i += 1;
    }
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }

    let mut out = String::new();
    out.push_str(&lines[..i].concat());
    out.push_str(block);
    out.push_str(eol);
    out.push_str(&lines[i..].concat());
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::run;
    use crate::primitives::PrimitiveError;
    use crate::schema::primitives::WriteSupersessionAnnotationArgs;
    use std::path::{Path, PathBuf};

    fn spec(status: &str) -> String {
        format!(
            "---\nstatus: {status}\ndependencies: []\n---\n\n# 005 — Workflows\n\nScaffold \
             workflow files during bootstrap.\n\n## Problem\n\nStuff.\n"
        )
    }

    fn repo_with(feature: &str, body: &str) -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs").join(feature);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("spec.md"), body).unwrap();
        let path = dir.join("spec.md");
        (tmp, path)
    }

    fn args(feature: &str, by: &str, substance: &str) -> WriteSupersessionAnnotationArgs {
        WriteSupersessionAnnotationArgs {
            feature: feature.into(),
            superseded_by: by.into(),
            substance: substance.into(),
        }
    }

    fn read(path: &Path) -> String {
        std::fs::read_to_string(path).unwrap()
    }

    #[test]
    fn writes_a_blockquoted_annotation_after_the_lead_paragraph() {
        let (tmp, path) = repo_with("005-workflows", &spec("done"));
        let result = run(
            &args(
                "005-workflows",
                "043-workflows-sunset",
                "the feature was removed",
            ),
            tmp.path(),
        )
        .unwrap();
        assert!(result.written);
        assert!(!result.already_present);

        let out = read(&path);
        let line = out
            .lines()
            .find(|l| l.starts_with("> **Sunset"))
            .expect("annotation present");
        // The link is what a reader follows; the blockquote is what keeps
        // `derive-dependencies` from harvesting it into an edge.
        assert!(line.contains("[043-workflows-sunset](../043-workflows-sunset/spec.md)"));
        assert!(line.contains("the feature was removed"));

        // Placed after the H1 and its lead paragraph, not before them.
        let h1 = out.find("# 005 — Workflows").unwrap();
        let lead = out.find("Scaffold workflow files").unwrap();
        let ann = out.find("> **Sunset").unwrap();
        assert!(h1 < lead && lead < ann, "annotation landed out of order");
    }

    #[test]
    fn a_second_annotation_from_a_different_spec_stacks_above_the_first() {
        let (tmp, path) = repo_with("005-workflows", &spec("done"));
        run(
            &args("005-workflows", "043-sunset", "workflows removed"),
            tmp.path(),
        )
        .unwrap();
        let result = run(
            &args(
                "005-workflows",
                "050-constitution",
                "the config surface moved",
            ),
            tmp.path(),
        )
        .unwrap();
        assert!(result.written, "a different spec is an accumulation");

        let out = read(&path);
        let newer = out.find("050-constitution").unwrap();
        let older = out.find("043-sunset").unwrap();
        // Newest first: a later, broader supersession scopes the notes
        // beneath it rather than replacing them.
        assert!(newer < older, "expected newest-first stacking");
    }

    #[test]
    fn a_repeat_from_the_same_spec_writes_nothing() {
        let (tmp, path) = repo_with("005-workflows", &spec("done"));
        run(
            &args("005-workflows", "043-sunset", "workflows removed"),
            tmp.path(),
        )
        .unwrap();
        let after_first = read(&path);

        let result = run(
            &args(
                "005-workflows",
                "043-sunset",
                "a differently-worded substance",
            ),
            tmp.path(),
        )
        .unwrap();
        assert!(!result.written);
        assert!(result.already_present);
        // A re-run of an interrupted declaration converges — byte-identical,
        // and the second substance is not appended anywhere.
        assert_eq!(read(&path), after_first);
    }

    #[test]
    fn the_frontmatter_is_untouched_at_every_lifecycle_state() {
        // The write is mechanical and takes no back-edge, so it must not
        // move the status — including on the non-`done` spec that is
        // accepted with guidance rather than refused.
        for status in ["done", "clarified", "in-progress", "draft"] {
            let (tmp, path) = repo_with("005-workflows", &spec(status));
            let before = read(&path);
            let head_before = before.split("\n---\n").next().unwrap().to_string();

            run(&args("005-workflows", "043-sunset", "removed"), tmp.path()).unwrap();

            let after = read(&path);
            let head_after = after.split("\n---\n").next().unwrap().to_string();
            assert_eq!(head_before, head_after, "frontmatter moved at {status}");
            assert!(after.contains(&format!("status: {status}")));
        }
    }

    #[test]
    fn a_superseded_spec_that_does_not_exist_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run(&args("404-nowhere", "043-sunset", "removed"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    #[test]
    fn blank_substance_is_refused() {
        // The whole point of the seam: the frame is generated, the sentence
        // a reader needs is not.
        let (tmp, _) = repo_with("005-workflows", &spec("done"));
        let err = run(&args("005-workflows", "043-sunset", "   "), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidArgument { .. }));
    }

    #[test]
    fn a_spec_cannot_annotate_itself() {
        let (tmp, _) = repo_with("005-workflows", &spec("done"));
        let err = run(
            &args("005-workflows", "005-workflows", "removed"),
            tmp.path(),
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidArgument { .. }));
    }

    /// The whole annotation design rests on one property of a *different*
    /// module: `spec_links` excludes blockquote-prefixed lines, so the banner
    /// can link its superseding spec without the annotated spec acquiring a
    /// dependency on its own successor. Nothing else pins that. If the
    /// exemption is ever narrowed, every annotation this primitive has
    /// written silently inverts an edge in the dependency graph, and the
    /// first symptom would be a cycle in a corpus nobody had touched.
    ///
    /// So this asserts the composition end to end — real primitive output
    /// through the real scanner — and asserts the negative case too, because
    /// a test that only checks "no link found" would keep passing if the
    /// scanner stopped finding links at all.
    #[test]
    fn the_annotations_link_is_exempt_from_dependency_harvesting() {
        use crate::primitives::spec_links::harvestable_lines;

        let (tmp, path) = repo_with("005-workflows", &spec("done"));
        run(
            &args(
                "005-workflows",
                "043-workflows-sunset",
                "the feature was removed",
            ),
            tmp.path(),
        )
        .unwrap();

        let annotated = read(&path);
        let target = "../043-workflows-sunset/spec.md";

        // The link is really in the file …
        assert!(
            annotated.contains(target),
            "the annotation should carry the link a reader follows"
        );

        // … and the scanner that decides edges does not see it.
        let harvestable: Vec<&str> = harvestable_lines(&annotated)
            .iter()
            .map(|l| l.text)
            .collect();
        assert!(
            !harvestable.iter().any(|line| line.contains(target)),
            "a blockquoted annotation must induce no dependency edge; harvestable lines were \
             {harvestable:?}"
        );

        // The control: the same link, same file, blockquote prefix stripped,
        // *is* harvested. Without this the assertion above would survive the
        // scanner returning nothing at all.
        let unquoted: String = annotated
            .lines()
            .map(|line| line.strip_prefix("> ").unwrap_or(line))
            .collect::<Vec<_>>()
            .join("\n");
        let harvestable_unquoted: Vec<&str> = harvestable_lines(&unquoted)
            .iter()
            .map(|l| l.text)
            .collect();
        assert!(
            harvestable_unquoted
                .iter()
                .any(|line| line.contains(target)),
            "un-blockquoted, the same link must be harvested — otherwise this test proves \
             nothing about the blockquote"
        );
    }
}
