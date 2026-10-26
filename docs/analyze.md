# Analyze

The deep reference for `/analyze` — the audit that reads a feature's artifacts against each other, and the `analysis.md` it leaves behind. The [README](../README.md#commands) and [docs/slash-commands.md](slash-commands.md#analyze--a-report-of-where-a-features-own-artifacts-disagree) cover when to reach for it; this is where the check families, the severity tiers, and the meaning of every field in the record live.

The command's own procedure — the numbered runtime steps — is [`framework/commands/analyze.md`](../framework/commands/analyze.md), and the markdown-only reference for each check is [`framework/reference/analyze.md`](../framework/reference/analyze.md), installed at `.ductus/reference/analyze.md`. The record was introduced by [047 — Analyze findings durability](../specs/047-analyze-findings-durability/spec.md), which put it in `spec.md` frontmatter; [057 — Analyze artifact and record relocation](../specs/057-analyze-artifact-and-record-relocation/spec.md) moved it to `analysis.md`, the artifact this command owns; [058 — Findings route at discovery](../specs/058-findings-route-at-discovery/spec.md) replaced its inbox capture with a disposition for every finding.

## What it does

`/analyze` audits one feature's `spec.md`, `plan.md`, `tasks.md`, `data-model.md` and `scenarios/*.md` against **each other**, against the feature's declared dependencies, and against the project's loaded rule files. `/review` is its counterpart for **code**; `/analyze` never reads source.

Detection is read-only; what it finds is then decided. Three kinds of write are in scope and no others:

1. **Dispositions** — after detection, every live finding is **fixed**, **routed**, or **discarded** with its reason, and each fix or route is written only after the operator confirms it. A fix is a mechanical edit to an artifact the run audited. A route writes the finding where a gate will read it — a task, a scenario, a spec edit, a new spec, or a rule file the project owns — chosen by `/groom`'s decision tree, reopening a `done` spec it lands in. A hard-fail or blocking finding cannot be discarded. When anything was written, detection runs again, so the record describes the state the dispositions left. With nobody to confirm (`ductus exec`), nothing is written and each finding is recorded as undispositioned. Nothing is ever written to the inbox.
2. **Record** — `specs/{feature}/analysis.md`, written whole on **every** run, including a clean one and one whose scope was empty. Its frontmatter is the record; its body is a fixed section skeleton. `spec.md` never carries the record.
3. **Revert** — with `--fix` only, a `done` spec whose review state, scenario questions, or dispositions have drifted is set back to `in-progress`, with a non-silent notice naming the spec and what drifted.

The line is between *detecting* and *deciding*: detection never mutates an artifact it audits, every disposition write is one the operator confirmed, and `--fix` never edits content.

Flags: `--all` scans every feature under the spec root (project-level checks still run once, not once per feature) and groups the disposition prompts by spec, each offering to leave the rest of that spec's findings undispositioned; `--fix` performs the reverts above; a bare feature identifier overrides the session target.

## What it checks

| Family | Tier | Subject |
| --- | --- | --- |
| Frontmatter schema | Hard fail | `status` and `dependencies` present and valid; the block parses |
| Dependency graph | Blocking | Each dependency exists, carries a compatible status, and the subgraph is acyclic |
| Spec integrity | Blocking | Acceptance criteria present and non-placeholder; open questions consistent with status; no implementation code |
| Artifact completeness | Blocking | `plan.md` / `tasks.md` present at `planned` and later |
| Plan and task consistency | Blocking | Plan cites the spec and lists decisions and files; tasks are numbered and carry done-when conditions |
| Plan record | Advisory | Each `plan.md` section outside the design record — the plan template's own `##` sections — on a spec at `planned` or later, reported even when a stored discard decides it, so the discard is matched rather than expired; `/prune` moves such a section's durable pieces home and removes it |
| Artifact size | Advisory | Each of `spec.md`, `plan.md`, `tasks.md`, `data-model.md` and every scenario larger than `.ductus/config.toml` `[artifacts] read-size-bytes` (50,000 bytes by default), which an agent may not read in one call; the finding names the fixes for its kind and, on a `done` spec, which reopen it, and a stored discard holds until the file grows into another read page |
| Rule integrity and citations | Blocking / advisory | Cited rule IDs resolve; deprecated citations and non-firing `## Applicable Rules` entries are advisory |
| Review state drift | Blocking | A `done` spec whose `review.md` record is missing a run or reports `blocking: true` |
| Analyze state drift | Blocking | A `done` spec whose `analysis.md`, as this run writes it, reports `blocking: true` |
| Disposition drift | Blocking | A `done` spec whose `review.md` or `analysis.md` records undispositioned findings |
| Scenario open questions | Blocking at `done`, advisory otherwise | Unresolved `## Open Questions` in any `scenarios/*.md` |
| Scenario consistency | Advisory | Scenario sections present; a still-pending scenario has a task |
| Grounding | Advisory | Descriptive claims about the existing system are cited or hedged (form, never truth) |
| Link-adjacent decision drift | Advisory | Prose asserting an open state that its own sibling link's target contradicts |
| Acceptance-criterion path existence | Advisory | A path named in a `done` spec's criterion that no longer resolves |
| Body path existence | Advisory | A path named in a `done` spec's body, plan, data model or scenarios that existed in this repository and no longer resolves |
| Acceptance-criterion labels | Advisory | Duplicate `AC{n}`, a lowered `next-criterion`, an unlabelled criterion |
| Cross-spec / cross-service references | Advisory | Events, errors and data models align; a provably broken cross-service reference |
| Project-level consistency | Advisory | Generator drift, anchor resolution, command frontmatter, orphaned references, un-folded branch specs |
| Unexamined targets | Informational | Every target a family could not examine — see [Unexamined](#unexamined) |

Advisory families introduced with a **published promotion criterion** (grounding, Applicable-Rules citations, both decision-drift checks) stay advisory until that criterion is met; the criteria live with each check in `framework/reference/analyze.md`. The plan record carries none and stays advisory: its heading set is a detector, not a verdict, so a design section under another heading fires too, and one stored discard with its reason settles it. Artifact size carries none either and stays advisory for the same kind of reason: bytes are a heuristic about a host's read cap the check cannot see.

## Severity tiers

- **Hard fail** — required-field violations and malformed frontmatter. The spec is not valid until these are fixed.
- **Blocking** — structural or content issues that must be fixed before the next pipeline gate fires.
- **Advisory** — issues that should be fixed but do not block advancement.
- **Informational** — observations that are neither errors nor warnings. Notably the unexamined-target set and the cross-service reference unknowns. Informational entries are **not findings**: they take no disposition and never gate.

## The record

Each run writes `specs/{feature}/analysis.md`. The record is that file's own
frontmatter — top-level keys, not a nested block:

```yaml
---
spec: 047-analyze-findings-durability
last-run: 2026-09-06T19:02:22Z
analyzed-against: 15845324188478f97e99f320e6e05d5ae350fad7
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 4
analyzed-digest:
  data-model.md: 9c1b…
  review.md: e0b8…
  scenarios/retry-on-timeout.md: 3f2a…
  spec.md: 71d4…
  tasks.md: b085…
unexamined-by-reason:
  root-absent: 4
blocking: false
dispositions:
  fixed: 1
  routed: 1
  discarded: 2
  undispositioned: 0
decisions:
  - key: "grounding — plan.md states the retry budget without a citation"
    outcome: routed
    target: specs/047-analyze-findings-durability/tasks.md
    decided-at: 2026-09-06T19:02:22Z
    decided-by: dev@example.com
  - key: "link-adjacent-drift — spec.md calls 046 an open question; 046 has none"
    outcome: discarded
    reason: "Historical prose in a Resolved Questions entry"
    decided-at: 2026-09-06T19:02:22Z
    decided-by: dev@example.com
  - key: "criterion-labels — AC7 appears twice"
    outcome: discarded
    reason: "The duplicate is in a quoted example block"
    decided-at: 2026-09-06T19:02:22Z
    decided-by: dev@example.com
---
```

| Field | Type | Meaning |
| --- | --- | --- |
| `spec` | Feature slug | The feature the record belongs to, mirroring `review.md`'s |
| `last-run` | ISO-8601 UTC timestamp, or `null` | When the analysis ran. A **missing `analysis.md`** is the never-analyzed state; a `null` here says the same thing for a record that exists |
| `analyzed-against` | Commit sha, or `null` | The HEAD sha at the time of the run — **provenance**, not the staleness basis |
| `hard-fail` | Integer | Hard-fail findings: malformed frontmatter and missing required fields |
| `blocking-findings` | Integer | Blocking-tier findings |
| `advisory` | Integer | Advisory-tier findings. Recorded, **never** gated on |
| `unexamined` | Integer | Targets the run could not examine — the size of the informational skipped set |
| `unexamined-by-reason` | Map of `reason: count` | The `unexamined` total broken out over a closed reason set. Omitted entirely when empty |
| `analyzed-digest` | Map of `path: sha256` | Per-path digest of every `.md` under the feature **as this run read it**, keyed within the feature directory, `review.md` included and this file excluded whole. The staleness basis. Omitted when empty, which only a pre-digest record can be |
| `analyzed-unreadable` | List of paths | Subjects that exist but could not be read when the digest was taken, recorded rather than digested as empty. Omitted when empty |
| `dispositions` | Map of four integers | What the run did with its findings, in every tier: `fixed`, `routed`, `discarded`, `undispositioned`. **Derived**; always written. Absent means the record predates dispositions |
| `decisions` | List | Stored routed and discarded decisions, keyed `{family} — {message}`, each with its outcome, target or reason, `decided-at`, and `decided-by`. Omitted when empty |
| `blocking` | Boolean | **Derived**, never supplied: `true` when `hard-fail` or `blocking-findings` exceeds zero |

The file is rewritten whole on every run, so the record and the report below it can never disagree. `spec.md` is left alone entirely — it carries neither this record nor the review one, and a residual block there is reported as a violation rather than tolerated under the open-schema rule. A spec whose frontmatter does not parse gets **no** record: that spec is one the analysis would have hard-failed on, and writing a clean record into it would invert the whole mechanism.

### `last-run`

The field the completion gate reads first. Its *absence* is the signal: a feature with no `analysis.md`, or a record with `last-run: null`, has never completed an analysis, and the gate blocks `done` on exactly that. A present-but-unparseable `analysis.md` is a **third** state — undeterminable — and must never be collapsed into never-analyzed. Before the record existed, a spec that had passed both pipeline gates and one that had passed only the review were byte-identical on disk — which is why the record is written on every run, clean ones included. A run that declines to write one is indistinguishable from a run that never happened.

### `analyzed-against` and `analyzed-digest`

`analyzed-against` is the HEAD sha at the time of the run, so the counts are attributable to a known tree. It is **provenance, not the staleness basis**, and is read for exactly one thing: the mechanical-sweep rename exemption, which genuinely needs two trees.

`analyzed-digest` is what freshness compares. Staleness was a commit comparison in the first cut of this check and that design produced false blocks: `/{project}:analyze` reads the **working tree**, while `analyzed-against` records a *commit*, and the two coincide only when the tree is clean — which at the moment analyze runs it usually is not, since `/{project}:review` has just written `review.md` and `mark-task` rewrote `tasks.md` before that. Diffing the sha therefore blocked records whose run had genuinely read the current content, as soon as that content was committed. The digest states what the run actually read, so committing content the analysis already examined does not stale it. The exclusion is of **this file, whole** — `write-analysis` rewrites the record and the body in the same call, after the subjects are read, so a digest covering either half could never match — which means `spec.md` is digested whole. Before the relocation the same exclusion had to perform block surgery on `spec.md`; without any exclusion the rule flagged all 54 of this repo's recorded specs, and with it, 1.

The operational rule is still to **write the record last**, after every edit to the spec is in — which is what the command's own step ordering does (detect → decide → re-check → record → render). A run's own disposition writes therefore never stale its record: they land before it. What changed is the consequence of getting it wrong: a record written before a further edit is now reported as stale by the completion gate rather than standing as a quietly-outdated claim. A record carrying **no** digest — every one written before the field existed — reads `undeterminable`: not current, not stale, and not a sha-diff fallback. It does not block, and it clears on the next run.

The review record in `review.md` works identically over its own narrower subject set (`scenarios/*.md` and `data-model.md`) through `reviewed-digest`. One comparison, two subject sets.

### `hard-fail` and `blocking-findings`

The two counts that gate. Together they derive `blocking`, and either above zero holds the spec out of `done` until the findings are resolved and the analysis re-run. They are separate rather than summed because they answer different questions: `hard-fail` means the spec is not even valid to read, `blocking-findings` means it is valid and wrong.

### `advisory`

Recorded and never gated on, which is the deliberate asymmetry with the review record — there, an outstanding SHOULD does block `done`, because §implement-phase says advisory is not ignorable at the review gate. Analyze's advisory tier is advisory by design: some of its checks were introduced advisory **with their own published promotion criteria**, and the rest — the plan record and un-folded branch specs among them — stay advisory for good, so gating on them here would promote every one of them at once, past the criteria some declare and against the design of the rest. The count still rides the gate's guidance line, so a blocked spec says how much advisory work stands.

### `unexamined`

The honesty field, and the one with no counterpart in the review record. A clean analyze is **two different states** — every target examined and clean, or some target unexaminable and the rest clean — and a record carrying only finding counts collapses that into the reassuring reading, inside the artifact a later gate trusts. That is `QUAL-CLAIM-001` in the worst possible place.

It is written even when zero: a zero that was computed and a field that was never written are not the same claim.

**Read it with `unexamined-by-reason`, never alone.** A bare total says *that* something was unexamined and nothing about what, and the reasons are not equivalent — an exclusion by construction and a file that could not be read both increment it.

### `unexamined-by-reason`

The breakdown over a closed reason set. When supplied it is the **authority**: `unexamined` is derived by summing it, so the total and its breakdown cannot disagree. The map is omitted when empty, so a fully-examined run carries no map rather than a map of zeroes.

Two classes live in the set, and they call for opposite responses. **Excluded by construction** is a target a check decided not to examine; it recurs on every run, the exclusion is correct, and nothing is owed. **Could not be read** is a real gap in what the run could see, and is the class worth acting on. Which class each reason belongs to is set once, in `analyze.md`'s [Unexamined targets](../framework/reference/analyze.md#unexamined-targets-informational); that section and the checks it covers describe what each reason means.

`artifact-unreadable` carries one exception to the whole "an unknown is never escalated into a defect" rule: on a spec at `status: done` it is a **blocking finding** rather than a skipped target, so it appears in `blocking-findings` and not here. The rule it breaks is about *other* files — another spec's frontmatter, an upstream service's state — where the defect belongs to someone else. This one names the spec's own artifact in its own directory that its own analysis could not read. The concrete case is Scenario open questions: an unreadable scenario contributes no questions and, as a skip, no finding, so a scenario carrying unresolved questions that will not parse would sail through the gate built to catch exactly that. Below `done` it stays a skipped target — the questions check is advisory there, and an unreadable artifact mid-work is a state to report rather than a gate to fail.

### `blocking`

Derived by the runtime from `hard-fail` and `blocking-findings`, never accepted from the caller — for the same reason `unexamined` is derived from its breakdown: a value a caller can contradict is one that will eventually be contradicted.

### `dispositions`

What the run did with its findings — `fixed`, `routed`, `discarded`, `undispositioned` — beside the tier counts that say how many there are. A run that found five findings and decided none must not be byte-identical to one that decided all five. It is **derived**: `undispositioned` is the live tier total less the live findings routed or discarded, so a finding the caller did not itemize counts as undispositioned and can never read as handled. `fixed` counts findings gone from the re-check, so it sits outside the tier totals.

The completion gate blocks while `undispositioned` is above zero. That is not a promotion of the advisory tier: the block asks for a **decision**, not a fix, and discarding a false positive with its reason clears it. A record with **no** map predates dispositions, and absence is not zero — the gate blocks an `in-progress` spec on it until the command re-runs, while on a `done` spec it is not drift.

### `decisions`

Detection is stateless, so a finding decided in one run fires again in the next — a routed one until its routed work lands. Each routed or discarded decision is stored here, keyed `{family} — {message}`. A later run matches each finding to the stored decision describing the same issue, by judgment rather than byte equality since many messages are worded by the host, and counts it under the stored outcome without asking again. A stored decision whose finding no longer fires is pruned. A run that could not read a target — a reason in the **Could not be read** class of `analyze.md`'s [Unexamined targets](../framework/reference/analyze.md#unexamined-targets-informational) — or could not resolve a registered shared constitution retains it instead, since its finding may simply not have been looked at. A reason **Excluded by construction** does not count: it recurs on every run, and counting it would keep the decision forever. A reworded finding the host does not match to its stored decision is a new finding. A list that does not parse is reported by `validate-frontmatter`, and the writer refuses to write over it rather than read it as empty.

## The report body

Below the frontmatter, `analysis.md` carries a fixed section skeleton,
rendered whole on every run and never appended to:

```text
# Analysis — {feature}

## Summary
## Hard failures
## Blocking findings
## Advisory findings
## Unexamined targets
## Fixed in this run
```

Each tier section lists its **live** findings, one per line, beside its
disposition:

```text
- {family} — {message} — `{path}` — **routed** to `{target}`
- {family} — {message} — `{path}` — **discarded**: {reason}
- {family} — {message} — `{path}` — **undispositioned**
```

When a tier's count exceeds the findings the writer was handed, the section
adds `{n} finding(s) not itemized — counted as undispositioned.`, so the body
can never understate what the frontmatter counts. `## Fixed in this run` lists
the findings the run fixed, which the re-check no longer produces.

**No section carries a `- [ ]` item**, and that is enforced mechanically
rather than by convention: any checkbox marker a finding's text arrives with
is stripped rather than trusted not to be there. A checkbox is what would turn
this report into a second triage queue; a routed finding's work lives in the
artifact it was routed to, not here.

## Reading a record

Taking the example block above: the run examined the spec at `15845324` on 2026-09-06 and found nothing that gates — `blocking: false`, so the completion gate passes on this spec. Three advisory findings stand, and every one is decided: one was routed to a task on the spec, whose checkbox holds `done` until the task lands, and two were discarded with their reasons, so `undispositioned: 0` and none of them holds `done` through the record. A fourth was fixed in the run and is gone from the counts. Four targets went unexamined, and the breakdown settles what that means: all four are `root-absent`, an exclusion by construction, so nothing is owed and the clean result is as clean as it looks. Had those four been `artifact-unreadable` or `target-missing`, the same `unexamined: 4` would have meant the opposite — a gap in what the run could see, on a spec whose record otherwise reads as verified.

## Where the record is read

- **The completion gate.** `check-review-gate` runs the analyze checks after every review check, because the pipeline is `review → analyze → done` and naming the later gate for an earlier defect sends a contributor to the wrong command. An absent `analysis.md` or a `null` `last-run` blocks with *"spec has not been analyzed"*; `blocking: true` blocks naming both counts, with the advisory and unexamined counts on the guidance line. **There is no grandfather clause here, and there must not be one** — this gate fires at the moment a spec is being completed, so the record is always writable.
- **Freshness, after presence.** A third check asks whether the recorded analysis still describes the current artifacts, by comparing **content rather than commits**: the record carries `analyzed-digest`, a per-path digest of every `.md` under the feature (`review.md` included) as the run read it from disk, with `analysis.md` itself excluded. Without this check, `review → fix → done` passed on an analysis from before the fixes — the presence check asks only whether `last-run` is set. `analyzed-against` is provenance, not the basis: it records where `HEAD` was, while analyze reads the working tree, so comparing it blocked records whose analysis had genuinely read the current content the moment that content was committed. It is still read for the mechanical-sweep rename exemption, which needs two trees. `/{project}:review` renders the same state as a row in its own summary from the same comparison, so the row and the gate cannot disagree. A record with no digest reads undeterminable — not current, not stale — and clears on the next run. The review record carries `reviewed-digest` and works identically over its own narrower subject set (`scenarios/*.md`, `data-model.md`), which is why the gate's old notice about durable contracts it could not examine is gone: the comparison reads the working tree, so that state is answered rather than reported.
- **The disposition checks, last.** After every other review and analyze check, the gate blocks on a record — `review.md` or `analysis.md` — with no `dispositions:` map, which predates dispositions, and then on either record's `undispositioned` above zero. Each names the command to re-run. There is no exemption for the missing map, for the same reason there is no grandfather clause above.
- **The `disposition-drift` check family.** A `done` spec whose `review.md` or `analysis.md` records undispositioned findings has drifted, and `--fix` reverts it to `in-progress`. `analysis.md` is judged from the record the run itself writes, not the one it replaces, so a run that decides every finding clears its own drift. A map-less record on a `done` spec is not drift: it predates the field.
- **The analyze-state drift check.** The counterpart to review-state drift: a `done` spec whose analysis reports `blocking: true` has drifted. Since spec 058 it is judged from the record the run writes, after recording it, never from the record the run replaces, whose finding could never clear. A `done` spec with no `analysis.md` at all predates the record; the run writes one, and the CI template's record check and `/audit` Family 37 bound the population that has none.
- **`/audit` Family 37.** Counts the part of that population that has a `review.md`, against a committed high-water mark. A `done` spec with neither artifact is exempt from the CI check too, but Family 37 counts it separately and does not hold it to the mark, because it needs a review first. The mark keeps the exemption bounded and shrinking rather than a silent permanent hiding place. The set cannot legitimately grow: the completion gate has no grandfather clause, so growth means it was bypassed. The backlog is not backfillable — an analyze record asserts *that a run happened*, which nothing on disk substantiates, and writing one for a run that did not happen is the fabrication the record exists to prevent.
