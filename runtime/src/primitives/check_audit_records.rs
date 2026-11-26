//! `check-audit-records` — every `done` spec's review and analyze record, each
//! file read once and parsed by the read and parse of the loaders
//! `check-review-gate` reads them by (spec 085), so the adopter CI template's
//! audit-record gate parses no frontmatter of its own (§runtime-boundary,
//! principle 3).
//!
//! A record is absent (never run, so it joins that record's exempt set),
//! present and unreadable (undeterminable, a finding), or present, when a null
//! `last-run`, an absent `blocking` or `blocking: true` is a finding. Each
//! exempt set is held against its committed mark in
//! `.ductus/audit-record-baseline.txt`. A spec the walk could not examine is a
//! finding or a skip with its reason, never a clean pass (`QUAL-CLAIM-001`).
//! Reads the working tree.

use std::collections::BTreeSet;
use std::path::Path;

use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::host::Host;
use crate::primitives::{
    ANALYSIS_RECORD_FILE, PrimitiveError, REVIEW_RECORD_FILE, RecordLoad, Result,
    frontmatter_status, list_feature_dirs, list_untracked_specs, parse_record, read_record,
    read_text, validate_contained, validate_feature_argument,
};
use crate::schema::paths;
use crate::schema::primitives::{
    AnalyzeBlock, AuditBaseline, AuditBaselineState, AuditExemptSet, AuditRecordFinding,
    AuditRecordFindingKind, AuditRecordKind, AuditRecordSkip, AuditRecordSkipReason,
    CheckAuditRecordsArgs, CheckAuditRecordsResult, ListsForm, ReviewBlock,
};

const PRIMITIVE: &str = "check-audit-records";

/// The status whose records are read.
const DONE: &str = "done";

/// The committed high-water marks, from the project root.
pub(crate) const BASELINE_FILE: &str = ".ductus/audit-record-baseline.txt";

/// The review set's key in the baseline file.
const REVIEW_KEY: &str = "review-exempt";

/// The analyze set's key in the baseline file.
const ANALYZE_KEY: &str = "analyze-exempt";

/// The whole answer, before the result is bounded: what the CLI reports from,
/// so a finding the bound moves to a file is still named on stderr.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditRecords {
    /// The spec root walked, from the project root.
    pub specs_root: String,
    /// Specs whose `spec.md` frontmatter was read.
    pub examined: usize,
    /// Of those, the specs at `done`.
    pub done: usize,
    /// The baseline file and its state.
    pub baseline: AuditBaseline,
    /// The review set, then the analyze set.
    pub exempt: [AuditExemptSet; 2],
    /// Each finding, in the order the result lists them.
    pub findings: Vec<AuditRecordFinding>,
    /// Each feature directory not examined.
    pub skipped: Vec<AuditRecordSkip>,
    /// The `done` specs with no `review.md`.
    pub review_exempt: Vec<String>,
    /// The `done` specs with no `analysis.md`.
    pub analyze_exempt: Vec<String>,
}

/// Execute the `check-audit-records` primitive: [`compute`], then [`bound`].
///
/// # Errors
///
/// As [`compute`] and [`bound`].
pub fn run(_args: &CheckAuditRecordsArgs, repo: &Path) -> Result<CheckAuditRecordsResult> {
    bound(repo, compute(repo)?)
}

/// Read every `done` spec's records under the configured spec root, each
/// record file once, and the baseline file.
///
/// # Errors
///
/// The configuration's and the command namespace's load errors
/// ([`paths::Paths::load`], [`Host::load`]); [`PrimitiveError::InvalidPath`]
/// for a spec root that leaves the project, and [`PrimitiveError::Io`] for one
/// that is absent or cannot be listed, never zero specs examined;
/// [`PrimitiveError::Git`] when the repository's status cannot be read, so
/// which specs are untracked has no answer; and an I/O error resolving the
/// project root.
pub fn compute(repo: &Path) -> Result<AuditRecords> {
    compute_reading(repo, &mut read_record)
}

/// [`compute`], each record file's text read through `read`: [`read_record`]
/// outside the tests, which count the reads through it.
fn compute_reading(
    repo: &Path,
    read: &mut dyn FnMut(&Path) -> RecordLoad<String>,
) -> Result<AuditRecords> {
    let specs_root = paths::Paths::load(repo)?.specs_root;
    let project = Host::load(repo)?.project;
    open_root(repo, &specs_root)?;
    let untracked: BTreeSet<String> = list_untracked_specs(repo, &specs_root)?
        .into_iter()
        .collect();
    let mut walk = Walk::default();
    for feature in list_feature_dirs(repo, &specs_root)? {
        walk.spec(repo, &specs_root, &feature, &untracked, &project, read)?;
    }
    let (baseline, marks) = read_baseline(repo, &mut walk.findings)?;
    let exempt = [
        exempt_set(AuditRecordKind::Review, &walk.review_exempt, marks.review),
        exempt_set(
            AuditRecordKind::Analyze,
            &walk.analyze_exempt,
            marks.analyze,
        ),
    ];
    for (set, specs) in exempt
        .iter()
        .zip([&walk.review_exempt, &walk.analyze_exempt])
    {
        if set.over {
            for feature in specs {
                walk.findings
                    .push(over_mark(&specs_root, feature, set, &project));
            }
        }
    }
    Ok(AuditRecords {
        specs_root,
        examined: walk.examined,
        done: walk.done,
        baseline,
        exempt,
        findings: walk.findings,
        skipped: walk.skipped,
        review_exempt: walk.review_exempt,
        analyze_exempt: walk.analyze_exempt,
    })
}

/// Fail the call on a spec root that leaves the project, is absent or cannot be
/// listed: [`list_feature_dirs`] reads each of those as no features, which here
/// would be zero specs examined.
fn open_root(repo: &Path, root: &str) -> Result<()> {
    validate_contained(repo, root)?;
    let dir = repo.join(root);
    std::fs::read_dir(&dir)
        .map(drop)
        .map_err(|source| PrimitiveError::Io { path: dir, source })
}

/// What the walk has found so far.
#[derive(Default)]
struct Walk {
    examined: usize,
    done: usize,
    findings: Vec<AuditRecordFinding>,
    skipped: Vec<AuditRecordSkip>,
    review_exempt: Vec<String>,
    analyze_exempt: Vec<String>,
}

impl Walk {
    /// One feature directory: a finding, a skip, a spec examined, or a `done`
    /// spec examined with its two records read.
    fn spec(
        &mut self,
        repo: &Path,
        root: &str,
        feature: &str,
        untracked: &BTreeSet<String>,
        project: &str,
        read: &mut dyn FnMut(&Path) -> RecordLoad<String>,
    ) -> Result<()> {
        let dir_rel = format!("{root}/{feature}");
        match validate_feature_argument(repo, root, feature) {
            Ok(()) => {}
            Err(PrimitiveError::InvalidPath { path, reason }) => {
                self.findings.push(AuditRecordFinding {
                    kind: AuditRecordFindingKind::OutsideProject,
                    path: dir_rel,
                    message: format!(
                        "'{path}' {reason}, so nothing under this spec was read and its audit records were never examined — remove the link"
                    ),
                });
                return Ok(());
            }
            Err(other) => return Err(other),
        }
        let spec_rel = format!("{dir_rel}/spec.md");
        if untracked.contains(&spec_rel) {
            self.skip(feature, AuditRecordSkipReason::Untracked);
            return Ok(());
        }
        let spec_path = repo.join(&spec_rel);
        if !spec_path.is_file() {
            self.skip(feature, AuditRecordSkipReason::NoSpecFile);
            return Ok(());
        }
        let status = read_text(&spec_path)
            .ok()
            .and_then(|text| frontmatter_status(&text, &spec_path));
        let Some(status) = status else {
            self.findings.push(AuditRecordFinding {
                kind: AuditRecordFindingKind::SpecUnreadable,
                path: spec_rel,
                message: "spec.md frontmatter could not be read or names no status, so this spec's audit records were never examined — fix the frontmatter".into(),
            });
            return Ok(());
        };
        self.examined += 1;
        if status != DONE {
            return Ok(());
        }
        self.done += 1;
        let dir = repo.join(&dir_rel);
        for kind in [AuditRecordKind::Review, AuditRecordKind::Analyze] {
            let file = record_file(kind);
            let path = dir.join(file);
            let text = read(&path);
            let load = match kind {
                AuditRecordKind::Review => {
                    fields::<ReviewBlock>(text, &path, |r| (r.last_run.clone(), r.blocking))
                }
                AuditRecordKind::Analyze => {
                    fields::<AnalyzeBlock>(text, &path, |r| (r.last_run.clone(), r.blocking))
                }
            };
            match judge(load) {
                Judged::Exempt => match kind {
                    AuditRecordKind::Review => self.review_exempt.push(feature.to_string()),
                    AuditRecordKind::Analyze => self.analyze_exempt.push(feature.to_string()),
                },
                Judged::Clean => {}
                Judged::Finding(kind_found, detail) => {
                    let path = format!("{dir_rel}/{file}");
                    let message = record_message(kind, kind_found, &detail, &path, project);
                    self.findings.push(AuditRecordFinding {
                        kind: kind_found,
                        path,
                        message,
                    });
                }
            }
        }
        Ok(())
    }

    fn skip(&mut self, feature: &str, reason: AuditRecordSkipReason) {
        self.skipped.push(AuditRecordSkip {
            feature: feature.to_string(),
            reason,
        });
    }
}

/// The fields of a record the gate turns on: `last-run`, `blocking`, and whether
/// the frontmatter carries a `blocking` key at all.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Fields {
    last_run: Option<String>,
    blocking: bool,
    blocking_key: bool,
}

/// The record file's text, `text`, parsed by the gate's parse ([`parse_record`])
/// into `T` and reduced to [`Fields`] by `read`, its `blocking` key read from the
/// same text by [`BlockingProbe`], so the file is read once. A probe that does
/// not parse reads as no key, never as a pass.
fn fields<T: DeserializeOwned>(
    text: RecordLoad<String>,
    path: &Path,
    read: impl FnOnce(&T) -> (Option<String>, bool),
) -> RecordLoad<Fields> {
    let text = match text {
        RecordLoad::Absent => return RecordLoad::Absent,
        RecordLoad::Unreadable(reason) => return RecordLoad::Unreadable(reason),
        RecordLoad::Present(text) => text,
    };
    match parse_record::<T>(&text, path) {
        Err(reason) => RecordLoad::Unreadable(reason),
        Ok(record) => {
            let (last_run, blocking) = read(&record);
            let blocking_key = parse_record::<BlockingProbe>(&text, path)
                .is_ok_and(|probe| probe.blocking.is_some());
            RecordLoad::Present(Fields {
                last_run,
                blocking,
                blocking_key,
            })
        }
    }
}

/// The record's `blocking` key as the file carries it. The record types read an
/// absent key as `false` (`#[serde(default)]`), and absent is not `false` here.
#[derive(Deserialize)]
struct BlockingProbe {
    #[serde(default)]
    blocking: Option<serde_norway::Value>,
}

/// What one record of a `done` spec is.
#[derive(Debug, PartialEq, Eq)]
enum Judged {
    /// Absent: never run, so it joins its record's exempt set.
    Exempt,
    /// Present, with a `last-run`, a `blocking` key and `blocking: false`.
    Clean,
    /// A finding, with the reason the load gave, empty when there is none.
    Finding(AuditRecordFindingKind, String),
}

/// Judge a record from its load.
fn judge(load: RecordLoad<Fields>) -> Judged {
    let record = match load {
        RecordLoad::Absent => return Judged::Exempt,
        RecordLoad::Unreadable(reason) => {
            return Judged::Finding(AuditRecordFindingKind::RecordUnreadable, reason);
        }
        RecordLoad::Present(record) => record,
    };
    if record
        .last_run
        .as_deref()
        .is_none_or(|run| run.trim().is_empty())
    {
        return Judged::Finding(AuditRecordFindingKind::NoLastRun, String::new());
    }
    if !record.blocking_key {
        return Judged::Finding(AuditRecordFindingKind::NoBlocking, String::new());
    }
    if record.blocking {
        Judged::Finding(AuditRecordFindingKind::Blocking, String::new())
    } else {
        Judged::Clean
    }
}

/// The file a record lives in.
fn record_file(kind: AuditRecordKind) -> &'static str {
    match kind {
        AuditRecordKind::Review => REVIEW_RECORD_FILE,
        AuditRecordKind::Analyze => ANALYSIS_RECORD_FILE,
    }
}

/// The record's name and the command that writes it.
fn record_names(kind: AuditRecordKind) -> (&'static str, &'static str) {
    match kind {
        AuditRecordKind::Review => ("review", "review"),
        AuditRecordKind::Analyze => ("analyze", "analyze"),
    }
}

/// A record finding's message, naming the command that fixes it in the
/// project's namespace.
fn record_message(
    record: AuditRecordKind,
    kind: AuditRecordFindingKind,
    detail: &str,
    path: &str,
    project: &str,
) -> String {
    let file = record_file(record);
    let (name, command) = record_names(record);
    let reason = if detail.is_empty() {
        String::new()
    } else {
        format!(" ({detail})")
    };
    match kind {
        AuditRecordFindingKind::RecordUnreadable => format!(
            "{file} exists but carries no readable frontmatter record{reason} — undeterminable is not clean; re-run /{project}:{command}"
        ),
        AuditRecordFindingKind::NoLastRun => {
            format!("done spec has a null or absent {name}.last-run — run /{project}:{command}")
        }
        AuditRecordFindingKind::NoBlocking => format!(
            "{file} carries a {name} record with no blocking field — the gate turns on it, and absent is not false; re-run /{project}:{command}"
        ),
        _ => format!(
            "done spec has {name}.blocking=true — see {path}, and resolve its findings with /{project}:{command}"
        ),
    }
}

/// The marks the baseline file holds, zero for a key it omits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Marks {
    review: usize,
    analyze: usize,
}

/// Read the baseline file, pushing a finding for one that is malformed,
/// unreadable or a link leaving the project. Every mark not read is zero.
///
/// # Errors
///
/// An I/O error resolving the project root.
fn read_baseline(
    repo: &Path,
    findings: &mut Vec<AuditRecordFinding>,
) -> Result<(AuditBaseline, Marks)> {
    let mut marks = Marks::default();
    let mut finding = |kind, message: String| {
        findings.push(AuditRecordFinding {
            kind,
            path: BASELINE_FILE.into(),
            message,
        });
    };
    let state = match validate_contained(repo, BASELINE_FILE) {
        Err(PrimitiveError::InvalidPath { reason, .. }) => {
            finding(
                AuditRecordFindingKind::BaselineOutsideProject,
                format!(
                    "{BASELINE_FILE} {reason}, so it was not read — the exempt counts were held against marks of zero"
                ),
            );
            AuditBaselineState::OutsideProject
        }
        Err(other) => return Err(other),
        Ok(()) => match std::fs::read(repo.join(BASELINE_FILE)) {
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => AuditBaselineState::Absent,
            Err(err) => {
                finding(
                    AuditRecordFindingKind::BaselineUnreadable,
                    unreadable_message(&err.to_string()),
                );
                AuditBaselineState::Unreadable
            }
            Ok(bytes) => match String::from_utf8(bytes) {
                Err(_) => {
                    finding(
                        AuditRecordFindingKind::BaselineUnreadable,
                        unreadable_message("it is not UTF-8 text"),
                    );
                    AuditBaselineState::Unreadable
                }
                Ok(text) => match parse_marks(&text, &mut marks) {
                    Ok(()) => AuditBaselineState::Read,
                    Err((key, value)) => {
                        finding(
                            AuditRecordFindingKind::BaselineMalformed,
                            format!(
                                "{BASELINE_FILE}: {key} is '{value}', not a count — the marks after it were read as zero; write a whole number"
                            ),
                        );
                        AuditBaselineState::Malformed
                    }
                },
            },
        },
    };
    Ok((
        AuditBaseline {
            path: BASELINE_FILE.into(),
            state,
        },
        marks,
    ))
}

fn unreadable_message(reason: &str) -> String {
    format!(
        "{BASELINE_FILE} could not be read ({reason}) — the exempt counts were held against marks of zero"
    )
}

/// Parse the baseline's lines into `marks`: text after `#` is a comment, a line
/// without `:` is ignored, and so is a key other than the two. Stops at the
/// first known key whose value is not a whole number, returning it and its
/// value, so the marks read before it stand.
fn parse_marks(text: &str, marks: &mut Marks) -> std::result::Result<(), (String, String)> {
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or_default().trim();
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let slot = match key {
            REVIEW_KEY => &mut marks.review,
            ANALYZE_KEY => &mut marks.analyze,
            _ => continue,
        };
        let value = value.trim();
        let count = Some(value)
            .filter(|value| !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()))
            .and_then(|value| value.parse::<usize>().ok());
        let Some(count) = count else {
            return Err((key.to_string(), value.to_string()));
        };
        *slot = count;
    }
    Ok(())
}

/// One record's exempt set against its mark.
fn exempt_set(record: AuditRecordKind, specs: &[String], mark: usize) -> AuditExemptSet {
    AuditExemptSet {
        record,
        key: match record {
            AuditRecordKind::Review => REVIEW_KEY,
            AuditRecordKind::Analyze => ANALYZE_KEY,
        }
        .into(),
        count: specs.len(),
        mark,
        over: specs.len() > mark,
    }
}

/// The finding for one spec of an exempt set over its mark, on its `spec.md`.
fn over_mark(root: &str, feature: &str, set: &AuditExemptSet, project: &str) -> AuditRecordFinding {
    let (name, command) = record_names(set.record);
    AuditRecordFinding {
        kind: AuditRecordFindingKind::ExemptOverMark,
        path: format!("{root}/{feature}/spec.md"),
        message: format!(
            "done spec has no {name} record, and the exempt set ({count}) now exceeds its baseline ({mark}) — the set cannot grow, so the completion gate was bypassed. Run /{project}:{command} against {feature}, or raise {BASELINE_FILE} deliberately ({key}: {count}) and say why in the commit.",
            count = set.count,
            mark = set.mark,
            key = set.key,
        ),
    }
}

/// The result, held to the read size: past it, `findings`, `skipped` and both
/// exempt lists go to the file the result names, with their counts inline.
///
/// # Errors
///
/// Those `bounded_lists::place` returns.
pub fn bound(repo: &Path, found: AuditRecords) -> Result<CheckAuditRecordsResult> {
    let lists = serde_json::json!({
        "findings": found.findings,
        "skipped": found.skipped,
        "review-exempt-specs": found.review_exempt,
        "analyze-exempt-specs": found.analyze_exempt,
    });
    let mut result = CheckAuditRecordsResult {
        specs_root: found.specs_root,
        examined_count: found.examined,
        done_count: found.done,
        skipped_count: found.skipped.len(),
        findings_count: found.findings.len(),
        baseline: found.baseline,
        exempt: found.exempt.to_vec(),
        findings: Some(found.findings),
        skipped: Some(found.skipped),
        review_exempt_specs: Some(found.review_exempt),
        analyze_exempt_specs: Some(found.analyze_exempt),
        form: ListsForm::Inline,
        lists_file: None,
    };
    if let Some(file) = super::bounded_lists::place(
        repo,
        PRIMITIVE,
        super::bounded_lists::ALL_SPECS,
        &result,
        &lists,
    )? {
        result.findings = None;
        result.skipped = None;
        result.review_exempt_specs = None;
        result.analyze_exempt_specs = None;
        result.form = ListsForm::File;
        result.lists_file = Some(file);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::path::PathBuf;

    use super::*;
    use crate::primitives::git_fixture;
    use tempfile::TempDir;

    const DONE_SPEC: &str = "---\nstatus: done\ndependencies: []\n---\n\n# S\n";
    const OPEN_SPEC: &str = "---\nstatus: in-progress\ndependencies: []\n---\n\n# S\n";
    const LAST_RUN: &str = "last-run: 2026-11-25T00:00:00Z\n";
    const NOT_BLOCKING: &str = "blocking: false\n";
    const REVIEW: &str = "---\nspec: x\nlast-run: 2026-11-25T00:00:00Z\nmust-violations: 0\nshould-violations: 0\nlow-confidence: 0\nblocking: false\n---\n\n# Review\n";
    const ANALYSIS: &str = "---\nspec: x\nlast-run: 2026-11-25T00:00:00Z\nhard-fail: 0\nblocking-findings: 0\nadvisory: 0\nunexamined: 0\nblocking: false\n---\n\n# Analysis\n";
    const CONFIG: &str = "[paths]\nspecs-root = \"specs\"\n\n[host]\nproject = \"acme\"\n";

    /// Where the project sits in its repository.
    #[derive(Clone, Copy, Debug)]
    enum Layout {
        /// At the repository's root.
        Root,
        /// In `proj/`, a subdirectory of it (`git_fixture::subdirectory_project`).
        Subdirectory,
    }

    const LAYOUTS: [Layout; 2] = [Layout::Root, Layout::Subdirectory];

    /// A project in a git repository, its config naming the `acme` namespace.
    struct Project {
        _tmp: TempDir,
        repository: git2::Repository,
        root: PathBuf,
    }

    impl Project {
        fn new(layout: Layout) -> Self {
            let tmp = TempDir::new().unwrap();
            let (repository, root) = match layout {
                Layout::Root => (
                    git2::Repository::init(tmp.path()).unwrap(),
                    tmp.path().to_path_buf(),
                ),
                Layout::Subdirectory => git_fixture::subdirectory_project(tmp.path()),
            };
            git_fixture::write(&root.join(".ductus/config.toml"), CONFIG);
            std::fs::create_dir_all(root.join("specs")).unwrap();
            Self {
                _tmp: tmp,
                repository,
                root,
            }
        }

        fn write(&self, rel: &str, body: &str) -> &Self {
            git_fixture::write(&self.root.join(rel), body);
            self
        }

        /// A `done` spec carrying both records, clean.
        fn done(&self, feature: &str) -> &Self {
            self.write(&format!("specs/{feature}/spec.md"), DONE_SPEC)
                .write(&format!("specs/{feature}/review.md"), REVIEW)
                .write(&format!("specs/{feature}/analysis.md"), ANALYSIS)
        }

        fn commit(&self) -> &Self {
            git_fixture::commit_all(&self.repository, "fixture");
            self
        }

        fn compute(&self) -> AuditRecords {
            compute(&self.root).unwrap()
        }
    }

    fn kinds(found: &AuditRecords) -> Vec<(AuditRecordFindingKind, &str)> {
        found
            .findings
            .iter()
            .map(|finding| (finding.kind, finding.path.as_str()))
            .collect()
    }

    /// A clean corpus passes with its counts, from the repository's root and
    /// from a subdirectory: two `done` specs and one open spec without records.
    #[test]
    fn a_clean_corpus_passes_with_its_counts() {
        for layout in LAYOUTS {
            let project = Project::new(layout);
            project
                .done("001-a")
                .done("002-b")
                .write("specs/003-c/spec.md", OPEN_SPEC)
                .commit();
            let found = project.compute();
            assert!(found.findings.is_empty(), "{layout:?}: {found:?}");
            assert_eq!(
                (found.examined, found.done, found.skipped.len()),
                (3, 2, 0),
                "{layout:?}"
            );
            assert_eq!(found.specs_root, "specs");
            assert_eq!(found.baseline.state, AuditBaselineState::Absent);
            assert_eq!(found.baseline.path, BASELINE_FILE);
            for (set, key) in found.exempt.iter().zip([REVIEW_KEY, ANALYZE_KEY]) {
                assert_eq!(
                    (set.key.as_str(), set.count, set.mark, set.over),
                    (key, 0, 0, false)
                );
            }
            assert_eq!(
                [found.exempt[0].record, found.exempt[1].record],
                [AuditRecordKind::Review, AuditRecordKind::Analyze]
            );
            let result = run(&CheckAuditRecordsArgs::default(), &project.root).unwrap();
            assert_eq!(result.form, ListsForm::Inline);
            assert_eq!(
                (
                    result.examined_count,
                    result.done_count,
                    result.findings_count
                ),
                (3, 2, 0)
            );
        }
    }

    /// How a case turns a clean record into one carrying its condition.
    enum Edit {
        /// The record becomes this text.
        Whole(&'static str),
        /// This line of the clean record becomes that one.
        Replace(&'static str, &'static str),
    }

    impl Edit {
        fn apply(&self, clean: &str) -> String {
            match self {
                Self::Whole(text) => (*text).to_string(),
                Self::Replace(from, to) => clean.replace(from, to),
            }
        }
    }

    /// Each record finding's conditions: every YAML spelling of a null
    /// `last-run`, an empty, blank or absent one, an absent `blocking`, a null
    /// one, which no record type loads, and `blocking: true`.
    const RECORD_CASES: [(&str, Edit, AuditRecordFindingKind); 12] = {
        use AuditRecordFindingKind::{Blocking, NoBlocking, NoLastRun, RecordUnreadable};
        use Edit::{Replace, Whole};
        [
            ("no frontmatter", Whole("# A record\n"), RecordUnreadable),
            (
                "YAML that does not parse",
                Whole("---\nlast-run: [\n---\n"),
                RecordUnreadable,
            ),
            (
                "blocking: null",
                Replace(NOT_BLOCKING, "blocking: null\n"),
                RecordUnreadable,
            ),
            (
                "last-run: null",
                Replace(LAST_RUN, "last-run: null\n"),
                NoLastRun,
            ),
            (
                "last-run: Null",
                Replace(LAST_RUN, "last-run: Null\n"),
                NoLastRun,
            ),
            (
                "last-run: NULL",
                Replace(LAST_RUN, "last-run: NULL\n"),
                NoLastRun,
            ),
            ("last-run: ~", Replace(LAST_RUN, "last-run: ~\n"), NoLastRun),
            (
                "an empty last-run",
                Replace(LAST_RUN, "last-run:\n"),
                NoLastRun,
            ),
            (
                "a blank last-run",
                Replace(LAST_RUN, "last-run: \"  \"\n"),
                NoLastRun,
            ),
            ("no last-run", Replace(LAST_RUN, ""), NoLastRun),
            ("no blocking key", Replace(NOT_BLOCKING, ""), NoBlocking),
            (
                "blocking: true",
                Replace(NOT_BLOCKING, "blocking: true\n"),
                Blocking,
            ),
        ]
    };

    /// Each record's finding, on either record, fails with its condition
    /// present and passes without it: the clean record beside it is no finding.
    #[test]
    fn each_record_finding_fails_with_its_condition_and_passes_without_it() {
        for layout in LAYOUTS {
            for (file, clean, other, other_clean) in [
                ("review.md", REVIEW, "analysis.md", ANALYSIS),
                ("analysis.md", ANALYSIS, "review.md", REVIEW),
            ] {
                for (name, edit, kind) in &RECORD_CASES {
                    let record = edit.apply(clean);
                    assert_ne!(record, clean, "{name}: the edit changed nothing");
                    let project = Project::new(layout);
                    project
                        .write("specs/001-a/spec.md", DONE_SPEC)
                        .write(&format!("specs/001-a/{file}"), &record)
                        .write(&format!("specs/001-a/{other}"), other_clean)
                        .commit();
                    let found = project.compute();
                    let path = format!("specs/001-a/{file}");
                    assert_eq!(
                        kinds(&found),
                        vec![(*kind, path.as_str())],
                        "{layout:?} {file} {name}"
                    );
                    assert_eq!((found.examined, found.done), (1, 1));
                    assert!(
                        found.exempt.iter().all(|set| set.count == 0),
                        "a present record is in no exempt set"
                    );

                    project
                        .write(&format!("specs/001-a/{file}"), clean)
                        .commit();
                    let found = project.compute();
                    assert!(
                        found.findings.is_empty(),
                        "{layout:?} {file} {name}: {found:?}"
                    );
                    assert_eq!((found.examined, found.done), (1, 1));
                }
            }
        }
    }

    /// A quoted `"null"` is a string, as the gate reads it: a record that ran.
    #[test]
    fn a_quoted_null_last_run_is_a_string() {
        let project = Project::new(Layout::Root);
        project
            .done("001-a")
            .write(
                "specs/001-a/review.md",
                &REVIEW.replace(LAST_RUN, "last-run: \"null\"\n"),
            )
            .commit();
        let found = project.compute();
        assert!(found.findings.is_empty(), "{found:?}");
        assert_eq!(found.done, 1);
    }

    fn present(last_run: Option<&str>, blocking: bool, blocking_key: bool) -> RecordLoad<Fields> {
        RecordLoad::Present(Fields {
            last_run: last_run.map(str::to_string),
            blocking,
            blocking_key,
        })
    }

    /// The judgment of each load: absent is exempt, unreadable and a record
    /// with no `last-run` or no `blocking` key are findings, and `blocking:
    /// true` is one.
    #[test]
    fn a_record_is_judged_by_its_load() {
        use AuditRecordFindingKind::{Blocking, NoBlocking, NoLastRun, RecordUnreadable};
        let ran = Some("2026-11-25T00:00:00Z");
        let finding = |kind| Judged::Finding(kind, String::new());
        assert_eq!(judge(RecordLoad::Absent), Judged::Exempt);
        assert_eq!(
            judge(RecordLoad::Unreadable("bad".into())),
            Judged::Finding(RecordUnreadable, "bad".into())
        );
        assert_eq!(judge(present(None, false, true)), finding(NoLastRun));
        assert_eq!(judge(present(Some(" \t"), false, true)), finding(NoLastRun));
        assert_eq!(judge(present(ran, false, true)), Judged::Clean);
        assert_eq!(judge(present(ran, true, true)), finding(Blocking));
        assert_eq!(judge(present(ran, false, false)), finding(NoBlocking));
    }

    /// The probe reads the key from the text the record was parsed from: a
    /// frontmatter with the key loads with it, one without loads without it.
    #[test]
    fn the_probe_reads_the_key_from_the_records_own_text() {
        let path = Path::new("specs/001-a/review.md");
        let read = |r: &ReviewBlock| (r.last_run.clone(), r.blocking);
        let with = fields::<ReviewBlock>(RecordLoad::Present(REVIEW.into()), path, read);
        let without = fields::<ReviewBlock>(
            RecordLoad::Present(REVIEW.replace(NOT_BLOCKING, "")),
            path,
            read,
        );
        assert!(
            matches!(
                with,
                RecordLoad::Present(Fields {
                    blocking_key: true,
                    ..
                })
            ),
            "{with:?}"
        );
        assert!(
            matches!(
                without,
                RecordLoad::Present(Fields {
                    blocking_key: false,
                    blocking: false,
                    ..
                })
            ),
            "{without:?}"
        );
        assert_eq!(
            fields::<ReviewBlock>(RecordLoad::Absent, path, read),
            RecordLoad::Absent
        );
    }

    /// A run reads each record file once, its probe included, and every
    /// `done` spec's two.
    #[test]
    fn each_record_file_is_read_once() {
        let project = Project::new(Layout::Root);
        project.done("001-a").done("002-b").commit();
        let mut reads: Vec<std::path::PathBuf> = Vec::new();
        let found = compute_reading(&project.root, &mut |path: &Path| {
            reads.push(path.to_path_buf());
            read_record(path)
        })
        .unwrap();
        assert!(found.findings.is_empty(), "{found:?}");
        let mut read: Vec<String> = reads
            .iter()
            .map(|path| {
                path.strip_prefix(&project.root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        read.sort();
        assert_eq!(
            read,
            vec![
                "specs/001-a/analysis.md",
                "specs/001-a/review.md",
                "specs/002-b/analysis.md",
                "specs/002-b/review.md",
            ]
        );
    }

    /// A `spec.md` whose status cannot be read is a finding and is not examined:
    /// no frontmatter, one that never closes, one naming no status, one that
    /// does not parse as spec frontmatter, and bytes that are not UTF-8. With a
    /// readable status the same spec is examined.
    #[test]
    fn a_spec_whose_status_cannot_be_read_is_a_finding_and_not_examined() {
        let cases: [(&str, &[u8]); 5] = [
            ("no frontmatter", b"# S\n"),
            ("unclosed", b"---\nstatus: done\n"),
            ("no status", b"---\ndependencies: []\n---\n\n# S\n"),
            (
                "a list holding a scalar",
                b"---\nstatus: draft\ndependencies: x\n---\n",
            ),
            ("not UTF-8", &[0xff, 0xfe, 0x00]),
        ];
        for layout in LAYOUTS {
            for (name, bytes) in cases {
                let project = Project::new(layout);
                project.done("001-a");
                std::fs::write(project.root.join("specs/001-a/spec.md"), bytes).unwrap();
                project.commit();
                let found = project.compute();
                assert_eq!(
                    kinds(&found),
                    vec![(
                        AuditRecordFindingKind::SpecUnreadable,
                        "specs/001-a/spec.md"
                    )],
                    "{layout:?} {name}"
                );
                assert_eq!((found.examined, found.done), (0, 0), "{name}");

                project.write("specs/001-a/spec.md", DONE_SPEC).commit();
                let found = project.compute();
                assert!(found.findings.is_empty(), "{name}: {found:?}");
                assert_eq!((found.examined, found.done), (1, 1), "{name}");
            }
        }
    }

    /// A spec not at `done` is examined and its records are not read.
    #[test]
    fn a_spec_not_done_has_no_record_read() {
        let project = Project::new(Layout::Root);
        project
            .write("specs/001-a/spec.md", OPEN_SPEC)
            .write(
                "specs/001-a/review.md",
                &REVIEW.replace(NOT_BLOCKING, "blocking: true\n"),
            )
            .write("specs/001-a/analysis.md", "# no frontmatter\n")
            .commit();
        let found = project.compute();
        assert!(found.findings.is_empty(), "{found:?}");
        assert_eq!((found.examined, found.done), (1, 0));
        assert!(found.review_exempt.is_empty() && found.analyze_exempt.is_empty());
    }

    /// A `done` spec with one record present and the other absent counts only
    /// in the absent record's exempt set, which is no finding at its mark.
    #[test]
    fn one_absent_record_counts_only_in_its_own_exempt_set() {
        for (absent, set) in [("review.md", 0), ("analysis.md", 1)] {
            let project = Project::new(Layout::Root);
            project.done("001-a").done("002-b");
            std::fs::remove_file(project.root.join(format!("specs/002-b/{absent}"))).unwrap();
            project
                .write(BASELINE_FILE, "review-exempt: 1\nanalyze-exempt: 1\n")
                .commit();
            let found = project.compute();
            assert!(found.findings.is_empty(), "{absent}: {found:?}");
            let counts: Vec<usize> = found.exempt.iter().map(|set| set.count).collect();
            let mut expected = vec![0, 0];
            expected[set] = 1;
            assert_eq!(counts, expected, "{absent}");
            let lists = [&found.review_exempt, &found.analyze_exempt];
            assert_eq!(lists[set], &vec!["002-b".to_string()]);
            assert!(lists[1 - set].is_empty(), "{absent}: {found:?}");
        }
    }

    /// An exempt set at its mark passes; past it, each spec in the set is a
    /// finding on its `spec.md`, after the walk's and the baseline's findings.
    #[test]
    fn an_exempt_set_past_its_mark_is_a_finding_per_spec() {
        for layout in LAYOUTS {
            let project = Project::new(layout);
            project
                .write("specs/001-a/spec.md", DONE_SPEC)
                .write("specs/001-a/review.md", REVIEW)
                .write("specs/002-b/spec.md", DONE_SPEC)
                .write("specs/002-b/review.md", REVIEW)
                .write(BASELINE_FILE, "analyze-exempt: 2\n")
                .commit();
            let found = project.compute();
            assert!(found.findings.is_empty(), "{layout:?}: {found:?}");
            assert_eq!(
                (
                    found.exempt[1].count,
                    found.exempt[1].mark,
                    found.exempt[1].over
                ),
                (2, 2, false)
            );

            project
                .write(BASELINE_FILE, "analyze-exempt: 1\n")
                .write("specs/003-c/spec.md", DONE_SPEC)
                .write("specs/003-c/analysis.md", ANALYSIS)
                .write(
                    "specs/003-c/review.md",
                    &REVIEW.replace(NOT_BLOCKING, "blocking: true\n"),
                )
                .commit();
            let found = project.compute();
            assert_eq!(
                kinds(&found),
                vec![
                    (AuditRecordFindingKind::Blocking, "specs/003-c/review.md"),
                    (
                        AuditRecordFindingKind::ExemptOverMark,
                        "specs/001-a/spec.md"
                    ),
                    (
                        AuditRecordFindingKind::ExemptOverMark,
                        "specs/002-b/spec.md"
                    ),
                ],
                "{layout:?}"
            );
            assert_eq!(
                (
                    found.exempt[1].count,
                    found.exempt[1].mark,
                    found.exempt[1].over
                ),
                (2, 1, true)
            );
            assert!(!found.exempt[0].over);
            let message = &found.findings[1].message;
            assert!(
                message.contains("(2) now exceeds its baseline (1)")
                    && message.contains("/acme:analyze against 001-a")
                    && message.contains("(analyze-exempt: 2)"),
                "{message}"
            );
        }
    }

    fn baseline_of(text: &[u8]) -> (AuditRecords, Project) {
        let project = Project::new(Layout::Root);
        project.done("001-a");
        std::fs::write(project.root.join(BASELINE_FILE), text).unwrap();
        project.commit();
        (project.compute(), project)
    }

    fn marks(found: &AuditRecords) -> (usize, usize) {
        (found.exempt[0].mark, found.exempt[1].mark)
    }

    /// The baseline's states: absent and every mark zero; read, with comments,
    /// lines without a colon and unknown keys ignored, an omitted key zero;
    /// malformed at the first known key that is not a whole number, the marks
    /// before it standing and the rest zero; and unreadable.
    #[test]
    fn the_baseline_is_read_in_each_of_its_states() {
        let (found, _p) = baseline_of(
            b"# marks\nreview-exempt: 3 # three\nnote without a colon\nother-key: x\n\n",
        );
        assert_eq!(found.baseline.state, AuditBaselineState::Read);
        assert_eq!(marks(&found), (3, 0));
        assert!(found.findings.is_empty(), "{found:?}");

        let (found, _p) = baseline_of(b"analyze-exempt: 007\nreview-exempt: 2\n");
        assert_eq!(
            (found.baseline.state, marks(&found)),
            (AuditBaselineState::Read, (2, 7))
        );

        for (text, expected) in [
            (&b"analyze-exempt: 3\nreview-exempt: two\n"[..], (0, 3)),
            (b"review-exempt: two\nanalyze-exempt: 3\n", (0, 0)),
            (b"review-exempt: +5\n", (0, 0)),
            (b"review-exempt: -1\n", (0, 0)),
            (b"review-exempt:\n", (0, 0)),
            (b"review-exempt: 99999999999999999999999\n", (0, 0)),
        ] {
            let (found, _p) = baseline_of(text);
            assert_eq!(
                found.baseline.state,
                AuditBaselineState::Malformed,
                "{}",
                String::from_utf8_lossy(text)
            );
            assert_eq!(marks(&found), expected);
            assert_eq!(
                kinds(&found),
                vec![(AuditRecordFindingKind::BaselineMalformed, BASELINE_FILE)]
            );
        }

        let (found, _p) = baseline_of(&[0xff, 0xfe]);
        assert_eq!(found.baseline.state, AuditBaselineState::Unreadable);
        assert_eq!(
            kinds(&found),
            vec![(AuditRecordFindingKind::BaselineUnreadable, BASELINE_FILE)]
        );

        let project = Project::new(Layout::Root);
        project.done("001-a").commit();
        std::fs::create_dir_all(project.root.join(BASELINE_FILE)).unwrap();
        let found = project.compute();
        assert_eq!(found.baseline.state, AuditBaselineState::Unreadable);
        assert_eq!(found.findings.len(), 1);

        let project = Project::new(Layout::Root);
        project.done("001-a").commit();
        let found = project.compute();
        assert_eq!(
            (found.baseline.state, marks(&found)),
            (AuditBaselineState::Absent, (0, 0))
        );
        assert!(found.findings.is_empty(), "{found:?}");
    }

    /// A malformed or unreadable baseline still has its exempt sets counted
    /// against the marks it left, so a set past them is still a finding.
    #[test]
    fn a_baseline_that_cannot_be_read_still_holds_the_sets() {
        let project = Project::new(Layout::Root);
        project
            .write("specs/001-a/spec.md", DONE_SPEC)
            .write("specs/001-a/analysis.md", ANALYSIS)
            .write(BASELINE_FILE, "review-exempt: one\n")
            .commit();
        let found = project.compute();
        assert_eq!(
            kinds(&found),
            vec![
                (AuditRecordFindingKind::BaselineMalformed, BASELINE_FILE),
                (
                    AuditRecordFindingKind::ExemptOverMark,
                    "specs/001-a/spec.md"
                ),
            ]
        );
        assert_eq!(found.exempt[0].count, 1);
    }

    /// A baseline linked out of the project is never read: its marks are zero,
    /// whatever the file outside says.
    #[cfg(unix)]
    #[test]
    fn a_baseline_linked_out_of_the_project_is_a_finding_and_never_read() {
        let project = Project::new(Layout::Root);
        project.done("001-a").commit();
        let _outside =
            crate::primitives::outward::file(&project.root, BASELINE_FILE, "review-exempt: 5\n");
        let found = project.compute();
        assert_eq!(found.baseline.state, AuditBaselineState::OutsideProject);
        assert_eq!(marks(&found), (0, 0));
        assert_eq!(
            kinds(&found),
            vec![(
                AuditRecordFindingKind::BaselineOutsideProject,
                BASELINE_FILE
            )]
        );
    }

    /// A feature directory that is, or holds, a link leaving the project is a
    /// finding naming the link, never read, and not examined.
    #[cfg(unix)]
    #[test]
    fn a_spec_linked_out_of_the_project_is_a_finding() {
        for layout in LAYOUTS {
            let project = Project::new(layout);
            project.done("002-in").commit();
            let _dir = crate::primitives::outward::dir(&project.root, "specs/001-out", |dir| {
                std::fs::write(dir.join("spec.md"), DONE_SPEC).unwrap();
            });
            let _file =
                crate::primitives::outward::file(&project.root, "specs/002-in/notes.md", "outside");
            let found = project.compute();
            assert_eq!(
                kinds(&found),
                vec![
                    (AuditRecordFindingKind::OutsideProject, "specs/001-out"),
                    (AuditRecordFindingKind::OutsideProject, "specs/002-in"),
                ],
                "{layout:?}"
            );
            assert!(
                found.findings[1].message.contains("specs/002-in"),
                "{found:?}"
            );
            assert_eq!((found.examined, found.done), (0, 0));

            std::fs::remove_file(project.root.join("specs/001-out")).unwrap();
            std::fs::remove_file(project.root.join("specs/002-in/notes.md")).unwrap();
            let found = project.compute();
            assert!(found.findings.is_empty(), "{found:?}");
            assert_eq!((found.examined, found.done), (1, 1));
        }
    }

    /// An untracked spec and a directory with no `spec.md` are skipped with their
    /// reasons and counted, never examined, from the root and a subdirectory.
    #[test]
    fn an_untracked_spec_and_a_bare_directory_are_skipped() {
        for layout in LAYOUTS {
            let project = Project::new(layout);
            project.done("001-a").commit();
            project
                .write("specs/002-draft/spec.md", DONE_SPEC)
                .write("specs/003-bare/notes.md", "# notes\n");
            let found = project.compute();
            assert_eq!(
                found.skipped,
                vec![
                    AuditRecordSkip {
                        feature: "002-draft".into(),
                        reason: AuditRecordSkipReason::Untracked
                    },
                    AuditRecordSkip {
                        feature: "003-bare".into(),
                        reason: AuditRecordSkipReason::NoSpecFile
                    },
                ],
                "{layout:?}"
            );
            assert_eq!((found.examined, found.done), (1, 1), "{layout:?}");
            assert!(found.findings.is_empty(), "{found:?}");
            assert!(found.review_exempt.is_empty(), "the draft has no review.md");

            project.commit();
            let found = project.compute();
            assert_eq!(found.examined, 2, "{layout:?}: tracked now");
            assert_eq!(found.review_exempt, vec!["002-draft".to_string()]);
        }
    }

    /// A spec root that is absent, is a file, or leaves the project fails the
    /// call naming it, never zero specs examined; an empty one is examined and
    /// holds nothing, which the CLI fails.
    #[test]
    fn a_spec_root_that_cannot_be_listed_is_an_error() {
        let project = Project::new(Layout::Root);
        std::fs::remove_dir_all(project.root.join("specs")).unwrap();
        let err = compute(&project.root).unwrap_err();
        assert!(matches!(err, PrimitiveError::Io { .. }), "{err:?}");
        assert!(err.to_string().contains("specs"), "{err}");

        std::fs::write(project.root.join("specs"), "a file").unwrap();
        let err = compute(&project.root).unwrap_err();
        assert!(matches!(err, PrimitiveError::Io { .. }), "{err:?}");

        std::fs::remove_file(project.root.join("specs")).unwrap();
        std::fs::create_dir_all(project.root.join("specs")).unwrap();
        let found = project.compute();
        assert_eq!((found.examined, found.findings.len()), (0, 0));
    }

    #[cfg(unix)]
    #[test]
    fn a_spec_root_linked_out_of_the_project_is_refused() {
        let project = Project::new(Layout::Root);
        std::fs::remove_dir_all(project.root.join("specs")).unwrap();
        let _outside = crate::primitives::outward::dir(&project.root, "specs", |_| {});
        let result = compute(&project.root);
        assert!(crate::primitives::outward::refused(&result), "{result:?}");
    }

    /// Each message names its fix in the project's own command namespace.
    #[test]
    fn a_message_names_the_projects_own_command() {
        let project = Project::new(Layout::Root);
        project
            .write("specs/001-a/spec.md", DONE_SPEC)
            .write("specs/001-a/review.md", &REVIEW.replace(LAST_RUN, ""))
            .write(
                "specs/001-a/analysis.md",
                &ANALYSIS.replace(NOT_BLOCKING, ""),
            )
            .commit();
        let found = project.compute();
        assert_eq!(found.findings.len(), 2, "{found:?}");
        assert!(
            found.findings[0].message.contains("/acme:review"),
            "{found:?}"
        );
        assert!(
            found.findings[1].message.contains("/acme:analyze"),
            "{found:?}"
        );
        assert!(found.findings[1].message.contains("absent is not false"));
    }

    /// Past the read size the four lists move to the file the result names,
    /// with the counts inline, from the root and a subdirectory.
    #[test]
    fn past_the_read_size_the_lists_move_to_a_file() {
        for layout in LAYOUTS {
            let project = Project::new(layout);
            project.write(
                ".ductus/config.toml",
                &format!("{CONFIG}\n[artifacts]\nread-size-bytes = 100\n"),
            );
            project
                .write("specs/001-a/spec.md", DONE_SPEC)
                .write("specs/002-b/spec.md", DONE_SPEC)
                .write(
                    "specs/002-b/review.md",
                    &REVIEW.replace(NOT_BLOCKING, "blocking: true\n"),
                )
                .write("specs/003-c/notes.md", "# notes\n")
                .write(BASELINE_FILE, "review-exempt: 1\nanalyze-exempt: 2\n")
                .commit();
            let result = run(&CheckAuditRecordsArgs::default(), &project.root).unwrap();
            assert_eq!(result.form, ListsForm::File, "{layout:?}");
            assert!(
                result.findings.is_none()
                    && result.skipped.is_none()
                    && result.review_exempt_specs.is_none()
                    && result.analyze_exempt_specs.is_none()
            );
            assert_eq!(
                (
                    result.examined_count,
                    result.done_count,
                    result.findings_count,
                    result.skipped_count
                ),
                (2, 2, 1, 1)
            );
            assert_eq!((result.exempt[0].count, result.exempt[1].count), (1, 2));
            let shown = serde_json::to_value(&result).unwrap();
            let file = project.root.join(result.lists_file.unwrap());
            let lists: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
            assert_eq!(lists["findings"][0]["path"], "specs/002-b/review.md");
            assert_eq!(lists["skipped"][0]["reason"], "no-spec-file");
            assert_eq!(lists["review-exempt-specs"], serde_json::json!(["001-a"]));
            assert_eq!(
                lists["analyze-exempt-specs"],
                serde_json::json!(["001-a", "002-b"])
            );
            super::super::bounded_lists::assert_counts_name_the_file(PRIMITIVE, &shown, &lists);
        }
    }

    /// The result round-trips to itself.
    #[test]
    fn the_result_round_trips() {
        let project = Project::new(Layout::Root);
        project
            .done("001-a")
            .write("specs/002-b/spec.md", DONE_SPEC)
            .write(
                "specs/002-b/review.md",
                &REVIEW.replace(NOT_BLOCKING, "blocking: true\n"),
            )
            .commit();
        let result = run(&CheckAuditRecordsArgs::default(), &project.root).unwrap();
        let text = serde_json::to_string(&result).unwrap();
        let back: CheckAuditRecordsResult = serde_json::from_str(&text).unwrap();
        assert_eq!(back, result, "{text}");
        assert!(text.contains("\"kind\":\"blocking\""), "{text}");
    }
}
