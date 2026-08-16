//! `check-review-gate` — evaluate `/ductus:implement`'s pre-done review gate.
//!
//! The deterministic surface behind the completion gate's step 13 (spec
//! 022, scenario coverage-expansion-primitives), which the host previously
//! walked by hand on every completion attempt: first the feature
//! directory's markdown lint (through the `lint-markdown` machinery,
//! replacing the raw `npx markdownlint-cli2` invocation), then the spec
//! frontmatter `review:` block. The first failing check wins and produces
//! the canonical `blocked: …` message — with the adopter's `[host]
//! project` command namespace substituted into the `/{project}:review`
//! references — plus, on `must-violations`, the resolve-or-waive
//! guidance. A blocked gate is a domain outcome the host acts on (halt,
//! do not propose the transition), never an operational error.

use std::collections::BTreeSet;
use std::path::Path;

use crate::host::Host;
use crate::primitives::{
    PrimitiveError, Result, lint_markdown, read_spec, read_text, split_frontmatter,
    validate_no_traversal,
};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckReviewGateArgs, CheckReviewGateResult, Frontmatter, LintMarkdownArgs, LintMarkdownResult,
    ReviewGateBlock,
};

/// Execute the `check-review-gate` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` is empty,
/// absolute, or carries a parent-directory component,
/// [`PrimitiveError::FeatureNotFound`] when the feature directory does
/// not exist, [`PrimitiveError::Io`] when `spec.md` is unreadable or
/// `npx` cannot be spawned, or [`PrimitiveError::Yaml`] for a malformed
/// frontmatter block. Every gate verdict — including all three block
/// reasons — is a domain outcome in the result.
pub fn run(args: &CheckReviewGateArgs, repo: &Path) -> Result<CheckReviewGateResult> {
    run_with_lint(args, repo, lint_markdown::run)
}

/// Implementation seam that lets unit tests inject a canned lint outcome
/// instead of spawning `npx markdownlint-cli2`. The MCP and CLI surfaces
/// both call [`run`], which forwards the real `lint-markdown` primitive.
pub(crate) fn run_with_lint(
    args: &CheckReviewGateArgs,
    repo: &Path,
    lint: impl FnOnce(&LintMarkdownArgs, &Path) -> Result<LintMarkdownResult>,
) -> Result<CheckReviewGateResult> {
    validate_no_traversal(&args.feature)?;
    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    let rel_dir = format!("{root}/{}", args.feature);

    // Gate check 1: every markdown file in the feature directory passes
    // markdownlint (recursive — scenarios/ included; `**` matches zero or
    // more directories, so the feature dir's own files are covered).
    let lint_result = lint(
        &LintMarkdownArgs {
            paths: vec![format!("{rel_dir}/**/*.md")],
            fix: false,
        },
        repo,
    )?;
    if !lint_result.clean {
        let message = if lint_result.violations.is_empty() {
            // Non-zero exit with nothing parseable: a config or runtime
            // error, or a violation shape the parser does not recognize.
            format!(
                "blocked: markdownlint-cli2 exited {} for {rel_dir} — resolve the lint failure before completing",
                lint_result.exit_code
            )
        } else {
            format!(
                "blocked: {} markdownlint violation(s) in {rel_dir} — resolve them before completing",
                lint_result.violations.len()
            )
        };
        return Ok(CheckReviewGateResult {
            passed: false,
            blocked_by: Some(ReviewGateBlock::MarkdownLint),
            message: Some(message),
            guidance: None,
            violations: lint_result.violations,
        });
    }

    // Gate check 2: no scenario under this feature carries an unresolved
    // open question (spec 046).
    if let Some(blocked) = scenario_question_block(&feature_dir, repo, &args.feature) {
        return Ok(blocked);
    }

    // Gate checks 3 and 4: the spec frontmatter `review:` block.
    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, _body) = split_frontmatter(&content, &spec_path)?;
    let frontmatter: Frontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;
    let project = Host::load(repo).project;

    let review = match frontmatter.review {
        Some(review) if review.last_run.is_some() => review,
        // Absent block or null `last-run`: the spec has never completed a
        // review.
        _ => {
            return Ok(CheckReviewGateResult {
                passed: false,
                blocked_by: Some(ReviewGateBlock::NotReviewed),
                message: Some(format!(
                    "blocked: spec has not been reviewed — run /{project}:review before completing"
                )),
                guidance: None,
                violations: vec![],
            });
        }
    };

    if review.blocking {
        return Ok(CheckReviewGateResult {
            passed: false,
            blocked_by: Some(ReviewGateBlock::MustViolations),
            message: Some(format!(
                "blocked: spec has {} MUST violation(s) — see {rel_dir}/review.md",
                review.must_violations
            )),
            guidance: Some(format!(
                "Resolve the violations and re-run /{project}:review, or run \
                 /{project}:review --waive <rule-id> --reason \"...\" for each waivable finding."
            )),
            violations: vec![],
        });
    }

    // Gate check 5: the recorded review still describes the current code.
    if let Some(stale) =
        stale_review_block(repo, &rel_dir, review.reviewed_against.as_deref(), &project)
    {
        return Ok(stale);
    }

    Ok(CheckReviewGateResult {
        passed: true,
        blocked_by: None,
        message: None,
        guidance: None,
        violations: vec![],
    })
}

/// The staleness gate check: `Some(blocked)` when one of the spec's
/// **durable contracts** — a `scenarios/*.md` file or `data-model.md` —
/// changed after `reviewed-against`.
///
/// The other four checks ask whether a review exists and whether it passed.
/// None of them asks whether it still *applies*. A review recorded against
/// one commit and never re-run reads as a pass forever, so hand-editing a
/// `review.md` to mark findings resolved produces a record that satisfies
/// every automated check while describing a diff that is gone. That is not
/// hypothetical: `gvrn-v0.26.2` shipped with 022's review pointing three
/// commits behind the tag, and nothing noticed.
///
/// Scoped to the durable contracts, and that scoping was corrected under
/// measurement rather than reasoned about. The first cut used the plan's
/// **Affected Files**; run across this repo it blocked **34 of 48** specs,
/// because old specs list shared surfaces (`AGENTS.md`, `README.md`,
/// `framework/bootstrap/ductus.md`) that every later spec also touches — so
/// completing spec 004 was blocked by spec 042 having edited `AGENTS.md`.
/// A gate that blocks seven specs in eight is one people route around.
/// `tasks.md` and `plan.md` are excluded for the same reason: the first is
/// ephemeral by construction (§tasks-phase), the second churns as Affected
/// Files are revised. This is now the identical rule `/{project}:audit`
/// Family 19 applies at release time, which is the point — two enforcement
/// moments, one definition of stale.
///
/// `review.md` and `spec.md` are not contracts for this purpose:
/// `write-review` touches both, so counting them would make every review
/// stale the instant it was recorded.
///
/// Fails **open** on anything it cannot determine — no git repo, an
/// unparseable `reviewed-against`. A gate that blocks on its own inability
/// to check would be a gate people route around; the honest checks above
/// still run.
fn stale_review_block(
    repo: &Path,
    rel_dir: &str,
    reviewed_against: Option<&str>,
    project: &str,
) -> Option<CheckReviewGateResult> {
    let base = reviewed_against?.trim();
    if base.is_empty() {
        return None;
    }
    let repository = git2::Repository::discover(repo).ok()?;
    let base_tree = repository
        .find_commit(git2::Oid::from_str(base).ok()?)
        .ok()?
        .tree()
        .ok()?;
    let head_tree = repository.head().ok()?.peel_to_commit().ok()?.tree().ok()?;
    let diff = repository
        .diff_tree_to_tree(Some(&base_tree), Some(&head_tree), None)
        .ok()?;

    let prefix = format!("{rel_dir}/");
    let mut stale: BTreeSet<String> = BTreeSet::new();
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path().or_else(|| delta.old_file().path()) {
                let path = path.to_string_lossy().replace('\\', "/");
                if let Some(rest) = path.strip_prefix(&prefix)
                    && is_durable_contract(rest)
                {
                    stale.insert(path);
                }
            }
            true
        },
        None,
        None,
        None,
    )
    .ok()?;

    if stale.is_empty() {
        return None;
    }
    let shown: Vec<&str> = stale.iter().take(3).map(String::as_str).collect();
    let more = stale.len().saturating_sub(shown.len());
    let tail = if more > 0 {
        format!(" (+{more} more)")
    } else {
        String::new()
    };
    Some(CheckReviewGateResult {
        passed: false,
        blocked_by: Some(ReviewGateBlock::ReviewStale),
        message: Some(format!(
            "blocked: review is stale — {} durable contract(s) changed since reviewed-against {}: {}{tail}",
            stale.len(),
            &base[..base.len().min(8)],
            shown.join(", ")
        )),
        guidance: Some(format!(
            "Re-run /{project}:review so the recorded verdict describes the current code."
        )),
        violations: vec![],
    })
}

/// A scenario or the data model — the artifacts a review actually reads.
/// Mirrors `scripts/audit/review-freshness.sh`'s rule exactly.
fn is_durable_contract(rel_within_feature: &str) -> bool {
    let is_md = Path::new(rel_within_feature)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
    (rel_within_feature.starts_with("scenarios/") && is_md) || rel_within_feature == "data-model.md"
}

/// The scenario-open-questions gate check: `Some(blocked)` when any
/// scenario under `feature_dir` carries an unresolved open question,
/// `None` when the check passes.
///
/// A scenario is an organizational split of the spec — it exists to keep
/// `spec.md` from becoming one huge document — so its questions are the
/// spec's questions for the purpose of completeness, and the spec is not
/// `done` while any remain (spec 046). Resolution stays independent:
/// the guidance points at scenario-targeted clarify, the only command that
/// can act on them.
///
/// Only `## Open Questions` counts. Per §spec-requirements an open
/// question is an *undecided blocker*; one deferred pending a condition is
/// resolved-with-a-condition and belongs in `## Resolved Questions`, which
/// the parser does not read. That is a convention rather than a skip
/// marker on purpose — an exemptible section would let anything blocking
/// be relabelled to ship past this gate.
///
/// Ordered ahead of the `review:` checks because an unresolved design
/// question is the more upstream defect — reviewing a design that is about
/// to change wastes the review.
///
/// Reads through `read-spec`'s collector rather than re-deriving the list,
/// so the gate blocks on exactly the questions the user was shown.
fn scenario_question_block(
    feature_dir: &Path,
    repo: &Path,
    feature: &str,
) -> Option<CheckReviewGateResult> {
    let scan = read_spec::collect_scenario_open_questions(feature_dir);
    let questions = scan.questions;
    if questions.is_empty() {
        // An unreadable scenario does not block — nothing can be proven about
        // a file that will not parse, and this gate must not fail closed on
        // its own inability to read. It is reported by `read-spec` and by
        // `check-artifacts`' skipped list instead, so the pass is still
        // distinguishable from a fully-examined one.
        return None;
    }
    let scenarios = read_spec::scenario_names(&questions);
    let project = Host::load(repo).project;
    Some(CheckReviewGateResult {
        passed: false,
        blocked_by: Some(ReviewGateBlock::ScenarioOpenQuestions),
        message: Some(format!(
            "blocked: {} unresolved open question(s) in scenario(s) {} — resolve them before completing",
            questions.len(),
            scenarios.join(", ")
        )),
        guidance: Some(format!(
            "Run /{project}:target {feature}/<scenario> then /{project}:clarify to resolve each \
             scenario's questions in place. A question that is deferred rather than undecided \
             (\"not now; revisit when X lands\") is resolved with a condition — move it to the \
             scenario's Resolved Questions with its trigger recorded."
        )),
        violations: vec![],
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::schema::primitives::MarkdownViolation;
    use std::fs;
    use tempfile::tempdir;

    const REVIEWED_CLEAN: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 1\n  low-confidence: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const REVIEWED_BLOCKING: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 3\n  should-violations: 0\n  low-confidence: 0\n  blocking: true\n---\n\n# 007 — Gate\n";
    const NEVER_REVIEWED: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: null\n  reviewed-against: null\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const NO_REVIEW_BLOCK: &str =
        "---\nstatus: in-progress\ndependencies: []\n---\n\n# 007 — Gate\n";

    fn seed(repo: &Path, spec: &str) {
        fs::create_dir_all(repo.join("specs/007-gate")).unwrap();
        fs::write(repo.join("specs/007-gate/spec.md"), spec).unwrap();
        // Pin the slash-command namespace so canonical messages are
        // deterministic (the default is the tempdir's random basename).
        fs::write(repo.join(".govern.toml"), "[host]\nproject = \"ductus\"\n").unwrap();
    }

    fn args() -> CheckReviewGateArgs {
        CheckReviewGateArgs {
            feature: "007-gate".into(),
        }
    }

    // The wrapper matches the `run_with_lint` seam signature.
    #[allow(clippy::unnecessary_wraps)]
    fn clean_lint(_: &LintMarkdownArgs, _: &Path) -> Result<LintMarkdownResult> {
        Ok(LintMarkdownResult {
            violations: vec![],
            clean: true,
            exit_code: 0,
        })
    }

    #[test]
    fn passes_when_lint_clean_and_review_current() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed);
        assert!(result.blocked_by.is_none());
        assert!(result.message.is_none());
        assert!(result.guidance.is_none());
        assert!(result.violations.is_empty());
    }

    // --- gate check 5: review staleness -------------------------------------

    /// Init a repo, stage everything, commit; returns the commit sha.
    fn git_commit_all(repo: &Path, message: &str) -> String {
        let repository =
            git2::Repository::open(repo).unwrap_or_else(|_| git2::Repository::init(repo).unwrap());
        let mut index = repository.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let parent = repository
            .head()
            .ok()
            .and_then(|h| h.target())
            .and_then(|oid| repository.find_commit(oid).ok());
        let parents: Vec<&git2::Commit> = parent.as_ref().into_iter().collect();
        repository
            .commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
            .unwrap()
            .to_string()
    }

    /// A spec whose review points at `sha`, plus a plan claiming `runtime/src/`.
    fn seed_reviewed_at(repo: &Path, sha: &str) {
        fs::write(
            repo.join("specs/007-gate/spec.md"),
            format!(
                "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  \
                 reviewed-against: {sha}\n  must-violations: 0\n  should-violations: 0\n  \
                 low-confidence: 0\n  blocking: false\n---\n\n# 007 — Gate\n"
            ),
        )
        .unwrap();
    }

    #[test]
    fn a_change_inside_the_plans_affected_files_marks_the_review_stale() {
        // The gvrn-v0.26.2 shape: a passing review whose recorded sha predates
        // a change to the spec's own surface. Every other check passes.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        fs::create_dir_all(tmp.path().join("specs/007-gate/scenarios")).unwrap();
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();
        let base = git_commit_all(tmp.path(), "base");

        seed_reviewed_at(tmp.path(), &base);
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\nNew contract text.\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();
        git_commit_all(tmp.path(), "change a durable contract");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed, "{result:?}");
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::ReviewStale));
        let message = result.message.unwrap();
        assert!(message.contains("review is stale"), "{message}");
        assert!(message.contains("scenarios/retry.md"), "{message}");
        assert!(
            result.guidance.unwrap().contains("/ductus:review"),
            "guidance must name the command that clears it"
        );
    }

    #[test]
    fn bookkeeping_churn_leaves_the_review_current() {
        // Scope discipline: a repo-wide test would mark every review stale on
        // the next unrelated commit, which teaches people to ignore the gate.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");

        seed_reviewed_at(tmp.path(), &base);
        // tasks.md is ephemeral by construction and plan.md churns; neither is
        // a durable contract, so ticking a checkbox must not block completion.
        fs::write(
            tmp.path().join("specs/007-gate/tasks.md"),
            "# T\n\n- [x] done\n",
        )
        .unwrap();
        fs::write(tmp.path().join("unrelated.md"), "elsewhere\n").unwrap();
        git_commit_all(tmp.path(), "bookkeeping only");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
    }

    #[test]
    fn review_bookkeeping_does_not_make_a_review_stale() {
        // write-review touches review.md and the spec's frontmatter, so
        // counting them would make every review stale the instant it landed.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");

        seed_reviewed_at(tmp.path(), &base);
        fs::write(tmp.path().join("specs/007-gate/review.md"), "# Review\n").unwrap();
        git_commit_all(tmp.path(), "record the review");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
    }

    #[test]
    fn staleness_fails_open_on_an_unresolvable_sha() {
        // A gate that blocked on its own inability to check is one people
        // route around, so an unparseable `reviewed-against` passes.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN); // reviewed-against: abc123
        fs::create_dir_all(tmp.path().join("specs/007-gate/scenarios")).unwrap();
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();
        git_commit_all(tmp.path(), "base");
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\nchanged\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();
        git_commit_all(tmp.path(), "change a contract");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
    }

    #[test]
    fn lint_violations_block_before_review_state_is_consulted() {
        let tmp = tempdir().unwrap();
        // Even a never-reviewed spec reports the lint block first — the
        // gate's documented order.
        seed(tmp.path(), NEVER_REVIEWED);
        let violation = MarkdownViolation {
            path: "specs/007-gate/plan.md".into(),
            line: 12,
            rule: "MD012".into(),
            message: "Multiple consecutive blank lines".into(),
        };
        let canned = violation.clone();
        let result = run_with_lint(&args(), tmp.path(), move |_, _| {
            Ok(LintMarkdownResult {
                violations: vec![canned],
                clean: false,
                exit_code: 1,
            })
        })
        .unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MarkdownLint));
        assert_eq!(
            result.message.as_deref(),
            Some(
                "blocked: 1 markdownlint violation(s) in specs/007-gate — resolve them before completing"
            )
        );
        assert_eq!(result.violations, vec![violation]);
    }

    #[test]
    fn unparseable_lint_failure_blocks_with_exit_code() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), |_, _| {
            Ok(LintMarkdownResult {
                violations: vec![],
                clean: false,
                exit_code: 2,
            })
        })
        .unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MarkdownLint));
        assert_eq!(
            result.message.as_deref(),
            Some(
                "blocked: markdownlint-cli2 exited 2 for specs/007-gate — resolve the lint failure before completing"
            )
        );
        assert!(result.violations.is_empty());
    }

    #[test]
    fn lint_receives_the_recursive_feature_dir_glob() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let mut seen: Option<LintMarkdownArgs> = None;
        run_with_lint(&args(), tmp.path(), |lint_args, _| {
            seen = Some(lint_args.clone());
            clean_lint(lint_args, Path::new(""))
        })
        .unwrap();
        let seen = seen.unwrap();
        assert_eq!(seen.paths, vec!["specs/007-gate/**/*.md".to_string()]);
        assert!(!seen.fix, "the gate never lints in fix mode");
    }

    #[test]
    fn null_last_run_blocks_not_reviewed() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NEVER_REVIEWED);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotReviewed));
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: spec has not been reviewed — run /ductus:review before completing")
        );
        assert!(result.guidance.is_none());
    }

    #[test]
    fn absent_review_block_blocks_not_reviewed() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NO_REVIEW_BLOCK);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotReviewed));
    }

    #[test]
    fn blocking_review_blocks_with_must_count_and_waive_guidance() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_BLOCKING);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MustViolations));
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: spec has 3 MUST violation(s) — see specs/007-gate/review.md")
        );
        let guidance = result.guidance.unwrap();
        assert!(guidance.contains("re-run /ductus:review"), "{guidance}");
        assert!(guidance.contains("--waive <rule-id>"), "{guidance}");
    }

    #[test]
    fn honors_host_project_in_messages() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NEVER_REVIEWED);
        fs::write(
            tmp.path().join(".govern.toml"),
            "[host]\nproject = \"anvil\"\n",
        )
        .unwrap();
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: spec has not been reviewed — run /anvil:review before completing")
        );
    }

    #[test]
    fn honors_configured_specs_root() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("governance/007-gate")).unwrap();
        fs::write(
            tmp.path().join("governance/007-gate/spec.md"),
            REVIEWED_CLEAN,
        )
        .unwrap();
        fs::write(
            tmp.path().join(".govern.toml"),
            "[host]\nproject = \"ductus\"\n\n[paths]\nspecs-root = \"governance\"\n",
        )
        .unwrap();
        let mut seen_glob = String::new();
        let result = run_with_lint(&args(), tmp.path(), |lint_args, _| {
            seen_glob = lint_args.paths[0].clone();
            Ok(LintMarkdownResult {
                violations: vec![],
                clean: true,
                exit_code: 0,
            })
        })
        .unwrap();
        assert!(result.passed);
        assert_eq!(seen_glob, "governance/007-gate/**/*.md");
    }

    #[test]
    fn missing_feature_directory_errors() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let err = run_with_lint(
            &CheckReviewGateArgs {
                feature: "099-absent".into(),
            },
            tmp.path(),
            clean_lint,
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    /// Write a scenario file under the seeded feature.
    fn seed_scenario(repo: &Path, name: &str, body: &str) {
        let dir = repo.join("specs/007-gate/scenarios");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), body).unwrap();
    }

    const SCENARIO_WITH_QUESTIONS: &str = "---\nsection: Behavior\n---\n\n# Wire contract\n\n## Open Questions\n\n- Bracket operator or empty operand?\n";

    #[test]
    fn blocks_on_unresolved_scenario_questions_naming_the_scenario() {
        let tmp = tempdir().unwrap();
        // Review is current and lint is clean — only the scenario question
        // stands between this spec and `done`.
        seed(tmp.path(), REVIEWED_CLEAN);
        seed_scenario(tmp.path(), "wire-contract.md", SCENARIO_WITH_QUESTIONS);

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::ScenarioOpenQuestions)
        );
        let message = result.message.unwrap();
        assert!(
            message.contains("wire-contract"),
            "the blocked message must name the scenario, got: {message}"
        );
        assert!(message.contains('1'), "and the count, got: {message}");
        // The guidance points at the command that can actually resolve
        // them — scenario-targeted clarify, not feature-targeted.
        assert!(result.guidance.unwrap().contains("/ductus:clarify"));
    }

    #[test]
    fn scenario_questions_do_not_block_when_all_are_resolved() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        seed_scenario(
            tmp.path(),
            "settled.md",
            "---\nsection: Behavior\n---\n\n## Open Questions\n\n*None — captured during scenario authoring.*\n\n## Resolved Questions\n\n- **Answered** — yes.\n",
        );
        assert!(
            run_with_lint(&args(), tmp.path(), clean_lint)
                .unwrap()
                .passed
        );
    }

    #[test]
    fn markdown_lint_still_wins_over_the_scenario_check() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        seed_scenario(tmp.path(), "wire-contract.md", SCENARIO_WITH_QUESTIONS);

        // Both checks would block; gate order must report the first.
        let dirty = |_: &LintMarkdownArgs, _: &Path| {
            Ok(LintMarkdownResult {
                violations: vec![MarkdownViolation {
                    path: "specs/007-gate/spec.md".into(),
                    line: 3,
                    rule: "MD012".into(),
                    message: "Multiple consecutive blank lines".into(),
                }],
                clean: false,
                exit_code: 1,
            })
        };
        let result = run_with_lint(&args(), tmp.path(), dirty).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MarkdownLint));
    }

    #[test]
    fn scenario_questions_are_reported_before_a_missing_review() {
        let tmp = tempdir().unwrap();
        // Never reviewed AND carrying a scenario question: the scenario
        // check is ordered first, so the contributor resolves the design
        // question before being sent to run a review that would go stale.
        seed(tmp.path(), NEVER_REVIEWED);
        seed_scenario(tmp.path(), "wire-contract.md", SCENARIO_WITH_QUESTIONS);

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::ScenarioOpenQuestions)
        );
    }

    #[test]
    fn a_question_deferred_into_resolved_questions_does_not_block() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        // The convention spec 046 settles on: a question that is deferred
        // rather than undecided is resolved *with a condition* and lives in
        // Resolved Questions. There is deliberately no exemptible section
        // under Open Questions — that would let anything blocking be
        // relabelled to ship past this gate.
        seed_scenario(
            tmp.path(),
            "deferred.md",
            "---\nsection: Behavior\n---\n\n## Open Questions\n\n*None — all resolved.*\n\n## Resolved Questions\n\n- **Should Family 10 enforce uniqueness?** Deferred — revisit when the first sunset commit lands.\n",
        );
        assert!(
            run_with_lint(&args(), tmp.path(), clean_lint)
                .unwrap()
                .passed
        );
    }

    #[test]
    fn the_gate_guidance_offers_both_exits() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        seed_scenario(tmp.path(), "wire-contract.md", SCENARIO_WITH_QUESTIONS);
        let guidance = run_with_lint(&args(), tmp.path(), clean_lint)
            .unwrap()
            .guidance
            .unwrap();
        assert!(guidance.contains("clarify"), "resolve exit: {guidance}");
        assert!(
            guidance.contains("Resolved Questions"),
            "defer exit: {guidance}"
        );
    }

    #[test]
    fn an_unparseable_scenario_never_blocks_the_gate() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        // Truncated frontmatter and no questions section: nothing can be
        // proven about it, and an unknown is never escalated into a
        // done-blocking finding.
        seed_scenario(tmp.path(), "broken.md", "---\nsection: Behavior\n");
        assert!(
            run_with_lint(&args(), tmp.path(), clean_lint)
                .unwrap()
                .passed
        );
    }

    #[test]
    fn feature_without_scenarios_is_unaffected() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        assert!(
            run_with_lint(&args(), tmp.path(), clean_lint)
                .unwrap()
                .passed
        );
    }

    #[test]
    fn rejects_traversal_and_absolute_feature() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        for bad in ["../007-gate", "/etc", ""] {
            let err = run_with_lint(
                &CheckReviewGateArgs {
                    feature: bad.into(),
                },
                tmp.path(),
                clean_lint,
            )
            .unwrap_err();
            assert!(
                matches!(err, PrimitiveError::InvalidPath { .. }),
                "expected InvalidPath for {bad:?}"
            );
        }
    }
}
