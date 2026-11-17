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

/// Write `content` to `path` only when nothing lives there, atomically, for
/// content that is not text: the tempfile [`crate::primitives::write_atomic_bytes`]
/// writes, persisted with `persist_noclobber`, which fails rather than replace
/// an existing file. Of two writers racing on one path exactly one creates it;
/// the loser gets `false`, writing nothing, and its caller reports the file as
/// already there (`BE-RACE-003`; spec 022 task 43). Text goes through
/// [`crate::primitives::write_new_atomic`], the same operation on a `&str`.
///
/// # Errors
///
/// [`PrimitiveError::Io`] when the parent cannot be created, the tempfile
/// cannot be written or synced, or the persist fails for any reason but the
/// destination already existing.
pub(crate) fn write_new_bytes(path: &Path, content: &[u8]) -> Result<bool> {
    use std::io::Write;

    let io = |at: &Path| {
        let at = at.to_path_buf();
        move |source| PrimitiveError::Io { path: at, source }
    };
    let parent = match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    std::fs::create_dir_all(parent).map_err(io(parent))?;
    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(io(parent))?;
    tmp.as_file_mut().write_all(content).map_err(io(path))?;
    tmp.as_file_mut().sync_all().map_err(io(path))?;
    match tmp.persist_noclobber(path) {
        Ok(_) => Ok(true),
        Err(err) if err.error.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(err) => Err(PrimitiveError::Io {
            path: path.into(),
            source: err.error,
        }),
    }
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
mod write_new_bytes_tests {
    #![allow(clippy::unwrap_used)]

    use super::write_new_bytes;

    /// Binary content is created, with its parent, and an existing file is
    /// never replaced: `false` and its bytes untouched (`BE-RACE-003`).
    #[test]
    fn creates_once_and_never_replaces() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("a/b/file.bin");
        assert!(write_new_bytes(&path, &[0xff, 0x00, 0x7f]).unwrap());
        assert_eq!(std::fs::read(&path).unwrap(), [0xff, 0x00, 0x7f]);
        assert!(!write_new_bytes(&path, b"second").unwrap());
        assert_eq!(std::fs::read(&path).unwrap(), [0xff, 0x00, 0x7f]);
    }
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
    //! outside comments and outside every `#[cfg(test)] mod`'s body, wherever in
    //! the file the module sits, and not a `fn` definition. `write_new_atomic` is a different name and is not
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
            2,
            "the `update` strategy alone, whose contract is to replace the destination with its \
             manifest source's bytes, never computed from the destination's own content; \
             `create` and `skip-if-conflict` write through `write_new_bytes`",
        ),
        (
            "src/primitives/bounded_lists.rs",
            1,
            "writes a file under .ductus/results/ named by its content's hash, so a second writer \
             of that name writes the same bytes: a repeat that is harmless (`BE-RACE-003`'s \
             carve-out)",
        ),
        (
            "src/primitives/brief_re_read.rs",
            2,
            "replaces by design: a base briefed again clears its round directory first and \
             writes the round's record and brief whole, never from the files' own content",
        ),
        (
            "src/primitives/create_feature.rs",
            1,
            "writes spec.md into the feature directory this call just claimed with `create_dir`, \
             which the operating system grants to one of any racers, so no other writer of this \
             call's file exists",
        ),
        (
            "src/primitives/create_plan_artifacts.rs",
            1,
            "the confirmed `overwrite` branch alone, which replaces an artifact with its fresh \
             template; an artifact observed absent is written through `write_new_bytes`",
        ),
        (
            "src/primitives/fetch_archive.rs",
            1,
            "replaces by design: a re-fetch overwrites the staging file the caller names, which \
             lies outside the project, whole and never from its own content",
        ),
        (
            "src/primitives/merge_status_hook.rs",
            1,
            "waived: spec 080's `departs: BE-TXN-002` entry covers `merge-status-hook` alone",
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

    /// The offset just past the `}` closing the block whose `{` ends
    /// `source[..open]`, reading Rust's comments, string and character
    /// literals, so a brace inside one is not counted. The end of `source` when
    /// the block never closes.
    fn block_end(source: &str, open: usize) -> usize {
        let bytes = source.as_bytes();
        let mut depth = 1usize;
        let mut at = open;
        while at < bytes.len() {
            match bytes[at] {
                b'/' if bytes.get(at + 1) == Some(&b'/') => {
                    at += source[at..].find('\n').unwrap_or(bytes.len() - at);
                }
                b'/' if bytes.get(at + 1) == Some(&b'*') => {
                    at += source[at + 2..]
                        .find("*/")
                        .map_or(bytes.len() - at, |n| n + 4);
                }
                b'"' => {
                    // A raw string closes on a quote and its own hashes.
                    let hashes = bytes[..at].iter().rev().take_while(|b| **b == b'#').count();
                    let raw = bytes
                        .get(at - hashes - 1)
                        .is_some_and(|b| *b == b'r')
                        .then_some(hashes);
                    at += 1;
                    while at < bytes.len() {
                        match (bytes[at], raw) {
                            (b'\\', None) => at += 1,
                            (b'"', None) => break,
                            (b'"', Some(n))
                                if bytes[at + 1..].iter().take_while(|b| **b == b'#').count()
                                    >= n =>
                            {
                                at += n;
                                break;
                            }
                            _ => {}
                        }
                        at += 1;
                    }
                    at += 1;
                }
                b'\'' => {
                    // A character literal, not a lifetime: `'x'` or `'\n'`.
                    let close = if bytes.get(at + 1) == Some(&b'\\') {
                        source[at + 3..].find('\'').map(|n| at + 3 + n)
                    } else {
                        source[at + 1..]
                            .chars()
                            .next()
                            .map(|c| at + 1 + c.len_utf8())
                            .filter(|end| bytes.get(*end) == Some(&b'\''))
                    };
                    at = close.map_or(at + 1, |end| end + 1);
                }
                b'{' => {
                    depth += 1;
                    at += 1;
                }
                b'}' => {
                    depth -= 1;
                    at += 1;
                    if depth == 0 {
                        return at;
                    }
                }
                _ => at += 1,
            }
        }
        bytes.len()
    }

    /// `source` without the body of any `#[cfg(test)] mod`, wherever it sits.
    fn without_test_modules(source: &str) -> String {
        let test_module =
            Regex::new(r"(?m)^[ \t]*#\[cfg\(test\)\]\s*\n[ \t]*(?:pub(?:\([a-z]+\))? )?mod \w+ \{")
                .unwrap();
        let mut out = String::new();
        let mut from = 0;
        while let Some(found) = test_module.find_at(source, from) {
            out.push_str(&source[from..found.start()]);
            from = block_end(source, found.end());
        }
        out.push_str(&source[from..]);
        out
    }

    /// The number of raw write calls in `source`'s non-test code.
    fn raw_write_calls(source: &str) -> usize {
        let call = Regex::new(r"\bwrite_atomic(?:_bytes)?\s*\(").unwrap();
        let definition = Regex::new(r"\bfn\s+write_atomic(?:_bytes)?\b").unwrap();
        let production = without_test_modules(source);
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

    /// A test module in the middle of a file hides its own body only: a raw
    /// call in production code after it is counted, as is one in a module
    /// whose literals hold braces (spec 022 task 43, O32).
    #[test]
    fn the_scan_counts_a_call_after_a_test_module() {
        let source = r##"
    fn first() { write_atomic(&p, &t)?; }
    #[cfg(test)]
    mod project_artifact_tests {
        fn t() { write_atomic(&p, "}").unwrap(); let c = '{'; let q = '\''; let r = r#"}"#; }
        fn u<'a>(x: &'a str) { /* } */ write_atomic_bytes(&p, b"{").unwrap(); }
    }
    fn second() { write_atomic_bytes(&p, &b)?; }
    #[cfg(test)]
    mod tests {
        fn t() { write_atomic(&p, &t).unwrap(); }
    }
    fn third() { write_atomic(&p, &t)?; }
    "##;
        assert_eq!(raw_write_calls(source), 3);
    }
}
