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
use crate::schema::status::ALLOWED_STATUSES;

/// Execute the `validate-frontmatter` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::Io`] when the file cannot
/// be read or [`crate::primitives::PrimitiveError::MissingFrontmatter`]
/// when no `---` fence pair is present. YAML parse failures surface as
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
                severity: "blocking".into(),
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
                severity: "blocking".into(),
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
                    severity: "blocking".into(),
                    field: "status".into(),
                    message: format!("status '{s}' is not one of {}", ALLOWED_STATUSES.join("|")),
                });
            }
        }
        Some(_) => findings.push(FrontmatterFinding {
            severity: "blocking".into(),
            field: "status".into(),
            message: "status must be a string".into(),
        }),
        None => findings.push(FrontmatterFinding {
            severity: "blocking".into(),
            field: "status".into(),
            message: "status is missing".into(),
        }),
    }

    match map.get("dependencies") {
        Some(YamlValue::Sequence(items)) => {
            for (i, item) in items.iter().enumerate() {
                if !matches!(item, YamlValue::String(_)) {
                    findings.push(FrontmatterFinding {
                        severity: "blocking".into(),
                        field: format!("dependencies[{i}]"),
                        message: "dependency entry must be a string feature name".into(),
                    });
                }
            }
        }
        Some(_) => findings.push(FrontmatterFinding {
            severity: "blocking".into(),
            field: "dependencies".into(),
            message: "dependencies must be a list".into(),
        }),
        None => findings.push(FrontmatterFinding {
            severity: "blocking".into(),
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

    if let Some(review) = map.get("review") {
        validate_review_block(review, &mut findings);
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
            severity: "blocking".into(),
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
            severity: "blocking".into(),
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
            severity: "blocking".into(),
            field: "cross-spec-impact".into(),
            message: "cross-spec-impact must be a list of feature names".into(),
        });
        return;
    };
    for entry in entries {
        if !matches!(entry, YamlValue::String(_)) {
            findings.push(FrontmatterFinding {
                severity: "blocking".into(),
                field: "cross-spec-impact".into(),
                message: "cross-spec-impact entry must be a string feature name".into(),
            });
        }
    }
}

fn validate_review_block(review: &YamlValue, findings: &mut Vec<FrontmatterFinding>) {
    let YamlValue::Mapping(map) = review else {
        findings.push(FrontmatterFinding {
            severity: "blocking".into(),
            field: "review".into(),
            message: "review must be a mapping".into(),
        });
        return;
    };
    for key in ["must-violations", "should-violations", "low-confidence"] {
        if let Some(value) = map.get(key)
            && !matches!(value, YamlValue::Number(_))
        {
            findings.push(FrontmatterFinding {
                severity: "blocking".into(),
                field: format!("review.{key}"),
                message: "must be a number".into(),
            });
        }
    }
    if let Some(value) = map.get("blocking")
        && !matches!(value, YamlValue::Bool(_))
    {
        findings.push(FrontmatterFinding {
            severity: "blocking".into(),
            field: "review.blocking".into(),
            message: "must be a boolean".into(),
        });
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
        assert!(result.findings.is_empty());
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
        assert!(findings_for("status: draft\ndependencies: []\n").is_empty());
        assert!(
            findings_for("status: draft\ndependencies: []\ncross-spec-impact: []\n").is_empty()
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
        assert_eq!(findings[0].severity, "blocking");
    }

    #[test]
    fn a_non_string_cross_spec_impact_entry_is_blocking() {
        let findings = findings_for("status: draft\ndependencies: []\ncross-spec-impact: [50]\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].field, "cross-spec-impact");
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
        assert_eq!(findings[0].severity, "blocking");
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
    fn missing_status_is_blocking() {
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
        assert_eq!(result.findings[0].severity, "blocking");
        assert_eq!(result.findings[0].field, "status");
        assert_eq!(result.findings[0].message, "status is missing");
    }

    #[test]
    fn missing_dependencies_is_blocking() {
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
        assert_eq!(result.findings[0].severity, "blocking");
        assert_eq!(result.findings[0].field, "dependencies");
        assert_eq!(result.findings[0].message, "dependencies is missing");
    }

    #[test]
    fn empty_frontmatter_reports_both_missing_fields() {
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
        assert!(result.findings.iter().all(|f| f.severity == "blocking"));
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
