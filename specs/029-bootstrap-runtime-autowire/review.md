---
spec: 029-bootstrap-runtime-autowire
last-run: 2026-10-21T22:55:56Z
reviewed-against: cd31408891a3ff561e14fa2d3692f51b6dabb7f2
diff-base: bea69444cf4d62f501e86731ab8941649a5d2525
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 33
scope: 36
skipped-passes: []
reviewed-digest:
  scenarios/archive-fetch-direct-codeload.md: 30724d9cac10aed4737039a5711639761d7188071b42474e2add83e8de8bc4bb
  scenarios/project-inputs-asked-once.md: 300da5f046c71060e85aed09ffeb1d1779f09fffa8816e0a4fc375b4dcfb9960
  scenarios/runtime-probe-parity-audit.md: f03e440661b5aa0e0f0e373fe9be513e6500454009bcfe8e478a80da83fa78e6
  scenarios/state-a-deterministic-path-forcing.md: 1cccd7f45cae5546febc54603ca1d467065e778fc2d7b79500861264c5660555
blocking: false
dispositions:
  fixed: 9
  routed: 3
  discarded: 3
  undispositioned: 0
decisions:
  - key: "security: `audit.md` step 1 and `analyze.md`'s drift check send the agent to `.ductus/bin/ductus run-generator`, which the seeded `Bash(.ductus/bin/ductus *)` allow runs with no prompt until 022 task 18 narrows it — `framework/commands/audit.md:31`"
    outcome: routed
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the constitution's principle 5 and §runtime-host-integration mapping still say every capability is an MCP tool, and deviations require their own amendment — `framework/constitution.md:623`"
    outcome: routed
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: Auggie's retired list lacks `mcp:ductus:run-generator`, which an Auggie project that ran `/configure` before still holds — `framework/bootstrap/configure/auggie.md`"
    outcome: routed
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 2's generated-block extraction repeats the marker literals that `scripts/tests/test-gen-configure-mcp.sh` and the generator use — `scripts/audit/manifest-parity.sh`"
    outcome: discarded
    reason: scripts/tests has no lib.sh to share a helper through, and check-zero's gen-configure-mcp.sh --dry-run already fails when the markers drift
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the analyze walker's seed derivation leaves paths unbound for a refused feature rather than raising InvalidPath — `runtime/src/interpreter/mod.rs`"
    outcome: discarded
    reason: it opens nothing, and the walk's first step, read-spec on the same feature, raises the InvalidPath before any step reads paths
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs`"
    outcome: discarded
    reason: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
---

# Review — 029-bootstrap-runtime-autowire

## Summary

Reopen of 029 for 022 task 14's answer (b): `run-generator` came off the MCP surface, so 029's `state-a-deterministic-path-forcing` stopped listing it among the MCP tools. Diff base `bea69444` by `--since`, the commit before the fix (`cc4b3bd9`), since the default base, the reopen's parent, holds none of it; the window runs to `cd314088`, 36 scope entries.

How it was read. Two reviewers ran the five passes over `bea69444..1a762901` through git, splitting the eleven loaded rule files: the security pass against security-backend, security-frontend, api-backend, accessibility-frontend and performance-frontend; the reuse, quality, efficiency and simplicity passes against quality-cross, configuration-cross, reliability-backend, concurrency-backend, observability-backend and performance-backend. They found no MUST violation, one low-confidence SHOULD (`QUAL-GROUND-001`: Family 2 read the generated block by its markers without checking them; fixed in `48b0d744`) and the thirteen observations below. Each fix round after that (`48b0d744`, `2f4a7c19`, `f1c2aefa`, `4e3871c8`, `c88bf167`, `cc922323`) was re-read by an independent agent for correctness, claims and tests, six re-reads, each fixing what the last found, with probes run on macOS awk and Ubuntu's mawk and tests checked by mutation; the sixth found no MUST and one SHOULD, an untrue comment sentence, fixed in `cd314088`. Those re-reads read `scripts/gen-configure-mcp.sh`, `scripts/tests/test-gen-configure-mcp.sh` and `framework-checks.yml`'s new step (n), which the window gained after the two passes; the security pass did not read those three.

examined 33 of 36. Not read, pattern-scanned only for what the window touches: `README.md`, `framework/bootstrap/configure/antigravity.md`, `scripts/audit/installer-registry-parity.sh`. The five generated copies (`.claude/commands/ductus/{analyze,audit,configure}.md`, `.pi/prompts/ductus-{analyze,audit}.md`) were read in the diff only.

Two observations are carried byte-identically from 022's review at `9e7b7943`, the only two of its 89 stored decisions whose file is in this scope; `runtime/src/interpreter/mod.rs` changed since in one doc-comment line (`cc4b3bd9`), and both still hold.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- security: `audit.md` step 1 and `analyze.md`'s drift check send the agent to `.ductus/bin/ductus run-generator`, which the seeded `Bash(.ductus/bin/ductus *)` allow runs with no prompt until 022 task 18 narrows it — `framework/commands/audit.md:31` — **routed** to `specs/022-deterministic-runtime/tasks.md`
- other: 022's plan left task 15 item 1 (`lint-markdown` and `check-review-gate` running project code) out of Agent reach's parts — `specs/022-deterministic-runtime/plan.md` — **fixed**
- other: the constitution's principle 5 and §runtime-host-integration mapping still say every capability is an MCP tool, and deviations require their own amendment — `framework/constitution.md:623` — **routed** to `specs/022-deterministic-runtime/tasks.md`
- bug: Family 2 stopped comparing an MCP allow entry hand-added outside the generated block — `scripts/audit/manifest-parity.sh` — **fixed**
- other: Auggie's retired list lacks `mcp:ductus:run-generator`, which an Auggie project that ran `/configure` before still holds — `framework/bootstrap/configure/auggie.md` — **routed** to `specs/022-deterministic-runtime/tasks.md`
- other: 022's spec.md still said every primitive is an MCP tool in three places — `specs/022-deterministic-runtime/spec.md` — **fixed**
- other: registry.rs said in the present tense that task 18 keeps the CLI form out of the seeded allow, before task 18 is applied — `runtime/src/schema/registry.rs` — **fixed**
- other: audit.md's preamble said every Invoke step calls an MCP tool, above a step that does not — `framework/commands/audit.md:29` — **fixed**
- other: 022's plan said the manifest is held equal to `TOOL_NAMES`, which no longer exists — `specs/022-deterministic-runtime/plan.md` — **fixed**
- convention: the parser test name `primitive_names_match_mcp_tool_names_exactly` no longer matched its assertion — `runtime/src/parser/mod.rs` — **fixed**
- bug: Family 9 kept trailing whitespace on a `# cli-only:` value, so a padded line flagged audit.md step 1 — `scripts/audit/primitive-promotion-candidates.sh` — **fixed**
- convention: AGENTS.md's wiring gotcha did not say a CLI-only primitive takes a `# cli-only:` line — `AGENTS.md` — **fixed**
- reuse: Family 2's generated-block extraction repeats the marker literals that `scripts/tests/test-gen-configure-mcp.sh` and the generator use — `scripts/audit/manifest-parity.sh` — **discarded**: scripts/tests has no lib.sh to share a helper through, and check-zero's gen-configure-mcp.sh --dry-run already fails when the markers drift
- other: the analyze walker's seed derivation leaves paths unbound for a refused feature rather than raising InvalidPath — `runtime/src/interpreter/mod.rs` — **discarded**: it opens nothing, and the walk's first step, read-spec on the same feature, raises the InvalidPath before any step reads paths
- other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs` — **discarded**: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold

## Skipped passes

*None.*

## Unexamined governance

*None.*
