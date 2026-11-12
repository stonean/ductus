# 066 — Audit Records and the Completion Gate Data Model

The argument and result shapes of the primitives that write a spec's audit records and gate its completion. They live in `runtime/src/schema/primitives.rs` as Rust types with kebab-case `serde` derives, and their JSON is the stable host contract, under the conventions [022's data model](../022-deterministic-runtime/data-model.md) states for every primitive: the protocol envelope, the severity tiers, and its closing notes on paths, timestamps and shas. The records' frontmatter fields are the constitution's (§text-first-artifacts, Audit records); `review.md`'s body and waiver record are [020's](../020-code-review/data-model.md), and the `dispositions:` and `decisions:` shapes are [058's](../058-findings-route-at-discovery/data-model.md).

## `compute-review-scope` — the review's diff base and scope

Args:

```json
{ "feature": "042-widget", "since": null }
```

Result, inline and past the read size:

```json
{
  "diff-base": "<sha>",
  "base-source": "reopen",
  "review-owed": true,
  "form": "inline",
  "scope-count": 2,
  "modified-since-count": 2,
  "plan-affected-count": 1,
  "deleted-count": 1,
  "excluded-count": 0,
  "scope": ["runtime/src/primitives/read_spec.rs", "specs/042-widget/data-model.md"],
  "modified-since": ["specs/042-widget/data-model.md", "specs/042-widget/research.md"],
  "plan-affected": ["runtime/src/primitives/read_spec.rs"],
  "deleted": ["specs/042-widget/research.md"],
  "excluded": []
}
```

```json
{
  "diff-base": "<sha>",
  "base-source": "reopen",
  "review-owed": true,
  "form": "file",
  "scope-count": 1056,
  "modified-since-count": 1049,
  "plan-affected-count": 12,
  "deleted-count": 3,
  "excluded-count": 0,
  "lists-file": ".ductus/results/compute-review-scope/042-widget/<sha256>.json"
}
```

`diff-base` is the later of the parent of the commit that moved the spec into `in-progress` and the last finished review commit since (`compute-review-scope`; `base-source` names which); `HEAD`'s sha while that move is uncommitted; empty when the spec never reached `in-progress` and no `since` was given; or `since` verbatim when supplied. `modified-since` is every file changed between `diff-base` and the working tree inside the project directory — committed, staged, unstaged and untracked but not ignored — named from the project root and sorted. `plan-affected` is every path the first cell of a plan `## Affected Files` table row names: its first backticked span, and each later one holding no whitespace and starting with none of `-`, `/` and `§`. `deleted` is the `modified-since` paths absent from the working tree, `excluded` those the scope rule leaves out, and `scope` what `compute-review-scope`'s rule keeps of `modified-since`, with `plan-affected` on a first review, less `deleted`, so the denominator holds only what a review can read (scenarios `review-base-includes-the-transition-commit`, `review-scope-includes-uncommitted-work`, `review-scope-union`, `review-scope-reads-every-cell-path-and-sets-deleted-files-apart`).

**Size** (scenario `scope-results-have-a-bounded-size`). The lists are inline, `form: "inline"`, while the result's compact JSON is at most `[artifacts] read-size-bytes`, 50,000 bytes by default; a result exactly at it fits. Past it, `form` is `"file"`, no list is inline, and `lists-file` names, from the project root, a file holding `diff-base` and the five lists whole as pretty-printed JSON, one entry per line, under `.ductus/results/compute-review-scope/{feature}/` and named by the sha256 of its bytes, so a concurrent call never reads another's lists. The stated maximum is that setting, or, below it, the file form's own size, which no list enlarges: 354 bytes of compact JSON with a 40-character sha and the longest `base-source`, plus the feature directory's name and each count digit past the first. The counts are inline in both forms, so a zero `scope-count`, never an absent list, is an empty scope. `.ductus/results/` ignores itself (a `.gitignore` of `*`), so no list file enters a scope, and a write removes the same feature's list files older than 24 hours. A malformed read size halts the call, as it halts every reader of the setting, and a list file that cannot be written fails it. `write-review` derives `scope` from the unbounded lists, and `ductus exec` reads the lists back from the list file at the `compute-review-scope` step, halting there with `lists-file-unreadable` when it cannot, so `performReview`'s request carries `scope` whole (spec 080).

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

Reads the `waivers` list in `review.md`'s frontmatter and classifies each `(rule, file)` anchor against the passes' findings — a waiver listing several paths contributes one anchor per path (scenario `process-waivers-file-lists`). **applied**: the file exists and the rule still fires there. **expired**: it does not, and `write-review` drops that path on the next write, and the waiver with its last path. **retained**: the run was dimension-restricted (`skipped-passes` non-empty) and the anchor did not fire against the passes that ran, so it is left untouched — a partial run never prunes. `notices` carries the `waiver expired` / `waiver retained` / `malformed waiver` / `duplicate waiver` lines in entry order, each stored value printed flattened to one line (scenario `process-waivers-notices-flatten-stored-text`). `fired` is MCP-only.

## `process-decisions` — classify stored decisions before dispositions

Args:

```json
{ "feature": "042-widget", "record": "analysis",
  "fired": ["grounding — plan.md cites a missing path"], "restricted": false }
```

Result:

```json
{ "form": "inline", "matched-count": 1, "expired-count": 0, "retained-count": 0, "lapsed-count": 0, "notices-count": 0,
  "matched": [{ "key": "grounding — plan.md cites a missing path", "outcome": "deferred", "target": "specs/042-widget/tasks.md", "task": "3" }],
  "expired": [], "retained": [], "lapsed": [], "notices": [] }
```

Classifies the deferred and waived decisions in `review.md`'s or `analysis.md`'s `decisions:` list against this run's finding keys, before the host gives any disposition, so a settled finding is counted under its stored outcome rather than decided again. **matched**: the key fired. **expired**: it did not, on an unrestricted run, and the writer drops it — for a deferred finding, the moment its work landed. **retained**: it did not, but `restricted` says the run did not evaluate every source. **lapsed**: it fired, but the pipeline work its deferral names is complete — a task checked, or a spec `done` — so the writer drops it and the finding needs a new disposition (the primitive's description says when). A malformed entry or a repeated key produces a notice; the primitive prunes nothing. `fired` is MCP-only, and each key is compared in the flattened single-line form the writers store. Read-only but for a list file; a `decisions:` list that does not parse is an error, never an empty result.

**Size** (scenario `process-decisions-results-have-a-bounded-size`). The result is bounded as `compute-review-scope`'s is (its **Size**): past `[artifacts] read-size-bytes`, `form` is `"file"`, none of the five lists is inline, and `lists-file` names a file under `.ductus/results/process-decisions/{feature}/` holding `matched`, `expired`, `retained`, `lapsed` and `notices` whole. Each list's count is inline in both forms, so a zero count, never an absent list, is an empty list. The file form's own size is 225 bytes of compact JSON plus the feature directory's name and each count digit past the first.

## `write-review` — observations and the review's denominator

The finding, waiver and scope arguments are 020's. This section adds the observations and the `examined` / `scope` pair.

Args (the added half only):

```json
{ "observations": [{ "text": "perf: the config file is re-read on every primitive call", "path": "runtime/src/schema/paths.rs",
  "disposition": { "outcome": "deferred", "target": "specs/022-deterministic-runtime/tasks.md", "work-item": "PROJ-1234" } }],
  "decided-by": "dev@example.com", "examined": 38 }
```

Result (the added half only):

```json
{ "observations": 1, "dispositions": { "fixed": 0, "deferred": 1, "waived": 0, "undispositioned": 0 }, "examined": 38, "scope": 46 }
```

An **observation** is something the reviewer judged real that maps to no loaded rule, so it cannot be a finding. `path` is optional. `text` must be non-empty, and `text`, `path` and `decision-key` must each be one line, since an observation renders as one list item and a decided one's line becomes its decision key; a newline or control character in any of them is refused before anything is written. Observations never enter `must-violations` / `should-violations` / `low-confidence`, never affect `blocking`, and never change the exit code; they render in their own `## Observations` report section, after `## Waived findings` and before `## Skipped passes`, which emits `*None.*` when empty. The report frontmatter carries no `observations:` key.

Each observation carries a disposition — fixed, deferred with its work, waived with its reason, or undispositioned by default — rendered beside it. The record counts the four outcomes in `dispositions:`, and a newly deferred or waived observation is stored under `decisions:`; nothing is written to the inbox. `performReview`'s response carries an optional `observations` array, accumulated across passes as `findings` is; each entry carries `text` and `path` alone, so an exec run records every observation undispositioned (required by spec [017 — Derive, Don't Ask](../017-derive-dont-ask/spec.md) AC25).

**`scope` is derived and `examined` is claimed.** `scope` is resolved by the primitive through `compute-review-scope` against the run's own `diff-base`, so no caller can shrink the subject to match what it read; `examined` is the caller's count of in-scope files the passes actually read, which only the host knows. Both are written to `review.md`'s frontmatter; an unstated `examined` is recorded as absent, never as zero. Nothing reads the pair; a person reads it at the completion gate (scenario `a-review-states-what-it-read`).

**The fix-round loop** adds the `fix-rounds` argument and the six record fields `write-review` derives from it, defined in [078's data model](../078-review-fix-rounds-are-re-read-within-a-bound/data-model.md) (scenario `the-fix-round-loop-is-recorded-and-gated`). `invalidate-review` drops the six with the run's other fields: a round waiver names a round of that run.

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
    "disposition": { "outcome": "waived", "reason": "..." } }],
  "decided-by": "dev@example.com"
}
```

Result:

```json
{ "spec-path": "specs/042-widget/spec.md", "blocking": false, "unexamined": 105, "replaced": true,
  "dispositions": { "fixed": 0, "deferred": 0, "waived": 1, "undispositioned": 7 } }
```

Writes `specs/{feature}/analysis.md` — the run's record in its frontmatter and the report body's fixed skeleton — and refuses a spec whose `spec.md` frontmatter does not parse rather than recording a clean run into it. Each finding arrives with its tier and disposition. The writer derives `dispositions:`, counting a live finding the call does not itemize as undispositioned, and stores each deferred or waived finding under `decisions:`, keyed `{family} — {message}` or by the `decision-key` the host matched it to. It refuses a waiver on a hard-fail or blocking finding, which is why `tier` is required, and requires `decided-by` when a decision is new.

**Derived, never accepted:**

- `blocking` — from `hard-fail` and `blocking-findings`.
- `unexamined` — summed from `unexamined-by-reason` when a breakdown is supplied, so the total and its parts cannot disagree; a caller-supplied total is used only when there is no breakdown.
- Two reasons the writer adds to `unexamined-by-reason` itself: `constitution-unresolved`, one per registered shared constitution that did not resolve (spec 055), and `constitution-registry-unreadable` when the `[constitutions]` registry will not parse. Each is an unexamined input rather than a skipped target, so neither is in the table below, and each counts toward `unexamined`.
- `analyzed-digest` / `analyzed-unreadable` — taken by the primitive from the feature directory through the same function `check-review-gate` recomputes them with; a subject that exists but could not be read is recorded in `analyzed-unreadable` rather than digested as empty. `write-review` derives `reviewed-digest` / `reviewed-unreadable` the same way over the narrower contract set; only `compute-review-scope` reads them (spec 081).

**`unexamined-by-reason` is what makes `unexamined` actionable**, because the reasons split into classes calling for opposite responses:

| class | reasons | response |
| --- | --- | --- |
| excluded by construction | `not-a-live-claim`, `ships-to-adopter`, `root-absent`, `never-existed`, `gitignored`, `status-gated`, `anchor-target-unjudgeable` | none; the exclusion is correct |
| could not be read | `target-missing`, `target-unparseable`, `no-readable-state`, `referrer-unreadable`, `anchor-target-unreadable`, `rule-citations-not-checked`, `rule-not-installed`, `rule-assessments-not-checked`, `rule-file-unreadable`, `references-not-checked`, `applicable-rules-not-checked`, `grounding-not-checked`, `integrity-not-checked` | a real gap in what the run saw |
| the spec's own artifact | `artifact-unreadable` | **blocking at `done`**, where it is a finding, never a skip |

The first three `could not be read` reasons are `check-artifacts` skips. The rest are gaps no check decided on: `anchor-target-unreadable` counts the `§` references `resolve-anchor` could not judge because a document their line names could not be read or a registered constitution did not resolve, and `anchor-target-unjudgeable`, its excluded-by-construction twin, those naming only absent documents or matching nothing beside a file that is not markdown (067's `shared-constitution-references-are-judged`); `referrer-unreadable` counts the referrers `check-orphaned-references` reports in `skipped`; `rule-citations-not-checked` counts the spec's citations when `check-rule-ids` read no rule file (`examined: 0`), recorded unexamined rather than missing; `rule-not-installed` counts the spec's citations of a shipped rule whose file the project never installed (`check-rule-ids`' `not-installed`), known and no finding, unexamined because no rule text was loaded (spec 033); `rule-assessments-not-checked` counts the loaded rules steps 11 and 12 did not ask about — a rule with no Verification, or whose Statement carries no RFC 2119 keyword to take a tier from — and, on the exec path, a step that loaded no rule at all or whose prose names no MUST or SHOULD tier; `rule-file-unreadable` counts the rule files steps 11 and 12 could not read, whose rules could not even be counted (spec 060); and `references-not-checked`, `applicable-rules-not-checked`, `grounding-not-checked` and `integrity-not-checked` are `/{project}:analyze` steps 13–15, host prose the exec walker no-ops, recorded once each on every exec run (`integrity-not-checked` is the part of step 15 judging §Spec integrity and §Plan consistency's semantic items, beside `grounding-not-checked`, its other part; spec 067). The exec walker tallies every one of these. On the interactive path the host records `referrer-unreadable` and `rule-citations-not-checked`, and records `rule-file-unreadable` and `rule-assessments-not-checked` for a rule file it could not read or a rule it could not place in either tier; the exec path's per-step count and steps 13–15's reasons do not arise there, since the host runs those steps itself.

`artifact-unreadable` is the one reason that escalates: below `done` it is a skipped target, and at `done` it is a blocking finding counted in `blocking-findings`, so the gate holds the spec. The reason set is closed and the map is omitted when empty.

**The subject depends on the spec's status when the record is written.** `criterion-path-existence` and `body-path-existence`, the sources of most skipped targets, examine `in-progress` and `done` specs and record a `status-gated` skip at any other status ([067's `status-gated-families-are-not-silent`](../067-analyze-and-audit-checks/scenarios/status-gated-families-are-not-silent.md)), and other families still condition on status, so a record written at one status describes a different subject than the same spec at another. Write the record last, after the status is settled.

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
  "guidance": "Resolve the violations and re-run /ductus:review, run /ductus:review --waive <rule-id> --reason \"...\" for each waivable finding, or defer each to pipeline work on another spec (/ductus:review --defer <rule-id> --target <path>), whose gate then holds it.",
  "violations": []
}
```

The deterministic surface behind `/ductus:implement`'s completion gate. Evaluates every check in documented order, first failure wins. `blocked-by` names the failing one:

0. **`already-done`** — the spec is at `status: done`. A blocked variant, never `passed: true`, ordered first so a completed spec is never held against a lint violation introduced after it closed, or against the freshness checks its own completing `set-status` disturbs. Only the exact value `done` short-circuits; an unrecognized status falls through, since `validate-frontmatter` owns reporting it (scenario `a-done-spec-has-no-transition-to-gate`).
1. **`markdown-lint`** — the feature directory's markdown lint (recursive `{root}/{feature}/**/*.md` through the `lint-markdown` machinery). Violations are echoed in `violations`, or a non-zero exit the parser could not attribute.
2. **`scenario-open-questions`** — one or more scenarios carry unresolved open questions; the message names the count and the scenarios, and `guidance` points at scenario-targeted `/{project}:clarify`. The list comes from `read-spec`'s collector ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)). Then **`unchecked-task`** — `tasks.md` holds an unchecked box under any heading, outside a fence or comment (spec 079): the message names the count and the first few.
3. **`undischarged-cross-spec-impact`** — see **Cross-spec impact** below.
4. **`not-reviewed`** (`review.md` absent, or its `last-run` missing or null) and 5. **`must-violations`** (`blocking: true`) — the review record, read from `review.md`. A `review.md` that exists but carries no parseable record is **undeterminable**, reported as such rather than as never-reviewed.
6. Retired by spec 081, which kept the numbers: the gate never blocks on `reviewed-digest`, so `review-stale` is no `blocked-by` value. A review is owed for code alone (check 10), and a changed scenario or data model owes an analysis (check 13).
7. **`record-predates-dispositions`** — `review.md` has no `dispositions:` map; absence is not zero. The message names the record and the command to re-run.
8. **`undispositioned-findings`** — an outstanding gating finding: the review record's `must-violations` plus `should-violations`, which leave out a waived violation and one its deferral releases, above zero; the message names the count. It never reads `dispositions.undispositioned`, which does not split by tier, so an undecided advisory finding or observation does not hold it here. For a record the runtime writes, only a SHOULD neither waived nor released by its deferral reaches it, each other count blocking earlier, and the message names each deferral that holds `done` with its work (spec 076); the sum keeps them as a guard against a hand-edited record. The wire name predates the narrowing (scenario `only-gating-findings-hold-done`).
9. **`fix-round-loop-open`** — `review.md` still carries `fix-rounds-open`, so its run stopped between its two writes (spec 079) — then **`fix-round-not-re-read`** — `review.md`'s `not-re-read` exceeds its `round-waivers` that carry a non-blank reason, counted as the record states them, so a hand-edited record with fewer waivers blocks whatever its other counts say. A record with neither field passes, and the passing `guidance` says it does not record whether its fixes were re-read (scenario `the-fix-round-loop-is-recorded-and-gated`; the message and the fields in [078's data model](../078-review-fix-rounds-are-re-read-within-a-bound/data-model.md)).
10. **`review-owed`** — the record lists `open-findings`, a broken contract left open being what reaches this, or `compute-review-scope` reads `review-owed` (spec 079): the message names what changed or was left open, and a review clears it once its record is committed. A change confined to the project's artifacts, a spec's own scenarios and data model included, owes none (spec 081). A window that cannot be read passes, and the passing `guidance` says so.
11. **`not-analyzed`** (`analysis.md` absent, or its `last-run` missing or null) and 12. **`analyze-findings`** (`blocking: true`) — the analyze record, read from `analysis.md`, with the same undeterminable third state, ordered after every review check because the pipeline is `review → analyze → done`. An advisory analyze finding never blocks (scenario `write-analysis-and-the-second-gate`).
13. **`analyze-stale`** — see **Record freshness** below.
14. **`record-predates-dispositions`** and 15. **`undispositioned-findings`** again, for `analysis.md`: no map, then its `hard-fail` plus `blocking-findings` above zero.

The analyze half's staleness check follows its presence and failure checks: those say a record is missing or failing, it says a passing record is out of date. Every review check comes before every analyze check, since `review.md` is one of an analysis's subjects, so an analysis run ahead of a review the spec owes is stale when that review writes (spec 079). **No check reads `folds-into`**: a pending fold never holds `done` ([051](../051-branch-scoped-spec-numbering/spec.md), scenario `a-pending-fold-never-holds-done`).

`message` is the canonical blocked text with the adopter's `[host] project` command namespace substituted; `guidance` carries the resolve, waive or defer follow-up on `must-violations` and the scenario-targeted-clarify pointer on `scenario-open-questions`. Every verdict is a domain outcome — the host halts on a blocked gate, and the primitive never errors for one. An unreadable scenario contributes no questions and never blocks; `read-spec` and `check-artifacts` report it, so not blocking and not reporting stay separate obligations.

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

The gate's one staleness check, the analyze record's, is `analyze_subjects::freshness_of`, returning `RecordFreshness` (`never-run` / `current` / `stale` / `undeterminable`). The review's digest is the same computation over a narrower subject set, and no gate check compares it (spec 081); `compute-review-scope` does, to put each contract it says changed in the scope of a review that runs.

| record | subject set | digest field |
| --- | --- | --- |
| `review.md` | `scenarios/*.md`, `data-model.md` | `reviewed-digest` |
| `analysis.md` | every `.md` under the feature — `review.md` and `spec.md` included, `spec.md` digested with each acceptance criterion's checkbox normalized to unticked, the criteria `mark-criterion` can flip (scenario `checkbox-state-is-outside-the-analysis-digest`), `analysis.md` itself **excluded**; on a `done` spec the comparison also ignores an in-place edit to `plan.md` or `tasks.md`, which the digest still records, and still counts a deleted one (scenario `a-done-specs-plan-edit-leaves-its-analysis-current`) | `analyzed-digest` |

`review.md` and `spec.md` are the review's output and a body it does not judge, so counting them would stale every review; both are analyze's inputs, so excluding them there would exempt the edits that most often invalidate an analysis. `analysis.md` is excluded from its own set because the record is written after its subjects are read.

**The comparison is of content, not commits**: each record carries a per-path sha256 of what its run read from the working tree. `reviewed-against` and `analyzed-against` are provenance, read only for the mechanical-sweep rename exemption, whose candidates are reported rather than dropped when a tree is unavailable (scenarios `the-committed-tree-horizon`, `review-staleness-on-done-specs`). **A record with no digest is `undeterminable`** — not current, not stale, not a sha-diff fallback — and does not block, so a passing gate still carries `guidance` naming an analyze record it could not judge; a review record with no digest names no changed contract to `compute-review-scope`. `reviewed-digest` is an `Option` because a spec with no scenarios and no data model records an **empty** digest, which is current; `spec.md` is always an analyze subject, so an empty analyze digest cannot occur.

## `diff-cross-spec` — implement's cross-spec impact surface

Args:

```json
{ "feature": "042-widget" }
```

Result:

```json
{
  "window-base": "b39f2727dc6939ad145ede1830205e0d122075d3",
  "current-head": "550c8ddc33e6895da0ce6c81fa6f6e2c42049e9f",
  "cross-spec-paths": ["specs/007-sibling/spec.md"],
  "form": "inline",
  "cross-spec-count": 1
}
```

The filter behind `/ductus:implement`'s per-task summary and completion gate. `window-base` is where the spec's window starts, and `cross-spec-paths` lists the paths under the spec root, outside the feature's own directory, that differ from it and that the spec's own work changed, sorted, with `{specs-root}/inbox.md` excluded; the primitive's compiled description has the rules. The working-tree diff is why the per-task summary, which fires before the task's commit, sees the run's uncommitted sibling edits. A zero `cross-spec-count` is the no-impact domain outcome. The result is bounded as `compute-review-scope`'s is (its **Size**): past `[artifacts] read-size-bytes`, `form` is `"file"`, `cross-spec-paths` is absent, and `lists-file` names a file under `.ductus/results/diff-cross-spec/{feature}/` holding it whole; the file form's own size is 269 bytes of compact JSON plus the feature directory's name and each count digit past the first. Read-only but for that file.
