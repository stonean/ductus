---
description: Merge a spec into another and remove its directory, re-pointing every inbound pointer first.
argument-hint: "<feature> --into <feature>"
---

# Consolidate

Remove a spec directory whose content belongs with another, after re-pointing every pointer that named it.

## Purpose

**This is the only command that removes a durable artifact.** `/ductus:prune` destroys `tasks.md`, which the framework classes as ephemeral work-tracking; `spec.md` is a durable source of truth, and consolidation deletes it along with everything else in its directory. Recovery is git history and nothing else.

It exists because the alternative is worse. A corpus of small overlapping specs accumulates ones whose content was never a separate concern, and an operator's only current recourse is `rm -rf` — no pointer rewriting, no anti-stranding refusal, no confirmation naming what is lost. Safety comes from the guards, not from withholding the command.

Consolidation is the opposite of supersession, and the two must not be collapsed:

| | Content | Source directory | Prior standing |
| --- | --- | --- | --- |
| `/ductus:supersede` | stays, annotated | survives | shipped and delivered |
| `/ductus:consolidate` | merges into target | removed | redundant or never distinct |

A spec that shipped and was later countered is the historical record of what shipped; folding it into the spec that removed it would invert the relationship. A spec that overlapped a sibling from the start was never a separate concern and should not stay one. If the earlier spec delivered something, the command is `/ductus:supersede`.

It is a **cleanup command**, in the family of [041 — Task pruning](../../specs/041-task-pruning/spec.md): operator-initiated, confirmed, and not a pipeline state transition.

## Context

Parse `$ARGUMENTS` for the **source** feature — the spec being removed — and resolve it through `resolve-feature` (`ambiguous` and `not-found` are domain outcomes to surface, not errors to swallow). There is no session-target fallback: the argument to a deletion is named explicitly or the run stops.

The **target** comes from `--into` and is resolved the same way. It must be a directory holding a `spec.md`; a target that is not a home is where content would be stranded, and the primitives refuse it rather than trusting this command to have checked.

## Consolidation performs no content migration

None of what `/ductus:fold` does happens here — no body edit, no scenario creation, no task append, no status change, no review invalidation. Fold moves content because a branch-scoped spec was never meant to stand alone; consolidation assumes the merge already happened, or that nothing needed merging. **The guard proves the target exists; it never proves anything landed there.** That is why the confirmation must name content loss rather than directory removal.

**Nor does it compare the two specs.** Whether the target actually covers what the source claimed is a real question and a deliberate non-goal: it is semantic judgment over two documents, which is what the operator's agent is for, and building it as a step here would put a slow comparison in front of every consolidation to serve the minority that wants it. An operator who wants the check asks for it in the same conversation, before confirming at step 4.

**The source spec's scenarios go with its directory.** They are destroyed, not migrated — deliberately unlike fold-back, which creates one scenario under the upstream spec for every scenario the retiring spec carried. A scenario is a distinct artifact with its own open-question gate, and an operator who reads "the spec is removed" does not necessarily picture them, so the confirmation names them one by one rather than folding them into a general claim about content.

Adding `--into` to `/ductus:fold` instead was considered and rejected: fold's enumeration step, its post-merge instruction, and its single-source rule for the fold target are all specific to the branch-scoped staging form, and carrying a sequential path through them would qualify seven load-bearing statements.

### Flags

| Flag | Behavior |
| --- | --- |
| `--into <feature>` | The spec the source's content belongs with — the **target**. Required. It must hold a `spec.md`, which is the anti-stranding condition both primitives enforce |

## Scope Boundaries

- This command re-points inbound pointers across the spec root and removes the **source** feature directory. It writes **nothing** to the target spec — not its body, its scenarios, its `tasks.md`, its `status`, or its `review:` block.
- Removal is irreversible and writes no backup: recovery is git history, exactly as for `/ductus:prune`.
- `dependencies:` and `references:` are never hand-edited here. They are derived from body links, and the pre-commit generators regenerate them from the rewritten bodies on the next commit.
- A `supersedes:` edge is **never re-pointed without the operator settling it**, and the presence of one is never on its own a reason to refuse.
- Do NOT read or modify source code or test files — consolidation merges specifications, not implementations.
- Reference: §spec-lifecycle, §numbering, §cross-spec-impact, §text-first-artifacts, §pipeline-boundaries, §spec-phase (spec-root resolution) (constitution loaded by `/ductus:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write) per the Markdown-only reference below.

**An interrupted consolidation is completed by re-running it.** The runtime provides no transaction spanning the rewrite and the removal — the condition `/ductus:fold` documents for the same pair of steps — so the recovery is a second run, and each write is built so a re-run is a no-op where the first attempt landed:

| Step | What makes a re-run safe |
| --- | --- |
| 5 — `rewrite-spec-links` | Idempotent by construction: once re-pointed, no link names the source, so a second pass rewrites nothing. It refuses an absent target *before* writing, which is what keeps that idempotence from becoming a trap — a rewrite that ran against nothing could not be undone by re-running. |
| 6 — `retire-feature` | Already-absent is `retired: false`, a domain outcome rather than a failure: the previous run finished, and this one converges. |

An interruption *before* the rewrite leaves the corpus exactly as it was. One *between* the two leaves pointers re-pointed and the directory present, which the second run finishes.

**On `ductus exec` steps 2 and 3 do not run**, because neither dispatches a primitive: nothing enumerates what the removal destroys, and no `supersedes:` edge is settled. The confirmation still fires and still requires an answer, so a human is in the loop — but the prompt they answer will not name the scenarios being destroyed or the declared edges that point at the source, which is most of what the confirmation exists to tell them. Treat the exec path as unsuitable for consolidating a spec anything else points at, and prefer the interactive walk; the reduction is documented rather than silent, per §runtime-host-integration's two-paths guarantee.

1. Invoke `read-spec` (with `include-body`) against both specs — the source for its status, its body, and its own `supersedes:` list, and the target to establish that it holds a readable `spec.md`. A target that does not is the refusal both primitives enforce; report it and stop before anything is examined further, since there is no home for the content to have landed in.

<!-- audit:ignore-promotion -->
2. Enumerate what the removal destroys (host responsibility; a directory walk, not a decision). List the source directory's contents by name — **each scenario individually**, then `plan.md`, `tasks.md`, `review.md`, `data-model.md`, and any other artifact present. The scenarios are named one by one rather than summarized: they are destroyed with the directory and migrated nowhere, and "the spec is removed" does not make an operator picture them. This list is what the confirmation in step 4 carries.

<!-- audit:ignore-promotion -->
3. Settle every `supersedes:` edge that touches the source, in both directions (host responsibility; the operator decides each one, and no primitive may decide it for them). Read the `supersedes:` key of every spec under the spec root:

   - **Inbound** — a spec whose `supersedes:` names the source. Name each one and offer three answers: **re-point** it at the target, **drop** the entry, or **cancel** the consolidation. Default to **none of them** — the operator answers, and an unanswered edge leaves the run stopped rather than silently resolved. A `supersedes:` edge is a claim about a relationship, not a location: re-pointing it would give the declaring spec a claim to supersede the target, an assertion nobody made and usually false. That is the hazard a rewrite must not guess at, which is why this is settled by hand while body links are re-pointed wholesale in step 5.
   - **Outbound** — the specs the source itself supersedes. Their annotations cite a spec that is about to disappear, so name them so the operator knows which banners will be left pointing at nothing.

   **The presence of a `supersedes:` edge is never on its own a reason to refuse.** Refusing would block exactly the tangled-corpus cleanup this command exists for.

4. Invoke `gate-confirm` with a `gate` name (e.g. `consolidate-remove`) and a `prompt` that names **content loss**, not merely the removal of a directory: this command migrates nothing, so everything step 2 enumerated is destroyed — the scenarios by name among it. Name the source and target, the `supersedes:` decisions settled in step 3, and that recovery is git history alone. No write happens before this step; denial ends the run cleanly with nothing written and nothing removed.

5. Invoke `rewrite-spec-links` with `from` set to the source feature and `to` set to the target feature, re-pointing every inbound body link across the spec root — the same primitive `/ductus:fold` uses, unchanged, because its own contract already covers a retiring **or renamed** directory. Matching is by whole path segment, so a directory whose name merely shares a prefix is untouched, and a cross-service URL naming another repository's spec is never rewritten. The result's `examined` count is what bounds an empty `rewritten` as *nothing pointed here* rather than *nothing was checked*. Frontmatter indexes are left alone by design; `supersedes:` was settled by hand in step 3 and is not this primitive's subject.

6. Invoke `retire-feature` with the source feature, the target as its fold target, and the **explicit opt-in** that permits removing a sequential directory. The refusal on the sequential form is gated here, never removed: `/ductus:fold` does not pass the opt-in, so a mistyped feature name during a fold still meets it unchanged, and the flag is not a weaker guard but the record that a second, explicit decision was made. The anti-stranding refusal — the target must hold a `spec.md` — is untouched and applies to both callers, and it is re-checked here even though step 5 established the same fact, because the primitive is callable on its own and the guard on the one irreversible step must not rest on a caller's promise.

<!-- audit:ignore-promotion -->
7. Report the consolidation (host responsibility): the source removed, the target it was consolidated into, the re-pointing counts step 5 returned, and each `supersedes:` edge as the operator settled it. When `.ductus/session.toml` still points at the removed directory, say so and direct the user to `/ductus:target` — this command does not re-target, because consolidating a spec asserts that its content belongs with the target, not that the operator's next piece of work does. Recommend committing before anything else: `dependencies:` and `references:` across the corpus are regenerated by the pre-commit hook from the rewritten body links, so the first commit after a consolidation is what makes those indexes correct.

## Markdown-only reference

With no ductus runtime registered, the host performs the same walk and the same writes with its own file tools (Read, Edit, Write) — no shell-pipeline substitution — one contract, two paths (§runtime-host-integration).

### Deciding between consolidation and supersession

The operator's real decision for any countered pair is which of two outcomes applies, and the table under **Purpose** is the whole of it. Read it as one question: *did the earlier spec deliver something?*

- **Yes** — it is the record of what shipped. Keep it, annotate it, and declare the relation with `/ductus:supersede`. Removing it would delete the account of a decision that was enacted and later countered.
- **No** — it overlapped a sibling from the start, or was never distinct. Consolidate it.

A spec that is not `done` is the common case for consolidation, and the reason is the same one that makes supersession wrong there: a `draft` or `clarified` spec delivered nothing to counter.

### Enumerating what is destroyed

List the source directory's contents before confirming, and name **each scenario separately**. The confirmation is the operator's only look at what they are losing, and a general claim about "the spec's content" is not one — a scenario carries its own behavior, its own edge cases, and its own open-question gate, none of which survive and none of which migrate.

Everything else in the directory goes too: `plan.md`, `tasks.md`, `review.md`, `data-model.md`, research notes. None of it is copied anywhere. The target's own artifacts are not touched at all — this command writes nothing into the target beyond what a re-pointed inbound link does to third-party specs.

### Settling `supersedes:` edges

A declared pointer cannot be ignored the way a derived index can, and it cannot be rewritten the way a body link can. Walk both directions before confirming:

- **Inbound** (`supersedes:` naming the source): offer **re-point**, **drop**, or **cancel**, defaulting to none. Re-pointing is a claim that the declaring spec supersedes the *target* — usually false, and never something to infer. A precise report an operator answers beats a rewrite that guessed wrong.
- **Outbound** (the source's own `supersedes:` list): those specs carry annotations citing a spec about to disappear. Nothing here repairs them automatically; naming them is what lets the operator decide whether to.

Never refuse on the presence of an edge alone. The corpus this command exists to clean up is exactly the one where such edges are dense.

### Re-pointing and removing

1. **Re-point inbound body links** from the source directory to the target, across the spec root, matching whole path segments only. Leave `dependencies:` and `references:` alone — they are derived from body links, and the pre-commit generators rebuild them from the corrected bodies on the next commit.
2. **Remove the source directory**, but only when the target holds a `spec.md`. That check is what stops a removal from stranding content nothing else holds, and it runs at removal time regardless of having already run during the rewrite.
3. **A sequential `NNN-slug` directory is removable only here**, through an explicit opt-in that this command passes and `/ductus:fold` does not. The sequential form is otherwise permanent — a spec in it is completed, not retired — and the gate is what keeps an irreversible operation out of reach of a typo.
4. **An already-absent directory is not a failure.** It means a previous run finished, so a re-run converges instead of halting.
