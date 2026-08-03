---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-03T04:08:54Z
reviewed-against: 1f7ee722e3c8ae91f7cd4d03aeeca9de7032c6b0
diff-base: 2bd364ddd775cc1dd231601280e1529f4627ee84
must-violations: 0
should-violations: 0
low-confidence: 2
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Review of the four runtime scenarios landed since `2bd364d` — `numbered-heading-grammar-single-source`, `config-resolution-single-probe`, `sibling-symlink-trust-boundary`, and `criterion-non-assertion-phrasings` — plus the review-record reconciliation and the `/gov:review` reconciliation rule. **0 MUST — not blocking.** Rule files loaded: the backend + cross set (api, concurrency, configuration, observability, performance, quality, reliability, security); the frontend files were not selected (no frontend surface in scope). The security rules govern authentication, credentials, sessions, tokens, and outbound calls — none of which this change introduces; the runtime opens no network, handles no secrets, and persists no credentials. The one security-adjacent change moves in the safe direction: `traverses_symlink` closes a path-traversal gap by refusing to follow a symlink committed inside a feature directory, and it does so with `symlink_metadata` (which does not follow links) so the repeat-run determinism AC8 requires is preserved rather than traded away. `QUAL-CLAIM-001` is honored throughout — every newly-suppressed subject lands in `skipped` with a closed reason (`target-unparseable` for a symlinked sibling, `not-a-live-claim` for an exempted criterion) rather than being silently dropped, so a partially-examined feature can never read as verified-clean. One SHOULD was recorded and is now **resolved in-review**: the non-assertion marker list existed in four places with no mechanical binding, and this change had to hand-sync three of them — closed by `/gov:audit` Family 18 (`marker-list-parity.sh`), which derives the list from its canonical source and fails closed when the derivation yields nothing, landed under 026 as the audit's home spec. Two low-confidence notes cover a small eager-allocation change in `prune_tasks` and the post-`done` edit to 045's data-model. Verification at this HEAD: 857 lib tests plus 10 integration suites green (11/11), `clippy -D warnings` clean, `cargo fmt` clean, markdownlint 0 issues across 382 files, framework self-audit exit 0, and `check-artifacts` reporting zero blocking findings across all 47 specs. The `criterion-path-existence` sweep was measured before and after and diffed on `(spec, path)` keys: 25 findings → 21, suppressing exactly the four intended cases and nothing else.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None remaining.* The finding below is **resolved**.

### SHOULD: QUAL-GROUND-001 — the non-assertion marker list is restated in four places with no mechanical binding — **RESOLVED**

- **File**: `runtime/src/primitives/check_artifacts.rs:997-1030`
- **Rule**: Code whose correctness depends on an external contract it does not own — a database schema, another service's API shape, a config key, a file or wire format — SHOULD bind to that contract's canonical source rather than restating it.
- **Finding**: `NON_ASSERTION_MARKERS` is the implementation of a list the constitution's canonical-sources map assigns to `specs/045-decision-state-drift-detection/data-model.md` ("Canonical source for the open-state tell list … the acceptance-criterion path grammar"). The same list is additionally restated in `022`'s `criterion-path-existence-family` scenario and in `framework/commands/analyze.md`, each carrying its own spelled-out count. Nothing binds them: this change added three phrases and had to hand-edit three documents plus the code, and a missed one would have left a canonical source lying about the shipped behavior — precisely the drift this family exists to detect. The duplication predates this change; the change widened it.
- **Auto-fixable**: no
- **Suggested fix**: Bind rather than restate. The nearest precedent in this repo is `/gov:audit` Family 17 (`framework-self-audit`), which derives the agent config dirs from the Agent Registry table instead of mirroring them as shell literals, and emits a finding when the derivation yields nothing rather than falling back. An equivalent audit family could parse the marker table out of 045's data-model and assert set equality against `NON_ASSERTION_MARKERS`, failing closed on an empty derivation. Deferred here because it is a new audit family rather than a change to this scenario's surface.
- **Status**: **resolved 2026-08-03** — the suggested fix, built as `/gov:audit` Family 18 (`scripts/audit/marker-list-parity.sh`) under 026's [family-18-marker-list-parity](../026-framework-self-audit/scenarios/family-18-marker-list-parity.md) scenario (026 task 23), routed to 026 as the audit's home spec rather than spawned as its own. It derives the marker set from 045's canonical table and compares it against `NON_ASSERTION_MARKERS` and `analyze.md`'s shipped restatement in both directions, plus the declared array length and the spelled-out count in all three markdown homes. The failure direction is right: an empty derivation, a duplicate row, or a missing file is a finding rather than a silent pass, so the family cannot rot into exiting 0 while checking nothing. `analyze.md` stays a restatement by necessity — it ships to adopters, who have no copy of 045's data-model — so binding it was the only available fix. Verified against five injected drift modes (marker dropped from the array, marker added to the table only, canonical heading renamed, stale count word, restored baseline); shellcheck clean; the full 18-family suite exits 0.

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-EFFICIENCY — the collapsed single parse allocates eagerly for numbered headings above the task level

- **File**: `runtime/src/primitives/prune_tasks.rs:231-234`
- **Rule**: Flag repeated work and allocation on paths where a cheaper form is available.
- **Finding**: `segment` now maps `split_numbered_heading` to owned `String`s before testing the level, so a numbered heading at a level other than `task_level` (a flat-task remnant in a phased file) allocates two `String`s that are immediately discarded; the previous form tested the cheap `heading_is_numeric` predicate first and allocated only for real tasks. The owned map is deliberate — the borrowed return would hold a borrow of `heading` across the `if let`, whose else-arm moves that heading into the phase-name slot — so the alternative reintroduces either the double parse the scenario removed or an `unwrap`. Recorded low-confidence because the input is a single bounded `tasks.md` and the affected headings are rare.
- **Auto-fixable**: no
- **Suggested fix**: Leave as-is unless a large mixed-structure `tasks.md` shows up in profiling. If it does, hoist `let is_numbered = heading_is_numeric(&heading);` for the `is_phase` test and allocate inside the `level == task_level` branch.

### LOW-CONFIDENCE: QUAL-PROCESS — a done spec's data-model was edited without the back-edge

- **File**: `specs/045-decision-state-drift-detection/data-model.md:110-120`
- **Rule**: AGENTS.md §Workflow — runtime work routes to spec 022 via the back-edge; the requiring spec keeps its requirement, constitution amendments, command-source documentation, and acceptance criteria.
- **Finding**: 045 is `done`, and this change edited its `data-model.md` marker table (fourteen phrases, new migration-subject group) without reopening it. The edit is not optional — that table is the designated canonical source, so leaving it at thirteen would have made a done spec's canonical record contradict shipped behavior. But AGENTS.md's routing rule contemplates 022's `data-model.md` absorbing runtime changes, not the requiring spec's, so the correct handling of a canonical table that lives on a done spec is genuinely unspecified. Recorded low-confidence on the process question, not on the content.
- **Auto-fixable**: no
- **Suggested fix**: Either treat a canonical-source table sync as a mechanical edit (like a rename sweep, which explicitly keeps a spec at `done`) and say so in AGENTS.md §Workflow, or move the marker table to 022's `data-model.md` where runtime contracts already live and leave 045 pointing at it. The second is more consistent with the routing rule and would also collapse one of the four restatements the QUAL-GROUND-001 finding names.

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
