---
section: "`/{project}:analyze`"
---

# Analysis-drift-judges-the-record-it-writes

## Context

`check-artifacts`' disposition-drift family reports a `done` spec whose `review.md` or `analysis.md` records undispositioned findings. `/{project}:analyze` calls `check-artifacts` during detection, so for `analysis.md` the finding describes the record the run is about to replace, not the one it will write.

That finding cannot converge. It is blocking, so it cannot be discarded. No mechanical fix removes it before step 20, because it reads the prior record. So each run records it undispositioned, the new record is blocking, and the next run reports drift again, now joined by analyze-state drift. Only `--fix` or a reopening route ends the loop, although the finding's message says a re-run suffices. On a `done` spec, `--all`'s option to leave the rest undispositioned creates this state at once.

Found by the 058 review (2026-09-25).

## Behavior

Inside `/{project}:analyze`, disposition drift is judged per record against the record that will stand after the run.

- **`review.md`**: judged during detection, as now. `/{project}:analyze` does not rewrite that record, so the finding asks for a `/{project}:review` re-run and says so.
- **`analysis.md`**: not raised during detection. After step 20 writes the record, a `done` spec whose new `dispositions.undispositioned` is above zero is reported as disposition drift from that record, at render time. Since scenario `findings-cost-in-proportion-to-risk` the report is advisory and `--fix` does not revert on it: the undecided findings it counts that hold `done`, the hard-fail and blocking ones, already surface as analyze-state drift, and the rest hold nothing; until then the spec was reverted `done → in-progress`, as the other drift triggers are.

A run that dispositions every live finding therefore leaves no drift behind, and a run that leaves findings undecided still reports it. `/{project}:analyze` is the only command that renders this family, so `check-artifacts` judges `review.md` alone, and `/{project}:analyze` judges `analysis.md` itself from `write-analysis`'s returned `dispositions`.

## Edge Cases

- **A `done` spec whose prior `analysis.md` predates dispositions (no map).** Not drift, as now. The run writes a map, and from then on the rule above applies.
- **Under `ductus exec`.** Nothing is dispositioned, so a `done` spec with any live finding gets a new record that counts it undispositioned. The exec run reports nothing: drift is judged at step 21, a host responsibility the walker skips. An interactive run reports the drift and clears it by dispositioning the finding, so it does not loop.
- **`--all`.** Each spec is judged against its own new record, after its own write.
- **The markdown-only path** judges the drift the same way, from the `analysis.md` it just wrote.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
