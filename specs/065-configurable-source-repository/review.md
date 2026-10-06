---
spec: 065-configurable-source-repository
last-run: 2026-10-06T14:30:43Z
reviewed-against: ca83e4fb71e4ea5a911bb51e3b83022dec0ce93b
diff-base: a1e426937188436d33704823bb3252c5618a97a1
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 7
scope: 22
skipped-passes: []
reviewed-digest:
  scenarios/a-non-canonical-origin-is-announced.md: caf57cb05fd97724d2f3d158f7a1ea3a9010664a018c4d01f089dbdfb7736b01
blocking: false
dispositions:
  fixed: 0
  routed: 1
  discarded: 1
  undispositioned: 0
decisions:
  - key: "bug: an inline `DUCTUS_REPO=… sh` install leaves the agent without the variable, so the fork's bootstrap fetches everything after it from stonean/ductus — the installer's next-step line never says to export it — `install.sh`"
    outcome: routed
    target: specs/065-configurable-source-repository/scenarios/a-non-canonical-origin-is-announced.md
    decided-at: 2026-10-06T14:30:43Z
    decided-by: andy@stone.dev
  - key: "other: compute-review-scope takes the plan's Affected Files brace shorthand `{plan.md, tasks.md, spec.md}` as one literal path — `specs/065-configurable-source-repository/plan.md`"
    outcome: discarded
    reason: "An observation about the pipeline's own machinery made while reviewing framework work (AGENTS.md): it adds one phantom entry to scope and hides nothing, since the three files it abbreviates are in scope by name."
    decided-at: 2026-10-06T14:30:43Z
    decided-by: andy@stone.dev
---

# Review — 065-configurable-source-repository

## Summary

Reviewed 065 over a1e42693..ca83e4fb, a --since override: 065 was reopened (19918a2b) after its change landed (e4784497), so the default window — from the reopen's parent — held none of the code, and the previous record's reviewed-against (65c06ef7) is a pre-rebase commit absent from this history. 0 MUST, 0 SHOULD, 1 low-confidence (CFG-ENV-001 on the bootstrap's per-fetch DUCTUS_REPO expansion); not blocking. Two CFG-ENV issues found in this pass were fixed before the record (613842d4: DUCTUS_REPO added to the docs/runtime.md inventory; install.sh's default named CANONICAL_REPO), and one observation was routed to the announcement scenario and implemented (ca83e4fb). examined 7 of 22: install.sh (in full), framework/bootstrap/ductus.md (every DUCTUS_REPO line and Source resolution, not the whole file), scripts/tests/test-install.sh (the stub and cases S, T, U), docs/runtime.md (the inventory), and 065's spec.md, scenario and tasks.md. Not read: framework/bootstrap/govern.md, confirmed byte-identical to ductus.md by cmp; the brace-shorthand entry, which names no file; and the 022 and 064 entries, the audit scripts, the gitignore template and the inbox — this window's changes to other specs, verified by their own tests and reviewed under those specs.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: CFG-ENV-001 — DUCTUS_REPO is re-expanded at every fetch site with its default spelled inline, not resolved once

- **File**: `framework/bootstrap/ductus.md:194-709`
- **Rule**: Every environment variable MUST be declared as either optional with a default fallback defined as a named constant, or required with no default (in which case CFG-ENV-003 governs its startup validation). Secrets MUST be declared required and MUST NOT carry an in-application default value (see BE-DATA-003). All environment variables MUST be read once at startup and the value cached; per-call reads from os.environ (or equivalent) are forbidden.
- **Finding**: Each of the bootstrap's fetch commands (lines 214, 234, 243, 250, 386, 388, 484, 705) spells `${DUCTUS_REPO:-stonean/ductus}`, so the agent's shell reads the variable afresh per command and the default literal is repeated rather than named. Source resolution already reads it once (line 194) and settles the run's other source values there. Low confidence: the rule is written for application code reading its environment, and these sites are agent-executed procedure prose whose literal default is also what Family 13 derives the repository's own slug from.
- **Auto-fixable**: no
- **Suggested fix**: Settle a `{source-repo}` value in Source resolution beside `{raw-ref}` and `{archive-ref}`, and name it at the fetch sites, updating self-url-resolution.sh to derive the slug from the settled value's default.

## Waived findings

*None.*

## Observations

- bug: an inline `DUCTUS_REPO=… sh` install leaves the agent without the variable, so the fork's bootstrap fetches everything after it from stonean/ductus — the installer's next-step line never says to export it — `install.sh` — **routed** to `specs/065-configurable-source-repository/scenarios/a-non-canonical-origin-is-announced.md`
- other: compute-review-scope takes the plan's Affected Files brace shorthand `{plan.md, tasks.md, spec.md}` as one literal path — `specs/065-configurable-source-repository/plan.md` — **discarded**: An observation about the pipeline's own machinery made while reviewing framework work (AGENTS.md): it adds one phantom entry to scope and hides nothing, since the three files it abbreviates are in scope by name.

## Skipped passes

*None.*

## Unexamined governance

*None.*
