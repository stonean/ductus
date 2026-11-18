---
status: in-progress
dependencies: [022-deterministic-runtime, 066-audit-records-and-the-completion-gate]
next-criterion: 6
cross-spec-impact: [022-deterministic-runtime, 066-audit-records-and-the-completion-gate]
---

# 067 — Analyze and Audit Checks

The runtime primitives that check a spec corpus without judging it: the artifact families `/{project}:analyze` runs, the frontmatter, dependency, anchor and rule-citation checks beside them, the two derived frontmatter indexes, the link and reference integrity checks, and the `/{project}:audit` families with a runtime half. The parser, the interpreter and the shared scanners they use are [022 — Deterministic Runtime](../022-deterministic-runtime/spec.md)'s; the records these checks feed are [066 — Audit Records and the Completion Gate](../066-audit-records-and-the-completion-gate/spec.md)'s. The argument and result shapes are in [data-model.md](data-model.md).

> **Signpost:** [033](../033-rule-surface-setting/spec.md) changed `check-rule-ids`: a citation of a shipped rule whose file `[rules] surfaces` kept uninstalled is reported `not-installed`, an informational state recorded unexamined under `rule-not-installed`, never `missing`.
>
> **Signpost:** [079](../079-review-and-analysis-work-converges/spec.md) added a condition to review-state drift: a `done` spec holding an unticked criterion, with no grandfather rule.
>
> **Signpost:** [073 — Decided rule departures are recorded](../073-decided-rule-departures-are-recorded/spec.md) added the `check-departures` primitive and its `Departure entries (blocking)` family to the analyze checks, and the report's `## Waived rules` section.
>
> **Signpost:** [050 — Constitution](../050-constitution/spec.md)'s AC24 forbids a line-number citation in a spec artifact and removes this repository's existing ones. AC5's `line-number-citations` family is the check that enforces it, and reports an adopter's when that spec's analysis next runs.

## Motivation

022's data model had reached 99,953 bytes against a 50,000-byte read size, and trimming could not close the gap: measured by eight-word overlap, only 2 KB of its prose repeated another file. Its content was contract. The deterministic checks were one of its two largest slices with a single subject, so they became a spec of their own.

## Artifact checks

`check-artifacts` runs the deterministic artifact families for one feature — completeness, task and scenario consistency, review-state and disposition drift, scenario open questions, link-adjacent drift, criterion paths, body paths, criterion labels, spec integrity, plan consistency and cross-spec impact drift — at the severity tiers the constitution assigns. A target a family cannot examine is reported as skipped with a reason from a closed set, never escalated into a finding, except the spec's own unreadable artifact at `done`.

## Frontmatter, dependency, anchor and rule checks

`validate-frontmatter` checks a file against the constitution's frontmatter schema and reports each finding at the tier its condition earns. `traverse-deps` verifies each declared dependency exists and has a compatible status, and reports cycles. `resolve-anchor` resolves each `§` reference against a markers file and classifies the ones that are not claims about it. `check-rule-ids` verifies cited rule IDs exist and are not deprecated, and reports how many rule files it read.

## Derived frontmatter indexes

`derive-dependencies` and `derive-references` regenerate a spec's `dependencies:` and `references:` from the links in its body. Both report rather than write unless asked, and both name every spec they did not examine or could not parse, so an empty `updated` is never mistaken for a corpus in sync.

## Link and reference integrity

`check-corpus-links` reports every relative markdown link in the spec corpus, or the tracked repository, that resolves to nothing. `check-orphaned-references` reports the adopter-owned files that point at a ductus-managed path a migration has moved.

> **Signpost:** [051 — Branch-scoped spec numbering](../051-branch-scoped-spec-numbering/spec.md) widened the scanner this check shares with `rewrite-spec-links` (2026-10-28, its task 30): a managed path behind a prefix that names the project root from the referrer (`./specs/…`, `/specs/…` in a root markdown file, `../specs/…` from the spec root's `system.md`) is now a reference, as `data-model.md` §`check-orphaned-references` states.

## Audit checks

`check-command-flags` holds each command's declared `argument-hint` against the body a host substitutes into, and `check-promotion-coverage` reports how much of a rules file is routed to its canonical source. Each is the runtime half of a `/{project}:audit` family, as `resolve-anchor` is of Family 39 and `read-deprecation-labels`, which returns the deprecation labels of the rule files, is of Family 40.

## Acceptance Criteria

- [x] AC1: `check-artifacts` runs the `artifact-completeness`, `task-consistency`, `scenario-consistency`, `review-state-drift`, `scenario-open-questions`, `link-adjacent-drift`, `criterion-path-existence`, `body-path-existence`, `line-number-citations`, `criterion-labels`, `disposition-drift`, `cross-spec-impact-drift`, `spec-integrity` and `plan-consistency` families, and none of them judges `analysis.md`: every family runs inside `/{project}:analyze`, whose own run is about to replace that record, so `/{project}:analyze` judges it from the record it writes.
- [x] AC2: `resolve-anchor` classifies each `§` reference as `qualified` (its line names a document other than the markers file: a markdown one, a file with an extension, or a URL to a markdown document, or the reference is a code span holding a path and the reference), `intra-document` (the anchor names a heading in the citing file) or `markers` (a claim about the markers file, and the only kind that can be unresolved), reporting the first two as counts so a large exclusion is never silent. The document match is line-scoped, the markers file is excluded from qualification, and local headings match longest-first. **A fourth kind since spec 071:** `cross-document`, a reference pointing into a document the **Shared Files** manifest ships — on a line whose only document other than the markers file is such a document, or written as an explicit `**§X** in` pair — is decided before the other three, judged against that document and counted; it can be unresolved, so `markers` is no longer the only kind that can (scenario `pointers-into-shipped-files-are-judged`). **Longest-first never decided a match:** a local heading resolves a reference when the text after `§` begins with it, so the order the headings are tried in cannot change a result. Since 071 the heading must also end where a word ends, and a heading line inside a fenced code block or an HTML comment is not a heading. **Judged since scenario `shared-constitution-references-are-judged` (2026-11-11):** a `qualified` reference is no longer only counted. It resolves against a heading of a readable document its line names, a registered shared constitution's markers, a heading of the citing file or a marker, and can be unresolved; one it cannot judge carries an `unjudged` reason, `anchor-target-unjudgeable` or `anchor-target-unreadable`, counted beside the kinds. The markers file is recognized by its resolved path, and a code span holding a path and one reference is a reference.
- [x] AC3: `check-artifacts` reports `artifact-unreadable` as a blocking finding on a spec at `done` and as a skipped target below it — the one exception to the rule that an unknown is never escalated into a defect, scoped to the spec's own artifact, since an unreadable scenario carrying unresolved questions would otherwise pass the gate built to catch it.
- [x] AC4: `check-promotion-coverage` reports all four terms of `unclassified = rule-bearing − (table-keyed ∪ pointer-citing)`, not the difference alone. `sections` is required with no default; a named section that does not occur is reported in `missing-sections` rather than contributing zero; `unmatched-keys` reports a reworded entry whose table key no longer matches; `table-state` separates `no-table` from a computed zero; and a supplied `table-file` that cannot be read is an error. `/{project}:audit` Family 38 renders the coverage line to stderr where it never affects the exit code, and an unmatched key and a missing section are findings.
- [ ] AC5: **Line-number citations are reported.** `check-artifacts` runs a `line-number-citations` family over a spec's `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, `research.md` and `scenarios/*.md`. It reports as a blocking finding each `:N` or `:N-M` suffix after a path ending in a file extension, each `#LN` anchor in a link, and each inline code span holding only such a suffix, a line number set apart from its file (owner decision, 2026-11-17), inline code spans included and fenced code and blockquotes excluded. No citation is exempt, a commit-pinned one included, and none is grandfathered. A prose line reference is not detected, and a colon and number after anything but a file path, such as a host's port, is not a citation. The family does not examine `review.md` or `analysis.md`. AC1's family list names it, and `framework/reference/analyze.md` states its scope and severity.

## Open Questions

*None.*

## Resolved Questions

- **Why a spec of its own rather than a scenario on 022?** `/{project}:specify`'s routing gate offered 022, the runtime-owning spec. Declined: 022's data model exceeded one read, and these primitives share one subject — checking the corpus without judging it — that no other part of 022 needs to understand. (Operator-approved split, 2026-10-15.)
- **Why two new specs and not one?** One subject per spec: this one holds the checks, which have readers the audit records do not — the `/{project}:audit` families and the pre-commit hook — and the records and the gate are 066's. Size does not decide it: one spec holding both would carry a data model of about 40 KB (the two as built, 19,775 and 20,201 bytes by `wc -c` on 2026-10-15), some 10 KB under the 50,000-byte read size, where two keep each near 20 KB; 022's 44,823 bytes are the same either way. The dependency runs 067 → 066 → 022.
- **Why split from audit-records-and-the-completion-gate:** for the reason the entry above gives. The two plans share `runtime/src/primitives/check_artifacts.rs` only because its cross-spec-impact-drift family calls the gate's cross-spec-impact classification, which 066 owns, and 066's task 23 changed what that family names. The family stays this spec's, as the classification stays 066's.
- **Which line-number citations does the `line-number-citations` family report, and at what tier?** **Every one, as a blocking finding**: a `:N` or `:N-M` suffix after a path ending in a file extension, or a `#LN` anchor in a link, anywhere in the examined files outside fenced code and blockquotes. None is grandfathered: 050's AC24 removes the existing ones here, and an adopter's are reported when that spec's analysis next runs and fixed with the work that reopened it (owner, 2026-11-17), so the family has nothing to tell apart and reads no git history. A prose reference ("lines 100-103") is not detected, because a pattern broad enough to find one also flags a requirement such as the `# managed-by: ductus` sentinel on line 2, so prose stays the rule's alone. A commit-pinned citation is not exempt. The agent's recommendation, adopted by the operator, 2026-11-17.
