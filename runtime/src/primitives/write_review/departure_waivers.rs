//! Tests (spec 073 AC5, AC8): a spec's departure entries waive their rule
//! in the review, composed the way a review run composes the two primitives —
//! `process-waivers` classifies the recorded waivers against what fires, then
//! `write-review` records the run with its result.
//!
//! Each fixture commits a spec holding the entries, so attribution reads real history.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;

use git2::{IndexAddOption, Repository, Signature, Time};
use serde_json::{Value, json};

use crate::primitives::{process_waivers, write_review};
use crate::schema::primitives::{
    FiredFinding, ProcessWaiversArgs, ProcessWaiversResult, WriteReviewArgs, WriteReviewResult,
};

const FEATURE: &str = "001-widget";
/// 2023-11-14T22:13:20Z, the author time every fixture commit carries.
const COMMIT_SECONDS: i64 = 1_700_000_000;
const MUST_RULE: &str = "BE-COMPAT-001";
const SHOULD_RULE: &str = "BE-LOG-002";

fn write(dir: &Path, rel: &str, body: &str) {
    let path = dir.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

fn commit(repository: &Repository, message: &str) {
    let mut index = repository.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
    let signature = Signature::new(
        "Decider",
        "decider@example.com",
        &Time::new(COMMIT_SECONDS, 0),
    )
    .unwrap();
    let parent = repository
        .head()
        .ok()
        .and_then(|head| head.peel_to_commit().ok());
    let parents: Vec<&git2::Commit> = parent.iter().collect();
    repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )
        .unwrap();
}

fn spec_with(entries: &str) -> String {
    format!(
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# Widget\n\n## Applicable Rules\n\n{entries}\n## Acceptance Criteria\n\n- [ ] AC1: it works\n"
    )
}

const MUST_ENTRY: &str = "- departs: `BE-COMPAT-001`\n  - reason: the credential field is removed rather than versioned\n";
const SHOULD_ENTRY: &str = "- departs: `BE-LOG-002`\n  - reason: the hot path logs nothing\n";

/// A repository holding the spec with `entries` and three source files, committed.
fn fixture(dir: &Path, entries: &str) -> Repository {
    let repository = Repository::init(dir).unwrap();
    write(
        dir,
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(entries),
    );
    for file in ["a", "b", "c"] {
        write(dir, &format!("src/{file}.rs"), "fn f() {}\n");
    }
    commit(&repository, "seed");
    repository
}

fn finding(rule: &str, severity: &str, file: &str) -> Value {
    json!({"rule": rule, "severity": severity, "file": file, "confidence": "high",
           "summary": "s", "finding": "f", "rule-text": "t"})
}

/// One review run as the host drives it: classify the recorded waivers against
/// `fired`, then record the run with what that returned.
fn run_review(
    dir: &Path,
    findings: &[Value],
    reviewed_at: &str,
    extra: &Value,
) -> (ProcessWaiversResult, WriteReviewResult) {
    let fired = findings
        .iter()
        .map(|finding| FiredFinding {
            rule: finding["rule"].as_str().unwrap().to_string(),
            file: finding["file"].as_str().unwrap().to_string(),
        })
        .collect();
    let classified = process_waivers::run(
        &ProcessWaiversArgs {
            feature: FEATURE.to_string(),
            fired,
            ..ProcessWaiversArgs::default()
        },
        dir,
    )
    .unwrap();
    let mut args = json!({
        "feature": FEATURE,
        "reviewed-at": reviewed_at,
        "reviewed-against": "abc1234",
        "diff-base": "",
        "empty-scope": false,
        "findings": findings,
        "applied-waivers": classified.applied,
        "expired-waivers": classified.expired,
    });
    for (key, value) in extra.as_object().unwrap() {
        args[key] = value.clone();
    }
    let args: WriteReviewArgs = serde_json::from_value(args).unwrap();
    let written = write_review::run(&args, dir).unwrap();
    (classified, written)
}

fn review_md(dir: &Path) -> String {
    fs::read_to_string(dir.join(format!("specs/{FEATURE}/review.md"))).unwrap()
}

/// The `waivers:` entries of the record, each as `rule`, its files, and the entry itself.
fn waivers(dir: &Path) -> Vec<Value> {
    let text = review_md(dir);
    let frontmatter = text.split("---").nth(1).unwrap();
    let parsed: Value = serde_norway::from_str(frontmatter).unwrap();
    parsed
        .get("waivers")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

fn files_of(waiver: &Value) -> Vec<String> {
    match &waiver["file"] {
        Value::String(file) => vec![file.clone()],
        other => other
            .as_array()
            .unwrap()
            .iter()
            .map(|file| file.as_str().unwrap().to_string())
            .collect(),
    }
}

const T1: &str = "2026-11-12T10:00:00Z";
const T2: &str = "2026-11-12T11:00:00Z";

#[test]
fn a_must_departure_waives_the_rule_at_both_files_and_writes_each_anchor() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let findings = [
        finding(MUST_RULE, "must", "src/a.rs"),
        finding(MUST_RULE, "must", "src/b.rs"),
    ];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(written.must_violations, 0, "{written:?}");
    assert!(!written.blocking);
    assert_eq!(written.waived, 2);
    assert!(written.malformed_departures.is_empty(), "{written:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(stored[0]["rule"], MUST_RULE);
    assert_eq!(files_of(&stored[0]), ["src/a.rs", "src/b.rs"]);
    assert_eq!(stored[0]["departure"], true);
    assert_eq!(
        stored[0]["reason"],
        "the credential field is removed rather than versioned"
    );
    assert_eq!(stored[0]["waived-by"], "decider@example.com");
    assert_eq!(stored[0]["waived-at"], "2023-11-14T22:13:20Z");
}

#[test]
fn a_file_added_later_is_covered_with_no_second_decision() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let first = [
        finding(MUST_RULE, "must", "src/a.rs"),
        finding(MUST_RULE, "must", "src/b.rs"),
    ];
    run_review(tmp.path(), &first, T1, &json!({}));
    let later = [
        finding(MUST_RULE, "must", "src/a.rs"),
        finding(MUST_RULE, "must", "src/b.rs"),
        finding(MUST_RULE, "must", "src/c.rs"),
    ];
    let (classified, written) = run_review(tmp.path(), &later, T2, &json!({}));
    assert_eq!(
        classified.applied.len(),
        2,
        "the stored anchors still apply"
    );
    assert!(classified.expired.is_empty(), "{classified:?}");
    assert_eq!(written.must_violations, 0, "{written:?}");
    assert_eq!(written.waived, 3);
    let mut covered: Vec<String> = waivers(tmp.path()).iter().flat_map(files_of).collect();
    covered.sort();
    assert_eq!(covered, ["src/a.rs", "src/b.rs", "src/c.rs"]);
}

#[test]
fn a_removed_entry_expires_its_waivers_and_the_must_blocks_again() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), MUST_ENTRY);
    let findings = [
        finding(MUST_RULE, "must", "src/a.rs"),
        finding(MUST_RULE, "must", "src/b.rs"),
    ];
    run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(waivers(tmp.path()).len(), 1);

    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(""),
    );
    commit(&repository, "drop the departure");
    let (classified, written) = run_review(tmp.path(), &findings, T2, &json!({}));
    assert!(classified.applied.is_empty(), "{classified:?}");
    assert_eq!(classified.expired.len(), 2);
    assert_eq!(
        classified
            .notices
            .iter()
            .filter(|notice| notice.starts_with("waiver expired: rule BE-COMPAT-001 at src/"))
            .count(),
        2,
        "{:?}",
        classified.notices
    );
    assert_eq!(written.must_violations, 2, "{written:?}");
    assert!(written.blocking);
    assert!(waivers(tmp.path()).is_empty(), "no waiver stored");
}

#[test]
fn a_removed_entry_expires_on_a_dimension_restricted_run_too() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), MUST_ENTRY);
    run_review(
        tmp.path(),
        &[finding(MUST_RULE, "must", "src/a.rs")],
        T1,
        &json!({}),
    );
    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(""),
    );
    commit(&repository, "drop the departure");
    let classified = process_waivers::run(
        &ProcessWaiversArgs {
            feature: FEATURE.to_string(),
            fired: Vec::new(),
            skipped_passes: vec!["efficiency".to_string()],
            ..ProcessWaiversArgs::default()
        },
        tmp.path(),
    )
    .unwrap();
    assert_eq!(classified.expired.len(), 1, "{classified:?}");
    assert!(classified.retained.is_empty(), "{classified:?}");
}

#[test]
fn a_should_departure_leaves_no_should_violation() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), SHOULD_ENTRY);
    let findings = [finding(SHOULD_RULE, "should", "src/a.rs")];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(written.should_violations, 0, "{written:?}");
    assert_eq!(written.waived, 1);
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0]["rule"], SHOULD_RULE);
    assert_eq!(stored[0]["departure"], true);
}

#[test]
fn a_hand_waive_on_the_same_rule_and_file_stays_the_first_claim() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let findings = [
        finding(MUST_RULE, "must", "src/a.rs"),
        finding(MUST_RULE, "must", "src/b.rs"),
    ];
    // The hand waive covers every file the rule fires at, the entry's too; a later run with
    // a file the hand waive did not hold is the entry's.
    let (_, written) = run_review(
        tmp.path(),
        &findings[..1],
        T1,
        &json!({"new-waivers": [{"rule": MUST_RULE, "reason": "by hand"}], "decided-by": "hand@example.com"}),
    );
    assert_eq!(written.must_violations, 0, "{written:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "no second claim on src/a.rs: {stored:?}");
    assert_eq!(stored[0]["reason"], "by hand");
    assert_eq!(stored[0]["waived-by"], "hand@example.com");
    assert!(stored[0].get("departure").is_none(), "{stored:?}");

    let (classified, written) = run_review(tmp.path(), &findings, T2, &json!({}));
    assert_eq!(classified.applied.len(), 1);
    assert_eq!(written.must_violations, 0, "{written:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 2, "{stored:?}");
    assert_eq!(files_of(&stored[0]), ["src/a.rs"]);
    assert_eq!(stored[0]["reason"], "by hand");
    assert_eq!(files_of(&stored[1]), ["src/b.rs"]);
    assert_eq!(stored[1]["departure"], true);
}

#[test]
fn a_hand_waive_and_an_entry_in_one_run_store_each_file_once() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let findings = [
        finding(MUST_RULE, "must", "src/a.rs"),
        finding(MUST_RULE, "must", "src/b.rs"),
    ];
    let (_, written) = run_review(
        tmp.path(),
        &findings,
        T1,
        &json!({"new-waivers": [{"rule": MUST_RULE, "reason": "by hand"}], "decided-by": "hand@example.com"}),
    );
    assert_eq!(written.must_violations, 0, "{written:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(files_of(&stored[0]), ["src/a.rs", "src/b.rs"]);
    assert_eq!(stored[0]["reason"], "by hand");
}

#[test]
fn a_malformed_entry_waives_nothing_and_the_run_names_it() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(
        tmp.path(),
        "- departs: `BE-COMPAT-001`\n  - cited: §anchor\n",
    );
    let findings = [finding(MUST_RULE, "must", "src/a.rs")];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(written.must_violations, 1, "{written:?}");
    assert!(written.blocking);
    assert_eq!(written.malformed_departures.len(), 1, "{written:?}");
    // The entry sits on line 10 of the fixture spec.
    assert_eq!(
        written.malformed_departures,
        ["spec.md line 10: no `reason:`"],
        "{written:?}"
    );
    assert!(waivers(tmp.path()).is_empty(), "no waiver stored");
}

#[test]
fn an_unattributed_entry_stores_unattributed_and_the_runs_time() {
    let tmp = tempfile::tempdir().unwrap();
    // No repository: history cannot say who decided it.
    let dir = tmp.path();
    write(
        dir,
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(MUST_ENTRY),
    );
    write(dir, "src/a.rs", "fn f() {}\n");
    let findings = [finding(MUST_RULE, "must", "src/a.rs")];
    let (_, written) = run_review(dir, &findings, T1, &json!({}));
    assert_eq!(written.must_violations, 0, "{written:?}");
    let stored = waivers(dir);
    assert_eq!(stored[0]["waived-by"], "unattributed");
    assert_eq!(stored[0]["waived-at"], T1);
    // A second run still applies it: a blank waived-at would be malformed.
    let (classified, written) = run_review(dir, &findings, T2, &json!({}));
    assert!(classified.notices.is_empty(), "{:?}", classified.notices);
    assert_eq!(written.must_violations, 0);
}

#[test]
fn a_marked_waiver_with_an_entry_is_classified_as_any_other() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    run_review(
        tmp.path(),
        &[finding(MUST_RULE, "must", "src/a.rs")],
        T1,
        &json!({}),
    );
    // The rule no longer fires and the run's scope did not read the file: the ordinary
    // retention applies to a marked waiver whose rule still has its entry.
    let (classified, _) = run_review(tmp.path(), &[], T2, &json!({}));
    assert!(classified.expired.is_empty(), "{classified:?}");
    assert_eq!(classified.retained.len(), 1, "{classified:?}");
    // And an unmarked waiver whose rule has no entry is untouched by the marker's test.
    fs::write(
        tmp.path().join(format!("specs/{FEATURE}/review.md")),
        "---\nspec: 001-widget\nlast-run: 2026-01-01T00:00:00Z\nblocking: false\nwaivers:\n  - rule: OTHER-1\n    file: src/b.rs\n    reason: kept\n    waived-at: 2026-01-01T00:00:00Z\n    waived-by: dev@example.com\n---\n\n# Review\n",
    )
    .unwrap();
    let (classified, _) = run_review(
        tmp.path(),
        &[finding("OTHER-1", "must", "src/b.rs")],
        T2,
        &json!({}),
    );
    assert_eq!(classified.applied.len(), 1, "{classified:?}");
}

#[test]
fn a_rule_two_entries_name_derives_one_waiver_from_the_first_and_is_named() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(
        tmp.path(),
        &format!(
            "{MUST_ENTRY}- departs: `BE-COMPAT-001`\n  - reason: a second, different reason\n"
        ),
    );
    let findings = [finding(MUST_RULE, "must", "src/a.rs")];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(written.must_violations, 0, "{written:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(
        stored[0]["reason"],
        "the credential field is removed rather than versioned"
    );
    assert_eq!(written.malformed_departures.len(), 1, "{written:?}");
    assert!(
        written.malformed_departures[0].contains("BE-COMPAT-001"),
        "{written:?}"
    );
}

#[test]
fn a_run_writing_its_record_twice_counts_the_stored_departure_waived_both_times() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let findings = [finding(MUST_RULE, "must", "src/a.rs")];
    let args: WriteReviewArgs = serde_json::from_value(json!({
        "feature": FEATURE, "reviewed-at": T1, "reviewed-against": "abc1234",
        "diff-base": "", "empty-scope": false, "findings": findings,
    }))
    .unwrap();
    // One set of arguments, as a review's two writes carry: the second finds the waiver
    // stored and has no host-applied anchor for it.
    let first = write_review::run(&args, tmp.path()).unwrap();
    let second = write_review::run(&args, tmp.path()).unwrap();
    assert_eq!(first.must_violations, 0, "{first:?}");
    assert_eq!(second.must_violations, 0, "{second:?}");
    assert_eq!(waivers(tmp.path()).len(), 1);
}

/// A record seeded with one waiver, whose `extra` lines (indented four spaces) follow it.
fn seed_waiver(dir: &Path, extra: &str) {
    fs::write(
        dir.join(format!("specs/{FEATURE}/review.md")),
        format!(
            "---\nspec: 001-widget\nlast-run: 2026-01-01T00:00:00Z\nblocking: false\nwaivers:\n  - rule: {MUST_RULE}\n    file: src/a.rs\n    reason: kept\n    waived-at: 2026-01-01T00:00:00Z\n    waived-by: dev@example.com\n{extra}---\n\n# Review\n"
        ),
    )
    .unwrap();
}

fn classify(dir: &Path) -> crate::primitives::Result<ProcessWaiversResult> {
    process_waivers::run(
        &ProcessWaiversArgs {
            feature: FEATURE.to_string(),
            fired: vec![FiredFinding {
                rule: MUST_RULE.to_string(),
                file: "src/a.rs".to_string(),
            }],
            ..ProcessWaiversArgs::default()
        },
        dir,
    )
}

#[test]
fn a_spec_that_cannot_be_read_is_not_read_while_no_waiver_is_marked() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    seed_waiver(tmp.path(), "");
    fs::write(
        tmp.path().join(format!("specs/{FEATURE}/spec.md")),
        [0xff, 0xfe, 0xfd],
    )
    .unwrap();
    let classified = classify(tmp.path()).unwrap();
    assert_eq!(classified.applied.len(), 1, "{classified:?}");
    // A marked waiver does read it, and the failure is the read's.
    seed_waiver(tmp.path(), "    departure: true\n");
    assert!(classify(tmp.path()).is_err());
}

#[test]
fn a_hand_written_marker_is_expired_without_an_entry_and_ordinary_with_one() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), MUST_ENTRY);
    seed_waiver(tmp.path(), "    departure: true\n");
    let classified = classify(tmp.path()).unwrap();
    assert_eq!(classified.applied.len(), 1, "{classified:?}");
    assert!(classified.expired.is_empty(), "{classified:?}");

    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(""),
    );
    commit(&repository, "drop the departure");
    let classified = classify(tmp.path()).unwrap();
    assert_eq!(classified.expired.len(), 1, "{classified:?}");
    assert_eq!(
        classified.notices,
        ["waiver expired: rule BE-COMPAT-001 at src/a.rs (kept)"]
    );
}

#[test]
fn a_non_boolean_marker_fails_the_call_naming_the_field() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    seed_waiver(tmp.path(), "    departure: maybe\n");
    let err = classify(tmp.path()).unwrap_err().to_string();
    assert!(err.contains("departure"), "{err}");
}

#[test]
fn an_entry_whose_rule_fires_nowhere_stores_nothing_and_refuses_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let findings = [finding("OTHER-1", "must", "src/a.rs")];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(written.must_violations, 1, "{written:?}");
    assert_eq!(written.waived, 0);
    assert!(written.malformed_departures.is_empty(), "{written:?}");
    assert!(waivers(tmp.path()).is_empty(), "no waiver stored");
    assert!(!review_md(tmp.path()).contains("waivers:"));
}
