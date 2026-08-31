---
spec: 050-constitution
reviewed-at: 2026-08-31T18:20:00Z
reviewed-against: 07d8487f9acf89f6987075804872d55f0ec5c4c5
diff-base: c164d1bc527641d3cebb9066256d92c0a131f9c2
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 050-constitution

## Summary

Review of the §grounding subsection *a partial read is not a read*. Zero MUST violations, zero SHOULD violations, zero low-confidence findings, not blocking. Scope is 9 paths — the constitution, `CLAUDE.md`, the new scenario, and this spec's own artifacts.

**The rule closes a real hole rather than restating an existing one.** §grounding required that a source be consulted and said nothing about the consultation being complete; "Read the file; do not recall it" is satisfied exactly as well by a read that returned 2% of its subject. Nothing else in the constitution covered it — verified by scanning for a competing statement on truncation, previews, or partial reads, which found only the new text. The rule is placed inside §grounding after *Sources, in order of authority*, so the ordering reads: which sources are authoritative, then what counts as having read one.

**It is bound to an existing rule rather than asserted fresh.** The subsection names `QUAL-CLAIM-001` explicitly and states the correspondence — that rule requires a *result* to distinguish examined from unexaminable; this requires the agent's own *reading* to do the same. A reviewer can therefore cite the binding instead of arguing the analogy, which is what makes it enforceable in review rather than merely true.

**It states its own limit, which is the part most likely to have been skipped.** Nothing intercepts a tool choice, so the rule is a governed requirement cited by `/{project}:review` and `/{project}:analyze`, not a gate. Saying so is not a hedge: a rule that implied enforcement it lacks would be committing the defect it describes — a claim overstating what was verified. That is the same discipline `completion-claims-carry-no-caveats` applies to status fields, applied here to the rule's own reach.

**Single home, no second copy.** `CLAUDE.md` gains a one-line pointer at §grounding rather than a restatement, so the rule cannot drift between two files — the failure the SSOT invariants family exists to catch. The neighbouring trunk-based entry deliberately does **not** move into the constitution: adopters receive that document and this project's branching model is not theirs to inherit. That asymmetry is the reason the two entries are shaped differently and is worth stating, since a later editor could reasonably try to "tidy" them into one form.

**Adopter-facing by design.** The failure is not specific to this repository. Any adopter whose `AGENTS.md`, constitution, or command procedures grow past a tool's output cap has the same hole, and the agent reading them has the same incentive to accept the preview.

**Bounds on this review.** The subsection is prose in a governing document; there is no code to exercise and no test that can assert it — which is itself why the rule names its lack of enforcement. What was checked: placement inside §grounding, the 30 section markers intact, no competing statement elsewhere in the constitution, markdownlint clean across the constitution and all 050 artifacts, and `/{project}:audit` at 34/34. What was not checked, because nothing can: whether a future agent obeys it.

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

*None.*

## Skipped passes

*None.*
