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

See [spec 026](https://github.com/stonean/ductus/blob/main/specs/026-framework-self-audit/spec.md) for the design and the [026 plan](https://github.com/stonean/ductus/blob/main/specs/026-framework-self-audit/plan.md) for the check families and the check-zero precondition pass. The family set has grown well past the original design — `scripts/audit/run-all.sh` runs exactly the families enumerated in the markdown-only reference at `.ductus/reference/audit.md`, a correspondence Family 28 holds in both directions. No count is stated here on purpose: a number in prose is a third copy of a fact two lists already carry, and it is the copy nothing updates (this sentence said twenty-two while thirty-three ran). Family numbers are stable identifiers: Family 3 (registry equivalence) was retired with the workflows feature (spec 043), leaving a numbering gap.

## Scope Boundaries

- Read-only against the framework's cross-cutting artifacts. Do NOT modify any file.
- No session target required; the command operates on the framework as a whole.
- Reference: §drift-prevention, §principles. The constitution is loaded by other pipeline commands; `/audit` re-reads it independently because it runs without `/ductus:target`.

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime, except step 1's `run-generator`, which is not one and runs through the CLI; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

**Where a named section lives.** This command's markdown-only reference — the procedure for the path without the runtime, and the policy these steps apply — is `.ductus/reference/audit.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there; read it when a step points at it.

1. Invoke `run-generator` against `scripts/audit/run-all.sh` — the orchestrator that runs the check-zero precondition pass followed by the family check scripts. `run-generator` is not an MCP tool, since it runs any project file with `bash` (spec 022 task 14), so on the runtime path invoke it through the CLI, `.ductus/bin/ductus run-generator --script scripts/audit/run-all.sh`; `ductus exec` dispatches it in-process. The script emits findings to stdout under per-family headers and exits 0 (no findings) or 1 (any family produced findings).

Otherwise, walk the markdown-only path in `.ductus/reference/audit.md`.

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
