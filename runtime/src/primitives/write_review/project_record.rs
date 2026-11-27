//! The project record (spec 084): `write-review` with no `feature` writes
//! `{specs-root}/review.md`, and `seed` writes the record no review wrote.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;

use git2::{Oid, Repository};

use super::run;
use crate::primitives::git_fixture::{commit_all, write};
use crate::primitives::{PrimitiveError, RecordLoad, load_review_record};
use crate::schema::primitives::{
    Disposition, DispositionOutcome, RecordFreshness, ReviewFinding, SeedOutcome, WriteReviewArgs,
};
use crate::schema::severity::ReviewSeverity;

const RECORD: &str = "specs/review.md";

fn seed_args() -> WriteReviewArgs {
    WriteReviewArgs {
        seed: true,
        ..WriteReviewArgs::default()
    }
}

/// A project with spec 001 in history, seeded, with one commit no spec claims
/// after the seed's. Returns the repository and the commit that added the seed.
fn seeded_project(root: &Path) -> (Repository, Oid) {
    let repository = Repository::init(root).unwrap();
    write(
        &root.join("specs/001-a/spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# A\n\n## Acceptance Criteria\n\n- [ ] AC1: a\n",
    );
    write(
        &root.join("specs/001-a/tasks.md"),
        "# Tasks\n\n## 1. A task\n\n- [ ] do it\n\n- **Done when**: done.\n",
    );
    commit_all(&repository, "spec(001): add");
    run(&seed_args(), root).unwrap();
    write(&root.join("hooks/pre-push"), "#!/bin/sh\n");
    let added = commit_all(&repository, "chore: install");
    write(&root.join("src/a.rs"), "fn a() {}\n");
    commit_all(&repository, "chore: tidy");
    (repository, added)
}

fn head(repository: &Repository) -> String {
    repository.head().unwrap().target().unwrap().to_string()
}

/// A project review's arguments over the window after `base`.
fn review_args(repository: &Repository, base: Oid) -> WriteReviewArgs {
    WriteReviewArgs {
        reviewed_at: "2026-11-24T00:00:00Z".into(),
        reviewed_against: head(repository),
        diff_base: base.to_string(),
        examined: Some(1),
        unread: Some(Vec::new()),
        ..WriteReviewArgs::default()
    }
}

fn must(file: &str, disposition: Disposition) -> ReviewFinding {
    ReviewFinding {
        site: Some("walk".into()),
        rule: "BE-INPUT-001".into(),
        severity: ReviewSeverity::Must,
        file: file.into(),
        line_range: "1-1".into(),
        confidence: "high".into(),
        summary: "a summary".into(),
        finding: "a finding".into(),
        rule_text: "the rule".into(),
        auto_fixable: false,
        suggested_fix: String::new(),
        disposition,
        decision_key: None,
    }
}

fn record_text(root: &Path) -> String {
    std::fs::read_to_string(root.join(RECORD)).unwrap()
}

fn is_invalid(err: &PrimitiveError, named: &str) -> bool {
    matches!(err, PrimitiveError::InvalidArgument { argument, .. } if argument == named)
}

/// AC6: with no `feature` the review of a fixture's unclaimed commit is
/// recorded in `{specs-root}/review.md`, in the review record's schema, with
/// no criteria and no contracts digest, and with the window's start it used.
#[test]
fn a_project_review_records_the_unclaimed_commit_in_the_spec_roots_record() {
    let tmp = tempfile::tempdir().unwrap();
    let (repository, added) = seeded_project(tmp.path());
    let result = run(&review_args(&repository, added), tmp.path()).unwrap();
    assert_eq!(result.path, RECORD);
    assert_eq!(result.spec_path, None);
    assert_eq!(result.seed, None);
    assert_eq!(
        result.scope, 1,
        "src/a.rs, and not the install's hooks/pre-push"
    );
    assert_eq!(result.analyze_freshness, RecordFreshness::NeverRun);
    assert_eq!(result.criteria_judged, None);

    let text = record_text(tmp.path());
    assert!(text.starts_with("---\nspec: project\nlast-run: 2026-11-24T00:00:00Z\n"));
    assert!(text.contains(&format!("\ndiff-base: {added}\n")));
    assert!(text.contains("\n# Review — project\n"));
    assert!(text.contains("1 of 1 commits in the window no spec claimed."));
    for absent in ["reviewed-digest", "criteria-", "scenario:"] {
        assert!(
            !text.contains(absent),
            "the project record states no {absent}"
        );
    }
    let RecordLoad::Present(record) = load_review_record(&tmp.path().join("specs")) else {
        panic!("the project record parses as a review record");
    };
    assert_eq!(record.last_run.as_deref(), Some("2026-11-24T00:00:00Z"));
    assert_eq!(record.reviewed_against, Some(head(&repository)));
    assert!(!record.blocking);
    // Nothing was written under a directory named for the subject.
    assert!(!tmp.path().join("specs/project").exists());
    // The feature's own record is untouched: none was written.
    assert!(!tmp.path().join("specs/001-a/review.md").exists());
}

/// The project record's write refuses an outcome outside its scope as a spec's does:
/// the install's `hooks/pre-push`, before the window, is refused and nothing is
/// written, and `src/a.rs`, which the window holds, is recorded (spec 086, AC1).
#[test]
fn a_project_review_refuses_an_outcome_outside_its_scope() {
    let tmp = tempfile::tempdir().unwrap();
    let (repository, added) = seeded_project(tmp.path());
    let before = record_text(tmp.path());
    let mut args = review_args(&repository, added);
    args.findings = vec![must("hooks/pre-push", Disposition::default())];
    let err = run(&args, tmp.path()).unwrap_err();
    assert!(is_invalid(&err, "findings[0].file"), "{err}");
    assert!(
        err.to_string()
            .contains("`hooks/pre-push` is not in the run's scope"),
        "{err}"
    );
    assert_eq!(record_text(tmp.path()), before, "a refusal writes nothing");
    args.findings = vec![must("src/a.rs", Disposition::default())];
    let result = run(&args, tmp.path()).unwrap();
    assert_eq!(result.must_violations, 1, "{result:?}");
}

/// What only a spec has is refused for the project, never dropped, and each
/// refusal leaves the seed as it was.
#[test]
fn a_project_review_refuses_what_only_a_spec_has_and_an_empty_base() {
    let tmp = tempfile::tempdir().unwrap();
    let (repository, added) = seeded_project(tmp.path());
    let before = record_text(tmp.path());
    let base = review_args(&repository, added);
    let cases = [
        (
            "criteria",
            WriteReviewArgs {
                criteria: Some(Vec::new()),
                ..base.clone()
            },
        ),
        (
            "scenario",
            WriteReviewArgs {
                scenario: Some("a-scenario".into()),
                ..base.clone()
            },
        ),
        (
            "diff-base",
            WriteReviewArgs {
                diff_base: String::new(),
                ..base.clone()
            },
        ),
        (
            "diff-base",
            WriteReviewArgs {
                diff_base: "no-such-commit".into(),
                ..base.clone()
            },
        ),
        (
            "unread",
            WriteReviewArgs {
                unread: Some(vec!["hooks/pre-push".into()]),
                ..base.clone()
            },
        ),
    ];
    for (argument, args) in cases {
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(is_invalid(&err, argument), "{argument}: {err}");
        assert_eq!(record_text(tmp.path()), before, "{argument} wrote nothing");
    }
}

/// The moved-scope guard holds for the project: a commit after
/// `reviewed-against` that changed a path in the scope refuses the write.
#[test]
fn a_project_review_refuses_a_scope_that_moved_after_it_was_read() {
    let tmp = tempfile::tempdir().unwrap();
    let (repository, added) = seeded_project(tmp.path());
    let args = review_args(&repository, added);
    write(&tmp.path().join("src/a.rs"), "fn a() { 1 }\n");
    commit_all(&repository, "chore: tidy again");
    let err = run(&args, tmp.path()).unwrap_err();
    assert!(is_invalid(&err, "reviewed-against"), "{err}");
}

/// Only an absent `feature` is the project: an empty one is refused, and
/// writes no project record.
#[test]
fn an_empty_feature_is_refused_and_is_not_the_project() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = Repository::init(tmp.path()).unwrap();
    write(&tmp.path().join("src/a.rs"), "fn a() {}\n");
    let base = commit_all(&repository, "chore: first");
    let args = WriteReviewArgs {
        feature: Some(String::new()),
        ..review_args(&repository, base)
    };
    let err = run(&args, tmp.path()).unwrap_err();
    assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err}");
    assert!(!tmp.path().join(RECORD).exists());
}

/// A deferral to a spec's pending work releases a project finding, since the
/// project record has no work of its own: every spec is another.
#[test]
fn a_deferral_to_a_specs_pending_task_releases_a_project_finding() {
    let tmp = tempfile::tempdir().unwrap();
    let (repository, added) = seeded_project(tmp.path());
    let deferred = Disposition {
        outcome: DispositionOutcome::Deferred,
        target: Some("specs/001-a/tasks.md".into()),
        task: Some("1".into()),
        ..Disposition::default()
    };
    let args = WriteReviewArgs {
        findings: vec![must("src/a.rs", deferred)],
        decided_by: Some("the operator".into()),
        ..review_args(&repository, added)
    };
    let result = run(&args, tmp.path()).unwrap();
    assert_eq!(result.must_violations, 0);
    assert!(!result.blocking);
    // Undeferred, the same finding counts and blocks.
    let args = WriteReviewArgs {
        findings: vec![must("src/a.rs", Disposition::default())],
        ..review_args(&repository, added)
    };
    let result = run(&args, tmp.path()).unwrap();
    assert_eq!(result.must_violations, 1);
    assert!(result.blocking);
}

/// The project record keeps its waiver list from run to run, as a feature's does.
#[test]
fn a_project_review_keeps_the_records_waivers() {
    let tmp = tempfile::tempdir().unwrap();
    let (repository, added) = seeded_project(tmp.path());
    write(
        &tmp.path().join(RECORD),
        "---\nspec: project\nlast-run: null\nmust-violations: 0\nshould-violations: 0\n\
         low-confidence: 0\nblocking: false\nwaivers:\n  - rule: BE-INPUT-001\n    anchors:\n      \
         - file: src/a.rs\n        site: walk\n        covers: 1\n    reason: \"a reason long enough\"\n    \
         waived-at: 2026-11-01T00:00:00Z\n    waived-by: the operator\n---\n\n# Review — project\n",
    );
    commit_all(&repository, "chore: a waiver");
    run(&review_args(&repository, added), tmp.path()).unwrap();
    let text = record_text(tmp.path());
    assert!(text.contains("rule: BE-INPUT-001"), "{text}");
    assert!(text.contains("a reason long enough"));
}

/// The seed: `last-run: null`, `reviewed-against` at `HEAD` as the runtime
/// read it, and a Summary saying no review ran. It parses as a review record.
#[test]
fn seed_writes_a_record_no_review_wrote() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = Repository::init(tmp.path()).unwrap();
    write(&tmp.path().join("src/a.rs"), "fn a() {}\n");
    commit_all(&repository, "chore: first");
    let result = run(&seed_args(), tmp.path()).unwrap();
    assert_eq!(result.seed, Some(SeedOutcome::Written));
    assert_eq!(result.path, RECORD);
    let text = record_text(tmp.path());
    let at = head(&repository);
    assert!(text.starts_with(&format!(
        "---\nspec: project\nlast-run: null\nreviewed-against: {at}\n"
    )));
    assert!(text.contains("No review ran"));
    assert!(text.contains(&format!("before `{at}` was not reviewed")));
    let RecordLoad::Present(record) = load_review_record(&tmp.path().join("specs")) else {
        panic!("the seed parses as a review record");
    };
    assert_eq!(record.last_run, None);
    assert_eq!(record.reviewed_against, Some(at));
    assert!(!record.blocking);
}

/// A second seed writes nothing, over a seed and over a review's record alike.
#[test]
fn seed_leaves_an_existing_record_as_it_is() {
    let tmp = tempfile::tempdir().unwrap();
    let (repository, added) = seeded_project(tmp.path());
    let seeded = record_text(tmp.path());
    let again = run(&seed_args(), tmp.path()).unwrap();
    assert_eq!(again.seed, Some(SeedOutcome::Kept));
    assert_eq!(again.path, RECORD);
    assert_eq!(record_text(tmp.path()), seeded);

    run(&review_args(&repository, added), tmp.path()).unwrap();
    let reviewed = record_text(tmp.path());
    assert_ne!(reviewed, seeded);
    assert_eq!(
        run(&seed_args(), tmp.path()).unwrap().seed,
        Some(SeedOutcome::Kept)
    );
    assert_eq!(record_text(tmp.path()), reviewed);
}

/// A seed takes no other argument: a feature, a run's field or a finding
/// refuses it, and nothing is written.
#[test]
fn seed_refuses_any_other_argument() {
    let tmp = tempfile::tempdir().unwrap();
    let repository = Repository::init(tmp.path()).unwrap();
    write(
        &tmp.path().join("specs/001-a/spec.md"),
        "---\nstatus: draft\ndependencies: []\n---\n",
    );
    commit_all(&repository, "spec(001): add");
    let cases = [
        WriteReviewArgs {
            feature: Some("001-a".into()),
            ..seed_args()
        },
        WriteReviewArgs {
            reviewed_at: "2026-11-24T00:00:00Z".into(),
            ..seed_args()
        },
        WriteReviewArgs {
            findings: vec![must("src/a.rs", Disposition::default())],
            ..seed_args()
        },
        WriteReviewArgs {
            empty_scope: true,
            ..seed_args()
        },
    ];
    for args in cases {
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(is_invalid(&err, "seed"), "{err}");
        assert!(!tmp.path().join(RECORD).exists());
        assert!(!tmp.path().join("specs/001-a/review.md").exists());
    }
}

/// In a repository with no commit the seed is written without
/// `reviewed-against`, which nothing reads, and says no commit existed.
#[test]
fn seed_on_an_unborn_head_states_no_reviewed_against() {
    let tmp = tempfile::tempdir().unwrap();
    Repository::init(tmp.path()).unwrap();
    assert_eq!(
        run(&seed_args(), tmp.path()).unwrap().seed,
        Some(SeedOutcome::Written)
    );
    let text = record_text(tmp.path());
    assert!(!text.contains("reviewed-against"), "{text}");
    assert!(text.contains("No commit existed when it was written"));
    let RecordLoad::Present(record) = load_review_record(&tmp.path().join("specs")) else {
        panic!("the seed parses as a review record");
    };
    assert_eq!((record.last_run, record.reviewed_against), (None, None));
}
