---
section: "Behavior"
---

# Reopen-done-spec-on-scenario

## Context

Groom's Step 4 durable-requirement branch creates a scenario under the matched spec and sets it as the session target (035's behavior). When the matched spec's status is already `done`, the new scenario carries an unimplemented task, but the spec's status is left unchanged — it stays `done`. A follow-on `/ductus:implement` against the now-targeted but still-`done` spec gate-fails ("already done"), because the implement gate refuses to run on a done spec. This is the same gap `/ductus:amend`'s scenario route already closes when it records a scenario on a done spec.

## Behavior

When groom creates a scenario under a matched spec (Step 4 durable-requirement branch) and that spec's status is `done`, groom reopens it `done → in-progress` as part of the same routing action — the same guarded `set-status` write `/ductus:amend`'s scenario route performs (constitution §spec-lifecycle, "Backward via owed code": the appended task is owed work). The reopen happens together with creating the scenario, appending the task, and setting the session target, so the follow-on `/ductus:implement` finds an actionable `in-progress` target instead of a gate-failing `done` one. When the matched spec is not `done`, its status is left unchanged.

## Edge Cases

- A matched spec already in `draft`, `clarified`, `planned`, or `in-progress` is left unchanged — groom only reopens from `done`.
- The reopen uses a `set-status` write guarded on `from: done`, so a concurrent status change is detected rather than blindly overwritten.
- The Step 3 spec-edit route reopens only when the written edit leaves owed work (§spec-lifecycle, "Backward via owed code") and is out of scope for this scenario, which covers only the Step 4 scenario-creation route.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
