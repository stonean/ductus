//! Integration test (spec 078): the four fix-round primitives driven through
//! one `/{project}:review` loop, called as the host calls them.
//!
//! Each primitive's unit tests prove its own contract. This proves that the
//! contracts compose: the base `start-fix-rounds` returns is what the first
//! round is diffed from; the result file a re-read writes at `brief-re-read`'s
//! `result` path is what `read-re-read` and `check-prescriptions` read; a
//! re-read's snapshot bases the round after it, and a matched check's snapshot
//! the round after that; the loop's own files never enter a round; the diff
//! command each brief names shows exactly that round; and the run moves no
//! ref, index or `HEAD`.
//!
//! The round files are found through `brief-re-read`'s `brief` and `result`
//! fields, as a host finds them, since the round-directory helpers are
//! crate-private.

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use std::fs;
use std::path::Path;

use git2::{IndexAddOption, Repository, Signature};

use ductus::primitives::{
    brief_re_read, check_prescriptions, check_review_gate, read_re_read, start_fix_rounds,
    write_review,
};
use ductus::schema::primitives::{
    BriefReReadArgs, BriefReReadResult, CheckPrescriptionsArgs, CheckPrescriptionsResult,
    CheckReviewGateArgs, PrescriptionVerdict, ReReadState, ReadReReadArgs, ReadReReadResult,
    ReviewGateBlock, RoundKind, StartFixRoundsArgs, WriteReviewArgs,
};

const FEATURE: &str = "001-widget";

fn write(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

/// A committed project, then an unstaged edit and an untracked file: the tree
/// as a review's passes leave it, which the run's base must hold whole.
fn fixture(dir: &Path) -> Repository {
    let repository = Repository::init(dir).unwrap();
    write(
        &dir.join("specs").join(FEATURE).join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# Widget\n",
    );
    write(&dir.join("src/a.rs"), "fn a() -> u8 { 1 }\n");
    write(&dir.join("src/b.rs"), "fn b() -> u8 { 2 }\n");
    write(&dir.join("src/c.rs"), "fn c() -> u8 { 3 }\n");
    write(&dir.join("docs/guide.md"), "# Guide\n");
    {
        let mut index = repository.index().unwrap();
        index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let signature = Signature::now("Test", "test@example.com").unwrap();
        repository
            .commit(Some("HEAD"), &signature, &signature, "init", &tree, &[])
            .unwrap();
    }
    write(&dir.join("docs/guide.md"), "# Guide\n\nUnstaged.\n");
    write(&dir.join("notes.md"), "untracked\n");
    repository
}

/// What a review run must leave as it found it: `HEAD`, every ref and its
/// target, and the index file's bytes.
fn untouched_state(dir: &Path, repository: &Repository) -> (String, Vec<String>, Vec<u8>) {
    let head = repository.head().unwrap().target().unwrap().to_string();
    let refs = repository
        .references()
        .unwrap()
        .map(|reference| {
            let reference = reference.unwrap();
            format!(
                "{} {:?}",
                reference.name().unwrap(),
                reference.target().map(|oid| oid.to_string())
            )
        })
        .collect();
    (head, refs, fs::read(dir.join(".git/index")).unwrap())
}

fn brief(dir: &Path, base: &str) -> BriefReReadResult {
    brief_re_read::run(
        &BriefReReadArgs {
            feature: FEATURE.to_string(),
            base: base.to_string(),
        },
        dir,
    )
    .unwrap()
}

fn read(dir: &Path, base: &str) -> ReadReReadResult {
    read_re_read::run(
        &ReadReReadArgs {
            feature: FEATURE.to_string(),
            base: base.to_string(),
        },
        dir,
    )
    .unwrap()
}

fn check(dir: &Path, base: &str) -> CheckPrescriptionsResult {
    check_prescriptions::run(
        &CheckPrescriptionsArgs {
            feature: FEATURE.to_string(),
            base: base.to_string(),
        },
        dir,
    )
    .unwrap()
}

/// A round briefed at `base` needing a re-read, with exactly `paths`: its
/// brief lists those paths and no other, and the diff command it names, run
/// from the project root, shows those files and no other. Returns the round's
/// snapshot.
fn assert_round(dir: &Path, round: &BriefReReadResult, base: &str, paths: &[&str]) -> String {
    assert_eq!(round.kind, RoundKind::ReRead, "{round:?}");
    assert_eq!(round.paths.as_deref().unwrap(), paths, "{round:?}");
    let snapshot = round.snapshot.clone().unwrap();

    let text = fs::read_to_string(dir.join(round.brief.as_ref().unwrap())).unwrap();
    let (_, listed) = text.split_once("````json\n").unwrap();
    let (listed, _) = listed.split_once("\n````").unwrap();
    let listed: Vec<String> = serde_json::from_str(listed).unwrap();
    assert_eq!(
        listed, paths,
        "the brief lists other paths than its round's"
    );

    let command = format!("git diff --relative {base} {snapshot}");
    assert!(text.contains(&command), "the brief names no `{command}`");
    let output = common::git_command()
        .args(["diff", "--relative", "--name-only", base, &snapshot])
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let mut shown: Vec<String> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    shown.sort();
    assert_eq!(
        shown, paths,
        "the brief's diff command shows another change"
    );
    snapshot
}

/// Two re-read rounds and the diff-checked round that ends the loop. Round 1
/// is the review's own fixes; its re-read prescribes an edit. Round 2 applies
/// that edit and changes a file nobody prescribed, so the check differs and
/// round 2 is re-read whole from round 1's snapshot. Its re-read prescribes
/// one more edit, round 3 applies exactly that and matches, and the matched
/// check's snapshot leaves nothing to re-read.
#[test]
fn two_rounds_each_brief_only_their_own_paths() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    let repository = fixture(dir);
    let before = untouched_state(dir, &repository);

    let base = start_fix_rounds::run(
        &StartFixRoundsArgs {
            feature: FEATURE.to_string(),
        },
        dir,
    )
    .unwrap()
    .base;

    // Round 1: the review's fixes. The unstaged `docs/guide.md` and the
    // untracked `notes.md` were in the tree when the passes ended, so the base
    // holds them and they are no part of the round.
    write(&dir.join("src/a.rs"), "fn a() -> u8 { 10 }\n");
    write(&dir.join("src/b.rs"), "fn b() -> u8 { 20 }\n");
    let round_1 = brief(dir, &base);
    let snapshot_1 = assert_round(dir, &round_1, &base, &["src/a.rs", "src/b.rs"]);

    // Its re-read: a SHOULD on `src/a.rs` with a prescribed edit, and a NOTE.
    write(
        &dir.join(round_1.result.as_ref().unwrap()),
        r#"{"complete": true, "findings": [
            {"severity": "SHOULD", "path": "src/a.rs", "text": "the value regressed",
             "edits": [{"path": "src/a.rs", "old": "{ 10 }", "new": "{ 11 }"}]},
            {"severity": "NOTE", "path": "src/b.rs", "text": "a style point"}]}"#,
    );
    let read_1 = read(dir, &base);
    assert_eq!(read_1.state, ReReadState::Complete, "{read_1:?}");
    assert_eq!(
        (
            read_1.must,
            read_1.should,
            read_1.notes,
            read_1.unprescribed
        ),
        (0, 1, 1, 0)
    );

    // Round 2: the prescribed edit, and a change to `src/c.rs` that nothing
    // prescribed, so the diff check cannot stand in for a re-read.
    write(&dir.join("src/a.rs"), "fn a() -> u8 { 11 }\n");
    write(&dir.join("src/c.rs"), "fn c() -> u8 { 30 }\n");
    let check_1 = check(dir, &base);
    assert_eq!(check_1.verdict, PrescriptionVerdict::Differs, "{check_1:?}");
    assert_eq!(check_1.unprescribed.as_deref().unwrap(), ["src/c.rs"]);
    assert_eq!(
        (
            check_1.differing.as_deref().map(<[_]>::len),
            check_1.failed.as_deref().map(<[_]>::len)
        ),
        (Some(0), Some(0))
    );

    // So round 2 is re-read whole, from round 1's snapshot: its brief holds
    // round 2's files alone — not `src/b.rs`, which only round 1 changed, and
    // not the round files round 1 wrote under `.ductus/`.
    let round_2 = brief(dir, &snapshot_1);
    assert_round(dir, &round_2, &snapshot_1, &["src/a.rs", "src/c.rs"]);

    write(
        &dir.join(round_2.result.as_ref().unwrap()),
        r#"{"complete": true, "findings": [
            {"severity": "MUST", "path": "src/c.rs", "text": "the value is wrong",
             "edits": [{"path": "src/c.rs", "old": "{ 30 }", "new": "{ 31 }"}]}]}"#,
    );
    let read_2 = read(dir, &snapshot_1);
    assert_eq!(read_2.state, ReReadState::Complete, "{read_2:?}");
    assert_eq!((read_2.must, read_2.should, read_2.unprescribed), (1, 0, 0));

    // Round 3 applies exactly that edit: the check matches with nothing open,
    // and its snapshot, the next base, leaves no round.
    write(&dir.join("src/c.rs"), "fn c() -> u8 { 31 }\n");
    let check_2 = check(dir, &snapshot_1);
    assert_eq!(check_2.verdict, PrescriptionVerdict::Matched, "{check_2:?}");
    assert_eq!((check_2.examined, check_2.open), (1, 0));
    let after = brief(dir, check_2.snapshot.as_ref().unwrap());
    assert_eq!(after.kind, RoundKind::Empty, "{after:?}");
    assert_eq!(after.paths.as_deref().map(<[_]>::len), Some(0));
    assert!(after.snapshot.is_none() && after.brief.is_none());

    // The review committed nothing.
    assert_eq!(untouched_state(dir, &repository), before);
}

/// A commit of the whole work tree, as a landing in a shared checkout is one.
fn commit(repository: &Repository, subject: &str) {
    let mut index = repository.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
    let signature = Signature::now("Test", "test@example.com").unwrap();
    let parent = repository.head().unwrap().peel_to_commit().unwrap();
    repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            subject,
            &tree,
            &[&parent],
        )
        .unwrap();
}

/// A commit of what the index holds, leaving the rest of the work tree as it is,
/// as a session landing only its own files does.
fn commit_paths(repository: &Repository, subject: &str) {
    let mut index = repository.index().unwrap();
    let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
    let signature = Signature::now("Test", "test@example.com").unwrap();
    let parent = repository.head().unwrap().peel_to_commit().unwrap();
    repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            subject,
            &tree,
            &[&parent],
        )
        .unwrap();
}

/// Spec 079: another spec's landing between rounds leaves the loop, without the
/// host reporting anything, and a path a fix also touched stays in it.
#[test]
fn another_specs_commit_between_rounds_leaves_the_round() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    let repository = fixture(dir);
    write(
        &dir.join("specs/002-y/spec.md"),
        "---\nstatus: draft\ndependencies: []\n---\n",
    );
    commit(
        &repository,
        "chore: a sibling, and the tree the review starts from",
    );
    let start = start_fix_rounds::run(
        &StartFixRoundsArgs {
            feature: FEATURE.to_string(),
        },
        dir,
    )
    .unwrap()
    .base;

    write(&dir.join("src/landed.rs"), "fn landed() {}\n");
    write(&dir.join("src/both.rs"), "fn landed() {}\n");
    commit(&repository, "feat(002): another session's landing");
    write(&dir.join("src/both.rs"), "fn fixed_on_top() {}\n");
    write(&dir.join("src/a.rs"), "fn a() -> u8 { 11 }\n");

    let round = brief(dir, &start);
    assert_eq!(round.kind, RoundKind::ReRead, "{round:?}");
    assert_eq!(round.paths.as_deref().unwrap(), ["src/a.rs", "src/both.rs"]);
    assert_eq!(round.excluded.as_deref().unwrap(), ["src/landed.rs"]);
    let text = fs::read_to_string(dir.join(round.brief.as_ref().unwrap())).unwrap();
    assert!(!text.contains("src/landed.rs"), "{text}");

    // A landing after the re-read does not spoil the check of its prescribed
    // edit: it is no change of the round's, prescribed or not.
    write(
        &dir.join(round.result.as_ref().unwrap()),
        r#"{"complete": true, "findings": [
            {"severity": "SHOULD", "path": "src/a.rs", "text": "the value regressed",
             "edits": [{"path": "src/a.rs", "old": "{ 11 }", "new": "{ 12 }"}]}]}"#,
    );
    write(&dir.join("src/a.rs"), "fn a() -> u8 { 12 }\n");
    write(&dir.join("src/landed_later.rs"), "fn later() {}\n");
    repository
        .index()
        .and_then(|mut index| {
            index.add_path(Path::new("src/landed_later.rs"))?;
            index.write()
        })
        .unwrap();
    commit_paths(&repository, "feat(002): a later landing");
    let checked = check(dir, &start);
    assert_eq!(checked.verdict, PrescriptionVerdict::Matched, "{checked:?}");
}

/// Spec 079, AC14: a review cut off after its first write holds `done`, and its
/// last write releases it, while a record written with no loop, as `ductus
/// exec` writes one, was never held.
#[test]
fn a_review_cut_off_between_its_writes_holds_done() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    fixture(dir);
    let gate = || {
        check_review_gate::run(
            &CheckReviewGateArgs {
                feature: FEATURE.to_string(),
            },
            dir,
        )
        .unwrap()
    };
    let write = |loop_open: bool| {
        write_review::run(
            &WriteReviewArgs {
                feature: FEATURE.to_string(),
                reviewed_at: "2026-11-05T00:00:00Z".to_string(),
                reviewed_against: "abc".to_string(),
                diff_base: "HEAD".to_string(),
                loop_open,
                ..WriteReviewArgs::default()
            },
            dir,
        )
        .unwrap()
    };

    write(true);
    assert!(
        fs::read_to_string(dir.join("specs").join(FEATURE).join("review.md"))
            .unwrap()
            .contains("fix-rounds-open: true")
    );
    assert_ne!(
        gate().blocked_by,
        None,
        "the gate must not pass a run cut off between its writes"
    );

    write(false);
    let record = fs::read_to_string(dir.join("specs").join(FEATURE).join("review.md")).unwrap();
    assert!(!record.contains("fix-rounds-open"), "{record}");
    assert_ne!(gate().blocked_by, Some(ReviewGateBlock::FixRoundLoopOpen));
}
