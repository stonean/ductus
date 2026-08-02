---
spec: 026-framework-self-audit
reviewed-at: 2026-08-02T02:29:53Z
reviewed-against: e7296cb9dc1d58c6c193d3a50f11a0abe4ea01ae
diff-base: 8928fcae982d9e5db2f53455a993b1693fb7e631
must-violations: 0
should-violations: 1
low-confidence: 0
captured-issues: 3
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

0 MUST, 1 SHOULD, 0 low-confidence across all five passes, against tasks 19 and 21 — the `report_step` purity refactor and the new Family 17 host-namespace-parity check. Not blocking.

Two findings surfaced during the run; one was fixed before this record was written and one is recorded. The fixed one: Family 17 collected installed namespaces into a space-joined string and re-split it with `for ns in $installed`, so a namespace directory containing a space would have split into bogus entries and the singular/plural count would have been wrong. Replaced with an indexed array (Bash 3.2 compatible), with behavior verified unchanged in both directions — clean when the namespaces agree, byte-identical finding when reverted to the pre-fix config. The recorded one is the QUAL-GROUND-001 finding below.

Task 19 verification went beyond the checkbox. Subtask 1 was already on disk from `00286b1` but left unchecked, so it was verified rather than re-done — and two deviations from the scenario text are now recorded rather than quietly absorbed: the helper is `audit_family NAME` + `emit LOCATION MESSAGE FIX` rather than the specified `audit_emit FAMILY LOCATION MESSAGE FIX`, and the extraction shortened family scripts by 6 lines each rather than the ~10 the done-when estimated (`ssot-invariants.sh` grew by 1). Subtask 2's `report_step` is now a pure function of six named arguments, with the walker assigning every field explicitly at the "open a new step" branch — that assignment, not the function signature alone, is what makes the purity hold. Parity was proven against a probe command file exercising a prose-only step, a primitive-bearing step, and an allowlisted step: byte-identical output and exit code.

Security: the new script's only external input is a TOML file parsed by python3 through a quoted heredoc with the path passed as `argv[1]`, never interpolated into the script body, so there is no injection surface; the remaining operations are directory globs and `basename`. Reuse: `report_step` and the `lib.sh` `emit` contract each exist once; Family 17's duplication of `Host::load` is the subject of the finding below rather than a separate reuse hit. Efficiency: one bounded directory walk per agent config dir. Simplicity: the `${#installed[@]}` count replaced a hand-rolled counting loop.

Deterministic corroboration at this HEAD: `bash scripts/audit/run-all.sh` exits 0, shellcheck clean across the new and modified scripts, `scripts/lint-procedure-parseability.sh` exits 0 against the new audit.md step, markdownlint clean over 365 files. Family 17's registration was proven rather than assumed — `run-all.sh` is silent on clean runs, so exit 0 alone cannot distinguish "passed" from "never ran"; reverting `.govern/config.toml` produced the Family 17 header and finding, and restoring it returned exit 0.

Recorded against a committed sha deliberately, following the same correction made during 022's review: the working tree is committed first so `reviewed-against` and the reviewed code agree, preserving the documented idempotency invariant.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-GROUND-001 — Family 17 re-implements four runtime-owned contracts as shell literals with no guard that fails when they change

- **File**: `scripts/audit/host-namespace-parity.sh:68-115`
- **Rule**: Code whose correctness depends on an external contract it does not own — a database schema, another service's API shape, a config key, a file or wire format — SHOULD bind to that contract in a way that fails loudly when the assumption is wrong (a typed or generated binding, a schema/migration reference, a startup or first-use validation, or a test that exercises the real shape) rather than silently encoding an unverified assumption.
- **Finding**: The family reproduces `Host::load`'s namespace resolution in bash, encoding four contracts it does not own: the `[host] project` key name and TOML shape (owned by `HostBlock` in runtime/src/host.rs), the `commands`/`command` subdirectory names (owned by `Host::command_file_candidates`), the four agent config-dir names (owned by the agent registry), and the new-wins config resolution order (owned by `schema::paths::config_display_name`). All four are literals with no generated binding, no reference to the Rust source, and no test exercising the real resolver. If the runtime gains a fifth agent directory, a third layout, or renames the key, this family keeps exiting 0 while silently checking the wrong thing — a drift-detector that has itself drifted, which is worse than no check because it reads as positive assurance.
- **Auto-fixable**: no
- **Suggested fix**: Preferred: expose the resolved namespace from the runtime (a `gvrn` subcommand or a primitive) and have the family consume it, so the shell and the Rust cannot diverge. Cheaper interim: register the four literals in Family 6's SSOT-tracked rule list so a change in `host.rs` surfaces as drift. Accepting as-is is also defensible for v1 — 026's own plan records the same trade-off for Family 6's hand-maintained list — but the acceptance should be explicit rather than implicit.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] **Release sequence for the 045/046 work** — 022 is now `done` and `gvrn-v0.25.0` is tagged and published, so the checklist's state is stale again and needs a refresh or retirement.
- [ ] `mark-task`'s checkbox-form done-when reconciliation is tick-only, never untick — decide whether that asymmetry should stand (logged during 022 task 71, with both candidate resolutions and a recommendation).
- [ ] Running `gen-spec-deps.sh` manually right after creating a spec reported "No changes (all specs in sync)" while `dependencies` stayed `[]` — `list_specs()` enumerates via `git ls-files`, so a brand-new spec is invisible until staged.

## Skipped passes

*None.*
