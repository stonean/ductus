---
spec: 026-framework-self-audit
reviewed-at: 2026-08-19T15:56:20Z
reviewed-against: 6d11a7d1baa4102684f11cce43f2ccca2c3dad6f
diff-base: 5e4f1892a6fbd7b1639b1f828a644a4086a9a1d8
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Clean at `6d11a7d` — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations. Both inbox items this spec opened are closed by the work rather than carried.

Scope: two new families (`done-spec-criteria.sh`, `audit-family-parity.sh`), their two scenarios, their registration in `run-all.sh` / `audit.md` / `README.md`, AC20–AC21, tasks 32–33, and two `AGENTS.md` entries.

**Family 27** reports a spec at `status: done` carrying an unchecked acceptance criterion. `QUAL-CLAIM-001` is the rule it embodies and the rule it obeys: status is parsed from the frontmatter block rather than grepped — `status: done` appears in prose across the corpus, including in this family's own scenario — the checkbox is read only inside the Acceptance Criteria section, fences are skipped, specs are enumerated from git with untracked ones skipped *and counted*, the examined `done`-spec count is reported on stderr, and an empty enumeration is a finding rather than the pass two empty sets would produce. The finding names both repairs because the family genuinely cannot tell which is right.

**Family 28** binds three registries that had drifted. Both sets are derived, never hardcoded — a hardcoded expectation would be a third copy of the fact under test — and compared as `(number, script)` pairs, so an entry naming the right family against the wrong script is caught from both directions. Retired numbers need no special case: Family 3 is spent and absent from both, so they agree.

**Its own first draft carried the defect it exists to prevent**, and that is worth recording rather than quietly fixing. Anchoring the extraction to `(Family N — …)` matched only families 14+; the older entries are bare `(Family 7).`, so the first run reported twelve *correct* entries as undocumented. A parity check whose findings are wrong is worse than none — it would have sent a maintainer rewriting a list that was already right. The script header carries that, and the both-spellings rule is now explicit.

**Neither family was trusted until it failed.** Family 27 was proven red against a seeded unchecked criterion on a `done` spec, a corpus with no `done` specs, and no tracked specs at all — and proven to stay *clean* on the four near-misses that would make it noise: a fenced checkbox, a prose `status: done`, a checkbox in a different section, and an unchecked criterion on an `in-progress` spec. Family 28 was proven red against each list losing a family in turn, a missing README entry, an empty derivation on either side, a right-number/wrong-script entry, and a missing subject file. Both proofs ran in isolated fixture repos, so the real tree was never mutated to test a check against it.

**Family 25 then caught an unbalanced `**` in the first new `AGENTS.md` entry** — a glob written inside an inline code span. It is behaving exactly as specified: it deliberately does not strip code spans, which is what makes the per-line check exact for its two files. The line was rewritten rather than the family loosened. That is the correct direction, and it is the second time this session an existing check earned its place.

Verified against the whole CI surface rather than the part that looked related — the miss that produced the previous round: `markdownlint-cli2` (442 files), six `scripts/lint-*.sh`, both `scripts/tests/*.sh` suites, `shellcheck -S warning` over every tracked shell file, `actionlint`, all three generators plus `derive-dependencies --write` / `derive-references --write` with a clean tree after, `scripts/audit/run-all.sh` re-run *after* committing, and under `runtime/`: `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings`, and `cargo test --release --locked` (1013 unit tests plus every integration and parity target).

`check-artifacts` also caught a real defect mid-implementation: `next-criterion` was left at 20 against a body whose highest label is AC21, so the next assignment would have reissued a live label. Corrected to 22 and re-verified clean.

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
