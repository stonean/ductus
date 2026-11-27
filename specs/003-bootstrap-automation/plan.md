---
title: "003-bootstrap-automation — plan"
---

# 003 — Bootstrap Automation Plan

## Overview

Install the standard pipeline commands in `.claude/commands/ductus/`, generated from the `framework/commands/` sources with `{project}` replaced by `ductus`. They give `ductus` the same pipeline enforcement as adopting projects.

## Technical Decisions

### Standard commands are generated from the sources

`scripts/gen-claude-commands.sh` globs `framework/commands/*.md`, substitutes `{project}` and `{cli-config-dir}`, writes `.claude/commands/ductus/`, and deletes any file whose source is gone, so the copies track the sources and the set rather than a fixed list. The pre-commit hook runs it (`.githooks/pre-commit`), and `/ductus:audit`'s check-zero precondition, `scripts/audit/check-zero.sh`, fails on a stale copy. This ensures this repo dogfoods the exact same commands adopting projects use.

### The per-language `.gitignore` fetch is bounded

`/ductus` fetches each language's patterns from `raw.githubusercontent.com/github/gitignore` with `curl --proto '=https' --tlsv1.2 --connect-timeout {connect-timeout} --max-time {request-timeout}`, HTTPS only at TLS 1.2 or later and bounded as **Source resolution**'s fetches are (`framework/bootstrap/ductus.md`, §Shared Files' **.gitignore** merge, step 5, and **Derived paths**). Stated here because this spec owns the merge, and the bound came from 022 task 18's and 061's work.

### Session file path

Standard commands reference `.ductus/session.toml` for session state, because it is the one session-state path every adopter shares: project-name-agnostic, gitignored and per-contributor, and written by the runtime's `write-session` (`framework/bootstrap/ductus.md` §Session state). The retired `.govern.session.toml` survives only as the legacy fallback AC2 names.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `.claude/commands/ductus/*.md` | Generate | One file per command source (from `framework/commands/*.md`) |
| `scripts/gen-claude-commands.sh` | Create | Generates the copies from the sources |

## Trade-offs

### Considered: a single `/ductus:work` command instead of the standard commands

Rejected. `ductus` should use the same commands as adopting projects. A custom command would diverge from the dogfooding principle and miss bugs or friction in the templates.

### Considered: skipping the configure command for this repo

Rejected. Even though this repo already has a settings file, the configure command is part of the standard set. Keeping it maintains parity with adopting projects.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Resolved Questions section.
