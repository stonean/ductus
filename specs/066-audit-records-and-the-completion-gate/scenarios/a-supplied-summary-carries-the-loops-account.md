---
section: "The review record"
---

# A-supplied-summary-carries-the-loops-account

## Context

[078 — Review fix rounds are re-read within a bound](../../078-review-fix-rounds-are-re-read-within-a-bound/spec.md) requires that a round not re-read because every path is exempt be named in the record's Summary (its AC2), and that the Summary name each waived round. `write-review` rendered that account only in the Summary it generated: a Summary the caller supplied was returned as given (`runtime/src/primitives/write_review.rs`, `report_summary`), and a real review always supplies one, since the review reference's **Window and coverage** section requires it to say what the passes did not read. So the account reached the record only when the host remembered to write it, which §design-principles rules out. 078's first review (2026-11-04) found it, and the owner routed it here (2026-11-04), as the spec owning the audit records; 078's task 16 delivered it (owner, 2026-11-09).

## Behavior

- **The loop's account follows the Summary whoever wrote it.** When the run passes `fix-rounds`, `write-review` appends the account it renders today — the rounds run, the re-reads finished, the rounds diff-checked, each exempt round, each waived one and each un-re-read one without a waiver, and a budget stop — to a Summary the caller supplies, as it does to the one it generates.
- **A run that passes no `fix-rounds` renders a supplied Summary exactly as given**, as today.
- **078's data model says the Summary, not the generated Summary, renders the account.**

## Edge Cases

- **A supplied Summary that already names the rounds.** The account is still appended: the record's own derivation is the one a reader can trust, and a duplicate sentence costs nothing a gate reads.
- **A supplied Summary ending without a full stop.** The account is joined after a space, as on the generated one.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
