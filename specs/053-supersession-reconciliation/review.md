---
spec: 053-supersession-reconciliation
reviewed-at: 2026-08-30T22:17:42Z
reviewed-against: 2e39f32a0222befbecd2de837f13cf026dbee975
diff-base: 97d07ff
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 053-supersession-reconciliation

## Summary

Scope was this spec's whole implementation: `read-supersession-pair`, the criterion granularity on `write-supersession-annotation`, the `classifyClaims` extension point, the registration, the reconciliation steps in the shared Declaration semantics, and the constitution and `/{project}:analyze` documentation. The diff base is named explicitly (`97d07ff`, the plan commit) rather than derived: the `planned → in-progress` transition was recorded after the work rather than before it, so the derived window would have covered the status change alone.

All five passes ran. No outstanding findings.

**The design decision worth reviewing is the one that carries the spec's safety property.** AC7 bounds reconciliation's read, and the bound lives in a primitive with no argument for a plan, a data model, a tasks file, a source path, or a third spec — rather than in prose the host is asked to honour. That is the difference between this and the check the project measured and rejected at 455 pairs with every sampled firing a false positive. The extension-point payload is built from that primitive's result, so the bound holds at the boundary too, and a test asserts the *absence* of the excluded fields, because "we did not add them" is a property nothing else checks.

The reuse pass found nothing to raise and one thing worth naming: the criterion granularity went onto the existing annotation writer as an argument rather than into a second primitive. The constitution states one rule at three granularities, and spec 052's review had just measured the cost of splitting one concept across two implementations — two surfaces answering "is this spec already annotated?" differently, drifting until a review compared them. `read_spec`'s section and checkbox parsers are shared for the same reason: a reconciliation that disagreed with `read-spec` about what a spec's criteria *are* would classify claims the rest of the pipeline cannot see.

The quality pass looked hardest at the three outcomes that must not read alike (AC3, AC11, AC12), and they are separated by construction rather than by a caller's care: `unreadable` names what could not be examined and excludes it from `examined`; `guidance` marks a superseded spec with nothing to classify, which is examined-and-empty rather than examined-and-clean; and `still-standing` is a recorded classification rather than an omission, so examined-and-untouched cannot be mistaken for never-looked-at.

AC2 and AC6 are refusals, and both are kept by giving the code no way to break them: no primitive consumes the conflict list, and the criterion writer appends to the line while touching neither the checkbox nor the text, so a superseded criterion stays ticked. The criterion annotation's no-link property is proven the hard way — the test first asserts the criterion line *is* harvestable, since a proof over an unharvested line would prove nothing, and only then that no harvestable line carries the link.

The security, efficiency, and simplicity passes produced nothing. The reads are contained, no new primitive was added where an argument served, and the section-level granularity was deliberately left unimplemented rather than built for no caller.

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

- convention: reconciliation's conflict surfacing, the three-state report, and the body-prose edit gate live only in command prose — there is no test pinning that a conflict is never auto-resolved, the way `two_spec_commands.rs` pins step order. The step-order test does establish that no primitive consumes the classification's conflicts, which is the structural half; what is unpinned is the host's obligation to report all three states distinguishably. Worth a scenario if the report ever grows a second implementation. — `framework/commands/supersede.md`

## Skipped passes

*None.*
