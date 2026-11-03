//! Stored finding decisions — the `decisions:` list both audit records carry
//! beside their counts (spec 058).
//!
//! A finding deferred or waived in one run fires again in the next, because
//! detection is stateless: a deferred finding keeps firing until its work
//! lands, and a waived one keeps firing forever. Without a stored decision
//! every re-run would ask again, and a deferred finding would come back
//! undispositioned. So each deferral or waiver is stored in the record of the
//! command that made it, and a later run counts a matching finding under the
//! stored outcome without asking — unless a deferral's named work is complete
//! while its finding still fires, when `process-decisions` reports it lapsed
//! (spec 076).
//!
//! The lifecycle mirrors review waivers (`framework/commands/review.md`,
//! §Per-run waiver processing): matched while the key fires, expired when an
//! unrestricted run no longer produces it, retained when the run did not
//! evaluate its source. An entry missing a required field or its companion, or
//! carrying a field its outcome forbids, is reported and never pruned; an entry that does not deserialize at all fails
//! the whole list (see [`RawDecision`]).

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use crate::primitives::{PrimitiveError, Result, flatten_line};
use crate::schema::primitives::{
    DecisionOutcome, DecisionRef, Disposition, DispositionOutcome, Dispositions,
};

/// The frontmatter key both records store decisions under.
pub(crate) const DECISIONS_KEY: &str = "decisions";

/// One stored decision with every known field optional — each a string, the
/// derived `gating` a bool — so an entry missing a field still deserializes
/// and [`Self::to_ref`] reports it as a notice, and so a re-render preserves
/// the full entry. An entry that is not a mapping, or a known field holding
/// the wrong type (`reason: [a, b]`, `gating: "yes"`), does not deserialize, and that fails the whole list rather than
/// the one entry. Unknown adopter-authored fields, of any type, are captured
/// by `extra` and re-emitted verbatim, the §text-first-artifacts open-schema
/// rule the waiver list already follows (`RawWaiverFull`).
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub(crate) struct RawDecision {
    #[serde(default)]
    pub(crate) key: Option<String>,
    #[serde(default)]
    pub(crate) outcome: Option<String>,
    #[serde(default)]
    pub(crate) target: Option<String>,
    #[serde(default)]
    pub(crate) task: Option<String>,
    #[serde(default, rename = "work-item")]
    pub(crate) work_item: Option<String>,
    #[serde(default)]
    pub(crate) reason: Option<String>,
    /// Derived on every write, never authored: `true` on a deferral of a
    /// gating finding that does not release it (spec 076), which
    /// `check-review-gate` reads to name what holds `done`.
    #[serde(default)]
    pub(crate) gating: Option<bool>,
    #[serde(default, rename = "decided-at")]
    pub(crate) decided_at: Option<String>,
    #[serde(default, rename = "decided-by")]
    pub(crate) decided_by: Option<String>,
    #[serde(flatten)]
    pub(crate) extra: BTreeMap<String, serde_norway::Value>,
}

impl RawDecision {
    /// The entry as a [`DecisionRef`], or — when it is unusable — the first
    /// field that makes it so, named as the field to fix.
    ///
    /// An outcome's companion is required with it — a deferral names its work,
    /// a target or a work item or both; a waiver states its reason and names
    /// no work — because an entry missing it records a decision nobody can
    /// audit, which is no better than no decision.
    ///
    /// `routed` and `discarded` are read as `deferred` and `waived`: they are
    /// the names a record written before spec 076 carries, and it must keep
    /// parsing until its writer next runs and renders the new ones. Nothing
    /// writes the old names.
    pub(crate) fn to_ref(&self) -> std::result::Result<DecisionRef, &'static str> {
        let present = |value: &Option<String>| nonblank(value.as_deref()).map(str::to_string);
        let key = present(&self.key).ok_or("key")?;
        let outcome = match self.outcome.as_deref().map(str::trim) {
            Some("deferred" | "routed") => DecisionOutcome::Deferred,
            Some("waived" | "discarded") => DecisionOutcome::Waived,
            _ => return Err("outcome"),
        };
        present(&self.decided_at).ok_or("decided-at")?;
        present(&self.decided_by).ok_or("decided-by")?;
        let (target, task, work_item) = (
            present(&self.target),
            present(&self.task),
            present(&self.work_item),
        );
        let reason = match outcome {
            DecisionOutcome::Deferred => {
                if target.is_none() && work_item.is_none() {
                    return Err("target");
                }
                if task.is_some() && !names_a_tasks_file(target.as_deref()) {
                    return Err("task");
                }
                present(&self.reason)
            }
            DecisionOutcome::Waived => {
                // A waiver names no future work (spec 076).
                if target.is_some() {
                    return Err("target");
                }
                if task.is_some() {
                    return Err("task");
                }
                if work_item.is_some() {
                    return Err("work-item");
                }
                Some(present(&self.reason).ok_or("reason")?)
            }
        };
        Ok(DecisionRef {
            key,
            outcome,
            target,
            task,
            work_item,
            reason,
        })
    }
}

/// Whether `target` is a `tasks.md`, the only target a deferral's `task` can
/// name a task in.
fn names_a_tasks_file(target: Option<&str>) -> bool {
    target.is_some_and(|target| {
        std::path::Path::new(target.trim()).file_name() == Some("tasks.md".as_ref())
    })
}

/// `value` when it holds more than whitespace, as it was written. The one
/// blank test the decision readers and both writers share.
pub(crate) fn nonblank(value: Option<&str>) -> Option<&str> {
    value.filter(|text| !text.trim().is_empty())
}

/// Check that `disposition` carries the companions its outcome requires — a
/// deferral names its work (a target, a work item, or both), a waiver states
/// its reason and names no work — the one check both writers apply (spec
/// 076). `at` is the disposition's argument path
/// (`observations[0].disposition`) and `primitive` the writer, for the
/// refusal. Returns each companion given but the work-item ID, trimmed, with
/// its argument path, so a writer can screen it further; empty for an outcome
/// that takes none. The ID is not returned because it is stored exactly as
/// given: it is screened here for the one thing that would break the record,
/// a line break.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] naming `{at}.target` when a deferral
/// names neither a target nor a work item; `{at}.work-item` when a work-item
/// ID spans more than one line; `{at}.task` when a task has no `tasks.md`
/// target; `{at}.target`, `{at}.task` or `{at}.work-item` when a waiver names
/// work; and `{at}.reason` when a waiver states no reason.
pub(crate) fn require_companion<'a>(
    primitive: &str,
    at: &str,
    disposition: &'a Disposition,
) -> Result<Vec<(String, &'a str)>> {
    let refuse = |field: &str, reason: &str| PrimitiveError::InvalidArgument {
        primitive: primitive.into(),
        argument: format!("{at}.{field}"),
        reason: reason.into(),
    };
    let target = nonblank(disposition.target.as_deref());
    let task = nonblank(disposition.task.as_deref());
    let work_item = nonblank(disposition.work_item.as_deref());
    let reason = nonblank(disposition.reason.as_deref());
    match disposition.outcome {
        DispositionOutcome::Deferred => {
            if target.is_none() && work_item.is_none() {
                return Err(refuse(
                    "target",
                    "a deferral names its work: a target, a work item, or both",
                ));
            }
            if work_item.is_some_and(|id| id.contains(['\n', '\r'])) {
                return Err(refuse("work-item", "a work-item ID is one line"));
            }
            if task.is_some() && !names_a_tasks_file(target) {
                return Err(refuse("task", "a task is numbered in a tasks.md target"));
            }
        }
        DispositionOutcome::Waived => {
            for (field, value) in [("target", target), ("task", task), ("work-item", work_item)] {
                if value.is_some() {
                    return Err(refuse(field, "a waiver names no future work"));
                }
            }
            if reason.is_none() {
                return Err(refuse("reason", "a waiver states its reason"));
            }
        }
        DispositionOutcome::Fixed | DispositionOutcome::Undispositioned => return Ok(Vec::new()),
    }
    Ok([("target", target), ("task", task), ("reason", reason)]
        .into_iter()
        .filter_map(|(field, value)| value.map(|value| (format!("{at}.{field}"), value.trim())))
        .collect())
}

/// Refuse a **new** deferral whose pipeline work does not exist: its `target`
/// must resolve inside the project, and its `task` must number a task in that
/// `tasks.md` (spec 076). A deferral already stored under `key` is not new —
/// its work may since have gone, which `process-decisions` reports as a lapse,
/// never as a refusal — unless this run drops it through `expired`, as it does
/// a lapsed one: the re-deferral that replaces it is new. A deferral naming
/// only a work item names nothing the pipeline can look for.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] naming `{at}.target` for a target that
/// does not exist, or `{at}.task` for a task its `tasks.md` does not hold or a
/// `tasks.md` that is not a feature's; [`PrimitiveError::InvalidPath`] for a
/// target the containment screen refuses; and [`PrimitiveError::Io`] when the
/// named `tasks.md` cannot be read or the project root cannot be resolved.
pub(crate) fn refuse_missing_work(
    repo: &Path,
    primitive: &str,
    at: &str,
    disposition: &Disposition,
    key: &str,
    stored: &[RawDecision],
    expired: &[DecisionRef],
) -> Result<()> {
    let standing = stored_decision(stored, key).filter(|decision| {
        !expired
            .iter()
            .any(|gone| same_key(&gone.key, &decision.key))
    });
    if disposition.outcome != DispositionOutcome::Deferred
        || standing.is_some_and(|decision| decision.outcome == DecisionOutcome::Deferred)
    {
        return Ok(());
    }
    let Some(target) = nonblank(disposition.target.as_deref()).map(str::trim) else {
        return Ok(());
    };
    let refuse = |field: &str, reason: String| PrimitiveError::InvalidArgument {
        primitive: primitive.into(),
        argument: format!("{at}.{field}"),
        reason,
    };
    super::validate_contained(repo, target)?;
    if !repo.join(target).exists() {
        return Err(refuse("target", format!("`{target}` does not exist")));
    }
    let Some(task) = nonblank(disposition.task.as_deref()).map(str::trim) else {
        return Ok(());
    };
    let root = crate::schema::paths::Paths::load(repo).specs_root;
    let feature = Path::new(target)
        .parent()
        .filter(|dir| dir.parent() == Some(Path::new(&root)))
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .ok_or_else(|| refuse("task", format!("`{target}` is not a feature's tasks.md")))?;
    let tasks = super::read_tasks::read_checked(
        &crate::schema::primitives::ReadTasksArgs {
            feature: feature.to_string(),
        },
        repo,
        &root,
    )?;
    if tasks.tasks.iter().any(|candidate| candidate.number == task) {
        Ok(())
    } else {
        Err(refuse("task", format!("`{target}` has no task {task}")))
    }
}

/// The feature directory a repo-relative `target` lies in, when it lies under
/// the spec root `root` at all — its first component below the root, which a
/// caller confirms is a spec by finding its `spec.md`.
pub(crate) fn target_feature<'a>(root: &str, target: &'a str) -> Option<&'a str> {
    Path::new(target.trim())
        .strip_prefix(root)
        .ok()
        .and_then(|inside| inside.components().next())
        .and_then(|first| match first {
            std::path::Component::Normal(name) => name.to_str(),
            _ => None,
        })
}

/// Whether `disposition` defers a finding to pipeline work on a spec other
/// than `feature` — the one deferral that releases a gating finding from
/// `feature`'s gate, because that spec's own gate then holds the work (spec
/// 076). A deferral to its own spec, to a rule file, or to an external work
/// item alone releases nothing.
pub(crate) fn releases(repo: &Path, root: &str, feature: &str, disposition: &Disposition) -> bool {
    disposition.outcome == DispositionOutcome::Deferred
        && nonblank(disposition.target.as_deref())
            .and_then(|target| target_feature(root, target))
            .is_some_and(|other| {
                other != feature && repo.join(root).join(other).join("spec.md").is_file()
            })
}

/// Derive every stored entry's `gating` flag from this run: `true` on a
/// deferral whose key is in `holding` — a gating finding deferred to work
/// that does not release it — and absent on every other entry, so a flag an
/// earlier run wrote never outlives the state it described.
pub(crate) fn mark_gating(merged: &mut [RawDecision], holding: &[String]) {
    for entry in merged.iter_mut() {
        let deferred = entry
            .to_ref()
            .is_ok_and(|decision| decision.outcome == DecisionOutcome::Deferred);
        let key = entry.key.as_deref().unwrap_or_default();
        entry.gating = (deferred && holding.iter().any(|held| same_key(held, key))).then_some(true);
    }
}

/// The stored decision a run applies for `key`: the first well-formed entry
/// carrying it. A later duplicate is reported and ignored by
/// `process-decisions`, so matching any entry instead would let an ignored
/// duplicate stand in for the applied one.
pub(crate) fn stored_decision(stored: &[RawDecision], key: &str) -> Option<DecisionRef> {
    stored
        .iter()
        .filter_map(|entry| entry.to_ref().ok())
        .find(|decision| same_key(&decision.key, key))
}

/// Refuse two findings that share a key but carry different outcomes.
///
/// One key is one finding and gets one disposition (spec 058), whatever the
/// two outcomes are — fixed and undispositioned included — so the writer
/// refuses rather than recording one finding as, say, both deferred and waived.
/// Findings sharing a key *and* an outcome pass: each is still counted, as the
/// tier counts and `observations` count it, and [`merge`] stores their one
/// decision once. `keyed` is every finding the run records, as its key and
/// outcome; keys compare through [`same_key`].
///
/// # Errors
///
/// [`DecisionRefusal::ConflictingDecisions`] naming the first key given a
/// second outcome.
pub(crate) fn one_outcome_per_key(
    keyed: impl IntoIterator<Item = (String, DispositionOutcome)>,
) -> std::result::Result<(), DecisionRefusal> {
    let mut seen: Vec<(String, DispositionOutcome)> = Vec::new();
    for (key, outcome) in keyed {
        match seen.iter().find(|(first, _)| same_key(first, &key)) {
            Some((_, first)) if *first != outcome => {
                return Err(DecisionRefusal::ConflictingDecisions(key));
            }
            Some(_) => {}
            None => seen.push((key, outcome)),
        }
    }
    Ok(())
}

/// Whether two decision keys name the same finding: equal once each is
/// flattened to one line, the normalization every writer stores keys under.
/// The comparison [`merge`] and [`one_outcome_per_key`] use, over the
/// normalization `process-decisions` compares keys under, so a key a person
/// hand-edited is judged the same way by the run that reads it and the run
/// that rewrites it.
pub(crate) fn same_key(a: &str, b: &str) -> bool {
    flatten_line(a) == flatten_line(b)
}

/// The decision `disposition` stores under `key`, or `None` for a fixed or
/// undispositioned finding, which stores nothing.
///
/// The one conversion both writers use, so an observation and an analyze
/// finding store their decisions under one policy: the key and the companion
/// text pass through [`flatten_line`], the same normalization
/// `process-decisions` applies to the keys a host fires, so a stored key and a
/// fired key for the same finding agree.
///
/// The one exception is a work-item ID, stored exactly as given: it names an
/// item in a tracker the pipeline never reads, so any rewrite of it would be a
/// different ID (spec 076). [`require_companion`] has already refused one
/// spanning lines.
pub(crate) fn decision_for(disposition: &Disposition, key: &str) -> Option<DecisionRef> {
    let flat = |value: &Option<String>| nonblank(value.as_deref()).map(flatten_line);
    let outcome = match disposition.outcome {
        DispositionOutcome::Deferred => DecisionOutcome::Deferred,
        DispositionOutcome::Waived => DecisionOutcome::Waived,
        DispositionOutcome::Fixed | DispositionOutcome::Undispositioned => return None,
    };
    let deferred = outcome == DecisionOutcome::Deferred;
    Some(DecisionRef {
        key: flatten_line(key),
        outcome,
        target: flat(&disposition.target).filter(|_| deferred),
        task: flat(&disposition.task).filter(|_| deferred),
        work_item: nonblank(disposition.work_item.as_deref())
            .filter(|_| deferred)
            .map(str::to_string),
        reason: flat(&disposition.reason),
    })
}

/// Append a record's `dispositions:` map — always, all four counts, because
/// its absence has a meaning: a record without it predates dispositions, and
/// the gate reads that absence as a record to re-run. Shared by both writers,
/// so the two records carry one shape.
pub(crate) fn render_dispositions(block: &mut String, counts: Dispositions) {
    use std::fmt::Write as _;
    let _ = writeln!(block, "dispositions:");
    let _ = writeln!(block, "  fixed: {}", counts.fixed);
    let _ = writeln!(block, "  deferred: {}", counts.deferred);
    let _ = writeln!(block, "  waived: {}", counts.waived);
    let _ = writeln!(block, "  undispositioned: {}", counts.undispositioned);
}

/// A deferral's work as a report or notice names it: the target in a code
/// span, followed by its task number when it names one, and the external work
/// item in a code span, joined when both are given (spec 076). Each value
/// passes through `plain`, the caller's own normalization.
pub(crate) fn work_reference(
    target: Option<&str>,
    task: Option<&str>,
    work_item: Option<&str>,
    plain: impl Fn(&str) -> String,
) -> String {
    let pipeline = target.map(|target| match task {
        Some(task) => format!("`{}` task {}", plain(target), plain(task)),
        None => format!("`{}`", plain(target)),
    });
    let external = work_item.map(|id| format!("work item `{}`", plain(id)));
    match (pipeline, external) {
        (Some(pipeline), Some(external)) => format!("{pipeline} and {external}"),
        (Some(named), None) | (None, Some(named)) => named,
        (None, None) => "no named work".to_string(),
    }
}

/// The suffix a report renders beside a finding for its disposition — fixed,
/// deferred to its [`work_reference`], waived with its reason, or
/// undispositioned, each outcome in bold — with the companion text passed
/// through `plain`, each writer's own normalization for its report body.
pub(crate) fn disposition_suffix(
    disposition: &Disposition,
    plain: impl Fn(&str) -> String,
) -> String {
    let companion = |value: &Option<String>| plain(value.as_deref().unwrap_or(""));
    match disposition.outcome {
        DispositionOutcome::Fixed => "**fixed**".to_string(),
        DispositionOutcome::Deferred => format!(
            "**deferred** to {}",
            work_reference(
                nonblank(disposition.target.as_deref()),
                nonblank(disposition.task.as_deref()),
                nonblank(disposition.work_item.as_deref()),
                &plain,
            )
        ),
        DispositionOutcome::Waived => {
            format!("**waived**: {}", companion(&disposition.reason))
        }
        DispositionOutcome::Undispositioned => "**undispositioned**".to_string(),
    }
}

/// Why a writer refused a run's decisions.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DecisionRefusal {
    /// A new decision exists and no `decided-by` was supplied ([`merge`]).
    DecidedByMissing,
    /// Two findings sharing this key were given different outcomes. One key
    /// is one finding and gets one disposition (spec 058), so the writer
    /// refuses rather than choosing between them ([`one_outcome_per_key`]).
    ConflictingDecisions(String),
}

impl DecisionRefusal {
    /// The refusal as the writer's `InvalidArgument`: `primitive` names the
    /// writer, `list` the argument its findings arrived in, and `noun` one of
    /// them, for the message.
    pub(crate) fn into_error(self, primitive: &str, list: &str, noun: &str) -> PrimitiveError {
        let (argument, reason) = match self {
            Self::DecidedByMissing => (
                "decided-by".to_string(),
                format!("required when {noun} is newly deferred or waived"),
            ),
            Self::ConflictingDecisions(key) => (
                list.to_string(),
                format!(
                    "two entries share the key `{key}` but were given different outcomes; \
                     one key is one finding and gets one disposition"
                ),
            ),
        };
        PrimitiveError::InvalidArgument {
            primitive: primitive.into(),
            argument,
            reason,
        }
    }
}

/// The decisions recorded in `feature_dir/file`'s frontmatter.
///
/// An absent file or key is an empty list; a list that will not parse is an
/// error, never an empty list (see [`super::read_recorded_list`]).
pub(crate) fn read_decisions(feature_dir: &Path, file: &str) -> Result<Vec<RawDecision>> {
    super::read_recorded_list(feature_dir, file, DECISIONS_KEY)
}

/// The `decisions:` list a writer records.
///
/// Every stored entry survives unless this run expired it or decided its key
/// afresh; a malformed entry always survives, because an entry that cannot be
/// read cannot be proven dead. This run's decisions are first collapsed to one
/// per key, the first kept — two findings sharing a key are one finding with
/// one outcome (spec 058), which [`one_outcome_per_key`] has already held each
/// writer to — and each is then either a **match**, the stored entry
/// `process-decisions` applies for that key with the same outcome, kept
/// byte-for-byte with its original companion, `decided-at`, and `decided-by`;
/// or **new**, stamped with `decided_at` and `decided_by`, replacing every
/// stored entry for its key. Keys compare through [`same_key`] throughout.
/// Matching on key and outcome rather than on the companion's wording is the
/// data model's rule: a host that restates a matched waiver's reason, or
/// normalizes a deferral's target path, has not made a new decision.
///
/// # Errors
///
/// [`DecisionRefusal::DecidedByMissing`] when a new decision exists and
/// `decided_by` is absent or blank: a decision nobody can attribute is not
/// auditable, so the writer refuses rather than storing one.
pub(crate) fn merge(
    stored: Vec<RawDecision>,
    expired: &[DecisionRef],
    decided: &[DecisionRef],
    decided_at: &str,
    decided_by: Option<&str>,
) -> std::result::Result<Vec<RawDecision>, DecisionRefusal> {
    let mut once: Vec<&DecisionRef> = Vec::new();
    for decision in decided {
        if !once.iter().any(|seen| same_key(&seen.key, &decision.key)) {
            once.push(decision);
        }
    }
    // The entry `process-decisions` applies for a key is the first well-formed
    // one; a later duplicate is reported and ignored. Matching against any
    // entry instead let an ignored duplicate stand in for the applied one and
    // swallow a re-decision. An entry this write drops through `expired` — a
    // lapsed deferral among them — stands for nothing, so the decision that
    // replaces it is fresh even with the same outcome (spec 076).
    let fresh: Vec<&DecisionRef> = once
        .into_iter()
        .filter(|decision| {
            expired
                .iter()
                .any(|gone| same_key(&gone.key, &decision.key))
                || stored_decision(&stored, &decision.key)
                    .is_none_or(|stored| stored.outcome != decision.outcome)
        })
        .collect();
    let author = decided_by.map(str::trim).filter(|who| !who.is_empty());
    if !fresh.is_empty() && author.is_none() {
        return Err(DecisionRefusal::DecidedByMissing);
    }
    let dropped = |entry: &RawDecision| {
        let Ok(well_formed) = entry.to_ref() else {
            return false; // malformed entries are never pruned
        };
        expired
            .iter()
            .any(|gone| same_key(&gone.key, &well_formed.key))
            || fresh.iter().any(|new| same_key(&new.key, &well_formed.key))
    };
    let mut merged: Vec<RawDecision> = stored.into_iter().filter(|entry| !dropped(entry)).collect();
    for decision in fresh {
        merged.push(RawDecision {
            key: Some(decision.key.clone()),
            outcome: Some(
                match decision.outcome {
                    DecisionOutcome::Deferred => "deferred",
                    DecisionOutcome::Waived => "waived",
                }
                .to_string(),
            ),
            target: decision.target.clone(),
            task: decision.task.clone(),
            work_item: decision.work_item.clone(),
            reason: decision.reason.clone(),
            gating: None,
            decided_at: Some(decided_at.to_string()),
            decided_by: author.map(str::to_string),
            extra: BTreeMap::new(),
        });
    }
    Ok(merged)
}

/// The outcome name to render for a stored entry: a name a record written
/// before spec 076 carries becomes its current one, so a re-render never
/// writes `routed` or `discarded`; anything else, a malformed value included,
/// is kept as it was written.
fn rendered_outcome(outcome: Option<&str>) -> Option<&str> {
    match outcome.map(str::trim) {
        Some("routed") => Some("deferred"),
        Some("discarded") => Some("waived"),
        _ => outcome,
    }
}

/// Append the `decisions:` list to a rendered record, known fields first and
/// every adopter-authored extra after them, verbatim — through the entry
/// renderer the waiver list uses.
pub(crate) fn render(block: &mut String, decisions: &[RawDecision]) {
    use std::fmt::Write as _;
    if decisions.is_empty() {
        return;
    }
    let _ = writeln!(block, "{DECISIONS_KEY}:");
    for decision in decisions {
        super::write_review::render_list_entry(
            block,
            "",
            &[
                ("key", decision.key.as_deref().into()),
                (
                    "outcome",
                    rendered_outcome(decision.outcome.as_deref()).into(),
                ),
                ("target", decision.target.as_deref().into()),
                ("task", decision.task.as_deref().into()),
                ("work-item", decision.work_item.as_deref().into()),
                ("reason", decision.reason.as_deref().into()),
                (
                    "gating",
                    super::write_review::EntryValue::Flag(decision.gating == Some(true)),
                ),
                ("decided-at", decision.decided_at.as_deref().into()),
                ("decided-by", decision.decided_by.as_deref().into()),
            ],
            &decision.extra,
        );
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn write(dir: &Path, file: &str, frontmatter: &str) {
        std::fs::write(
            dir.join(file),
            format!("---\n{frontmatter}---\n\n# Record\n"),
        )
        .unwrap();
    }

    #[test]
    fn an_absent_file_or_key_is_an_empty_list() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(
            read_decisions(dir.path(), "analysis.md").unwrap(),
            [] as [RawDecision; 0]
        );
        write(dir.path(), "analysis.md", "hard-fail: 0\n");
        assert_eq!(
            read_decisions(dir.path(), "analysis.md").unwrap(),
            [] as [RawDecision; 0]
        );
    }

    #[test]
    fn a_stored_decision_round_trips_with_its_extras() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "review.md",
            "decisions:\n\
             \u{20} - key: \"perf: a() is called in a loop — `src/a.rs`\"\n\
             \u{20}   outcome: deferred\n\
             \u{20}   target: specs/031-x/scenarios/loop.md\n\
             \u{20}   decided-at: 2026-09-25T14:40:00Z\n\
             \u{20}   decided-by: dev@example.com\n\
             \u{20}   ticket: OPS-12\n",
        );
        let decisions = read_decisions(dir.path(), "review.md").unwrap();
        assert_eq!(decisions.len(), 1);
        let decision = &decisions[0];
        assert_eq!(
            decision.extra.get("ticket"),
            Some(&serde_norway::Value::String("OPS-12".into())),
            "an adopter-authored field must survive the parse"
        );
        assert_eq!(
            decision.to_ref(),
            Ok(DecisionRef {
                key: "perf: a() is called in a loop — `src/a.rs`".into(),
                outcome: DecisionOutcome::Deferred,
                target: Some("specs/031-x/scenarios/loop.md".into()),
                task: None,
                work_item: None,
                reason: None,
            })
        );
    }

    #[test]
    fn an_unparseable_list_is_an_error_not_an_empty_list() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "analysis.md", "decisions: not-a-list\n");
        assert!(
            read_decisions(dir.path(), "analysis.md").is_err(),
            "a list that silently read as empty would re-ask every settled question"
        );
    }

    /// An entry with no fields at all is malformed — reported, applied to
    /// nothing — and is the one malformed entry a re-render drops: it holds no
    /// decision to lose, as an all-empty waiver holds none. What the
    /// re-rendered record must not do is stop reading back: the list is
    /// written without it, and a list left with no entries reads as empty.
    #[test]
    fn an_all_empty_entry_is_malformed_and_a_rerender_drops_it() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "review.md", "decisions:\n  - {}\n");
        let entries = read_decisions(dir.path(), "review.md").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].to_ref(), Err("key"));

        let mut block = String::new();
        render(
            &mut block,
            &merge(entries, &[], &[], "2026-09-25T00:00:00Z", None).unwrap(),
        );
        write(dir.path(), "review.md", &block);
        assert_eq!(read_decisions(dir.path(), "review.md").unwrap(), Vec::new());
    }

    fn stored(key: &str, outcome: &str, companion: &str) -> RawDecision {
        RawDecision {
            key: Some(key.into()),
            outcome: Some(outcome.into()),
            target: (outcome == "deferred").then(|| companion.to_string()),
            task: None,
            work_item: None,
            reason: (outcome == "waived").then(|| companion.to_string()),
            gating: None,
            decided_at: Some("2026-09-01T00:00:00Z".into()),
            decided_by: Some("first@example.com".into()),
            extra: BTreeMap::new(),
        }
    }

    fn decided(key: &str, outcome: DecisionOutcome, companion: &str) -> DecisionRef {
        DecisionRef {
            key: key.into(),
            outcome,
            target: (outcome == DecisionOutcome::Deferred).then(|| companion.to_string()),
            task: None,
            work_item: None,
            reason: (outcome == DecisionOutcome::Waived).then(|| companion.to_string()),
        }
    }

    #[test]
    fn a_matching_decision_keeps_its_original_stamp() {
        let merged = merge(
            vec![stored("k", "waived", "noise")],
            &[],
            &[decided("k", DecisionOutcome::Waived, "noise")],
            "2026-09-25T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(
            merged[0].decided_by.as_deref(),
            Some("first@example.com"),
            "a re-matched decision is the same decision, not a new one"
        );
    }

    #[test]
    fn a_changed_decision_replaces_the_stored_one() {
        let merged = merge(
            vec![stored("k", "waived", "noise")],
            &[],
            &[decided(
                "k",
                DecisionOutcome::Deferred,
                "specs/031-x/tasks.md",
            )],
            "2026-09-25T00:00:00Z",
            Some("second@example.com"),
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].outcome.as_deref(), Some("deferred"));
        assert_eq!(merged[0].decided_by.as_deref(), Some("second@example.com"));
        assert_eq!(
            merged[0].decided_at.as_deref(),
            Some("2026-09-25T00:00:00Z")
        );
    }

    #[test]
    fn expired_entries_drop_and_malformed_ones_never_do() {
        let malformed = RawDecision {
            key: Some("broken".into()),
            ..RawDecision::default()
        };
        let merged = merge(
            vec![
                stored("gone", "deferred", "specs/031-x/tasks.md"),
                malformed,
            ],
            &[decided(
                "gone",
                DecisionOutcome::Deferred,
                "specs/031-x/tasks.md",
            )],
            &[],
            "2026-09-25T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].key.as_deref(), Some("broken"));
    }

    #[test]
    fn a_new_decision_without_an_author_is_refused() {
        assert_eq!(
            merge(
                Vec::new(),
                &[],
                &[decided("k", DecisionOutcome::Waived, "noise")],
                "2026-09-25T00:00:00Z",
                Some("  "),
            ),
            Err(DecisionRefusal::DecidedByMissing)
        );
    }

    #[test]
    fn a_rendered_list_reads_back_to_the_same_entries() {
        let dir = tempfile::tempdir().unwrap();
        let mut original = stored(
            "perf: a() loops — `src/a.rs`",
            "deferred",
            "specs/031-x/tasks.md",
        );
        original.extra.insert(
            "ticket".into(),
            serde_norway::Value::String("OPS-12".into()),
        );
        let mut block = String::new();
        render(&mut block, std::slice::from_ref(&original));
        write(dir.path(), "review.md", &block);
        assert_eq!(
            read_decisions(dir.path(), "review.md").unwrap(),
            vec![original]
        );
    }

    #[test]
    fn each_missing_companion_is_its_own_defect() {
        let deferred_without_target = RawDecision {
            key: Some("k".into()),
            outcome: Some("deferred".into()),
            decided_at: Some("2026-09-25T00:00:00Z".into()),
            decided_by: Some("dev@example.com".into()),
            ..RawDecision::default()
        };
        assert_eq!(deferred_without_target.to_ref(), Err("target"));

        let waived_without_reason = RawDecision {
            outcome: Some("waived".into()),
            reason: Some("  ".into()),
            ..deferred_without_target.clone()
        };
        assert_eq!(waived_without_reason.to_ref(), Err("reason"));

        let unknown_outcome = RawDecision {
            outcome: Some("fixed".into()),
            ..deferred_without_target.clone()
        };
        assert_eq!(
            unknown_outcome.to_ref(),
            Err("outcome"),
            "fixed is never stored: a fixed finding stops firing"
        );

        let no_author = RawDecision {
            target: Some("specs/031-x/tasks.md".into()),
            decided_by: None,
            ..deferred_without_target
        };
        assert_eq!(no_author.to_ref(), Err("decided-by"));
    }

    /// The data model's rule: a stored decision matches on key and outcome. A
    /// host restating a matched waiver's reason in other words has not made
    /// a new decision, so the original stamp — and its wording — stand, and
    /// no `decided-by` is needed.
    #[test]
    fn a_rematch_with_reworded_companion_text_keeps_the_original() {
        let merged = merge(
            vec![stored("k", "waived", "noise")],
            &[],
            &[decided("k", DecisionOutcome::Waived, "just noise, really")],
            "2026-09-25T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(merged, vec![stored("k", "waived", "noise")]);
    }

    /// A key decided with a different outcome is a new decision and replaces
    /// the stored one.
    #[test]
    fn a_changed_outcome_is_a_new_decision() {
        let merged = merge(
            vec![stored("k", "waived", "noise")],
            &[],
            &[decided(
                "k",
                DecisionOutcome::Deferred,
                "specs/001-x/tasks.md",
            )],
            "2026-09-25T00:00:00Z",
            Some("second@example.com"),
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].outcome.as_deref(), Some("deferred"));
        assert_eq!(merged[0].decided_by.as_deref(), Some("second@example.com"));
    }

    /// Two findings sharing a key are one finding: the same decision twice is
    /// stored once.
    #[test]
    fn this_runs_decisions_collapse_by_key() {
        let twice = decided("k", DecisionOutcome::Waived, "noise");
        let merged = merge(
            Vec::new(),
            &[],
            &[twice.clone(), twice],
            "2026-09-25T00:00:00Z",
            Some("dev@example.com"),
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
    }

    /// One key, one disposition: two outcomes for one key are refused
    /// whichever two they are — fixed and undispositioned store no decision,
    /// yet still contradict a deferred or waived one. Keys compare flattened,
    /// and a repeated outcome is no contradiction.
    #[test]
    fn two_outcomes_for_one_key_are_refused_whichever_they_are() {
        use DispositionOutcome::{Deferred, Fixed, Undispositioned, Waived};
        let all = [Fixed, Deferred, Waived, Undispositioned];
        for first in all {
            for second in all {
                let verdict = one_outcome_per_key([
                    ("k".to_string(), first),
                    ("other".to_string(), second),
                    (" k\n".to_string(), second),
                ]);
                if first == second {
                    assert_eq!(verdict, Ok(()));
                } else {
                    assert_eq!(
                        verdict,
                        Err(DecisionRefusal::ConflictingDecisions(" k\n".into())),
                        "{first:?} beside {second:?}"
                    );
                }
            }
        }
    }

    /// A known field holding a non-string, or an entry that is not a mapping,
    /// does not deserialize — and fails the whole list, not the one entry.
    #[test]
    fn a_wrong_typed_field_or_a_non_mapping_entry_fails_the_whole_list() {
        let dir = tempfile::tempdir().unwrap();
        let good = "  - key: k\n    outcome: waived\n    reason: noise\n    \
                    decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        for bad in ["  - key: j\n    reason: [a, b]\n", "  - just a string\n"] {
            write(dir.path(), "review.md", &format!("decisions:\n{good}{bad}"));
            assert!(
                matches!(
                    read_decisions(dir.path(), "review.md"),
                    Err(PrimitiveError::Yaml { .. })
                ),
                "{bad:?}"
            );
        }
    }

    /// Two observations matched to one stored waiver are one finding, and a
    /// host that restates the reason differently for each has not made two
    /// decisions: the same outcome twice collapses, whatever its wording. Only
    /// a different outcome is a contradiction.
    #[test]
    fn one_outcome_restated_with_different_wording_is_not_a_conflict() {
        let merged = merge(
            vec![stored("k", "waived", "noise")],
            &[],
            &[
                decided("k", DecisionOutcome::Waived, "noise"),
                decided("k", DecisionOutcome::Waived, "noise, as before"),
            ],
            "2026-09-25T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(merged, vec![stored("k", "waived", "noise")]);
    }

    /// A key a person hand-edited with surrounding spaces is the same key to
    /// every reader: re-matching it is not a new decision (no `decided-by`,
    /// no second entry), as `process-decisions` already matched it.
    #[test]
    fn a_hand_edited_key_is_the_same_decision() {
        let merged = merge(
            vec![stored(" k ", "waived", "noise")],
            &[],
            &[decided("k", DecisionOutcome::Waived, "noise")],
            "2026-09-25T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(merged, vec![stored(" k ", "waived", "noise")]);
    }

    /// `process-decisions` applies the first entry for a key and ignores a
    /// later duplicate, so a re-decision is judged against that first entry.
    /// Judged against any entry, the ignored `deferred` duplicate matched a
    /// re-decision to `deferred`, nothing was stored, and the next run still
    /// applied the `waived` entry — the decision was lost.
    #[test]
    fn a_re_decision_is_not_lost_to_an_ignored_duplicate() {
        let merged = merge(
            vec![
                stored("k", "waived", "noise"),
                stored("k", "deferred", "specs/001-x/tasks.md"),
            ],
            &[],
            &[decided(
                "k",
                DecisionOutcome::Deferred,
                "specs/001-x/tasks.md",
            )],
            "2026-09-26T00:00:00Z",
            Some("second@example.com"),
        )
        .unwrap();
        assert_eq!(merged.len(), 1, "{merged:?}");
        assert_eq!(merged[0].outcome.as_deref(), Some("deferred"));
        assert_eq!(merged[0].decided_by.as_deref(), Some("second@example.com"));
    }

    /// Spec 076: a deferral that lapsed arrives in `expired`, and the new
    /// disposition the run gives its finding — another deferral included — is
    /// stored with this run's stamp, never dropped with the entry it replaces.
    #[test]
    fn a_re_deferral_after_a_lapse_is_stored() {
        let merged = merge(
            vec![stored("k", "deferred", "specs/001-x/tasks.md")],
            &[decided(
                "k",
                DecisionOutcome::Deferred,
                "specs/001-x/tasks.md",
            )],
            &[decided(
                "k",
                DecisionOutcome::Deferred,
                "specs/002-y/tasks.md",
            )],
            "2026-11-03T00:00:00Z",
            Some("second@example.com"),
        )
        .unwrap();
        assert_eq!(merged.len(), 1, "{merged:?}");
        assert_eq!(merged[0].target.as_deref(), Some("specs/002-y/tasks.md"));
        assert_eq!(merged[0].decided_by.as_deref(), Some("second@example.com"));
    }

    /// The shared conversion flattens the key and companion the way
    /// `process-decisions` flattens a fired key, so the two agree.
    #[test]
    fn decision_for_flattens_line_hazards() {
        let disposition = Disposition {
            outcome: DispositionOutcome::Waived,
            target: None,
            task: None,
            work_item: None,
            reason: Some("false\u{1b}positive".into()),
        };
        let decision = decision_for(&disposition, "grounding — split\nmessage").unwrap();
        assert_eq!(decision.key, "grounding — split message");
        assert_eq!(decision.reason.as_deref(), Some("false positive"));
        let fixed = Disposition {
            outcome: DispositionOutcome::Fixed,
            ..Disposition::default()
        };
        assert_eq!(decision_for(&fixed, "k"), None);
    }

    /// Spec 076: a record written before the rename carries `routed` and
    /// `discarded`, and both still read as the current outcomes.
    #[test]
    fn a_pre_076_outcome_name_reads_as_its_current_one() {
        let mut routed = stored("a", "deferred", "specs/001-x/tasks.md");
        routed.outcome = Some("routed".into());
        let mut discarded = stored("b", "waived", "noise");
        discarded.outcome = Some("discarded".into());
        assert_eq!(routed.to_ref().unwrap().outcome, DecisionOutcome::Deferred);
        assert_eq!(discarded.to_ref().unwrap().outcome, DecisionOutcome::Waived);
    }

    /// Spec 076: a stored entry is re-rendered under the current name, so no
    /// writer leaves `routed` or `discarded` behind.
    #[test]
    fn a_re_render_writes_the_current_outcome_names() {
        let mut routed = stored("a", "deferred", "specs/001-x/tasks.md");
        routed.outcome = Some("routed".into());
        let mut discarded = stored("b", "waived", "noise");
        discarded.outcome = Some("discarded".into());
        let mut block = String::new();
        render(&mut block, &[routed, discarded]);
        assert!(block.contains("outcome: deferred"), "{block}");
        assert!(block.contains("outcome: waived"), "{block}");
        assert!(
            !block.contains("routed") && !block.contains("discarded"),
            "{block}"
        );
    }

    #[test]
    fn the_dispositions_map_renders_only_the_current_names() {
        let mut block = String::new();
        render_dispositions(
            &mut block,
            Dispositions {
                fixed: 0,
                deferred: 1,
                waived: 2,
                undispositioned: 0,
            },
        );
        assert_eq!(
            block,
            "dispositions:\n  fixed: 0\n  deferred: 1\n  waived: 2\n  undispositioned: 0\n"
        );
    }

    fn deferral(target: Option<&str>, task: Option<&str>, work_item: Option<&str>) -> Disposition {
        Disposition {
            outcome: DispositionOutcome::Deferred,
            target: target.map(str::to_string),
            task: task.map(str::to_string),
            work_item: work_item.map(str::to_string),
            reason: None,
        }
    }

    fn refused<T: std::fmt::Debug>(result: Result<T>) -> String {
        match result {
            Err(PrimitiveError::InvalidArgument { argument, .. }) => argument,
            other => panic!("expected a refusal, got {other:?}"),
        }
    }

    /// Spec 076: a deferral names its work — a target, a work item, or both —
    /// and each refusal names the argument to fix.
    #[test]
    fn a_deferral_names_its_work() {
        let at = "findings[0].disposition";
        let check = |d: &Disposition| require_companion("w", at, d).map(|c| c.len());
        assert!(check(&deferral(None, None, Some("PROJ-1"))).is_ok());
        assert!(
            check(&deferral(
                Some("specs/001-x/tasks.md"),
                Some("3"),
                Some("PROJ-1")
            ))
            .is_ok()
        );
        assert_eq!(
            refused(check(&deferral(None, None, None))),
            format!("{at}.target")
        );
        assert_eq!(
            refused(check(&deferral(None, None, Some("  ")))),
            format!("{at}.target")
        );
        assert_eq!(
            refused(check(&deferral(None, None, Some("PROJ-1\nPROJ-2")))),
            format!("{at}.work-item")
        );
        assert_eq!(
            refused(check(&deferral(
                Some("specs/001-x/spec.md"),
                Some("3"),
                None
            ))),
            format!("{at}.task")
        );
    }

    /// Spec 076: a waiver states its reason and names no future work.
    #[test]
    fn a_waiver_names_no_work() {
        let waiver = |work_item: Option<&str>, reason: Option<&str>| Disposition {
            outcome: DispositionOutcome::Waived,
            work_item: work_item.map(str::to_string),
            reason: reason.map(str::to_string),
            ..Disposition::default()
        };
        let check = |d: &Disposition| require_companion("w", "o", d).map(|c| c.len());
        assert!(check(&waiver(None, Some("noise"))).is_ok());
        assert_eq!(
            refused(check(&waiver(Some("PROJ-1"), Some("noise")))),
            "o.work-item"
        );
        assert_eq!(refused(check(&waiver(None, None))), "o.reason");
    }

    /// Spec 076: a work-item ID is stored exactly as given, while the target
    /// beside it is flattened like every other companion.
    #[test]
    fn a_work_item_id_is_stored_as_given() {
        // An escape is what flattening rewrites: kept in the ID, replaced in
        // the target.
        let id = " PROJ-12\u{1b}(x) ";
        let decision = decision_for(
            &deferral(Some("specs/001-x/\u{1b}tasks.md"), None, Some(id)),
            "k",
        )
        .unwrap();
        assert_eq!(decision.work_item.as_deref(), Some(id));
        assert_eq!(decision.target.as_deref(), Some("specs/001-x/ tasks.md"));
    }

    /// Spec 076: a stored deferral may name only a work item; one naming no
    /// work, and a waiver naming some, are malformed.
    #[test]
    fn a_stored_deferral_may_name_only_a_work_item() {
        let mut entry = stored("k", "deferred", "unused");
        entry.target = None;
        entry.work_item = Some("PROJ-1".into());
        assert_eq!(entry.to_ref().unwrap().work_item.as_deref(), Some("PROJ-1"));
        entry.work_item = None;
        assert_eq!(entry.to_ref(), Err("target"));
        let mut waiver = stored("w", "waived", "noise");
        waiver.work_item = Some("PROJ-1".into());
        assert_eq!(waiver.to_ref(), Err("work-item"));
    }

    /// Spec 076: a new deferral names work that exists; one already stored is
    /// not re-checked, because its work going away is a lapse.
    #[test]
    fn a_new_deferral_names_work_that_exists() {
        let repo = tempfile::tempdir().unwrap();
        let feature = repo.path().join("specs/001-x");
        std::fs::create_dir_all(&feature).unwrap();
        std::fs::write(
            feature.join("tasks.md"),
            "# Tasks\n\n## 1. One\n\n- [ ] a\n\n- **Done when**: a.\n",
        )
        .unwrap();
        let check = |d: &Disposition, stored: &[RawDecision]| {
            refuse_missing_work(repo.path(), "w", "o", d, "k", stored, &[])
        };
        assert!(
            check(
                &deferral(Some("specs/001-x/tasks.md"), Some("1"), None),
                &[]
            )
            .is_ok()
        );
        assert!(check(&deferral(None, None, Some("PROJ-1")), &[]).is_ok());
        let gone = "specs/001-x/scenarios/gone.md";
        assert_eq!(
            refused(check(&deferral(Some(gone), None, None), &[])),
            "o.target"
        );
        assert_eq!(
            refused(check(
                &deferral(Some("specs/001-x/tasks.md"), Some("7"), None),
                &[]
            )),
            "o.task"
        );
        assert!(
            check(
                &deferral(Some(gone), None, None),
                &[stored("k", "deferred", gone)]
            )
            .is_ok()
        );
    }

    /// Spec 076: a deferral renders the work it names — a target with its
    /// task, a work item, or both — never an empty code span.
    #[test]
    fn a_deferral_renders_the_work_it_names() {
        let plain = |s: &str| s.to_string();
        let suffix = |d: &Disposition| disposition_suffix(d, plain);
        assert_eq!(
            suffix(&deferral(Some("specs/001-x/tasks.md"), None, None)),
            "**deferred** to `specs/001-x/tasks.md`"
        );
        assert_eq!(
            suffix(&deferral(None, None, Some("PROJ-1"))),
            "**deferred** to work item `PROJ-1`"
        );
        assert_eq!(
            suffix(&deferral(
                Some("specs/001-x/tasks.md"),
                Some("7"),
                Some("PROJ-1")
            )),
            "**deferred** to `specs/001-x/tasks.md` task 7 and work item `PROJ-1`"
        );
    }
}
