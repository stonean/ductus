# Analyze

The deep reference for `/analyze` — the audit that reads a feature's artifacts against each other, and the `analyze:` record it leaves behind. The [README](../README.md#commands) and [docs/slash-commands.md](slash-commands.md#analyze--a-report-of-where-a-features-own-artifacts-disagree) cover when to reach for it; this is where the check families, the severity tiers, and the meaning of every field in the record live.

The command's own procedure — the numbered runtime steps and the markdown-only reference for each check — is [`framework/commands/analyze.md`](../framework/commands/analyze.md). The record was introduced by [047 — Analyze findings durability](../specs/047-analyze-findings-durability/spec.md).

## What it does

`/analyze` audits one feature's `spec.md`, `plan.md`, `tasks.md`, `data-model.md` and `scenarios/*.md` against **each other**, against the feature's declared dependencies, and against the project's loaded rule files. `/review` is its counterpart for **code**; `/analyze` never reads source.

It is read-only *on its subject*. Three writes are in scope and no others:

1. **Capture** — every surviving finding is appended to `{specs-root}/inbox.md` before anything is rendered, so an audit's results outlive the session that ran it. Appends are deduplicated on `{category}: {family} — {message}`, so re-running against an unchanged repo appends nothing.
2. **Record** — the `analyze:` frontmatter block on the spec, written on **every** run, including a clean one and one whose scope was empty.
3. **Revert** — with `--fix` only, a `done` spec whose review state or scenario questions have drifted is set back to `in-progress`, with a non-silent notice naming the spec and what drifted.

The line is between the *subject* and the *observation*: analyze never mutates an artifact it audits, and `--fix` is the only path that writes a status.

Flags: `--all` scans every feature under the spec root (project-level checks still run once, not once per feature); `--fix` performs the reverts above; a bare feature identifier overrides the session target.

## What it checks

| Family | Tier | Subject |
| --- | --- | --- |
| Frontmatter schema | Hard fail | `status` and `dependencies` present and valid; the block parses |
| Dependency graph | Blocking | Each dependency exists, carries a compatible status, and the subgraph is acyclic |
| Spec integrity | Blocking | Acceptance criteria present and non-placeholder; open questions consistent with status; no implementation code |
| Artifact completeness | Blocking | `plan.md` / `tasks.md` present at `planned` and later |
| Plan and task consistency | Blocking | Plan cites the spec and lists decisions and files; tasks are numbered and carry done-when conditions |
| Rule integrity and citations | Blocking / advisory | Cited rule IDs resolve; deprecated citations and non-firing `## Applicable Rules` entries are advisory |
| Review state drift | Blocking | A `done` spec whose `review:` block is missing a run or reports `blocking: true` |
| Analyze state drift | Blocking | A `done` spec whose `analyze:` block is missing a run or reports `blocking: true` |
| Scenario open questions | Blocking at `done`, advisory otherwise | Unresolved `## Open Questions` in any `scenarios/*.md` |
| Scenario consistency | Advisory | Scenario sections present; a still-pending scenario has a task |
| Grounding | Advisory | Descriptive claims about the existing system are cited or hedged (form, never truth) |
| Link-adjacent decision drift | Advisory | Prose asserting an open state that its own sibling link's target contradicts |
| Acceptance-criterion path existence | Advisory | A path named in a `done` spec's criterion that no longer resolves |
| Acceptance-criterion labels | Advisory | Duplicate `AC{n}`, a lowered `next-criterion`, an unlabelled criterion |
| Cross-spec / cross-service references | Advisory | Events, errors and data models align; a provably broken cross-service reference |
| Project-level consistency | Advisory | Generator drift, anchor resolution, command frontmatter, orphaned references, un-folded branch specs |
| Unexamined targets | Informational | Every target a family could not examine — see [Unexamined](#unexamined) |

Advisory families introduced with a **published promotion criterion** (grounding, Applicable-Rules citations, both decision-drift checks) stay advisory until that criterion is met; the criteria live with each check in `framework/commands/analyze.md`.

## Severity tiers

- **Hard fail** — required-field violations and malformed frontmatter. The spec is not valid until these are fixed.
- **Blocking** — structural or content issues that must be fixed before the next pipeline gate fires.
- **Advisory** — issues that should be fixed but do not block advancement.
- **Informational** — observations that are neither errors nor warnings. Notably the unexamined-target set and the cross-service reference unknowns. Informational entries are **not findings**: they are not captured to the inbox and never gate.

## The record

Each run writes an `analyze:` block into the spec's frontmatter:

```yaml
analyze:
  last-run: 2026-09-06T19:02:22Z
  analyzed-against: 15845324188478f97e99f320e6e05d5ae350fad7
  hard-fail: 0
  blocking-findings: 0
  advisory: 3
  unexamined: 4
  unexamined-by-reason:
    root-absent: 4
  blocking: false
```

| Field | Type | Meaning |
| --- | --- | --- |
| `last-run` | ISO-8601 UTC timestamp, or `null` | When the analysis ran. `null` (the template's initial value) or an absent block means **never analyzed** |
| `analyzed-against` | Commit sha, or `null` | The HEAD sha the run examined — what the counts below describe |
| `hard-fail` | Integer | Hard-fail findings: malformed frontmatter and missing required fields |
| `blocking-findings` | Integer | Blocking-tier findings |
| `advisory` | Integer | Advisory-tier findings. Recorded, **never** gated on |
| `unexamined` | Integer | Targets the run could not examine — the size of the informational skipped set |
| `unexamined-by-reason` | Map of `reason: count` | The `unexamined` total broken out over a closed reason set. Omitted entirely when empty |
| `blocking` | Boolean | **Derived**, never supplied: `true` when `hard-fail` or `blocking-findings` exceeds zero |

The block is spliced in place, so every sibling frontmatter key — `status`, `dependencies`, `review:` — survives untouched. A spec whose frontmatter does not parse gets **no** record: that spec is one the analysis would have hard-failed on, and writing a clean record into it would invert the whole mechanism.

### `last-run`

The field the completion gate reads first. Its *absence* is the signal: a spec with no `analyze:` block, or with `last-run: null`, has never completed an analysis, and the gate blocks `done` on exactly that. Before the record existed, a spec that had passed both pipeline gates and one that had passed only the review were byte-identical on disk — which is why the record is written on every run, clean ones included. A run that declines to write one is indistinguishable from a run that never happened.

### `analyzed-against`

The HEAD sha at the time of the run, so the counts are attributable to a known tree. **Nothing gates on its freshness** — deliberately, and unlike `review.reviewed-against`, which the review gate does check. Analyze's subject includes `tasks.md`, rewritten on every completed task, so a naive staleness check would fire on nearly every run and be learned-ignored. The consequence is real and worth knowing: a record written before a further edit to the spec stays honest about *what it examined* while no longer describing the current body. The operational rule is to **write the record last**, after every edit to the spec is in — which is what the command's own step ordering does (capture → record → render).

### `hard-fail` and `blocking-findings`

The two counts that gate. Together they derive `blocking`, and either above zero holds the spec out of `done` until the findings are resolved and the analysis re-run. They are separate rather than summed because they answer different questions: `hard-fail` means the spec is not even valid to read, `blocking-findings` means it is valid and wrong.

### `advisory`

Recorded and never gated on, which is the deliberate asymmetry with the `review:` block — there, an outstanding SHOULD does block `done`, because §implement-phase says advisory is not ignorable at the review gate. Analyze's advisory tier is a different contract: its members are checks introduced advisory **with their own published promotion criteria**, and gating on them here would promote every one of them at once, past the criteria each declares. The count still rides the gate's guidance line, so a blocked spec says how much advisory work stands.

### `unexamined`

The honesty field, and the one with no counterpart in `review:`. A clean analyze is **two different states** — every target examined and clean, or some target unexaminable and the rest clean — and a record carrying only finding counts collapses that into the reassuring reading, inside the artifact a later gate trusts. That is `QUAL-CLAIM-001` in the worst possible place.

It is written even when zero: a zero that was computed and a field that was never written are not the same claim.

**Read it with `unexamined-by-reason`, never alone.** A bare total says *that* something was unexamined and nothing about what, and the reasons are not equivalent — an exclusion by construction and a file that could not be read both increment it.

### `unexamined-by-reason`

The breakdown over a closed reason set. When supplied it is the **authority**: `unexamined` is derived by summing it, so the total and its breakdown cannot disagree. The map is omitted when empty, so a fully-examined run carries no map rather than a map of zeroes.

Two classes live in the set, and they call for opposite responses:

| Reason | Class | What it means |
| --- | --- | --- |
| `not-a-live-claim` | Excluded by construction | The acceptance criterion asserts the path is *gone* (`deleted`, `renamed from`, `if it exists`, …), so its absence confirms the criterion rather than contradicting it |
| `ships-to-adopter` | Excluded by construction | A **Shared Files** manifest destination — a path this project ships into an adopter's checkout, where it does resolve |
| `root-absent` | Excluded by construction | The candidate path's own top-level segment does not exist here, so nothing beneath it is provable either way |
| `target-missing` | Could not be read | A sibling link's target does not resolve to a file |
| `target-unparseable` | Could not be read | The target traverses a symlink, or is a scenario file that could not be read at all |
| `no-readable-state` | Could not be read | The target exists but carries no state the tell's class can be evaluated against |
| `artifact-unreadable` | Could not be read | The spec's **own** artifact — typically a scenario — could not be read, so the check never examined its subject |

**Excluded by construction** is correct and nothing is owed. **Could not be read** is a real gap in what the run could see, and is the class worth acting on.

`artifact-unreadable` carries one exception to the whole "an unknown is never escalated into a defect" rule: on a spec at `status: done` it is a **blocking finding** rather than a skipped target, so it appears in `blocking-findings` and not here. The rule it breaks is about *other* files — another spec's frontmatter, an upstream service's state — where the defect belongs to someone else. This one names the spec's own artifact in its own directory that its own analysis could not read. The concrete case is Scenario open questions: an unreadable scenario contributes no questions and, as a skip, no finding, so a scenario carrying unresolved questions that will not parse would sail through the gate built to catch exactly that. Below `done` it stays a skipped target — the questions check is advisory there, and an unreadable artifact mid-work is a state to report rather than a gate to fail.

### `blocking`

Derived by the runtime from `hard-fail` and `blocking-findings`, never accepted from the caller — for the same reason `unexamined` is derived from its breakdown: a value a caller can contradict is one that will eventually be contradicted.

## Reading a record

Taking the example block above: the run examined the spec at `15845324` on 2026-09-06 and found nothing that gates — `blocking: false`, so the completion gate passes on this spec. Three advisory findings stand; they are in `inbox.md` and are real work, just not work that holds `done`. Four targets went unexamined, and the breakdown settles what that means: all four are `root-absent`, an exclusion by construction, so nothing is owed and the clean result is as clean as it looks. Had those four been `artifact-unreadable` or `target-missing`, the same `unexamined: 4` would have meant the opposite — a gap in what the run could see, on a spec whose record otherwise reads as verified.

## Where the record is read

- **The completion gate.** `check-review-gate` runs the `analyze:` checks after every `review:` check, because the pipeline is `review → analyze → done` and naming the later gate for an earlier defect sends a contributor to the wrong command. An absent block or a `null` `last-run` blocks with *"spec has not been analyzed"*; `blocking: true` blocks naming both counts, with the advisory and unexamined counts on the guidance line. **There is no grandfather clause here, and there must not be one** — this gate fires at the moment a spec is being completed, so the record is always writable.
- **Freshness, after presence.** A third check asks whether the recorded analysis still describes the current artifacts, by comparing **content rather than commits**: the record carries `analyzed-digest`, a per-path digest of every `.md` under the feature (`review.md` included) as the run read it from disk, with the spec's own `analyze:` block excised. Without this check, `review → fix → done` passed on an analysis from before the fixes — the presence check asks only whether `last-run` is set. `analyzed-against` is provenance, not the basis: it records where `HEAD` was, while analyze reads the working tree, so comparing it blocked records whose analysis had genuinely read the current content the moment that content was committed. It is still read for the mechanical-sweep rename exemption, which needs two trees. `/{project}:review` renders the same state as a row in its own summary from the same comparison, so the row and the gate cannot disagree. A record with no digest reads undeterminable — not current, not stale — and clears on the next run. The `review:` record now carries `reviewed-digest` and works identically over its own narrower subject set (`scenarios/*.md`, `data-model.md`), which is why the gate's old notice about durable contracts it could not examine is gone: the comparison reads the working tree, so that state is answered rather than reported.
- **The `analyze-state-drift` check family.** The counterpart to review-state drift: a `done` spec with `last-run` unset or `blocking: true` has drifted. Here a grandfather rule *does* apply — a `done` spec with no `analyze:` block at all predates the record.
- **`/audit` Family 37.** Counts exactly that grandfathered population against a committed high-water mark, so the exemption is bounded and shrinking rather than a silent permanent hiding place. The set cannot legitimately grow: the completion gate has no grandfather clause, so growth means it was bypassed. The backlog is not backfillable — an analyze record asserts *that a run happened*, which nothing on disk substantiates, and writing one for a run that did not happen is the fabrication the record exists to prevent.
