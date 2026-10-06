---
description: Record a raw item to the inbox.
argument-hint: "[item text]"
---

# Log

Record a raw item to the inbox.

## Purpose

Append a todo you capture by hand to `specs/inbox.md` for later grooming — this command is the inbox's only producer. Use it when a bug, idea, or open issue occurs to you and you want to capture it without breaking flow. A finding a pipeline run produces does not come here: `/ductus:review`, `/ductus:analyze`, and `/ductus:implement` disposition their own findings in the run that found them (§brownfield-inbox, Finding dispositions). The item stays raw until `/ductus:groom` walks it through the bug decision tree and routes it to one of five destinations: a rule, a new spec, a spec edit, a scenario, or — for project maintenance belonging to no feature — a chore, done in the grooming pass and removed. An item that is not actionable is discarded.

## Context

This command does not require a session target — items in the inbox span the whole project. If `$ARGUMENTS` is provided, use it as the item text. If empty, ask the user what to log.

## Scope Boundaries

- This command only appends a single line to `specs/inbox.md`. Do NOT modify any other file. Do NOT read or write source code, test files, specs, plans, or scenarios.
- Do NOT walk the decision tree, classify the item, or suggest a spec — that is `/ductus:groom`'s job. Keep the recording step fast and uninterpreted.
- Reference: §brownfield-inbox, §spec-phase (spec-root resolution) (constitution loaded by `/ductus:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write) per the Markdown-only reference below.

<!-- audit:ignore-promotion -->
1. Capture the item. If `$ARGUMENTS` is provided, treat it as the item text. Otherwise, ask the user: "What do you want to log?" Optionally ask follow-up questions if the item is so terse it would be unrecoverable later (e.g., "broken" with no context) — one short clarification is enough; do not interrogate.

2. Invoke `append-inbox` with the item text to append `- [ ] {item text}` as a new checkbox bullet to `specs/inbox.md` (the checkbox form the inbox template and constitution §bug-handling document — inbox items clear by being done and removed). The create-if-missing semantics live in the primitive: when the file does not exist, it is created before the append (from the project inbox template when one is on disk, else with a minimal `# Inbox` heading). The item is a single line — recording stays fast and uninterpreted. The result's `item-count` field carries the new inbox total (comment/fence-aware) for the report.

3. Invoke `lint-markdown` against the inbox file step 2 actually wrote — the inbox under the configured `[paths] specs-root`, **not** a hardcoded `specs/inbox.md`. append-inbox resolves the root itself, so a literal here lints a path it never touched and reports clean on a file that does not exist.

<!-- audit:ignore-promotion -->
4. Report: the line that was added; the new total item count in the inbox (`append-inbox`'s `item-count` result); and the suggested next step: "Run `/ductus:groom` when you're ready to walk the inbox and route items to their proper homes." **Stop here.** Do not start grooming or implementation. The user invokes `/ductus:groom` explicitly.

## Markdown-only reference

With no ductus runtime registered, the host performs the same append with its own file tools (Read, Edit, Write) — no shell-pipeline substitution (§runtime-host-integration):

1. If `specs/inbox.md` does not exist, create it with a minimal heading (`# Inbox`) followed by a blank line.
2. Append the item to the inbox list as a new checkbox bullet:

   ```markdown
   - [ ] {item text}
   ```

3. Run `npx markdownlint-cli2` on the modified file.

Either path ends with the same report: the line that was added, the new total item count in the inbox, and the `/ductus:groom` pointer — and stops there.
