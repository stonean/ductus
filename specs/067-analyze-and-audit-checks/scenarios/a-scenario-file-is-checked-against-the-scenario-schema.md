---
section: "Frontmatter, dependency, anchor and rule checks"
---

# A-scenario-file-is-checked-against-the-scenario-schema

## Context

The constitution's frontmatter schema has a row per artifact kind: a spec file requires `status` and `dependencies`, a scenario file requires `section` (legacy `spec-ref` accepted). `validate-frontmatter` applies the spec row to every file. Pointed at a scenario whose frontmatter is only `section:`, it answers hard-fail `status is missing` and `dependencies is missing` (0.59.11 and HEAD f24c55c6, 2026-10-25) — a well-formed scenario reported malformed, and a scenario missing `section` never reported for that.

## Behavior

- The path chooses the row. A file named `spec.md` takes the spec row; a `.md` file whose parent directory is `scenarios/` takes the scenario row; `review.md` and `analysis.md` take their audit-record checks; any other path is refused with an error naming the four kinds, never checked against a row that does not describe it.
- Against the scenario row, a missing or empty `section` with no legacy `spec-ref` is the hard fail, and a missing `status` or `dependencies` is not a finding.
- A `spec.md` run checks the spec row, unchanged, and also each scenario beside it, as it already checks both audit records, each scenario finding keyed by its `scenarios/{slug}.md` path. So `/{project}:analyze` step 2 checks every scenario through the primitive, the exec walk included, and the host no longer judges scenarios itself.

## Edge Cases

- A scenario carrying both `section` and legacy `spec-ref`: valid.
- A scenario carrying stray `status:` or `dependencies:` keys: informational unknown fields, not a spec-schema check.
- A path that is neither a `spec.md`, under `scenarios/`, nor an audit record (a plan, `tasks.md`, `inbox.md`): refused, as above.
- A feature with no `scenarios/` directory, or an empty one: a `spec.md` run adds no scenario finding.
- A scenario file that cannot be read, has no frontmatter fence, or whose frontmatter does not parse: a hard fail keyed by its path in a `spec.md` run, as an unreadable record is, never a silent skip; on a direct scenario path, the same errors a direct `spec.md` path returns today.
- A direct `review.md` or `analysis.md` path whose record is absent: refused like any missing file; one that does not parse, or whose `decisions:` list does not parse, is the hard fail a `spec.md` run reports for it.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **How does `validate-frontmatter` choose a schema, and do the audit records' rows join the same selection now or later?** By the path, now, and a `spec.md` run covers the scenarios beside it (owner, 2026-11-11, option (a2)). The primitive already selects by path: a file named `spec.md` also checks the `review.md` and `analysis.md` beside it (`validate_record_artifacts` in `runtime/src/primitives/validate_frontmatter.rs`), so the records joined through the spec, and only a direct record path was open. An explicit kind argument would change every caller — analyze step 2, the exec binding in `runtime/src/interpreter/mod.rs`, and four test files — and still need the path checked to refuse a mismatch. No primitive judged scenarios: analyze step 2 left them to the host, and the exec walk no-ops that prose without recording it unexamined (`HOST_DETECTION` in `runtime/src/interpreter/analyze_tally.rs` names three other steps), so under `ductus exec` a malformed scenario passed silently. Covering scenarios from the `spec.md` run closes that for one loop. Refusing any other path is breaking under BE-COMPAT-001 in principle; no caller passes one.
