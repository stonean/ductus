---
section: "Follow-on scenarios"
---

# A-retired-filename-leaves-a-decision-record

## Context

[§drift-prevention](../../../framework/constitution.md#drift-prevention) tells a contributor what to do when a name is retired — *"update every reference across the project's live artifacts in the same change"* — and then says what that costs: *"The sweep is kept apart from unrelated edits, and mechanical when each change is one token substitution, uniform across those artifacts"*.

That holds for a retired **identifier**. A spec slug, a command, a capability, a parenthetical descriptor: the moment it is renamed it is dead everywhere, so every occurrence is wrong and one substitution fixes all of them.

It does not hold for a retired **filename that a compatibility path still reads**, and the corpus proves it in both directions at once.

`.govern.toml` is the worked example. [042](../../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) moved the per-project config off the repo root and [049](../../049-rename-govern-to-ductus/spec.md) renamed the directory, so the current file is `.ductus/config.toml`. Measured 2026-09-13, the retired name still stands in 448 places across 119 tracked files.

**Outside the spec corpus, nearly every occurrence is correct.** The runtime's config resolution ladder reads the legacy name as its oldest tier so a pre-migration adopter still resolves (69 hits across 18 files under `runtime/src/`); a migration procedure must name the file it migrates, and `governance-config-rename.md` existed precisely to rename `.governance.toml` **to** `.govern.toml`; the commands, the bootstrap and the audit scripts spell that same ladder in prose and in shell. A blanket substitution breaks all three, and breaks them **silently** — the ladder still compiles, the migration still parses, and only an adopter on the old layout ever finds out.

**Inside the spec corpus the opposite is true, and nothing said so.** 317 of those hits are under `specs/`, 157 of them in living `spec.md` and scenario files spread across 17 feature directories. A spec corpus is a durable description of current state together with the record of the decisions that produced it. Three hundred references to a file that no longer exists is neither: it is residue, and it teaches a reader the wrong name in the same breath the spec claims to describe current behavior. 020's AC11 told adopters to configure a file 049 had already renamed, and stood ticked across four releases before `317d678` corrected it.

So one retired name needs opposite treatments on either side of that boundary, and §drift-prevention's single sentence covers neither.

## Behavior

**A retired filename that a compatibility path still reads is not swept like a retired identifier.** §drift-prevention states the distinction, and two rules follow from it.

**Exempt the sites that read or migrate the legacy name.** They are named as a *category* rather than as this repository's paths, because the rule ships to adopters whose layouts differ: a resolution ladder or fallback tier that reads the old name so an unmigrated adopter still resolves; a migration procedure, whose whole subject is the rename and which must therefore name both sides; and the prose, shell and configuration that spell either. An occurrence in one of those is **correct**, and it stays.

**Everywhere else the target end state is the current name plus a decision record.** A retired filename survives in the corpus only in the few references that record the decision to change it — here, 042's move and 049's rename — and not as a residue every later reader has to re-classify. This is the corpus-level form of what the constitution already states one tier down: git history is the record of what *was*, so a living artifact never has to be. The measure of a finished rename is not that the old name is gone, but that a reader meeting it knows immediately why it is still there.

**A ticked acceptance criterion is swept like any other reference.** The requirement a criterion states is unchanged when only the file's name moved — `.ductus/config.toml` accepts the key `.govern.toml` accepted, and the criterion is as true after the substitution as before — so naming the file correctly asserts nothing new. A criterion whose **behavior** was superseded is not annotated either: the change that superseded it rewrites it to the requirement that holds, or deletes it where the behavior is gone ([§drift-prevention](../../../framework/constitution.md#drift-prevention)'s Decision resolution). An annotation says at length what the rewrite says exactly, and it keeps the retired name or the retired requirement in the corpus permanently — the residue this rule exists to remove. [017](../../017-derive-dont-ask/spec.md) and [027](../../027-bootstrap-migration-registry/spec.md) annotated theirs before this was settled; those annotations are removed rather than swept, since after the substitution they would read *"`.ductus/config.toml` no longer exists"*.

## Edge Cases

- **One file, both dispositions.** 022's `cli-config-dir-per-contributor` states the resolution ladder in its Behavior, where naming the legacy tier is correct, and names the then-current session file two lines later, where it is drift. The unit of judgment is the occurrence, not the file — which is what makes this a per-hit pass and not a sweep.
- **A `plan.md`, `review.md` or `tasks.md` occurrence.** A `plan.md` records the design as it stands, so it is swept like a spec body. A checked task's text is not kept at all: its spent section is pruned, and the commit keeps the record. `review.md` is regenerated whole by the spec's next review.
- **The artifact that *is* the decision record.** 042's and 049's own spec bodies name the retired file throughout, because the rename is their subject. They are the couple of references the end state deliberately preserves.
- **A migration between two names both now retired.** `governance-config-rename.md` converted `.governance.toml` to `.govern.toml` and named both: a procedure that no longer says what it converts cannot be audited against what it did. Its sunset (027, `c061c1f0`) moved its record to `CHANGELOG.md`, which names both still.
- **A project with no compatibility path.** Then the exemption has nothing to exempt and the sweep is the ordinary uniform substitution §drift-prevention already describes. The distinction costs such a project nothing, which is why it is stated as a narrowing rather than as a second procedure.
- **A spec carrying a stale annotation from an earlier pass.** Removing it rewords the line, so the analysis record and the review digest read that file as changed and the reworded criterion is judged at the spec's next review; the spec stays `done`, since the edit leaves no unchecked box or unticked criterion (spec 084). Committing it apart from the sweep does not rescue it: the staleness window is `reviewed-against..HEAD`, not a single commit, so both edits land inside it.
- **The last occurrence, corrected alone.** `mechanical_sweep`'s third part requires the rewrite pair to appear in more than one file, which is what separates a rename from a one-cell contract change. A single straggler fixed in its own commit is therefore read as changed beyond spelling, which owes its spec a fresh analysis and review but reopens nothing (spec 084) — one more reason the pass runs as one sweep rather than as a trickle.

## What this does not do

**Nothing detects the residue.** No check counts occurrences of a retired filename, and none could without being told which name was retired and which sites are the compatibility path — both of which are judgment. `check-corpus-links` catches a dead *link*; a dead filename sitting in prose resolves to nothing and is invisible to it. Saying so is the minimum owed while that stays true: the enforcement is that the renaming spec scopes its own sweep to include the corpus, and the rule is what tells it to.

**It does not decide which files the freshness checks read as unchanged — the diff does.** A `done` spec reopens only for owed code (spec 084), so neither the substitution nor an annotation reopens one. `mechanical_sweep` computes from the diff itself whether a change is the uniform substitution §drift-prevention's mechanical sweep describes, so neither this scenario nor the pass that performs the work gets to choose: a file taking only the substitution is read as no change by the analysis record's freshness and the review record's digest, and a file taking a criterion annotation is not. The split and its measured cost are recorded in Resolved Questions.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Is the corpus rewrite a mechanical edit under [§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle) case (a), or does each living spec it touches take the back-edge?** **Neither — the diff decides, per file, and none of it is the author's to choose; a `done` spec reopens only while code is owed (spec 084), so a corpus rewrite reopens none by being one.** `mechanical_sweep` (`runtime/src/primitives/mechanical_sweep.rs`) implements §drift-prevention's mechanical-sweep rule and derives the verdict from the diff deliberately: *"A commit trailer or an opt-out flag would make correctness depend on an author remembering to set it."* Its three parts are one-for-one line runs, an unchanged token count per line, and a rewrite pair appearing in more than one file — and `is_token_char` is `[A-Za-z0-9_.:/-]`, so `.govern.toml` and `.ductus/config.toml` are each a **single token**. A file taking only the substitution satisfies all three and keeps the exemption. A file whose line gains tokens, a reworded criterion for one, makes the token-count check return `None`, so it reads as changed beyond spelling and no exemption reaches it. Measured over the 17 affected spec directories: **8 are prose-only** (005, 016, 018, 022, 023, 028, 032, 050) and read as spelling alone; **9 carry a hit on an acceptance-criterion line**, 042 being the decision record.
