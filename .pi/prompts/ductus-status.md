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
- The pipeline view is acquired through the single `render-status` primitive — one runtime call returns the pre-rendered view alone, without `dashboard`'s structured payload, which nothing here reads (spec 080). The procedure does not read individual spec files, list directories, or shell out for that view.
- Cross-service reference resolution is read-only and folded into that same call: the runtime resolves each spec's `references:` index internally (the same classification the resolve-references primitive exposes) to render the readout, reading only `.ductus/config.toml` and the registered local checkouts — the canonical repo URL is never fetched.
- Reference: §text-first-artifacts (the schema is the authoritative source for which fields appear in the payload); [030 — Cross-Service References](https://github.com/stonean/ductus/blob/main/specs/030-cross-service-references/spec.md) for the reference semantics surfaced here.

## Instructions

> **For agent runtimes**: the Invoke steps below call the ductus runtime's MCP tools; without the server, stop and say to restart or re-run `/ductus` (§runtime-host-integration).
>
> **For this command specifically**: the single deterministic tool for the pipeline view is `render-status`. One MCP call returns the pre-rendered view — the session target, the table of every spec, the summary, the cross-service references readout, the specs with work left grouped by next action, and the `/ductus:target` prompt. Do not substitute shell utilities (no `ls`, no `for` loops over spec directories, no `cat .ductus/config.toml`) for that call. Cross-service reference resolution for specs that carry a `references:` index is folded into the same call (the runtime resolves each index internally for the readout).

**Where a named section lives.** This command's reference is `.ductus/reference/status.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there. When a step points at one, invoke `read-section` with that path and every section the step names, in one call, rather than reading the whole file.

<!-- audit:ignore-promotion -->
1. Invoke `render-status` to load the pipeline view in one call. Its `rendered-markdown` holds the whole view as one markdown fragment, rendered by the runtime from the pipeline state — the target block, the table, the summary one item per line, the cross-service references readout, the specs with work left grouped by next action, and the closing `/ductus:target` prompt — held to the configured read size. Its `project` names the slash-command namespace the view's hints use.

<!-- audit:ignore-promotion -->
1. Emit the `rendered-markdown` fragment as the pipeline view. It is returned data the host may restyle — do not recompute or re-list the facts it carries, the specs with work left and the prompt to select one included, since the fragment already holds them.

## Status → next action

| Status | Next Action |
| --- | --- |
| draft | /ductus:clarify |
| clarified | /ductus:plan |
| planned | /ductus:implement |
| in-progress | /ductus:implement |
| done | done (spec is complete) |

When a scenario is targeted and the scenario itself has one or more open questions, the next action is `/ductus:clarify` (scenario-targeted, resolves scenario-level open questions regardless of parent spec status).

When a spec's `scenario-open-question-count` is at least one, its Next Action is `clarify (scenario)` at any status, including `done` — a spec is not complete while its scenarios carry questions (§spec-lifecycle). `clarify (recovery)` takes precedence when both apply: spec-body questions at `clarified` or later are the more upstream defect. The cell is exclusive; the callouts are not, each rendering on its own.

When a spec declares `folds-into`, its Status cell is the frontmatter status **qualified** with the pending fold — `in-progress (fold pending)`, `done (fold pending)` — at every status. A declared fold target is owed work, so the view never reports such a spec as simply `done`: the fold is owed whatever the status, until fold-back retires the spec (§numbering, §spec-lifecycle). It never holds the spec short of `done`, though, and the Next Action says so: **done first, then fold.** Below `done` the Next Action is the ordinary one from the table above, because a staging spec completes its own pipeline like any other; at `done` it is `/ductus:fold → {target}`, the upstream consolidation that remains. The status is kept beside the qualification rather than replaced by it, because `in-progress` and `done` are different situations for the operator: the first still has its own pipeline to finish, the second has only the fold left. Either `clarify` override above takes precedence over the fold — an open question is the more upstream defect, and content gets settled before it gets moved.

A fold target that does not resolve in this tree is called out in the outstanding-fold callout, and on the row once the fold is its Next Action (`… (target not in this tree)`). That is a **report, not a verdict**: before the merge the target normally lives on the branch this one forked from, so an unresolvable target is the expected state and only a typo is worth correcting. Existence is enforced at fold-back, which is the first tree in which both specs exist.
