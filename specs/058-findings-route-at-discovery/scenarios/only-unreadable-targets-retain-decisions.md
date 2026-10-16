---
section: "Decisions persist across runs"
---

# Only-unreadable-targets-retain-decisions

## Context

`/{project}:analyze` step 18 sets `restricted` whenever the run left any target unexamined. A restricted run retains every stored decision whose finding did not fire, because that finding's source might not have been looked at.

Most unexamined targets are excluded by construction rather than missed. `not-a-live-claim`, `ships-to-adopter`, and `root-absent` are decisions the checks made about what to examine, and they recur on every run. Measured 2026-09-25 by grepping `^unexamined:` over `specs/*/analysis.md`: 20 of 55 records carry a nonzero count, mostly with those reasons. On those specs a stored decision whose finding stopped firing is never pruned, which contradicts §Decisions persist across runs and AC32 ("a re-run in which the finding no longer fires prunes the stored decision").

`066-audit-records-and-the-completion-gate`'s data model already splits the reasons into two classes: *excluded by construction* (`not-a-live-claim`, `ships-to-adopter`, `root-absent`) and *could not be read* (`target-missing`, `target-unparseable`, `no-readable-state`), plus `artifact-unreadable` for a spec's own artifact.

Found by the 058 review (2026-09-25).

## Behavior

`restricted` is set only when a skipped target's reason is in the *could not be read* class: `target-missing`, `target-unparseable`, `no-readable-state`, `artifact-unreadable`, and each reason `analyze.md`'s Unexamined targets section classifies with them. Targets excluded by construction never restrict, because the run decided not to examine them. It did not fail to.

`/{project}:analyze` step 18 states the rule, and `docs/analyze.md` follows it. The class list lives in one place and is named where the step uses it, so a reason added later is classified once.

## Edge Cases

- **A run whose only skips are by construction** prunes normally.
- **A run with one unreadable target** retains every non-firing decision, as now. Being precise about which decisions that target could have produced is not attempted, because that would tie decision keys to family names.
- **An unresolved shared constitution** (`constitution-unresolved`) is an unexamined input, not a target. The run examined fewer rules than declared, so it restricts.
- **Under `ductus exec`** nothing is matched or pruned, as now.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
