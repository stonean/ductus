---
spec: 017-derive-dont-ask
reviewed-at: 2026-08-16T13:52:53Z
reviewed-against: 090ab0258fbbe93ce2e6044b4e0cde43a499d716
diff-base: 096dbc0cf65a2322c91bfa895a825ea60c5a23f8
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 017-derive-dont-ask

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

Re-run after this spec took the `done → in-progress` back-edge to record the outcome of the assessment its `generator-sync-claim-honesty` scenario specifies. The previous verdict dated from 2026-08-02 and had gone stale on four durable contracts; three of those four changed during the 0.28.0 cycle, while the spec sat at `done`. Worth noting for its own sake: the staleness gate fires on the `done` transition, so a spec already at `done` is not re-checked when its contracts move underneath it — this review is the first thing to look at 017's subject since the release.

Scope resolved to 545 files (the modified-since set, larger than the plan's 43 Affected Files), of which 90 are code — ~46.4k lines. That surface is the same one reviewed against 013 at this HEAD plus exactly one file, `runtime/src/primitives/prune_tasks.rs`, which was examined here and is clean: `validate_no_traversal` guards its feature argument, every missing precondition has its own error variant rather than a silent pass, and the `--reset` status gate returns a distinct `PruneGate::BlockedNeedsForce` variant instead of quietly declining — the QUAL-CLAIM-001 compliant shape — with 11 unit tests.

**Three findings were raised in this window and all three are fixed**, so the counts state what is outstanding rather than what was found; each carries a Status line naming the commit. All three sit in 017's scope and two of them squarely in its plan's Affected Files (`scripts/gen-help-tables.sh`, `scripts/gen-spec-deps.sh`, `.githooks/pre-commit`) — the generator machinery this spec owns. The first is the one this spec's own scenario had specified and left unanswered.

Rust posture verified rather than assumed: `unsafe_code = "forbid"`, clippy `all`+`pedantic` with `unwrap_used`/`expect_used` warned and CI promoting warnings to errors, `cargo clippy --release --all-targets --locked -- -D warnings` clean, 972 tests green, no `todo!`/`unimplemented!`/TODO/FIXME under `runtime/src` (QUAL-STUB-001 clean). 4 of the 11 loaded rule files declare themselves design-time/analyze-enforced rather than code-pass rules; AGENTS.md carries no `Code Style` or `Testing` section, so two of the four AGENTS.md inputs the command names do not exist here.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None outstanding.* All three findings below were fixed in-window.

### SHOULD: QUAL-CLAIM-001 — "help.md in sync" was asserted over a hardcoded command list

- **File**: `scripts/gen-help-tables.sh:190-193`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both.
- **Finding**: This is the question [`generator-sync-claim-honesty`](scenarios/generator-sync-claim-honesty.md) requires be asked of this generator, asked at last. The tables were built from a command list hardcoded in the script, so a command present under `framework/commands/` but unlisted was never examined while the run still reported `No changes (help.md in sync)` at exit 0 — the same shape the rule's Source note documents for `gen-spec-deps.sh`'s `git ls-files` scoping. Nothing else covered it: `help.md` appears in `scripts/audit/` only in a prose comment in `installer-command-parity.sh`, whose subject is `ductus.md`'s installer manifest and whose header concedes help.md merely "tends to get updated" — the author-diligence dependency the framework forbids.
- **Auto-fixable**: no
- **Status**: fixed in `49a14d3`. Reproduced first — a scratch command file left the generator printing `No changes (help.md in sync)` at exit 0 while help.md never mentioned it. The command groups are arrays now, feeding both the rendered tables and a coverage assertion against the directory (minus the same maintainer-only exclusion `installer-command-parity.sh` uses); the same probe now exits 6 naming the command and the remedy, and output on a clean tree is byte-identical. The message names its subject: `No changes (14 command(s) in sync)`. `check-zero` runs this generator, so `/ductus:audit` and the release gate inherit the check. The scenario and task 35 now record the outcome (`090ab02`), and `gen-configure-mcp.sh` — assessed in the same pass — is sound and deliberately unchanged.

### SHOULD: QUAL-CLAIM-001 — green CI never examined the shell surface

- **File**: `.github/workflows/framework-checks.yml:35-70`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both.
- **Finding**: Nothing in the repo invoked shellcheck, though a tuned `.shellcheckrc` was committed. This spec's own machinery is disproportionately shell — `gen-spec-deps.sh`, `gen-help-tables.sh`, `gen-readme-table.sh`, `install-hooks.sh`, `.githooks/pre-commit` and the shipped adopter hook are all in its plan's Affected Files — so a green `framework-checks` run could not distinguish "017's generators are clean" from "they were never examined", while the Rust half was gated at `clippy -D warnings` on three OSes.
- **Auto-fixable**: no
- **Status**: fixed in `49a14d3`. Adds step (j) covering 42 tracked scripts, asserting shellcheck is present and the enumeration non-empty *before* linting so the step cannot pass by examining nothing, and avoiding `mapfile` so it runs verbatim on bash 3.2. Verified clean across all 42, and exit 1 on an introduced SC2034.

### SHOULD: QUAL-SIMPLICITY — run_gen set GEN_STDOUT, which no test read

- **File**: `scripts/tests/test-gen-spec-deps.sh:341`
- **Rule**: Identify overengineering: premature abstraction, unnecessary indirection, configuration that could be a constant, branches that are dead under the current spec.
- **Finding**: `run_gen` captured the generator's stdout into `GEN_STDOUT` alongside `GEN_STDERR` and `GEN_RC`, but no test read it (shellcheck SC2034) — a dead assignment rather than a hollow test, since the assertions that matter did run.
- **Auto-fixable**: yes
- **Status**: fixed in `49a14d3` — by adding the missing assertions rather than dropping the capture. Investigating it surfaced the larger gap, which belongs to this spec: the zero-rewrite reporting `generator-sync-claim-honesty` exists to specify (`N tracked; D drifted; M untracked skipped`) was asserted nowhere, so a regression to a bare "all specs in sync" would have gone unnoticed. New test R covers all three clauses and their omission; verified by reverting the message to its pre-scenario form, which fails three of its assertions.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- Architectural exploration: re-frame the runtime's LLM extension points as named Skills loaded at the seam. Speculative; **On hold per user 2026-07-11.**

## Skipped passes

*None.*
