//! Stored rule verdicts and whether they still hold (spec 082).
//!
//! `/{project}:analyze` judges each loaded rule's Verification against the
//! spec, and `write-analysis` records what fired. A later run reuses a stored
//! verdict while nothing it was judged against has changed, so a spec that has
//! not changed is judged once, not on every run, and a different session
//! cannot judge it differently. Three callers decide currency, and they must
//! not disagree: `discover-rule-files` (the interactive host's list), the exec
//! walker (which skips current rules) and `write-analysis` (which carries
//! current verdicts forward, derives the unjudged list and detects
//! contradictions). This module is the one decision they share, as
//! [`analyze_subjects::freshness_of`](crate::primitives::analyze_subjects)
//! is for the completion gate and `/{project}:review`'s `analyze` row.
//!
//! A stored verdict is current while four things match (the plan's
//! *A stored verdict is current while four things match*):
//!
//! 1. the design subjects, `spec.md`, `plan.md`, `data-model.md` and each
//!    scenario, digest as the record's `analyzed-digest` entries for them say;
//! 2. the set of rule files loaded now is the set the record's `rule-digest`
//!    names, so a file added or removed makes no verdict reusable
//!    (`BE-CACHE-002`);
//! 3. the rule file the rule lives in digests as the record's `rule-digest`
//!    entry for it says (a Statement or Verification edit moves it);
//! 4. the rule was judged: the record's `unjudged-rules` does not list it.
//!
//! A rule that did not fire has no verdict to be current, so a current rule is
//! one of three things, [`Currency`]: not fired, fired (with its verdict), or
//! stale. Treating "no entry" as stale would judge every quiet rule on every
//! run, and treating it as unjudged would name them all in
//! `rule-assessments-not-checked`.

use std::collections::BTreeMap;
use std::path::{Component, Path};

use crate::primitives::analyze_subjects::{is_design_subject, subject_digest};
use crate::primitives::resolve_anchor::heading_lines;
use crate::primitives::rule_sections::{parse_rule_sections, rules_digest};
use crate::primitives::{RecordLoad, Result, discover_rule_files, load_analyze_record, read_text};
#[cfg(test)]
use crate::schema::primitives::FindingTier;
use crate::schema::primitives::{AnalyzeBlock, RuleVerdict};
use crate::schema::severity::RuleSeverity;

/// An assessable rule: one with a tier and a Verification, which is the only
/// kind a run judges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AssessableRule {
    /// Its ID, from its heading.
    pub(crate) id: String,
    /// Its tier, from its Statement's RFC 2119 keyword.
    pub(crate) tier: RuleSeverity,
    /// Whether its Statement carries both MUST and SHOULD, so a finding of it
    /// may be counted in either tier.
    pub(crate) mixed: bool,
}

/// A rule file a run loaded: its basename, the digest of its rules' text and
/// the assessable rules in heading order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LoadedRuleFile {
    /// The file's basename, the key of the record's `rule-digest`.
    pub(crate) name: String,
    /// [`rules_digest`] of its content.
    pub(crate) digest: String,
    /// Its assessable rules, in heading order.
    pub(crate) rules: Vec<AssessableRule>,
}

impl LoadedRuleFile {
    /// Read a rule file's `content` the way every reader of a rule does.
    pub(crate) fn read(name: &str, content: &str) -> Self {
        let rules = parse_rule_sections(content)
            .into_iter()
            .filter_map(|section| match (section.tier(), &section.verification) {
                (Some(tier), Some(_)) => Some(AssessableRule {
                    mixed: section.carries_both_tiers(),
                    id: section.id,
                    tier,
                }),
                _ => None,
            })
            .collect();
        Self {
            name: name.to_string(),
            digest: rules_digest(content),
            rules,
        }
    }
}

/// Every rule file in the project's rule-file directory, as
/// `/{project}:analyze` loads them (all of them, whatever the surfaces), and
/// the number it could not read.
#[derive(Debug, Default)]
pub(crate) struct Universe {
    /// The files read, in directory order.
    pub(crate) files: Vec<LoadedRuleFile>,
    /// Files listed that could not be read: their rules cannot be named, so
    /// they are counted, never dropped.
    pub(crate) unreadable: u32,
}

/// Load the universe from the rule-file directory.
///
/// # Errors
///
/// The errors `discover_rule_files::resolve_rules_dir` and `list_rule_files`
/// return: a project config that does not parse and a directory that cannot
/// be listed. An absent directory is an empty universe.
pub(crate) fn load_universe(repo: &Path) -> Result<Universe> {
    let mut universe = Universe::default();
    let (Some(dir), _) = discover_rule_files::resolve_rules_dir(repo)? else {
        return Ok(universe);
    };
    for name in discover_rule_files::list_rule_files(&dir)? {
        match std::fs::read_to_string(dir.join(&name)) {
            Ok(content) => universe.files.push(LoadedRuleFile::read(&name, &content)),
            Err(_) => universe.unreadable += 1,
        }
    }
    Ok(universe)
}

/// Why no stored verdict is reusable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NoReuse {
    /// No `analysis.md`, or one that carries no `rule-digest`, which a record
    /// written before spec 082 does not.
    NoPriorVerdicts,
    /// A design subject changed, was added or was removed, or one could not
    /// be read.
    SubjectsChanged,
    /// The rule files loaded now are not the set the record's `rule-digest`
    /// names: a file added or removed. A verdict is current only for the set
    /// it was judged under (`BE-CACHE-002`), so none is reusable; a file whose
    /// content alone changed keeps the per-file rule.
    RuleFilesChanged,
    /// The record does not parse. Distinct from holding no verdicts: the
    /// run judges every rule, and says why.
    RecordUnreadable(String),
}

impl NoReuse {
    /// The reason as `discover-rule-files` reports it.
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::NoPriorVerdicts => "no-prior-verdicts",
            Self::SubjectsChanged => "subjects-changed",
            Self::RuleFilesChanged => "rule-files-changed",
            Self::RecordUnreadable(_) => "record-unreadable",
        }
    }
}

/// Where one rule stands against the prior record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Currency {
    /// Judged against the current subjects and rule text, and did not fire.
    NotFired,
    /// Judged against them, and fired, with the verdict stored.
    Fired(RuleVerdict),
    /// Not judged against them: judge it.
    Stale,
}

/// The standing of every assessable rule of the loaded files.
#[derive(Debug)]
pub(crate) struct Standing {
    /// Why nothing is reusable, when nothing is.
    pub(crate) no_reuse: Option<NoReuse>,
    /// Each distinct assessable rule in file then heading order.
    rules: Vec<(AssessableRule, Currency)>,
}

impl Standing {
    /// Each assessable rule with where it stands, in file then heading order.
    pub(crate) fn rules(&self) -> &[(AssessableRule, Currency)] {
        &self.rules
    }

    /// Where `id` stands, `None` for a rule no loaded file defines.
    pub(crate) fn currency(&self, id: &str) -> Option<&Currency> {
        self.rules
            .iter()
            .find(|(rule, _)| rule.id == id)
            .map(|(_, currency)| currency)
    }

    /// The rules to judge: every stale one.
    pub(crate) fn judge(&self) -> impl Iterator<Item = &AssessableRule> {
        self.rules
            .iter()
            .filter(|(_, currency)| *currency == Currency::Stale)
            .map(|(rule, _)| rule)
    }

    /// The current fired verdicts, each with its rule.
    pub(crate) fn reusable(&self) -> impl Iterator<Item = (&AssessableRule, &RuleVerdict)> {
        self.rules
            .iter()
            .filter_map(|(rule, currency)| match currency {
                Currency::Fired(verdict) => Some((rule, verdict)),
                Currency::NotFired | Currency::Stale => None,
            })
    }
}

/// Where each assessable rule of `files` stands against `feature_dir`'s
/// `analysis.md`.
///
/// Never fails: a record that cannot be read is a [`NoReuse`] reason, so the
/// caller judges everything and says why, and a design subject that cannot be
/// read or reached is a changed one.
pub(crate) fn standing(feature_dir: &Path, files: &[LoadedRuleFile]) -> Standing {
    let mut rules: Vec<(AssessableRule, Currency)> = Vec::new();
    for rule in files.iter().flat_map(|file| &file.rules) {
        if !rules.iter().any(|(seen, _)| seen.id == rule.id) {
            rules.push((rule.clone(), Currency::Stale));
        }
    }
    let prior = match load_analyze_record(feature_dir) {
        RecordLoad::Absent => return unreusable(rules, NoReuse::NoPriorVerdicts),
        RecordLoad::Unreadable(reason) => {
            return unreusable(rules, NoReuse::RecordUnreadable(reason));
        }
        RecordLoad::Present(prior) => prior,
    };
    if prior.rule_digest.is_empty() {
        return unreusable(rules, NoReuse::NoPriorVerdicts);
    }
    if !design_subjects_match(feature_dir, &prior) {
        return unreusable(rules, NoReuse::SubjectsChanged);
    }
    let loaded_now: std::collections::BTreeSet<&str> =
        files.iter().map(|file| file.name.as_str()).collect();
    let judged_under: std::collections::BTreeSet<&str> =
        prior.rule_digest.keys().map(String::as_str).collect();
    if loaded_now != judged_under {
        return unreusable(rules, NoReuse::RuleFilesChanged);
    }
    // A rule is current when its own file is, so a rule defined in two files
    // is current only when both are.
    let mut currency: BTreeMap<&str, Currency> = BTreeMap::new();
    for file in files {
        let file_current = prior.rule_digest.get(&file.name) == Some(&file.digest);
        for rule in &file.rules {
            let here = if file_current && !prior.unjudged_rules.contains(&rule.id) {
                match prior.rule_verdicts.get(&rule.id) {
                    Some(verdict) => Currency::Fired(verdict.clone()),
                    None => Currency::NotFired,
                }
            } else {
                Currency::Stale
            };
            let entry = currency.entry(&rule.id).or_insert_with(|| here.clone());
            if here == Currency::Stale {
                *entry = Currency::Stale;
            }
        }
    }
    for (rule, standing) in &mut rules {
        if let Some(found) = currency.remove(rule.id.as_str()) {
            *standing = found;
        }
    }
    Standing {
        no_reuse: None,
        rules,
    }
}

fn unreusable(rules: Vec<(AssessableRule, Currency)>, why: NoReuse) -> Standing {
    Standing {
        no_reuse: Some(why),
        rules,
    }
}

/// Whether the design subjects now are the ones the prior record was taken
/// over: the same set, each with the same digest, none unreadable.
///
/// An unreadable entry that is no markdown file is a directory the walk could
/// not reach, and everything beneath it is unreachable, so it blocks too; one
/// that is a markdown file blocks only when it is a design subject, since a
/// `tasks.md` the record could not read decides nothing here.
fn design_subjects_match(feature_dir: &Path, prior: &AnalyzeBlock) -> bool {
    let is_markdown = |path: &str| {
        Path::new(path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
    };
    let blocks = |path: &String| is_design_subject(path) || !is_markdown(path);
    let now = subject_digest(feature_dir, is_design_subject);
    if now.unreadable.iter().any(blocks) || prior.analyzed_unreadable.iter().any(blocks) {
        return false;
    }
    let recorded: BTreeMap<&String, &String> = prior
        .analyzed_digest
        .iter()
        .filter(|(path, _)| is_design_subject(path))
        .collect();
    let current: BTreeMap<&String, &String> = now.digests.iter().collect();
    recorded == current
}

/// Whether `cite`, `<path>#<Heading>`, names a design subject of the feature
/// and a heading it holds.
///
/// The path is a design subject by [`is_design_subject`], a regular file
/// inside the feature directory (a link is not one, and `..` is refused); the
/// heading is compared by equality with a heading
/// [`heading_lines`](crate::primitives::resolve_anchor) reads, the reader
/// `resolve-anchor` and `read-section` share, so a heading in a fence or a
/// comment is content, not a heading. A citation that fails any of these
/// does not resolve, and a verdict resting on it is not stored.
pub(crate) fn cite_resolves(feature_dir: &Path, cite: &str) -> bool {
    let Some((path, heading)) = cite.split_once('#') else {
        return false;
    };
    let (path, heading) = (path.trim(), heading.trim());
    if heading.is_empty()
        || !is_design_subject(path)
        || Path::new(path)
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return false;
    }
    let file = feature_dir.join(path);
    if !std::fs::symlink_metadata(&file).is_ok_and(|meta| meta.file_type().is_file()) {
        return false;
    }
    read_text(&file).is_ok_and(|text| {
        heading_lines(&text)
            .iter()
            .any(|found| found.text == heading)
    })
}

/// The earlier verdict a new `finding` contradicts, when it does: a rule
/// that did not fire, or one the spec addressed, and now fires against the
/// same subjects (AC3). Findings against findings, and any verdict against a
/// stale or unjudged rule, contradict nothing.
pub(crate) fn contradiction(earlier: &Currency, now: &RuleVerdict) -> Option<&'static str> {
    if !matches!(now, RuleVerdict::Finding { .. }) {
        return None;
    }
    match earlier {
        Currency::NotFired => Some("not-fired"),
        Currency::Fired(RuleVerdict::Addressed { .. }) => Some("addressed"),
        Currency::Fired(RuleVerdict::Finding { .. }) | Currency::Stale => None,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::analyze_subjects::is_analyze_subject;
    use std::fmt::Write as _;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    const MUST_RULES: &str = "### TS-MUST-001\n\n> A MUST b.\n\n**Verification:** Check one.\n\n\
                              ### TS-MUST-002\n\n> A MUST c.\n\n**Verification:** Check two.\n";
    const SHOULD_RULES: &str =
        "### TS-SHOULD-001\n\n> A SHOULD d.\n\n**Verification:** Check three.\n";

    fn files(must: &str, should: &str) -> Vec<LoadedRuleFile> {
        vec![
            LoadedRuleFile::read("a-cross.md", must),
            LoadedRuleFile::read("b-cross.md", should),
        ]
    }

    fn feature() -> TempDir {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("scenarios")).unwrap();
        fs::write(tmp.path().join("spec.md"), "# Spec\n\n## Goal\n").unwrap();
        fs::write(tmp.path().join("plan.md"), "# Plan\n\n## Observability\n").unwrap();
        fs::write(tmp.path().join("tasks.md"), "# Tasks\n\n- [ ] one\n").unwrap();
        fs::write(tmp.path().join("scenarios/s.md"), "# S\n\n## Context\n").unwrap();
        tmp
    }

    /// A prior `analysis.md` as `write-analysis` writes it: the analyze
    /// subjects' digest, the rule files' digests, and `extra` (verdicts and
    /// the unjudged list, as frontmatter text).
    fn record(dir: &Path, files: &[LoadedRuleFile], extra: &str) {
        let subjects = subject_digest(dir, is_analyze_subject);
        let mut text = String::from("---\nspec: 001-x\nlast-run: 2026-11-12T00:00:00Z\n");
        if !subjects.digests.is_empty() {
            text.push_str("analyzed-digest:\n");
            for (path, digest) in &subjects.digests {
                let _ = writeln!(text, "  {path}: {digest}");
            }
        }
        if !files.is_empty() {
            text.push_str("rule-digest:\n");
            for file in files {
                let _ = writeln!(text, "  {}: {}", file.name, file.digest);
            }
        }
        text.push_str(extra);
        text.push_str("---\n\n# Analysis\n");
        fs::write(dir.join("analysis.md"), text).unwrap();
    }

    const ONE_FIRED: &str = "rule-verdicts:\n  TS-MUST-001: {verdict: finding, tier: must, at: \"spec.md#Goal\", message: \"bad\"}\n";

    fn states(standing: &Standing) -> Vec<(&str, &Currency)> {
        standing
            .rules()
            .iter()
            .map(|(rule, currency)| (rule.id.as_str(), currency))
            .collect()
    }

    fn all_stale(standing: &Standing) -> bool {
        standing.rules().iter().all(|(_, c)| *c == Currency::Stale)
    }

    /// A record whose subjects and rule files are unchanged makes every rule
    /// current, in the three states: fired with its verdict, not fired, and
    /// nothing left to judge (AC1).
    #[test]
    fn an_unchanged_spec_and_rule_set_leave_nothing_to_judge() {
        let tmp = feature();
        let loaded = files(MUST_RULES, SHOULD_RULES);
        record(tmp.path(), &loaded, ONE_FIRED);
        let standing = standing(tmp.path(), &loaded);
        assert_eq!(standing.no_reuse, None);
        assert_eq!(
            states(&standing),
            [
                (
                    "TS-MUST-001",
                    &Currency::Fired(RuleVerdict::Finding {
                        tier: FindingTier::Must,
                        at: "spec.md#Goal".into(),
                        message: "bad".into()
                    })
                ),
                ("TS-MUST-002", &Currency::NotFired),
                ("TS-SHOULD-001", &Currency::NotFired),
            ]
        );
        assert_eq!(standing.judge().count(), 0);
        let reusable: Vec<&str> = standing.reusable().map(|(r, _)| r.id.as_str()).collect();
        assert_eq!(reusable, ["TS-MUST-001"]);
    }

    /// A design subject's edit judges every rule again, whichever subject it
    /// is, and a `tasks.md` or `review.md` edit judges none (AC1, the
    /// Resolved Questions).
    #[test]
    fn a_design_subject_edit_judges_every_rule_and_a_tasks_or_review_edit_none() {
        let loaded = files(MUST_RULES, SHOULD_RULES);
        for edited in ["spec.md", "plan.md", "scenarios/s.md"] {
            let tmp = feature();
            record(tmp.path(), &loaded, ONE_FIRED);
            fs::write(tmp.path().join(edited), "# Edited\n").unwrap();
            let standing = standing(tmp.path(), &loaded);
            assert_eq!(
                standing.no_reuse,
                Some(NoReuse::SubjectsChanged),
                "{edited}"
            );
            assert!(all_stale(&standing), "{edited}");
        }
        let tmp = feature();
        record(tmp.path(), &loaded, ONE_FIRED);
        fs::write(tmp.path().join("tasks.md"), "# Tasks\n\n- [x] one\n").unwrap();
        fs::write(tmp.path().join("review.md"), "# Review\n").unwrap();
        let standing = standing(tmp.path(), &loaded);
        assert_eq!(standing.no_reuse, None);
        assert_eq!(standing.judge().count(), 0);
    }

    /// A design subject added (a first `data-model.md`) or removed (a
    /// deleted scenario) changes the set, so every rule is judged again.
    #[test]
    fn a_design_subject_added_or_removed_judges_every_rule() {
        let loaded = files(MUST_RULES, SHOULD_RULES);
        let tmp = feature();
        record(tmp.path(), &loaded, "");
        fs::write(tmp.path().join("data-model.md"), "# Data\n").unwrap();
        assert!(all_stale(&standing(tmp.path(), &loaded)));

        let tmp = feature();
        record(tmp.path(), &loaded, "");
        fs::remove_file(tmp.path().join("scenarios/s.md")).unwrap();
        let standing = standing(tmp.path(), &loaded);
        assert_eq!(standing.no_reuse, Some(NoReuse::SubjectsChanged));
        assert!(all_stale(&standing));
    }

    /// A rule file whose rules changed judges that file's rules again and
    /// leaves the other files' current (AC5).
    #[test]
    fn an_edited_rule_file_judges_its_own_rules_only() {
        let tmp = feature();
        record(tmp.path(), &files(MUST_RULES, SHOULD_RULES), ONE_FIRED);
        let edited = files(
            &MUST_RULES.replace("Check two.", "Check two, and three."),
            SHOULD_RULES,
        );
        let standing = standing(tmp.path(), &edited);
        assert_eq!(standing.no_reuse, None);
        let stale: Vec<&str> = standing.judge().map(|r| r.id.as_str()).collect();
        assert_eq!(stale, ["TS-MUST-001", "TS-MUST-002"]);
        assert_eq!(
            standing.currency("TS-SHOULD-001"),
            Some(&Currency::NotFired)
        );
    }

    /// A rule the prior run left unjudged is judged now, and so is a rule
    /// in a file the prior record never loaded (newly loaded).
    #[test]
    fn an_unjudged_or_newly_loaded_rule_is_judged() {
        let tmp = feature();
        let before = files(MUST_RULES, SHOULD_RULES);
        record(tmp.path(), &before, "unjudged-rules:\n  - TS-MUST-002\n");
        assert_eq!(
            standing(tmp.path(), &before)
                .judge()
                .map(|r| r.id.as_str())
                .collect::<Vec<_>>(),
            ["TS-MUST-002"]
        );

        let mut after = before.clone();
        after.push(LoadedRuleFile::read(
            "c-cross.md",
            "### TS-NEW-001\n\n> A MUST e.\n\n**Verification:** Check four.\n",
        ));
        record(tmp.path(), &before, "");
        let standing = standing(tmp.path(), &after);
        assert_eq!(standing.no_reuse, Some(NoReuse::RuleFilesChanged));
        assert_eq!(
            standing.judge().map(|r| r.id.as_str()).collect::<Vec<_>>(),
            ["TS-MUST-001", "TS-MUST-002", "TS-SHOULD-001", "TS-NEW-001"],
            "a file added since the record makes no verdict reusable"
        );
    }

    /// A verdict is current only for the rule-file set it was judged under:
    /// a file added or removed makes none reusable, and the unchanged set
    /// still reuses (`BE-CACHE-002`).
    #[test]
    fn a_changed_rule_file_set_makes_no_verdict_reusable() {
        let tmp = feature();
        let before = files(MUST_RULES, SHOULD_RULES);
        record(tmp.path(), &before, ONE_FIRED);

        let same = standing(tmp.path(), &before);
        assert_eq!(same.no_reuse, None);
        assert_eq!(same.reusable().count(), 1);
        assert_eq!(same.judge().count(), 0);

        let mut added = before.clone();
        added.push(LoadedRuleFile::read("c-cross.md", "No rules here.\n"));
        let added = standing(tmp.path(), &added);
        assert_eq!(added.no_reuse, Some(NoReuse::RuleFilesChanged));
        assert_eq!(
            added.no_reuse.as_ref().unwrap().code(),
            "rule-files-changed"
        );
        assert_eq!(added.reusable().count(), 0);
        assert!(all_stale(&added));

        let removed = standing(tmp.path(), &before[..1]);
        assert_eq!(removed.no_reuse, Some(NoReuse::RuleFilesChanged));
        assert_eq!(removed.reusable().count(), 0);
        assert!(all_stale(&removed));
    }

    /// A stored verdict naming a rule no loaded file defines is dropped: it
    /// is in no state, and no reusable list carries it.
    #[test]
    fn a_removed_rule_s_verdict_is_dropped() {
        let tmp = feature();
        let before = files(MUST_RULES, SHOULD_RULES);
        record(tmp.path(), &before, ONE_FIRED);
        let only_should = vec![before[1].clone()];
        let standing = standing(tmp.path(), &only_should);
        assert_eq!(standing.no_reuse, Some(NoReuse::RuleFilesChanged));
        assert_eq!(standing.currency("TS-MUST-001"), None);
        assert_eq!(standing.reusable().count(), 0);
        assert_eq!(standing.rules().len(), 1);
    }

    /// With no record, none carrying a digest, or one that does not parse,
    /// nothing is reusable and the reason says which; an unreadable record
    /// is not an empty one.
    #[test]
    fn nothing_is_reusable_without_a_readable_record_carrying_a_digest() {
        let loaded = files(MUST_RULES, SHOULD_RULES);
        let tmp = feature();
        let absent = standing(tmp.path(), &loaded);
        assert_eq!(absent.no_reuse, Some(NoReuse::NoPriorVerdicts));
        assert!(all_stale(&absent));

        record(tmp.path(), &[], "");
        assert_eq!(
            standing(tmp.path(), &loaded).no_reuse,
            Some(NoReuse::NoPriorVerdicts),
            "a record written before spec 082 has no rule-digest"
        );

        record(tmp.path(), &loaded, "rule-verdicts: [TS-MUST-001]\n");
        let unreadable = standing(tmp.path(), &loaded);
        assert!(
            matches!(unreadable.no_reuse, Some(NoReuse::RecordUnreadable(_))),
            "{:?}",
            unreadable.no_reuse
        );
        assert_eq!(unreadable.no_reuse.unwrap().code(), "record-unreadable");
        assert!(all_stale(&standing(tmp.path(), &loaded)));
    }

    /// A design subject the record could not read, or the walk cannot reach
    /// now, is not a matching one.
    #[test]
    fn an_unreadable_design_subject_is_a_changed_one() {
        let loaded = files(MUST_RULES, SHOULD_RULES);
        let tmp = feature();
        record(tmp.path(), &loaded, "analyzed-unreadable:\n  - plan.md\n");
        assert_eq!(
            standing(tmp.path(), &loaded).no_reuse,
            Some(NoReuse::SubjectsChanged)
        );
        record(tmp.path(), &loaded, "analyzed-unreadable:\n  - scenarios\n");
        assert_eq!(
            standing(tmp.path(), &loaded).no_reuse,
            Some(NoReuse::SubjectsChanged),
            "a directory the walk could not reach hides its files"
        );
        record(tmp.path(), &loaded, "analyzed-unreadable:\n  - tasks.md\n");
        assert_eq!(standing(tmp.path(), &loaded).no_reuse, None);
    }

    /// The universe is every rule file in the rule-file directory, and a file
    /// that cannot be read is counted.
    #[test]
    fn the_universe_is_every_rule_file_in_the_directory() {
        let tmp = tempdir().unwrap();
        assert_eq!(load_universe(tmp.path()).unwrap().files.len(), 0);
        let dir = tmp.path().join("framework/rules");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a-cross.md"), MUST_RULES).unwrap();
        fs::write(dir.join("b-backend.md"), SHOULD_RULES).unwrap();
        fs::write(dir.join("c-cross.md"), [0xff, 0xfe]).unwrap();
        fs::write(dir.join("notes.txt"), "not a rule file").unwrap();
        let universe = load_universe(tmp.path()).unwrap();
        let names: Vec<&str> = universe.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, ["a-cross.md", "b-backend.md"]);
        assert_eq!(universe.unreadable, 1);
        assert_eq!(universe.files[0].rules.len(), 2);
        assert_eq!(universe.files[0].digest, rules_digest(MUST_RULES));
    }

    /// A rule ID two files define is one rule, current only while both files
    /// are.
    #[test]
    fn a_rule_two_files_define_is_one_rule_current_while_both_files_are() {
        let tmp = feature();
        let twice = "### TS-MUST-001\n\n> A MUST b.\n\n**Verification:** Check one.\n";
        let before = vec![
            LoadedRuleFile::read("a-cross.md", twice),
            LoadedRuleFile::read("b-cross.md", twice),
        ];
        record(tmp.path(), &before, "");
        let same = standing(tmp.path(), &before);
        assert_eq!(same.rules().len(), 1);
        assert_eq!(same.currency("TS-MUST-001"), Some(&Currency::NotFired));

        let edited = vec![
            before[0].clone(),
            LoadedRuleFile::read(
                "b-cross.md",
                &twice.replace("Check one.", "Check one, two."),
            ),
        ];
        let changed = standing(tmp.path(), &edited);
        assert_eq!(changed.rules().len(), 1);
        assert_eq!(changed.currency("TS-MUST-001"), Some(&Currency::Stale));
    }

    /// A rule with no Verification or no keyword is not assessable.
    #[test]
    fn only_a_rule_with_a_tier_and_a_verification_is_assessable() {
        let file = LoadedRuleFile::read(
            "x.md",
            "### TS-AA-001\n\n> A MUST b.\n\n**Verification:** v.\n\n\
             ### TS-AA-002\n\n> A MUST c.\n\n**Rationale:** none.\n\n\
             ### TS-AA-003\n\n> Only a note.\n\n**Verification:** v.\n",
        );
        let ids: Vec<&str> = file.rules.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["TS-AA-001"]);
    }

    /// A citation resolves when it names a design subject and a heading it
    /// holds; every other form does not (AC6).
    #[test]
    fn a_citation_resolves_only_to_a_heading_a_design_subject_holds() {
        let tmp = feature();
        let dir = tmp.path();
        fs::write(
            dir.join("data-model.md"),
            "# Data\n\n```md\n## In a fence\n```\n\n## Entities\n",
        )
        .unwrap();
        // Real headings in files that are no design subject.
        fs::write(dir.join("tasks.md"), "# Tasks\n\n## One task\n").unwrap();
        fs::write(dir.join("analysis.md"), "# Analysis\n\n## Summary\n").unwrap();
        for good in [
            "plan.md#Observability",
            "spec.md#Goal",
            "scenarios/s.md#Context",
            "data-model.md#Entities",
            " plan.md # Observability ",
        ] {
            assert!(cite_resolves(dir, good), "{good}");
        }
        for bad in [
            "",
            "plan.md",
            "plan.md#",
            "plan.md#Missing",
            "plan.md#observability",
            "tasks.md#One task",
            "analysis.md#Summary",
            "gone.md#Goal",
            "scenarios/gone.md#Context",
            "data-model.md#In a fence",
            "scenarios/../tasks.md#Tasks",
            "../plan.md#Observability",
            "/plan.md#Observability",
            "notes.txt#Goal",
        ] {
            assert!(!cite_resolves(dir, bad), "{bad}");
        }
    }

    /// A citation into a link is not into a file of the feature.
    #[cfg(unix)]
    #[test]
    fn a_citation_into_a_link_does_not_resolve() {
        let tmp = feature();
        let outside = tempdir().unwrap();
        fs::write(outside.path().join("o.md"), "# O\n\n## Heading\n").unwrap();
        std::os::unix::fs::symlink(
            outside.path().join("o.md"),
            tmp.path().join("data-model.md"),
        )
        .unwrap();
        assert!(!cite_resolves(tmp.path(), "data-model.md#Heading"));
    }

    /// A new finding contradicts a rule that did not fire and one the spec
    /// addressed, and nothing else: not a finding, not a stale rule, and no
    /// other new verdict.
    #[test]
    fn a_new_finding_contradicts_a_quiet_or_addressed_rule_only() {
        let finding = RuleVerdict::Finding {
            tier: FindingTier::Must,
            at: "spec.md#Goal".into(),
            message: "m".into(),
        };
        let addressed = RuleVerdict::Addressed {
            cite: "plan.md#Observability".into(),
        };
        assert_eq!(
            contradiction(&Currency::NotFired, &finding),
            Some("not-fired")
        );
        assert_eq!(
            contradiction(&Currency::Fired(addressed.clone()), &finding),
            Some("addressed")
        );
        assert_eq!(
            contradiction(&Currency::Fired(finding.clone()), &finding),
            None
        );
        assert_eq!(contradiction(&Currency::Stale, &finding), None);
        assert_eq!(contradiction(&Currency::NotFired, &addressed), None);
        assert_eq!(
            contradiction(&Currency::Fired(finding.clone()), &addressed),
            None
        );
    }
}
