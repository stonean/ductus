---
spec: 013-text-first-artifacts
reviewed-at: 2026-08-16T12:53:08Z
reviewed-against: c24f40e6b870ff46ef399f6ab6a85f8e0724d60c
diff-base: d924627ca6c3f4478a40e3bcdaf8b4d608a835c9
must-violations: 0
should-violations: 3
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 013-text-first-artifacts

## Summary

Scope resolved to 532 files (the modified-since set, which is larger than the plan's 35 Affected Files), of which 89 are code — 51 Rust, 34 shell, 4 hooks, ~45.6k lines. The breadth is an artifact of 013 reopening on 2026-08-14 immediately before the whole 0.28.0 release landed; the remaining 398 markdown artifacts are `/ductus:analyze`'s subject, not the five code passes'. 4 of the 11 loaded rule files (concurrency-, observability-, performance-backend, reliability-backend) declare themselves design-time/analyze-enforced rather than code-pass rules, so the code passes ran against quality-cross, the two security files, api-backend, configuration-cross and the two frontend files. AGENTS.md carries no `Code Style` or `Testing` section, so two of the four AGENTS.md inputs the command names do not exist here.

Rust posture is strong and was verified, not assumed: `unsafe_code = "forbid"`, clippy `all`+`pedantic` with `unwrap_used`/`expect_used` warned and CI promoting all warnings to errors, `cargo clippy --release --all-targets --locked -- -D warnings` run locally clean, and no `todo!`/`unimplemented!`/TODO/FIXME anywhere under `runtime/src` (QUAL-STUB-001 clean). Two candidate findings were investigated and dismissed on evidence: `fetch-archive` resolving its `archive` argument through `resolve_path` without `validate_no_traversal` is a documented deliberate design (`primitives/mod.rs:711-723` names downloaded archives as operator/machine-local paths that must accept absolute input), and `fetch_archive.rs`'s SSRF guard is thorough — https-only, internal-range denial incl. the metadata endpoint, IPv4-mapped unwrapping, per-hop redirect re-validation, and DNS-rebinding address pinning. `install.sh` is clean (TLS-pinned curl, allowlisted agent arm with an explicit reject, quoted expansions).

3 SHOULD findings, 0 MUST, 0 low-confidence. Separately, one defect with no mapping to any loaded rule was logged to `specs/inbox.md` rather than invented as a rule violation: a live `GVRN_`-prefixed env var survives the 049 rename in `runtime/src/primitives/fetch_archive.rs:278`, which makes 049's AC1 read as met when it is not.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — "help.md in sync" is asserted over a hardcoded command list, not the command set

- **File**: `scripts/gen-help-tables.sh:190-193`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both.
- **Finding**: The generator builds its five tables from a command list hardcoded at lines 91-155, then compares the rendered help.md against the file and reports `No changes (help.md in sync)` with exit 0. A command present under framework/commands/ but absent from that hardcoded list is invisible to the very message claiming sync — the same shape the rule's own Source note documents for gen-spec-deps.sh's git-ls-files scoping. Verified empirically: adding a scratch command file left the generator reporting `No changes (help.md in sync)` at exit 0 while help.md contained no mention of it. No audit family covers this — help.md appears in scripts/audit/ exactly once, in a prose comment in installer-command-parity.sh, whose subject is ductus.md's installer manifest rather than help.md; that family's own header notes help.md merely "tends to get updated", which is the author-diligence dependency AGENTS.md's second Design Principle forbids.
- **Auto-fixable**: no
- **Suggested fix**: Either derive the command list from framework/commands/*.md (with an explicit exclusion list, as installer-command-parity.sh already does for maintainer-only commands) so an unlisted command cannot exist, or add an audit family asserting help.md's command set equals framework/commands/*.md minus exclusions. Failing both, quantify the claim — `No changes (N commands in sync)` — so the message states what it examined.

### SHOULD: QUAL-CLAIM-001 — green CI never examines the shell surface — 34 in-scope scripts have no static analysis

- **File**: `.github/workflows/framework-checks.yml:35-70`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both.
- **Finding**: framework-checks runs markdownlint, four bespoke lints, procedure parseability and the self-audit; generators.yml runs the generators and their tests; runtime.yml runs fmt and clippy -D warnings on three OSes. Nothing anywhere in the repo invokes shellcheck — verified by grep across .github/, scripts/, .githooks/ and framework/ — yet a tuned .shellcheckrc is committed (and is itself in this review's scope), configuring external-sources and source-path for scripts that are never linted. So the Rust half of the codebase is gated at -D warnings while 34 shell scripts, including every audit family that constitutes the release gate, are gated by nothing, and a green framework-checks run does not distinguish "the shell scripts are clean" from "the shell scripts were never examined". The gap is not theoretical: running shellcheck now surfaces a real (if minor) hit, and AGENTS.md's first Design Principle records that sibling-coupling.sh shipped a GNU-awk extension that silently disabled a release gate on every macOS machine.
- **Auto-fixable**: no
- **Suggested fix**: Add a shellcheck step to framework-checks.yml covering scripts/**/*.sh, .ductus/scripts/**/*.sh, install.sh, .githooks/pre-commit and framework/bootstrap/hooks/*, at -S warning to start. The committed .shellcheckrc already carries the right configuration, so the job is a single step.

### SHOULD: QUAL-SIMPLICITY — run_gen sets GEN_STDOUT, which no test ever reads

- **File**: `scripts/tests/test-gen-spec-deps.sh:341`
- **Rule**: Identify overengineering: premature abstraction, unnecessary indirection, configuration that could be a constant, branches that are dead under the current spec.
- **Finding**: The run_gen helper captures the generator's stdout into GEN_STDOUT alongside GEN_STDERR and GEN_RC, but no test in the file reads GEN_STDOUT — shellcheck SC2034. The assertions that matter do run (tests read GEN_RC and GEN_STDERR), so no check is silently missing; this is a dead assignment rather than a hollow test.
- **Auto-fixable**: yes
- **Suggested fix**: Drop the GEN_STDOUT capture and its mention in the helper's `Sets globals:` comment, or add the missing stdout assertion if one was intended.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- Architectural exploration: re-frame the runtime's LLM extension points as named Skills loaded at the seam. Speculative; **On hold per user 2026-07-11.**

## Skipped passes

*None.*
