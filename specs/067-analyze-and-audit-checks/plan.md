# 067 — Analyze and Audit Checks Plan

Implements [067 — Analyze and Audit Checks](spec.md).

## Overview

No runtime behavior changes. The primitives this spec specifies already ship under 022; the work moves their contract here — twenty-five scenarios, the data-model sections for the checks, and four acceptance criteria — and gives `check-corpus-links` the documented shape it never had.

## Technical Decisions

### One subject per spec, and the dependency runs one way

067 holds the deterministic checks; [066](../066-audit-records-and-the-completion-gate/spec.md) holds the records their findings feed and the gate that reads them, and 022 the parser, the protocol, the severity tiers and the shared scanners. 067 depends on both, neither links it, and 022 names it only as a backticked slug, so `derive-dependencies` sees `067 → 066 → 022` and no cycle.

### Scenarios move with `git mv`

Each scenario is moved, not copied, so its history follows it and the rename is visible in the log. A scenario changing contracts in two specs moves with the one whose contract its Behavior mostly changes and is not split. Its `section:` is remapped to a heading of this spec, and its links into 022 or 066 become `../../022-deterministic-runtime/` and `../../066-audit-records-and-the-completion-gate/` paths. `scripts/audit/marker-list-parity.sh` reads `criterion-path-existence-family.md` by path, so it changes in the same commit as the move.

### The data model states shapes and field meanings

The sections moved from 022 keep every argument and result shape and every field meaning, including `scenario-consistency`'s referencing-task rule, which the constitution's canonical-sources map names, and drop the rationale their scenarios already state. `check-corpus-links` gains its first documented shape, taken from `runtime/src/schema/primitives.rs`.

### The routing gate keeps offering 022

`derive-routing-candidates` offers the lowest-numbered spec whose plan claims a `runtime/` path (`runtime_home` in `runtime/src/primitives/derive_routing_candidates.rs`), which stays 022. Which spec owns a primitive is the routing decision, stated in `AGENTS.md`'s routing entry. Listing this spec's runtime files below is what scopes its review.

### `check-promotion-coverage` refuses an empty `sections`, a breaking change

Since review round 1 (`2be18da6`) the primitive refuses an empty `sections` on both surfaces rather than reporting zero counts, and the CLI's repeatable `--section` parses as empty when omitted, so an optional argument became required. `BE-COMPAT-001` classes that as breaking; it ships with the pending 0.60.0 minor bump, the pre-1.0 boundary for a breaking change (071's plan). The one in-repo caller, `scripts/audit/promotion-coverage.sh`, passes four `--section` flags.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `specs/067-analyze-and-audit-checks/spec.md` | Create | The spec |
| `specs/067-analyze-and-audit-checks/data-model.md` | Create | The checks' shapes |
| `specs/067-analyze-and-audit-checks/scenarios/*.md` | Move | The twenty-five scenarios moved from 022 |
| `scripts/audit/marker-list-parity.sh` | Modify | Reads `criterion-path-existence-family.md` at its new path |
| `framework/constitution.md` | Modify | The canonical-sources row for the scenario-to-task referencing rule names this data model |
| `runtime/src/primitives/check_artifacts.rs` | Specify | `check-artifacts` |
| `runtime/src/primitives/validate_frontmatter.rs` | Specify | `validate-frontmatter` |
| `runtime/src/primitives/traverse_deps.rs` | Specify | `traverse-deps` |
| `runtime/src/primitives/resolve_anchor.rs` | Specify | `resolve-anchor` |
| `runtime/src/primitives/check_rule_ids.rs` | Specify | `check-rule-ids` |
| `runtime/src/primitives/derive_dependencies.rs` | Specify | `derive-dependencies` |
| `runtime/src/primitives/derive_references.rs` | Specify | `derive-references` |
| `runtime/src/primitives/check_corpus_links.rs` | Specify | `check-corpus-links` |
| `runtime/src/primitives/check_orphaned_references.rs` | Specify | `check-orphaned-references` |
| `runtime/src/primitives/check_command_flags.rs` | Specify | `check-command-flags` |
| `runtime/src/primitives/check_promotion_coverage.rs` | Specify | `check-promotion-coverage` |

## Data Model

[data-model.md](data-model.md) defines the argument and result shapes of `check-artifacts` and its families, `validate-frontmatter`, `traverse-deps`, `resolve-anchor`, `check-rule-ids`, `derive-dependencies` / `derive-references`, `check-corpus-links`, `check-orphaned-references`, `check-command-flags` and `check-promotion-coverage`.

## Trade-offs

- **One new spec for the checks and the records together**, which is keeping the checks with the records in 066 — rejected: the checks have readers the records do not, the `/{project}:audit` families and the pre-commit hook. Size does not decide it: one spec holding both would carry a data model of about 40 KB (the two as built, 19,775 and 20,201 bytes by `wc -c` on 2026-10-15), some 10 KB under the 50,000-byte read size, where two keep each near 20 KB; 022's 44,823 bytes are the same either way.
- **Splitting each mixed scenario across specs** — rejected: it would rewrite each such requirement document to move part of its Behavior, when a pointer serves every reader.
