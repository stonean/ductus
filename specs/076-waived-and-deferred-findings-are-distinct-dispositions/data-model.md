# 076 — Waived and deferred findings are distinct dispositions Data Model

The shapes 076 changes in the two audit records and the primitives that read and write them. It builds on [058's data model](../058-findings-route-at-discovery/data-model.md), which defines `dispositions:` and `decisions:`, and on 020's waiver record. Types are `Dispositions`, `DispositionOutcome`, `Disposition`, `DecisionOutcome` and `DecisionRef` in `runtime/src/schema/primitives.rs`, and `RawDecision` in `runtime/src/primitives/decisions.rs`.

## `dispositions` — both records

```yaml
dispositions:
  fixed: 1
  deferred: 2
  waived: 1
  undispositioned: 0
```

| Field | Type | Notes |
| --- | --- | --- |
| `fixed` | u32 | Unchanged. |
| `deferred` | u32 | Was `routed`. Live findings deferred this run, plus live findings matching a stored deferral that did not lapse. Read under the alias `routed`. |
| `waived` | u32 | Was `discarded`. Live findings waived this run, plus live findings matching a stored waiver decision. Read under the alias `discarded`. |
| `undispositioned` | u32 | Unchanged. Includes a finding whose deferral lapsed this run and that was given no new disposition. |

- **Aliases are read-only.** Both writers render only the new keys, so a record that carries `routed` or `discarded` parses and is rewritten with the new keys on its writer's next run. A map carrying both a key and its alias (`routed` and `deferred`) does not parse, and the record is undeterminable, as any malformed map is.
- **What is counted** is unchanged: observations only in `review.md`, findings of every tier in `analysis.md`.

## `decisions` entry — both records

```yaml
decisions:
  - key: "grounding — plan.md cites runtime/src/foo.rs:12, which does not exist"
    outcome: deferred
    target: specs/031-some-feature/tasks.md
    task: "7"
    work-item: PROJ-1234
    decided-at: 2026-11-02T14:40:00Z
    decided-by: dev@example.com
  - key: "rule BE-AUTHN-001 at src/api/login.ts"
    outcome: deferred
    work-item: "https://gitlab.example.com/acme/api/-/work_items/42"
    gating: true
    decided-at: 2026-11-02T14:41:00Z
    decided-by: dev@example.com
  - key: "applicable-rules — BE-AUTHN-001 is listed but its trigger does not fire"
    outcome: waived
    reason: "Spec cites the rule for a future endpoint; no trigger surface yet"
    decided-at: 2026-11-02T14:42:00Z
    decided-by: dev@example.com
```

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `key` | string | yes | As in 058, plus a third form: a review MUST or SHOULD violation is keyed `rule {rule-id} at {file}`, one entry per file the rule fires at. |
| `outcome` | `deferred` \| `waived` | yes | `routed` is read as `deferred` and `discarded` as `waived`; writers emit only the new names. |
| `target` | string (repo-relative path) | when `deferred` without `work-item` | The pipeline work: a scenario, a `tasks.md`, a `spec.md`, a feature directory created through `/{project}:specify`, or a rule file the project owns. Must exist when the deferral is recorded. |
| `task` | string (task number) | no | Only with a `target` that is a `tasks.md`; the number must name a task in it when recorded. |
| `work-item` | string | when `deferred` without `target` | External work-item ID. Non-empty, one line, stored byte-identical, never read. A blank one counts as absent. |
| `reason` | string | when `waived`; optional when `deferred` | Free text; an empty string is invalid where required. |
| `gating` | bool | no | **Derived, never authored.** Written `true` by the writer on a deferral of a gating finding (review MUST or SHOULD, analyze hard-fail or blocking) that does not release the gate; omitted otherwise. `check-review-gate` reads it to name what holds `done`. |
| `decided-at`, `decided-by` | as in 058 | yes | Unchanged. |
| (additional fields) | any | no | Open-schema, preserved verbatim. |

**Malformed**, beside 058's cases: a `deferred` entry with neither `target` nor `work-item`; a `task` whose `target` is not a `tasks.md`; a `waived` entry carrying `target`, `task` or `work-item`. Each produces a notice and applies to nothing.

## Release and lapse — derived on every run, never stored

**Release.** A deferral releases its gating finding when its `target` lies under a feature directory other than the reviewed or analyzed spec's. A deferral whose target is on the same spec, whose target is a rule file, or that names only a `work-item` does not release it.

**Completion of the named work**, which decides a lapse:

| Target | Complete when |
| --- | --- |
| `tasks.md` with `task` | Every subtask of that task is checked, or the task or the file is absent |
| `tasks.md` without `task` (a record written before 076) | The owning spec is `done`, or absent |
| Scenario, `spec.md`, or feature directory | The owning spec is `done`, or absent |
| Rule file | Always: the amendment was written when the deferral was recorded |
| `work-item` only | Never: the deferral retires only when its finding stops firing |

A target that exists but does not parse is neither complete nor incomplete: the deferral is matched as stored, and the run emits a notice naming the target.

## `Disposition` — writer input

| Field | Type | Notes |
| --- | --- | --- |
| `outcome` | `fixed` \| `deferred` \| `waived` \| `undispositioned` | No input aliases: command files and the runtime ship in lockstep. |
| `target` | string, optional | As the decision field. |
| `task` | string, optional | As the decision field. |
| `work-item` | string, optional | As the decision field. |
| `reason` | string, optional | Required when `waived`. |

**Refusals**, each `PrimitiveError::InvalidArgument` naming the argument path:

| Condition | Argument named | Reason |
| --- | --- | --- |
| `deferred` with neither `target` nor a non-blank `work-item` | `{at}.target` | a deferral names its work: a target, a work item, or both |
| `work-item` spans more than one line | `{at}.work-item` | a work-item ID is one line |
| `target` does not exist | `{at}.target` | the named path |
| `task` without a `tasks.md` target, or naming no task in it | `{at}.task` | the task number and file |
| `waived` without `reason` | `{at}.reason` | unchanged from 058's discard refusal |

## `ReviewFinding` — the `write-review` `findings` entry

| Field | Type | Notes |
| --- | --- | --- |
| `disposition` | `Disposition`, optional | New. Only `deferred` is accepted; a waiver stays `waivers:` and `--waive`. |
| `decision-key` | string, optional | New. The stored decision the host matched; absent, `rule {rule-id} at {file}`. |

A finding with a releasing deferral renders under a **Deferred findings** section and is excluded from `must-violations` and `should-violations`, as a waived one is. A finding with a non-releasing deferral stays in its MUST or SHOULD bucket, rendered with its deferral. A finding both deferred and matched by an applied waiver is refused.

## `analysis.md` gating counts

`hard-fail` and `blocking-findings` count **outstanding** findings: the re-check's count, which the host still supplies, minus the live findings in that tier whose deferral releases. `blocking` is derived from the two as before. Released findings render under a **Deferred to other specs** section of the report.

## `process-decisions` result

| Field | Type | Notes |
| --- | --- | --- |
| `lapsed` | list of decision refs | New. Stored deferrals whose key fired and whose named work is complete. The host passes them to the writer in `expired-decisions`, which drops them, and asks for a new disposition. |

New notices: `deferral lapsed: {key} — {target}[ task {n}] is complete and the finding still fires`, and `deferral target unreadable: {key} — {target}`.

`DecisionRef` gains optional `task` and `work-item`.

## `/{project}:review` flags

| Flag | Notes |
| --- | --- |
| `--defer <rule-id>` | Defers the rule's violations, one decision per file it fires at. Needs `--target` or `--work-item`. |
| `--target <path>`, `--task <n>` | The pipeline work. `--task` needs a `tasks.md` target. |
| `--work-item <id>` | The external ID. |
| `--reason <text>` | Optional with `--defer`; still required with `--waive`. |

`--defer` and `--waive` naming the same rule in one invocation is an operator error. A deferral that replaces a stored waiver anchor passes that anchor in `expired-waivers`; a waiver that replaces a stored deferral passes it in `expired-decisions`. So a finding holds one disposition through the drop lists the writers already have.
