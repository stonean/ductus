# groom — markdown-only reference

With no ductus runtime registered, the host performs the same walk and the same writes with its own file tools (Read, Edit, Write) — no shell-pipeline substitution — one contract, two paths (§runtime-host-integration). The routing decision per item stays the same decision tree; the sections below carry the full detail both paths follow.

**Where a named section lives.** Read from `/{project}:groom`, this file has its command, `groom.md`, already loaded, and a section named below that has no heading in this file is in the command file, along with the numbered Instructions steps this file cites. The other commands that route through **§Groom decision tree** read that section without loading `groom.md`, so it is written to be self-contained: every section it names is a heading in this file or a constitution marker. A bare `§name` is a heading in this file or a marker in the constitution, `.ductus/constitution.md`; a `§name` with another file's path on the same line is a heading in that file, unless it is a constitution marker.

## Groom decision tree

For each item, walk the steps in order; the first matching step names the route.

`/{project}:review`, `/{project}:analyze`, and `/{project}:implement` walk this same tree to route a **finding** (§brownfield-inbox, Finding dispositions). Two branches read differently for a finding, because the inbox is not a disposition: where an item would be left in the inbox, a finding is routed to a new spec created through `/{project}:specify`'s procedure or discarded with its reason; and a finding against a rule file the project does not own — managed by `/ductus` and not pinned — is discarded with that reason, since the next update overwrites any amendment.

**Step 1: Is this a cross-cutting concern with no covering rule?** (route: `rule`)

- Apply the four-indicator promotion checklist (§rules in `.ductus/constitution.md`): cross-cutting, citable, governance-recognized category, generalizable wording. If the item qualifies, recommend promoting it to a rule.
- If a loaded rule file already covers the domain (e.g., `specs/rules/security-backend.md` for an authentication concern, `specs/rules/configuration-cross.md` for an env-var concern), recommend the user amend the relevant rule file directly — note that local edits to rule files are overwritten by `/ductus` unless the file is pinned in `.ductus/config.toml`, so amendments belong upstream in the framework rather than in adopting projects.
- If no rule file covers the domain, creating a new rule file is its own feature spec (out of `/{project}:groom`'s scope). Leave the item in the inbox unmodified — every subsequent groom pass walks every unmigrated item, including this one, so it stays surfaced. Ask the user whether to skip and continue.
- If the item is feature-specific rather than cross-cutting, fall through to Step 2.

**Step 2: Does a spec exist for this behavior?** (route: `spec`, no matched feature)

- Search `specs/` for a feature directory that covers this area.
- If no spec exists — recommend creating one via `/{project}:specify`. Ask the user whether to create the spec now or skip this item. (If the item is a chore rather than a feature gap — see Step 4 — it never needs a spec; do it in this pass and remove it.)

**Step 3: Is the spec ambiguous or incomplete?** (route: `spec`, matched feature)

- If the existing spec does not cover the reported behavior clearly — recommend updating the spec directly. Offer to help update the spec section. When the spec edit is confirmed, **set the session target** to this feature (see **Setting the session target**).

**Step 4: Spec is clear — durable requirement, or a chore?** (route: `scenario` or `chore`)

- The spec covers the area, so there is no rule, missing-spec, or spec-edit gap. Now decide what the item *is* (§bug-handling, durability test):
  - **A durable behavioral requirement** (route: `scenario`) — a new behavior, edge case, or contract the spec covers at a high level but does not yet describe in detail. Create a scenario inline under the matching spec's `scenarios/` directory using the `specs/templates/scenario.md` template, then append a task to the spec's `tasks.md` referencing the new scenario. **If the matched spec's status is `done`, reopen it `done → in-progress`** as part of the same routing action (see **Reopening a `done` spec**), so a follow-on `/{project}:implement` finds an actionable target instead of gate-failing on a `done` spec; a spec already in `draft`, `clarified`, `planned`, or `in-progress` is left unchanged. After the scenario and task are written and any reopen is applied, **set the session target** to this feature plus the new scenario (see **Setting the session target**).
  - **A chore** (route: `chore`) — project maintenance (lint or formatting cleanup, dependency cleanup, repo hygiene, a standalone refactor) that adds no durable requirement. It is **not** spec material: do not create a rule, spec, or scenario, and do not append it to any `tasks.md` — spec tasks are feature work derived from a plan, not standalone chores. Do it **in this pass** and remove the bullet — §bug-handling is explicit that a chore spawns no rule, spec or scenario and, once done, is removed because it is resolved, not carried. The project's lint/format/test tooling covers the common cases. Leave the item in place **only** when the fix proves blocked or turns out not to be mechanical, and say which; that exception is the whole of it, and a chore parked for any other reason is re-read and re-routed on every subsequent grooming pass while the inbox stops reflecting real backlog.

An item that is not actionable or already resolved takes the `discard` route: remove it from `specs/inbox.md` with no other write. An item **blocked on an external event** — one whose premise cannot be evaluated until something outside the project changes — counts as not actionable, so it is discarded rather than left in place carrying a hold note; the inbox has no hold state. An item that is merely *undecided* is actionable: it routes to a scenario under its covering spec, and the decision lives in that scenario's Open Questions.

## Inbox upkeep

A chore is fixed in the grooming pass rather than parked, and an item blocked on an external event is discarded rather than held, both per **Groom decision tree** above.

- **Remove an inbox item by matching its own text, never by position.** The removal primitive matches the bullet's text by equality, because line numbers computed up front shift the moment the first removal lands, and a positional batch deletes unrelated items while every call succeeds. Extract each item's line and strip its checkbox rather than retyping the text, assert the match is unique, and read the diff of removed bullets against what you intended. Never drive a by-position edit over any list that shrinks as you walk it.

## Setting the session target

When an item routes to an existing spec, set the session target to that spec as part of the routing, so the operator's next command (`/{project}:amend`, `/{project}:implement`) needs no manual `/{project}:target`:

- **Spec edit (Step 3)** — target the matched feature: write `feature` and `path` (the repo-relative `specs/{feature}` directory).
- **Scenario creation (Step 4)** — target the matched feature **plus the new scenario**: write `feature`, `path`, `scenario` (the new scenario slug), and `scenario-path` (`specs/{feature}/scenarios/{slug}.md`) — the same target shape `/{project}:amend`'s scenario route writes.
- **Rule items (Step 1), new-spec items (Step 2 → `/{project}:specify`), chores, and discards** set **no** target — a rule file and a chore have no single spec home, and `/{project}:specify` already targets the spec it creates.

Write the target the way every session-target write does. On the runtime path the `write-session` primitive performs the write: it sets this process's own target when the process has a session identity, and the shared default `.ductus/session.toml` in every case, preserving the default's `cli-config-dir` (§concurrent-features) — except a default that does not parse, which the write replaces and reports under `unreadable`, its `cli-config-dir` lost with it. Display what the write reports: any `notices` it delivered, the `peers` — each other session on this feature, with its scenario when it has one and when it last used its target, since this write is when the writing session is told who shares its feature (§concurrent-features) — and any `expired` or `unreadable` session. On the markdown-only path, write the shared default by hand: first read any existing `.ductus/session.toml` to capture its `cli-config-dir` (the per-contributor agent identity written by `/ductus`) and carry it forward, then rewrite the file atomically (tempfile + rename) with the target keys plus `set-at` (ISO 8601 UTC). Dropping `cli-config-dir` would strip the agent identity, so it is carried forward whenever the existing file parses.

Across a multi-item run, each spec-routed item performs this write, so the session target follows the current item and ends pointing at the most recently groomed spec.

## Reopening a `done` spec

When a Step 4 scenario is created under a matched spec whose status is `done`, reopen it `done → in-progress` as part of the same routing action — the documented back-edge §spec-lifecycle ("Backward via new scenario") defines, the same one `/{project}:amend`'s scenario route performs. The new scenario carries an unimplemented task, so leaving the spec `done` would make the follow-on `/{project}:implement` gate-fail; the reopen keeps status and on-disk content consistent.

- **Only from `done`.** A matched spec in `draft`, `clarified`, `planned`, or `in-progress` is left unchanged — groom never advances or otherwise moves a non-`done` status.
- **Guarded.** Confirm the on-disk status is still `done` immediately before writing; if it has changed since the match (a concurrent edit), surface the discrepancy rather than blindly overwriting — guarding the reopen against a lost concurrent update the way every status back-edge does. On the runtime path the `set-status` primitive's `from: done` argument enforces this guard.
- **No separate prompt, but the reopen is named.** When the matched spec is `done`, the per-item routing confirmation names the reopen alongside the scenario and target (see step 3 of the Instructions), so that single confirmation is the consent for the reopen and the operator approves the `done → in-progress` change before it happens — no separate prompt is added. The completion summary also names any spec reopened, so the status change is surfaced both before (at the confirmation) and after (in the summary).
- **The spec-edit route is out of scope.** It has its own back-edge (§spec-lifecycle, "Backward via meaningful body edit") and this scenario-reopen does not apply to it.
