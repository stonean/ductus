---
section: "Rule set scope"
---

# A-bound-test-pins-the-stated-figure

## Context

A test of a bound — a size limit, a timeout, a retry count — that reads the constant it checks passes whatever the constant is, so moving the constant past the figure the spec states fails nothing. `QUAL-TEST-001` requires a test to fail when the behavior it names is removed, and a bound test that reads its own constant is one it does not visibly catch: the behavior is the stated figure, and the test pins only the value incidental to it. Reported 2026-10-25 at HEAD f24c55c6 as a candidate no shipped rule covers.

## Behavior

- `quality-cross.md` makes it a finding that a test of a bound reads the constant under test instead of pinning the figure its spec or contract states, so changing the constant fails the test.
- The requirement is stated either as part of `QUAL-TEST-001` or as a rule of its own, whichever the open question settles.

## Edge Cases

- A bound with no stated figure anywhere (an internal tuning constant): the trigger does not fire.
- A test that pins the figure and also reads the constant to assert the two agree: satisfies the requirement.
- A figure stated in more than one artifact with different values: a drift finding for the artifacts, not for the test.

## Open Questions

- Is this a clause of `QUAL-TEST-001` (a test that pins only a value incidental to the behavior does not catch its removal) or a rule of its own — and does either form fit the in-flight "rules are as short as their required content" pass, which cuts examples a rule does not need?

## Resolved Questions

*None yet.*
