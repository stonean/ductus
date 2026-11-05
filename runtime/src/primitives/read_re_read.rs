//! `read-re-read` — read a fix round's re-read result (spec 078).
//!
//! Whether a re-read found a MUST or SHOULD is read from the file the re-read
//! wrote, not from the fixer's reading of its report, and a re-read that was
//! cut off — no file, or one that never said `"complete": true` — reads as
//! not finished, never as clean (`QUAL-CLAIM-001`).
//!
//! Shapes: `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`.

use std::path::Path;

#[cfg(doc)]
use crate::primitives::PrimitiveError;
use crate::primitives::fix_rounds::{self, RESULT_FILE, ResultFile};
use crate::primitives::{ProjectRepository, Result};
use crate::schema::paths;
use crate::schema::primitives::{
    ListsForm, ReReadSeverity, ReReadState, ReadReReadArgs, ReadReReadResult,
};

/// Execute the `read-re-read` primitive. Read-only, but for the list file a
/// result past the read size names, with `bounded_lists::place`'s
/// `.ductus/results/.gitignore` and its removal of the feature's stale list
/// files.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] when `feature` is not a feature
/// directory name, [`PrimitiveError::InvalidPath`] when it would leave the
/// spec root or the round directory resolves outside the project,
/// [`PrimitiveError::FeatureNotFound`] when the feature directory is absent,
/// [`PrimitiveError::Git`] when `base` names no commit, and
/// [`PrimitiveError::Io`] when the result file exists but cannot be read or
/// a list file cannot be written, and [`PrimitiveError::InvalidReadSize`] for
/// a malformed `[artifacts] read-size-bytes`. An error is never a clean
/// re-read: the host counts the round as not re-read.
pub fn run(args: &ReadReReadArgs, repo: &Path) -> Result<ReadReReadResult> {
    let layout = paths::Paths::load(repo);
    fix_rounds::validate_feature("read-re-read", repo, &layout.specs_root, &args.feature)?;
    let project = ProjectRepository::discover(repo)?;
    let base = fix_rounds::resolve_base(&project, &args.base)?;
    let dir = fix_rounds::round_dir_to_read(repo, &args.feature, &base)?;
    let result = match fix_rounds::read_result(&dir.join(RESULT_FILE))? {
        ResultFile::Incomplete(problem) => ReadReReadResult {
            state: ReReadState::Incomplete,
            problem: Some(problem),
            ..ReadReReadResult::default()
        },
        ResultFile::Unparseable(problem) => ReadReReadResult {
            state: ReReadState::Unparseable,
            problem: Some(problem),
            ..ReadReReadResult::default()
        },
        ResultFile::Complete(mut findings) => {
            // A complete result is scoped by the round it answers: with no
            // readable `round.toml` there is no round to scope it by, so it is
            // no re-read, never a clean one.
            let record = match fix_rounds::read_round_record(&dir)? {
                Ok(record) => record,
                Err(problem) => {
                    return bounded(
                        repo,
                        &args.feature,
                        ReadReReadResult {
                            state: ReReadState::Unparseable,
                            problem: Some(problem),
                            ..ReadReReadResult::default()
                        },
                    );
                }
            };
            let demoted = fix_rounds::demote_out_of_round(&mut findings, &record.paths);
            let count = |severity| findings.iter().filter(|f| f.severity == severity).count();
            ReadReReadResult {
                state: ReReadState::Complete,
                problem: None,
                must: count(ReReadSeverity::Must),
                should: count(ReReadSeverity::Should),
                notes: count(ReReadSeverity::Note),
                unprescribed: findings
                    .iter()
                    .filter(|f| fix_rounds::is_unprescribed(f))
                    .count(),
                demoted,
                findings: Some(findings),
                ..ReadReReadResult::default()
            }
        }
    };
    bounded(repo, &args.feature, result)
}

/// Bound `result` to one read (spec 078, scenario
/// `fix-round-results-have-a-bounded-size`): inline while it fits `[artifacts]
/// read-size-bytes`, and past it with `findings` in the file `lists-file`
/// names, as `compute-review-scope` bounds its lists. The state and every
/// count stay inline, so a file-form result never reads as a clean re-read.
fn bounded(repo: &Path, feature: &str, mut result: ReadReReadResult) -> Result<ReadReReadResult> {
    let findings = result.findings.take().unwrap_or_default();
    result.findings_count = findings.len();
    let lists = serde_json::json!({ "findings": &findings });
    result.findings = Some(findings);
    if let Some(file) = super::bounded_lists::place(repo, "read-re-read", feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.findings = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use git2::Repository;

    use super::*;
    use crate::primitives::PrimitiveError;
    use crate::primitives::git_fixture::{commit_all, write};

    const FEATURE: &str = "001-x";

    /// A repository with a feature, and the round directory a brief at
    /// `HEAD` would use. Returns the tempdir, the base sha and the result
    /// file's path.
    fn fixture() -> (tempfile::TempDir, String, std::path::PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let repository = Repository::init(tmp.path()).unwrap();
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n",
        );
        let base = commit_all(&repository, "init").to_string();
        let round = tmp.path().join(fix_rounds::round_rel(FEATURE, &base));
        write(
            &round.join(fix_rounds::ROUND_FILE),
            &toml::to_string(&fix_rounds::RoundRecord {
                snapshot: base.clone(),
                paths: vec!["src/a.rs".to_string()],
            })
            .unwrap(),
        );
        (tmp, base, round.join(RESULT_FILE))
    }

    fn read(tmp: &tempfile::TempDir, base: &str) -> ReadReReadResult {
        run(
            &ReadReReadArgs {
                feature: FEATURE.to_string(),
                base: base.to_string(),
            },
            tmp.path(),
        )
        .unwrap()
    }

    fn finding(severity: &str, edits: &str) -> String {
        format!(r#"{{"severity": "{severity}", "path": "src/a.rs", "text": "x"{edits}}}"#)
    }

    /// No file is a re-read that did not finish, never a clean one.
    #[test]
    fn no_file_is_incomplete() {
        let (tmp, base, _) = fixture();
        let result = read(&tmp, &base);
        assert_eq!(result.state, ReReadState::Incomplete);
        assert!(result.problem.unwrap().contains("does not exist"));
    }

    #[test]
    fn complete_missing_or_false_is_incomplete() {
        for body in [
            r#"{"findings": []}"#,
            r#"{"complete": false, "findings": []}"#,
        ] {
            let (tmp, base, path) = fixture();
            write(&path, body);
            assert_eq!(read(&tmp, &base).state, ReReadState::Incomplete, "{body}");
        }
    }

    /// A file cut off mid-write, a misspelled key, an unknown severity, a
    /// multi-line text and an empty `old` are each unparseable.
    #[test]
    fn a_malformed_file_is_unparseable() {
        for body in [
            r#"{"complete": true, "findings": [{"severity": "MUST", "pa"#.to_string(),
            r#"{"complete": true, "findings": [], "extra": 1}"#.to_string(),
            format!(
                r#"{{"complete": true, "findings": [{}]}}"#,
                finding("SHOULD", r#", "edit": []"#)
            ),
            format!(r#"{{"complete": true, "findings": [{}]}}"#, finding("BLOCKER", "")),
            r#"{"complete": true, "findings": [{"severity": "NOTE", "path": "a", "text": "two\nlines"}]}"#.to_string(),
            format!(
                r#"{{"complete": true, "findings": [{}]}}"#,
                finding("SHOULD", r#", "edits": [{"path": "src/a.rs", "old": "", "new": "x"}]"#)
            ),
            r#"{"complete": "yes", "findings": []}"#.to_string(),
            r#"{"complete": true, "findings": [{"severity": "NOTE", "path": "", "text": "x"}]}"#.to_string(),
            format!(
                r#"{{"complete": true, "findings": [{}]}}"#,
                finding("SHOULD", r#", "edits": [{"path": "", "old": "a", "new": "b"}]"#)
            ),
        ] {
            let (tmp, base, path) = fixture();
            write(&path, &body);
            let result = read(&tmp, &base);
            assert_eq!(result.state, ReReadState::Unparseable, "{body}");
            assert!(result.problem.is_some());
            assert_eq!(result.findings.as_deref().unwrap(), []);
        }
    }

    /// A complete file is counted by severity, and its MUST and SHOULD
    /// findings without edits are counted apart.
    #[test]
    fn a_complete_file_is_counted() {
        let (tmp, base, path) = fixture();
        let edit = r#", "edits": [{"path": "src/a.rs", "old": "a", "new": "b"}]"#;
        write(
            &path,
            &format!(
                r#"{{"complete": true, "findings": [{}, {}, {}, {}, {}]}}"#,
                finding("MUST", ""),
                finding("SHOULD", edit),
                finding("SHOULD", ""),
                finding("NOTE", ""),
                finding("NOTE", edit),
            ),
        );
        let result = read(&tmp, &base);
        assert_eq!(result.state, ReReadState::Complete);
        assert_eq!(
            (
                result.must,
                result.should,
                result.notes,
                result.unprescribed
            ),
            (1, 2, 2, 2)
        );
        let findings = result.findings.as_deref().unwrap();
        assert_eq!((findings.len(), result.findings_count), (5, 5));
        assert_eq!(findings[1].edits[0].new, "b");
    }

    #[test]
    fn an_empty_complete_file_is_a_clean_re_read() {
        let (tmp, base, path) = fixture();
        write(&path, r#"{"complete": true, "findings": []}"#);
        let result = read(&tmp, &base);
        assert_eq!(result.state, ReReadState::Complete);
        assert_eq!((result.must, result.should, result.notes), (0, 0, 0));
    }

    /// Past the read size a re-read's findings are in the file `lists-file`
    /// names, with the state and every count inline and no list, so a
    /// counts-only result never reads as a clean re-read (scenario
    /// `fix-round-results-have-a-bounded-size`).
    #[test]
    fn findings_past_the_read_size_are_in_a_file() {
        let (tmp, base, path) = fixture();
        write(
            &path,
            &format!(
                r#"{{"complete": true, "findings": [{}]}}"#,
                finding("SHOULD", "")
            ),
        );
        write(
            &tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 1\n",
        );
        let result = read(&tmp, &base);
        assert_eq!(result.state, ReReadState::Complete);
        assert_eq!(
            (result.form, result.findings_count, result.should),
            (ListsForm::File, 1, 1)
        );
        assert_eq!(result.findings, None);
        let file = result
            .lists_file
            .expect("a file-form result names its file");
        let lists: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(file)).unwrap()).unwrap();
        assert_eq!(lists["findings"][0]["severity"], "SHOULD");
    }

    /// A finding the re-read labelled MUST or SHOULD on a path the round did
    /// not change counts as a NOTE, by the round's own `round.toml` (AC4).
    #[test]
    fn a_finding_outside_the_round_counts_as_a_note() {
        let (tmp, base, path) = fixture();
        write(
            &path,
            &format!(
                r#"{{"complete": true, "findings": [{}, {}]}}"#,
                finding("SHOULD", ""),
                r#"{"severity": "MUST", "path": "src/elsewhere.rs", "text": "x"}"#
            ),
        );
        let result = read(&tmp, &base);
        assert_eq!(result.state, ReReadState::Complete);
        assert_eq!(
            (result.must, result.should, result.notes, result.demoted),
            (0, 1, 1, 1)
        );
        assert_eq!(result.unprescribed, 1);
        assert_eq!(
            result.findings.as_deref().unwrap()[1].severity,
            ReReadSeverity::Note
        );
    }

    /// A complete result with no readable `round.toml` has no round to be
    /// scoped by, so it is no re-read, never a clean one.
    #[test]
    fn a_result_without_its_round_is_unparseable() {
        let (tmp, base, path) = fixture();
        write(&path, r#"{"complete": true, "findings": []}"#);
        std::fs::remove_file(path.with_file_name(fix_rounds::ROUND_FILE)).unwrap();
        let result = read(&tmp, &base);
        assert_eq!(result.state, ReReadState::Unparseable);
        assert!(result.problem.unwrap().contains("round.toml"));
    }

    /// `feature` names the round directory read, so a name outside the
    /// feature-name allowlist is refused (`BE-INPUT-002`).
    #[test]
    fn a_name_outside_the_allowlist_is_refused() {
        let (tmp, base, _) = fixture();
        let err = run(
            &ReadReReadArgs {
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
