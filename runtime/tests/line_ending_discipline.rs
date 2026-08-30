//! Source-level lint (spec 051, scenario `rewrites-preserve-line-endings`):
//! every primitive that writes a **text** file must account for the line
//! endings of the file it is writing.
//!
//! This exists because the per-primitive CRLF cases in `crlf_preservation.rs`
//! verify the writers someone thought to list, which is not the same as
//! verifying the property. That distinction is not theoretical: the sweep was
//! written against seven writers found by reading the code, and a review pass
//! immediately found two more it did not cover — `prune-tasks --reset`, whose
//! `content` parameter was named `_content` because it had been deliberately
//! ignored, and `append-inbox`, which kept the file's endings for everything
//! except the line it appended. Both were the same defect as the one that
//! prompted the work, found the same way it was: by chance.
//!
//! So this test enumerates the **code** rather than a list. A new primitive
//! that writes text with no handling anywhere fails here on the day it is
//! added — the "a fourth writer inherits the behavior rather than
//! rediscovering the defect" promise enforced rather than merely stated.
//!
//! **What it cannot catch, stated because a green run would otherwise imply
//! it.** The evidence check is per **file**, not per function. A file that
//! handles endings in one writer and not in another passes. That is not
//! hypothetical either: both misses above were in files that already carried
//! an evidence token — `prune_tasks.rs` used `line_ending_of` in its
//! keep-pending path while `reduce_reset` ignored content entirely, and
//! `append_inbox.rs` used `split_inclusive` in its bullet scanner while
//! `append_after` did not. Reverting both fixes leaves this test green.
//!
//! Per-path coverage is `crlf_preservation.rs`'s job: it drives each writer
//! against a CRLF fixture and asserts no bare LF survives. The two tests are
//! complements and neither is sufficient — this one catches a writer nobody
//! thought about, that one catches a path inside a writer somebody did.
//! Closing the gap properly would mean per-function analysis, which is a
//! parser rather than a lint; the seam is recorded here instead of being
//! left for the next reader to discover the way this one was.
//!
//! **Subject**: files under `runtime/src/primitives/` that call
//! `write_atomic(` — the text writer. `write_atomic_bytes` is deliberately
//! out of scope: its callers copy bytes (template installs, archive
//! extraction), and bytes are preserved by construction.
//!
//! **Evidence** that a writer accounts for endings, any one of:
//!
//! - `line_ending_of` / `with_line_ending` — detect and restore.
//! - `split_inclusive('\n')` — the terminator never leaves the data, so
//!   there is nothing to restore. Stronger than detect-and-restore, and the
//!   pattern a new writer should reach for first (`mark-task`,
//!   `mark-criterion`, `set-status`, `label-criteria`).
//!
//! Anything else needs an entry in [`EXEMPT`] with a reason that survives
//! being read aloud.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::PathBuf;

/// Writers whose output is not a rewrite of an existing text file, with the
/// reason each is out of scope. Kept deliberately short: an entry here is a
/// claim that the file has nothing to preserve, and a wrong one silently
/// re-opens the defect this test exists to catch.
const EXEMPT: &[(&str, &str)] = &[
    (
        "create_scenario.rs",
        "creates a new scenario file; there is no prior content whose endings could be preserved",
    ),
    (
        "write_session.rs",
        "renders .ductus/session.toml wholesale from typed fields — generated state, not authored prose",
    ),
    (
        "migrate_session_file.rs",
        "relocates and re-renders the generated session file; same reasoning as write-session",
    ),
    (
        "merge_permissions.rs",
        "writes settings JSON through the serializer, which owns its own formatting",
    ),
    (
        "merge_managed_block.rs",
        "tracks each line's terminator alongside the line (`has_newline`) rather than re-joining, \
         so endings never leave the data; covered by its own CRLF case",
    ),
];

fn primitives_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/primitives")
}

/// Strip `#[cfg(test)]` module bodies so a writer cannot satisfy the lint
/// with evidence that only appears in its tests.
fn without_test_module(source: &str) -> &str {
    match source.find("#[cfg(test)]") {
        Some(idx) => &source[..idx],
        None => source,
    }
}

#[test]
fn every_text_writer_accounts_for_line_endings() {
    const EVIDENCE: &[&str] = &[
        "line_ending_of",
        "with_line_ending",
        "split_inclusive('\\n')",
    ];

    let dir = primitives_dir();
    let mut examined: Vec<String> = Vec::new();
    let mut writers: Vec<String> = Vec::new();
    let mut offenders: Vec<String> = Vec::new();

    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    entries.sort();

    for path in &entries {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if name == "mod.rs" {
            // The helpers' own home, not a primitive.
            continue;
        }
        examined.push(name.clone());
        let source = fs::read_to_string(path).unwrap();
        let body = without_test_module(&source);
        if !body.contains("write_atomic(") {
            continue;
        }
        writers.push(name.clone());
        if EXEMPT.iter().any(|(f, _)| *f == name) {
            continue;
        }
        if !EVIDENCE.iter().any(|marker| body.contains(marker)) {
            offenders.push(name);
        }
    }

    // QUAL-CLAIM-001: a pass must not be indistinguishable from a run that
    // found nothing to look at. The corpus is real or this test is vacuous.
    assert!(
        examined.len() > 20,
        "expected the primitives directory to hold the corpus; examined only {examined:?}"
    );
    assert!(
        writers.len() >= 15,
        "expected many text writers; found {writers:?}"
    );

    assert!(
        offenders.is_empty(),
        "these primitives write a text file without accounting for its line endings:\n  {}\n\n\
         Use `split_inclusive('\\n')` so terminators never leave the data (preferred), or \
         `line_ending_of` + `with_line_ending` to detect and restore. If the writer creates a \
         new file or emits generated state, add it to EXEMPT in this test with the reason.",
        offenders.join("\n  ")
    );
}

/// An `EXEMPT` entry naming a file that no longer writes text is stale — it
/// would silently keep exempting a name that a later refactor could reuse.
#[test]
fn no_exemption_outlives_its_writer() {
    let dir = primitives_dir();
    for (name, reason) in EXEMPT {
        let path = dir.join(name);
        assert!(path.is_file(), "EXEMPT names {name}, which does not exist");
        let source = fs::read_to_string(&path).unwrap();
        assert!(
            without_test_module(&source).contains("write_atomic("),
            "EXEMPT still lists {name} ({reason}), but it no longer writes text — drop the entry"
        );
        assert!(
            reason.len() > 30,
            "EXEMPT entry for {name} needs a reason that explains itself"
        );
    }
}
