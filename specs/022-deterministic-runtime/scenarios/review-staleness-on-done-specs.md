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

The `check-artifacts` staleness arm this scenario first specified was not built; the fourth Resolved Question records why. What it delivered is in `check-review-gate`'s review-record freshness.

**A rename sweep does not stale a review.** The freshness computation — `analyze_subjects::freshness_of` over `is_review_contract` (a `scenarios/*.md` file or `data-model.md`), from whose result `stale_review_block` builds the block — drops a changed contract whose change since `reviewed-against` was a uniform repo-wide rename, through the `primitives::mechanical_sweep` exemption `review-freshness.sh` has always carried. When the two trees it needs are unavailable, the exemption does not run and every candidate is reported.

**An abbreviated `reviewed-against` resolves.** The recorded sha is resolved with `revparse_single`, so a short sha names its commit rather than a zero-padded id that matches nothing.

**The existing `review-state-drift` arms keep their grandfather rule.** A `done` spec with no review record at all predates `/{project}:review` and is exempt — keyed on the absence of `review.md` since spec 057, and that re-keying was not cosmetic: keyed on the block's absence, the relocation made the predicate true of every spec and the family would have exempted the whole corpus while exiting green.

## Edge Cases

- A mechanical sweep (a rename, a criterion-label backfill) that touches a durable contract without changing its meaning: a uniform substitution must not report every `done` spec stale. `review-freshness.sh` has always carried this exemption; the shared Rust path did **not** inherit it, which is the defect the measurement found and the second Resolved Question records. It does now, via `primitives::mechanical_sweep`.
- `reviewed-against` names a commit absent from a shallow clone: the rename exemption cannot run, so its candidates are reported rather than dropped. A CI checkout needs `fetch-depth: 0`, the same requirement Family 19 already carries and was already bitten by.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Which code path does `check-review-gate` actually share?**
  `analyze_subjects::freshness_of` with `is_review_contract` — from whose
  result `stale_review_block` only builds the block — not
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
  section said would be "worse than none", already present.

  The founding observation was itself an instance: `017-derive-dont-ask`'s
  three "stale" contracts all changed in exactly one commit, `9da4a7a
  feat(049): sweep the framework, scripts, and specs onto ductus`. The gate
  fired on a rename.

- **What was done about it?** The exemption was ported to Rust as
  `primitives::mechanical_sweep` and wired into the gate's freshness
  computation (now `analyze_subjects::freshness_of`), so the
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

- **Does the `check-artifacts` staleness arm still need building?** No — resolved 2026-08-16: `framework-checks.yml` runs Family 19 over every `done` spec on every push to `main`, so contracts never move unwatched, and the two-week silence the Context describes was the rename exemption correctly holding; what remains is a visibility gap, better closed by surfacing Family 19's verdict in `/{project}:analyze` than by a third implementation of the rule.
