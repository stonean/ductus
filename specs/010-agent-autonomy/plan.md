---
title: "010-agent-autonomy — plan"
---

# 010 — Agent Autonomy Plan

## Overview

Land five small, independent `ductus` changes:

1. Stuck detection in `/ductus:implement` (no new artifact — reads `git log` and `tasks.md`)
2. `--auto` flag on `/ductus:implement`
3. "Skills" index section in the `AGENTS.md` project template
4. Cost-levers cross-reference paragraph in the constitution
5. Concurrent-features note (one session target per agent process, per `062-concurrent-session-targets`; point at `git worktree` and platform isolation)

The work is almost entirely prompt and prose — no application code, no new schemas, no events, no error codes, no data model. The risk is in getting the command-file parity right so `framework/commands/*.md` and `.claude/commands/ductus/*.md` stay in sync via `scripts/gen-claude-commands.sh`.

## Technical Decisions

### Stuck-detection algorithm

`/ductus:implement` gains a setup-time check that runs before walking tasks:

1. Read `tasks.md` and identify the next incomplete task (first `- [ ]` checkbox group).
2. Run `git log --oneline -- specs/{feature}/tasks.md` to count commits that touched `tasks.md` since the spec entered `in-progress`.
3. If `git log` shows ≥ 3 commits on `tasks.md` since `in-progress` AND the same task is still the next incomplete one (i.e., no checkboxes have flipped to `[x]` between those commits for that task), surface the cycle to the user with the message: "Task {N} ({title}) has been touched in {count} prior implement runs without completing. Consider decomposing it into smaller subtasks before continuing." Pause and wait for user direction.
4. If `--auto` is set, the stuck-detection event still pauses (per spec: "auto mode does not power through cycles").

Threshold of 3 chosen as the smallest count that distinguishes routine multi-session work (1–2 invocations) from a cycle. The threshold is not configurable in v1 — keep the rule simple; if it proves wrong in practice, revisit.

The detection uses commit count on `tasks.md`, not on the full affected-files list, because:

- `tasks.md` is updated each time a task is touched (checkbox flips, retries, scope changes), so its commit history is the most reliable signal of "this task was the focus."
- Affected-file commit history is noisy (one commit can touch many files for unrelated reasons).
- It avoids the implementation having to parse the plan's affected-files table.

### `--auto` flag

`/ductus:implement --auto` is a per-invocation flag. Argument parsing in the command instructions:

- The `Context` section already supports `$ARGUMENTS` as a feature override. Extend to recognize `--auto` as a known flag, stripped from the value before treating remaining text as the feature override.
- Default is unset (default off).

Behavior with `--auto`:

- Skip the per-task commit confirmation. The agent commits the task's code with its `tasks.md` tick on its own (see below, and step 8 of the implement reference's walk, spec 083) and proceeds.
- All other gates still fire and pause:
  - The completion transition (`in-progress`→`done`). `planned`→`in-progress` is not gated at all: invoking `/ductus:implement` is the approval to start, with or without `--auto` (000's `implement-skips-planned-prompt` scenario)
  - Stuck-detection events
  - Discovering an out-of-bounds file write (current "If you need to modify files outside the plan's affected files list, notify the user…")
  - Spec edits, plan edits, or new tasks discovered mid-implement
  - Risky actions (destructive ops, secrets, force pushes — covered by the agent's safety rules)

Auto-mode commit policy: commit, do not push. Push is a hard-to-reverse, externally visible action; it stays gated even with `--auto`. The flag's job is to remove per-task confirmation friction, not to publish changes silently. The instructions document this explicitly.

The session file is unchanged — autonomy is not session state.

### Cost-levers cross-reference: location

`### Cost levers` is a short subsection of `## Guiding Principles` in `framework/constitution.md`. A named subsection is greppable and gets its own anchor for cross-references.

Content: one paragraph naming `ductus`'s cost levers (stuck detection, default-off autonomy) and saying token budgets are the AI platform's, which is where runtime cost controls live. No commitment to a specific platform. Paragraph stays short so it reads as guidance, not a manual.

### Concurrent-features note: location

Add a new short subsection `### Concurrent Features` under `## Pipeline Boundaries` in the constitution. A subsection there is preferred over a note in the AGENTS.md template because:

- One session target per agent process is a constitutional property of the pipeline, not an adopter-customizable choice — it belongs with the other pipeline rules.
- Pipeline Boundaries is already where serial-by-design rules live (the bullet list).
- AGENTS.md is the adopter's project-level operating doc; constitutional invariants are out of scope for it.

Content: one paragraph stating that each agent process holds its own session target, and that `git worktree` and platform features (Claude Code's `isolation: "worktree"` agent parameter, Cursor's worktree integration, etc.) remain the answer for isolating edits (`062-concurrent-session-targets`, §concurrent-features). 3–5 sentences.

### Skills index in `AGENTS.md` template

Insert a new optional `## Skills` section after `## Project Structure` and before `## Code Style` in `framework/templates/project/agents.md`. The section is empty by default with an HTML-comment guide explaining when and how to populate it:

```markdown
## Skills

<!-- Optional. List skill files (Anthropic/Claude Code "skills" — context-loaded
     instruction packs) that augment AGENTS.md for specific task types. Leave
     empty if you don't decompose into skills.

     Example:
     | Skill | Activates on |
     | --- | --- |
     | `skills/security-review.md` | Code review on auth or session paths |
     | `skills/db-migration.md` | Editing migration files |

     Per-platform mapping (Claude Code skills, Cursor rules, etc.) is the
     adopter's call — `ductus` defines the index pattern, not the location.
-->
```

Empty-by-default ensures backwards-compatibility for projects that don't decompose. The HTML comment teaches the pattern in place without bloating the rendered template.

### Command file parity

One of 010's deliverables touches `framework/commands/*.md`: `implement.md` (stuck detection + `--auto`). Per `CLAUDE.md`, never edit `.claude/commands/ductus/*.md` directly — always edit the source under `framework/commands/` and run `./scripts/gen-claude-commands.sh`. The implementation runs the generator after editing the sources; tasks.md includes the generator step explicitly.

`framework/bootstrap/configure/claude.md` is the source for `.claude/commands/ductus/configure.md`; the generator writes the configure file from the bootstrap source. It is edited at the source location and the generator picks it up.

### What this plan does NOT do

Explicit non-goals to keep the scope tight:

- No new file format, no new artifact type, no schema change. Everything rides on existing markdown conventions.
- No platform-specific shipping (no Claude Code skills directory, no Cursor rules directory). `ductus` documents the pattern only.
- No execution log, no per-task token tracking, no budget files, no complexity routing — all explicitly declined in the spec.
- No multi-target session within one process, no `--feature` flag on commands, no worktree management — declined in the spec. Each agent process holds its own target (062); one process holds one.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/implement.md` | Modify | Add stuck-detection step; accept `--auto` flag with documented gates |
| `framework/templates/project/agents.md` | Modify | Insert optional `## Skills` index section (empty by default) |
| `framework/constitution.md` | Modify | Add `### Cost levers` subsection and `### Concurrent Features` subsection |
| `.claude/commands/ductus/implement.md` | Regenerate | Output of `gen-claude-commands.sh` after editing source |

## Trade-offs

### Stuck-detection threshold of 3, not configurable

A fixed threshold keeps the rule simple and predictable. A configurable threshold would require a place to store the value (somewhere in the session file? `AGENTS.md`?) and a way to surface it in command output. None of those carry their weight for v1. If the threshold proves wrong in practice, change the constant and reissue.

### Stuck detection reads `tasks.md` commits, not affected-files commits

Affected-files commit history is noisier and forces the implementation to parse the plan's affected-files table. `tasks.md` commit history is a clean signal because `tasks.md` is touched on every implement-pass that does work in the feature. The downside: a stuck task that doesn't trigger any `tasks.md` commits (e.g., the agent keeps trying without flipping checkboxes or even noting attempts) goes undetected. Acceptable — every realistic implement loop already touches `tasks.md` for the checkbox transitions.

### Auto-mode commits but does not push

Push is hard-to-reverse and externally visible. Keeping it gated preserves the spirit of `--auto` (skip per-task confirmation) without granting the agent silent publishing rights. Adopters who want full auto-publish can wrap `/ductus:implement --auto` in a script that pushes after each session — they're opting into more risk explicitly.

### Skills index empty by default

An empty section with an HTML-comment guide adds slight visual weight to AGENTS.md compared to omitting the section entirely. Acceptable because the section teaches the pattern in place — adopters who don't decompose see exactly what skills would look like and can add them when ready, without re-reading `ductus` docs.

### Concurrent-features note in constitution, not AGENTS.md template

Pipe-and-pull on this one: AGENTS.md is the adopter's customizable doc, but one session target per agent process is a constitutional invariant of `ductus`. Putting the note in the constitution communicates the invariant correctly; putting it in AGENTS.md would imply it's an adopter choice (it isn't). Adopters can still reference the constitutional section from their own docs.

## Open Questions Resolved

All open questions were resolved during clarification. See `spec.md` "Resolved Questions" section.

The plan-time decisions made above (locations for the cost-conscious paragraph and concurrent-features note; stuck-detection threshold of 3; auto-mode commit-but-don't-push) are not new open questions — they are implementation-detail choices that the spec deliberately left to planning.
