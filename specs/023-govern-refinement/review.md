---
spec: 023-govern-refinement
reviewed-at: 2026-08-14T20:16:13Z
reviewed-against: 78eae97a3ada26e7c9c017745ddf960b23ce8285
diff-base: e71bd410a7d8d6bdad82188bdc16a2af85ee945a
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 023-govern-refinement

## Summary

Clean and non-blocking: 0 MUST, 0 SHOULD, 0 low-confidence. The two SHOULD findings from the prior run at `81cd872` were both fixed in `78eae97` and no longer fire, so they are recorded here rather than carried as open entries. `QUAL-GROUND-001` (the reconcile pass restated `scenario-consistency`'s matching rule with nothing failing on divergence) is closed by giving the rule one canonical home: `specs/022-deterministic-runtime/data-model.md` states it, §drift-prevention's canonical-source map points there, `framework/commands/amend.md` cites it instead of asserting it independently, and `bare_slug_reference_satisfies_the_mapping` locks the hand-written authoring form the previous test set never exercised — the gap through which the narrower path-match rule originally shipped. `QUAL-CLAIM-001` (a silent pass reading as "no scenario needs a task" when it examined none) is closed by having step 2 name what went unexamined and the pass report what it looked at when it offers nothing, with the line omitted when the feature has no scenarios and therefore no subject to overstate. Scope, stated because the counts would otherwise overstate what was examined: the diff base `e71bd41` is where 023 entered `in-progress` months ago, so the mechanical scope spans most of the repository. This run examined in full the files this feature's work touched — `framework/commands/amend.md` and its generated copy, `scenarios/extend-existing-scenario-task.md`, `spec.md`, `framework/constitution.md`, `specs/022-deterministic-runtime/data-model.md`, and `runtime/src/primitives/check_artifacts.rs` — plus the rest of the 55 files changed since the previously-recorded verdict at `1eda6f6`. The other `runtime/src/**` changes in that window belong to spec 022's release work and carry their own verdict in `specs/022-deterministic-runtime/review.md`; they were not re-reviewed here. Everything outside the window was covered by the `1eda6f6` verdict and is unmodified since. The full runtime suite (868 unit tests plus every integration suite) passed on the pre-commit hook for `78eae97`; `amend.md` remains on the procedure-parseability legacy-prose allowlist, so its own text is verified by lint and the self-audit rather than by a walker test.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- Architectural exploration: re-frame the runtime's LLM extension points as named Anthropic-style Skills the host loads at the seam — speculative, **on hold per user 2026-07-11**. Still open in `specs/inbox.md`.
- No command adds an acceptance criterion to a non-`draft` spec — `/gov:clarify` gates on `draft`, `/gov:amend` writes only a question or a scenario+task, `/gov:plan` gates on `clarified`. Captured 2026-08-14 while adding criterion 33 to this spec during its completion gate. Still open in `specs/inbox.md`; run `/gov:groom` to route it.

## Skipped passes

*None.*
