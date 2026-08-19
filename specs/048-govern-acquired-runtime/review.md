---
spec: 048-govern-acquired-runtime
scenario: state-a-version-checks-the-pin
reviewed-at: 2026-08-19T16:29:28Z
reviewed-against: 171d50249565cb97da31134ee68c959ef1c1f5ce
diff-base: e63cd5f7a77ad44ab749478c1b0cc8905d1bcdfb
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 048-govern-acquired-runtime

## Summary

Clean at `171d502` — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: `framework/bootstrap/ductus.md`, its byte-identical `govern.md` mirror, `framework/bootstrap/hooks/ductus-pre-commit`, the new scenario, and task 17.

**The defect was found in a real adopter project, which is the only test of composition this project has.** Detection resolved State A on tool-inventory introspection alone; State A then declared the runtime live and emitted nothing; and §Runtime acquisition — the only place `{pin}` is compared against anything — runs only in State B. A registered-but-old runtime therefore passed silently on every run. The adopter's store held `0.29.10` against a pin of `0.31.0`, so the pre-commit hook that same `/ductus` run refreshed called primitives that binary does not carry, with the shell generators they replaced already deleted, and every commit halted.

`QUAL-CLAIM-001` is the rule, one level up from code: **`/ductus` reported success.** A tool *was* in the inventory, so detection answered truthfully — it answered the wrong question. That is the same failure mode as a check that cannot run wearing the costume of one that passed, and it is why the fix is a version comparison rather than a better message.

**Two fixes, at different distances from the failure.** State A now probes the resolved binary and branches three ways — silent on a match, Branch 1's existing warning for a project-supplied `[runtime] path` whose owner chose it deliberately, and otherwise acquire `{pin}`. The non-obvious clause is what follows an acquisition: re-acquiring does **not** refresh the running MCP server, which was spawned once at session start and holds the old binary regardless of what is now in the store, so the run switches to `{pointer-path} <primitive>` for the remainder. Continuing to call the live tools would execute the stale code the check just diagnosed. That is the move State B already makes, for the same reason, so this adds no new mechanism.

**The hook guard exists because the State A fix cannot rescue anyone already broken** — reaching it requires running `/ductus`, and a project whose commits are halting may not get that far. It probes *capability* rather than parsing a version, which is the stronger form here: there is no pin for the hook to drift from, and a hand-placed binary or a primitive this hook grows later is covered by the same question. Proven against a runtime lacking the subcommands (halts naming the installed version and the missing subcommand) and against a current one (silent), in isolated fixtures.

**Two existing families were the risk and both were checked directly rather than assumed.** Family 22 exercises the shipped hook in an adopter-shaped tree — its stub exits 0 for every argument, so the capability probe passes, and its assertions are substring matches, so the two extra `--help` invocations do not disturb them; it is green. Family 21 confirms `govern.md` is byte-identical, which is what keeps a fix from reaching only the adopters who can already reach the fixed procedure.

**This is the mirror of the greenfield defect fixed immediately before it.** That one blocked adopters with *no* runtime; this one silently degraded adopters with the *wrong* one. Between them, acquisition was correct only for the adopter who was already current — worth recording, because two defects that look unrelated shared one cause: `{pin}` was consulted in exactly one branch of a two-branch decision.

Verified against the whole CI surface: markdownlint, six `lint-*` scripts, both `scripts/tests` suites, shellcheck over every tracked shell file (including the edited hook), actionlint, all three generators plus both derive primitives with a clean tree after, `scripts/audit/run-all.sh` re-run after committing, and under `runtime/` `cargo fmt --check`, `clippy -D warnings`, and `cargo test --release --locked`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
