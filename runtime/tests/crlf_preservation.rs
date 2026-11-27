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
    AppendInboxArgs, AppendQuestionArgs, AppendTaskArgs, BackfillSectionDigestArgs,
    InvalidateReviewArgs, PruneReviewArgs, PruneTasksArgs, RemoveInboxItemArgs,
    RewriteSpecLinksArgs, WriteAnalysisArgs, WriteReviewArgs,
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

const SPEC: &str = "---\nstatus: in-progress\ndependencies: []\nnext-criterion: 2\n---\n\n# 050 — Alpha\n\n## Motivation\n\nWhy.\n\n## Acceptance Criteria\n\n- [ ] AC1: Something is true.\n\n## Open Questions\n\n*None.*\n";

/// A recorded review, in the artifact that owns it (spec 057).
const REVIEW_RECORD: &str = "---\nspec: 050-alpha\nlast-run: 2026-08-01T00:00:00Z\nreviewed-against: abc123\nmust-violations: 0\nblocking: false\n---\n\n# Review — 050-alpha\n\n## Summary\n\nClean.\n";

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
    assert_eq!(result.rewritten_count, 1, "the sibling link should move");

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
            dedup_title: false,
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
            feature: Some("050-alpha".into()),
            all: false,
            reset: false,
            force: true,
            apply: true,
        },
        repo,
    )
    .unwrap();

    assert_crlf(&tasks, "prune-tasks");
    // The reduction was written, so the endings checked above are the
    // rewritten file's, not the untouched fixture's.
    let after = read(&tasks);
    assert!(
        !after.contains("## 1. Spent"),
        "spent section kept: {after}"
    );
    assert!(after.contains("## 2. Pending\r\n"), "{after}");
}

/// `--reset` rewrites the file down to its `# ` heading — which it lifts
/// *from the file being reset*, so this is a rewrite of existing content and
/// not the creation of new content. Its `content` parameter was named
/// `_content` precisely because it had been ignored, which is what let the
/// first sweep of this scenario walk past it.
#[test]
fn prune_tasks_reset_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    write_crlf(&repo.join("specs/050-alpha/spec.md"), SPEC);
    let tasks = repo.join("specs/050-alpha/tasks.md");
    write_crlf(
        &tasks,
        "# 050 — Alpha Tasks\n\n## 1. Spent\n\n- [x] did it\n\n- **Done when**: done.\n",
    );

    primitives::prune_tasks::run(
        &PruneTasksArgs {
            feature: Some("050-alpha".into()),
            all: false,
            reset: true,
            force: true,
            apply: true,
        },
        repo,
    )
    .unwrap();

    assert_crlf(&tasks, "prune-tasks --reset");
    let after = read(&tasks);
    assert!(after.contains("# 050 — Alpha Tasks"), "{after}");
    // The reset was written: the task is gone and the template body is in.
    assert!(
        !after.contains("## 1. Spent"),
        "spent section kept: {after}"
    );
    assert!(
        after.contains("<!--"),
        "no template guidance comment: {after}"
    );
}

/// Commit every file in `repository`'s work tree with `message`.
fn commit_all(repository: &git2::Repository, message: &str) {
    let mut index = repository.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();
    let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("Fixture", "fixture@example.com").unwrap();
    let parents: Vec<git2::Commit> = repository
        .head()
        .ok()
        .and_then(|head| head.peel_to_commit().ok())
        .into_iter()
        .collect();
    let parents: Vec<&git2::Commit> = parents.iter().collect();
    repository
        .commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
        .unwrap();
}

/// `prune-review` splices lines out of a record's frontmatter and rewrites a line of its
/// body (spec 086): both halves keep CRLF, a removal at the frontmatter's last line,
/// whose newline the closing fence holds, included.
#[test]
fn prune_review_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    let repository = git2::Repository::init(repo).unwrap();
    fs::create_dir_all(repo.join("src")).unwrap();
    fs::write(repo.join("src/foreign.rs"), "fn f() {}\n").unwrap();
    commit_all(&repository, "chore: code no spec claims");
    write_crlf(&repo.join("specs/050-alpha/spec.md"), SPEC);
    commit_all(&repository, "chore: seed");
    let record = repo.join("specs/050-alpha/review.md");
    write_crlf(
        &record,
        "---\nspec: 050-alpha\nlast-run: 2026-08-01T00:00:00Z\nblocking: false\ndecisions:\n  \
         - key: \"bug: a finding — `src/foreign.rs`\"\n    outcome: waived\n    reason: why\n    \
         decided-at: 2026-08-01T00:00:00Z\n    decided-by: dev@example.com\nwaivers:\n  \
         - rule: SEC-BE-014\n    anchors:\n      - file: src/foreign.rs\n        site: s\n        \
         covers: 1\n    reason: why\n    waived-at: 2026-08-01T00:00:00Z\n    \
         waived-by: dev@example.com\n---\n\n# Review — 050-alpha\n\n## Excluded from scope\n\n\
         - `src/a.rs` — 050-alpha\n- `src/b.rs`\n",
    );
    let args = |apply: bool, digest: Option<String>| PruneReviewArgs {
        feature: Some("050-alpha".into()),
        apply,
        digest,
        ..PruneReviewArgs::default()
    };
    let preview = primitives::prune_review::run(&args(false, None), repo)
        .unwrap()
        .summary
        .unwrap();
    assert_eq!((preview.drops_count, preview.rewrites_count), (2, 1));

    let applied = primitives::prune_review::run(&args(true, preview.digest), repo)
        .unwrap()
        .summary
        .unwrap();
    assert!(applied.applied);
    assert_crlf(&record, "prune-review");
    // Both halves were rewritten, so the endings checked above are the edited file's.
    let after = read(&record);
    assert!(!after.contains("src/foreign.rs"), "{after}");
    assert!(after.contains("blocking: false\r\n---\r\n"), "{after}");
    assert!(
        after.contains("- `src/a.rs`\r\n- `src/b.rs`\r\n"),
        "{after}"
    );
}

/// An append is the other half of the same rule: the existing content keeps
/// its endings for free, so the bug is the *appended* line arriving with a
/// different one — a file that disagrees with itself rather than one that
/// converted cleanly.
#[test]
fn append_inbox_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    let inbox = repo.join("specs/inbox.md");
    write_crlf(&inbox, "# Inbox\n\n- [ ] an existing item\n");

    primitives::append_inbox::run(
        &AppendInboxArgs {
            text: "a newly appended item".into(),
        },
        repo,
    )
    .unwrap();

    assert_crlf(&inbox, "append-inbox");
    let after = read(&inbox);
    assert!(after.contains("a newly appended item"), "{after}");
    assert!(after.contains("an existing item"), "{after}");
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
            feature: Some("050-alpha".into()),
            reviewed_at: "2026-08-29T00:00:00Z".into(),
            reviewed_against: "deadbeef".into(),
            diff_base: String::new(),
            ..Default::default()
        },
        repo,
    )
    .unwrap();

    // The spec is no longer a write target, so its CRLF survives by not being
    // touched at all — asserted rather than assumed, because "untouched" is the
    // new contract and a future edit that reintroduced a spec write would have
    // to break this line to do it.
    assert_crlf(&spec, "write-review (spec untouched)");
    assert_eq!(
        read(&spec),
        read_expected_spec(),
        "spec.md must be untouched"
    );

    // The record is the file that gets written.
    let record = repo.join("specs/050-alpha/review.md");
    let after = read(&record);
    assert!(after.contains("reviewed-against: deadbeef"), "{after}");
}

/// `SPEC` as it lands on disk, for the untouched comparison.
fn read_expected_spec() -> String {
    SPEC.replace('\n', "\r\n")
}

#[test]
fn invalidate_review_preserves_crlf_across_both_halves() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    let spec = repo.join("specs/050-alpha/spec.md");
    write_crlf(&spec, SPEC);
    // The record it invalidates, CRLF like everything else in this checkout.
    let record = repo.join("specs/050-alpha/review.md");
    write_crlf(&record, REVIEW_RECORD);

    let result = primitives::invalidate_review::run(
        &InvalidateReviewArgs {
            feature: "050-alpha".into(),
        },
        repo,
    )
    .unwrap();
    assert!(result.invalidated);

    // The record is rewritten in place, so its line ending is the one that has
    // to survive — frontmatter and report body alike, since the splice is LF
    // while the body is carried through as it was read.
    assert_crlf(&record, "invalidate-review");
    let after = read(&record);
    assert!(after.contains("last-run: null"), "{after}");
    assert!(after.contains("## Summary"), "{after}");
    // And the spec is untouched, CRLF included.
    assert_crlf(&spec, "invalidate-review (spec untouched)");
    assert_eq!(
        read(&spec),
        read_expected_spec(),
        "spec.md must be untouched"
    );
}

/// `backfill-section-digest` places its field in a CRLF record in the
/// record's own ending (spec 088, AC3): the record, written by
/// `write-analysis` over the CRLF subjects, its `section-digest` then
/// removed and the whole made CRLF, gains the field and stays uniformly CRLF.
#[test]
fn backfill_section_digest_preserves_crlf() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    write_crlf(&repo.join("specs/050-alpha/spec.md"), SPEC);
    primitives::write_analysis::run(
        &WriteAnalysisArgs {
            feature: "050-alpha".into(),
            analyzed_at: "2026-11-26T00:00:00Z".into(),
            analyzed_against: "abc".into(),
            ..Default::default()
        },
        repo,
    )
    .unwrap();
    let record = repo.join("specs/050-alpha/analysis.md");
    let mut without = String::new();
    let mut skipping = false;
    for line in read(&record).replace("\r\n", "\n").split_inclusive('\n') {
        if line.starts_with("section-digest:") {
            skipping = true;
            continue;
        }
        if skipping && line.starts_with("  ") {
            continue;
        }
        skipping = false;
        without.push_str(line);
    }
    write_crlf(&record, &without);

    let result =
        primitives::backfill_section_digest::run(&BackfillSectionDigestArgs::default(), repo)
            .unwrap();
    assert_eq!(result.written_count, 1, "{result:?}");
    assert_crlf(&record, "backfill-section-digest");
    assert!(read(&record).contains("section-digest:\r\n"));
}
