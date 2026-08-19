---
section: "Follow-on scenarios"
---

# Family-24-rename-sweep-residue

## Context

049 renamed the project with a word-boundary `govern` → `ductus` substitution. `govern` was in use as both a noun (the project) and an ordinary English verb, and the sweep could not tell them apart: it renamed the noun correctly and replaced the verb with a proper noun, producing sentences like *"a class of behavior the framework should ductus at the rules tier"*.

`framework/rules/security-frontend.md` shows the mechanism inside a single sentence — *"The other `FE-DEPS` rules **ductus** what code is loaded … none **governs** what a dependency does"* — where the inflected form survived only because it never matched the word boundary.

Eight sites survived across the corpus. Three of them ship to adopters, and one is `framework/templates/project/agents.md`, a `create`-strategy file: the broken sentence is written into a new adopter's `AGENTS.md` once and corrected by no later run. All 23 existing families were green the entire time; the residue was found by an ad-hoc grep during an unrelated review, which is the failure this family exists to remove.

## Behavior

The family reports any occurrence of the project name in a position where English grammar requires a verb. Two constructions, both drawn from closed word classes and therefore exact rather than heuristic:

- a **modal** (`should`, `must`, `shall`, `may`, `might`, `would`, `could`, `can`) immediately followed by the project name — a modal is always followed by a bare infinitive, and the project name is a proper noun, so the pair cannot be correct;
- the project name immediately followed by a **demonstrative or wh-word** (`this`, `what`, `how`, `across`, `whether`, `these`, `those`) — a proper noun does not take one.

Measured against the corpus: the union reports 8 findings at the commit before the repair — exactly the 8 real sites and no others — and 0 after it.

## Edge Cases

- **The project name as a legitimate object.** `the` is deliberately absent from the second list. *"with `PATH` stripped of ductus the same commit succeeds"* is correct prose, and admitting `the` would report it.
- **`to ductus` and `that ductus`.** Both excluded: *"a change to ductus"* and *"the version that ductus pins"* are ordinary and correct, so neither bigram carries signal.
- **Inflected forms.** `governs`, `governed`, and `governing` never matched the original sweep and were never damaged. The family looks for the *replacement*, not the survivor.
- **A degenerate scan.** A run that examines no files reports a finding rather than exiting clean: a corpus-wide grep that matches nothing is otherwise indistinguishable from a clean corpus (§design-principles). The examined-file count is reported on stderr for the same reason.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
