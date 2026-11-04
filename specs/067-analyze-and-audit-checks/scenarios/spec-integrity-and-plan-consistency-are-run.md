---
section: "Artifact checks"
---

# Spec-integrity-and-plan-consistency-are-run

## Context

`framework/reference/analyze.md` states two blocking families, §Spec integrity (blocking) and §Plan consistency (blocking if plan exists). `docs/analyze.md` lists the first as Blocking, and 000's AC8 says `/{project}:analyze` checks spec integrity. But no numbered step of `framework/commands/analyze.md` runs either family, and `check-artifacts` implements neither, so a run on the runtime path or under `ductus exec` records `blocking-findings: 0` without examining them, and nothing reports them unexamined. Found while verifying 077's work (2026-11-03). Deferred here rather than waived, as `8359fbf0` had recorded it (owner, 2026-11-03, option a).

## Behavior

- Every `/{project}:analyze` run applies §Spec integrity and §Plan consistency on every path: a numbered step runs each check, a deterministic one through a primitive and a semantic one as a host step, and each failure is a Blocking finding.
- A check the path cannot run is recorded unexamined under a reason in **§Unexamined targets (informational)**, never folded into a clean result.

## Edge Cases

- A `draft` spec with no criteria is not flagged: the minimum belongs to the clarify gate.
- A spec with no `plan.md` skips §Plan consistency, as its heading says.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
