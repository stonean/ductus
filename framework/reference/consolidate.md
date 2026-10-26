# consolidate — markdown-only reference

With no ductus runtime registered, the host performs the same walk and the same writes with its own file tools (Read, Edit, Write) — no shell-pipeline substitution — one contract, two paths (§runtime-host-integration).

**Where a named section lives.** This file is read alongside its command, `consolidate.md`, which is already loaded whenever this one is. A section named below that has no heading in this file is in the command file, along with the numbered Instructions steps this file cites. A bare `§name` is a heading in this file or a constitution marker; a `§name` with another file's path on the same line is a heading in that file.

## Deciding whether to consolidate

Read it as one question: *does the source spec still describe something true?*

- **No** — it overlapped a sibling from the start, was never distinct, or a later spec absorbed what it covered. Consolidate it.
- **Yes** — some of what it describes still holds. Do not consolidate: reopen it `done → in-progress` and edit the part that stopped being true, which leaves one accurate description where removing it would leave none and re-point every pointer onto a spec that never made those claims.

A spec that is not `done` is the common case for consolidation: it delivered nothing, so there is nothing in it that could still be true.

## Enumerating what is destroyed

List the source directory's contents before confirming, and name **each scenario separately**. The confirmation is the operator's only look at what they are losing, and a general claim about "the spec's content" is not one — a scenario carries its own behavior, its own edge cases, and its own open-question gate, none of which survive and none of which migrate.

Everything else in the directory goes too: `plan.md`, `tasks.md`, `review.md`, `analysis.md`, `data-model.md`, research notes. None of it is copied anywhere. The target's own artifacts are not touched at all — this command writes nothing into the target beyond what a re-pointed inbound link does to third-party specs.

## Re-pointing and removing

1. **Re-point inbound body links** from the source directory to the target, across the spec root, matching whole path segments only. Leave `dependencies:` and `references:` alone — they are derived from body links, and the pre-commit generators rebuild them from the corrected bodies on the next commit.
2. **Remove the source directory**, but only when the target holds a `spec.md`. That check is what stops a removal from stranding content nothing else holds, and it runs at removal time regardless of having already run during the rewrite.
3. **A sequential `NNN-slug` directory is removable only here**, through an explicit opt-in that this command passes and `/{project}:fold` does not. The sequential form is otherwise permanent — a spec in it is completed, not retired — and the gate is what keeps an irreversible operation out of reach of a typo.
4. **An already-absent directory is not a failure.** It means a previous run finished, so a re-run converges instead of halting.
5. **Clear the session when it named the source.** The directory is gone, and §concurrent-features forbids leaving a target pointing at one that is: every follow-on command resolves it and fails, one step removed from the command that broke it. Clear rather than re-target — that rule states the distinction, and this command sits on the clear side because its target is a spec that already existed and that the operator may have no interest in. Preserve the per-contributor `cli-config-dir`. A session naming any other feature is left alone.
