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

use crate::primitives::process_waivers::{self, Classified};
use crate::primitives::write_review;
use crate::schema::primitives::{
    FiredFinding, ListsForm, ProcessWaiversArgs, WriteReviewArgs, WriteReviewResult,
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

/// A repository whose one commit holds the spec with `entries`, and three source files
/// the run's window then holds: written after it, so each is changed since the commit
/// [`seed`] names.
fn fixture(dir: &Path, entries: &str) -> Repository {
    let repository = Repository::init(dir).unwrap();
    write(
        dir,
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(entries),
    );
    commit(&repository, "seed");
    // The one file outside the scope: a commit no spec claims leaves its code to the
    // project record (spec 084, AC6), and it belongs to no spec, so even a first review
    // does not read it (spec 086, AC6).
    write(dir, "src/out.rs", "fn f() {}\n");
    commit(&repository, "chore: code no spec claims");
    for file in ["a", "b", "c"] {
        write(dir, &format!("src/{file}.rs"), "fn f() {}\n");
    }
    repository
}

/// The fixture's `HEAD`, the sha a review's scope is resolved against and its
/// record's `reviewed-against` (066 scenario
/// `a-commit-after-the-scope-was-resolved-refuses-the-write`).
fn head(dir: &Path) -> String {
    let repository = Repository::open(dir).unwrap();
    let head = repository.head().unwrap().peel_to_commit().unwrap().id();
    head.to_string()
}

/// The sha of the repository's first commit, the `diff-base` every run takes.
fn seed(dir: &Path) -> String {
    let repository = Repository::open(dir).unwrap();
    let mut walk = repository.revwalk().unwrap();
    walk.set_sorting(git2::Sort::REVERSE).unwrap();
    walk.push_head().unwrap();
    walk.next().unwrap().unwrap().to_string()
}

fn finding(rule: &str, severity: &str, file: &str) -> Value {
    json!({"rule": rule, "severity": severity, "file": file, "site": "walk", "confidence": "high",
           "summary": "s", "finding": "f", "rule-text": "t"})
}

/// One review run as the host drives it: classify the recorded waivers against
/// `fired`, then record the run with what that returned.
fn run_review(
    dir: &Path,
    findings: &[Value],
    reviewed_at: &str,
    extra: &Value,
) -> (Classified, WriteReviewResult) {
    let fired = findings
        .iter()
        .map(|finding| FiredFinding {
            site: finding["site"].as_str().map(str::to_string),
            line_range: finding["line-range"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            rule: finding["rule"].as_str().unwrap().to_string(),
            file: finding["file"].as_str().unwrap().to_string(),
        })
        .collect();
    let classified = process_waivers::compute(
        &ProcessWaiversArgs {
            feature: Some(FEATURE.to_string()),
            fired,
            ..ProcessWaiversArgs::default()
        },
        dir,
    )
    .unwrap();
    let mut args = json!({
        "feature": FEATURE,
        "reviewed-at": reviewed_at,
        "reviewed-against": head(dir),
        "diff-base": seed(dir),
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

/// The files an entry covers: its anchors' for a hand waiver, its `file` for a departure
/// waiver's.
fn files_of(waiver: &Value) -> Vec<String> {
    if let Some(anchors) = waiver.get("anchors").and_then(Value::as_array) {
        return anchors
            .iter()
            .map(|anchor| anchor["file"].as_str().unwrap().to_string())
            .collect();
    }
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

/// An unscoped departure waiver covers every finding of its rule in each file, at any site and
/// whatever their number, and a site added later: the entry is the spec's decision, so it
/// carries neither a site nor a count (spec 020's `waiver-expiry`, rule 5).
#[test]
fn a_departure_waiver_covers_every_site_in_its_file() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let at = |site: &str, range: &str| {
        let mut found = finding(MUST_RULE, "must", "src/a.rs");
        found["site"] = json!(site);
        found["line-range"] = json!(range);
        found
    };
    let mut findings = vec![at("alpha", "1-2"), at("beta", "10-12"), at("alpha", "30")];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(
        (written.waived, written.must_violations),
        (3, 0),
        "{written:?}"
    );
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(files_of(&stored[0]), ["src/a.rs"]);
    assert!(stored[0].get("anchors").is_none(), "{stored:?}");

    findings.push(at("gamma", "50"));
    let (classified, written) = run_review(tmp.path(), &findings, T2, &json!({}));
    assert_eq!(classified.applied.len(), 1, "{classified:?}");
    assert!(classified.applied[0].site.is_none(), "{classified:?}");
    assert!(classified.expired.is_empty(), "{classified:?}");
    assert_eq!(
        (written.waived, written.must_violations),
        (4, 0),
        "{written:?}"
    );
}

/// A departure waiver is read by its `file`, whatever else a hand edit gave it, as
/// `write-review` refreshes it: an `anchors` list on it does not move it, so
/// `process-waivers` applies it at its file and the entry still covers a file it does not
/// list yet (spec 020's plan, A waiver's anchors).
#[test]
fn a_departure_waiver_is_read_by_its_file_whatever_else_it_holds() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    write(
        tmp.path(),
        &format!("specs/{FEATURE}/review.md"),
        &format!(
            "---\nspec: {FEATURE}\nlast-run: 2026-01-01T00:00:00Z\nblocking: false\nwaivers:\n  - rule: {MUST_RULE}\n    file: src/a.rs\n    anchors:\n      - file: src/b.rs\n        site: other\n        covers: 1\n    reason: the credential field is removed rather than versioned\n    waived-at: 2023-11-14T22:13:20Z\n    waived-by: decider@example.com\n    departure: true\n---\n\n# Review\n"
        ),
    );
    let at = |file: &str, site: &str| {
        let mut found = finding(MUST_RULE, "must", file);
        found["site"] = json!(site);
        found
    };
    let findings = [
        at("src/a.rs", "alpha"),
        at("src/a.rs", "beta"),
        at("src/b.rs", "other"),
    ];
    let (classified, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(classified.applied.len(), 1, "{classified:?}");
    assert_eq!(classified.applied[0].file, "src/a.rs", "{classified:?}");
    assert!(classified.applied[0].site.is_none(), "{classified:?}");
    assert_eq!(
        (written.waived, written.must_violations),
        (3, 0),
        "{written:?}"
    );
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
    let classified = process_waivers::compute(
        &ProcessWaiversArgs {
            feature: Some(FEATURE.to_string()),
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
        ["spec.md line 10: the entry has no `reason:`, so it waives nothing"],
        "{written:?}"
    );
    assert!(waivers(tmp.path()).is_empty(), "no waiver stored");
}

#[test]
fn an_unattributed_entry_stores_unattributed_and_the_runs_time() {
    let tmp = tempfile::tempdir().unwrap();
    // The spec is not committed: history cannot say who decided it.
    let dir = tmp.path();
    let repository = Repository::init(dir).unwrap();
    write(dir, "README.md", "seed\n");
    commit(&repository, "seed");
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
    // And the time it was first stored stays, so the record does not change run to run.
    assert_eq!(waivers(dir)[0]["waived-at"], T1);
}

#[test]
fn a_waiver_stored_unattributed_follows_its_entry_once_the_entry_is_committed() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    let repository = Repository::init(dir).unwrap();
    write(dir, "README.md", "seed\n");
    commit(&repository, "seed");
    write(
        dir,
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(MUST_ENTRY),
    );
    write(dir, "src/a.rs", "fn f() {}\n");
    let findings = [finding(MUST_RULE, "must", "src/a.rs")];
    run_review(dir, &findings, T1, &json!({}));
    assert_eq!(waivers(dir)[0]["waived-by"], "unattributed");

    commit(&repository, "decide it");
    run_review(dir, &findings, T2, &json!({}));
    let stored = waivers(dir);
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(stored[0]["waived-by"], "decider@example.com");
    assert_eq!(stored[0]["waived-at"], "2023-11-14T22:13:20Z");
}

#[test]
fn an_edited_reason_reaches_the_stored_waiver_and_the_report() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), MUST_ENTRY);
    let findings = [finding(MUST_RULE, "must", "src/a.rs")];
    run_review(tmp.path(), &findings, T1, &json!({}));
    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(
            "- departs: `BE-COMPAT-001`\n  - reason: the field is versioned in a later spec\n",
        ),
    );
    commit(&repository, "reword the reason");
    let (classified, written) = run_review(tmp.path(), &findings, T2, &json!({}));
    assert_eq!(classified.applied.len(), 1, "{classified:?}");
    assert_eq!(written.must_violations, 0, "{written:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(
        stored[0]["reason"],
        "the field is versioned in a later spec"
    );
    // The entry's introducing commit is unchanged by an edit.
    assert_eq!(stored[0]["waived-by"], "decider@example.com");
    let report = review_md(tmp.path());
    assert!(report.contains("the field is versioned in a later spec"));
    assert!(
        !report.contains("removed rather than versioned"),
        "{report}"
    );
}

/// `write-review` refuses a violation on a file outside the run's scope, writing
/// nothing, since the run did not read it (spec 086, AC1); the entry waives its rule
/// only in the scope.
fn refused_outside_the_scope(dir: &Path, findings: &[Value]) -> String {
    let args: WriteReviewArgs = serde_json::from_value(json!({
        "feature": FEATURE, "reviewed-at": T1, "reviewed-against": head(dir),
        "diff-base": seed(dir), "empty-scope": false, "findings": findings,
    }))
    .unwrap();
    let err = write_review::run(&args, dir).unwrap_err().to_string();
    assert!(
        !dir.join(format!("specs/{FEATURE}/review.md")).exists(),
        "a refusal writes nothing"
    );
    err
}

#[test]
fn a_violation_outside_the_review_scope_is_refused_and_one_inside_is_waived() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    // `src/out.rs` is committed by a commit no spec claims, so the run's scope leaves it.
    let err = refused_outside_the_scope(
        tmp.path(),
        &[
            finding(MUST_RULE, "must", "src/a.rs"),
            finding(MUST_RULE, "must", "src/out.rs"),
        ],
    );
    assert!(
        err.contains("`src/out.rs` is not in the run's scope"),
        "{err}"
    );
    let findings = [finding(MUST_RULE, "must", "src/a.rs")];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(written.must_violations, 0, "{written:?}");
    assert!(!written.blocking);
    assert_eq!(written.waived, 1);
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(files_of(&stored[0]), ["src/a.rs"]);
}

/// A finding deferred to an external work item, as a person's deferral records it.
fn deferred(rule: &str, file: &str, key: Option<&str>) -> Value {
    let mut found = finding(rule, "must", file);
    found["disposition"] = json!({"outcome": "deferred", "work-item": "JIRA-1"});
    if let Some(key) = key {
        found["decision-key"] = json!(key);
    }
    found
}

#[test]
fn a_new_deferral_of_a_violation_an_entry_decides_is_refused_naming_the_entry() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let findings = [deferred(MUST_RULE, "src/a.rs", None)];
    let args: WriteReviewArgs = serde_json::from_value(json!({
        "feature": FEATURE, "reviewed-at": T1, "reviewed-against": head(tmp.path()),
        "diff-base": seed(tmp.path()), "empty-scope": false, "findings": findings,
        "decided-by": "person@example.com",
    }))
    .unwrap();
    let err = write_review::run(&args, tmp.path())
        .unwrap_err()
        .to_string();
    assert!(err.contains("BE-COMPAT-001"), "{err}");
    assert!(err.contains("src/a.rs"), "{err}");
    assert!(err.contains("spec.md line 10"), "{err}");
    assert!(err.contains("remove the entry first"), "{err}");
    assert!(
        !tmp.path()
            .join(format!("specs/{FEATURE}/review.md"))
            .exists()
    );
}

#[test]
fn a_stored_deferral_of_a_violation_an_entry_decides_is_superseded_not_stored_again() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), "");
    let key = format!("rule {MUST_RULE} at src/a.rs in `walk`");
    // The deferral is stored while the spec holds no entry.
    let (_, written) = run_review(
        tmp.path(),
        &[deferred(MUST_RULE, "src/a.rs", None)],
        T1,
        &json!({"decided-by": "person@example.com"}),
    );
    assert!(written.superseded_deferrals.is_empty(), "{written:?}");
    assert!(review_md(tmp.path()).contains(&key));

    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(MUST_ENTRY),
    );
    commit(&repository, "decide it");
    // The run matched it to its stored decision, as a host does.
    let findings = [deferred(MUST_RULE, "src/a.rs", Some(&key))];
    for at in [T2, "2026-11-12T12:00:00Z"] {
        let (_, written) = run_review(
            tmp.path(),
            &findings,
            at,
            &json!({"decided-by": "person@example.com"}),
        );
        assert_eq!(written.must_violations, 0, "{written:?}");
        assert_eq!(written.waived, 1, "{written:?}");
        assert_eq!(written.superseded_deferrals.len(), 1, "{written:?}");
        assert!(written.superseded_deferrals[0].contains("spec.md line 10"));
        let report = review_md(tmp.path());
        assert!(!report.contains("decisions:"), "pruned: {report}");
        assert_eq!(waivers(tmp.path()).len(), 1);
    }
}

/// BE-PAYLOAD-001: `malformed-departures` and `superseded-deferrals` are inline
/// while the result fits `[artifacts] read-size-bytes`; past it both leave the
/// result for the one file `lists-file` names, with `analyze-stale-paths`, and
/// each count stays inline in both forms. A result with neither reads no
/// setting.
#[test]
fn the_departure_lists_past_the_read_size_name_a_file_holding_both_whole() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), "");
    let key = format!("rule {MUST_RULE} at src/a.rs in `walk`");
    run_review(
        tmp.path(),
        &[deferred(MUST_RULE, "src/a.rs", None)],
        T1,
        &json!({"decided-by": "person@example.com"}),
    );
    // A decided rule, and a malformed entry beside it.
    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(&format!("{MUST_ENTRY}- departs: `{SHOULD_RULE}`\n")),
    );
    commit(&repository, "decide it");
    let read_size = |bytes: u64| {
        write(
            tmp.path(),
            ".ductus/config.toml",
            &format!("[artifacts]\nread-size-bytes = {bytes}\n"),
        );
    };
    let findings = [deferred(MUST_RULE, "src/a.rs", Some(&key))];
    let extra = json!({"decided-by": "person@example.com"});

    read_size(1_000_000);
    let (_, inline) = run_review(tmp.path(), &findings, T2, &extra);
    assert_eq!(inline.form, ListsForm::Inline, "{inline:?}");
    assert_eq!(inline.lists_file, None);
    assert_eq!(inline.malformed_departures.len(), 1, "{inline:?}");
    assert_eq!(inline.superseded_deferrals.len(), 1, "{inline:?}");
    assert_eq!(inline.malformed_departures_count, 1);
    assert_eq!(inline.superseded_deferrals_count, 1);

    // The bound is on the whole result: at its compact size it fits, a byte
    // under it does not.
    let at = u64::try_from(serde_json::to_vec(&inline).unwrap().len()).unwrap();
    read_size(at);
    let (_, fits) = run_review(tmp.path(), &findings, T2, &extra);
    assert_eq!(fits.form, ListsForm::Inline, "{fits:?}");
    read_size(at - 1);
    let (_, past) = run_review(tmp.path(), &findings, T2, &extra);
    assert_eq!(past.form, ListsForm::File, "{past:?}");

    read_size(10);
    let (_, file) = run_review(tmp.path(), &findings, T2, &extra);
    assert_eq!(file.form, ListsForm::File, "{file:?}");
    assert!(file.malformed_departures.is_empty(), "{file:?}");
    assert!(file.superseded_deferrals.is_empty(), "{file:?}");
    assert_eq!(file.malformed_departures_count, 1);
    assert_eq!(file.superseded_deferrals_count, 1);
    let listed: Value = serde_json::from_str(
        &fs::read_to_string(tmp.path().join(file.lists_file.as_ref().unwrap())).unwrap(),
    )
    .unwrap();
    assert_eq!(
        listed["malformed-departures"],
        json!(inline.malformed_departures)
    );
    assert_eq!(
        listed["superseded-deferrals"],
        json!(inline.superseded_deferrals)
    );
    let wire = serde_json::to_value(&file).unwrap();
    assert!(wire.get("malformed-departures").is_none(), "{wire}");
    assert!(wire.get("superseded-deferrals").is_none(), "{wire}");
    crate::primitives::bounded_lists::assert_counts_name_the_file("write-review", &wire, &listed);

    // Either list alone is enough to move: the superseded one with no malformed
    // entry, the malformed one with no deferral.
    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(MUST_ENTRY),
    );
    let (_, only_superseded) = run_review(tmp.path(), &findings, T2, &extra);
    assert_eq!(only_superseded.form, ListsForm::File, "{only_superseded:?}");
    assert_eq!(only_superseded.superseded_deferrals_count, 1);
    assert_eq!(only_superseded.malformed_departures_count, 0);
    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(&format!("- departs: `{SHOULD_RULE}`\n")),
    );
    let (_, only_malformed) = run_review(tmp.path(), &[], T2, &json!({}));
    assert_eq!(only_malformed.form, ListsForm::File, "{only_malformed:?}");
    assert_eq!(only_malformed.malformed_departures_count, 1);
    assert_eq!(only_malformed.superseded_deferrals_count, 0);

    // Nothing to move reads no setting: a malformed read size does not halt it.
    write(
        tmp.path(),
        ".ductus/config.toml",
        "[artifacts]\nread-size-bytes = \"big\"\n",
    );
    write(
        tmp.path(),
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(MUST_ENTRY),
    );
    let (_, quiet) = run_review(tmp.path(), &[], T2, &json!({}));
    assert_eq!(quiet.form, ListsForm::Inline);
    assert_eq!(quiet.malformed_departures_count, 0);
}

#[test]
fn a_marked_waiver_with_an_entry_is_classified_as_any_other() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), MUST_ENTRY);
    run_review(
        tmp.path(),
        &[finding(MUST_RULE, "must", "src/a.rs")],
        T1,
        &json!({}),
    );
    // Committed, the files leave the window.
    commit(&repository, "the work");
    // The rule no longer fires and the run's scope did not read the file: the ordinary
    // retention applies to a marked waiver whose rule still has its entry.
    let (classified, _) = run_review(tmp.path(), &[], T2, &json!({}));
    assert!(classified.expired.is_empty(), "{classified:?}");
    assert_eq!(classified.retained.len(), 1, "{classified:?}");
    // And an unmarked waiver whose rule has no entry is untouched by the marker's test.
    fs::write(
        tmp.path().join(format!("specs/{FEATURE}/review.md")),
        "---\nspec: 001-widget\nlast-run: 2026-01-01T00:00:00Z\nblocking: false\nwaivers:\n  - rule: OTHER-1\n    anchors:\n      - file: src/b.rs\n        site: walk\n        covers: 1\n    reason: kept\n    waived-at: 2026-01-01T00:00:00Z\n    waived-by: dev@example.com\n---\n\n# Review\n",
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

/// A second entry for a rule waives nothing, even a finding the first does not decide: a
/// first entry scoped to `src/a.rs` in `walk` leaves `src/b.rs`'s finding counted, which an
/// unscoped second entry would otherwise waive.
#[test]
fn a_second_entry_for_a_rule_waives_nothing_the_first_leaves_counted() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), &format!("{SCOPED_ENTRY}{MUST_ENTRY}"));
    let findings = [
        finding(MUST_RULE, "must", "src/a.rs"),
        finding(MUST_RULE, "must", "src/b.rs"),
    ];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(written.must_violations, 1, "{written:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(
        stored[0]["reason"],
        "the guard is declined at this one check"
    );
    assert_eq!(written.malformed_departures.len(), 1, "{written:?}");
}

#[test]
fn a_run_writing_its_record_twice_counts_the_stored_departure_waived_both_times() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), MUST_ENTRY);
    let findings = [finding(MUST_RULE, "must", "src/a.rs")];
    let args: WriteReviewArgs = serde_json::from_value(json!({
        "feature": FEATURE, "reviewed-at": T1, "reviewed-against": head(tmp.path()),
        "diff-base": seed(tmp.path()), "empty-scope": false, "findings": findings,
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

/// A record seeded with one waiver, whose `extra` lines (indented four spaces) follow it: a
/// hand waiver anchored at `walk` of `src/a.rs`, or, when `extra` marks it a departure
/// waiver, one listing the file.
fn seed_waiver(dir: &Path, extra: &str) {
    let anchored = if extra.contains("departure") {
        "file: src/a.rs"
    } else {
        "anchors:\n      - file: src/a.rs\n        site: walk\n        covers: 1"
    };
    fs::write(
        dir.join(format!("specs/{FEATURE}/review.md")),
        format!(
            "---\nspec: 001-widget\nlast-run: 2026-01-01T00:00:00Z\nblocking: false\nwaivers:\n  - rule: {MUST_RULE}\n    {anchored}\n    reason: kept\n    waived-at: 2026-01-01T00:00:00Z\n    waived-by: dev@example.com\n{extra}---\n\n# Review\n"
        ),
    )
    .unwrap();
}

fn classify(dir: &Path) -> crate::primitives::Result<Classified> {
    process_waivers::compute(
        &ProcessWaiversArgs {
            feature: Some(FEATURE.to_string()),
            fired: vec![FiredFinding {
                site: Some("walk".to_string()),
                line_range: String::new(),
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

// -- scoped entries (spec 073 §Scoped entries: AC10, AC11, AC13) --------------

/// A scoped entry covering `src/a.rs` in `walk`, the site [`finding`] names.
const SCOPED_ENTRY: &str = "- departs: `BE-COMPAT-001`\n  - reason: the guard is declined at this one check\n  - covers:\n    - `src/a.rs` in `walk`\n";
const T3: &str = "2026-11-12T12:00:00Z";

/// A MUST finding of [`MUST_RULE`] at `file`, in `site` or naming none.
fn at_site(file: &str, site: Option<&str>) -> Value {
    let mut found = finding(MUST_RULE, "must", file);
    found["site"] = site.map_or(Value::Null, |site| json!(site));
    found
}

/// The `(file, site)` of each anchor a stored waiver holds.
fn sites_of(waiver: &Value) -> Vec<(String, String)> {
    waiver["anchors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|anchor| {
            (
                anchor["file"].as_str().unwrap().to_string(),
                anchor["site"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

fn site(file: &str, site: &str) -> (String, String) {
    (file.to_string(), site.to_string())
}

/// Rewrite the spec with `entries` and commit it.
fn decide(dir: &Path, repository: &Repository, entries: &str) {
    write(
        dir,
        &format!("specs/{FEATURE}/spec.md"),
        &spec_with(entries),
    );
    commit(repository, "decide it");
}

/// AC10: the covered site is waived, and the rule's findings at another site of the same
/// file, naming no site, or in another file count, on a first run and on one that finds
/// the waiver stored.
#[test]
fn a_scoped_entry_waives_its_rule_at_its_sites_only() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), SCOPED_ENTRY);
    let findings = [
        at_site("src/a.rs", Some("walk")),
        at_site("src/a.rs", Some("other")),
        at_site("src/a.rs", None),
        at_site("src/b.rs", Some("walk")),
    ];
    for at in [T1, T2] {
        let (classified, written) = run_review(tmp.path(), &findings, at, &json!({}));
        assert_eq!(
            (written.waived, written.must_violations),
            (1, 3),
            "{written:?}"
        );
        assert!(written.blocking);
        if at == T2 {
            // The stored waiver is read by its site anchor and applied there alone.
            assert_eq!(classified.applied.len(), 1, "{classified:?}");
            assert_eq!(classified.applied[0].site.as_deref(), Some("walk"));
        }
        let stored = waivers(tmp.path());
        assert_eq!(stored.len(), 1, "{stored:?}");
        assert_eq!(stored[0]["departure"], true);
        assert!(stored[0].get("file").is_none(), "{stored:?}");
        assert_eq!(sites_of(&stored[0]), [site("src/a.rs", "walk")]);
        assert!(
            stored[0]["anchors"][0].get("covers").is_none(),
            "{stored:?}"
        );
    }
}

fn deferred_at(file: &str, site: &str, key: Option<&str>) -> Value {
    let mut found = deferred(MUST_RULE, file, key);
    found["site"] = json!(site);
    found
}

fn key_at(site: &str) -> String {
    format!("rule {MUST_RULE} at src/a.rs in `{site}`")
}

/// AC11: a new deferral at a site the entry does not cover is stored, and one at a site
/// it covers is refused, naming the entry.
#[test]
fn a_scoped_entry_refuses_a_new_deferral_only_at_a_site_it_covers() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), SCOPED_ENTRY);
    let decided_by = json!({"decided-by": "person@example.com"});
    let (_, written) = run_review(
        tmp.path(),
        &[deferred_at("src/a.rs", "other", None)],
        T1,
        &decided_by,
    );
    assert!(written.superseded_deferrals.is_empty(), "{written:?}");
    assert!(review_md(tmp.path()).contains(&key_at("other")));

    let args: WriteReviewArgs = serde_json::from_value(json!({
        "feature": FEATURE, "reviewed-at": T2, "reviewed-against": head(tmp.path()),
        "diff-base": seed(tmp.path()), "empty-scope": false,
        "findings": [deferred_at("src/a.rs", "walk", None)],
        "decided-by": "person@example.com",
    }))
    .unwrap();
    let err = write_review::run(&args, tmp.path())
        .unwrap_err()
        .to_string();
    assert!(err.contains("spec.md line 10"), "{err}");
    assert!(err.contains("remove the entry first"), "{err}");
}

/// AC11: once a scoped entry is committed, a stored deferral at its site is superseded
/// and one at another site stands.
#[test]
fn a_scoped_entry_supersedes_a_stored_deferral_only_at_a_site_it_covers() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), "");
    let decided_by = json!({"decided-by": "person@example.com"});
    let (_, written) = run_review(
        tmp.path(),
        &[
            deferred_at("src/a.rs", "walk", None),
            deferred_at("src/a.rs", "other", None),
        ],
        T1,
        &decided_by,
    );
    assert!(written.superseded_deferrals.is_empty(), "{written:?}");

    decide(tmp.path(), &repository, SCOPED_ENTRY);
    let findings = [
        deferred_at("src/a.rs", "walk", Some(&key_at("walk"))),
        deferred_at("src/a.rs", "other", Some(&key_at("other"))),
    ];
    let (_, written) = run_review(tmp.path(), &findings, T2, &decided_by);
    assert_eq!(written.superseded_deferrals.len(), 1, "{written:?}");
    assert_eq!(written.waived, 1, "{written:?}");
    let report = review_md(tmp.path());
    assert!(!report.contains(&key_at("walk")), "superseded: {report}");
    assert!(report.contains(&key_at("other")), "stands: {report}");
}

/// AC13: a malformed `covers:`, in each form AC13 names, derives no waiver, and the run
/// names the entry.
#[test]
fn a_malformed_covers_waives_nothing_and_the_run_names_it() {
    // Not a list, empty, a blank file, a blank site, a repeated item, an absolute file,
    // and a file holding `..`.
    let forms = [
        ("  - covers: `src/a.rs` in `walk`\n", "is not a list"),
        ("  - covers:\n", "lists no site"),
        (
            "  - covers:\n    - ` ` in `walk`\n",
            "not two non-blank code spans",
        ),
        (
            "  - covers:\n    - `src/a.rs` in ` `\n",
            "not two non-blank code spans",
        ),
        (
            "  - covers:\n    - `src/a.rs` in `walk`\n    - `src/a.rs` in `walk`\n",
            "twice",
        ),
        (
            "  - covers:\n    - `/src/a.rs` in `walk`\n",
            "absolute or holds",
        ),
        (
            "  - covers:\n    - `src/../src/a.rs` in `walk`\n",
            "absolute or holds",
        ),
    ];
    for (covers, phrase) in forms {
        let tmp = tempfile::tempdir().unwrap();
        fixture(
            tmp.path(),
            &format!("- departs: `BE-COMPAT-001`\n  - reason: r\n{covers}"),
        );
        let (_, written) = run_review(
            tmp.path(),
            &[at_site("src/a.rs", Some("walk"))],
            T1,
            &json!({}),
        );
        assert_eq!(
            (written.waived, written.must_violations),
            (0, 1),
            "{covers}: {written:?}"
        );
        assert_eq!(
            written.malformed_departures.len(),
            1,
            "{covers}: {written:?}"
        );
        assert!(
            written.malformed_departures[0].contains(phrase),
            "{covers}: {:?} lacks {phrase}",
            written.malformed_departures
        );
        assert!(waivers(tmp.path()).is_empty(), "{covers}: no waiver stored");
    }
}

/// An item dropped from `covers:` stops waiving its site at once, with the expiry notice,
/// and the entry's edited reason reaches the stored waiver and the report.
#[test]
fn a_dropped_covers_item_expires_its_anchor_and_an_edited_reason_reaches_the_report() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(
        tmp.path(),
        "- departs: `BE-COMPAT-001`\n  - reason: declined at both\n  - covers:\n    - `src/a.rs` in `walk`\n    - `src/a.rs` in `other`\n",
    );
    let findings = [
        at_site("src/a.rs", Some("walk")),
        at_site("src/a.rs", Some("other")),
    ];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(
        (written.waived, written.must_violations),
        (2, 0),
        "{written:?}"
    );

    decide(
        tmp.path(),
        &repository,
        "- departs: `BE-COMPAT-001`\n  - reason: declined at walk alone\n  - covers:\n    - `src/a.rs` in `walk`\n",
    );
    let (classified, written) = run_review(tmp.path(), &findings, T2, &json!({}));
    assert_eq!(classified.expired.len(), 1, "{classified:?}");
    assert_eq!(classified.expired[0].site.as_deref(), Some("other"));
    assert!(
        classified
            .notices
            .iter()
            .any(|notice| notice
                .starts_with("waiver expired: rule BE-COMPAT-001 at src/a.rs in other")),
        "{:?}",
        classified.notices
    );
    assert_eq!(
        (written.waived, written.must_violations),
        (1, 1),
        "{written:?}"
    );
    let stored = waivers(tmp.path());
    assert_eq!(sites_of(&stored[0]), [site("src/a.rs", "walk")]);
    assert_eq!(stored[0]["reason"], "declined at walk alone");
    let report = review_md(tmp.path());
    assert!(report.contains("declined at walk alone"), "{report}");
    assert!(!report.contains("declined at both"), "{report}");
}

/// An entry switched from unscoped to scoped, and back, expires the waiver stored in the
/// old form with its notice, and the run stores the entry's current form.
#[test]
fn switching_an_entry_between_scoped_and_unscoped_expires_the_stored_form() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), MUST_ENTRY);
    let findings = [
        at_site("src/a.rs", Some("walk")),
        at_site("src/a.rs", Some("other")),
    ];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    assert_eq!(written.waived, 2, "{written:?}");
    assert_eq!(files_of(&waivers(tmp.path())[0]), ["src/a.rs"]);

    decide(tmp.path(), &repository, SCOPED_ENTRY);
    let (classified, written) = run_review(tmp.path(), &findings, T2, &json!({}));
    assert_eq!(classified.expired.len(), 1, "{classified:?}");
    assert!(classified.expired[0].site.is_none(), "{classified:?}");
    assert!(
        classified
            .notices
            .iter()
            .any(|notice| notice.starts_with("waiver expired: rule BE-COMPAT-001 at src/a.rs (")),
        "{:?}",
        classified.notices
    );
    assert_eq!(
        (written.waived, written.must_violations),
        (1, 1),
        "{written:?}"
    );
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert!(stored[0].get("file").is_none(), "{stored:?}");
    assert_eq!(sites_of(&stored[0]), [site("src/a.rs", "walk")]);

    decide(tmp.path(), &repository, MUST_ENTRY);
    let (classified, written) = run_review(tmp.path(), &findings, T3, &json!({}));
    assert_eq!(classified.expired.len(), 1, "{classified:?}");
    assert_eq!(classified.expired[0].site.as_deref(), Some("walk"));
    assert_eq!(
        (written.waived, written.must_violations),
        (2, 0),
        "{written:?}"
    );
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert!(stored[0].get("anchors").is_none(), "{stored:?}");
    assert_eq!(files_of(&stored[0]), ["src/a.rs"]);
}

/// A run's two writes carry one set of arguments, so the second finds the scoped waiver
/// stored and stores the site once.
#[test]
fn a_run_writing_its_record_twice_stores_a_scoped_entrys_site_once() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), SCOPED_ENTRY);
    let findings = [at_site("src/a.rs", Some("walk"))];
    let args: WriteReviewArgs = serde_json::from_value(json!({
        "feature": FEATURE, "reviewed-at": T1, "reviewed-against": head(tmp.path()),
        "diff-base": seed(tmp.path()), "empty-scope": false, "findings": findings,
    }))
    .unwrap();
    let first = write_review::run(&args, tmp.path()).unwrap();
    let second = write_review::run(&args, tmp.path()).unwrap();
    assert_eq!(first.must_violations, 0, "{first:?}");
    assert_eq!(second.must_violations, 0, "{second:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(sites_of(&stored[0]), [site("src/a.rs", "walk")]);
}

// -- lost coverage (spec 073 AC12) -------------------------------------------

/// AC12: a covered file not in the tree is named, with the entry's line and its sites.
#[test]
fn a_covered_file_not_in_the_tree_is_a_notice_naming_the_entry_and_its_sites() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(
        tmp.path(),
        "- departs: `BE-COMPAT-001`\n  - reason: r\n  - covers:\n    - `src/gone.rs` in `walk`\n    - `src/gone.rs` in `(top level)`\n",
    );
    let (_, written) = run_review(tmp.path(), &[], T1, &json!({}));
    assert_eq!(written.departure_notices.len(), 1, "{written:?}");
    let notice = &written.departure_notices[0];
    assert!(notice.contains("spec.md line 10"), "{notice}");
    assert!(
        notice.contains("src/gone.rs in `walk`, `(top level)`"),
        "{notice}"
    );
    assert!(notice.contains("not in the tree"), "{notice}");
    assert_eq!(written.departure_notices_count, 1);
}

/// AC12: the rule firing in a covered file at a site the entry does not list, or naming
/// none, is named with the site it fires at, once per site; a finding in a file the entry
/// does not cover is no notice.
#[test]
fn the_rule_firing_at_an_uncovered_site_of_a_covered_file_is_a_notice_naming_both_sites() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), SCOPED_ENTRY);
    let findings = [
        at_site("src/a.rs", Some("walk")),
        at_site("src/a.rs", Some("other")),
        at_site("src/a.rs", Some("other")),
        at_site("src/a.rs", None),
        at_site("src/b.rs", Some("other")),
    ];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({}));
    let notices = &written.departure_notices;
    assert_eq!(notices.len(), 2, "{notices:?}");
    for notice in notices {
        assert!(notice.contains("spec.md line 10"), "{notice}");
        assert!(notice.contains("covers src/a.rs in `walk`"), "{notice}");
        assert!(notice.contains("which it does not cover"), "{notice}");
    }
    assert!(notices[0].contains("fires there at `other`"), "{notices:?}");
    assert!(notices[1].contains("fires there at no site"), "{notices:?}");
}

/// AC12: a covered site where the rule does not fire, in a file in the tree, is no notice.
#[test]
fn a_covered_site_where_the_rule_does_not_fire_is_no_notice() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path(), SCOPED_ENTRY);
    let (_, written) = run_review(
        tmp.path(),
        &[at_site("src/b.rs", Some("walk"))],
        T1,
        &json!({}),
    );
    assert_eq!(written.departure_notices, [] as [String; 0], "{written:?}");
    assert_eq!(written.departure_notices_count, 0);
}

/// Spec 073's Edge Cases: the rule firing in a covered file the tree holds but the run's
/// scope does not is not the run's to record, since the run did not read that file:
/// `write-review` refuses it, writing nothing (spec 086, AC1). The rule firing in a covered
/// file the run's own window carries in, which the run does record, is the case below.
#[test]
fn the_rule_firing_in_a_covered_file_outside_the_scope_is_refused() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(
        tmp.path(),
        "- departs: `BE-COMPAT-001`\n  - reason: r\n  - covers:\n    - `src/out.rs` in `walk`\n",
    );
    let err = refused_outside_the_scope(tmp.path(), &[at_site("src/out.rs", Some("other"))]);
    assert!(
        err.contains("`src/out.rs` is not in the run's scope"),
        "{err}"
    );
}

/// Commit a full review's record that left `src/out.rs` unread, then a later run's record,
/// at `HEAD`, that skipped a pass and left nothing unread. The window still starts where the
/// full review ended, so the default window carries `src/out.rs` in from the record it
/// starts at, while the scope resolved from that same base as `--since` reads only the
/// record at `HEAD` (spec 079, AC30). `src/a.rs` changes in the window. Returns the base.
fn carried_by_the_window_start(dir: &Path, repository: &Repository) -> String {
    let record = |last_run: &str, extra: &str| {
        format!(
            "---\nspec: {FEATURE}\nlast-run: {last_run}\nmust-violations: 0\n\
             should-violations: 0\nblocking: false\ndispositions:\n  fixed: 0\n  deferred: 0\n  \
             waived: 0\n  undispositioned: 0\n{extra}---\n\n# Review — {FEATURE}\n"
        )
    };
    let rel = format!("specs/{FEATURE}/review.md");
    write(
        dir,
        &rel,
        &record("2026-11-10T00:00:00Z", "unread:\n  - src/out.rs\n"),
    );
    commit(repository, "review(001): a run that left src/out.rs unread");
    write(dir, "src/a.rs", "fn g() {}\n");
    write(
        dir,
        &rel,
        &record(
            "2026-11-11T00:00:00Z",
            "skipped-passes:\n  - quality\nunread: []\n",
        ),
    );
    commit(repository, "review(001): a run that skipped a pass");
    let scope = crate::primitives::compute_review_scope::compute(
        &crate::schema::primitives::ComputeReviewScopeArgs {
            feature: FEATURE.to_string(),
            since: None,
        },
        dir,
    )
    .unwrap();
    assert!(
        scope.scope.iter().any(|path| path == "src/out.rs"),
        "the default window carries src/out.rs in: {:?}",
        scope.scope
    );
    scope.diff_base
}

/// A finding on a path the run's own window carries in, one the record the window starts
/// at left unread, is the run's to record, so `write-review` accepts it (spec 086, AC1). A
/// departure entry does not waive it: the entry decides only in the scope resolved from the
/// diff base as `--since`, which leaves out what that record handed on, so the finding stays
/// counted while the one in the window is waived (spec 020's `waiver-expiry`; 086's plan,
/// *Departures are unchanged*).
#[test]
fn a_violation_on_a_path_the_window_carries_in_is_accepted_and_still_counts() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(tmp.path(), MUST_ENTRY);
    let base = carried_by_the_window_start(tmp.path(), &repository);
    let findings = [
        finding(MUST_RULE, "must", "src/a.rs"),
        finding(MUST_RULE, "must", "src/out.rs"),
    ];
    let (_, written) = run_review(tmp.path(), &findings, T1, &json!({"diff-base": base}));
    assert_eq!(written.must_violations, 1, "{written:?}");
    assert!(written.blocking);
    assert_eq!(written.waived, 1, "{written:?}");
    let stored = waivers(tmp.path());
    assert_eq!(stored.len(), 1, "{stored:?}");
    assert_eq!(files_of(&stored[0]), ["src/a.rs"]);
}

/// Spec 073's Edge Cases: the rule firing, at a site the entry does not cover, in a covered
/// file the run's own window carries in is no notice, since the entry's scope does not hold
/// that file; the finding is accepted and stays counted (spec 086, AC1).
#[test]
fn the_rule_firing_in_a_covered_file_the_window_carries_in_is_no_notice() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = fixture(
        tmp.path(),
        "- departs: `BE-COMPAT-001`\n  - reason: r\n  - covers:\n    - `src/out.rs` in `walk`\n",
    );
    let base = carried_by_the_window_start(tmp.path(), &repository);
    let (_, written) = run_review(
        tmp.path(),
        &[at_site("src/out.rs", Some("other"))],
        T1,
        &json!({"diff-base": base}),
    );
    assert_eq!(written.departure_notices, [] as [String; 0], "{written:?}");
    assert_eq!(
        (written.waived, written.must_violations),
        (0, 1),
        "{written:?}"
    );
    assert_eq!(waivers(tmp.path()), [] as [Value; 0]);
}

/// Spec 073's Edge Cases and `BE-INPUT-004`: a covered file reached through a symbolic
/// link leaving the project is named and never probed, so the notice is the same whether
/// or not the link's target holds the file.
#[cfg(unix)]
#[test]
fn a_covered_file_leaving_the_project_is_a_notice_and_never_probed() {
    let outside = tempfile::tempdir().unwrap();
    fs::write(outside.path().join("present.rs"), "fn f() {}\n").unwrap();
    let tmp = tempfile::tempdir().unwrap();
    fixture(
        tmp.path(),
        "- departs: `BE-COMPAT-001`\n  - reason: r\n  - covers:\n    - `src/link/present.rs` in `walk`\n    - `src/link/absent.rs` in `walk`\n",
    );
    std::os::unix::fs::symlink(outside.path(), tmp.path().join("src/link")).unwrap();
    let (_, written) = run_review(tmp.path(), &[], T1, &json!({}));
    let notices = &written.departure_notices;
    assert_eq!(notices.len(), 2, "{notices:?}");
    for notice in notices {
        assert!(notice.contains("spec.md line 10"), "{notice}");
        assert!(
            notice.contains("leaves the project, so it is never probed"),
            "{notice}"
        );
    }
}

/// BE-PAYLOAD-001: `departure-notices` alone is enough to move the lists past the read
/// size, held whole in the one file `lists-file` names with its count inline.
#[test]
fn departure_notices_past_the_read_size_move_to_the_list_file() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(
        tmp.path(),
        "- departs: `BE-COMPAT-001`\n  - reason: r\n  - covers:\n    - `src/gone.rs` in `walk`\n",
    );
    let (_, inline) = run_review(tmp.path(), &[], T1, &json!({}));
    assert_eq!(inline.form, ListsForm::Inline, "{inline:?}");
    assert_eq!(inline.departure_notices.len(), 1, "{inline:?}");
    write(
        tmp.path(),
        ".ductus/config.toml",
        "[artifacts]\nread-size-bytes = 10\n",
    );
    let (_, file) = run_review(tmp.path(), &[], T2, &json!({}));
    assert_eq!(file.form, ListsForm::File, "{file:?}");
    assert!(file.departure_notices.is_empty(), "{file:?}");
    assert_eq!(file.departure_notices_count, 1);
    let listed: Value = serde_json::from_str(
        &fs::read_to_string(tmp.path().join(file.lists_file.as_ref().unwrap())).unwrap(),
    )
    .unwrap();
    assert_eq!(listed["departure-notices"], json!(inline.departure_notices));
    let wire = serde_json::to_value(&file).unwrap();
    assert!(wire.get("departure-notices").is_none(), "{wire}");
    crate::primitives::bounded_lists::assert_counts_name_the_file("write-review", &wire, &listed);
}
