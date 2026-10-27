---
section: "Follow-on scenarios"
---

# A-status-commit-holds-only-the-transition

## Context

On 2026-09-27, `017-derive-dont-ask` and `036-quality-cross-rules` were reopened for `008-security-rules`' scenario `a-statement-carries-one-obligation-keyword`. The status flip was meant to be committed ahead of the body edits it opened, following [§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle)'s advice to commit a status transition as its own step. Both `spec.md` files already held uncommitted signposts, so the flip was staged alone, by writing an index entry built from `HEAD`'s copy of each file.

The pre-commit hook then ran `git add` on every staged `spec.md` from the working tree, after `label-criteria`. Both `.githooks/pre-commit` and the shipped `framework/bootstrap/hooks/ductus-pre-commit` do this. So the commit carried the signposts under the transition's message, and the status stayed `done`, because the flip existed only in the index entry the hook replaced. Nothing errored, and the commit's own diff was the only signal. It was caught by reading that diff and undone before any push.

The advice is correct but incomplete: it says to commit the transition separately without saying what "separately" has to mean on disk. The hook that makes the difference ships to every adopter, so the gap belongs to every project, not only to this repository ([§drift-prevention](../../../framework/constitution.md#drift-prevention), *Shared knowledge stays in git*).

## Behavior

§spec-lifecycle's bullet on restoring a spec directory, which advises committing a status transition as its own step, states the condition that advice depends on: when the commit is made, the working copy of the spec file holds the transition and nothing else. The pre-commit hook restages each staged spec file whole from the working tree, so a transition staged alone, by an index edit or by hunk staging, commits whatever else the working file holds under the transition's message. It also loses the transition itself when the index was the only place it existed. To separate a transition from body edits already on disk, set the edits aside, commit the transition, then restore them, or make the transition first.

The bullet states the reason with the rule, as the bullet it extends does: the hook's restaging is what makes an apparently separate commit carry more than its message says.

## Edge Cases

- **The spec file holds no other uncommitted edit.** There is nothing to separate, and staging the file commits exactly the transition.
- **Edits in another file of the same spec.** They are unaffected. The hook restages only files already staged, so a data model or scenario left unstaged stays out of the commit.
- **Detecting it after the fact.** The only signal is the commit's own diff. Read it against the message before recording anything that depends on the commit, which [§grounding](../../../framework/constitution.md#grounding) already requires of any commit a claim rests on.
- **Setting edits aside is a restore.** Parking the body edits and restoring the file is the operation the same bullet warns about. The parked copy carries the pre-transition status, so re-apply the transition, and any ticked checkbox, when putting it back.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
