---
title: "003-bootstrap-automation — plan"
---

# 003 — Bootstrap Automation Plan

## Overview

Install the standard pipeline commands in `.claude/commands/ductus/`, copied from the `framework/commands/` templates with `{project}` replaced by `ductus`. They give `ductus` the same pipeline enforcement as adopting projects.

## Technical Decisions

### Standard commands are literal copies with placeholder replacement

Each command template in `framework/commands/` is copied to `.claude/commands/ductus/` with every occurrence of `{project}` replaced by `ductus`. No other modifications. This ensures this repo dogfoods the exact same commands adopting projects use. If a command template is updated later, the copy is re-derived from the template — by hand as designed here, and by `scripts/gen-claude-commands.sh` since the Trade-offs entry below was reversed. The copy-and-substitute shape survived that reversal; what changed is that a script does it over a directory glob rather than a contributor over a fixed list.

### Session file path

Standard commands reference `.ductus/session.toml` for session state.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `.claude/commands/ductus/help.md` | Create | Pipeline overview (from template) |
| `.claude/commands/ductus/target.md` | Create | Set session target (from template) |
| `.claude/commands/ductus/status.md` | Create | Spec dashboard (from template) |
| `.claude/commands/ductus/configure.md` | Create | Configure permissions (from template) |
| `.claude/commands/ductus/specify.md` | Create | Create new spec (from template) |
| `.claude/commands/ductus/clarify.md` | Create | Resolve questions (from template) |
| `.claude/commands/ductus/plan.md` | Create | Create plan and tasks (from template) |
| `.claude/commands/ductus/implement.md` | Create | Execute tasks (from template) |
| `.claude/commands/ductus/analyze.md` | Create | Audit artifacts (from template) |

## Trade-offs

### Considered: generating standard commands dynamically from templates

Rejected here, and **later adopted** — recorded per [§drift-prevention](../../framework/constitution.md#drift-prevention)'s *Decision resolution* rule, which fires when a previously-rejected option is taken up. The rejection rested on two premises that did not hold: that copying with replacement stays simple, and that this repo's project name is fixed. The command set grew from ten to sixteen, and 049 renamed the project to `ductus`, so hand-copied files drifted from the templates they came from. `scripts/gen-claude-commands.sh` now globs `framework/commands/*.md`, substitutes `{project}` and `{cli-config-dir}`, writes `.claude/commands/ductus/`, and deletes any file whose source is gone; the pre-commit hook runs it, and `/ductus:audit`'s check-zero precondition fails on a stale copy.

### Considered: a single `/ductus:work` command instead of ten standard commands

Rejected. `ductus` should use the same commands as adopting projects. A custom command would diverge from the dogfooding principle and miss bugs or friction in the templates.

### Considered: skipping the configure command for this repo

Rejected. Even though this repo already has a settings file, the configure command is part of the standard set. Keeping it maintains parity with adopting projects.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Resolved Questions section.
