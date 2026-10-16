---
status: planned
dependencies: [022-deterministic-runtime, 066-audit-records-and-the-completion-gate]
next-criterion: 5
---

# 067 — Analyze and Audit Checks

The runtime primitives that check a spec corpus without judging it: the artifact families `/{project}:analyze` runs, the frontmatter, dependency, anchor and rule-citation checks beside them, the two derived frontmatter indexes, the link and reference integrity checks, and the two `/{project}:audit` families with a runtime half. The parser, the interpreter and the shared scanners they use are [022 — Deterministic Runtime](../022-deterministic-runtime/spec.md)'s; the records these checks feed are [066 — Audit Records and the Completion Gate](../066-audit-records-and-the-completion-gate/spec.md)'s. The argument and result shapes are in [data-model.md](data-model.md).

## Motivation

022's data model had reached 99,953 bytes against a 50,000-byte read size, and trimming could not close the gap: measured by eight-word overlap, only 2 KB of its prose repeated another file. Its content was contract. The deterministic checks were one of its two largest slices with a single subject, so they became a spec of their own.

## Artifact checks

`check-artifacts` runs the deterministic artifact families for one feature — completeness, task and scenario consistency, review-state and disposition drift, scenario open questions, link-adjacent drift, criterion paths and criterion labels — at the severity tiers the constitution assigns. A target a family cannot examine is reported as skipped with a reason from a closed set, never escalated into a finding, except the spec's own unreadable artifact at `done`.

## Frontmatter, dependency, anchor and rule checks

`validate-frontmatter` checks a file against the constitution's frontmatter schema and reports each finding at the tier its condition earns. `traverse-deps` verifies each declared dependency exists and has a compatible status, and reports cycles. `resolve-anchor` resolves each `§` reference against a markers file and classifies the ones that are not claims about it. `check-rule-ids` verifies cited rule IDs exist and are not deprecated, and reports how many rule files it read.

## Derived frontmatter indexes

`derive-dependencies` and `derive-references` regenerate a spec's `dependencies:` and `references:` from the links in its body. Both report rather than write unless asked, and both name every spec they did not examine or could not parse, so an empty `updated` is never mistaken for a corpus in sync.

## Link and reference integrity

`check-corpus-links` reports every relative markdown link in the spec corpus, or the tracked repository, that resolves to nothing. `check-orphaned-references` reports the adopter-owned files that point at a ductus-managed path a migration has moved.

## Audit checks

`check-command-flags` holds each command's declared `argument-hint` against the body a host substitutes into, and `check-promotion-coverage` reports how much of a rules file is routed to its canonical source. Each is the runtime half of a `/{project}:audit` family.

## Acceptance Criteria

- [ ] AC1: `check-artifacts` runs the `artifact-completeness`, `task-consistency`, `scenario-consistency`, `review-state-drift`, `scenario-open-questions`, `link-adjacent-drift`, `criterion-path-existence`, `criterion-labels` and `disposition-drift` families, and none of them judges `analysis.md`: every family runs inside `/{project}:analyze`, whose own run is about to replace that record, so `/{project}:analyze` judges it from the record it writes.
- [ ] AC2: `resolve-anchor` classifies each `§` reference as `qualified` (its line names a markdown document other than the markers file), `intra-document` (the anchor names a heading in the citing file) or `markers` (a claim about the markers file, and the only kind that can be unresolved), reporting the first two as counts so a large exclusion is never silent. The document match is line-scoped, the markers file is excluded from qualification, and local headings match longest-first.
- [ ] AC3: `check-artifacts` reports `artifact-unreadable` as a blocking finding on a spec at `done` and as a skipped target below it — the one exception to the rule that an unknown is never escalated into a defect, scoped to the spec's own artifact, since an unreadable scenario carrying unresolved questions would otherwise pass the gate built to catch it.
- [ ] AC4: `check-promotion-coverage` reports all four terms of `unclassified = rule-bearing − (table-keyed ∪ pointer-citing)`, not the difference alone. `sections` is required with no default; a named section that does not occur is reported in `missing-sections` rather than contributing zero; `unmatched-keys` reports a reworded entry whose table key no longer matches; `table-state` separates `no-table` from a computed zero; and a supplied `table-file` that cannot be read is an error. `/{project}:audit` Family 38 renders the coverage line to stderr where it never affects the exit code, and an unmatched key and a missing section are findings.

## Open Questions

*None.*

## Resolved Questions

- **Why a spec of its own rather than a scenario on 022?** `/{project}:specify`'s routing gate offered 022, the runtime-owning spec. Declined: 022's data model exceeded one read, and these primitives share one subject — checking the corpus without judging it — that no other part of 022 needs to understand. (Operator-approved split, 2026-10-15.)
- **Why two new specs and not one?** One spec holding both the checks and the audit records would have left its data model, and 022's, within 2–4 KB of the read size. The records and the gate are 066's; the dependency runs 067 → 066 → 022.
