---
section: "Follow-on scenarios"
---

# A-retired-filename-leaves-a-decision-record

## Context

[§drift-prevention](../../../framework/constitution.md#drift-prevention) tells a contributor what to do when a name is retired — *"update every reference across the project's live artifacts in the same change"* — and then says what that costs: *"The sweep is uniform find-and-replace, which makes it a mechanical edit."*

That holds for a retired **identifier**. A spec slug, a command, a capability, a parenthetical descriptor: the moment it is renamed it is dead everywhere, so every occurrence is wrong and one substitution fixes all of them.

It does not hold for a retired **filename that a compatibility path still reads**, and the corpus proves it in both directions at once.

`.govern.toml` is the worked example. [042](../../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) moved the per-project config off the repo root and [049](../../049-rename-govern-to-ductus/spec.md) renamed the directory, so the current file is `.ductus/config.toml`. Measured 2026-09-13, the retired name still stands in 448 places across 119 tracked files.

**Outside the spec corpus, nearly every occurrence is correct.** The runtime's config resolution ladder reads the legacy name as its oldest tier so a pre-migration adopter still resolves (69 hits across 18 files under `runtime/src/`); a migration procedure must name the file it migrates, and `governance-config-rename.md` exists precisely to rename `.governance.toml` **to** `.govern.toml`; the commands, the bootstrap and the audit scripts spell that same ladder in prose and in shell. A blanket substitution breaks all three, and breaks them **silently** — the ladder still compiles, the migration still parses, and only an adopter on the old layout ever finds out.

**Inside the spec corpus the opposite is true, and nothing said so.** 317 of those hits are under `specs/`, 157 of them in living `spec.md` and scenario files spread across 17 feature directories. A spec corpus is a durable description of current state together with the record of the decisions that produced it. Three hundred references to a file that no longer exists is neither: it is residue, and it teaches a reader the wrong name in the same breath the spec claims to describe current behavior. 020's AC11 told adopters to configure a file 049 had already renamed, and stood ticked across four releases before `317d678` corrected it.

So one retired name needs opposite treatments on either side of that boundary, and §drift-prevention's single sentence covers neither.

## Behavior

**A retired filename that a compatibility path still reads is not swept like a retired identifier.** §drift-prevention states the distinction, and two rules follow from it.

**Exempt the sites that read or migrate the legacy name.** They are named as a *category* rather than as this repository's paths, because the rule ships to adopters whose layouts differ: a resolution ladder or fallback tier that reads the old name so an unmigrated adopter still resolves; a migration procedure, whose whole subject is the rename and which must therefore name both sides; and the prose, shell and configuration that spell either. An occurrence in one of those is **correct**, and it stays.

**Everywhere else the target end state is the current name plus a decision record.** A retired filename survives in the corpus only in the few references that record the decision to change it — here, 042's move and 049's rename — and not as a residue every later reader has to re-classify. This is the corpus-level form of what the constitution already states one tier down: git history is the record of what *was*, so a living artifact never has to be. The measure of a finished rename is not that the old name is gone, but that a reader meeting it knows immediately why it is still there.

**A ticked acceptance criterion is annotated, never rewritten.** A criterion is a claim about what was *delivered*, so substituting today's filename into it records a fiction — the behavior delivered named the file that was current at the time. The criterion's own text is left byte-identical and a superseded-naming annotation is appended to it. The corpus already does this: [017](../../017-derive-dont-ask/spec.md) and [027](../../027-bootstrap-migration-registry/spec.md) annotate theirs, and 022's AC6, AC9, AC12, AC16 and AC17 annotate superseded *behavior* the same way. The rule records an existing practice rather than inventing one.

## Edge Cases

- **One file, both dispositions.** 022's `cli-config-dir-per-contributor` states the resolution ladder in its Behavior, where naming the legacy tier is correct, and names the then-current session file two lines later, where it is drift. The unit of judgment is the occurrence, not the file — which is what makes this a per-hit pass and not a sweep.
- **A `plan.md`, `review.md` or `tasks.md` occurrence.** These are the design record and ephemeral work-tracking; they carry decisions as they were made, and they are left as written for the same reason a commit message is.
- **The artifact that *is* the decision record.** 042's and 049's own spec bodies name the retired file throughout, because the rename is their subject. They are the couple of references the end state deliberately preserves.
- **A migration between two names both now retired.** `governance-config-rename.md` converts `.governance.toml` to `.govern.toml`. It still names both: a procedure that no longer says what it converts cannot be audited against what it did.
- **A project with no compatibility path.** Then the exemption has nothing to exempt and the sweep is the ordinary uniform substitution §drift-prevention already describes. The distinction costs such a project nothing, which is why it is stated as a narrowing rather than as a second procedure.

## What this does not do

**Nothing detects the residue.** No check counts occurrences of a retired filename, and none could without being told which name was retired and which sites are the compatibility path — both of which are judgment. `check-corpus-links` catches a dead *link*; a dead filename sitting in prose resolves to nothing and is invisible to it. Saying so is the minimum owed while that stays true: the enforcement is that the renaming spec scopes its own sweep to include the corpus, and the rule is what tells it to.

**It does not settle whether the corpus rewrite reopens the specs it touches.** That question decides whether roughly fourteen `done` specs take the back-edge, and it is recorded as an open question below rather than assumed away in the pass that performs the work.

## Open Questions

- **Is the corpus rewrite a mechanical edit under [§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle) case (a), or does each living spec it touches take the back-edge?** The two halves of case (a)'s test point opposite ways. The substitution *changes no claim*: a spec saying `/ductus` reads `.govern.toml` means the config file, so naming it correctly asserts nothing new. But the diff is not *determinable without author judgment* — deciding which occurrences are the compatibility path, which are the decision record, and which are drift is exactly the judgment case (a) excludes, and is why this pass is per-hit rather than a sweep. The answer sets the cost: mechanical keeps roughly fourteen specs at `done`; meaningful takes each through the back-edge, `/ductus:review` and `/ductus:analyze`. Resolve it before the audit task runs, not while it runs.

## Resolved Questions

*None yet.*
