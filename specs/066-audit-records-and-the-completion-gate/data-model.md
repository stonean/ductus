# 066 — Audit Records and the Completion Gate Data Model

The argument and result shapes of the primitives that write a spec's audit records and gate its completion. They live in `runtime/src/schema/primitives.rs` as Rust types with kebab-case `serde` derives, and their JSON is the stable host contract, under the conventions [022's data model](../022-deterministic-runtime/data-model.md) states for every primitive: the protocol envelope, the severity tiers, and its closing notes on paths, timestamps and shas. The records' frontmatter fields are the constitution's (§text-first-artifacts, Audit records); `review.md`'s body and waiver record are [020's](../020-code-review/data-model.md), and the `dispositions:` and `decisions:` shapes are [058's](../058-findings-route-at-discovery/data-model.md).

## `compute-review-scope` — the review's diff base and scope

Args:

```json
{ "feature": "042-widget", "since": null }
```

Result:

```json
{
  "diff-base": "<sha>",
  "scope": ["runtime/src/primitives/read_spec.rs", "specs/042-widget/data-model.md"],
  "modified-since": ["specs/042-widget/data-model.md"],
  "plan-affected": ["runtime/src/primitives/read_spec.rs"]
}
```

`diff-base` is the parent of the commit that moved the spec into `in-progress`; `HEAD`'s sha while that move is uncommitted; empty when the spec never reached `in-progress` and no `since` was given; or `since` verbatim when supplied. `modified-since` is every file changed between `diff-base` and the working tree inside the project directory — committed, staged, unstaged and untracked but not ignored — named from the project root and sorted. `plan-affected` is the plan's `## Affected Files` list, and `scope` is the union of the two (scenarios `review-base-includes-the-transition-commit`, `review-scope-includes-uncommitted-work`, `review-scope-union`).

## `process-waivers` — classify recorded waivers before the write

Args:

```json
{
  "feature": "042-widget",
  "fired": [{ "rule": "BE-INPUT-001", "file": "src/api.rs" }],
  "skipped-passes": []
}
```

Result:

```json
{
  "applied": [{ "rule": "BE-INPUT-001", "file": "src/api.rs", "reason": "validated upstream" }],
  "expired": [],
  "retained": [],
  "notices": []
}
```

Reads the `waivers` list in `review.md`'s frontmatter and classifies each `(rule, file)` anchor against the passes' findings — a waiver listing several paths contributes one anchor per path (scenario `process-waivers-file-lists`). **applied**: the file exists and the rule still fires there. **expired**: it does not, and `write-review` drops that path on the next write, and the waiver with its last path. **retained**: the run was dimension-restricted (`skipped-passes` non-empty) and the anchor did not fire against the passes that ran, so it is left untouched — a partial run never prunes. `notices` carries the `waiver expired` / `waiver retained` / `malformed waiver` / `duplicate waiver` lines in entry order. `fired` is MCP-only.

## `process-decisions` — classify stored decisions before dispositions

Args:

```json
{ "feature": "042-widget", "record": "analysis",
  "fired": ["grounding — plan.md cites a missing path"], "restricted": false }
```

Result:

```json
{ "matched": [{ "key": "grounding — plan.md cites a missing path", "outcome": "routed", "target": "specs/042-widget/tasks.md" }],
  "expired": [], "retained": [], "notices": [] }
```

Classifies the routed and discarded decisions in `review.md`'s or `analysis.md`'s `decisions:` list against this run's finding keys, before the host proposes any disposition, so a settled finding is counted under its stored outcome rather than asked about again. **matched**: the key fired. **expired**: it did not, on an unrestricted run, and the writer drops it — for a routed finding, the moment the routed work landed. **retained**: it did not, but `restricted` says the run did not evaluate every source. A malformed entry or a repeated key produces a notice; the primitive prunes nothing. `fired` is MCP-only, and each key is compared in the flattened single-line form the writers store. Read-only; a `decisions:` list that does not parse is an error, never an empty result.

## `write-review` — observations and the review's denominator

The finding, waiver and scope arguments are 020's. This section adds the observations and the `examined` / `scope` pair.

Args (the added half only):

```json
{ "observations": [{ "text": "perf: the config file is re-read on every primitive call", "path": "runtime/src/schema/paths.rs",
  "disposition": { "outcome": "routed", "target": "specs/022-deterministic-runtime/tasks.md" } }],
  "decided-by": "dev@example.com", "examined": 38 }
```

Result (the added half only):

```json
{ "observations": 1, "dispositions": { "fixed": 0, "routed": 1, "discarded": 0, "undispositioned": 0 }, "examined": 38, "scope": 46 }
```

An **observation** is something the reviewer judged real that maps to no loaded rule, so it cannot be a finding. `path` is optional. `text` must be non-empty, and `text`, `path` and `decision-key` must each be one line, since an observation renders as one list item and a decided one's line becomes its decision key; a newline or control character in any of them is refused before anything is written. Observations never enter `must-violations` / `should-violations` / `low-confidence`, never affect `blocking`, and never change the exit code; they render in their own `## Observations` report section, after `## Waived findings` and before `## Skipped passes`, which emits `*None.*` when empty. The report frontmatter carries no `observations:` key.

Each observation carries a disposition — fixed, routed with its target, discarded with its reason, or undispositioned by default — rendered beside it. The record counts the four outcomes in `dispositions:`, and a newly routed or discarded observation is stored under `decisions:`; nothing is written to the inbox. `performReview`'s response carries an optional `observations` array, accumulated across passes as `findings` is; each entry carries `text` and `path` alone, so an exec run records every observation undispositioned (required by spec [017 — Derive, Don't Ask](../017-derive-dont-ask/spec.md) AC25).

**`scope` is derived and `examined` is claimed.** `scope` is resolved by the primitive through `compute-review-scope` against the run's own `diff-base`, so no caller can shrink the subject to match what it read; `examined` is the caller's count of in-scope files the passes actually read, which only the host knows. Both are written to `review.md`'s frontmatter; an unstated `examined` is recorded as absent, never as zero. Nothing reads the pair; a person reads it at the completion gate (scenario `a-review-states-what-it-read`).

## `write-analysis` — record that analyze ran

Args:

```json
{
  "feature": "042-widget", "analyzed-at": "2026-09-06T14:00:00Z",
  "analyzed-against": "abc123", "hard-fail": 0, "blocking-findings": 0,
  "advisory": 8,
  "unexamined-by-reason": [["not-a-live-claim", 79], ["root-absent", 26]],
  "findings": [{ "tier": "advisory", "family": "link-adjacent-drift",
    "message": "...", "path": "specs/042-widget/spec.md",
    "disposition": { "outcome": "discarded", "reason": "..." } }],
  "decided-by": "dev@example.com"
}
```

Result:

```json
{ "spec-path": "specs/042-widget/spec.md", "blocking": false, "unexamined": 105, "replaced": true,
  "dispositions": { "fixed": 0, "routed": 0, "discarded": 1, "undispositioned": 7 } }
```

Writes `specs/{feature}/analysis.md` — the run's record in its frontmatter and the report body's fixed skeleton — and refuses a spec whose `spec.md` frontmatter does not parse rather than recording a clean run into it. Each finding arrives with its tier and disposition. The writer derives `dispositions:`, counting a live finding the call does not itemize as undispositioned, and stores each routed or discarded finding under `decisions:`, keyed `{family} — {message}` or by the `decision-key` the host matched it to. It refuses a discard on a hard-fail or blocking finding, which is why `tier` is required, and requires `decided-by` when a decision is new.

**Derived, never accepted:**

- `blocking` — from `hard-fail` and `blocking-findings`.
- `unexamined` — summed from `unexamined-by-reason` when a breakdown is supplied, so the total and its parts cannot disagree; a caller-supplied total is used only when there is no breakdown.
- Two reasons the writer adds to `unexamined-by-reason` itself: `constitution-unresolved`, one per registered shared constitution that did not resolve (spec 055), and `constitution-registry-unreadable` when the `[constitutions]` registry will not parse. Each is an unexamined input rather than a skipped target, so neither is in the table below, and each counts toward `unexamined`.
- `analyzed-digest` / `analyzed-unreadable` — taken by the primitive from the feature directory through the same function `check-review-gate` recomputes them with; a subject that exists but could not be read is recorded in `analyzed-unreadable` rather than digested as empty. `write-review` derives `reviewed-digest` / `reviewed-unreadable` the same way over the narrower contract set.

**`unexamined-by-reason` is what makes `unexamined` actionable**, because the reasons split into classes calling for opposite responses:

| class | reasons | response |
| --- | --- | --- |
| excluded by construction | `not-a-live-claim`, `ships-to-adopter`, `root-absent`, `never-existed` | none; the exclusion is correct |
| could not be read | `target-missing`, `target-unparseable`, `no-readable-state`, `referrer-unreadable`, `rule-citations-not-checked`, `rule-assessments-not-checked`, `rule-file-unreadable`, `references-not-checked`, `applicable-rules-not-checked`, `grounding-not-checked` | a real gap in what the run saw |
| the spec's own artifact | `artifact-unreadable` | **blocking at `done`**, where it is a finding, never a skip |

The first three `could not be read` reasons are `check-artifacts` skips. The rest are gaps no check decided on: `referrer-unreadable` counts the referrers `check-orphaned-references` reports in `skipped`; `rule-citations-not-checked` counts the spec's citations when `check-rule-ids` read no rule file (`examined: 0`), recorded unexamined rather than missing; `rule-assessments-not-checked` counts the loaded rules steps 11 and 12 did not ask about — a rule with no Verification, or whose Statement carries no RFC 2119 keyword to take a tier from — and, on the exec path, a step that loaded no rule at all or whose prose names no MUST or SHOULD tier; `rule-file-unreadable` counts the rule files steps 11 and 12 could not read, whose rules could not even be counted (spec 060); and `references-not-checked`, `applicable-rules-not-checked` and `grounding-not-checked` are `/{project}:analyze` steps 13–15, host prose the exec walker no-ops, recorded once each on every exec run. The exec walker tallies every one of these. On the interactive path the host records `referrer-unreadable` and `rule-citations-not-checked`, and records `rule-file-unreadable` and `rule-assessments-not-checked` for a rule file it could not read or a rule it could not place in either tier; the exec path's per-step count and steps 13–15's reasons do not arise there, since the host runs those steps itself.

`artifact-unreadable` is the one reason that escalates: below `done` it is a skipped target, and at `done` it is a blocking finding counted in `blocking-findings`, so the gate holds the spec. The reason set is closed and the map is omitted when empty.

**The subject depends on the spec's status when the record is written.** `criterion-path-existence`, the source of most skipped targets, examines `done` specs only, so a record written while a spec is reopened describes a smaller subject than the same spec at `done`. Write the record last, after the status is settled.

## `check-review-gate` — evaluate implement's pre-done review gate

Args:

```json
{ "feature": "042-widget" }
```

Result:

```json
{
  "passed": false,
  "blocked-by": "must-violations",
  "message": "blocked: spec has 3 MUST violation(s) — see specs/042-widget/review.md",
  "guidance": "Resolve the violations and re-run /ductus:review, or run /ductus:review --waive <rule-id> --reason \"...\" for each waivable finding.",
  "violations": []
}
```

The deterministic surface behind `/ductus:implement`'s completion gate. Evaluates every check in documented order, first failure wins. `blocked-by` names the failing one:

0. **`already-done`** — the spec is at `status: done`. A blocked variant, never `passed: true`, ordered first so a completed spec is never held against a lint violation introduced after it closed, or against the freshness checks its own completing `set-status` disturbs. Only the exact value `done` short-circuits; an unrecognized status falls through, since `validate-frontmatter` owns reporting it (scenario `a-done-spec-has-no-transition-to-gate`).
1. **`markdown-lint`** — the feature directory's markdown lint (recursive `{root}/{feature}/**/*.md` through the `lint-markdown` machinery). Violations are echoed in `violations`, or a non-zero exit the parser could not attribute.
2. **`scenario-open-questions`** — one or more scenarios carry unresolved open questions; the message names the count and the scenarios, and `guidance` points at scenario-targeted `/{project}:clarify`. The list comes from `read-spec`'s collector ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)).
3. **`undischarged-cross-spec-impact`** — see **Cross-spec impact** below.
4. **`not-reviewed`** (`review.md` absent, or its `last-run` missing or null) and 5. **`must-violations`** (`blocking: true`) — the review record, read from `review.md`. A `review.md` that exists but carries no parseable record is **undeterminable**, reported as such rather than as never-reviewed.
6. **`review-stale`** — see **Record freshness** below.
7. **`not-analyzed`** (`analysis.md` absent, or its `last-run` missing or null) and 8. **`analyze-findings`** (`blocking: true`) — the analyze record, read from `analysis.md`, with the same undeterminable third state, ordered after every review check because the pipeline is `review → analyze → done`. An advisory analyze finding never blocks (scenario `write-analysis-and-the-second-gate`).
9. **`analyze-stale`** — see **Record freshness** below.
10. **`record-predates-dispositions`** — `review.md` or `analysis.md` has no `dispositions:` map; absence is not zero. The message names the record, review before analysis, and the command to re-run.
11. **`undispositioned-findings`** — either record's `dispositions.undispositioned` is above zero. The block asks for a decision, not a fix: discarding a false positive with its reason clears it.

Within each record's half the staleness check comes last: the others say a record is missing or failing, it says a passing record is out of date. Checks 10 and 11 follow every other review and analyze check, each of which names a more upstream defect. **No check reads `folds-into`**: a pending fold never holds `done` ([051](../051-branch-scoped-spec-numbering/spec.md), scenario `a-pending-fold-never-holds-done`).

`message` is the canonical blocked text with the adopter's `[host] project` command namespace substituted; `guidance` carries the resolve-or-waive follow-up on `must-violations` and the scenario-targeted-clarify pointer on `scenario-open-questions`. Every verdict is a domain outcome — the host halts on a blocked gate, and the primitive never errors for one. An unreadable scenario contributes no questions and never blocks; `read-spec` and `check-artifacts` report it, so not blocking and not reporting stay separate obligations.

### Cross-spec impact

`cross-spec-impact:` is a hand-authored list of feature slugs a spec owes a change to under §cross-spec-impact ([050](../050-constitution/spec.md), scenario `a-declared-cross-spec-impact-gates-done`). `cross-spec-impact` in the result is a per-entry list:

| state | meaning |
| --- | --- |
| `discharged` | the named spec links back to the declaring spec, from its own body or from a scenario under it |
| `undischarged` | the named spec exists and carries no such link |
| `target-missing` | the entry names no feature directory — reported, never treated as discharged |
| `self-reference` | the entry names the declaring spec, which cannot owe itself a change |

**Discharge is the reciprocal link, not the key's removal**, read with `derive-dependencies`' matcher over the **pointer** scope rather than the edge scope, which drops blockquoted lines — a signpost on a `done` spec is written as a blockquote, and the edge scope would hide it. The list is empty both when the key is absent or empty and when an earlier check blocked first, and `blocked-by` tells them apart; on a pass it is populated. There is no `--fix`: which section of the affected spec carries the signpost is the routing judgment `/{project}:amend` puts to the operator. A missing target blocks here, where an owed fold never blocks at all (scenario `the-cross-spec-impact-gate`).

### Record freshness

Both staleness checks are one computation — `analyze_subjects::freshness_of`, returning `RecordFreshness` (`never-run` / `current` / `stale` / `undeterminable`) — over different subjects:

| record | subject set | digest field |
| --- | --- | --- |
| `review.md` | `scenarios/*.md`, `data-model.md` | `reviewed-digest` |
| `analysis.md` | every `.md` under the feature — `review.md` and `spec.md` included, `analysis.md` itself **excluded** | `analyzed-digest` |

`review.md` and `spec.md` are the review's output and a body it does not judge, so counting them would stale every review; both are analyze's inputs, so excluding them there would exempt the edits that most often invalidate an analysis. `analysis.md` is excluded from its own set because the record is written after its subjects are read.

**The comparison is of content, not commits**: each record carries a per-path sha256 of what its run read from the working tree. `reviewed-against` and `analyzed-against` are provenance, read only for the mechanical-sweep rename exemption, whose candidates are reported rather than dropped when a tree is unavailable (scenarios `the-committed-tree-horizon`, `review-staleness-on-done-specs`). **A record with no digest is `undeterminable`** — not current, not stale, not a sha-diff fallback — and does not block, so a passing gate still carries `guidance` naming it. `reviewed-digest` is an `Option` because a spec with no scenarios and no data model records an **empty** digest, which is current; `spec.md` is always an analyze subject, so an empty analyze digest cannot occur.

## `diff-cross-spec` — implement's cross-spec impact surface

Args:

```json
{ "feature": "042-widget" }
```

Result:

```json
{
  "first-commit": "b39f2727dc6939ad145ede1830205e0d122075d3",
  "current-head": "550c8ddc33e6895da0ce6c81fa6f6e2c42049e9f",
  "cross-spec-paths": ["specs/007-sibling/spec.md"]
}
```

The filter behind `/ductus:implement`'s per-task summary and completion gate. Diffs the feature's first spec-dir commit — the base `derive-boundary` computes, through the shared `first_commit_for_prefix` revwalk — against the **working tree** (index and untracked files included), scoped to the spec root: `cross-spec-paths` lists changed paths outside the feature's own directory, sorted, with `{specs-root}/inbox.md` excluded. The working-tree diff is why the per-task summary, which fires before the task's commit, sees the run's uncommitted sibling edits. Read-only; an empty list is the no-impact domain outcome.
