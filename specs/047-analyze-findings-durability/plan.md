# 047 — Analyze Findings Durability Plan

Implements [047 — Analyze Findings Durability](spec.md).

## Overview

Prose-only. `/{project}:analyze` gains one capture step before its render step,
and the constitution's auto-capture section gains analyze as a surfacing gate.
No runtime change: `append-inbox` already exists, is already in
`PRIMITIVE_NAMES`, and already carries the `dedup-prefix` guard this needs.

That "no runtime change" is the whole reason this is small. The capability was
already built — 022's `append-inbox-comment-aware-write` scenario shipped the
comment-aware append and the dedup guard — it simply was never wired into the
one command whose entire output is findings.

## Technical Decisions

### The inbox, not a new artifact

Settled in the spec's Resolved Questions. Restated here because it is the
decision that sizes the work: choosing the inbox turns a new-primitive,
new-artifact, 47-file change into a two-file prose edit.

### `append-inbox`'s dedup guard is the idempotency mechanism

`append-inbox` takes an optional `dedup-prefix`; when an existing bullet starts
with that prefix, nothing is written and the result reports `deduped: true`.
The capture step passes `{category}: {family} — {message}`, which is a true
prefix of the rendered bullet.

**The message is in the key, and the first draft of this plan had that wrong.**
The initial design keyed on family plus "subject path", assuming the finding
carried the missing path as a field. It does not: `ArtifactFinding` is
`{family, severity, message, path}`, and `path` is the **citing artifact** —
the `spec.md` the criterion lives in — not the path that failed to resolve.
Dogfooding the key against this repo before shipping it measured the damage:
21 live findings collapse to **8** keys, so spec 012's `specs/errors.md` and
`specs/events.md` would be dropped because `specs/system.md` keyed first, and
one of spec 008's two rule files likewise. The distinguishing subject exists
only inside `message`, so the message has to be in the key.

The cost is that re-wording a check's message re-appends its findings once.
That is strictly better than silently discarding distinct findings, and it is a
rare event — check messages are stable prose. Recorded here rather than
silently corrected, because the wrong key would have failed *quietly*: the
inbox would have looked correct while holding a third fewer findings than the
audit found.

### Capture before render

The append precedes the render step so an interrupted run still records what it
found. This also keeps the step above the `audit:ignore-promotion` render step,
which matters mechanically: the render step is host-responsibility prose, and
inserting a primitive-dispatching step *after* it would put a backticked
primitive name inside a step the parser treats as host-only — the trap AGENTS.md
§Gotchas records for `audit:ignore-promotion`.

### Step numbering

The capture step becomes step 14 and the render step shifts 14 → 15. Renumbering
is mechanical, but any cross-reference to "step 14" in the command body has to
move with it; the implementation greps for step references before renumbering.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/analyze.md` | Modify | Capture step before render; markdown-only reference section |
| `framework/constitution.md` | Modify | §Automatic issue capture — analyze as a surfacing gate, scope widened |
| `.claude/commands/gov/analyze.md` | Modify | Regenerated mirror |
| `specs/047-analyze-findings-durability/tasks.md` | Modify | Task tracking |
