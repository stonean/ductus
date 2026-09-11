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

use std::collections::BTreeMap;
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
    // The breakdown is the authority when supplied: a total a caller can
    // contradict is a total that will eventually be contradicted, which is
    // the same reason `blocking` is derived rather than accepted.
    let by_reason: BTreeMap<String, u32> =
        args.unexamined_by_reason
            .iter()
            .fold(BTreeMap::new(), |mut acc, (reason, count)| {
                *acc.entry(reason.clone()).or_default() += *count;
                acc
            });
    // A registered shared constitution this run could not read is an unexamined
    // *input* to the analysis, not an unexamined target, but it lands in the same
    // breakdown because it makes the same claim false: that everything bearing on
    // the verdict was looked at. Derived here rather than accepted as an argument,
    // for the reason the comment above gives — a caller that had to supply it
    // could omit it, and `QUAL-CLAIM-001` exists precisely to stop a clean result
    // standing in for an unexamined one (spec 055, AC8).
    let mut by_reason = by_reason;
    // Deliberately NOT `?`. This primitive's contract is that it writes a
    // record on every run, because the record's *absence* is what a later
    // gate reads as "never analyzed" — so a config that will not parse must
    // not be able to suppress it. An unreadable registry is recorded as its
    // own reason instead, which is the honest answer: nothing registered was
    // loaded, and the run cannot say how many sources that was.
    match crate::primitives::resolve_constitutions::run(
        &crate::schema::primitives::ResolveConstitutionsArgs {},
        repo,
    ) {
        Ok(governance) if !governance.skipped.is_empty() => {
            let count = u32::try_from(governance.skipped.len()).unwrap_or(u32::MAX);
            *by_reason
                .entry("constitution-unresolved".to_string())
                .or_default() += count;
        }
        Ok(_) => {}
        Err(_) => {
            *by_reason
                .entry("constitution-registry-unreadable".to_string())
                .or_default() += 1;
        }
    }
    let by_reason = by_reason;

    let unexamined = if by_reason.is_empty() {
        args.unexamined
    } else {
        by_reason.values().copied().sum()
    };
    // The digest of what this run examined, taken here rather than accepted
    // as an argument. `/{project}:analyze` is read-only, so the subjects are
    // byte-identical to the ones its passes read moments ago — and deriving it
    // means no caller can record a digest it did not take, the same discipline
    // that derives `blocking` and sums `unexamined` from its breakdown. It
    // also guarantees this digest and the one the gate recomputes come from
    // one function, which is what makes the two surfaces agree.
    let subjects = crate::primitives::analyze_subjects::subject_digest(
        &spec_path
            .parent()
            .map_or_else(|| repo.to_path_buf(), std::path::Path::to_path_buf),
        crate::primitives::analyze_subjects::is_analyze_subject,
    );
    let block = render_analyze_yaml(args, blocking, unexamined, &by_reason, &subjects);
    let new_fm = splice_top_level_block(fm_text, "analyze", &block);
    let rendered = crate::primitives::with_line_ending(
        &format!("---\n{new_fm}\n---\n{body}"),
        crate::primitives::line_ending_of(&content),
    );
    write_atomic(&spec_path, &rendered)?;

    Ok(WriteAnalysisResult {
        spec_path: rel_path(&spec_path, repo),
        blocking,
        unexamined,
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
fn render_analyze_yaml(
    args: &WriteAnalysisArgs,
    blocking: bool,
    unexamined: u32,
    by_reason: &BTreeMap<String, u32>,
    subjects: &crate::primitives::analyze_subjects::SubjectDigest,
) -> String {
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
    let _ = writeln!(block, "  unexamined: {unexamined}");
    // The record's description of its own subject. Paths are feature-relative
    // and the digests are hex, so neither needs quoting; both are derived from
    // the filesystem rather than supplied, so neither can carry a newline.
    if !subjects.digests.is_empty() {
        let _ = writeln!(block, "  analyzed-digest:");
        for (path, digest) in &subjects.digests {
            let _ = writeln!(block, "    {path}: {digest}");
        }
    }
    if !subjects.unreadable.is_empty() {
        let _ = writeln!(block, "  analyzed-unreadable:");
        for path in &subjects.unreadable {
            let _ = writeln!(block, "    - {path}");
        }
    }
    // Omitted when empty, so a fully-examined run carries no map rather than
    // a map of zeroes. Reasons are a closed set of kebab-case identifiers, so
    // no quoting is needed; a reason outside it would be a caller defect the
    // arg parser has already rejected as unparseable rather than reshaped.
    if !by_reason.is_empty() {
        let _ = writeln!(block, "  unexamined-by-reason:");
        for (reason, count) in by_reason {
            let _ = writeln!(block, "    {reason}: {count}");
        }
    }
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
            unexamined_by_reason: vec![],
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

    /// A bare total answers *that* something was unexamined and nothing
    /// about what — and the reasons are not equivalent. The breakdown is what
    /// makes the number actionable.
    #[test]
    fn the_breakdown_is_written_and_sorted() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined_by_reason: vec![
                    ("ships-to-adopter".into(), 10),
                    ("not-a-live-claim".into(), 81),
                    ("artifact-unreadable".into(), 1),
                ],
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 92);
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(spec.contains("  unexamined: 92"));
        // BTreeMap order, so the rendering is byte-stable across runs.
        let block = spec.split("unexamined-by-reason:").nth(1).unwrap();
        let order: Vec<&str> = block
            .lines()
            .skip(1) // the remainder of the `unexamined-by-reason:` line itself
            .take_while(|l| l.starts_with("    "))
            .map(|l| l.trim().split(':').next().unwrap())
            .collect();
        assert_eq!(
            order,
            vec![
                "artifact-unreadable",
                "not-a-live-claim",
                "ships-to-adopter"
            ]
        );
    }

    /// The breakdown is the authority: a total a caller can contradict is a
    /// total that will eventually be contradicted, which is why `blocking` is
    /// derived too.
    #[test]
    fn a_supplied_total_cannot_contradict_its_breakdown() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined: 999,
                unexamined_by_reason: vec![("root-absent".into(), 4)],
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 4);
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(spec.contains("  unexamined: 4"));
        // Anchored to the field, not to the bare digits: the block now also
        // carries hex digests, and a bare `999` match can land inside one.
        assert!(!spec.contains("unexamined: 999"));
    }

    /// A fully-examined run carries no map rather than a map of zeroes.
    #[test]
    fn an_empty_breakdown_omits_the_map_and_keeps_the_supplied_total() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined: 3,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 3);
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(spec.contains("  unexamined: 3"));
        assert!(!spec.contains("unexamined-by-reason"));
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

    /// Register one `[constitutions.*]` entry pointing at `path`.
    fn with_constitution(tmp: &TempDir, alias: &str, path: &str) {
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(
            cfg.join("config.toml"),
            format!(
                "[constitutions.{alias}]\nrepo = \"https://example.test/g\"\npath = \"{path}\"\n"
            ),
        )
        .unwrap();
    }

    #[test]
    fn an_unresolved_constitution_is_recorded_as_unexamined() {
        // AC8: an analysis that ran without a registered source must not record a
        // clean, fully-examined run. The reason lands in the breakdown the record
        // already carries, so the pre-done gate and any reader see it.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_constitution(&tmp, "acme", "nowhere");

        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.unexamined, 1);
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(spec.contains("  unexamined: 1"), "{spec}");
        assert!(spec.contains("constitution-unresolved: 1"), "{spec}");
    }

    #[test]
    fn a_resolved_constitution_records_nothing_unexamined() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_constitution(&tmp, "acme", "gov");
        let gov = tmp.path().join("gov");
        fs::create_dir_all(&gov).unwrap();
        fs::write(gov.join("constitution.md"), "# House rules\n").unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(
            result.unexamined, 0,
            "a source that was read is not unexamined"
        );
    }

    #[test]
    fn no_registered_constitution_leaves_the_count_untouched() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.unexamined, 0);
        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(
            !spec.contains("constitution-unresolved"),
            "a project with none registered reads exactly as before: {spec}"
        );
    }

    #[test]
    fn a_malformed_config_still_records_the_run() {
        // Regression: the registry read was `?`-propagated, so an unrelated
        // config typo suppressed the analyze record entirely -- and an absent
        // record is exactly what the pre-done gate reads as "never analyzed".
        // This primitive writes a record on every run, by contract.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(cfg.join("config.toml"), "[constitutions.acme]\nrepo = \n").unwrap();

        let result = run(&args(), tmp.path());
        assert!(result.is_ok(), "a config typo must not suppress the record");

        let spec = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert!(
            spec.contains("constitution-registry-unreadable: 1"),
            "{spec}"
        );
        assert!(spec.contains("  unexamined: 1"), "{spec}");
    }
}
