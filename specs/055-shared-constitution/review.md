---
spec: 055-shared-constitution
reviewed-at: 2026-09-11T16:12:09Z
reviewed-against: bc216c85218a30650828175a80063a7ddd83359a
diff-base: dd2fc0c502466bb6db80612b4f10db124fb674f0
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 4
skipped-passes: []
---

# Review — 055-shared-constitution

## Summary

No outstanding violations across all five passes; nothing blocks `done`. This run regenerates a report the previous one could not: that review was written by an MCP server spawned before task 5 existed, so its `write-review` rendered the pre-feature skeleton and omitted the `## Unexamined governance` section this very spec added. This session's server execs the current binary (verified by inode against `runtime/target/release/ductus`), so the section is present — and reads `*None.*`, correctly: this repository registers no `[constitutions.*]` entries, so nothing went unexamined. The quality pass found one real defect and it was fixed before this report was written, so it is not carried as a finding: task 5 inserted `render_unexamined_governance` between `render_skipped`'s doc comment and the function it documents, leaving the new function's rustdoc summary describing a different function and `render_skipped` undocumented. Fixed in `bc216c8` as task 11 — routed to this spec's `tasks.md` rather than `specs/inbox.md`, since a defect in what the in-progress spec built belongs on that spec (§brownfield-inbox). BE-INPUT-004 was re-evaluated against `resolve_constitutions::classify` and does not fire: the `path` value is committed project config, not request-time input, and `validate_no_traversal`'s own doc comment scopes it to primitives taking host- or LLM-supplied paths — the same judgment `resolve-references` records for `[services]`. The four Captured issues below remain outstanding in `specs/inbox.md` for `/ductus:groom`; no new observation was recorded, because everything this run judged real is either fixed or already captured there.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: the clap subcommand enum in `runtime/src/main.rs` is the one primitive-wiring surface nothing pins — deleting a `Command` variant and its dispatch arm leaves the full suite green, so a primitive can be missing from the `ductus <name>` CLI undetected. Every other site is pinned (registry set-equality in tests/mcp.rs, the `#[tool]` listing, `dispatch_handles_every_registry_primitive`, and `PRIMITIVE_NAMES` being a direct alias). A test iterating `PRIMITIVE_REGISTRY` against the clap command list would close it. (found 2026-09-11 during 055)
- [ ] security: the decision not to call `validate_no_traversal` on a `[constitutions.*]` path rests on the claim that the value is hand-edited committed config — true today, but `/ductus:link` already sets the precedent for a command that writes a registry table programmatically. If anything ever writes `[constitutions]`, that trust claim silently becomes false and nothing checks it. Either keep hand-edit-only as a stated invariant or add the traversal check. — `runtime/src/primitives/resolve_constitutions.rs` (captured during review of 055-shared-constitution)
- [ ] convention: `schema/constitutions.rs` and `schema/services.rs` are near-identical registry modules, and `duplicate_paths` / `duplicate_repos` differ only in which field they group by. Deliberate at introduction (modelling on services was the point), but a third registry would make the case for one generic group-by-field helper. — `runtime/src/schema/constitutions.rs` (captured during review of 055-shared-constitution)
- [ ] other: `ConstitutionEntry.description` is parsed, documented in the config schema, and never surfaced — it reaches no primitive result and no report, unlike `ServiceEntry.description`, which the dashboard renders. Either surface it where a source is named or drop it from the schema. — `runtime/src/schema/constitutions.rs` (captured during review of 055-shared-constitution)

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
