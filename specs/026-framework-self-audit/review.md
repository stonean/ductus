---
spec: 026-framework-self-audit
reviewed-at: 2026-08-31T01:15:00Z
reviewed-against: 8cee61ad0c6f5b3f8a6b8f6e5c2a1f7d3b9e4c60
diff-base: 2b885326a1125ebbd5515420039bb5d74152014c
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Review of Family 34 — step-reference integrity, the family added to this spec by spec 054's captured observation. Zero MUST violations, zero SHOULD violations, zero low-confidence findings, not blocking.

**Built as a primitive, which the suite's own contract required.** `scripts/audit/README.md` states that a deterministic, mechanical check belongs in the runtime and that reaching for an embedded `python3` heredoc to parse markdown structure is the signal a check took the fallback without earning it. This one parses markdown structure, so it is `check-step-references` with `step-reference-integrity.sh` as the thin entry point — the Family 30/31 shape. The scenario's task list originally said "deriving ... in the script", which contradicted that contract and was corrected rather than followed.

**The important property is what it declines to claim.** Of the four stale references that motivated the family, it catches one. Renumbering shifts a reference onto a *different existing* step — after 054's removals `specify.md` still had a step 6 and `consolidate.md` still had a step 4 — so three of the four resolve, to the wrong step. Only the self-reference is reachable without knowing what each step does. Closing that gap would mean matching prose against step content, a heuristic that fires falsely on correct references, and 045 already set the standard there in rejecting the criterion-supersession check at 455 pairs: a family that fires falsely is worse than the silence it replaces. The one-of-four bound is stated in the scenario, the primitive's header, the script's header, `audit.md`, and this suite's README — five places, because the failure mode being guarded against is a reader concluding the family covers the incident that produced it.

**Subject bounds are measured, not asserted.** 19 files examined, 15 carrying one procedure. `amend.md` restarts numbering under each `###` subsection and `status.md` uses three separate one-item lists; both are legitimate authoring, and MD029 is disabled for these files so nothing pushes them to be otherwise. Merging those into one step set would have manufactured findings — the first implementation did exactly that and reported `amend.md` as discontinuous across 26 numbers, `status.md` across three, and `help.md` as an extraction failure. All three were false positives, caught before the family shipped by running it against the live corpus rather than a fixture. They are now named in `not-a-procedure` rather than examined. 96 `step N` mentions outside the Instructions section are counted, never resolved, so a clean exit is never read as *every step reference in the corpus resolves*; both counts go to stderr.

**Proven to fail before being kept**, which the task required. Reintroducing the exact self-reference 054 removed turns the family red with exit 1 and the correct message; restoring the file returns exit 0. An assertion nobody has watched fail is one nobody knows works — the same vacuity guard `AGENTS.md` requires of a new family, and the one 052's review relied on.

`QUAL-CLAIM-001` is satisfied structurally rather than by wording: `examined`, `with-steps`, `not-a-procedure`, `references-out-of-subject`, and `skipped` are distinct fields, so a caller can always tell *examined and clean* from *could not examine*. An unreachable runtime and an empty examined set are both findings.

Registered in all three registries Family 28 holds together — `run-all.sh`, `audit.md`, and this suite's README — and Family 28 confirms the sets agree at 33 (families 1–2 and 4–34; Family 3 is retired). Runtime tools 69 → 70, configure files regenerated in the order `AGENTS.md` records.

**Bounds on this review.** The scope is 39 paths from this spec's in-progress parent, spanning Family 34 and the worked-example substitution in `link-check-consolidation.md`. Both were examined. The 33 pre-existing families were not re-reviewed; they are unchanged by this work and carry their own history.

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

## Observations

*None.*

## Skipped passes

*None.*
