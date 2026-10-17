---
section: "Follow-on scenarios"
---

# Reports End With the Remaining Work

## Context

An agent closed a round of work with a summary several paragraphs long: what changed in each spec, what the review found, what CI said, and, partway through, a release still owed. The operator could not tell from it whether anything remained — *"your summary is not clear, i'm not sure if there is work to do. it's a lot of text when it must be much more concise. you should end all summaries with either "No further work identified" or list identified work that is one of "must", "should", or "advisory""* (2026-10-17).

The summary was accurate. Its defect was shape: whether work remains is the question the operator reads a closing reply to answer, and narrative makes them search for it. The longer the reply, the better it hides the answer, and a remaining item mentioned in passing reads the same as a finished one.

The tiers are the pipeline's own. Review findings are MUST or SHOULD, analyze findings are blocking or advisory, and an outstanding SHOULD gates `done` unless waived ([§implement-phase](../../../framework/constitution.md#implement-phase)). Reusing them gives the operator a severity they already know how to act on.

The rule is universal rather than this repository's, because every project running the pipeline has an operator reading the agent's closing replies ([§drift-prevention](../../../framework/constitution.md#drift-prevention), *Shared knowledge stays in git*).

## Behavior

[§pipeline-boundaries](../../../framework/constitution.md#pipeline-boundaries) carries a bullet, beside *Report outcomes, not edits*, stating that a reply reporting on finished work is short and ends with exactly one of two things: the line *No further work identified*, or a list of the work still outstanding with each item tagged **must**, **should** or **advisory**.

- **Must**: the work is wrong or incomplete until the item is done — a blocking finding, a failing check, a broken contract.
- **Should**: the item is expected before the work counts as finished, as an outstanding SHOULD is at the review gate, unless a recorded reason waives it.
- **Advisory**: the item is worth knowing and nothing holds on it.

The bullet states its reason, the reader, and its boundary: the list reports work and does not dispose of it. Each item still takes one of the dispositions [§design-principles](../../../framework/constitution.md#design-principles) names, and the list says where each is recorded or that the decision is the operator's.

## Edge Cases

- **Nothing remains.** The last line is *No further work identified*. It means no item exists in any of the three tiers, not that nothing could be improved.
- **A remaining item is a caveat on a completion claim.** Not permitted, and this rule does not license it. Work that is not complete is fixed, recorded where the pipeline surfaces it, or decided out of scope ([§design-principles](../../../framework/constitution.md#design-principles)). The list names the record. It never stands in for one.
- **An item only the operator can decide** — cutting a release, a scope call, a status transition. It is listed with its tier and marked as the operator's decision, and the reply asks for that decision.
- **A reply mid-work**, such as a progress note or a question. The rule governs replies that report on finished work. A question asked mid-task is not a report.
- **Durable artifacts.** Commit messages, `review.md`, `analysis.md`, and spec and plan prose keep their own shapes. The rule covers chat replies, as *Report outcomes, not edits* does.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
