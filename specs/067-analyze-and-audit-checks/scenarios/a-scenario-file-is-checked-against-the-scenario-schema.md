---
section: "Frontmatter, dependency, anchor and rule checks"
---

# A-scenario-file-is-checked-against-the-scenario-schema

## Context

The constitution's frontmatter schema has a row per artifact kind: a spec file requires `status` and `dependencies`, a scenario file requires `section` (legacy `spec-ref` accepted). `validate-frontmatter` applies the spec row to every file. Pointed at a scenario whose frontmatter is only `section:`, it answers hard-fail `status is missing` and `dependencies is missing` (0.59.11 and HEAD f24c55c6, 2026-10-25) — a well-formed scenario reported malformed, and a scenario missing `section` never reported for that.

## Behavior

- A file under a feature's `scenarios/` directory is checked against the scenario row: a missing `section` with no legacy `spec-ref` is the hard fail, and a missing `status` or `dependencies` is not a finding.
- A `spec.md` is checked against the spec row, unchanged.
- No file is silently checked against a row that does not describe it.

## Edge Cases

- A scenario carrying both `section` and legacy `spec-ref`: valid.
- A scenario carrying stray `status:` or `dependencies:` keys: informational unknown fields, not a spec-schema check.
- A path that is neither a `spec.md` nor under `scenarios/` (a plan, an audit record, `inbox.md`).

## Open Questions

- How does `validate-frontmatter` choose a schema: by the path (a file under `scenarios/` takes the scenario row, `spec.md` the spec row) with any other path refused, or by an explicit argument naming the artifact kind — and do the audit records' rows join the same selection now or later?

## Resolved Questions

*None yet.*
