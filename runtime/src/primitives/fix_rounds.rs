//! The fix-round loop's shared parts (spec 078): the round directory, the
//! working-tree snapshot every round's base is, and the `[review]
//! re-reads-off` switch.
//!
//! Shapes: `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use git2::{IndexAddOption, Oid, Signature};

use crate::primitives::compute_review_scope::{project_diff_options, project_paths};
use crate::primitives::discover_rule_files::{MIN_REASON_LEN, rendered, toml_type_name};
use crate::primitives::{PrimitiveError, ProjectRepository, Result};
use crate::schema::paths;
use crate::schema::primitives::ReReadFinding;

/// Where the loop's files live, from the project root.
pub(crate) const ROUNDS_DIR: &str = ".ductus/review-rounds";

/// The `[review]` key that switches the fix-round loop off. Its value is the
/// reason, which every round the loop would have re-read is waived with.
pub(crate) const RE_READS_OFF: &str = "re-reads-off";

/// A briefed round's record, `round.toml`: what `check-prescriptions` needs to
/// apply the re-read's edits to the tree the re-read read.
pub(crate) const ROUND_FILE: &str = "round.toml";
/// The brief a briefed round's re-read follows.
pub(crate) const BRIEF_FILE: &str = "brief.md";
/// The file a re-read writes its findings and prescriptions to, last.
pub(crate) const RESULT_FILE: &str = "result.json";

/// `round.toml`: a briefed round's base, the snapshot its re-read reads, and
/// the paths that changed between them.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct RoundRecord {
    /// The round's base sha.
    pub(crate) base: String,
    /// The snapshot sha the brief was written at.
    pub(crate) snapshot: String,
    /// The round's changed paths, from the project root.
    pub(crate) paths: Vec<String>,
}

/// A briefed round's directory, `{feature rounds}/{base}`, from the project
/// root. `base` is a full sha the caller resolved, so it is a safe name.
pub(crate) fn round_rel(feature: &str, base: &str) -> String {
    format!("{ROUNDS_DIR}/{feature}/{base}")
}

/// A briefed round's directory for reading, checked to resolve inside the
/// project and created nowhere: a reader writes no ignore file.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] when it, or a link on the way to it,
/// resolves outside the project.
pub(crate) fn round_dir_to_read(repo: &Path, feature: &str, base: &str) -> Result<PathBuf> {
    let rel = round_rel(feature, base);
    super::validate_contained(repo, &rel)?;
    Ok(repo.join(rel))
}

/// A re-read's `result.json`, as `read-re-read` and `check-prescriptions` read
/// it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ResultFile {
    /// No file, or one whose `complete` is not `true`: the re-read did not
    /// finish. Carries the reason.
    Incomplete(String),
    /// Not JSON of the result file's shape. Carries the reason.
    Unparseable(String),
    /// A finished re-read and its findings.
    Complete(Vec<ReReadFinding>),
}

/// The result file's top level. Unknown fields are refused, as they are in
/// each finding and edit, so a misspelled key is a reported problem rather
/// than a silently dropped prescription.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ResultDoc {
    #[serde(default)]
    complete: bool,
    findings: Vec<ReReadFinding>,
}

/// Read the result file at `path`.
///
/// # Errors
///
/// [`PrimitiveError::Io`] when the file exists but cannot be read; an absent
/// file is [`ResultFile::Incomplete`], since a re-read that never wrote one
/// did not finish.
pub(crate) fn read_result(path: &Path) -> Result<ResultFile> {
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok(ResultFile::Incomplete(format!(
                "{} does not exist",
                path.display()
            )));
        }
        Err(source) => {
            return Err(PrimitiveError::Io {
                path: path.into(),
                source,
            });
        }
    };
    let doc: ResultDoc = match serde_json::from_slice(&bytes) {
        Ok(doc) => doc,
        Err(err) => {
            return Ok(ResultFile::Unparseable(super::flatten_line(
                &err.to_string(),
            )));
        }
    };
    if let Some(problem) = doc
        .findings
        .iter()
        .enumerate()
        .find_map(|(i, f)| malformed(i, f))
    {
        return Ok(ResultFile::Unparseable(problem));
    }
    if !doc.complete {
        return Ok(ResultFile::Incomplete("`complete` is not true".to_string()));
    }
    Ok(ResultFile::Complete(doc.findings))
}

/// What is wrong with a finding the schema admits: an empty or multi-line
/// `path` or `text`, or an edit with an empty `path` or `old`. An empty `old`
/// would match everywhere, and a new file is never a prescription.
fn malformed(index: usize, finding: &ReReadFinding) -> Option<String> {
    let line = |value: &str| !value.is_empty() && !value.chars().any(super::is_line_hazard);
    if !line(&finding.path) {
        return Some(format!(
            "finding [{index}]: `path` must be one non-empty line"
        ));
    }
    if !line(&finding.text) {
        return Some(format!(
            "finding [{index}]: `text` must be one non-empty line"
        ));
    }
    finding.edits.iter().enumerate().find_map(|(e, edit)| {
        if !line(&edit.path) {
            Some(format!(
                "finding [{index}] edit [{e}]: `path` must be one non-empty line"
            ))
        } else if edit.old.is_empty() {
            Some(format!("finding [{index}] edit [{e}]: `old` is empty"))
        } else {
            None
        }
    })
}

/// The `[review] re-reads-off` reason from the resolved config file, flattened
/// to one line, or `None` when the key is absent and the loop runs.
///
/// The reason is held to the length `[[review.disabled-rule-files]]` requires
/// of its own, since both are the audit trail for switching part of a review
/// off. Any other value halts the caller rather than running or skipping the
/// loop on a guess (`CFG-ENV-008`), as does a `review` that is not a table.
///
/// # Errors
///
/// [`PrimitiveError::InvalidReReadsOff`] for a value that is not a string, or
/// one shorter than [`MIN_REASON_LEN`] once trimmed, and for a `review` that
/// is not a table, each naming the config file; [`PrimitiveError::Toml`] when
/// the file does not parse; [`PrimitiveError::Io`] when it cannot be read.
pub(crate) fn re_reads_off(repo: &Path) -> Result<Option<String>> {
    let (path, _) = paths::resolve_config(repo);
    if !path.is_file() {
        return Ok(None);
    }
    let content = super::read_text(&path)?;
    let config: toml::Table = toml::from_str(&content).map_err(|source| PrimitiveError::Toml {
        path: path.clone(),
        source,
    })?;
    let invalid = |problem: String| PrimitiveError::InvalidReReadsOff {
        path: path.clone(),
        problem,
    };
    let review = match config.get("review") {
        None => return Ok(None),
        Some(toml::Value::Table(review)) => review,
        Some(other) => {
            return Err(invalid(format!(
                "`review` is {} ({}), not a table",
                toml_type_name(other),
                rendered(other)
            )));
        }
    };
    let reason = match review.get(RE_READS_OFF) {
        None => return Ok(None),
        Some(toml::Value::String(reason)) => reason,
        Some(other) => {
            return Err(invalid(format!(
                "the value is {} ({}), not a string",
                toml_type_name(other),
                rendered(other)
            )));
        }
    };
    let length = reason.trim().chars().count();
    if length < MIN_REASON_LEN {
        return Err(invalid(format!(
            "the reason {} is {length} characters once trimmed, under the minimum of {MIN_REASON_LEN}",
            rendered(&toml::Value::String(reason.clone()))
        )));
    }
    Ok(Some(super::collapse_whitespace(&super::flatten_line(
        reason,
    ))))
}

/// The identity a snapshot commit is made under, so a snapshot needs no
/// `user.name` or `user.email` from the operator's git config.
const SNAPSHOT_NAME: &str = "ductus";
/// See [`SNAPSHOT_NAME`].
const SNAPSHOT_EMAIL: &str = "ductus@localhost";
/// The snapshot commit's message. Nothing reads it; it says what the object is
/// to anyone who finds it with `git cat-file`.
const SNAPSHOT_MESSAGE: &str = "ductus fix-round snapshot\n";

/// The feature's round directory, checked to resolve inside the project, with
/// the rounds directory's own `.gitignore` in place.
///
/// The `.gitignore` holds `*`, so nothing under the rounds directory ever
/// enters a snapshot or a `modified-since` list, whatever the repository's own
/// `.gitignore` says — the reason `.ductus/sessions/` ignores itself too.
/// `feature` must already have passed `validate_feature_argument`.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] when the directory, or a link on the way to
/// it, resolves outside the project; [`PrimitiveError::Io`] when the
/// `.gitignore` cannot be written.
pub(crate) fn feature_rounds_dir(repo: &Path, feature: &str) -> Result<PathBuf> {
    let rel = format!("{ROUNDS_DIR}/{feature}");
    super::validate_contained(repo, &rel)?;
    ignore_everything(&repo.join(ROUNDS_DIR))?;
    Ok(repo.join(rel))
}

/// Write `dir/.gitignore` containing `*` unless it exists. Shared with the
/// results directory list files are written to (`bounded_lists`).
///
/// Created with `create_new`, so two first writers cannot replace one
/// another's file: the second finds it present and leaves it. A rename-based
/// write raced on Windows for the session directory's identical file
/// (`crate::session`, `ignore_everything`), which serializes behind a lock
/// this directory has no need of.
pub(crate) fn ignore_everything(dir: &Path) -> Result<()> {
    std::fs::create_dir_all(dir).map_err(|source| PrimitiveError::Io {
        path: dir.into(),
        source,
    })?;
    let path = dir.join(".gitignore");
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
    {
        Ok(mut file) => file
            .write_all(b"*\n")
            .map_err(|source| PrimitiveError::Io { path, source }),
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(source) => Err(PrimitiveError::Io { path, source }),
    }
}

/// Remove every earlier round of the feature, leaving its directory empty:
/// a review run starts from no rounds (spec 078, *The budget*).
///
/// `dir` comes from [`feature_rounds_dir`], which judged where it resolves.
/// `remove_dir_all` removes a link in its way rather than following it.
///
/// # Errors
///
/// [`PrimitiveError::Io`] when the directory cannot be removed or recreated.
pub(crate) fn clear_rounds(dir: &Path) -> Result<()> {
    match std::fs::remove_dir_all(dir) {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(PrimitiveError::Io {
                path: dir.into(),
                source,
            });
        }
    }
    std::fs::create_dir_all(dir).map_err(|source| PrimitiveError::Io {
        path: dir.into(),
        source,
    })
}

/// A snapshot of the whole working tree: a commit object whose tree holds
/// every tracked file as it is on disk, every untracked file that is not
/// ignored, and no deleted one, with `HEAD` as its parent (none on an unborn
/// `HEAD`). Returns the commit's sha and the number of files its tree holds.
///
/// It moves no ref and writes no index file. The tree is built in the
/// repository's index **in memory** — loaded, updated from the working tree,
/// and written out as tree objects only — and the index is never written
/// back, so the file on disk keeps its bytes. The commit is created with no
/// ref to update, so `HEAD`, every branch and every reflog stay as they were.
/// Nothing references the object; git keeps an unreferenced object for
/// `gc.pruneExpire`, two weeks by default, which outlives a review run.
///
/// The whole work tree, not only the project directory: the diff a round's
/// paths come from is bounded to the project already, and a pathspec here
/// would treat a project path holding a glob character as a pattern.
///
/// # Errors
///
/// [`PrimitiveError::Git`] for any libgit2 failure — an index holding
/// unresolved conflicts, which cannot be written as a tree, among them.
pub(crate) fn snapshot(project: &ProjectRepository) -> Result<(String, usize)> {
    let repository = &project.repository;
    let mut index = repository.index()?;
    // libgit2's `add_all` applies the working tree's whole diff to the index:
    // it adds new and modified files, skips ignored ones, and drops the entry
    // of a file deleted from disk (`the_snapshot_is_the_working_tree` holds
    // all three).
    index.add_all(["*"], IndexAddOption::DEFAULT, None)?;
    let files = index.len();
    let tree = repository.find_tree(index.write_tree()?)?;
    let parent = match repository.head() {
        Ok(head) => Some(head.peel_to_commit()?),
        Err(err) if err.code() == git2::ErrorCode::UnbornBranch => None,
        Err(err) => return Err(err.into()),
    };
    let parents: Vec<&git2::Commit<'_>> = parent.iter().collect();
    let signature = Signature::now(SNAPSHOT_NAME, SNAPSHOT_EMAIL)?;
    let commit = repository.commit(
        None,
        &signature,
        &signature,
        SNAPSHOT_MESSAGE,
        &tree,
        &parents,
    )?;
    Ok((commit.to_string(), files))
}

/// The files inside the project that differ between two snapshots, from the
/// project root and sorted: a round's changed paths.
///
/// Tree to tree, because both are whole working-tree snapshots. A diff from a
/// snapshot to the working tree through the index — `compute-review-scope`'s
/// `modified-since` — reports every unstaged edit as changed, since the
/// snapshot holds it and the index does not, so every round would re-list
/// every earlier round's unstaged fixes (spec 078, the round's base).
///
/// # Errors
///
/// [`PrimitiveError::Git`] when either sha names no commit, or the diff fails.
pub(crate) fn changed_between(
    project: &ProjectRepository,
    base: &str,
    snapshot: &str,
) -> Result<Vec<String>> {
    let repository = &project.repository;
    let tree = |sha: &str| -> Result<git2::Tree<'_>> {
        Ok(repository.find_commit(Oid::from_str(sha)?)?.tree()?)
    };
    let mut opts = project_diff_options(project);
    let diff = repository.diff_tree_to_tree(
        Some(&tree(base)?),
        Some(&tree(snapshot)?),
        Some(&mut opts),
    )?;
    project_paths(project, &diff)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::git_fixture::write;

    const REASON: &str = "Reviews here run on a host without subagents.";

    fn config(body: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        write(&tmp.path().join(".ductus/config.toml"), body);
        tmp
    }

    fn problem(err: &PrimitiveError) -> String {
        match err {
            PrimitiveError::InvalidReReadsOff { problem, .. } => problem.clone(),
            other => panic!("not InvalidReReadsOff: {other:?}"),
        }
    }

    #[test]
    fn an_absent_key_or_file_runs_the_loop() {
        assert_eq!(
            re_reads_off(tempfile::tempdir().unwrap().path()).unwrap(),
            None
        );
        assert_eq!(re_reads_off(config("").path()).unwrap(), None);
        assert_eq!(
            re_reads_off(config("[review]\ntech-stack-verified = true\n").path()).unwrap(),
            None
        );
    }

    /// A valid reason comes back on one line, its whitespace collapsed.
    #[test]
    fn a_valid_reason_switches_the_loop_off() {
        let tmp = config(&format!("[review]\nre-reads-off = \"{REASON}\"\n"));
        assert_eq!(re_reads_off(tmp.path()).unwrap().as_deref(), Some(REASON));
        let tmp = config(
            "[review]\nre-reads-off = \"\"\"\nReviews here run   on a host\nwithout subagents.\"\"\"\n",
        );
        assert_eq!(re_reads_off(tmp.path()).unwrap().as_deref(), Some(REASON));
    }

    /// A reason under the minimum halts, counting trimmed characters: the
    /// padding around thirteen characters does not lift them to sixteen.
    #[test]
    fn a_short_reason_halts() {
        let tmp = config("[review]\nre-reads-off = \"   fifteen chars  \"\n");
        let err = re_reads_off(tmp.path()).unwrap_err();
        assert!(
            problem(&err).contains("13 characters once trimmed"),
            "{err}"
        );
        let tmp = config("[review]\nre-reads-off = \"sixteen chars ok\"\n");
        assert!(re_reads_off(tmp.path()).unwrap().is_some());
    }

    #[test]
    fn a_value_that_is_not_a_string_halts() {
        let tmp = config("[review]\nre-reads-off = true\n");
        let err = re_reads_off(tmp.path()).unwrap_err();
        assert!(
            problem(&err).contains("a boolean (true), not a string"),
            "{err}"
        );
        let tmp = config("review = \"off\"\n");
        let err = re_reads_off(tmp.path()).unwrap_err();
        assert!(problem(&err).contains("`review` is a string"), "{err}");
    }

    /// The error names the file the value came from, on each tier of the
    /// config chain, and the setting.
    #[test]
    fn the_error_names_the_setting_and_each_config_file() {
        for file in paths::CONFIG_CHAIN {
            let tmp = tempfile::tempdir().unwrap();
            write(&tmp.path().join(file), "[review]\nre-reads-off = 1\n");
            let message = re_reads_off(tmp.path()).unwrap_err().to_string();
            assert!(message.contains(file), "{file}: {message}");
            assert!(message.contains("`[review] re-reads-off`"), "{message}");
        }
    }
}
