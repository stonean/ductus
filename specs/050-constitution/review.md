---
spec: 050-constitution
scenario: findings-route-by-scope
reviewed-at: 2026-08-25T18:22:15Z
reviewed-against: 670308bf522d3ef43174d429ff0d9832afe9bb81
diff-base: 670308bf522d3ef43174d429ff0d9832afe9bb81
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 050-constitution

## Summary

Five passes over the scope (`framework/constitution.md`, `AGENTS.md`, `specs/050-constitution/plan.md`, `specs/045-decision-state-drift-detection/spec.md`, `specs/inbox.md`) against the eleven selected rule files: 0 MUST, 0 SHOULD, 0 low-confidence. The scope is governance prose, and the loaded rule set verifies code patterns, so no rule ID anchors here — the counts are honest about what the rules could examine, not a statement that the change is complete.

The finding of record is an **incomplete prose-claim sweep**, and it is the reason this spec stays `in-progress`. The scope-routing rule added to §brownfield-inbox falsifies a behavioral claim at the enforcement point where it fires most often: `framework/commands/implement.md` step 5 (*Capture incidental issues*) still instructs an agent to append **any** issue outside the current task's scope to `specs/inbox.md`, which is now wrong for the entire middle tier the new bullet exists to define. The shipped `framework/templates/project/inbox.md` header and this repo's own `specs/inbox.md` header enumerate what belongs in the inbox and never mention the scope test, so an adopter reading only the header routes spec-scoped findings there. Neither file is in this review's scope — the finding anchors to the in-scope constitution change that falsified them — and the drift is invisible to the audit families, which check anchors and manifests rather than meaning. Recorded as task 11 rather than captured to the inbox, per the rule this spec just landed: the finding is inside the in-progress spec's scope, so the inbox would route it back here.

Two adjacencies were checked and cleared. Against §bug-handling: the new bullet does not turn `tasks.md` into a durable record — it names `/{project}:amend` as the route for a requirement gap and keeps a chore with no feature home in the inbox, which is the clause §bug-handling's *never standalone chores* is aimed at. Against `groom.md`: the five-route decision tree governs items that have already reached the inbox, so preventing an arrival contradicts nothing in it. `scripts/audit/run-all.sh` is clean on the committed tree, Family 1 (cross-doc claim consistency) and Family 6 (SSOT invariants) included, and `npx markdownlint-cli2` is clean over all five files.

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
