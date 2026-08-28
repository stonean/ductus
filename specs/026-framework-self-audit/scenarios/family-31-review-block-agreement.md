---
section: "Check Families"
---

# Family-31-review-block-agreement

## Context

A reviewed spec records the same review twice. `spec.md`'s frontmatter carries
a `review:` block — `last-run`, `reviewed-against`, `must-violations`,
`should-violations`, `low-confidence`, `blocking`, and any `waivers` — and the
`review.md` written alongside it carries its own frontmatter with
`reviewed-at`, `reviewed-against`, and the same three counts. Nothing holds the
two in agreement.

They drifted, and for weeks. Both [031](../../031-agent-mcp-wiring/spec.md) and
[041](../../041-task-pruning/spec.md) carried `should-violations: 1` in
`spec.md` while their own `review.md` recorded `0`. The drift was invisible
because every gate reads exactly one of the two files and never the other:
`check-review-gate` and `/ductus:analyze`'s review-drift check read the
`spec.md` block; Family 19 (review-freshness) resolves `reviewed-against` to
decide whether a review predates its code but never compares the counts on
either side of it.

The root cause in the 031 case was a waiver moved into `review.md` by hand with
no matching `review.waivers` entry in `spec.md`. The waived finding had no
structural existence on the gate's side, so the count it was supposed to
retire never dropped.

The cost runs in both directions. A stale non-zero count reads as outstanding
review work that does not exist — the signal that sent a maintainer back to
re-derive two clean specs before a release tag. A stale zero is worse: it would
hide real findings from `check-review-gate`, `/ductus:analyze`, and the
`in-progress → done` transition all three feed, every one of them trusting a
number the report beside it contradicts.

This is the shape `/audit` already exists to catch — Family 2 (manifest
parity), Family 18 (marker-list parity), Family 23 (sweep-target manifest
parity), Family 28 (audit family registry parity) are each a pair of places
recording one fact with nothing binding them. The review state is that pair,
and it is the one pair whose divergence a release gate acts on.

## Behavior

A new `/audit` family asserts that, for every spec carrying both a `spec.md`
`review:` block and a `review.md`, the fields the two files record about the
same review agree. Both sides are derived from the frontmatter actually on
disk; nothing about the expected values is hardcoded, which would be a third
copy of the fact under test — the shape Family 28 uses for the audit registry.

The compared pairs, keyed by meaning rather than by key name, since the two
files spell one of them differently:

| `spec.md` `review:` | `review.md` frontmatter |
| --- | --- |
| `last-run` | `reviewed-at` |
| `reviewed-against` | `reviewed-against` |
| `must-violations` | `must-violations` |
| `should-violations` | `should-violations` |
| `low-confidence` | `low-confidence` |

Each mismatch is reported with the spec, the field, and both values, so the
maintainer can see which side is stale without opening either file. The
suggested fix names `review.md` as the source of record — it is the artifact
`/ductus:review` writes from the pass it just ran, while the `spec.md` block is
the summary copied forward for the gates — so a disagreement is repaired by
re-deriving the block, not by editing the report to match it.

The family also asserts the two consistency rules internal to the `spec.md`
block, because the observed failure reached the counts through them:

- **`blocking` agrees with `must-violations`.** `blocking: false` alongside a
  non-zero `must-violations` claims a spec may advance while its own record
  says it may not.
- **A waiver has structural existence on both sides.** A finding waived in
  `review.md` with no corresponding `review.waivers` entry in `spec.md` is the
  031 root cause exactly: the waiver is invisible to every gate, and the count
  it should have retired never moves.

Fields that exist on only one side — `diff-base`, `captured-issues`,
`skipped-passes` in `review.md`; `blocking`, `waivers` in `spec.md` — are not
compared as pairs. They are not duplicated facts, and demanding they match
would invent a binding the artifacts never claimed.

## Edge Cases

- **A spec with one file and not the other is not a mismatch.** A spec with a
  `review:` block and no `review.md`, or a `review.md` with no block, is a
  different defect with a different repair, and Family 19 and
  `check-review-gate` already have opinions about the first. This family's
  subject is the intersection: specs where both records exist and can
  therefore disagree. The count of specs examined goes to stderr so a
  shrinking intersection is visible rather than silent.
- **An empty subject set is a finding, not a pass.** Zero specs with both
  files means the enumeration or the frontmatter parse broke, and a parity
  check comparing nothing reports agreement — the false green Families 17, 18,
  23, and 28 all fail closed on.
- **The timestamps are compared as recorded, not as instants.** `last-run` and
  `reviewed-at` are written from the same run, so equality is the assertion;
  a check that allowed "close enough" would tolerate exactly the copy-forward
  that was skipped.
- **Malformed or absent frontmatter on either side is a finding.** A
  `review.md` whose frontmatter does not parse is reported as an extraction
  failure rather than skipped, since silently dropping a spec from the subject
  set is how the empty-set false green begins.
- **Frontmatter only, never prose.** `review.md` bodies quote counts and sha
  fragments in their narrative, and spec bodies cite other specs' review
  state. Both extractions are anchored to the frontmatter block, so a mention
  in running text is never a subject.
- **This family cannot exempt 026 from itself.** `026`'s own `review:` block
  and `review.md` are in the subject set like every other spec's, which is
  correct: a check that skipped its own spec would be blind to the one
  divergence it authored.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
