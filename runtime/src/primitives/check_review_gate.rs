//! `check-review-gate` — evaluate `/ductus:implement`'s pre-done review gate.
//!
//! The deterministic surface behind the completion gate's step 13 (spec
//! 022, scenario coverage-expansion-primitives), which the host previously
//! walked by hand on every completion attempt: first the feature
//! directory's markdown lint (through the `lint-markdown` machinery,
//! replacing the raw `npx markdownlint-cli2` invocation), then unresolved
//! scenario open questions, then an undischarged `folds-into` fold, then
//! the spec frontmatter `review:` block, then whether the recorded review
//! is still current. The first failing
//! check wins and produces the canonical `blocked: …` message — with the
//! adopter's `[host] project` command namespace substituted into the
//! `/{project}:review` references — plus, on `must-violations`, the
//! resolve-or-waive guidance and, on `review-stale`, the re-run guidance.
//! A blocked gate is a domain outcome the host acts on (halt, do not
//! propose the transition), never an operational error.
//!
//! A **passing** gate can still carry `guidance`: the staleness check
//! compares committed trees, so it names any durable contract that was
//! uncommitted and therefore outside what it examined. That notice never
//! changes `passed` — see [`unexaminable_contracts_guidance`].

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
/// frontmatter block. Every gate verdict — including all six block
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

    // The spec's frontmatter, read once for gate checks 3 through 5.
    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, _body) = split_frontmatter(&content, &spec_path)?;
    let frontmatter: Frontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;
    let project = Host::load(repo).project;

    // Gate check 3: the spec has no undischarged fold.
    if let Some(blocked) = pending_fold_block(frontmatter.folds_into.as_deref(), &project) {
        return Ok(blocked);
    }

    // Gate checks 4 and 5: the spec frontmatter `review:` block.
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

    // Gate check 6: the recorded review still describes the current code.
    if let Some(stale) =
        stale_review_block(repo, &rel_dir, review.reviewed_against.as_deref(), &project)
    {
        return Ok(stale);
    }

    // Gate checks 7 and 8: the spec frontmatter `analyze:` block.
    if let Some(blocked) = analyze_gate_block(frontmatter.analyze.as_ref(), &project) {
        return Ok(blocked);
    }

    // The gate passes. Before saying so, name what it could not examine: the
    // staleness diff above compares committed trees, so a durable contract
    // living only in the working tree is outside it. At this moment that is
    // the normal state rather than an edge case — a scenario written during
    // the session is uncommitted, `reviewed-against` is HEAD, the diff is
    // empty, and a bare `passed: true` reads as "examined and current" when
    // nothing was examined. Reporting it does not block: committing before
    // reviewing is a workflow choice. It only stops the clean verdict from
    // being silent about its own blind spot, which is `QUAL-CLAIM-001`
    // applied to the gate itself — the same failure `stale_review_block`
    // already records against an unresolvable `reviewed-against`.
    Ok(CheckReviewGateResult {
        passed: true,
        blocked_by: None,
        message: None,
        guidance: unexaminable_contracts_guidance(repo, &rel_dir),
        violations: vec![],
    })
}

/// Gate checks 7 and 8 — the spec's `analyze:` block: a completed analysis
/// whose findings do not hold the spec out of `done`.
///
/// Ordered after every `review:` check because the pipeline is
/// `review → analyze → done`. A spec whose review is missing or failing has
/// not reached the point where analysis is the next thing owed, and naming the
/// later gate for an earlier defect sends a contributor to the wrong command.
///
/// **No grandfather clause, and there must not be one.** The `analyze-state-drift`
/// family exempts a `done` spec that predates the record, because it audits a
/// corpus written before the field existed. This gate runs at the moment a
/// spec is being completed *now*, so the record is always writable — an
/// exemption here would be a permanent hole rather than a bounded
/// transitional one.
///
/// Advisory findings never block; see `AnalyzeBlock::advisory` for why this
/// does not mirror the review gate's treatment of an outstanding SHOULD. They
/// ride the guidance line instead, together with the unexamined count, so a
/// blocked gate still says what the analysis did not look at.
fn analyze_gate_block(
    analyze: Option<&crate::schema::primitives::AnalyzeBlock>,
    project: &str,
) -> Option<CheckReviewGateResult> {
    let analyze = match analyze {
        Some(analyze) if analyze.last_run.is_some() => analyze,
        // Absent block or null `last-run`: the spec has never completed an
        // analysis. This is the state every spec was in before the record
        // existed, which is how two specs reached `done` on 2026-09-05 with
        // only the review gate run — one of them published to crates.io
        // before anything noticed, because nothing could.
        _ => {
            return Some(CheckReviewGateResult {
                passed: false,
                blocked_by: Some(ReviewGateBlock::NotAnalyzed),
                message: Some(format!(
                    "blocked: spec has not been analyzed — run /{project}:analyze before completing"
                )),
                guidance: None,
                violations: vec![],
            });
        }
    };

    if !analyze.blocking {
        return None;
    }

    Some(CheckReviewGateResult {
        passed: false,
        blocked_by: Some(ReviewGateBlock::AnalyzeFindings),
        message: Some(format!(
            "blocked: spec has {} hard-fail and {} blocking analyze finding(s) — \
             run /{project}:analyze to see them",
            analyze.hard_fail, analyze.blocking_findings
        )),
        guidance: Some(format!(
            "Resolve them and re-run /{project}:analyze. Advisory findings do not block; \
             the recorded run carries {} of them and {} unexamined target(s).",
            analyze.advisory, analyze.unexamined
        )),
        violations: vec![],
    })
}

/// Durable contracts under `rel_dir` with uncommitted changes — modified,
/// staged, or untracked. `None` when every one of them is committed, so the
/// common case stays quiet and its silence genuinely means "examined".
///
/// Scoped to the same durable contracts the staleness check uses, and for the
/// same reason: widening it to every dirty file under the feature would fire
/// on nearly every run (`tasks.md` is rewritten by `mark-task` on each task)
/// and be learned-ignored, which is worse than not reporting at all.
///
/// A git failure returns guidance rather than `None`. Returning `None` there
/// would emit the same value for "every contract is committed" and "the
/// working tree could not be inspected", which is the `QUAL-CLAIM-001`
/// conflation this whole function exists to remove one level up — a
/// self-inflicted instance of it, caught by this scenario's own review.
fn unexaminable_contracts_guidance(repo: &Path, rel_dir: &str) -> Option<String> {
    let cannot_inspect = |reason: &str| {
        Some(format!(
            "Staleness could not be determined: the working tree could not be inspected ({reason}), \
             so uncommitted durable contracts — if any — were not examined."
        ))
    };

    let Ok(repository) = git2::Repository::discover(repo) else {
        return cannot_inspect("no git repository found");
    };
    let mut options = git2::StatusOptions::new();
    options
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false)
        // Bound the walk to the feature tree. Without a pathspec this is a
        // full worktree status on every completion attempt, to answer a
        // question only about this spec's durable contracts.
        .pathspec(rel_dir);
    let Ok(statuses) = repository.statuses(Some(&mut options)) else {
        return cannot_inspect("git status failed");
    };

    let prefix = format!("{rel_dir}/");
    // `path()` errors on a non-UTF-8 path, which cannot be a durable contract
    // under the validated slug grammar anyway.
    let dirty: BTreeSet<String> = statuses
        .iter()
        .filter_map(|entry| entry.path().ok().map(|p| p.replace('\\', "/")))
        .filter(|path| path.strip_prefix(&prefix).is_some_and(is_durable_contract))
        .collect();

    if dirty.is_empty() {
        return None;
    }
    let shown: Vec<&str> = dirty.iter().take(3).map(String::as_str).collect();
    let more = dirty.len().saturating_sub(shown.len());
    let tail = if more > 0 {
        format!(" (+{more} more)")
    } else {
        String::new()
    };
    Some(format!(
        "Staleness could not be determined against {} uncommitted durable contract(s): {}{tail}. \
         The check compares committed trees, so these were not examined; commit them and re-run \
         the review if the recorded verdict should describe them.",
        dirty.len(),
        shown.join(", ")
    ))
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
/// A candidate that changed only by a repo-wide rename is **not** stale, per
/// [`crate::primitives::mechanical_sweep`]. This was missing until 2026-08-16
/// and the omission was expensive: measured across this repo, the un-exempted
/// rule called **19 of 46 `done` specs** stale, every one of them a
/// consequence of 049's `govern → ductus` sweep and none a real contract
/// change. `/{project}:audit` Family 19 had applied the exemption all along,
/// so the two enforcement moments disagreed on 19 specs while the whole point
/// of scoping them identically was that they would not. The incident that
/// exposed it — `017-derive-dont-ask` blocking on three contracts — was itself
/// a false positive: all three changed in the rename commit.
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
    // `revparse_single`, not `Oid::from_str`: the latter zero-pads a short hex
    // string into a 40-char id that matches nothing, so an abbreviated
    // `reviewed-against` — `012-multi-agent-govern` records `d904430` — made
    // the whole check fail open with no signal, while Family 19's
    // `git cat-file -e` resolved it and checked. A check that silently cannot
    // run is the failure mode this repo pays for most.
    let base_tree = repository
        .revparse_single(base)
        .ok()?
        .peel_to_commit()
        .ok()?
        .tree()
        .ok()?;
    let head_tree = repository.head().ok()?.peel_to_commit().ok()?.tree().ok()?;
    let diff = repository
        .diff_tree_to_tree(Some(&base_tree), Some(&head_tree), None)
        .ok()?;

    let prefix = format!("{rel_dir}/");
    let mut candidates: BTreeSet<String> = BTreeSet::new();
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path().or_else(|| delta.old_file().path()) {
                let path = path.to_string_lossy().replace('\\', "/");
                if let Some(rest) = path.strip_prefix(&prefix)
                    && is_durable_contract(rest)
                {
                    candidates.insert(path);
                }
            }
            true
        },
        None,
        None,
        None,
    )
    .ok()?;

    // A contract that changed only in spelling states what it stated before,
    // so a repo-wide rename must not stale the review — the same §spec-lifecycle
    // case (a) rule that keeps the sweep from reopening the spec. Building the
    // index is the expensive step, so it is skipped entirely when nothing is a
    // candidate, which is the common case.
    let stale: BTreeSet<String> = if candidates.is_empty() {
        candidates
    } else {
        let index = crate::primitives::mechanical_sweep::SweepIndex::build(
            &repository,
            &base_tree,
            &head_tree,
        );
        candidates
            .into_iter()
            .filter(|path| index.changed_beyond_spelling(path))
            .collect()
    };

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

/// The pending-fold gate check: `Some(blocked)` when the spec declares a
/// `folds-into` target, `None` when it declares none.
///
/// Ordered beside the scenario-open-questions check and ahead of the
/// `review:` block because it is the same kind of claim: a spec carrying an
/// obligation nobody has discharged is not a candidate for `done`, so
/// whether its review is fresh does not yet matter. The consequence is that
/// the branch-scoped form has no `done` state at all — it is retired by
/// fold-back, not completed, which is the honest reading of a staging spec
/// (spec 051 AC35).
///
/// **Presence only** — the target's existence is deliberately not checked.
/// A branch-scoped spec exists because upstream moved, so before the merge
/// its target normally lives on the upstream branch; requiring it to resolve
/// would block the feature's normal case over a tree this gate cannot see.
/// `retire-feature` enforces existence at fold-back, in the first tree
/// holding both.
///
/// Absence is never a finding. A sequential spec has no fold target by
/// definition, and removing the key by hand is the supported way to make a
/// branch-scoped spec stand on its own.
fn pending_fold_block(folds_into: Option<&str>, project: &str) -> Option<CheckReviewGateResult> {
    let target = folds_into?;
    Some(CheckReviewGateResult {
        passed: false,
        blocked_by: Some(ReviewGateBlock::PendingFold),
        message: Some(format!(
            "blocked: spec folds into {target} and the fold has not happened — \
             resolve it before completing"
        )),
        guidance: Some(format!(
            "Run /{project}:fold to fold this spec into {target} and retire its \
             directory: a branch-scoped spec is retired by fold-back, not completed. \
             A spec that should stand on its own instead is renamed to the sequential \
             NNN- form with its folds-into key removed."
        )),
        violations: vec![],
    })
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

    /// Reviewed clean, but the analyze record is absent entirely — the state
    /// every spec was in before this block existed, and the one that let two
    /// specs reach `done` (and one of them crates.io) on a single gate.
    const NEVER_ANALYZED: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\n---\n\n# 007 — Gate\n";

    /// The block is present but `last-run` is null — the same "never ran"
    /// state spelled differently, and the review gate treats its own
    /// equivalent identically.
    const ANALYZE_NULL_RUN: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: null\n  blocking: false\n---\n\n# 007 — Gate\n";

    const ANALYZE_BLOCKING: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 1\n  blocking-findings: 2\n  advisory: 0\n  unexamined: 0\n  blocking: true\n---\n\n# 007 — Gate\n";

    /// Advisory findings and unexamined targets recorded, nothing gating.
    const ANALYZE_ADVISORY_ONLY: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 7\n  unexamined: 4\n  blocking: false\n---\n\n# 007 — Gate\n";

    const REVIEWED_CLEAN: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 1\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const REVIEWED_BLOCKING: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 3\n  should-violations: 0\n  low-confidence: 0\n  blocking: true\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const NEVER_REVIEWED: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: null\n  reviewed-against: null\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const NO_REVIEW_BLOCK: &str = "---\nstatus: in-progress\ndependencies: []\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";

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

    /// A branch-scoped spec: reviewed, clean, and every other check would
    /// pass — the fold is the only thing holding it short of `done`.
    const PENDING_FOLD: &str = "---\nstatus: in-progress\ndependencies: []\nfolds-into: 050-upstream\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";

    #[test]
    fn a_declared_fold_blocks_the_done_transition() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), PENDING_FOLD);

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::PendingFold));
        assert_eq!(
            result.message.as_deref(),
            Some(
                "blocked: spec folds into 050-upstream and the fold has not happened — resolve it before completing"
            )
        );
        let guidance = result.guidance.expect("the block carries guidance");
        assert!(guidance.contains("/ductus:fold"), "{guidance}");
        assert!(guidance.contains("retired by fold-back"), "{guidance}");
    }

    /// The block does not depend on the target resolving, and deliberately:
    /// a branch-scoped spec exists because upstream moved, so before the
    /// merge its target normally lives on the branch this tree forked from.
    /// Checking resolvability here would refuse the feature's normal case
    /// over a tree the gate cannot see.
    #[test]
    fn the_fold_block_does_not_depend_on_the_target_existing() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), PENDING_FOLD);
        // Nothing named `050-upstream` is anywhere in this corpus.
        assert!(!tmp.path().join("specs/050-upstream").exists());

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert_eq!(result.blocked_by, Some(ReviewGateBlock::PendingFold));
    }

    /// Ordering: an unresolved scenario question is the more upstream
    /// defect and keeps the cell, so the fold block is not what a
    /// contributor is sent to fix first.
    #[test]
    fn a_scenario_question_outranks_a_pending_fold() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), PENDING_FOLD);
        fs::create_dir_all(tmp.path().join("specs/007-gate/scenarios")).unwrap();
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/a.md"),
            "---\nsection: X\n---\n\n# A\n\n## Open Questions\n\n- Which way?\n",
        )
        .unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::ScenarioOpenQuestions)
        );
    }

    /// The converse, so the new check cannot be read as blocking every
    /// spec: absence of the key is never a finding.
    #[test]
    fn a_spec_without_a_fold_target_is_untouched_by_the_check() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert_ne!(result.blocked_by, Some(ReviewGateBlock::PendingFold));
    }

    /// The gap this block closes. A spec with a clean review and no analyze
    /// record used to pass the gate — which is how two specs reached `done`
    /// on 2026-09-05 with only half the pipeline run, one of them published
    /// irreversibly before anything noticed, because nothing could.
    #[test]
    fn a_reviewed_but_unanalyzed_spec_is_blocked() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NEVER_ANALYZED);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotAnalyzed));
        assert!(
            result
                .message
                .as_deref()
                .unwrap()
                .contains("has not been analyzed")
        );
    }

    #[test]
    fn a_null_analyze_last_run_blocks_like_an_absent_block() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_NULL_RUN);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotAnalyzed));
    }

    #[test]
    fn blocking_analyze_findings_block_and_name_both_tiers() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_BLOCKING);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AnalyzeFindings));
        let message = result.message.as_deref().unwrap();
        assert!(message.contains("1 hard-fail"), "{message}");
        assert!(message.contains("2 blocking"), "{message}");
    }

    /// Advisory findings are recorded and never gate — the deliberate
    /// asymmetry with the review gate's treatment of an outstanding SHOULD.
    /// The unexamined count rides along in the guidance rather than being
    /// dropped, so a passing gate still says what the analysis could not look
    /// at.
    #[test]
    fn advisory_findings_and_unexamined_targets_do_not_block() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_ADVISORY_ONLY);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed);
        assert!(result.blocked_by.is_none());
    }

    /// Gate order: a spec failing both gates is told about the review, not
    /// the analysis. Naming the later gate for an earlier defect sends a
    /// contributor to the wrong command.
    #[test]
    fn a_missing_review_is_reported_before_a_missing_analysis() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# 007 — Gate\n",
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotReviewed));
    }

    #[test]
    fn passes_when_lint_clean_and_review_current() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed);
        assert!(result.blocked_by.is_none());
        assert!(result.message.is_none());
        assert!(result.violations.is_empty());
        // This fixture is a bare tempdir with no git repository, so the gate
        // genuinely cannot inspect the working tree and says so. The
        // assertion used to read `guidance.is_none()`, which is precisely the
        // conflation this scenario removes: silence has to mean "examined and
        // clean", and it cannot mean that here. The genuinely-clean case is
        // `a_clean_tree_emits_no_unexaminable_guidance`, which commits first.
        assert!(
            result
                .guidance
                .is_some_and(|g| g.contains("could not be inspected")),
            "a non-repo fixture must not read as an examined-clean tree"
        );
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
                 low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n"
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
    fn a_repo_wide_rename_sweep_does_not_stale_the_review() {
        // The 019 defect, as a test. A uniform token substitution across live
        // artifacts is a mechanical edit: it does not reopen a done spec, so it
        // must not stale its review either. Measured before the fix, the
        // un-exempted rule called 19 of this repo's 46 done specs stale, all of
        // them this shape. Two files, because uniformity is a repo-wide
        // property — a rewrite in one file only is a contract change.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        fs::create_dir_all(tmp.path().join("specs/007-gate/scenarios")).unwrap();
        for (path, body) in [
            (
                "specs/007-gate/scenarios/retry.md",
                "# Retry\n\nRun `/govern` to sync.\n",
            ),
            (
                "specs/007-gate/data-model.md",
                "# Model\n\nWritten by `/govern`.\n",
            ),
            ("docs/elsewhere.md", "Run `/govern` to sync.\n"),
        ] {
            let full = tmp.path().join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(full, body).unwrap();
        }
        let base = git_commit_all(tmp.path(), "base");

        seed_reviewed_at(tmp.path(), &base);
        for (path, body) in [
            (
                "specs/007-gate/scenarios/retry.md",
                "# Retry\n\nRun `/ductus` to sync.\n",
            ),
            (
                "specs/007-gate/data-model.md",
                "# Model\n\nWritten by `/ductus`.\n",
            ),
            ("docs/elsewhere.md", "Run `/ductus` to sync.\n"),
        ] {
            fs::write(tmp.path().join(path), body).unwrap();
        }
        git_commit_all(tmp.path(), "sweep: govern -> ductus");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(
            result.passed,
            "a rename sweep must not stale the review: {result:?}"
        );
    }

    #[test]
    fn a_real_contract_change_inside_a_rename_sweep_still_stales() {
        // The exemption must not become a blanket amnesty: a meaning change
        // riding along with a sweep is still a contract change, because its
        // rewrite is not one the sweep made anywhere else.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        fs::create_dir_all(tmp.path().join("specs/007-gate/scenarios")).unwrap();
        for (path, body) in [
            (
                "specs/007-gate/scenarios/retry.md",
                "# Retry\n\nRun `/govern`. Timeout is 30s.\n",
            ),
            ("docs/elsewhere.md", "Run `/govern` to sync.\n"),
        ] {
            let full = tmp.path().join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(full, body).unwrap();
        }
        let base = git_commit_all(tmp.path(), "base");

        seed_reviewed_at(tmp.path(), &base);
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\nRun `/ductus`. Timeout is 60s.\n",
        )
        .unwrap();
        fs::write(
            tmp.path().join("docs/elsewhere.md"),
            "Run `/ductus` to sync.\n",
        )
        .unwrap();
        git_commit_all(tmp.path(), "sweep plus a timeout change");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed, "{result:?}");
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::ReviewStale));
        assert!(
            result.message.unwrap().contains("scenarios/retry.md"),
            "the finding must name the contract that actually changed"
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

    // --- unexaminable contracts: what the gate could not look at -------------

    #[test]
    fn an_untracked_scenario_is_reported_as_unexaminable() {
        // The observed 2026-08-27 shape. `create-scenario` wrote the file this
        // session, so it exists on disk and in no tree the staleness diff
        // consults; `reviewed-against` is HEAD, the diff is empty, and without
        // this the gate reports a bare clean verdict.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");
        seed_reviewed_at(tmp.path(), &base);

        fs::create_dir_all(tmp.path().join("specs/007-gate/scenarios")).unwrap();
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/fresh.md"),
            "# Fresh\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "the notice must not block: {result:?}");
        assert!(result.blocked_by.is_none());
        assert!(result.message.is_none());
        let guidance = result
            .guidance
            .expect("clean verdict must name its blind spot");
        assert!(guidance.contains("could not be determined"), "{guidance}");
        assert!(guidance.contains("scenarios/fresh.md"), "{guidance}");
    }

    #[test]
    fn a_modified_committed_contract_is_reported_as_unexaminable() {
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

        // Edited but not committed: invisible to a tree-to-tree diff.
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\nUncommitted contract change.\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        let guidance = result.guidance.expect("dirty contract must be named");
        assert!(guidance.contains("scenarios/retry.md"), "{guidance}");
    }

    #[test]
    fn a_staged_only_change_is_reported_as_unexaminable() {
        // Staged is still not committed, so the diff cannot see it either.
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
            "# Retry\n\nStaged change.\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();
        let repository = git2::Repository::open(tmp.path()).unwrap();
        let mut index = repository.index().unwrap();
        index
            .add_path(Path::new("specs/007-gate/scenarios/retry.md"))
            .unwrap();
        index.write().unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        assert!(
            result.guidance.unwrap().contains("scenarios/retry.md"),
            "a staged-only change is still uncommitted"
        );
    }

    #[test]
    fn a_genuine_stale_block_wins_over_the_unexaminable_notice() {
        // Both conditions at once. The notice is not a softer substitute for a
        // check that actually fired, so the block must survive.
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

        // Committed change -> genuinely stale.
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\nCommitted contract change.\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();
        git_commit_all(tmp.path(), "stale it");
        // Plus an uncommitted one.
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/other.md"),
            "# Other\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed, "{result:?}");
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::ReviewStale));
        assert!(result.message.unwrap().contains("review is stale"));
    }

    #[test]
    fn a_clean_tree_emits_no_unexaminable_guidance() {
        // The common case stays quiet, so its silence means "examined".
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
        git_commit_all(tmp.path(), "commit the reviewed-at edit");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        assert!(result.guidance.is_none(), "{:?}", result.guidance);
    }

    #[test]
    fn dirty_bookkeeping_files_are_not_reported_as_unexaminable() {
        // `tasks.md` is rewritten by `mark-task` on every task, so widening the
        // scope past durable contracts would fire on nearly every run and be
        // learned-ignored. Same scoping as the staleness check itself.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");
        seed_reviewed_at(tmp.path(), &base);
        git_commit_all(tmp.path(), "commit the reviewed-at edit");

        for name in ["tasks.md", "plan.md", "review.md"] {
            fs::write(tmp.path().join("specs/007-gate").join(name), "# churn\n").unwrap();
        }

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        assert!(
            result.guidance.is_none(),
            "bookkeeping churn is not a contract: {:?}",
            result.guidance
        );
    }

    #[test]
    fn a_working_tree_that_cannot_be_inspected_is_not_reported_as_clean() {
        // The QUAL-CLAIM-001 finding this scenario's own review raised against
        // it: `None` for "could not look" is the same value as `None` for
        // "nothing dirty", so a caller reads a bare pass as assurance. Not a
        // git repository at all is the reachable form — `stale_review_block`
        // returns before touching git when `reviewed-against` is empty, so the
        // discovery here can be the first one attempted.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);

        let guidance = unexaminable_contracts_guidance(tmp.path(), "specs/007-gate");
        let guidance = guidance.expect("an uninspectable tree must not read as clean");
        assert!(guidance.contains("could not be inspected"), "{guidance}");
        assert!(
            !guidance.contains("uncommitted durable contract(s):"),
            "it must not claim to have found specific files: {guidance}"
        );
    }
}
