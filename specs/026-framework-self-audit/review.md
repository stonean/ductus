---
spec: 026-framework-self-audit
reviewed-at: 2026-05-18T01:30:00Z
reviewed-against: b160cfb
diff-base: e6ba1af55f022750bdafce220cef1e395b08e416
must-violations: 0
should-violations: 1
low-confidence: 1
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Reviewed `/audit` and its nine family check scripts (Phase B/C/D + the Family 9 pulled in mid-implement). Stack: text-first markdown + bash with one Rust touch to `gen-claude-commands.sh` (a new `--check` flag added during Phase A to close check-zero's gap). Loaded rule files: `configuration-cross.md` only — none of its CFG-* triggers fire against the diff (no env-var lookups, operator-tunable constants, or shared cross-module values introduced). All five passes ran. 0 MUST, 1 SHOULD (acknowledged boilerplate across the nine family scripts — **since fixed**, see §Resolved after the review run), 1 low-confidence (bash function variable scoping in primitive-promotion-candidates). `blocking: no`.

**Scope.** `framework/commands/audit.md` + generated mirror; `scripts/audit/` (10 scripts — check-zero, run-all, 8 family scripts + Family 9 — plus README); `scripts/gen-claude-commands.sh` (--check mode added); `.github/workflows/{markdown-only-pipeline,runtime-release}.yml` (v1 soft-launch advisory steps); spec + tasks edits.

## MUST violations (blocking)

_None._

## SHOULD violations (advisory)

_None open._ The one finding this run produced (REUSE-001) was fixed later — see §Resolved after the review run. The frontmatter counts describe what the run against `b160cfb` found and stay in lockstep with the `review:` block in `spec.md`; a future `/gov:review` re-run against `HEAD` is what moves them.

## Resolved after the review run

### SHOULD: REUSE-001 — boilerplate duplication across family scripts (fixed 2026-07-30)

- **File**: `scripts/audit/*.sh` (cross-doc-consistency, manifest-parity, registry-equivalence, placeholder-roundtrip, template-alignment, ssot-invariants, sibling-coupling, introducing-drift, primitive-promotion-candidates)
- **Rule**: AGENTS.md §design-principles ("Never design framework features that depend on human diligence") — by extension, "Don't Repeat Yourself" applied to the framework's own auditor.
- **Finding**: Each family script repeats the same boilerplate: `set -uo pipefail`, `ROOT=...`, `cd "$ROOT"`, `drift=0`, an `emit()` function with the same pipe-separated output shape. ~10 lines × 9 scripts = ~90 lines of structurally-identical code. A shared `scripts/audit/lib.sh` would let each family script `source` the boilerplate.
- **Auto-fixable**: yes (mechanical extraction)
- **Trade-off accepted at the time**: shared lib couples the nine scripts and reduces their stand-alone-invocation ergonomics (`bash scripts/audit/X.sh` would require lib.sh on relative path). Per-script standalone invocation is documented in `scripts/audit/README.md` as the per-family contract — useful for triaging a single check failure in CI. Extracting the lib was an option, not a blocker.
- **Resolution**: extracted to `scripts/audit/lib.sh`, which owns `ROOT`/`cd`, the `drift` accumulator, and `emit LOCATION MESSAGE SUGGESTED-FIX`; `audit_family NAME` sets the leading column so the call sites keep their three-argument shape. Every script in the directory sources it — the fifteen families plus `check-zero.sh` (whose hand-rolled finding line is now an `emit` call) and `run-all.sh` (which uses `drift` but renders per-family headers rather than finding lines). The boilerplate that had grown to ~10 lines × 17 scripts is now one four-line header apiece. `installer-registry-parity.sh` held a second copy of the finding shape inside its python heredoc; that python now prints tab-separated records the shell renders through `emit`, matching `migration-coverage.sh`, so the pipe-separated line exists in exactly one place.
- **Fails closed**: the source line carries `|| exit 1`. Without it a missing `lib.sh` left `ssot-invariants.sh` — the always-`exit 0` stub — reporting clean while its `stderr` was swallowed by the aggregator (which only prints captured output for non-zero families). The other scripts failed closed only incidentally, via `set -u` on the unbound `drift` at their final `exit`.
- **Standalone-invocation ergonomics preserved**: the lib path is derived from `${BASH_SOURCE[0]}`, not the caller's `cwd`, so `bash scripts/audit/X.sh` works from the repo root, from an absolute path in any directory, and as `./X.sh` from inside `scripts/audit/`. The per-family contract in `scripts/audit/README.md` is unchanged and now documents the shared header.
- **Verified**: all seventeen scripts were run pre- and post-refactor against paired pristine `git archive HEAD` copies — clean, then with identical injected drift (broken placeholders, deleted templates/registry/`govern.md`/constitution, removed MCP permission line, stale `` `/capture` `` reference, hardcoded runtime path, prose-only command step, perturbed `install.sh` settings seed) — and produced identical stdout, stderr, and exit codes in every case. Across the drift rounds every script except `ssot-invariants.sh` (the stub, which never emits) and `sibling-coupling.sh` (needs a specific two-spec configuration) rendered real findings through the shared `emit`. `run-all.sh` against the repo is byte-identical to the pre-refactor baseline (clean, exit 0). Also checked: every script under macOS system bash 3.2 with no stderr; `shellcheck -x` clean from any working directory (`source-path=SCRIPTDIR`); and with `lib.sh` removed, every script — `ssot-invariants.sh` included — exits 1.

## Low-confidence findings

### LOW-CONFIDENCE: QUALITY-001 — bash function scope reliance in primitive-promotion-candidates

- **File**: `scripts/audit/primitive-promotion-candidates.sh:46-69` (the `flush_step` function)
- **Rule**: implicit best practice — functions should not silently rely on caller-scope mutable state.
- **Finding**: `flush_step()` reads and mutates seven caller-scope variables (`step_start_line`, `step_buffer`, `step_has_primitive`, `step_has_llm_marker`, `step_has_ignore`, plus emit's `drift`). Bash semantics make this work (functions inherit caller scope by default), but the pattern is fragile to refactor — adding a `local` keyword anywhere in the function would silently break it. **Confidence: 70%** (works as tested but brittle).
- **Auto-fixable**: no — refactoring to pass state explicitly is non-mechanical
- **Suggested fix**: Convert the per-file loop into a function that returns the step list, then iterate the returned list to apply flush logic. Or document the scope contract explicitly in the function's leading comment so future maintainers don't introduce `local` keywords. v2 can return a structured array via a temp file.

## Waived findings

_None._

## Skipped passes

_None._

## Pass notes

### Security

No security rules apply at the framework level for the diff in scope. The bash scripts are read-only file comparisons; no HTTP, authentication, persistence, or shell-out beyond running other framework scripts with `--dry-run`. The new `--check` mode added to `gen-claude-commands.sh` creates a tempfile via `mktemp` with the `trap 'rm -rf "$tmpdir"' EXIT` cleanup pattern — correct.

### Reuse

One SHOULD finding (REUSE-001 above) on boilerplate duplication across the nine family scripts. Deferred at the time as intentional per the per-family standalone-invocation contract; since fixed by `scripts/audit/lib.sh`, which keeps that contract intact. Other extractor helpers (`extract_claude_mcp`, `extract_auggie_mcp`, `extract_paths`, etc.) appear similar at a glance but have meaningful per-family differences in regex shape and field semantics — not deduplicable without an awk-level abstraction that would obscure the per-family intent.

### Quality

One low-confidence finding (QUALITY-001 above) on bash function variable scoping in `primitive-promotion-candidates.sh`'s `flush_step`. The pattern works as tested but is fragile to refactor. Bash 3.2 compatibility was the major concern across all scripts (macOS default shell) — verified by smoke-testing each script locally; no associative arrays, no `${var,,}` lowercasing, parallel scalars used instead.

`gen-claude-commands.sh`'s `--check` mode: walked the diff line-by-line. Tempfile handling is correct (trap cleanup); the orphan-detection loop catches files in DEST that no longer have a source. Unit-test equivalent (a manual drift-injection smoke test in Phase A task 2) confirmed correct fail/exit semantics.

### Efficiency

N/A. Each script iterates `framework/commands/*.md` × primitives or specs × files — both small bounded sets (15 command files × ~25 primitive names; ~27 specs). Runtime is sub-second per family.

### Simplicity

Family 6 (`ssot-invariants.sh`) is a stub that exits 0 always. Strict reading is overengineering ("a check that does nothing"); the counterpoint accepted at design-time is that the script _is_ the planning artifact — header documents the curated list and the promotion path. Per the spec body Family 6 description, real pattern-based detection requires concrete duplicate cases to write the patterns against, which v1 doesn't yet have. Not a finding under the v1 design intent.

The mid-implement pull of Family 9 added an AC and a check family without going through clarify (the standard back-edge for adding scope to an in-progress spec). User-directed expansion is captured in the Family 9 commit message; not a quality issue, but worth noting as a pattern: future scope expansions on in-progress specs should ideally go through clarify or be captured as scenarios. For v1 of `/audit`, the bundling-candidate check (Family 7) would have flagged exactly this kind of mid-implement mutation if the new AC were in a _different_ in-progress spec.
