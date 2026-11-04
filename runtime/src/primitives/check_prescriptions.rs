//! `check-prescriptions` — the diff check of a fix round against the previous
//! re-read's prescribed edits (spec 078).
//!
//! A round that only applies the edits a re-read prescribed is verified by
//! applying those edits to the tree that re-read read and comparing the result
//! with the working tree, instead of by another re-read. The edits come from
//! the re-read's own `result.json`, so no prescription passes through the
//! fixer.
//!
//! Defined by `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use git2::{ObjectType, Oid};

use crate::primitives::fix_rounds::{self, RESULT_FILE, ROUND_FILE, ResultFile, RoundRecord};
use crate::primitives::{PrimitiveError, ProjectRepository, Result};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckPrescriptionsArgs, CheckPrescriptionsResult, FailedEdit, PrescriptionVerdict,
    ReReadFinding,
};

/// Execute the `check-prescriptions` primitive.
///
/// It snapshots the working tree, as `brief-re-read` does, so it writes git
/// objects and nothing else.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] when `feature` would leave the spec root or
/// the round directory resolves outside the project,
/// [`PrimitiveError::FeatureNotFound`] when the feature directory is absent,
/// [`PrimitiveError::Git`] when `base` names no commit or a libgit2 call
/// fails, and [`PrimitiveError::Io`] when a round file exists but cannot be
/// read. A missing, incomplete or unparseable round file is the
/// `could-not-run` verdict, not an error; neither is ever a match.
pub fn run(args: &CheckPrescriptionsArgs, repo: &Path) -> Result<CheckPrescriptionsResult> {
    let layout = paths::Paths::load(repo);
    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;
    if !repo.join(&layout.specs_root).join(&args.feature).is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root.clone(),
            feature: args.feature.clone(),
        });
    }
    let project = ProjectRepository::discover(repo)?;
    let base = fix_rounds::resolve_base(&project, &args.base)?;
    let dir = fix_rounds::round_dir_to_read(repo, &args.feature, &base)?;
    let could_not_run = |problem: String| CheckPrescriptionsResult {
        problem: Some(problem),
        ..CheckPrescriptionsResult::default()
    };
    let (record, findings) = match load_round(&dir)? {
        Ok(round) => round,
        Err(problem) => return Ok(could_not_run(problem)),
    };
    let by_file = match edits_by_file(repo, &findings) {
        Ok(by_file) => by_file,
        Err(problem) => return Ok(could_not_run(problem)),
    };
    let open = findings
        .iter()
        .filter(|f| fix_rounds::is_unprescribed(f))
        .count();

    let (now, _) = fix_rounds::snapshot(&project)?;
    let (failed, differing) = compare(&project, &record.snapshot, &now, &by_file)?;
    let prescribed: BTreeSet<&str> = by_file.keys().copied().collect();
    let unprescribed: Vec<String> = fix_rounds::changed_between(&project, &record.snapshot, &now)?
        .into_iter()
        .filter(|path| !prescribed.contains(path.as_str()))
        .collect();

    let verdict = if failed.is_empty() && differing.is_empty() && unprescribed.is_empty() {
        PrescriptionVerdict::Matched
    } else {
        PrescriptionVerdict::Differs
    };
    Ok(CheckPrescriptionsResult {
        verdict,
        problem: None,
        snapshot: (verdict == PrescriptionVerdict::Matched).then_some(now),
        examined: by_file.len(),
        differing,
        unprescribed,
        failed,
        open,
    })
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
    let record_path = dir.join(ROUND_FILE);
    let record: RoundRecord = match std::fs::read_to_string(&record_path) {
        Ok(text) => match toml::from_str(&text) {
            Ok(record) => record,
            Err(err) => {
                return Ok(Err(format!(
                    "{} does not parse: {}",
                    record_path.display(),
                    super::flatten_line(&err.to_string())
                )));
            }
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Err(format!(
                "{} does not exist: no round was briefed at this base",
                record_path.display()
            )));
        }
        Err(source) => {
            return Err(PrimitiveError::Io {
                path: record_path,
                source,
            });
        }
    };
    Ok(match fix_rounds::read_result(&dir.join(RESULT_FILE))? {
        ResultFile::Complete(findings) => Ok((record, findings)),
        ResultFile::Incomplete(problem) | ResultFile::Unparseable(problem) => {
            Err(format!("the previous re-read's result: {problem}"))
        }
    })
}

/// One prescribed edit: its finding's index, its own, and its `old` and `new`.
type Edit<'a> = (usize, usize, &'a str, &'a str);

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
/// blob `now` holds — so a checkout that converts line endings on disk
/// compares alike.
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
        let mut text = super::blob_text(repository, &read_tree, &git_path)?;
        for &(finding, edit, old, new) in edits {
            let occurrences = text.as_deref().map_or(0, |t| occurrences(t, old));
            if occurrences == 1 {
                text = text.map(|t| t.replacen(old, new, 1));
            } else {
                failed.push(FailedEdit {
                    finding,
                    edit,
                    path: path.to_string(),
                    occurrences,
                });
            }
        }
        let expected = text
            .map(|t| Oid::hash_object(ObjectType::Blob, t.as_bytes()))
            .transpose()?;
        let actual = match now_tree.get_path(Path::new(&git_path)) {
            Ok(entry) => Some(entry.id()),
            Err(err) if err.code() == git2::ErrorCode::NotFound => None,
            Err(err) => return Err(err.into()),
        };
        if expected.is_none() || expected != actual {
            differing.push(path.to_string());
        }
    }
    Ok((failed, differing))
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
        assert_eq!(result.failed[0].occurrences, 0);

        let (tmp, base) = briefed(r#"{"path": "a.txt", "old": "l", "new": "L"}"#);
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs);
        assert_eq!((result.failed[0].finding, result.failed[0].edit), (0, 0));
        assert_eq!(result.failed[0].occurrences, 3);
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
        assert_eq!(result.unprescribed, ["b.txt"]);
        assert_eq!(result.differing, [] as [String; 0]);
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
            assert_eq!(result.differing, ["a.txt"]);
        }
    }

    /// An unapplied prescription differs: the file still holds the old text.
    #[test]
    fn an_unapplied_prescription_differs() {
        let (tmp, base) = briefed(AGAIN);
        let result = check(&tmp, &base);
        assert_eq!(result.verdict, PrescriptionVerdict::Differs);
        assert_eq!(result.differing, ["a.txt"]);
    }

    /// A MUST or SHOULD the re-read gave no edit stays open after a match.
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
                "\"findings\": [{\"severity\": \"MUST\", \"path\": \"b.txt\", \"text\": \"y\"}, {\"severity\": \"NOTE\", \"path\": \"b.txt\", \"text\": \"z\"}, ",
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
}
