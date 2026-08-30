---
status: planned
dependencies: [013-text-first-artifacts, 041-task-pruning, 051-branch-scoped-spec-numbering]
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 46
---

# 052 — Spec supersession and consolidation

Record what happens when a later spec counters an earlier one, and give the corpus a way to act on it: a declared supersession that annotates and keeps, and a consolidation that merges and removes.

## Motivation

Spec bodies are living documents that represent current state, so the framework's answer to a countered decision was always to edit the earlier spec. The machinery for that existed — the `done → in-progress` back-edge, the mechanical-edit classification, the criterion-level supersession rule — but nothing recorded the *relation*. A spec that countered an earlier one said so nowhere a reader or a tool could find.

The gap showed up in two places. Twelve of fifty-one specs carried a supersession annotation of some kind, in three different shapes (a whole-spec sunset banner, a section-level post-completion note, a per-criterion inline annotation), and only the third had a written rule. The other two were convention, applied by hand, discoverable only by reading specs that already had one. An adopter inheriting the framework had no way to learn the pattern existed.

The second was recovery cost. Detecting supersession after the fact was measured and rejected: 455 criterion pairs tested, 215 would have fired, every sampled one a false positive. The information is cheap at the moment the countering spec is written and effectively unrecoverable afterward — but nothing asked for it then.

Left alone, the two compounded. A corpus accumulated specs whose decisions later specs contradicted, with no marker distinguishing a live decision from a countered one, and the reflex was to delete the stale ones — which stranded every inbound pointer, since deletion re-points nothing.

## Two outcomes, one question

A countered spec resolves one of two ways, and the operator's real decision is which applies:

| | Content | Source directory | Prior standing |
| --- | --- | --- | --- |
| **Supersede** | stays, annotated | survives | shipped and delivered |
| **Consolidate** | merges into target | removed | redundant or never distinct |

They are opposite operations and must not be collapsed. A spec that shipped and was later countered is the historical record of what shipped; folding it into the spec that removed it would invert the relationship. A spec that overlapped a sibling from the start was never a separate concern and should not stay one.

Both must be reachable from this spec, because an operator who learns only one will use it for both.

## Declaring supersession

Supersession is **declared, not derived** — at creation, where the countering spec is being written, or over a pair that already exists. Either route captures the intent at the moment the author holds it, which is the only moment it is cheap; nothing recovers it afterward.

- `/{project}:specify --supersedes <feature>` writes a `supersedes:` frontmatter key on the new spec — a first-class pointer field, following the `folds-into:` precedent established by [051 — Branch-scoped spec numbering](../051-branch-scoped-spec-numbering/spec.md), and permitted immediately by the open-schema rule in [013 — Text-first artifacts](../013-text-first-artifacts/spec.md).
- **The pointer is frontmatter, never a body link.** `derive-dependencies` harvests body links to sibling specs into `dependencies:`, so a link here would make the superseding spec declare a dependency on the spec it supersedes — silently, on the first commit through the pre-commit hook. Citing a superseded spec by name in prose is the existing rule for the criterion-level case and holds here.
- The flag is offered as a **classification on a derived routing candidate**, not as a cold flag. `derive-routing-candidates` already runs before anything is scaffolded; when it surfaces a candidate, "this supersedes it" joins "amend it" and "unrelated" as an answer. Forgetting the flag then costs nothing, because the candidate surfaces anyway — a bare flag would be a diligence dependency, which §design-principles forbids.
- Lexical derivation cannot reach every case (matching is over slug vocabulary), so the flag remains available directly for work whose predecessor shares no vocabulary with it.

### The reciprocal annotation

The key on the new spec is bookkeeping. What prevents a reader — human or agent — from mistaking a countered spec for a live one is the annotation written on the **superseded** spec, naming the spec that countered it and what no longer holds.

The annotation is assembled the way every authored artifact crosses the runtime boundary — the **content-ingestion convention** `create-scenario` already uses. The primitive contributes the *frame*: placement, the blockquote wrapper, the `> **Sunset ({link}):**` citation, and the closing sentence that the spec stays `done` as the record of what shipped. The author contributes the *substance*: what no longer holds. A generated banner can name the superseding spec and the date and nothing else, and the sentence a reader actually needs is the one naming what stopped being true.

The blockquote is load-bearing rather than cosmetic. `derive-dependencies` exempts blockquote-prefixed lines, so a banner may link the superseding spec without the annotated spec acquiring a dependency on it — which is how `005-workflows` links `043-workflows-sunset` in its sunset note while its `dependencies:` names only `004` and `010`. A **criterion-level** annotation has no such exemption: it is a plain list item, so it cites by name and points at the banner that carries the link.

Phrase the annotation as a **non-claim** ("no longer exists", "is removed") rather than as an assertion whose truth depends on whether its paths resolve — `check-artifacts`' criterion-path-existence family skips such text as `not-a-live-claim`, and a banner phrased as a live claim would fall under a check it was never meant to answer to.

Writing it is a **mechanical edit** under §spec-lifecycle: it changes no claim the superseded spec makes about what it delivered, so it takes no back-edge and the spec stays `done`. That is what makes it cheap enough to apply without operator intervention.

The annotation has three granularities, and this spec's cross-spec impact is to codify all three in the constitution — the whole-spec sunset banner and the section-level post-completion note alongside the criterion-level rule already written down.

## Declaring on an existing pair

The surface is its own command rather than a flag on an existing one, and the reason is scope rather than taste. The commands split on how many specs they write: `amend`, `prune`, `clarify`, `plan`, and `implement` each write one, while `fold` writes two. A supersession is inherently two-spec — the key on one, the annotation on the other — so it does not fit the single-spec boundary those commands declare, and widening one to accommodate it is the erosion rejected for `/{project}:fold` in §Consolidation. `/{project}:supersede` therefore joins `/{project}:fold` and `/{project}:consolidate` in the two-spec group.

That distinction is currently undocumented, and it is the thing that explains why several operations are separate commands rather than flags. The README states it, and names which commands sit on each side.

A corpus adopted before this feature existed carries conflicts nobody declared, and `--supersedes` reaches only specs not yet written. The relation is therefore separable from spec creation: a supersession may be declared over **two specs that already exist**, producing the same key and the same reciprocal annotation as a declaration made at creation. Reconciliation over the pair's individual claims belongs to 053 and is **not** required for a declaration here to be complete — this spec's outcome is the recorded relation, not the claim-level walk. This is the path an adopting project uses to clean up, and it is the case that motivated the feature — a corpus of small overlapping specs whose later decisions counter earlier ones, with deletion as the operator's only current recourse.

For any given pair the operator's real choice is which of this spec's two outcomes applies: reconcile and keep both, or consolidate and remove one.

## Consolidation

Consolidation removes a spec directory whose content belongs with another. The mechanism already exists and is exercised by fold-back: `rewrite-spec-links` re-points every inbound pointer at the target, and `retire-feature` removes the directory, refusing when the target holds no `spec.md` so content is never stranded.

- `/{project}:consolidate <spec> --into <spec>` is a **cleanup command**, in the family of [041 — Task pruning](../041-task-pruning/spec.md): operator-initiated, confirmed, not a pipeline state transition, with git history as the only recovery.
- It performs **none** of fold's content migration — no body edit, no scenario creation, no task append, no status change, no review invalidation. Fold moves content because a staging spec was never meant to stand alone; consolidation assumes the merge already happened or that nothing needed merging.
- `retire-feature` currently refuses the sequential directory form outright. That refusal exists because a sequential spec is completed rather than removed, and it is relaxed here for an explicitly targeted consolidation — not removed. The anti-stranding guard is unchanged.
- Because it does not migrate content, the confirmation must name **content loss**, not merely directory removal: the guard proves the target exists, never that anything landed there.

**The source spec's scenarios go with its directory.** Consolidation migrates nothing, so a scenario under the removed spec is destroyed along with everything else in it — deliberately unlike fold-back, which creates one scenario under the upstream spec for every scenario the retiring spec carried. The confirmation names them explicitly rather than leaving them inside a general claim about content: a scenario is a distinct artifact with its own open-question gate, and an operator who reads "the spec is removed" does not necessarily picture them.

Adding `--into` to `/{project}:fold` was considered and rejected. Fold's purpose, its enumeration step, its post-merge instruction, and its single-source rule for the fold target are all specific to the branch-scoped staging form; carrying a sequential path through them would qualify seven load-bearing statements and erode the one thing fold says clearly.

## Interruption and re-runs

Every command here writes two specs, and the runtime provides no transaction spanning them — the same condition fold-back documents, where the recovery for an interruption is not a rollback but a second run. The three commands inherit that contract, so each write is built so a re-run is a no-op where the first attempt already landed.

Declaring a supersession that is **already declared** is therefore not an error and not a duplicate: the `supersedes:` entry is not added twice, and no second annotation is written for a superseding spec already named in one. This is distinct from accumulation, which stacks annotations from *different* superseding specs — the same spec declaring twice is a re-run, not a second supersession.

An already-applied step reports that outcome as a domain result rather than a failure, matching `retire-feature`'s already-absent and `invalidate-review`'s already-invalidated.

## See also

Reconciliation — what happens to the superseded spec's individual claims once a supersession is declared — is specified separately in [053 — Supersession reconciliation](../053-supersession-reconciliation/spec.md). The split runs one way: this spec ships without it, since a declaration plus an annotation is useful on its own, while reconciliation is meaningless without a declared edge to scope it. The link sits under this heading deliberately — `## See also` is the author opt-out that keeps a navigational pointer out of `dependencies:`, and 053 already depends on this spec, so a harvested edge here would close a cycle.

## Acceptance Criteria

- [ ] AC1: `/{project}:specify --supersedes <feature>` writes a `supersedes:` key naming that feature into the new spec's frontmatter
- [ ] AC3: Creating a spec with `--supersedes` writes an annotation onto the superseded spec naming the superseding spec
- [ ] AC5: `--supersedes` is offered as a selectable classification when `derive-routing-candidates` surfaces a candidate, and omitting the flag entirely leaves spec creation behaving exactly as it does today
- [ ] AC6: The constitution documents the supersession annotation at whole-spec, section, and criterion granularity, including that the citation names the superseding spec rather than linking it
- [ ] AC7: `/{project}:consolidate <spec> --into <spec>` re-points every inbound pointer to the source directory at the target, then removes the source directory
- [ ] AC8: `/{project}:consolidate` refuses when the target directory holds no `spec.md`, and no pointer is rewritten by the refused run
- [ ] AC9: `/{project}:consolidate` writes nothing to the target spec's body, scenarios, `tasks.md`, `status`, or `review:` block
- [ ] AC10: The confirmation prompt shown before a consolidation names the loss of the source spec's content, not only the removal of its directory
- [ ] AC11: `retire-feature` accepts a sequential feature directory only through an explicitly targeted consolidation, and `/{project}:fold` remains unable to retire one
- [ ] AC12: `/{project}:fold` gains no flag or argument for naming a fold target, and its `folds-into:` single-source rule is unchanged
- [ ] AC13: A spec declaring `supersedes: <feature>` whose named target's body does not name it back is reported as an advisory finding, and the check never infers an undeclared supersession
- [ ] AC14: The annotation written on a superseded spec is blockquoted, so the link it carries to the superseding spec induces no `dependencies:` edge on the spec being annotated
- [ ] AC15: A criterion-level supersession annotation cites the superseding spec by name rather than by link, since a plain list item's link is harvested into `dependencies:`
- [ ] AC16: The annotation's substance — what no longer holds — is author-supplied; the primitive contributes the frame (placement, blockquote, citation, and the record-of-what-shipped closer) and never invents the substance
- [ ] AC17: `supersedes:` holds a list of feature slugs, so one spec may supersede several in a single declaration
- [ ] AC18: A second supersession annotation on an already-annotated spec accumulates rather than replacing the existing one, and the newest is placed first
- [ ] AC24: A supersession naming a spec that is not `done` is accepted rather than refused, with consolidation named as the likelier outcome and the reason stated
- [ ] AC25: `/{project}:consolidate` is installed into adopter projects by the bootstrap, and its documentation identifies it as the only command that removes a durable artifact
- [ ] AC26: Consolidation names every spec whose `supersedes:` points at the source and offers re-point, drop, or cancel, defaulting to none of them
- [ ] AC27: Consolidation names the specs the source itself superseded, whose annotations cite a spec the removal will delete
- [ ] AC28: No `supersedes:` edge is re-pointed without the operator settling it, and consolidation is never refused on the presence of such an edge alone
- [ ] AC29: The reciprocity check reports its coverage as bounded to declared `supersedes:` edges, so a corpus of undeclared supersessions is never reported as clean
- [ ] AC33: A retroactive supersession is declared through its own command over two existing specs, not through a flag on a single-spec command
- [ ] AC34: The README states which commands write to one spec and which write to two, and places `fold`, `consolidate`, and `supersede` in the two-spec group
- [ ] AC38: The frontmatter schema documents `supersedes:` as a hand-authored list, distinct from the generated `dependencies:` and `references:` indexes that must not be edited by hand
- [ ] AC39: Spec 051's account of `retire-feature`'s sequential-form refusal is updated to record that this spec relaxes it for an explicitly targeted consolidation
- [ ] AC40: A supersession can be declared over two specs that already exist, producing the same `supersedes:` key and reciprocal annotation as a declaration made at creation, with no dependency on reconciliation
- [ ] AC41: A spec that receives only the supersession annotation keeps whatever status it already had, at any lifecycle state — the write is mechanical and takes no back-edge
- [ ] AC42: Consolidation destroys the source spec's scenarios with its directory and migrates none of them to the target, and the confirmation names the scenarios specifically rather than only naming content
- [ ] AC43: Re-declaring an already-declared supersession adds no second `supersedes:` entry and writes no second annotation for the same superseding spec
- [ ] AC44: An interrupted declaration or consolidation converges when re-run, each step reporting an already-applied outcome as a domain result rather than a failure
- [ ] AC45: A declaring command writes no markdown link to the superseded spec into the superseding spec's body, and `derive-dependencies` derives no edge from the superseding spec to the superseded one

## Open Questions

## Resolved Questions

- **Should a `supersedes:` edge be discharged like `folds-into:`, or is it purely descriptive?** Descriptive. It names a completed fact, so it gates no transition and appears on no backlog surface; `folds-into:` blocks precisely because it names work *not yet done*. Verification is handled separately and deterministically: because the edge is **declared**, a check can confirm the named spec's body names the superseding spec back without ever inferring the relation. That is what distinguishes it from the criterion-supersession check that was measured and rejected (455 pairs tested, 215 would fire, every sample a false positive) — that check asked *"does A supersede B?"*, this one asks *"does A's body name B?"*. It lands as an advisory `check-artifacts` family surfaced through `/{project}:analyze`, beside the existing link-adjacent-decision-drift and acceptance-criterion-path-existence families, both of which are likewise advisory and likewise check a claim against the tree.
- **Can the reciprocal annotation be generated deterministically?** Partly — split it at the content-ingestion seam. The primitive frames (placement, blockquote, citation, the record-of-what-shipped closer); the author supplies what no longer holds, which is the only part a reader needs and the only part nothing can generate. The blockquote wrapper is what lets the banner link the superseding spec without inducing a `dependencies:` edge, since `derive-dependencies` exempts blockquote-prefixed lines; a criterion-level annotation gets no exemption and so cites by name. Both halves are already practised in `005-workflows` — its banner links `043` from a blockquote while its `dependencies:` names only `004` and `010`, and its AC1 cites `043` by name, pointing at the banner "which carries the link".
- **One key or a list, and does a second annotation replace or accumulate?** A list, and accumulate. Spec `043-workflows-sunset` superseded material in four specs in one change (`005`, `004`, `019`, `010`), so a scalar key would have forced either four specs or a false record — this is where `supersedes:` diverges from the correctly-scalar `folds-into:`, since a staging spec has exactly one home while a superseding spec may counter several predecessors. Annotations stack newest-first: `005-workflows` carries four today, and its sunset banner *scopes* the three post-completion notes beneath it ("everything below … describes behavior that no longer exists") rather than deleting them. Replacing an earlier annotation would destroy the record of an intermediate state, which is what the annotation exists to preserve.
- **Should a supersession name a spec that is not `done`?** It is accepted, but consolidation is recommended instead. The two-outcomes table gives supersession's prior standing as *shipped and delivered*, which is the entire justification for keeping the superseded spec on disk; a spec at `draft` or `clarified` delivered nothing, so annotating it would mark a decision that was never enacted as one that was and then stopped being true — the *records a fiction* failure the reconciliation triage exists to catch. Refusing would still be wrong: `in-progress` is genuinely ambiguous, and the operator may hold context the status does not carry. The stance matches `derive-routing-candidates` — it reports, it does not veto.
- **Is `/{project}:consolidate` adopter-facing or maintainer-only?** Adopter-facing. The `/{project}:prune` precedent is weaker than it looks — prune destroys `tasks.md`, which the project classes as ephemeral work-tracking, whereas `spec.md` is a durable source of truth — so the question is right to flag the asymmetry. What decides it is the alternative: this feature exists because an adopting corpus needed cleaning up, and a maintainer-only command does not prevent an adopter removing specs, it only guarantees they do it with `rm -rf` — no pointer rewriting, no anti-stranding refusal, no confirmation naming what is lost. Safety comes from the guards, not from restricting distribution. The documentation carries the one thing the maintainer-only reading was right about: this is the only command that removes a durable artifact, and it should not read like an ordinary table row.
- **Should consolidation refuse over a `supersedes:` edge?** No — surface both directions and let the operator settle each; never re-point silently. A `supersedes:` edge is a declared pointer like `folds-into:`, so it cannot be ignored the way a derived index can, but unlike a body link it is a *claim about a relationship*: re-pointing it would give the declaring spec a claim to supersede the consolidation target, an assertion nobody made and usually false. That is the hazard `check-orphaned-references` refuses to take on — a rewrite that guessed wrong is worse than a precise report. Inbound edges offer re-point, drop, or cancel; outbound edges are named because the specs the source superseded carry annotations citing a spec that is about to disappear. Refusing outright would block exactly the tangled-corpus cleanup this feature exists for.
- **Is there a migration obligation for the specs already carrying hand-written annotations?** No automatic migration, because none is available. The two-tier-corpus objection that justified `criterion-label-backfill` applies in principle, but that backfill is deterministic — a label is assigned by counter and position, with no judgment — whereas backfilling `supersedes:` would require deciding which spec supersedes which, the inference measured at 455 pairs with 215 firing and every sample a false positive. A migration guessing those edges would write false relations into every adopter's frontmatter unattended, which is worse than the two-tier corpus it would be avoiding. The existing annotations stay valid: codifying a convention they already follow does not invalidate them, and they carry the substance a reader needs. What they lack is the machine-readable edge, so the reciprocity check never fires on them — a coverage bound the check must state rather than report as a clean corpus. Backfill is opt-in and per pair, through the retroactive declaration path, which an adopting project needs regardless.
- **What surface does a retroactive declaration take?** Its own command, `/{project}:supersede <superseded> --by <superseding>`. A flag on `/{project}:amend` is the strongest alternative — amend is classifier-driven and already owns the lifecycle back-edges that Q on editing invokes — but a supersession writes to two specs, and amend declares a single-spec scope. The corpus already splits this way: one-spec commands are `amend`, `prune`, `clarify`, `plan`, `implement`; two-spec commands are `fold` and now `consolidate` and `supersede`. All three share one reconciliation by reference, the way `/{project}:groom` and `/{project}:specify` share one routing tree. The cost is honest — this spec adds two commands to the surface — and the mitigation is that each keeps a purpose statement needing no qualifier.
