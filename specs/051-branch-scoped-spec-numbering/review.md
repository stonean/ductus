---
spec: 051-branch-scoped-spec-numbering
reviewed-at: 2026-08-30T01:55:08Z
reviewed-against: b985b9fff44c1b72af7df31455c224013d53f73d
diff-base: eb52e24a8bf9ce362430abe583d2ef63dd9bafc9
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 051-branch-scoped-spec-numbering

## Summary

0 MUST, 0 SHOULD, 0 low-confidence; not blocking. No captured issues outstanding.

**The quality pass found one regression, in this window's own work.** `spec_corpus` — the helper task 24 added so audit families stop globbing the corpus — returned nothing both when `dashboard` could not be read and when a project genuinely has no specs, and both callers tested emptiness. A fresh adopter, or any repository before its first spec, would have collected a precondition finding from two families for the offence of being empty. Fixed in `b985b9f` by reading the exit status, which already distinguished the two: the JSON parse fails only for an unusable dashboard, while a zero-spec corpus parses fine and yields no rows.

Worth naming precisely because it is `QUAL-CLAIM-001` inverted. That rule guards a clean result that overstates what was examined, and the reflex it trains — an empty result is suspicious — produced the opposite error here: a finding asserting a failure that never happened. Both come from one cause, a result whose shape cannot distinguish two states, and the remedy is the same in both directions.

**Task 24's substance holds, and it is larger than the defect that prompted it.** The reported gap was five shell surfaces carrying the three-digit rule. Writing the scenario as a property rather than a list surfaced the more serious half: a digits-only pattern does not merely skip a spec past 999, it rejects `1234.1-staged` outright, so the acceptance criteria of **every branch-scoped spec** went unlabelled — this feature's own directory form, broken since it shipped and live rather than latent. Fixing the five sites as catalogued would have left it.

**The three surfaces that keep a pattern each have a reason recorded.** Both pre-commit hooks run before any binary is resolvable; `lint-frontmatter.sh` exists to find malformed frontmatter and so cannot ask a frontmatter-parsing primitive for its corpus. Those copies are held to `parse_feature_dir` by a runtime test rather than by a reader, and the property is one-sided on purpose — the pattern must accept everything the grammar accepts, since a false negative is a spec silently dropped while a false positive is a path a primitive resolves and declines. The two places the pattern is knowingly looser are pinned so they read as decisions.

**The reuse pass approves of the direction rather than flagging it.** The two audit families that could ask the runtime now do, which retired their hand-rolled awk `status:` scans along with their globs — the shape `AGENTS.md` names as a design failure, a further frontmatter parser in a repo whose runtime parses frontmatter for a living. `ductus_bin` and `spec_corpus` moved to `lib.sh` because a fourth copy of the three-tier fallback is how those tiers drift. `sibling-coupling`'s `a_slug` derivation was widened along with the rest; it remains self-consistent, since the suppression grep and the suggested wording both read the same value.

Security, efficiency and simplicity found nothing against the loaded rules. Every guard added in this window was confirmed against the regression it guards rather than only observed to pass: reverting either hook fails the runtime agreement test by name, reverting the shipped hook fails Family 22's two new cases, and reverting either line-ending fix fails its behavioral case. A check that has never been seen to fail is not yet known to work.

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
