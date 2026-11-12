//! `validate-frontmatter` — full frontmatter schema check.
//!
//! Ports the semantics of `scripts/lint-frontmatter.sh` with real YAML
//! parsing rather than the shell-side shape check: every issue is reported
//! as a `FrontmatterFinding` rather than printed to stdout.
//!
//! The path chooses which row of the constitution's frontmatter schema it is
//! checked against — a spec, a scenario, or an audit record — and any other
//! path is refused ([`run`]). A `spec.md` run covers the whole feature: the
//! spec, both audit records beside it, and every scenario under it, so
//! `/{project}:analyze` step 2 leaves no scenario to the host and the exec
//! walk, which no-ops host prose, checks them too (spec 067 scenario
//! `a-scenario-file-is-checked-against-the-scenario-schema`).

use std::path::Path;

use serde_norway::Value as YamlValue;

use crate::primitives::{
    ANALYSIS_RECORD_FILE, FeatureForm, PrimitiveError, REVIEW_RECORD_FILE, Result, contained_path,
    list_scenario_files, parse_feature_dir, read_text, split_frontmatter,
};
use crate::schema::primitives::{
    FrontmatterFinding, ValidateFrontmatterArgs, ValidateFrontmatterResult,
};
use crate::schema::severity::AnalyzeSeverity;
use crate::schema::status::ALLOWED_STATUSES;

/// Execute the `validate-frontmatter` primitive.
///
/// **The path chooses the row** of the constitution's frontmatter schema
/// (§text-first-artifacts), decided in this order so every name is
/// unambiguous ([`artifact_kind`]):
///
/// 1. a `.md` file whose parent directory is named `scenarios` is a
///    scenario, whatever its own name, so `scenarios/spec.md` and
///    `scenarios/review.md` take the scenario row;
/// 2. a file named `spec.md` takes the spec row, and its run also checks the
///    `review.md` and `analysis.md` beside it and every scenario under its
///    `scenarios/` directory, so validating the spec covers the whole feature
///    and no caller has to learn a second path to check;
/// 3. a file named `review.md` or `analysis.md` takes that audit record's
///    checks alone;
/// 4. any other path is refused, never checked against a row that does not
///    describe it (spec 067 scenario
///    `a-scenario-file-is-checked-against-the-scenario-schema`).
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::InvalidArgument`] for a path
/// of none of the four kinds, [`crate::primitives::PrimitiveError::Io`] when
/// the named file cannot be read (a direct record path that is absent
/// included), [`crate::primitives::PrimitiveError::MissingFrontmatter`] when a
/// named spec or scenario has no opening `---` fence, or
/// [`crate::primitives::PrimitiveError::UnclosedFrontmatter`] when no closing
/// one follows it. YAML parse failures surface as findings, not operational
/// errors, and so does every defect in an artifact a `spec.md` run reads
/// beside the spec: one bad scenario never hides the rest of the run.
/// [`crate::primitives::PrimitiveError::InvalidPath`] when `path` is absolute,
/// climbs out with `..`, or leaves the project through a symlink
/// ([`crate::primitives::contained_path`]).
pub fn run(args: &ValidateFrontmatterArgs, repo: &Path) -> Result<ValidateFrontmatterResult> {
    let path = contained_path(repo, &args.path)?;
    let Some(kind) = artifact_kind(&path) else {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "validate-frontmatter".into(),
            argument: "path".into(),
            reason: format!(
                "{:?} has no row in the frontmatter schema: name a feature's spec.md, a scenario \
                 under its scenarios/ directory, or its review.md or analysis.md",
                args.path
            ),
        });
    };
    let mut findings: Vec<FrontmatterFinding> = Vec::new();
    match kind {
        Artifact::Scenario => {
            let content = read_text(&path)?;
            let (fm_text, _body) = split_frontmatter(&content, &path)?;
            for (field, message) in scenario_row_problems(fm_text) {
                findings.push(FrontmatterFinding {
                    severity: AnalyzeSeverity::HardFail,
                    field: field.into(),
                    message,
                });
            }
        }
        Artifact::Spec => {
            validate_spec_row(&path, &mut findings)?;
            if let Some(dir) = path.parent() {
                validate_record_artifacts(dir, &mut findings);
                validate_scenarios_beside(dir, &mut findings);
            }
        }
        Artifact::Record(file) => {
            // Read first so an absent record is refused like any missing file,
            // rather than passing as the never-run state a `spec.md` run
            // rightly reads its absence as.
            read_text(&path)?;
            if let Some(dir) = path.parent() {
                validate_record(dir, file, &mut findings);
            }
        }
    }
    let clean = findings.is_empty();
    Ok(ValidateFrontmatterResult { findings, clean })
}

/// The schema row a path is checked against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Artifact {
    /// A feature's `spec.md`.
    Spec,
    /// A file under a feature's `scenarios/` directory.
    Scenario,
    /// One audit record, by its file name.
    Record(&'static str),
}

/// The row `path` names, or `None` for a path that has none. The scenario
/// test comes first so a file under `scenarios/` is a scenario whatever its
/// name; the extension test is case-insensitive, as the shared scenario
/// listing's is ([`crate::primitives::list_scenario_files`]).
fn artifact_kind(path: &Path) -> Option<Artifact> {
    let name = path.file_name()?.to_str()?;
    let under_scenarios = path
        .parent()
        .and_then(Path::file_name)
        .is_some_and(|dir| dir == "scenarios");
    let markdown = Path::new(name)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
    if under_scenarios && markdown {
        return Some(Artifact::Scenario);
    }
    match name {
        "spec.md" => Some(Artifact::Spec),
        REVIEW_RECORD_FILE => Some(Artifact::Record(REVIEW_RECORD_FILE)),
        ANALYSIS_RECORD_FILE => Some(Artifact::Record(ANALYSIS_RECORD_FILE)),
        _ => None,
    }
}

/// The spec row: `status` and `dependencies` required, and the optional keys
/// a spec may carry checked for shape.
fn validate_spec_row(path: &Path, findings: &mut Vec<FrontmatterFinding>) -> Result<()> {
    let content = read_text(path)?;
    let (fm_text, _body) = split_frontmatter(&content, path)?;

    let parsed: YamlValue = match serde_norway::from_str(fm_text) {
        Ok(v) => v,
        Err(e) => {
            findings.push(FrontmatterFinding {
                severity: AnalyzeSeverity::HardFail,
                field: String::new(),
                message: format!("frontmatter is not valid YAML: {e}"),
            });
            return Ok(());
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
            return Ok(());
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
        validate_folds_into(folds_into, findings);
    }

    if let Some(impact) = map.get("cross-spec-impact") {
        validate_cross_spec_impact(impact, findings);
    }

    validate_no_residual_records(map, findings);

    Ok(())
}

/// A scenario's frontmatter against the scenario row, each problem as its
/// field and message: `section`, or the legacy `spec-ref` of a pre-017
/// scenario, must be a non-empty string — the two keys
/// [`crate::primitives::ScenarioFrontmatter`] folds. A missing `status` or
/// `dependencies` is no finding, since neither belongs to this row, and any
/// other key is an unknown field the open-schema rule admits.
fn scenario_row_problems(fm_text: &str) -> Vec<(&'static str, String)> {
    let parsed: YamlValue = match serde_norway::from_str(fm_text) {
        Ok(value) => value,
        Err(e) => return vec![("", format!("frontmatter is not valid YAML: {e}"))],
    };
    let empty = serde_norway::Mapping::new();
    let map = match &parsed {
        YamlValue::Mapping(map) => map,
        YamlValue::Null => &empty,
        _ => return vec![("", "frontmatter must be a mapping".to_string())],
    };
    let names_a_section = |key: &str| matches!(map.get(key), Some(YamlValue::String(text)) if !text.trim().is_empty());
    if names_a_section("section") || names_a_section("spec-ref") {
        return Vec::new();
    }
    let problem = match (map.get("section"), map.get("spec-ref")) {
        (Some(_), _) => ("section", "section must be a non-empty string".to_string()),
        (None, Some(_)) => (
            "spec-ref",
            "spec-ref must be a non-empty string (or set section, which replaced it)".to_string(),
        ),
        (None, None) => (
            "section",
            "section is missing (a pre-017 scenario may carry spec-ref instead)".to_string(),
        ),
    };
    vec![problem]
}

/// Check each scenario under `dir`'s `scenarios/` directory, in the shared
/// scenario order. Every defect is a hard-fail finding whose `field` is the
/// scenario's `scenarios/{file}` path, as a record's is its file name: one
/// that cannot be read, has no frontmatter fence, or never closes one is
/// reported, never raised, so the rest of the run still reports. No
/// directory, or an empty one, adds nothing.
fn validate_scenarios_beside(dir: &Path, findings: &mut Vec<FrontmatterFinding>) {
    let scenarios = dir.join("scenarios");
    for name in list_scenario_files(&scenarios) {
        let field = format!("scenarios/{name}");
        let scenario = scenarios.join(&name);
        let mut report = |message: String| {
            findings.push(FrontmatterFinding {
                severity: AnalyzeSeverity::HardFail,
                field: field.clone(),
                message,
            });
        };
        let content = match read_text(&scenario) {
            Ok(content) => content,
            Err(PrimitiveError::Io { source, .. }) => {
                report(format!("{field} could not be read: {source}"));
                continue;
            }
            Err(error) => {
                report(format!("{field} could not be read: {error}"));
                continue;
            }
        };
        match split_frontmatter(&content, &scenario) {
            Ok((fm_text, _body)) => {
                for (_, message) in scenario_row_problems(fm_text) {
                    report(format!("{field}: {message}"));
                }
            }
            Err(PrimitiveError::MissingFrontmatter { .. }) => {
                report(format!(
                    "{field} has no frontmatter (no leading `---` block)"
                ));
            }
            Err(PrimitiveError::UnclosedFrontmatter { .. }) => {
                report(format!(
                    "{field}'s frontmatter opens with `---` but never closes"
                ));
            }
            Err(error) => report(format!("{field}'s frontmatter could not be read: {error}")),
        }
    }
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
    for file in [REVIEW_RECORD_FILE, ANALYSIS_RECORD_FILE] {
        validate_record(dir, file, findings);
    }
}

/// Validate one record artifact beside a spec, `file` being
/// [`REVIEW_RECORD_FILE`] or [`ANALYSIS_RECORD_FILE`]: absent is no finding
/// here (the direct-path caller refuses an absent file first), a record that
/// will not load is a hard fail, and so is a `decisions:` list that will not
/// parse.
fn validate_record(dir: &Path, file: &str, findings: &mut Vec<FrontmatterFinding>) {
    use crate::primitives::RecordLoad;

    let unreadable = if file == REVIEW_RECORD_FILE {
        match crate::primitives::load_review_record(dir) {
            RecordLoad::Unreadable(reason) => Some(reason),
            RecordLoad::Absent | RecordLoad::Present(_) => None,
        }
    } else {
        match crate::primitives::load_analyze_record(dir) {
            RecordLoad::Unreadable(reason) => Some(reason),
            RecordLoad::Absent | RecordLoad::Present(_) => None,
        }
    };
    if let Some(reason) = unreadable {
        findings.push(FrontmatterFinding {
            severity: AnalyzeSeverity::HardFail,
            field: file.into(),
            message: format!("{file} exists but carries no readable record: {reason}"),
        });
        return;
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
        assert_eq!(findings[0].severity, AnalyzeSeverity::Blocking);
        assert_eq!(
            findings[0].message,
            "cross-spec-impact entry must be a string feature name"
        );
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
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
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
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
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

    /// A rule verdict map, unjudged list or digest map that does not parse is
    /// a hard fail naming the record, never an analysis that judged nothing
    /// (spec 082).
    #[test]
    fn a_malformed_rule_verdict_map_is_a_hard_fail() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\nstatus: draft\ndependencies: []\n---\n\n# X\n").unwrap();
        std::fs::write(
            tmp.path().join("analysis.md"),
            "---\nlast-run: 2026-09-25T00:00:00Z\nrule-verdicts:\n  BE-LOG-002: {verdict: maybe}\n---\n",
        )
        .unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
            },
            tmp.path(),
        )
        .unwrap();
        let found = result
            .findings
            .iter()
            .find(|f| f.field == "analysis.md")
            .unwrap_or_else(|| panic!("no finding: {:?}", result.findings));
        assert_eq!(found.severity, AnalyzeSeverity::HardFail);
        assert!(
            found.message.contains("carries no readable record"),
            "{}",
            found.message
        );
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
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
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
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
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

    /// Lay `files` (repo-relative path, content) out in a fresh tree and run
    /// the validator on `path` there.
    fn run_in_tree(files: &[(&str, &str)], path: &str) -> Result<ValidateFrontmatterResult> {
        let tmp = tempfile::tempdir().unwrap();
        for (rel, content) in files {
            let file = tmp.path().join(rel);
            std::fs::create_dir_all(file.parent().unwrap()).unwrap();
            std::fs::write(file, content).unwrap();
        }
        run(&ValidateFrontmatterArgs { path: path.into() }, tmp.path())
    }

    const SPEC: &str = "specs/001-x/spec.md";
    const CLEAN_SPEC: &str = "---\nstatus: draft\ndependencies: []\n---\n\n# X\n";
    const SCENARIO: &str = "specs/001-x/scenarios/a-case.md";

    /// A scenario carrying only `section` is well formed: the spec row's
    /// `status` and `dependencies` are not asked of it.
    #[test]
    fn a_section_only_scenario_is_clean() {
        let result = run_in_tree(
            &[(SCENARIO, "---\nsection: \"Behavior\"\n---\n\n# A\n")],
            SCENARIO,
        )
        .unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    #[test]
    fn a_scenario_missing_section_is_a_hard_fail() {
        for frontmatter in ["owner: x\n", "", "section: \"  \"\n", "section: 7\n"] {
            let result = run_in_tree(
                &[(SCENARIO, &format!("---\n{frontmatter}---\n\n# A\n"))],
                SCENARIO,
            )
            .unwrap();
            assert_eq!(
                result.findings.len(),
                1,
                "{frontmatter:?}: {:?}",
                result.findings
            );
            assert_eq!(result.findings[0].severity, AnalyzeSeverity::HardFail);
            assert_eq!(result.findings[0].field, "section", "{frontmatter:?}");
            assert!(!result.clean);
        }
    }

    /// A pre-017 scenario carries `spec-ref` instead, and is well formed.
    #[test]
    fn a_legacy_spec_ref_scenario_is_clean() {
        let result = run_in_tree(
            &[(SCENARIO, "---\nspec-ref: \"Behavior\"\n---\n\n# A\n")],
            SCENARIO,
        )
        .unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    #[test]
    fn a_scenario_carrying_both_keys_is_clean() {
        let result = run_in_tree(
            &[(
                SCENARIO,
                "---\nsection: \"B\"\nspec-ref: \"B\"\n---\n\n# A\n",
            )],
            SCENARIO,
        )
        .unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    /// Stray spec-row keys on a scenario are unknown fields, not a spec-row
    /// check: an out-of-set `status` there draws no finding.
    #[test]
    fn stray_spec_keys_on_a_scenario_draw_no_finding() {
        let result = run_in_tree(
            &[(
                SCENARIO,
                "---\nsection: \"B\"\nstatus: nonsense\ndependencies: 5\n---\n\n# A\n",
            )],
            SCENARIO,
        )
        .unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    /// A file under `scenarios/` is a scenario whatever its name, so the
    /// order of the kind test decides `scenarios/spec.md`.
    #[test]
    fn a_file_under_scenarios_takes_the_scenario_row_whatever_its_name() {
        for name in ["spec.md", "review.md"] {
            let path = format!("specs/001-x/scenarios/{name}");
            let result = run_in_tree(
                &[(path.as_str(), "---\nsection: \"B\"\n---\n\n# A\n")],
                &path,
            )
            .unwrap();
            assert!(result.clean, "{name}: {:?}", result.findings);
        }
    }

    /// A `spec.md` run covers the scenarios beside it, keyed by path, and
    /// still reports the spec's own finding.
    #[test]
    fn a_spec_run_reports_a_malformed_scenario_by_its_path() {
        let result = run_in_tree(
            &[
                (SPEC, "---\nstatus: draft\n---\n\n# X\n"),
                (SCENARIO, "---\nowner: x\n---\n\n# A\n"),
                (
                    "specs/001-x/scenarios/b-fine.md",
                    "---\nsection: \"B\"\n---\n",
                ),
            ],
            SPEC,
        )
        .unwrap();
        let fields: Vec<&str> = result.findings.iter().map(|f| f.field.as_str()).collect();
        assert_eq!(
            fields,
            ["dependencies", "scenarios/a-case.md"],
            "{:?}",
            result.findings
        );
        let scenario = &result.findings[1];
        assert_eq!(scenario.severity, AnalyzeSeverity::HardFail);
        assert!(
            scenario.message.contains("section is missing"),
            "{}",
            scenario.message
        );
    }

    /// A scenario a `spec.md` run cannot parse is a hard fail keyed by its
    /// path, never an error that would hide the rest of the run.
    #[test]
    fn an_unparseable_scenario_in_a_spec_run_is_a_hard_fail_not_an_error() {
        let result = run_in_tree(
            &[
                (SPEC, CLEAN_SPEC),
                (
                    "specs/001-x/scenarios/a-no-fence.md",
                    "# A\n\nno frontmatter\n",
                ),
                (
                    "specs/001-x/scenarios/b-unclosed.md",
                    "---\nsection: B\n\n# B\n",
                ),
                (
                    "specs/001-x/scenarios/c-bad-yaml.md",
                    "---\nsection: [unclosed\n---\n",
                ),
            ],
            SPEC,
        )
        .unwrap();
        let fields: Vec<&str> = result.findings.iter().map(|f| f.field.as_str()).collect();
        assert_eq!(
            fields,
            [
                "scenarios/a-no-fence.md",
                "scenarios/b-unclosed.md",
                "scenarios/c-bad-yaml.md"
            ],
            "{:?}",
            result.findings
        );
        assert!(
            result
                .findings
                .iter()
                .all(|f| f.severity == AnalyzeSeverity::HardFail)
        );
        assert!(result.findings[0].message.contains("no frontmatter"));
        assert!(result.findings[1].message.contains("never closes"));
        assert!(result.findings[2].message.contains("not valid YAML"));
    }

    /// A scenario a `spec.md` run cannot open is a hard fail keyed by its
    /// path, as an unreadable record is, never a silent skip.
    #[cfg(unix)]
    #[test]
    fn a_scenario_a_spec_run_cannot_read_is_a_hard_fail() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let scenario = tmp.path().join(SCENARIO);
        std::fs::create_dir_all(scenario.parent().unwrap()).unwrap();
        std::fs::write(tmp.path().join(SPEC), CLEAN_SPEC).unwrap();
        std::fs::write(&scenario, "---\nsection: \"B\"\n---\n").unwrap();
        std::fs::set_permissions(&scenario, std::fs::Permissions::from_mode(0o000)).unwrap();
        if std::fs::File::open(&scenario).is_ok() {
            return; // running as root: permissions do not bind
        }
        let result = run(&ValidateFrontmatterArgs { path: SPEC.into() }, tmp.path());
        std::fs::set_permissions(&scenario, std::fs::Permissions::from_mode(0o644)).unwrap();
        let result = result.unwrap();
        let fields: Vec<&str> = result.findings.iter().map(|f| f.field.as_str()).collect();
        assert_eq!(fields, ["scenarios/a-case.md"], "{:?}", result.findings);
        assert_eq!(result.findings[0].severity, AnalyzeSeverity::HardFail);
        assert!(
            result.findings[0].message.contains("could not be read"),
            "{}",
            result.findings[0].message
        );
        assert!(
            !result.findings[0]
                .message
                .contains(tmp.path().to_str().unwrap()),
            "the message names no machine-local path: {}",
            result.findings[0].message
        );
    }

    /// A spec whose own frontmatter will not parse still has its records and
    /// scenarios checked, so one pass surfaces every issue.
    #[test]
    fn a_spec_that_will_not_parse_still_has_its_scenarios_checked() {
        let result = run_in_tree(
            &[
                (SPEC, "---\nstatus: [unclosed\n---\n\n# X\n"),
                (SCENARIO, "---\nowner: x\n---\n"),
                ("specs/001-x/review.md", "# Review\n\nno record here\n"),
            ],
            SPEC,
        )
        .unwrap();
        let fields: Vec<&str> = result.findings.iter().map(|f| f.field.as_str()).collect();
        assert_eq!(
            fields,
            ["", "review.md", "scenarios/a-case.md"],
            "{:?}",
            result.findings
        );
    }

    /// No `scenarios/` directory, or an empty one, adds nothing.
    #[test]
    fn a_spec_run_without_scenarios_adds_nothing() {
        let result = run_in_tree(&[(SPEC, CLEAN_SPEC)], SPEC).unwrap();
        assert!(result.clean, "{:?}", result.findings);
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("specs/001-x/scenarios")).unwrap();
        std::fs::write(tmp.path().join(SPEC), CLEAN_SPEC).unwrap();
        let result = run(&ValidateFrontmatterArgs { path: SPEC.into() }, tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    /// A direct record path takes that record's checks: one that will not
    /// load is a hard fail, and so is a `decisions:` list that will not parse.
    #[test]
    fn a_direct_record_path_takes_its_record_checks() {
        let unparseable = run_in_tree(
            &[("specs/001-x/review.md", "# Review\n\nno record here\n")],
            "specs/001-x/review.md",
        )
        .unwrap();
        assert_eq!(unparseable.findings.len(), 1, "{:?}", unparseable.findings);
        assert_eq!(unparseable.findings[0].field, "review.md");
        assert_eq!(unparseable.findings[0].severity, AnalyzeSeverity::HardFail);
        let decisions = run_in_tree(
            &[(
                "specs/001-x/analysis.md",
                "---\nlast-run: 2026-09-25T00:00:00Z\ndecisions: not-a-list\n---\n",
            )],
            "specs/001-x/analysis.md",
        )
        .unwrap();
        let fields: Vec<&str> = decisions
            .findings
            .iter()
            .map(|f| f.field.as_str())
            .collect();
        assert_eq!(
            fields,
            ["analysis.md decisions"],
            "{:?}",
            decisions.findings
        );
    }

    /// An absent record named directly is refused like any missing file, not
    /// read as the never-run state a `spec.md` run reads its absence as.
    #[test]
    fn an_absent_direct_record_path_is_an_error() {
        let error = run_in_tree(&[(SPEC, CLEAN_SPEC)], "specs/001-x/review.md").unwrap_err();
        assert!(matches!(error, PrimitiveError::Io { .. }), "{error}");
    }

    /// A path with no row in the schema is refused, never checked against a
    /// row that does not describe it.
    #[test]
    fn a_path_of_no_schema_row_is_refused() {
        for path in [
            "specs/001-x/plan.md",
            "specs/001-x/tasks.md",
            "specs/inbox.md",
        ] {
            let error = run_in_tree(&[(path, "---\nstatus: draft\n---\n")], path).unwrap_err();
            assert!(
                matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "path"),
                "{path}: {error}"
            );
            assert!(error.to_string().contains("spec.md"), "{error}");
        }
    }

    /// Run the validator over a frontmatter block, returning the findings.
    fn findings_for(frontmatter: &str) -> Vec<FrontmatterFinding> {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, format!("---\n{frontmatter}---\n\n# X\n")).unwrap();
        run(
            &ValidateFrontmatterArgs {
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
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
            assert_eq!(findings[0].severity, AnalyzeSeverity::Blocking, "{bad:?}");
        }
        // Wrong type, not just wrong shape.
        let findings = findings_for("status: draft\ndependencies: []\nfolds-into: [022-a]\n");
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert_eq!(findings[0].field, "folds-into");
        assert_eq!(findings[0].severity, AnalyzeSeverity::Blocking);
        assert_eq!(
            findings[0].message,
            "folds-into must be a string feature name"
        );
    }

    #[test]
    fn an_absent_fold_target_is_never_a_finding() {
        // A sequential spec has none by definition, and removing the key
        // by hand is the supported way to make a staged spec stand alone.
        let findings = findings_for("status: draft\ndependencies: []\n");
        assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn unknown_status_is_hard_fail() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\nstatus: wibble\ndependencies: []\n---\n\n# X\n").unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.clean);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].field, "status");
        assert_eq!(result.findings[0].severity, AnalyzeSeverity::HardFail);
        assert!(
            result.findings[0].message.starts_with("status 'wibble'"),
            "{}",
            result.findings[0].message
        );
    }

    #[test]
    fn missing_status_is_hard_fail() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.md");
        std::fs::write(&path, "---\ndependencies: []\n---\n\n# X\n").unwrap();
        let result = run(
            &ValidateFrontmatterArgs {
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
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
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
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
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
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
                path: path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.clean);
        assert_eq!(result.findings.len(), 1, "{:?}", result.findings);
        assert_eq!(result.findings[0].field, "dependencies");
        assert_eq!(result.findings[0].severity, AnalyzeSeverity::HardFail);
        assert_eq!(result.findings[0].message, "dependencies must be a list");
    }

    /// A block that is not YAML stops the check at the parse: one hard-fail
    /// naming no field, since no field could be read.
    #[test]
    fn frontmatter_that_is_not_valid_yaml_is_hard_fail() {
        let findings = findings_for("status: [draft\ndependencies: []\n");
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].severity, AnalyzeSeverity::HardFail);
        assert_eq!(findings[0].field, "");
        assert!(
            findings[0]
                .message
                .starts_with("frontmatter is not valid YAML: "),
            "{}",
            findings[0].message
        );
    }

    /// YAML that parses to something other than a mapping (here a sequence)
    /// has no fields to check, so it is one hard-fail rather than a finding
    /// per missing field.
    #[test]
    fn frontmatter_that_is_not_a_mapping_is_hard_fail() {
        let findings = findings_for("- status\n- dependencies\n");
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].severity, AnalyzeSeverity::HardFail);
        assert_eq!(findings[0].field, "");
        assert_eq!(findings[0].message, "frontmatter must be a mapping");
    }

    #[test]
    fn a_non_string_status_is_hard_fail() {
        let findings = findings_for("status: 3\ndependencies: []\n");
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].severity, AnalyzeSeverity::HardFail);
        assert_eq!(findings[0].field, "status");
        assert_eq!(findings[0].message, "status must be a string");
    }

    /// Each non-string entry is named by its index, so a list holding a good
    /// entry and a bad one reports the bad one alone.
    #[test]
    fn a_non_string_dependency_entry_is_hard_fail() {
        let findings = findings_for("status: draft\ndependencies: [001-a, 3]\n");
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].severity, AnalyzeSeverity::HardFail);
        assert_eq!(findings[0].field, "dependencies[1]");
        assert_eq!(
            findings[0].message,
            "dependency entry must be a string feature name"
        );
    }
}
