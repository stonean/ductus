---
section: "5–6. /configure canonical allow-set"
---

# Configure-permission-pattern-safety

## Context

Spec 023 §§5–6 defined *what* `/configure` puts in the canonical allow-set — explicit per-path entries for ductus-owned state files (§5) and an unconditional MCP-tool block sourced from `framework/runtime-tools.txt` (§6). The sibling scenario `configure-dedup-permissions` added *how many* copies of each entry may exist. Neither says anything about the **shape** of an individual pattern, and one shape is unsafe.

Seven entries in `framework/bootstrap/configure/claude.md` (lines 51–57, mirrored into the generated `.claude/commands/ductus/configure.md`) place the wildcard *before* the git subcommand:

```text
Bash(git -C * add *)      Bash(git -C * commit *)   Bash(git -C * push *)
Bash(git -C * log *)      Bash(git -C * diff *)     Bash(git -C * status *)
Bash(git -C * show *)
```

Claude Code's permission matcher lets that leading `*` span inserted options, not just the path. `git -C . -c core.pager='!sh -c "…"' status` matches `Bash(git -C * status *)` and is approved with **no prompt**. Both `-c` and `--exec-path` run arbitrary commands, so the canonical allow-set ships an arbitrary-execution hole into every adopter that runs `/ductus:configure`. Claude Code now warns about each of the seven at session start, which is how this surfaced.

The framework already knows the hazard in one place: `framework/bootstrap/configure/antigravity.md:89` deliberately omits the `-C` variants, reasoning that Antigravity's token-prefix `command()` matching "would over-broaden them." That reasoning applies to Claude's matcher too — it was just never carried across. `auggie.md` (anchored `^git add` regexes) and `opencode.md` (`git add *` keys) carry no `-C` variants either, so `claude.md` is the sole host affected.

The deny-side `git -C *` entries (`claude.md:147-152`) are **not** affected: an over-broad *denial* only refuses more, so the same wildcard is safe there and must stay as-is.

## Behavior

- The canonical allow-set gains a shape invariant: **no allow pattern may place a wildcard before the subcommand and option region of the command it approves.** A wildcard is permitted only after the subcommand (`Bash(git status *)`), where it can no longer introduce an option that changes what executes.
- The seven `Bash(git -C * <sub> *)` allow entries are removed from `framework/bootstrap/configure/claude.md` §§5–6. `git -C` invocations fall through to the host's normal Ask prompt — the resolution `antigravity.md:89` already documents and accepts.
- The removal is stated as a rationale comment alongside the remaining git entries, so a future editor re-adding a `-C` variant reads why it was dropped rather than re-deriving it from a startup warning.
- The deny-side `git -C *` entries at `claude.md:147-152` are explicitly retained and documented as intentionally broad: denial over-matching is strictly safer, and narrowing them would weaken the guard.
- `.claude/commands/ductus/configure.md` follows from the source rewrite via the normal command generator (the pre-commit hook on `framework/bootstrap/configure/claude.md`, spec 017 row 11); no separate edit.
- `/ductus:configure` does not retroactively strip an unsafe pattern already present in an adopter's `settings.local.json` — §§5–6 add entries and `configure-dedup-permissions` removes exact-match duplicates; neither route rewrites a non-canonical entry an adopter owns. Adopters configured before this change keep the seven rules until they remove them, and their host's startup warning is what surfaces it.
- `/ductus:audit` gains a check that fails when any host's canonical allow-set contains a wildcard-before-subcommand pattern, so the class cannot silently return through a future edit to any of the four `configure/*.md` files.

## Edge Cases

- A host whose matcher is anchored by construction (`auggie.md`'s `^git add` regexes) cannot express the unsafe shape; the audit check is per-host and skips a host whose format makes the shape unrepresentable.
- A non-git command with the same shape (any `Bash(cmd * sub *)` where the leading `*` can absorb options) is in scope for the invariant — the rule is about wildcard position, not about git specifically.
- `opencode.md`'s `git config *` entry is a distinct question (a broad subcommand wildcard, not a wildcard *before* the subcommand) and is out of scope for this scenario.
- An adopter who pinned `configure/claude.md` in `.ductus/config.toml` keeps their pinned copy, unsafe entries included; the pin contract already accepts that trade.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
