---
section: "Follow-on scenarios"
---

# A-decision-is-attributed-to-whoever-made-it

## Context

An adopter on 0.61.0 reported that `framework/reference/clarify.md` step 4 alone says to record a decision as the operator's only when the operator made it, and an adopted recommendation as "the agent's recommendation, adopted by the operator, DATE". Every command that records a decision in prose faces the same choice (a review's or an analysis's stored decision reason, a plan's rationale, a commit message) and none states it, so a recommendation the operator only adopted reads as the operator's own call.

## Behavior

- The constitution states the rule once, in §recommendations: a decision is attributed to whoever made it in the prose of every record a command writes, the operator's only when the operator made it, and one the agent recommended and the operator adopted as "the agent's recommendation, adopted by the operator, DATE".
- `framework/reference/clarify.md` step 4 points at the rule instead of stating it.
- The constitution stays within the 50,000 bytes `scripts/lint-governing-size.sh` holds (AC23).

## Edge Cases

- A decision the disposition rule made stays the rule's (`decided-by: rule`): neither the operator's nor a recommendation.
- A structured `decided-by` field names the person who decided or adopted; the rule governs the prose saying how the decision was reached.
- One approval of several recommendations adopts each: each reads as the agent's recommendation, adopted by the operator, with that approval's date.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
