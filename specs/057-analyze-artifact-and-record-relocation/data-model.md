# 057 — Analyze artifact and record relocation Data Model

The two audit records, at their new homes. Field shapes are carried over from
`runtime/src/schema/primitives.rs` — `ReviewBlock` at line 33, `AnalyzeBlock` at
line 109 — with the changes each table's Notes section names.

> **Changed by [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md).**
> Both records dropped `captured-issues` and gained a `dispositions:` map and a
> `decisions:` list, and `analysis.md`'s `## Captured issues` section became
> `## Fixed in this run`. The rows and the skeleton below show the records as
> they ship; 058's data model owns the shapes of the two new fields.

## Review record — `specs/{feature}/review.md` frontmatter

| Field | Type | Source | Notes |
| --- | --- | --- | --- |
| `spec` | string | `review.md` | Feature slug. Unchanged. |
| `last-run` | ISO-8601 UTC, nullable | merged | Was `last-run` in the block and `reviewed-at` in the report — one instant, two names. `last-run` survives. |
| `reviewed-against` | sha, nullable | both (identical) | Provenance only, never the staleness basis. |
| `diff-base` | sha, nullable | `review.md` | Report-only field; carried into the merged record. |
| `must-violations` | u32 | both (identical) | |
| `should-violations` | u32 | both (identical) | |
| `low-confidence` | u32 | both (identical) | |
| `captured-issues` | u32 | `review.md` | Report-only field. **Removed by 058**, with the inbox write it counted. |
| `examined` | u32 | both (identical) | |
| `scope` | u32 | both (identical) | |
| `skipped-passes` | list of strings | `review.md` | Report-only field. |
| `reviewed-digest` | map path → sha256 | `spec.md` block | Subject set unchanged: `scenarios/*.md` and `data-model.md`, excluding `review.md` and `spec.md`. See the plan on why the `spec.md` exclusion's rationale lapses without the set changing. |
| `blocking` | bool | `spec.md` block | Block-only field. Derived, not authored. |
| `waivers` | list | `spec.md` block | Block-only field. The one AC12 field most easily lost in a merge, because only one side ever had it. |
| `dispositions` | map `fixed`/`deferred`/`waived`/`undispositioned` → u32 | 058 | What the run did with its observations. Always written; absence means the record predates dispositions. |
| `decisions` | list | 058 | As [058's data model](../058-findings-route-at-discovery/data-model.md) and [076's](../076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md) define its entries. |

**Notes.** Six fields were byte-identical across the two records and collapse to
one. Five existed on exactly one side and are carried, not dropped — that is the
whole of AC12. Absence of the file is the never-reviewed state (AC5); the file
is never written empty to signal it.

## Analyze record — `specs/{feature}/analysis.md` frontmatter

| Field | Type | Notes |
| --- | --- | --- |
| `spec` | string | Feature slug, mirroring `review.md`'s. New field. |
| `last-run` | ISO-8601 UTC, nullable | Unchanged name. |
| `analyzed-against` | sha, nullable | Provenance only. |
| `analyzed-digest` | map path → sha256 | The staleness basis. Subject set is every `.md` under the feature directory, `review.md` included — now excising **`analysis.md`'s own record** rather than `spec.md`'s `analyze:` block, so `spec.md` is digested whole. |
| `analyzed-unreadable` | list of strings | Subjects that exist but could not be read. Recorded rather than digested as empty. |
| `hard-fail` | u32 | |
| `blocking-findings` | u32 | Named for the tier, not for the derived flag below. |
| `advisory` | u32 | Recorded, never gated on. |
| `unexamined` | u32 | The field that makes a clean run honest: clean-with-nothing-skipped and clean-with-something-skipped are two states. |
| `unexamined-by-reason` | map reason → count | |
| `captured-issues` | integer | Findings this run appended to the inbox. **Removed by 058**: the inbox is no longer a destination. The pair it formed with the tier counts is what `dispositions` now provides. |
| `blocking` | bool | Derived. |
| `dispositions` | map `fixed`/`deferred`/`waived`/`undispositioned` → u32 | Added by 058. What the run did with its findings, in every tier; `undispositioned` is derived as the live tier total less the live findings deferred or waived. Always written. |
| `decisions` | list | Added by 058. Stored deferred and waived finding decisions, keyed `{family} — {message}`. Omitted when empty. |

**Notes.** No field is added or removed by the relocation; the record is the one
`write-analysis` already produces. What changes is the file it lands in and the
excision the digest performs.

## `analysis.md` body

A fixed skeleton, rendered whole on every run (AC14), mirroring `review.md`:

```text
# Analysis — {feature}

## Summary
## Hard failures
## Blocking findings
## Advisory findings
## Unexamined targets
## Fixed in this run
```

**Notes.** No section carries `- [ ]` items — AC13, checked mechanically, by
stripping any checkbox marker a caller's finding text arrives with rather than
trusting the caller not to send one. A checkbox is what would make this a
triage queue, and a finding is decided in the run that finds it (058).

Each tier section lists its live findings, each beside its disposition, because
since 058 `write-analysis` receives every finding with its tier. As 057
delivered it, the writer received only per-tier counts plus one list of captured
inbox bullets that recorded no tier, so the tier sections carried counts and a
sixth section, for captured issues, carried the text. 058 replaced that section
with `## Fixed in this run`, which lists the findings a run fixed and the
re-check no longer produced.

## `relocate-audit-records` result

The migration primitive's report, per spec.

| Field | Type | Notes |
| --- | --- | --- |
| `spec-path` | string | The spec examined, repo-relative. |
| `relocated` | list of strings | Artifacts written. Empty on a converged spec. |
| `disagreements` | list of strings | `file:key` for every key where the spec block and an existing artifact carried different values. The block wins; the difference is reported rather than resolved, because it is the only evidence the two copies ever drifted. |
| `changed` | bool | Whether `spec.md` was rewritten. `false` on a converged spec — what makes a re-run over a partially migrated corpus safe. |

**Notes.** There is no `conflicts` field. An existing artifact is merged into,
not refused: every pre-migration spec has a `review.md`, so refusal would have
made the review record unmigratable.

## `Frontmatter` — `specs/{feature}/spec.md`

| Field | Change |
| --- | --- |
| `status` | unchanged |
| `dependencies` | unchanged |
| `tags` | unchanged |
| `folds-into` | unchanged |
| `cross-spec-impact` | unchanged |
| `review` | **removed** |
| `analyze` | **removed** |

**Notes.** `Frontmatter` at `primitives.rs:697` loses two `Option<…>` fields. The
open-schema rule means a residual block in an unmigrated tree deserializes
harmlessly — which is why AC6 makes `validate-frontmatter` report it explicitly
rather than relying on the type to reject it.
