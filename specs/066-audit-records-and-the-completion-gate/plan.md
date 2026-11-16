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
| `runtime/src/primitives/mechanical_sweep.rs` | Specify | The rename exemption freshness applies |
| `runtime/src/primitives/diff_cross_spec.rs` | Specify | `diff-cross-spec` |
| `runtime/src/primitives/bounded_lists.rs` | Specify | The read-size bound `compute-review-scope` and `diff-cross-spec` share (scenario `scope-results-have-a-bounded-size`) |
| `runtime/tests/mechanical_sweep_parity.rs` | Specify | Holds the sweep exemption to built fixtures (scenarios `review-staleness-on-done-specs`, `a-renamed-file-contributes-its-rewrites`) |
| `runtime/src/primitives/check_artifacts.rs` | Specify | The cross-spec-impact-drift message that names an affected spec read through a link out (task 23) |
| `runtime/src/schema/descriptions.rs` | Specify | `WRITE_REVIEW`'s and `WRITE_ANALYSIS`' statement of the guard (task 24) |
| `framework/commands/review.md` | Specify | Step 11 says what a `changed-since-read` refusal means and to re-run the write (task 24) |
| `framework/commands/analyze.md` | Specify | Step 21 says the same for `write-analysis` (task 24) |
| `runtime/tests/line_ending_discipline.rs` | Specify | The two writers' line-ending exemptions follow their write calls onto the guard (task 24; 022 task 38 widens the subject to guarded writes) |
| `scripts/mutation/compare-and-set-066.py` | Create | The mutation list proving each record write refuses a record changed since its read (task 24) |
| `scripts/mutation/gate-cross-spec-containment.py` | Create | The mutation list proving the gate contains each path it builds from a cross-spec-impact slug (task 23) |

## Data Model

[data-model.md](data-model.md) defines the argument and result shapes of `compute-review-scope`, `process-waivers`, `process-decisions`, `write-review`'s observations and denominator, `write-analysis`, `check-review-gate` with its cross-spec impact and record-freshness contracts, and `diff-cross-spec`.

## Trade-offs

- **One new spec for the records and the checks together** — rejected: they are two subjects, and the checks have readers the records do not. Size does not decide it: one spec holding both would carry a data model of about 40 KB (the two as built, 19,775 and 20,201 bytes by `wc -c` on 2026-10-15), some 10 KB under the 50,000-byte read size, where two keep each near 20 KB; 022's 44,823 bytes are the same either way.
- **Splitting each mixed scenario across specs** — rejected: it would rewrite each such requirement document to move part of its Behavior, when a pointer serves every reader.
- **Leaving the primitives' sections in 022 behind a pointer** — rejected: the pointer would not shrink the file a reader has to open.
