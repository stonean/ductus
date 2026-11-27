//! `process-decisions` — classify the deferred and waived decisions an audit
//! record carries against this run's finding keys (specs 058 and 076).
//!
//! The host must know which findings already carry a decision **before** it
//! decides dispositions, or it decides again what was answered on an
//! earlier run. So classification happens here, ahead of the writer, the way
//! `process-waivers` classifies waivers ahead of `write-review`:
//!
//! - **matched** — the key fired this run. The finding counts under the stored
//!   outcome and nothing is asked.
//! - **lapsed** — the key fired, but the pipeline work its deferral names is
//!   complete: the deferred work landed and the finding still stands, so the
//!   finding needs a new disposition and the writer drops the entry (spec 076).
//!   A deferral naming only an external work item never lapses, because the
//!   pipeline never reads the tracker; one whose named `tasks.md`, or owning
//!   spec's status, cannot be read — or whose feature directory, or the
//!   `spec.md` or `tasks.md` read in it, resolves outside the project — is
//!   matched, with a notice, so a check that could not run looks like neither.
//! - **expired** — the key did not fire on an unrestricted run, or, on any run,
//!   more findings fire at a violation's key than its `covers`, or the key is a
//!   violation's recorded before sites (spec 020's AC19). The writer drops the
//!   entry. For a deferred decision that did not fire, this is the moment the
//!   deferred work landed.
//! - **retained** — the key did not fire, but the run did not evaluate every
//!   source, or its scope does not hold the file a review key names, so absence
//!   proves nothing and the entry is kept (spec 079).
//! - **dropped** — the key did not fire on a run neither restricted nor given
//!   `since`, on a file still on disk that the run's scope does not hold and that
//!   does not belong to the spec, so no review of the spec will read it (spec
//!   086). The writer drops the entry, and the notice says why, never that the
//!   finding stopped firing (`QUAL-CLAIM-001`). One whose ownership was not
//!   examined is retained, naming the reason.
//! - **malformed** — a required field or an outcome's companion is missing,
//!   or a field its outcome forbids is present; reported, applied to nothing,
//!   and never pruned.
//! - **duplicate** — a repeated key; the first entry applies.
//!
//! Each list grows with the record's `decisions:` list, so the result is
//! bounded as `compute-review-scope`'s is: inline while it fits `[artifacts]
//! read-size-bytes`, and past it with every list in the file it names and
//! their counts inline (`bounded_lists::place`; 066 scenario
//! `process-decisions-results-have-a-bounded-size`). The host drops each
//! `expired` and `lapsed` decision and asks again about each lapsed finding,
//! so no form may read as a short complete list. A retained decision asks
//! nothing of the host, so it is never inline: whenever one is retained, the
//! file holds the `retained` list and each one's own notice
//! (`retained-notices`), and `notices` carries one line per reason instead
//! (066 task 22).
//!
//! Read-only but for that list file: frontmatter mutation belongs to
//! `write-review` and `write-analysis`.

use std::cell::OnceCell;
use std::path::Path;

use crate::primitives::compute_review_scope::RunScope;
use crate::primitives::decisions::{self, Work, read_decisions};
use crate::primitives::review_subject::ReviewSubject;
use crate::primitives::spec_ownership::LazyOwnership;
use crate::primitives::write_review::collapse_at_sites;
use crate::primitives::{
    ANALYSIS_RECORD_FILE, PrimitiveError, REVIEW_RECORD_FILE, Result, flatten_line,
};
use crate::schema::paths;
use crate::schema::primitives::{
    DecisionOutcome, DecisionRecord, DecisionRef, ListsForm, ProcessDecisionsArgs,
    ProcessDecisionsResult,
};

/// Execute the `process-decisions` primitive: [`compute`], returned inline
/// while it fits `[artifacts] read-size-bytes`, and past it with its lists in
/// the file it names and only their counts inline (`bounded_lists::place`).
/// Retained decisions and their own notices are in that file in either form,
/// written whenever one is retained, and `notices` summarizes them.
///
/// # Errors
///
/// Every error [`compute`] returns, and those `bounded_lists::place` returns
/// for a malformed read size or a list file that cannot be written.
pub fn run(args: &ProcessDecisionsArgs, repo: &Path) -> Result<ProcessDecisionsResult> {
    let classified = compute(args, repo)?;
    let subject = ReviewSubject::of(args.feature.as_deref());
    // The host acts on no retained decision: it neither asks about one nor
    // passes one to the writer. So each, with its own notice, goes to the list
    // file, and the result carries their count and one line per reason (066
    // task 22), since in one session they were 131K of the 136K characters
    // this primitive returned, and every later call re-read them.
    let retained_at: Vec<usize> = classified.retained_why.iter().map(|&(_, at)| at).collect();
    let (retained_notices, notices): (Vec<_>, Vec<_>) = classified
        .notices
        .into_iter()
        .enumerate()
        .partition(|(index, _)| retained_at.binary_search(index).is_ok());
    let retained_notices: Vec<String> = retained_notices.into_iter().map(|(_, n)| n).collect();
    let mut notices: Vec<String> = notices.into_iter().map(|(_, n)| n).collect();
    for class in Retained::ALL {
        let count = classified
            .retained_why
            .iter()
            .filter(|&&(why, _)| why == class)
            .count();
        if count > 0 {
            notices.push(class.summary(count, classified.ownership_reason.as_deref()));
        }
    }
    let lists = serde_json::json!({
        "matched": &classified.matched,
        "expired": &classified.expired,
        "retained": &classified.retained,
        "lapsed": &classified.lapsed,
        "dropped": &classified.dropped,
        "notices": &notices,
        "retained-notices": &retained_notices,
    });
    let any_retained = !classified.retained.is_empty();
    let mut result = ProcessDecisionsResult {
        form: ListsForm::Inline,
        matched_count: classified.matched.len(),
        expired_count: classified.expired.len(),
        retained_count: classified.retained.len(),
        lapsed_count: classified.lapsed.len(),
        dropped_count: classified.dropped.len(),
        notices_count: notices.len(),
        matched: Some(classified.matched),
        expired: Some(classified.expired),
        retained: (!any_retained).then(Vec::new),
        lapsed: Some(classified.lapsed),
        dropped: Some(classified.dropped),
        ownership: classified.ownership,
        ownership_reason: classified.ownership_reason,
        notices: Some(notices),
        lists_file: None,
    };
    if let Some(file) =
        super::bounded_lists::place(repo, "process-decisions", subject.name(), &result, &lists)?
    {
        result.form = ListsForm::File;
        result.matched = None;
        result.expired = None;
        result.retained = None;
        result.lapsed = None;
        result.dropped = None;
        result.notices = None;
        result.lists_file = Some(file);
    } else if any_retained {
        result.lists_file = Some(super::bounded_lists::write(
            repo,
            "process-decisions",
            subject.name(),
            &lists,
        )?);
    }
    Ok(result)
}

/// A record's stored decisions classified, every list whole: what
/// `process-decisions` resolves, before its result is bounded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Classified {
    /// See [`ProcessDecisionsResult::matched`].
    pub(crate) matched: Vec<DecisionRef>,
    /// See [`ProcessDecisionsResult::expired`].
    pub(crate) expired: Vec<DecisionRef>,
    /// See [`ProcessDecisionsResult::retained`].
    pub(crate) retained: Vec<DecisionRef>,
    /// See [`ProcessDecisionsResult::lapsed`].
    pub(crate) lapsed: Vec<DecisionRef>,
    /// See [`ProcessDecisionsResult::dropped`].
    pub(crate) dropped: Vec<DecisionRef>,
    /// See [`ProcessDecisionsResult::ownership`].
    pub(crate) ownership: Option<crate::schema::primitives::OwnershipState>,
    /// See [`ProcessDecisionsResult::ownership_reason`].
    pub(crate) ownership_reason: Option<String>,
    /// See [`ProcessDecisionsResult::notices`]. Holds each retained
    /// decision's own notice, which [`run`] moves to the list file.
    pub(crate) notices: Vec<String>,
    /// For each `retained` entry, in order, why it was retained and the index
    /// of its notice in `notices`.
    pub(crate) retained_why: Vec<(Retained, usize)>,
}

impl Classified {
    /// Expire `decision`, whose key is `key`, with a notice saying `why`.
    fn expire(&mut self, key: &str, why: &str, decision: DecisionRef) {
        self.notices
            .push(format!("decision expired: {key} — {why}"));
        self.expired.push(decision);
    }

    /// Place `decision`, whose key did not fire, on `path`, which the run's scope does not
    /// hold, `why` saying so: retained when the scope could not be read, when ownership is
    /// not asked, as on a run given `since` (a restricted run never reaches here), or when
    /// the file belongs to the spec;
    /// dropped when it does not; and retained naming why when its ownership was not
    /// examined (spec 086).
    fn place_unread(
        &mut self,
        (key, path, why): (&str, &str, &str),
        scope_unreadable: bool,
        ownership: Option<&LazyOwnership<'_>>,
        decision: DecisionRef,
    ) {
        if scope_unreadable {
            self.retain(Retained::ScopeUnreadable, why, decision);
            return;
        }
        match ownership.map(|ownership| ownership.belongs(path)) {
            Some(Ok(false)) => self.drop_unowned(key, path, decision),
            Some(Err(reason)) => {
                let why = format!(
                    "{why}, and whether it belongs to this spec was not examined: {reason}"
                );
                self.retain(Retained::OwnershipNotExamined, &why, decision);
            }
            Some(Ok(true)) | None => self.retain(Retained::NotRead, why, decision),
        }
    }

    /// Drop `decision`, whose key is `key`, on `file`, which does not belong to the spec,
    /// with a notice saying so rather than that the finding stopped firing.
    fn drop_unowned(&mut self, key: &str, file: &str, decision: DecisionRef) {
        self.notices.push(format!(
            "decision dropped: {key} — no plan entry names `{}` and no commit attributed to \
             the spec changed it ({})",
            flatten_line(file),
            describe(&decision)
        ));
        self.dropped.push(decision);
    }

    /// Keep `decision`, which did not fire, with its own notice saying `why`,
    /// and record its class and that notice's index for [`run`].
    fn retain(&mut self, class: Retained, why: &str, decision: DecisionRef) {
        self.retained_why.push((class, self.notices.len()));
        self.notices.push(format!(
            "decision retained: {} — {why} ({})",
            flatten_line(&decision.key),
            describe(&decision)
        ));
        self.retained.push(decision);
    }
}

/// Why a stored decision that did not fire was kept: the classes [`run`]
/// summarizes in one notice line each.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Retained {
    /// The run could not read its window's scope, a check that could not run,
    /// so its line stands apart from the others.
    ScopeUnreadable,
    /// The run was restricted: it did not evaluate every source.
    NotEvaluated,
    /// The run's scope does not hold the file the review key names.
    NotRead,
    /// The run's scope does not hold the file, and whether it belongs to the spec
    /// could not be told, so it may not be dropped (spec 086).
    OwnershipNotExamined,
}

impl Retained {
    /// Every class, in the order [`run`] reports them.
    const ALL: [Self; 4] = [
        Self::ScopeUnreadable,
        Self::NotEvaluated,
        Self::NotRead,
        Self::OwnershipNotExamined,
    ];

    /// The summary line for `count` decisions of this class, `ownership` naming why
    /// ownership was not examined.
    fn summary(self, count: usize, ownership: Option<&str>) -> String {
        let (noun, each) = if count == 1 {
            ("decision", "it is")
        } else {
            ("decisions", "each is")
        };
        let why = match self {
            Self::ScopeUnreadable => "this run's scope could not be read".to_string(),
            Self::NotEvaluated => "this run did not evaluate every source".to_string(),
            Self::NotRead => "this run did not read the file its key names".to_string(),
            Self::OwnershipNotExamined => format!(
                "this run did not read the file its key names, and whether it belongs to this \
                 spec was not examined: {}",
                flatten_line(ownership.unwrap_or_default())
            ),
        };
        format!(
            "{count} {noun} retained — {why}; {each} in the `retained` list, with its notice in `retained-notices`, of the file `lists-file` names"
        )
    }
}

/// Classify the record's stored decisions against `args.fired` and, on the review record,
/// the keys of `args.fired-violations`, every list whole.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute,
/// or carries a parent-directory component,
/// or when the feature directory, or a link inside it, resolves outside the
/// project, or when with no `feature` the spec root does,
/// [`PrimitiveError::FeatureNotFound`]
/// when the feature has no `spec.md`, [`PrimitiveError::UnclosedFrontmatter`]
/// when the record's frontmatter block never closes,
/// [`PrimitiveError::Yaml`] when its frontmatter or `decisions:` list does
/// not parse — never an empty result, which would re-ask every settled
/// question — [`PrimitiveError::InvalidArgument`] when `fired-violations` is given for
/// the analysis record, the analysis record is asked for with no `feature`, or an
/// entry's `decision-key` is not its own key, or
/// [`PrimitiveError::Io`] on read failure. An absent record, or
/// one that opens no frontmatter block, holds no decisions: the writers
/// overwrite such a file.
pub(crate) fn compute(args: &ProcessDecisionsArgs, repo: &Path) -> Result<Classified> {
    let layout = paths::Paths::load(repo)?;
    let (_, feature_dir) =
        ReviewSubject::resolve(args.feature.as_deref(), repo, &layout.specs_root)?;
    let file = record_file(args)?;
    refuse_a_foreign_violation_key(args)?;
    let stored = read_decisions(&feature_dir, file)?;
    let fired = fired_keys(args);

    let mut result = Classified::default();
    let mut seen: Vec<String> = Vec::new();
    // Resolved only when a stored finding on a file did not fire, since it walks the
    // window. A review reads its scope, so only its record can have left a file unread.
    let scope: OnceCell<RunScope> = OnceCell::new();
    // Asked only of an unfired entry on a file outside the scope, on a spec's record and
    // a run that evaluated every source in its own window (spec 086).
    let ownership = args
        .feature
        .as_deref()
        .filter(|_| args.since.is_none() && args.record == DecisionRecord::Review)
        .map(|feature| LazyOwnership::new(repo, feature));
    for (index, entry) in stored.iter().enumerate() {
        let decision = match entry.to_ref() {
            Ok(decision) => decision,
            Err(field) => {
                result.notices.push(format!(
                    "malformed decision at {file} decisions[{index}]: missing or invalid '{field}'"
                ));
                continue;
            }
        };
        // Keys compare flattened, as the writers store them and `merge`
        // compares them, so a hand-edited key is one decision to every reader.
        // Every notice prints that flattened key, and flattens the stored text
        // it names, so an escape in a hand-edited record never reaches the
        // operator.
        let key = flatten_line(&decision.key);
        if seen.contains(&key) {
            result.notices.push(format!(
                "duplicate decision: {key} — entry [{index}] ignored"
            ));
            continue;
        }
        seen.push(key.clone());

        let firing = fired.iter().filter(|held| **held == key).count();
        if let Some(why) = expired_by_anchor(args.record, &decision, &key, firing) {
            result.expire(&key, &why, decision);
            continue;
        }

        if firing > 0 {
            if decision.outcome == DecisionOutcome::Deferred {
                match decisions::work_state(
                    repo,
                    &layout.specs_root,
                    decision.target.as_deref(),
                    decision.task.as_deref(),
                ) {
                    Work::Complete => {
                        result.notices.push(format!(
                            "deferral lapsed: {key} — {} is complete and the finding still fires",
                            decisions::work_reference(
                                decision.target.as_deref(),
                                decision.task.as_deref(),
                                None,
                                crate::primitives::flatten_line,
                            )
                        ));
                        result.lapsed.push(decision);
                        continue;
                    }
                    Work::Unreadable => result.notices.push(format!(
                        "deferral target unreadable: {key} — `{}`",
                        flatten_line(decision.target.as_deref().unwrap_or_default())
                    )),
                    Work::Pending | Work::External => {}
                }
            }
            result.matched.push(decision);
        } else if args.restricted {
            result.retain(
                Retained::NotEvaluated,
                "its source was not evaluated this run",
                decision,
            );
        } else if let Some((path, why)) = decisions::key_anchor(&key)
            .filter(|_| args.record == DecisionRecord::Review)
            .and_then(|path| {
                scope
                    .get_or_init(|| {
                        RunScope::resolve(args.feature.as_deref(), args.since.as_deref(), repo)
                    })
                    .why_unread(path, repo)
                    .map(|why| (path, why))
            })
        {
            let unreadable = matches!(scope.get(), Some(RunScope::Unreadable));
            result.place_unread((&key, path, &why), unreadable, ownership.as_ref(), decision);
        } else {
            let why = format!("the finding no longer fires ({})", describe(&decision));
            result.expire(&key, &why, decision);
        }
    }
    if let Some(ownership) = &ownership {
        (result.ownership, result.ownership_reason) = ownership.state();
    }
    Ok(result)
}

/// The record `args.record` names, refusing `fired-violations` on the analysis record, whose
/// findings no site keys.
fn record_file(args: &ProcessDecisionsArgs) -> Result<&'static str> {
    match args.record {
        DecisionRecord::Review => Ok(REVIEW_RECORD_FILE),
        DecisionRecord::Analysis if !args.fired_violations.is_empty() => {
            Err(PrimitiveError::InvalidArgument {
                primitive: "process-decisions".into(),
                argument: "fired-violations".into(),
                reason: "only a review's violations are keyed by site; an analysis passes its findings' keys as `fired`".into(),
            })
        }
        DecisionRecord::Analysis if args.feature.is_none() => Err(PrimitiveError::InvalidArgument {
            primitive: "process-decisions".into(),
            argument: "record".into(),
            reason: "the project has a review record and no analysis record; name a feature, or pass `review`".into(),
        }),
        DecisionRecord::Analysis => Ok(ANALYSIS_RECORD_FILE),
    }
}

/// Refuse a `fired-violations` entry whose `decision-key` is not its own key, naming it: a
/// stored deferral covers only the findings at its own anchor, so no host match keys a
/// finding to another (spec 020's AC19).
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] naming `fired-violations[i].decision-key`.
fn refuse_a_foreign_violation_key(args: &ProcessDecisionsArgs) -> Result<()> {
    for (index, violation) in args.fired_violations.iter().enumerate() {
        if let Some(reason) = decisions::foreign_violation_key(
            &violation.rule,
            &violation.file,
            violation.site.as_deref(),
            violation.decision_key.as_deref(),
        ) {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "process-decisions".into(),
                argument: format!("fired-violations[{index}].decision-key"),
                reason,
            });
        }
    }
    Ok(())
}

/// This run's fired keys, each flattened as the writers store keys, so a finding whose
/// message carried a stray line break still matches the decision stored for it: `fired`
/// once per entry, then a review's `fired-violations`, keyed and counted as `write-review`
/// stores a deferral's `covers`, two overlapping reports at one site once and findings at
/// different sites apart.
fn fired_keys(args: &ProcessDecisionsArgs) -> Vec<String> {
    let violations = collapse_at_sites(&args.fired_violations)
        .into_iter()
        .map(|violation| {
            decisions::violation_key(&violation.rule, &violation.file, violation.site.as_deref())
        });
    args.fired
        .iter()
        .cloned()
        .chain(violations)
        .map(|key| flatten_line(&key))
        .collect()
}

/// Why `decision` expires whatever the run read, restricted or not, or `None` when it is
/// classified by whether its key fired: a violation's key recorded before sites names no
/// site, so it covers nothing (spec 020's `waiver-expiry`, rule 7), and one that covers
/// fewer findings than fire, `firing`, is a count nobody judged (rule 6). The reason
/// reads as the tail of the notice.
fn expired_by_anchor(
    record: DecisionRecord,
    decision: &DecisionRef,
    key: &str,
    firing: usize,
) -> Option<String> {
    if record == DecisionRecord::Review && decisions::is_pre_site_violation_key(key) {
        return Some(format!("recorded before sites ({})", describe(decision)));
    }
    let covers = decision.covers?;
    (firing > covers as usize).then(|| {
        format!(
            "{firing} findings fire where it covers {covers} ({})",
            describe(decision)
        )
    })
}

/// The decision's outcome and its companion, for a notice, each stored text
/// flattened to one line.
fn describe(decision: &DecisionRef) -> String {
    match decision.outcome {
        DecisionOutcome::Deferred => format!(
            "deferred to {}",
            decisions::work_reference(
                decision.target.as_deref(),
                decision.task.as_deref(),
                decision.work_item.as_deref(),
                crate::primitives::flatten_line,
            )
        ),
        DecisionOutcome::Waived => {
            format!(
                "waived: {}",
                flatten_line(decision.reason.as_deref().unwrap_or_default())
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    const DEFERRED: &str = "  - key: \"grounding — plan.md cites a missing path\"\n    \
                          outcome: deferred\n    target: specs/031-x/tasks.md\n    \
                          decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
    const WAIVED: &str = "  - key: \"applicable-rules — BE-AUTHN-001 does not fire\"\n    \
                             outcome: waived\n    reason: cited for a future endpoint\n    \
                             decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";

    fn repo_with(record: &str, decisions: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\n---\n# 001\n",
        )
        .unwrap();
        let target = tmp.path().join("specs/031-x");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::write(
            target.join("spec.md"),
            "---\nstatus: in-progress\n---\n# 031\n",
        )
        .unwrap();
        if !decisions.is_empty() {
            std::fs::write(
                dir.join(record),
                format!("---\nlast-run: 2026-09-25T00:00:00Z\ndecisions:\n{decisions}---\n"),
            )
            .unwrap();
        }
        tmp
    }

    fn args(record: DecisionRecord, fired: &[&str], restricted: bool) -> ProcessDecisionsArgs {
        ProcessDecisionsArgs {
            feature: Some("001-x".into()),
            record,
            fired: fired.iter().map(|k| (*k).to_string()).collect(),
            fired_violations: Vec::new(),
            restricted,
            since: None,
            fired_json: None,
            fired_violations_json: None,
        }
    }

    /// Spec 084: with no `feature` the decisions are the project's review record's, in
    /// the spec root, and the project has no analysis record to ask for.
    #[test]
    fn the_project_records_decisions_are_classified_with_no_feature() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("specs")).unwrap();
        std::fs::write(
            tmp.path().join("specs/review.md"),
            format!("---\nspec: project\nlast-run: null\ndecisions:\n{WAIVED}---\n"),
        )
        .unwrap();
        let project = |record| ProcessDecisionsArgs {
            feature: None,
            ..args(
                record,
                &["applicable-rules — BE-AUTHN-001 does not fire"],
                false,
            )
        };
        let result = run(&project(DecisionRecord::Review), tmp.path()).unwrap();
        assert_eq!((result.matched_count, result.expired_count), (1, 0));
        let err = run(&project(DecisionRecord::Analysis), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == "record"),
            "{err}"
        );
    }

    /// A stored decision, waived, under `key`.
    fn waived(key: &str) -> String {
        format!(
            "  - key: \"{key}\"\n    outcome: waived\n    reason: a person's reason\n    \
             decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n"
        )
    }

    /// 001-x in progress with `src/old.rs`, its own seed's, before its window and
    /// `src/new.rs` inside it, the window starting at an earlier review's committed
    /// record, so the run is no first review, which would read every file the spec's
    /// commits changed (spec 086, AC6); its record holds `keys` uncommitted.
    fn windowed(keys: &[&str]) -> tempfile::TempDir {
        windowed_after(&[], keys)
    }

    /// [`windowed`], with each of `unclaimed` added first by a commit no spec claims, so
    /// it lies outside the window and belongs to no spec (spec 086).
    fn windowed_after(unclaimed: &[&str], keys: &[&str]) -> tempfile::TempDir {
        use crate::primitives::git_fixture::{commit_all, write};
        let tmp = tempfile::tempdir().unwrap();
        let repo = git2::Repository::init(tmp.path()).unwrap();
        if !unclaimed.is_empty() {
            for file in unclaimed {
                write(&tmp.path().join(file), "fn f() {}\n");
            }
            commit_all(&repo, "chore: code no spec claims");
        }
        let spec = tmp.path().join("specs/001-x/spec.md");
        write(&spec, "---\nstatus: planned\n---\n# 001\n");
        write(&tmp.path().join("src/old.rs"), "fn old() {}\n");
        commit_all(&repo, "chore: seed");
        write(&spec, "---\nstatus: in-progress\n---\n# 001\n");
        commit_all(&repo, "status(001): in-progress");
        write(
            &tmp.path().join("specs/001-x/review.md"),
            "---\nspec: 001-x\nlast-run: 2026-09-20T00:00:00Z\nmust-violations: 0\n\
             blocking: false\ndispositions:\n  fixed: 0\n  deferred: 0\n  waived: 0\n  \
             undispositioned: 0\n---\n",
        );
        commit_all(&repo, "review(001): an earlier run");
        write(&tmp.path().join("src/new.rs"), "fn new() {}\n");
        commit_all(&repo, "feat(001): new");
        let decisions: String = keys.iter().map(|key| waived(key)).collect();
        write(
            &tmp.path().join("specs/001-x/review.md"),
            &format!("---\nlast-run: 2026-09-25T00:00:00Z\ndecisions:\n{decisions}---\n"),
        );
        tmp
    }

    /// A key whose finding did not fire, on a file the run never opened, says nothing
    /// about that finding, so a person's decision stays for the run that reads it.
    #[test]
    fn a_silent_decision_on_a_file_the_run_did_not_read_is_retained() {
        let tmp = windowed(&["bug: a finding — `src/old.rs:3`"]);
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(result.expired, [] as [DecisionRef; 0]);
        assert_eq!(result.retained.len(), 1);
        assert!(
            result.notices[0].contains("this run did not read `src/old.rs`"),
            "{:?}",
            result.notices
        );
    }

    #[test]
    fn a_silent_violation_on_a_file_the_run_did_not_read_is_retained() {
        let tmp = windowed(&["rule BE-X-001 at src/old.rs in `walk`"]);
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(result.retained.len(), 1);
    }

    // -- a violation's deferral covers the findings at its anchor (020 AC19) -------------

    const SITE_KEY: &str = "rule BE-X-001 at src/a.rs in `walk`";

    /// A stored decision under `key`, waived, covering `covers` findings.
    fn covering(key: &str, covers: Option<u32>) -> String {
        let covers = covers.map_or_else(String::new, |covers| format!("    covers: {covers}\n"));
        format!(
            "  - key: \"{key}\"\n{covers}    outcome: waived\n    reason: a person's reason\n    \
             decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n"
        )
    }

    /// A decision matches while no more findings share its key than it covers.
    #[test]
    fn a_decision_matches_while_no_more_findings_fire_than_it_covers() {
        let tmp = repo_with("review.md", &covering(SITE_KEY, Some(2)));
        for fired in [vec![SITE_KEY], vec![SITE_KEY, SITE_KEY]] {
            let result = compute(&args(DecisionRecord::Review, &fired, false), tmp.path()).unwrap();
            assert_eq!(result.matched.len(), 1, "{fired:?}");
            assert_eq!(result.matched[0].covers, Some(2));
            assert_eq!(result.expired, [] as [DecisionRef; 0]);
        }
    }

    /// Past its count the decision covers none of the findings and expires with the count
    /// notice, on a restricted run too, since a partial run's surplus is still a surplus.
    #[test]
    fn more_findings_than_it_covers_expire_the_decision_with_the_count_notice() {
        let tmp = repo_with("review.md", &covering(SITE_KEY, Some(1)));
        for restricted in [false, true] {
            let result = compute(
                &args(DecisionRecord::Review, &[SITE_KEY, SITE_KEY], restricted),
                tmp.path(),
            )
            .unwrap();
            assert_eq!(result.matched, [] as [DecisionRef; 0], "{restricted}");
            assert_eq!(result.expired.len(), 1);
            assert_eq!(
                result.notices,
                vec![format!(
                    "decision expired: {SITE_KEY} — 2 findings fire where it covers 1 (waived: a person's reason)"
                )]
            );
        }
    }

    /// A key recorded before sites, `rule {id} at {file}`, covers nothing: it expires
    /// whether or not it fired and on a restricted run too, with the notice naming that.
    #[test]
    fn a_violation_key_recorded_before_sites_expires_whatever_fired() {
        let key = "rule BE-AUTHN-001 at src/a.rs";
        let tmp = repo_with("review.md", &waived(key));
        let expected = vec![format!(
            "decision expired: {key} — recorded before sites (waived: a person's reason)"
        )];
        for (fired, restricted) in [(vec![key], false), (vec![], false), (vec![key], true)] {
            let result = compute(
                &args(DecisionRecord::Review, &fired, restricted),
                tmp.path(),
            )
            .unwrap();
            assert_eq!(
                result.matched,
                [] as [DecisionRef; 0],
                "{fired:?} {restricted}"
            );
            assert_eq!(result.retained, [] as [DecisionRef; 0]);
            assert_eq!(result.expired.len(), 1);
            assert_eq!(result.notices, expected);
        }
    }

    /// Only a key whose word after `rule` is a rule ID reads as a violation's recorded before
    /// sites: an observation keyed `rule engine at startup leaks` matches while it fires, and
    /// a deferral keyed by a rule ID with no site still expires as recorded before sites
    /// (spec 020 task 22). `BE-X-001` is no rule ID by that grammar, whose category takes at
    /// least two characters.
    #[test]
    fn only_a_rule_ids_key_reads_as_recorded_before_sites() {
        let observation = "rule engine at startup leaks";
        let tmp = repo_with("review.md", &waived(observation));
        let result = compute(
            &args(DecisionRecord::Review, &[observation], false),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1, "{result:?}");
        assert_eq!(result.notices, [] as [String; 0]);

        let key = "rule BE-AUTHN-001 at src/a.rs";
        let deferred = format!(
            "  - key: \"{key}\"\n    outcome: deferred\n    work-item: PROJ-1\n    \
             decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n"
        );
        let tmp = repo_with("review.md", &deferred);
        let result = compute(&args(DecisionRecord::Review, &[key], false), tmp.path()).unwrap();
        assert_eq!(result.matched, [] as [DecisionRef; 0], "{result:?}");
        assert_eq!(
            result.notices,
            vec![format!(
                "decision expired: {key} — recorded before sites (deferred to work item `PROJ-1`)"
            )]
        );
    }

    /// A `fired-violations` entry whose `decision-key` is not its own key is refused, naming
    /// it, so no host match keys a finding to another site's deferral; its own key keys it as
    /// its rule, file and site do (spec 020's AC19).
    #[test]
    fn a_fired_violations_decision_key_that_is_not_its_own_is_refused() {
        let tmp = repo_with("review.md", &covering(SITE_KEY, Some(1)));
        let at = |site: &str, key: Option<&str>| crate::schema::primitives::FiredViolation {
            rule: "BE-X-001".into(),
            file: "src/a.rs".into(),
            site: Some(site.into()),
            decision_key: key.map(Into::into),
            ..Default::default()
        };
        let mut review = args(DecisionRecord::Review, &[], false);
        review.fired_violations = vec![at("walk2", Some(SITE_KEY))];
        let error = compute(&review, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, reason, .. }
                if argument == "fired-violations[0].decision-key"
                    && reason.contains(SITE_KEY)
                    && reason.contains("rule BE-X-001 at src/a.rs in `walk2`")),
            "{error}"
        );

        review.fired_violations = vec![at("walk", Some(SITE_KEY))];
        let result = compute(&review, tmp.path()).unwrap();
        assert_eq!(result.matched.len(), 1, "{result:?}");
    }

    /// `fired-violations` keys a review's violations by site, so the analysis record refuses
    /// it, naming the argument, rather than counting keys its findings never carry.
    #[test]
    fn the_analysis_record_refuses_fired_violations() {
        let tmp = repo_with("analysis.md", DEFERRED);
        let mut analysis = args(DecisionRecord::Analysis, &[], false);
        analysis.fired_violations = vec![crate::schema::primitives::FiredViolation {
            rule: "BE-AUTHN-001".into(),
            file: "src/a.rs".into(),
            site: Some("walk".into()),
            ..Default::default()
        }];
        let error = compute(&analysis, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "fired-violations"),
            "{error}"
        );
        analysis.fired_violations.clear();
        assert!(compute(&analysis, tmp.path()).is_ok());
    }

    /// An observation's decision has no count and no site: it matches however many times
    /// its key fires, and an analysis key that reads like a violation's is not one.
    #[test]
    fn an_observation_decision_without_covers_is_unchanged() {
        let key = "bug: a finding — `src/a.rs`";
        let tmp = repo_with("review.md", &waived(key));
        let result = compute(
            &args(DecisionRecord::Review, &[key, key, key], false),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
        assert_eq!(result.matched[0].covers, None);

        let analysis_key = "rule X at somewhere — a message";
        let tmp = repo_with("analysis.md", &waived(analysis_key));
        let result = compute(
            &args(DecisionRecord::Analysis, &[analysis_key], false),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1, "{result:?}");
    }

    /// A `covers` of zero covers nothing, so the entry is malformed under that field.
    #[test]
    fn a_covers_of_zero_is_malformed() {
        let tmp = repo_with("review.md", &covering(SITE_KEY, Some(0)));
        let result = compute(
            &args(DecisionRecord::Review, &[SITE_KEY], false),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched, [] as [DecisionRef; 0]);
        assert_eq!(
            result.notices,
            vec!["malformed decision at review.md decisions[0]: missing or invalid 'covers'"]
        );
    }

    /// The run read the file and the finding is gone, so the decision is over.
    #[test]
    fn a_silent_decision_on_a_file_the_run_read_expires() {
        let tmp = windowed(&["bug: a finding — `src/new.rs`"]);
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(result.expired.len(), 1);
        assert_eq!(result.retained, [] as [DecisionRef; 0]);
    }

    /// Nothing is left for the finding to fire on, whatever the scope holds.
    #[test]
    fn a_silent_decision_on_a_file_gone_from_disk_expires() {
        let tmp = windowed(&["bug: a finding — `src/gone.rs`"]);
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(result.expired.len(), 1);
    }

    /// With no file to test, a key behaves as it did: only firing keeps it.
    #[test]
    fn a_silent_decision_naming_no_file_expires() {
        let tmp = windowed(&["bug: a finding with no path"]);
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(result.expired.len(), 1);
    }

    /// A firing key is matched wherever its file lies.
    #[test]
    fn a_firing_decision_outside_the_scope_is_matched() {
        let key = "bug: a finding — `src/old.rs`";
        let tmp = windowed(&[key]);
        let result = compute(&args(DecisionRecord::Review, &[key], false), tmp.path()).unwrap();
        assert_eq!(result.matched.len(), 1);
        assert_eq!(result.retained, [] as [DecisionRef; 0]);
    }

    /// AC2: an unfired decision on a file outside the scope that does not belong to the
    /// spec is dropped and named, never expired; one on a file that belongs, unchanged
    /// since the record was written, is retained (spec 086).
    #[test]
    fn a_silent_decision_on_a_file_that_does_not_belong_is_dropped_and_named() {
        let foreign = "bug: a finding — `src/foreign.rs:3`";
        let owned = "bug: a finding — `src/old.rs`";
        let tmp = windowed_after(&["src/foreign.rs"], &[foreign, owned]);
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(keys(&result.dropped), [foreign], "{:?}", result.notices);
        assert_eq!(keys(&result.retained), [owned], "{:?}", result.notices);
        assert_eq!(result.expired, [] as [DecisionRef; 0]);
        assert!(
            result
                .notices
                .iter()
                .any(|n| n.starts_with(&format!("decision dropped: {foreign}"))
                    && n.contains("no plan entry names `src/foreign.rs`")),
            "{:?}",
            result.notices
        );
        assert_eq!(
            result.ownership,
            Some(crate::schema::primitives::OwnershipState::Examined)
        );
        let out = run(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(out.dropped_count, 1);
    }

    /// AC4: a decision on a file deleted from disk expires, as before, ahead of any test
    /// of whether the file belonged (spec 086).
    #[test]
    fn a_silent_decision_on_a_deleted_file_expires_and_is_not_dropped() {
        let gone = "bug: a finding — `src/foreign.rs`";
        let tmp = windowed_after(&["src/foreign.rs"], &[gone]);
        std::fs::remove_file(tmp.path().join("src/foreign.rs")).unwrap();
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(keys(&result.expired), [gone]);
        assert_eq!(result.dropped, [] as [DecisionRef; 0]);
        assert_eq!(result.ownership, None, "nothing reached the ownership test");
    }

    /// A restricted run and a run given `since` judge no decision by ownership: each is
    /// retained, and no ownership is reported (spec 086, Edge Cases).
    #[test]
    fn a_restricted_run_and_a_since_run_drop_nothing() {
        let foreign = "bug: a finding — `src/foreign.rs`";
        let tmp = windowed_after(&["src/foreign.rs"], &[foreign]);
        let restricted = compute(&args(DecisionRecord::Review, &[], true), tmp.path()).unwrap();
        let mut narrowed = args(DecisionRecord::Review, &[], false);
        narrowed.since = Some("HEAD".to_string());
        let since = compute(&narrowed, tmp.path()).unwrap();
        for result in [restricted, since] {
            assert_eq!(keys(&result.retained), [foreign], "{:?}", result.notices);
            assert_eq!(result.dropped, [] as [DecisionRef; 0]);
            assert_eq!(result.ownership, None);
        }
    }

    /// A shallow history cannot say what belongs, so nothing is dropped: the decision is
    /// retained under its own class, naming the reason (spec 086).
    #[test]
    fn a_shallow_history_drops_nothing_and_says_why() {
        let foreign = "bug: a finding — `src/foreign.rs`";
        let tmp = windowed_after(&["src/foreign.rs"], &[foreign]);
        let repo = git2::Repository::open(tmp.path()).unwrap();
        let mut walk = repo.revwalk().unwrap();
        walk.push_head().unwrap();
        let first = walk
            .map(|oid| oid.unwrap())
            .find(|oid| {
                repo.find_commit(*oid).unwrap().summary().ok().flatten()
                    == Some("chore: code no spec claims")
            })
            .unwrap();
        // A shallow clone cut at its first commit: the window, which starts at the earlier
        // review, can still be placed, but a cut history cannot say what belongs.
        std::fs::write(tmp.path().join(".git/shallow"), format!("{first}\n")).unwrap();
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(keys(&result.retained), [foreign], "{:?}", result.notices);
        assert_eq!(result.dropped, [] as [DecisionRef; 0]);
        assert_eq!(
            result.ownership,
            Some(crate::schema::primitives::OwnershipState::NotExamined)
        );
        assert!(
            result
                .ownership_reason
                .as_deref()
                .is_some_and(|reason| reason.contains("--unshallow")),
            "{result:?}"
        );
        let out = run(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert!(
            out.notices
                .iter()
                .flatten()
                .any(|n| n.contains("whether it belongs to this spec was not examined")),
            "{out:?}"
        );
    }

    /// A narrowed window reads less, so the same file is unread under it.
    #[test]
    fn the_runs_since_decides_what_it_read() {
        let tmp = windowed(&["bug: a finding — `src/new.rs`"]);
        let mut narrowed = args(DecisionRecord::Review, &[], false);
        narrowed.since = Some("HEAD".to_string());
        let result = compute(&narrowed, tmp.path()).unwrap();
        assert_eq!(result.retained.len(), 1, "{:?}", result.notices);
    }

    /// An analysis has no window: it reads its subjects whatever a commit changed.
    #[test]
    fn an_analysis_expires_a_silent_decision_wherever_its_file_lies() {
        use crate::primitives::git_fixture::write;
        let key = "bug: a finding — `src/old.rs`";
        let tmp = windowed(&[]);
        write(
            &tmp.path().join("specs/001-x/analysis.md"),
            &format!(
                "---\nlast-run: 2026-09-25T00:00:00Z\ndecisions:\n{}---\n",
                waived(key)
            ),
        );
        let result = compute(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap();
        assert_eq!(result.expired.len(), 1);
    }

    /// No window to read is no evidence the file was read, and the notice says which.
    #[test]
    fn a_scope_that_cannot_be_read_retains_and_says_so() {
        let tmp = repo_with(
            REVIEW_RECORD_FILE,
            &waived("bug: a finding — `src/absent.rs`"),
        );
        let result = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(result.retained.len(), 1);
        assert!(
            result.notices[0].contains("this run's scope could not be read"),
            "{:?}",
            result.notices
        );
    }

    #[test]
    fn a_firing_key_matches_under_its_stored_outcome() {
        let tmp = repo_with("analysis.md", &format!("{DEFERRED}{WAIVED}"));
        let result = compute(
            &args(
                DecisionRecord::Analysis,
                &[
                    "grounding — plan.md cites a missing path",
                    "applicable-rules — BE-AUTHN-001 does not fire",
                ],
                false,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 2);
        assert_eq!(result.matched[0].outcome, DecisionOutcome::Deferred);
        assert_eq!(
            result.matched[0].target.as_deref(),
            Some("specs/031-x/tasks.md")
        );
        assert_eq!(result.matched[1].outcome, DecisionOutcome::Waived);
        assert!(result.expired.is_empty() && result.retained.is_empty());
        assert_eq!(result.notices, [] as [String; 0]);
    }

    #[test]
    fn a_silent_key_expires_on_an_unrestricted_run() {
        let tmp = repo_with("analysis.md", DEFERRED);
        let result = compute(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap();
        assert_eq!(result.expired.len(), 1, "the deferred work landed");
        assert!(result.notices[0].starts_with("decision expired: grounding"));
    }

    #[test]
    fn a_silent_key_is_retained_on_a_restricted_run() {
        let tmp = repo_with("review.md", WAIVED);
        let result = compute(&args(DecisionRecord::Review, &[], true), tmp.path()).unwrap();
        assert!(
            result.expired.is_empty(),
            "absence proves nothing when the source was skipped"
        );
        assert_eq!(result.retained.len(), 1);
        assert!(result.notices[0].starts_with("decision retained:"));
    }

    #[test]
    fn a_restricted_run_still_matches_a_firing_key() {
        let tmp = repo_with("review.md", WAIVED);
        let result = compute(
            &args(
                DecisionRecord::Review,
                &["applicable-rules — BE-AUTHN-001 does not fire"],
                true,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
    }

    #[test]
    fn a_missing_companion_is_malformed_and_never_pruned() {
        let no_target = "  - key: k\n    outcome: deferred\n    \
                         decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        let no_reason = "  - key: j\n    outcome: waived\n    \
                         decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        let tmp = repo_with("analysis.md", &format!("{no_target}{no_reason}"));
        let result = compute(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap();
        assert!(
            result.matched.is_empty() && result.expired.is_empty() && result.retained.is_empty()
        );
        assert_eq!(
            result.notices,
            vec![
                "malformed decision at analysis.md decisions[0]: missing or invalid 'target'",
                "malformed decision at analysis.md decisions[1]: missing or invalid 'reason'",
            ]
        );
    }

    #[test]
    fn the_first_of_a_duplicated_key_wins() {
        let tmp = repo_with("analysis.md", &format!("{DEFERRED}{DEFERRED}"));
        let result = compute(
            &args(
                DecisionRecord::Analysis,
                &["grounding — plan.md cites a missing path"],
                false,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
        assert_eq!(result.notices.len(), 1);
        assert!(result.notices[0].starts_with("duplicate decision:"));
    }

    /// A key hand-edited with surrounding spaces is the same key: it matches
    /// the finding it names, and a second entry for that key is a duplicate,
    /// exactly as `merge` judges them when it rewrites the list.
    #[test]
    fn keys_compare_flattened_for_matching_and_duplicates() {
        let padded = DEFERRED.replace(
            "\"grounding — plan.md cites a missing path\"",
            "\" grounding — plan.md cites a missing path \"",
        );
        let tmp = repo_with("analysis.md", &format!("{padded}{DEFERRED}"));
        let result = compute(
            &args(
                DecisionRecord::Analysis,
                &["grounding — plan.md cites a missing path"],
                false,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
        assert_eq!(
            result.matched[0].key,
            " grounding — plan.md cites a missing path "
        );
        assert_eq!(result.notices.len(), 1);
        assert!(result.notices[0].starts_with("duplicate decision:"));
    }

    #[test]
    fn a_record_with_no_decisions_yields_empty_lists() {
        let tmp = repo_with("analysis.md", "");
        let result = compute(
            &args(DecisionRecord::Analysis, &["anything"], false),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result, Classified::default());
    }

    /// A record that opens no frontmatter block holds no decisions — as
    /// `write-analysis` and `write-review`, which overwrite such a record,
    /// read it — rather than refusing the run.
    #[test]
    fn a_record_without_frontmatter_holds_no_decisions() {
        for record in [DecisionRecord::Analysis, DecisionRecord::Review] {
            let tmp = repo_with("analysis.md", "");
            let dir = tmp.path().join("specs/001-x");
            std::fs::write(dir.join("analysis.md"), "# Analysis\n\nhand-edited\n").unwrap();
            std::fs::write(dir.join("review.md"), "# Review\n\nhand-edited\n").unwrap();
            let result = compute(&args(record, &["anything"], false), tmp.path()).unwrap();
            assert_eq!(result, Classified::default());
        }
    }

    /// A frontmatter block that opens and never closes may hold decisions
    /// nobody can read, so it is refused and named as unclosed.
    #[test]
    fn a_record_whose_frontmatter_never_closes_is_refused() {
        let tmp = repo_with("analysis.md", "");
        std::fs::write(
            tmp.path().join("specs/001-x/analysis.md"),
            format!("---\nspec: 001-x\ndecisions:\n{DEFERRED}\n# Analysis\n"),
        )
        .unwrap();
        let error = compute(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap_err();
        assert!(error.to_string().contains("never closes"), "{error}");
        assert!(matches!(error, PrimitiveError::UnclosedFrontmatter { .. }));
    }

    #[test]
    fn the_record_argument_selects_the_file() {
        let tmp = repo_with("review.md", DEFERRED);
        let from_analysis =
            compute(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap();
        assert!(
            from_analysis.expired.is_empty(),
            "analysis.md holds no decisions here"
        );
        let from_review = compute(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(from_review.expired.len(), 1);
    }

    #[test]
    fn a_missing_feature_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(matches!(
            compute(&args(DecisionRecord::Review, &[], false), tmp.path()),
            Err(PrimitiveError::FeatureNotFound { .. })
        ));
    }

    /// A fired key is compared in the form the writers store it: a finding
    /// whose message carried a stray line break still matches its decision.
    #[test]
    fn a_fired_key_with_a_line_break_matches_its_flattened_decision() {
        let tmp = repo_with("analysis.md", DEFERRED);
        let result = compute(
            &args(
                DecisionRecord::Analysis,
                &["grounding — plan.md cites\na missing path"],
                false,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
        assert_eq!(result.expired, [] as [DecisionRef; 0]);
    }

    /// A deferral of `key` naming `target`, and `task` when given.
    fn deferral(key: &str, target: &str, task: Option<&str>) -> String {
        let task = task
            .map(|n| format!("    task: \"{n}\"\n"))
            .unwrap_or_default();
        format!(
            "  - key: \"{key}\"\n    outcome: deferred\n    target: {target}\n{task}    \
             decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n"
        )
    }

    fn write(tmp: &tempfile::TempDir, path: &str, content: &str) {
        let path = tmp.path().join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    /// Run with every key in `fired` firing, unrestricted.
    fn classify(tmp: &tempfile::TempDir, fired: &[&str]) -> Classified {
        compute(&args(DecisionRecord::Analysis, fired, false), tmp.path()).unwrap()
    }

    fn keys(decisions: &[DecisionRef]) -> Vec<&str> {
        decisions.iter().map(|d| d.key.as_str()).collect()
    }

    /// Spec 076: a deferral to a task lapses once the task is checked or gone,
    /// and holds while its spec is open and the task has an unchecked box.
    #[test]
    fn a_deferral_to_a_task_lapses_when_the_task_is_complete() {
        let tasks = "specs/031-x/tasks.md";
        let tmp = repo_with(
            "analysis.md",
            &[
                deferral("open", tasks, Some("1")),
                deferral("checked", tasks, Some("2")),
                deferral("gone", tasks, Some("9")),
            ]
            .concat(),
        );
        write(
            &tmp,
            tasks,
            "# Tasks\n\n## 1. Open\n\n- [x] a\n- [ ] b\n\n- **Done when**: b.\n\n\
             ## 2. Checked\n\n- [x] a\n\n- **Done when**: a.\n",
        );
        let result = classify(&tmp, &["open", "checked", "gone"]);
        assert_eq!(keys(&result.matched), ["open"]);
        assert_eq!(keys(&result.lapsed), ["checked", "gone"]);
        assert!(
            result.notices.iter().any(|n| n
                == "deferral lapsed: checked — `specs/031-x/tasks.md` task 2 is complete \
                    and the finding still fires"),
            "{:?}",
            result.notices
        );
    }

    /// Spec 076: a deferral to a spec's work lapses once that spec is `done`
    /// or gone; one to a rule file lapses as soon as its finding fires again.
    #[test]
    fn a_deferral_to_a_spec_or_rule_lapses_when_its_work_is_complete() {
        let tmp = repo_with(
            "analysis.md",
            &[
                deferral("pending", "specs/031-x/scenarios/a.md", None),
                deferral("done", "specs/032-y/spec.md", None),
                deferral("retired", "specs/033-z/spec.md", None),
                deferral("rule", "specs/rules/security-backend.md", None),
                deferral(
                    "framework rule",
                    "framework/rules/security-backend.md",
                    None,
                ),
            ]
            .concat(),
        );
        write(
            &tmp,
            "specs/032-y/spec.md",
            "---\nstatus: done\n---\n# 032\n",
        );
        let result = classify(
            &tmp,
            &["pending", "done", "retired", "rule", "framework rule"],
        );
        assert_eq!(keys(&result.matched), ["pending"]);
        assert_eq!(
            keys(&result.lapsed),
            ["done", "retired", "rule", "framework rule"]
        );
    }

    /// Spec 076: a deferral naming only a work item never lapses, and one
    /// whose target cannot be read is matched with a notice.
    #[test]
    fn external_and_unreadable_deferrals_are_matched() {
        let external = "  - key: \"external\"\n    outcome: deferred\n    work-item: PROJ-1\n    \
                        decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        let tmp = repo_with(
            "analysis.md",
            &[
                external.to_string(),
                deferral("unreadable", "specs/034-w/spec.md", None),
            ]
            .concat(),
        );
        write(
            &tmp,
            "specs/034-w/spec.md",
            "---\nstatus: [not, a, string\n---\n",
        );
        let result = classify(&tmp, &["external", "unreadable"]);
        assert_eq!(keys(&result.matched), ["external", "unreadable"]);
        assert_eq!(result.lapsed, [] as [DecisionRef; 0]);
        assert!(
            result
                .notices
                .iter()
                .any(|n| n == "deferral target unreadable: unreadable — `specs/034-w/spec.md`"),
            "{:?}",
            result.notices
        );
    }

    /// Spec 076: a stored deferral's work is kept as written, so the notices
    /// flatten it before naming it — the target a lapse names, and the work
    /// item an expiry names — and an escape in the record reaches neither.
    #[test]
    fn notices_name_the_work_flattened() {
        let external = "  - key: \"external\"\n    outcome: deferred\n    \
                        work-item: \"PROJ-2\\u001b[0m\"\n    \
                        decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        let tmp = repo_with(
            "analysis.md",
            &[
                deferral("rule", "\"framework/rules/a\\u001b.md\"", None),
                external.to_string(),
            ]
            .concat(),
        );
        let result = classify(&tmp, &["rule"]);
        assert_eq!(keys(&result.lapsed), ["rule"]);
        assert_eq!(keys(&result.expired), ["external"]);
        let lapsed = "deferral lapsed: rule — `framework/rules/a .md` is complete and the \
                      finding still fires";
        let expired = "decision expired: external — the finding no longer fires (deferred \
                       to work item `PROJ-2 [0m`)";
        for notice in [lapsed, expired] {
            assert!(
                result.notices.iter().any(|n| n == notice),
                "{:?}",
                result.notices
            );
        }
    }

    /// Spec 076: a stored key, target and reason are kept as written, so a
    /// hand-edited record can carry an escape in any of them; every notice
    /// flattens what it prints, and none reaches the operator raw.
    #[test]
    fn notices_flatten_every_stored_text_they_print() {
        let waiver = |key: &str| {
            format!(
                "  - key: \"{key}\"\n    outcome: waived\n    reason: \"noise\\u001b[2J\"\n    \
                 decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n"
            )
        };
        let tmp = repo_with(
            "analysis.md",
            &[
                waiver("dup"),
                waiver("dup\\u001b"),
                deferral("lap\\u001b", "framework/rules/a.md", None),
                deferral("unr\\u001b", "\"specs/034-w/sce\\u001bnario.md\"", None),
                waiver("exp\\u001b"),
            ]
            .concat(),
        );
        write(
            &tmp,
            "specs/034-w/spec.md",
            "---\nstatus: [not, a, string\n---\n",
        );
        let result = classify(&tmp, &["dup", "lap", "unr"]);
        let retained = compute(&args(DecisionRecord::Analysis, &[], true), tmp.path())
            .unwrap()
            .notices;
        let notices = [result.notices, retained].concat();
        for notice in [
            "duplicate decision: dup — entry [1] ignored",
            "deferral lapsed: lap — `framework/rules/a.md` is complete and the finding still fires",
            "deferral target unreadable: unr — `specs/034-w/sce nario.md`",
            "decision expired: exp — the finding no longer fires (waived: noise [2J)",
            "decision retained: exp — its source was not evaluated this run (waived: noise [2J)",
        ] {
            assert!(notices.iter().any(|n| n == notice), "{notices:?}");
        }
        assert!(!notices.iter().any(|n| n.contains('\u{1b}')), "{notices:?}");
    }

    fn read_size(tmp: &tempfile::TempDir, bytes: u64) {
        write(
            tmp,
            ".ductus/config.toml",
            &format!("[artifacts]\nread-size-bytes = {bytes}\n"),
        );
    }

    /// The counts a result carries in either form, in list order.
    fn counts(result: &ProcessDecisionsResult) -> (usize, usize, usize, usize, usize) {
        (
            result.matched_count,
            result.expired_count,
            result.retained_count,
            result.lapsed_count,
            result.notices_count,
        )
    }

    /// The lists the file `file` names holds, from under this primitive's
    /// results directory for 001-x.
    fn lists_in(tmp: &tempfile::TempDir, file: &str) -> serde_json::Value {
        assert!(
            file.starts_with(".ductus/results/process-decisions/001-x/"),
            "{file}"
        );
        serde_json::from_str(&std::fs::read_to_string(tmp.path().join(file)).unwrap()).unwrap()
    }

    /// `whole`'s notices as `run` returns them, each retained decision's own
    /// line moved to the second list and `summary` appended to the first.
    fn split_notices(whole: &Classified, summary: Option<&str>) -> (Vec<String>, Vec<String>) {
        let (own, mut rest): (Vec<String>, Vec<String>) = whole
            .notices
            .iter()
            .cloned()
            .partition(|n| n.starts_with("decision retained:"));
        assert_eq!(own.len(), whole.retained.len());
        rest.extend(summary.map(str::to_string));
        (rest, own)
    }

    /// A dropped decision moves to the file with the other lists past the read size, so
    /// the host passes each to the writer from there (spec 086).
    #[test]
    fn a_dropped_decision_moves_to_the_file_past_the_read_size() {
        let foreign = "bug: a finding — `src/foreign.rs`";
        let tmp = windowed_after(&["src/foreign.rs"], &[foreign]);
        read_size(&tmp, 10);
        let out = run(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(out.form, ListsForm::File, "{out:?}");
        assert_eq!((out.dropped.as_ref(), out.dropped_count), (None, 1));
        let listed: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(tmp.path().join(out.lists_file.as_deref().unwrap())).unwrap(),
        )
        .unwrap();
        assert_eq!(
            listed["dropped"].as_array().map(Vec::len),
            Some(1),
            "{listed}"
        );
        assert_eq!(listed["dropped"][0]["key"], foreign);
    }

    /// A result that fits is inline, with its counts. Past `[artifacts]
    /// read-size-bytes` every list moves whole to the file the result names,
    /// none of them inline, and the counts still say how many — so the
    /// expired and lapsed decisions the host drops all reach it (066 scenario
    /// `process-decisions-results-have-a-bounded-size`).
    #[test]
    fn a_result_past_the_read_size_names_a_file_holding_every_list() {
        let tasks = "specs/031-x/tasks.md";
        let tmp = repo_with(
            "analysis.md",
            &[
                deferral("open", tasks, Some("1")),
                deferral("checked", tasks, Some("2")),
                WAIVED.to_string(),
                DEFERRED.to_string(),
            ]
            .concat(),
        );
        write(
            &tmp,
            tasks,
            "# Tasks\n\n## 1. Open\n\n- [ ] a\n\n- **Done when**: a.\n\n\
             ## 2. Checked\n\n- [x] a\n\n- **Done when**: a.\n",
        );
        for restricted in [false, true] {
            let call = args(DecisionRecord::Analysis, &["open", "checked"], restricted);
            read_size(&tmp, 1_000_000);
            let whole = compute(&call, tmp.path()).unwrap();
            assert_eq!(
                (whole.matched.len(), whole.lapsed.len()),
                (1, 1),
                "{whole:?}"
            );
            assert_eq!(
                (whole.expired.len(), whole.retained.len()),
                if restricted { (0, 2) } else { (2, 0) },
                "{whole:?}"
            );
            // A restricted run retains the two silent decisions, so they and
            // their notices are in the file in either form (066 task 22).
            let (notices, retained_notices) = split_notices(
                &whole,
                restricted.then_some(
                    "2 decisions retained — this run did not evaluate every source; each is in \
                     the `retained` list, with its notice in `retained-notices`, of the file \
                     `lists-file` names",
                ),
            );
            let inline = run(&call, tmp.path()).unwrap();
            assert_eq!(inline.form, ListsForm::Inline);
            assert_eq!(inline.lists_file.is_some(), restricted, "{inline:?}");
            assert_eq!(
                (
                    &inline.matched,
                    &inline.expired,
                    &inline.retained,
                    &inline.lapsed,
                    &inline.notices
                ),
                (
                    &Some(whole.matched.clone()),
                    &Some(whole.expired.clone()),
                    &(!restricted).then(Vec::new),
                    &Some(whole.lapsed.clone()),
                    &Some(notices.clone())
                )
            );

            read_size(&tmp, 10);
            let bounded = run(&call, tmp.path()).unwrap();
            assert_eq!(bounded.form, ListsForm::File);
            assert_eq!(
                (
                    &bounded.matched,
                    &bounded.expired,
                    &bounded.retained,
                    &bounded.lapsed,
                    &bounded.dropped,
                    &bounded.notices
                ),
                (&None, &None, &None, &None, &None, &None)
            );
            let want = (
                whole.matched.len(),
                whole.expired.len(),
                whole.retained.len(),
                whole.lapsed.len(),
                notices.len(),
            );
            assert_eq!((counts(&inline), counts(&bounded)), (want, want));
            let expected = serde_json::json!({
                "matched": whole.matched,
                "expired": whole.expired,
                "retained": whole.retained,
                "lapsed": whole.lapsed,
                "dropped": whole.dropped,
                "notices": notices,
                "retained-notices": retained_notices,
            });
            for file in [&inline.lists_file, &bounded.lists_file]
                .into_iter()
                .flatten()
            {
                assert_eq!(lists_in(&tmp, file), expected);
            }
        }
    }

    /// A decision retained on a file the run did not read is in the file the
    /// result names, with its own notice, and the result carries its count
    /// and one line saying why, however small the result (066 task 22).
    #[test]
    fn retained_decisions_reach_the_file_and_not_the_result() {
        let tmp = windowed(&["bug: a finding — `src/old.rs:3`"]);
        let call = args(DecisionRecord::Review, &[], false);
        let result = run(&call, tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::Inline);
        assert_eq!((result.retained_count, &result.retained), (1, &None));
        assert_eq!(
            result.notices,
            Some(vec![
                "1 decision retained — this run did not read the file its key names; it is in \
                 the `retained` list, with its notice in `retained-notices`, of the file \
                 `lists-file` names"
                    .to_string()
            ])
        );
        assert_eq!(result.notices_count, 1);
        let lists = lists_in(&tmp, &result.lists_file.expect("a lists file"));
        assert_eq!(
            lists["retained"][0]["key"],
            "bug: a finding — `src/old.rs:3`"
        );
        let own = lists["retained-notices"].as_array().unwrap();
        assert_eq!(own.len(), 1);
        assert!(
            own[0]
                .as_str()
                .unwrap()
                .contains("this run did not read `src/old.rs`"),
            "{own:?}"
        );
    }

    /// A scope that could not be read is a check that could not run, so its
    /// summary line is its own, never the not-read line.
    #[test]
    fn a_scope_that_cannot_be_read_has_its_own_summary_line() {
        let tmp = repo_with(
            REVIEW_RECORD_FILE,
            &waived("bug: a finding — `src/absent.rs`"),
        );
        let result = run(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(
            result.notices,
            Some(vec![
                "1 decision retained — this run's scope could not be read; it is in the \
                 `retained` list, with its notice in `retained-notices`, of the file \
                 `lists-file` names"
                    .to_string()
            ])
        );
    }

    /// A run that retains nothing writes no file and keeps an empty list
    /// inline.
    #[test]
    fn a_run_that_retains_nothing_writes_no_file() {
        let tmp = windowed(&["bug: a finding — `src/new.rs`"]);
        let result = run(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(result.expired_count, 1);
        assert_eq!(
            (&result.retained, &result.lists_file),
            (&Some(Vec::new()), &None)
        );
        assert!(
            !tmp.path()
                .join(".ductus/results/process-decisions")
                .exists()
        );
    }
}
