---
description: Merge a spec into another and remove its directory, re-pointing every inbound pointer first.
argument-hint: "<feature> --into <feature>"
---

# Consolidate

Remove a spec directory whose content belongs with another, after re-pointing every pointer that named it.

## Purpose

**This is the only command that removes a durable artifact.** `/ductus:prune` destroys `tasks.md`, which the framework classes as ephemeral work-tracking; `spec.md` is a durable source of truth, and consolidation deletes it along with everything else in its directory. Recovery is git history and nothing else.

It exists because the alternative is worse. A corpus of small overlapping specs accumulates ones whose content was never a separate concern, and an operator's only current recourse is `rm -rf` — no pointer rewriting, no anti-stranding refusal, no confirmation naming what is lost. Safety comes from the guards, not from withholding the command.

The question this command answers is whether the source spec still describes something true. A spec whose content was never a separate concern from a sibling, or that a later spec absorbed outright, is consolidated. A spec that still describes live behavior is not — it is edited in place instead, through the `done → in-progress` back-edge, which leaves one true description where consolidation would leave none.

It is a **cleanup command**, in the family of [041 — Task pruning](../../specs/041-task-pruning/spec.md): operator-initiated, confirmed, and not a pipeline state transition.

## Context

Parse `$ARGUMENTS` for the **source** feature — the spec being removed — and resolve it through `resolve-feature` (`ambiguous` and `not-found` are domain outcomes to surface, not errors to swallow). There is no session-target fallback: the argument to a deletion is named explicitly or the run stops.

The **target** comes from `--into` and is resolved the same way. It must be a directory holding a `spec.md`; a target that is not a home is where content would be stranded, and the primitives refuse it rather than trusting this command to have checked.

## Consolidation performs no content migration

None of what `/ductus:fold` does happens here — no body edit, no scenario creation, no task append, no status change, no review invalidation. Fold moves content because a branch-scoped spec was never meant to stand alone; consolidation assumes the merge already happened, or that nothing needed merging. **The guard proves the target exists; it never proves anything landed there.** That is why the confirmation must name content loss rather than directory removal.

**Nor does it compare the two specs.** Whether the target actually covers what the source claimed is a real question and a deliberate non-goal: it is semantic judgment over two documents, which is what the operator's agent is for, and building it as a step here would put a slow comparison in front of every consolidation to serve the minority that wants it. An operator who wants the check asks for it in the same conversation, before confirming at step 3.

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
- Do NOT read or modify source code or test files — consolidation merges specifications, not implementations.
- Reference: §spec-lifecycle, §numbering, §cross-spec-impact, §text-first-artifacts, §pipeline-boundaries, §spec-phase (spec-root resolution) (constitution loaded by `/ductus:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write) per the Markdown-only reference below.

**An interrupted consolidation is completed by re-running it.** The runtime provides no transaction spanning the rewrite and the removal — the condition `/ductus:fold` documents for the same pair of steps — so the recovery is a second run, and each write is built so a re-run is a no-op where the first attempt landed:

| Step | What makes a re-run safe |
| --- | --- |
| 4 — `rewrite-spec-links` | Idempotent by construction: once re-pointed, no link names the source, so a second pass rewrites nothing. It refuses an absent target *before* writing, which is what keeps that idempotence from becoming a trap — a rewrite that ran against nothing could not be undone by re-running. |
| 5 — `retire-feature` | Already-absent is `retired: false`, a domain outcome rather than a failure: the previous run finished, and this one converges. |

An interruption *before* the rewrite leaves the corpus exactly as it was. One *between* the two leaves pointers re-pointed and the directory present, which the second run finishes.

**On `ductus exec` step 2 does not run**, because it dispatches no primitive: nothing enumerates what the removal destroys. The confirmation still fires and still requires an answer, so a human is in the loop — but the prompt they answer will not name the scenarios being destroyed, which is most of what the confirmation exists to tell them. Treat the exec path as unsuitable for consolidating a spec anything else points at, and prefer the interactive walk; the reduction is documented rather than silent, per §runtime-host-integration's two-paths guarantee.

1. Invoke `read-spec` (with `include-body`) against both specs — the source for its status and its body, and the target to establish that it holds a readable `spec.md`. A target that does not is the refusal both primitives enforce; report it and stop before anything is examined further, since there is no home for the content to have landed in.

<!-- audit:ignore-promotion -->
2. Enumerate what the removal destroys (host responsibility; a directory walk, not a decision). List the source directory's contents by name — **each scenario individually**, then `plan.md`, `tasks.md`, `review.md`, `data-model.md`, and any other artifact present. The scenarios are named one by one rather than summarized: they are destroyed with the directory and migrated nowhere, and "the spec is removed" does not make an operator picture them. This list is what the confirmation in step 3 carries.

3. Invoke `gate-confirm` with a `gate` name (e.g. `consolidate-remove`) and a `prompt` that names **content loss**, not merely the removal of a directory: this command migrates nothing, so everything step 2 enumerated is destroyed — the scenarios by name among it. Name the source and target, and that recovery is git history alone. No write happens before this step; denial ends the run cleanly with nothing written and nothing removed.

4. Invoke `rewrite-spec-links` with `from` set to the source feature and `to` set to the target feature, re-pointing every inbound body link across the spec root — the same primitive `/ductus:fold` uses, unchanged, because its own contract already covers a retiring **or renamed** directory. Matching is by whole path segment, so a directory whose name merely shares a prefix is untouched, and a cross-service URL naming another repository's spec is never rewritten. The result's `examined` count is what bounds an empty `rewritten` as *nothing pointed here* rather than *nothing was checked*. Frontmatter indexes are left alone by design: they are derived from body links and regenerate on the next commit.

5. Invoke `retire-feature` with the source feature, the target as its fold target, and the **explicit opt-in** that permits removing a sequential directory. The refusal on the sequential form is gated here, never removed: `/ductus:fold` does not pass the opt-in, so a mistyped feature name during a fold still meets it unchanged, and the flag is not a weaker guard but the record that a second, explicit decision was made. The anti-stranding refusal — the target must hold a `spec.md` — is untouched and applies to both callers, and it is re-checked here even though step 4 established the same fact, because the primitive is callable on its own and the guard on the one irreversible step must not rest on a caller's promise.

<!-- audit:ignore-promotion -->
6. Clear the session target when it named the source (host responsibility; conditional, so nothing dispatches when it did not). The source directory no longer exists, and §concurrent-features forbids leaving the session pointing at it: a target naming a directory that is gone is a dangling pointer in a file the framework owns, and every follow-on command resolves it and fails one step removed from the command that broke it. Clear it with write-session's clear mode, which removes the target block while preserving the per-contributor `cli-config-dir` so the agent identity survives. **Clear rather than re-target**, and the reason is stated in §concurrent-features: `/ductus:fold` re-targets because its content moved to a spec that continues the work, while this command's target is a spec that already existed and that the operator may have no interest in — they were removing something, not adopting it. Re-targeting there would assert an intent nobody stated. A session naming any other feature is not this removal's business and is left untouched.

<!-- audit:ignore-promotion -->
7. Report the consolidation (host responsibility): the source removed, the target it was consolidated into, and the re-pointing counts step 4 returned. Say whether the session was cleared, so the operator knows they now have no target rather than discovering it from the next command. Recommend committing before anything else: `dependencies:` and `references:` across the corpus are regenerated by the pre-commit hook from the rewritten body links, so the first commit after a consolidation is what makes those indexes correct.

## Markdown-only reference

With no ductus runtime registered, the host performs the same walk and the same writes with its own file tools (Read, Edit, Write) — no shell-pipeline substitution — one contract, two paths (§runtime-host-integration).

### Deciding whether to consolidate

Read it as one question: *does the source spec still describe something true?*

- **No** — it overlapped a sibling from the start, was never distinct, or a later spec absorbed what it covered. Consolidate it.
- **Yes** — some of what it describes still holds. Do not consolidate: reopen it `done → in-progress` and edit the part that stopped being true, which leaves one accurate description where removing it would leave none and re-point every pointer onto a spec that never made those claims.

A spec that is not `done` is the common case for consolidation: it delivered nothing, so there is nothing in it that could still be true.

### Enumerating what is destroyed

List the source directory's contents before confirming, and name **each scenario separately**. The confirmation is the operator's only look at what they are losing, and a general claim about "the spec's content" is not one — a scenario carries its own behavior, its own edge cases, and its own open-question gate, none of which survive and none of which migrate.

Everything else in the directory goes too: `plan.md`, `tasks.md`, `review.md`, `data-model.md`, research notes. None of it is copied anywhere. The target's own artifacts are not touched at all — this command writes nothing into the target beyond what a re-pointed inbound link does to third-party specs.

### Re-pointing and removing

1. **Re-point inbound body links** from the source directory to the target, across the spec root, matching whole path segments only. Leave `dependencies:` and `references:` alone — they are derived from body links, and the pre-commit generators rebuild them from the corrected bodies on the next commit.
2. **Remove the source directory**, but only when the target holds a `spec.md`. That check is what stops a removal from stranding content nothing else holds, and it runs at removal time regardless of having already run during the rewrite.
3. **A sequential `NNN-slug` directory is removable only here**, through an explicit opt-in that this command passes and `/ductus:fold` does not. The sequential form is otherwise permanent — a spec in it is completed, not retired — and the gate is what keeps an irreversible operation out of reach of a typo.
4. **An already-absent directory is not a failure.** It means a previous run finished, so a re-run converges instead of halting.
5. **Clear the session when it named the source.** The directory is gone, and §concurrent-features forbids leaving a target pointing at one that is: every follow-on command resolves it and fails, one step removed from the command that broke it. Clear rather than re-target — that rule states the distinction, and this command sits on the clear side because its target is a spec that already existed and that the operator may have no interest in. Preserve the per-contributor `cli-config-dir`. A session naming any other feature is left alone.
