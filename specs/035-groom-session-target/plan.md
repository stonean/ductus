# 035 — Groom sets the session target from the routed item Plan

Implements [035 — Groom sets the session target from the routed item](spec.md).

## Overview

Edit `framework/commands/groom.md` so that the two decision-tree branches that route an item to an existing spec also write the session target, and the per-item confirmation names that target. Markdown-tier change — when this plan was written `groom.md` was a markdown-only command carrying no runtime-primitive preamble, so the session write is described in prose, preserving any `cli-config-dir`, exactly as `specify.md` / `amend.md` described their markdown-only session writes. (`022-deterministic-runtime` has since moved `groom.md` onto the runtime primitives — see Trade-offs.) The generated `.claude/commands/ductus/groom.md` regenerates from the source.

## Technical Decisions

### Where the target is written

Two branches route to an existing spec; both set the target:

- **Step 3 (spec edit)** — target is the matched **feature** (`feature` + `path` only, no scenario fields).
- **Step 4 durable-requirement branch (scenario creation)** — target is the matched feature **plus the new scenario** (`feature` + `path` + `scenario` + `scenario-path`), consistent with how `amend.md`'s scenario route sets the session target. A follow-on `/ductus:implement` then works the scenario's task directly.

Branches that do **not** write a target: Step 1 (rule item — amend a rule file, no spec home), Step 2 (no spec → hand off to `/ductus:specify`, which targets the spec it creates), and the Step 4 chore branch (done in the pass and then removed).

### How the target is written

A markdown-only session write, mirroring `specify.md` / `amend.md`'s fallback path: read any existing `.ductus/session.toml` first to capture `cli-config-dir`, then rewrite the file via tempfile + rename with the new `feature` / `path` (/ `scenario` / `scenario-path`) plus `set-at` (ISO 8601 UTC), carrying `cli-config-dir` forward. No new runtime-primitive preamble was added to `groom.md` by this plan; moving groom onto the `write-session`/`create-scenario`/`append-task` primitives was left as a separate refactor, which `022-deterministic-runtime` has since performed (noted in Trade-offs).

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

- **Markdown-only session write vs. `write-session` primitive** — chose markdown-only to match groom's style at the time (it then used no runtime primitives) and keep the change small. The deterministic-primitive version was a larger, separate refactor of groom onto `write-session`/`create-scenario`/`append-task`, out of scope here. **Superseded:** `022-deterministic-runtime` performed exactly that refactor in `0abc237e` (2026-07-11), thirteen days after this spec closed — `groom.md` now opens with the runtime-host-integration preamble and invokes `gate-confirm`, `create-scenario`, `append-task`, `set-status`, `write-session` and `remove-inbox-item`, so the rejected alternative is the shipped design. `write-session` writes the same target, so 035's behavior is unchanged by the move.
- **Scenario-target vs. spec-target for Step 4** — chose scenario-target (feature + scenario), matching `amend.md`'s scenario route, so a follow-on `/ductus:implement` lands on the scenario's task. Step 3 stays spec-only.
- **Target follows the current item vs. set-once** — chose follow-the-current-item: a single session target can only hold one value, and the most-recently-groomed spec is the most likely next action.
- **No separate prompt** — folding the target into the existing routing confirmation keeps groom's prompt count unchanged (procedural-fidelity), while still showing the operator the target.
