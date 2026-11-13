//! `check-departures` — a spec's decided rule departures and the findings on
//! them (spec 073, AC1).
//!
//! The entry grammar and each entry's attribution are
//! [`crate::primitives::departures`]'s; the rule's state (unknown, retired,
//! deprecated, not installed) is `check-rule-ids`' reading of the rule files
//! `discover-rule-files` loads under `all`, as `/{project}:analyze` loads them;
//! a `cited:` is judged by `resolve-anchor` against the constitution's
//! markers. This module adds only the tiers. With no rule file loaded
//! nothing is known of a rule, so a token that is, whole, shaped like a rule
//! ID (`check_rule_ids::is_a_rule_id`) is none of unknown, retired or
//! deprecated: its entry's rule is unexamined as `rule-citations-not-checked`.
//! A token shaped like no rule ID is unknown whether or not a file is loaded.

use std::collections::HashMap;
use std::path::Path;

use crate::primitives::departures::{self, Attribution, Entry};
use crate::primitives::discover_rule_files;
use crate::primitives::rule_sections::parse_rule_sections;
use crate::primitives::{PrimitiveError, Result, check_rule_ids, read_text, resolve_anchor};
use crate::schema::paths::{self, Paths};
use crate::schema::primitives::{
    CheckDeparturesArgs, CheckDeparturesResult, CheckRuleIdsArgs, DepartureEntry, DepartureFinding,
    DepartureUnexamined, DiscoverRuleFilesArgs, FindingTier, ListsForm, ResolveAnchorArgs,
    RuleCitation,
};
use crate::schema::severity::{AnalyzeSeverity, RuleSeverity};

/// The family every finding carries.
const FAMILY: &str = "departure-entries";

/// Execute the `check-departures` primitive.
///
/// # Errors
///
/// [`PrimitiveError::FeatureNotFound`] when the feature directory is absent;
/// the feature-argument refusals of `validate_feature_argument`;
/// [`PrimitiveError::Io`] when `spec.md` or a rule file cannot be read; and
/// the errors of `discover-rule-files`, `check-rule-ids` and `resolve-anchor`
/// (a malformed config or `[constitutions.*]` registry), which halt the run
/// rather than read as an empty answer; and those `bounded_lists::place`
/// returns for a malformed read size or a list file that cannot be written.
pub fn run(args: &CheckDeparturesArgs, repo: &Path) -> Result<CheckDeparturesResult> {
    let (entries, findings, unexamined) = compute(args, repo)?;
    bounded(repo, &args.feature, entries, findings, unexamined)
}

/// Bound the result to one read (BE-PAYLOAD-001, as `read-spec` bounds its
/// lists): inline while it fits `[artifacts] read-size-bytes`, and past it
/// with `entries`, `findings` and `unexamined` in the file `lists-file`
/// names. The counts and `form` stay inline in both forms.
fn bounded(
    repo: &Path,
    feature: &str,
    entries: Vec<DepartureEntry>,
    findings: Vec<DepartureFinding>,
    unexamined: Vec<DepartureUnexamined>,
) -> Result<CheckDeparturesResult> {
    let lists = serde_json::json!({
        "entries": entries,
        "findings": findings,
        "unexamined": unexamined,
    });
    let mut result = CheckDeparturesResult {
        form: ListsForm::Inline,
        entries_count: entries.len(),
        entries: Some(entries),
        findings_count: findings.len(),
        findings: Some(findings),
        unexamined_count: unexamined.len(),
        unexamined: Some(unexamined),
        lists_file: None,
    };
    if let Some(file) =
        super::bounded_lists::place(repo, "check-departures", feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.entries = None;
        result.findings = None;
        result.unexamined = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// The lists, whole, before they are bounded.
type Lists = (
    Vec<DepartureEntry>,
    Vec<DepartureFinding>,
    Vec<DepartureUnexamined>,
);

fn compute(args: &CheckDeparturesArgs, repo: &Path) -> Result<Lists> {
    let root = Paths::load(repo)?.specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;
    if !repo.join(&root).join(&args.feature).is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    let spec_rel = format!("{root}/{}/spec.md", args.feature);
    let content = read_text(&repo.join(&spec_rel))?;
    let parsed = departures::parse(&content);

    let mut found: Vec<(usize, DepartureFinding)> = Vec::new();
    let mut unexamined: Vec<DepartureUnexamined> = Vec::new();
    let finding = |tier: AnalyzeSeverity, line: usize, message: String| {
        (
            line,
            DepartureFinding {
                family: FAMILY.into(),
                tier,
                message: format!("line {line}: {message}"),
                path: spec_rel.clone(),
            },
        )
    };

    for problem in &parsed.problems {
        found.push(finding(
            AnalyzeSeverity::Blocking,
            problem.line,
            problem.kind.message(),
        ));
    }
    if parsed.entries.is_empty() {
        return Ok(finish(Vec::new(), found, unexamined));
    }

    let rules = RuleFacts::load(repo, &spec_rel)?;
    let names: Vec<String> = parsed.entries.iter().map(|e| e.rule.clone()).collect();
    let attribution = departures::attribute(repo, &spec_rel, &names);
    let mut entries = Vec::new();
    for entry in &parsed.entries {
        judge_rule(
            &rules.state(&entry.rule),
            entry,
            &mut found,
            &mut unexamined,
            &finding,
        );
        let (decided_by, decided_at) = attribution
            .get(&entry.rule)
            .map_or_else(|| Attribution::Unattributed.decided(), Attribution::decided);
        entries.push(DepartureEntry {
            rule: entry.rule.clone(),
            tier: rules.tier(&entry.rule),
            reason: entry.reason.clone(),
            cited: entry.cited.clone(),
            decided_by,
            decided_at,
            line: to_line(entry.line),
        });
    }

    judge_citations(
        repo,
        &spec_rel,
        &parsed.entries,
        &mut found,
        &mut unexamined,
        &finding,
    )?;
    Ok(finish(entries, found, unexamined))
}

/// Raise the finding, or record the unexamined target, a rule's state calls
/// for.
fn judge_rule(
    state: &State,
    entry: &Entry,
    found: &mut Vec<(usize, DepartureFinding)>,
    unexamined: &mut Vec<DepartureUnexamined>,
    finding: &impl Fn(AnalyzeSeverity, usize, String) -> (usize, DepartureFinding),
) {
    match state {
        State::Unknown => found.push(finding(
            AnalyzeSeverity::Blocking,
            entry.line,
            format!(
                "the entry departs from `{}`, which no loaded rule file defines",
                entry.rule
            ),
        )),
        State::Retired => found.push(finding(
            AnalyzeSeverity::Blocking,
            entry.line,
            format!(
                "the entry departs from `{}`, a rule this spec's `retired-rules:` lists as removed",
                entry.rule
            ),
        )),
        State::Deprecated => found.push(finding(
            AnalyzeSeverity::Advisory,
            entry.line,
            format!(
                "the entry departs from `{}`, a deprecated rule: re-point or remove it",
                entry.rule
            ),
        )),
        State::NotInstalled => unexamined.push(DepartureUnexamined {
            reason: check_rule_ids::NOT_INSTALLED.into(),
            line: to_line(entry.line),
        }),
        State::NotChecked => unexamined.push(DepartureUnexamined {
            reason: check_rule_ids::CITATIONS_NOT_CHECKED.into(),
            line: to_line(entry.line),
        }),
        State::Defined => {}
    }
}

fn finish(
    entries: Vec<DepartureEntry>,
    mut found: Vec<(usize, DepartureFinding)>,
    mut unexamined: Vec<DepartureUnexamined>,
) -> Lists {
    found.sort_by_key(|(line, _)| *line);
    unexamined.sort_by_key(|item| item.line);
    (
        entries,
        found.into_iter().map(|(_, finding)| finding).collect(),
        unexamined,
    )
}

fn to_line(line: usize) -> u32 {
    u32::try_from(line).unwrap_or(u32::MAX)
}

/// Judge each `cited:` through `resolve-anchor` against the constitution's
/// markers: it must hold exactly one `§` reference. A reference it cannot
/// judge is recorded unexamined, never passed. An unresolved one is no
/// finding here: step 4 reports it over the whole spec, once.
fn judge_citations(
    repo: &Path,
    spec_rel: &str,
    entries: &[Entry],
    found: &mut Vec<(usize, DepartureFinding)>,
    unexamined: &mut Vec<DepartureUnexamined>,
    finding: &impl Fn(AnalyzeSeverity, usize, String) -> (usize, DepartureFinding),
) -> Result<()> {
    if entries.iter().all(|entry| entry.cited_lines.is_none()) {
        return Ok(());
    }
    let anchors = resolve_anchor::run(
        &ResolveAnchorArgs {
            path: spec_rel.to_owned(),
            markers_path: paths::constitution_path(repo).map(str::to_owned),
        },
        repo,
    )?;
    for entry in entries {
        let Some((first, last)) = entry.cited_lines else {
            continue;
        };
        let here: Vec<_> = anchors
            .references
            .iter()
            .filter(|reference| (first..=last).contains(&(reference.line as usize)))
            .collect();
        if here.is_empty() {
            found.push(finding(
                AnalyzeSeverity::Advisory,
                first,
                "`cited:` names no `§anchor` reference to resolve".into(),
            ));
        } else if here.len() > 1 {
            found.push(finding(
                AnalyzeSeverity::Advisory,
                first,
                format!(
                    "`cited:` names {} `§anchor` references; it takes exactly one",
                    here.len()
                ),
            ));
        }
        for reference in here {
            if let Some(reason) = &reference.unjudged {
                unexamined.push(DepartureUnexamined {
                    reason: reason.clone(),
                    line: reference.line,
                });
            }
        }
    }
    Ok(())
}

/// What a rule ID is, as `check-rule-ids` reads it against the loaded files.
enum State {
    /// No rule file was loaded, so nothing was checked against: the rule's
    /// state is unknown, never a finding (`check-rule-ids`' `examined: 0`).
    /// Only a token `check-rule-ids` reads as a citation: any other cannot be
    /// a rule ID, so no rule file is needed to judge it unknown.
    NotChecked,
    Defined,
    Deprecated,
    Retired,
    NotInstalled,
    Unknown,
}

/// The loaded rule files' reading of each ID a spec's entries name.
struct RuleFacts {
    /// How many rule files were read; `0` means every state below is blind.
    examined: usize,
    citations: HashMap<String, RuleCitation>,
    tiers: HashMap<String, FindingTier>,
}

impl RuleFacts {
    fn load(repo: &Path, spec_rel: &str) -> Result<Self> {
        let discovered = discover_rule_files::run(
            &DiscoverRuleFilesArgs {
                all: true,
                ..DiscoverRuleFilesArgs::default()
            },
            repo,
        )?;
        let files: Vec<String> = discovered
            .selected
            .iter()
            .map(|name| format!("{}/{name}", discovered.rules_dir))
            .collect();
        let mut tiers = HashMap::new();
        for file in &files {
            for section in parse_rule_sections(&read_text(&repo.join(file))?) {
                let tier = match section.tier() {
                    Some(RuleSeverity::Must) => Some(FindingTier::Must),
                    Some(RuleSeverity::Should) => Some(FindingTier::Should),
                    _ => None,
                };
                if let Some(tier) = tier {
                    tiers.entry(section.id).or_insert(tier);
                }
            }
        }
        let checked = check_rule_ids::run(
            &CheckRuleIdsArgs {
                path: spec_rel.to_owned(),
                rule_files: files,
            },
            repo,
        )?;
        Ok(Self {
            examined: checked.examined,
            citations: checked
                .citations
                .into_iter()
                .map(|citation| (citation.rule_id.clone(), citation))
                .collect(),
            tiers,
        })
    }

    fn state(&self, rule: &str) -> State {
        if self.examined == 0 && check_rule_ids::is_a_rule_id(rule) {
            return State::NotChecked;
        }
        match self.citations.get(rule) {
            Some(c) if c.retired => State::Retired,
            Some(c) if c.not_installed => State::NotInstalled,
            Some(c) if c.deprecated => State::Deprecated,
            Some(c) if c.found => State::Defined,
            _ => State::Unknown,
        }
    }

    fn tier(&self, rule: &str) -> Option<FindingTier> {
        self.tiers.get(rule).copied()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::write;

    const FEATURE: &str = "001-x";
    const SPEC: &str = "specs/001-x/spec.md";

    const RULES: &str = "# Demo rules\n\n\
        ### DM-MUST-001\n\n> The service MUST do the thing.\n\n\
        ### DM-SHLD-001\n\n> The service SHOULD do the thing.\n\n\
        ### DM-OLD-001\n\n> The service MUST do the old thing.\n\n\
        **DEPRECATED in 0.5.0:** replaced by DM-MUST-001.\n";

    const CONSTITUTION: &str =
        "# Constitution\n\n<!-- §shared-exceptions -->\n## Shared exceptions\n\ntext\n";

    fn entry(rule: &str) -> String {
        format!("- departs: `{rule}`\n  - reason: because\n")
    }

    /// A project with the demo rule file, a constitution, and a spec holding
    /// `body` under `## Applicable Rules`.
    fn project(frontmatter_extra: &str, body: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        write(&tmp.path().join("specs/rules/demo-cross.md"), RULES);
        write(&tmp.path().join(".ductus/constitution.md"), CONSTITUTION);
        write(
            &tmp.path().join(SPEC),
            &format!(
                "---\nstatus: draft\ndependencies: []\n{frontmatter_extra}---\n\n# Spec\n\n## Applicable Rules\n\n{body}\n## Next\n\ntext\n"
            ),
        );
        tmp
    }

    /// A result's three lists, whole.
    struct Whole {
        entries: Vec<DepartureEntry>,
        findings: Vec<DepartureFinding>,
        unexamined: Vec<DepartureUnexamined>,
    }

    /// The result of `run` in the inline form, which every fixture here fits.
    fn check(tmp: &tempfile::TempDir) -> Whole {
        let result = run(
            &CheckDeparturesArgs {
                feature: FEATURE.into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.form, ListsForm::Inline);
        Whole {
            entries: result.entries.unwrap(),
            findings: result.findings.unwrap(),
            unexamined: result.unexamined.unwrap(),
        }
    }

    fn tiers(result: &Whole) -> Vec<(AnalyzeSeverity, &str)> {
        result
            .findings
            .iter()
            .map(|f| (f.tier, f.message.as_str()))
            .collect()
    }

    #[test]
    fn a_clean_entry_is_listed_with_its_tier_and_no_finding() {
        let tmp = project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: the field is removed\n  - cited: §shared-exceptions\n- departs: `DM-SHLD-001`\n  - reason: not worth it\n",
        );
        let result = check(&tmp);
        assert_eq!(result.findings, vec![]);
        assert_eq!(result.unexamined, vec![]);
        assert_eq!(result.entries.len(), 2);
        assert_eq!(result.entries[0].rule, "DM-MUST-001");
        assert_eq!(result.entries[0].tier, Some(FindingTier::Must));
        assert_eq!(
            result.entries[0].cited.as_deref(),
            Some("§shared-exceptions")
        );
        assert_eq!(result.entries[1].tier, Some(FindingTier::Should));
        assert_eq!(result.entries[0].decided_by, "unattributed");
        assert_eq!(result.entries[0].decided_at, None);
        assert_eq!(result.entries[0].line, 10);
    }

    #[test]
    fn a_missing_or_empty_reason_is_blocking_and_the_entry_is_not_listed() {
        let tmp = project(
            "",
            "- departs: `DM-MUST-001`\n  - cited: §shared-exceptions\n- departs: `DM-SHLD-001`\n  - reason:\n",
        );
        let result = check(&tmp);
        assert_eq!(result.entries, vec![]);
        assert_eq!(result.findings.len(), 2, "{:?}", result.findings);
        assert!(
            result
                .findings
                .iter()
                .all(|f| f.tier == AnalyzeSeverity::Blocking)
        );
        assert!(result.findings[0].message.contains("no `reason:`"));
        assert!(result.findings[1].message.contains("`reason:` is empty"));
        // The clean counterpart is the first test: a reason yields no finding.
    }

    #[test]
    fn the_other_structural_problems_are_blocking() {
        let tmp = project(
            "",
            "- departs: `DM-MUST-001` `DM-SHLD-001`\n  - reason: r\n- departs: `DM-OLD-001`\n  - reason: r\n  - why: r\n- departs: nothing here\n  - reason: r\n",
        );
        let result = check(&tmp);
        assert_eq!(result.entries, vec![]);
        assert_eq!(result.findings.len(), 3, "{:?}", result.findings);
        assert!(
            result
                .findings
                .iter()
                .all(|f| f.tier == AnalyzeSeverity::Blocking)
        );
    }

    #[test]
    fn an_unknown_rule_is_blocking_and_a_known_one_is_not() {
        let tmp = project("", &entry("DM-NONE-999"));
        let result = check(&tmp);
        assert_eq!(
            tiers(&result).iter().map(|(t, _)| *t).collect::<Vec<_>>(),
            vec![AnalyzeSeverity::Blocking]
        );
        assert!(
            result.findings[0]
                .message
                .contains("no loaded rule file defines")
        );
        assert_eq!(result.entries[0].tier, None);
        assert_eq!(check(&project("", &entry("DM-MUST-001"))).findings, vec![]);
    }

    #[test]
    fn a_retired_rule_is_blocking_only_when_the_spec_lists_it_retired() {
        let tmp = project("retired-rules:\n  - DM-GONE-001\n", &entry("DM-GONE-001"));
        let result = check(&tmp);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].tier, AnalyzeSeverity::Blocking);
        assert!(result.findings[0].message.contains("retired-rules"));
        // Unlisted, the same ID is merely unknown: a different message.
        let unlisted = check(&project("", &entry("DM-GONE-001")));
        assert!(!unlisted.findings[0].message.contains("retired-rules"));
    }

    #[test]
    fn a_rule_two_entries_name_is_blocking_on_the_later_entry() {
        let tmp = project(
            "",
            &format!("{}{}", entry("DM-MUST-001"), entry("DM-MUST-001")),
        );
        let result = check(&tmp);
        assert_eq!(result.findings.len(), 1, "{:?}", result.findings);
        assert_eq!(result.findings[0].tier, AnalyzeSeverity::Blocking);
        assert!(result.findings[0].message.contains("line 12"));
        // Both entries stay listed, each with its own attribution fields.
        assert_eq!(result.entries.len(), 2);
        assert_eq!(
            result.entries.iter().map(|e| e.line).collect::<Vec<_>>(),
            vec![10, 12]
        );
        assert!(result.entries.iter().all(|e| e.rule == "DM-MUST-001"
            && e.decided_by == "unattributed"
            && e.decided_at.is_none()));
        let two = check(&project(
            "",
            &format!("{}{}", entry("DM-MUST-001"), entry("DM-SHLD-001")),
        ));
        assert_eq!(two.findings, vec![]);
    }

    #[test]
    fn a_deprecated_rule_is_advisory() {
        let result = check(&project("", &entry("DM-OLD-001")));
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].tier, AnalyzeSeverity::Advisory);
        assert!(result.findings[0].message.contains("deprecated"));
        assert_eq!(result.entries[0].tier, Some(FindingTier::Must));
    }

    #[test]
    fn a_citation_without_exactly_one_reference_is_advisory_and_an_unresolved_one_is_step_4s() {
        let bad = check(&project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: §no-such-anchor\n",
        ));
        // `resolve-anchor` (step 4) reports the unresolved reference over the
        // whole spec, so it is counted there and not again here.
        assert_eq!(bad.findings, vec![], "{:?}", bad.findings);
        assert_eq!(bad.unexamined, vec![]);
        let none = check(&project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: the shared doc\n",
        ));
        assert_eq!(none.findings.len(), 1);
        assert!(none.findings[0].message.contains("names no `§anchor`"));
        let good = check(&project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: §shared-exceptions\n",
        ));
        assert_eq!(good.findings, vec![]);
    }

    #[test]
    fn a_citation_of_two_references_is_advisory_though_each_resolves() {
        let two = check(&project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: §shared-exceptions and §shared-exceptions\n",
        ));
        assert_eq!(two.findings.len(), 1, "{:?}", two.findings);
        assert_eq!(two.findings[0].tier, AnalyzeSeverity::Advisory);
        assert!(two.findings[0].message.contains("exactly one"));
        // Clean counterpart: one resolving reference yields none.
        let one = check(&project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: §shared-exceptions\n",
        ));
        assert_eq!(one.findings, vec![]);
    }

    #[test]
    fn a_reference_on_a_wrapped_citations_continuation_line_is_judged() {
        let good = check(&project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: see the\n    §shared-exceptions\n",
        ));
        assert_eq!(good.findings, vec![], "{:?}", good.findings);
        assert_eq!(good.unexamined, vec![]);
        let bad = check(&project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: see the\n    §no-such-anchor\n",
        ));
        assert_eq!(bad.findings, vec![], "{:?}", bad.findings);
    }

    #[test]
    fn only_the_cited_lines_are_judged_for_references() {
        // A stray `§` in the reason, and one on the line after the entry,
        // are no citation: neither is judged.
        let tmp = project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: see §stray-one\n  - cited: §shared-exceptions\n\nnote §stray-two\n",
        );
        let result = check(&tmp);
        assert_eq!(result.findings, vec![]);
        assert_eq!(result.unexamined, vec![]);
    }

    #[test]
    fn a_rule_file_only_the_all_selection_loads_defines_a_known_rule() {
        let tmp = project("", &entry("FE-ONLY-001"));
        write(
            &tmp.path().join("specs/rules/demo-frontend.md"),
            "# Frontend\n\n### FE-ONLY-001\n\n> The page MUST render.\n",
        );
        write(
            &tmp.path().join(".ductus/config.toml"),
            "[rules]\nsurfaces = [\"backend\"]\n",
        );
        let result = check(&tmp);
        assert_eq!(result.findings, vec![]);
        assert_eq!(result.entries[0].tier, Some(FindingTier::Must));
    }

    #[test]
    fn findings_are_in_line_order_whatever_found_them() {
        // An unknown rule is found after the structural problems, yet its
        // line is the earlier one.
        let tmp = project(
            "",
            "- departs: `DM-NONE-999`\n  - reason: r\n- departs: `DM-MUST-001`\n  - cited: §shared-exceptions\n",
        );
        let result = check(&tmp);
        assert_eq!(result.findings.len(), 2, "{:?}", result.findings);
        assert!(result.findings[0].message.starts_with("line 10:"));
        assert!(result.findings[1].message.starts_with("line 12:"));
    }

    /// `check-rule-ids` examines 0 files, so step 5 raises nothing and records
    /// the citations `rule-citations-not-checked`; an entry's rule is judged
    /// the same way, whatever its ID (unknown, listed retired, shipped), and a
    /// structural problem is still raised.
    #[test]
    fn with_no_rule_file_loaded_an_entrys_rule_is_unexamined_never_a_finding() {
        let tmp = project(
            "retired-rules:\n  - DM-GONE-001\n",
            "- departs: `DM-NONE-999`\n  - reason: r\n- departs: `DM-GONE-001`\n  - reason: r\n- departs: `BE-API-001`\n  - reason: r\n- departs: `DM-MUST-001`\n  - cited: §shared-exceptions\n",
        );
        std::fs::remove_file(tmp.path().join("specs/rules/demo-cross.md")).unwrap();
        let result = check(&tmp);
        // Only the entry with no `reason:` is a finding.
        assert_eq!(result.findings.len(), 1, "{:?}", result.findings);
        assert!(result.findings[0].message.contains("no `reason:`"));
        assert_eq!(
            result.unexamined,
            vec![
                DepartureUnexamined {
                    reason: "rule-citations-not-checked".into(),
                    line: 12,
                },
                DepartureUnexamined {
                    reason: "rule-citations-not-checked".into(),
                    line: 14,
                },
                DepartureUnexamined {
                    reason: "rule-citations-not-checked".into(),
                    line: 16,
                },
            ]
        );
        assert_eq!(result.entries.len(), 3);
        assert!(result.entries.iter().all(|e| e.tier.is_none()));
    }

    /// A token step 5 reads as no citation (it is not shaped like a rule ID)
    /// leaves no record there, so with no rule file loaded it is still a
    /// Blocking finding here, where a rule-shaped one is unexamined.
    #[test]
    fn with_no_rule_file_loaded_a_token_that_cannot_be_a_rule_id_is_still_unknown() {
        let tmp = project(
            "",
            "- departs: `not-a-rule`\n  - reason: r\n- departs: `DM-NONE-999`\n  - reason: r\n",
        );
        std::fs::remove_file(tmp.path().join("specs/rules/demo-cross.md")).unwrap();
        let result = check(&tmp);
        assert_eq!(result.findings.len(), 1, "{:?}", result.findings);
        assert_eq!(result.findings[0].tier, AnalyzeSeverity::Blocking);
        assert!(result.findings[0].message.starts_with("line 10:"));
        assert!(
            result.findings[0]
                .message
                .contains("no loaded rule file defines")
        );
        assert_eq!(
            result.unexamined,
            vec![DepartureUnexamined {
                reason: "rule-citations-not-checked".into(),
                line: 12,
            }]
        );
    }

    /// Text around a rule-shaped substring is no rule ID, so it is unknown with no rule
    /// file loaded, as it is with one, where it matches no rule.
    #[test]
    fn with_no_rule_file_loaded_text_around_a_rule_id_is_not_a_rule_id() {
        let tmp = project(
            "",
            "- departs: `see DM-MUST-001 here`\n  - reason: r\n- departs: `DM-MUST-001`\n  - reason: r\n",
        );
        std::fs::remove_file(tmp.path().join("specs/rules/demo-cross.md")).unwrap();
        let result = check(&tmp);
        assert_eq!(result.findings.len(), 1, "{:?}", result.findings);
        assert!(result.findings[0].message.starts_with("line 10:"));
        assert!(
            result.findings[0]
                .message
                .contains("no loaded rule file defines")
        );
        assert_eq!(
            result.unexamined,
            vec![DepartureUnexamined {
                reason: "rule-citations-not-checked".into(),
                line: 12,
            }]
        );
    }

    #[test]
    fn a_citation_that_cannot_be_judged_is_unexamined_never_a_pass_or_a_finding() {
        let tmp = project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: `docs/missing.md` §anything\n",
        );
        let result = check(&tmp);
        assert_eq!(result.findings, vec![]);
        assert_eq!(
            result.unexamined,
            vec![DepartureUnexamined {
                reason: "anchor-target-unjudgeable".into(),
                line: 12,
            }]
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_citation_into_an_unreadable_document_is_unexamined_as_unreadable() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = project(
            "",
            "- departs: `DM-MUST-001`\n  - reason: r\n  - cited: `docs/locked.md` §anything\n",
        );
        let locked = tmp.path().join("docs/locked.md");
        write(&locked, "# Locked\n\n## Anything\n");
        std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000)).unwrap();
        if std::fs::read_to_string(&locked).is_ok() {
            return; // running as a user the mode does not bind
        }
        let result = check(&tmp);
        assert_eq!(result.findings, vec![]);
        assert_eq!(result.unexamined.len(), 1);
        assert_eq!(result.unexamined[0].reason, "anchor-target-unreadable");
    }

    #[test]
    fn a_rule_the_project_never_installed_is_unexamined_and_no_finding() {
        // BE-API-001 is a shipped rule this project has no file for.
        let result = check(&project("", &entry("BE-API-001")));
        assert_eq!(result.findings, vec![]);
        assert_eq!(
            result.unexamined,
            vec![DepartureUnexamined {
                reason: "rule-not-installed".into(),
                line: 10,
            }]
        );
        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.entries[0].tier, None);
    }

    #[test]
    fn the_template_s_commented_example_and_the_legacy_bullet_yield_no_entry() {
        let template = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../framework/templates/spec/spec.md"),
        )
        .unwrap();
        let tmp = project("", "");
        let body = template
            .split("## Applicable Rules")
            .nth(1)
            .expect("the template has the heading");
        let section = body.split("\n## ").next().unwrap();
        write(
            &tmp.path().join(SPEC),
            &format!(
                "---\nstatus: draft\ndependencies: []\n---\n\n# Spec\n\n## Applicable Rules{section}\n- `DM-MUST-001` — a legacy citation\n\n## Next\n"
            ),
        );
        let result = check(&tmp);
        assert_eq!(result.entries, vec![]);
        assert_eq!(result.findings, vec![]);
        assert_eq!(result.unexamined, vec![]);
        // The same grammar outside a comment is an entry.
        write(
            &tmp.path().join(SPEC),
            "---\nstatus: draft\ndependencies: []\n---\n\n# Spec\n\n## Applicable Rules\n\n- departs: `DM-MUST-001`\n  - reason: r\n",
        );
        assert_eq!(check(&tmp).entries.len(), 1);
    }

    #[test]
    fn an_entry_carries_the_attribution_of_the_commit_that_introduced_it() {
        let tmp = project("", &entry("DM-MUST-001"));
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let mut index = repository.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let sig = git2::Signature::new(
            "Alice",
            "alice@example.com",
            &git2::Time::new(1_609_459_200, 0),
        )
        .unwrap();
        repository
            .commit(Some("HEAD"), &sig, &sig, "add", &tree, &[])
            .unwrap();
        let result = check(&tmp);
        assert_eq!(result.entries[0].decided_by, "alice@example.com");
        assert_eq!(
            result.entries[0].decided_at.as_deref(),
            Some("2021-01-01T00:00:00Z")
        );
    }

    fn read_size(tmp: &tempfile::TempDir, bytes: u64) {
        write(
            &tmp.path().join(".ductus/config.toml"),
            &format!("[artifacts]\nread-size-bytes = {bytes}\n"),
        );
    }

    /// BE-PAYLOAD-001: the result is inline while it fits the read size; past
    /// it the three lists move whole to the file `lists-file` names, with
    /// `form` and each count inline in both forms.
    #[test]
    fn a_result_past_the_read_size_names_a_file_holding_every_list() {
        // One entry with no `reason:` (a finding), and three entries whose
        // rule is unexamined: every list is non-empty.
        let tmp = project(
            "",
            "- departs: `DM-NONE-999`\n  - reason: r\n- departs: `BE-API-001`\n  - reason: r\n- departs: `DM-MUST-001`\n  - cited: §shared-exceptions\n",
        );
        std::fs::remove_file(tmp.path().join("specs/rules/demo-cross.md")).unwrap();
        let call = CheckDeparturesArgs {
            feature: FEATURE.into(),
        };
        read_size(&tmp, 1_000_000);
        let whole = compute(&call, tmp.path()).unwrap();
        assert_eq!(
            (whole.0.len(), whole.1.len(), whole.2.len()),
            (2, 1, 2),
            "{whole:?}"
        );
        let inline = run(&call, tmp.path()).unwrap();
        assert_eq!(inline.form, ListsForm::Inline);
        assert_eq!(inline.lists_file, None);
        assert_eq!(inline.entries.as_ref(), Some(&whole.0));
        assert_eq!(inline.findings.as_ref(), Some(&whole.1));
        assert_eq!(inline.unexamined.as_ref(), Some(&whole.2));

        read_size(&tmp, 10);
        let bounded = run(&call, tmp.path()).unwrap();
        assert_eq!(bounded.form, ListsForm::File);
        assert_eq!(
            (&bounded.entries, &bounded.findings, &bounded.unexamined),
            (&None, &None, &None)
        );
        assert_eq!(
            (
                bounded.entries_count,
                bounded.findings_count,
                bounded.unexamined_count
            ),
            (2, 1, 2)
        );
        let file = bounded.lists_file.clone().unwrap();
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(&file)).unwrap())
                .unwrap();
        assert_eq!(lists["entries"], serde_json::to_value(&whole.0).unwrap());
        assert_eq!(lists["findings"], serde_json::to_value(&whole.1).unwrap());
        assert_eq!(lists["unexamined"], serde_json::to_value(&whole.2).unwrap());
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "check-departures",
            &serde_json::to_value(&bounded).unwrap(),
            &lists,
        );
    }

    #[test]
    fn a_missing_feature_is_an_error() {
        let tmp = project("", "");
        assert!(matches!(
            run(
                &CheckDeparturesArgs {
                    feature: "999-none".into()
                },
                tmp.path()
            ),
            Err(PrimitiveError::FeatureNotFound { .. })
        ));
    }
}
