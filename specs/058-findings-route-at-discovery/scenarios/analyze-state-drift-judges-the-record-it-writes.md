---
section: "`/{project}:analyze`"
---

# Analyze-state-drift-judges-the-record-it-writes

## Context

`check-artifacts`' analyze-state-drift family (spec 047) reports a `done` spec whose `analysis.md` records `last-run` unset or `blocking: true`. `/{project}:analyze` calls `check-artifacts` during detection, so the finding describes the record the run is about to replace, not the one it will write.

That finding cannot converge, for the reason [analysis-drift-judges-the-record-it-writes](analysis-drift-judges-the-record-it-writes.md) gives for disposition drift. It is blocking, so it cannot be discarded. No fix removes it before step 20, and step 19's re-check reads the same prior record, so step 20 writes `blocking-findings` of at least one and `blocking: true` again, even when the run fixed the finding that made the prior record blocking. The next run reports the drift again. Only `--fix` ends the loop, through the disposition-drift trigger. A route of the drift finding stores a decision that never expires, and then nothing reverts the spec.

The family predates 058. Found by 058's second review (2026-09-25), which applied the disposition-drift argument to its sibling.

## Behavior

Inside `/{project}:analyze`, analyze-state drift is judged against the record that will stand after the run, as disposition drift on `analysis.md` is.

- **During detection, `check-artifacts` does not judge `analysis.md`.** Neither its `last-run` nor its `blocking` produces a finding, so the prior record never enters the tier counts step 20 writes.
- **After step 20 writes the record**, a `done` spec whose new record is `blocking: true` is reported as analyze-state drift from that record, at render time. The hard-fail and blocking findings that made it so are already in the record's tiers. The drift report names the state and is not counted again as a finding, because a record cannot count its own verdict.
- **`--fix` does not revert on this check.** A done spec whose new record is blocking still holds the blocking findings; fixing them clears it, and routing one onto this spec reopens it. One whose new record carries undispositioned findings reverted through disposition drift until scenario `findings-cost-in-proportion-to-risk` made that report advisory.

A run that fixes every hard-fail and blocking finding on a `done` spec therefore leaves no drift behind. A run that leaves one live still reports it, and keeps reporting it until the finding stops firing.

## Edge Cases

- **A `done` spec with no `analysis.md`** stays grandfathered, as now; `/{project}:audit` Family 37 counts that set.
- **`last-run` unset** cannot hold for the record the run writes, so `/{project}:analyze` never reports it. The CI hook still reads the committed record directly, so a hand-edited record is caught there.
- **A blocking finding routed to the `done` spec in hand** reopens that spec `done → in-progress`, so no drift is reported on it.
- **Under `ductus exec`**, the record carries the walker's tier counts, so a `done` spec with a live blocking finding gets a record that is `blocking: true`. The exec run reports nothing: drift is judged at step 21, a host responsibility the walker skips. An interactive run reports the drift, and one that fixes the finding clears it, so it does not loop.
- **The markdown-only path** judges the drift the same way, from the `analysis.md` it just wrote.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
