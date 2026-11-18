# 066 — Audit Records and the Completion Gate Plan

Implements [066 — Audit Records and the Completion Gate](spec.md).

## Overview

The primitives this spec specifies ship under 022; the work moves their contract here — the scenarios, the data-model sections for the seven primitives, and five acceptance criteria — gives two primitives the documented shapes they never had, and records the bounded-result and review-scope changes made since.

## Technical Decisions

### One subject per spec, and the dependency runs one way

066 holds the two audit records and the gate that reads them; `067-analyze-and-audit-checks` holds the checks whose findings feed the analyze record. 066 depends on 022 for the parser, the protocol, the severity tiers and the shared helpers, and 067 depends on both. 066 never links 067, and 022 names neither by link, so `derive-dependencies` sees `067 → 066 → 022` and no cycle.

### Scenarios move with `git mv`

Each scenario is moved, not copied, so its history follows it and the rename is visible in the log. A scenario changing contracts in two specs moves with the one whose contract its Behavior mostly changes and is not split. Its `section:` is remapped to a heading of this spec, and its links into 022 become `../../022-deterministic-runtime/` paths.

### The data model states shapes and field meanings

The sections moved from 022 keep every argument and result shape and every field meaning, and drop the rationale their scenarios already state, pointing at the scenario instead. `compute-review-scope` and `process-waivers` gain their first documented shapes, taken from `runtime/src/schema/primitives.rs`.

### The routing gate keeps offering 022

`derive-routing-candidates` offers the lowest-numbered spec whose plan claims a `runtime/` path (`runtime_home` in `runtime/src/primitives/derive_routing_candidates.rs`), which stays 022, so work naming one of these primitives is offered there first. Which of 022, 066 or 067 owns a primitive is the routing decision, stated in `AGENTS.md`'s routing entry. Listing this spec's runtime files below is what scopes its review.

### Freshness paths go to a file past the read size

`RecordFreshness.paths` is the only list in a freshness answer that grows with the tree: `write-review`'s result otherwise carries only the two departure lists 073 added, bounded in the same call (073 task 13), and `check-review-gate`'s message already shows three paths and a count. Both results use `bounded_lists::place`, the mechanism `scope-results-have-a-bounded-size` settled, with their own list key `analyze-stale-paths` and its `analyze-stale-paths-count` inline in both forms.

- **Past the read size, a `stale` freshness carries no `paths` key.** A computed `stale` always holds a path, so the key is skipped when empty (`skip_serializing_if` on `RecordFreshness::Stale.paths`) and the file form never reads as a short complete list; `state` still says `stale`, and `form`, `lists-file` and the count say where the list is. Rejected: a fixed cap on the list, which hides paths, and a separate field beside an intact `paths`, which would leave the maximum unbounded.
- **`write-review` bounds its whole result** in one `bounded_lists::place` call (`bound_lists`): its file holds `analyze-stale-paths` and, since 073 task 13, `malformed-departures` and `superseded-deferrals`, each key present even when empty. It has written `review.md` by then, as `rewrite-spec-links` has written before it bounds. A result with none of the three lists to move reads no setting, so a malformed `[artifacts] read-size-bytes` halts `write-review` only when a list is to be bounded, and its constitution resolution stays the unmeasured one it shares with `write-analysis` (055's test sets the analysis record aside before its `write-review` call, since a stale one would otherwise make the call read the malformed size it uses as its lever).
- **`check-review-gate` carries the list beside its message** (`analyze-stale-paths`, absent when empty and in the file form) and holds it in the same file as `violations` and `cross-spec-impact`.

### The gate contains each path it builds from a `cross-spec-impact:` slug

`classify_cross_spec_entry` contains the affected spec's `spec.md`, its `scenarios/` directory and each scenario with `validate_contained` before it probes, lists or opens them (`BE-INPUT-004`; the *Paths drawn from committed text* bullet of `specs/022-deterministic-runtime/plan.md` §Path arguments: a lexical screen, then canonical containment), listing no scenario until every one is contained so the verdict never depends on directory order. `parse_feature_dir` already rules out `..` and an absolute name, so the slug needs no screen of its own; it admits a `/` inside a sequential slug, which the component-by-component link check covers. A path that leaves the project is a fifth per-entry state, `outside-project`, with the link in the entry's new optional `detail`: it holds `done` as `undischarged` does, since nothing proved a back-link, and it is a state of its own because the remedy is replacing the link, not adding a signpost. Rejected: reading it as `undischarged`, whose guidance says to add a signpost, and `target-missing`, a typo. `check-artifacts`' cross-spec-impact-drift family inherits the state and says so in its message.

**`BE-COMPAT-001`:** `check-review-gate`'s `cross-spec-impact[].state` gains `outside-project`, a new value of a response enum the schema does not document as open to new values, and the entry gains the optional `detail`, omitted when absent. The value is **breaking**; the field is additive. It ships with the pending minor bump (pre-1.0, the compatibility boundary).

### The two record writes are guarded (`BE-TXN-002`)

`write-review` and `write-analysis` each take a `ReadSnapshot` of their record before any helper reads its lists, and rename the regenerated record over it with `write_if_unchanged` (`runtime/src/primitives/mod.rs`; the lock and its bounds are `specs/022-deterministic-runtime/plan.md` §Compare before persist, the helper 041's task 27). A record written in between is kept and the call fails with `changed-since-read`; `/ductus:review` and `/ductus:analyze` re-run the write from the run's argument files, which reads the record again. The owner closed the class on 2026-11-16 by fixing each owner's primitives rather than waiving them. Rejected: plumbing the snapshot's text into each reader, since a change after the snapshot refuses whichever helper read it.

**`BE-COMPAT-001`:** a write that used to succeed over a record changed since its read now fails with the new operational error `changed-since-read`, **breaking** at the call's contract, shipping with the pending minor bump alongside 041's.

### A stripped line-number citation is a mechanical edit (AC6)

`line_citations` (`runtime/src/primitives/line_citations.rs`) holds the one definition of a line-number citation, which `067-analyze-and-audit-checks`' `line-number-citations` family reads as well. A citation is a `:N`, `:N-M` or `:N:C` suffix after a path token that follows neither `:` nor `@`, so a URL's host and a `user@host` never match, or a `#LN` or `#LN-LM` anchor after any path token. The token must end in an extension and, when it holds no `/`, in one from the module's closed list of source and text extensions, so a host and port such as `db.internal:5432` is not a citation (owner decision, 2026-11-17). A host whose last label is also a listed extension, such as `docs.rs` with a port, still reads as one, since no list can tell the two apart. `find_set_apart` reads a third form, an inline code span holding only such a suffix, which only the check reports: removing one deletes a span, so it is no strip.

- **`mechanical_sweep`'s `pair_run` compares each removed line with its citations stripped against the added line.** A line whose only change strips one or more of its citations, each kept or stripped on its own (`strips_to`, which tries each citation both ways up to `MAX_ONE_BY_ONE` in a line or item and past that counts only a whole strip), adds no rewrite (owner decision, 2026-11-18). Beside another change on the line, a `:N`, `:N-M` or `:N:C` strip, which stays one token, is judged on that change alone; an anchor or an en-dash range splits into tokens and makes the line a rewrite, the conservative direction. Adding or renumbering a citation is still a rewrite, since only the removed side is stripped.
- **The one index is read in three places**, `classify-spec-edit`, the analysis record's freshness at the gate and `compute-review-scope`, so the strip is exempt in all three. `compute-review-scope` reads it only for the contracts the review record's digest says changed: a strip-only commit inside the window still lists its files, as a rename sweep's does (owner decision, 2026-11-18). The index describes committed trees, so freshness keeps a path whose working tree differs from its `HEAD` blob in what the digest covers, line endings and `spec.md`'s criterion ticks aside (`exempt_renames`).
- **`classify-spec-edit` reads a criterion or open question whose only change strips one or more of its citations as unchanged**, through the same `strips_to`, as it already removes cross-service references, so a strip inside a criterion is a correction, and an untick beside one still reopens. Only the removed side is stripped, so a citation added to a criterion still reopens. That primitive is `079-review-and-analysis-work-converges`', so 079 records the change.
- **Stored rule verdicts (082) are not exempt.** They compare raw digests (`design_subjects_match`), so a stripped spec's next analysis judges its rules again.

Rejected: an extension-free definition, which would make removing a port from a host in a requirement a mechanical edit and have the check flag every host and port; and matching only tokens with a `/`, which would leave every bare-filename citation.

**`BE-COMPAT-001`:** the change is to three primitives' verdicts for an input they already accepted: `classify-spec-edit` returns `mechanical` or `correction`, values it already returns, where it returned a reopen, and the freshness at the gate and `compute-review-scope`'s digest route read a strip-only change as current. No field, type, default or enum value is added, removed or changed, so the schema does not change and none of the rule's breaking cases applies; it ships with the pending minor bump.

### The unexamined reasons are a closed set (AC2, task 26)

`write-analysis` refuses an `unexamined-by-reason` reason outside the three classes of the data model's table, on every path, before it reads or writes the spec or its records (owner decision, 2026-11-18). A reason outside them falls in no class, so a reader could not tell a gap from an exclusion. The runtime holds the set once, as `UNEXAMINED_REASONS` in `write_analysis.rs`, and `the_reason_list_is_the_data_models_table` holds it equal to the table, which `runtime.yml` therefore triggers on. The two reasons the writer derives from the `[constitutions]` registry are refused from a caller, since they are never accepted.

**`BE-COMPAT-001`:** a call that recorded an unknown reason now fails, naming the argument `unexamined-by-reason`, **breaking** for such a caller; every reason the exec walker and `/{project}:analyze`'s steps supply is in the set. It ships with the pending minor bump.

### Criteria are judged in review, and the gate judges nothing (task 27)

Scenario `criteria-are-verified-in-review` moves the pipeline's only check of code against acceptance criteria from `/{project}:implement`'s completion gate into `/{project}:review` (owner approval, 2026-11-18).

- **A criteria step beside the five passes, not a sixth pass.** `/{project}:review` runs `verifyCriteria` after its passes, in the same subagent, over every criterion of the spec, reading the code a criterion's claim names wherever it lies; the scope and its window are unchanged. A dimension-restricting flag restricts the passes only. Rejected: a sixth pass, which would falsify the 24 live claims of "five passes" or "five dimensions" outside the review records, 020's AC12 and AC16 among them, for no behavior the step lacks.
- **The verdict writes the tick.** Each verdict drives one `mark-criterion` call with `checked` set to `met`, so review ticks a met criterion and unticks a not-met one; the exec walker's `mark-criterion` binding already reads `llm:verifyCriteria` that way. `/{project}:review`'s Scope Boundaries add `spec.md`'s criterion checkboxes to what it writes.
- **`write-review` records the verdicts.** It takes `criteria`, one verdict per criterion addressed by label or index, reads the criteria from `spec.md` itself, and derives `criteria-judged`, `criteria-met`, `criteria-not-met` (each not-met criterion's label) and `criteria-verdicts` (each met criterion's label and the sha256 of its text, tick and label normalized). A not-met verdict sets `blocking`, so `holds_done_until_reviewed` holds the record and the next window does not move; a verdict addressing a criterion `spec.md` lacks is refused.
- **`review-owed` reads the criteria.** `compute-review-scope` reads `review-owed: true` while `spec.md` holds an unticked criterion, or a ticked one whose digest is not among the record's `criteria-verdicts`, whatever the window holds, and names each in `criteria-owing`. A record without `criteria-verdicts` holds no verdict, so a spec's first reopening after this lands owes a review.
- **The gate checks ticks deterministically.** `check-review-gate` blocks `unticked-criterion` directly after `unchecked-task`, reading the criteria through the parser `read-spec` shares.
- **`/{project}:implement`'s completion gate loses its criteria steps** (`read-spec`, `verifyCriteria` and `mark-criterion`); `verifyCriteria`'s request and response shapes are unchanged and move with the step to review.

**`BE-COMPAT-001`:** breaking: `check-review-gate`'s `blocked-by` gains `unticked-criterion`; `ductus exec implement` no longer sends `verifyCriteria`, and `ductus exec review` does; `review-owed` reads true where it read false. Additive: `write-review`'s `criteria` argument and the record's four fields, and `compute-review-scope`'s `criteria-owing`. It ships with the pending minor bump.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Create | The spec |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Create | The seven primitives' shapes |
| `specs/066-audit-records-and-the-completion-gate/scenarios/*.md` | Move | The scenarios moved from 022 |
| `runtime/src/primitives/compute_review_scope.rs` | Specify | `compute-review-scope` |
| `runtime/src/primitives/process_waivers.rs` | Specify | `process-waivers` |
| `runtime/src/primitives/process_decisions.rs` | Specify | `process-decisions` |
| `runtime/src/primitives/decisions.rs` | Specify | The `decisions:` reader the classifier and both writers share |
| `runtime/src/primitives/write_review.rs` | Specify | `write-review` |
| `runtime/src/primitives/write_analysis.rs` | Specify | `write-analysis` |
| `runtime/src/primitives/check_review_gate.rs` | Specify | `check-review-gate` |
| `runtime/src/schema/primitives.rs` | Specify | The result fields `RecordFreshness.paths`' bound adds |
| `runtime/src/primitives/analyze_subjects.rs` | Specify | The digest and freshness computation both records and the gate share |
| `runtime/src/primitives/mechanical_sweep.rs` | Specify | The rename exemption freshness applies, and the citation-strip exemption (task 25) |
| `runtime/src/primitives/diff_cross_spec.rs` | Specify | `diff-cross-spec` |
| `runtime/src/primitives/bounded_lists.rs` | Specify | The read-size bound `compute-review-scope` and `diff-cross-spec` share (scenario `scope-results-have-a-bounded-size`) |
| `runtime/tests/mechanical_sweep_parity.rs` | Specify | Holds the sweep exemption to built fixtures (scenarios `review-staleness-on-done-specs`, `a-renamed-file-contributes-its-rewrites`) |
| `runtime/src/primitives/check_artifacts.rs` | Specify | The cross-spec-impact-drift message that names an affected spec read through a link out (task 23) |
| `runtime/src/schema/descriptions.rs` | Specify | `WRITE_REVIEW`'s and `WRITE_ANALYSIS`' statement of the guard (task 24), `CLASSIFY_SPEC_EDIT`'s statement of the strip (tasks 25 and 26), and `WRITE_ANALYSIS`' of the closed reason set (task 26) |
| `.github/workflows/runtime.yml` | Modify | Triggers on 066's data model, whose reason table a test holds the runtime's list to (task 26) |
| `framework/commands/review.md` | Modify | Step 11 says what a `changed-since-read` refusal means and to re-run the write (task 24); the criteria step, and the criterion ticks it writes (task 27) |
| `framework/commands/analyze.md` | Specify | Step 21 says the same for `write-analysis` (task 24) |
| `runtime/tests/line_ending_discipline.rs` | Specify | The two writers' line-ending exemptions follow their write calls onto the guard (task 24; 022 task 38 widens the subject to guarded writes) |
| `scripts/mutation/compare-and-set-066.py` | Create | The mutation list proving each record write refuses a record changed since its read (task 24) |
| `scripts/mutation/gate-cross-spec-containment.py` | Create | The mutation list proving the gate contains each path it builds from a cross-spec-impact slug (task 23) |
| `runtime/src/primitives/line_citations.rs` | Create | The one definition of a line-number citation, and its strip (task 25) |
| `runtime/src/primitives/classify_spec_edit.rs` | Modify | Criteria and open questions compared with their citations stripped (task 25) |
| `framework/constitution.md` | Modify | §spec-lifecycle names the strip beside mechanical cases (a) and (b) (task 25); §implement-phase places criterion verification in review, and the review record table gains the criteria fields (task 27) |
| `scripts/mutation/line-citation-strip-066.py` | Create | The mutation list proving a strip is mechanical and nothing else becomes so (task 25) |
| `runtime/src/schema/extensions.rs` | Modify | `verifyCriteria`'s docs name `/{project}:review` as its sender (task 27) |
| `runtime/src/interpreter/mod.rs` | Modify | The `mark-criterion` binding's docs name the review walk (task 27) |
| `AGENTS.md` | Modify | The pre-done verification ordering entry retired, since review now verifies (task 27) |
| `framework/reference/review.md` | Modify | What the criteria step reads and how the subagent runs it (task 27) |
| `framework/commands/implement.md` | Modify | The completion gate without its criteria steps (task 27) |
| `framework/reference/implement.md` | Modify | §Completion gate (after all tasks) without criterion verification (task 27) |
| `runtime/src/interpreter/payload.rs` | Modify | The `verifyCriteria` request built for `/{project}:review` (task 27) |
| `runtime/tests/golden/*.jsonl` | Modify | The implement and review walks re-blessed (task 27) |
| `scripts/mutation/criteria-in-review-066.py` | Create | The mutation list proving each criteria behavior (task 27) |

## Data Model

[data-model.md](data-model.md) defines the argument and result shapes of `compute-review-scope`, `process-waivers`, `process-decisions`, `write-review`'s observations and denominator, `write-analysis`, `check-review-gate` with its cross-spec impact and record-freshness contracts, and `diff-cross-spec`.

## Trade-offs

- **One new spec for the records and the checks together** — rejected: they are two subjects, and the checks have readers the records do not. Size does not decide it: one spec holding both would carry a data model of about 40 KB (the two as built, 19,775 and 20,201 bytes by `wc -c` on 2026-10-15), some 10 KB under the 50,000-byte read size, where two keep each near 20 KB; 022's 44,823 bytes are the same either way.
- **Splitting each mixed scenario across specs** — rejected: it would rewrite each such requirement document to move part of its Behavior, when a pointer serves every reader.
- **Leaving the primitives' sections in 022 behind a pointer** — rejected: the pointer would not shrink the file a reader has to open.
