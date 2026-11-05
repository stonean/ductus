# 076 — Waived and deferred findings are distinct dispositions Plan

Implements [076 — Waived and deferred findings are distinct dispositions](spec.md).

## Overview

A finding takes one of three dispositions: fixed, deferred or waived. The constitution defines them once. The runtime renames `routed` to `deferred` and `discarded` to `waived` and reads the old names from records already on disk. A deferral carries pipeline work, an external work-item ID, or both. Review violations gain deferral beside fix and waive. A deferral can release a gating finding, as §Deferrals in `framework/reference/review.md` states. A deferral lapses when the work it names is complete while its finding still fires. Every live artifact is swept by meaning, and the four affected specs carry signposts.

## Technical Decisions

### The definitions live once, in the constitution's Finding dispositions

§brownfield-inbox, Finding dispositions (`framework/constitution.md`, the *Three dispositions* bullet) becomes Fixed / Deferred / Waived and carries both definitions, including the question meaning of *deferred* (AC1, AC9). The other constitution lines point at it rather than restate it:

- §design-principles (`framework/constitution.md:49`) lists a waiver under *decide it is out of scope and record the decision with its reason*, no longer under *record it where the pipeline will surface it again* (AC2).
- §spec-requirements (`framework/constitution.md:136`) cites the definition for a question deferred with its trigger (AC9).
- §implement-phase (`framework/constitution.md:213`) says a SHOULD is addressed only when fixed, waived with its rationale, or deferred to another spec.
- The *only a gating one holds `done`* bullet (`framework/constitution.md:298`) says an analyze hard-fail or blocking finding is fixed or deferred, and review MUST and SHOULD violations are fixed, deferred or waived.

### The runtime renames, reading the old names only from disk

- `DispositionOutcome` and `DecisionOutcome` (`runtime/src/schema/primitives.rs:159`, `:196`) become `Deferred` and `Waived`.
- `Dispositions` (`runtime/src/schema/primitives.rs:141`) renames its fields to `deferred` and `waived`, with `#[serde(alias = "routed")]` and `#[serde(alias = "discarded")]`. Every field stays required, so a defaulted count still cannot read as decided.
- `RawDecision::to_ref` (`runtime/src/primitives/decisions.rs:68`) accepts `routed` and `discarded` as the new outcomes.
- `covering_key` (`runtime/src/primitives/check_artifact_size.rs:189`) accepts `waived` and `discarded`.
- `render_dispositions` (`runtime/src/primitives/decisions.rs:203`) and `disposition_suffix` (`:216`) emit only the new names (AC14).
- Host inputs take no alias: command files and the runtime ship in lockstep, so an input alias would be code no caller reaches.

### A deferral's companion is a target, a work item, or both

`require_companion` (`runtime/src/primitives/decisions.rs:105`) generalizes to the refusals in the data model's *Disposition* table (AC3, AC10, AC11). `decision_for` (`:176`) stores `task` and `work-item`. It keeps `work-item` byte-identical, unlike the companion text it flattens today, so a multi-line ID is refused rather than flattened (AC4). Existence of a `target`, and of a `task` within it, is checked against the tree at write time; the task check uses `read-tasks`' parser.

### A review violation's deferral is a stored decision, not a new list

A deferred MUST or SHOULD violation is stored in `review.md`'s existing `decisions:`, keyed `rule {rule-id} at {file}`, the form waiver notices already use (`framework/reference/review.md`, §Per-run waiver processing). This reuses one lifecycle (`process-decisions`, `decisions::merge`, `one_outcome_per_key`) for observations, analyze findings and violations. Waivers stay in `waivers:` under `process-waivers`, unchanged.

`ReviewFinding` gains `disposition` and `decision-key`. `/{project}:review` gains `--defer` with `--target`/`--task` and/or `--work-item` (AC12, AC15). The flag shapes are in the data model.

### Release is computed by the writers, so the gate reads counts

A deferral releases a gating finding as `framework/reference/review.md` §Deferrals states (data model, *Release and lapse*).

- `write-review` excludes a released violation from `must-violations`/`should-violations`, as it excludes a waived one (`runtime/src/primitives/write_review.rs:165`), and renders it under Deferred findings.
- `write-analysis` records `hard-fail`/`blocking-findings` net of released findings and derives `blocking` from them (`runtime/src/primitives/write_analysis.rs:127`). The host-supplied gross counts still bound the itemized findings (`:329`).
- A non-releasing deferral of a gating finding stays counted, and its decision carries the derived `gating: true`.
- `check-review-gate`'s outstanding-findings block (`runtime/src/primitives/check_review_gate.rs:318`) names each such decision with its work reference (AC16). Its guidance (`:354`) names deferral as the fix-later exit.

### Lapse is a `process-decisions` classification

`process-decisions` gains a `lapsed` result. It covers a fired key whose stored deferral names work that is complete: a task read through `read-tasks`' parser, or a spec status read from its frontmatter (AC17). The host passes lapsed refs to the writer's existing `expired-decisions` drop list, and the finding takes a new disposition. Under `ductus exec`, with nobody to ask, the finding is recorded undispositioned, and a gating one blocks.

An unreadable target is matched as stored, with a notice, so a check that could not run never looks like a lapse or a hold.

`prune-tasks` has no renumbering step (`runtime/src/primitives/prune_tasks.rs`: no `renumber` hit), and `append-task` numbers `max + 1` (`AppendTaskResult.task_number`, `runtime/src/schema/primitives.rs:2753`). So a task number names one task for its life.

### One disposition per finding goes through the existing drop lists

For observations and analyze findings, `decisions::merge` already replaces every stored entry for a key decided again (058 data model, *Duplicate key*).

For a violation, a `--defer` that replaces a stored waiver anchor passes it in `expired-waivers`, and a `--waive` that replaces a stored deferral passes it in `expired-decisions`. `write-review` refuses a finding that is both deferred and matched by an applied waiver (AC13).

### The waiver example loses its work reference

`framework/reference/review.md:726` drops `ticket` from its example open-schema waiver fields (AC6). §Waivers gains a pointer to deferral as the fix-later path.

### The sweep is by meaning, over the declared live-artifact set

The scope is the set `AGENTS.md` declares between its `audit:sweep-targets` markers, classified by who emits each use (AC7, AC18).

- The inbox's `discard` route keeps its name.
- A migration procedure under `framework/migrations/` keeps the words it was written with.
- The decision tree in `framework/reference/groom.md` reads *waived* and *deferred* where it disposes of a finding; `framework/commands/groom.md` speaks only of inbox items and is unchanged.
- `framework/commands/help.md` is regenerated by `scripts/gen-help-tables.sh` from each command's frontmatter, never hand-edited.

### No migration entry

The read aliases make every existing record parse, and each record is rewritten whole on its writer's next run. An adopter's own CI that reads `dispositions.routed` is the one consumer the aliases cannot reach. `runtime/CHANGELOG.md` names the rename for it.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | Definitions; §design-principles, §spec-requirements, §implement-phase, Finding dispositions |
| `framework/commands/review.md` | Modify | `--defer` flags; three exits; step 10–11 disposition wording |
| `framework/commands/analyze.md` | Modify | Deferral with a work item; renamed outcomes |
| `framework/commands/implement.md` | Modify | Disposition task outcomes |
| `framework/reference/review.md` | Modify | §Waivers, deferral of violations, Deferred findings section |
| `framework/reference/analyze.md` | Modify | Decisions persist, lapse, release |
| `framework/reference/implement.md` | Modify | Disposition task outcomes |
| `framework/reference/groom.md` | Modify | Decision tree wording for findings |
| `framework/commands/help.md` | Regenerate | Analyze description, from its frontmatter via `scripts/gen-help-tables.sh` |
| `runtime/src/schema/primitives.rs` | Modify | Outcome enums, `Dispositions`, `Disposition`, `DecisionRef`, `ReviewFinding`, `process-decisions` result |
| `runtime/src/primitives/decisions.rs` | Modify | Aliases, companions, `gating`, rendering |
| `runtime/src/primitives/process_decisions.rs` | Modify | `lapsed`, target reads, notices |
| `runtime/src/primitives/write_review.rs` | Modify | Violation deferrals, release, Deferred findings |
| `runtime/src/primitives/write_analysis.rs` | Modify | Net gating counts, Deferred to other specs |
| `runtime/src/primitives/check_review_gate.rs` | Modify | Naming holding deferrals; guidance |
| `runtime/src/primitives/check_artifact_size.rs` | Modify | `waived` covers a size |
| `runtime/src/primitives/check_artifacts.rs` | Modify | The review-drift message names the three exits |
| `runtime/tests/golden/implement-basic.jsonl` | Regenerate | `BLESS=1` after `framework/commands/implement.md` changes |
| `runtime/src/mcp/server.rs` | Modify | Tool descriptions |
| `runtime/tests/mcp.rs` | Modify | Wire-shape tests |
| `runtime/CHANGELOG.md` | Modify | The rename and deferral |
| `docs/analyze.md` | Modify | Disposition terms |
| `docs/slash-commands.md` | Modify | Disposition terms, `--defer` |
| `README.md` | Modify | Disposition terms |
| `AGENTS.md` | Modify | Entries that name a finding *discarded* or *routed* |
| `specs/020-code-review/spec.md` | Modify | Signpost |
| `specs/020-code-review/data-model.md` | Modify | Renamed keys; pointer to 076's data model |
| `specs/058-findings-route-at-discovery/spec.md` | Modify | Signpost |
| `specs/058-findings-route-at-discovery/data-model.md` | Modify | Renamed keys; pointer to 076's data model |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Modify | Signpost |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Modify | Renamed keys; gate block 11 |
| `specs/073-decided-rule-departures-are-recorded/spec.md` | Modify | Signpost |

## Data Model

[data-model.md](data-model.md) defines:

- the renamed `dispositions:` keys and their read aliases;
- the `decisions:` entry's new `task`, `work-item` and derived `gating` fields, and the key form for a review violation;
- the release and completion rules;
- the writer refusals;
- `ReviewFinding`'s disposition;
- `analysis.md`'s net gating counts;
- `process-decisions`' `lapsed` result;
- `/{project}:review`'s `--defer` flags.

## Trade-offs

- **A `deferrals:` list in `review.md`, mirroring `waivers:`.** Rejected: it would be a second implementation of matched, expired and retained, and a third list per record. `decisions:` already carries that lifecycle.
- **The gate computing release from `decisions:`.** Rejected: a stored decision does not know its finding's tier, and the writer does. So the writer derives the counts and the `gating` flag.
- **Analysis keeping gross counts beside a released count the gate subtracts.** Rejected: outstanding would be computed in two places, and review's counts are already net of waivers. Net counts give both records one meaning: what still holds `done`.
- **Aliases on host input.** Rejected: there is no caller older than the runtime it ships with.
- **A migration rewriting adopters' records.** Rejected: every record parses through the aliases, and its next run rewrites it.
- **The rename is breaking, within major version zero** (`BE-COMPAT-001`). A reader of a record's `dispositions:` keys sees `deferred` and `waived` where it saw `routed` and `discarded`, and a host sending either old outcome is refused. The runtime is 0.x, and SemVer 2.0.0 lets anything change in 0.y.z (item 4) and requires a major bump only for X > 0 (item 8), so it ships in a minor release. Records written before keep parsing through the read aliases, and the CHANGELOG entry names the rename for adopters' own CI.
- **Limitation:** a deferral naming a task identifies it by number, so a hand edit that renumbers `tasks.md` misdirects it. The runtime never renumbers.
- **Limitation:** the pipeline never reads an external tracker, so a deferral naming only a work item records intent it cannot verify. That is why it never releases a gating finding.

## Cross-spec impact

Each spec listed in the spec's `cross-spec-impact:` gets a blockquote signpost linking back to 076 and naming the contract it owes (AC8).

| Spec | Status | What it owes |
| --- | --- | --- |
| `020-code-review` | `in-progress` | Its review record's data model renames the keys and points at 076 for violation deferrals. |
| `058-findings-route-at-discovery` | `done` → reopened | Its Three dispositions and data model rename `routed` and `discarded` and point at 076. It is reopened `done → in-progress` by `set-status` and needs its review, analysis and `done` transition again. |
| `066-audit-records-and-the-completion-gate` | `in-progress` | Its data model's `process-decisions` and gate block 11 entries describe `lapsed` and the named holding deferrals. |
| `073-decided-rule-departures-are-recorded` | `draft` | Its analyze waiver takes *waived* as defined here; its open question on discarding a blocking finding reads *waiving*. |
