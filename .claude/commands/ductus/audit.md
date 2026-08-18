---
description: Audit framework artifacts for cross-doc, cross-manifest, cross-registry drift. Maintainer-only.
---

<!-- audit:ignore-placeholders:file -->
<!-- This command is maintainer-only and not scaffolded into adopter projects,
     so its /ductus: references are literal, not templating drift. -->

# Audit

Audit `ductus`'s own framework artifacts for the kinds of drift `/ductus:analyze` is not scoped to catch. Maintainer-only — adopters never invoke this command. Runs without a session target.

## Purpose

`/ductus:analyze` audits a single feature spec's artifacts against each other (frontmatter, plan, tasks, data-model, dependencies, rule citations). Its contract is bounded to one feature directory plus declared dependencies, so it cannot see drift across the framework: pipeline diagrams in the constitution vs. the introduction, `configure/claude.md` vs. `configure/auggie.md` canonical permission set, migration registry vs. procedure files, etc.

`/audit` fills that gap. It loads no rule files — its checks are about *framework consistency*, not spec quality. Each check family produces structured findings on stdout. Exit code is binary: `0` when no findings, `1` when any finding is present. CI uses the exit code as a release gate.

See [spec 026](../../specs/026-framework-self-audit/spec.md) for the design and the [026 plan](../../specs/026-framework-self-audit/plan.md) for the check families and the check-zero precondition pass. The family set has grown since the original design — `scripts/audit/run-all.sh` runs the twenty-one families enumerated in the markdown-only reference below. Family numbers are stable identifiers: Family 3 (registry equivalence) was retired with the workflows feature (spec 043), leaving a numbering gap.

## Scope Boundaries

- Read-only against the framework's cross-cutting artifacts. Do NOT modify any file.
- No session target required; the command operates on the framework as a whole.
- Reference: §drift-prevention, §principles. The constitution is loaded by other pipeline commands; `/audit` re-reads it independently because it runs without `/ductus:target`.

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

1. Invoke `run-generator` against `scripts/audit/run-all.sh` — the orchestrator that runs the check-zero precondition pass followed by the family check scripts. The script emits findings to stdout under per-family headers and exits 0 (no findings) or 1 (any family produced findings).

Otherwise, walk the markdown-only path below.

## Markdown-only reference

When the runtime is not on `PATH`, walk the same scripts directly. Each prints findings to stdout and exits `0` (no findings) or `1` (findings present). Aggregate across all families; `/audit`'s exit code is the logical OR.

1. Run `scripts/audit/check-zero.sh` — generator/lint precondition. Halt on findings; do not run family checks against known-stale generator output.
2. Run `scripts/audit/cross-doc-consistency.sh` (Family 1).
3. Run `scripts/audit/manifest-parity.sh` (Family 2).
4. Run `scripts/audit/placeholder-roundtrip.sh` (Family 4).
5. Run `scripts/audit/template-alignment.sh` (Family 5).
6. Run `scripts/audit/ssot-invariants.sh` (Family 6).
7. Run `scripts/audit/sibling-coupling.sh` (Family 7).
8. Run `scripts/audit/introducing-drift.sh` (Family 8).
9. Run `scripts/audit/primitive-promotion-candidates.sh` (Family 9).
10. Run `scripts/audit/migration-coverage.sh` (Family 10).
11. Run `scripts/audit/consolidation-pair.sh` (Family 11).
12. Run `scripts/audit/fixture-session-shape.sh` (Family 12).
13. Run `scripts/audit/runtime-hardcoded-paths.sh` (Family 13).
14. Run `scripts/audit/installer-registry-parity.sh` (Family 14 — `install.sh` agent list and dest paths match the `ductus.md` **Agent Registry**, and each agent's pre-seeded settings file matches its registry `settings_template`).
15. Run `scripts/audit/runtime-probe-parity.sh` (Family 15 — the ductus binary probe is in parity between each agent's **Agent Registry** `settings_template` seed and its `configure/{key}.md` set: present in both or neither, never one only).
16. Run `scripts/audit/installer-command-parity.sh` (Family 16 — the `/ductus` **Per-Agent Scaffolding** slash-command manifest lists exactly the `framework/commands/*.md` files, minus the maintainer-only commands (`audit`) intentionally not shipped to adopters).

17. Run `scripts/audit/host-namespace-parity.sh` (Family 17 — the namespace the runtime renders (`[host] project`, else the repo directory basename, as `Host::load` resolves it) matches a namespace actually installed under an agent config dir, so no rendered next-action names a namespace the operator cannot invoke).

18. Run `scripts/audit/marker-list-parity.sh` (Family 18 — the `criterion-path-existence` non-assertion marker list agrees across its canonical source, the runtime array, and the adopter-facing restatement in `analyze.md`, including the spelled-out counts; a derivation that yields no markers is a finding rather than a silent pass).

19. Run `scripts/audit/review-freshness.sh` (Family 19 — no `done` spec ships with a review that predates its own code: a spec whose durable contracts changed since its `review.reviewed-against` sha is stale, which every other check passes because they assert only that a review exists and does not block).

20. Run `scripts/audit/version-agreement.sh` (Family 20 — the repo-root `version` pin, `runtime/Cargo.toml`, and the newest `runtime/CHANGELOG.md` heading carry the same SemVer. The release tag is deliberately not compared: the release commit precedes the tag push, so asserting it here would fail every release mid-flight).

21. Run `scripts/audit/transitional-bootstrap-parity.sh` (Family 21 — the retired `framework/bootstrap/govern.md` path stays byte-identical to `framework/bootstrap/ductus.md`. Every pre-rename adopter's self-update fetch resolves to the retired path, so drift there ships stale content to them verbatim and a deletion 404s their run before migrations).

22. Run `scripts/audit/adopter-shell-behavior.sh` (Family 22 — the shipped adopter shell works in an adopter's tree, not just in ours. Stands up a fixture with a non-default `[paths] specs-root`, config only at the converged tier, and the runtime reachable only through `.ductus/bin/ductus`, then runs the real `framework/bootstrap/hooks/ductus-pre-commit` in it. This repo runs *different copies* of that job — its own `.githooks/pre-commit`, the default spec root, a locally built runtime — so every assumption those mask is invisible to a green run here; three silent defects reached adopters through that gap on 2026-08-17. The runtime is stubbed, keeping the family hermetic and identical in CI).

## Boundary with `/ductus:analyze`

| Concern | Owner |
| --- | --- |
| Spec's frontmatter parses; required fields present | `/ductus:analyze` |
| Dependency graph well-formed for one feature | `/ductus:analyze` |
| Rule IDs cited in spec exist in loaded rule files | `/ductus:analyze` |
| Plan / tasks / data-model present per status tier | `/ductus:analyze` |
| Cross-doc claim consistency (pipeline diagrams, back-edge wording, etc.) | `/audit` |
| Manifest / permission / registry parity | `/audit` |
| Sibling-spec coupling (bundling candidates) | `/audit` |
| Introducing-spec body drift (current-tense prose around renamed names) | `/audit` |

Rule of thumb: `/ductus:analyze` reads within one spec's directory plus its declared dependencies; `/audit` reads across the framework's cross-cutting artifacts. The two never duplicate a check.

## Output

`/audit` writes findings to stdout in a maintainer-friendly format: family header, then one finding per row with location / message / suggested-fix columns. Exit code `0` when no findings; `1` when any finding is present. No `audit.md` artifact is produced — the audit runs interactively, not stored as a per-run report.
