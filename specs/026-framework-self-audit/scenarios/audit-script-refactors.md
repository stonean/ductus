---
section: "Follow-on scenarios"
---

# Audit-script-refactors

## Context

Two refactor findings against `scripts/audit/*.sh` surfaced during 026's review, both bundled here because they touch the same surface:

- **REUSE-001 — shared boilerplate extraction.** Each of the nine family check scripts ([`check-zero`](../../../scripts/audit/check-zero.sh), [`cross-doc-consistency`](../../../scripts/audit/cross-doc-consistency.sh), [`manifest-parity`](../../../scripts/audit/manifest-parity.sh), `registry-equivalence` (since retired with the workflows feature), [`placeholder-roundtrip`](../../../scripts/audit/placeholder-roundtrip.sh), [`template-alignment`](../../../scripts/audit/template-alignment.sh), [`ssot-invariants`](../../../scripts/audit/ssot-invariants.sh), [`sibling-coupling`](../../../scripts/audit/sibling-coupling.sh), [`introducing-drift`](../../../scripts/audit/introducing-drift.sh), [`primitive-promotion-candidates`](../../../scripts/audit/primitive-promotion-candidates.sh)) repeats ~10 lines of identical setup: `set -uo pipefail`, `ROOT="$(git rev-parse --show-toplevel)"`, `cd "$ROOT"`, `drift=0`, the `emit()` function with pipe-separated output. A shared `scripts/audit/lib.sh` exposing `audit_emit FAMILY LOCATION MESSAGE FIX` plus the ROOT/cd setup would eliminate ~90 lines of duplication.
- **QUALITY-001 — `flush_step` caller-scope mutation.** [`scripts/audit/primitive-promotion-candidates.sh`](../../../scripts/audit/primitive-promotion-candidates.sh)'s `flush_step()` function reads and mutates seven caller-scope variables (`step_start_line`, `step_buffer`, `step_has_primitive`, `step_has_llm_marker`, `step_has_ignore`, plus `emit`'s `drift`). Bash semantics make this work — functions inherit caller scope by default — but the pattern is fragile: adding a `local` keyword anywhere in the function would silently break it. Confidence 70%: correct as tested but brittle against refactor.

Origin: spec 026 review SHOULD / low-confidence findings, 2026-05-18. Captured via the inbox.

## Behavior

**Both shipped, and the signatures differ from the ones sketched here — what shipped is what this section now records.**

REUSE-001 shipped as a shared `scripts/audit/lib.sh` that every family check script sources (`00286b10`, 2026-07-30):

```bash
set -uo pipefail
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1   # provides ROOT, cd, emit, drift
audit_family cross-doc
```

The family label is set once by `audit_family NAME` rather than repeated as a first argument, so the emitter is `emit LOCATION MESSAGE FIX` — three arguments, not the four `audit_emit FAMILY LOCATION MESSAGE FIX` sketched above. It writes the pipe-separated row and sets `drift=1`; family scripts shrink to their per-family logic plus a final `exit "$drift"`. Two details the sketch did not have and that carry weight: the path is derived from `${BASH_SOURCE[0]}` rather than `$0`, so sourcing still resolves when the script is invoked through `bash <path>`, and the `|| exit 1` makes a missing `lib.sh` fail closed instead of running on with `emit` undefined — which would let a family that exits 0 by design report clean. The orchestrator [`run-all.sh`](../../../scripts/audit/run-all.sh) is unaffected, as expected: it shells out to each family script, so the lib stays internal to them.

QUALITY-001 shipped in `cc897efc` (2026-08-01). `flush_step` is gone; `primitive-promotion-candidates.sh` carries `report_step FILE START_LINE HAS_IGNORE HAS_PRIMITIVE HAS_LLM BUFFER`, a pure function of its arguments that reads no caller-scope state and clears none, leaving the walker owning step state end-to-end. That is the explicit-arguments half of the sketch; the return-the-count-via-stdout half was not needed, because `emit` already owns `drift` and nothing has to be handed back.

Both were pure refactors with no observable behavior change, verified by `bash scripts/audit/run-all.sh` exit-code parity and a stdout-row diff against the pre-refactor state.

## Edge Cases

- **A family script wants different `emit()` semantics** (e.g., severity column). Add the column to the shared `emit` signature with a sensible default; family scripts opt in.
- **Standalone invocation ergonomics under REUSE-001.** `bash scripts/audit/X.sh` continues to work because the lib is sourced via `$(dirname "${BASH_SOURCE[0]}")/lib.sh`, which resolves regardless of CWD. The documented trade-off in [`scripts/audit/README.md`](../../../scripts/audit/README.md) becomes a non-issue.
- **QUALITY-001 introduces a subshell** that breaks `drift` accumulation. Use a temp file or explicit return-by-stdout to preserve the count; verify with a fixture that has both annotated and non-annotated flagged steps.

## Open Questions

*None.*

## Resolved Questions

- **Bundle REUSE-001 and QUALITY-001 in one scenario or split?** **Bundled.** Both touch `scripts/audit/` and are pure refactors with no behavior change. A combined PR keeps the diff coherent (single review of the audit-script surface) and lets the lib.sh extraction land in the same commit window as the `flush_step` rewrite — which can use `audit_emit` directly. Confirmed 2026-05-18 during the inbox-emptying groom pass.
