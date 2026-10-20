---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-20T12:25:46Z
reviewed-against: e70447381561512b07454bf01716564bc91c9976
diff-base: 8eecb53608af1b51f7eff5e513fa26bdafbf50ef
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 40
scope: 63
skipped-passes: []
reviewed-digest:
  data-model.md: ded66d44180f7288cd2914d8409b18b7761fa37b97a7cec299da63162ffdf2cb
  scenarios/an-invalid-threshold-fails-fast.md: fb341859bc463f2c51d84856d76f158208a4b8180f9d94bc5537fc9f1bb62c71
  scenarios/an-unknown-artifacts-key-is-named.md: 912ce563e772f628884c9096a8fafb00526379e935c99f40d7ca96090d986e1a
blocking: false
dispositions:
  fixed: 14
  routed: 0
  discarded: 5
  undispositioned: 0
decisions:
  - key: "other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: discarded
    reason: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
    decided-at: 2026-10-20T12:25:46Z
    decided-by: andy@stone.dev
  - key: "perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs`"
    outcome: discarded
    reason: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
    decided-at: 2026-10-20T12:25:46Z
    decided-by: andy@stone.dev
  - key: "other: the pre-commit hook runs three primitives that read [paths], so one typo prints three identical stderr lines per commit — `framework/bootstrap/hooks/ductus-pre-commit`"
    outcome: discarded
    reason: once per process is the contract, and each hook step is a separate run reading the file
    decided-at: 2026-10-20T12:25:46Z
    decided-by: andy@stone.dev
  - key: "other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs`"
    outcome: discarded
    reason: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
    decided-at: 2026-10-20T12:25:46Z
    decided-by: andy@stone.dev
---

# Review — 063-oversized-artifacts-warn-with-a-fix

## Summary

Five-pass review of 063 over `8eecb536..e7044738` plus the working tree: spec 019's unknown-config-key change, whose `check-artifact-size` notice is this spec's (task 13). The 63 scope entries are that change set and 063's plan-affected files. **Read this session** (`examined: 40`): 063's `spec.md`, `plan.md`, `data-model.md`, `tasks.md` and its new scenario in full; `runtime/src/schema/config_keys.rs` in full, by this pass and by two parallel reviewer passes over `e7044738`, and the diffs of the other fifteen changed runtime files and both test files; the diffs of `README.md`, `AGENTS.md`, `.github/workflows/runtime.yml`, `framework/bootstrap/ductus.md`, `framework/bootstrap/ductus-procedure.md`, the five changed `framework/commands/*.md`, and 019's and 022's changed artifacts; and all eleven `framework/rules/*.md` in full (`configuration-cross.md` and `quality-cross.md` by this pass, the other nine split between the two reviewers). **Not read** (23): the ten generated mirrors under `.claude/commands/ductus/` and `.pi/prompts/` (`scripts/gen-claude-commands.sh --check` in sync); `framework/bootstrap/govern.md` (`cmp` identical to `ductus.md`); and twelve plan-affected files this change does not touch, which 063's review of `2bfdfa4e` read: `docs/analyze.md`, `framework/bootstrap/configure/claude.md`, `framework/runtime-tools.txt`, `runtime/CHANGELOG.md`, `runtime/src/interpreter/analyze_tally.rs`, `runtime/src/interpreter/mod.rs`, `runtime/src/main.rs`, `runtime/src/primitives/mod.rs` (only `is_line_hazard` and `flatten_line` were read), `runtime/src/schema/registry.rs`, `runtime/tests/golden/plan-basic.jsonl`, `runtime/tests/walker.rs`, and 058's `only-unreadable-targets-retain-decisions.md`. **Findings: none.** `CFG-ENV-012` is met by its warning branch; `BE-COMPAT-001` fires on dashboard's new `config.unknown-keys` and 019's plan classifies it; `BE-INPUT-011` is met by showing each key quoted and escaped. **Observations: 19.** Fourteen were fixed in the run. The gravest: `warn_unknown_keys` wrote with `eprintln!`, which panics on a broken stderr pipe, so a config with one unknown key failed every primitive that read it. That was reproduced through the binary (exit 101); the write's error is now dropped, and a binary test covers it. Also fixed: the lock is held only for the insert; a tab or a bidirectional control in a shown key is escaped; every stderr reader has a binary test; the installer test checks entry fields and owned sections; and the docs say where each report goes, over MCP included. Nine new mutations each fail a test, beside the 22 run before `e7044738`. Four observations are discarded with reasons, one carried from 022's standing set, and one matched the stored spent-task-history discard. Twenty-five stored decisions expire: their files are outside this scope, or, for 019's task 1, the routed work has landed.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs` — **discarded**: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
- bug: warn_unknown_keys wrote with eprintln!, which panics when stderr is a pipe whose reader has gone, so a config with one unknown key failed every primitive that read it — `runtime/src/schema/config_keys.rs` — **fixed**
- perf: the warned-line lock was held across the config parse and the stderr write — `runtime/src/schema/config_keys.rs` — **fixed**
- other: 019's plan named no synchronization for the process-wide warned set (BE-RACE-001, BE-RACE-003) — `specs/019-config-decisions/plan.md` — **fixed**
- other: README and the installer said every command reading a section names its unknown keys as it runs, which over MCP is false for the stderr readers — `README.md` — **fixed**
- other: 022's scenario and 019's data model called the stderr report the same line, though it is prefixed and names the file by the path read — `specs/022-deterministic-runtime/scenarios/config-readers-name-unknown-keys.md` — **fixed**
- other: a registered service checkout's [paths] is read through Paths::load, so its unknown keys reach this project's stderr with nothing saying so — `specs/022-deterministic-runtime/scenarios/config-readers-name-unknown-keys.md` — **fixed**
- other: dashboard names a key both on stderr and in its callout in one CLI run, which nothing said was by design — `specs/022-deterministic-runtime/scenarios/config-readers-name-unknown-keys.md` — **fixed**
- other: status.md's markdown-only rule pointed at a path adopters do not have and skipped [workflows] and the legacy [host] cli-config-dir — `framework/commands/status.md` — **fixed**
- other: the installer test checked neither the array-of-tables entry fields nor that every documented section is owned — `runtime/src/schema/config_keys.rs` — **fixed**
- other: a LazyLock wrapped a Mutex whose constructor is const — `runtime/src/schema/config_keys.rs` — **fixed**
- other: dashboard's unknown-key parse carried an unwrap_or_default no input could reach — `runtime/src/primitives/dashboard.rs` — **fixed**
- security: a tab or a bidirectional control in a shown key was printed raw, so a tab passed for a space and an override could reorder the line — `runtime/src/schema/config_keys.rs` — **fixed**
- other: only the [paths] and [services] stderr warnings had a test — `runtime/tests/config_unknown_keys.rs` — **fixed**
- other: the docs said every reader reports an unparseable config, while derive-references and check-orphaned-references read one as empty — `runtime/src/schema/config_keys.rs` — **fixed**
- perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs` — **discarded**: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
- other: the pre-commit hook runs three primitives that read [paths], so one typo prints three identical stderr lines per commit — `framework/bootstrap/hooks/ductus-pre-commit` — **discarded**: once per process is the contract, and each hook step is a separate run reading the file
- other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs` — **discarded**: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement

## Skipped passes

*None.*

## Unexamined governance

*None.*
