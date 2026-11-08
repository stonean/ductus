---
spec: 068-api-backend-rules
last-run: 2026-11-08T22:28:33Z
reviewed-against: ea3e198d0182dad81ad60dc3719e8b0250d8337e
diff-base: bbae384087f75bc7bde65033bde2041621637d43
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 4
excluded: 29
contract-breaks: 0
skipped-passes: []
fix-rounds: 4
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "other: round 1's re-read NOTE, that BE-ERR-002's and BE-ERR-004's Not-flagged clauses hand ERRENV a changed code or an envelope \"for an HTTP API\", a term only api-backend.md's header narrows, so a reader of security-backend.md alone could route a GraphQL API's changed code to BE-ERRENV-002 — `framework/rules/security-backend.md`"
    outcome: waived
    reason: "068's Resolved Question on whether ERRENV reaches RPC protocols decided security-backend.md needs no edit: a yield can hand ERRENV only what ERRENV's own Statements cover, which api-backend.md's header now defines, and the file is 008's, outside this spec"
    decided-at: 2026-11-08T22:28:33Z
    decided-by: rule
---

# Review — 068-api-backend-rules

## Summary

Review of 068's first reopening over `bbae3840..ea3e198d` (base-source `reopen`, the parent of the commit that took 068 in-progress), scope 4, examined 4. `framework/rules/api-backend.md` was read whole, as it stands with this run's fixes: all 23 rules against the loaded rule files, the rule schema in `specs/008-security-rules/data-model.md` (§Rule entry fields, §Verification phrasing, §Severity classification) and 068's AC1–AC16. `specs/068-api-backend-rules/spec.md` and `tasks.md` were read by the window's diff, the spec's scope table and Resolved Questions against the rule file. `plan.md`, unchanged since the base, entered the scope as uncommitted work through round 3's edit and was then read whole against the rule file. 0 MUST, 0 SHOULD. Six observations, five fixed by the rule and one waived by it. Fixed: `BE-COMPAT-001`'s Not-flagged clause names `BE-ERRENV-002` the owner for a resource-oriented HTTP API, as AC15 limits ERRENV, and `spec.md`'s Boundaries sentence describing it says the same; `BE-APIVER-002`'s gRPC sentence says the headers this rule requires of it, so a versionless API's method sends no `Sunset`; `BE-APIVER-001`'s Source names graphql.org's *Versioning* section without a `§`, which the self-audit's shipped-anchor family had rejected, failing framework-checks on `e28d928b` and `ea3e198d` (`scripts/audit/run-all.sh` exits 0 over the fixed tree); round 2's NOTE, the same token in `spec.md`, `plan.md` and `tasks.md`; and `plan.md`'s design record, which now names the `Sunset` exception for a versionless API and the gRPC metadata as the rule states them. Waived: round 1's NOTE on `security-backend.md`'s wording, which 068's Resolved Questions keep unedited. Fix rounds: round 1 (the first two fixes) and round 2 (the anchor) re-read clean by independent agents, rounds 3 (spec, plan, tasks) and 4 (plan) exempt, then empty; budget-stop false. Every external claim was checked against copies fetched 2026-11-08 by task 6 (`a8f419f7` names each document and URL); the fixes add none. The loop ran in a detached worktree of `ea3e198d`, because another session's uncommitted `runtime/src/interpreter/mod.rs`, `runtime/src/interpreter/payload.rs` and `specs/080-deterministic-runtime-efficiency/plan.md` sat in the main tree; they are outside this record and were not read. The primitives ran from `runtime/target/release/ductus` as built 2026-11-08T20:40:52Z, driven over stdio. The 29 excluded paths are 080's commits `57a63c35` and `401eb77a`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- bug: BE-COMPAT-001's Not-flagged clause names BE-ERRENV-002 the owner of a changed error code "for an HTTP API", a term the header defines as excluding an RPC protocol carried over HTTP only for the ERRENV rules, so a GraphQL or JSON-RPC API's changed code could be read as BE-ERRENV-002's against AC15 — `framework/rules/api-backend.md` — **fixed** (decided by the rule)
- bug: BE-APIVER-002's "A deprecated gRPC method MUST send them" can read as both headers, which a versionless API's deprecated endpoint MUST NOT send in Sunset's case — `framework/rules/api-backend.md` — **fixed** (decided by the rule)
- convention: BE-APIVER-001's Source cited a web page's section as "§Versioning", which the self-audit's shipped-anchor family reads as a § reference naming no heading or constitution marker, failing framework-checks on e28d928b and ea3e198d — `framework/rules/api-backend.md:47` — **fixed** (decided by the rule)
- other: round 1's re-read NOTE, that BE-ERR-002's and BE-ERR-004's Not-flagged clauses hand ERRENV a changed code or an envelope "for an HTTP API", a term only api-backend.md's header narrows, so a reader of security-backend.md alone could route a GraphQL API's changed code to BE-ERRENV-002 — `framework/rules/security-backend.md` — **waived**: 068's Resolved Question on whether ERRENV reaches RPC protocols decided security-backend.md needs no edit: a yield can hand ERRENV only what ERRENV's own Statements cover, which api-backend.md's header now defines, and the file is 008's, outside this spec (decided by the rule)
- convention: round 2's re-read NOTE, that the versionless Resolved Question still cited graphql.org's *Schema Design* as "§Versioning", which resolve-anchor reports unresolved; plan.md and tasks.md carried the same token — `specs/068-api-backend-rules/spec.md` — **fixed** (decided by the rule)
- convention: plan.md's design record omitted the Sunset exception task 6 decided for a versionless API's deprecated endpoint, and still said a deprecated gRPC method sends both headers and cited *PROTOCOL-HTTP2*, the document's old title — `specs/068-api-backend-rules/plan.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/configure.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `.ductus/reference/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/bootstrap/configure/auggie.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/bootstrap/configure/claude.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/commands/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/reference/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/runtime-tools.txt` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/mod.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/main.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/mcp/server.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/dashboard.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/mod.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/pipeline_view.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/render_status.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/schema/descriptions.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/schema/primitives.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/schema/registry.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/tests/golden/status-basic.jsonl` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/tests/measure_reopenings.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/tests/result_sizes.rs` — 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/062-concurrent-session-targets/plan.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
