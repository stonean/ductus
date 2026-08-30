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

    if let Some(supersedes) = map.get("supersedes") {
        // The spec's own feature directory, used only for the
        // self-reference check. A scenario file's parent is `scenarios`,
        // which matches no feature name, so the check simply never fires
        // there — correct, since the key belongs to spec files.
        let own_feature = path
            .parent()
            .and_then(std::path::Path::file_name)
            .and_then(std::ffi::OsStr::to_str);
        validate_supersedes(supersedes, own_feature, &mut findings);
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

/// Check the optional `supersedes` key: the specs this one supersedes
/// (spec 052).
///
/// **Shape only, never resolvability** — the same split `folds-into`
/// takes, for a different reason. There the target routinely lives on
/// another branch; here both specs normally exist, but the key is written
/// by a declaring command that has already refused an unresolvable target,
/// and a second copy of that check here would be the drift this project
/// spends its effort avoiding. What shape validation owns is the corpus
/// nobody declared through a command: a hand-edited key.
///
/// Any feature form is accepted. `folds-into` requires the sequential form
/// because chaining one staging spec into another would leave a fold with
/// no permanent home; superseding carries no such chain, so restricting the
/// form here would invent a constraint the spec does not state.
///
/// A **self-reference is rejected**: a spec cannot supersede itself, and
/// unlike `derive-dependencies` — which records self-links deliberately so
/// its cycle check can surface them — nothing downstream would catch it.
fn validate_supersedes(
    supersedes: &YamlValue,
    own_feature: Option<&str>,
    findings: &mut Vec<FrontmatterFinding>,
) {
    let YamlValue::Sequence(entries) = supersedes else {
        findings.push(FrontmatterFinding {
            severity: "blocking".into(),
            field: "supersedes".into(),
            message: "supersedes must be a list of feature names; one spec may supersede \
                      several, so the key is a list even when it holds a single entry"
                .into(),
        });
        return;
    };

    for entry in entries {
        let YamlValue::String(name) = entry else {
            findings.push(FrontmatterFinding {
                severity: "blocking".into(),
                field: "supersedes".into(),
                message: "supersedes entries must be strings naming a feature".into(),
            });
            continue;
        };

        if parse_feature_dir(name).is_none() {
            findings.push(FrontmatterFinding {
                severity: "blocking".into(),
                field: "supersedes".into(),
                message: format!("supersedes entry {name:?} is not a feature name"),
            });
            continue;
        }

        if own_feature == Some(name.as_str()) {
            findings.push(FrontmatterFinding {
                severity: "blocking".into(),
                field: "supersedes".into(),
                message: format!("a spec cannot supersede itself ({name:?})"),
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

    /// Write a spec inside a real feature directory, so the self-reference
    /// check has a feature name to compare against. `findings_for` puts
    /// `spec.md` at the tempdir root, whose name matches no feature.
    fn findings_for_feature(feature: &str, frontmatter: &str) -> Vec<FrontmatterFinding> {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs").join(feature);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("spec.md");
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
    fn a_well_shaped_supersedes_list_is_clean() {
        // Several entries in one declaration is the normal case, not an
        // edge one: 043 superseded material in four specs at once.
        let findings = findings_for_feature(
            "052-supersession",
            "status: draft\ndependencies: []\nsupersedes: [005-workflows, 019-config-decisions]\n",
        );
        assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn a_superseded_spec_absent_from_the_tree_is_not_a_finding() {
        // Shape only. Resolvability is the declaring command's to refuse,
        // and a second copy of that check here is the drift this project
        // spends its effort avoiding.
        let findings = findings_for_feature(
            "052-supersession",
            "status: draft\ndependencies: []\nsupersedes: [404-nowhere-near-here]\n",
        );
        assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn supersedes_must_be_a_list() {
        // A scalar is refused rather than coerced: one spec may supersede
        // several, so the key is a list even holding a single entry.
        let findings = findings_for_feature(
            "052-supersession",
            "status: draft\ndependencies: []\nsupersedes: 005-workflows\n",
        );
        assert!(findings.iter().any(|f| f.field == "supersedes"));
    }

    #[test]
    fn a_supersedes_entry_that_is_not_a_feature_name_is_blocking() {
        let findings = findings_for_feature(
            "052-supersession",
            "status: draft\ndependencies: []\nsupersedes: [not a feature]\n",
        );
        assert!(
            findings
                .iter()
                .any(|f| f.field == "supersedes" && f.severity == "blocking")
        );
    }

    #[test]
    fn a_spec_cannot_supersede_itself() {
        // Nothing downstream would catch this. `derive-dependencies`
        // records self-links on purpose so its cycle check surfaces them;
        // there is no equivalent pass here.
        let findings = findings_for_feature(
            "052-supersession",
            "status: draft\ndependencies: []\nsupersedes: [052-supersession]\n",
        );
        assert!(
            findings
                .iter()
                .any(|f| f.field == "supersedes" && f.message.contains("cannot supersede itself"))
        );
    }
}
