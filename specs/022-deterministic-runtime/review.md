---
spec: 022-deterministic-runtime
scenario: review-gate-unexaminable-contracts
reviewed-at: 2026-08-27T15:57:13Z
reviewed-against: 6ef37297f51f6675297ae36ebf77d27a10210102
diff-base: a4df343d01d56901d79bb82476ec69c9e47a0126
must-violations: 0
should-violations: 1
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviewed at `6ef3729`, the commit carrying the work — committed before reviewing this time, so `reviewed-against` names the code the verdict describes rather than the commit before it. `--since=a4df343` gives a `modified-since` of exactly the seven committed files.

Scope: `runtime/src/primitives/check_review_gate.rs` (new `unexaminable_contracts_guidance`, two doc-comment miscounts corrected, six new tests), `framework/commands/implement.md` and its generated mirror (the gate enumerations), `runtime/tests/golden/implement-basic.jsonl`, and the 022 spec artifacts.

0 MUST, 1 SHOULD. The SHOULD is `QUAL-CLAIM-001` against the new function itself: it returns `None` both when every durable contract is committed and when git cannot be queried, which is the same conflation of "examined and clean" with "could not examine" that this scenario exists to remove one level up. It is advisory and does not block; the codebase already accepts a documented fail-open of the same shape in `stale_review_block`, so closing it is a consistency call. It is recorded rather than waived because the reviewer did not decide it — see the finding for the two exits.

Security: no new attack surface. The function reads git status and writes nothing; there is no eval, no user-controlled input, and no path is constructed from anything but the validated feature slug. Reuse: correct — it shares `is_durable_contract` with `stale_review_block` rather than restating the scoping rule, and it follows the established `StatusOptions` + `pathspec` pattern already used in `primitives/mod.rs`, including the pathspec bound that keeps this off a full-worktree walk on every completion attempt. Quality: `path()` is handled as the `Result` git2 0.21 returns rather than assumed UTF-8, results are collected into a `BTreeSet` so the output is deduplicated and ordered, and the `+N more` tail matches `stale_review_block`'s existing truncation. Efficiency: one pathspec-bounded status walk. Simplicity: no new abstraction — a single function returning `Option<String>` into a field the result already had.

The golden change is mechanical and was verified as such: `implement-basic.jsonl` differs from its previous revision by exactly the two fixture git shas (`first-commit`, `current-head`) and nothing else, which is the expected consequence of editing `framework/commands/implement.md` — the parity harness copies that file into the fixture repo, so its tree hash moves. It was re-blessed, not hand-edited.

Verification: full suite green with `npx` resolvable — 1019 lib tests, parity 11/11, mcp 26/26, plus every other target; `cargo fmt` and `clippy --all-targets` clean; the full 29-family audit green; `markdownlint-cli2 '**/*.md'` clean. Behavior confirmed against this repository rather than fixtures alone: 022, carrying its uncommitted scenario, returns `passed: true` with guidance naming `scenarios/review-gate-unexaminable-contracts.md`; 023, fully committed, returns `passed: true` with no guidance.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — unexaminable_contracts_guidance returns None both when every contract is committed and when git cannot be queried

- **File**: `runtime/src/primitives/check_review_gate.rs:180-200`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — through a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: The new function chains `.ok()?` on `Repository::discover` and `statuses`, so a git failure returns `None` — the identical value it returns when every durable contract is genuinely committed. The gate then emits a bare `passed: true` and the caller reads it as "examined and current", which is precisely the shape this scenario was written to remove one level up. Largely unreachable in practice, since `stale_review_block` has already discovered the same repository immediately before, but it is reachable when `reviewed-against` is empty and that function returns before touching git.
- **Auto-fixable**: no
- **Suggested fix**: Distinguish the two outcomes — return a two-variant result (or a distinct guidance string naming that the working tree could not be inspected) so a caller can tell "nothing dirty" from "could not look". Advisory: the codebase already accepts a documented fail-open in `stale_review_block` (tested by `staleness_fails_open_on_an_unresolvable_sha`), so this is a consistency call rather than a defect.

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
