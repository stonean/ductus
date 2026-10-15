//! `validate-frontmatter` — full frontmatter schema check.
//!
//! Ports the semantics of `scripts/lint-frontmatter.sh` with real YAML
//! parsing rather than the shell-side shape check: every issue is reported
//! as a `FrontmatterFinding` rather than printed to stdout.

use std::path::Path;

use serde_norway::Value as YamlValue;

use crate::primitives::{
    FeatureForm, Result, parse_feature_dir, read_text, resolve_path, split_frontmatter,
};
use crate::schema::primitives::{
    FrontmatterFinding, ValidateFrontmatterArgs, ValidateFrontmatterResult,
};
use crate::schema::severity::AnalyzeSeverity;
use crate::schema::status::ALLOWED_STATUSES;

/// Execute the `validate-frontmatter` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::Io`] when the file cannot
/// be read, [`crate::primitives::PrimitiveError::MissingFrontmatter`]
/// when no opening `---` fence is present, or
/// [`crate::primitives::PrimitiveError::UnclosedFrontmatter`] when no closing
/// one follows it. YAML parse failures surface as
/// findings, not operational errors.
pub fn run(args: &ValidateFrontmatterArgs, repo: &Path) -> Result<ValidateFrontmatterResult> {
    let path = resolve_path(repo, &args.path);
    let content = read_text(&path)?;
    let (fm_text, _body) = split_frontmatter(&content, &path)?;

    let mut findings: Vec<FrontmatterFinding> = Vec::new();
    let parsed: YamlValue = match serde_norway::from_str(fm_text) {
        Ok(v) => v,
        Err(e) => {
            findings.push(FrontmatterFinding {
                severity: AnalyzeSeverity::HardFail,
                field: String::new(),
                message: format!("frontmatter is not valid YAML: {e}"),
            });
            return Ok(ValidateFrontmatterResult {
                findings,
                clean: false,
            });
        }
    };

    // An empty frontmatter block (`---\n---\n`) parses as YAML null; treat
    // it as an empty mapping so the required-field checks below report
    // per-field findings rather than a misleading "must be a mapping".
    let empty_map = serde_norway::Mapping::new();
    let map = match &parsed {
        YamlValue::Mapping(map) => map,
        YamlValue::Null => &empty_map,
        _ => {
            findings.push(FrontmatterFinding {
                severity: AnalyzeSeverity::HardFail,
                field: String::new(),
                message: "frontmatter must be a mapping".into(),
            });
            return Ok(ValidateFrontmatterResult {
                findings,
                clean: false,
            });
        }
    };

    // `status` and `dependencies` are required on spec frontmatter —
    // absence is hard-fail per constitution §text-first-artifacts
    // (Validation Severity), same tier as an invalid value.
    match map.get("status") {
        Some(YamlValue::String(s)) => {
            if !ALLOWED_STATUSES.contains(&s.as_str()) {
                findings.push(FrontmatterFinding {
                    severity: AnalyzeSeverity::HardFail,
                    field: "status".into(),
                    message: format!("status '{s}' is not one of {}", ALLOWED_STATUSES.join("|")),
                });
            }
        }
        Some(_) => findings.push(FrontmatterFinding {
            severity: AnalyzeSeverity::HardFail,
            field: "status".into(),
            message: "status must be a string".into(),
        }),
        None => findings.push(FrontmatterFinding {
            severity: AnalyzeSeverity::HardFail,
            field: "status".into(),
            message: "status is missing".into(),
        }),
    }

    match map.get("dependencies") {
        Some(YamlValue::Sequence(items)) => {
            for (i, item) in items.iter().enumerate() {
                if !matches!(item, YamlValue::String(_)) {
                    findings.push(FrontmatterFinding {
                        severity: AnalyzeSeverity::HardFail,
                        field: format!("dependencies[{i}]"),
                        message: "dependency entry must be a string feature name".into(),
                    });
                }
            }
        }
        Some(_) => findings.push(FrontmatterFinding {
            severity: AnalyzeSeverity::HardFail,
            field: "dependencies".into(),
            message: "dependencies must be a list".into(),
        }),
        None => findings.push(FrontmatterFinding {
            severity: AnalyzeSeverity::HardFail,
            field: "dependencies".into(),
            message: "dependencies is missing".into(),
        }),
    }

    if let Some(folds_into) = map.get("folds-into") {
        validate_folds_into(folds_into, &mut findings);
    }

    if let Some(impact) = map.get("cross-spec-impact") {
        validate_cross_spec_impact(impact, &mut findings);
    }

    validate_no_residual_records(map, &mut findings);

    // Each record is validated where it now lives. Validating the spec covers
    // the whole feature, so no caller has to learn a second path to check.
    if path.file_name().is_some_and(|name| name == "spec.md")
        && let Some(dir) = path.parent()
    {
        validate_record_artifacts(dir, &mut findings);
    }

    let clean = findings.is_empty();
    Ok(ValidateFrontmatterResult { findings, clean })
}

/// Check the optional `folds-into` key: the upstream spec a
/// branch-scoped spec folds back into (spec 051).
///
/// **Shape only, never resolvability.** The value routinely names a spec
/// this working tree cannot see: a branch-scoped spec exists because the
/// upstream branch moved, so its target normally lives on that branch —
/// sometimes created there after this branch forked. Requiring the target
/// to resolve would fire on the feature's normal case, and could not see
/// the tree that would satisfy it in any event. Existence is enforced at
/// fold-back, which runs after the merge, in the first tree holding both.
///
/// Requiring the *sequential* form is not incidental: it is what forbids
/// chaining one branch-scoped spec into another, so a fold always ends at
/// a permanent home in one hop.
///
/// Absence is never a finding. A sequential spec has no fold target by
/// definition, and while `create-feature` refuses branch-scoped creation
/// without one, removing the key by hand is the supported way to make
/// such a spec stand on its own.
fn validate_folds_into(folds_into: &YamlValue, findings: &mut Vec<FrontmatterFinding>) {
    let YamlValue::String(target) = folds_into else {
        findings.push(FrontmatterFinding {
            severity: AnalyzeSeverity::Blocking,
            field: "folds-into".into(),
            message: "folds-into must be a string feature name".into(),
        });
        return;
    };
    if !matches!(
        parse_feature_dir(target),
        Some(FeatureForm::Sequential { .. })
    ) {
        findings.push(FrontmatterFinding {
            severity: AnalyzeSeverity::Blocking,
            field: "folds-into".into(),
            message: format!(
                "folds-into {target:?} is not a sequential feature name (NNN-slug); a \
                 branch-scoped spec folds into a permanent spec, not another staged one"
            ),
        });
    }
}

/// Check the optional `cross-spec-impact` key: the specs this one owes a
/// change to under §cross-spec-impact (spec 050).
///
/// **Shape only.** Whether the named spec exists, and whether the obligation
/// has been discharged, are the pre-`done` gate's questions
/// (`check-review-gate`), answered against the corpus at the moment of the
/// transition. Answering them here would duplicate that gate in a command
/// that runs on artifacts rather than on a transition, and the two would
/// drift.
///
/// It is checked here at all because a malformed value would otherwise reach
/// the gate as a deserialization error rather than as a named finding, and
/// naming the field is this command's job.
///
/// Absence is never a finding, and neither is an empty list: most specs
/// affect no other spec, so the two are the same state.
fn validate_cross_spec_impact(impact: &YamlValue, findings: &mut Vec<FrontmatterFinding>) {
    let YamlValue::Sequence(entries) = impact else {
        findings.push(FrontmatterFinding {
            severity: AnalyzeSeverity::Blocking,
            field: "cross-spec-impact".into(),
            message: "cross-spec-impact must be a list of feature names".into(),
        });
        return;
    };
    for entry in entries {
        if !matches!(entry, YamlValue::String(_)) {
            findings.push(FrontmatterFinding {
                severity: AnalyzeSeverity::Blocking,
                field: "cross-spec-impact".into(),
                message: "cross-spec-impact entry must be a string feature name".into(),
            });
        }
    }
}

/// Report a `review:` or `analyze:` block still present in spec frontmatter.
///
/// The open-schema rule admits fields nothing has claimed; it does not
/// re-admit a field this schema has moved (spec 057). A second copy of a
/// gate-read record is the drift condition, not an unknown field, so it is
/// reported rather than ignored — and the finding names the remedy, because an
/// operator who has just upgraded has no other way to know what moved.
///
/// The block's *fields* are no longer checked here. They are validated where
/// the record now lives, by deserializing the artifact into the record type —
/// a strictly stronger check than the hand-rolled number-and-bool pass this
/// replaced, and one that cannot drift from the type it validates.
fn validate_no_residual_records(
    map: &serde_norway::Mapping,
    findings: &mut Vec<FrontmatterFinding>,
) {
    for key in ["review", "analyze"] {
        if map.get(key).is_some() {
            let home = if key == "review" {
                "review.md"
            } else {
                "analysis.md"
            };
            findings.push(FrontmatterFinding {
                severity: AnalyzeSeverity::Blocking,
                field: key.into(),
                message: format!(
                    "`{key}:` no longer belongs in spec frontmatter — the record lives in {home}. Run the record-relocation migration to move it."
                ),
            });
        }
    }
}

/// Validate the two record artifacts beside a spec.
///
/// **Absence is never a finding.** A feature with no `review.md` has not been
/// reviewed, and that is a state the pre-`done` gate reports — not a schema
/// defect. What *is* a defect is an artifact that exists and carries no
/// readable record: the gate cannot tell from it whether the run happened, so
/// it is reported here rather than left for a later command to stumble on.
fn validate_record_artifacts(dir: &Path, findings: &mut Vec<FrontmatterFinding>) {
    use crate::primitives::RecordLoad;

    let review_unreadable = match crate::primitives::load_review_record(dir) {
        RecordLoad::Unreadable(reason) => Some(reason),
        RecordLoad::Absent | RecordLoad::Present(_) => None,
    };
    let analysis_unreadable = match crate::primitives::load_analyze_record(dir) {
        RecordLoad::Unreadable(reason) => Some(reason),
        RecordLoad::Absent | RecordLoad::Present(_) => None,
    };
    for (file, unreadable) in [
        (crate::primitives::REVIEW_RECORD_FILE, review_unreadable),
        (crate::primitives::ANALYSIS_RECORD_FILE, analysis_unreadable),
    ] {
        if let Some(reason) = unreadable {
            findings.push(FrontmatterFinding {
                severity: AnalyzeSeverity::HardFail,
                field: file.into(),
                message: format!("{file} exists but carries no readable record: {reason}"),
            });
            continue;
        }
        // The stored decisions live beside the record rather than in it, so a
        // malformed list does not make the record unreadable — and must not
        // read as empty either, because an empty list re-asks every settled
        // question (spec 058). Reported here so the defect is named before a
        // writer refuses over it.
        if let Err(error) = crate::primitives::decisions::read_decisions(dir, file) {
            findings.push(FrontmatterFinding {
                severity: AnalyzeSeverity::HardFail,
                field: format!("{file} decisions"),
                message: format!("{file}'s decisions: list does not parse: {error}"),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::path::PathBuf;

    fn fixture_repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/primitives/sample-repo")
    }

    #[test]
    fn fixture_spec_is_clean() {
        let repo = fixture_repo();
        let result = run(
            &ValidateFrontmatterArgs {
                path: "specs/001-basic/spec.md".into(),
            },
            &repo,
        )
        .unwrap();
        assert!(result.clean, "expected clean, got {:?}", result.findings);
        assert_eq!(result.findings, [] as [FrontmatterFinding; 0]);
    }

    /// Run the validator over a frontmatter block, returning the findings.
    #[test]
    fn a_well_formed_cross_spec_impact_list_is_clean() {
        let findings = findings_for(
            "status: draft\ndependencies: []\ncross-spec-impact: [050-constitution]\n",
        );
        assert!(findings.is_empty(), "{findings:?}");
    }

    /// Absent and empty are the same state: most specs affect no other spec,
    /// so neither is a finding.
    #[test]
    fn an_empty_or_absent_cross_spec_impact_is_never_a_finding() {
        assert_eq!(
            findings_for("status: draft\ndependencies: []\n"),
            [] as [FrontmatterFinding; 0]
        );
        assert_eq!(
            findings_for("status: draft\ndependencies: []\ncross-spec-impact: []\n"),
            [] as [FrontmatterFinding; 0]
        );
    }

    /// A bare string is the natural typo for a one-entry list, and without
    /// this it would surface at the pre-`done` gate as a YAML deserialization
    /// error rather than as a named field finding.
    #[test]
    fn a_scalar_cross_spec_impact_is_blocking() {
        let findings =
            findings_for("status: draft\ndependencies: []\ncross-spec-impact: 050-constitution\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].field, "cross-spec-impact");
        assert_eq!(findings[0].severity, AnalyzeSeverity::Blocking);
    }

    #[test]
    fn a_non_string_cross_spec_impact_entry_is_blocking() {
        let findings = findings_for("status: draft\ndependencies: []\ncross-spec-impact: [50]\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].field, "cross-spec-impact");
    }

    /// A residual `review:` or `analyze:` block is reported, not tolerated.
    ///
    /// The open-schema rule admits fields nothing has claimed; it must not
    /// re-admit a field this schema has moved, or the relocation would leave
    /// two homes for one gate-read fact and no signal that it had.
    #[test]
    fn a_residual_record_block_in_the_spec_is_reported() {
        for key in ["review", "analyze"] {
            let findings = findings_for(&format!(
                "status: done\ndependencies: []\n{key}:\n  blocking: false\n"
            ));
            let found = findings
                .iter()
                .find(|f| f.field == key)
                .unwrap_or_else(|| panic!("no finding for residual {key}: {findings:?}"));
            assert_eq!(found.severity, AnalyzeSeverity::Blocking);
            assert!(
                found.message.contains("no longer belongs"),
                "{}",
                found.message
            );
            assert!(
                found.message.contains("migration"),
                "the finding names the remedy: {}",
                found.message
            );
        }
    }

    /// A migrated spec validates clean — the check fires on the residual, not
    /// on the relocation.
    #[test]
    fn a_migrated_spec_is_clean() {
        let findings = findings_for("status: done\ndependencies: []\n");
        assert!(findings.is_empty(), "{findings:?}");
    }

    /// An **absent** record artifact is never a finding: a feature with no
    /// review.md has not been reviewed, which is a state the gate reports, not
    /// a schema defect.
    #[test]
    fn an_absent_record_artifact_is_not_a_finding() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\nstatus: draft\ndependencies: []\n---\n\n# X\n").unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    /// A record artifact that exists and carries nothing readable **is** a
    /// defect: the gate cannot tell from it whether the run happened.
    #[test]
    fn an_unreadable_record_artifact_is_a_hard_fail() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\nstatus: draft\ndependencies: []\n---\n\n# X\n").unwrap();
        // Present, but with no frontmatter fence at all.
        std::fs::write(tmp.path().join("review.md"), "# Review\n\nno record here\n").unwrap();
        // Present and fenced, but the record's types do not parse.
        std::fs::write(
            tmp.path().join("analysis.md"),
            "---\nhard-fail: not-a-number\n---\n\n# Analysis\n",
        )
        .unwrap();

        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.clean);
        for field in ["review.md", "analysis.md"] {
            let found = result
                .findings
                .iter()
                .find(|f| f.field == field)
                .unwrap_or_else(|| panic!("no finding for {field}: {:?}", result.findings));
            assert_eq!(found.severity, AnalyzeSeverity::HardFail);
            assert!(
                found.message.contains("carries no readable record"),
                "{}",
                found.message
            );
        }
    }

    /// A `decisions:` list that will not parse is named, never read as empty —
    /// and an absent list is no finding at all.
    #[test]
    fn a_malformed_decisions_list_is_a_hard_fail_in_either_record() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\nstatus: draft\ndependencies: []\n---\n\n# X\n").unwrap();
        for file in ["review.md", "analysis.md"] {
            std::fs::write(
                tmp.path().join(file),
                "---\nlast-run: 2026-09-25T00:00:00Z\ndecisions: not-a-list\n---\n",
            )
            .unwrap();
        }
        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        for field in ["review.md decisions", "analysis.md decisions"] {
            let found = result
                .findings
                .iter()
                .find(|f| f.field == field)
                .unwrap_or_else(|| panic!("no finding for {field}: {:?}", result.findings));
            assert_eq!(found.severity, AnalyzeSeverity::HardFail);
            assert!(
                found.message.contains("does not parse"),
                "{}",
                found.message
            );
        }
    }

    #[test]
    fn an_absent_decisions_list_is_no_finding() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\nstatus: draft\ndependencies: []\n---\n\n# X\n").unwrap();
        std::fs::write(
            tmp.path().join("analysis.md"),
            "---\nlast-run: 2026-09-25T00:00:00Z\nblocking: false\n---\n",
        )
        .unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(
            !result
                .findings
                .iter()
                .any(|f| f.field.ends_with("decisions")),
            "{:?}",
            result.findings
        );
    }

    fn findings_for(frontmatter: &str) -> Vec<FrontmatterFinding> {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, format!("---\n{frontmatter}---\n\n# X\n")).unwrap();
        run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap()
        .findings
    }

    #[test]
    fn a_fold_target_naming_an_absent_feature_is_not_a_finding() {
        // The target normally lives on the upstream branch, so it is
        // absent from the tree that declares it. That is the feature's
        // normal case, not a defect — and nothing here could see the
        // tree that would resolve it anyway.
        let findings =
            findings_for("status: draft\ndependencies: []\nfolds-into: 022-nowhere-near-here\n");
        assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn a_fold_target_naming_a_branch_scoped_spec_is_blocking() {
        // Chaining one staged spec into another would leave a fold that
        // does not end at a permanent home.
        let findings = findings_for("status: draft\ndependencies: []\nfolds-into: 5678.1-other\n");
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert_eq!(findings[0].field, "folds-into");
        assert_eq!(findings[0].severity, AnalyzeSeverity::Blocking);
    }

    #[test]
    fn a_malformed_fold_target_is_blocking() {
        for bad in ["not-a-feature", "22-short", ""] {
            let findings = findings_for(&format!(
                "status: draft\ndependencies: []\nfolds-into: {bad:?}\n"
            ));
            assert_eq!(findings.len(), 1, "expected one finding for {bad:?}");
            assert_eq!(findings[0].field, "folds-into");
        }
        // Wrong type, not just wrong shape.
        let findings = findings_for("status: draft\ndependencies: []\nfolds-into: [022-a]\n");
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert_eq!(findings[0].field, "folds-into");
    }

    #[test]
    fn an_absent_fold_target_is_never_a_finding() {
        // A sequential spec has none by definition, and removing the key
        // by hand is the supported way to make a staged spec stand alone.
        let findings = findings_for("status: draft\ndependencies: []\n");
        assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn unknown_status_is_blocking() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\nstatus: wibble\ndependencies: []\n---\n\n# X\n").unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.clean);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].field, "status");
    }

    #[test]
    fn missing_status_is_hard_fail() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\ndependencies: []\n---\n\n# X\n").unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.clean);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].severity, AnalyzeSeverity::HardFail);
        assert_eq!(result.findings[0].field, "status");
        assert_eq!(result.findings[0].message, "status is missing");
    }

    #[test]
    fn missing_dependencies_is_hard_fail() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\nstatus: draft\n---\n\n# X\n").unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.clean);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].severity, AnalyzeSeverity::HardFail);
        assert_eq!(result.findings[0].field, "dependencies");
        assert_eq!(result.findings[0].message, "dependencies is missing");
    }

    #[test]
    fn empty_frontmatter_reports_both_missing_fields_as_hard_fail() {
        // Present-but-empty frontmatter is a validation finding, not a
        // MissingFrontmatter halt (scenario spec-side-parser-hardening).
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\n---\n\n# X\n").unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.clean);
        let fields: Vec<&str> = result.findings.iter().map(|f| f.field.as_str()).collect();
        assert_eq!(fields, vec!["status", "dependencies"]);
        assert!(
            result
                .findings
                .iter()
                .all(|f| f.severity == AnalyzeSeverity::HardFail)
        );
    }

    #[test]
    fn dependencies_must_be_a_list() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(
            &path,
            "---\nstatus: draft\ndependencies: not-a-list\n---\n\n# X\n",
        )
        .unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path.to_string_lossy().into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.clean);
        assert!(result.findings.iter().any(|f| f.field == "dependencies"));
    }
}
