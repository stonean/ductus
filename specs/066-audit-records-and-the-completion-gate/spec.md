---
status: in-progress
dependencies: [022-deterministic-runtime]
next-criterion: 6
---

# 066 — Audit Records and the Completion Gate

The runtime primitives that record a spec's two audits — `/{project}:review` in `review.md` and `/{project}:analyze` in `analysis.md` — and the pre-`done` gate that reads them. The parser, the interpreter, the protocol and the shared surfaces these primitives build on are [022 — Deterministic Runtime](../022-deterministic-runtime/spec.md)'s; the record schema itself is the constitution's (§text-first-artifacts, Audit records). The argument and result shapes are in [data-model.md](data-model.md).

## Motivation

022's data model had reached 99,953 bytes against a 50,000-byte read size, and trimming could not close the gap: measured by eight-word overlap, only 2 KB of its prose repeated another file. Its content was contract. The primitives that write and judge the audit records were one of its two largest slices with a single subject, so they became a spec of their own.

## The review record

`compute-review-scope` resolves the review's diff base — the parent of the commit that moved the spec into `in-progress`, `HEAD` while that move is uncommitted, or a `--since` override — and its scope, the union of the plan's Affected Files and every file changed since the base in the working tree. `process-waivers` classifies the recorded waivers against the passes' findings and `process-decisions` the stored decisions, before anything is written. `write-review` writes `review.md`: the report, and in its frontmatter the record — the violation counts and `blocking`, the observations' `dispositions:`, `examined` as the caller's claim over a `scope` the primitive derives, and `reviewed-digest`.

## The analyze record

`write-analysis` writes `analysis.md`: the record in its frontmatter and a fixed report skeleton below it. It derives `blocking`, `unexamined`, the `dispositions:` map and the digest of what the run read, and never accepts them from the caller.

## The completion gate

`check-review-gate` evaluates `/{project}:implement`'s pre-`done` gate in a documented order, the first failure winning: a spec already `done`, the feature's markdown lint, unresolved scenario questions, an undischarged `cross-spec-impact:` entry, the review record and its freshness, the analyze record and its freshness, a record that predates dispositions, and an outstanding gating finding. `diff-cross-spec` reports the sibling specs the feature's own run has changed, for the summary `/{project}:implement` shows beside the gate.

## Record freshness

A record is stale when the files it read have changed since, compared by content: each record carries a per-path sha256 of its subjects as the run read them from disk. The review's subjects are the spec's durable contracts, `scenarios/*.md` and `data-model.md`; the analysis's are every markdown file under the feature except `analysis.md`, and on a `done` spec the comparison also ignores an in-place edit to `plan.md` or `tasks.md` (`scenarios/a-done-specs-plan-edit-leaves-its-analysis-current.md`). The recorded commit is provenance, read only to exempt a mechanical rename sweep.

## Acceptance Criteria

- [x] AC1: `write-analysis` writes `analysis.md` — the run's record in its frontmatter and a fixed report skeleton below it — deriving `blocking` from the gating tiers rather than accepting it, and refusing a spec whose `spec.md` frontmatter does not parse. `check-review-gate` reads the review record from `review.md` and the analyze record from `analysis.md`, and reports `not-analyzed` and `analyze-findings` after every review check.
- [x] AC2: `write-analysis` records `unexamined-by-reason` over the closed skip-reason set and derives `unexamined` by summing it, so the total and its parts cannot disagree. The map is omitted when empty, a malformed pair is rejected rather than defaulted to zero, and the written total is returned so a caller can confirm agreement.
- [x] AC3: Review staleness compares content, not commits: `write-review` records `reviewed-digest`, a per-path sha256 of the durable contracts (`scenarios/*.md` and `data-model.md`) as the review read them from disk, and `check-review-gate` compares it against the same files now. `reviewed-against` is provenance, read only for the mechanical-sweep rename exemption, whose candidates are reported rather than dropped when a tree is unavailable. A record with no digest is `undeterminable` — not current, not stale — and does not block, and the passing verdict's `guidance` names the record it could not judge; a spec with no durable contracts records an empty digest, which reads as current.
- [x] AC4: `check-review-gate` reports a spec at `status: done` as having no transition to gate — its own blocked variant, never `passed: true` — ordered first, ahead of the markdown lint, so a completed spec is never held against a later lint violation or a freshness check its own completing `set-status` disturbed. Only the exact value `done` short-circuits; an unrecognized status falls through to the checks, and `/{project}:implement` halts on the outcome cleanly.
- [x] AC5: `check-review-gate` blocks `in-progress → done` on an undischarged `cross-spec-impact:` entry, ordered after the scenario-question check and ahead of the review-record checks. Discharge is the reciprocal link — the named spec's body or a scenario under it links back — read with `derive-dependencies`' matcher over the pointer scope, so a blockquoted signpost discharges. The result reports each entry as `discharged`, `undischarged`, `target-missing` or `self-reference` on a pass as well as a block, and the check never writes.

## Open Questions

*None.*

## Resolved Questions

- **Why a spec of its own rather than a scenario on 022?** `/{project}:specify`'s routing gate offered 022, the runtime-owning spec. Declined: 022's data model exceeded one read, and these primitives share one subject — the two audit records and the gate that reads them — that no other part of 022 needs to understand. (Operator-approved split, 2026-10-15.)
- **Why two new specs and not one?** One subject per spec: this one holds the audit records and the gate that reads them, and `067-analyze-and-audit-checks` the checks whose findings feed the analyze record, which have readers these primitives do not — the `/{project}:audit` families and the pre-commit hook. Size does not decide it: one spec holding both would carry a data model of about 40 KB (the two as built, 19,775 and 20,201 bytes by `wc -c` on 2026-10-15), some 10 KB under the 50,000-byte read size, where two keep each near 20 KB; 022's 44,823 bytes are the same either way. The dependency runs 067 → 066 → 022.
