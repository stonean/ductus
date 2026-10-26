---
description: Display the pipeline view for all feature specs.
parity:
  strict-stdout: true
---

# Status

Display the pipeline view for all feature specs.

## Purpose

Read-only overview of every feature's progress through the pipeline. Shows which specs are ready to advance, which are blocked, and what the current session target is.

## Scope Boundaries

- This is a read-only command. Do NOT modify any files.
- The pipeline view is acquired through the single `dashboard` primitive — one runtime call returns the structured payload plus the pre-rendered pipeline view. On the runtime path the procedure does not read individual spec files, list directories, or shell out for that view; the markdown-only fallback (`.ductus/reference/status.md`) necessarily reads the artifacts on disk, but still without shell-pipeline substitution.
- Cross-service reference resolution is read-only and, on the runtime path, folded into that same call: the runtime resolves each spec's `references:` index internally (the same classification the resolve-references primitive exposes) to render the readout. On the markdown-only path the host reads `.ductus/config.toml` and the linked specs with host file tools (see **§Resolving cross-service references** in `.ductus/reference/status.md`). Both paths read only `.ductus/config.toml` and the registered local checkouts — the canonical repo URL is never fetched.
- Reference: §text-first-artifacts (the schema is the authoritative source for which fields appear in the payload); [030 — Cross-Service References](https://github.com/stonean/ductus/blob/main/specs/030-cross-service-references/spec.md) for the reference semantics surfaced here.

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).
>
> **For this command specifically**: the single deterministic tool for the pipeline view is `dashboard`. One MCP call returns the session target, the per-spec inventory, the repo-wide tags-union, the `.ductus/config.toml` review-state summary, and the pre-rendered pipeline view. Do not substitute shell utilities (no `ls`, no `for` loops over spec directories, no `cat .ductus/config.toml`) for that call. Cross-service reference resolution for specs that carry a `references:` index is folded into the same call on the runtime path (the runtime resolves each index internally for the readout); on the markdown-only path, use the host file tools documented in **§Resolving cross-service references** in `.ductus/reference/status.md`. Neither substitutes a shell pipeline.

**Where a named section lives.** This command's markdown-only reference — the procedure for the path without the runtime, and the policy these steps apply — is `.ductus/reference/status.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there; read it when a step points at it.

<!-- audit:ignore-promotion -->
1. Invoke `dashboard` to load the full pipeline state in one call. Alongside the structured payload (session-target — this process's own when it has a session identity, with session-identity, session-source, session-notices and session-unreadable beside it — specs, tags-union, config, inbox-standing), the result carries a `rendered-markdown` field: the preamble, dashboard table, counts and callouts, and cross-service references readout pre-rendered as one markdown fragment (the runtime resolves each spec's `references:` index internally for the readout, so no separate per-spec call is needed on this path).

<!-- audit:ignore-promotion -->
1. Emit the `rendered-markdown` fragment as the pipeline view. It is returned data the host may restyle — do not recompute the underlying facts from the structured payload when the fragment is present. On the markdown-only path, derive the same facts with host file tools and render the four pieces by hand per **§Rendering reference** in `.ductus/reference/status.md`.

<!-- audit:ignore-promotion -->
1. List any non-done specs (excluding the current target, if any) and prompt the user to run `/ductus:target` to select one.

## Status → next action

| Status | Next Action |
| --- | --- |
| draft | /ductus:clarify |
| clarified | /ductus:plan |
| planned | /ductus:implement |
| in-progress | /ductus:implement |
| done | done (spec is complete) |

When a scenario is targeted and the scenario itself has one or more open questions, the next action is `/ductus:clarify` (scenario-targeted, resolves scenario-level open questions regardless of parent spec status).

When a spec's `scenario-open-question-count` is at least one, its Next Action is `clarify (scenario)` at any status, including `done` — a spec is not complete while its scenarios carry questions (§spec-lifecycle). `clarify (recovery)` takes precedence when both apply: spec-body questions at `clarified` or later are the more upstream defect. The cell is exclusive; the callouts are not (see **§Rendering reference** in `.ductus/reference/status.md`).

When a spec declares `folds-into`, its Status cell is the frontmatter status **qualified** with the pending fold — `in-progress (fold pending)`, `done (fold pending)` — at every status. A declared fold target is owed work, so the view never reports such a spec as simply `done`: the fold is owed whatever the status, until fold-back retires the spec (§numbering, §spec-lifecycle). It never holds the spec short of `done`, though, and the Next Action says so: **done first, then fold.** Below `done` the Next Action is the ordinary one from the table above, because a staging spec completes its own pipeline like any other; at `done` it is `/ductus:fold → {target}`, the upstream consolidation that remains. The status is kept beside the qualification rather than replaced by it, because `in-progress` and `done` are different situations for the operator: the first still has its own pipeline to finish, the second has only the fold left. Either `clarify` override above takes precedence over the fold — an open question is the more upstream defect, and content gets settled before it gets moved.

A fold target that does not resolve in this tree is called out in the outstanding-fold callout, and on the row once the fold is its Next Action (`… (target not in this tree)`). That is a **report, not a verdict**: before the merge the target normally lives on the branch this one forked from, so an unresolvable target is the expected state and only a typo is worth correcting. Existence is enforced at fold-back, which is the first tree in which both specs exist.
