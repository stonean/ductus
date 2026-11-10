---
spec: 029-bootstrap-runtime-autowire
last-run: 2026-11-10T15:53:28Z
reviewed-against: 9cf5e5a8193ee7d4430f1abbc94251a53664e618
diff-base: 1097c24ad33247571b0107d939633d1d883b4e50
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 2
excluded: 0
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/archive-fetch-direct-codeload.md: e3fe33a84f9378bef18b43c2bfefc76b7e0cb2c6be657e3ba4d5e09e5285e64e
  scenarios/project-inputs-asked-once.md: f02f30c48f75111a396b20855bf98d7a2a8d798733963a0dc6515a15fd0233e9
  scenarios/runtime-probe-parity-audit.md: d398e9519a2510d3310e2aa4bbdb984594f8f0b8eee9fd45cba618995cc385fc
  scenarios/state-a-deterministic-path-forcing.md: 3cd44d4ec3dbeea6d80f61b259212d83d475537f4f204d2a1abee27524e4234a
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
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

Window from `1097c24a` (`base-source: last-review`, 029's second review), `review-owed: true`: 2 scope entries, each a file, nothing excluded or deleted. They are `9cf5e5a8`'s fix, which names `framework/bootstrap/ductus.md` beside the four § references in `runtime-probe-parity-audit` that the analysis's detection could not resolve, and task 15's record of it.

Examined 2 of 2:

- The scenario, read whole. Each concrete claim was checked against `scripts/audit/runtime-probe-parity.sh` and `framework/bootstrap/ductus.md` as `1097c24a` left them, which this window leaves unchanged. `resolve-anchor` now qualifies all nine of its references.
- The window's one added line of `tasks.md`.

Of the 12 rule files the passes applied the QUAL rules; no other rule's subject is in these files. No finding and no observation. The 18 stored decisions name files outside this scope and are retained. The owner chose this fix and review over a waiver (2026-11-10).

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

*None.*

## Skipped passes

*None.*

## Excluded from scope

*None.*

## Unexamined governance

*None.*
