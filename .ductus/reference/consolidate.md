# consolidate — reference

**Where a named section lives.** A section named below with no heading here is in the command file, `consolidate.md`, with the numbered steps this file cites. A bare `§name` is a heading here or a constitution marker; one with another file's path on its line is a heading there, unless it is a constitution marker.

## Deciding whether to consolidate

Read it as one question: *does the source spec still describe something true?*

- **No** — it overlapped a sibling from the start, was never distinct, or a later spec absorbed what it covered. Consolidate it.
- **Yes** — some of what it describes still holds. Do not consolidate: edit the part that stopped being true, reopening it `done → in-progress` when `classify-spec-edit` says the edit does, which leaves one accurate description where removing it would leave none and re-point every pointer onto a spec that never made those claims.

A spec that is not `done` is the common case for consolidation: it delivered nothing, so there is nothing in it that could still be true.

## Enumerating what is destroyed

List the source directory's contents before confirming, and name **each scenario separately**, saying for each whether the target holds a scenario of the same file name — evidence of a copy or a move by name, not of its content, so the scenario is still named as destroyed. The confirmation is the operator's only look at what they are losing, and a general claim about "the spec's content" is not one — a scenario carries its own behavior, its own edge cases, and its own open-question gate, none of which survive and none of which migrate.

Everything else in the directory goes too: `plan.md`, `tasks.md`, `review.md`, `analysis.md`, `data-model.md`, research notes. None of it is copied anywhere. Into the target this command writes only the links its own files held to the source, re-pointed relative to the file holding them when they climb to the spec root; nothing else of the target's is touched.

## Re-pointing and removing

1. **Re-point inbound pointers** from the source directory to the target: body links across the spec root, matching whole path segments only, and the references in the files `check-orphaned-references` reads — the spec root's `system.md` and the root files `CLAUDE.md`, `AGENTS.md`, `README.md` and `.githooks/pre-commit` — by the rule §Re-pointing inbound pointers states in `.ductus/reference/fold.md`, which writes a link inside the target that climbs to the spec root relative to its own file and lists the lines that still name the source. Leave `dependencies:` and `references:` alone — they are derived from body links, and the pre-commit generators rebuild them from the corrected bodies on the next commit.
2. **Remove the source directory**, but only when the target holds a `spec.md`. That check is what stops a removal from stranding content nothing else holds, and it runs at removal time regardless of having already run during the rewrite.
3. **A sequential `NNN-slug` directory is removable only here**, through an explicit opt-in that this command passes and `/ductus:fold` does not. The sequential form is otherwise permanent — a spec in it is completed, not retired — and the gate is what keeps an irreversible operation out of reach of a typo.
4. **An already-absent directory is not a failure.** It means a previous run finished, so a re-run converges instead of halting.
5. **Clear the session when it named the source.** The directory is gone, and §concurrent-features forbids leaving a target pointing at one that is: every follow-on command resolves it and fails, one step removed from the command that broke it. Clear rather than re-target — that rule states the distinction, and this command sits on the clear side because its target is a spec that already existed and that the operator may have no interest in. Preserve the per-contributor `cli-config-dir`. A session naming any other feature is left alone.
