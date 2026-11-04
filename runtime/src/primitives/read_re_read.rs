//! `read-re-read` — read a fix round's re-read result (spec 078).
//!
//! Whether a re-read found a MUST or SHOULD is read from the file the re-read
//! wrote, not from the fixer's reading of its report, and a re-read that was
//! cut off — no file, or one that never said `"complete": true` — reads as
//! not finished, never as clean (`QUAL-CLAIM-001`).
//!
//! Defined by `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`.

use std::path::Path;

use crate::primitives::fix_rounds::{self, RESULT_FILE, ResultFile};
use crate::primitives::{PrimitiveError, ProjectRepository, Result};
use crate::schema::paths;
use crate::schema::primitives::{ReReadSeverity, ReReadState, ReadReReadArgs, ReadReReadResult};

/// Execute the `read-re-read` primitive. Read-only.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] when `feature` would leave the spec root or
/// the round directory resolves outside the project,
/// [`PrimitiveError::FeatureNotFound`] when the feature directory is absent,
/// [`PrimitiveError::Git`] when `base` names no commit, and
/// [`PrimitiveError::Io`] when the result file exists but cannot be read. An
/// error is never a clean re-read: the host counts the round as not re-read.
pub fn run(args: &ReadReReadArgs, repo: &Path) -> Result<ReadReReadResult> {
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
    Ok(match fix_rounds::read_result(&dir.join(RESULT_FILE))? {
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
        ResultFile::Complete(findings) => {
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
                findings,
            }
        }
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use git2::Repository;

    use super::*;
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
        let result = tmp
            .path()
            .join(fix_rounds::round_rel(FEATURE, &base))
            .join(RESULT_FILE);
        (tmp, base, result)
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
            assert_eq!(result.findings, []);
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
        assert_eq!(result.findings.len(), 5);
        assert_eq!(result.findings[1].edits[0].new, "b");
    }

    #[test]
    fn an_empty_complete_file_is_a_clean_re_read() {
        let (tmp, base, path) = fixture();
        write(&path, r#"{"complete": true, "findings": []}"#);
        let result = read(&tmp, &base);
        assert_eq!(result.state, ReReadState::Complete);
        assert_eq!((result.must, result.should, result.notes), (0, 0, 0));
    }
}
