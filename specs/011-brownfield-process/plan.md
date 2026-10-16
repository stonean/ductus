---
title: "011-brownfield-process — plan"
---

# 011 — Brownfield Process Plan

## Overview

Three workstreams: (1) capture sparse brownfield input as a skeleton spec, (2) rename `triage` to `inbox` across all `ductus` artifacts, (3) document the brownfield process, scenario promotion, and cross-spec impact patterns in the constitution and README.

## Technical Decisions

### Capture is `/specify` with sparse input

Capturing an existing feature uses `/{project}:specify` (`framework/commands/specify.md`) with a sparse description: it creates a numbered spec directory, drafts a skeleton spec from the freeform input, and writes the session target, without pressure for comprehensive criteria. There is no separate capture command.

### Inbox rename strategy

The rename is a direct find-and-replace across all files that reference `triage`. The word "triage" is replaced with "inbox" in file names, headings, section markers, command references, and prose. Files are renamed (not copied) to preserve git history.

The rename touches 16 files. To keep the diff reviewable, the rename is done as a single task before other content changes.

### Ductus file manifest updates

`framework/bootstrap/ductus.md` references `triage` in three places: the file manifest (template source → destination), the slash command manifest, and the post-scaffolding output. All three update to `inbox`.

### Constitution additions

Three new sections added to `constitution.md`:

- **Brownfield process** — under the existing brownfield triage section (renamed to brownfield inbox). Documents the capture → incremental growth → promotion lifecycle.
- **Scenario promotion** — under the existing scenarios section. Documents when and how to promote a scenario to its own spec.
- **Cross-spec impact** — new pipeline boundary. Documents that changes land where they belong with a signpost back to the originating spec.

### Cross-spec signposts

A brief note is added to 006-bug-workflow's spec.md indicating that `triage` was renamed to `inbox` by 011. The note does not change 006's status — it is informational only, not a behavioral change.

A brief note is added to 007-govern-workflow's spec.md indicating that the ductus command gains a triage → inbox migration step and `/capture` in the command manifest by 011. This is also informational — 007's status does not change.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/groom.md` | Modify | Update heading, references, and content from triage to inbox. |
| `.claude/commands/ductus/groom.md` | Modify | Update heading, references, and content from triage to inbox. |
| `framework/templates/project/inbox.md` | Modify | Update heading and content from triage to inbox. |
| `framework/bootstrap/ductus.md` | Modify | Update file manifest, command manifest, post-scaffolding output. |
| `framework/constitution.md` | Modify | Rename triage section to inbox; add brownfield process, scenario promotion, cross-spec impact. |
| `README.md` | Modify | Rename triage to inbox; update brownfield section |
| `framework/commands/help.md` | Modify | Update triage references to inbox. |
| `.claude/commands/ductus/help.md` | Modify | Update triage references to inbox. |
| `AGENTS.md` | Modify | Update triage references to inbox |
| `specs/006-bug-workflow/spec.md` | Modify | Add signpost noting triage → inbox rename by 011 |

Not modified (historical specs — self-contained at time of writing):

- `specs/006-bug-workflow/plan.md`
- `specs/006-bug-workflow/tasks.md`
- `specs/007-govern-workflow/spec.md`
- `specs/007-govern-workflow/plan.md`

## Open Questions Resolved

All open questions were resolved during the clarify phase. No new questions surfaced during planning.
