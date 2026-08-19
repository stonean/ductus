---
section: "Follow-on scenarios"
---

# Family-18-marker-list-parity

## Context

`criterion-path-existence`'s non-assertion marker list decides which findings
the family suppresses — a criterion carrying one of the phrases is exempted
whole, and each of its paths is recorded as `not-a-live-claim` instead of
flagged. The list is restated in four places, and three of them are load-bearing
rather than incidental:

- `specs/045-decision-state-drift-detection/data-model.md` — the canonical
  source, per the constitution's canonical-sources map;
- `runtime/src/primitives/check_artifacts.rs` — `NON_ASSERTION_MARKERS`, the
  implementation;
- `framework/commands/analyze.md` — the adopter-facing restatement. This one
  **cannot** be replaced by a pointer: `analyze.md` ships to adopter projects,
  which have no copy of 045's data-model;
- `specs/022-deterministic-runtime/scenarios/criterion-path-existence-family.md`
  — carries the count and the group names, not the phrases.

Nothing compared them. Adding three phrases under 022's
`criterion-non-assertion-phrasings` meant hand-editing all four, and a missed
one would have left a canonical source asserting behavior the runtime does not
have — the drift [§drift-prevention](../../../framework/constitution.md#drift-prevention)
exists to catch, inside the check built to catch it.

Surfaced as a `QUAL-GROUND-001` SHOULD by `/ductus:review` on 022 (2026-08-03).
Same shape as the finding that produced
[family-17-contract-binding](family-17-contract-binding.md), and closed the
same way: derive from the canonical source, fail closed when derivation yields
nothing.

## Behavior

`scripts/audit/marker-list-parity.sh` is Family 18, wired into
`scripts/audit/run-all.sh` after Family 17.

- **18a** derives the marker set from the table under `### The criterion must
  be a live claim` in 045's data-model. **A derivation yielding zero markers is
  a finding**, not a silent pass — without that arm the family would keep
  exiting 0 while checking nothing the moment the table moved, which reads as
  assurance rather than absence. Duplicate rows are also reported.
- **18b** parses `NON_ASSERTION_MARKERS` out of `check_artifacts.rs`, comparing
  as a set in both directions and asserting the declared array length matches
  the literal count. Comment lines are stripped first, so a phrase quoted in a
  `//` comment is never read as an entry.
- **18c** parses the shipped restatement in `analyze.md` and compares both ways.
- **18d** compares the spelled-out count word in all three markdown restatements
  against the derived size.

Two parsing conventions are load-bearing and documented in the script:

- A phrase whose **trailing space** is significant is written as a code
  span followed by the literal text `+ space` in markdown, because a trailing space inside an inline
  code span trips markdownlint MD038. The derivation reverses that in one place.
- In `analyze.md` only parenthesised groups that are *entirely* comma-separated
  code spans count as marker groups. That structural filter separates the
  groups from prose parentheticals and from unrelated code spans in the same
  sentence (the `not-a-live-claim` reason name is one), without guessing where
  the sentence ends — `e.g.` carries dots, so sentence-splitting is not
  available.

Verified against five injected drift modes: a marker dropped from the Rust
array (caught, with the length mismatch reported alongside), a marker added to
the canonical table only (caught in all three consumers plus all three counts),
the canonical heading renamed (caught as the fail-closed empty derivation), a
stale count word, and the restored baseline (exit 0).

## Edge Cases

- **Group names and row order are not checked.** The contract is the phrase
  *set*; grouping is editorial and read by people, not code.
- **The 022 scenario carries no phrases**, only a count and group names. Only
  its count is checked (18d) — asserting phrases against a document that
  deliberately omits them would force a restatement this family exists to avoid.
- **A marker containing parentheses** — the parenthetical-rename marker is exactly that — is handled:
  the group regex alternates over non-paren characters and whole code spans, so
  a span carries its own parens without terminating the group.
- **`analyze.md` cannot become a pointer.** It ships to adopters who have no
  copy of 045's data-model, so the restatement is their only view of the list.
  Binding it is the fix; removing it is not.
- **A missing file named by the family** is a finding rather than a skip, for
  the same fail-closed reason as the empty derivation.
- **`python3` absent** halts with a precondition finding, matching Family 17.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
