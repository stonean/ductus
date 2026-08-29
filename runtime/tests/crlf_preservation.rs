//! Integration coverage (spec 051, scenario `rewrites-preserve-line-endings`):
//! a primitive that rewrites a file it did not create gives the file back
//! with the line endings it found.
//!
//! The regression this guards is silent on the machine most likely to run it.
//! `str::lines()` strips a trailing `\r`, so a writer that re-joins with a
//! bare `\n` converts a CRLF checkout's file to LF as a side effect of
//! changing one line — invisible in a repository whose files are already LF,
//! which is why it survived review of each writer individually and was found
//! only by comparing them against each other.
//!
//! Two of the writers covered here were worse than a clean conversion: they
//! reassembled *part* of a file and carried the rest through untouched,
//! leaving the halves disagreeing. A uniformly-converted file at least reads
//! as one deliberate change; a half-converted one is indistinguishable from a
//! hand-edit.
//!
//! Each case asserts the property directly — **no bare LF survives** — rather
//! than comparing against an expected blob, so a writer that grows a new
//! `push('\n')` fails here without anyone re-blessing a fixture.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

use ductus::primitives;
use ductus::schema::primitives::{
    AppendQuestionArgs, AppendTaskArgs, InvalidateReviewArgs, PruneTasksArgs, RemoveInboxItemArgs,
    RewriteSpecLinksArgs, WriteReviewArgs,
};

/// Every `\n` in `text` is preceded by `\r` — i.e. the file is uniformly CRLF
/// with no bare LF anywhere. The assertion is one-sided on purpose: a file
/// that lost its endings and one that kept only some both fail it.
fn is_uniformly_crlf(text: &str) -> bool {
    text.match_indices('\n')
        .all(|(i, _)| i > 0 && text.as_bytes()[i - 1] == b'\r')
}

/// Write `text` with every line ending converted to CRLF.
fn write_crlf(path: &Path, text: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let crlf = text.replace('\n', "\r\n");
    assert!(
        is_uniformly_crlf(&crlf),
        "fixture must start uniformly CRLF"
    );
    fs::write(path, crlf).unwrap();
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

fn assert_crlf(path: &Path, what: &str) {
    let after = read(path);
    assert!(
        is_uniformly_crlf(&after),
        "{what} left a bare LF in {}:\n{after:?}",
        path.display()
    );
}

const SPEC: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-08-01T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nnext-criterion: 2\n---\n\n# 050 — Alpha\n\n## Motivation\n\nWhy.\n\n## Acceptance Criteria\n\n- [ ] AC1: Something is true.\n\n## Open Questions\n\n*None.*\n";

#[test]
fn rewrite_spec_links_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    write_crlf(&repo.join("specs/050-alpha/spec.md"), SPEC);
    // A sibling whose body links at the retiring directory — the one line
    // this rewrite is supposed to touch.
    write_crlf(
        &repo.join("specs/060-beta/spec.md"),
        "---\nstatus: draft\ndependencies: []\n---\n\n# 060 — Beta\n\nSee [staged](../1234.1-staged/spec.md) for detail.\n\nMore prose.\n",
    );
    write_crlf(
        &repo.join("specs/1234.1-staged/spec.md"),
        "---\nstatus: draft\ndependencies: []\nfolds-into: 050-alpha\n---\n\n# staged\n",
    );

    let result = primitives::rewrite_spec_links::run(
        &RewriteSpecLinksArgs {
            from: "1234.1-staged".into(),
            to: "050-alpha".into(),
        },
        repo,
    )
    .unwrap();
    assert_eq!(result.rewritten.len(), 1, "the sibling link should move");

    let beta = repo.join("specs/060-beta/spec.md");
    assert_crlf(&beta, "rewrite-spec-links");
    let after = read(&beta);
    assert!(after.contains("../050-alpha/spec.md"), "{after}");
    // Only the link changed: every other line survives verbatim.
    assert!(after.contains("More prose.\r\n"), "{after}");
}

#[test]
fn remove_inbox_item_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    let inbox = repo.join("specs/inbox.md");
    write_crlf(
        &inbox,
        "# Inbox\n\n- [ ] first item\n- [ ] second item\n- [ ] third item\n",
    );

    let result = primitives::remove_inbox_item::run(
        &RemoveInboxItemArgs {
            item: "second item".into(),
        },
        repo,
    )
    .unwrap();
    assert!(result.removed);

    assert_crlf(&inbox, "remove-inbox-item");
    let after = read(&inbox);
    assert!(!after.contains("second item"), "{after}");
    assert!(after.contains("third item"), "{after}");
}

#[test]
fn append_question_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    let spec = repo.join("specs/050-alpha/spec.md");
    write_crlf(&spec, SPEC);

    primitives::append_question::run(
        &AppendQuestionArgs {
            feature: "050-alpha".into(),
            question: "Does the ending survive?".into(),
            scenario: None,
        },
        repo,
    )
    .unwrap();

    assert_crlf(&spec, "append-question");
    assert!(read(&spec).contains("Does the ending survive?"));
}

#[test]
fn append_task_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    write_crlf(&repo.join("specs/050-alpha/spec.md"), SPEC);
    let tasks = repo.join("specs/050-alpha/tasks.md");
    write_crlf(
        &tasks,
        "# 050 — Alpha Tasks\n\n## 1. First\n\n- [x] did it\n\n- **Done when**: done.\n",
    );

    let result = primitives::append_task::run(
        &AppendTaskArgs {
            feature_path: "specs/050-alpha".into(),
            title: "Second".into(),
            done_when: "it is done.".into(),
            body: None,
            slug: Some("second-thing".into()),
            parent_heading: None,
        },
        repo,
    )
    .unwrap();
    assert!(result.appended);

    assert_crlf(&tasks, "append-task");
    let after = read(&tasks);
    assert!(after.contains("## 2. Second"), "{after}");
    // The pre-existing block is untouched, endings included.
    assert!(after.contains("## 1. First\r\n"), "{after}");
}

#[test]
fn prune_tasks_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    write_crlf(&repo.join("specs/050-alpha/spec.md"), SPEC);
    let tasks = repo.join("specs/050-alpha/tasks.md");
    write_crlf(
        &tasks,
        "# 050 — Alpha Tasks\n\n## 1. Spent\n\n- [x] did it\n\n- **Done when**: done.\n\n## 2. Pending\n\n- [ ] not yet\n\n- **Done when**: later.\n",
    );

    primitives::prune_tasks::run(
        &PruneTasksArgs {
            feature: "050-alpha".into(),
            reset: false,
            force: true,
            apply: true,
        },
        repo,
    )
    .unwrap();

    assert_crlf(&tasks, "prune-tasks");
    let after = read(&tasks);
    assert!(
        after.contains("## 2. Pending") || after.contains("Pending"),
        "{after}"
    );
}

/// `write-review` and `invalidate-review` share a frontmatter splice that
/// reassembled the block with `\n` while carrying the body through as read —
/// so a CRLF spec came back with halves that disagreed. Mixed is the outcome
/// worth pinning, because nothing downstream can tell it from a hand-edit.
#[test]
fn write_review_preserves_crlf_across_both_halves() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    let spec = repo.join("specs/050-alpha/spec.md");
    write_crlf(&spec, SPEC);

    primitives::write_review::run(
        &WriteReviewArgs {
            feature: "050-alpha".into(),
            reviewed_at: "2026-08-29T00:00:00Z".into(),
            reviewed_against: "deadbeef".into(),
            diff_base: "cafebabe".into(),
            ..Default::default()
        },
        repo,
    )
    .unwrap();

    assert_crlf(&spec, "write-review");
    let after = read(&spec);
    assert!(after.contains("reviewed-against: deadbeef"), "{after}");
    // The body half came through the same writer, not around it.
    assert!(after.contains("## Motivation"), "{after}");
}

#[test]
fn invalidate_review_preserves_crlf_across_both_halves() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    let spec = repo.join("specs/050-alpha/spec.md");
    write_crlf(&spec, SPEC);

    let result = primitives::invalidate_review::run(
        &InvalidateReviewArgs {
            feature: "050-alpha".into(),
        },
        repo,
    )
    .unwrap();
    assert!(result.invalidated);

    assert_crlf(&spec, "invalidate-review");
    let after = read(&spec);
    assert!(after.contains("last-run: null"), "{after}");
    assert!(after.contains("## Motivation"), "{after}");
}
