//! `measure-reopenings` — whether a spec's review and analysis converged, from the
//! repository's history (spec 079, AC1, AC17, AC19). Its method is
//! `schema::descriptions::MEASURE_REOPENINGS`.
//!
//! A commit is judged against its own parents, never its walk-order neighbours, as
//! `check-stuck` does, so a merged side branch adds no transition. Every commit is walked,
//! since `git log -- path`'s history simplification drops a closing on a branch whose
//! merge kept the other side, and an adopter's work lives on branches. Every directory
//! seen counts, so a fold or a consolidation cannot erase a miss.
//!
//! The walk goes newest first and skips a commit past the window with no closing commit
//! waiting for its start, so `/{project}:status` reads trees and records only for the
//! window and the reopenings closed in it; the sorted walk still parses every commit
//! header once. A path is followed by name, so a renamed
//! spec's history starts at the rename. A reopening squashed into one commit shows no
//! pass: when its status moves to `done` from another status it is reported as not
//! counted, and from `done` to `done` nothing can show it.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use git2::{Commit, ErrorCode, ObjectType, Oid, Repository, Sort, Tree};
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::primitives::decisions::{RULE_DECIDER, RawDecision};
use crate::primitives::read_tasks::unchecked_lines;
use crate::primitives::{
    ANALYSIS_RECORD_FILE, ProjectRepository, REVIEW_RECORD_FILE, Result, flatten_line,
    frontmatter_status, parse_feature_dir, split_frontmatter,
};
use crate::schema::paths;
use crate::schema::primitives::{
    AnalyzeBlock, DecisionRecord, ListsForm, MeasureReopeningsArgs, MeasureReopeningsResult,
    NotCountedReason, NotCountedReopening, Reopening, ReopeningCounts, ReopeningState, ReviewBlock,
    RunDecisions,
};
use crate::session::{iso8601_utc, parse_iso8601_utc};

/// The runs of one kind a reopening may hold before each further run needs the
/// previous one to have left a gating finding (AC1).
const REOPENING_RUN_BOUND: usize = 1;

/// How far back reopenings count when the caller names no `since`, so an old miss
/// ages out of `/{project}:status` and `--since` still reaches it.
const REOPENING_WINDOW_DAYS: u64 = 30;

const SECONDS_PER_DAY: u64 = 86_400;
/// `YYYY-MM-DD`, the leading part of an ISO-8601 timestamp.
const DATE_LENGTH: usize = 10;

const IN_PROGRESS: &str = "in-progress";
const DONE: &str = "done";
const SPEC_FILE: &str = "spec.md";
const TASKS_FILE: &str = "tasks.md";

/// Execute the `measure-reopenings` primitive: [`compute`], returned inline while it
/// fits `[artifacts] read-size-bytes`, and past it with its lists in the file it
/// names and only their counts inline.
///
/// # Errors
///
/// Every error [`compute`] returns, and those `bounded_lists::place` returns for a
/// malformed read size or a list file that cannot be written.
pub fn run(args: &MeasureReopeningsArgs, repo: &Path) -> Result<MeasureReopeningsResult> {
    let measurement = compute(args, repo, SystemTime::now())?;
    let counts = measurement.counts();
    let mut result = MeasureReopeningsResult {
        state: measurement.state,
        since: measurement.since,
        reopenings: Some(measurement.reopenings.clone()),
        not_counted: Some(measurement.not_counted.clone()),
        waiting: Some(measurement.waiting.clone()),
        closed_count: counts.closed_count,
        over_bound_count: counts.over_bound_count,
        not_counted_count: counts.not_counted_count,
        waiting_count: counts.waiting_count,
        runs: counts.runs,
        form: ListsForm::Inline,
        lists_file: None,
    };
    let lists = serde_json::json!({
        "reopenings": measurement.reopenings,
        "not-counted": measurement.not_counted,
        "waiting": measurement.waiting,
    });
    if let Some(file) = super::bounded_lists::place(
        repo,
        "measure-reopenings",
        super::bounded_lists::ALL_SPECS,
        &result,
        &lists,
    )? {
        result.form = ListsForm::File;
        result.reopenings = None;
        result.not_counted = None;
        result.waiting = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// The counts `dashboard` carries, over the default window at `HEAD`. Never fails:
/// a measurement that could not run is the state [`ReopeningState::NotMeasured`]
/// with its reason, so `/{project}:status` still renders and says so.
pub(crate) fn counts(repo: &Path, now: SystemTime) -> ReopeningCounts {
    let args = MeasureReopeningsArgs {
        since: None,
        at: None,
    };
    match compute(&args, repo, now) {
        Ok(measurement) => measurement.counts(),
        Err(error) => ReopeningCounts {
            state: ReopeningState::NotMeasured,
            reason: Some(flatten_line(&error.to_string())),
            ..ReopeningCounts::default()
        },
    }
}

/// A measurement whole, before its result is bounded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Measurement {
    /// See [`MeasureReopeningsResult::state`].
    pub(crate) state: ReopeningState,
    /// See [`MeasureReopeningsResult::since`].
    pub(crate) since: String,
    /// See [`MeasureReopeningsResult::reopenings`].
    pub(crate) reopenings: Vec<Reopening>,
    /// See [`MeasureReopeningsResult::not_counted`].
    pub(crate) not_counted: Vec<NotCountedReopening>,
    /// See [`MeasureReopeningsResult::waiting`].
    pub(crate) waiting: Vec<String>,
}

impl Measurement {
    fn counts(&self) -> ReopeningCounts {
        ReopeningCounts {
            state: self.state,
            since: Some(self.since.clone()),
            closed_count: self.reopenings.len() + self.not_counted.len(),
            over_bound_count: self.reopenings.iter().filter(|r| r.over_bound).count(),
            not_counted_count: self.not_counted.len(),
            waiting_count: self.waiting.len(),
            runs: self.reopenings.iter().map(|r| r.reviews + r.analyses).sum(),
            reason: None,
        }
    }
}

/// Measure the reopenings closed in the window, at `now` for the default window.
///
/// # Errors
///
/// [`crate::primitives::PrimitiveError::Git`] for any libgit2 failure, among them a
/// `since` or `at` that names no commit, a project no work tree contains, and an
/// object a partial clone does not hold, so a walk that could not finish never
/// reads as a clean count.
pub(crate) fn compute(
    args: &MeasureReopeningsArgs,
    repo: &Path,
    now: SystemTime,
) -> Result<Measurement> {
    let layout = paths::Paths::load(repo);
    let project = ProjectRepository::discover(repo)?;
    let repository = &project.repository;
    let at = match &args.at {
        Some(reference) => repository.revparse_single(reference)?.peel_to_commit()?,
        None => match repository.head() {
            Ok(head) => head.peel_to_commit()?,
            Err(error) if error.code() == ErrorCode::UnbornBranch => {
                return Ok(Measurement {
                    state: ReopeningState::NothingToCompare,
                    since: default_since(now).0,
                    ..Measurement::default()
                });
            }
            Err(error) => return Err(error.into()),
        },
    };
    let mut boundary = Boundary::resolve(repository, args.since.as_deref(), &at, now)?;
    let mut history = History::new(repository, project.to_git(&layout.specs_root));
    let waiting = waiting_features(&mut history, &at)?;
    let since = boundary.text();
    if repository.is_shallow() {
        return Ok(Measurement {
            state: ReopeningState::ShallowHistory,
            since,
            waiting,
            ..Measurement::default()
        });
    }
    let (reopenings, not_counted) = walk(&mut history, &at, &mut boundary)?;
    let state = if reopenings.is_empty() && not_counted.is_empty() {
        ReopeningState::NothingToCompare
    } else {
        ReopeningState::Measured
    };
    Ok(Measurement {
        state,
        since,
        reopenings,
        not_counted,
        waiting,
    })
}

/// The default window's start: UTC midnight the window's days before `now`, so one
/// day's runs of `/{project}:status` agree, as the date the result reports and in
/// epoch seconds.
fn default_since(now: SystemTime) -> (String, i64) {
    let window = Duration::from_secs(REOPENING_WINDOW_DAYS * SECONDS_PER_DAY);
    let since = now.checked_sub(window).unwrap_or(UNIX_EPOCH);
    let seconds = epoch_seconds(since);
    let stamp = iso8601_utc(since);
    (
        stamp.get(..DATE_LENGTH).unwrap_or(&stamp).to_string(),
        seconds - seconds.rem_euclid(i64::try_from(SECONDS_PER_DAY).unwrap_or(1)),
    )
}

fn epoch_seconds(time: SystemTime) -> i64 {
    time.duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|elapsed| i64::try_from(elapsed.as_secs()).ok())
        .unwrap_or(0)
}

/// Which closing commits count, and which commits the walk can skip.
enum Boundary {
    /// Closing commits authored at or after this instant.
    Date { text: String, seconds: i64 },
    /// Closing commits `at` reaches and the ref does not. Named commits leave the
    /// set as the walk visits them, so an empty set means none is left to find.
    Commits {
        text: String,
        reachable: HashSet<Oid>,
    },
}

impl Boundary {
    /// `since` as a date first, since a ref is never spelled like one, then as a
    /// ref. An absent `since` is the default window.
    fn resolve(
        repository: &Repository,
        since: Option<&str>,
        at: &Commit<'_>,
        now: SystemTime,
    ) -> Result<Self> {
        let Some(text) = since else {
            let (text, seconds) = default_since(now);
            return Ok(Self::Date { text, seconds });
        };
        let date =
            parse_iso8601_utc(text).or_else(|| parse_iso8601_utc(&format!("{text}T00:00:00Z")));
        if let Some(date) = date {
            return Ok(Self::Date {
                text: text.to_string(),
                seconds: epoch_seconds(date),
            });
        }
        let commit = repository.revparse_single(text)?.peel_to_commit()?;
        let mut walk = repository.revwalk()?;
        walk.push(at.id())?;
        walk.hide(commit.id())?;
        Ok(Self::Commits {
            text: commit.id().to_string(),
            reachable: walk.collect::<std::result::Result<_, _>>()?,
        })
    }

    fn text(&self) -> String {
        match self {
            Self::Date { text, .. } | Self::Commits { text, .. } => text.clone(),
        }
    }

    /// Whether a reopening closed by `commit` counts. Called once per commit.
    fn holds(&mut self, commit: &Commit<'_>) -> bool {
        match self {
            Self::Date { seconds, .. } => commit.author().when().seconds() >= *seconds,
            Self::Commits { reachable, .. } => reachable.remove(&commit.id()),
        }
    }

    /// Whether the walk can skip `commit` while no pass is open: it is dated before the
    /// window, or no commit after a ref is left to visit. A commit's committer date is no earlier
    /// than its author date, so a commit before the date cannot be in the window.
    fn passed(&self, commit: &Commit<'_>) -> bool {
        match self {
            Self::Date { seconds, .. } => commit.time().seconds() < *seconds,
            Self::Commits { reachable, .. } => reachable.is_empty(),
        }
    }
}

/// One run of one kind, as its last write in the reopening states it.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Run {
    /// Its `last-run`.
    stamp: String,
    /// The run left a gating finding.
    gating: bool,
    /// The decisions stamped with this run's time that a person made.
    decisions_by_person: usize,
}

/// Each run's count of a person's decisions, oldest first.
fn run_decisions(record: DecisionRecord, runs: &[Run]) -> impl Iterator<Item = RunDecisions> + '_ {
    runs.iter().map(move |run| RunDecisions {
        record,
        last_run: run.stamp.clone(),
        decisions_by_person: run.decisions_by_person,
    })
}

/// A record as a tree holds it.
#[derive(Clone, Debug, PartialEq, Eq)]
enum RecordState {
    Absent,
    Unparseable,
    /// The record parses and names no run.
    NoRun,
    Run(Run),
}

impl RecordState {
    fn last_run(&self) -> Option<&str> {
        match self {
            Self::Run(run) => Some(run.stamp.as_str()),
            _ => None,
        }
    }
}

/// Both records of one version of a feature directory.
struct Records {
    review: RecordState,
    analysis: RecordState,
}

/// The decisions a record stores, read beside its own block.
#[derive(Deserialize)]
struct StoredDecisions {
    #[serde(default)]
    decisions: Vec<RawDecision>,
    #[serde(default)]
    waivers: Vec<StoredWaiver>,
    #[serde(default, rename = "round-waivers")]
    round_waivers: Vec<StoredWaiver>,
}

/// Who waived a rule or a fix round, and when.
#[derive(Deserialize)]
struct StoredWaiver {
    #[serde(default, rename = "waived-at")]
    waived_at: Option<String>,
    #[serde(default, rename = "waived-by")]
    waived_by: Option<String>,
}

impl StoredDecisions {
    /// What a person recorded at the run stamped `last_run`, the one count a record can
    /// support: a decision the rule made is not a person's, a round `[review]
    /// re-reads-off` waives is the config file's, and a person's decline with no reason
    /// leaves no entry at all (spec 079, AC17).
    fn by_person_at(&self, last_run: &str) -> usize {
        let at_this_run = |at: &Option<String>| at.as_deref().map(str::trim) == Some(last_run);
        let decisions = self
            .decisions
            .iter()
            .filter(|decision| at_this_run(&decision.decided_at))
            .filter(|decision| decision.decided_by.as_deref().map(str::trim) != Some(RULE_DECIDER))
            .count();
        let waivers = self
            .waivers
            .iter()
            .chain(&self.round_waivers)
            .filter(|waiver| at_this_run(&waiver.waived_at))
            .filter(|waiver| {
                !waiver.waived_by.as_deref().is_some_and(|by| {
                    paths::CONFIG_CHAIN
                        .iter()
                        .any(|file| by.trim().ends_with(file))
                })
            })
            .count();
        decisions + waivers
    }
}

/// Parse a record's bytes, with `fields` reading its `last-run` and whether it left
/// a gating finding.
fn record_state<T: DeserializeOwned>(
    bytes: &[u8],
    name: &str,
    fields: fn(&T) -> (Option<&str>, bool),
) -> RecordState {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return RecordState::Unparseable;
    };
    let Ok((frontmatter, _)) = split_frontmatter(text, Path::new(name)) else {
        return RecordState::Unparseable;
    };
    let (Ok(block), Ok(stored)) = (
        serde_norway::from_str::<T>(frontmatter),
        serde_norway::from_str::<StoredDecisions>(frontmatter),
    ) else {
        return RecordState::Unparseable;
    };
    let (last_run, gating) = fields(&block);
    let Some(last_run) = last_run else {
        return RecordState::NoRun;
    };
    RecordState::Run(Run {
        stamp: last_run.to_string(),
        gating,
        decisions_by_person: stored.by_person_at(last_run),
    })
}

fn review_fields(record: &ReviewBlock) -> (Option<&str>, bool) {
    let gating = record.blocking
        || record.must_violations > 0
        || record.should_violations > 0
        || record.contract_breaks.is_some_and(|breaks| breaks > 0)
        || record.fix_rounds_open == Some(true)
        || record.budget_stop == Some(true)
        || !record.skipped_passes.is_empty();
    (record.last_run.as_deref(), gating)
}

fn analysis_fields(record: &AnalyzeBlock) -> (Option<&str>, bool) {
    let gating = record.blocking || record.hard_fail > 0 || record.blocking_findings > 0;
    (record.last_run.as_deref(), gating)
}

/// The spec directories of a commit and what they hold, each read once per distinct
/// version rather than once per commit, since most commits share them.
struct History<'a> {
    repository: &'a Repository,
    specs_root: String,
    directories: HashMap<Oid, Rc<BTreeMap<String, Oid>>>,
    statuses: HashMap<Oid, Option<String>>,
    reviews: HashMap<Oid, RecordState>,
    analyses: HashMap<Oid, RecordState>,
}

impl<'a> History<'a> {
    fn new(repository: &'a Repository, specs_root: String) -> Self {
        Self {
            repository,
            specs_root,
            directories: HashMap::new(),
            statuses: HashMap::new(),
            reviews: HashMap::new(),
            analyses: HashMap::new(),
        }
    }

    /// Each feature directory `commit` holds, with its tree. Membership is
    /// `parse_feature_dir`'s, the corpus's single rule.
    fn directories_of(&mut self, commit: &Commit<'_>) -> Result<Rc<BTreeMap<String, Oid>>> {
        let tree = commit.tree()?;
        let root = match tree.get_path(Path::new(&self.specs_root)) {
            Ok(entry) if entry.kind() == Some(ObjectType::Tree) => entry.id(),
            Ok(_) => return Ok(Rc::default()),
            Err(error) if error.code() == ErrorCode::NotFound => return Ok(Rc::default()),
            Err(error) => return Err(error.into()),
        };
        if let Some(cached) = self.directories.get(&root) {
            return Ok(Rc::clone(cached));
        }
        // A tree that cannot be read is a walk that could not finish, never a commit
        // holding no feature directory.
        let specs = self.repository.find_tree(root)?;
        let found: BTreeMap<String, Oid> = specs
            .iter()
            .filter(|entry| entry.kind() == Some(ObjectType::Tree))
            .filter_map(|entry| Some((entry.name().ok()?.to_string(), entry.id())))
            .filter(|(name, _)| parse_feature_dir(name).is_some())
            .collect();
        let found = Rc::new(found);
        self.directories.insert(root, Rc::clone(&found));
        Ok(found)
    }

    /// The bytes of a file directly in a feature directory's tree, when it is there.
    fn file_in(&self, directory: Oid, name: &str) -> Result<Option<Vec<u8>>> {
        let tree = self.repository.find_tree(directory)?;
        let Some(entry) = tree.get_name(name) else {
            return Ok(None);
        };
        if entry.kind() != Some(ObjectType::Blob) {
            return Ok(None);
        }
        Ok(Some(
            self.repository.find_blob(entry.id())?.content().to_vec(),
        ))
    }

    /// The spec's `status:`, parsed once per `spec.md` blob, as `check-stuck`
    /// memoizes it: a directory changes far more often than its spec does.
    fn status_of(&mut self, directory: Oid) -> Result<Option<String>> {
        let tree = self.repository.find_tree(directory)?;
        let Some(entry) = tree.get_name(SPEC_FILE) else {
            return Ok(None);
        };
        if entry.kind() != Some(ObjectType::Blob) {
            return Ok(None);
        }
        if let Some(cached) = self.statuses.get(&entry.id()) {
            return Ok(cached.clone());
        }
        let blob = self.repository.find_blob(entry.id())?;
        let status = std::str::from_utf8(blob.content())
            .ok()
            .and_then(|text| frontmatter_status(text, Path::new(SPEC_FILE)));
        self.statuses.insert(entry.id(), status.clone());
        Ok(status)
    }

    /// Whether the reopening holding this entry into `in-progress` opened earlier: an
    /// earlier entry, or a `done` it has since left, on the first-parent chain. A trip
    /// through `draft` never reaches `done`, so it stays in one reopening.
    fn opened_earlier(&mut self, entry: &Commit<'_>, feature: &str) -> Result<bool> {
        let mut current = entry.parent(0).ok();
        while let Some(ancestor) = current {
            let directories = self.directories_of(&ancestor)?;
            let Some(directory) = directories.get(feature) else {
                return Ok(false);
            };
            if matches!(
                self.status_of(*directory)?.as_deref(),
                Some(IN_PROGRESS | DONE)
            ) {
                return Ok(true);
            }
            current = ancestor.parent(0).ok();
        }
        Ok(false)
    }

    /// Both records of one version of a feature directory, each parsed once per
    /// blob.
    fn records_of(&mut self, directory: Oid) -> Result<Records> {
        let tree = self.repository.find_tree(directory)?;
        Ok(Records {
            review: record_in(
                self.repository,
                &tree,
                REVIEW_RECORD_FILE,
                &mut self.reviews,
                review_fields,
            )?,
            analysis: record_in(
                self.repository,
                &tree,
                ANALYSIS_RECORD_FILE,
                &mut self.analyses,
                analysis_fields,
            )?,
        })
    }
}

/// The record named `name` in a feature directory's `tree`, through `cache`.
fn record_in<T: DeserializeOwned>(
    repository: &Repository,
    tree: &Tree<'_>,
    name: &str,
    cache: &mut HashMap<Oid, RecordState>,
    fields: fn(&T) -> (Option<&str>, bool),
) -> Result<RecordState> {
    let Some(entry) = tree.get_name(name) else {
        return Ok(RecordState::Absent);
    };
    if entry.kind() != Some(ObjectType::Blob) {
        return Ok(RecordState::Absent);
    }
    if let Some(cached) = cache.get(&entry.id()) {
        return Ok(cached.clone());
    }
    let blob = repository.find_blob(entry.id())?;
    let state = record_state(blob.content(), name, fields);
    cache.insert(entry.id(), state.clone());
    Ok(state)
}

/// The `in-progress` specs at `at` with no unchecked task, by the grammar every
/// tasks reader shares, so a template's example box inside a comment is no task. A
/// spec with no `tasks.md` has none. One that is not UTF-8 cannot be read, so it is
/// not claimed to have none.
fn waiting_features(history: &mut History<'_>, at: &Commit<'_>) -> Result<Vec<String>> {
    let directories = history.directories_of(at)?;
    let mut waiting = Vec::new();
    for (name, directory) in directories.iter() {
        if history.status_of(*directory)?.as_deref() != Some(IN_PROGRESS) {
            continue;
        }
        let no_unchecked_task = match history.file_in(*directory, TASKS_FILE)? {
            None => true,
            Some(bytes) => {
                String::from_utf8(bytes).is_ok_and(|text| unchecked_lines(&text).is_empty())
            }
        };
        if no_unchecked_task {
            waiting.push(name.clone());
        }
    }
    Ok(waiting)
}

/// A reopening whose closing commit the walk has met and whose start it has not.
struct OpenPass {
    end: Oid,
    reviews: Vec<Run>,
    analyses: Vec<Run>,
    unreadable: bool,
}

impl OpenPass {
    fn new(end: Oid) -> Self {
        Self {
            end,
            reviews: Vec::new(),
            analyses: Vec::new(),
            unreadable: false,
        }
    }

    /// Take in one version of the records. The walk runs newest first, so the first
    /// version naming a run is that run's last write, whose findings are the ones it
    /// left.
    fn observe(&mut self, records: &Records) {
        for (runs, state) in [
            (&mut self.reviews, &records.review),
            (&mut self.analyses, &records.analysis),
        ] {
            match state {
                RecordState::Unparseable => self.unreadable = true,
                RecordState::Run(run) if !runs.iter().any(|r| r.stamp == run.stamp) => {
                    runs.push(run.clone());
                }
                _ => {}
            }
        }
    }

    /// The reopening whole, given the commit that started it and the runs its parents
    /// already name, which belong to an earlier reopening.
    fn close(
        self,
        feature: &str,
        start: Oid,
        carried_in: &HashSet<String>,
    ) -> std::result::Result<Reopening, NotCountedReopening> {
        let not_counted = |reason| NotCountedReopening {
            feature: feature.to_string(),
            start: Some(start.to_string()),
            end: self.end.to_string(),
            reason,
        };
        if self.unreadable {
            return Err(not_counted(NotCountedReason::RecordUnparseable));
        }
        let oldest_first = |runs: &[Run]| -> Vec<Run> {
            runs.iter()
                .rev()
                .filter(|run| !carried_in.contains(&run.stamp))
                .cloned()
                .collect()
        };
        let reviews = oldest_first(&self.reviews);
        let analyses = oldest_first(&self.analyses);
        if reviews.is_empty() && analyses.is_empty() {
            return Err(not_counted(NotCountedReason::NoRecordedRun));
        }
        Ok(Reopening {
            feature: feature.to_string(),
            start: start.to_string(),
            end: self.end.to_string(),
            reviews: reviews.len(),
            analyses: analyses.len(),
            over_bound: over_bound(&reviews) || over_bound(&analyses),
            decisions_by_person: reviews
                .iter()
                .chain(&analyses)
                .map(|run| run.decisions_by_person)
                .sum(),
            run_decisions: run_decisions(DecisionRecord::Review, &reviews)
                .chain(run_decisions(DecisionRecord::Analysis, &analyses))
                .collect(),
        })
    }

    /// A closing commit with no in-progress pass to delimit it.
    fn without_a_start(self, feature: &str) -> NotCountedReopening {
        NotCountedReopening {
            feature: feature.to_string(),
            start: None,
            end: self.end.to_string(),
            reason: NotCountedReason::NoInProgressCommit,
        }
    }
}

/// Whether a run past the bound followed a run of its kind that left no gating
/// finding.
fn over_bound(oldest_first: &[Run]) -> bool {
    oldest_first
        .windows(2)
        .skip(REOPENING_RUN_BOUND - 1)
        .any(|pair| !pair[0].gating)
}

/// Walk from `at`, newest first, delimiting each reopening the boundary counts. A
/// pass is opened only under an `in-progress` parent, and the shallow case returned
/// earlier, so the walk reaches a commit entering `in-progress` before history ends
/// and no pass is left open.
fn walk(
    history: &mut History<'_>,
    at: &Commit<'_>,
    boundary: &mut Boundary,
) -> Result<(Vec<Reopening>, Vec<NotCountedReopening>)> {
    let repository = history.repository;
    let mut revwalk = repository.revwalk()?;
    revwalk.push(at.id())?;
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;

    let mut reopenings = Vec::new();
    let mut not_counted = Vec::new();
    let mut open: HashMap<String, OpenPass> = HashMap::new();
    for oid in revwalk {
        let commit = repository.find_commit(oid?)?;
        let counts = boundary.holds(&commit);
        // A commit past the window ends nothing: a child can be dated older than its
        // parent, as an imported or date-preserving rebase's is, so what lies behind it
        // may still be inside.
        if !counts && open.is_empty() && boundary.passed(&commit) {
            continue;
        }
        let directories = history.directories_of(&commit)?;
        let parents: Vec<Rc<BTreeMap<String, Oid>>> = commit
            .parents()
            .map(|parent| history.directories_of(&parent))
            .collect::<Result<_>>()?;
        for (feature, directory) in directories.iter() {
            if parents
                .iter()
                .all(|parent| parent.get(feature) == Some(directory))
                && !parents.is_empty()
            {
                continue;
            }
            let status = history.status_of(*directory)?;
            let mut parent_statuses = Vec::new();
            for parent in &parents {
                if let Some(earlier) = parent.get(feature) {
                    parent_statuses.push(history.status_of(*earlier)?);
                }
            }
            let any_parent =
                |wanted: &str| parent_statuses.iter().any(|s| s.as_deref() == Some(wanted));

            let closing =
                status.as_deref() == Some(DONE) && !parent_statuses.is_empty() && !any_parent(DONE);
            if closing {
                if let Some(earlier) = open.remove(feature) {
                    not_counted.push(earlier.without_a_start(feature));
                }
                if counts {
                    let pass = OpenPass::new(commit.id());
                    if any_parent(IN_PROGRESS) {
                        open.insert(feature.clone(), pass);
                    } else {
                        not_counted.push(pass.without_a_start(feature));
                    }
                }
            }

            let Some(pass) = open.get_mut(feature) else {
                continue;
            };
            pass.observe(&history.records_of(*directory)?);
            let entered = status.as_deref() == Some(IN_PROGRESS) && !any_parent(IN_PROGRESS);
            let left_done =
                status.as_deref() != Some(DONE) && any_parent(DONE) && !any_parent(IN_PROGRESS);
            if left_done || (entered && !history.opened_earlier(&commit, feature)?) {
                let mut carried_in = HashSet::new();
                for parent in &parents {
                    if let Some(earlier) = parent.get(feature) {
                        let records = history.records_of(*earlier)?;
                        for state in [&records.review, &records.analysis] {
                            carried_in.extend(state.last_run().map(str::to_string));
                        }
                    }
                }
                if let Some(pass) = open.remove(feature) {
                    match pass.close(feature, commit.id(), &carried_in) {
                        Ok(reopening) => reopenings.push(reopening),
                        Err(uncounted) => not_counted.push(uncounted),
                    }
                }
            }
        }
    }
    // The walk met them newest first, and a feature's come in closing order.
    reopenings.reverse();
    not_counted.reverse();
    reopenings.sort_by(|a, b| a.feature.cmp(&b.feature));
    not_counted.sort_by(|a, b| a.feature.cmp(&b.feature));
    Ok((reopenings, not_counted))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::PrimitiveError;
    use crate::primitives::git_fixture::{commit_all, subdirectory_project, write};
    use git2::{IndexAddOption, Signature, Time};
    use std::path::PathBuf;

    const FEATURE: &str = "012-a";
    /// A date all but the dated tests' commits fall after.
    const LONG_AGO: &str = "2000-01-01";

    fn spec(status: &str) -> String {
        format!("---\nstatus: {status}\ndependencies: []\n---\n\n# 012\n")
    }

    fn review(last_run: &str, extra: &str) -> String {
        format!("---\nspec: {FEATURE}\nlast-run: {last_run}\n{extra}---\n\n# Review\n")
    }

    fn analysis(last_run: &str, extra: &str) -> String {
        format!("---\nspec: {FEATURE}\nlast-run: {last_run}\n{extra}---\n\n# Analysis\n")
    }

    /// A repository holding spec 012 at `planned`, its working copy under `root`.
    struct Fixture {
        _dir: tempfile::TempDir,
        root: PathBuf,
        repository: Repository,
    }

    impl Fixture {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().to_path_buf();
            let repository = Repository::init(&root).unwrap();
            let fixture = Self {
                _dir: dir,
                root,
                repository,
            };
            fixture.write("spec.md", &spec("planned"));
            commit_all(&fixture.repository, "plan(012): plan");
            fixture
        }

        /// As [`Self::new`], with an earlier commit the spec is absent from.
        fn after_an_earlier_commit() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().to_path_buf();
            let repository = Repository::init(&root).unwrap();
            write(&root.join("README.md"), "earlier\n");
            commit_all(&repository, "chore: earlier");
            let fixture = Self {
                _dir: dir,
                root,
                repository,
            };
            fixture.write("spec.md", &spec("planned"));
            commit_all(&fixture.repository, "plan(012): plan");
            fixture
        }

        fn write(&self, file: &str, body: &str) {
            write(&self.root.join("specs").join(FEATURE).join(file), body);
        }

        fn commit(&self, file: &str, body: &str) -> Oid {
            self.write(file, body);
            commit_all(&self.repository, "work(012): step")
        }

        fn status(&self, status: &str) -> Oid {
            self.commit("spec.md", &spec(status))
        }

        /// Make the commit at an exact time, for its author and committer alike.
        fn commit_at(&self, seconds: i64, parents: &[Oid], update_head: bool) -> Oid {
            let mut index = self.repository.index().unwrap();
            index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
            index.write().unwrap();
            let tree = self
                .repository
                .find_tree(index.write_tree().unwrap())
                .unwrap();
            let signature =
                Signature::new("Test", "test@example.com", &Time::new(seconds, 0)).unwrap();
            let commits: Vec<Commit<'_>> = parents
                .iter()
                .map(|oid| self.repository.find_commit(*oid).unwrap())
                .collect();
            let refs: Vec<&Commit<'_>> = commits.iter().collect();
            self.repository
                .commit(
                    update_head.then_some("HEAD"),
                    &signature,
                    &signature,
                    "work(012): dated",
                    &tree,
                    &refs,
                )
                .unwrap()
        }

        fn head(&self) -> Oid {
            self.repository.head().unwrap().target().unwrap()
        }

        fn measure(&self) -> Measurement {
            self.measure_since(LONG_AGO)
        }

        fn measure_since(&self, since: &str) -> Measurement {
            compute(
                &MeasureReopeningsArgs {
                    since: Some(since.to_string()),
                    at: None,
                },
                &self.root,
                SystemTime::now(),
            )
            .unwrap()
        }

        /// The one closed reopening, counted.
        fn only_reopening(&self) -> Reopening {
            let measurement = self.measure();
            assert_eq!(measurement.state, ReopeningState::Measured);
            assert_eq!(measurement.not_counted, Vec::new());
            assert_eq!(measurement.reopenings.len(), 1, "{measurement:?}");
            measurement.reopenings[0].clone()
        }

        /// The one closed reopening, not counted.
        fn only_not_counted(&self) -> NotCountedReopening {
            let measurement = self.measure();
            assert_eq!(measurement.state, ReopeningState::Measured);
            assert_eq!(measurement.reopenings, Vec::new());
            assert_eq!(measurement.not_counted.len(), 1, "{measurement:?}");
            measurement.not_counted[0].clone()
        }
    }

    #[test]
    fn one_review_and_one_analysis_are_within_the_bound() {
        let fixture = Fixture::new();
        let start = fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.commit("analysis.md", &analysis("2026-01-01T00:00:01Z", ""));
        let end = fixture.status("done");
        assert_eq!(
            fixture.only_reopening(),
            Reopening {
                feature: FEATURE.into(),
                start: start.to_string(),
                end: end.to_string(),
                reviews: 1,
                analyses: 1,
                over_bound: false,
                decisions_by_person: 0,
                run_decisions: vec![
                    RunDecisions {
                        record: DecisionRecord::Review,
                        last_run: "2026-01-01T00:00:00Z".to_string(),
                        decisions_by_person: 0,
                    },
                    RunDecisions {
                        record: DecisionRecord::Analysis,
                        last_run: "2026-01-01T00:00:01Z".to_string(),
                        decisions_by_person: 0,
                    },
                ],
            }
        );
    }

    #[test]
    fn a_second_clean_run_of_either_kind_is_over_the_bound() {
        for (file, write_record) in [
            ("review.md", review as fn(&str, &str) -> String),
            ("analysis.md", analysis),
        ] {
            let fixture = Fixture::new();
            fixture.status("in-progress");
            fixture.commit(file, &write_record("2026-01-01T00:00:00Z", ""));
            fixture.commit(file, &write_record("2026-01-02T00:00:00Z", ""));
            fixture.status("done");
            assert!(fixture.only_reopening().over_bound, "{file}");
            assert_eq!(fixture.measure().counts().over_bound_count, 1, "{file}");
        }
    }

    /// A finished run's first write carries the mark and its last clears it, so only a
    /// run cut off with the mark still set leaves the gate demanding another.
    #[test]
    fn a_run_whose_last_write_cleared_the_mark_left_nothing() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        let first = "2026-01-01T00:00:00Z";
        fixture.commit("review.md", &review(first, "fix-rounds-open: true\n"));
        fixture.commit("review.md", &review(first, ""));
        fixture.commit("review.md", &review("2026-01-02T00:00:00Z", ""));
        fixture.status("done");

        let reopening = fixture.only_reopening();
        assert_eq!(reopening.reviews, 2);
        assert!(reopening.over_bound);
    }

    /// A record that says it stopped for neither reason left nothing, so a second clean
    /// review after it is over the bound.
    #[test]
    fn a_finished_run_recording_no_stop_left_nothing() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        let finished = "budget-stop: false\nskipped-passes: []\n";
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", finished));
        fixture.commit("review.md", &review("2026-01-02T00:00:00Z", ""));
        fixture.status("done");
        assert!(fixture.only_reopening().over_bound);
    }

    #[test]
    fn a_second_run_after_a_gating_finding_is_within_the_bound() {
        let cases = [
            ("review.md", "blocking: true\n"),
            ("review.md", "must-violations: 1\n"),
            ("review.md", "should-violations: 1\n"),
            ("review.md", "contract-breaks: 1\n"),
            ("review.md", "fix-rounds-open: true\n"),
            ("review.md", "budget-stop: true\n"),
            ("review.md", "skipped-passes: [security]\n"),
            ("analysis.md", "blocking: true\n"),
            ("analysis.md", "hard-fail: 1\n"),
            ("analysis.md", "blocking-findings: 1\n"),
        ];
        for (file, finding) in cases {
            let fixture = Fixture::new();
            fixture.status("in-progress");
            let record = if file == "review.md" {
                review
            } else {
                analysis
            };
            fixture.commit(file, &record("2026-01-01T00:00:00Z", finding));
            fixture.commit(file, &record("2026-01-02T00:00:00Z", ""));
            fixture.status("done");
            assert!(!fixture.only_reopening().over_bound, "{file} {finding}");
        }
    }

    #[test]
    fn a_non_gating_count_does_not_excuse_a_second_run() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit(
            "review.md",
            &review("2026-01-01T00:00:00Z", "low-confidence: 3\n"),
        );
        fixture.commit("review.md", &review("2026-01-02T00:00:00Z", ""));
        fixture.status("done");
        assert!(fixture.only_reopening().over_bound);
    }

    /// Each further run needs the run before it to have left a gating finding, so
    /// an early finding does not excuse a later clean run's successor.
    #[test]
    fn a_third_run_after_a_clean_second_is_over_the_bound() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit(
            "review.md",
            &review("2026-01-01T00:00:00Z", "must-violations: 1\n"),
        );
        fixture.commit("review.md", &review("2026-01-02T00:00:00Z", ""));
        fixture.commit("review.md", &review("2026-01-03T00:00:00Z", ""));
        fixture.status("done");
        let reopening = fixture.only_reopening();
        assert_eq!(reopening.reviews, 3);
        assert!(reopening.over_bound);
    }

    /// A review's two writes share one `last-run`, so they are one run, and what it
    /// left is its last write's count: a finding fixed in the run excuses nothing.
    #[test]
    fn a_runs_two_writes_are_one_run_judged_by_the_last() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        let when = "2026-01-01T00:00:00Z";
        fixture.commit("review.md", &review(when, "must-violations: 1\n"));
        fixture.commit("review.md", &review(when, ""));
        fixture.status("done");
        let reopening = fixture.only_reopening();
        assert_eq!(reopening.reviews, 1);
        assert!(!reopening.over_bound);

        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review(when, "must-violations: 1\n"));
        fixture.commit("review.md", &review(when, ""));
        fixture.commit("review.md", &review("2026-01-02T00:00:00Z", ""));
        fixture.status("done");
        assert!(fixture.only_reopening().over_bound);
    }

    #[test]
    fn a_reopening_with_no_run_is_not_counted() {
        let fixture = Fixture::new();
        let start = fixture.status("in-progress");
        let end = fixture.status("done");
        assert_eq!(
            fixture.only_not_counted(),
            NotCountedReopening {
                feature: FEATURE.into(),
                start: Some(start.to_string()),
                end: end.to_string(),
                reason: NotCountedReason::NoRecordedRun,
            }
        );
        assert_eq!(fixture.measure().counts().closed_count, 1);
    }

    /// A record carried in from the reopening before names no run of this one.
    #[test]
    fn a_record_from_an_earlier_reopening_is_no_run_of_this_one() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.commit("analysis.md", &analysis("2026-01-01T00:00:01Z", ""));
        fixture.status("done");
        fixture.status("in-progress");
        fixture.status("done");
        let measurement = fixture.measure();
        assert_eq!(measurement.reopenings.len(), 1);
        assert_eq!(measurement.not_counted.len(), 1);
        assert_eq!(
            measurement.not_counted[0].reason,
            NotCountedReason::NoRecordedRun
        );
        assert_eq!(measurement.counts().closed_count, 2);
        assert_eq!(measurement.counts().runs, 2);
    }

    /// The carried value is excluded by name, so a run in the reopening still counts
    /// and judges the second run of its own reopening alone.
    #[test]
    fn a_run_in_a_later_reopening_is_counted_beside_a_carried_record() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-02T00:00:00Z", ""));
        fixture.status("done");
        let measurement = fixture.measure();
        assert_eq!(measurement.reopenings.len(), 2, "{measurement:?}");
        assert!(measurement.reopenings.iter().all(|r| r.reviews == 1));
        assert!(measurement.reopenings.iter().all(|r| !r.over_bound));
    }

    /// Work committed with the back-edge flip belongs to the window it opens.
    #[test]
    fn a_run_committed_with_the_reopening_flip_counts() {
        let fixture = Fixture::new();
        fixture.write("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("in-progress");
        fixture.status("done");
        assert_eq!(fixture.only_reopening().reviews, 1);
    }

    /// A run committed with the closing flip is inside the reopening, and one
    /// committed after it is not.
    #[test]
    fn the_closing_commit_is_inside_and_a_later_one_is_not() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.write("analysis.md", &analysis("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        fixture.commit("review.md", &review("2026-01-02T00:00:00Z", ""));
        let reopening = fixture.only_reopening();
        assert_eq!(reopening.analyses, 1);
        assert_eq!(reopening.reviews, 0);
    }

    #[test]
    fn a_record_that_does_not_parse_is_not_counted() {
        for damaged in ["no frontmatter\n", "---\nlast-run: [\n---\n\n# Review\n"] {
            let fixture = Fixture::new();
            fixture.status("in-progress");
            fixture.commit("review.md", damaged);
            fixture.status("done");
            assert_eq!(
                fixture.only_not_counted().reason,
                NotCountedReason::RecordUnparseable,
                "{damaged}"
            );
        }
    }

    #[test]
    fn decisions_that_do_not_parse_are_not_counted() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit(
            "analysis.md",
            &analysis("2026-01-01T00:00:00Z", "decisions: not-a-list\n"),
        );
        fixture.status("done");
        assert_eq!(
            fixture.only_not_counted().reason,
            NotCountedReason::RecordUnparseable
        );
    }

    /// An earlier write that did not parse leaves the run's findings unknown, even
    /// though a later write of the record does parse.
    #[test]
    fn a_damaged_write_anywhere_in_the_reopening_leaves_it_uncounted() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", "no frontmatter\n");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        assert_eq!(
            fixture.only_not_counted().reason,
            NotCountedReason::RecordUnparseable
        );
    }

    #[test]
    fn a_record_naming_no_run_is_no_run() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit(
            "review.md",
            "---\nspec: 012-a\nlast-run: null\n---\n\n# Review\n",
        );
        fixture.status("done");
        assert_eq!(
            fixture.only_not_counted().reason,
            NotCountedReason::NoRecordedRun
        );
    }

    #[test]
    fn decisions_by_person_counts_a_persons_waivers_at_that_run_and_not_the_configs() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        let when = "2026-01-01T00:00:00Z";
        let older = "2025-01-01T00:00:00Z";
        let waiver = |at: &str, by: &str| format!("    waived-at: {at}\n    waived-by: {by}\n");
        let fields = format!(
            "waivers:\n  - rule: R\n    file: f\n    reason: r\n{}  - rule: R\n    file: f\n    reason: r\n{}round-waivers:\n  - round: 1\n    reason: r\n{}  - round: 2\n    reason: r\n{}  - round: 3\n    reason: r\n{}",
            waiver(when, "owner@example.com"),
            waiver(older, "owner@example.com"),
            waiver(when, "owner@example.com"),
            waiver(when, ".ductus/config.toml"),
            waiver(older, "owner@example.com"),
        );
        fixture.commit("review.md", &review(when, &fields));
        fixture.status("done");
        let reopening = fixture.only_reopening();
        assert_eq!(reopening.decisions_by_person, 2);
        assert_eq!(reopening.run_decisions[0].decisions_by_person, 2);
    }

    #[test]
    fn decisions_by_person_counts_only_a_runs_own_and_never_the_rules() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        let when = "2026-01-01T00:00:00Z";
        let decision = |at: &str, by: &str| {
            format!(
                "  - key: k\n    outcome: waived\n    reason: r\n    decided-at: {at}\n    decided-by: {by}\n"
            )
        };
        let decisions = format!(
            "decisions:\n{}{}{}",
            decision(when, "owner@example.com"),
            decision(when, RULE_DECIDER),
            decision("2025-01-01T00:00:00Z", "owner@example.com"),
        );
        fixture.commit("review.md", &review(when, &decisions));
        let analysis_when = "2026-01-01T00:00:01Z";
        let analysis_decisions =
            format!("decisions:\n{}", decision(analysis_when, "a@example.com"));
        fixture.commit("analysis.md", &analysis(analysis_when, &analysis_decisions));
        fixture.status("done");
        let reopening = fixture.only_reopening();
        assert_eq!(reopening.decisions_by_person, 2);
        assert_eq!(
            reopening.run_decisions,
            [
                RunDecisions {
                    record: DecisionRecord::Review,
                    last_run: when.to_string(),
                    decisions_by_person: 1,
                },
                RunDecisions {
                    record: DecisionRecord::Analysis,
                    last_run: analysis_when.to_string(),
                    decisions_by_person: 1,
                },
            ]
        );
    }

    #[test]
    fn no_closed_reopening_is_nothing_to_compare() {
        let fixture = Fixture::new();
        assert_eq!(fixture.measure().state, ReopeningState::NothingToCompare);
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        let measurement = fixture.measure();
        assert_eq!(measurement.state, ReopeningState::NothingToCompare);
        assert_eq!(measurement.counts().closed_count, 0);
    }

    #[test]
    fn a_repository_with_no_commit_is_nothing_to_compare() {
        let dir = tempfile::tempdir().unwrap();
        Repository::init(dir.path()).unwrap();
        let measurement = compute(
            &MeasureReopeningsArgs {
                since: None,
                at: None,
            },
            dir.path(),
            SystemTime::now(),
        )
        .unwrap();
        assert_eq!(measurement.state, ReopeningState::NothingToCompare);
    }

    /// A shallow clone cannot hold its status commits, so it says it cannot measure
    /// though `waiting`, which reads one tree, still answers.
    #[test]
    fn a_shallow_history_says_it_cannot_measure() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        let head = fixture.status("done");
        fixture.status("in-progress");
        let cut = fixture.head();
        assert_ne!(cut, head);
        std::fs::write(
            fixture.repository.path().join("shallow"),
            format!("{cut}\n"),
        )
        .unwrap();
        let reopened = Repository::open(&fixture.root).unwrap();
        assert!(reopened.is_shallow());
        let measurement = fixture.measure();
        assert_eq!(measurement.state, ReopeningState::ShallowHistory);
        assert_eq!(measurement.reopenings, Vec::new());
        assert_eq!(measurement.waiting, vec![FEATURE.to_string()]);
    }

    /// A tip dated before the window, as a date-preserving rebase's or an imported
    /// commit's is, hides nothing behind it, so a closing there still counts.
    #[test]
    fn a_commit_dated_before_the_window_ends_nothing() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.commit("analysis.md", &analysis("2026-01-01T00:00:01Z", ""));
        fixture.status("done");
        let tip = fixture.head();
        fixture.commit_at(631_152_000, &[tip], true);
        let measurement = fixture.measure_since("2000-01-01");
        assert_eq!(
            measurement.state,
            ReopeningState::Measured,
            "{measurement:?}"
        );
        assert_eq!(measurement.reopenings.len(), 1, "{measurement:?}");
    }

    #[test]
    fn waiting_is_the_in_progress_specs_with_no_unchecked_task() {
        let fixture = Fixture::new();
        let write_spec = |name: &str, status: &str, tasks: Option<&str>| {
            let dir = fixture.root.join("specs").join(name);
            write(&dir.join("spec.md"), &spec(status));
            if let Some(tasks) = tasks {
                write(&dir.join("tasks.md"), tasks);
            }
        };
        write_spec(
            "020-all-ticked",
            "in-progress",
            Some("## 1. A\n\n- [x] a\n"),
        );
        write_spec("021-no-tasks", "in-progress", None);
        write_spec(
            "022-template",
            "in-progress",
            Some("# T\n\n<!--\n## 1. Example\n\n- [ ] an example\n-->\n\n## 1. A\n\n- [x] a\n"),
        );
        write_spec("023-pending", "in-progress", Some("## 1. A\n\n- [ ] a\n"));
        write_spec(
            "024-clause",
            "in-progress",
            Some("## 1. A\n\n- [x] a\n- [ ] Done when: it is.\n"),
        );
        write_spec("025-done", "done", Some("## 1. A\n\n- [x] a\n"));
        write_spec("026-planned", "planned", None);
        write_spec(
            "027-unnumbered",
            "in-progress",
            Some("## 1. A\n\n- [x] a\n\n## Later\n\n- [ ] owed\n"),
        );
        write_spec("rules", "in-progress", None);
        commit_all(&fixture.repository, "chore: seed");
        assert_eq!(
            fixture.measure().waiting,
            vec!["020-all-ticked", "021-no-tasks", "022-template"]
        );
    }

    #[test]
    fn a_date_counts_by_the_closing_commits_author_date() {
        let (before, since, after) = (1_700_000_000, "2023-11-15", 1_700_100_000);
        let fixture = Fixture::new();
        let mut parent = fixture.head();
        // A reopening closed before the date, and one started before it and closed
        // after it, with its run before the date.
        for (status, at) in [
            ("in-progress", before - 400),
            ("done", before - 300),
            ("in-progress", before - 200),
        ] {
            fixture.write("spec.md", &spec(status));
            parent = fixture.commit_at(at, &[parent], true);
        }
        fixture.write("review.md", &review("2023-11-14T00:00:00Z", ""));
        parent = fixture.commit_at(before - 100, &[parent], true);
        fixture.write("spec.md", &spec("done"));
        fixture.commit_at(after, &[parent], true);
        let measurement = fixture.measure_since(since);
        assert_eq!(measurement.reopenings.len(), 1, "{measurement:?}");
        assert_eq!(measurement.reopenings[0].reviews, 1);
        assert_eq!(measurement.not_counted, Vec::new());
        assert_eq!(measurement.since, since);
    }

    /// The walk goes on past the date while one spec's closing awaits its start, and
    /// another spec's closing met on the way is still before the date.
    #[test]
    fn a_closing_before_the_date_stays_out_while_another_awaits_its_start() {
        let (before, since, after) = (1_700_000_000, "2023-11-15", 1_700_100_000);
        let fixture = Fixture::new();
        let mut parent = fixture.head();
        let other = fixture.root.join("specs/034-b/spec.md");
        fixture.write("spec.md", &spec("in-progress"));
        parent = fixture.commit_at(before - 300, &[parent], true);
        write(&other, &spec("in-progress"));
        parent = fixture.commit_at(before - 200, &[parent], true);
        write(&other, &spec("done"));
        parent = fixture.commit_at(before - 100, &[parent], true);
        fixture.write("spec.md", &spec("done"));
        fixture.commit_at(after, &[parent], true);
        let measurement = fixture.measure_since(since);
        assert_eq!(measurement.reopenings, Vec::new());
        assert_eq!(measurement.not_counted.len(), 1, "{measurement:?}");
        assert_eq!(measurement.not_counted[0].feature, FEATURE);
    }

    /// A specs tree the repository cannot read is a walk that could not finish, so
    /// it reports an error, never a commit holding no feature directory.
    #[test]
    fn a_specs_tree_that_cannot_be_read_is_an_error_not_an_empty_history() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        let tree = fixture.repository.head().unwrap().peel_to_tree().unwrap();
        let specs = tree.get_path(Path::new("specs")).unwrap().id().to_string();
        let (directory, file) = specs.split_at(2);
        let objects = fixture.repository.path().join("objects");
        std::fs::remove_file(objects.join(directory).join(file)).unwrap();

        let error = compute(
            &MeasureReopeningsArgs {
                since: Some("2020-01-01".to_string()),
                at: None,
            },
            &fixture.root,
            SystemTime::now(),
        );
        assert!(matches!(error, Err(PrimitiveError::Git(_))), "{error:?}");
    }

    #[test]
    fn a_full_timestamp_is_a_date_too() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        assert_eq!(
            fixture
                .measure_since("2000-01-01T00:00:00Z")
                .reopenings
                .len(),
            1
        );
        assert_eq!(
            fixture
                .measure_since("2999-01-01T00:00:00Z")
                .reopenings
                .len(),
            0
        );
    }

    /// The default window is the constant's days before `now`: a closing inside it
    /// counts and one before it does not.
    #[test]
    fn the_default_window_counts_back_from_now() {
        let fixture = Fixture::new();
        let mut parent = fixture.head();
        let now = 1_800_000_000_i64;
        let day = 86_400;
        let window = i64::try_from(REOPENING_WINDOW_DAYS).unwrap();
        for (status, at) in [
            ("in-progress", now - (window + 3) * day),
            ("done", now - (window + 2) * day),
            ("in-progress", now - 3 * day),
        ] {
            fixture.write("spec.md", &spec(status));
            parent = fixture.commit_at(at, &[parent], true);
        }
        fixture.write("review.md", &review("2027-01-01T00:00:00Z", ""));
        parent = fixture.commit_at(now - 2 * day, &[parent], true);
        fixture.write("spec.md", &spec("done"));
        fixture.commit_at(now - day, &[parent], true);
        let measurement = compute(
            &MeasureReopeningsArgs {
                since: None,
                at: None,
            },
            &fixture.root,
            UNIX_EPOCH + Duration::from_secs(u64::try_from(now).unwrap()),
        )
        .unwrap();
        assert_eq!(measurement.reopenings.len(), 1, "{measurement:?}");
        assert_eq!(measurement.not_counted, Vec::new());
    }

    /// `--since` a ref counts what `at` reaches and the ref does not: AC20's sample,
    /// from the landing commit on, though the closing's start is before it.
    #[test]
    fn a_ref_counts_the_closings_after_it() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.status("done");
        fixture.status("in-progress");
        let landing = fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        let measurement = fixture.measure_since(&landing.to_string());
        assert_eq!(measurement.reopenings.len(), 1, "{measurement:?}");
        assert_eq!(measurement.reopenings[0].reviews, 1);
        assert_eq!(measurement.not_counted, Vec::new());
        assert_eq!(measurement.since, landing.to_string());
    }

    /// With a ref, the walk skips the rest once every commit after it has been met, and a
    /// date's own test does not apply.
    #[test]
    fn a_ref_at_head_counts_nothing() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        let head = fixture.status("done");
        let measurement = fixture.measure_since(&head.to_string());
        assert_eq!(measurement.state, ReopeningState::NothingToCompare);
    }

    /// `at` measures as of that commit: a later closing and a later status do not
    /// exist there.
    #[test]
    fn at_measures_as_of_that_commit() {
        let fixture = Fixture::new();
        let start = fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        let first_close = fixture.status("done");
        fixture.status("in-progress");
        fixture.status("done");
        let measurement = compute(
            &MeasureReopeningsArgs {
                since: Some(LONG_AGO.into()),
                at: Some(first_close.to_string()),
            },
            &fixture.root,
            SystemTime::now(),
        )
        .unwrap();
        assert_eq!(measurement.reopenings.len(), 1);
        assert_eq!(measurement.reopenings[0].start, start.to_string());
        assert_eq!(measurement.not_counted, Vec::new());

        let at_start = compute(
            &MeasureReopeningsArgs {
                since: Some(LONG_AGO.into()),
                at: Some(start.to_string()),
            },
            &fixture.root,
            SystemTime::now(),
        )
        .unwrap();
        assert_eq!(at_start.state, ReopeningState::NothingToCompare);
        assert_eq!(at_start.waiting, vec![FEATURE.to_string()]);
    }

    /// A trip through `draft` never reaches `done`, so it is one reopening, its runs on
    /// both sides counted together, and a second clean review is over the bound.
    #[test]
    fn a_trip_through_draft_is_one_reopening() {
        let fixture = Fixture::new();
        let start = fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("draft");
        fixture.status("planned");
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-02T00:00:00Z", ""));
        let end = fixture.status("done");
        let reopening = fixture.only_reopening();
        assert_eq!(reopening.start, start.to_string());
        assert_eq!(reopening.end, end.to_string());
        assert_eq!(reopening.reviews, 2);
        assert!(reopening.over_bound);
    }

    /// The back-edge from `done` itself: the reopening starts where it left `done`, so
    /// a run recorded before it returned to `in-progress` is this reopening's.
    #[test]
    fn leaving_done_for_draft_starts_the_reopening() {
        let fixture = Fixture::new();
        fixture.status("done");
        let start = fixture.status("draft");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("in-progress");
        let end = fixture.status("done");
        let reopenings: Vec<_> = fixture
            .measure()
            .reopenings
            .into_iter()
            .filter(|r| r.end == end.to_string())
            .collect();
        assert_eq!(reopenings.len(), 1, "{reopenings:?}");
        assert_eq!(reopenings[0].start, start.to_string());
        assert_eq!(reopenings[0].reviews, 1);
    }

    /// A merge carrying a side branch's entry starts nothing, though its first parent
    /// had not entered `in-progress`: the entry on the branch does.
    #[test]
    fn a_merge_carrying_an_entry_from_its_second_parent_starts_nothing() {
        let fixture = Fixture::new();
        let planned = fixture.repository.head().unwrap().target().unwrap();
        fixture.write("spec.md", &spec("in-progress"));
        let start = fixture.commit_at(100, &[planned], false);
        fixture.write("review.md", &review("2026-01-01T00:00:00Z", ""));
        let side = fixture.commit_at(101, &[start], false);
        fixture.write("spec.md", &spec("planned"));
        fixture.write("notes.md", "main\n");
        let main = fixture.commit_at(102, &[planned], true);
        fixture.write("spec.md", &spec("in-progress"));
        fixture.commit_at(103, &[main, side], true);
        let end = fixture.status("done");
        let reopening = fixture.only_reopening();
        assert_eq!(reopening.start, start.to_string());
        assert_eq!(reopening.end, end.to_string());
        assert_eq!(reopening.reviews, 1);
    }

    /// A spec never `done` starts at its first entry, whatever preceded it, with the
    /// scan for an earlier one ending where the spec did not yet exist.
    #[test]
    fn a_first_entry_starts_the_reopening_of_a_spec_never_done() {
        let fixture = Fixture::after_an_earlier_commit();
        fixture.status("clarified");
        fixture.status("planned");
        let start = fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        assert_eq!(fixture.only_reopening().start, start.to_string());
    }

    /// A status that reaches `done` from anywhere but `in-progress`, as a squashed
    /// reopening does, shows no pass, and that is said rather than left out.
    #[test]
    fn a_closing_with_no_pass_is_not_counted() {
        let fixture = Fixture::new();
        let end = fixture.status("done");
        assert_eq!(
            fixture.only_not_counted(),
            NotCountedReopening {
                feature: FEATURE.into(),
                start: None,
                end: end.to_string(),
                reason: NotCountedReason::NoInProgressCommit,
            }
        );
    }

    #[test]
    fn a_spec_first_seen_as_done_closes_nothing() {
        let fixture = Fixture::new();
        write(&fixture.root.join("specs/034-new/spec.md"), &spec("done"));
        commit_all(&fixture.repository, "chore: adopt a finished spec");
        assert_eq!(fixture.measure().state, ReopeningState::NothingToCompare);
    }

    /// A pass made on a side branch closes once, on that branch, and the merge that
    /// carries it adds no transition. A commit naming no status change adds none.
    #[test]
    fn a_merged_side_branch_adds_no_closing() {
        let fixture = Fixture::new();
        let base = fixture.head();
        let t = 1_800_000_000;
        fixture.write("spec.md", &spec("in-progress"));
        let side_start = fixture.commit_at(t, &[base], false);
        fixture.write("review.md", &review("2026-01-01T00:00:00Z", ""));
        let side_run = fixture.commit_at(t + 1, &[side_start], false);
        fixture.write("spec.md", &spec("done"));
        let side_end = fixture.commit_at(t + 2, &[side_run], false);

        // Main moves on with an unrelated file and the spec as it stood.
        fixture.write("spec.md", &spec("planned"));
        std::fs::remove_file(fixture.root.join("specs").join(FEATURE).join("review.md")).unwrap();
        write(&fixture.root.join("README.md"), "main\n");
        let main = fixture.commit_at(t + 3, &[base], true);

        fixture.write("spec.md", &spec("done"));
        fixture.write("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.commit_at(t + 4, &[main, side_end], true);

        let reopening = fixture.only_reopening();
        assert_eq!(reopening.start, side_start.to_string());
        assert_eq!(reopening.end, side_end.to_string());
        assert_eq!(reopening.reviews, 1);
    }

    /// Two branches each closing the spec can meet the walk with one closing still
    /// waiting for its start. Neither is folded into the other, and the one whose
    /// start was not reached says so rather than counting as clean.
    #[test]
    fn a_closing_met_while_another_waits_leaves_that_one_not_counted() {
        let fixture = Fixture::new();
        let base = fixture.head();
        let t = 1_800_000_000;
        fixture.write("spec.md", &spec("in-progress"));
        let side = fixture.commit_at(t, &[base], false);
        fixture.write("spec.md", &spec("done"));
        let side_end = fixture.commit_at(t + 3, &[side], false);
        fixture.write("spec.md", &spec("in-progress"));
        let main = fixture.commit_at(t + 1, &[base], true);
        fixture.write("spec.md", &spec("done"));
        let main_end = fixture.commit_at(t + 2, &[main], true);
        fixture.write("plan.md", "merged\n");
        fixture.commit_at(t + 4, &[main_end, side_end], true);
        let measurement = fixture.measure();
        assert_eq!(measurement.reopenings, Vec::new());
        assert_eq!(
            measurement.not_counted,
            vec![
                NotCountedReopening {
                    feature: FEATURE.into(),
                    start: Some(main.to_string()),
                    end: main_end.to_string(),
                    reason: NotCountedReason::NoRecordedRun,
                },
                NotCountedReopening {
                    feature: FEATURE.into(),
                    start: None,
                    end: side_end.to_string(),
                    reason: NotCountedReason::NoInProgressCommit,
                },
            ]
        );
    }

    /// The status leaving `in-progress` for another before `done` ends the pass, so
    /// the closing does not reach back to it.
    #[test]
    fn a_closing_from_another_status_takes_no_earlier_pass() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("planned");
        let end = fixture.status("done");
        assert_eq!(
            fixture.only_not_counted(),
            NotCountedReopening {
                feature: FEATURE.into(),
                start: None,
                end: end.to_string(),
                reason: NotCountedReason::NoInProgressCommit,
            }
        );
    }

    /// A pass the repository's first commit opened has no parent to differ from.
    #[test]
    fn a_pass_started_by_the_first_commit_counts() {
        let dir = tempfile::tempdir().unwrap();
        let repository = Repository::init(dir.path()).unwrap();
        let in_spec = |file: &str, body: &str| {
            write(&dir.path().join("specs").join(FEATURE).join(file), body);
            commit_all(&repository, "work(012): step")
        };
        let start = in_spec("spec.md", &spec("in-progress"));
        in_spec("review.md", &review("2026-01-01T00:00:00Z", ""));
        in_spec("spec.md", &spec("done"));
        let measurement = compute(
            &MeasureReopeningsArgs {
                since: Some(LONG_AGO.into()),
                at: None,
            },
            dir.path(),
            SystemTime::now(),
        )
        .unwrap();
        assert_eq!(measurement.reopenings.len(), 1, "{measurement:?}");
        assert_eq!(measurement.reopenings[0].start, start.to_string());
    }

    /// A rebase moves a commit's committer date and keeps its author date, which is
    /// the one the method names.
    #[test]
    fn a_date_goes_by_the_author_not_the_committer() {
        let (authored, since, rebased) = (1_700_000_000, "2023-11-15", 1_700_100_000);
        let fixture = Fixture::new();
        let parent = fixture.head();
        fixture.write("spec.md", &spec("in-progress"));
        let started = fixture.commit_at(authored - 100, &[parent], true);
        fixture.write("spec.md", &spec("done"));
        let tree = {
            let mut index = fixture.repository.index().unwrap();
            index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
            index.write().unwrap();
            let id = index.write_tree().unwrap();
            fixture.repository.find_tree(id).unwrap()
        };
        let author = Signature::new("A", "a@example.com", &Time::new(authored, 0)).unwrap();
        let committer = Signature::new("C", "c@example.com", &Time::new(rebased, 0)).unwrap();
        let parent = fixture.repository.find_commit(started).unwrap();
        fixture
            .repository
            .commit(Some("HEAD"), &author, &committer, "s", &tree, &[&parent])
            .unwrap();
        assert_eq!(
            fixture.measure_since(since).state,
            ReopeningState::NothingToCompare
        );
    }

    #[test]
    fn the_default_window_starts_at_utc_midnight() {
        let day = REOPENING_WINDOW_DAYS * SECONDS_PER_DAY;
        let now = UNIX_EPOCH + Duration::from_secs(day + SECONDS_PER_DAY + 5 * 3_600);
        let start = i64::try_from(SECONDS_PER_DAY).unwrap();
        assert_eq!(default_since(now), ("1970-01-02".to_string(), start));
    }

    /// A spec retired after it closed still closed: a fold or a consolidation must
    /// not erase a miss from the count, or from AC20's sample.
    #[test]
    fn a_spec_no_longer_in_the_tree_still_counts() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        std::fs::remove_dir_all(fixture.root.join("specs").join(FEATURE)).unwrap();
        write(&fixture.root.join("README.md"), "retired\n");
        commit_all(&fixture.repository, "chore: retire 012");
        assert_eq!(fixture.only_reopening().feature, FEATURE);
        assert_eq!(fixture.measure().waiting, Vec::<String>::new());
    }

    #[test]
    fn a_project_in_a_subdirectory_reads_its_own_paths() {
        let dir = tempfile::tempdir().unwrap();
        let (repository, project) = subdirectory_project(dir.path());
        let in_spec = |file: &str, body: &str| {
            write(&project.join("specs").join(FEATURE).join(file), body);
            commit_all(&repository, "work(012): step");
        };
        in_spec("spec.md", &spec("in-progress"));
        in_spec("review.md", &review("2026-01-01T00:00:00Z", ""));
        in_spec("review.md", &review("2026-01-02T00:00:00Z", ""));
        in_spec("spec.md", &spec("done"));
        // Another project's spec in the same repository is not this one's.
        write(
            &dir.path().join("specs/099-other/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repository, "chore: another project");
        let measurement = compute(
            &MeasureReopeningsArgs {
                since: Some(LONG_AGO.into()),
                at: None,
            },
            &project,
            SystemTime::now(),
        )
        .unwrap();
        assert_eq!(measurement.reopenings.len(), 1, "{measurement:?}");
        assert!(measurement.reopenings[0].over_bound);
        assert_eq!(measurement.waiting, Vec::<String>::new());
    }

    #[test]
    fn a_ref_naming_no_commit_is_an_error() {
        let fixture = Fixture::new();
        for args in [
            MeasureReopeningsArgs {
                since: Some("no-such-ref".into()),
                at: None,
            },
            MeasureReopeningsArgs {
                since: None,
                at: Some("no-such-ref".into()),
            },
        ] {
            assert!(matches!(
                compute(&args, &fixture.root, SystemTime::now()),
                Err(PrimitiveError::Git(_))
            ));
        }
    }

    #[test]
    fn no_repository_is_an_error_and_a_state_for_the_dashboard() {
        let dir = tempfile::tempdir().unwrap();
        let args = MeasureReopeningsArgs {
            since: None,
            at: None,
        };
        assert!(compute(&args, dir.path(), SystemTime::now()).is_err());
        let counts = counts(dir.path(), SystemTime::now());
        assert_eq!(counts.state, ReopeningState::NotMeasured);
        assert!(counts.reason.is_some_and(|reason| !reason.is_empty()));
    }

    /// Past the read size the lists move to a file and their counts stay inline.
    #[test]
    fn a_result_past_the_read_size_moves_its_lists_to_a_file() {
        let fixture = Fixture::new();
        write(
            &fixture.root.join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 200\n",
        );
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        fixture.status("in-progress");
        let args = MeasureReopeningsArgs {
            since: Some(LONG_AGO.into()),
            at: None,
        };
        let result = run(&args, &fixture.root).unwrap();
        assert_eq!(result.form, ListsForm::File);
        assert_eq!(result.reopenings, None);
        assert_eq!(result.not_counted, None);
        assert_eq!(result.waiting, None);
        assert_eq!(result.closed_count, 1);
        assert_eq!(result.waiting_count, 1);
        let file = fixture.root.join(result.lists_file.unwrap());
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
        assert_eq!(lists["reopenings"][0]["feature"], FEATURE);
        assert_eq!(lists["waiting"][0], FEATURE);
    }

    #[test]
    fn a_result_within_the_read_size_stays_inline() {
        let fixture = Fixture::new();
        fixture.status("in-progress");
        fixture.commit("review.md", &review("2026-01-01T00:00:00Z", ""));
        fixture.status("done");
        let args = MeasureReopeningsArgs {
            since: Some(LONG_AGO.into()),
            at: None,
        };
        let result = run(&args, &fixture.root).unwrap();
        assert_eq!(result.form, ListsForm::Inline);
        assert_eq!(result.reopenings.map(|list| list.len()), Some(1));
        assert_eq!(result.runs, 1);
    }
}
