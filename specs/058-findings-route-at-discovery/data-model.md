# 058 — Findings route at discovery Data Model

The shapes 058 adds to or removes from the two audit records and the
primitives that write them. Field shapes follow `ReviewBlock` and
`AnalyzeBlock` in `runtime/src/schema/primitives.rs`, and the waiver record in
the 020 data model.

## `dispositions` — in both `review.md` and `analysis.md` frontmatter

```yaml
dispositions:
  fixed: 1
  routed: 2
  discarded: 1
  undispositioned: 0
```

| Field | Type | Notes |
| --- | --- | --- |
| `fixed` | u32 | Findings fixed in the run. A fixed finding is gone from the re-check, so it is counted from the run's first pass. |
| `routed` | u32 | Live findings routed this run, plus live findings matching a stored routed decision. |
| `discarded` | u32 | Live findings discarded this run, plus live findings matching a stored discarded decision. |
| `undispositioned` | u32 | Live findings with neither. Recorded and reported; the gate does not read it, since only a gating finding holds `done`. |

**Notes.**

- **What is counted.** In `review.md` the map counts **observations** only.
  MUST and SHOULD violations keep their own counts and their fix-or-waive
  model. In `analysis.md` it counts findings of every tier.
- **Absent is not zero.** `Option<Dispositions>` on both structs. `None` means
  the record predates 058, and the gate blocks an `in-progress` spec on it.
  `Some` with every count zero means a run that found nothing.
- **Derived, never authored.** Both writers compute the map from the
  dispositions they are given. `analysis.md`'s `undispositioned` is the live
  tier total (`hard-fail + blocking-findings + advisory`) minus the live
  findings carrying `routed` or `discarded`. A finding the caller does not
  itemize therefore counts as undispositioned.

## `decisions` — the persisted list, in both records

```yaml
decisions:
  - key: "grounding — plan.md cites runtime/src/foo.rs:12, which does not exist"
    outcome: routed
    target: specs/031-some-feature/scenarios/foo-grounding.md
    decided-at: 2026-09-25T14:40:00Z
    decided-by: dev@example.com
  - key: "applicable-rules — BE-AUTHN-001 is listed but its trigger does not fire"
    outcome: discarded
    reason: "Spec cites the rule for a future endpoint; no trigger surface yet"
    decided-at: 2026-09-25T14:41:00Z
    decided-by: dev@example.com
```

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `key` | string | yes | Analyze: `{family} — {message}`. Review: the observation's rendered line, as `observation_line` builds it — its text, then an em dash and its path in backticks, or the text alone. |
| `outcome` | `routed` \| `discarded` | yes | `fixed` is never stored: a fixed finding stops firing. `undispositioned` is never stored: it is the absence of a decision. |
| `target` | string (repo-relative path) | when `routed` | The scenario, `tasks.md`, spec, or rule file the finding was routed to. |
| `reason` | string | when `discarded` | Free text; an empty string is invalid. |
| `decided-at` | ISO 8601 timestamp | yes | The run's own timestamp (`reviewed-at` or `analyzed-at`), stamped by the writer when the decision is new. A re-matched decision keeps its original. |
| `decided-by` | string (email) | yes | The writer's `decided-by` argument — `git config user.email`, as `waived-by` is. |
| (additional fields) | any | no | Open-schema, preserved verbatim on re-render, as waiver extras are. |

**Lifecycle.** The rules mirror review waivers (`framework/commands/review.md`
§Per-run waiver processing, §Malformed and duplicate waivers):

- **Matched.** The key fired this run. The entry is re-rendered unchanged, and
  the finding counts under its outcome without being asked about.
- **Expired.** The key did not fire, and the run evaluated its source. The
  entry is dropped on this write. For a routed finding, this is the moment the
  routed work landed.
- **Retained.** The key did not fire, but the run did not evaluate its source:
  a review with a pass that did not run (a dimension-restricting flag, or an
  empty scope), or an analysis that could not read a target it meant to
  examine (a skipped target in the *could not be read* class) or could not
  resolve a registered shared constitution. A target excluded by construction
  does not count
  ([only-unreadable-targets-retain-decisions](scenarios/only-unreadable-targets-retain-decisions.md)).
  The entry is re-rendered unchanged.
- **Malformed.** A required field is missing, or the outcome's companion field
  (`target` or `reason`) is missing. It produces a notice, is never expired, and
  applies to nothing. An entry with no fields at all is the one exception: it
  holds no decision to lose, so a re-render drops it, as it drops an all-empty
  waiver.
- **Duplicate key.** Keys compare flattened to one line, as the writers store
  them. The first entry applies; each duplicate produces a notice and is never
  applied. It is dropped when its key expires or is decided again, as a
  duplicate waiver goes with its key, because a new decision replaces every
  stored entry for that key.
- **Unparseable list.** `validate-frontmatter` reports it as a hard failure
  naming the file, and the writers refuse to write rather than treat it as
  empty. So does a record whose frontmatter does not parse at all, since its
  list cannot be read either. `write-analysis` overwrites a prior
  `analysis.md` that opens no frontmatter block: it stores no decisions, and
  re-running `/{project}:analyze` repairs it.

A hard-fail or blocking analyze finding is never stored as `discarded`, because
the writer rejects that outcome for those tiers. A stored `routed` decision for
one keeps the finding blocking until the re-check no longer produces it.

## Removed fields

| Where | Field | Replaced by |
| --- | --- | --- |
| `review.md` frontmatter | `captured-issues` | `dispositions` |
| `analysis.md` frontmatter | `captured-issues` | `dispositions` |
| `review.md` body | `## Captured issues` | — (observations carry their disposition inline) |
| `analysis.md` body | `## Captured issues` | `## Fixed in this run` |
| `compute-review-scope` result | `captured-issues` | — |
| `diff-cross-spec` result | `inbox-additions`, `inbox-standing` | — |
| `write-review` args | `captured-issues` | — |
| `write-review` result | `observations-captured`, `inbox-standing` | `dispositions` |
| `write-analysis` args | `captured-issues` | `findings` |
| `write-analysis` result | `captured-issues` | `dispositions` |
| `append-inbox` args | `dedup-prefix` | — |
| `append-inbox` result | `deduped` | — |

`append-inbox`'s dedup guard existed for `/{project}:implement`'s auto-capture
and the adoption security audit, the two writers 058 removed; `/{project}:log`
never passed it, so it had no caller left. A caller that still passes
`dedup-prefix` is refused on MCP and the CLI, which reject an unknown argument
by name; only the exec interpreter, which binds a primitive's arguments from a
wider context, ignores it, and the append then always happens.

A record written before 058 that carries `captured-issues` still parses,
because neither record struct denies unknown fields. The key is dropped the
next time its writer runs.

## Changed primitive inputs

Neither writer takes a list of new decisions. Each derives them from the
dispositioned findings it is handed — every routed or discarded live finding
not already stored with the same outcome — and requires `decided-by` only when
at least one is new. The shared disposition shape is `Disposition`
(`outcome`, a `DispositionOutcome`, plus `target` and `reason`).

### `write-review` args

| Field | Type | Notes |
| --- | --- | --- |
| `observations` | list of `ReviewObservation` | Each now carries its disposition (below). |
| `expired-decisions` | list of decision refs | From `process-decisions`' `expired`; dropped on this write. |
| `decided-by` | string, optional | Author of this run's new decisions. Required when any observation is newly routed or discarded. |

### `ReviewObservation` — the `write-review` `observations` entry

| Field | Type | Notes |
| --- | --- | --- |
| `text` | string | Unchanged. Single line, non-empty. |
| `path` | string, optional | Unchanged. |
| `disposition.outcome` | `fixed` \| `routed` \| `discarded` \| `undispositioned` | New. Defaults to `undispositioned`. |
| `disposition.target` | string | Required when `routed`. |
| `disposition.reason` | string | Required when `discarded`. |
| `decision-key` | string, optional | The key of a stored decision the host matched this observation to. |

### `write-analysis` args

| Field | Type | Notes |
| --- | --- | --- |
| `findings` | list of `AnalysisFinding` | Replaces `captured-issues`. |
| `expired-decisions` | list of decision refs | From `process-decisions`' `expired`; dropped on this write. |
| `decided-by` | string, optional | Author of this run's new decisions. Required when any live finding is newly routed or discarded. |

The tier counts stay host-supplied scalars from the re-check.

### `AnalysisFinding` — the new `write-analysis` `findings` entry

Its optional `decision-key` names the stored decision the host matched the
finding to, because many analyze messages are host-worded and do not
reproduce byte for byte (scenario
`analyze-findings-match-decisions-by-host-judgment`). Absent, the key is
`{family} — {message}`.

| Field | Type | Notes |
| --- | --- | --- |
| `tier` | `AnalysisTier`: `hard-fail` \| `blocking` \| `advisory` | Tier as detected. Required: the discard refusal reads it, so no default is safe. |
| `family` | string | The detecting family, e.g. `review-state-drift`, `grounding`. |
| `message` | string | Single line. With `family`, forms the key. |
| `path` | string | The citing artifact. |
| `live` | bool | `false` for a finding fixed in the run and absent from the re-check. Defaults to `true`. |
| `disposition` | as above | `discarded` is rejected when `tier` is `hard-fail` or `blocking`. |
| `decision-key` | string, optional | The key of a stored decision the host matched this finding to. A blank one is no key. |

### `AppendTaskArgs.dedup-title`

`bool`, default `false`. When `true`, an existing **pending** task section whose
title equals `title` and whose checkbox items equal the ones this call would
write is returned with `appended: false` rather than appended again. The items
are part of the match because a disposition task's title carries only the
finding's summary, and its `{path} — {detail}` body item is what tells two
findings sharing a summary apart. A spent section never matches.

## New primitive: `process-decisions`

| Arg | Type | Notes |
| --- | --- | --- |
| `feature` | string | Feature directory under the spec root. |
| `record` | `review` \| `analysis` | Which record's `decisions:` to read. |
| `fired` | list of strings | This run's finding keys (MCP only, as `process-waivers`' `fired` is). |
| `restricted` | bool | The run did not evaluate every source: a review pass did not run, or an analysis could not read a target it meant to examine or could not resolve a registered shared constitution. |

| Result field | Type | Notes |
| --- | --- | --- |
| `matched` | list of decision refs | Key, outcome, target or reason. |
| `expired` | list of decision refs | Passed to the writer as `expired-decisions`. |
| `retained` | list of decision refs | Re-rendered unchanged. |
| `notices` | list of strings | Malformed, duplicate, expired, and retained entries, each named. |

## New gate blocks — `ReviewGateBlock`

| Variant | When | Message names |
| --- | --- | --- |
| `record-predates-dispositions` | An `in-progress` spec's `review.md` or `analysis.md` has no `dispositions:` map | The record, and the command to re-run |
| `undispositioned-findings` | An outstanding gating finding: the review record's unwaived MUST and SHOULD violations, or the analysis's hard-fail and blocking findings, above zero. The wire name predates scenario `findings-cost-in-proportion-to-risk`, when it read `dispositions.undispositioned` | The count per record, and the command to re-run |

Both are checked after every existing review and analyze check, in the order
predates (review, then analysis), then outstanding gating findings (review, then analysis).

## New `check-artifacts` family — `disposition-drift`

At `done`, `Advisory`, one finding when `review.md`'s `undispositioned` is
above zero, with `path` = `spec.md`. A record without a `dispositions:` map
produces nothing. `analysis.md` is not judged here: `/{project}:analyze` judges
it after writing it, from the record it wrote
([analysis-drift-judges-the-record-it-writes](scenarios/analysis-drift-judges-the-record-it-writes.md)). `/{project}:analyze --fix` does not revert on it: the findings it
counts do not hold `done`.

## `DashboardResult.inbox-standing`

The existing `InboxStanding` (in `primitives.rs`), moved from the
`write-review` and `diff-cross-spec` results. `render_callouts` renders it as
one `Inbox:` line in one of four states:

- `Inbox: N item(s) outstanding, oldest YYYY-MM-DD — run /{project}:groom to route`
- `Inbox: N item(s) outstanding, age undeterminable — run /{project}:groom to route`
- `Inbox: ✓ clean`
- `Inbox: ? no readable {specs-root}/inbox.md — nothing examined`
