//! `check-rule-ids` — verify cited rule IDs exist in rule files and aren't
//! deprecated.

#![allow(clippy::expect_used)]

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

use crate::primitives::rule_sections::{heading_id_regex, section_label_lines};
use crate::primitives::{Result, contained_path, read_text, split_frontmatter};
use crate::schema::primitives::{CheckRuleIdsArgs, CheckRuleIdsResult, RuleCitation};

/// Execute the `check-rule-ids` primitive.
///
/// # Errors
///
/// Returns [`crate::primitives::PrimitiveError::Io`] when the target file
/// or any rule file cannot be read, and
/// [`crate::primitives::PrimitiveError::InvalidPath`], before any is read,
/// when one of them is absolute, climbs out with `..`, or leaves the project
/// through a symlink ([`crate::primitives::contained_path`]).
pub fn run(args: &CheckRuleIdsArgs, repo: &Path) -> Result<CheckRuleIdsResult> {
    // Every path is checked before any is read.
    let target = contained_path(repo, &args.path)?;
    let rule_paths = args
        .rule_files
        .iter()
        .map(|rule_file| contained_path(repo, rule_file))
        .collect::<Result<Vec<_>>>()?;
    let mut known: HashMap<String, bool> = HashMap::new();
    for path in &rule_paths {
        let content = read_text(path)?;
        for cap in heading_id_regex().captures_iter(&content) {
            let Some(whole) = cap.get(0) else { continue };
            let id = cap[1].to_string();
            let deprecated = section_is_deprecated(&content, whole.start());
            // A rule ID defined by more than one heading (unusual, but
            // representable) is deprecated when ANY defining section
            // carries the label.
            known
                .entry(id)
                .and_modify(|d| *d = *d || deprecated)
                .or_insert(deprecated);
        }
    }

    let content = read_text(&target)?;
    let (retired_rules, scanned) = retired_rules_and_body(&target, &content);
    let mut citations: Vec<RuleCitation> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut missing: HashSet<String> = HashSet::new();
    let mut deprecated_hits: HashSet<String> = HashSet::new();
    let mut retired_hits: HashSet<String> = HashSet::new();
    let mut not_installed_hits: HashSet<String> = HashSet::new();
    for cap in citation_regex().captures_iter(scanned) {
        let id = cap[0].to_string();
        if !seen.insert(id.clone()) {
            continue;
        }
        match known.get(&id).copied() {
            Some(true) => {
                deprecated_hits.insert(id.clone());
                citations.push(RuleCitation {
                    rule_id: id,
                    found: true,
                    deprecated: true,
                    retired: false,
                    not_installed: false,
                });
            }
            Some(false) => citations.push(RuleCitation {
                rule_id: id,
                found: true,
                deprecated: false,
                retired: false,
                not_installed: false,
            }),
            // No loaded file defines it, and the citing spec lists it as a
            // rule it records the retirement of: a record, not a citation
            // of a current requirement. Its own state, never found, never
            // missing.
            None if retired_rules.contains(&id) => {
                retired_hits.insert(id.clone());
                citations.push(RuleCitation {
                    rule_id: id,
                    found: false,
                    deprecated: false,
                    retired: true,
                    not_installed: false,
                });
            }
            // No file it was given defines it, but the framework ships it:
            // its file was never installed (a surface the project does not
            // enforce). Informational, known and not installed, never missing
            // (spec 033 scenario `an-out-of-surface-citation-resolves`).
            None if shipped_rule_ids().contains(id.as_str()) => {
                not_installed_hits.insert(id.clone());
                citations.push(RuleCitation {
                    rule_id: id,
                    found: false,
                    deprecated: false,
                    retired: false,
                    not_installed: true,
                });
            }
            None => {
                missing.insert(id.clone());
                citations.push(RuleCitation {
                    rule_id: id,
                    found: false,
                    deprecated: false,
                    retired: false,
                    not_installed: false,
                });
            }
        }
    }

    let mut missing: Vec<String> = missing.into_iter().collect();
    missing.sort();
    let mut deprecated: Vec<String> = deprecated_hits.into_iter().collect();
    deprecated.sort();
    let mut retired: Vec<String> = retired_hits.into_iter().collect();
    retired.sort();
    let mut not_installed: Vec<String> = not_installed_hits.into_iter().collect();
    not_installed.sort();
    Ok(CheckRuleIdsResult {
        citations,
        missing,
        deprecated,
        retired,
        not_installed,
        examined: args.rule_files.len(),
    })
}

/// The IDs of every rule the framework's rule files define at this runtime's
/// version, compiled in from `shipped_rule_ids.txt` (one ID per line) so a
/// citation of a rule whose file a project never installed is told from one
/// no rule defines. The index is generated from `framework/rules/*.md`, and
/// `shipped_rule_index_matches_the_framework_rules` fails when it drifts.
fn shipped_rule_ids() -> &'static HashSet<&'static str> {
    static IDS: OnceLock<HashSet<&'static str>> = OnceLock::new();
    IDS.get_or_init(|| {
        include_str!("shipped_rule_ids.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .collect()
    })
}

/// The rule IDs a spec's optional `retired-rules:` frontmatter list names —
/// the removed rules the spec records the retirement of (spec 067 scenario
/// `a-record-cites-a-removed-rule`) — and the text scanned for citations.
///
/// The scanned text is the body when the target has frontmatter, so the list
/// itself is no citation: a listed ID is checked only by the body citations it
/// exempts, and a mistyped one exempts nothing but its own. A target without
/// parseable frontmatter is scanned whole.
///
/// Only a `spec.md` carries the list, so a citation of a removed rule in any
/// other artifact stays missing. A `retired-rules` that is not a list, or an
/// entry that is not a string, yields nothing for that part: the citation
/// stays missing, the blocking side, and `validate-frontmatter` owns
/// diagnosing the file.
fn retired_rules_and_body<'a>(target: &Path, content: &'a str) -> (HashSet<String>, &'a str) {
    let Ok((fm_text, body)) = split_frontmatter(content, target) else {
        return (HashSet::new(), content);
    };
    if target.file_name().is_none_or(|name| name != "spec.md") {
        return (HashSet::new(), body);
    }
    let retired = serde_norway::from_str::<serde_norway::Value>(fm_text)
        .ok()
        .and_then(|value| {
            value
                .get("retired-rules")
                .and_then(serde_norway::Value::as_sequence)
                .map(|ids| {
                    ids.iter()
                        .filter_map(|id| id.as_str().map(str::to_string))
                        .collect()
                })
        })
        .unwrap_or_default();
    (retired, body)
}

/// Plain text citations: `BE-AUTHN-001` anywhere in a body (not the heading
/// line itself for the rule file's self-references). The category segment
/// mirrors [`heading_id_regex`] and allows digit-bearing categories.
fn citation_regex() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"\b[A-Z]{2,5}-[A-Z][A-Z0-9]+-\d{3,4}\b").expect("hard-coded regex compiles")
    })
}

/// Whether the rule whose heading starts at `heading_start` carries the
/// canonical deprecation label `**DEPRECATED in {version}:**` (per
/// `specs/008-security-rules/data-model.md` and constitution §rules) in its own
/// section: [`section_label_lines`], which `read-deprecation-labels` reads
/// too.
fn section_is_deprecated(content: &str, heading_start: usize) -> bool {
    !section_label_lines(content, heading_start).is_empty()
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
    fn fixture_spec_cites_known_rule() {
        let repo = fixture_repo();
        let result = run(
            &CheckRuleIdsArgs {
                path: "specs/001-basic/spec.md".into(),
                rule_files: vec!["framework/rules/security-backend.md".into()],
            },
            &repo,
        )
        .unwrap();

        let ids: Vec<&str> = result
            .citations
            .iter()
            .map(|c| c.rule_id.as_str())
            .collect();
        assert!(ids.contains(&"BE-AUTHN-001"));
        assert!(
            result.missing.is_empty(),
            "unexpected missing: {:?}",
            result.missing
        );
        assert_eq!(result.deprecated, [] as [String; 0]);
    }

    /// `missing` is only meaningful against a denominator. With no
    /// `--rule-file`, `known` is empty and **every** citation falls through to
    /// `missing` — and `/{project}:analyze` treats `missing` as blocking, so an
    /// unguarded invocation yields blocking findings against a correct spec.
    /// `examined` is what tells those two states apart: the same `missing`
    /// list over `examined: 0` means *nothing was checked against*, over
    /// `examined: 1` it means *the ID really is absent*.
    #[test]
    fn examined_distinguishes_a_real_miss_from_an_empty_rule_file_list() {
        let tmp = tempfile::tempdir().unwrap();
        let spec_path = tmp.path().join("spec.md");
        std::fs::write(&spec_path, "# Demo\n\nReferences BE-DEMO-001.\n").unwrap();
        let rule_path = tmp.path().join("rules.md");
        std::fs::write(&rule_path, "# Rules\n\n### BE-DEMO-001\n\n> Hashed.\n").unwrap();

        // Given nothing to check against: the citation reports missing, and
        // `examined: 0` is the field that says why.
        let blind = run(
            &CheckRuleIdsArgs {
                path: spec_path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                rule_files: vec![],
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(blind.missing, vec!["BE-DEMO-001".to_string()]);
        assert_eq!(blind.examined, 0, "no rule file was read");

        // Given the file that defines it: nothing missing, denominator 1.
        let seeing = run(
            &CheckRuleIdsArgs {
                path: spec_path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                rule_files: vec![
                    rule_path
                        .strip_prefix(tmp.path())
                        .unwrap()
                        .to_string_lossy()
                        .into(),
                ],
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(seeing.missing, [] as [String; 0]);
        assert_eq!(seeing.examined, 1);

        // The point: without `examined` these two runs are distinguishable
        // only by what the caller happens to remember passing.
        assert_ne!(blind.examined, seeing.examined);
    }

    #[test]
    fn missing_rule_is_flagged() {
        let tmp = tempfile::tempdir().unwrap();
        let spec_path = tmp.path().join("spec.md");
        std::fs::write(
            &spec_path,
            "# Demo\n\nReferences BE-AUTHN-001 and BE-MISSING-999.\n",
        )
        .unwrap();
        let rule_path = tmp.path().join("rules.md");
        std::fs::write(&rule_path, "# Rules\n\n### BE-AUTHN-001\n\n> Hashed.\n").unwrap();
        let result = run(
            &CheckRuleIdsArgs {
                path: spec_path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                rule_files: vec![
                    rule_path
                        .strip_prefix(tmp.path())
                        .unwrap()
                        .to_string_lossy()
                        .into(),
                ],
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.missing, vec!["BE-MISSING-999".to_string()]);
        let cited: HashMap<&str, &RuleCitation> = result
            .citations
            .iter()
            .map(|c| (c.rule_id.as_str(), c))
            .collect();
        assert!(
            cited
                .get("BE-AUTHN-001")
                .is_some_and(|c| c.found && !c.deprecated)
        );
        assert!(cited.get("BE-MISSING-999").is_some_and(|c| !c.found));
    }

    #[test]
    fn deprecated_rule_is_flagged() {
        let tmp = tempfile::tempdir().unwrap();
        let spec_path = tmp.path().join("spec.md");
        std::fs::write(&spec_path, "Cites BE-OLD-001.\n").unwrap();
        let rule_path = tmp.path().join("rules.md");
        std::fs::write(
            &rule_path,
            "### BE-OLD-001\n\n**DEPRECATED in 0.5.0:** replaced by BE-NEW-002.\n",
        )
        .unwrap();
        let result = run(
            &CheckRuleIdsArgs {
                path: spec_path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                rule_files: vec![
                    rule_path
                        .strip_prefix(tmp.path())
                        .unwrap()
                        .to_string_lossy()
                        .into(),
                ],
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.deprecated, vec!["BE-OLD-001".to_string()]);
        assert_eq!(result.missing, [] as [String; 0]);
    }

    #[test]
    fn live_rule_adjacent_to_deprecated_neighbor_is_not_flagged() {
        // Scenario primitive-robustness-hardening: the old 256-byte
        // window after any ID occurrence bled into the NEXT rule's
        // section — a short live rule directly above a deprecated rule
        // false-flagged. The scan is now scoped to the rule's own
        // section (heading to next same-or-higher-level heading).
        let tmp = tempfile::tempdir().unwrap();
        let spec_path = tmp.path().join("spec.md");
        std::fs::write(&spec_path, "Cites BE-LIVE-001 and BE-OLD-002.\n").unwrap();
        let rule_path = tmp.path().join("rules.md");
        std::fs::write(
            &rule_path,
            "### BE-LIVE-001\n\n> Short live rule.\n\n\
             ### BE-OLD-002\n\n**DEPRECATED in 0.5.0:** replaced by BE-NEW-003.\n",
        )
        .unwrap();
        let result = run(
            &CheckRuleIdsArgs {
                path: spec_path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                rule_files: vec![
                    rule_path
                        .strip_prefix(tmp.path())
                        .unwrap()
                        .to_string_lossy()
                        .into(),
                ],
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(
            result.deprecated,
            vec!["BE-OLD-002".to_string()],
            "only the genuinely deprecated rule may be flagged"
        );
        let cited: HashMap<&str, &RuleCitation> = result
            .citations
            .iter()
            .map(|c| (c.rule_id.as_str(), c))
            .collect();
        assert!(
            cited
                .get("BE-LIVE-001")
                .is_some_and(|c| c.found && !c.deprecated),
            "live rule adjacent to a deprecated neighbor must not be flagged"
        );
        assert!(
            cited
                .get("BE-OLD-002")
                .is_some_and(|c| c.found && c.deprecated)
        );
    }

    #[test]
    fn deprecation_label_inside_deeper_subsection_still_counts() {
        // A `####` subsection belongs to the `###` rule's section; a
        // label there still deprecates the rule. The section ends only
        // at the next same-or-higher-level heading.
        let tmp = tempfile::tempdir().unwrap();
        let spec_path = tmp.path().join("spec.md");
        std::fs::write(&spec_path, "Cites BE-SUB-001 and BE-NEXT-002.\n").unwrap();
        let rule_path = tmp.path().join("rules.md");
        std::fs::write(
            &rule_path,
            "### BE-SUB-001\n\n> Rule text.\n\n#### Notes\n\n**DEPRECATED in 0.6.0:** see BE-NEXT-002.\n\n\
             ### BE-NEXT-002\n\n> Live successor.\n",
        )
        .unwrap();
        let result = run(
            &CheckRuleIdsArgs {
                path: spec_path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                rule_files: vec![
                    rule_path
                        .strip_prefix(tmp.path())
                        .unwrap()
                        .to_string_lossy()
                        .into(),
                ],
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.deprecated, vec!["BE-SUB-001".to_string()]);
        let cited: HashMap<&str, &RuleCitation> = result
            .citations
            .iter()
            .map(|c| (c.rule_id.as_str(), c))
            .collect();
        assert!(
            cited
                .get("BE-NEXT-002")
                .is_some_and(|c| c.found && !c.deprecated)
        );
    }

    /// Em-dash-dense rule prose (scenario spec-side-parser-hardening) is read
    /// line by line to the section's end: a label after many lines of
    /// multibyte text still marks its rule, and the live neighbor below it,
    /// em-dashes on its own lines, stays live. The scan slices only at line
    /// boundaries, so no byte offset can land inside a character.
    #[test]
    fn multibyte_rule_sections_are_scanned_line_by_line() {
        let dense = "\u{2014} prose \u{2014}\n".repeat(40);
        let rule_content = format!(
            "### BE-EM-001\n\n{dense}**DEPRECATED in v2.0:** superseded \u{2014} see BE-EM-002.\n\n\
             ### BE-EM-002\n\n{dense}"
        );

        let tmp = tempfile::tempdir().unwrap();
        let spec_path = tmp.path().join("spec.md");
        std::fs::write(&spec_path, "Cites BE-EM-001 and BE-EM-002.\n").unwrap();
        let rule_path = tmp.path().join("rules.md");
        std::fs::write(&rule_path, &rule_content).unwrap();
        let result = run(
            &CheckRuleIdsArgs {
                path: spec_path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                rule_files: vec![
                    rule_path
                        .strip_prefix(tmp.path())
                        .unwrap()
                        .to_string_lossy()
                        .into(),
                ],
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.missing, [] as [String; 0]);
        assert_eq!(result.deprecated, vec!["BE-EM-001".to_string()]);
        assert_eq!(
            result.citations,
            [
                RuleCitation {
                    rule_id: "BE-EM-001".into(),
                    found: true,
                    deprecated: true,
                    retired: false,
                    not_installed: false,
                },
                RuleCitation {
                    rule_id: "BE-EM-002".into(),
                    found: true,
                    deprecated: false,
                    retired: false,
                    not_installed: false,
                },
            ]
        );
    }

    #[test]
    fn digit_bearing_category_is_recognized() {
        // Schema permits `[A-Z][A-Z0-9]*` categories; `FE-A11YFORM-*` and
        // `FE-A11YMEDIA-*` (accessibility-frontend.md) carry digits. The ID
        // regex must harvest those headings and match their citations.
        let tmp = tempfile::tempdir().unwrap();
        let spec_path = tmp.path().join("spec.md");
        std::fs::write(&spec_path, "Cites FE-A11YFORM-001 and FE-A11YMEDIA-002.\n").unwrap();
        let rule_path = tmp.path().join("rules.md");
        std::fs::write(
            &rule_path,
            "### FE-A11YFORM-001\n\n> Labels.\n\n### FE-A11YMEDIA-002\n\n> Alt text.\n",
        )
        .unwrap();
        let result = run(
            &CheckRuleIdsArgs {
                path: spec_path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .into(),
                rule_files: vec![
                    rule_path
                        .strip_prefix(tmp.path())
                        .unwrap()
                        .to_string_lossy()
                        .into(),
                ],
            },
            tmp.path(),
        )
        .unwrap();
        assert!(
            result.missing.is_empty(),
            "digit-bearing categories misreported as missing: {:?}",
            result.missing
        );
        let found: Vec<&str> = result
            .citations
            .iter()
            .filter(|c| c.found)
            .map(|c| c.rule_id.as_str())
            .collect();
        assert!(found.contains(&"FE-A11YFORM-001"));
        assert!(found.contains(&"FE-A11YMEDIA-002"));
    }

    /// Run the primitive on `target` (a file written under a temp project)
    /// against one rule file defining `BE-LIVE-001` only.
    fn run_on(target: &str, content: &str) -> CheckRuleIdsResult {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(target), content).unwrap();
        std::fs::write(tmp.path().join("rules.md"), "### BE-LIVE-001\n\n> Live.\n").unwrap();
        run(
            &CheckRuleIdsArgs {
                path: target.into(),
                rule_files: vec!["rules.md".into()],
            },
            tmp.path(),
        )
        .unwrap()
    }

    fn citation<'a>(result: &'a CheckRuleIdsResult, id: &str) -> &'a RuleCitation {
        result
            .citations
            .iter()
            .find(|c| c.rule_id == id)
            .unwrap_or_else(|| panic!("{id} not cited in {:?}", result.citations))
    }

    /// Spec 067 `a-record-cites-a-removed-rule`: a spec's `retired-rules:`
    /// list passes its own citation of a removed rule, in a state of its own.
    #[test]
    fn a_listed_removed_rule_is_retired_not_missing() {
        let result = run_on(
            "spec.md",
            "---\nstatus: done\nretired-rules: [BE-OLD-005]\n---\n\nBE-OLD-005 was removed.\n",
        );
        assert_eq!(result.retired, vec!["BE-OLD-005".to_string()]);
        assert_eq!(result.missing, [] as [String; 0]);
        let cited = citation(&result, "BE-OLD-005");
        assert!(cited.retired && !cited.found && !cited.deprecated);
    }

    /// The exemption is the list and nothing else: the same citation in a
    /// spec with no list, or listing a different ID, stays the blocking
    /// finding, so a removed rule cited as a current requirement is caught.
    #[test]
    fn an_unlisted_removed_rule_stays_missing() {
        for content in [
            "---\nstatus: done\n---\n\nBE-OLD-005 MUST hold.\n",
            "---\nstatus: done\nretired-rules: [BE-OLD-006]\n---\n\nBE-OLD-005 MUST hold.\n",
            "---\nstatus: done\nretired-rules: BE-OLD-005\n---\n\nBE-OLD-005 MUST hold.\n",
            "---\nstatus: done\nretired-rules: [5]\n---\n\nBE-OLD-005 MUST hold.\n",
        ] {
            let result = run_on("spec.md", content);
            assert_eq!(result.missing, vec!["BE-OLD-005".to_string()], "{content}");
            assert!(result.retired.is_empty(), "{content}");
            assert!(!citation(&result, "BE-OLD-005").retired, "{content}");
        }
    }

    /// Only a spec records a retirement: the same list in another artifact,
    /// plan.md here, exempts nothing.
    #[test]
    fn a_list_outside_a_spec_exempts_nothing() {
        let result = run_on(
            "plan.md",
            "---\nretired-rules: [BE-OLD-005]\n---\n\nBE-OLD-005 was removed.\n",
        );
        assert_eq!(result.missing, vec!["BE-OLD-005".to_string()]);
        assert_eq!(result.retired, [] as [String; 0]);
    }

    /// A listed ID a loaded file still defines is found, as today: the list
    /// exempts nothing until the rule is gone.
    #[test]
    fn a_listed_rule_a_loaded_file_defines_is_found() {
        let result = run_on(
            "spec.md",
            "---\nstatus: done\nretired-rules: [BE-LIVE-001]\n---\n\nBE-LIVE-001 holds.\n",
        );
        assert!(citation(&result, "BE-LIVE-001").found);
        assert!(!citation(&result, "BE-LIVE-001").retired);
        assert_eq!(result.retired, [] as [String; 0]);
        assert_eq!(result.missing, [] as [String; 0]);
    }

    /// A mistyped ID on the list passes its own spec's citation of the same
    /// mistyped ID and no other citation: the list is checked only by the
    /// citations it exempts.
    #[test]
    fn a_mistyped_listed_id_exempts_only_its_own_citation() {
        let result = run_on(
            "spec.md",
            "---\nstatus: done\nretired-rules: [BE-OLD-0005]\n---\n\nBE-OLD-0005 and BE-OLD-005.\n",
        );
        assert_eq!(result.retired, vec!["BE-OLD-0005".to_string()]);
        assert_eq!(result.missing, vec!["BE-OLD-005".to_string()]);
    }

    /// A spec with no frontmatter, or frontmatter that is not YAML, has no
    /// list to read: the citation stays missing and the run does not error.
    #[test]
    fn a_target_whose_list_cannot_be_read_keeps_its_citations_missing() {
        for content in [
            "BE-OLD-005 was removed.\n",
            "---\nretired-rules: [BE-OLD-005\n---\n\nBE-OLD-005\n",
        ] {
            let result = run_on("spec.md", content);
            assert_eq!(result.missing, vec!["BE-OLD-005".to_string()], "{content}");
        }
    }

    /// The IDs every `framework/rules/*.md` heading defines, sorted, one per
    /// line: what `shipped_rule_ids.txt` must hold.
    fn framework_rule_ids() -> String {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/rules");
        let mut ids: Vec<String> = Vec::new();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|ext| ext != "md") {
                continue;
            }
            let content = std::fs::read_to_string(&path).unwrap();
            ids.extend(
                heading_id_regex()
                    .captures_iter(&content)
                    .map(|cap| cap[1].to_string()),
            );
        }
        assert!(!ids.is_empty(), "no rule IDs under {}", dir.display());
        ids.sort();
        ids.dedup();
        let mut index = String::new();
        for id in &ids {
            index.push_str(id);
            index.push('\n');
        }
        index
    }

    /// The compiled index of shipped rule IDs is the IDs `framework/rules/`
    /// defines. Regenerate with `BLESS_RULE_INDEX=1 cargo test --lib
    /// shipped_rule_index`, and read the diff: a removed ID is a rule a
    /// framework release retired.
    #[test]
    fn shipped_rule_index_matches_the_framework_rules() {
        let expected = framework_rule_ids();
        let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/primitives/shipped_rule_ids.txt");
        if std::env::var_os("BLESS_RULE_INDEX").is_some() {
            std::fs::write(&file, &expected).unwrap();
        }
        assert_eq!(
            include_str!("shipped_rule_ids.txt"),
            expected,
            "runtime/src/primitives/shipped_rule_ids.txt drifted from framework/rules/*.md: \
             regenerate with `BLESS_RULE_INDEX=1 cargo test --lib shipped_rule_index`, then rebuild"
        );
    }

    /// Spec 033 `an-out-of-surface-citation-resolves`: a project with
    /// `surfaces = ["backend"]` never receives `security-frontend.md`, so its
    /// citation of `FE-XSS-001` is known and not installed, never missing,
    /// while an ID no shipped file defines stays missing and an installed
    /// rule is found.
    #[test]
    fn an_out_of_surface_citation_is_not_installed_rather_than_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join(".ductus")).unwrap();
        std::fs::write(
            root.join(".ductus/config.toml"),
            "[rules]\nsurfaces = [\"backend\"]\n",
        )
        .unwrap();
        std::fs::create_dir_all(root.join("specs/rules")).unwrap();
        std::fs::write(
            root.join("specs/rules/security-backend.md"),
            "### BE-AUTHN-001\n\n> Hashed.\n",
        )
        .unwrap();
        std::fs::write(
            root.join("spec.md"),
            "Cites BE-AUTHN-001, FE-XSS-001 and FE-NOPE-999.\n",
        )
        .unwrap();
        let result = run(
            &CheckRuleIdsArgs {
                path: "spec.md".into(),
                rule_files: vec!["specs/rules/security-backend.md".into()],
            },
            root,
        )
        .unwrap();
        assert_eq!(result.not_installed, vec!["FE-XSS-001".to_string()]);
        assert_eq!(result.missing, vec!["FE-NOPE-999".to_string()]);
        let xss = citation(&result, "FE-XSS-001");
        assert!(xss.not_installed && !xss.found && !xss.deprecated && !xss.retired);
        assert!(citation(&result, "BE-AUTHN-001").found);
        assert!(!citation(&result, "FE-NOPE-999").not_installed);
    }

    /// A shipped rule whose file is installed is found, not not-installed.
    #[test]
    fn an_installed_shipped_rule_is_found() {
        let result = run_on("spec.md", "BE-LIVE-001 and FE-XSS-001.\n");
        assert!(citation(&result, "BE-LIVE-001").found);
        assert_eq!(result.not_installed, vec!["FE-XSS-001".to_string()]);
    }
}
