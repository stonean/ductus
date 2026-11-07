//! End-to-end walker integration test.
//!
//! Builds a synthetic procedure that exercises every step kind
//! (Primitive, Extension, Prose-with-gate-trigger, Prose-noop) and runs
//! the walker against mocked stdin/stdout buffers. Asserts the expected
//! JSON envelope sequence.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::io::Cursor;
use std::path::{Path, PathBuf};

use git2::{IndexAddOption, Repository, Signature};
use serde_json::{Map, Value};

use ductus::interpreter::{WalkOutcome, Walker};
use ductus::schema::procedure::{Procedure, SourceRange, Step, StepNumber};

fn loc() -> SourceRange {
    SourceRange {
        start_line: 1,
        start_col: 1,
        end_line: 1,
        end_col: 1,
    }
}

fn fixture_repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/primitives/sample-repo")
}

/// Stage everything under `repo` and commit; returns the new commit sha.
fn commit_all(repo: &Repository, message: &str) -> String {
    let mut index = repo.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();
    let parent = repo
        .head()
        .ok()
        .and_then(|h| h.target())
        .and_then(|oid| repo.find_commit(oid).ok());
    let parents: Vec<&git2::Commit> = parent.as_ref().into_iter().collect();
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
        .unwrap()
        .to_string()
}

/// Write `body` to `path`, creating parent directories as needed.
fn write_file(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, body).unwrap();
}

#[test]
fn walks_a_procedure_exercising_every_step_kind() {
    let procedure = Procedure {
        command: "smoke".into(),
        steps: vec![
            // Step 1: prose noop.
            Step::Prose {
                number: StepNumber(vec![1]),
                text: "Preamble — do nothing observable.".into(),
                location: loc(),
            },
            // Step 2: real primitive against the sample-repo fixture.
            Step::Primitive {
                number: StepNumber(vec![2]),
                name: "read-spec".into(),
                prose: "Invoke `read-spec` for the targeted feature.".into(),
                location: loc(),
            },
            // Step 3: extension point — host echoes back a response. A
            // single-request point; `assessSpecQuality` fans out per loaded
            // rule and is exercised by the interpreter's own tests.
            Step::Extension {
                number: StepNumber(vec![3]),
                identifier: "askClarifyQuestion".into(),
                prose: "Ask the user the open question.".into(),
                location: loc(),
            },
            // Step 4: gate trigger via prose.
            Step::Prose {
                number: StepNumber(vec![4]),
                text: "Ask the user to approve the result.".into(),
                location: loc(),
            },
        ],
    };

    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-basic".into()));
    context.insert("include-body".into(), Value::Bool(false));

    // Mock the host: it must answer two prompts in order — one
    // llm-response (request-id req-1) and one gate-response
    // (request-id req-2). Counter starts at 1 per Walker::fresh_request_id.
    let host_responses = "\
        {\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"answer\":\"yes\"}}\n\
        {\"type\":\"gate-response\",\"request-id\":\"req-2\",\"confirmed\":true}\n\
    ";

    let mut reader = Cursor::new(host_responses.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        fixture_repo(),
        context,
        &mut reader,
        &mut writer,
    );
    let outcome = walker.run().unwrap();
    assert_eq!(outcome, WalkOutcome::Complete);

    let envelopes: Vec<Value> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();

    // Expected sequence:
    //   progress(read-spec dispatch)           — step 2
    //   llm-request(askClarifyQuestion)        — step 3
    //   progress(received llm-response)        — step 3
    //   gate-confirm(step-4)                   — step 4
    //   progress(gate confirmed)               — step 4
    //   complete                               — end
    let types: Vec<&str> = envelopes
        .iter()
        .map(|v| v["type"].as_str().unwrap())
        .collect();
    assert_eq!(
        types,
        vec![
            "progress",     // step 2 dispatch
            "llm-request",  // step 3
            "progress",     // step 3 received
            "gate-confirm", // step 4
            "progress",     // step 4 confirmed
            "complete",     // end
        ]
    );

    // Step-number annotations and primitive markers land on the right
    // envelopes.
    assert_eq!(envelopes[0]["primitive"], "read-spec");
    assert_eq!(envelopes[0]["step"], "2");
    assert_eq!(envelopes[1]["extension-point"], "askClarifyQuestion");
    assert_eq!(envelopes[1]["request-id"], "req-1");
    assert_eq!(envelopes[3]["request-id"], "req-2");
    assert_eq!(envelopes[3]["gate"], "step-4");
    assert!(envelopes[5]["runtime-version"].is_string());
}

#[test]
fn walker_halts_on_primitive_failure_with_error_envelope() {
    let procedure = Procedure {
        command: "fail-fast".into(),
        steps: vec![
            Step::Primitive {
                number: StepNumber(vec![1]),
                name: "read-spec".into(),
                prose: String::new(),
                location: loc(),
            },
            Step::Primitive {
                number: StepNumber(vec![2]),
                name: "read-tasks".into(),
                prose: String::new(),
                location: loc(),
            },
        ],
    };
    let mut context = Map::new();
    context.insert("feature".into(), Value::String("999-missing".into()));

    let mut reader = Cursor::new(String::new());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        fixture_repo(),
        context,
        &mut reader,
        &mut writer,
    );
    let outcome = walker.run().unwrap();
    match outcome {
        WalkOutcome::Errored { code, .. } => assert_eq!(code, "primitive-failure"),
        WalkOutcome::Complete => panic!("expected Errored, got Complete"),
    }

    let envelopes: Vec<Value> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let types: Vec<&str> = envelopes
        .iter()
        .map(|v| v["type"].as_str().unwrap())
        .collect();
    // First primitive's dispatch progress, then error. The second
    // primitive never gets touched.
    assert_eq!(types, vec!["progress", "error"]);
    assert_eq!(envelopes[1]["code"], "primitive-failure");
    assert!(envelopes[1]["runtime-version"].is_string());
}

/// Task 46a ABI test: a primitive's structured result threads through the
/// walker context to a later extension's payload builder and a later
/// primitive. Walks the review pipeline shape
/// `compute-review-scope → discover-rule-files → performReview → write-review`
/// and asserts that `compute-review-scope`'s `scope`/`diff-base` and
/// `discover-rule-files`'s `selected`/`rules-dir` reach
/// `build_perform_review_request` (as `scope-files`/`rule-files`) and
/// `write-review` (which renders `diff-base` and the accumulated findings).
/// The observations leg is asserted by [`assert_observations_threaded`].
///
/// The observations channel must be threaded the whole way — pass response →
/// walker accumulator → `write-review` → the report section and the record's
/// `dispositions:` map. Without this leg the report's `## Observations` section
/// would render `*None.*` on every run whether or not the reviewer recorded
/// any: a check that cannot run, indistinguishable from one that passed.
///
/// The exec walker has no operator to ask, so the observation arrives with no
/// disposition and must be recorded as **undispositioned** — owed, not handled
/// — and must never reach the inbox (spec 058).
fn assert_observations_threaded(repo_root: &std::path::Path, review: &str) {
    assert!(
        review.contains(
            "## Observations\n\n- perf: a() is called in a loop — `src/a.rs` — **undispositioned**"
        ),
        "accumulated performReview observations reached the report:\n{review}"
    );
    assert!(
        review.contains("  undispositioned: 1\n"),
        "an exec-path observation is counted as owed:\n{review}"
    );
    assert!(
        !repo_root.join("specs/inbox.md").exists(),
        "a review writes nothing to the inbox"
    );
}

#[test]
fn review_primitive_results_thread_into_perform_review_and_write_review() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::init(tmp.path()).unwrap();
    let spec = |status: &str| format!("---\nstatus: {status}\ndependencies: []\n---\n\n# X\n");
    let spec_path = tmp.path().join("specs/001-x/spec.md");

    // History: 001-x goes planned → in-progress, then a source file is added —
    // the review scope. The diff base is the **parent** of the transition
    // commit, so the transition itself is inside the window (scenario
    // `review-base-includes-the-transition-commit`); the rule file lands in
    // that parent commit, which is therefore the boundary rather than a member
    // of the window.
    write_file(&spec_path, &spec("planned"));
    write_file(
        &tmp.path().join("framework/rules/security-backend.md"),
        "# Security\n\n- **SEC-BE-001**: no secrets in logs.\n",
    );
    let diff_base = commit_all(&repo, "feat: plan");
    write_file(&spec_path, &spec("in-progress"));
    commit_all(&repo, "chore: begin");
    write_file(&tmp.path().join("src/a.rs"), "fn a() {}\n");
    commit_all(&repo, "feat: implement");

    let procedure = Procedure {
        command: "review".into(),
        steps: vec![
            Step::Primitive {
                number: StepNumber(vec![1]),
                name: "compute-review-scope".into(),
                prose: String::new(),
                location: loc(),
            },
            Step::Primitive {
                number: StepNumber(vec![2]),
                name: "discover-rule-files".into(),
                prose: String::new(),
                location: loc(),
            },
            Step::Extension {
                number: StepNumber(vec![3]),
                identifier: "performReview".into(),
                prose: "Run the security pass.".into(),
                location: loc(),
            },
            Step::Primitive {
                number: StepNumber(vec![4]),
                name: "write-review".into(),
                prose: String::new(),
                location: loc(),
            },
        ],
    };

    // Seed only the values the review pipeline can't derive itself.
    // `diff-base` is deliberately NOT seeded — it must come from
    // `compute-review-scope` for this test to prove the threading.
    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-x".into()));
    context.insert(
        "reviewed-at".into(),
        Value::String("2026-07-06T00:00:00Z".into()),
    );
    context.insert("reviewed-against".into(), Value::String("headsha0".into()));
    context.insert("pass".into(), Value::String("security".into()));

    // One security pass returns a single MUST finding against the scoped file,
    // plus one observation — something real that matched no loaded rule.
    let responses = "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"findings\":[{\"rule\":\"SEC-BE-001\",\"severity\":\"must\",\"file\":\"src/a.rs\",\"line-range\":\"1\",\"confidence\":\"high\"}],\"observations\":[{\"text\":\"perf: a() is called in a loop\",\"path\":\"src/a.rs\"}]}}\n";

    let mut reader = Cursor::new(responses.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        tmp.path().to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let envelopes: Vec<Value> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();

    // `compute-review-scope`'s `scope` and `discover-rule-files`'s
    // `selected`/`rules-dir` reached `build_perform_review_request`.
    let request = envelopes
        .iter()
        .find(|e| e["type"] == "llm-request" && e["extension-point"] == "performReview")
        .expect("a performReview llm-request was emitted");
    let scope_files = request["request"]["scope-files"].as_array().unwrap();
    assert!(
        scope_files.iter().any(|f| f["path"] == "src/a.rs"),
        "compute-review-scope `scope` threaded into the performReview payload: {scope_files:?}"
    );
    let rule_files = request["request"]["rule-files"].as_array().unwrap();
    assert!(
        rule_files
            .iter()
            .any(|f| f["name"] == "security-backend.md"),
        "discover-rule-files `selected`/`rules-dir` threaded into the payload: {rule_files:?}"
    );

    // `write-review` consumed `compute-review-scope`'s `diff-base` (rendered
    // to review.md frontmatter) and the accumulated performReview `findings`.
    let review = std::fs::read_to_string(tmp.path().join("specs/001-x/review.md")).unwrap();
    assert!(
        review.contains(&format!("diff-base: {diff_base}")),
        "diff-base threaded from compute-review-scope into write-review:\n{review}"
    );
    assert!(
        review.contains("SEC-BE-001"),
        "accumulated performReview findings reached write-review:\n{review}"
    );

    assert_observations_threaded(tmp.path(), &review);

    assert!(
        review.contains("blocking: true"),
        "the MUST finding set blocking in the record:\n{review}"
    );
}

/// Protocol robustness (data-model §JSON-over-stdio ignore-and-continue):
/// while suspended awaiting an `llm-response`, a wrong-type envelope, a
/// malformed line, a blank keepalive line, and a response for a superseded
/// request-id are each logged to stderr and skipped — the walk still
/// completes once the valid response arrives, with no `error` envelope.
#[test]
fn stray_and_malformed_stdin_lines_are_ignored_while_awaiting_llm_response() {
    let procedure = Procedure {
        command: "noise".into(),
        steps: vec![Step::Extension {
            number: StepNumber(vec![1]),
            identifier: "askClarifyQuestion".into(),
            prose: String::new(),
            location: loc(),
        }],
    };

    // Noise before the valid response: wrong-type envelope, malformed JSON,
    // blank keepalive, then an llm-response for a superseded request-id.
    let host_lines = "\
        {\"type\":\"gate-response\",\"request-id\":\"req-1\",\"confirmed\":true}\n\
        this is not json\n\
        \n\
        {\"type\":\"llm-response\",\"request-id\":\"req-0\",\"response\":{\"answer\":\"stale\"}}\n\
        {\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"answer\":\"yes\"}}\n\
    ";
    let mut reader = Cursor::new(host_lines.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        fixture_repo(),
        Map::new(),
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let types: Vec<String> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| {
            serde_json::from_str::<Value>(l).unwrap()["type"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect();
    // llm-request, progress(received), complete — no error envelope, and
    // the consumed response is the one matching req-1.
    assert_eq!(types, vec!["llm-request", "progress", "complete"]);
}

/// Same ignore-and-continue rule while awaiting a `gate-response`: stray
/// and malformed lines are skipped and the gate still resolves.
#[test]
fn stray_and_malformed_stdin_lines_are_ignored_while_awaiting_gate_response() {
    let procedure = Procedure {
        command: "noise-gate".into(),
        steps: vec![Step::Prose {
            number: StepNumber(vec![1]),
            text: "Ask the user to approve the change.".into(),
            location: loc(),
        }],
    };
    let host_lines = "\
        {\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{}}\n\
        {not json}\n\
        {\"type\":\"gate-response\",\"request-id\":\"req-9\",\"confirmed\":false}\n\
        {\"type\":\"gate-response\",\"request-id\":\"req-1\",\"confirmed\":true}\n\
    ";
    let mut reader = Cursor::new(host_lines.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        fixture_repo(),
        Map::new(),
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let envelopes: Vec<Value> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let types: Vec<&str> = envelopes
        .iter()
        .map(|v| v["type"].as_str().unwrap())
        .collect();
    // The superseded req-9 denial was ignored; the matching req-1
    // confirmation resolved the gate and the walk completed.
    assert_eq!(types, vec!["gate-confirm", "progress", "complete"]);
    assert!(
        envelopes[1]["message"]
            .as_str()
            .unwrap()
            .contains("confirmed"),
        "gate resolved from the matching response: {envelopes:?}"
    );
}

/// Stdin EOF while suspended awaiting a response stays an operational
/// error (the one non-ignorable inbound condition).
#[test]
fn stdin_eof_while_awaiting_response_is_an_operational_error() {
    let procedure = Procedure {
        command: "eof".into(),
        steps: vec![Step::Extension {
            number: StepNumber(vec![1]),
            identifier: "askClarifyQuestion".into(),
            prose: String::new(),
            location: loc(),
        }],
    };
    // Only noise, then EOF — never a valid response.
    let mut reader = Cursor::new("not json\n".to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        fixture_repo(),
        Map::new(),
        &mut reader,
        &mut writer,
    );
    let err = walker.run().unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
}

/// Waiver-processing-order, exec-path binding: when `fired` is not seeded,
/// the walker binds the findings accumulated by the `performReview` passes
/// to `process-waivers`' `fired` argument, so a waiver whose rule still
/// fires classifies as applied — not mass-expired against an empty set.
#[test]
fn process_waivers_binds_fired_from_accumulated_pass_findings() {
    let tmp = tempfile::tempdir().unwrap();
    let spec_dir = tmp.path().join("specs/001-x");
    std::fs::create_dir_all(&spec_dir).unwrap();
    std::fs::write(
        spec_dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# X\n",
    )
    .unwrap();
    // Waivers live in `review.md`'s frontmatter (spec 057 task 5).
    std::fs::write(
        spec_dir.join("review.md"),
        "---\nspec: 001-x\nlast-run: 2026-07-07T00:00:00Z\nblocking: false\nwaivers:\n  \
         - rule: SEC-BE-001\n    file: src/a.rs\n    \
         reason: \"Internal-only path behind mTLS; rule targets public endpoints\"\n    \
         waived-at: 2026-07-07T00:00:00Z\n    waived-by: test@example.com\n---\n\n# Review — 001-x\n",
    )
    .unwrap();
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(tmp.path().join("src/a.rs"), "fn a() {}\n").unwrap();

    // Corrected review order: the pass produces the findings, then
    // process-waivers classifies against them, then write-review renders.
    let procedure = Procedure {
        command: "review".into(),
        steps: vec![
            Step::Extension {
                number: StepNumber(vec![1]),
                identifier: "performReview".into(),
                prose: String::new(),
                location: loc(),
            },
            Step::Primitive {
                number: StepNumber(vec![2]),
                name: "process-waivers".into(),
                prose: String::new(),
                location: loc(),
            },
            Step::Primitive {
                number: StepNumber(vec![3]),
                name: "write-review".into(),
                prose: String::new(),
                location: loc(),
            },
        ],
    };

    // `fired` is deliberately NOT seeded — it must come from the pass's
    // accumulated `findings` for this test to prove the binding.
    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-x".into()));
    context.insert(
        "reviewed-at".into(),
        Value::String("2026-07-07T00:00:00Z".into()),
    );
    context.insert("reviewed-against".into(), Value::String("headsha0".into()));
    context.insert("diff-base".into(), Value::String(String::new()));

    let responses = "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"findings\":[{\"rule\":\"SEC-BE-001\",\"severity\":\"must\",\"file\":\"src/a.rs\",\"line-range\":\"1\",\"confidence\":\"high\"}]}}\n";
    let mut reader = Cursor::new(responses.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        tmp.path().to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let review = std::fs::read_to_string(spec_dir.join("review.md")).unwrap();
    // The waiver applied (rule still fires per the pass findings), so the
    // MUST is excluded from the blocking count ...
    assert!(
        review.contains("must-violations: 0"),
        "waiver classified as applied against the pass findings:\n{review}"
    );
    // ... and rendered with the waiver reason — proof it was applied, not
    // expired against an empty `fired` set.
    assert!(
        review.contains("Internal-only path behind mTLS"),
        "applied waiver reason reached write-review:\n{review}"
    );
    assert!(
        review.contains("blocking: false"),
        "the waived finding does not block:\n{review}"
    );
    assert!(
        review.contains("SEC-BE-001"),
        "the still-valid waiver was not pruned from the record:\n{review}"
    );
}

/// Regression for the review-exec-wiring waiver gap: `process-waivers` emits
/// `applied`/`expired`, and `write-review` reads them via serde alias, so an
/// applied waiver threads through the walker context (which merges primitive
/// results by bare key) and excludes its finding from the blocking count.
/// Without the alias the waived MUST would re-block on the exec path.
#[test]
fn applied_waiver_threads_from_process_waivers_into_write_review() {
    let tmp = tempfile::tempdir().unwrap();
    let spec_dir = tmp.path().join("specs/001-x");
    std::fs::create_dir_all(&spec_dir).unwrap();
    // Spec carries a waiver anchored at (SEC-BE-001, src/a.rs).
    std::fs::write(
        spec_dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# X\n",
    )
    .unwrap();
    // Waivers live in `review.md`'s frontmatter (spec 057 task 5).
    std::fs::write(
        spec_dir.join("review.md"),
        "---\nspec: 001-x\nlast-run: 2026-07-07T00:00:00Z\nblocking: false\nwaivers:\n  \
         - rule: SEC-BE-001\n    file: src/a.rs\n    \
         reason: \"Internal-only path behind mTLS; rule targets public endpoints\"\n    \
         waived-at: 2026-07-07T00:00:00Z\n    waived-by: test@example.com\n---\n\n# Review — 001-x\n",
    )
    .unwrap();
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(tmp.path().join("src/a.rs"), "fn a() {}\n").unwrap();

    let procedure = Procedure {
        command: "review".into(),
        steps: vec![
            Step::Primitive {
                number: StepNumber(vec![1]),
                name: "process-waivers".into(),
                prose: String::new(),
                location: loc(),
            },
            Step::Primitive {
                number: StepNumber(vec![2]),
                name: "write-review".into(),
                prose: String::new(),
                location: loc(),
            },
        ],
    };

    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-x".into()));
    context.insert(
        "reviewed-at".into(),
        Value::String("2026-07-07T00:00:00Z".into()),
    );
    context.insert("reviewed-against".into(), Value::String("headsha0".into()));
    context.insert("diff-base".into(), Value::String(String::new()));
    // The firing finding — in a live exec run the performReview passes supply
    // it; seeded here so process-waivers can classify the waiver as applied.
    context.insert(
        "fired".into(),
        serde_json::json!([{ "rule": "SEC-BE-001", "file": "src/a.rs" }]),
    );
    context.insert(
        "findings".into(),
        serde_json::json!([{
            "rule": "SEC-BE-001", "severity": "must", "file": "src/a.rs",
            "line-range": "1", "confidence": "high"
        }]),
    );

    let mut reader = Cursor::new(String::new());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        tmp.path().to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let review = std::fs::read_to_string(spec_dir.join("review.md")).unwrap();
    // The waived MUST is excluded from the blocking count ...
    assert!(
        review.contains("must-violations: 0"),
        "applied waiver excluded the MUST from the count:\n{review}"
    );
    // ... and rendered with the waiver reason, which only the applied waiver
    // carries — proof it threaded from process-waivers via the `applied` alias.
    assert!(
        review.contains("Internal-only path behind mTLS"),
        "applied waiver reason reached write-review:\n{review}"
    );
    assert!(
        review.contains("blocking: false"),
        "a waived finding does not block:\n{review}"
    );
}

/// Gate convention, primitive shape: a `gate-confirm` primitive step IS a
/// blocking gate by virtue of the primitive — no "ask the user to
/// approve" phrase required (prune.md step 6's shape). Denial is a clean
/// `complete` with `confirmed: false`, and later steps never run.
#[test]
fn gate_confirm_primitive_step_blocks_without_the_phrase() {
    let procedure = Procedure {
        command: "test".into(),
        steps: vec![
            Step::Primitive {
                number: StepNumber(vec![1]),
                name: "gate-confirm".into(),
                prose: "Invoke `gate-confirm` with a prompt naming the destructive write.".into(),
                location: loc(),
            },
            Step::Primitive {
                number: StepNumber(vec![2]),
                name: "read-spec".into(),
                prose: String::new(),
                location: loc(),
            },
        ],
    };
    let response = "{\"type\":\"gate-response\",\"request-id\":\"req-1\",\"confirmed\":false}\n";
    let mut reader = Cursor::new(response.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-basic".into()));
    let mut walker = Walker::new(
        &procedure,
        fixture_repo(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);
    let envelopes: Vec<Value> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let types: Vec<&str> = envelopes
        .iter()
        .map(|v| v["type"].as_str().unwrap())
        .collect();
    // gate-confirm, progress(denied), complete(confirmed: false) — the
    // step-2 read-spec dispatch never happens.
    assert_eq!(types, vec!["gate-confirm", "progress", "complete"]);
    assert_eq!(envelopes[0]["gate"], "step-1");
    assert_eq!(envelopes[2]["result"]["confirmed"], false);
}

/// Gate convention, prose shape: the phrase trigger remains the fallback
/// for a prose step with no primitive.
#[test]
fn prose_step_with_gate_phrase_blocks() {
    let procedure = Procedure {
        command: "test".into(),
        steps: vec![Step::Prose {
            number: StepNumber(vec![6]),
            text: "Ask the user to approve the transition from clarified to planned.".into(),
            location: loc(),
        }],
    };
    let response = "{\"type\":\"gate-response\",\"request-id\":\"req-1\",\"confirmed\":true}\n";
    let mut reader = Cursor::new(response.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        fixture_repo(),
        Map::new(),
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);
    let envelopes: Vec<Value> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let types: Vec<&str> = envelopes
        .iter()
        .map(|v| v["type"].as_str().unwrap())
        .collect();
    assert_eq!(types, vec!["gate-confirm", "progress", "complete"]);
    // The gate name reflects the step's real document number.
    assert_eq!(envelopes[0]["gate"], "step-6");
}

/// Gate convention, dispatch-wins rule: a non-gate primitive step whose
/// prose happens to contain the phrase still dispatches its primitive —
/// it is never silently converted into a gate that drops the dispatch.
#[test]
fn primitive_step_with_gate_phrase_keeps_its_dispatch() {
    let procedure = Procedure {
        command: "test".into(),
        steps: vec![Step::Primitive {
            number: StepNumber(vec![1]),
            name: "read-spec".into(),
            prose: "Invoke `read-spec`, then ask the user to approve the summary.".into(),
            location: loc(),
        }],
    };
    let mut reader = Cursor::new(String::new());
    let mut writer: Vec<u8> = Vec::new();
    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-basic".into()));
    let mut walker = Walker::new(
        &procedure,
        fixture_repo(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);
    let envelopes: Vec<Value> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let types: Vec<&str> = envelopes
        .iter()
        .map(|v| v["type"].as_str().unwrap())
        .collect();
    // progress(read-spec dispatch), complete — no gate-confirm envelope.
    assert_eq!(types, vec!["progress", "complete"]);
    assert_eq!(envelopes[0]["primitive"], "read-spec");
}

/// End-to-end coverage for prune.md's confirmation gate on the exec path:
/// the shipped command file parses with document step numbers 1..10, step 6
/// is a `gate-confirm` primitive (no phrase), and walking the procedure
/// blocks there — a denied gate leaves tasks.md untouched. The plan preview
/// at step 3 dispatches with the walker's feature binding, and the fixture's
/// missing plan does not halt the walk, which goes on to the gate.
#[test]
fn prune_command_gate_blocks_on_the_exec_path() {
    let source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("runtime/.. exists")
            .join("framework/commands/prune.md"),
    )
    .unwrap();
    let procedure = ductus::parser::parse(&source, "prune").expect("prune.md parses");

    // Step numbers follow the document (1..10) and step 6 is the gate.
    let numbers: Vec<Vec<u32>> = procedure
        .steps
        .iter()
        .map(|s| match s {
            Step::Primitive { number, .. }
            | Step::Extension { number, .. }
            | Step::Prose { number, .. } => number.0.clone(),
        })
        .collect();
    assert_eq!(numbers, (1..=10).map(|n| vec![n]).collect::<Vec<_>>());
    match &procedure.steps[5] {
        Step::Primitive { name, .. } => assert_eq!(name, "gate-confirm"),
        other => panic!("prune.md step 6 must be the gate-confirm primitive, got {other:?}"),
    }

    // Walk against a throwaway repo: preview (step 2) reads tasks.md, the
    // plan preview (step 3) finds no plan, the gate (step 6) blocks, and
    // denial stops before the apply (step 7).
    let tmp = tempfile::tempdir().unwrap();
    let feature_dir = tmp.path().join("specs/001-basic");
    std::fs::create_dir_all(&feature_dir).unwrap();
    std::fs::write(
        feature_dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# 001\n",
    )
    .unwrap();
    let tasks_body =
        "# 001\n\n## 1. Spent\n\n- [x] Done subtask.\n\n## 2. Pending\n\n- [ ] Open subtask.\n";
    std::fs::write(feature_dir.join("tasks.md"), tasks_body).unwrap();

    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-basic".into()));
    let response = "{\"type\":\"gate-response\",\"request-id\":\"req-1\",\"confirmed\":false}\n";
    let mut reader = Cursor::new(response.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        tmp.path().to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let envelopes: Vec<Value> = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let types: Vec<&str> = envelopes
        .iter()
        .map(|v| v["type"].as_str().unwrap())
        .collect();
    // progress(step-1 resolve-session, spec 062), progress(step-2
    // prune-tasks preview), progress(step-3 prune-plan preview),
    // gate-confirm(step-6), progress(denied), complete(confirmed: false).
    assert_eq!(
        types,
        vec![
            "progress",
            "progress",
            "progress",
            "gate-confirm",
            "progress",
            "complete"
        ]
    );
    assert_eq!(envelopes[0]["primitive"], "resolve-session");
    assert_eq!(envelopes[0]["step"], "1");
    assert_eq!(envelopes[1]["primitive"], "prune-tasks");
    assert_eq!(envelopes[1]["step"], "2");
    assert_eq!(envelopes[2]["primitive"], "prune-plan");
    assert_eq!(envelopes[2]["step"], "3");
    assert_eq!(envelopes[3]["gate"], "step-6");
    assert_eq!(envelopes[5]["result"]["confirmed"], false);

    // The denied gate wrote nothing. A confirmed one writes (below), so this
    // can fail.
    let after = std::fs::read_to_string(feature_dir.join("tasks.md")).unwrap();
    assert_eq!(after, tasks_body, "tasks.md must be untouched after denial");
}

/// A confirmed gate authorizes the write after it: prune.md step 7 runs
/// `prune-tasks` with `apply`, so the spent section goes and the pending one
/// stays. Before the walker bound `apply` at a confirmed gate, step 7
/// previewed a second time and the walk completed having written nothing.
#[test]
fn prune_command_applies_after_a_confirmed_gate() {
    let source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("runtime/.. exists")
            .join("framework/commands/prune.md"),
    )
    .unwrap();
    let procedure = ductus::parser::parse(&source, "prune").expect("prune.md parses");

    let tmp = tempfile::tempdir().unwrap();
    let feature_dir = tmp.path().join("specs/001-basic");
    std::fs::create_dir_all(&feature_dir).unwrap();
    std::fs::write(
        feature_dir.join("spec.md"),
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# 001\n",
    )
    .unwrap();
    std::fs::write(
        feature_dir.join("tasks.md"),
        "# 001\n\n## 1. Spent\n\n- [x] Done subtask.\n\n## 2. Pending\n\n- [ ] Open subtask.\n",
    )
    .unwrap();

    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-basic".into()));
    let response = "{\"type\":\"gate-response\",\"request-id\":\"req-1\",\"confirmed\":true}\n";
    let mut reader = Cursor::new(response.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        tmp.path().to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let after = std::fs::read_to_string(feature_dir.join("tasks.md")).unwrap();
    assert!(
        !after.contains("## 1. Spent"),
        "the confirmed reduction removes the spent section:\n{after}"
    );
    assert!(
        after.contains("## 2. Pending") && after.contains("- [ ] Open subtask."),
        "the pending section survives whole:\n{after}"
    );
}

/// The size check spec 063 adds to `/clarify` and `/plan` sits where the plan
/// puts it — after the spec's answers and the plan's tasks are written, ahead
/// of each command's gate — and an oversized artifact never halts either
/// walk: the step after it still dispatches, so the transition is not
/// blocked (AC2).
#[test]
fn clarify_and_plan_size_checks_warn_without_halting_the_walk() {
    let commands = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("runtime/.. exists")
        .join("framework/commands");
    let name_at = |steps: &[Step], n: u32| -> String {
        steps
            .iter()
            .find_map(|s| match s {
                Step::Primitive { number, name, .. } if number.0 == vec![n] => Some(name.clone()),
                _ => None,
            })
            .unwrap_or_default()
    };
    // (command, size step, the primitive step before it, the step after it)
    for (command, size_step, before, after) in [
        ("clarify", 10, (9, "label-criteria"), (13, "set-status")),
        ("plan", 8, (3, "create-plan-artifacts"), (9, "gate-confirm")),
    ] {
        let source = std::fs::read_to_string(commands.join(format!("{command}.md"))).unwrap();
        let procedure = ductus::parser::parse(&source, command).expect("command parses");
        assert_eq!(
            name_at(&procedure.steps, size_step),
            "check-artifact-size",
            "{command}.md step {size_step}"
        );
        assert_eq!(name_at(&procedure.steps, before.0), before.1, "{command}");
        assert_eq!(name_at(&procedure.steps, after.0), after.1, "{command}");

        // Walk the parsed size step, then a read-only step, over a spec whose
        // plan is three read pages at a 100-byte threshold.
        let tmp = tempfile::tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/001-basic");
        std::fs::create_dir_all(&feature_dir).unwrap();
        std::fs::write(
            feature_dir.join("spec.md"),
            "---\nstatus: draft\ndependencies: []\n---\n\n# 001\n",
        )
        .unwrap();
        std::fs::write(feature_dir.join("plan.md"), "x".repeat(250)).unwrap();
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 100\n",
        )
        .unwrap();
        let size = procedure
            .steps
            .iter()
            .find(|s| matches!(s, Step::Primitive { name, .. } if name == "check-artifact-size"))
            .unwrap()
            .clone();
        let next = Step::Primitive {
            number: StepNumber(vec![size_step + 1]),
            name: "read-spec".into(),
            prose: String::new(),
            location: loc(),
        };
        let walk = Procedure {
            command: command.into(),
            steps: vec![size, next],
        };
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-basic".into()));
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &walk,
            tmp.path().to_path_buf(),
            context,
            &mut reader,
            &mut writer,
        );
        assert_eq!(walker.run().unwrap(), WalkOutcome::Complete, "{command}");
        let primitives: Vec<String> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .filter_map(|v| v["primitive"].as_str().map(str::to_string))
            .collect();
        assert_eq!(
            primitives,
            ["check-artifact-size", "read-spec"],
            "{command}"
        );
    }
}

/// A project whose 001-x went in progress and then gained `n` source files,
/// and a sibling spec `n` scenarios, with `[artifacts] read-size-bytes` small
/// enough that both scope results pass it (066 scenario
/// `scope-results-have-a-bounded-size`).
fn over_the_read_size(n: usize) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::init(tmp.path()).unwrap();
    let spec = |status: &str| format!("---\nstatus: {status}\ndependencies: []\n---\n\n# X\n");
    write_file(&tmp.path().join("specs/001-x/spec.md"), &spec("planned"));
    write_file(&tmp.path().join("specs/002-y/spec.md"), &spec("planned"));
    commit_all(&repo, "plan");
    write_file(
        &tmp.path().join("specs/001-x/spec.md"),
        &spec("in-progress"),
    );
    commit_all(&repo, "begin");
    for i in 0..n {
        write_file(
            &tmp.path().join(format!("src/f{i}.rs")),
            &format!("fn f{i}() {{}}\n"),
        );
        write_file(
            &tmp.path().join(format!("specs/002-y/scenarios/s{i}.md")),
            "x\n",
        );
    }
    commit_all(&repo, "feat(001): work");
    write_file(
        &tmp.path().join(".ductus/config.toml"),
        "[artifacts]\nread-size-bytes = 300\n",
    );
    tmp
}

/// On the exec path a scope past the read size reaches the review whole: the
/// walker's `performReview` request loads every in-scope file from the list
/// file `compute-review-scope` names, and carries no inline `scope`, so the
/// file form is what it read.
#[test]
fn a_scope_past_the_read_size_reaches_the_review_through_its_list_file() {
    let tmp = over_the_read_size(12);
    let procedure = Procedure {
        command: "review".into(),
        steps: vec![
            Step::Primitive {
                number: StepNumber(vec![1]),
                name: "compute-review-scope".into(),
                prose: String::new(),
                location: loc(),
            },
            Step::Extension {
                number: StepNumber(vec![2]),
                identifier: "performReview".into(),
                prose: "Run the security pass.".into(),
                location: loc(),
            },
        ],
    };
    let mut context = Map::new();
    context.insert("feature".into(), Value::String("001-x".into()));
    context.insert("pass".into(), Value::String("security".into()));
    let responses =
        "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"findings\":[]}}\n";
    let mut reader = Cursor::new(responses.to_string());
    let mut writer: Vec<u8> = Vec::new();
    let mut walker = Walker::new(
        &procedure,
        tmp.path().to_path_buf(),
        context,
        &mut reader,
        &mut writer,
    );
    assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

    let request = std::str::from_utf8(&writer)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .find(|e| e["type"] == "llm-request" && e["extension-point"] == "performReview")
        .expect("a performReview llm-request was emitted");
    assert!(
        request["request"].get("scope").is_none(),
        "the scope was inline, so the file form went unexercised: {request}"
    );
    let paths: Vec<&str> = request["request"]["scope-files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["path"].as_str().unwrap())
        .collect();
    for i in 0..12 {
        let file = format!("src/f{i}.rs");
        assert!(paths.contains(&file.as_str()), "{file} missing: {paths:?}");
    }
}
