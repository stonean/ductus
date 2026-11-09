//! `check-prescriptions` — the diff check of a fix round against the previous re-read's
//! prescribed edits (spec 078). What it compares and returns is
//! `schema::descriptions::CHECK_PRESCRIPTIONS`.
//!
//! The edits come from the re-read's own `result.json`, so no prescription passes through
//! the fixer.
//!
//! Shapes: `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use git2::{ObjectType, Oid};

#[cfg(doc)]
use crate::primitives::PrimitiveError;
use crate::primitives::fix_rounds::{self, RESULT_FILE, ResultFile, RoundRecord};
use crate::primitives::{ProjectRepository, Result};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckPrescriptionsArgs, CheckPrescriptionsResult, FailedEdit, ListsForm, PrescriptionVerdict,
    ReReadFinding, ReReadSeverity,
};

/// Execute the `check-prescriptions` primitive.
///
/// It snapshots the working tree, as `brief-re-read` does, so it writes git
/// objects, and past the read size the list file its result names, with
/// `bounded_lists::place`'s `.ductus/results/.gitignore` and its removal of
/// the feature's stale list files, and nothing else.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] when `feature` is not a feature
/// directory name, [`PrimitiveError::InvalidPath`] when it would leave the
/// spec root or the round directory resolves outside the project,
/// [`PrimitiveError::FeatureNotFound`] when the feature directory is absent,
/// [`PrimitiveError::Git`] when `base` names no commit or a libgit2 call
/// fails, [`PrimitiveError::Io`] when a round file exists but cannot be read
/// or a list file cannot be written, and [`PrimitiveError::InvalidReadSize`]
/// for a malformed `[artifacts] read-size-bytes`. A missing, incomplete or
/// unparseable round file is the `could-not-run` verdict, not an error;
/// neither is ever a match.
pub fn run(args: &CheckPrescriptionsArgs, repo: &Path) -> Result<CheckPrescriptionsResult> {
    let layout = paths::Paths::load(repo);
    fix_rounds::validate_feature(
        "check-prescriptions",
        repo,
        &layout.specs_root,
        &args.feature,
    )?;
    let project = ProjectRepository::discover(repo)?;
    let base = fix_rounds::resolve_base(&project, &args.base)?;
    let dir = fix_rounds::round_dir_to_read(repo, &args.feature, &base)?;
    let could_not_run = |problem: String| CheckPrescriptionsResult {
        problem: Some(problem),
        ..CheckPrescriptionsResult::default()
    };
    let (record, mut findings) = match load_round(&dir)? {
        Ok(round) => round,
        Err(problem) => return bounded(repo, &args.feature, could_not_run(problem)),
    };
    // A finding outside the round is a NOTE, as `read-re-read` counts it.
    fix_rounds::demote_out_of_round(&mut findings, &record.paths);
    let by_file = match edits_by_file(repo, &findings) {
        Ok(by_file) => by_file,
        Err(problem) => return bounded(repo, &args.feature, could_not_run(problem)),
    };
    let open = findings
        .iter()
        .filter(|f| fix_rounds::is_unprescribed(f))
        .count();

    let (now, _) = fix_rounds::snapshot(&project)?;
    let (failed, differing) = compare(&project, &record.snapshot, &now, &by_file)?;
    let prescribed: BTreeSet<&str> = by_file.keys().copied().collect();
    let subject = fix_rounds::RoundSubject {
        repo,
        specs_root: &layout.specs_root,
        feature: &args.feature,
    };
    let unprescribed: Vec<String> =
        fix_rounds::round_paths(&project, subject, &record.snapshot, &now)?
            .paths
            .into_iter()
            .filter(|path| !prescribed.contains(path.as_str()))
            .collect();

    let verdict = if failed.is_empty() && differing.is_empty() && unprescribed.is_empty() {
        PrescriptionVerdict::Matched
    } else {
        PrescriptionVerdict::Differs
    };
    bounded(
        repo,
        &args.feature,
        CheckPrescriptionsResult {
            verdict,
            problem: None,
            snapshot: (verdict == PrescriptionVerdict::Matched).then_some(now),
            examined: by_file.len(),
            differing: Some(differing),
            unprescribed: Some(unprescribed),
            failed: Some(failed),
            open,
            ..CheckPrescriptionsResult::default()
        },
    )
}

/// Bound `result` to one read (spec 078, scenario
/// `fix-round-results-have-a-bounded-size`): inline while it fits `[artifacts]
/// read-size-bytes`, and past it with `differing`, `unprescribed` and `failed`
/// in the file `lists-file` names, as `compute-review-scope` bounds its lists.
/// The verdict and every count stay inline, so a file-form result never reads
/// as a matched check.
fn bounded(
    repo: &Path,
    feature: &str,
    mut result: CheckPrescriptionsResult,
) -> Result<CheckPrescriptionsResult> {
    let differing = result.differing.take().unwrap_or_default();
    let unprescribed = result.unprescribed.take().unwrap_or_default();
    let failed = result.failed.take().unwrap_or_default();
    result.differing_count = differing.len();
    result.unprescribed_count = unprescribed.len();
    result.failed_count = failed.len();
    let lists = serde_json::json!({
        "differing": differing,
        "unprescribed": unprescribed,
        "failed": failed,
    });
    result.differing = Some(differing);
    result.unprescribed = Some(unprescribed);
    result.failed = Some(failed);
    if let Some(file) =
        super::bounded_lists::place(repo, "check-prescriptions", feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.differing = None;
        result.unprescribed = None;
        result.failed = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// The previous round's record and its re-read's findings, or the reason the
/// check cannot run: no `round.toml`, one that does not parse, or a result
/// file that is missing, incomplete or unparseable.
///
/// # Errors
///
/// [`PrimitiveError::Io`] when a round file exists but cannot be read.
fn load_round(
    dir: &Path,
) -> Result<std::result::Result<(RoundRecord, Vec<ReReadFinding>), String>> {
    let record = match fix_rounds::read_round_record(dir)? {
        Ok(record) => record,
        Err(problem) => return Ok(Err(problem)),
    };
    Ok(match fix_rounds::read_result(&dir.join(RESULT_FILE))? {
        ResultFile::Complete(findings) => Ok((record, findings)),
        ResultFile::Incomplete(problem) | ResultFile::Unparseable(problem) => {
            Err(format!("the previous re-read's result: {problem}"))
        }
    })
}

/// One prescribed edit: its finding's index, its own, whether its finding is
/// a NOTE, and its `old` and `new`.
type Edit<'a> = (usize, usize, bool, &'a str, &'a str);

/// Every edit, grouped by file in the order the result file lists them, or
/// the reason the check cannot run: an edit naming a path outside the
/// project, or no edit at all, over which a match would pass vacuously.
fn edits_by_file<'a>(
    repo: &Path,
    findings: &'a [ReReadFinding],
) -> std::result::Result<BTreeMap<&'a str, Vec<Edit<'a>>>, String> {
    let mut by_file: BTreeMap<&str, Vec<Edit<'_>>> = BTreeMap::new();
    for (f, finding) in findings.iter().enumerate() {
        for (e, edit) in finding.edits.iter().enumerate() {
            if super::validate_contained(repo, &edit.path).is_err() {
                return Err(format!(
                    "finding [{f}] edit [{e}] names a path outside the project: {}",
                    super::flatten_line(&edit.path)
                ));
            }
            by_file.entry(edit.path.as_str()).or_default().push((
                f,
                e,
                finding.severity == ReReadSeverity::Note,
                edit.old.as_str(),
                edit.new.as_str(),
            ));
        }
    }
    if by_file.is_empty() {
        return Err("the previous re-read prescribed no edit".to_string());
    }
    Ok(by_file)
}

/// Apply each file's edits to its text in the `read` snapshot and compare the
/// result with the `now` snapshot. Returns the edits that could not apply and
/// the files whose result is not what `now` holds.
///
/// Compared as git stores both — the edited text hashed as a blob against the
/// blob `now` holds, with the mode the `read` snapshot gave the file, so a mode
/// change is a change no edit prescribed (spec 078 AC5) — so a checkout that
/// converts line endings on disk compares alike. An `old` is matched in that
/// stored text, which is what the round's `git diff` shows.
///
/// # Errors
///
/// [`PrimitiveError::Git`] when a snapshot or a blob cannot be read.
fn compare(
    project: &ProjectRepository,
    read: &str,
    now: &str,
    by_file: &BTreeMap<&str, Vec<Edit<'_>>>,
) -> Result<(Vec<FailedEdit>, Vec<String>)> {
    let repository = &project.repository;
    let read_tree = repository.find_commit(Oid::from_str(read)?)?.tree()?;
    let now_tree = repository.find_commit(Oid::from_str(now)?)?.tree()?;
    let mut failed = Vec::new();
    let mut differing = Vec::new();
    for (&path, edits) in by_file {
        let git_path = project.to_git(path);
        let text = super::blob_text(repository, &read_tree, &git_path)?;
        let mode = read_tree
            .get_path(Path::new(&git_path))
            .ok()
            .map(|entry| entry.filemode());
        let actual = match now_tree.get_path(Path::new(&git_path)) {
            Ok(entry) => Some((entry.id(), entry.filemode())),
            Err(err) if err.code() == git2::ErrorCode::NotFound => None,
            Err(err) => return Err(err.into()),
        };
        // A NOTE's edit the operator declines leaves text the re-read already
        // judged acceptable, so the file matches with every edit applied or
        // with only its MUST and SHOULD findings' edits applied (spec 078,
        // task 13); either way every edit applied must apply cleanly.
        let all = apply_edits(text.as_deref(), edits, path, |_| true);
        let blocking = apply_edits(text.as_deref(), edits, path, |&(_, _, note, _, _)| !note);
        let matches = |(expected, failed): &(Option<String>, Vec<FailedEdit>)| -> Result<bool> {
            let expected = expected
                .as_ref()
                .map(|t| Oid::hash_object(ObjectType::Blob, t.as_bytes()))
                .transpose()?
                .zip(mode);
            Ok(failed.is_empty() && expected.is_some() && expected == actual)
        };
        if !matches(&all)? && !matches(&blocking)? {
            failed.extend(all.1);
            differing.push(path.to_string());
        }
    }
    Ok((failed, differing))
}

/// `text` with the `edits` that `keep` accepts applied in order, each `old`
/// replaced only where it occurs at exactly one position, and the edits whose
/// `old` did not. A file the snapshot does not hold as text applies none.
fn apply_edits(
    text: Option<&str>,
    edits: &[Edit<'_>],
    path: &str,
    keep: impl Fn(&Edit<'_>) -> bool,
) -> (Option<String>, Vec<FailedEdit>) {
    let mut text = text.map(str::to_string);
    let mut failed = Vec::new();
    for edit in edits.iter().filter(|edit| keep(edit)) {
        let &(finding, edit_index, _, old, new) = edit;
        let found = text.as_deref().map_or(0, |t| occurrences(t, old));
        if found == 1 {
            text = text.map(|t| t.replacen(old, new, 1));
        } else {
            failed.push(FailedEdit {
                finding,
                edit: edit_index,
                path: path.to_string(),
                occurrences: found,
            });
        }
    }
    (text, failed)
}

/// Every position `needle` starts at in `text`, overlapping ones included: an
/// edit applies only where its text is unambiguous.
fn occurrences(text: &str, needle: &str) -> usize {
    text.char_indices()
        .filter(|&(i, _)| text[i..].starts_with(needle))
        .count()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use git2::Repository;

    use super::*;
    use crate::primitives::PrimitiveError;
    use crate::primitives::git_fixture::{commit_all, write};
    use crate::primitives::{brief_re_read, start_fix_rounds};
    use crate::schema::primitives::{BriefReReadArgs, StartFixRoundsArgs};

    const FEATURE: &str = "001-x";

    /// A round briefed after an edit to `a.txt`, the re-read's result
    /// prescribing `edits`, and the path to `a.txt`. Returns the tempdir and
    /// the briefed round's base.
    fn briefed(edits: &str) -> (tempfile::TempDir, String) {
        let tmp = tempfile::tempdir().unwrap();
        let repository = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n",
        );
        write(&tmp.path().join("a.txt"), "hello world\n");
        write(&tmp.path().join("b.txt"), "b\n");
        commit_all(&repository, "init");
        let base = start_fix_rounds::run(
            &StartFixRoundsArgs {
                feature: FEATURE.to_string(),
            },
            tmp.path(),
        )
        .unwrap()
        .base;
        write(&tmp.path().join("a.txt"), "hello world, again\n");
        let brief = brief_re_read::run(
            &BriefReReadArgs {
                feature: FEATURE.to_string(),
                base: base.clone(),
            },
            tmp.path(),
        )
        .unwrap();
        write(
            &tmp.path().join(brief.result.unwrap()),
            &format!(
                r#"{{"complete": true, "findings": [{{"severity": "SHOULD", "path": "a.txt", "text": "x", "edits": [{edits}]}}]}}"#
            ),
        );
        (tmp, base)
    }

    fn check(tmp: &tempfile::TempDir, base: &str) -> CheckPrescriptionsResult {
        run(
            &CheckPrescriptionsArgs {
                feature: FEATURE.to_string(),
                base: base.to_string(),
            },
            tmp.path(),
        )
        .unwrap()
    }

    const AGAIN: &str = r#"{"path": "a.txt", "old": ", again", "new": ", once more"}"#;

    /// A round that applies exactly the prescribed edit matches, and returns
    /// the snapshot it compared, the next round's base.
    #[test]
    fn exactly_the_prescribed_edit_matches() {
        let (tmp, base) = briefed(AGAIN);
        write(&tmp.path().join("a.txt"), "hello world, once more\n");
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Matched, "{result:?}");
        assert_eq!((result.examined, result.open), (1, 0));
        assert!(result.snapshot.is_some());
    }

    /// A mode change beside exactly the prescribed edit is a change no edit
    /// prescribed, so the file differs and the round is re-read (spec 078 AC5).
    #[cfg(unix)]
    #[test]
    fn a_mode_change_beside_the_prescribed_edit_differs() {
        use std::os::unix::fs::PermissionsExt;
        let (tmp, base) = briefed(AGAIN);
        let file = tmp.path().join("a.txt");
        write(&file, "hello world, once more\n");
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o755)).unwrap();
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs, "{result:?}");
        assert_eq!(result.differing.as_deref().unwrap(), ["a.txt"]);
    }

    /// In a project below its repository's root, a prescription's path is
    /// from the project root and is found in git's tree, so exactly the
    /// prescribed edit still matches.
    #[test]
    fn a_subdirectory_project_matches_exactly_the_prescribed_edit() {
        let tmp = tempfile::tempdir().unwrap();
        let (repository, project) =
            crate::primitives::git_fixture::subdirectory_project(tmp.path());
        write(
            &project.join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n",
        );
        write(&project.join("a.txt"), "hello world\n");
        commit_all(&repository, "init");
        let base = start_fix_rounds::run(
            &StartFixRoundsArgs {
                feature: FEATURE.to_string(),
            },
            &project,
        )
        .unwrap()
        .base;
        write(&project.join("a.txt"), "hello world, again\n");
        let brief = brief_re_read::run(
            &BriefReReadArgs {
                feature: FEATURE.to_string(),
                base: base.clone(),
            },
            &project,
        )
        .unwrap();
        write(
            &project.join(brief.result.unwrap()),
            &format!(
                r#"{{"complete": true, "findings": [{{"severity": "SHOULD", "path": "a.txt", "text": "x", "edits": [{AGAIN}]}}]}}"#
            ),
        );
        write(&project.join("a.txt"), "hello world, once more\n");
        let result = run(
            &CheckPrescriptionsArgs {
                feature: FEATURE.to_string(),
                base,
            },
            &project,
        )
        .unwrap();
        assert_eq!(result.verdict, PrescriptionVerdict::Matched, "{result:?}");
        assert_eq!(result.examined, 1);
    }

    /// The feature's own `review.md`, rewritten after the re-read as every
    /// run rewrites it, is no unprescribed change, so exactly the prescribed
    /// edit still matches (scenario `a-reviews-own-record-is-no-fix-round`).
    #[test]
    fn the_reviews_own_record_is_not_unprescribed() {
        let (tmp, base) = briefed(AGAIN);
        write(&tmp.path().join("a.txt"), "hello world, once more\n");
        write(
            &tmp.path().join("specs/001-x/review.md"),
            "---\nspec: 001-x\n---\n",
        );
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Matched, "{result:?}");
        assert_eq!(result.unprescribed_count, 0);
    }

    /// A finding outside the round is a NOTE here too, as `read-re-read` counts
    /// it, so a SHOULD with no edit on a path the round did not change leaves
    /// nothing open (AC4).
    #[test]
    fn a_finding_outside_the_round_leaves_nothing_open() {
        let (tmp, base) = briefed(AGAIN);
        write(
            &tmp.path()
                .join(fix_rounds::round_rel(FEATURE, &base))
                .join(RESULT_FILE),
            &format!(
                r#"{{"complete": true, "findings": [{{"severity": "SHOULD", "path": "a.txt", "text": "x", "edits": [{AGAIN}]}}, {{"severity": "SHOULD", "path": "b.txt", "text": "outside"}}]}}"#
            ),
        );
        write(&tmp.path().join("a.txt"), "hello world, once more\n");
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Matched, "{result:?}");
        assert_eq!(result.open, 0);
    }

    /// A NOTE's edit the operator declines still matches: the file holds its
    /// text with only the SHOULD's edit applied (task 13). With both applied
    /// it matches too.
    #[test]
    fn a_declined_note_edit_still_matches() {
        for (disk, label) in [
            ("hello world, once more\n", "note declined"),
            ("hi world, once more\n", "both applied"),
        ] {
            let (tmp, base) = briefed(AGAIN);
            write(
                &tmp.path()
                    .join(fix_rounds::round_rel(FEATURE, &base))
                    .join(RESULT_FILE),
                &format!(
                    r#"{{"complete": true, "findings": [{{"severity": "SHOULD", "path": "a.txt", "text": "x", "edits": [{AGAIN}]}}, {{"severity": "NOTE", "path": "a.txt", "text": "y", "edits": [{{"path": "a.txt", "old": "hello", "new": "hi"}}]}}]}}"#
                ),
            );
            write(&tmp.path().join("a.txt"), disk);
            let result = check(&tmp, &base);
            assert_eq!(
                result.verdict,
                PrescriptionVerdict::Matched,
                "{label}: {result:?}"
            );
        }
    }

    /// A SHOULD's edit left unapplied still differs, whatever the NOTE's.
    #[test]
    fn a_declined_should_edit_still_differs() {
        let (tmp, base) = briefed(AGAIN);
        write(
            &tmp.path()
                .join(fix_rounds::round_rel(FEATURE, &base))
                .join(RESULT_FILE),
            &format!(
                r#"{{"complete": true, "findings": [{{"severity": "SHOULD", "path": "a.txt", "text": "x", "edits": [{AGAIN}]}}, {{"severity": "NOTE", "path": "a.txt", "text": "y", "edits": [{{"path": "a.txt", "old": "hello", "new": "hi"}}]}}]}}"#
            ),
        );
        write(&tmp.path().join("a.txt"), "hi world, again\n");
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs);
        assert_eq!(result.differing.as_deref().unwrap(), ["a.txt"]);
    }

    /// Two edits to one file apply in order, each to the text the one before
    /// left.
    #[test]
    fn edits_to_one_file_apply_in_order() {
        let (tmp, base) = briefed(&format!(
            r#"{AGAIN}, {{"path": "a.txt", "old": "once more", "new": "twice"}}"#
        ));
        write(&tmp.path().join("a.txt"), "hello world, twice\n");
        assert_eq!(check(&tmp, &base).verdict, PrescriptionVerdict::Matched);
    }

    /// An edit whose `old` occurs nowhere, or twice, fails with its count.
    #[test]
    fn an_old_text_that_is_not_unique_fails() {
        let (tmp, base) = briefed(r#"{"path": "a.txt", "old": "absent", "new": "x"}"#);
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs);
        assert_eq!(result.failed.as_deref().unwrap()[0].occurrences, 0);

        let (tmp, base) = briefed(r#"{"path": "a.txt", "old": "l", "new": "L"}"#);
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs);
        assert_eq!(
            (
                result.failed.as_deref().unwrap()[0].finding,
                result.failed.as_deref().unwrap()[0].edit
            ),
            (0, 0)
        );
        assert_eq!(result.failed.as_deref().unwrap()[0].occurrences, 3);
    }

    /// Overlapping occurrences count, so `aa` in `aaa` is ambiguous.
    #[test]
    fn overlapping_occurrences_count() {
        assert_eq!(occurrences("aaa", "aa"), 2);
        assert_eq!(occurrences("a world", "world"), 1);
        assert_eq!(occurrences("é world", "world"), 1);
    }

    /// A file the round changed that no edit names is unprescribed.
    #[test]
    fn an_unprescribed_change_differs() {
        let (tmp, base) = briefed(AGAIN);
        write(&tmp.path().join("a.txt"), "hello world, once more\n");
        write(&tmp.path().join("b.txt"), "b, changed\n");
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs);
        assert_eq!(result.unprescribed.as_deref().unwrap(), ["b.txt"]);
        assert_eq!(result.differing.as_deref().unwrap(), [] as [String; 0]);
        // Not the next round's base: the round is re-read from its own.
        assert_eq!(result.snapshot, None);
    }

    /// A prescribed file edited past its prescription differs, a formatter's
    /// whitespace change included.
    #[test]
    fn a_prescribed_file_edited_further_differs() {
        for disk in ["hello world, once more!\n", "hello world,  once more\n"] {
            let (tmp, base) = briefed(AGAIN);
            write(&tmp.path().join("a.txt"), disk);
            let result = check(&tmp, &base);
            assert_eq!(result.verdict, PrescriptionVerdict::Differs, "{disk:?}");
            assert_eq!(result.differing.as_deref().unwrap(), ["a.txt"]);
        }
    }

    /// An unapplied prescription differs: the file still holds the old text.
    #[test]
    fn an_unapplied_prescription_differs() {
        let (tmp, base) = briefed(AGAIN);
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs);
        assert_eq!(result.differing.as_deref().unwrap(), ["a.txt"]);
    }

    /// A MUST or SHOULD the re-read gave no edit, on a path the round changed,
    /// stays open after a match.
    #[test]
    fn an_unprescribed_finding_stays_open() {
        let (tmp, base) = briefed(AGAIN);
        let result_path = tmp
            .path()
            .join(fix_rounds::round_rel(FEATURE, &base))
            .join(RESULT_FILE);
        let text = std::fs::read_to_string(&result_path).unwrap();
        write(
            &result_path,
            &text.replace(
                "\"findings\": [",
                "\"findings\": [{\"severity\": \"MUST\", \"path\": \"a.txt\", \"text\": \"y\"}, {\"severity\": \"NOTE\", \"path\": \"b.txt\", \"text\": \"z\"}, ",
            ),
        );
        write(&tmp.path().join("a.txt"), "hello world, once more\n");
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Matched);
        assert_eq!(result.open, 1);
    }

    /// No briefed round, a missing, incomplete or unparseable result, a
    /// result with no edit, and an edit outside the project each make the
    /// check unable to run, never a match.
    #[test]
    fn each_could_not_run_cause() {
        let (tmp, base) = briefed(AGAIN);
        let head = Repository::discover(tmp.path())
            .unwrap()
            .head()
            .unwrap()
            .target()
            .unwrap()
            .to_string();
        let result = check(&tmp, &head);
        assert_eq!(result.verdict, PrescriptionVerdict::CouldNotRun);
        assert!(result.problem.unwrap().contains("no round was briefed"));

        let result_path = tmp
            .path()
            .join(fix_rounds::round_rel(FEATURE, &base))
            .join(RESULT_FILE);
        for (body, expect) in [
            (None, "does not exist"),
            (Some(r#"{"complete": false, "findings": []}"#), "not true"),
            (Some(r#"{"complete": true, "findings": [ "#), "result"),
            (
                Some(r#"{"complete": true, "findings": []}"#),
                "prescribed no edit",
            ),
            (
                Some(
                    r#"{"complete": true, "findings": [{"severity": "SHOULD", "path": "a.txt", "text": "x", "edits": [{"path": "../out.txt", "old": "a", "new": "b"}]}]}"#,
                ),
                "outside the project",
            ),
        ] {
            match body {
                Some(body) => write(&result_path, body),
                None => std::fs::remove_file(&result_path).unwrap(),
            }
            let result = check(&tmp, &base);
            assert_eq!(result.verdict, PrescriptionVerdict::CouldNotRun, "{body:?}");
            assert!(
                result.problem.as_deref().unwrap().contains(expect),
                "{result:?}"
            );
            assert_eq!(result.snapshot, None);
        }
    }

    /// Past the read size the three lists are in the file `lists-file`
    /// names, with the verdict and every count inline and no list, so a
    /// counts-only result never reads as a matched check (scenario
    /// `fix-round-results-have-a-bounded-size`). Each list holds an entry, so
    /// a count left at zero fails it.
    #[test]
    fn lists_past_the_read_size_are_in_a_file() {
        let (tmp, base) = briefed(r#"{"path": "a.txt", "old": "absent", "new": "x"}"#);
        write(&tmp.path().join("b.txt"), "b, changed\n");
        write(
            &tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 1\n",
        );
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs);
        assert_eq!(
            (
                result.form,
                result.differing_count,
                result.unprescribed_count,
                result.failed_count
            ),
            (ListsForm::File, 1, 2, 1)
        );
        assert_eq!(
            (&result.differing, &result.unprescribed, &result.failed),
            (&None, &None, &None)
        );
        let file = result
            .lists_file
            .expect("a file-form result names its file");
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(file)).unwrap()).unwrap();
        assert_eq!(
            lists["unprescribed"],
            serde_json::json!([".ductus/config.toml", "b.txt"])
        );
        assert_eq!(lists["differing"], serde_json::json!(["a.txt"]));
        assert_eq!(lists["failed"][0]["occurrences"], 0);
    }

    /// `feature` names the round directory read, so a name outside the
    /// feature-name allowlist is refused (`BE-INPUT-002`).
    #[test]
    fn a_name_outside_the_allowlist_is_refused() {
        let (tmp, base) = briefed(AGAIN);
        let err = run(
            &CheckPrescriptionsArgs {
                feature: ".".to_string(),
                base,
            },
            tmp.path(),
        )
        .unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidArgument { .. }),
            "{err:?}"
        );
    }
}
