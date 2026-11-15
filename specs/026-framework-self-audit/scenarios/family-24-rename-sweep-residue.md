---
section: "Follow-on scenarios"
---

# Family-24-rename-sweep-residue

## Context

049 renamed the project with a word-boundary `govern` → `ductus` substitution. `govern` was in use as both a noun (the project) and an ordinary English verb, and the sweep could not tell them apart: it renamed the noun correctly and replaced the verb with a proper noun, producing sentences like *"a class of behavior the framework should ductus at the rules tier"*.

`framework/rules/security-frontend.md` shows the mechanism inside a single sentence — *"The other `FE-DEPS` rules **ductus** what code is loaded … none **governs** what a dependency does"* — where the inflected form survived only because it never matched the word boundary.

**Nine** sites survived across the corpus. Three of them ship to adopters, and one is `framework/templates/project/agents.md`, a `create`-strategy file: the broken sentence is written into a new adopter's `AGENTS.md` once and corrected by no later run. All 23 existing families were green the entire time; the residue was found by an ad-hoc grep during an unrelated review, which is the failure this family exists to remove.

The ninth is `framework/rules/api-backend.md`, found 2026-09-15 and **invisible to the family's first two constructions by construction**. It read *"this rule and `BE-ERRENV-002` ductus its contract shape and code stability"* — the name followed by a **possessive determiner**, which is neither a demonstrative nor a wh-word. It ships to adopters too, so every adopter's rule set carried the broken sentence, and the family exited 0 over 514 markdown files for as long as it stood. A check that cannot see a live instance of the thing it exists to find is the [§design-principles](../../../framework/constitution.md#design-principles) failure this family was written against, turned on the family itself.

## Behavior

The family reports any occurrence of the project name in a position where English grammar requires a verb. **Three** constructions, all drawn from closed word classes and therefore exact rather than heuristic:

- a **modal** (`should`, `must`, `shall`, `may`, `might`, `would`, `could`, `can`) immediately followed by the project name — a modal is always followed by a bare infinitive, and the project name is a proper noun, so the pair cannot be correct;
- the project name immediately followed by a **demonstrative or wh-word** (`this`, `what`, `how`, `across`, `whether`, `these`, `those`) — a proper noun does not take one;
- the project name immediately followed by a **possessive determiner** (`its`, `their`, `our`, `your`, `his`, `her`) — a proper noun does not take one either, so the pair marks a verb slot. Added 2026-09-15 after the ninth site.

Measured against the corpus: the two-construction union reported 8 findings at the commit before the original repair and 0 after. **That was true of what it could see and an undercount of the corpus**, so *exactly the 8 real sites and no others* no longer stands — the ninth site sat outside both constructions. The third construction is calibrated on **recall over known-true instances** rather than on a count: at `9da4a7ae^` the verb took a possessive exactly once (`govern its`, the `api-backend.md` site) and `their` / `our` / `your` / `his` / `her` zero times. It is added as the whole closed class rather than as `its` alone because the existing lists are **grammar-calibrated, not frequency-calibrated** — measured at the same commit, three of the seven followers (`whether`, `these`, `those`) and seven of the eight modals had zero instances and are listed anyway.

## Edge Cases

- **The project name as a legitimate object.** `the` is deliberately absent from the second list. *"with `PATH` stripped of ductus the same commit succeeds"* is correct prose, and admitting `the` would report it.
- **`to ductus` and `that ductus`.** Both excluded: *"a change to ductus"* and *"the version that ductus pins"* are ordinary and correct, so neither bigram carries signal.
- **Inflected forms.** `governs`, `governed`, and `governing` never matched the original sweep and were never damaged. The family looks for the *replacement*, not the survivor.
- **`my` is excluded from the possessive class, by measurement rather than by argument.** It is the one member that fails the same test `the`, `to` and `that` fail: `README.md` documents adoption as *"/ductus my-project"*, so the name is followed by `my` in correct prose on the project's front page. Found by probing the **narrowed** direction before shipping the widening rather than by reasoning about it, and it costs no recall — `govern my` also occurs zero times at `9da4a7ae^`.
- **A ditransitive sentence would be a false positive, and that is accepted rather than hidden.** *"give ductus its due"* puts a possessive after the name legitimately. Measured absent — the construction occurs 0 times across the 584 tracked markdown files as of 2026-09-15 — where *"a change to ductus"* and *"stripped of ductus the same commit"* are both common, which is why those two exclusions stand and this one does not. If such a sentence is ever written, quote it or reword it rather than widening the exclusions.
- **Prose that documents this defect must quote the example.** The family strips code spans and double-quoted spans before matching, so a quoted example is the one place the broken phrasing is deliberate and invisible. Emphasis alone does not protect it: the inbox item recording the ninth site was written with `*italics*` and no quotes, and the widened family reported it immediately. Wrap such examples in `"…"` or backticks, the way this scenario and the README entry already do.
- **A degenerate scan.** A run that examines no files reports a finding rather than exiting clean: a corpus-wide grep that matches nothing is otherwise indistinguishable from a clean corpus (§design-principles). The examined-file count is reported on stderr for the same reason.
- **A path the family did not write as a literal is contained first.** Under the rule the plan's decision *Every family contains the paths it did not write as a literal* states once (`BE-INPUT-004`), any path of these kinds resolving outside the repository is reported and never touched: a tracked markdown file.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
