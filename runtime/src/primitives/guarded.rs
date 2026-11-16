//! The read and the write of a single-file read-modify-write primitive, held
//! to the compare-before-persist guard (`BE-TXN-002`; spec 022 task 38, plan
//! §Compare before persist).
//!
//! `append-question`, `append-task`, `label-criteria`, `mark-criterion`,
//! `mark-task`, `set-status`, `append-inbox`, `remove-inbox-item`,
//! `merge-managed-block` and `merge-permissions` each read one file, compute
//! its replacement and rename that over it. [`read_required`] and
//! [`write_guarded`] are the two halves they share: the read keeps a
//! [`ReadSnapshot`], and the write refuses with
//! [`PrimitiveError::ChangedSinceRead`] when the file changed in between, so
//! the other writer's change is kept and the operator re-runs the command.
//! The guard is atomic among the writers that go through it; any writer that
//! does not go through it can still land between the re-read and the rename.

use std::path::Path;

use crate::primitives::{PrimitiveError, ReadSnapshot, Result};

/// Read the file at `rel` (project-relative) that must exist, keeping the
/// snapshot the write compares against. A file that is not there is the
/// [`PrimitiveError::Io`] a plain read of it returns, naming `repo/rel`.
///
/// # Errors
///
/// [`PrimitiveError::InvalidPath`] when `rel` leaves the project,
/// [`PrimitiveError::Io`] when the file is absent or not UTF-8.
pub(crate) fn read_required(repo: &Path, rel: &str) -> Result<(ReadSnapshot, String)> {
    let snapshot = ReadSnapshot::read(repo, rel)?;
    let text = snapshot
        .text()?
        .ok_or_else(|| PrimitiveError::Io {
            path: repo.join(rel),
            source: std::io::Error::from(std::io::ErrorKind::NotFound),
        })?
        .to_string();
    Ok((snapshot, text))
}

/// Replace the file `snapshot` read with `content`, refusing when it changed
/// since.
///
/// # Errors
///
/// [`PrimitiveError::ChangedSinceRead`] when the file changed, was created or
/// went since the read: nothing is written. Otherwise as
/// [`ReadSnapshot::write_if_unchanged`].
pub(crate) fn write_guarded(snapshot: &ReadSnapshot, content: &str) -> Result<()> {
    #[cfg(test)]
    tests_seam::fire();
    snapshot.write_if_unchanged(content)
}

/// Run `hook` once between the read and the write of the next
/// [`write_guarded`] on this thread, as a second writer landing there would.
/// Test builds only; production has no hook.
#[cfg(test)]
pub(crate) fn between_read_and_write<R>(
    hook: impl FnOnce() + 'static,
    run: impl FnOnce() -> R,
) -> R {
    tests_seam::HOOK.with(|slot| *slot.borrow_mut() = Some(Box::new(hook)));
    let out = run();
    tests_seam::HOOK.with(|slot| *slot.borrow_mut() = None);
    out
}

#[cfg(test)]
mod tests_seam {
    use std::cell::RefCell;

    type Hook = Box<dyn FnOnce()>;

    thread_local! {
        pub(super) static HOOK: RefCell<Option<Hook>> = const { RefCell::new(None) };
    }

    pub(super) fn fire() {
        if let Some(hook) = HOOK.with(|slot| slot.borrow_mut().take()) {
            hook();
        }
    }
}

#[cfg(test)]
mod raw_writes_allowlist {
    //! No primitive replaces a file it read without the guard (`BE-TXN-002`;
    //! spec 022 task 38, plan §Compare before persist).
    //!
    //! A non-test call of the raw `write_atomic` or `write_atomic_bytes` under
    //! `runtime/src/` must sit in a file [`ALLOWED`] names, with the number of
    //! calls it holds and why each is not a lost-update window. A new primitive
    //! that rewrites a file it read takes the guard
    //! (`ReadSnapshot::write_if_unchanged`), or joins [`ALLOWED`] visibly in
    //! review. The check reads the crate's own source, so it needs no build step.
    //!
    //! What it sees: a call spelled `write_atomic(` or `write_atomic_bytes(`
    //! outside comments and outside the file's trailing `#[cfg(test)] mod`, and
    //! not a `fn` definition. `write_new_atomic` is a different name and is not
    //! counted: it is create-only and refuses an existing file, so it cannot
    //! replace another writer's change. It does not see the function passed as a
    //! value, and it cannot tell a call that reads its file first from one that
    //! does not: [`ALLOWED`]'s reasons are the review's judgment of that.

    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};

    use regex::Regex;

    /// Each file holding raw calls: the number of calls, and why none is a
    /// read-modify-write of a project file another writer can change.
    const ALLOWED: &[(&str, usize, &str)] = &[
        (
            "src/primitives/mod.rs",
            2,
            "the helper itself: `write_atomic`'s forward to `write_atomic_bytes`, and the guarded \
             write's rename under the compare-and-set lock",
        ),
        (
            "src/primitives/apply_manifest.rs",
            4,
            "regenerates each destination whole from its manifest source, never from the \
             destination's own content",
        ),
        (
            "src/primitives/bounded_lists.rs",
            1,
            "writes a fresh file under .ductus/results/ named by its content, never read first",
        ),
        (
            "src/primitives/brief_re_read.rs",
            2,
            "writes a round's record and brief into a directory it creates, whole",
        ),
        (
            "src/primitives/create_feature.rs",
            1,
            "creates the new feature's spec.md from a template, after refusing an existing feature",
        ),
        (
            "src/primitives/create_plan_artifacts.rs",
            1,
            "copies a template into a plan artifact that does not exist yet",
        ),
        (
            "src/primitives/fetch_archive.rs",
            1,
            "writes the downloaded archive to a staging path the caller names, whole",
        ),
        (
            "src/primitives/merge_status_hook.rs",
            1,
            "waived: spec 080's `departs: BE-TXN-002` entry covers `merge-status-hook` alone",
        ),
        (
            "src/primitives/migrate_session_file.rs",
            1,
            "writes the destination session file whole from the legacy file's values, once",
        ),
        (
            "src/session.rs",
            3,
            "the session files and their ignore file, each written while the session lock is held",
        ),
    ];

    fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
            .unwrap_or_else(|err| panic!("cannot read {}: {err}", dir.display()))
            .map(|entry| entry.unwrap().path())
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                rust_files(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }

    /// The number of raw write calls in `source`'s non-test code.
    fn raw_write_calls(source: &str) -> usize {
        let call = Regex::new(r"\bwrite_atomic(?:_bytes)?\s*\(").unwrap();
        let definition = Regex::new(r"\bfn\s+write_atomic(?:_bytes)?\b").unwrap();
        let test_module =
            Regex::new(r"(?m)^[ \t]*#\[cfg\(test\)\]\s*\n[ \t]*(?:pub(?:\([a-z]+\))? )?mod \w+ \{")
                .unwrap();
        let production = test_module
            .find(source)
            .map_or(source, |found| &source[..found.start()]);
        production
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .filter(|line| call.is_match(line) && !definition.is_match(line))
            .count()
    }

    /// Calls per file, relative to `runtime/`, for the files holding any.
    fn measured() -> (BTreeMap<String, usize>, usize) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = Vec::new();
        rust_files(&root.join("src"), &mut files);
        let mut found = BTreeMap::new();
        for file in &files {
            let source = std::fs::read_to_string(file).unwrap();
            let calls = raw_write_calls(&source);
            if calls > 0 {
                let rel = file
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                found.insert(rel, calls);
            }
        }
        (found, files.len())
    }

    #[test]
    fn every_raw_write_is_the_helpers_or_on_the_allowlist() {
        let (found, examined) = measured();
        assert!(examined > 50, "the walk read only {examined} source files");
        assert!(
            !found.is_empty(),
            "no raw write call found in {examined} files: the scan is broken, not the crate clean"
        );
        let allowed: BTreeMap<&str, usize> =
            ALLOWED.iter().map(|(file, n, _)| (*file, *n)).collect();
        let mut problems = Vec::new();
        for (file, calls) in &found {
            match allowed.get(file.as_str()) {
                None => problems.push(format!(
                    "{file} holds {calls} raw write call(s) and is not on the allowlist: take the \
                     guard (`ReadSnapshot::write_if_unchanged`), or add it with its reason"
                )),
                Some(n) if n != calls => problems.push(format!(
                    "{file} holds {calls} raw write call(s), the allowlist says {n}"
                )),
                Some(_) => {}
            }
        }
        for (file, _, reason) in ALLOWED {
            if !found.contains_key(*file) {
                problems.push(format!(
                    "{file} is on the allowlist and holds no raw write call"
                ));
            }
            if reason.trim().is_empty() {
                problems.push(format!("{file} is on the allowlist with no reason"));
            }
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    #[test]
    fn the_scan_counts_a_call_and_skips_comments_definitions_and_test_code() {
        let source = "\
    //! mentions write_atomic(path, text) in prose
    pub(crate) fn write_atomic(path: &Path) -> Result<()> {
        write_atomic_bytes(path, b\"\")
    }
    fn run() {
        // write_atomic(&p, &t)?;
        write_atomic(&p, &t)?;
        crate::primitives::write_atomic_bytes (&p, &b)?;
        write_new_atomic(&p, &t)?;
    }
    #[cfg(test)]
    mod tests {
        fn t() { write_atomic(&p, &t).unwrap(); }
    }
    ";
        assert_eq!(raw_write_calls(source), 3);
    }
}
