---
section: "Follow-on scenarios"
---

# Orphan-check-adopter-authored-references

## Context

`check-orphaned-references` examines a fixed `REFERRERS` list — `CLAUDE.md`, `AGENTS.md`, `README.md`, and `.githooks/pre-commit` — for references to framework paths a migration has moved. The `ductus-rename` migration's step 3 repairs `.govern/constitution.md` references in exactly the first three of those.

Both halves are scoped to files the framework itself authored, which is a principled boundary: the framework repairs what it wrote. The blind spot it leaves is not. An adopter's `specs/system.md` is `create`-strategy — seeded once and owned by the adopter thereafter — so when the adopter writes their own reference to a framework path (a link to the constitution, a tree diagram naming the per-project directory), nothing rescaffolds the file, no migration step names it, and the orphan check does not look at it.

The run therefore completes clean while a tracked file points at a directory that no longer exists. Observed on a live adopter bootstrap, where a host caught it by reading the file rather than by any check.

## Behavior

`check-orphaned-references` includes the adopter-owned `specs/system.md` in `REFERRERS` and **reports** a stale framework-path reference found there.

It does not repair it. The repair asymmetry is deliberate and stays as-is: the framework rewrites only references it authored, and an adopter-authored reference belongs to the adopter. Reporting is the whole of the fix — the finding names the file and the dangling target so the adopter can act, instead of the run exiting clean on a reference already known to be broken.

The path resolves through `[paths] specs-root`, so a project that renamed its spec root is examined at its configured location rather than at a hardcoded `specs/`.

## Edge Cases

- **The file is absent.** An adopter who deleted or never received `specs/system.md` yields no finding — and that outcome stays distinguishable from having examined the file and found it clean, per §design-principles: a subject the check could not reach is surfaced as such, never folded into a clean result.
- **A non-default `[paths] specs-root`.** The check follows the configured root. A hardcoded `specs/` would silently examine nothing on such a project — the same failure shape already recorded for `config_path_of` in the shipped `specs-root.sh`.
- **The reference is already correct.** A file pointing at the current framework path produces no finding; the check reports dangling targets, not every framework path an adopter happens to mention.
- **A framework path that never moved.** No finding — only targets a migration relocated are candidates.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
