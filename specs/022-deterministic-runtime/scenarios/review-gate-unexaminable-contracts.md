---
section: "Follow-on scenarios"
---

# Review-gate-unexaminable-contracts

## Context

[review-staleness-gate](review-staleness-gate.md) gave `check-review-gate` a fourth check: a spec cannot reach `done` while a durable contract has changed since its recorded `reviewed-against`. Two defects survive it, and they compound.

**The prose never learned about the fourth check.** `framework/commands/implement.md` still enumerates three. Step 14 lists "the feature directory's markdown lint … then unresolved scenario open questions, then the spec frontmatter `review:` block", and the completion gate's step 5 — which that step names as the markdown-only path — opens with "the primitive, which evaluates **all three checks** below in this order". `ReviewStale` appears in neither, so the two paths enforce different gates: the deterministic path blocks on a stale review, and an adopter walking the documented prose because no runtime is registered does not. That is the §runtime-host-integration two-paths guarantee broken in the direction that matters — the fallback is the weaker one, and it is weaker silently, since the prose asserts a count that was correct before the check was added and reads as complete now.

**Neither path can see an uncommitted contract.** `stale_review_block` resolves `reviewed-against` to a commit and calls `diff_tree_to_tree(base_tree, head_tree)`; Family 19 runs `git diff --name-only {base}..HEAD`. Both compare committed trees, so a durable contract that exists only in the working tree is invisible to them. At the moment `/ductus:implement` proposes the transition this is the normal state, not an edge case: the scenario was written minutes ago and the operator has not committed yet, so `reviewed-against` is HEAD, the diff is empty, and the gate reports clean. The staleness the gate exists to prevent becomes *real* on the next commit — after the gate, and after `done`.

Observed 2026-08-27 on `023-govern-refinement`. Its gate passed on every check, `done` was set, and the first CI push failed Family 19 against `scenarios/configure-permission-pattern-safety.md` — a contract that had been sitting uncommitted in the working tree throughout the gate. The review then had to be re-run and re-committed to bind it to the commit that carried the work. Nothing local could have reported it: Family 19 skips specs that are not yet `done`, so it does not evaluate the spec until after the transition it would have warned about, and while the pre-commit hook does run the generators and the runtime's `fmt` / `clippy` / `test` checks, none of them looks at review freshness.

This is `QUAL-CLAIM-001` applied to the gate itself. `passed: true` is emitted both when the gate examined the durable contracts and found them current, and when it could not examine them at all — and the caller cannot tell which. The primitive already argues this position against itself: `stale_review_block`'s comment records that an abbreviated `reviewed-against` once "made the whole check fail open with no signal", and calls a check that silently cannot run "the failure mode this repo pays for most". The uncommitted-contract case is that same failure, reached through a different door.

## Behavior

- `framework/commands/implement.md` enumerates **four** checks wherever it enumerates the gate — step 14's inline list, its `blocked_by` list, and the completion gate's step 5. The "all three checks" count is corrected, and `ReviewStale` is described with its blocked message in the same shape as the other three, so the markdown-only path enforces the gate the primitive enforces. The generated `.claude/commands/ductus/implement.md` follows from the source rewrite via the command generator.
- `check-review-gate` reports what it could not examine. When any durable contract under the feature (`scenarios/*.md`, `data-model.md`) has uncommitted changes in the working tree — modified, staged, or untracked — the result carries a `guidance` string naming those paths and stating that staleness could not be determined against them.
- **The gate does not block on this.** `passed` is unchanged, and the operator may still take the spec to `done`; committing before reviewing is a workflow choice, not an error. What changes is that the clean verdict stops being silent about its own blind spot — the caller is told the check ran against committed state only, and which contracts were outside it.
- `/ductus:implement` surfaces that guidance in the completion-gate summary it presents before asking for the transition, so the operator sees it at the moment the decision is theirs rather than in CI afterward.
- The guidance is absent when every durable contract is committed — the common case stays quiet, and its silence then genuinely means "examined and current".

## Edge Cases

- A feature with no scenarios and no `data-model.md` has no durable contract to be uncommitted; no guidance is emitted, matching `stale_review_block`'s existing exemption for the same shape.
- A durable contract that is uncommitted **and** whose committed version already changed since `reviewed-against` is a genuine `ReviewStale` block. The block wins — guidance is not a softer substitute for a check that actually fired.
- An untracked scenario counts. It is the exact shape of the observed failure: a file `create-scenario` wrote during this session, present on disk and absent from every tree the diff consults.
- A dirty file under the feature directory that is not a durable contract (`tasks.md`, `plan.md`, `review.md`, `spec.md`) is not reported. The gate's staleness scoping is deliberate and this guidance inherits it, or the notice fires on nearly every run and is learned-ignored.
- The markdown-only path performs the same detection with the host's own tooling and reports the same guidance; a path that silently skipped it would re-create, one level down, the divergence this scenario closes.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
