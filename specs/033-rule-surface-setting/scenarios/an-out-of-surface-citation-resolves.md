---
section: "Enforcement behavior"
---

# An-out-of-surface-citation-resolves

## Context

`/{project}:analyze` loads every rule file present in the rule-file directory so that a citation to a rule on any surface resolves (`framework/reference/analyze.md` §Rules (blocking and advisory); AC7). But `/ductus` installs only the rule files whose surface `[rules] surfaces` lists (the **Rule-file surface filter** in `framework/bootstrap/ductus.md` §Shared Files). A project that sets `surfaces = ["backend"]` on its first scaffold never receives `security-frontend.md`, so a backend spec that cites `FE-XSS-001` — the example `analyze.md` gives for why analyze loads every file — gets a blocking "unknown rule" finding from `check-rule-ids`. AC7's guarantee holds only when the out-of-surface file happens to be on disk already. Surfaced 2026-10-19 by the rule-file review.

## Behavior

- A citation to a rule on a surface the project does not enforce resolves without a blocking finding, whether or not that surface's rule file was ever installed: `check-rule-ids` reports a citation of an ID that no file it was given defines, and that the framework ships, in a distinct informational state — known, not installed — never as missing.
- The set of shipped rule IDs `check-rule-ids` reads that state from is an index of every rule the framework's rule files define at the runtime's version, so it needs no file in the adopter's tree.
- `[rules] surfaces` keeps scoping `/{project}:review` enforcement and `/{project}` keeps installing only the configured surfaces; the fix does not make `/{project}:review` enforce an out-of-surface file.

## Edge Cases

- A citation to a rule ID that exists on no surface at all is still reported as unknown, the Blocking finding it is today.
- A citation of a shipped rule whose file is installed resolves as found, as today.
- A citation of a rule from a project-authored rule file that is not installed: no shipped index holds it, so it is unknown.
- `/{project}:analyze` records each informational citation as unexamined, since the rule's text was not available to judge it against, and never folds it into a clean result.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which mechanism closes the gap?** (c): `check-rule-ids` reports a citation to a known out-of-surface rule as an informational unknown — known, not installed — rather than missing, read from an index of the shipped rule IDs (owner, 2026-11-12). Where the index lives, compiled into the runtime binary or shipped as a generated file with a drift check, is chosen when the task is planned and recorded in `plan.md` with its reason; the runtime ships in lockstep with the framework, so either holds the IDs of the version an adopter runs. The change is to `check-rule-ids`, the surface 067 owns, so `cross-spec-impact:` names it. It amends AC7, which said citations resolve against the full rule-file set. Rejected: (a) installing every rule file regardless of `surfaces`, which reverses this spec's second gap — a backend-only project would receive the three frontend files, 107 rules and 123,216 bytes at `ab2ad085`, each assessed by every analysis; and (b) resolving such a citation silently as found, which needs the same index and hides that analyze could not read the rule.
