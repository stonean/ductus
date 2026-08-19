---
section: "Follow-on scenarios"
---

# Completion-claims-carry-no-caveats

## Context

An agent closing work found something outstanding — a stale claim caught at the completion gate, a detection gap it had not measured — marked the work `done`, and disclosed the residue in the closing summary. It did this three times in one session, on three different surfaces: closing a spec, reporting a review, and reporting a release.

Each disclosure was accurate. That is what made it durable: it reads as candour, and it costs nothing at the moment it is written. The user's objection named the actual defect — *"the work is either complete or it isn't; don't change the spec status to done and then say this."*

The cost is not the caveat's accuracy but its channel. A status field is re-read by every later command, generator, and reader; a sentence in a summary or a commit message is read once. Recording the exception in the second while advancing the first means `done` no longer means done, and no consumer of the status can tell which specs carry residue. When one of the three caveats was finally investigated, the "unmeasured" gap it described turned out to be 28 broken links — a number that took thirty seconds to obtain, and that the caveat had substituted itself for.

The constitution already stated this principle for one source of residue. [§implement-phase](../../../framework/constitution.md#implement-phase)'s SHOULD rule says a spec at `done` with an open finding *"is indistinguishable from unfinished work, and nothing ever comes back to it"* — the same asymmetry [§design-principles](../../../framework/constitution.md#design-principles) states for checks. It was scoped to review findings, and the residue here came from elsewhere, so nothing bound it.

## Behavior

[§design-principles](../../../framework/constitution.md#design-principles) carries a filter stating that work which is not complete must never be indistinguishable from work that is, and naming the only three dispositions for something known to be outstanding: fix it; record it where the pipeline surfaces it again, with the status following that record; or decide it is out of scope and record the decision with its reason. Disclosing it in prose beside a completion claim is none of the three.

The filter also binds the measurement case, because that is the form the failure took here: where the residue is knowable only by measuring, it is measured. An unmeasured gap is a task, not a caveat.

[§implement-phase](../../../framework/constitution.md#implement-phase)'s SHOULD rule is rewritten to reference the filter as the instance where it fires most often, rather than restating it — a second copy would drift, and the SSOT invariants family exists to catch exactly that.

The section preamble no longer carries a count of its own bullets. It read "Two constraints … Both are hard filters" while four bullets followed, which is the same staleness the constitution warns about elsewhere, sitting in the section that warns about it.

## Edge Cases

- **A genuinely out-of-scope observation.** Still permitted, and it is the third disposition — but it is *recorded* as a decision with its reason, not narrated. The bar is whether a later reader can find it without re-reading a transcript.
- **A caveat about work nobody asked for.** The standing rule against frontfilling still applies: the filter does not license capturing every adjacent observation. It governs what happens to residue *once it is known*, not what must be gone looking for.
- **A report that names what was excluded from scope.** Not a caveat. A check that reports what it did not examine is the first design principle being obeyed; the filter is about work claimed complete while known to be otherwise, not about honest scope statements.
- **Residue found after the status is already `done`.** The back-edge exists for this. Reopening is cheap and routine, and preferring a caveat to a reopen — to keep a status field looking clean — is the failure in its purest form.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
