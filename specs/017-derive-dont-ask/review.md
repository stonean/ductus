---
spec: 017-derive-dont-ask
reviewed-at: 2026-08-27T22:44:49Z
reviewed-against: bb96fef3d83dec618fbadbccc7e021a73720ce5d
diff-base: 31348ff7dfd8b8b3cd108b8d0e7829c8b184dd14
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 017-derive-dont-ask

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes; not blocking.

017 was reopened for a factual correction, not new behavior: `generator-sync-claim-honesty` asserted that the drifted-but-unwritten state was specific to the dependency derivation, on the reasoning that the reference generator "writes every spec it enumerates, so for it enumerated and written are the same set". That held only because the reference derivation had narrowed its enumeration to the staged set — which is what made a whole class of drift unreportable. The scenario now carries a dated section recording the inversion, and points at 022's `derive-references-unstaged-drift-is-reported` as the authority for the primitive's behavior.

The correction was verified empirically before it was written, not argued from the code: a scratch repo with a `[services]` alias renamed and no spec staged reproduced the reported symptom on runtime 0.33.0, and the same repo now reports the drifted spec under `--staged`. The claim this scenario makes about its own subject is therefore checkable rather than asserted.

**Diff base overridden with `--since`, for the second time this round.** The default base is the commit at which the spec advanced to `in-progress`, which here is `b05e1df` — the commit that carries the correction, because the back-edge flip and the edit landed together. Reviewing "since" it would have excluded `scenarios/generator-sync-claim-honesty.md` entirely: the whole subject. This shape hit both 017 and 020 in the same round and is recorded as an observation for `/ductus:groom` to route, because it is outside this spec's scope and deserves a scenario of its own rather than a note in a summary.

Passes: the change is prose in a spec artifact, so the security, reuse, and efficiency passes have no subject in scope. Quality confirmed the corrected claim against the reproduction and against both primitives' current code; simplicity found nothing.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] convention: `check_command_flags::argument_hint` hand-rolls frontmatter-block extraction that `primitives::split_frontmatter` already provides — `runtime/src/primitives/check_command_flags.rs` (captured during review of 022-deterministic-runtime)
- [ ] perf: the adopter pre-commit hook now performs two independent full walks of the tracked spec corpus per commit — `framework/bootstrap/hooks/ductus-pre-commit` (captured during review of 022-deterministic-runtime)

## Observations

- bug: `/ductus:review`'s default diff base is the commit at which the spec advanced to `in-progress` — but when `/ductus:amend`'s back-edge flip is committed together with the work it authorises, that commit IS the work, so the review window starts after it and the subject is excluded. The review then reports 0 findings because it examined nothing, which is `QUAL-CLAIM-001` at the command level rather than in a result payload. Hit twice in one round (017 and 020, 2026-08-27); both needed a manual `--since=<base>~1` to see their own deliverables. Candidate fixes: resolve the base to the commit *before* the status transition, or have `compute-review-scope` report when the resolved scope contains none of the spec's own recently-changed artifacts. Deserves a scenario on 020 or 022 rather than an inbox line alone. — `runtime/src/primitives/compute_review_scope.rs`

## Skipped passes

*None.*
