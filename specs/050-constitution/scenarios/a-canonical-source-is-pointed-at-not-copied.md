---
section: "Behavior"
---

# A-canonical-source-is-pointed-at-not-copied

## Context

[§drift-prevention](../../../framework/constitution.md#drift-prevention)'s *Canonical sources* already required that other documents "reference the canonical source rather than restate it" — a MUST, and one adopters receive. What it did not say is what *referencing* means, and the gap had a shape: a document can satisfy "does not restate" while carrying a verbatim reproduction, because a copy is not a restatement. Nobody reading the rule would call a 500-line pasted snapshot a reference, and nothing in the text ruled it out.

The instance that exposed it was [020](../../020-code-review/spec.md). Its §Embedded artifacts section was a frozen copy of `framework/commands/review.md` — lines 242–802 of an 802-line spec, so **70% of the spec was a copy of another file**, wrapping a fenced snapshot against a live source that had since grown to 894 lines. It had fallen more than half behind, and the canonical-sources table one section up already named each command's own source as authoritative for command behaviour, so the copy was redundant on the day it was written. **It was replaced by a pointer on 2026-09-13**, taking 020 from 803 lines to 275; the section now records what 020 delivered to each command source and names the commit holding the original bytes. The account below is kept in the past tense it belongs in — the rule outlives its instance, and this scenario is the record of why it exists.

What made it worth a rule rather than a fix was that **nothing reported it, and nothing could**. The snapshot sat inside a code fence. `check-corpus-links` does not read inside fences; neither does `resolve-anchor`, nor any of the audit families. Measuring the section's own extent misreads it too — a scan for the next `##` heading stops 540 lines early, at a heading that belongs to the *copied* file. The divergence was found by a person reading the spec, which is the only instrument that had ever been able to find it.

The operator's ruling on 2026-09-13 settled the disposition and generalized it in the same breath: the pointer pattern "must be used when available", for token cost as well as drift.

## Behavior

**A reference is a pointer. A reproduction of the source's content is not a reference, and is never the right form.**

The constitution states this under *Canonical sources* with the two reasons it turns on, because the rule is weaker without them — a contributor who knows only "prefer a pointer" treats it as style, and style loses to the convenience of pasting.

- **Nothing can detect an embedded copy.** This is the [§design-principles](../../../framework/constitution.md#design-principles) asymmetry — a check that cannot run must never look like one that passed — reaching a case where no check runs *at all*. A fenced snapshot is not merely unchecked; it is unreachable by every instrument the project has, so its green status is not evidence of anything. A stale link is caught. A stale copy is not.
- **Every reader pays for it, in every session.** A copy is read in full by each contributor and each agent that loads the file, for as long as it exists; a pointer costs one line. This cost never appears in a diff — the copy is added once and charged forever — which is precisely why it has to be written down rather than left to notice.

Where the copy exists to preserve what something *used to* say, git history already holds it. A document body describes current state ([§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle)), so it never has to carry a frozen duplicate to serve as a record — the same reasoning that retires an obsolete scenario by deleting it rather than annotating it, and that keeps a retired spec out of the corpus rather than parked at `done`.

**Replacing an embedded copy is not a mechanical edit.** The replacement changes what the document asserts — from *this is the content* to *the content lives there* — so it is a factual correction, not a uniform token substitution, and it does not earn [§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle) case (a)'s mechanical-sweep exemption. A pass that removed several would reopen each spec it touched, except where the copy sat in a plan or task list, whose edits take no back-edge.

## Edge Cases

- **A short quotation supporting an argument** is not an embedded copy. Quoting a sentence to reason about it is how documents cite each other; the rule governs reproduction *in place of* a pointer, not illustration alongside one. The test is whether a reader could get the same thing by following the link — if the quote exists so they need not, it is a copy.
- **A template is not a copy of a canonical source.** `framework/templates/` exists to be reproduced into adopter repos; that is its function, and its canonical source is itself.
- **An example that must stay fixed to remain meaningful** — a golden file, a parity fixture, a migration procedure naming both sides of a rename — is a record of a specific state by design, and tracking the live source would destroy it. These are the same survivors [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s retired-filename rule protects, judged the same way: by tense. A copy describing what *is* goes; one describing what *was*, and saying so, stays.
- **The source and the copy living in one repository** does not make the copy cheap. 020 and `framework/commands/review.md` are in the same tree, one `git show` apart, and it drifted by more than 400 lines anyway.

## What this does not do

**Nothing enforces this, and the reason is the rule's own subject.** No check can read inside a fence and decide whether the bytes there are a copy of some other file — that is a similarity judgment across an unbounded set of candidate sources, not a lookup. `check-corpus-links` resolves links, which an embedded copy deliberately does not have. So this stays contributor and reviewer discipline, and saying so is required rather than optional: [§design-principles](../../../framework/constitution.md#design-principles) rejects a rule that implies enforcement it does not have, and [§grounding](../../../framework/constitution.md#grounding) makes the same demand of any claim about the machinery.

What *is* mechanically true, and worth stating so nobody reaches for the wrong tool: the failure is invisible to link checking by construction, so adding it to a link checker is not the deferred design — there is no derivable one today.

## Resolved Questions

- **Why the constitution rather than `AGENTS.md`?** Because it governs adopters too. An adopter's spec corpus accumulates embedded copies for exactly the reasons this one did, and `AGENTS.md` is contributor-side — it does not propagate. The operator directed this explicitly on 2026-09-13 after the rule was first landed as a contributor-side entry alone. `AGENTS.md` keeps a mirror that references this section rather than restating it, which is the rule applied to itself.
- **Why state the token cost in a governance document?** Because it is the half a contributor cannot see. Drift is at least discoverable once someone reads both files; the per-session cost of a copy is never visible in a diff, is paid by every future reader, and grows without bound. A rule that gave only the drift reason would be read as a tidiness preference, and lose to the convenience of pasting.
- **Does this reopen every spec carrying a copy?** No sweep is implied. The rule binds new writing, and an existing copy is corrected when it is next met — each correction being a back-edge on its own spec, as 020's was. Enumerating and clearing the corpus at once would be a distinct, measured piece of work, not a consequence of writing the rule down.
