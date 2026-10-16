# 066 — Audit Records and the Completion Gate Plan

Implements [066 — Audit Records and the Completion Gate](spec.md).

## Overview

No runtime behavior changes. The primitives this spec specifies already ship under 022; the work moves their contract here — sixteen scenarios, the data-model sections for the seven primitives, and five acceptance criteria — and gives two primitives the documented shapes they never had.

## Technical Decisions

### One subject per spec, and the dependency runs one way

066 holds the two audit records and the gate that reads them; `067-analyze-and-audit-checks` holds the checks whose findings feed the analyze record. 066 depends on 022 for the parser, the protocol, the severity tiers and the shared helpers, and 067 depends on both. 066 never links 067, and 022 names neither by link, so `derive-dependencies` sees `067 → 066 → 022` and no cycle.

### Scenarios move with `git mv`

Each scenario is moved, not copied, so its history follows it and the rename is visible in the log. A scenario changing contracts in two specs moves with the one whose contract its Behavior mostly changes and is not split. Its `section:` is remapped to a heading of this spec, and its links into 022 become `../../022-deterministic-runtime/` paths.

### The data model states shapes and field meanings

The sections moved from 022 keep every argument and result shape and every field meaning, and drop the rationale their scenarios already state, pointing at the scenario instead. `compute-review-scope` and `process-waivers` gain their first documented shapes, taken from `runtime/src/schema/primitives.rs`.

### The routing gate keeps offering 022

`derive-routing-candidates` offers the lowest-numbered spec whose plan claims a `runtime/` path (`runtime_home` in `runtime/src/primitives/derive_routing_candidates.rs`), which stays 022, so work naming one of these primitives is offered there first. Which of 022, 066 or 067 owns a primitive is the routing decision, stated in `AGENTS.md`'s routing entry. Listing this spec's runtime files below is what scopes its review.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Create | The spec |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Create | The seven primitives' shapes |
| `specs/066-audit-records-and-the-completion-gate/scenarios/*.md` | Move | The sixteen scenarios moved from 022 |
| `runtime/src/primitives/compute_review_scope.rs` | Specify | `compute-review-scope` |
| `runtime/src/primitives/process_waivers.rs` | Specify | `process-waivers` |
| `runtime/src/primitives/process_decisions.rs` | Specify | `process-decisions` |
| `runtime/src/primitives/decisions.rs` | Specify | The `decisions:` reader the classifier and both writers share |
| `runtime/src/primitives/write_review.rs` | Specify | `write-review` |
| `runtime/src/primitives/write_analysis.rs` | Specify | `write-analysis` |
| `runtime/src/primitives/check_review_gate.rs` | Specify | `check-review-gate` |
| `runtime/src/primitives/analyze_subjects.rs` | Specify | The digest and freshness computation both records and the gate share |
| `runtime/src/primitives/mechanical_sweep.rs` | Specify | The rename exemption freshness applies |
| `runtime/src/primitives/diff_cross_spec.rs` | Specify | `diff-cross-spec` |
| `scripts/audit/review-freshness.sh` | Specify | `/{project}:audit` Family 19, the release-gate half of the staleness rule `mechanical_sweep` implements |
| `runtime/tests/mechanical_sweep_parity.rs` | Specify | Holds Family 19 and `mechanical_sweep` to one rule (scenarios `review-staleness-on-done-specs`, `a-renamed-file-contributes-its-rewrites`) |

## Data Model

[data-model.md](data-model.md) defines the argument and result shapes of `compute-review-scope`, `process-waivers`, `process-decisions`, `write-review`'s observations and denominator, `write-analysis`, `check-review-gate` with its cross-spec impact and record-freshness contracts, and `diff-cross-spec`.

## Trade-offs

- **One new spec for the records and the checks together** — rejected: they are two subjects, and the checks have readers the records do not. Size does not decide it: one spec holding both would carry a data model of about 40 KB (the two as built, 19,775 and 20,201 bytes by `wc -c` on 2026-10-15), some 10 KB under the 50,000-byte read size, where two keep each near 20 KB; 022's 44,823 bytes are the same either way.
- **Splitting each mixed scenario across specs** — rejected: it would rewrite each such requirement document to move part of its Behavior, when a pointer serves every reader.
- **Leaving the primitives' sections in 022 behind a pointer** — rejected: the pointer would not shrink the file a reader has to open.
