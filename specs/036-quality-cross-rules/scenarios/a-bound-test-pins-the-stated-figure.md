---
section: "Rule set scope"
---

# A-bound-test-pins-the-stated-figure

## Context

A test of a bound — a size limit, a timeout, a retry count — that reads the constant it checks passes whatever the constant is, so moving the constant past the figure the spec states fails nothing. `QUAL-TEST-001` says a test SHOULD fail when the behavior it names is removed, and a bound test that reads its own constant is one it does not visibly catch: the behavior is the stated figure, and the test pins only the value incidental to it. Reported 2026-10-25 at HEAD f24c55c6 as a candidate no shipped rule covers.

## Behavior

- `quality-cross.md` makes it a finding that a test of a bound reads the constant under test instead of pinning the figure its spec or contract states, so changing the constant fails the test.
- The requirement is a sixth condition `QUAL-TEST-001`'s Verification flags, bounded by the Edge Cases below.

## Edge Cases

- A bound with no stated figure anywhere (an internal tuning constant): the trigger does not fire.
- A test that pins the figure and also reads the constant to assert the two agree: satisfies the requirement.
- A figure stated in more than one artifact with different values: the test is not flagged, and the disagreement is the artifacts' drift, which a review reports as an observation on them.
- A test exercising the bound on one side only, accepting the figure but never a value just past it, lets the constant rise unnoticed, so it does not pin the figure: pinning is asserting the figure itself, or the behavior at it and just past it.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **A clause of `QUAL-TEST-001`, or a rule of its own, and does either fit 008's "rules are as short as their required content" pass?** A clause: `QUAL-TEST-001`'s Verification gains a sixth flagged condition, and no ID is minted (decided by the owner's delegation, 2026-11-08). Its Statement already covers the case, since changing the constant breaks the stated behavior while the test passes, so a rule of its own would restate that Statement, and each finding under it would be a `QUAL-TEST-001` finding too, reported under two IDs against 008's data model (`specs/008-security-rules/data-model.md`), §Verification phrasing item 5. 008's scenario `rules-are-as-short-as-their-required-content` cuts restatement and keeps every trigger and compliant case, which a clause is and a second Statement is not. The clause is not clause (2) reworded: (2) flags assertions on a value incidental to the behavior, such as a length, and this one an expected value taken from the code under test, the constant, in place of the figure a spec or contract states. Rejected: a rule of its own, drafted at 851 bytes plus a 74-byte yield in `QUAL-TEST-001` and a criterion on this spec, against the 456 the clause added (`wc -c framework/rules/quality-cross.md`, 8,602 at `ec24b8a8` to 9,058 at `8e9d76ef`), for no case the clause misses; both forms touch the same files and land in the review this spec already owes.
