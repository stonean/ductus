---
section: "Enforcement behavior"
---

# An-out-of-surface-citation-resolves

## Context

`/{project}:analyze` loads every rule file present in the rule-file directory so that a citation to a rule on any surface resolves (`framework/commands/analyze.md` §Rules; AC7). But `/ductus` installs only the rule files whose surface `[rules] surfaces` lists (`framework/bootstrap/ductus.md` §Rule-file surface filter). A project that sets `surfaces = ["backend"]` on its first scaffold never receives `security-frontend.md`, so a backend spec that cites `FE-XSS-001` — the example `analyze.md` gives for why analyze loads every file — gets a blocking "unknown rule" finding from `check-rule-ids`. AC7's guarantee holds only when the out-of-surface file happens to be on disk already. Surfaced 2026-10-19 by the rule-file review.

## Behavior

- A citation to a rule on a surface the project does not enforce resolves without a blocking finding, whether or not that surface's rule file was ever installed.
- `[rules] surfaces` keeps scoping `/{project}:review` enforcement; the fix does not make `/{project}:review` enforce an out-of-surface file.

## Edge Cases

- A citation to a rule ID that exists on no surface at all is still reported as unknown.

## Open Questions

- Which mechanism closes the gap: (a) /ductus installs every rule file regardless of [rules] surfaces, so surfaces scopes /review only; (b) /analyze resolves citations against the framework's full rule set even when a file is not installed; or (c) check-rule-ids reports a citation to a known out-of-surface rule as an informational unknown rather than missing?

## Resolved Questions

*None yet.*
