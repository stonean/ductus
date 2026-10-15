//! `relocate-audit-records` — move a spec's `review:` and `analyze:` blocks
//! into the artifacts that own them.
//!
//! The sweep behind spec 057's migration. Each block is rewritten as top-level
//! frontmatter on `review.md` / `analysis.md`, and removed from `spec.md`.
//!
//! ## Why a primitive rather than a script
//!
//! The corpus is 54 specs in this repository alone, and adopters have their
//! own. `framework/migrations/criterion-label-backfill.md` settled the same
//! question for the same reason: its sweep runs through the primitive "never by
//! hand — 700 criteria across 49 specs in this repository's own backfill, where
//! a hand edit is a silent renumbering waiting to happen." A shell script
//! would also have to parse frontmatter, which [§runtime-boundary] principle 3
//! rules out on its own.
//!
//! ## What it will not do
//!
//! **Invent a record.** A spec carrying no block gets no artifact. Writing one
//! whose every field is null would assert that a run happened, which is the
//! objection 047 raised against backfilling and the reason an absent artifact
//! is meaningful at all.
//!
//! **Touch body prose.** The edit is confined to the frontmatter block, so it
//! is case (c) of §spec-lifecycle's mechanical-edit rule and a `done` spec
//! stays `done`. The body is compared before and after, and a run that finds
//! itself having changed it halts — the same stopping condition
//! `criterion-label-backfill` states for its own sweep.
//!
//! **Silently pick a winner when both sides carry a value.** Every spec in a
//! pre-migration corpus already has a `review.md`, so "the artifact exists"
//! is the *normal* case for the review half, not a conflict to refuse — the
//! block is merged into the artifact's frontmatter instead. The spec block
//! wins on a disagreement, because it is the copy every gate actually read,
//! and each disagreeing key is **reported**: 031 and 041 are exactly why a
//! migration must not resolve that difference quietly.
//!
//! `analysis.md` has no such case — it never existed before this — so its
//! half is a plain write.
//!
//! [§runtime-boundary]: https://github.com/stonean/ductus/blob/main/framework/constitution.md#runtime-boundary

use std::fmt::Write as _;
use std::path::Path;

use crate::primitives::{
    ANALYSIS_RECORD_FILE, PrimitiveError, REVIEW_RECORD_FILE, Result, read_text, rel_path,
    split_frontmatter, validate_no_traversal, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{RelocateAuditRecordsArgs, RelocateAuditRecordsResult};

/// Execute the `relocate-audit-records` primitive against the given repo root.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidPath`] when `feature` carries a parent-directory
///   component.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory is missing.
/// - [`PrimitiveError::Io`] when a file cannot be read or written.
/// - [`PrimitiveError::MissingFrontmatter`] when `spec.md` has no `---` fences,
///   and [`PrimitiveError::UnclosedFrontmatter`] when its block never closes.
pub fn run(args: &RelocateAuditRecordsArgs, repo: &Path) -> Result<RelocateAuditRecordsResult> {
    validate_no_traversal(&args.feature)?;
    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }

    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, body) = split_frontmatter(&content, &spec_path)?;

    let (kept, blocks) = split_record_blocks(fm_text);
    if blocks.is_empty() {
        // Converged. Reported as a domain outcome rather than an error, so a
        // re-run over a partially migrated corpus completes the remainder
        // instead of halting on the specs it already finished.
        return Ok(RelocateAuditRecordsResult {
            spec_path: rel_path(&spec_path, repo),
            relocated: vec![],
            disagreements: vec![],
            changed: false,
        });
    }

    let mut relocated = Vec::new();
    let mut disagreements = Vec::new();
    for (key, lines) in &blocks {
        let (file, heading) = artifact_for(key);
        let artifact = feature_dir.join(file);
        let rendered = render_artifact(
            &artifact,
            &args.feature,
            heading,
            file,
            lines,
            &mut disagreements,
        )?;
        write_atomic(&artifact, &rendered)?;
        relocated.push(rel_path(&artifact, repo));
    }

    let rewritten = crate::primitives::with_line_ending(
        &format!("---\n{}\n---\n{body}", kept.trim_end_matches('\n')),
        crate::primitives::line_ending_of(&content),
    );

    // The body must survive byte-identical. A relocation that rewrote prose
    // would be a meaningful edit under §spec-lifecycle, and a `done` spec would
    // owe a back-edge it is not taking.
    let (_, new_body) = split_frontmatter(&rewritten, &spec_path)?;
    if new_body != body {
        return Err(PrimitiveError::Io {
            path: spec_path.clone(),
            source: std::io::Error::other(
                "relocation would have changed the spec body; refusing to write",
            ),
        });
    }

    write_atomic(&spec_path, &rewritten)?;
    Ok(RelocateAuditRecordsResult {
        spec_path: rel_path(&spec_path, repo),
        relocated,
        disagreements,
        changed: true,
    })
}

/// The artifact's new content: the block merged into whatever is already
/// there, or a fresh file when nothing is.
///
/// Merging is the normal path for `review.md` — every pre-migration spec has
/// one — so this is where the block's keys win over the artifact's, and where
/// each key the two disagreed on is recorded for the caller to surface.
fn render_artifact(
    artifact: &Path,
    feature: &str,
    heading: &str,
    file: &str,
    lines: &[String],
    disagreements: &mut Vec<String>,
) -> Result<String> {
    let existing = if artifact.exists() {
        let text = read_text(artifact)?;
        let (fm, body) = split_frontmatter(&text, artifact)?;
        Some((fm.to_string(), body.to_string()))
    } else {
        None
    };

    let mut out = String::from("---\n");
    if let Some((existing_fm, _)) = &existing {
        let block_keys = keyed(lines);
        for line in existing_fm.lines() {
            let Some(key) = top_level_key(line) else {
                continue;
            };
            // A key the block spells differently is the same field, so it is
            // compared and dropped rather than carried forward beside its
            // other spelling. See [`LEGACY_ALIASES`].
            let block_key = legacy_alias(key).unwrap_or(key);
            if let Some(block_value) = block_keys.get(block_key) {
                // The block wins, but the difference is named rather than
                // quietly dropped. The disagreement names the key as the
                // artifact spelled it, so an operator can see which side
                // carried what.
                let artifact_value = line.split_once(':').map(|(_, v)| v.trim());
                if artifact_value.is_some_and(|v| v != block_value.trim()) {
                    disagreements.push(format!("{file}:{key}"));
                }
                continue;
            }
            if let Some(current) = legacy_alias(key) {
                // The block carries no value for this field, so the
                // artifact's is the only one there is — renamed rather than
                // emitted under a spelling nothing reads any more.
                let value = line.split_once(':').map_or("", |(_, v)| v.trim());
                let _ = writeln!(out, "{current}: {value}");
                continue;
            }
            out.push_str(line);
            out.push('\n');
        }
    } else {
        let _ = writeln!(out, "spec: {feature}");
    }
    for line in lines {
        out.push_str(line);
        out.push('\n');
    }
    out.push_str("---\n");

    // The report body is the artifact's own and is carried verbatim: a
    // migration that replaced a real review report with a placeholder would
    // destroy the findings it was moving the record beside.
    if let Some((_, body)) = existing {
        out.push_str(&body);
    } else {
        let _ = writeln!(out, "\n# {heading} — {feature}\n");
        out.push_str("## Summary\n\n");
        out.push_str(
            "Relocated from the spec's frontmatter by the record-relocation \
             migration. The counts above are the recorded run's; this report \
             body begins at the next run.\n",
        );
    }
    Ok(out)
}

/// Artifact keys the spec block spelled differently — one field, two names.
///
/// `review.md` recorded the review's instant as `reviewed-at` while the
/// spec's `review:` block called the same instant `last-run`, which is why
/// the reconciliation check had to key that pair *by meaning rather than by
/// name* to compare the two at all. The merge folds the pair onto `last-run`
/// (spec 057, AC12): carrying both forward would preserve the two-names
/// condition inside the artifact that exists to end it, and a later reader
/// could not tell which spelling was authoritative.
const LEGACY_ALIASES: &[(&str, &str)] = &[("reviewed-at", "last-run")];

/// The current spelling of a retired key name, when it has one.
fn legacy_alias(key: &str) -> Option<&'static str> {
    LEGACY_ALIASES
        .iter()
        .find_map(|(legacy, current)| (key == *legacy).then_some(*current))
}

/// A top-level frontmatter key, or `None` for an indented or blank line.
fn top_level_key(line: &str) -> Option<&str> {
    if line.trim().is_empty() || line.starts_with([' ', '\t']) {
        return None;
    }
    line.split(':').next().map(str::trim)
}

/// The block's top-level keys, mapped to their raw values, for merge comparison.
fn keyed(lines: &[String]) -> std::collections::BTreeMap<&str, &str> {
    lines
        .iter()
        .filter_map(|line| {
            let key = top_level_key(line)?;
            let value = line.split_once(':').map(|(_, v)| v)?;
            Some((key, value))
        })
        .collect()
}

/// The artifact and report heading a record key belongs to.
fn artifact_for(key: &str) -> (&'static str, &'static str) {
    if key == "review" {
        (REVIEW_RECORD_FILE, "Review")
    } else {
        (ANALYSIS_RECORD_FILE, "Analysis")
    }
}

/// Split frontmatter into the lines that stay and the record blocks that move.
///
/// Each block's lines are de-indented one level, since they move from nested
/// under `review:` / `analyze:` to top-level in their own file.
fn split_record_blocks(fm_text: &str) -> (String, Vec<(String, Vec<String>)>) {
    let mut kept = String::new();
    let mut blocks: Vec<(String, Vec<String>)> = Vec::new();
    let mut current: Option<String> = None;

    for line in fm_text.lines() {
        let indented = line.starts_with([' ', '\t']);
        if !indented {
            let key = line.split(':').next().unwrap_or("").trim();
            if matches!(key, "review" | "analyze") && line.trim_end().ends_with(':') {
                current = Some(key.to_string());
                blocks.push((key.to_string(), Vec::new()));
                continue;
            }
            current = None;
        }
        if current.is_some() && indented {
            if let Some((_, lines)) = blocks.last_mut() {
                lines.push(line.strip_prefix("  ").unwrap_or(line).to_string());
            }
            continue;
        }
        current = None;
        kept.push_str(line);
        kept.push('\n');
    }
    // A key present with no indented body carries no record to move.
    blocks.retain(|(_, lines)| !lines.is_empty());
    (kept, blocks)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::TempDir;

    const SPEC: &str = "---\nstatus: done\ndependencies: []\nreview:\n  last-run: 2026-08-01T00:00:00Z\n  must-violations: 0\n  blocking: false\nanalyze:\n  last-run: 2026-08-02T00:00:00Z\n  advisory: 3\n  blocking: false\nnext-criterion: 7\n---\n\n# 001 — X\n\n## Behavior\n\nProse that must survive.\n";

    fn repo(spec: &str) -> TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("spec.md"), spec).unwrap();
        tmp
    }

    fn args() -> RelocateAuditRecordsArgs {
        RelocateAuditRecordsArgs {
            feature: "001-x".into(),
        }
    }

    #[test]
    fn both_records_move_and_the_spec_keeps_everything_else() {
        let tmp = repo(SPEC);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.changed);
        assert_eq!(result.relocated.len(), 2);
        assert_eq!(result.disagreements, [] as [String; 0]);

        let dir = tmp.path().join("specs/001-x");
        let review = fs::read_to_string(dir.join("review.md")).unwrap();
        assert!(
            review.contains("last-run: 2026-08-01T00:00:00Z"),
            "{review}"
        );
        assert!(review.contains("spec: 001-x"), "{review}");
        assert!(
            fs::read_to_string(dir.join("analysis.md"))
                .unwrap()
                .contains("advisory: 3")
        );

        // The record deserializes at its new home — the point of the move, not
        // merely that the text landed somewhere.
        let (fm, _) = split_frontmatter(&review, Path::new("review.md")).unwrap();
        let record: crate::schema::primitives::ReviewBlock = serde_norway::from_str(fm).unwrap();
        assert_eq!(record.last_run.as_deref(), Some("2026-08-01T00:00:00Z"));

        let spec = fs::read_to_string(dir.join("spec.md")).unwrap();
        assert!(!spec.contains("review:"), "{spec}");
        assert!(!spec.contains("analyze:"), "{spec}");
        assert!(spec.contains("status: done"), "{spec}");
        assert!(spec.contains("next-criterion: 7"), "{spec}");
        assert!(spec.contains("Prose that must survive."), "{spec}");
    }

    /// Re-running over a converged spec writes nothing, which is what makes an
    /// interrupted sweep resumable rather than corrupting.
    #[test]
    fn a_second_run_is_a_no_op() {
        let tmp = repo(SPEC);
        run(&args(), tmp.path()).unwrap();
        let dir = tmp.path().join("specs/001-x");
        let spec_first = fs::read_to_string(dir.join("spec.md")).unwrap();
        let review_first = fs::read_to_string(dir.join("review.md")).unwrap();

        let second = run(&args(), tmp.path()).unwrap();
        assert!(!second.changed);
        assert_eq!(second.relocated, [] as [String; 0]);
        assert_eq!(fs::read_to_string(dir.join("spec.md")).unwrap(), spec_first);
        assert_eq!(
            fs::read_to_string(dir.join("review.md")).unwrap(),
            review_first
        );
    }

    /// A spec with no block gets no artifact. Writing one whose fields were all
    /// null would assert a run that nothing substantiates.
    #[test]
    fn a_spec_with_no_record_gets_no_artifact() {
        let tmp = repo("---\nstatus: draft\ndependencies: []\n---\n\n# 001 — X\n");
        let result = run(&args(), tmp.path()).unwrap();
        assert!(!result.changed);
        assert_eq!(result.relocated, [] as [String; 0]);
        let dir = tmp.path().join("specs/001-x");
        assert!(!dir.join("review.md").exists());
        assert!(!dir.join("analysis.md").exists());
    }

    /// The normal case for the review half: `review.md` already exists, so the
    /// block is merged into it rather than refused.
    ///
    /// Every spec in a pre-migration corpus has one, so a primitive that
    /// declined to merge could not migrate the review record at all.
    #[test]
    fn an_existing_artifact_is_merged_not_refused() {
        let tmp = repo(SPEC);
        let dir = tmp.path().join("specs/001-x");
        fs::write(
            dir.join("review.md"),
            "---\nspec: 001-x\nreviewed-at: 2026-08-01T00:00:00Z\ndiff-base: cafe\nmust-violations: 0\n---\n\n# Review — 001-x\n\n## Summary\n\nA real report body.\n",
        )
        .unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.changed);
        assert_eq!(result.relocated.len(), 2, "{result:?}");
        assert!(result.disagreements.is_empty(), "{result:?}");

        let review = fs::read_to_string(dir.join("review.md")).unwrap();
        // The artifact's own keys survive...
        assert!(review.contains("diff-base: cafe"), "{review}");
        // ...the block's keys arrive...
        assert!(review.contains("blocking: false"), "{review}");
        assert!(
            review.contains("last-run: 2026-08-01T00:00:00Z"),
            "{review}"
        );
        // ...and the existing report body is carried verbatim, not replaced by
        // the migration's placeholder.
        assert!(review.contains("A real report body."), "{review}");
        assert!(!review.contains("Relocated from the spec"), "{review}");
        // ...and the retired spelling of the block's `last-run` does not
        // survive beside it. One instant, one name.
        assert!(!review.contains("reviewed-at"), "{review}");

        let spec = fs::read_to_string(dir.join("spec.md")).unwrap();
        assert!(!spec.contains("review:"), "{spec}");
        assert!(!spec.contains("analyze:"), "{spec}");
    }

    /// The pair `reviewed-at` / `last-run` is one instant spelled twice, so a
    /// disagreement across it is named under the key the artifact used.
    ///
    /// Without the alias the two spellings never meet: the merge finds no
    /// `reviewed-at` in the block, carries it forward untouched, and the
    /// migrated artifact ends up asserting two different instants for one run
    /// — a worse state than the one the relocation exists to fix.
    #[test]
    fn the_retired_timestamp_spelling_is_folded_and_its_disagreement_named() {
        let tmp = repo(SPEC);
        let dir = tmp.path().join("specs/001-x");
        fs::write(
            dir.join("review.md"),
            "---\nspec: 001-x\nreviewed-at: 2026-07-04T00:00:00Z\n---\n\n# Review — 001-x\n",
        )
        .unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        let review = fs::read_to_string(dir.join("review.md")).unwrap();
        assert!(
            review.contains("last-run: 2026-08-01T00:00:00Z"),
            "the block wins: {review}"
        );
        assert!(!review.contains("reviewed-at"), "{review}");
        assert!(
            result
                .disagreements
                .contains(&"review.md:reviewed-at".to_string()),
            "{result:?}"
        );
    }

    /// When the block carries no timestamp, the artifact's is the only one
    /// there is — so it is renamed rather than emitted under a spelling
    /// nothing reads any more, which would leave the record unparseable.
    #[test]
    fn a_retired_spelling_with_no_block_counterpart_is_renamed() {
        let spec = "---\nstatus: done\ndependencies: []\nreview:\n  must-violations: 0\n  blocking: false\n---\n\n# 001 — X\n";
        let tmp = repo(spec);
        let dir = tmp.path().join("specs/001-x");
        fs::write(
            dir.join("review.md"),
            "---\nspec: 001-x\nreviewed-at: 2026-07-04T00:00:00Z\n---\n\n# Review — 001-x\n",
        )
        .unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        let review = fs::read_to_string(dir.join("review.md")).unwrap();
        assert!(
            review.contains("last-run: 2026-07-04T00:00:00Z"),
            "{review}"
        );
        assert!(!review.contains("reviewed-at"), "{review}");
        assert!(result.disagreements.is_empty(), "{result:?}");
    }

    /// A disagreement between the two copies is reported, not smoothed.
    ///
    /// This is the 031/041 shape: the block said one thing and the report said
    /// another, for weeks. The block wins — it is what the gate read — but a
    /// migration that resolved it silently would erase the only evidence the
    /// drift ever happened.
    #[test]
    fn a_disagreement_between_the_copies_is_named() {
        let tmp = repo(SPEC);
        let dir = tmp.path().join("specs/001-x");
        fs::write(
            dir.join("review.md"),
            "---\nspec: 001-x\nmust-violations: 9\n---\n\n# Review — 001-x\n",
        )
        .unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(
            result.disagreements,
            vec!["review.md:must-violations".to_string()],
            "{result:?}"
        );
        let review = fs::read_to_string(dir.join("review.md")).unwrap();
        assert!(review.contains("must-violations: 0"), "{review}");
        assert!(!review.contains("must-violations: 9"), "{review}");
    }

    /// A key with no indented body carries no record, so there is nothing to
    /// move and nothing to write.
    #[test]
    fn an_empty_block_is_not_a_record() {
        let tmp = repo("---\nstatus: draft\ndependencies: []\nreview:\n---\n\n# 001 — X\n");
        let result = run(&args(), tmp.path()).unwrap();
        assert!(!result.changed);
        assert!(!tmp.path().join("specs/001-x/review.md").exists());
    }

    #[test]
    fn a_traversing_feature_name_is_refused() {
        let tmp = repo(SPEC);
        let err = run(
            &RelocateAuditRecordsArgs {
                feature: "../../etc".into(),
            },
            tmp.path(),
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }
}
