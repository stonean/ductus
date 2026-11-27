//! `prune-review` — reduce a spec's `review.md` between reviews (spec 086, AC3 and AC5).
//!
//! A preview reports each stored decision, and each waiver anchor, on a file still on disk
//! that does not belong to the spec ([`spec_ownership`]): what the spec's next unrestricted
//! review would drop. It also counts each `## Excluded from scope` line that still names the
//! specs that claimed its path, as `write-review` wrote them before `64946fe7`, which the
//! path alone replaces. An apply makes those edits line by line and leaves every other byte
//! as the run wrote it: it reads the result back through the readers every consumer uses
//! and refuses, writing nothing, when anything but the dropped entries reads differently.
//!
//! A deleted file's entry is not dropped here; the spec's next unrestricted review expires
//! it (AC4). A record holding `fix-rounds-open: true` is left for the run that holds it. The
//! project record, `{specs-root}/review.md`, has no spec, so no walk reads it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::primitives::compute_review_scope::still_on_disk;
use crate::primitives::decisions::{RawDecision, key_anchor};
use crate::primitives::process_waivers::{AnchorList, RawWaiver, anchor_files};
use crate::primitives::prune_tasks::{
    BoundedPruneResult, bound_result, one_feature_or_all, size_of,
};
use crate::primitives::spec_ownership::{self, Ownership};
use crate::primitives::{
    PrimitiveError, REVIEW_RECORD_FILE, ReadSnapshot, Result, SkipScanner, flatten_line, hex,
    list_feature_dirs, parse_atx_heading, parse_feature_dir, recorded_list_in,
    split_frontmatter_with_offset,
};
use crate::schema::paths;
use crate::schema::primitives::{
    ListsForm, OwnershipState, PruneReviewArgs, PruneReviewResult, PruneReviewSummary,
    PruneReviewWalk, PruneWalkEntry, ReviewBlock, ReviewRecordDrop, ReviewRecordList,
    ReviewSkipReason, ReviewSkipped,
};

/// The two lists of a record's frontmatter this primitive drops from.
const DECISIONS: &str = "decisions";
const WAIVERS: &str = "waivers";

/// The body section whose old lines name the specs that claimed each path.
const EXCLUDED_HEADING: &str = "Excluded from scope";

/// Execute the `prune-review` primitive against the given project root: one feature's
/// preview or apply, or every feature's under `all`.
///
/// # Errors
///
/// - [`PrimitiveError::MissingArgument`] when neither `feature` nor `all` is given, or an
///   apply carries no `digest` (one feature) or no `digests` (`all`).
/// - [`PrimitiveError::InvalidArgument`] when both `feature` and `all` are given, when a
///   digest comes without `apply`, `digest` with `all` or `digests` with `feature`, and when
///   a `digests` key names no feature directory under the spec root.
/// - [`PrimitiveError::InvalidPath`] when `feature` would escape the spec root, or the
///   record is, or runs through, a link leaving the project.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory is absent.
/// - [`PrimitiveError::MissingFrontmatter`], [`PrimitiveError::UnclosedFrontmatter`] or
///   [`PrimitiveError::Yaml`] when the record, or its `decisions:` or `waivers:` list, does
///   not parse, so an undeterminable record is never rewritten as though it were read.
/// - [`PrimitiveError::UnsplicableRecord`] when the edits cannot be made line by line
///   without changing what they do not drop.
/// - [`PrimitiveError::ChangedSinceRead`] when the edits an apply derives again differ
///   from those its `digest` names, or the record changed between the apply's read and
///   its write (`BE-TXN-002`): nothing is written and the host previews again. Under `all`
///   such a record is listed in `skipped` instead, and the others are written.
/// - [`PrimitiveError::InvalidReadSize`] / [`PrimitiveError::Toml`] when
///   `[artifacts] read-size-bytes` is malformed, before any write.
/// - [`PrimitiveError::Io`] on filesystem failure.
pub fn run(args: &PruneReviewArgs, repo: &Path) -> Result<PruneReviewResult> {
    let root = paths::Paths::load(repo)?.specs_root;
    let feature = one_feature_or_all("prune-review", args.feature.as_deref(), args.all)?;
    check_apply(args, feature.is_none())?;
    // The read size bounds the result after an apply's write, so a malformed one halts
    // here, before anything is written.
    super::check_artifact_size::resolve_threshold(repo)?;
    let mut result = match feature {
        None => PruneReviewResult {
            summary: None,
            walk: Some(walk(repo, &root, args.apply.then_some(&args.digests))?),
            form: ListsForm::Inline,
            lists_file: None,
        },
        Some(feature) => PruneReviewResult {
            summary: Some(summarize(feature, args, repo, &root)?),
            walk: None,
            form: ListsForm::Inline,
            lists_file: None,
        },
    };
    let key = feature.unwrap_or(super::bounded_lists::ALL_SPECS);
    bound_result(repo, "prune-review", key, &mut result)?;
    Ok(result)
}

impl BoundedPruneResult for PruneReviewResult {
    fn lists(&self) -> serde_json::Value {
        match (&self.summary, &self.walk) {
            (Some(summary), _) => serde_json::json!({ "drops": &summary.drops }),
            (None, Some(walk)) => {
                serde_json::json!({ "features": &walk.features, "skipped": &walk.skipped })
            }
            (None, None) => serde_json::json!({}),
        }
    }

    fn move_to_file(&mut self, file: String) {
        self.form = ListsForm::File;
        self.lists_file = Some(file);
        if let Some(summary) = &mut self.summary {
            summary.drops = None;
        }
        if let Some(walk) = &mut self.walk {
            walk.features = None;
            walk.skipped = None;
        }
    }
}

/// Refuse an apply the call cannot make: one with no digest naming the edits it writes,
/// one feature's apply given a walk's `digests` or a walk's given one feature's `digest`,
/// and a digest with no apply to compare it.
fn check_apply(args: &PruneReviewArgs, all: bool) -> Result<()> {
    let refuse = |argument: &str, reason: &str| {
        Err(PrimitiveError::InvalidArgument {
            primitive: "prune-review".into(),
            argument: argument.into(),
            reason: reason.into(),
        })
    };
    let missing = |argument: &str, reason: &str| {
        Err(PrimitiveError::MissingArgument {
            primitive: "prune-review".into(),
            argument: argument.into(),
            reason: reason.into(),
        })
    };
    let digest = args
        .digest
        .as_deref()
        .is_some_and(|digest| !digest.trim().is_empty());
    let digests = !args.digests.is_empty();
    if all && digest {
        return refuse(
            "digest",
            "a walk's apply takes digests, each record's keyed by its feature",
        );
    }
    if !all && digests {
        return refuse(
            "digests",
            "one feature's apply takes the digest its preview reported",
        );
    }
    match (args.apply, all, digest || digests) {
        (true, false, false) => missing(
            "digest",
            "an apply writes the edits a preview reported; pass that preview's digest",
        ),
        (true, true, false) => missing(
            "digests",
            "a walk's apply writes the edits its preview reported; pass each listed \
             record's digest, keyed by its feature",
        ),
        (false, true, true) => refuse("digests", "only an apply takes digests; pass apply"),
        (false, false, true) => refuse("digest", "only an apply takes a digest; pass apply"),
        _ => Ok(()),
    }
}

/// Every feature's record, in corpus order, from one walk of the history, written when
/// `apply` holds each record's digest. A feature is listed when it has a drop or a rewrite,
/// or its ownership could not be examined; one with no record, one held by an open
/// fix-round loop, and one whose record cannot be read or pruned are skipped with the
/// reason, and the walk goes on.
fn walk(
    repo: &Path,
    root: &str,
    apply: Option<&BTreeMap<String, String>>,
) -> Result<PruneReviewWalk> {
    walk_between(repo, root, apply, |_| {})
}

/// [`walk`], calling `before_write` with each feature after its record is read and its
/// edits derived and before they are written, which a test replaces to change the record
/// in that window.
fn walk_between(
    repo: &Path,
    root: &str,
    apply: Option<&BTreeMap<String, String>>,
    mut before_write: impl FnMut(&str),
) -> Result<PruneReviewWalk> {
    let names = list_feature_dirs(repo, root)?;
    check_digests(apply, &names, root)?;
    let reads: Vec<(&str, Result<Option<Record>>)> = names
        .iter()
        .map(|feature| (feature.as_str(), Record::read(repo, root, feature)))
        .collect();
    let asked: Vec<&str> = reads
        .iter()
        .filter(|(_, read)| matches!(read, Ok(Some(record)) if !record.fix_rounds_open))
        .map(|(feature, _)| *feature)
        .collect();
    let owners = if asked.is_empty() {
        BTreeMap::new()
    } else {
        spec_ownership::of_specs_at(repo, root, &asked)
    };
    let mut features = Vec::new();
    let mut skipped = Vec::new();
    let mut skip = |feature: &str, reason, detail: Option<String>| {
        skipped.push(ReviewSkipped {
            feature: feature.to_string(),
            reason,
            detail,
        });
    };
    for (feature, read) in reads {
        let record = match read {
            Ok(None) => {
                skip(feature, ReviewSkipReason::NoReviewRecord, None);
                continue;
            }
            Ok(Some(record)) if record.fix_rounds_open => {
                skip(feature, ReviewSkipReason::FixRoundsOpen, None);
                continue;
            }
            Ok(Some(record)) => record,
            Err(error) => {
                skip(feature, unread_reason(&error), Some(error.to_string()));
                continue;
            }
        };
        let fallback = Ownership::NotExamined {
            reason: "the walk named no ownership for it".to_string(),
        };
        let ownership = owners.get(feature).unwrap_or(&fallback);
        let edits = match record.edits(ownership, repo) {
            Ok(edits) => edits,
            Err(error) => {
                skip(feature, unread_reason(&error), Some(error.to_string()));
                continue;
            }
        };
        let digest = apply.and_then(|digests| digests.get(feature));
        let applied = match write_confirmed(&record, &edits, digest, || before_write(feature))? {
            Ok(applied) => applied,
            Err(detail) => {
                skip(feature, ReviewSkipReason::ChangedSinceRead, detail);
                continue;
            }
        };
        let summary = record.summary(Some(&edits), Some(ownership), applied);
        if summary.drops_count > 0
            || summary.rewrites_count > 0
            || summary.ownership == Some(OwnershipState::NotExamined)
        {
            features.push(PruneWalkEntry {
                feature: feature.to_string(),
                summary,
            });
        }
    }
    Ok(PruneReviewWalk {
        examined: u32::try_from(names.len()).unwrap_or(u32::MAX),
        features_count: features.len(),
        features: Some(features),
        skipped_count: skipped.len(),
        skipped: Some(skipped),
    })
}

/// Refuse, before anything is read or written, an applying walk's digest keyed by a name
/// that is no feature directory, which no record can match.
fn check_digests(
    apply: Option<&BTreeMap<String, String>>,
    names: &[String],
    root: &str,
) -> Result<()> {
    match apply
        .into_iter()
        .flat_map(BTreeMap::keys)
        .find(|key| !names.contains(key))
    {
        Some(stray) => Err(PrimitiveError::InvalidArgument {
            primitive: "prune-review".into(),
            argument: "digests".into(),
            reason: format!(
                "`{}` is no feature directory under {root}",
                flatten_line(stray)
            ),
        }),
        None => Ok(()),
    }
}

/// Write `record`'s `edits` when an applying walk holds a `digest` for it, calling
/// `before_write` just before: `Ok(applied)`, `false` with no digest or nothing to write,
/// or `Err` with the detail when the edits are not those the digest names or the record
/// changed before the write, so the walk skips it as changed since read.
///
/// # Errors
///
/// [`PrimitiveError::Io`] when the write fails for another reason.
fn write_confirmed(
    record: &Record,
    edits: &Edits,
    digest: Option<&String>,
    before_write: impl FnOnce(),
) -> Result<std::result::Result<bool, Option<String>>> {
    let Some(digest) = digest else {
        return Ok(Ok(false));
    };
    if *digest != edits.digest {
        return Ok(Err(Some(
            "its edits are not those its preview's digest names".to_string(),
        )));
    }
    if !edits.changes() {
        return Ok(Ok(false));
    }
    before_write();
    match record.snapshot.write_if_unchanged(&edits.after) {
        Ok(()) => Ok(Ok(true)),
        Err(PrimitiveError::ChangedSinceRead { .. }) => Ok(Err(None)),
        Err(other) => Err(other),
    }
}

/// Why a walk passes over a record it could not read or prune.
fn unread_reason(error: &PrimitiveError) -> ReviewSkipReason {
    match error {
        PrimitiveError::InvalidPath { .. } => ReviewSkipReason::OutsideProject,
        PrimitiveError::UnsplicableRecord { .. } => ReviewSkipReason::Unsplicable,
        _ => ReviewSkipReason::RecordUnreadable,
    }
}

/// One feature's preview or apply.
fn summarize(
    feature: &str,
    args: &PruneReviewArgs,
    repo: &Path,
    root: &str,
) -> Result<PruneReviewSummary> {
    summarize_between(feature, args, repo, root, || {})
}

/// [`summarize`], calling `before_write` after everything is read and derived and before
/// the write, which a test replaces to change the record in that window.
fn summarize_between(
    feature: &str,
    args: &PruneReviewArgs,
    repo: &Path,
    root: &str,
    before_write: impl FnOnce(),
) -> Result<PruneReviewSummary> {
    super::validate_feature_argument(repo, root, feature)?;
    if !repo.join(root).join(feature).is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: root.to_string(),
            feature: feature.to_string(),
        });
    }
    let Some(record) = Record::read(repo, root, feature)? else {
        return Ok(Record::missing(root, feature));
    };
    if record.fix_rounds_open {
        return Ok(record.summary(None, None, false));
    }
    let ownership = spec_ownership::of_spec_at(repo, root, feature);
    let edits = record.edits(&ownership, repo)?;
    if args.apply && args.digest.as_deref() != Some(edits.digest.as_str()) {
        // What the operator confirmed is no longer what an apply would write.
        return Err(PrimitiveError::ChangedSinceRead {
            path: record.rel.as_str().into(),
        });
    }
    let applied = args.apply && edits.changes();
    before_write();
    if applied {
        record.snapshot.write_if_unchanged(&edits.after)?;
    }
    Ok(record.summary(Some(&edits), Some(&ownership), applied))
}

/// A review record read for pruning, with the snapshot its write compares against.
struct Record {
    /// The record's path from the project root.
    rel: String,
    snapshot: ReadSnapshot,
    text: String,
    /// The record carries `fix-rounds-open: true`.
    fix_rounds_open: bool,
}

/// What a record's pruning drops and rewrites, and the record it leaves.
struct Edits {
    drops: Vec<ReviewRecordDrop>,
    /// How many `## Excluded from scope` lines are rewritten as their path alone.
    rewrites: usize,
    after: String,
    /// The record as read and these drops, as an apply names them.
    digest: String,
}

impl Edits {
    /// Whether anything is dropped or rewritten, so a write is owed.
    fn changes(&self) -> bool {
        !self.drops.is_empty() || self.rewrites > 0
    }
}

/// Which anchors of one stored waiver are dropped.
struct WaiverCut {
    /// The waiver's index in `waivers:`.
    index: usize,
    /// The list its anchors are read from.
    list: AnchorList,
    /// The indexes, in that list, of the anchors dropped.
    anchors: BTreeSet<usize>,
    /// How many anchors the list holds, so a cut of every one drops the waiver whole.
    of: usize,
}

impl WaiverCut {
    fn whole(&self) -> bool {
        self.anchors.len() == self.of
    }
}

impl Record {
    /// The record of `feature`, or `None` when it has none. The record and both of its
    /// lists are parsed here, so one that does not parse is refused before anything is
    /// derived from it.
    fn read(repo: &Path, root: &str, feature: &str) -> Result<Option<Self>> {
        let rel = format!("{root}/{feature}/{REVIEW_RECORD_FILE}");
        let snapshot = ReadSnapshot::read(repo, &rel)?;
        let Some(text) = snapshot.text()?.map(str::to_string) else {
            return Ok(None);
        };
        let path = Path::new(&rel);
        let (fm, _, _) = split_frontmatter_with_offset(&text, path)?;
        let block: ReviewBlock =
            serde_norway::from_str(fm).map_err(|source| PrimitiveError::Yaml {
                path: path.into(),
                source,
            })?;
        recorded_list_in::<RawDecision>(fm, path, DECISIONS)?;
        recorded_list_in::<RawWaiver>(fm, path, WAIVERS)?;
        Ok(Some(Self {
            fix_rounds_open: block.fix_rounds_open == Some(true),
            rel,
            snapshot,
            text,
        }))
    }

    /// The summary of a feature with no record.
    fn missing(root: &str, feature: &str) -> PruneReviewSummary {
        PruneReviewSummary {
            path: format!("{root}/{feature}/{REVIEW_RECORD_FILE}"),
            missing: true,
            fix_rounds_open: false,
            drops: Some(Vec::new()),
            drops_count: 0,
            rewrites_count: 0,
            digest: None,
            ownership: None,
            ownership_reason: None,
            applied: false,
            size_before: size_of(""),
            size_after: size_of(""),
        }
    }

    /// The summary of this record: its edits, when it was examined.
    fn summary(
        &self,
        edits: Option<&Edits>,
        ownership: Option<&Ownership>,
        applied: bool,
    ) -> PruneReviewSummary {
        let (state, reason) = ownership.map(Ownership::state).unzip();
        let drops = edits.map(|edits| edits.drops.clone()).unwrap_or_default();
        PruneReviewSummary {
            path: self.rel.clone(),
            missing: false,
            fix_rounds_open: self.fix_rounds_open,
            drops_count: drops.len(),
            drops: Some(drops),
            rewrites_count: edits.map_or(0, |edits| edits.rewrites),
            digest: edits.map(|edits| edits.digest.clone()),
            ownership: state,
            ownership_reason: reason.flatten(),
            applied,
            size_before: size_of(&self.text),
            size_after: size_of(edits.map_or(self.text.as_str(), |edits| edits.after.as_str())),
        }
    }

    /// The drops and rewrites this record takes under `ownership`, and the record they
    /// leave, checked against the original read back through the list readers.
    fn edits(&self, ownership: &Ownership, repo: &Path) -> Result<Edits> {
        let path = Path::new(&self.rel);
        let text = self.text.as_str();
        let (fm, body, offset) = split_frontmatter_with_offset(text, path)?;
        let decisions: Vec<RawDecision> = recorded_list_in(fm, path, DECISIONS)?;
        let waivers: Vec<RawWaiver> = recorded_list_in(fm, path, WAIVERS)?;
        // Only a file that is there and does not belong is judged, by the test a review's
        // run applies: one gone from disk expires at the next review (AC4), and nothing
        // drops while ownership is unknown.
        let drops_file = |file: &str| match ownership {
            Ownership::Examined(owned) => still_on_disk(repo, file) && !owned.belongs(file),
            Ownership::NotExamined { .. } => false,
        };

        let mut drops: Vec<ReviewRecordDrop> = Vec::new();
        let mut dropped_decisions = BTreeSet::new();
        for (index, decision) in decisions.iter().enumerate() {
            let Ok(decision) = decision.to_ref() else {
                continue;
            };
            if let Some(file) = key_anchor(&decision.key).filter(|file| drops_file(file)) {
                dropped_decisions.insert(index);
                drops.push(ReviewRecordDrop {
                    list: ReviewRecordList::Decisions,
                    entry: decision.key.clone(),
                    file: file.to_string(),
                });
            }
        }
        let mut cuts = Vec::new();
        for (index, waiver) in waivers.iter().enumerate() {
            let Some((list, files)) = anchor_files(waiver) else {
                continue;
            };
            let anchors: BTreeSet<usize> = files
                .iter()
                .enumerate()
                .filter(|(_, file)| drops_file(file))
                .map(|(at, _)| at)
                .collect();
            for &at in &anchors {
                let drop = ReviewRecordDrop {
                    list: ReviewRecordList::Waivers,
                    entry: waiver.rule().unwrap_or_default().to_string(),
                    file: files[at].to_string(),
                };
                if !drops.contains(&drop) {
                    drops.push(drop);
                }
            }
            if !anchors.is_empty() {
                cuts.push(WaiverCut {
                    index,
                    list,
                    anchors,
                    of: files.len(),
                });
            }
        }

        let unsplicable = |reason: String| PrimitiveError::UnsplicableRecord {
            path: path.into(),
            reason,
        };
        let new_fm = splice_frontmatter(
            fm,
            (&dropped_decisions, decisions.len()),
            (&cuts, waivers.len()),
        )
        .map_err(unsplicable)?;
        let (new_body, rewrites) = rewrite_excluded(body);
        let body_start = text.len() - body.len();
        let after = format!(
            "{}{new_fm}{}{new_body}",
            &text[..offset],
            &text[offset + fm.len()..body_start]
        );
        read_back(fm, &after, path, &dropped_decisions, &cuts).map_err(unsplicable)?;
        let digest = digest_of(text, &drops);
        Ok(Edits {
            drops,
            rewrites,
            after,
            digest,
        })
    }
}

/// The sha256 an apply names a preview's edits by: the record as read, from which its
/// rewrites follow, and the drops derived from it, so a change to either, the record or
/// what belongs to the spec, refuses the apply.
fn digest_of(text: &str, drops: &[ReviewRecordDrop]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hasher.update([0]);
    hasher.update(serde_json::to_vec(drops).unwrap_or_default());
    hex(&hasher.finalize())
}

/// One top-level list of a frontmatter, by line index: the line of its key, each item's
/// first line, and the line after its last.
struct ListSpan {
    key: usize,
    items: Vec<usize>,
    end: usize,
}

impl ListSpan {
    /// The list under `key` in `lines`, as `write-review` renders one: a `{key}:` line at
    /// column 0, and each item opening `  - ` at two spaces, every line of it indented. `None`
    /// when no such key line is there.
    fn of(lines: &[&str], key: &str) -> Option<Self> {
        let opener = format!("{key}:");
        let at = lines.iter().position(|line| content(line) == opener)?;
        let mut items = Vec::new();
        let mut end = at + 1;
        while let Some(line) = lines.get(end).map(|line| content(line)) {
            if !line.is_empty() && !line.starts_with(' ') {
                break;
            }
            if line.starts_with("  - ") {
                items.push(end);
            }
            end += 1;
        }
        Some(Self {
            key: at,
            items,
            end,
        })
    }

    /// The lines of item `index`.
    fn item(&self, index: usize) -> std::ops::Range<usize> {
        self.items[index]..self.items.get(index + 1).copied().unwrap_or(self.end)
    }
}

/// A line without its line ending.
fn content(line: &str) -> &str {
    line.trim_end_matches(['\r', '\n'])
}

/// `fm` without the dropped decisions, the waivers dropped whole, and the dropped anchors
/// of the others, each removed as the lines it renders as, and a list emptied removed with
/// its key, as `write-review` writes a list with nothing in it. `Err` names what would not
/// split into one item per entry.
fn splice_frontmatter(
    fm: &str,
    (dropped, decisions): (&BTreeSet<usize>, usize),
    (cuts, waivers): (&[WaiverCut], usize),
) -> std::result::Result<String, String> {
    let lines: Vec<&str> = fm.split_inclusive('\n').collect();
    let mut removed: BTreeSet<usize> = BTreeSet::new();
    let span = |key: &str, entries: usize| -> std::result::Result<ListSpan, String> {
        ListSpan::of(&lines, key)
            .filter(|span| span.items.len() == entries)
            .ok_or_else(|| {
                format!(
                    "its `{key}:` list's {entries} entries are not {entries} items at two \
                     spaces under a `{key}:` line"
                )
            })
    };
    if !dropped.is_empty() {
        let list = span(DECISIONS, decisions)?;
        if dropped.len() == decisions {
            removed.extend(list.key..list.end);
        } else {
            for &index in dropped {
                removed.extend(list.item(index));
            }
        }
    }
    if !cuts.is_empty() {
        let list = span(WAIVERS, waivers)?;
        let whole = cuts.iter().filter(|cut| cut.whole()).count();
        if whole == waivers {
            removed.extend(list.key..list.end);
        }
        for cut in cuts {
            let item = list.item(cut.index);
            if cut.whole() {
                removed.extend(item);
            } else {
                removed.extend(anchor_lines(&lines, item, cut)?);
            }
        }
    }
    let mut spliced: String = lines
        .iter()
        .enumerate()
        .filter(|(at, _)| !removed.contains(at))
        .map(|(_, line)| *line)
        .collect();
    // The frontmatter ends without its last newline, which the closing fence holds, so a
    // removal reaching its last line leaves a newline the fence would double.
    if !fm.ends_with('\n') && spliced.ends_with('\n') {
        spliced.pop();
    }
    Ok(spliced)
}

/// The lines of the anchors `cut` drops from the waiver whose lines are `item`: each item,
/// at six spaces, of its `anchors:` list or its `file:` list, whose items must be one per
/// anchor the classifier reads.
fn anchor_lines(
    lines: &[&str],
    item: std::ops::Range<usize>,
    cut: &WaiverCut,
) -> std::result::Result<Vec<usize>, String> {
    let field = match cut.list {
        AnchorList::Anchors => "anchors:",
        AnchorList::File => "file:",
    };
    let at = item
        .clone()
        .find(|&at| {
            let line = content(lines[at]);
            line == format!("    {field}") || line == format!("  - {field}")
        })
        .ok_or_else(|| format!("a waiver's `{field}` is not a list on lines of its own"))?;
    let mut starts = Vec::new();
    let mut end = at + 1;
    while end < item.end {
        let line = content(lines[end]);
        if !line.is_empty() && !line.starts_with("      ") {
            break;
        }
        if line.starts_with("      - ") {
            starts.push(end);
        }
        end += 1;
    }
    if starts.len() != cut.of {
        return Err(format!(
            "a waiver's `{field}` list is not {} items at six spaces",
            cut.of
        ));
    }
    Ok(cut
        .anchors
        .iter()
        .flat_map(|&index| starts[index]..starts.get(index + 1).copied().unwrap_or(end))
        .collect())
}

/// `body` with each `## Excluded from scope` line that names the specs that claimed its
/// path rewritten as the path alone, and how many were. A heading or line inside a fenced
/// block or an HTML comment is not structure, so a Summary quoting one is left as it is.
fn rewrite_excluded(body: &str) -> (String, usize) {
    let mut scanner = SkipScanner::default();
    let mut inside = false;
    let mut out = String::with_capacity(body.len());
    let mut rewritten = 0;
    for line in body.split_inclusive('\n') {
        let text = content(line);
        let skipped = scanner.skip(text);
        if !skipped && let Some((level, heading)) = parse_atx_heading(text) {
            if level <= 2 {
                inside = level == 2 && heading.trim() == EXCLUDED_HEADING;
            }
        } else if inside
            && !skipped
            && let Some(path) = claimed_path(text)
        {
            out.push_str("- `");
            out.push_str(path);
            out.push('`');
            out.push_str(&line[text.len()..]);
            rewritten += 1;
            continue;
        }
        out.push_str(line);
    }
    (out, rewritten)
}

/// The path of an `## Excluded from scope` line as `write-review` rendered it before
/// `64946fe7`: ``- `{path}` — {spec}, {spec}``, each claimant a feature directory's name.
/// A fix round's ``- `{path}` — left out of fix round {n}`` names none, and is no such line.
fn claimed_path(line: &str) -> Option<&str> {
    let (path, claimants) = line.strip_prefix("- `")?.rsplit_once("` — ")?;
    (!path.is_empty()
        && !claimants.is_empty()
        && claimants
            .split(", ")
            .all(|claimant| parse_feature_dir(claimant).is_some()))
    .then_some(path)
}

/// Refuse a spliced record that does not read, through the readers every consumer uses,
/// as the original less what was dropped: every other frontmatter field the same value,
/// whether or not the record's schema names it, `decisions:` less the dropped entries,
/// and `waivers:` less the waivers dropped whole and the dropped anchors of the others.
fn read_back(
    before_fm: &str,
    after: &str,
    path: &Path,
    dropped: &BTreeSet<usize>,
    cuts: &[WaiverCut],
) -> std::result::Result<(), String> {
    let (after_fm, _, _) =
        split_frontmatter_with_offset(after, path).map_err(|error| error.to_string())?;
    if others(before_fm)? != others(after_fm)? {
        return Err("a field beside the two lists would change".to_string());
    }
    let list = |fm: &str, key: &str| {
        recorded_list_in::<serde_norway::Value>(fm, path, key).map_err(|error| error.to_string())
    };
    let expected: Vec<serde_norway::Value> = list(before_fm, DECISIONS)?
        .into_iter()
        .enumerate()
        .filter(|(index, _)| !dropped.contains(index))
        .map(|(_, entry)| entry)
        .collect();
    if list(after_fm, DECISIONS)? != expected {
        return Err("its `decisions:` list would not read as it did less the drops".to_string());
    }
    let mut expected = list(before_fm, WAIVERS)?;
    for cut in cuts.iter().rev() {
        if cut.whole() {
            expected.remove(cut.index);
            continue;
        }
        let field = match cut.list {
            AnchorList::Anchors => "anchors",
            AnchorList::File => "file",
        };
        if let Some(serde_norway::Value::Sequence(anchors)) = expected
            .get_mut(cut.index)
            .and_then(|waiver| waiver.get_mut(field))
        {
            for &at in cut.anchors.iter().rev() {
                anchors.remove(at);
            }
        }
    }
    if list(after_fm, WAIVERS)? != expected {
        return Err("its `waivers:` list would not read as it did less the drops".to_string());
    }
    Ok(())
}

/// The frontmatter `fm` as a YAML value without its two lists: every field the splice must
/// leave as it was, an open-schema field included.
fn others(fm: &str) -> std::result::Result<serde_norway::Value, String> {
    if fm.trim().is_empty() {
        return Ok(serde_norway::Value::Null);
    }
    let mut value: serde_norway::Value =
        serde_norway::from_str(fm).map_err(|error| error.to_string())?;
    if let serde_norway::Value::Mapping(map) = &mut value {
        map.remove(DECISIONS);
        map.remove(WAIVERS);
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::{commit_all, write};
    use crate::schema::primitives::{DecisionRecord, ProcessDecisionsArgs, ProcessWaiversArgs};
    use ReviewRecordList::{Decisions, Waivers};
    use ReviewSkipReason::{
        ChangedSinceRead, FixRoundsOpen, NoReviewRecord, OutsideProject, RecordUnreadable,
        Unsplicable,
    };
    use std::fs;

    const FEATURE: &str = "001-x";
    const RECORD: &str = "specs/001-x/review.md";
    const OTHER_RECORD: &str = "specs/002-y/review.md";

    const FOREIGN_KEY: &str = "bug: a finding — `src/foreign.rs:3`";
    const OWN_KEY: &str = "bug: a finding — `src/own.rs`";
    const PLANNED_KEY: &str = "bug: a finding — `src/planned.rs`";
    const GONE_KEY: &str = "bug: a finding — `src/gone.rs`";

    const PLAN: &str = "## Affected Files\n\n| File | Action | Purpose |\n| --- | --- | --- |\n\
                        | `src/planned.rs` | Modify | p |\n";

    const HEAD: &str =
        "---\nspec: 001-x\nlast-run: 2026-09-25T00:00:00Z\nmust-violations: 0\nx-custom: kept\n";
    const CLAIMED: &str = "- `src/a.rs` — 001-x, 002-y\n";
    const TAIL: &str = "blocking: false\n---\n\n## Summary\n\nA run.\n\n## Excluded from scope\n\n\
                        - `src/a.rs` — 001-x, 002-y\n- `src/b.rs`\n\
                        - `src/c.rs` — left out of fix round 2\n";

    fn decision(key: &str) -> String {
        format!(
            "  - key: \"{key}\"\n    outcome: waived\n    reason: a person's reason\n    \
             decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n"
        )
    }

    /// One item of a waiver's `anchors:` list.
    fn anchor(file: &str) -> String {
        format!("      - file: {file}\n        site: s\n        covers: 1\n")
    }

    fn waiver(rule: &str, files: &[&str]) -> String {
        let anchors: String = files.iter().map(|file| anchor(file)).collect();
        format!(
            "  - rule: {rule}\n    anchors:\n{anchors}    reason: why\n    \
             waived-at: 2026-05-10T14:40:00Z\n    waived-by: dev@example.com\n"
        )
    }

    /// A record holding `decisions` and `waivers` between its other fields.
    fn record(decisions: &[&str], waivers: &[String]) -> String {
        let mut text = HEAD.to_string();
        if !decisions.is_empty() {
            text.push_str("decisions:\n");
            for key in decisions {
                text.push_str(&decision(key));
            }
        }
        if !waivers.is_empty() {
            text.push_str("waivers:\n");
            for waiver in waivers {
                text.push_str(waiver);
            }
        }
        text.push_str(TAIL);
        text
    }

    /// A decision on a file of each kind, a waiver with an anchor that does not belong and
    /// one that does, and a waiver whose only anchor does not belong.
    fn full_record() -> String {
        record(
            &[FOREIGN_KEY, OWN_KEY, PLANNED_KEY, GONE_KEY],
            &[
                waiver("SEC-BE-014", &["src/foreign.rs", "src/own.rs"]),
                waiver("SEC-BE-015", &["src/other.rs"]),
            ],
        )
    }

    /// [`full_record`] as a prune leaves it.
    fn pruned() -> String {
        full_record()
            .replacen(&decision(FOREIGN_KEY), "", 1)
            .replacen(&anchor("src/foreign.rs"), "", 1)
            .replacen(&waiver("SEC-BE-015", &["src/other.rs"]), "", 1)
            .replacen(CLAIMED, "- `src/a.rs`\n", 1)
    }

    /// A repository with `src/foreign.rs`, `src/other.rs` and `src/planned.rs` from a
    /// commit no spec claims; spec 001-x, whose plan names `src/planned.rs` and whose own
    /// commit added `src/own.rs`; spec 002-y with no record; and 001-x's record holding
    /// `text`, uncommitted.
    fn project(text: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let repository = git2::Repository::init(root).unwrap();
        for file in ["src/foreign.rs", "src/other.rs", "src/planned.rs"] {
            write(&root.join(file), "fn f() {}\n");
        }
        #[cfg(unix)]
        std::os::unix::fs::symlink("nowhere.rs", root.join("src/link.rs")).unwrap();
        commit_all(&repository, "chore: code no spec claims");
        write(
            &root.join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\n---\n# 001\n",
        );
        write(&root.join("specs/001-x/plan.md"), PLAN);
        write(
            &root.join("specs/002-y/spec.md"),
            "---\nstatus: in-progress\n---\n# 002\n",
        );
        commit_all(&repository, "chore: seed");
        write(&root.join("src/own.rs"), "fn own() {}\n");
        commit_all(&repository, "feat(001): own");
        write(&root.join(RECORD), text);
        tmp
    }

    fn preview() -> PruneReviewArgs {
        PruneReviewArgs {
            feature: Some(FEATURE.into()),
            ..PruneReviewArgs::default()
        }
    }

    fn apply(digest: &str) -> PruneReviewArgs {
        PruneReviewArgs {
            feature: Some(FEATURE.into()),
            apply: true,
            digest: Some(digest.into()),
            ..PruneReviewArgs::default()
        }
    }

    fn all() -> PruneReviewArgs {
        PruneReviewArgs {
            all: true,
            ..PruneReviewArgs::default()
        }
    }

    fn apply_all(digests: BTreeMap<String, String>) -> PruneReviewArgs {
        PruneReviewArgs {
            all: true,
            apply: true,
            digests,
            ..PruneReviewArgs::default()
        }
    }

    fn summary(args: &PruneReviewArgs, repo: &Path) -> Result<PruneReviewSummary> {
        super::run(args, repo).map(|result| result.summary.expect("a single-feature summary"))
    }

    fn walked(args: &PruneReviewArgs, repo: &Path) -> Result<PruneReviewWalk> {
        super::run(args, repo).map(|result| result.walk.expect("a walk"))
    }

    fn digest(repo: &Path) -> String {
        summary(&preview(), repo).unwrap().digest.unwrap()
    }

    /// Each listed record's digest, keyed by its feature, as a host confirms a walk.
    fn digests(walk: &PruneReviewWalk) -> BTreeMap<String, String> {
        walk.features
            .as_deref()
            .unwrap()
            .iter()
            .map(|entry| (entry.feature.clone(), entry.summary.digest.clone().unwrap()))
            .collect()
    }

    fn read(repo: &Path, rel: &str) -> String {
        fs::read_to_string(repo.join(rel)).unwrap()
    }

    fn dropped(list: ReviewRecordList, entry: &str, file: &str) -> ReviewRecordDrop {
        ReviewRecordDrop {
            list,
            entry: entry.into(),
            file: file.into(),
        }
    }

    fn skip(feature: &str, reason: ReviewSkipReason) -> (String, ReviewSkipReason) {
        (feature.to_string(), reason)
    }

    fn skips(walk: &PruneReviewWalk) -> Vec<(String, ReviewSkipReason)> {
        walk.skipped
            .as_deref()
            .unwrap()
            .iter()
            .map(|skipped| (skipped.feature.clone(), skipped.reason))
            .collect()
    }

    fn listed(walk: &PruneReviewWalk) -> Vec<(String, bool)> {
        walk.features
            .as_deref()
            .unwrap()
            .iter()
            .map(|entry| (entry.feature.clone(), entry.summary.applied))
            .collect()
    }

    fn set_read_size(repo: &Path, value: &str) {
        write(
            &repo.join(".ductus/config.toml"),
            &format!("[artifacts]\nread-size-bytes = {value}\n"),
        );
    }

    /// AC5: a preview names each decision and waiver anchor on a file that does not belong
    /// to the spec and counts each line still naming its path's claimants, and writes
    /// nothing; an entry on a file the plan names, one on a file a commit attributed to the
    /// spec changed, and one on a deleted file stay (AC4).
    #[test]
    fn a_preview_names_each_entry_on_a_file_that_does_not_belong_and_writes_nothing() {
        let tmp = project(&full_record());
        let summary = summary(&preview(), tmp.path()).unwrap();
        assert_eq!(
            summary.drops.as_deref().unwrap(),
            [
                dropped(Decisions, FOREIGN_KEY, "src/foreign.rs"),
                dropped(Waivers, "SEC-BE-014", "src/foreign.rs"),
                dropped(Waivers, "SEC-BE-015", "src/other.rs"),
            ]
        );
        assert_eq!(summary.drops_count, 3);
        assert_eq!(summary.rewrites_count, 1);
        assert_eq!(summary.ownership, Some(OwnershipState::Examined));
        assert!(summary.digest.is_some());
        assert!(!summary.applied);
        assert_eq!(summary.size_after.bytes, pruned().len());
        assert_eq!(read(tmp.path(), RECORD), full_record());
    }

    /// AC5: an apply removes each dropped entry's lines and a dropped anchor's alone, and
    /// rewrites the claimant line, leaving every other byte as it was.
    #[test]
    fn an_apply_drops_them_and_rewrites_each_claimant_line_leaving_every_other_byte() {
        let tmp = project(&full_record());
        let applied = summary(&apply(&digest(tmp.path())), tmp.path()).unwrap();
        assert!(applied.applied);
        assert_eq!(read(tmp.path(), RECORD), pruned());
        assert_eq!(applied.size_after.bytes, pruned().len());
    }

    /// What a prune drops is what the spec's next unrestricted review drops: the same
    /// reading of a key's file, `:line` and all, and the same on-disk test, under which a
    /// dangling link is there (spec 086, AC2 and AC5).
    #[cfg(unix)]
    #[test]
    fn a_prune_drops_what_an_unrestricted_review_drops() {
        let link_key = "bug: a finding — `src/link.rs`";
        let tmp = project(&record(
            &[FOREIGN_KEY, OWN_KEY, PLANNED_KEY, GONE_KEY, link_key],
            &[
                waiver(
                    "SEC-BE-014",
                    &["src/foreign.rs", "src/own.rs", "src/link.rs"],
                ),
                waiver("SEC-BE-015", &["src/other.rs"]),
            ],
        ));
        let prune: BTreeSet<String> = summary(&preview(), tmp.path())
            .unwrap()
            .drops
            .unwrap()
            .iter()
            .map(|drop| format!("{:?} {} at {}", drop.list, drop.entry, drop.file))
            .collect();
        let decisions = crate::primitives::process_decisions::compute(
            &ProcessDecisionsArgs {
                feature: Some(FEATURE.into()),
                record: DecisionRecord::Review,
                fired: Vec::new(),
                fired_violations: Vec::new(),
                restricted: false,
                since: None,
                fired_json: None,
                fired_violations_json: None,
            },
            tmp.path(),
        )
        .unwrap();
        let waivers = crate::primitives::process_waivers::compute(
            &ProcessWaiversArgs {
                feature: Some(FEATURE.into()),
                ..ProcessWaiversArgs::default()
            },
            tmp.path(),
        )
        .unwrap();
        let review: BTreeSet<String> = decisions
            .dropped
            .iter()
            .map(|d| format!("Decisions {} at {}", d.key, key_anchor(&d.key).unwrap()))
            .chain(
                waivers
                    .dropped
                    .iter()
                    .map(|w| format!("Waivers {} at {}", w.rule, w.file)),
            )
            .collect();
        assert_eq!(prune, review, "{:?}", decisions.notices);
        assert_eq!(prune.len(), 5, "{prune:?}");
    }

    /// A record with nothing to drop or rewrite is not written: a change made after the
    /// read stands, where a write would have been refused for it.
    #[test]
    fn a_record_with_nothing_to_drop_or_rewrite_is_not_written() {
        let clean = record(&[OWN_KEY], &[]).replacen(CLAIMED, "- `src/a.rs`\n", 1);
        let tmp = project(&clean);
        let summary = summary(&preview(), tmp.path()).unwrap();
        assert_eq!((summary.drops_count, summary.rewrites_count), (0, 0));
        let other = format!("{clean}\n<!-- another writer -->\n");
        let applied = summarize_between(
            FEATURE,
            &apply(summary.digest.as_deref().unwrap()),
            tmp.path(),
            "specs",
            || fs::write(tmp.path().join(RECORD), &other).unwrap(),
        )
        .unwrap();
        assert!(!applied.applied);
        assert_eq!(read(tmp.path(), RECORD), other);
    }

    /// Only a line naming claimant specs, directly or in a subsection under `## Excluded
    /// from scope` and outside a fence or a comment, is rewritten; a fix round's line, a
    /// line naming something else and a line under another heading are left.
    #[test]
    fn only_a_line_naming_claimant_specs_under_its_heading_is_rewritten() {
        let body = "\n## Summary\n\n- `src/s.rs` — 001-x\n\n```text\n## Excluded from scope\n```\n\n\
                    - `src/t.rs` — 001-x\n\n## Excluded from scope\n\n\
                    - `src/a.rs` — 001-x, 1234.1-z\n- `src/c.rs` — left out of fix round 2\n\
                    - `src/d.rs` — 001-x, not a spec\n\n```text\n- `src/f.rs` — 001-x\n```\n\n\
                    <!--\n- `src/e.rs` — 001-x\n-->\n\n### Notes\n\n- `src/g.rs` — 002-y\n\n\
                    ## After\n\n- `src/h.rs` — 001-x\n";
        let (out, rewritten) = rewrite_excluded(body);
        assert_eq!(rewritten, 2);
        assert_eq!(
            out,
            body.replacen("- `src/a.rs` — 001-x, 1234.1-z\n", "- `src/a.rs`\n", 1)
                .replacen("- `src/g.rs` — 002-y\n", "- `src/g.rs`\n", 1)
        );
    }

    /// A record held by an open fix-round loop is left for the run that holds it: nothing
    /// is examined, and a walk skips it.
    #[test]
    fn a_record_holding_fix_rounds_open_is_left_as_it_is() {
        let held = full_record().replacen(
            "blocking: false\n",
            "blocking: false\nfix-rounds-open: true\n",
            1,
        );
        let tmp = project(&held);
        let summary = summary(&preview(), tmp.path()).unwrap();
        assert!(summary.fix_rounds_open);
        assert_eq!((summary.drops_count, summary.rewrites_count), (0, 0));
        assert_eq!(summary.digest, None);
        let walk = walked(&all(), tmp.path()).unwrap();
        assert_eq!(
            skips(&walk),
            [skip(FEATURE, FixRoundsOpen), skip("002-y", NoReviewRecord)]
        );
        assert_eq!(walk.features_count, 0);
        assert_eq!(read(tmp.path(), RECORD), held);
    }

    /// An undeterminable record is never rewritten as though it were read: one feature's
    /// call is refused naming the file, held by a fix-round loop or not, and a walk skips
    /// it and goes on.
    #[test]
    fn an_unparseable_record_or_list_is_refused_naming_the_file_and_skipped_by_a_walk() {
        for broken in [
            "no frontmatter\n".to_string(),
            "---\nspec: 001-x\nmust-violations: [\n---\n".to_string(),
            "---\nspec: 001-x\ndecisions: 3\n---\n".to_string(),
            "---\nspec: 001-x\nwaivers: {a: 1}\n---\n".to_string(),
            "---\nspec: 001-x\nmust-violations: lots\n---\n".to_string(),
            "---\nspec: 001-x\nfix-rounds-open: true\ndecisions: 3\n---\n".to_string(),
            "---\nspec: 001-x\nfix-rounds-open: true\nwaivers: {a: 1}\n---\n".to_string(),
        ] {
            let tmp = project(&broken);
            let error = summary(&preview(), tmp.path()).unwrap_err();
            assert!(error.to_string().contains(RECORD), "{broken}: {error}");
            let walk = walked(&all(), tmp.path()).unwrap();
            assert_eq!(
                skips(&walk),
                [
                    skip(FEATURE, RecordUnreadable),
                    skip("002-y", NoReviewRecord)
                ],
                "{broken}"
            );
            let detail = walk.skipped.unwrap()[0].detail.clone().unwrap();
            assert!(detail.contains(RECORD), "{detail}");
            assert_eq!(read(tmp.path(), RECORD), broken);
        }
    }

    /// A list that will not splice line by line, as one written in flow style, is refused
    /// writing nothing, and a walk skips it.
    #[test]
    fn a_flow_style_list_is_refused_as_unsplicable_and_skipped_by_a_walk() {
        let flow = format!(
            "{HEAD}decisions: [{{key: \"{FOREIGN_KEY}\", outcome: waived, reason: r, \
             decided-at: 2026-09-25T00:00:00Z, decided-by: dev@example.com}}]\n{TAIL}"
        );
        let tmp = project(&flow);
        let error = summary(&preview(), tmp.path()).unwrap_err();
        assert!(
            matches!(error, PrimitiveError::UnsplicableRecord { .. }),
            "{error}"
        );
        let walk = walked(&all(), tmp.path()).unwrap();
        assert_eq!(
            skips(&walk),
            [skip(FEATURE, Unsplicable), skip("002-y", NoReviewRecord)]
        );
        assert_eq!(read(tmp.path(), RECORD), flow);
    }

    /// `BE-TXN-002`: a record another writer changed between the apply's read and its write
    /// is refused, and the other writer's change stands.
    #[test]
    fn an_apply_refuses_a_record_changed_after_it_was_read() {
        let tmp = project(&full_record());
        let other = format!("{}\n<!-- another writer -->\n", full_record());
        let error = summarize_between(
            FEATURE,
            &apply(&digest(tmp.path())),
            tmp.path(),
            "specs",
            || fs::write(tmp.path().join(RECORD), &other).unwrap(),
        )
        .unwrap_err();
        assert!(
            matches!(error, PrimitiveError::ChangedSinceRead { .. }),
            "{error}"
        );
        assert_eq!(read(tmp.path(), RECORD), other);
    }

    /// The digest covers what belongs, so an apply after a commit gave the spec a file its
    /// preview would drop is refused, though the record's text is the same.
    #[test]
    fn an_apply_is_refused_once_a_commit_changes_what_belongs() {
        let tmp = project(&full_record());
        let confirmed = digest(tmp.path());
        let repository = git2::Repository::open(tmp.path()).unwrap();
        write(&tmp.path().join("src/foreign.rs"), "fn changed() {}\n");
        commit_all(&repository, "fix(001): foreign");
        let error = summary(&apply(&confirmed), tmp.path()).unwrap_err();
        assert!(
            matches!(error, PrimitiveError::ChangedSinceRead { .. }),
            "{error}"
        );
        assert_eq!(read(tmp.path(), RECORD), full_record());
        let fresh = summary(&preview(), tmp.path()).unwrap();
        assert!(
            !fresh
                .drops
                .unwrap()
                .iter()
                .any(|drop| drop.file == "src/foreign.rs")
        );
    }

    /// A list the drops empty goes with its key, as `write-review` writes an empty list.
    #[test]
    fn a_list_emptied_loses_its_key() {
        let text = record(&[FOREIGN_KEY], &[waiver("SEC-BE-015", &["src/other.rs"])]);
        let tmp = project(&text);
        summary(&apply(&digest(tmp.path())), tmp.path()).unwrap();
        assert_eq!(
            read(tmp.path(), RECORD),
            format!("{HEAD}{}", TAIL.replacen(CLAIMED, "- `src/a.rs`\n", 1))
        );
    }

    /// A removal reaching the frontmatter's last line leaves one newline before the
    /// closing fence, which holds the last.
    #[test]
    fn a_removal_at_the_frontmatters_last_line_leaves_one_newline_before_the_fence() {
        let kept = waiver("SEC-BE-014", &["src/own.rs"]);
        let gone = waiver("SEC-BE-015", &["src/other.rs"]);
        let text = format!("{HEAD}waivers:\n{kept}{gone}---\n\n## Summary\n");
        let tmp = project(&text);
        summary(&apply(&digest(tmp.path())), tmp.path()).unwrap();
        assert_eq!(
            read(tmp.path(), RECORD),
            format!("{HEAD}waivers:\n{kept}---\n\n## Summary\n")
        );
    }

    /// A waiver keeps each anchor that belongs, in an `anchors:` list and in a `file:`
    /// list, losing only the item of each that does not.
    #[test]
    fn a_waiver_keeps_the_anchors_that_belong_in_either_list() {
        let file_list = "  - rule: SEC-BE-016\n    file:\n      - src/foreign.rs\n      \
                         - src/own.rs\n    reason: why\n    waived-at: 2026-05-10T14:40:00Z\n    \
                         waived-by: dev@example.com\n";
        let text = record(
            &[],
            &[
                waiver("SEC-BE-014", &["src/own.rs", "src/foreign.rs"]),
                file_list.to_string(),
            ],
        );
        let tmp = project(&text);
        let summary = summary(&apply(&digest(tmp.path())), tmp.path()).unwrap();
        assert_eq!(
            summary.drops.unwrap(),
            [
                dropped(Waivers, "SEC-BE-014", "src/foreign.rs"),
                dropped(Waivers, "SEC-BE-016", "src/foreign.rs"),
            ]
        );
        assert_eq!(
            read(tmp.path(), RECORD),
            text.replacen(&anchor("src/foreign.rs"), "", 1)
                .replacen("      - src/foreign.rs\n", "", 1)
                .replacen(CLAIMED, "- `src/a.rs`\n", 1)
        );
    }

    /// The read-back refuses a splice that would change a field beside the two lists, one
    /// the record's schema does not name included, or leave a list reading other than as
    /// it did less its drops.
    #[test]
    fn a_splice_that_would_change_anything_but_the_drops_is_refused() {
        let before = "spec: 001-x\nx-custom: kept\ndecisions:\n  - key: a\n  - key: b";
        let path = Path::new(RECORD);
        let none = BTreeSet::new();
        assert_eq!(
            read_back(before, &format!("---\n{before}\n---\n"), path, &none, &[]),
            Ok(())
        );
        let lost = "---\nspec: 001-x\ndecisions:\n  - key: a\n  - key: b\n---\n";
        assert_eq!(
            read_back(before, lost, path, &none, &[]),
            Err("a field beside the two lists would change".to_string())
        );
        let short = "---\nspec: 001-x\nx-custom: kept\ndecisions:\n  - key: a\n---\n";
        assert!(read_back(before, short, path, &none, &[]).is_err());
        assert_eq!(
            read_back(before, short, path, &BTreeSet::from([1]), &[]),
            Ok(())
        );
        let waivers = "spec: 001-x\nwaivers:\n  - rule: a\n  - rule: b";
        let one = "---\nspec: 001-x\nwaivers:\n  - rule: a\n---\n";
        assert_eq!(
            read_back(waivers, one, path, &none, &[]),
            Err("its `waivers:` list would not read as it did less the drops".to_string())
        );
    }

    /// A history the walk cannot read whole drops nothing and says why, while the claimant
    /// lines, which the record's text alone decides, are still rewritten; a walk lists the
    /// record for its ownership alone.
    #[test]
    fn a_shallow_history_drops_nothing_and_says_why() {
        let tmp = project(&full_record());
        let repository = git2::Repository::open(tmp.path()).unwrap();
        let mut revwalk = repository.revwalk().unwrap();
        revwalk.push_head().unwrap();
        let first = revwalk
            .map(|oid| oid.unwrap())
            .find(|oid| {
                repository
                    .find_commit(*oid)
                    .unwrap()
                    .summary()
                    .ok()
                    .flatten()
                    == Some("chore: code no spec claims")
            })
            .unwrap();
        fs::write(tmp.path().join(".git/shallow"), format!("{first}\n")).unwrap();
        let summary = summary(&preview(), tmp.path()).unwrap();
        assert_eq!(summary.ownership, Some(OwnershipState::NotExamined));
        assert!(summary.ownership_reason.is_some());
        assert_eq!((summary.drops_count, summary.rewrites_count), (0, 1));

        write(
            &tmp.path().join(RECORD),
            &full_record().replacen(CLAIMED, "- `src/a.rs`\n", 1),
        );
        let walk = walked(&all(), tmp.path()).unwrap();
        assert_eq!(listed(&walk), [(FEATURE.to_string(), false)]);
    }

    /// An applying walk writes each record whose edits its digest still names, and skips
    /// one another writer changed before its write, whose change stands.
    #[test]
    fn an_applying_walk_writes_each_confirmed_record_and_skips_one_changed_before_its_write() {
        let tmp = project(&full_record());
        let second = full_record().replacen("spec: 001-x", "spec: 002-y", 1);
        write(&tmp.path().join(OTHER_RECORD), &second);
        let confirmed = digests(&walked(&all(), tmp.path()).unwrap());
        let other = format!("{second}\n<!-- another writer -->\n");
        let walk = walk_between(tmp.path(), "specs", Some(&confirmed), |feature| {
            if feature == "002-y" {
                fs::write(tmp.path().join(OTHER_RECORD), &other).unwrap();
            }
        })
        .unwrap();
        assert_eq!(listed(&walk), [(FEATURE.to_string(), true)]);
        assert_eq!(skips(&walk), [skip("002-y", ChangedSinceRead)]);
        assert_eq!(read(tmp.path(), RECORD), pruned());
        assert_eq!(read(tmp.path(), OTHER_RECORD), other);
    }

    /// An applying walk writes no record whose digest names other edits, skipping it as
    /// changed since read, and none it holds no digest for, which it lists unwritten.
    #[test]
    fn an_applying_walk_leaves_a_record_whose_digest_is_stale_or_absent() {
        let tmp = project(&full_record());
        let second = full_record().replacen("spec: 001-x", "spec: 002-y", 1);
        write(&tmp.path().join(OTHER_RECORD), &second);
        let stale = BTreeMap::from([(FEATURE.to_string(), "0".repeat(64))]);
        let walk = walked(&apply_all(stale), tmp.path()).unwrap();
        assert_eq!(listed(&walk), [("002-y".to_string(), false)]);
        assert_eq!(skips(&walk), [skip(FEATURE, ChangedSinceRead)]);
        assert!(walk.skipped.unwrap()[0].detail.is_some());
        assert_eq!(read(tmp.path(), RECORD), full_record());
        assert_eq!(read(tmp.path(), OTHER_RECORD), second);
    }

    /// A digest keyed by a name that is no feature directory refuses the applying walk
    /// before any record is written.
    #[test]
    fn an_applying_walk_refuses_a_digest_naming_no_feature_before_writing() {
        let tmp = project(&full_record());
        let mut confirmed = digests(&walked(&all(), tmp.path()).unwrap());
        confirmed.insert("009-gone".to_string(), "0".repeat(64));
        let error = walked(&apply_all(confirmed), tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "digests"),
            "{error}"
        );
        assert_eq!(read(tmp.path(), RECORD), full_record());
    }

    /// A walk, previewing or applying, makes one walk of the history for every record.
    #[test]
    fn a_walk_makes_one_ownership_walk_for_every_record() {
        let tmp = project(&full_record());
        let second = full_record().replacen("spec: 001-x", "spec: 002-y", 1);
        write(&tmp.path().join(OTHER_RECORD), &second);
        spec_ownership::WALKS.with(|walks| walks.set(0));
        let walk = walked(&all(), tmp.path()).unwrap();
        assert_eq!(walk.features_count, 2);
        assert_eq!(spec_ownership::WALKS.with(std::cell::Cell::get), 1);
        spec_ownership::WALKS.with(|walks| walks.set(0));
        let applied = walked(&apply_all(digests(&walk)), tmp.path()).unwrap();
        assert_eq!(
            listed(&applied),
            [(FEATURE.to_string(), true), ("002-y".to_string(), true)]
        );
        assert_eq!(spec_ownership::WALKS.with(std::cell::Cell::get), 1);
    }

    /// The project record has no spec, so no walk reads or writes it.
    #[test]
    fn the_project_record_is_never_walked() {
        let tmp = project(&full_record());
        let project_record = full_record().replacen("spec: 001-x\n", "", 1);
        write(&tmp.path().join("specs/review.md"), &project_record);
        let walk = walked(&all(), tmp.path()).unwrap();
        assert_eq!(walk.examined, 2);
        assert_eq!(listed(&walk), [(FEATURE.to_string(), false)]);
        assert_eq!(skips(&walk), [skip("002-y", NoReviewRecord)]);
        walked(&apply_all(digests(&walk)), tmp.path()).unwrap();
        assert_eq!(read(tmp.path(), "specs/review.md"), project_record);
        assert_eq!(read(tmp.path(), RECORD), pruned());
    }

    /// A record behind a link leaving the project is never read: one feature's call is
    /// refused, and a walk skips it under its own reason.
    #[cfg(unix)]
    #[test]
    fn a_record_behind_a_link_leaving_the_project_is_refused_and_skipped_by_a_walk() {
        let tmp = project(&full_record());
        let outside = tempfile::tempdir().unwrap();
        write(&outside.path().join("review.md"), &full_record());
        std::os::unix::fs::symlink(
            outside.path().join("review.md"),
            tmp.path().join(OTHER_RECORD),
        )
        .unwrap();
        let args = PruneReviewArgs {
            feature: Some("002-y".into()),
            ..PruneReviewArgs::default()
        };
        let error = summary(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(error, PrimitiveError::InvalidPath { .. }),
            "{error}"
        );
        let walk = walked(&all(), tmp.path()).unwrap();
        assert_eq!(skips(&walk), [skip("002-y", OutsideProject)]);
        assert_eq!(listed(&walk), [(FEATURE.to_string(), false)]);
    }

    /// A feature with no record is reported missing, never an error.
    #[test]
    fn a_feature_with_no_record_is_reported_missing() {
        let tmp = project(&full_record());
        let args = PruneReviewArgs {
            feature: Some("002-y".into()),
            ..PruneReviewArgs::default()
        };
        let summary = summary(&args, tmp.path()).unwrap();
        assert!(summary.missing);
        assert_eq!((summary.drops_count, summary.digest), (0, None));
    }

    /// Each apply shape takes the digest it writes by, and a digest comes only with an
    /// apply: every other combination is refused before anything is read.
    #[test]
    fn an_apply_takes_the_digest_its_shape_names_and_a_digest_takes_an_apply() {
        let tmp = project(&full_record());
        let some = BTreeMap::from([(FEATURE.to_string(), "d".to_string())]);
        let one = |apply: bool, digest: Option<&str>, digests: &BTreeMap<String, String>| {
            PruneReviewArgs {
                feature: Some(FEATURE.into()),
                apply,
                digest: digest.map(str::to_string),
                digests: digests.clone(),
                ..PruneReviewArgs::default()
            }
        };
        let walk = |apply: bool, digest: Option<&str>, digests: &BTreeMap<String, String>| {
            PruneReviewArgs {
                all: true,
                feature: None,
                ..one(apply, digest, digests)
            }
        };
        let none = BTreeMap::new();
        for (args, argument, missing) in [
            (one(true, None, &none), "digest", true),
            (one(true, Some("  "), &none), "digest", true),
            (walk(true, None, &none), "digests", true),
            (one(true, Some("d"), &some), "digests", false),
            (walk(true, Some("d"), &some), "digest", false),
            (one(false, Some("d"), &none), "digest", false),
            (walk(false, None, &some), "digests", false),
        ] {
            match super::run(&args, tmp.path()) {
                Err(PrimitiveError::MissingArgument {
                    argument: named, ..
                }) if missing => {
                    assert_eq!(named, argument, "{args:?}");
                }
                Err(PrimitiveError::InvalidArgument {
                    argument: named, ..
                }) if !missing => {
                    assert_eq!(named, argument, "{args:?}");
                }
                other => panic!("{args:?}: {other:?}"),
            }
        }
        assert_eq!(read(tmp.path(), RECORD), full_record());
    }

    /// Past the read size each shape's lists move to the file `lists-file` names, their
    /// counts staying inline.
    #[test]
    fn a_result_past_the_read_size_moves_its_lists_to_a_file() {
        let tmp = project(&full_record());
        set_read_size(tmp.path(), "200");
        let result = super::run(&preview(), tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        let file: serde_json::Value =
            serde_json::from_str(&read(tmp.path(), result.lists_file.as_deref().unwrap())).unwrap();
        let summary = result.summary.unwrap();
        assert_eq!((summary.drops, summary.drops_count), (None, 3));
        assert_eq!(file["drops"].as_array().unwrap().len(), 3);

        let result = super::run(&all(), tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        let file: serde_json::Value =
            serde_json::from_str(&read(tmp.path(), result.lists_file.as_deref().unwrap())).unwrap();
        let walk = result.walk.unwrap();
        assert_eq!((walk.features, walk.features_count), (None, 1));
        assert_eq!(walk.skipped, None);
        assert_eq!(file["features"].as_array().unwrap().len(), 1);
        assert_eq!(file["skipped"].as_array().unwrap().len(), 1);
    }

    /// A malformed read size halts an apply before it writes.
    #[test]
    fn a_malformed_read_size_halts_an_apply_before_any_write() {
        let tmp = project(&full_record());
        let confirmed = digest(tmp.path());
        set_read_size(tmp.path(), "\"big\"");
        let error = summary(&apply(&confirmed), tmp.path()).unwrap_err();
        assert!(
            matches!(error, PrimitiveError::InvalidReadSize { .. }),
            "{error}"
        );
        assert_eq!(read(tmp.path(), RECORD), full_record());
    }

    /// Both result shapes read back as they were written: the halves share no key.
    #[test]
    fn each_result_shape_round_trips() {
        let tmp = project(&full_record());
        for args in [preview(), all()] {
            let result = super::run(&args, tmp.path()).unwrap();
            let back: PruneReviewResult =
                serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
            assert_eq!(back, result, "{args:?}");
        }
    }

    /// A malformed decision or waiver is never judged: it survives on a file that does not
    /// belong, as it survives every review.
    #[test]
    fn a_malformed_entry_survives() {
        let no_outcome = decision(FOREIGN_KEY).replacen("    outcome: waived\n", "", 1);
        let no_reason =
            waiver("SEC-BE-015", &["src/other.rs"]).replacen("    reason: why\n", "", 1);
        let text = format!(
            "{HEAD}decisions:\n{no_outcome}waivers:\n{no_reason}{}",
            TAIL.replacen(CLAIMED, "- `src/a.rs`\n", 1)
        );
        let tmp = project(&text);
        let summary = summary(&preview(), tmp.path()).unwrap();
        assert_eq!(summary.drops.unwrap(), []);
        assert_eq!(summary.rewrites_count, 0);
    }

    /// A waiver's anchors that do not stand one item per line at six spaces, as items
    /// opening with a bare dash leave them, are refused rather than cut at a guessed line.
    #[test]
    fn a_waiver_whose_anchors_will_not_split_is_refused_as_unsplicable() {
        let bare = "  - rule: SEC-BE-014\n    anchors:\n      -\n        file: src/foreign.rs\n        \
                    site: s\n        covers: 1\n      -\n        file: src/own.rs\n        site: s\n        \
                    covers: 1\n    reason: why\n    waived-at: 2026-05-10T14:40:00Z\n    \
                    waived-by: dev@example.com\n";
        let flow = "  - rule: SEC-BE-014\n    anchors: [{file: src/foreign.rs, site: s, covers: 1}, \
                    {file: src/own.rs, site: s, covers: 1}]\n    reason: why\n    \
                    waived-at: 2026-05-10T14:40:00Z\n    waived-by: dev@example.com\n";
        for (waiver, why) in [
            (bare, "list is not 2 items at six spaces"),
            (flow, "is not a list on lines of its own"),
        ] {
            let tmp = project(&record(&[], &[waiver.to_string()]));
            let error = summary(&preview(), tmp.path()).unwrap_err();
            assert!(
                matches!(&error, PrimitiveError::UnsplicableRecord { reason, .. } if reason.contains(why)),
                "{waiver}: {error}"
            );
        }
    }

    /// The digest covers the record as read, so an apply after another writer changed the
    /// record since the preview is refused, though what it drops is the same.
    #[test]
    fn an_apply_is_refused_once_the_record_changed_since_its_preview() {
        let tmp = project(&full_record());
        let confirmed = digest(tmp.path());
        let changed = format!("{}- `src/z.rs` — 002-y\n", full_record());
        write(&tmp.path().join(RECORD), &changed);
        let error = summary(&apply(&confirmed), tmp.path()).unwrap_err();
        assert!(
            matches!(error, PrimitiveError::ChangedSinceRead { .. }),
            "{error}"
        );
        assert_eq!(read(tmp.path(), RECORD), changed);
    }

    /// A project no repository holds cannot say what belongs: one feature's preview and a
    /// walk drop nothing and name the repository as why, and the rewrites still apply.
    #[test]
    fn a_project_with_no_repository_drops_nothing_and_says_why() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\n---\n# 001\n",
        );
        write(&tmp.path().join("src/foreign.rs"), "fn f() {}\n");
        write(&tmp.path().join(RECORD), &full_record());
        let unread = |reason: Option<&str>| {
            reason.is_some_and(|reason| reason.starts_with("the repository could not be read"))
        };
        let summary = summary(&preview(), tmp.path()).unwrap();
        assert_eq!(summary.ownership, Some(OwnershipState::NotExamined));
        assert!(
            unread(summary.ownership_reason.as_deref()),
            "{:?}",
            summary.ownership_reason
        );
        assert_eq!((summary.drops_count, summary.rewrites_count), (0, 1));
        let walk = walked(&all(), tmp.path()).unwrap();
        let entry = &walk.features.unwrap()[0].summary;
        assert!(
            unread(entry.ownership_reason.as_deref()),
            "{:?}",
            entry.ownership_reason
        );
        assert_eq!(entry.drops_count, 0);
    }
}
