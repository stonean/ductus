---
spec: 000-slash-commands
reviewed-at: 2026-08-17T12:17:15Z
reviewed-against: ccdd3ac
diff-base: 1eda6f6f
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 000-slash-commands

## Summary

All five passes ran over this spec's last two open scenarios and the code that closed them. Clean: 0 MUST, 0 SHOULD, 0 low-confidence, nothing waived, no pass skipped.

Re-recorded against `ccdd3ac` rather than `11ff132d`. The prior verdict named the HEAD that existed *while* the review ran, and the scenarios it reviewed were then committed on top of it — so `/ductus:audit` Family 19 correctly reported the review as predating its own subject, and the `ductus-v0.29.7` release gate stopped the publish. The local audit had passed only because it ran pre-commit, where the edits were invisible to a `reviewed-against..HEAD` comparison.

Two of the three questions resolved against existing rules rather than new design. `criterion-route-after-draft`'s first turned on which command already performs a back-edge on a classified input — `/{project}:amend` documents that it does, `/{project}:clarify` documents that it does not — so the criterion route extends amend's classifier instead of widening a gate spec 014 narrowed deliberately. Its second was posed on a false premise: §spec-lifecycle states three back-edges exist, and the third covers any meaningful body edit, naming "new scope" and routing it via the same `/amend` flow used for scenarios. No lifecycle change was needed.

The third needed measurement, and measuring changed the answer twice — the mechanism was mis-located (the family skipped `done` specs wholesale rather than vouching coarsely), and the discriminator the question proposed does not discriminate, since both states postdate the `done` transition. Measured over 46 `done` specs, the file-shape alternative would have produced exactly one finding and it a false positive, the direction §tasks-phase forbids; the per-scenario history probe produces zero. Both directions were then proven against real history: a fixture reproducing the never-tasked shape fires, and the same fixture with a task added then pruned does not.

One SHOULD was raised in the efficiency pass and fixed rather than recorded: the first implementation walked history once per unmapped scenario where one walk answers every slug.

Verification: 953 tests pass (`--locked --release`), clippy clean on `--all-targets`, markdownlint clean over 420 files, and the new rule swept across all 47 `done` specs producing zero findings. The self-audit is verified **after** the commit this time, not before it.

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
