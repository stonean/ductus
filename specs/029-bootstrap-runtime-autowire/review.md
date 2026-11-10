---
spec: 029-bootstrap-runtime-autowire
last-run: 2026-11-10T15:30:33Z
reviewed-against: c0090a22016a8b24865a6ca0b94c19d4adebd575
diff-base: 2b9f683b3c14396f129d1c713768d1aa0dcaadce
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 13
scope: 15
excluded: 235
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/archive-fetch-direct-codeload.md: e3fe33a84f9378bef18b43c2bfefc76b7e0cb2c6be657e3ba4d5e09e5285e64e
  scenarios/project-inputs-asked-once.md: f02f30c48f75111a396b20855bf98d7a2a8d798733963a0dc6515a15fd0233e9
  scenarios/runtime-probe-parity-audit.md: 5963ec40e25021632ef57cbc397dc807020e36866839a66673858f9b2348aa95
  scenarios/state-a-deterministic-path-forcing.md: 3cd44d4ec3dbeea6d80f61b259212d83d475537f4f204d2a1abee27524e4234a
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 12
  undispositioned: 0
decisions:
  - key: "security: `audit.md` step 1 and `analyze.md`'s drift check send the agent to `.ductus/bin/ductus run-generator`, which the seeded `Bash(.ductus/bin/ductus *)` allow runs with no prompt until 022 task 18 narrows it — `framework/commands/audit.md:31`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the constitution's principle 5 and §runtime-host-integration mapping still say every capability is an MCP tool, and deviations require their own amendment — `framework/constitution.md:623`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: Auggie's retired list lacks `mcp:ductus:run-generator`, which an Auggie project that ran `/configure` before still holds — `framework/bootstrap/configure/auggie.md`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 2's generated-block extraction repeats the marker literals that `scripts/tests/test-gen-configure-mcp.sh` and the generator use — `scripts/audit/manifest-parity.sh`"
    outcome: waived
    reason: scripts/tests has no lib.sh to share a helper through, and check-zero's gen-configure-mcp.sh --dry-run already fails when the markers drift
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the analyze walker's seed derivation leaves paths unbound for a refused feature rather than raising InvalidPath — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: it opens nothing, and the walk's first step, read-spec on the same feature, raises the InvalidPath before any step reads paths
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description.
    decided-at: 2026-11-10T15:30:33Z
    decided-by: rule
  - key: "other: the bootstrap's own curls in framework/bootstrap/ductus.md carry no timeout and no --proto or --tlsv1.2, though runtime-acquisition.yml calls itself their executable copy — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "pre-existing and outside this window: the bootstrap's curls are steps the host agent runs, 017's review routed only the workflows' and the installer's to 022 task 6, and bounding them changes the shipped bootstrap and 048's acquisition plan, a decision of its own"
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)"
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "other: a greenfield --ref run whose installed ductus.md is stale writes the config before aborting, so its re-run is not greenfield and prompts — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: pre-existing and not made worse; the documented install path places a current bootstrap
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the real bootstrap is walked by the host, as step 8's per-directory loop already is
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924`"
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag."
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 15 adds another hand-rolled scan of the §Agent Registry table beside those of installer-registry-parity.sh, host-namespace-parity.sh, placeholder-roundtrip.sh and permission-entry-shape.sh, while lib.sh's agent_config_dirs reads one column of it — `scripts/audit/runtime-probe-parity.sh`"
    outcome: waived
    reason: Each family reads a different part of the table and lib.sh's helper returns one column, so a shared row reader is a refactor of four other families' code, not a mechanical fix.
    decided-at: 2026-11-10T15:30:33Z
    decided-by: rule
---

# Review — 029-bootstrap-runtime-autowire

## Summary

Window from `2b9f683b` (`base-source: last-review`, 029's last review), `review-owed: true`: 15 scope entries, each a file, no pattern. 235 paths the window changed are excluded as other specs' commits, and none is deleted. The scope holds task 15's commit `c0090a22`, `bed3c016`'s lockfile update, which names no spec, `26c62f63`'s line in `scripts/audit/README.md`, and 029's own records (`43838a85`, `a1a1357b`, `01629ccc`).

Examined 13 of 15. Two are not counted. `framework/bootstrap/govern.md` is held byte-identical to `ductus.md` and was compared with `cmp` after each edit, not read. `.ductus/reference/audit.md` is generated from `framework/reference/audit.md` by `scripts/gen-claude-commands.sh`, re-run and reported in sync (`--check`).

Read in part:

- `framework/bootstrap/ductus.md`: the Instructions, Agent Registry, MCP registration, Permission Setup and Pre-flight Phase (lines 20–200); ductus runtime detection through State B (287–372); and the Self-update check, Pre-flight abort and Collect Project Inputs (505–600). The window's six other hunks, 080's tasks 10, 27, 31, 34 and 36/40 and 078's task 17, were read for what they change in text 029's criteria cite.
- `framework/reference/audit.md`: its Family 14–17 entries, which hold its one changed line.
- `runtime/Cargo.lock` and `scripts/audit/README.md`: their window diffs.

The rest were read whole, each scenario's concrete claims checked against the bootstrap and the script.

Of the 12 rule files and 310 rules, the passes applied the QUAL, CFG, SEC-DEPS, BE-DEPS and BE-INPUT rules; no other rule's subject is in these files.

One SHOULD, fixed in the run: `QUAL-GROUND-001` at `scripts/audit/runtime-probe-parity.sh`. Its agent list was hard-coded, so an agent added to the registry with CLI entries would pass unchecked, as `lib.sh`'s `agent_config_dirs` documents. The family now takes the agent set from the §Agent Registry rows and reports an agent no `check_agent` or `skip_agent` line names, and the scenario's Behavior and **New agent rows** say so. Proven in scratch copies under `/bin/bash`: HEAD exits 0, an unnamed sixth registry row exits 1 naming it, and the seven earlier mutations still exit 1.

Observations:

- One is waived by the rule: the family adds another hand-rolled scan of the registry table.
- Eleven decisions that 002's, 055's, 063's, 071's and 077's reviews hold on `framework/bootstrap/ductus.md` still stand against its text. They are carried with their text, reason and disposition.
- Two in-scope decisions no longer apply and are not carried: 048's on the Family 19 README reason and 064's on Pi's MCP client.
- 029's own six stored decisions name files outside this scope and are retained.

Task 15's real adopter bootstrap ran on the stale path, as the owner chose (2026-11-10). Two fresh agent sessions ran; each abort carried State B's runtime line and its file list. Edit (2) is waived with its reason there.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md` — **waived**: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
- convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md` — **waived**: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **waived**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md` — **waived**: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **waived**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md` — **waived**: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description. (decided by the rule)
- other: the bootstrap's own curls in framework/bootstrap/ductus.md carry no timeout and no --proto or --tlsv1.2, though runtime-acquisition.yml calls itself their executable copy — `framework/bootstrap/ductus.md` — **waived**: pre-existing and outside this window: the bootstrap's curls are steps the host agent runs, 017's review routed only the workflows' and the installer's to 022 task 6, and bounding them changes the shipped bootstrap and 048's acquisition plan, a decision of its own
- other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md` — **waived**: contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)
- other: a greenfield --ref run whose installed ductus.md is stale writes the config before aborting, so its re-run is not greenfield and prompts — `framework/bootstrap/ductus.md` — **waived**: pre-existing and not made worse; the documented install path places a current bootstrap
- other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md` — **waived**: the real bootstrap is walked by the host, as step 8's per-directory loop already is
- other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924` — **waived**: An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag.
- reuse: Family 15 adds another hand-rolled scan of the §Agent Registry table beside those of installer-registry-parity.sh, host-namespace-parity.sh, placeholder-roundtrip.sh and permission-entry-shape.sh, while lib.sh's agent_config_dirs reads one column of it — `scripts/audit/runtime-probe-parity.sh` — **waived**: Each family reads a different part of the table and lib.sh's helper returns one column, so a shared row reader is a refactor of four other families' code, not a mechanical fix. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/analyze.md` — 022-deterministic-runtime, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/audit.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/clarify.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/configure.md` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/consolidate.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/fold.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/groom.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/help.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/implement.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/log.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/plan.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/prune.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/review.md` — 022-deterministic-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/specify.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.ductus/reference/analyze.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.ductus/reference/groom.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.ductus/reference/implement.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.ductus/reference/plan.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.ductus/reference/review.md` — 022-deterministic-runtime, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.githooks/pre-commit` — 080-deterministic-runtime-efficiency
- `.github/workflows/generators.yml` — 080-deterministic-runtime-efficiency
- `.pi/extensions/ductus.ts` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-analyze.md` — 022-deterministic-runtime, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-audit.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-clarify.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-configure.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-consolidate.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-fold.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-groom.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-help.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-implement.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-log.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-plan.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-prune.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-review.md` — 022-deterministic-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-specify.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `AGENTS.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `README.md` — 050-constitution, 055-shared-constitution, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `docs/developing-ductus.md` — 080-deterministic-runtime-efficiency
- `docs/runtime.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `docs/shared-constitution.md` — 050-constitution, 055-shared-constitution
- `docs/slash-commands.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `framework/bootstrap/configure/auggie.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/bootstrap/configure/claude.md` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/bootstrap/configure/pi.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/bootstrap/pi/ductus-bridge.ts` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/commands/analyze.md` — 022-deterministic-runtime, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/commands/audit.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/clarify.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/consolidate.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/fold.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/groom.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/help.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/commands/implement.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/log.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/plan.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/prune.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/review.md` — 022-deterministic-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/commands/specify.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/commands/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/constitution.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/reference/analyze.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/reference/groom.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/reference/implement.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/reference/plan.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `framework/reference/review.md` — 022-deterministic-runtime, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/runtime-tools.txt` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/CHANGELOG.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/hook.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/mod.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/payload.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/main.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/mcp/server.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/apply_manifest.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/bounded_lists.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/brief_re_read.rs` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_prescriptions.rs` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_review_gate.rs` — 047-analyze-findings-durability
- `runtime/src/primitives/classify_spec_edit.rs` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `runtime/src/primitives/compute_review_scope.rs` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/discover_rule_files.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/fix_rounds.rs` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/measure_reopenings.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/merge_permissions.rs` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/merge_status_hook.rs` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/mod.rs` — 022-deterministic-runtime, 064-pi-host-support, 078-review-fix-rounds-are-re-read-within-a-bound, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/prune_tasks.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/read_section.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/read_tasks.rs` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `runtime/src/primitives/render_help.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/render_status.rs` — 062-concurrent-session-targets
- `runtime/src/primitives/resolve_anchor.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/resolve_constitutions.rs` — 050-constitution, 055-shared-constitution
- `runtime/src/primitives/rule_sections.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/tool_loading.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/write_analysis.rs` — 050-constitution, 055-shared-constitution, 058-findings-route-at-discovery, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `runtime/src/primitives/write_review.rs` — 050-constitution, 052-spec-supersession-and-consolidation, 055-shared-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/schema/config_keys.rs` — 050-constitution, 055-shared-constitution, 078-review-fix-rounds-are-re-read-within-a-bound
- `runtime/src/schema/constitutions.rs` — 050-constitution, 055-shared-constitution
- `runtime/src/schema/descriptions.rs` — 022-deterministic-runtime, 023-govern-refinement, 050-constitution, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/schema/primitives.rs` — 022-deterministic-runtime, 023-govern-refinement, 050-constitution, 052-spec-supersession-and-consolidation, 054-remove-supersession, 055-shared-constitution, 062-concurrent-session-targets, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/schema/registry.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/tests/concurrent_sessions.rs` — 062-concurrent-session-targets
- `runtime/tests/fixtures/review-basic/specs/007-review/spec.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `runtime/tests/golden/implement-basic.jsonl` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/tests/golden/review-basic.jsonl` — 022-deterministic-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/tests/hook_status.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/tests/mcp.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/result_sizes.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/audit/marker-list-parity.sh` — 026-framework-self-audit
- `scripts/audit/rename-sweep-residue.sh` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `scripts/audit/step-reference-integrity.sh` — 026-framework-self-audit
- `scripts/gen-claude-commands.sh` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `scripts/measure-command-cost.py` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/mutation/analysis-standing-decision.py` — 058-findings-route-at-discovery
- `scripts/mutation/declined-note-edits.py` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/destination-checks.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/fix-round-record.py` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/loop-inputs-screened.py` — 078-review-fix-rounds-are-re-read-within-a-bound
- `scripts/mutation/merge-permissions-canonical.py` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `scripts/mutation/merge-status-hook.py` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/mutation/own-record-is-no-round.py` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/reference-and-rule-reads.py` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `scripts/mutation/render-help.py` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/mutation/reopen-measure.py` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `scripts/mutation/resolve-constitutions-size.py` — 050-constitution, 055-shared-constitution
- `scripts/mutation/scope-attribution.py` — 050-constitution, 052-spec-supersession-and-consolidation, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/scope-results-bounded.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/status-view-identity.py` — 062-concurrent-session-targets
- `scripts/mutation/tool-loading.py` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `scripts/tests/pi-bridge-harness.mjs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/tests/test-gen-claude-commands.sh` — 080-deterministic-runtime-efficiency
- `specs/007-govern-workflow/tasks.md` — 007-govern-workflow, 080-deterministic-runtime-efficiency
- `specs/015-tarball-fetch/analysis.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/spec.md` — 015-tarball-fetch
- `specs/022-deterministic-runtime/scenarios/config-readers-name-unknown-keys.md` — 022-deterministic-runtime, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/framework-list-dedup.md` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime, 023-govern-refinement, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/023-govern-refinement/spec.md` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `specs/026-framework-self-audit/analysis.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/review.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-19-mechanical-sweep-exemption.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-19-review-freshness.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-19-says-what-it-examined.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-28-audit-family-registry-parity.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/028-antigravity-agent/tasks.md` — 028-antigravity-agent
- `specs/045-decision-state-drift-detection/analysis.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/spec.md` — 045-decision-state-drift-detection
- `specs/047-analyze-findings-durability/analysis.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/plan.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/review.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/spec.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/050-constitution/plan.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/050-constitution/spec.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/050-constitution/tasks.md` — 050-constitution, 055-shared-constitution
- `specs/052-spec-supersession-and-consolidation/tasks.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/054-remove-supersession/analysis.md` — 054-remove-supersession
- `specs/054-remove-supersession/plan.md` — 054-remove-supersession
- `specs/054-remove-supersession/review.md` — 054-remove-supersession
- `specs/054-remove-supersession/spec.md` — 054-remove-supersession
- `specs/054-remove-supersession/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/055-shared-constitution/analysis.md` — 055-shared-constitution
- `specs/055-shared-constitution/data-model.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/plan.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/review.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/scenarios/a-registered-source-is-named-with-its-description.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/scenarios/target-reports-each-constitutions-size.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/spec.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/058-findings-route-at-discovery/analysis.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/data-model.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/plan.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/review.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/scenarios/analysis-drift-judges-the-record-it-writes.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/scenarios/analyze-findings-match-decisions-by-host-judgment.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/scenarios/findings-cost-in-proportion-to-risk.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/spec.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/061-updates-track-the-latest-release-tag/analysis.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/spec.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/tasks.md` — 061-updates-track-the-latest-release-tag
- `specs/062-concurrent-session-targets/analysis.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/data-model.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/plan.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/review.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/scenarios/first-writers-create-the-ignore-file-once.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/spec.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/064-pi-host-support/plan.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/064-pi-host-support/spec.md` — 022-deterministic-runtime, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-supplied-summary-carries-the-loops-account.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md` — 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/scenarios/a-reviews-own-record-is-no-fix-round.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/analysis.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/data-model.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 052-spec-supersession-and-consolidation, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/review.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/research.md` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/antigravity-keeps-the-model-path.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/configure-types-no-permission-entries.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/help-answers-with-no-model-turn.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/one-search-covers-referenced-sections.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/opencode-keeps-the-model-path.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/reference-reads-follow-the-cited-sections.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/rules-load-without-shell-reads.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/the-search-goes-out-with-the-first-call.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 007-govern-workflow, 022-deterministic-runtime, 023-govern-refinement, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
