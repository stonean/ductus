# 035 — Groom sets the session target from the routed item Plan

Implements [035 — Groom sets the session target from the routed item](spec.md).

## Overview

Edit `framework/commands/groom.md` so that the two decision-tree branches that route an item to an existing spec also write the session target, and the per-item confirmation names that target. The session write is the `write-session` primitive, which preserves any `cli-config-dir`, as `specify.md` and `amend.md` write theirs (see Trade-offs). The generated `.claude/commands/ductus/groom.md` regenerates from the source.

## Technical Decisions

### Where the target is written

Two branches route to an existing spec; both set the target:

- **Step 3 (spec edit)** — target is the matched **feature** (`feature` + `path` only, no scenario fields).
- **Step 4 durable-requirement branch (scenario creation)** — target is the matched feature **plus the new scenario** (`feature` + `path` + `scenario` + `scenario-path`), consistent with how `amend.md`'s scenario route sets the session target. A follow-on `/ductus:implement` then works the scenario's task directly.

Branches that do **not** write a target: Step 1 (rule item — amend a rule file, no spec home), Step 2 (no spec → hand off to `/ductus:specify`, which targets the spec it creates), and the Step 4 chore branch (done in the pass and then removed).

### How the target is written

The target is written with the `write-session` primitive, as `specify.md` and `amend.md` write theirs: it reads any existing `.ductus/session.toml` to capture `cli-config-dir`, then atomically rewrites the file with the new `feature` / `path` (/ `scenario` / `scenario-path`) plus `set-at` (ISO 8601 UTC), carrying `cli-config-dir` forward. `groom.md` step 8 invokes it, and `.ductus/reference/groom.md` §Setting the session target details the write.

### Consent model (no new prompt)

The per-item routing confirmation groom already requires ("wait for user confirmation before moving to the next item") is reworded to name the target it will set — e.g. *"Create a scenario under `033-rule-surface-setting` and set it as the session target? (Y/n)"*. That single confirmation is the consent for both the routing and the target write; no separate target prompt is added.

### Multi-item runs and completion

Each spec-routed item sets the target as it is processed, so the target follows the current item and ends pointing at the most recently groomed spec. The Completion section gains a line naming the final session target, or "session target unchanged" when no groomed item set one.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/groom.md` | Modify | Context note; Step 3 + Step 4 set the target; confirmation names it; Completion reports it |
| `.claude/commands/ductus/groom.md` | Regenerate | Generated copy (via the pre-commit `gen-claude-commands.sh`) |

## Trade-offs

- **Markdown-only session write vs. `write-session` primitive** — chose the `write-session` primitive: `groom.md` opens with the runtime-host-integration preamble and invokes `gate-confirm`, `create-scenario`, `append-task`, `set-status`, `write-session` and `remove-inbox-item`, so the session write is deterministic rather than prose. The rejected alternative, a markdown-only write, matched groom's style when 035 was written and kept the change small.
- **Scenario-target vs. spec-target for Step 4** — chose scenario-target (feature + scenario), matching `amend.md`'s scenario route, so a follow-on `/ductus:implement` lands on the scenario's task. Step 3 stays spec-only.
- **Target follows the current item vs. set-once** — chose follow-the-current-item: a single session target can only hold one value, and the most-recently-groomed spec is the most likely next action.
- **No separate prompt** — folding the target into the existing routing confirmation keeps groom's prompt count unchanged (procedural-fidelity), while still showing the operator the target.
