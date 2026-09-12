---
section: "Follow-on scenarios"
---

# The-inbox-row

## Context

Nothing reports how deep the inbox is. `dashboard` does not read it, `/{project}:status` does not mention it, `check-review-gate` does not consult it, and no `scripts/audit/` family touches it. The two surfaces that show it at all are both **window-scoped**: `/{project}:review`'s **Captured issues** section lists inbox lines added since its `diff-base`, and `/{project}:implement`'s completion summary lists `diff-cross-spec`'s `inbox-additions`, computed from the feature's first commit. Both answer *"what was captured while this feature was open"*. Neither answers *"what is outstanding"*, so an item older than the current feature is invisible by construction.

[§brownfield-inbox](../../../framework/constitution.md#brownfield-inbox)'s **Surface at completion** is the root, not the implementations: it requires that *"issues captured during a unit of work are presented back to the user when that work completes"* — window-scoped in its own wording. The bullet's stated purpose is broader than its scope: *"the framework does not rely on the agent remembering a mid-task finding, it makes every capture visible at the next gate."* A capture from three features ago is not visible at this gate, and remembering it is exactly what the sentence disclaims.

The `captured` line carries a second defect independent of scope: it *"is omitted when no issues were appended to the inbox in the review window."* An omitted line cannot be told apart from a line that was never computed, which is `QUAL-CLAIM-001` on the report's own surface — every other section of `review.md` renders `*None.*` rather than disappearing, and this one is the exception.

Observed 2026-09-11: six items stood in the inbox while `ductus-v0.47.0` was cut. They appeared in 055's reports only because all six happened to be added inside 055's window — a coincidence of timing, not the mechanism working.

**A bare count in `/{project}:status` was considered and rejected.** It is a number the operator must choose to go and look at, so it fails on the principle it was meant to serve: [§design-principles](../../../framework/constitution.md#design-principles) rejects a feature that depends on someone remembering, and remembering to run a status command before closing work is that dependency wearing a different hat. It also flattens the distinction that actually mattered — six fresh items and six where one has sat since May read identically.

The right shape already exists in this repository. `/{project}:review`'s **analyze row** renders on *every* run in one of four states, names the command the operator owes, and is explicitly *"a notice, not a gate"*. Its rationale is this problem verbatim: *"the only thing carrying an operator from a passing review to the second gate was memory: the diligence dependency §design-principles rejects."* The inbox needs that row, not a number in a command nobody is required to run.

## Behavior

**§brownfield-inbox is corrected first**, because the implementations follow its scope. **Surface at completion** requires that the **standing backlog** be surfaced at completion, not only the captures from the current unit of work. The window keeps its own role — it is what ties a finding to the work that produced it — so the requirement names both, and the bullet's promise about not relying on memory becomes true of the whole queue rather than the most recent slice of it.

**The `captured` line becomes a computed row** on `/{project}:review` and the `/{project}:implement` completion summary, modelled on the analyze row and rendered on every run:

```text
  inbox       6 items outstanding, oldest 2026-05-19 — run /ductus:groom to route
  inbox       ✓ clean
```

Three properties, each fixing one of the defects above:

- **Standing, with the window alongside.** The row states the outstanding total; the existing Captured issues section keeps listing this window's additions. Two numbers answering two questions, neither standing in for the other.
- **Never omitted.** A clean inbox renders a clean row. Examined-and-empty and not-computed stop being the same output, which is the rule every other section of the report already follows.
- **Oldest-item age.** Derived from `git blame` over `{specs-root}/inbox.md` — one call, content-based, so the atomic rewrites `append-inbox` and `remove-inbox-item` perform do not reset a surviving line's date. Age is what separates a working queue from a rotting one, and it requires no authored state: nothing is added to the file and no author has to remember anything.

**It is a notice, not a gate**, for the same reason the analyze row is. Gating `done` or a release on inbox depth would make capture expensive, and §brownfield-inbox's whole design rests on capture being free — *"the honest choice between a growing backlog and a silent one would push toward silence."* The row changes what the operator knows at the moment they decide, and withholds nothing.

## Edge Cases

- **No inbox file.** The row says so rather than reporting zero: a project with no `inbox.md` has not been examined-and-found-clean, and the two must not render alike.
- **An inbox with only the template's comment block.** Zero items, clean row. The shared inbox grammar the primitives already use ignores lines inside `<!-- … -->`, so the count uses it rather than a second parser.
- **`git blame` unavailable** — a shallow clone, or a file not yet committed. The count still renders and the age reports as undeterminable rather than being silently dropped or defaulted to today. This is a check that reads git history, so any CI job running it needs `fetch-depth: 0`, per §design-principles and the entry `AGENTS.md` §Workflow already carries.
- **An item added and removed inside one window.** It is absent from the standing count, correctly — it is not outstanding — and the Captured issues section still shows it as the window's activity with its disposition, which is the reconciliation that section already requires.
- **The oldest item is deliberately long-lived.** There is no such state: §brownfield-inbox gives the inbox no hold state, and an item blocked on an external event is discarded rather than held. An old item is therefore evidence the queue needs a `/{project}:groom` pass, which is what the row says.
- **A project that never uses the inbox.** Renders a clean row on every run. That is the cost of the rule, and it is the right direction: a one-line clean notice is what makes a non-clean one legible.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
