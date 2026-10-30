---
section: "Generators and Hooks"
---

# A-commit-stages-only-its-own-generated-files

## Context

`.githooks/pre-commit` runs `scripts/gen-configure-mcp.sh`, `scripts/gen-claude-commands.sh` and `scripts/gen-help-tables.sh` over the working tree, then `git add`s every output path whatever the commit staged: `framework/bootstrap/configure/claude.md` and `auggie.md`, `.claude/commands/ductus/`, `.pi/prompts/`, `.pi/extensions/`, `.ductus/reference/` and `framework/commands/help.md`. This repository runs several agent sessions in one tree on `main` (`AGENTS.md` §Workflow), so a commit regenerates from another session's unstaged sources and stages the result. On 2026-10-29 one session's commit carried another's uncommitted command copies and was rebuilt without them as `554d0ee3`, and `2741fb12` and `ed079593` were committed with `--no-verify` so as not to carry a third session's uncommitted allow entries. [tracked-specs-not-worktree](tracked-specs-not-worktree.md) closed this class of defect for spec files only. The shipped adopter hook, `framework/bootstrap/hooks/ductus-pre-commit`, stages only specs already in the commit, so only this repository's hook has it. Found by 048's 2026-10-29 review.

## Behavior

- What the hook adds to a commit from the generators is derived from that commit's staged sources: a source edit that is not staged never reaches the commit through a generated file.
- A working-tree file the commit does not stage, a source or a generated output, is neither staged nor overwritten by the hook.
- A commit whose staged sources change a generated output carries that output regenerated, so the committed tree's outputs agree with its sources, which `.github/workflows/generators.yml` still checks after a push.
- §ductus repo's pre-commit hook states the hook's staging rule as it runs.

## Edge Cases

- A commit stages a hand edit to `framework/bootstrap/configure/claude.md` while another session's `framework/runtime-tools.txt` entry is unstaged: the committed allow block and `.claude/commands/ductus/configure.md` reflect the staged `runtime-tools.txt`, not the working tree's.
- A generated output staged by hand that disagrees with the staged sources: the hook stages the regenerated output, so the commit's outputs still agree with its sources.
- A commit staging no generator source: the hook stages no generated file.
- A markdown file staged in part, from a patch applied to the index while another session's hunks stay in the working tree: the hook's markdown normalization fixes and re-stages only the staged text, so the unstaged hunks never reach the commit. Today it runs `markdownlint-cli2 --fix` on the working copy and `git add`s the whole file, which on 2026-10-29 put another session's uncommitted `framework/bootstrap/ductus.md` and `govern.md` hunks into `9565f306`, rebuilt without them as `c6a0dca2`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
