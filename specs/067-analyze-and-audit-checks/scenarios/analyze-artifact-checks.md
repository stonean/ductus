---
section: "Artifact checks"
---

# Analyze-artifact-checks

## Context

The 2026-07-11 coverage review found that several of `/ductus:analyze`'s deterministic check families exist only in its markdown-only reference — no owning primitive, no numbered step — so on the exec path it is unverifiable whether they run at all: artifact completeness (required files exist for the spec's status — plan.md/tasks.md on `planned`+), task consistency (numbering, done-when presence), scenario→task mapping (every scenario file's implementation task exists or the scenario is complete — honoring §tasks-phase's rule that pruned spent tasks do not count against it), review-state drift on `done` specs (`review.last-run` unset or `blocking: true` while status is `done`), and command-frontmatter completeness (project-level: every command file carries `description`/`argument-hint`).

## Behavior

A new `check-artifacts` primitive owns the residual deterministic families: given a feature, it owns the residual deterministic families (`ductus check-artifacts --help` lists them and the finding shape), scenario→task mapping among them, which treats a missing task as satisfied when any revision of `tasks.md` named the scenario, so the check never requires a spent task to persist per §tasks-phase — [`scenario-consistency-reads-tasks-history`](scenario-consistency-reads-tasks-history.md). `/ductus:analyze`'s Instructions gain a numbered step invoking it. Command-frontmatter completeness stays with the host (it reads the host's command directory, which the runtime does not own).

## Edge Cases

- A `draft` spec with no plan/tasks produces no completeness finding (files are required by status tier, not universally).
- A scenario whose task was pruned after completion produces no finding (tasks.md is ephemeral; §tasks-phase).
- Severity tiers are `check-artifacts`' own behavior, canonical in its compiled description; the primitive adds no policy.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
