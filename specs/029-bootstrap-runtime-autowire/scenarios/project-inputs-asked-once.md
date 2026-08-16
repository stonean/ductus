---
section: "Follow-on scenarios"
---

# Project-inputs-asked-at-most-once

## Context

`/ductus` collects three project inputs — name, description, languages. As 029 shipped, two flaws made the user re-enter them:

1. **Collected before the abort.** `§Inputs` prompted for all three *before* the Pre-flight Phase. On a State B adoption (ductus installed but unwired — the common first run) the user typed all three, the run then wired ductus and aborted, and the next session asked again.
2. **Never persisted as answers.** Even setting the restart aside, every routine re-run (update mode) re-prompted, because the answers were not stored where a later run could read them. The project name *was* already in `.govern.toml` as `[host] project`, but the procedure never read it back; description and languages had no persisted home at all.

Surfaced 2026-06-11 during end-to-end Antigravity testing: the adopter entered name/description/languages twice across the wire-then-restart cycle.

`.govern.toml` is the adopter-side configuration database (per [AGENTS.md](../../../AGENTS.md) Workflow and §Project Configuration). It is the proper home for these answers — not a new scratch file, and not a throwaway the abort discards.

## Behavior

Two changes, both anchored on `.govern.toml` as the source of truth:

- **Persist the answers in `.govern.toml`.** A `[project]` table holds all three answers — `name`, `description`, `languages`. `[host] project` is written from `project.name` as the runtime's slash-command namespace (the derived runtime view of the same value). The table is written host-side at §Collect Project Inputs (the host gathers inputs before the runtime walks, per §Instructions step 1), additively, preserving every other section — so no runtime primitive is involved and the persistence happens on every adoption path.
- **Read back; prompt only for what is missing.** Input collection moves to a new **§Collect Project Inputs** step that runs *after* the Pre-flight Phase (past its abort point). It resolves each input from the first available source — `$ARGUMENTS`, then `.govern.toml`'s `[project]` table (`[host] project` as a fallback for configs predating `[project]`), then an interactive prompt — and prompts only for what none of those supply.

Together these make the inputs **asked at most once**:

- A pre-flight abort (State B wiring, or a stale-`ductus.md` rewrite) performs **no input prompts at all** — collection is downstream of the abort.
- The first scaffolding session prompts for whatever `.govern.toml` does not yet carry, then persists it.
- Every later run — the post-restart session, and all routine update runs — reads the answers back from `.govern.toml` and prompts for nothing.

The §Pre-flight abort "everything past this point is skipped" list gains **Collect Project Inputs** as its first entry.

## Edge Cases

- **Inputs supplied via `$ARGUMENTS`.** Used directly; `.govern.toml` is still written so future runs need neither the flag nor a prompt.
- **State A or State C (no abort).** The run flows straight through pre-flight into §Collect Project Inputs and scaffolds in the same session — inputs resolved once, just slightly later in the procedure.
- **User wants to change an answer.** Edit the `[project]` value (or `[host] project`) in `.govern.toml`; the next `/ductus` reads the new value and re-runs the corresponding scaffold step. This is the documented way to change an input, replacing a re-prompt.
- **`.govern.toml` absent or `[project]` missing.** Treated as "not yet collected" — prompt, then write. A malformed `.govern.toml` still aborts per §Project Configuration before this step relies on it.
- **Agent-selection prompts are unaffected.** §Agent Selection still runs before pre-flight (it needs only `$ARGUMENTS` flags and on-disk config-dir detection). First-run auto-detect does not prompt; `--add-agent` does, and is out of scope here.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Persist the answers, or just reorder so the abort does not waste them?** Persist them — in `.govern.toml`, the existing adopter config database. An earlier draft dismissed persistence on the grounds that it would need "a new state file," which was wrong: `.govern.toml` is already that file and already holds the project name. Persistence is strictly broader than the reorder alone — it removes the re-ask on *every* update run, not just the one across the State-B restart. The reorder is kept on top of it so the aborting session stays completely silent (it asks nothing rather than asking-then-persisting-then-aborting). The walker-context contract (§Instructions step 1) holds: the host resolves inputs before the runtime walks the scaffolding procedure; pre-flight is host-prep that precedes both.
- **Where does the name live — `[project]` or `[host] project`?** All three inputs live in `[project]` — a section holds the thing it names, and `[project]` is the project's inputs. `[host] project` remains the runtime's slash-command namespace, written from `project.name` on every host-block update; `[project]` is the source of truth and `[host]` the derived view, so they cannot diverge as long as the user edits `[project] name` (the documented way to rename). The earlier "name only in `[host] project`" split was rejected: splitting one concept's answers across two tables to dodge a duplication that `/ductus` already keeps in sync is the wrong trade.
