---
section: "The primitive library"
---

# Project-directory-resolution-chain

## Context

Required by [049 — Rename govern to ductus](../../049-rename-govern-to-ductus/spec.md), which moves the per-project directory and therefore changes how the runtime resolves the two files that live in it.

Before 049, `runtime/src/schema/paths.rs` resolved the project config and the session file across **two** locations — the `.govern/` directory that [042](../../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) introduced, and the pre-042 repo-root files (`.govern.toml`, `.govern.session.toml`) kept as a fallback so an adopter who upgraded the binary before re-running the bootstrap was never broken. `active_path(repo, new, legacy)` implemented that pair for writes; `config_path` / `session_path` implemented it for reads.

049 renames the directory to `.ductus/`. The 042 guarantee has to hold one level up: an adopter sitting on `.govern/` when they upgrade the binary is in exactly the position a pre-042 adopter was in, and must keep working until the `ductus-rename` migration moves them. So the fallback **grows a tier** rather than swapping one out.

This is the only behavior change in 049 — the rest of that spec is a uniform token sweep — which is why it lands here rather than there, per `AGENTS.md`'s runtime-routing rule.

## Behavior

Both files resolve through a single ordered chain, newest first:

| Tier | Config | Session |
| --- | --- | --- |
| 1 (current) | `.ductus/config.toml` | `.ductus/session.toml` |
| 2 (042-era) | `.govern/config.toml` | `.govern/session.toml` |
| 3 (pre-042) | `.govern.toml` | `.govern.session.toml` |

- **Read** (`config_path`, `config_display_name`, `resolve_config`, `session_path`) returns the newest tier that exists. When none exists it returns the **oldest** — a missing file the caller already treats as "config absent" → defaults.
- **Write** (`config_path_for_write`, `session_path_for_write`) returns the newest tier that exists, and the **newest** tier when none exists. A fresh project therefore cuts over immediately, while a pre-migration write stays on the file that already holds the other sections rather than creating a partial newer file that would win on read and strand them.

Reads and writes differ only in that empty-chain fallback; every other case agrees. The chain is declared once per file (`CONFIG_CHAIN`, `SESSION_CHAIN`) and every resolver walks it, so the read path, the write path, and the provenance tag rendered in results cannot disagree about precedence — the property [042's review](../../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) established with `BE-RACE-001` and `resolve_config`'s single-probe rule.

The migration is still the sole cutover: no primitive moves a file between tiers.

## Edge Cases

- **All three tiers present.** Newest wins on both read and write. The older files are dead — nothing reads them — which is what licenses the migration's convergence rule to delete rather than skip.
- **Only the middle tier present.** The common case for an adopter who upgrades the binary before re-running the bootstrap: every resolver returns `.govern/`, and nothing resolves to an un-migrated `.ductus/`.
- **Split layout** (config on one tier, session on another). The chains are independent, so each file resolves on its own merits. A 042-era project whose session was written by the older `session-file-consolidate` migration is exactly this shape.
- **Nothing present.** Read names the oldest tier (absent → defaults); write names the newest (fresh project).
- **Empty chain.** Structurally impossible — both constants are non-empty literals — and asserted with `debug_assert!` rather than handled, since a silent empty-chain fallback would resolve every path to the repo root.

## Open Questions

*None — all resolved.*
