---
spec: 013-text-first-artifacts
reviewed-at: 2026-08-16T13:09:52Z
reviewed-against: 49a14d3c43cc0aa4a231c2c0fd40e14fb5ef6894
diff-base: d924627ca6c3f4478a40e3bcdaf8b4d608a835c9
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 013-text-first-artifacts

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

Three findings were raised during this review and **all three were fixed before it was finalised**, so the counts above state what is outstanding, not what was found. Each is recorded below with a **Status** line naming the commit that closed it, per the report's reconciliation rule. Every fix was verified by breaking the thing it guards and watching it go red — a gate seen only passing is not yet evidence (AGENTS.md §Design Principles).

Scope resolved to 532 files (the modified-since set, larger than the plan's 35 Affected Files), of which 89 are code — 51 Rust, 34 shell, 4 hooks, ~45.6k lines. The breadth is an artifact of 013 reopening on 2026-08-14 immediately before the whole 0.28.0 release landed; the remaining 398 markdown artifacts are `/ductus:analyze`'s subject, not the five code passes'. 4 of the 11 loaded rule files (concurrency-, observability-, performance-backend, reliability-backend) declare themselves design-time/analyze-enforced rather than code-pass rules. AGENTS.md carries no `Code Style` or `Testing` section, so two of the four AGENTS.md inputs the command names do not exist here.

Rust posture is strong and was verified, not assumed: `unsafe_code = "forbid"`, clippy `all`+`pedantic` with `unwrap_used`/`expect_used` warned and CI promoting all warnings to errors, `cargo clippy --release --all-targets --locked -- -D warnings` clean, 972 tests green, and no `todo!`/`unimplemented!`/TODO/FIXME under `runtime/src` (QUAL-STUB-001 clean). Two candidate findings were investigated and dismissed on evidence: `fetch-archive` resolving its `archive` argument through `resolve_path` without `validate_no_traversal` is documented deliberate design (`primitives/mod.rs:711-723` names downloaded archives as operator/machine-local paths that must accept absolute input), and the SSRF guard is thorough — https-only, internal-range denial incl. the metadata endpoint, IPv4-mapped unwrapping, per-hop redirect re-validation, DNS-rebinding address pinning. `install.sh` is clean (TLS-pinned curl, allowlisted agent arm with an explicit reject, quoted expansions).

One defect with no mapping to any loaded rule is logged to `specs/inbox.md` rather than invented as a rule violation, and remains **outstanding**: a live `GVRN_`-prefixed env var survives the 049 rename in `runtime/src/primitives/fetch_archive.rs:278`, which makes 049's AC1 read as met when it is not. Closing it is a decision (rename, dual-read, or recorded exception), not a mechanical fix.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None outstanding.* All three findings below were fixed in-window.

### SHOULD: QUAL-CLAIM-001 — "help.md in sync" was asserted over a hardcoded command list

- **File**: `scripts/gen-help-tables.sh:190-193`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both.
- **Finding**: The generator built its five tables from a command list hardcoded in the script, then compared the rendered help.md against the file and reported `No changes (help.md in sync)` at exit 0. A command present under `framework/commands/` but absent from that list was invisible to the very message claiming sync — the same shape the rule's own Source note documents for `gen-spec-deps.sh`'s `git ls-files` scoping. No audit family covered it: `help.md` appears in `scripts/audit/` exactly once, in a prose comment in `installer-command-parity.sh`, whose subject is `ductus.md`'s installer manifest; that family's header notes help.md merely "tends to get updated", which is the author-diligence dependency AGENTS.md's second Design Principle forbids.
- **Auto-fixable**: no
- **Status**: fixed in `49a14d3`. Reproduced first — a scratch command file left the generator printing `No changes (help.md in sync)` at exit 0 while help.md never mentioned it. The groups are arrays now, feeding both the rendered tables and a coverage assertion against the directory (minus the same maintainer-only exclusion `installer-command-parity.sh` uses), and the message names its subject: `No changes (14 command(s) in sync)`. The same probe now exits 6 naming the command and the remedy; output on a clean tree is byte-identical. `check-zero` runs this generator, so `/ductus:audit` and the release gate inherit the check. This also completes the assessment [017's `generator-sync-claim-honesty`](../017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md) requires of this script and never recorded; `gen-configure-mcp.sh` was assessed in the same pass and is sound.

### SHOULD: QUAL-CLAIM-001 — green CI never examined the shell surface

- **File**: `.github/workflows/framework-checks.yml:35-70`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both.
- **Finding**: Nothing anywhere in the repo invoked shellcheck — verified by grep across `.github/`, `scripts/`, `.githooks/` and `framework/` — yet a tuned `.shellcheckrc` is committed (and is itself in this review's scope), configuring `external-sources` and `source-path` for scripts that were never linted. The Rust half of the codebase is gated at `clippy -D warnings` on three OSes while 42 shell scripts, including every audit family that constitutes the release gate, were gated by nothing, so a green `framework-checks` run did not distinguish "the shell scripts are clean" from "the shell scripts were never examined". Not theoretical: running shellcheck surfaced a real hit, and AGENTS.md's first Design Principle records that `sibling-coupling.sh` shipped a GNU-awk extension that silently disabled a release gate on every macOS machine.
- **Auto-fixable**: no
- **Status**: fixed in `49a14d3`. Adds step (j) to `framework-checks.yml`, covering 42 tracked scripts. The step asserts shellcheck is present and the enumeration non-empty *before* linting, so it cannot pass by examining nothing — the same false green it exists to close. It avoids `mapfile` so a contributor on bash 3.2 can run it verbatim before pushing. Verified both ways: clean across all 42 files, and exit 1 on an introduced SC2034.

### SHOULD: QUAL-SIMPLICITY — run_gen set GEN_STDOUT, which no test read

- **File**: `scripts/tests/test-gen-spec-deps.sh:341`
- **Rule**: Identify overengineering: premature abstraction, unnecessary indirection, configuration that could be a constant, branches that are dead under the current spec.
- **Finding**: The `run_gen` helper captured the generator's stdout into `GEN_STDOUT` alongside `GEN_STDERR` and `GEN_RC`, but no test read it (shellcheck SC2034). The assertions that matter did run, so no check was silently missing — a dead assignment rather than a hollow test.
- **Auto-fixable**: yes
- **Status**: fixed in `49a14d3` — by adding the missing assertions rather than dropping the capture. Investigating it surfaced the larger gap: the zero-rewrite reporting that [017's `generator-sync-claim-honesty`](../017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md) exists to specify (`N tracked; D drifted; M untracked skipped`) was asserted nowhere, so a regression to a bare "all specs in sync" would have gone unnoticed. New test R covers all three clauses and their omission; `run_gen` takes pass-through args for the `--staged` leg. Verified by reverting the message to its pre-scenario form, which fails 3 of the new assertions.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- Architectural exploration: re-frame the runtime's LLM extension points as named Skills loaded at the seam. Speculative; **On hold per user 2026-07-11.**
- Stale old-project-name env var survives the 049 rename: `GVRN_FETCH_ALLOW_INSECURE_HOSTS` in `runtime/src/primitives/fetch_archive.rs:278`. Outstanding — needs a decision (rename, dual-read, or record as a deliberate exception on 049).

## Skipped passes

*None.*
