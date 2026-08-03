---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-03T13:59:06Z
reviewed-against: 52b89a7a55d9c068e0667f5585b2dea8c5d8d900
diff-base: 1f7ee722e3c8ae91f7cd4d03aeeca9de7032c6b0
must-violations: 0
should-violations: 2
low-confidence: 1
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Re-review covering the two 022 scenarios that landed after the prior pass — `criterion-adopter-scope-destinations` and the allocation follow-on to `numbered-heading-grammar-single-source`. **This review should have run before `gvrn-v0.26.2` was tagged and did not**: the prior record read `reviewed-against: 1f7ee722` while the tag sits at `334907f`, three commits later, and `check-review-gate` passed anyway because it tests for a `review.last-run` and a false `blocking` flag, never that `reviewed-against` matches HEAD. The gate cannot see a stale review; that gap is recorded as a SHOULD below. **0 MUST — not blocking.** The runtime diff is 187 lines across three files: the manifest-derived adopter-scope suppression in `check_artifacts.rs`, the `prune_tasks` allocation fix, and a doc-comment line in `primitives.rs`. Rule files loaded: the backend + cross set. Security is clean and the one security-adjacent surface improves — `adopter_destinations` reads a fixed repo-relative path with no caller input, so there is no traversal surface, and the new arm only ever *suppresses* a finding after the path has already failed to resolve, so it cannot mask a live path. `QUAL-CLAIM-001` holds: every suppressed candidate lands in `skipped` under the new closed-set reason `ships-to-adopter`, so `clean: true` with a non-empty `skipped` keeps its partially-examined meaning; the reason is documented in all four homes (022 data-model, the `check-artifacts-skipped-targets` scenario, 045's data-model, and `SkippedTarget`'s doc comment). Failure direction is correct throughout: a missing or unparseable manifest yields an empty set, which means nothing is suppressed and findings are still emitted — the opposite of Family 17's fail-closed requirement, and right for this direction, since here an empty derivation means checking *everything*. Verified rather than asserted: 860 lib tests plus 11 suites green, with the parity suite passing **unblessed** after the analyze capture step was corrected — which is what proves that fix was to stop dispatching `append-inbox` rather than to accept a changed stream. `clippy -D warnings` and `fmt` clean, markdownlint clean across 388 files, the 18-family self-audit exit 0, and a full `/gov:analyze` pass over the touched specs: frontmatter valid, dependencies compatible with no cycles, generator drift none, rule-ID citations clean, and repo-wide `criterion-path-existence` at zero findings.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-PROCESS — check-review-gate cannot detect a review that is stale against HEAD

- **File**: `specs/022-deterministic-runtime/review.md:3-5`
- **Rule**: AGENTS.md §Design Principles — never depend on human diligence; the framework makes state visible at the next gate rather than relying on the agent remembering.
- **Finding**: `gvrn-v0.26.2` was tagged at `334907f` while 022's review record read `reviewed-against: 1f7ee722` — three commits of runtime change, including this scenario's entire implementation, shipped without a review pass. Nothing caught it. `check-review-gate` asserts that `review.last-run` is set and `review.blocking` is false; it never compares `reviewed-against` to HEAD, so a review that predates the code it nominally covers reads as a pass. `/gov:analyze`'s `review-state-drift` family has the same blind spot — it flags an unset `last-run` or a true `blocking`, not a stale sha. Editing a `review.md` by hand to mark findings resolved (which is what happened here) therefore produces a record that satisfies every automated check while describing a diff that no longer exists.
- **Auto-fixable**: no
- **Suggested fix**: Give the staleness a gate. `check-review-gate` already reads the spec frontmatter and the runtime already resolves HEAD for `write-review`; comparing `review.reviewed-against` against the current sha — or, more usefully, against the last commit touching the spec's plan-affected scope — would turn this from invisible into a blocking or advisory finding. Scope it carefully: an exact-sha match would fire on every unrelated commit, so the useful test is whether any file in the review's resolved scope changed since `reviewed-against`. Route to 022 as a `check-review-gate` change with a companion `review-state-drift` arm in `check-artifacts`.

### SHOULD: QUAL-GROUND-001 — a second, unguarded parser of the Shared Files manifest now exists

- **File**: `runtime/src/primitives/check_artifacts.rs:1013-1050`
- **Rule**: Code whose correctness depends on an external contract it does not own — a database schema, another service's API shape, a config key, a file or wire format — SHOULD bind to that contract's canonical source rather than restating it.
- **Finding**: `adopter_destinations` parses the **Shared Files** manifest's markdown table shape out of `framework/bootstrap/govern.md`. That is the right canonical source — but the runtime is now the *second* independent parser of that same table: `scripts/audit/host-namespace-parity.sh` (Family 17) derives the agent config dirs from it in awk, with its own notion of which column is which. Nothing compares the two. A table reshaped in a way one parser tolerates and the other does not — a reordered column, a wrapped cell, a second backticked span — leaves them silently disagreeing, and this parser's failure is partial rather than total: rows it still matches are suppressed while rows it now misses are reported, so the output looks plausible either way.
- **Auto-fixable**: no
- **Suggested fix**: The failure direction is already the safe one — a total parse failure yields an empty set, suppresses nothing, and reports every finding — so this is not urgent. The durable fix is the Family 18 pattern applied to the manifest: an audit family asserting that the Rust and shell derivations of `framework/bootstrap/govern.md` agree on the destination set, failing when either yields nothing. That would also cover Family 17's derivation, which is currently guarded only by its own emptiness check.

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-EFFICIENCY — the manifest is re-read once per feature under --all

- **File**: `runtime/src/primitives/check_artifacts.rs:911-913`
- **Rule**: Flag repeated work on paths where a cheaper form is available.
- **Finding**: `adopter_destinations` is called once per `check_criterion_path_existence` invocation, which is once per feature. `/{project}:analyze --all` iterates all 48 specs, so `framework/bootstrap/govern.md` (~900 lines) is read and table-parsed 48 times to produce an identical 53-entry set. The read is already hoisted out of the per-candidate loop, which was the important move; hoisting further would mean caching across primitive invocations, and the primitive boundary is per-feature by design. Recorded low-confidence because the cost is a few milliseconds against a run that already reads every spec, plan, task file, and scenario.
- **Auto-fixable**: no
- **Suggested fix**: Leave as-is unless `--all` shows up in profiling. A cache would have to live above the primitive boundary — in the host's `--all` loop — which trades a clean per-feature contract for a saving that is not currently measurable.

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
