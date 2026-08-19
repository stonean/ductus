---
spec: 026-framework-self-audit
reviewed-at: 2026-08-19T15:38:58Z
reviewed-against: f6456b2aa988c29a91986b3c07f218688635cd82
diff-base: 0f0225f4f45aa6b34ed06c75dd6bf99ad81b2475
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Clean at `f6456b2` — 0 MUST, 0 SHOULD, 0 low-confidence. Two observations, both captured to the inbox.

This run reviews the close-out fix in `f6456b2`, not the whole window since `0f0225f` (which spans several specs' work that carried their own reviews). The change is three files: `framework/commands/audit.md`, its generated copy, and `specs/026-framework-self-audit/spec.md`.

**Family list restored to parity.** `run-all.sh` registers 25 families (1–2, 4–26); `audit.md` enumerated 1–2 and 4–23, so Families 24 (rename-sweep residue), 25 (unbalanced inline markup), and 26 (broken relative links) were absent in any form. The three missing were the three most recently added, which is the direction this drift always runs. Each new entry is written from its script's own header rather than paraphrased from the family name — what it asserts, the incident that motivated it, and what a clean exit does and does not mean, matching the surrounding entries' shape. Verified by comparing both extracted number sets, which now agree exactly.

**AC19 ticked.** Its work was complete and always had been: task 31 checked, `scripts/audit/broken-relative-links.sh` executable, registered at `run-all.sh:71`, listed in `scripts/audit/README.md`, exits 0. Only the checkbox lagged. It was ticked in place rather than through a `done → in-progress → done` cycle: §spec-lifecycle's back-edge fires on a *meaningful* edit — new scope, changed semantics — and this changes no requirement, it records that an existing one was met. Ticking criteria is what the completion gate itself does on the way to `done`, so this restores the state that gate should have left rather than creating a new one.

**`QUAL-CLAIM-001` is the rule both defects sit under**, one level up from code: each was a green signal that meant less than it appeared to. That is why neither is closed as a one-off — the two observations record the missing checks rather than the fixed instances, since the instances are cheap and the gaps will recur. Building either check is its own scope and is deliberately not done here.

Checks: `markdownlint-cli2` (440 files, 0 issues), `lint-procedure-parseability`, `lint-tool-coverage`, `lint-frontmatter`, `derive-dependencies` (no drift, 51 examined), `gen-help-tables --dry-run`, generated command copies re-rendered, and `scripts/audit/run-all.sh` — all clean, the audit re-run against the committed tree. Repo-wide, zero unchecked acceptance criteria and zero unchecked task checkboxes remain.

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

- other: the self-audit has no family asserting that a spec at `status: done` carries no unchecked acceptance criterion. 026 reached done in e9262df with AC19 unticked and every check stayed green — run-all.sh exits 0, check-artifacts reports the feature clean, CI passes. The completion gate is supposed to make this unreachable (it refuses the transition while any criterion is unchecked), so reaching it means the gate was bypassed or its marking step failed, and nothing downstream notices either way. — `scripts/audit/run-all.sh`
- other: nothing asserts parity between `/ductus:audit`'s enumerated family list in framework/commands/audit.md and the families run-all.sh actually registers. The list had stopped at 23 while run-all.sh ran 25 families, and the three missing were the most recently added — the direction this drift always goes. Families 18 and 23 already exist as list-vs-registry parity checks, so the shape is established; this registry just never got one. — `framework/commands/audit.md`

## Skipped passes

*None.*
