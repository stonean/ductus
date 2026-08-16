---
section: "Follow-on scenarios"
---

# Review-staleness-on-done-specs

## Context

[review-staleness-gate](review-staleness-gate.md) added `check-review-gate`'s `ReviewStale` block: a spec cannot reach `done` while durable contracts in its plan's Affected Files have changed since its recorded `reviewed-against`. That closes the transition. It does not close the state.

The gate fires **only** on `in-progress → done`. Once a spec is `done`, nothing re-evaluates its verdict, so contracts can move underneath it indefinitely and the recorded review goes on describing code that no longer exists. `check-artifacts`' `review-state-drift` family looks adjacent but is not: [022's data-model](../data-model.md) defines it as *"a `done` spec with `review.last-run` unset or `review.blocking: true`"* — a missing or blocking review, never a stale one. So a `done` spec whose review is merely out of date is reported clean by both mechanisms.

Observed 2026-08-16. `017-derive-dont-ask` was taken through the back-edge for an unrelated reason and its gate immediately reported staleness against **four** durable contracts. Three of them — `data-model.md`, `detect-dependency-cycles.md`, `skip-prose-cross-references.md` — had changed during the `0.28.0` cycle, while 017 sat at `done`. Its recorded verdict dated from 2026-08-02 and described the repository as it was before the release. Nothing surfaced that for two weeks, and nothing would have: it became visible only because an unrelated edit forced a transition, which is the definition of an undetected state rather than an accepted one.

The `0.28.0` cycle swept the whole corpus, so 017 is unlikely to be alone. This is the `QUAL-CLAIM-001` shape applied to the pipeline's own bookkeeping — a `done` spec presents a review verdict, and a reader cannot distinguish one that still describes the code from one that has not been looked at since the code moved.

## Behavior

**`check-artifacts`' `review-state-drift` family gains a staleness arm.** On a `done` spec it already reports an unset `review.last-run` and a `review.blocking: true`; it additionally reports a recorded `reviewed-against` against which durable contracts have since changed, naming the count and the first few paths exactly as `check-review-gate`'s block message does.

**The staleness computation is shared, not reimplemented.** The family calls the same code path `check-review-gate` uses — a second implementation is the drift this repo has already been bitten by, and two answers to "is this review stale?" would be worse than none. That path is `stale_review_block` plus `is_durable_contract` (a `scenarios/*.md` file or `data-model.md`), **not** `compute_review_scope::read_plan_affected`; see the first Resolved Question for why this paragraph originally named the wrong one and what following it would have cost.

**The finding is advisory, not blocking.** The existing `review-state-drift` arms are blocking because they describe a spec that never had a usable review; a stale review *was* usable and has aged, which is a different claim. Advisory also keeps the change from mass-reopening the corpus: `/{project}:analyze --fix` reverts `done → in-progress` on blocking findings, and a blocking staleness arm would sweep every spec the `0.28.0` cycle touched into `in-progress` in one run. The operator decides which stale verdicts are worth re-earning.

**The grandfather rule carries over unchanged.** A `done` spec with no `review:` block at all predates `/{project}:review` and is exempt, exactly as it is for the existing arms. A spec with a `review:` block but no resolvable `reviewed-against` (a rebased or pruned sha) is reported as *unresolvable* rather than as clean or stale — the distinction this scenario exists to preserve.

**Fail-open matches the gate.** Where `check-review-gate` fails open — no git repository, an unresolvable `reviewed-against`, a plan with no Affected Files table — the family reports the target under `skipped` with its reason rather than emitting a clean result, so a family that could not examine a spec is never indistinguishable from one that examined it and found nothing.

## Edge Cases

- A `done` spec whose contracts have not moved since `reviewed-against`: no finding, and it is counted as examined rather than skipped.
- A spec at any status other than `done`: out of scope — `check-review-gate` already covers the transition, and a pre-`done` spec's review is expected to be provisional.
- A mechanical sweep (a rename, a criterion-label backfill) that touches a durable contract without changing its meaning: a uniform substitution must not report every `done` spec stale. `review-freshness.sh` has always carried this exemption; the shared Rust path did **not** inherit it, which is the defect the measurement found and the second Resolved Question records. It does now, via `primitives::mechanical_sweep`.
- `reviewed-against` names a commit absent from a shallow clone: the CI checkout that runs this needs `fetch-depth: 0`, the same requirement Family 19 already carries and was already bitten by.
- The `--fix` interaction: advisory findings do not revert status, so `--fix` leaves a stale-but-done spec alone and says so.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Which code path does `check-review-gate` actually share?**
  `stale_review_block` and `is_durable_contract`, not
  `compute_review_scope::read_plan_affected`. Resolved 2026-08-16: this scenario's Behavior section and 022 task 88 both
  named `read_plan_affected`, and `check_review_gate.rs` references it nowhere.
  Following the letter would have been actively harmful — that function's own
  doc comment records that the Affected-Files scoping *was* tried and rejected
  because it blocked **34 of 48** specs (old specs list shared surfaces like
  `AGENTS.md` that every later spec touches), and adopting it here would have
  created exactly the second matching rule the same paragraph forbids. Both
  texts corrected.

- **How many `done` specs would the arm flag, and is advisory enough?**
  Measured 2026-08-16, before the design was finalised, as this scenario
  requires: **19 of 46**, and **all 19 were false positives**. Every one was a
  consequence of 049's `govern → ductus` rename — `005-workflows`, for
  instance, differed only in `/govern` → `/ductus` inside its `data-model.md`.
  Advisory would not have saved it: a family that reports noise on 41% of the
  corpus is one people stop reading, which is the failure the gate's own
  history already records at 34-of-48.

  The cause was the third error: this scenario asserted that the shared path
  inherits `review-freshness.sh`'s mechanical-sweep exemption. It did not.
  Family 19 had the exemption and reported **0**; the Rust gate had none and
  would have reported 19. Two implementations of one rule, disagreeing on 19
  specs, with nothing comparing them — the drift this scenario's own Behavior
  section says would be "worse than none", already present.

  The founding observation was itself an instance: `017-derive-dont-ask`'s
  three "stale" contracts all changed in exactly one commit, `9da4a7a
  feat(049): sweep the framework, scripts, and specs onto ductus`. The gate
  fired on a rename.

- **What was done about it?** The exemption was ported to Rust as
  `primitives::mechanical_sweep` and wired into `stale_review_block`, so the
  transition gate stops firing on sweeps. A third divergence surfaced while
  testing it: `git2::Oid::from_str` zero-pads an abbreviated sha into an id
  that matches nothing, so the gate failed **open** on
  `012-multi-agent-govern`'s `d904430` while Family 19's `git cat-file -e`
  resolved it — fixed with `revparse_single`.

  Family 19 keeps its own Python implementation: the CI job that runs the
  self-audit has no Rust toolchain and no runtime build, because it *gates*
  the build, so calling the runtime would make the gate depend on compiling
  the artifact it gates. The `mechanical_sweep_parity` integration test runs
  both over the real corpus and fails on the first disagreement, so the two
  cannot drift again in silence. It compares the 19 specs that were previously
  divergent and asserts it compared something, so a vacuous pass is itself a
  failure.

- **Does the `check-artifacts` staleness arm still need building?** Undecided,
  and deliberately left so. With the exemption in place both mechanisms now
  report 0 on this corpus, so the arm would add a third caller of a rule that
  currently finds nothing. The state this scenario describes — a `done` spec
  whose review has genuinely aged — remains uncovered between releases, but the
  case for the arm should be re-argued against a corpus where the rule actually
  fires, rather than inherited from a premise that has been shown false.
