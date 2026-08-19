---
section: "Check Families"
---

# Family-27-done-spec-unchecked-criteria

## Context

A spec at `status: done` whose Acceptance Criteria still carry an unchecked
box. The completion gate is supposed to make this unreachable —
[`implement.md`](../../../framework/commands/implement.md)'s criteria step
marks each verified criterion and then states that when any criterion remains
unchecked, the run reports the failures and does **not** propose the
transition. Reaching `done` with one unticked therefore means the gate was
bypassed or its marking step failed, and either is worth knowing.

It happened here. `026` reached `done` in `e9262df` with AC19 unchecked, and
every signal stayed green for the whole interval: `scripts/audit/run-all.sh`
exited 0, `check-artifacts` reported the feature `clean: true`, `/audit` had no
family for it, and CI passed. It was found by a hand-written `grep` while
sweeping for remaining work, which is the same way Family 24's residue was
found and the same reason that family exists.

What makes it worth a check rather than a one-line fix is the direction of the
failure. An unchecked criterion under a `done` spec reads, to anyone auditing
the corpus, as *work that was never finished* — so either the spec is lying
about being complete, or the criterion is lying about being unmet. Both are
cheap to repair and neither announces itself. `check-artifacts` deliberately
does not cover it: its `acceptance-criterion path existence` family asks
whether a *checked* criterion's paths exist, which is the opposite direction.

## Behavior

A new `/audit` family reports every spec at `status: done` that carries at
least one unchecked acceptance criterion, anchored to `file:line` with the
criterion's label so the reader can go straight to it.

The check is exact rather than heuristic: the subject is a checkbox in a known
section of a file with known frontmatter, so there is nothing to infer.

- **Only `done` specs.** A spec at `draft`, `clarified`, `planned`, or
  `in-progress` is *expected* to carry unchecked criteria — that is what those
  states mean. Flagging them would make the family fire on every spec in
  flight and be ignored within a day.
- **The suggested fix names both repairs**, because the family cannot tell
  which is right: tick the criterion if the work is done, or reopen the spec to
  `in-progress` if it is not.
- **Scope is reported.** The family names how many `done` specs it examined,
  so a clean exit reads as *"examined N done specs, all criteria checked"* and
  never as *"nothing to examine"* — the distinction
  [`QUAL-CLAIM-001`](../../../framework/rules/quality-cross.md) requires and
  the one this repo has paid for repeatedly.
- **An empty enumeration is a finding**, not a pass. Zero `done` specs in a
  repo that has 51 of them means the derivation broke, and the fail-closed
  direction is the one Families 17, 18, and 23 already take.

## Edge Cases

- **A spec with no Acceptance Criteria section at all.** Not a finding here.
  Section-completeness is `/ductus:analyze`'s artifact-tier concern, and
  duplicating it would cross the boundary `audit.md` §Notes draws between the
  two commands. This family asks only about criteria that exist.
- **A criterion inside a fenced code block.** Documentation quotes checkbox
  syntax — `analyze.md` and `implement.md` both do — so a fence-blind scan
  would report a document *describing* a criterion as carrying one. Fences are
  skipped, the same rule Family 25 and Family 26 already apply.
- **The frontmatter and the section must both be parsed, not grepped
  loosely.** `status: done` appears in prose across the corpus (this scenario
  contains it), so the status must come from the frontmatter block and the
  checkbox from the Acceptance Criteria section, not from a repo-wide grep for
  either token.
- **Specs are enumerated from git, not the worktree.** An untracked spec
  directory is invisible to `git ls-files`, so it is skipped and *counted* —
  the same rule `derive-dependencies` follows and reports as
  `untracked-skipped`. Silently ignoring one would let a spec escape the check
  by never having been committed.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
