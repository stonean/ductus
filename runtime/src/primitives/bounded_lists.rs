//! Bounding a result that carries path lists to one read (066 scenario
//! `scope-results-have-a-bounded-size`).
//!
//! `compute-review-scope` and `diff-cross-spec` return lists that grow with
//! the history since their base, and a result over a host's tool-output cap
//! arrives as an error with no result at all. So each result is returned
//! inline while its serialized form fits `[artifacts] read-size-bytes` — the
//! size spec 063 treats as one read, 50,000 bytes by default — and past it the
//! lists are written whole to a file the result names, with their counts
//! inline. One contract serves the MCP tool, the CLI and the `ductus exec`
//! walker alike.
//!
//! The file sits under `.ductus/results/{primitive}/{feature}/`, named by the
//! sha256 of its bytes, so two calls in one tree never overwrite each other's
//! lists and a repeated call reuses its file. The results directory ignores
//! itself, as `.ductus/review-rounds/` does, so no list file enters a scope.

use std::path::Path;
use std::time::{Duration, SystemTime};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::primitives::check_artifact_size::resolve_threshold;
use crate::primitives::{PrimitiveError, Result, fix_rounds, write_atomic};

/// The directory list files are written under, from the project root.
pub(crate) const RESULTS_DIR: &str = ".ductus/results";

/// How long a list file outlives the call that wrote it. A write removes the
/// same feature's files older than this, so the directory does not grow with
/// every review for the life of a checkout, while a concurrent reader's file,
/// written moments before, stays.
pub(crate) const STALE_AFTER: Duration = Duration::from_secs(24 * 60 * 60);

/// Where a result's lists go: `None` when `inline`, serialized, fits the
/// configured read size; otherwise the path, from the project root, of the
/// file `lists` was written to whole.
///
/// A result exactly at the size fits. `lists` is written as pretty JSON, one
/// entry per line, so a host whose reader cannot take the file in one call
/// reads it in parts by line offset.
///
/// # Errors
///
/// [`PrimitiveError::InvalidReadSize`] for a malformed `[artifacts]
/// read-size-bytes` and [`PrimitiveError::Toml`] for a config file that does
/// not parse, as every reader of the setting halts; [`PrimitiveError::Json`]
/// when a value does not serialize; [`PrimitiveError::InvalidPath`] when the
/// results directory, or a link on the way to it, resolves outside the
/// project; and [`PrimitiveError::Io`] when the file cannot be written. A
/// file that cannot be written fails the call: a result never names lists
/// that are not on disk.
pub(crate) fn place(
    repo: &Path,
    primitive: &str,
    feature: &str,
    inline: &impl Serialize,
    lists: &impl Serialize,
) -> Result<Option<String>> {
    let (threshold, _) = resolve_threshold(repo)?;
    let serialize_error = |source| PrimitiveError::Json {
        path: repo.join(RESULTS_DIR),
        source,
    };
    let size = serde_json::to_vec(inline).map_err(serialize_error)?.len();
    if u64::try_from(size).unwrap_or(u64::MAX) <= threshold.bytes {
        return Ok(None);
    }
    let mut body = serde_json::to_string_pretty(lists).map_err(serialize_error)?;
    body.push('\n');
    let name = format!("{}.json", super::hex(&Sha256::digest(body.as_bytes())));
    let dir_rel = format!("{RESULTS_DIR}/{primitive}/{feature}");
    super::validate_contained(repo, &dir_rel)?;
    fix_rounds::ignore_everything(&repo.join(RESULTS_DIR))?;
    let dir = repo.join(&dir_rel);
    std::fs::create_dir_all(&dir).map_err(|source| PrimitiveError::Io {
        path: dir.clone(),
        source,
    })?;
    prune_stale(&dir, &name, SystemTime::now());
    write_atomic(&dir.join(&name), &body)?;
    Ok(Some(format!("{dir_rel}/{name}")))
}

/// Remove the `.json` files in `dir`, other than `keep`, last modified more
/// than [`STALE_AFTER`] before `now`.
///
/// Best effort: a file that cannot be read or removed is left, since a stale
/// list file no result names is harmless, and failing the call over one would
/// trade the result for housekeeping.
fn prune_stale(dir: &Path, keep: &str, now: SystemTime) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let stale = path.extension().is_some_and(|ext| ext == "json")
            && entry.file_name() != keep
            && entry
                .metadata()
                .and_then(|meta| meta.modified())
                .is_ok_and(|modified| {
                    now.duration_since(modified)
                        .is_ok_and(|age| age > STALE_AFTER)
                });
        if stale {
            let _ = std::fs::remove_file(&path);
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn repo(config: Option<&str>) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        if let Some(config) = config {
            std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
            std::fs::write(tmp.path().join(".ductus/config.toml"), config).unwrap();
        }
        tmp
    }

    /// A result at the read size stays inline, one byte past it goes to a
    /// file that holds the lists whole, in a directory that ignores itself.
    #[test]
    fn a_result_past_the_read_size_is_written_whole_to_a_file() {
        let inline = serde_json::json!({"scope": ["a", "b"]});
        let size = serde_json::to_vec(&inline).unwrap().len();
        let tmp = repo(Some(&format!("[artifacts]\nread-size-bytes = {size}\n")));
        assert_eq!(
            place(tmp.path(), "p", "001-x", &inline, &inline).unwrap(),
            None
        );

        let tmp = repo(Some(&format!(
            "[artifacts]\nread-size-bytes = {}\n",
            size - 1
        )));
        let file = place(tmp.path(), "p", "001-x", &inline, &inline)
            .unwrap()
            .unwrap();
        assert!(file.starts_with(".ductus/results/p/001-x/"), "{file}");
        let read: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(tmp.path().join(&file)).unwrap())
                .unwrap();
        assert_eq!(read, inline);
        assert_eq!(
            std::fs::read_to_string(tmp.path().join(".ductus/results/.gitignore")).unwrap(),
            "*\n"
        );
    }

    /// The default read size is 50,000 bytes.
    #[test]
    fn the_default_read_size_is_fifty_thousand_bytes() {
        let tmp = repo(None);
        let fits = serde_json::Value::String("x".repeat(49_998));
        assert_eq!(place(tmp.path(), "p", "001-x", &fits, &fits).unwrap(), None);
        let past = serde_json::Value::String("x".repeat(49_999));
        assert!(
            place(tmp.path(), "p", "001-x", &past, &past)
                .unwrap()
                .is_some()
        );
    }

    /// Different lists get different files, the same lists the same one.
    #[test]
    fn a_file_is_named_by_its_content() {
        let tmp = repo(Some("[artifacts]\nread-size-bytes = 1\n"));
        let one = place(tmp.path(), "p", "001-x", &["a"], &["a"]).unwrap();
        let two = place(tmp.path(), "p", "001-x", &["b"], &["b"]).unwrap();
        let again = place(tmp.path(), "p", "001-x", &["a"], &["a"]).unwrap();
        assert_ne!(one, two);
        assert_eq!(one, again);
    }

    /// A malformed read size halts the call rather than choosing a form.
    #[test]
    fn a_malformed_read_size_halts() {
        let tmp = repo(Some("[artifacts]\nread-size-bytes = \"big\"\n"));
        assert!(matches!(
            place(tmp.path(), "p", "001-x", &["a"], &["a"]),
            Err(PrimitiveError::InvalidReadSize { .. })
        ));
    }

    /// `prune_stale` removes the feature's list files older than a day and
    /// keeps newer ones.
    #[test]
    fn prune_stale_removes_only_stale_files() {
        let tmp = repo(None);
        let dir = tmp.path().join("results");
        std::fs::create_dir_all(&dir).unwrap();
        for name in ["old.json", "new.json", "keep.json", "old.txt"] {
            std::fs::write(dir.join(name), "x").unwrap();
        }
        let now = SystemTime::now();
        for name in ["old.json", "keep.json", "old.txt"] {
            std::fs::File::options()
                .write(true)
                .open(dir.join(name))
                .unwrap()
                .set_modified(now - STALE_AFTER - Duration::from_secs(60))
                .unwrap();
        }
        prune_stale(&dir, "keep.json", now);
        let mut left: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        left.sort();
        assert_eq!(left, ["keep.json", "new.json", "old.txt"]);
    }

    /// A call past the read size prunes the feature's stale list files: the
    /// write does it, not only the helper.
    #[test]
    fn a_call_past_the_read_size_prunes_stale_files() {
        let tmp = repo(Some("[artifacts]\nread-size-bytes = 1\n"));
        let dir = tmp.path().join(RESULTS_DIR).join("p/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        let old = dir.join("old.json");
        std::fs::write(&old, "x").unwrap();
        std::fs::File::options()
            .write(true)
            .open(&old)
            .unwrap()
            .set_modified(SystemTime::now() - STALE_AFTER - Duration::from_secs(60))
            .unwrap();
        let file = place(tmp.path(), "p", "001-x", &["a"], &["a"])
            .unwrap()
            .unwrap();
        assert!(!old.exists(), "a stale list file outlived the call");
        assert!(tmp.path().join(file).is_file());
    }

    /// A results directory linked outside the project is refused before
    /// anything is written through it.
    #[cfg(unix)]
    #[test]
    fn a_results_directory_linked_outside_the_project_is_refused() {
        let tmp = repo(Some("[artifacts]\nread-size-bytes = 1\n"));
        let outside = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(outside.path(), tmp.path().join(RESULTS_DIR)).unwrap();
        assert!(matches!(
            place(tmp.path(), "p", "001-x", &["a"], &["a"]),
            Err(PrimitiveError::InvalidPath { .. })
        ));
        assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 0);
    }

    /// A list file that cannot be written fails the call, so a result never
    /// names lists that are not on disk: a directory where the file goes
    /// refuses the write.
    #[test]
    fn a_file_that_cannot_be_written_fails_the_call() {
        let tmp = repo(Some("[artifacts]\nread-size-bytes = 1\n"));
        let file = tmp.path().join(
            place(tmp.path(), "p", "001-x", &["a"], &["a"])
                .unwrap()
                .unwrap(),
        );
        std::fs::remove_file(&file).unwrap();
        std::fs::create_dir(&file).unwrap();
        assert!(matches!(
            place(tmp.path(), "p", "001-x", &["a"], &["a"]),
            Err(PrimitiveError::Io { .. })
        ));
    }

    /// A results directory that cannot be made fails the call too.
    #[test]
    fn a_directory_that_cannot_be_made_fails_the_call() {
        let tmp = repo(Some("[artifacts]\nread-size-bytes = 1\n"));
        std::fs::create_dir_all(tmp.path().join(RESULTS_DIR)).unwrap();
        std::fs::write(
            tmp.path().join(RESULTS_DIR).join("p"),
            "a file, not a directory",
        )
        .unwrap();
        assert!(matches!(
            place(tmp.path(), "p", "001-x", &["a"], &["a"]),
            Err(PrimitiveError::Io { .. })
        ));
    }
}
