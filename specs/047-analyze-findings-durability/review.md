---
spec: 047-analyze-findings-durability
reviewed-at: 2026-08-03T13:18:53Z
reviewed-against: 5d2bb407a8988f420dd5b7b23de041a4147e679c
diff-base: 5d2bb407a8988f420dd5b7b23de041a4147e679c
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 047-analyze-findings-durability

## Summary

Prose-only change across two files plus a regenerated mirror: `framework/commands/analyze.md` gains a capture step before its render step, and `framework/constitution.md` §Automatic issue capture widens to cover a command whose primary output is findings. **0 MUST, 0 SHOULD — not blocking.** Rule files loaded: the backend + cross set (api, concurrency, configuration, observability, performance, quality, reliability, security). None of them reach this diff — there is no application code, no auth or credential surface, no network or persistence, no constants or env vars. The security, efficiency, and reuse passes therefore find nothing to evaluate; the quality and simplicity passes carry the weight. No runtime change was needed, which is the design decision doing the most work here: `append-inbox` already existed with its `dedup-prefix` guard (022's `append-inbox-comment-aware-write`), is already in `PRIMITIVE_REGISTRY` (so `PRIMITIVE_NAMES` accepts the backticked name), and already has an interpreter dispatch arm — verified at `runtime/src/schema/registry.rs:54` and `runtime/src/interpreter/mod.rs:701`. The capability was built and simply never wired to the one command whose entire output is findings. Step ordering was checked against the AGENTS.md §Gotchas trap: the capture step sits *above* the `audit:ignore-promotion` render step, so no primitive is backticked inside a step the exec walker treats as host-only, and `lint-procedure-parseability.sh` exits 0 with the new step parsing as a single-primitive dispatch. Dogfooded rather than asserted: the dedup key was measured against this repo's 21 live findings, and the first key drafted (category + family + citing path) was found to collapse them to 8 — silently dropping 13, including two of spec 012's three missing paths — because `ArtifactFinding.path` is the citing artifact, not the missing subject. The key was corrected to include the message before anything shipped, re-measured at 21 keys for 21 findings, and the correction is recorded in the spec, the plan, and both copies of the command. Idempotency was then exercised against the real primitive: pass 1 appended 21, pass 2 appended 0 and reported `deduped` for every item. Verification: markdownlint 0 issues across 386 files, `lint-procedure-parseability.sh` exit 0, the 18-family framework self-audit exit 0 (check-zero confirms the generated mirror matches its source), and `check-artifacts` clean on 047.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
