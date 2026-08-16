---
spec: 041-task-pruning
reviewed-at: 2026-07-11T12:12:24Z
reviewed-against: c0bc8697bdef33bbb2024585fa75bd4299889fca
diff-base: 9ab3163db47064584fd29ef4a7eb041865be3767
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 041-task-pruning

## Summary

Clean review. The implementation — the `prune-tasks` runtime primitive plus the
`/ductus:prune` command, the shared `SkipScanner` parser fix, and the framework
consistency edits — carries **no MUST violations** and is not blocking. Rule
files applied: the backend + cross set (`security-backend`, `api-backend`,
`concurrency-backend`, `observability-backend`, `performance-backend`,
`reliability-backend`, `configuration-cross`, `quality-cross`); the frontend
rule files were not selected (no frontend surface in scope). One advisory
SHOULD (a reuse duplication) and two low-confidence notes were recorded below;
none blocked `done`. **All three are now dispositioned** — the SHOULD is fixed
under 022's `numbered-heading-grammar-single-source` scenario, and both
low-confidence notes are waived with rationale (2026-08-02). The code is covered by 489 passing library tests plus the
integration suites (parity, MCP), with `clippy -D warnings` and `fmt` clean, and
`scripts/audit/run-all.sh` reporting zero findings.

Security posture: the backend security rules ductus authentication, credentials,
sessions, tokens, and JWTs — none of which this feature introduces. `prune-tasks`
performs a local, confirmed rewrite of a single `tasks.md` within the resolved
feature directory; it opens no network, handles no secrets, and persists no
credentials. No security finding.

## MUST violations (blocking)

_None._

## SHOULD violations (advisory)

### SHOULD: QUAL-REUSE — numeric-heading helpers triplicated across tasks parsers — **RESOLVED**

- **File**: `runtime/src/primitives/prune_tasks.rs:~370` (`heading_is_numeric`, `split_numbered_heading`)
- **Rule**: quality-cross — prefer extracting logic duplicated across modules into shared code rather than re-implementing it.
- **Finding**: `prune_tasks.rs` defines local `heading_is_numeric` and `split_numbered_heading` helpers that duplicate `read_tasks.rs`'s module-local `split_numbered_heading`/`heading_starts_with_number` and `mod.rs`'s `heading_starts_with_number`. The numeric-heading check now exists in three modules. Note this follows an existing, deliberate convention — `read_tasks.rs` documents keeping its copy "module-local to avoid widening the crate-internal surface" — so this is consistent with the codebase, not a regression.
- **Auto-fixable**: no
- **Suggested fix**: optionally promote a single `pub(crate) fn split_numbered_heading` / `heading_is_numeric` to `primitives::mod` and have `read_tasks`, `prune_tasks` (and the `mod.rs` copy) call it. Advisory — defer if the module-local convention is preferred.
- **Status**: **resolved 2026-08-02** under 022's [numbered-heading-grammar-single-source](../022-deterministic-runtime/scenarios/numbered-heading-grammar-single-source.md) scenario (022 task 78). `primitives/mod.rs` now owns a borrowed `split_numbered_heading` with `heading_is_numeric` defined as `.is_some()` on it, so the predicate cannot drift from the splitter; all three private copies are gone and `prune_tasks`'s task branch parses once instead of testing then re-parsing. The module-local convention was weighed and set aside on the specific ground the original note raised but did not resolve: the three modules read _the same tasks file_, so a divergence would let one primitive see a task another does not — a cost the convention's "don't widen the crate-internal surface" rationale does not price in.

## Low-confidence findings

_None remaining._ Both notes below were dispositioned as waivers on 2026-08-02;
they are recorded in full under §Waived findings.

## Waived findings

### WAIVED: quality — keep-pending rewrites a file whose only reducible content is an empty phase container

- **File**: `runtime/src/primitives/prune_tasks.rs` (`reduce_keep_pending`, `dropped_any`)
- **Finding** (original confidence ~55): in phased mode, a `## Phase …` container with zero (or only spent) task sections is dropped, which sets `dropped_any` and therefore writes even when `removed_count == 0`. A user running `/ductus:prune` on a file whose only "prunable" element is an empty phase heading gets a write rather than a "nothing to prune" report.
- **Waiver rationale**: this is the documented data-model behavior, not a deviation from it — "drop a phase container with no surviving task section" is the contract, and an empty phase container _is_ reducible content, so the write is honest and `removed_count == 0` correctly reports that no task section was removed. Suppressing the write would make the primitive leave a file it had decided to change. The triggering state (a hand-edited empty phase heading) is unusual and self-correcting on the next prune. No code change.

### WAIVED: security (defense-in-depth) — `feature` arg is not run through `validate_no_traversal`

- **File**: `runtime/src/primitives/prune_tasks.rs:110-118`
- **Finding** (original confidence ~40): `run` builds `repo.join(&root).join(&args.feature)` and gates on `is_dir()` without calling `validate_no_traversal(&args.feature)`.
- **Waiver rationale**: this is the established convention, not an omission in 041. Every sibling feature-name primitive (`read-tasks`, `mark-task`, `set-status`, `check-stuck`, `derive-boundary`) does the same: `feature` is a host-resolved directory slug, and the traversal guard is reserved for caller-supplied _path_ arguments (`feature-path`, `slug`) in `append-task` / `create-scenario`. Fixing `prune-tasks` alone would make the codebase inconsistent while closing nothing, and the containment that matters is already enforced — the `is_dir()` gate under the resolved specs root. Applying the guard across all six was considered and declined by the user on 2026-08-02; if it is ever revisited it is a codebase-wide change owned by 022, not a 041 defect.

## Captured issues (pending /ductus:groom)

_None — no additions to `specs/inbox.md` in the review window._

## Skipped passes

_None — all five passes (security, reuse, quality, efficiency, simplicity) ran._
