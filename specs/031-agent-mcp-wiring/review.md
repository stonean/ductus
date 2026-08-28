---
spec: 031-agent-mcp-wiring
reviewed-at: 2026-08-28T01:24:04Z
reviewed-against: a9be853143093fc9891a87048ba286fc187ddfcd
diff-base: ae650c8bfbcf2e22535a571af9eaffd94f9d2067
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 031-agent-mcp-wiring

## Summary

Re-run 2026-08-28 against the current rule set. 0 MUST, 0 SHOULD outstanding, 1 waived; not blocking.

**Why this re-run happened.** The original review ran 2026-06-18, and **32 rule IDs now in force did not exist then** — the whole of `quality-cross.md`, `performance-backend.md`, `concurrency-backend.md`, `observability-backend.md`, and `reliability-backend.md` landed 2026-06-28, and `QUAL-CLAIM-001` on 2026-08-02. A verdict recorded before a rule exists is not evidence about that rule, so the counts were re-derived rather than trusted.

**The new rules were assessed, not assumed inapplicable.** 031's subject is text-first by construction — the plan's affected set is `framework/bootstrap/ductus.md`, `README.md`, `framework/migrations.toml`, command bodies, and spec artifacts, with no code. The backend families (`BE-QUERY-*`, `BE-CACHE-*`, `BE-POOL-*`, `BE-ASYNC-*`, `BE-RACE-*`, `BE-LOCK-*`, `BE-TXN-*`, `BE-COORD-*`, `BE-TRACE-*`, `BE-RETRY-*`, `BE-DRAIN-*`, `BE-BULK-*`) have no subject in scope. `FE-DEPS-005` governs frontend dependency egress; likewise none.

`QUAL-CLAIM-001` was the one worth checking closely, because 031's own scenario is a *verification* scenario and the rule governs claims that outrun what was examined. `antigravity-mcp-verification` is a model of compliance rather than a violation: it ran a positive control (the home-level config, which spawned the probe and produced 19 log references) to prove the method could detect a spawn at all before concluding from the negative result that project-local `.agents/mcp_config.json` is ignored, and it recorded the quota-exhaustion confound (`RESOURCE_EXHAUSTED 429`) along with why it does not affect the outcome — the sentinel spawns at MCP-init, before the model call. That is exactly the distinction between "examined and found nothing" and "could not examine" that the rule asks for.

**The stale count is corrected, and its cause removed.** `spec.md` recorded `should-violations: 1` while this report recorded `0`. The finding below was moved to Waived by hand on 2026-08-02 with its rationale, but no entry was ever added to `review.waivers` in the spec frontmatter — so the waiver had no structural existence, the count never dropped, and the two files disagreed. The waiver is now recorded in `spec.md`, `process-waivers` applies it, and this report renders it from that record rather than from prose. It will survive the next regeneration.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

### WAIVED: SIMPLICITY — `scope` field is descriptive metadata, not behaviorally load-bearing

- **File**: `framework/bootstrap/ductus.md`
- **Rule**: AGENTS.md §Design Principles / simplicity pass — avoid fields that are not load-bearing.
- **Finding**: The per-agent descriptor carries `scope` (`project-committed` / `user-global` / `home-level`) alongside `mechanism` (`write-file` / `surface-instruction`). Only `mechanism` drives State-B branching, and the exact location is already given by `target`. `scope` is therefore derivable — `user-global` (Auggie) and `home-level` (Antigravity) both map to `surface-instruction` and differ only in which home location `target` already names.
- **Auto-fixable**: no
- **Suggested fix**: Optionally drop `scope` from the descriptor and let `target` carry the location. Waived — see the rationale under Waived findings.
- **Waived**: `scope` documents a real conceptual distinction readers care about — committed-in-repo vs user-config-dir vs home-global — and the three-line table costs nothing; removing it would trade reader clarity for a metric. Keeping it was the finding's own recommendation.

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
