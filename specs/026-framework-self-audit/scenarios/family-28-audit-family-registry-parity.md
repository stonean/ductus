---
section: "Check Families"
---

# Family-28-audit-family-registry-parity

## Context

`/audit` runs the families `scripts/audit/run-all.sh` registers.
[`framework/commands/audit.md`](../../../framework/commands/audit.md)
enumerates the families for the maintainer who reads the command. Nothing held
the two together.

They drifted. `run-all.sh` registered 25 families (1–2 and 4–26) while
`audit.md`'s numbered list stopped at 23, so Families 24 (rename-sweep
residue), 25 (unbalanced inline markup), and 26 (broken relative links) were
absent from it in any form. The three missing were the three added most
recently, which is the direction this drift always runs: a family is written,
wired into `run-all.sh` because that is what makes it *execute*, and the doc
update is the step with no consequence if skipped.

That is the same shape Family 2 (manifest parity), Family 18 (marker-list
parity), and Family 23 (sweep-target manifest parity) already exist to catch —
a list and a registry that must agree, with nothing making them. This registry
simply never got its own check, which is a gap in `/audit`'s coverage *of
itself*.

The cost is not cosmetic. `audit.md` is the maintainer's map of what the
release gate asserts; a map that under-reports by three families invites the
conclusion that a concern is unchecked when it is checked, and — worse in the
other direction — makes a *deleted* family invisible, since a script dropped
from `run-all.sh` while its entry survives in `audit.md` reads as still
running. Both directions are failures of the same missing binding.

## Behavior

A new `/audit` family asserts that the family set `run-all.sh` registers and
the family set `audit.md` enumerates are equal, reporting each direction
separately because the repairs differ.

- **Registered but undocumented** — a family runs and the maintainer's map
  omits it. Suggested fix: add its entry to `audit.md`.
- **Documented but unregistered** — `audit.md` describes a family that no
  longer runs. Suggested fix: remove the entry, or re-register the script.

Both sets are derived, never hardcoded: the registered set from `run-all.sh`'s
`run_check "Family N — …"` lines, the documented set from `audit.md`'s
`(Family N — …)` entries. A hardcoded expectation would be a third copy of
exactly the fact under test.

Retired family numbers are handled by the same derivation rather than a
special case. Family 3 (registry equivalence) was retired with the workflows
feature (spec 043) and its number is permanently spent; because it appears in
neither list, the sets agree and nothing fires. Family numbers are stable
identifiers, so a gap is the correct state and needs no allowance.

The family also asserts a third binding, since the same drift reaches it: every
registered family's script is listed in
[`scripts/audit/README.md`](../../../scripts/audit/README.md) §Scripts, which
is the third place the family set is written down.

## Edge Cases

- **An empty derivation on either side is a finding, not a pass.** Zero
  registered families means the parse of `run-all.sh` broke, and a parity check
  that compares two empty sets reports agreement — the precise false green
  `/audit` exists to prevent, and the reason Families 17, 18, and 23 all fail
  closed on an empty extraction.
- **`audit.md`'s prose mentions a family number outside its list.** The
  boundary section and several entries reference other families by number in
  running prose (Family 23's entry names 042 and 049; the header names Family
  3). Only the enumerated `(Family N — …)` entries count, so the extraction is
  anchored to that form rather than to any mention of the digits.
- **The generated copy is not a second subject.** `.claude/commands/ductus/audit.md`
  is generator output; auditing it would report the generator rather than a
  defect. The source is the subject, and the generator's own sync is already
  gated by `check-zero` and the generators workflow.
- **This family must appear in its own lists.** It is registered in
  `run-all.sh` and documented in `audit.md` like any other, so it is inside its
  own subject set — which is correct, not circular: a family that exempted
  itself would be the one entry the check could never catch.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
