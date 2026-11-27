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
//! A stored verdict is current while five things hold (082's plan names the
//! first four, *A stored verdict is current while four things match*):
//!
//! 1. the design subjects, `spec.md`, `plan.md`, `data-model.md` and each
//!    scenario, digest as the record's `analyzed-digest` entries for them say,
//!    or, where they do not, the sections the verdict concerns digest as the
//!    record's `section-digest` says (AC7, [`SectionDelta`]);
//! 2. the set of rule files loaded now is the set the record's `rule-digest`
//!    names, so a file added or removed makes no verdict reusable
//!    (`BE-CACHE-002`);
//! 3. the rule's own text, its ID, Statement and Verification, is the text
//!    the run judged: its file digests as the record's `rule-digest` entry
//!    says, or, compared rule by rule with its copy at the record's
//!    `analyzed-against`, which stands in only when it digests as that entry
//!    says, the rule reads as it did there (spec 088, [`CopyUnavailable`]
//!    when it cannot stand in, and every rule of the file is stale);
//! 4. no rule its Statement or Verification names changed so, or was removed,
//!    and, while a file is judged whole, it names no ID that no loaded rule
//!    defines and the framework does not ship, which that file may have held
//!    (spec 088);
//! 5. the rule was judged: the record's `unjudged-rules` does not list it.
//!
//! A rule that did not fire has no verdict to be current, so a current rule is
//! one of four things, [`Currency`]: not fired, fired (with its verdict),
//! owed a judgment against the changed sections only, or stale. Treating "no
//! entry" as stale would judge every quiet rule on every run, and treating it
//! as unjudged would name them all in `rule-assessments-not-checked`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use crate::primitives::analyze_subjects::{is_design_subject, section_digest, subject_digest_with};
use crate::primitives::check_rule_ids::shipped_rule_ids;
use crate::primitives::departures::{self, Attribution};
use crate::primitives::resolve_anchor::heading_lines;
use crate::primitives::rule_sections::{named_rule_ids, parse_rule_sections, section_digests};
use crate::primitives::{
    ProjectRepository, RecordLoad, Result, blob_text, discover_rule_files, load_analyze_record,
    read_text,
};
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
    /// The other rule IDs its Statement and Verification name, whose change
    /// makes its verdict stale (spec 088).
    pub(crate) names: BTreeSet<String>,
}

/// A rule file a run loaded: its basename, its path, the digest of its rules'
/// text, each rule's own digest and the assessable rules in heading order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LoadedRuleFile {
    /// The file's basename, the key of the record's `rule-digest`.
    pub(crate) name: String,
    /// Its `/`-separated path from the project root, where history holds the
    /// copy a run judged (spec 088).
    pub(crate) path: String,
    /// The digest of its rules' text, by [`section_digests`].
    pub(crate) digest: String,
    /// Each rule's own text digested by ID, every rule section's, assessable
    /// or not ([`section_digests`]).
    pub(crate) texts: BTreeMap<String, String>,
    /// Its assessable rules, in heading order.
    pub(crate) rules: Vec<AssessableRule>,
}

impl LoadedRuleFile {
    /// Read the rule file at `path`, from the project root, whose text is
    /// `content`, the way every reader of a rule does.
    pub(crate) fn read(path: &str, content: &str) -> Self {
        let sections = parse_rule_sections(content);
        let (digest, texts) = section_digests(&sections);
        let rules = sections
            .into_iter()
            .filter_map(|section| match (section.tier(), &section.verification) {
                (Some(tier), Some(verification)) => {
                    let mut names = named_rule_ids(&section.statement);
                    names.extend(named_rule_ids(verification));
                    names.remove(&section.id);
                    Some(AssessableRule {
                        mixed: section.carries_both_tiers(),
                        id: section.id,
                        tier,
                        names,
                    })
                }
                _ => None,
            })
            .collect();
        Self {
            name: Path::new(path).file_name().map_or_else(
                || path.to_string(),
                |name| name.to_string_lossy().into_owned(),
            ),
            path: path.to_string(),
            digest,
            texts,
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
    let (Some(dir), rel) = discover_rule_files::resolve_rules_dir(repo)? else {
        return Ok(universe);
    };
    for name in discover_rule_files::list_rule_files(&dir)? {
        match std::fs::read_to_string(dir.join(&name)) {
            Ok(content) => universe
                .files
                .push(LoadedRuleFile::read(&format!("{rel}/{name}"), &content)),
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
    /// A design subject changed, was added or was removed and the record
    /// carries no `section-digest` to say which sections, written before
    /// AC7; or a design subject could not be read.
    SubjectsChanged,
    /// The rule files loaded now are not the set the record's `rule-digest`
    /// names: a file added or removed. A verdict is current only for the set
    /// it was judged under (`BE-CACHE-002`), so none is reusable; a file whose
    /// content alone changed has its rules compared one by one (spec 088).
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

/// Why a changed rule file's copy at the record's `analyzed-against` cannot
/// stand in for the text the run judged, so every rule of it is judged again
/// (spec 088, AC2). The closed set `discover-rule-files` reports under
/// `rejudged-files`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CopyUnavailable {
    /// The record names no `analyzed-against`.
    NoAnalyzedAgainst,
    /// The repository holds no commit by that sha: a shallow clone, a commit
    /// never fetched, or a sha that names none.
    CommitNotHeld,
    /// The commit's tree holds no such file at its path, or not as UTF-8.
    NotAtCommit,
    /// The copy there does not digest as the record's entry says: the run
    /// read rule text that commit does not hold, as an analysis run before
    /// its commit can.
    JudgedCopyDiffers,
    /// The project is in no repository, or git could not read the commit,
    /// its tree or the blob.
    HistoryUnreadable,
}

impl CopyUnavailable {
    /// The reason as `discover-rule-files` reports it.
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::NoAnalyzedAgainst => "no-analyzed-against",
            Self::CommitNotHeld => "commit-not-held",
            Self::NotAtCommit => "not-at-commit",
            Self::JudgedCopyDiffers => "judged-copy-differs",
            Self::HistoryUnreadable => "history-unreadable",
        }
    }
}

/// Where one rule stands against the prior record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Currency {
    /// Judged against the current subjects and rule text, and did not fire.
    NotFired,
    /// Judged against them, and fired, with the verdict stored; or a
    /// `finding` whose section is unchanged, which stands unjudged (AC7).
    Fired(RuleVerdict),
    /// Judged against the current rule text and every section but the
    /// changed ones, against which it is to be judged (AC7). `earlier` is its
    /// verdict there, `None` when it did not fire, and `carries` whether that
    /// verdict stands when the judgment of the changed sections does not
    /// replace it: an `addressed` one only while its `cite` is at an
    /// unchanged section, a `waived` one always.
    Rejudge {
        /// The stored verdict, `None` when the rule did not fire.
        earlier: Option<RuleVerdict>,
        /// Whether `earlier` stands when the judgment replaces nothing.
        carries: bool,
    },
    /// Not judged against them: judge it in full.
    Stale,
}

impl Currency {
    /// Whether a run owes the rule a judgment: in full, or against the
    /// changed sections.
    pub(crate) fn owes_judgment(&self) -> bool {
        matches!(self, Self::Stale | Self::Rejudge { .. })
    }

    /// The stored verdict that stands when this run's judgment stores none:
    /// a current fired one, or a carried one of a rule judged against the
    /// changed sections.
    pub(crate) fn carried(&self) -> Option<&RuleVerdict> {
        match self {
            Self::Fired(verdict)
            | Self::Rejudge {
                earlier: Some(verdict),
                carries: true,
            } => Some(verdict),
            Self::NotFired | Self::Rejudge { .. } | Self::Stale => None,
        }
    }

    /// The earlier verdict a finding at an unchanged section would
    /// contradict: `not-fired` or `addressed`, current or owed a judgment
    /// against the changed sections. A finding, a waiver and a stale rule
    /// cleared no section.
    fn cleared(&self) -> Option<&'static str> {
        match self {
            Self::NotFired | Self::Rejudge { earlier: None, .. } => Some("not-fired"),
            Self::Fired(RuleVerdict::Addressed { .. })
            | Self::Rejudge {
                earlier: Some(RuleVerdict::Addressed { .. }),
                ..
            } => Some("addressed"),
            Self::Fired(RuleVerdict::Finding { .. } | RuleVerdict::Waived { .. })
            | Self::Rejudge { .. }
            | Self::Stale => None,
        }
    }
}

/// The sections of the design subjects against the prior record's
/// `section-digest` (AC7): those digesting alike on both sides, and the
/// rest, the added and removed among them.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct SectionDelta {
    unchanged: std::collections::BTreeSet<String>,
    changed: std::collections::BTreeSet<String>,
}

impl SectionDelta {
    /// Compare the prior record's `section-digest` with the sections now.
    pub(crate) fn between(
        prior: &BTreeMap<String, String>,
        now: &BTreeMap<String, String>,
    ) -> Self {
        let mut delta = Self::default();
        for key in prior.keys().chain(now.keys()) {
            if prior.get(key) == now.get(key) {
                delta.unchanged.insert(key.clone());
            } else {
                delta.changed.insert(key.clone());
            }
        }
        delta
    }

    /// Whether `location`, `<path>#<Heading>`, names a section both digests
    /// hold alike. A location naming no heading, or a section either lacks,
    /// is never unchanged: holding a finding to nothing needs proof that its
    /// text was judged before.
    pub(crate) fn unchanged_at(&self, location: &str) -> bool {
        let Some((path, heading)) = location.split_once('#') else {
            return false;
        };
        let (path, heading) = (path.trim(), heading.trim());
        !heading.is_empty() && self.unchanged.contains(&format!("{path}#{heading}"))
    }
}

/// The standing of every assessable rule of the loaded files.
#[derive(Debug)]
pub(crate) struct Standing {
    /// Why nothing is reusable, when nothing is.
    pub(crate) no_reuse: Option<NoReuse>,
    /// Each distinct assessable rule in file then heading order.
    rules: Vec<(AssessableRule, Currency)>,
    /// The sections against the prior record's, when both sides can say
    /// (AC7); `None` for a record without `section-digest`, or none reusable.
    sections: Option<SectionDelta>,
    /// Each changed rule file whose copy at the record's `analyzed-against`
    /// could not stand in, with why, in load order (spec 088).
    rejudged_files: Vec<(String, CopyUnavailable)>,
}

impl Standing {
    /// Each changed rule file judged again whole, with why (spec 088).
    pub(crate) fn rejudged_files(&self) -> &[(String, CopyUnavailable)] {
        &self.rejudged_files
    }

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

    /// The rules to judge in full: every stale one.
    pub(crate) fn judge(&self) -> impl Iterator<Item = &AssessableRule> {
        self.rules
            .iter()
            .filter(|(_, currency)| *currency == Currency::Stale)
            .map(|(rule, _)| rule)
    }

    /// The rules to judge against [`Self::changed_sections`] only (AC7).
    pub(crate) fn rejudge(&self) -> impl Iterator<Item = &AssessableRule> {
        self.rules
            .iter()
            .filter(|(_, currency)| matches!(currency, Currency::Rejudge { .. }))
            .map(|(rule, _)| rule)
    }

    /// The keys of the sections changed since the prior record, the added and
    /// removed among them, in key order; empty when none did, or when the
    /// record cannot say.
    pub(crate) fn changed_sections(&self) -> impl Iterator<Item = &String> {
        self.sections.iter().flat_map(|delta| delta.changed.iter())
    }

    /// The fired verdicts that stand unless this run's judgment replaces
    /// them, each with its rule: every current one, and every carried one of
    /// a rule judged against the changed sections.
    pub(crate) fn reusable(&self) -> impl Iterator<Item = (&AssessableRule, &RuleVerdict)> {
        self.rules
            .iter()
            .filter_map(|(rule, currency)| currency.carried().map(|verdict| (rule, verdict)))
    }

    /// The earlier verdict `verdict`, passed for `id`, is held to nothing
    /// against (AC7): a `finding` at a section unchanged since that rule's
    /// not-fired or `addressed` verdict. Such a finding is a contradiction
    /// alone, stored nowhere and counted in no tier, by `write-analysis` and
    /// the exec tally alike.
    pub(crate) fn demoted(&self, id: &str, verdict: &RuleVerdict) -> Option<&'static str> {
        let RuleVerdict::Finding { at, .. } = verdict else {
            return None;
        };
        let earlier = self.currency(id)?.cleared()?;
        self.sections
            .as_ref()
            .is_some_and(|delta| delta.unchanged_at(at))
            .then_some(earlier)
    }
}

/// Where each assessable rule of `files` stands against `feature_dir`'s
/// `analysis.md`, `project` being the project root, from which history is
/// read for the copy of a changed rule file the run judged (spec 088).
///
/// Never fails: a record that cannot be read is a [`NoReuse`] reason, so the
/// caller judges everything and says why, a design subject that cannot be
/// read or reached is a changed one, and a judged copy that cannot be read is
/// a [`CopyUnavailable`] reason, its file judged again whole.
pub(crate) fn standing(project: &Path, feature_dir: &Path, files: &[LoadedRuleFile]) -> Standing {
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
    let Some(whole_files_match) = design_subjects_match(feature_dir, &prior, None) else {
        return unreusable(rules, NoReuse::SubjectsChanged);
    };
    let sections = (!prior.section_digest.is_empty())
        .then(|| SectionDelta::between(&prior.section_digest, &section_digest(feature_dir)));
    // With every design subject unchanged nothing is to be judged against;
    // otherwise the sections say what is, or, for a record that cannot say,
    // every rule is stale.
    let changed = match &sections {
        _ if whole_files_match => false,
        // A reordering changes a file and no section: nothing to judge.
        Some(delta) => !delta.changed.is_empty(),
        None => return unreusable(rules, NoReuse::SubjectsChanged),
    };
    let loaded_now: std::collections::BTreeSet<&str> =
        files.iter().map(|file| file.name.as_str()).collect();
    let judged_under: std::collections::BTreeSet<&str> =
        prior.rule_digest.keys().map(String::as_str).collect();
    if loaded_now != judged_under {
        return unreusable(rules, NoReuse::RuleFilesChanged);
    }
    let edits = RuleEdits::between(project, &prior, files);
    // A rule is current when its own text is, so a rule defined in two files
    // is current only when both copies are, and when no rule it names changed.
    let mut currency: BTreeMap<&str, Currency> = BTreeMap::new();
    for file in files {
        for rule in &file.rules {
            let here = if edits.own_text_current(file, &rule.id)
                && rule.names.is_disjoint(&edits.changed)
                && !prior.unjudged_rules.contains(&rule.id)
            {
                let verdict = prior.rule_verdicts.get(&rule.id);
                match &sections {
                    Some(delta) if changed => section_currency(delta, verdict),
                    _ => verdict.map_or(Currency::NotFired, |v| Currency::Fired(v.clone())),
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
        sections,
        rejudged_files: edits.rejudged_files,
    }
}

/// What the rule files changed since the prior record, rule by rule (spec
/// 088, AC2): each changed file compared with its copy at the record's
/// `analyzed-against`, behind the guard that the copy digests as the record's
/// `rule-digest` entry says, since `analyzed-against` is a caller's argument
/// and an analysis may run before its commit.
struct RuleEdits {
    /// Per changed file whose copy stood in, by name, its rules whose own
    /// text changed or that it did not hold.
    narrowed: BTreeMap<String, BTreeSet<String>>,
    /// Every changed, added or removed rule's ID, each rule of a file judged
    /// again whole and, while one is, each ID a loaded rule names that no
    /// loaded rule defines and the framework does not ship: what a naming
    /// rule is stale for.
    changed: BTreeSet<String>,
    /// Each file judged again whole, with why.
    rejudged_files: Vec<(String, CopyUnavailable)>,
}

impl RuleEdits {
    /// Compare each file of `files` whose digest differs from `prior`'s.
    fn between(project: &Path, prior: &AnalyzeBlock, files: &[LoadedRuleFile]) -> Self {
        let mut edits = Self {
            narrowed: BTreeMap::new(),
            changed: BTreeSet::new(),
            rejudged_files: Vec::new(),
        };
        // Opened once, and only when some file changed.
        let mut history = History::Unopened;
        for file in files {
            let Some(recorded) = prior.rule_digest.get(&file.name) else {
                continue;
            };
            if *recorded == file.digest {
                continue;
            }
            match judged_copy(project, prior, &mut history, file, recorded) {
                Ok(judged) => {
                    let edited: BTreeSet<String> = file
                        .texts
                        .iter()
                        .filter(|(id, text)| judged.get(*id) != Some(*text))
                        .map(|(id, _)| id.clone())
                        .collect();
                    edits.changed.extend(edited.iter().cloned());
                    edits
                        .changed
                        .extend(judged.into_keys().filter(|id| !file.texts.contains_key(id)));
                    edits.narrowed.insert(file.name.clone(), edited);
                }
                Err(why) => {
                    edits.changed.extend(file.texts.keys().cloned());
                    edits.rejudged_files.push((file.name.clone(), why));
                }
            }
        }
        // A file judged whole cannot say which rules it held, so a name no
        // loaded rule defines may be one it no longer holds: that name is
        // changed too, and each rule naming it stale (spec 088, AC2). A
        // shipped rule's ID is not, since one no loaded file defines sits in
        // a surface the project did not install.
        if !edits.rejudged_files.is_empty() {
            let defined: BTreeSet<&str> = files
                .iter()
                .flat_map(|file| file.texts.keys().map(String::as_str))
                .collect();
            let undefined: Vec<String> = files
                .iter()
                .flat_map(|file| &file.rules)
                .flat_map(|rule| &rule.names)
                .filter(|name| !defined.contains(name.as_str()))
                .filter(|name| !shipped_rule_ids().contains(name.as_str()))
                .cloned()
                .collect();
            edits.changed.extend(undefined);
        }
        edits
    }

    /// Whether `id`'s own text in `file` is the text the run judged.
    fn own_text_current(&self, file: &LoadedRuleFile, id: &str) -> bool {
        match self.narrowed.get(&file.name) {
            Some(edited) => !edited.contains(id),
            None => !self
                .rejudged_files
                .iter()
                .any(|(name, _)| *name == file.name),
        }
    }
}

/// The repository a call reads judged copies from, opened when a file first
/// needs one.
enum History {
    /// Not yet needed.
    Unopened,
    /// The project's repository.
    Open(ProjectRepository),
    /// The project is in no repository.
    Unavailable,
}

/// Each rule's own text in `file`'s copy at `prior`'s `analyzed-against`,
/// digested by ID, when that copy is the one the run judged: resolved as
/// `exempt_renames` resolves the field (`revparse_single`, peeled to its
/// commit), read as `ProjectRepository::read_at_head` reads one at `HEAD`,
/// and digesting as `recorded`, the record's entry for the file, says.
fn judged_copy(
    project: &Path,
    prior: &AnalyzeBlock,
    history: &mut History,
    file: &LoadedRuleFile,
    recorded: &str,
) -> std::result::Result<BTreeMap<String, String>, CopyUnavailable> {
    let sha = prior
        .analyzed_against
        .as_deref()
        .map(str::trim)
        .filter(|sha| !sha.is_empty())
        .ok_or(CopyUnavailable::NoAnalyzedAgainst)?;
    if matches!(history, History::Unopened) {
        *history = ProjectRepository::discover(project).map_or(History::Unavailable, History::Open);
    }
    let History::Open(project_repository) = &*history else {
        return Err(CopyUnavailable::HistoryUnreadable);
    };
    let git = &project_repository.repository;
    let commit = match git.revparse_single(sha) {
        Ok(object) => object
            .peel_to_commit()
            .map_err(|_| CopyUnavailable::HistoryUnreadable)?,
        Err(err) if err.code() == git2::ErrorCode::NotFound => {
            return Err(CopyUnavailable::CommitNotHeld);
        }
        Err(_) => return Err(CopyUnavailable::HistoryUnreadable),
    };
    let tree = commit
        .tree()
        .map_err(|_| CopyUnavailable::HistoryUnreadable)?;
    let text = blob_text(git, &tree, &project_repository.to_git(&file.path))
        .map_err(|_| CopyUnavailable::HistoryUnreadable)?
        .ok_or(CopyUnavailable::NotAtCommit)?;
    let (digest, texts) = section_digests(&parse_rule_sections(&text));
    if digest != recorded {
        return Err(CopyUnavailable::JudgedCopyDiffers);
    }
    Ok(texts)
}

/// Where a judged rule stands once some section changed (AC7): a quiet,
/// `addressed` or `waived` one is judged against the changed sections, an
/// `addressed` one carried only while its `cite` is at an unchanged section;
/// a `finding` stands while its `at` is, and is judged in full otherwise,
/// since a finding cleared no other section (the spec's Resolved Questions,
/// 2026-11-21).
fn section_currency(delta: &SectionDelta, verdict: Option<&RuleVerdict>) -> Currency {
    match verdict {
        None => Currency::Rejudge {
            earlier: None,
            carries: false,
        },
        Some(found @ RuleVerdict::Finding { at, .. }) => {
            if delta.unchanged_at(at) {
                Currency::Fired(found.clone())
            } else {
                Currency::Stale
            }
        }
        Some(addressed @ RuleVerdict::Addressed { cite }) => Currency::Rejudge {
            earlier: Some(addressed.clone()),
            carries: delta.unchanged_at(cite),
        },
        Some(waived @ RuleVerdict::Waived { .. }) => Currency::Rejudge {
            earlier: Some(waived.clone()),
            carries: true,
        },
    }
}

fn unreusable(rules: Vec<(AssessableRule, Currency)>, why: NoReuse) -> Standing {
    Standing {
        no_reuse: Some(why),
        rules,
        sections: None,
        rejudged_files: Vec::new(),
    }
}

/// Whether the design subjects now are the ones the prior record was taken
/// over, the same set, each with the same digest; `None` when one of them, on
/// either side, could not be read, which no section can settle. With
/// `spec_text`, `spec.md` is read as that text, which
/// `backfill-section-digest` uses to ask whether the subjects match with
/// `spec.md` at another status (spec 088, AC3).
///
/// An unreadable entry that is no markdown file is a directory the walk could
/// not reach, and everything beneath it is unreachable, so it blocks too; one
/// that is a markdown file blocks only when it is a design subject, since a
/// `tasks.md` the record could not read decides nothing here.
pub(crate) fn design_subjects_match(
    feature_dir: &Path,
    prior: &AnalyzeBlock,
    spec_text: Option<&str>,
) -> Option<bool> {
    let is_markdown = |path: &str| {
        Path::new(path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
    };
    let blocks = |path: &String| is_design_subject(path) || !is_markdown(path);
    let now = subject_digest_with(feature_dir, is_design_subject, spec_text);
    if now.unreadable.iter().any(blocks) || prior.analyzed_unreadable.iter().any(blocks) {
        return None;
    }
    let recorded: BTreeMap<&String, &String> = prior
        .analyzed_digest
        .iter()
        .filter(|(path, _)| is_design_subject(path))
        .collect();
    let current: BTreeMap<&String, &String> = now.digests.iter().collect();
    Some(recorded == current)
}

/// Whether `cite`, `<path>#<Heading>`, names a design subject of the feature
/// and a heading it holds.
///
/// The path is a design subject by [`is_design_subject`], a regular file
/// inside the feature directory reached through no link, at the file or at
/// any directory on the way (`BE-INPUT-004`), and `..` is refused; the
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
    let mut file = feature_dir.to_path_buf();
    for part in Path::new(path).components() {
        file.push(part);
        if !std::fs::symlink_metadata(&file).is_ok_and(|meta| !meta.file_type().is_symlink()) {
            return false;
        }
    }
    if !std::fs::symlink_metadata(&file).is_ok_and(|meta| meta.file_type().is_file()) {
        return false;
    }
    read_text(&file).is_ok_and(|text| {
        heading_lines(&text)
            .iter()
            .any(|found| found.text == heading)
    })
}

/// The earlier verdict a new `finding` contradicts while it still counts:
/// a rule that did not fire, or one the spec addressed, and now fires against
/// the same subjects, at a location naming no section (AC3), since one at a
/// section is [`Standing::demoted`]. Findings against findings, and any
/// verdict against a stale rule or one owed a judgment of changed sections,
/// contradict nothing.
pub(crate) fn contradiction(earlier: &Currency, now: &RuleVerdict) -> Option<&'static str> {
    if !matches!(now, RuleVerdict::Finding { .. }) {
        return None;
    }
    match earlier {
        Currency::NotFired => Some("not-fired"),
        Currency::Fired(RuleVerdict::Addressed { .. }) => Some("addressed"),
        Currency::Fired(RuleVerdict::Finding { .. } | RuleVerdict::Waived { .. })
        | Currency::Rejudge { .. }
        | Currency::Stale => None,
    }
}

/// The well-formed departure entries of a spec's `content`, by rule: the
/// only entries that waive anything. A rule two entries name is waived by
/// the first, as `check-departures` reports the second as a blocking finding.
pub(crate) fn departure_entries(content: &str) -> BTreeMap<String, departures::Entry> {
    let mut by_rule = BTreeMap::new();
    for entry in departures::parse(content).entries {
        by_rule.entry(entry.rule.clone()).or_insert(entry);
    }
    by_rule
}

// How many times this thread's `fill_waived` read history, so a test can
// observe that a write holding no `waived` verdict reads none.
#[cfg(test)]
thread_local! {
    pub(crate) static HISTORY_READS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Fill every `waived` verdict of `verdicts` from its departure entry and
/// the entry's history attribution (spec 073): the host's bare verdict and a
/// carried one alike, so no field is fabricated and none outlives the history
/// it was derived from. `spec_rel` is the spec's `/`-separated path from the
/// project root. History is read only when a `waived` verdict is held.
pub(crate) fn fill_waived(
    repo: &Path,
    spec_rel: &str,
    entries: &BTreeMap<String, departures::Entry>,
    verdicts: &mut BTreeMap<String, RuleVerdict>,
) {
    // A `waived` verdict is filled from its entry, so one with no entry is not a
    // candidate: `write-analysis` records a carried one whose entry is gone unjudged
    // before it gets here.
    let waived: Vec<(String, &departures::Entry)> = verdicts
        .iter()
        .filter(|(_, verdict)| matches!(verdict, RuleVerdict::Waived { .. }))
        .filter_map(|(rule, _)| entries.get(rule).map(|entry| (rule.clone(), entry)))
        .collect();
    if waived.is_empty() {
        return;
    }
    #[cfg(test)]
    HISTORY_READS.with(|reads| reads.set(reads.get() + 1));
    let rules: Vec<String> = waived.iter().map(|(rule, _)| rule.clone()).collect();
    let attribution = departures::attribute(repo, spec_rel, &rules);
    for (rule, entry) in waived {
        let (decided_by, decided_at) = attribution
            .get(&rule)
            .map_or_else(|| Attribution::Unattributed.decided(), Attribution::decided);
        verdicts.insert(
            rule,
            RuleVerdict::Waived {
                reason: entry.reason.clone(),
                decided_by,
                decided_at,
                cited: entry.cited.clone(),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::analyze_subjects::{is_analyze_subject, subject_digest};
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
    /// subjects' digest, the rule files' digests, the design subjects'
    /// sections, and `extra` (verdicts and the unjudged list, as frontmatter
    /// text).
    fn record(dir: &Path, files: &[LoadedRuleFile], extra: &str) {
        let mut sections = String::from("section-digest:\n");
        for (key, digest) in section_digest(dir) {
            let _ = writeln!(sections, "  {key:?}: {digest}");
        }
        record_before_sections(dir, files, &format!("{sections}{extra}"));
    }

    /// A prior `analysis.md` as `write-analysis` wrote it before AC7, with no
    /// `section-digest`.
    fn record_before_sections(dir: &Path, files: &[LoadedRuleFile], extra: &str) {
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
        let standing = standing(tmp.path(), tmp.path(), &loaded);
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

    /// Against a record written before AC7, a design subject's edit judges
    /// every rule again, whichever subject it is, and a `tasks.md` or
    /// `review.md` edit judges none (AC1, the Resolved Questions).
    #[test]
    fn a_design_subject_edit_judges_every_rule_and_a_tasks_or_review_edit_none() {
        let loaded = files(MUST_RULES, SHOULD_RULES);
        for edited in ["spec.md", "plan.md", "scenarios/s.md"] {
            let tmp = feature();
            record_before_sections(tmp.path(), &loaded, ONE_FIRED);
            fs::write(tmp.path().join(edited), "# Edited\n").unwrap();
            let standing = standing(tmp.path(), tmp.path(), &loaded);
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
        let standing = standing(tmp.path(), tmp.path(), &loaded);
        assert_eq!(standing.no_reuse, None);
        assert_eq!(standing.judge().count(), 0);
    }

    /// A design subject added (a first `data-model.md`) or removed (a
    /// deleted scenario) adds or removes its sections, so every quiet rule is
    /// judged against them (AC7); against a record written before AC7 the
    /// changed set judges every rule in full.
    #[test]
    fn a_design_subject_added_or_removed_judges_every_rule_against_its_sections() {
        let loaded = files(MUST_RULES, SHOULD_RULES);
        let tmp = feature();
        record(tmp.path(), &loaded, "");
        fs::write(tmp.path().join("data-model.md"), "# Data\n").unwrap();
        let added = standing(tmp.path(), tmp.path(), &loaded);
        assert_eq!(added.no_reuse, None);
        assert_eq!(added.rejudge().count(), 3);
        assert_eq!(
            added.changed_sections().collect::<Vec<_>>(),
            ["data-model.md", "data-model.md#Data"]
        );

        let tmp = feature();
        record(tmp.path(), &loaded, "");
        fs::remove_file(tmp.path().join("scenarios/s.md")).unwrap();
        let removed = standing(tmp.path(), tmp.path(), &loaded);
        assert_eq!(removed.rejudge().count(), 3);
        assert_eq!(
            removed.changed_sections().collect::<Vec<_>>(),
            [
                "scenarios/s.md",
                "scenarios/s.md#Context",
                "scenarios/s.md#S"
            ]
        );

        let tmp = feature();
        record_before_sections(tmp.path(), &loaded, "");
        fs::remove_file(tmp.path().join("scenarios/s.md")).unwrap();
        let standing = standing(tmp.path(), tmp.path(), &loaded);
        assert_eq!(standing.no_reuse, Some(NoReuse::SubjectsChanged));
        assert!(all_stale(&standing));
    }

    /// AC7: an edit to one section leaves each verdict standing for the
    /// others. A quiet rule and an `addressed` one are judged against the
    /// changed section alone, the latter carried only while its cite's
    /// section is unchanged; a `waived` one is carried; a finding at an
    /// unchanged section stands unjudged, and one at the changed section is
    /// judged in full.
    #[test]
    fn a_section_edit_judges_each_rule_as_its_verdict_says() {
        let tmp = tempdir().unwrap();
        fs::write(tmp.path().join("spec.md"), "# Spec\n\n## Goal\n\nOne.\n").unwrap();
        fs::write(
            tmp.path().join("plan.md"),
            "# Plan\n\n## Observability\n\nA.\n\n## Rollout\n\nB.\n",
        )
        .unwrap();
        let five = "### TS-MUST-001\n\n> A MUST b.\n\n**Verification:** v.\n\n\
                    ### TS-MUST-002\n\n> A MUST c.\n\n**Verification:** v.\n\n\
                    ### TS-MUST-003\n\n> A MUST d.\n\n**Verification:** v.\n\n\
                    ### TS-MUST-004\n\n> A MUST e.\n\n**Verification:** v.\n\n\
                    ### TS-MUST-005\n\n> A MUST f.\n\n**Verification:** v.\n\n\
                    ### TS-MUST-006\n\n> A MUST g.\n\n**Verification:** v.\n";
        let loaded = vec![LoadedRuleFile::read("a-cross.md", five)];
        record(
            tmp.path(),
            &loaded,
            "rule-verdicts:\n  \
             TS-MUST-002: {verdict: addressed, cite: \"plan.md#Observability\"}\n  \
             TS-MUST-003: {verdict: addressed, cite: \"plan.md#Rollout\"}\n  \
             TS-MUST-004: {verdict: waived, reason: \"r\", decided-by: \"x\"}\n  \
             TS-MUST-005: {verdict: finding, tier: must, at: \"spec.md#Goal\", message: \"m\"}\n  \
             TS-MUST-006: {verdict: finding, tier: must, at: \"plan.md#Rollout\", message: \"m\"}\n",
        );
        fs::write(
            tmp.path().join("plan.md"),
            "# Plan\n\n## Observability\n\nA.\n\n## Rollout\n\nC.\n",
        )
        .unwrap();
        let standing = standing(tmp.path(), tmp.path(), &loaded);
        assert_eq!(standing.no_reuse, None);
        assert_eq!(
            standing.changed_sections().collect::<Vec<_>>(),
            ["plan.md#Rollout"]
        );
        let rejudged = |carried: bool| move |c: &Currency| matches!(c, Currency::Rejudge { carries, .. } if *carries == carried);
        let of = |id: &str| standing.currency(id).unwrap();
        assert_eq!(
            of("TS-MUST-001"),
            &Currency::Rejudge {
                earlier: None,
                carries: false
            }
        );
        assert!(rejudged(true)(of("TS-MUST-002")));
        assert!(rejudged(false)(of("TS-MUST-003")));
        assert!(rejudged(true)(of("TS-MUST-004")));
        assert!(matches!(
            of("TS-MUST-005"),
            Currency::Fired(RuleVerdict::Finding { .. })
        ));
        assert_eq!(of("TS-MUST-006"), &Currency::Stale);
        let ids = |rules: Vec<&AssessableRule>| -> Vec<String> {
            rules.into_iter().map(|rule| rule.id.clone()).collect()
        };
        assert_eq!(ids(standing.judge().collect()), ["TS-MUST-006"]);
        assert_eq!(
            ids(standing.rejudge().collect()),
            ["TS-MUST-001", "TS-MUST-002", "TS-MUST-003", "TS-MUST-004"]
        );
        let reusable: Vec<&str> = standing
            .reusable()
            .map(|(rule, _)| rule.id.as_str())
            .collect();
        assert_eq!(reusable, ["TS-MUST-002", "TS-MUST-004", "TS-MUST-005"]);

        // A finding at an unchanged section after a quiet or addressed
        // verdict is demoted; at the changed section, or naming no section,
        // or after a finding, it is not.
        let finding = |at: &str| RuleVerdict::Finding {
            tier: FindingTier::Must,
            at: at.into(),
            message: "m".into(),
        };
        assert_eq!(
            standing.demoted("TS-MUST-001", &finding("spec.md#Goal")),
            Some("not-fired")
        );
        assert_eq!(
            standing.demoted("TS-MUST-003", &finding("plan.md#Observability")),
            Some("addressed")
        );
        assert_eq!(
            standing.demoted("TS-MUST-001", &finding("plan.md#Rollout")),
            None
        );
        assert_eq!(standing.demoted("TS-MUST-001", &finding("plan.md")), None);
        assert_eq!(
            standing.demoted("TS-MUST-001", &finding("plan.md#Missing")),
            None
        );
        assert_eq!(
            standing.demoted("TS-MUST-005", &finding("spec.md#Goal")),
            None
        );
        assert_eq!(
            standing.demoted("TS-MUST-004", &finding("spec.md#Goal")),
            None
        );
    }

    /// With no copy of the file the run judged, here a record naming no
    /// `analyzed-against`, a rule file whose rules changed judges every rule of
    /// that file again (spec 088) and leaves the other files' current (AC5).
    #[test]
    fn an_edited_rule_file_judges_its_own_rules_only() {
        let tmp = feature();
        record(tmp.path(), &files(MUST_RULES, SHOULD_RULES), ONE_FIRED);
        let edited = files(
            &MUST_RULES.replace("Check two.", "Check two, and three."),
            SHOULD_RULES,
        );
        let standing = standing(tmp.path(), tmp.path(), &edited);
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
            standing(tmp.path(), tmp.path(), &before)
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
        let standing = standing(tmp.path(), tmp.path(), &after);
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

        let same = standing(tmp.path(), tmp.path(), &before);
        assert_eq!(same.no_reuse, None);
        assert_eq!(same.reusable().count(), 1);
        assert_eq!(same.judge().count(), 0);

        let mut added = before.clone();
        added.push(LoadedRuleFile::read("c-cross.md", "No rules here.\n"));
        let added = standing(tmp.path(), tmp.path(), &added);
        assert_eq!(added.no_reuse, Some(NoReuse::RuleFilesChanged));
        assert_eq!(
            added.no_reuse.as_ref().unwrap().code(),
            "rule-files-changed"
        );
        assert_eq!(added.reusable().count(), 0);
        assert!(all_stale(&added));

        let removed = standing(tmp.path(), tmp.path(), &before[..1]);
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
        let standing = standing(tmp.path(), tmp.path(), &only_should);
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
        let absent = standing(tmp.path(), tmp.path(), &loaded);
        assert_eq!(absent.no_reuse, Some(NoReuse::NoPriorVerdicts));
        assert!(all_stale(&absent));

        record(tmp.path(), &[], "");
        assert_eq!(
            standing(tmp.path(), tmp.path(), &loaded).no_reuse,
            Some(NoReuse::NoPriorVerdicts),
            "a record written before spec 082 has no rule-digest"
        );

        record(tmp.path(), &loaded, "rule-verdicts: [TS-MUST-001]\n");
        let unreadable = standing(tmp.path(), tmp.path(), &loaded);
        assert!(
            matches!(unreadable.no_reuse, Some(NoReuse::RecordUnreadable(_))),
            "{:?}",
            unreadable.no_reuse
        );
        assert_eq!(unreadable.no_reuse.unwrap().code(), "record-unreadable");
        assert!(all_stale(&standing(tmp.path(), tmp.path(), &loaded)));
    }

    /// A design subject the record could not read, or the walk cannot reach
    /// now, is not a matching one.
    #[test]
    fn an_unreadable_design_subject_is_a_changed_one() {
        let loaded = files(MUST_RULES, SHOULD_RULES);
        let tmp = feature();
        record(tmp.path(), &loaded, "analyzed-unreadable:\n  - plan.md\n");
        assert_eq!(
            standing(tmp.path(), tmp.path(), &loaded).no_reuse,
            Some(NoReuse::SubjectsChanged)
        );
        record(tmp.path(), &loaded, "analyzed-unreadable:\n  - scenarios\n");
        assert_eq!(
            standing(tmp.path(), tmp.path(), &loaded).no_reuse,
            Some(NoReuse::SubjectsChanged),
            "a directory the walk could not reach hides its files"
        );
        record(tmp.path(), &loaded, "analyzed-unreadable:\n  - tasks.md\n");
        assert_eq!(standing(tmp.path(), tmp.path(), &loaded).no_reuse, None);
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
        assert_eq!(
            universe.files[0].digest,
            crate::primitives::rule_sections::rules_digest(MUST_RULES)
        );
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
        let same = standing(tmp.path(), tmp.path(), &before);
        assert_eq!(same.rules().len(), 1);
        assert_eq!(same.currency("TS-MUST-001"), Some(&Currency::NotFired));

        let edited = vec![
            before[0].clone(),
            LoadedRuleFile::read(
                "b-cross.md",
                &twice.replace("Check one.", "Check one, two."),
            ),
        ];
        let changed = standing(tmp.path(), tmp.path(), &edited);
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

    /// A citation into a link is not into a file of the feature, whether the
    /// link is the file or a directory on the way to it (`BE-INPUT-004`).
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
        std::os::unix::fs::symlink(outside.path(), tmp.path().join("scenarios/ln")).unwrap();
        assert!(!cite_resolves(tmp.path(), "scenarios/ln/o.md#Heading"));
        assert!(cite_resolves(tmp.path(), "scenarios/s.md#Context"));
    }

    /// AC7: sections that only move change a file's digest and no section's,
    /// so every verdict stays current and nothing is owed a judgment.
    #[test]
    fn a_reordering_of_sections_leaves_every_verdict_current() {
        let loaded = files(MUST_RULES, SHOULD_RULES);
        let tmp = feature();
        fs::write(
            tmp.path().join("plan.md"),
            "# Plan\n\n## A\n\nx\n\n## B\n\ny\n\n",
        )
        .unwrap();
        record(tmp.path(), &loaded, ONE_FIRED);
        fs::write(
            tmp.path().join("plan.md"),
            "# Plan\n\n## B\n\ny\n\n## A\n\nx\n\n",
        )
        .unwrap();
        let standing = standing(tmp.path(), tmp.path(), &loaded);
        assert_eq!(standing.no_reuse, None);
        assert_eq!(standing.changed_sections().count(), 0);
        assert_eq!(standing.judge().count(), 0);
        assert_eq!(standing.rejudge().count(), 0);
        assert_eq!(standing.reusable().count(), 1);
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

    // -- rule-by-rule currency through history (spec 088, AC2) ------------

    const M1: &str = "### TS-MUST-001\n\n> A MUST b.\n\n**Verification:** Check one.\n";
    const M2: &str =
        "### TS-MUST-002\n\n> A MUST c, as TS-MUST-001 asks.\n\n**Verification:** Check two.\n";
    const M3: &str =
        "### TS-MUST-003\n\n> A MUST d.\n\n**Verification:** Check three, as TS-MUST-002 does.\n";
    const S1: &str =
        "### TS-SHOULD-001\n\n> A SHOULD e, beside TS-MUST-001.\n\n**Verification:** Check four.\n";
    const S2: &str = "### TS-SHOULD-002\n\n> A SHOULD f.\n\n**Verification:** Check five.\n";
    const BOTH: [&str; 2] = ["a-cross.md", "b-cross.md"];

    /// `TS-MUST-002` names `TS-MUST-001` in its own file and `TS-SHOULD-001`
    /// names it from another; `TS-MUST-003` names it only through
    /// `TS-MUST-002`.
    fn named_a() -> String {
        [M1, M2, M3].join("\n")
    }

    fn named_b() -> String {
        [S1, S2].join("\n")
    }

    /// The rule files `rules` under `project`'s `specs/rules/` and the feature
    /// `specs/001-x`, committed in `repository`: the feature directory and
    /// the commit.
    fn committed(
        repository: &git2::Repository,
        project: &Path,
        rules: &[(&str, String)],
    ) -> (std::path::PathBuf, git2::Oid) {
        use crate::primitives::git_fixture::{commit_all, write};
        let feature = project.join("specs/001-x");
        write(&feature.join("spec.md"), "# Spec\n\n## Goal\n");
        write(&feature.join("plan.md"), "# Plan\n\n## Observability\n");
        for (name, body) in rules {
            write(&project.join("specs/rules").join(name), body);
        }
        (feature, commit_all(repository, "rules"))
    }

    /// The rule files `names` as they stand under `project`'s `specs/rules/`.
    fn loaded_from(project: &Path, names: &[&str]) -> Vec<LoadedRuleFile> {
        names
            .iter()
            .map(|name| {
                let rel = format!("specs/rules/{name}");
                LoadedRuleFile::read(&rel, &fs::read_to_string(project.join(&rel)).unwrap())
            })
            .collect()
    }

    fn edit(project: &Path, name: &str, body: &str) {
        crate::primitives::git_fixture::write(&project.join("specs/rules").join(name), body);
    }

    fn judged_ids(standing: &Standing) -> Vec<&str> {
        standing.judge().map(|rule| rule.id.as_str()).collect()
    }

    /// The record's `analyzed-against` line, as frontmatter text.
    fn at(commit: impl std::fmt::Display) -> String {
        format!("analyzed-against: {commit}\n")
    }

    /// A repository at a fresh temporary directory holding both files, with a
    /// record of a run at that commit.
    fn analyzed_project() -> (TempDir, git2::Repository, std::path::PathBuf) {
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let (feature, commit) = committed(
            &repository,
            tmp.path(),
            &[("a-cross.md", named_a()), ("b-cross.md", named_b())],
        );
        record(&feature, &loaded_from(tmp.path(), &BOTH), &at(commit));
        (tmp, repository, feature)
    }

    /// Spec 088, AC2: an edit to one rule's Verification judges that rule
    /// and each rule naming it, in its own file and in another, and no
    /// other: a rule naming it only through another keeps its verdict.
    #[test]
    fn an_edit_to_one_rule_judges_it_and_each_rule_naming_it_alone() {
        let (tmp, _repository, feature) = analyzed_project();
        edit(
            tmp.path(),
            "a-cross.md",
            &named_a().replace("Check one.", "Check one, and two."),
        );
        let standing = standing(tmp.path(), &feature, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(standing.no_reuse, None);
        assert_eq!(standing.rejudged_files(), []);
        assert_eq!(
            judged_ids(&standing),
            ["TS-MUST-001", "TS-MUST-002", "TS-SHOULD-001"]
        );
        assert_eq!(standing.currency("TS-MUST-003"), Some(&Currency::NotFired));
        assert_eq!(
            standing.currency("TS-SHOULD-002"),
            Some(&Currency::NotFired)
        );
    }

    /// Spec 088, AC2: a rule its file no longer holds stales each rule naming
    /// it, and a file whose copy history cannot supply is judged whole, each
    /// rule naming one of its rules with it.
    #[test]
    fn a_removed_rule_or_a_file_judged_whole_judges_each_rule_naming_one() {
        let (tmp, _repository, feature) = analyzed_project();
        let original = loaded_from(tmp.path(), &BOTH);
        edit(tmp.path(), "a-cross.md", &[M2, M3].join("\n"));
        let removed = standing(tmp.path(), &feature, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(removed.rejudged_files(), []);
        assert_eq!(judged_ids(&removed), ["TS-MUST-002", "TS-SHOULD-001"]);
        assert_eq!(removed.currency("TS-MUST-003"), Some(&Currency::NotFired));

        // A sha no commit of the repository has.
        record(&feature, &original, &at("0".repeat(40)));
        edit(
            tmp.path(),
            "a-cross.md",
            &named_a().replace("Check one.", "Check one, and two."),
        );
        let whole = standing(tmp.path(), &feature, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(
            whole.rejudged_files(),
            [("a-cross.md".to_string(), CopyUnavailable::CommitNotHeld)]
        );
        assert_eq!(
            judged_ids(&whole),
            ["TS-MUST-001", "TS-MUST-002", "TS-MUST-003", "TS-SHOULD-001"]
        );
        assert_eq!(whole.currency("TS-SHOULD-002"), Some(&Currency::NotFired));
    }

    /// Spec 088, AC2: a file judged whole cannot say which rules it held, so
    /// a rule in another file naming an ID no loaded rule defines, one the
    /// file may no longer hold, is judged with it, and a rule naming a defined
    /// rule nothing changed, or a shipped rule no loaded file defines, is not;
    /// while every changed file's copy stands in, such a name stales nothing.
    #[test]
    fn a_file_judged_whole_judges_each_rule_naming_an_id_no_loaded_rule_defines() {
        const S3: &str = "### TS-SHOULD-003\n\n> A SHOULD g, beside TS-MUST-099.\n\n**Verification:** Check six.\n";
        const S4: &str = "### TS-SHOULD-004\n\n> A SHOULD h, beside TS-SHOULD-002.\n\n**Verification:** Check seven.\n";
        // `BE-AUTHN-001` is shipped, and no file of the fixture defines it.
        const S5: &str = "### TS-SHOULD-005\n\n> A SHOULD i, beside BE-AUTHN-001.\n\n**Verification:** Check eight.\n";
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let (feature, commit) = committed(
            &repository,
            tmp.path(),
            &[
                ("a-cross.md", named_a()),
                ("b-cross.md", [S1, S2, S3, S4, S5].join("\n")),
            ],
        );
        let original = loaded_from(tmp.path(), &BOTH);
        record(&feature, &original, &at(commit));
        edit(tmp.path(), "a-cross.md", &[M2, M3].join("\n"));
        let narrowed = standing(tmp.path(), &feature, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(narrowed.rejudged_files(), []);
        assert_eq!(judged_ids(&narrowed), ["TS-MUST-002", "TS-SHOULD-001"]);
        assert_eq!(
            narrowed.currency("TS-SHOULD-003"),
            Some(&Currency::NotFired)
        );

        // A sha no commit of the repository has: nothing says `TS-MUST-001`
        // was `a-cross.md`'s, so `TS-SHOULD-001`, naming it, is judged, and so
        // is `TS-SHOULD-003`, naming an ID no rule file ever held.
        record(&feature, &original, &at("0".repeat(40)));
        let whole = standing(tmp.path(), &feature, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(
            whole.rejudged_files(),
            [("a-cross.md".to_string(), CopyUnavailable::CommitNotHeld)]
        );
        assert_eq!(
            judged_ids(&whole),
            [
                "TS-MUST-002",
                "TS-MUST-003",
                "TS-SHOULD-001",
                "TS-SHOULD-003"
            ]
        );
        assert_eq!(whole.currency("TS-SHOULD-002"), Some(&Currency::NotFired));
        assert_eq!(whole.currency("TS-SHOULD-004"), Some(&Currency::NotFired));
        assert_eq!(whole.currency("TS-SHOULD-005"), Some(&Currency::NotFired));
    }

    /// Spec 088, AC2: a shallow clone whose history stops after the
    /// record's `analyzed-against` cannot supply the judged copy, so the
    /// edited file is judged whole, naming why; the full repository judges
    /// the edited rule and its namers alone.
    #[test]
    fn a_shallow_clone_past_analyzed_against_judges_the_whole_file() {
        let tmp = tempdir().unwrap();
        let origin = tmp.path().join("origin");
        let repository = git2::Repository::init(&origin).unwrap();
        let (feature, commit) = committed(
            &repository,
            &origin,
            &[("a-cross.md", named_a()), ("b-cross.md", named_b())],
        );
        record(&feature, &loaded_from(&origin, &BOTH), &at(commit));
        edit(
            &origin,
            "a-cross.md",
            &named_a().replace("Check one.", "Check one, and two."),
        );
        crate::primitives::git_fixture::commit_all(&repository, "an edit");
        let clone = tmp.path().join("clone");
        let status = std::process::Command::new("git")
            .args(["clone", "--quiet", "--depth", "1"])
            .arg(format!("file://{}", origin.display()))
            .arg(&clone)
            .status()
            .unwrap();
        assert!(status.success());
        assert!(git2::Repository::open(&clone).unwrap().is_shallow());
        let shallow = standing(
            &clone,
            &clone.join("specs/001-x"),
            &loaded_from(&clone, &BOTH),
        );
        assert_eq!(
            shallow.rejudged_files(),
            [("a-cross.md".to_string(), CopyUnavailable::CommitNotHeld)]
        );
        assert_eq!(
            judged_ids(&shallow),
            ["TS-MUST-001", "TS-MUST-002", "TS-MUST-003", "TS-SHOULD-001"]
        );
        let full = standing(&origin, &feature, &loaded_from(&origin, &BOTH));
        assert_eq!(
            judged_ids(&full),
            ["TS-MUST-001", "TS-MUST-002", "TS-SHOULD-001"]
        );
    }

    /// Spec 088, AC2's guard: a run that judged rule text its
    /// `analyzed-against` does not hold, an uncommitted edit since reverted,
    /// leaves a copy at that commit that does not digest as the record says,
    /// so the file is judged whole rather than carrying a verdict no run gave
    /// the text that stands.
    #[test]
    fn a_copy_at_analyzed_against_that_is_not_the_text_judged_judges_the_whole_file() {
        let (tmp, repository, feature) = analyzed_project();
        let commit = repository.head().unwrap().target().unwrap();
        let mut judged = loaded_from(tmp.path(), &BOTH);
        judged[0] = LoadedRuleFile::read(
            "specs/rules/a-cross.md",
            &named_a().replace("Check two.", "Check two, uncommitted."),
        );
        record(&feature, &judged, &at(commit));
        let standing = standing(tmp.path(), &feature, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(
            standing.rejudged_files(),
            [("a-cross.md".to_string(), CopyUnavailable::JudgedCopyDiffers)]
        );
        assert_eq!(
            judged_ids(&standing),
            ["TS-MUST-001", "TS-MUST-002", "TS-MUST-003", "TS-SHOULD-001"]
        );
    }

    /// Spec 088's Edge Cases: a record naming no `analyzed-against`, a commit
    /// that does not hold the file, and a project in no repository each judge
    /// the changed file whole, naming why.
    #[test]
    fn a_copy_history_cannot_supply_is_named_with_its_reason() {
        let (tmp, _repository, analyzed) = analyzed_project();
        record(&analyzed, &loaded_from(tmp.path(), &BOTH), "");
        edit(
            tmp.path(),
            "a-cross.md",
            &named_a().replace("Check one.", "Check one, and two."),
        );
        let unnamed = standing(tmp.path(), &analyzed, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(
            unnamed.rejudged_files(),
            [("a-cross.md".to_string(), CopyUnavailable::NoAnalyzedAgainst)]
        );

        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let (one_file, commit) = committed(&repository, tmp.path(), &[("a-cross.md", named_a())]);
        edit(tmp.path(), "b-cross.md", &named_b());
        record(&one_file, &loaded_from(tmp.path(), &BOTH), &at(commit));
        edit(
            tmp.path(),
            "b-cross.md",
            &named_b().replace("Check five.", "Check six."),
        );
        let uncommitted = standing(tmp.path(), &one_file, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(
            uncommitted.rejudged_files(),
            [("b-cross.md".to_string(), CopyUnavailable::NotAtCommit)]
        );
        assert_eq!(judged_ids(&uncommitted), ["TS-SHOULD-001", "TS-SHOULD-002"]);

        let tmp = feature();
        record(tmp.path(), &files(MUST_RULES, SHOULD_RULES), &at("abc"));
        let edited = files(
            &MUST_RULES.replace("Check two.", "Check two, and three."),
            SHOULD_RULES,
        );
        let unreadable = standing(tmp.path(), tmp.path(), &edited);
        assert_eq!(
            unreadable.rejudged_files(),
            [("a-cross.md".to_string(), CopyUnavailable::HistoryUnreadable)]
        );
    }

    /// Spec 088, AC2 and its Edge Cases: rules reordered keep every verdict,
    /// and a rule moved in from another file is judged, as a rule new to its
    /// file is.
    #[test]
    fn a_reorder_keeps_every_verdict_and_a_rule_moved_in_is_judged() {
        let (tmp, _repository, feature) = analyzed_project();
        edit(tmp.path(), "a-cross.md", &[M3, M1, M2].join("\n"));
        let reordered = standing(tmp.path(), &feature, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(reordered.rejudged_files(), []);
        assert_eq!(reordered.judge().count(), 0);

        edit(tmp.path(), "a-cross.md", &[M1, M2, M3, S2].join("\n"));
        edit(tmp.path(), "b-cross.md", S1);
        let moved = standing(tmp.path(), &feature, &loaded_from(tmp.path(), &BOTH));
        assert_eq!(moved.rejudged_files(), []);
        assert_eq!(judged_ids(&moved), ["TS-SHOULD-002"]);
    }

    /// Spec 059's conversion: a project in a subdirectory of its repository
    /// finds a rule file's copy under the project's place in the work tree.
    #[test]
    fn a_project_in_a_subdirectory_reads_its_rule_file_s_copy() {
        let tmp = tempdir().unwrap();
        let (repository, project) =
            crate::primitives::git_fixture::subdirectory_project(tmp.path());
        let (feature, commit) = committed(
            &repository,
            &project,
            &[("a-cross.md", named_a()), ("b-cross.md", named_b())],
        );
        record(&feature, &loaded_from(&project, &BOTH), &at(commit));
        edit(
            &project,
            "a-cross.md",
            &named_a().replace("Check one.", "Check one, and two."),
        );
        let standing = standing(&project, &feature, &loaded_from(&project, &BOTH));
        assert_eq!(standing.rejudged_files(), []);
        assert_eq!(
            judged_ids(&standing),
            ["TS-MUST-001", "TS-MUST-002", "TS-SHOULD-001"]
        );
    }

    /// A rule's names are the other rule IDs its Statement and Verification
    /// hold, never its own, and prose outside both is not read.
    #[test]
    fn a_rule_s_names_are_the_other_ids_its_statement_and_verification_hold() {
        let file = LoadedRuleFile::read(
            "specs/rules/a-cross.md",
            "### TS-MUST-009\n\n> A MUST b, as TS-MUST-001 and TS-MUST-009 say.\n\n\
             **Verification:** Check, beside TS-SHOULD-002.\n\n**Rationale:** TS-MUST-003.\n",
        );
        assert_eq!(
            file.rules[0]
                .names
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["TS-MUST-001", "TS-SHOULD-002"]
        );
        assert_eq!(file.name, "a-cross.md");
        assert_eq!(file.path, "specs/rules/a-cross.md");
    }
}
