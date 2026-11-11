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

**Reversed by 058: no capture step remains, and this spec's later tasks changed
the runtime.** The overview above is recorded as taken. `/{project}:analyze`
now gives each live finding a disposition after detection, and appends nothing
to the inbox. The run record, its gate and its freshness check (tasks 3–5) are
designed in this spec's two scenarios, and the record is `analysis.md`'s since
057.

## Technical Decisions

### The inbox, not a new artifact

**Reversed by 057 and 058: `analysis.md` exists, and no finding goes to the
inbox.** The spec's Resolved Questions carry both reversals.

Settled in the spec's Resolved Questions. Restated here because it is the
decision that sizes the work: choosing the inbox turns a new-primitive,
new-artifact, 47-file change into a two-file prose edit.

### `append-inbox`'s dedup guard is the idempotency mechanism

**Retired by 058: nothing is appended.** A stored decision keyed
`{family} — {message}` keeps a re-run quiet, and the message stays in the key
for the reason below.

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

**Retired by 058: there is no capture step.** The record is written at step 21,
before the render at step 22.

The append precedes the render step so an interrupted run still records what it
found. This also keeps the step above the `audit:ignore-promotion` render step,
which matters mechanically: the render step is host-responsibility prose, and
inserting a primitive-dispatching step *after* it would put a backticked
primitive name inside a step the parser treats as host-only — the trap
`AGENTS.md` §Gotchas records for `audit:ignore-promotion`.

### Step numbering

**Superseded: the command's steps have been renumbered since, and none is a
capture step.**

The capture step becomes step 14 and the render step shifts 14 → 15. Renumbering
is mechanical, but any cross-reference to "step 14" in the command body has to
move with it; the implementation greps for step references before renumbering.

### The gate's new block reasons are breaking

`check-review-gate`'s `blocked-by` gained `not-analyzed`, `analyze-findings`
and `analyze-stale`, values in a response set no schema documents as open, and
the gate now holds `done` on a spec it passed before, so the change is breaking
(`BE-COMPAT-001`). The runtime is pre-1.0, so it shipped in minor releases:
the first two in 0.44.0, which `runtime/CHANGELOG.md` marks breaking for the
completion gate, and the content-based staleness check by 0.46.0.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/analyze.md` | Modify | Capture step before render; markdown-only reference section (both since retired, by 058 and 071) |
| `framework/constitution.md` | Modify | `framework/constitution.md` §brownfield-inbox (its *Automatic issue capture* subsection) — analyze as a surfacing gate, scope widened (since 058, its Finding dispositions subsection) |
| `.claude/commands/ductus/analyze.md` | Modify | Regenerated mirror |
| `specs/047-analyze-findings-durability/tasks.md` | Modify | Task tracking |
