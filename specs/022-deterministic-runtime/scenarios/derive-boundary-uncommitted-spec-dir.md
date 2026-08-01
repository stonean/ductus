---
section: "Follow-on scenarios"
---

# Derive-boundary-uncommitted-spec-dir

## Context

`derive-boundary` computes the write boundary by diffing against the spec directory's *first commit*. On a spec whose directory is still untracked there is no such commit, so the primitive fails with the operational error `no commits found that touch specs/{feature}` — halting the `/gov:implement` walk at step 2, before any task runs.

Nothing upstream catches the gap. `/gov:specify`, `/gov:clarify`, and `/gov:plan` all complete and advance status against an uncommitted spec directory, so the pipeline reaches `planned` and only *then* discovers it cannot start. Observed 2026-07-30 on `046-scenario-open-question-visibility`, freshly created and planned in one session with no commits.

The fail-closed rule is correct and is **not** the defect. The defect is twofold: the precondition is discovered one command later than the gate that creates it, and it is reported as a crash rather than as a next-step.

## Behavior

**`/gov:plan`'s validation gate checks the spec directory is committed before advancing `clarified → planned`.** The precondition surfaces at the gate that creates it, as a domain outcome naming the fix (`commit specs/{feature} before planning`) rather than an operational error. This is the primary fix: per [§design-principles](../../framework/constitution.md#design-principles) "never depend on human diligence", the pipeline should not reach `planned` in a state where `/gov:implement` provably cannot start.

**`derive-boundary` returns an empty-boundary domain outcome instead of erroring.** With no commit touching the spec directory, the boundary is *unknowable*, not *broken* — so the primitive reports an empty boundary plus guidance ("commit the spec directory, or seed a `write-boundary` in the session"), matching how the rest of the runtime treats provable-vs-unknowable states. Enforcement stays fail-closed on the empty result: the first out-of-spec `writeCode` edit halts with `out-of-boundary-edit` and a legible next-step, rather than the walk dying at step 2.

The two fixes are complementary, not alternatives. The gate stops the state from being reached; the domain outcome makes it legible if it is reached another way (a hand-edited status, a markdown-only run, a spec dir committed and then reverted).

## Edge Cases

- **Partially committed spec directory** (`spec.md` committed, `tasks.md` untracked) — derivation succeeds as it does today and the gate passes. The gate asserts that at least one commit *touches* the spec directory, not that the working tree is clean; requiring a clean tree would block the ordinary edit-then-plan flow.
- **Session-seeded `write-boundary` with an uncommitted spec dir** — the empty derivation unions with the seed under the precedence [`writecode-boundary-derivation`](writecode-boundary-derivation.md) settled (seeded ∪ derived), so the seed admits the walk and `/gov:implement` proceeds. This is the escape hatch the guidance string names.
- **Markdown-only path** — the host derives the boundary itself per the prose, so the same guidance is what the prose instructs the host to report; the gate check is likewise a prose step in `/gov:plan`, not a runtime-only behavior.
- **Back-edge re-entry** — a spec returning `done → in-progress` already has a committed spec directory, so the gate is a no-op on every reopen path.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
