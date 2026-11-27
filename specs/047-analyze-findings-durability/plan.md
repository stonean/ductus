# 047 — Analyze Findings Durability Plan

Implements [047 — Analyze Findings Durability](spec.md).

## Overview

`/{project}:analyze` gives each live finding a disposition after detection, records the run in `analysis.md`, and renders the report last. The pre-done gate and `/{project}:review`'s analyze row read that record, and its freshness is a content comparison against `analyzed-digest`. The run record, its gate and its freshness check are designed in this spec's two scenarios. The runtime carries them: `write-analysis`, `check-review-gate` and `analyze_subjects.rs`, all under `runtime/src/primitives/`.

## Technical Decisions

### Findings take a disposition, not an inbox entry

No gate reads the inbox, so a finding parked there let its spec reach `done` looking finished. Each live finding is fixed, deferred or waived with its reason in the run that finds it, and `analysis.md` stores each deferred or waived decision. The spec's Resolved Questions carry the decision.

### A stored decision's key includes the message

A stored decision is keyed `{family} — {message}`, and the message is in the key because `ArtifactFinding` is `{family, severity, message, path}` and `path` is the **citing artifact** — the `spec.md` the criterion lives in — not the path that failed to resolve. Keying on family plus `path` was measured against this repo: 21 live findings collapse to **8** keys, so spec 012's `specs/errors.md` and `specs/events.md` would be dropped because `specs/system.md` keyed first, and one of spec 008's two rule files likewise. The distinguishing subject exists only inside `message`, so the message has to be in the key.

The cost is that re-wording a check's message asks about its findings once more. That is strictly better than silently discarding distinct findings, and it is rare, since check messages are stable prose. The wrong key would have failed *quietly*, holding a third fewer findings than the audit found.

### The record is written before the report renders

The record is written at step 21, before the render at step 22, so an interrupted run still leaves what it found. The render step is host-responsibility prose, and a primitive-dispatching step inserted *after* it would put a backticked primitive name inside a step the parser treats as host-only — the trap `AGENTS.md` §Gotchas records for `audit:ignore-promotion`.

### The gate's block reasons are breaking

`check-review-gate`'s `blocked-by` gained `not-analyzed`, `analyze-findings` and `analyze-stale`, values in a response set no schema documents as open, and the gate now holds `done` on a spec it passed before, so the change is breaking (`BE-COMPAT-001`). The runtime is pre-1.0, so it shipped in minor releases: the first two in 0.44.0, which `runtime/CHANGELOG.md` marks breaking for the completion gate, and the content-based staleness check by 0.46.0.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/analyze.md` | Modify | Fix-and-route, record and render steps after detection |
| `framework/constitution.md` | Modify | §brownfield-inbox Finding dispositions names a `/{project}:analyze` finding, and its scope covers a command whose primary output is findings |
| `.claude/commands/ductus/analyze.md` | Modify | Regenerated mirror |
| `runtime/src/primitives/write_analysis.rs` | Modify | Writes the run record to `analysis.md` |
| `runtime/src/primitives/check_review_gate.rs` | Modify | The `not-analyzed`, `analyze-findings` and `analyze-stale` blocks |
| `runtime/src/primitives/analyze_subjects.rs` | Modify | The subject set and its digest |
| `specs/047-analyze-findings-durability/tasks.md` | Modify | Task tracking |
