---
section: "Command Set"
---

# Scenario-without-task-visibility

## Context

`/{project}:amend`'s scenario route is the sanctioned way to add a scenario: it writes `scenarios/{slug}.md`, appends a linked task to `tasks.md`, and takes the `done → in-progress` back-edge. A scenario added by hand — by an agent writing the file directly, or by a contributor — skips all three, and two separate mechanisms then fail to notice.

`/{project}:amend`'s re-open precondition detects only *uncommitted* deltas (`git status --porcelain`, untracked `??` scenario files — `framework/commands/amend.md:51-53`) and fires only on `done` specs (`:68`). Once the hand-added scenario is committed, that signal is gone permanently.

`/{project}:analyze`'s scenario→task mapping family explicitly does not flag a scenario under a `done` spec that has no task. That is a deliberate decision from [041-task-pruning](../../041-task-pruning/spec.md): the scenario→task linkage is not a durable index, because an implemented scenario's tasks are expected to have been pruned.

[046-scenario-open-question-visibility](../../046-scenario-open-question-visibility/spec.md) closes the case where the hand-added scenario carries open questions — those block `done` and produce a finding. A committed, **question-free** scenario with no task falls through every check: the spec stays `done` while carrying behavior that was never implemented, and nothing surfaces it.

## Behavior

- The signal does not depend on working-tree state. Whatever surfaces a scenario with no task fires on a committed scenario as well as an untracked one; `git status --porcelain` alone is insufficient as the sole trigger.
- 041's pruning decision is preserved. A `done` spec whose implemented scenario tasks were pruned remains a non-finding. This scenario does not reintroduce the scenario→task linkage as a durable index.
- The two states are therefore distinguished by positive evidence, not by the absence of a task alone. Which evidence is the open question below.
- The evidence is **`tasks.md` history, per scenario**: a scenario is a finding on a `done` spec only when no revision of `tasks.md` ever named its slug. An implemented scenario had a task before it was pruned; a hand-added, never-implemented one never did. Nothing is declared by an author, so the signal cannot degrade by being forgotten.
- **The finding is advisory and names what it could not establish.** History proves *no task ever existed*; it cannot separate a scenario documenting already-shipped behavior from one describing unimplemented work. The finding says so rather than asserting the second, and the remedy — the reopen and task append `/{project}:amend`'s scenario route would have written — is the **operator's** to trigger. Nothing is reopened automatically, because the check cannot prove which of the two states it found.
- **Failure to consult history suppresses the finding.** No git repository, an unreadable history, a non-UTF-8 blob — each returns "was tasked" and emits nothing. §tasks-phase mandates that direction (*"a pruned spent task never produces a finding"*), so a check that cannot look must not manufacture a finding out of its own blindness.

## Edge Cases

- **A scenario documenting already-shipped behavior, written after the fact** — §scenarios says an implemented scenario stays as documentation. This must not be reported as unimplemented work; it is the discriminator the open question turns on.
- **A `done` spec whose scenario tasks were pruned** — unchanged non-finding, per 041.
- **A hand-added scenario carrying open questions** — already covered by 046's `scenario-open-questions` family. This signal must not double-report the same spec.
- **A scenario under a non-`done` spec with no task** — the spec is already actionable and needs no reopen; existing scenario-consistency coverage applies unchanged.
- **A `tasks.md` reset to template state** (041's reset mode) — every scenario under the spec loses its task at once, which must not turn each one into a finding.

## Open Questions

*None — see Resolved Questions.*

## Resolved Questions

- **What positive evidence distinguishes a scenario documenting already-shipped
  behavior from one describing unimplemented work?** **None does — and the
  check is built to say so.** The signal is a narrower one: *did a task for this
  scenario ever exist*, answered from `tasks.md` history. Resolved 2026-08-17,
  after measuring.

  The three candidates the question listed were each ruled out on evidence.
  An **explicit marker** is what `AGENTS.md`'s second Design Principle forbids,
  and here the correlation is perfect rather than merely likely: the author who
  hand-adds a scenario *is by definition* the one who bypassed
  `/{project}:amend`, so the marker would be missed in exactly the population it
  exists to catch. **Reading the code** is excluded by `analyze.md`'s own Scope
  Boundaries (*"Do NOT read source code or test files"*). And **git history as
  the question framed it** — *the scenario file postdates the spec's last
  transition to `done`* — does not discriminate at all: both states postdate
  that transition, which is what puts them both on a `done` spec.

  The mechanism also turned out to be mis-located. The scenario→task family did
  not merely vouch coarsely on `done` specs; it **skipped them wholesale**
  (`if status == "done" { return; }`), before pruning evidence was ever
  consulted. The pruning fingerprint is a non-`done` concern.

  Measured over this repo's 46 `done` specs before choosing: **one** unmapped
  scenario existed, and it *was* tasked historically. So reusing the file-shape
  fingerprint on `done` specs would have produced exactly **one finding, and it
  a false positive** — the direction §tasks-phase forbids. Adding the
  per-scenario history probe produces **zero** findings on the same corpus, and
  a fixture reproducing the never-tasked shape does fire. The rule is therefore
  clean on real data and demonstrably able to fail, though it has no
  demonstrated true positive here: the defect has not occurred in 50 specs,
  plausibly because the sanctioned `/{project}:amend` route is what people use.
