---
section: "Follow-on scenarios"
---

# Criterion-label-assignment

## Context

[013 — Text-First Artifacts](../../013-text-first-artifacts/spec.md) requires every acceptance criterion to carry a stable `AC{n}:` label, assigned by the runtime rather than by the agent, and backed by a monotonic `next-criterion` frontmatter field so a retired label is never reissued. That spec owns the requirement, the frontmatter schema, and the corpus-wide backfill decision; the primitive-level changes land here, per §cross-spec-impact and this repo's rule that runtime work routes to 022.

The runtime already holds every piece this needs in a different shape: `append-task` derives the next in-band `## N.` number from the file it appends to, `read-spec` parses acceptance criteria with their checkbox state in body order, `mark-criterion` addresses a criterion by 0-based index, and `check-artifacts` carries the family set the audit invariant belongs in. What is missing is the assignment pass itself and label-aware addressing.

## Behavior

### The labelling pass

A primitive scans a spec's Acceptance Criteria and returns the labels it assigned:

- Criteria already carrying an `AC{n}:` label are left byte-identical — the pass never renumbers, so it is safe to run repeatedly and safe to run on the two specs already labelled by hand.
- Each unlabelled criterion receives the next label, taken from `max(highest label in body, next-criterion)` and incremented per assignment, in body order.
- The label is written after the checkbox and before the criterion's text (`- [ ] AC7: …`), leaving the text itself untouched.
- `next-criterion` is written back to the frontmatter, always greater than every label in the body. It is never lowered, including when the pass runs on a spec whose highest-labelled criterion was deleted.
- With nothing to assign, the pass is a no-op: no write, no frontmatter churn, and a result that says so rather than reporting a clean write it did not perform.

### Label-aware addressing

`mark-criterion` accepts a label as well as a 0-based index. A label that no criterion carries is a domain outcome naming the label, not a silent no-op or an out-of-range error — the two addressing modes must never disagree about which criterion is meant, and an unresolvable label means the caller's reference is stale, which is exactly the condition labels exist to surface.

`read-spec` reports each criterion's label alongside its text and checkbox state, so a host can cite a criterion without re-parsing the line.

### The audit invariant

A `check-artifacts` family reports, per spec: a duplicate `AC{n}` label within one spec; a `next-criterion` at or below the highest label present in the body; and — once the backfill has landed — an unlabelled criterion. Each is checkable from the artifact alone, with no git history read.

## Edge Cases

- **A spec with no Acceptance Criteria section, or an empty one** — nothing to assign; the pass writes nothing and does not create `next-criterion`.
- **A criterion whose text already begins with something label-shaped** (`- [ ] AC5 of spec 017 is superseded…`) — the grammar anchors on `AC{n}:` immediately after the checkbox, so prose mentioning a label elsewhere in the line is not mistaken for one.
- **A malformed or unparseable spec** — the pass refuses rather than writing a partial relabelling; a half-labelled criteria list is worse than an unlabelled one, because it looks complete.
- **`next-criterion` present but non-integer, or below 1** — a defect the audit reports; the pass does not silently repair it, since a corrupted counter may indicate a label was already reissued and repairing it in place would hide that.
- **Concurrent edits** — the write is atomic (tempfile + rename) like every other primitive write, so a concurrent hand edit is lost-update-protected at the file level rather than merged.
- **A spec labelled by hand with gaps** (`AC1`, `AC2`, `AC7`) — gaps are legal and untouched; the pass continues from `max + 1`, never filling them, because a gap means a retired label.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
