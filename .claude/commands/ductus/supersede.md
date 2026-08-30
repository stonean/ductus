---
description: Declare that one spec supersedes another, over two specs that already exist.
argument-hint: "[superseded-feature] --by <feature>"
---

# Supersede

Record that a later spec counters an earlier one: the `supersedes:` key on the superseding spec, and the reciprocal annotation on the spec it counters.

## Purpose

Retroactive declaration. `/ductus:specify --supersedes <feature>` captures the relation at creation, which is the moment it is cheapest; this command captures it over **two specs that already exist**, producing the same key and the same annotation. A corpus adopted before this feature existed carries conflicts nobody declared, and a flag on spec creation reaches only specs not yet written — so this is the path an adopting project uses to clean one up, per [052 — Spec supersession and consolidation](../../specs/052-spec-supersession-and-consolidation/spec.md).

Supersession **keeps** the earlier spec. It stays on disk, annotated, as the record of what shipped. The opposite operation — merging a spec into another and removing its directory — is `/ductus:consolidate`, and the two must not be confused: a spec that shipped and was later countered is the historical record of what shipped, while a spec that overlapped a sibling from the start was never a separate concern.

This command declares the relation and stops there. Reconciling the superseded spec's individual claims is separate work; a declaration plus an annotation is useful on its own, and nothing here waits on it.

## Context

Parse `$ARGUMENTS` for the **superseded** feature — the spec being countered — and resolve it through `resolve-feature` (a feature number, partial name, or full directory name; `ambiguous` and `not-found` are domain outcomes to surface, not errors to swallow). With no argument, use the session target from `.ductus/session.toml`; if neither resolves, stop and tell the user to name the spec being superseded.

The **superseding** spec comes from `--by` and is resolved the same way. It is required and has no session fallback: a declaration has two ends, and inferring one of them from whatever the session happens to point at would record a relation the operator never stated. A spec may not supersede itself — refuse that rather than writing a key that cites its own file.

Neither spec's status gates the declaration. Both directories must exist, which is what distinguishes this from the creation-time flag: there the superseding spec is being written, here both are already on disk, and that is the first and only moment resolvability can be enforced.

### Flags

| Flag | Behavior |
| --- | --- |
| `--by <feature>` | The **superseding** spec — the one that counters the spec named by the argument. Required; a declaration with only one end is not one |

## Scope Boundaries

- This command writes to **two** specs: the `supersedes:` frontmatter key on the superseding spec, and the reciprocal annotation in the body of the superseded one. It writes nothing else — no status change on either spec, no scenario, no task, no review invalidation, and no session-target write.
- The annotation is a **mechanical edit** under §spec-lifecycle: it changes no claim the annotated spec makes about what it delivered, so the spec keeps whatever status it holds and takes no back-edge, at any lifecycle state.
- `dependencies:` and `references:` are never hand-edited here. They are derived from body links; the annotation's link sits inside a blockquote precisely so it induces no edge.
- Do NOT read or modify source code, plans, tasks, or data models. A supersession is a relation between two specifications.
- Reference: §spec-requirements (the annotation, at all three granularities), §spec-lifecycle (mechanical edits and back-edges), §text-first-artifacts (the frontmatter schema, and which pointer fields are hand-authored), §spec-phase (spec-root resolution) (constitution loaded by `/ductus:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write) per the Markdown-only reference below.

**An interrupted declaration is completed by re-running it.** The runtime provides no transaction spanning two specs — the condition `/ductus:fold` documents for the same reason — so the recovery for an interruption between the two writes is not a rollback but a second run, and each write is built so a re-run is a no-op where the first attempt landed:

| Step | What makes a re-run safe |
| --- | --- |
| 3 — the `supersedes:` key | A list. An entry already naming the superseding target is not added twice; the key is left byte-identical. |
| 4 — `write-supersession-annotation` | An annotation already citing this superseding spec reports `already-present: true` and writes nothing. |

Both report the already-applied case as a **domain outcome**, not a failure — the same shape `retire-feature`'s already-absent and `invalidate-review`'s already-invalidated take. Re-declaring an already-declared supersession is a convergent re-run, and it is distinct from accumulation: two *different* superseding specs each get their own annotation, stacked newest first.

**On `ductus exec` this command is reduced, and the reduction is not benign.** Step 3 is the frontmatter append, which has no primitive of its own, so an exec walk dispatches step 4's annotation and never writes the `supersedes:` key — half a declaration, and the worse half to have alone: the annotation is what a *reader* needs, while the key is what every *check* reads, so an annotated spec carrying no key is invisible to `supersession-reciprocity` and to anything else that walks declared edges. Report the run as incomplete and name the key as still owed, rather than reporting a successful declaration; a host with file tools writes it directly and completes the pair. The reduction is documented here rather than left silent, per §runtime-host-integration's two-paths guarantee.

1. Invoke `read-spec` (with `include-body`) against both specs — the superseded one for its `status` and its body, the superseding one for its frontmatter. Report three things before anything is written: whether `supersedes:` already names the target, whether the superseded body already carries an annotation citing the superseding spec, and the superseded spec's status. A status other than `done` is **accepted, never refused**: supersession's justification is a spec that shipped and was later countered, which is the whole reason the superseded spec stays on disk, and a `draft` or `clarified` spec delivered nothing to counter — annotating it would record a decision as enacted-then-undone when it was never enacted. Say that, name `/ductus:consolidate` as the likelier outcome, and proceed if the operator still wants the declaration. `in-progress` is genuinely ambiguous and the operator may hold context the status does not carry, so this reports and does not veto.

2. Invoke `gate-confirm` with a `gate` name (e.g. `supersede-declare`) and a `prompt` naming **both** writes before either happens: the key added to the superseding spec, and the annotation added to the superseded one. Name any already-applied half step 1 found, so a convergent re-run reads as one rather than as a second declaration. Name the non-`done` guidance here too when it applies, so the operator answers it at the same moment they consent. No write happens before this step; denial ends the run cleanly with nothing written.

<!-- audit:ignore-promotion -->
3. Write the `supersedes:` key into the superseding spec's frontmatter (host responsibility; the frontmatter edit has no primitive of its own, and `supersedes:` is the one pointer field an author writes). It is a **list** of feature slugs — one spec routinely counters several in a single change, which is where it diverges from the scalar `folds-into:` — and it is **hand-authored**, deliberately unlike the generated `dependencies:` and `references:` indexes beside it, which no command writes by hand (§text-first-artifacts, Frontmatter Schema). Append the superseded feature only when the list does not already name it; an existing entry is left alone and reported. The key is absent rather than empty when nothing is superseded, so a first declaration creates it.

4. Invoke `write-supersession-annotation` with the superseded feature, the superseding feature, and the **substance** — what no longer holds — as one authored payload. This is the same content-ingestion convention the scenario writer uses: the primitive contributes the frame (placement at the top of the body, the blockquote wrapper, the `> **Sunset ({link}):**` citation, and the closer recording that the spec stays the record of what shipped), and never invents the substance. A generated banner can name the superseding spec and the date and nothing else; the sentence a reader actually needs is the one naming what stopped being true, and only an author can write it. Phrase it as a **non-claim** ("no longer exists", "is removed") rather than as an assertion whose truth depends on whether its paths resolve. An `already-present: true` result is the convergent re-run — report it and continue.

<!-- audit:ignore-promotion -->
5. Report the declaration (host responsibility): the two specs, which writes landed and which were already applied, and the superseded spec's status — unchanged, because the annotation is a mechanical edit. Recommend committing: the reciprocity check reads the declared edge against the annotated body, so it has something to check only once both writes are on disk together.

## Markdown-only reference

With no ductus runtime registered, the host performs the same walk and the same writes with its own file tools (Read, Edit, Write) — no shell-pipeline substitution — one contract, two paths (§runtime-host-integration).

### Declaration semantics

This section is the **single canonical statement** of what declaring a supersession means. `/ductus:specify --supersedes` performs the same declaration at creation time and points here rather than restating it; a second copy is how the two drift apart.

A supersession is **declared, not derived.** Detecting one after the fact was measured and rejected — 455 criterion pairs tested, 215 would have fired, every sampled one a false positive. The information is cheap at the moment the countering spec is written and effectively unrecoverable afterward, so it is captured by an author who holds the intent, never inferred by a check.

A declaration is two writes on two specs:

1. **The key.** `supersedes:` on the superseding spec, a list of feature slugs. Hand-authored, and the one pointer field an author writes — unlike `dependencies:` and `references:`, which are generated indexes. A list rather than a scalar because one spec routinely counters several at once; `folds-into:` is correctly scalar because a staging spec has exactly one home.
2. **The annotation.** A blockquoted banner at the top of the superseded spec's body, naming the superseding spec and what no longer holds, closing with the record that the spec stays as the account of what shipped.

The key is bookkeeping. **The annotation is what does the work**: it is what stops a reader — human or agent — mistaking a countered spec for a live one.

**The blockquote is load-bearing, not cosmetic.** `derive-dependencies` exempts blockquote-prefixed lines from harvesting, so the banner may link the superseding spec without the annotated spec acquiring a dependency on its own successor. A **criterion-level** annotation has no such exemption — it is a plain list item — so it cites the superseding spec **by name** and points at the banner that carries the link. The same rule is why the superseding spec's body carries no markdown link to the spec it supersedes: the pointer is frontmatter, and a body link would make the superseding spec declare a dependency on the very spec it counters, silently, on the first commit through the pre-commit hook.

**The frame is generated; the substance is authored.** The primitive contributes placement, the blockquote wrapper, the citation, and the closer. The author contributes the one sentence a reader needs — what stopped being true — phrased as a non-claim ("no longer exists", "is removed") rather than as an assertion whose truth depends on whether its paths resolve.

**The annotation is a mechanical edit.** It changes no claim the annotated spec makes about what it delivered, so it takes no back-edge and the spec keeps whatever status it holds, at any lifecycle state. That is what makes it cheap enough to apply without an operator intervening in the annotated spec's lifecycle.

**A second annotation accumulates; it does not replace.** Annotations from *different* superseding specs stack newest-first, because replacing one would destroy the record of the intermediate state the annotation exists to preserve. Re-declaring the *same* supersession is not accumulation — it is a re-run, and it writes nothing.

**The edge is descriptive.** It names a completed fact, so it gates no transition and appears on no backlog surface — unlike `folds-into:`, which blocks precisely because it names work not yet done. What verifies it is the `supersession-reciprocity` check surfaced through `/ductus:analyze`, which asks only whether the named spec's body names the superseding spec back. Its coverage is bounded to **declared** edges: a corpus of hand-written annotations carrying no key is invisible to it, and it reports that bound rather than presenting such a corpus as clean.

### Declaring by hand

1. **Resolve both specs.** The superseded one from `$ARGUMENTS` or the session target, the superseding one from `--by` — no session fallback for the second, and no self-reference. Both directories must exist; this is the one moment resolvability can be enforced, which is why the frontmatter schema validates `supersedes:` for shape only.
2. **Read both** before writing either. The superseded spec's status decides whether the non-`done` guidance applies; its body and the superseding spec's frontmatter decide whether either half is already applied.
3. **Confirm once**, naming both writes and any already-applied half.
4. **Append to `supersedes:`** on the superseding spec — creating the key when absent, and leaving it untouched when it already names the target.
5. **Write the annotation** at the top of the superseded spec's body, above any annotation already there. Blockquote every line of it. Change nothing else in the file — the frontmatter, and `status` in particular, is left byte-identical.
6. **Commit both specs together.** The reciprocity check reads the key against the annotated body; a commit carrying one half reports a finding that the other half was about to answer.

### When consolidation is the right answer instead

Supersession keeps the earlier spec because it shipped and its record is worth keeping. When the earlier spec is redundant rather than countered — it overlapped a sibling from the start, or was never distinct — the operation is `/ductus:consolidate <spec> --into <spec>`, which re-points every inbound pointer at the target and removes the source directory. They are opposite operations: one annotates and keeps, the other merges and removes. An operator who learns only one will use it for both, which is why each names the other.
