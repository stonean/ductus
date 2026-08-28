---
spec: 020-code-review
reviewed-at: 2026-08-28T00:11:31Z
reviewed-against: ca473fd324c763af21dc8ff5ee0d7220b490fb6c
diff-base: 7e98cc48963acaad87b9c2d86071bc8d5eaa5c27
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 020-code-review

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes; not blocking.

**The `--since` override this spec's previous review needed is no longer required.** That run had to be given `--since=<base>~1` by hand, because 020's back-edge flip and task 12's work landed in one commit and the default base excluded the whole subject. `compute-review-scope` now bases the window on the transition commit's parent, so this run resolved to `7e98cc4` unaided and covers `check_command_flags.rs` and `command-flag-hint-parity.sh` — the deliverables — without an operator noticing anything.

Window covers `framework/commands/review.md`'s `$ARGUMENTS` parse step and corrected `argument-hint`, the `check-command-flags` primitive, `/audit` Family 30's entry-point script, the three-way family registration, and this round's `argument_hint` refactor onto `split_frontmatter`.

Security: the primitive reads a fixed repo-relative directory with no caller-supplied path; the family script interpolates nothing into a shell word, passing the runtime's JSON to `python3` on stdin with every `emit` argument quoted. Quality: the byte scanner's slice bounds hold on non-ASCII input, the finding loop is fed by a here-doc so `drift` survives in the current shell, and delegating frontmatter extraction to `split_frontmatter` closed two cases the hand-rolled scan mishandled — an empty `---\n---\n` block and a CRLF opener — each now covered by a test. Reuse: the duplication this spec's prior review recorded as an observation is resolved, so nothing remains to carry. Efficiency and simplicity found nothing against the loaded rules.

AC14 asserts the behavior: the parse step covers every flag in the Flags table, `argument-hint` names each of them, both operator-error branches are specified, and Family 30 holds the two in agreement so a flag added later cannot silently reopen the gap.

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
