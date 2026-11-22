---
section: "5–6. /configure canonical allow-set"
---

# Configure-permission-pattern-safety

## Context

Spec 023 §§5–6 defined *what* `/configure` puts in the canonical allow-set — explicit per-path entries for ductus-owned state files (§5) and an unconditional MCP-tool block sourced from `framework/runtime-tools.txt` (§6). The sibling scenario `configure-dedup-permissions` added *how many* copies of each entry may exist. Neither says anything about the **shape** of an individual pattern, and one shape is unsafe.

Seven entries in `framework/bootstrap/configure/claude.md` (its **Git commands targeting another working tree** list, mirrored into the generated `.claude/commands/ductus/configure.md`) place the wildcard *before* the git subcommand:

```text
Bash(git -C * add *)      Bash(git -C * commit *)   Bash(git -C * push *)
Bash(git -C * log *)      Bash(git -C * diff *)     Bash(git -C * status *)
Bash(git -C * show *)
```

Claude Code's permission matcher lets that leading `*` span inserted options, not just the path. `git -C . -c core.pager='!sh -c "…"' status` matches `Bash(git -C * status *)` and is approved with **no prompt**. Both `-c` and `--exec-path` run arbitrary commands, so the canonical allow-set ships an arbitrary-execution hole into every adopter that runs `/ductus:configure`. Claude Code now warns about each of the seven at session start, which is how this surfaced.

`claude.md` is the only configure file that carries a permission set, and so the sole file affected: Pi has no permission file.

The deny-side `git -C *` entries (`claude.md`'s six deny entries, `Bash(git -C * mv *)` through `Bash(git -C * clean -fd *)`) are **not** affected: an over-broad *denial* only refuses more, so the same wildcard is safe there and must stay as-is.

`claude.md` is not the only allow set ductus ships. `/ductus`'s Claude permission seed (the Agent Registry's `settings_template` in `framework/bootstrap/ductus.md`, which `install.sh` writes too) carried `Bash(certutil -hashfile * SHA256)` from 022's task 18 on, and the audit check below read `claude.md` alone. Claude Code warns on it in an adopter, "allows any option at its * · use an exact value": its check, as read in the 2.1.330 binary, flags an allow rule whose `*` precedes a further word that is not an option, since the `*` also matches spaces.

## Behavior

- Every allow set ductus ships, `/configure`'s canonical set and `/ductus`'s Claude seed, holds a shape invariant: **no allow pattern may place a wildcard before the subcommand and option region of the command it approves.** A wildcard is permitted only after the subcommand (`Bash(git status *)`), where it can no longer introduce an option that changes what executes, and so never before a further word of the pattern.
- The seed's `Bash(certutil -hashfile * SHA256)` becomes `Bash(certutil -hashfile .ductus/bin/ductus.exe SHA256)`, the pointer's digest as the bootstrap spells it, and joins Claude's retired seed list, so `/ductus` removes the old entry from an existing settings file. The configured binary's digest, whose path differs per project, goes to the host's prompt (048's scenario `pointer-follows-the-store`).
- The seven `Bash(git -C * <sub> *)` allow entries are removed from `framework/bootstrap/configure/claude.md` §§5–6. `git -C` invocations fall through to the host's normal Ask prompt.
- The removal is stated as a rationale comment alongside the remaining git entries, so a future editor re-adding a `-C` variant reads why it was dropped rather than re-deriving it from a startup warning.
- The deny-side `git -C *` entries in `claude.md` (`Bash(git -C * mv *)` through `Bash(git -C * clean -fd *)`) are explicitly retained and documented as intentionally broad: denial over-matching is strictly safer, and narrowing them would weaken the guard.
- `.claude/commands/ductus/configure.md` follows from the source rewrite via the normal command generator (the pre-commit hook on `framework/bootstrap/configure/claude.md`, spec 017 row 11); no separate edit.
- ~~`/ductus:configure` does not retroactively strip an unsafe pattern already present in an adopter's `settings.local.json` — §§5–6 add entries and `configure-dedup-permissions` removes exact-match duplicates; neither route rewrites a non-canonical entry an adopter owns. Adopters configured before this change keep the seven rules until they remove them, and their host's startup warning is what surfaces it.~~ **Superseded** by [`configure-retires-formerly-canonical-entries`](configure-retires-formerly-canonical-entries.md) (AC36–AC38): the command now retires these seven entries from an adopter's file, via an explicit retired-entries list passed to `merge-permissions` as `revoke`. The reasoning above still holds for entries an adopter **owns** — retirement matches only what the framework itself once shipped — but seven of these approve arbitrary execution with no prompt, and leaving a security hole installed with a startup warning as the sole remedy put the repair on the adopter indefinitely. The bullet is kept rather than deleted: it is the record of what shipped.
- `/ductus:audit` gains a check that fails when any `configure/*.md` canonical allow-set, or the Claude seed's allow entries, contain a wildcard-before-subcommand pattern, so the class cannot silently return through a future edit to either. `install.sh`'s copy of the seed is held to the registry's by Family 14, and `govern.md`'s by Family 21.

## Edge Cases

- A non-git command with the same shape (any `Bash(cmd * sub *)` where the leading `*` can absorb options) is in scope for the invariant — the rule is about wildcard position, not about git specifically. `certutil -hashfile * SHA256` was one.
- An adopter who pinned `configure/claude.md` in `.ductus/config.toml` keeps their pinned copy, unsafe entries included; the pin contract already accepts that trade.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
