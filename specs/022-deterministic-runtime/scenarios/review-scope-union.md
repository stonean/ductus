---
section: "Follow-on scenarios"
---

# Review-scope-union

## Context

`compute-review-scope` resolved the review file scope as **whichever of the
plan's `Affected Files` and the files-modified-since-`diff-base` was larger** —
one set or the other, explicitly not their union. The rule came from
[020](../../020-code-review/spec.md)'s original scope definition and the command
source stated it emphatically ("not a union; ties resolve to the modified-since
set"), so it read as deliberate rather than as an oversight.

Choosing one set can exclude the files the work actually touched. A mature
spec's `plan.md` lists the whole surface its original implementation covered;
a follow-on scenario touches a handful of files. The plan set therefore wins on
size, and the review is scoped to files the change never went near — while the
report gives no sign that the changed code was never examined.

This is not hypothetical, and it was found from the inside: the review of
[026](../../026-framework-self-audit/spec.md)'s Family 23 resolved a 15-entry
plan-affected set against an 11-entry modified-since set, so the scope handed
to the passes excluded the new family script, `AGENTS.md`, and
`framework/constitution.md` — every file the change introduced or edited. The
gate reported clean on a subject it had not looked at.

The failure grows with spec maturity, which inverts the property a quality gate
should have: the longer a spec has been worked, the larger its plan list, and
the more reliably a small follow-on is reviewed against the wrong files. Most
work in this repository is now follow-on scenario work on mature specs, so this
was the common case rather than the corner one.

## Behavior

`scope` becomes the **union** of `plan-affected` and `modified-since`,
deduplicated and sorted. Both sets, because either alone can omit what the
review exists to look at: the plan names the feature's surface, the diff names
what changed, and a review wants each for a different reason.

`plan-affected` and `modified-since` continue to be returned alongside it
unchanged, so a caller that wants one or the other still has it.

The cost is a larger scope, which was the original rule's implicit motivation.
It is bounded by `|plan| + |modified|` and the two sets overlap heavily in
practice — on the 026 run that surfaced this, 15 and 11 union to 24 rather
than 26. That is the correct direction to err: a review that reads a few extra
files is slower, and one that omits the changed files is wrong.

## Edge Cases

- **Neither set is authoritative alone, which is why this is a union and not a
  swap.** Preferring `modified-since` would fix the case above and break its
  mirror — a spec whose diff base is old enough that the plan names files the
  diff has long since stopped touching. The original rule's tie-break already
  reached for `modified-since` as "authoritative for what the work actually
  touched"; the union keeps that instinct without discarding the other set.
- **An empty `plan-affected` leaves `scope` equal to `modified-since`.** A spec
  with no `plan.md`, or a plan with no Affected Files table, is unchanged by
  this — the union of a set with the empty set is itself, so the lightweight
  track behaves exactly as before.
- **Empty scope still short-circuits.** When both sets are empty the scope is
  empty and `/{project}:review` writes the nothing-to-review-yet, non-blocking
  report as it always has. The union cannot manufacture a non-empty scope.
- **Ordering is deterministic.** The union is collected through a `BTreeSet`,
  so `scope` is sorted and deduplicated regardless of the order the two inputs
  arrive in — the idempotency invariant the command documents (same code plus
  same rules produces an identical report) depends on it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
