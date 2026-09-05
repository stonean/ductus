//! `write-analysis` — record that `/ductus:analyze` ran, in the spec's
//! `analyze:` frontmatter block.
//!
//! The pipeline is `implement → review → analyze → done`, and until this
//! primitive existed only half of it left a trace. `check-review-gate` read
//! the `review:` block; Family 19 checked its freshness; Family 31 held it
//! against `review.md`. Analyze wrote nothing, so a spec that had passed both
//! gates and a spec that had passed only the first were **byte-identical on
//! disk**. Nothing could tell them apart, which meant nothing could enforce
//! the second gate, which meant the only thing holding it was whoever
//! remembered — the diligence dependency §design-principles rejects outright.
//!
//! That state was not hypothetical. On 2026-09-05 two specs were advanced to
//! `done` on the review gate alone and one of them was published to crates.io
//! before anyone noticed, because there was nothing to notice: every signal
//! the repository had said the spec was complete. The gap was found by being
//! asked, which is the definition of a diligence dependency.
//!
//! **This changes `/ductus:analyze`'s read-only contract, deliberately, and
//! the new line is between the subject and the observation.** Analyze still
//! never mutates an artifact it audits; `--fix` remains the only path that
//! does. Recording that the audit happened is not mutating the subject — it is
//! precisely what `write-review` does for the other gate, and precisely why
//! that gate was enforceable and this one was not.
//!
//! The block deliberately is **not** a copy of `review:`:
//!
//! - `advisory` is recorded and never gated on. An outstanding SHOULD blocks
//!   `done` at the review gate because §implement-phase says advisory is not
//!   ignorable there. Analyze's advisory tier is a different contract: its
//!   members are checks introduced advisory *with published promotion
//!   criteria* — grounding, Applicable-Rules citations, decision drift — and
//!   gating on them here would promote every one of them at once, past the
//!   criteria each declares.
//! - `unexamined` has no counterpart in `review:` at all, and is the field
//!   that makes this record honest. A clean analyze is two states, not one,
//!   and the command's own contract says so: "clean with nothing skipped is
//!   verified-clean, clean with something skipped is partially examined." A
//!   record carrying only finding counts would collapse that into the
//!   reassuring reading — inside the artifact a later gate trusts, which is
//!   the worst possible place for `QUAL-CLAIM-001`.
//!
//! Defined by `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md`.

use std::fmt::Write as _;
use std::path::Path;

use crate::primitives::write_review::splice_top_level_block;
use crate::primitives::{
    PrimitiveError, Result, read_text, rel_path, split_frontmatter, validate_no_traversal,
    write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{WriteAnalysisArgs, WriteAnalysisResult};

/// Execute the `write-analysis` primitive against the given repo root.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute, or
///   carries a parent-directory component.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory does not
///   exist.
/// - [`PrimitiveError::Io`] when `spec.md` cannot be read or written.
/// - [`PrimitiveError::Yaml`] when the frontmatter block is malformed —
///   never repaired here. A spec whose frontmatter does not parse is one the
///   analysis itself would have hard-failed on, and writing a record of a
///   clean run into it would be the exact inversion this primitive exists to
///   prevent.
pub fn run(args: &WriteAnalysisArgs, repo: &Path) -> Result<WriteAnalysisResult> {
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

    // Parse before writing. The value is not used, but a frontmatter block
    // that does not deserialize must not receive a record asserting a clean
    // analysis — see the `Yaml` note above.
    let _: crate::schema::primitives::Frontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;

    let replaced = fm_text
        .lines()
        .any(|line| !line.starts_with([' ', '\t']) && line.starts_with("analyze:"));

    let blocking = args.hard_fail > 0 || args.blocking_findings > 0;
    let block = render_analyze_yaml(args, blocking);
    let new_fm = splice_top_level_block(fm_text, "analyze", &block);
    let rendered = crate::primitives::with_line_ending(
        &format!("---\n{new_fm}\n---\n{body}"),
        crate::primitives::line_ending_of(&content),
    );
    write_atomic(&spec_path, &rendered)?;

    Ok(WriteAnalysisResult {
        spec_path: rel_path(&spec_path, repo),
        blocking,
        replaced,
    })
}

/// Render the `analyze:` YAML block (no trailing newline).
///
/// Every value is a timestamp, a sha, an integer, or a bool, so none needs the
/// quoting `render_review_yaml`'s open-schema waiver fields do — but the two
/// host-supplied strings still cannot be trusted to be single-line. An
/// embedded newline in `analyzed-against` would inject arbitrary frontmatter
/// keys, which is the injection `write-review` already guards; the guard lives
/// in [`single_line`] here for the same reason.
fn render_analyze_yaml(args: &WriteAnalysisArgs, blocking: bool) -> String {
    let mut block = String::from("analyze:\n");
    let _ = writeln!(block, "  last-run: {}", single_line(&args.analyzed_at));
    let _ = writeln!(
        block,
        "  analyzed-against: {}",
        single_line(&args.analyzed_against)
    );
    let _ = writeln!(block, "  hard-fail: {}", args.hard_fail);
    let _ = writeln!(block, "  blocking-findings: {}", args.blocking_findings);
    let _ = writeln!(block, "  advisory: {}", args.advisory);
    let _ = writeln!(block, "  unexamined: {}", args.unexamined);
    let _ = writeln!(block, "  blocking: {blocking}");
    block.trim_end_matches('\n').to_string()
}

/// Collapse any line break in a host-supplied scalar to a space.
///
/// `write-review` rejects such a value outright; this one flattens instead,
/// because both of these fields are machine-generated (a timestamp and a sha)
/// and a newline in either is a caller defect with no legitimate reading —
/// there is no user intent to preserve, only an injection to defuse.
fn single_line(value: &str) -> String {
    value.replace(['\n', '\r'], " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn spec_repo(frontmatter: &str) -> TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/042-demo");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\n{frontmatter}\n---\n\n# 042 — Demo\n\n## Behavior\n\nText.\n"),
        )
        .unwrap();
        tmp
    }

    fn args() -> WriteAnalysisArgs {
        WriteAnalysisArgs {
            feature: "042-demo".into(),
            analyzed_at: "2026-09-05T18:00:00Z".into(),
            analyzed_against: "abc123".into(),
            hard_fail: 0,
            blocking_findings: 0,
            advisory: 0,
            unexamined: 0,
        }
    }

    #[test]
    fn inserts_the_block_when_absent() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(&args(), tmp.path()).unwrap();
        assert!(!result.replaced);
        assert!(!result.blocking);
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(spec.contains("analyze:\n"));
        assert!(spec.contains("  last-run: 2026-09-05T18:00:00Z"));
        assert!(spec.contains("  blocking: false"));
        // Surrounding keys survive.
        assert!(spec.contains("status: in-progress"));
        assert!(spec.contains("dependencies: []"));
    }

    #[test]
    fn replaces_an_existing_block_without_disturbing_review() {
        let tmp = spec_repo(
            "status: done\ndependencies: []\nreview:\n  last-run: 2020-01-01T00:00:00Z\n  \
             blocking: false\nanalyze:\n  last-run: 2019-01-01T00:00:00Z\n  blocking: true\n\
             next-criterion: 7",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.replaced);
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(!spec.contains("2019-01-01T00:00:00Z"));
        assert!(spec.contains("  last-run: 2026-09-05T18:00:00Z"));
        // The sibling block and the key after it are untouched.
        assert!(spec.contains("review:\n  last-run: 2020-01-01T00:00:00Z"));
        assert!(spec.contains("next-criterion: 7"));
    }

    #[test]
    fn blocking_is_set_by_either_gating_tier() {
        for (hard, blocking_findings) in [(1, 0), (0, 1), (2, 3)] {
            let tmp = spec_repo("status: in-progress\ndependencies: []");
            let result = run(
                &WriteAnalysisArgs {
                    hard_fail: hard,
                    blocking_findings,
                    ..args()
                },
                tmp.path(),
            )
            .unwrap();
            assert!(result.blocking, "hard={hard} blocking={blocking_findings}");
        }
    }

    /// Advisory findings are recorded and never gate — the asymmetry with the
    /// review block is the design, not an omission.
    #[test]
    fn advisory_findings_are_recorded_but_do_not_block() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                advisory: 9,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.blocking);
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(spec.contains("  advisory: 9"));
        assert!(spec.contains("  blocking: false"));
    }

    /// The `QUAL-CLAIM-001` field: a clean run that could not examine
    /// everything must not record the same thing as one that could.
    #[test]
    fn unexamined_count_survives_a_clean_run() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(
            &WriteAnalysisArgs {
                unexamined: 3,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(spec.contains("  unexamined: 3"));
        assert!(spec.contains("  blocking: false"));
    }

    #[test]
    fn newline_in_a_scalar_cannot_inject_frontmatter_keys() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(
            &WriteAnalysisArgs {
                analyzed_against: "abc\nstatus: done".into(),
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(spec.contains("  analyzed-against: abc status: done"));
        assert!(!spec.contains("\nstatus: done"));
        assert!(spec.contains("status: in-progress"));
    }

    #[test]
    fn malformed_frontmatter_is_never_given_a_clean_record() {
        let tmp = spec_repo("status: in-progress\ndependencies: [oops");
        assert!(matches!(
            run(&args(), tmp.path()).unwrap_err(),
            PrimitiveError::Yaml { .. }
        ));
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(!spec.contains("analyze:"));
    }

    #[test]
    fn missing_feature_is_an_error() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        assert!(matches!(
            run(
                &WriteAnalysisArgs {
                    feature: "999-absent".into(),
                    ..args()
                },
                tmp.path()
            )
            .unwrap_err(),
            PrimitiveError::FeatureNotFound { .. }
        ));
    }
}
