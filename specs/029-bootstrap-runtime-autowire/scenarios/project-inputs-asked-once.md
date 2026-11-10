---
section: "Follow-on scenarios"
---

# Project-inputs-asked-at-most-once

## Context

`/ductus` collects three project inputs — name, description, languages. As 029 shipped, two flaws made the user re-enter them:

1. **Collected before the abort.** `§Inputs` prompted for all three *before* the Pre-flight Phase. On a State B adoption (ductus installed but unwired — the common first run) the user typed all three, the run then wired ductus and aborted, and the next session asked again.
2. **Never persisted as answers.** Even setting the restart aside, every routine re-run (update mode) re-prompted, because the answers were not stored where a later run could read them. The project name *was* already in `.ductus/config.toml` as `[host] project`, but the procedure never read it back; description and languages had no persisted home at all.

Surfaced 2026-06-11 during end-to-end Antigravity testing: the adopter entered name/description/languages twice across the wire-then-restart cycle.

`.ductus/config.toml` is the adopter-side configuration file (`framework/bootstrap/ductus.md` §Project Configuration). It is the proper home for these answers — not a new scratch file, and not a throwaway the abort discards.

## Behavior

Two changes, both anchored on `.ductus/config.toml` as the source of truth:

- **Persist the answers in `.ductus/config.toml`.** A `[project]` table holds all three answers — `name`, `description`, `languages`. `[host] project` is written from `project.name` as the runtime's slash-command namespace (the derived runtime view of the same value). The table is written host-side at `framework/bootstrap/ductus.md` §Collect Project Inputs (the host gathers inputs before the runtime walks, per §Instructions step 1 of the same file), additively, preserving every other section — so no runtime primitive is involved and the persistence happens on every adoption path.
- **Read back; prompt only for what is missing.** Input collection moves to a new **§Collect Project Inputs** step in `framework/bootstrap/ductus.md` that runs *after* the Pre-flight Phase (past its abort point). It resolves each input from the first available source — `$ARGUMENTS`, then `.ductus/config.toml`'s `[project]` table (`[host] project` as a fallback for configs predating `[project]`), then an interactive prompt — and prompts only for what none of those supply.

Together these make the inputs **asked at most once**:

- A pre-flight abort performs **no input prompts at all** — collection is downstream of the abort. `048-govern-acquired-runtime`'s scenario `state-b-continues-in-session` has since narrowed which writes abort: a stale-`ductus.md` rewrite still does, because the run must not proceed on instructions it has just replaced, while State-B wiring does not abort at all. The property this bullet protects is unchanged either way — an aborting run asks nothing, because §Collect Project Inputs sits past the abort point.
- The first scaffolding session prompts for whatever `.ductus/config.toml` does not yet carry, then persists it.
- Every later run — the post-restart session, and all routine update runs — reads the answers back from `.ductus/config.toml` and prompts for nothing.

The §Pre-flight abort "everything past this point is skipped" list in `framework/bootstrap/ductus.md` gains **Collect Project Inputs** as its first entry.

## Edge Cases

- **Inputs supplied via `$ARGUMENTS`.** Used directly; `.ductus/config.toml` is still written so future runs need neither the flag nor a prompt.
- **A run that does not abort.** The run flows straight through pre-flight into §Collect Project Inputs and scaffolds in the same session — inputs resolved once, just slightly later in the procedure. This was State A or State C when the scenario was written; `048-govern-acquired-runtime` retired State C and stopped State B aborting, so it is now every run except a stale-`ductus.md` rewrite.
- **User wants to change an answer.** Edit the `[project]` value (or `[host] project`) in `.ductus/config.toml`; the next `/ductus` reads the new value and re-runs the corresponding scaffold step. This is the documented way to change an input, replacing a re-prompt.
- **`.ductus/config.toml` absent or `[project]` missing.** Treated as "not yet collected" — prompt, then write. A malformed `.ductus/config.toml` still aborts per `framework/bootstrap/ductus.md` §Project Configuration before this step relies on it.
- **Agent-selection prompts are unaffected.** `framework/bootstrap/ductus.md` §Agent Selection still runs before pre-flight (it needs only `$ARGUMENTS` flags and on-disk config-dir detection). First-run auto-detect does not prompt; `--add-agent` does, and is out of scope here.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Persist the answers, or just reorder so the abort does not waste them?** Persist them — in `.ductus/config.toml`, the existing adopter config database. An earlier draft dismissed persistence on the grounds that it would need "a new state file," which was wrong: `.ductus/config.toml` is already that file and already holds the project name. Persistence is strictly broader than the reorder alone — it removes the re-ask on *every* update run, not just the one across the State-B restart. The reorder is kept on top of it so the aborting session stays completely silent (it asks nothing rather than asking-then-persisting-then-aborting). The walker-context contract (`framework/bootstrap/ductus.md` §Instructions step 1) holds: the host resolves inputs before the runtime walks the scaffolding procedure; pre-flight is host-prep that precedes both.
- **Where does the name live — `[project]` or `[host] project`?** All three inputs live in `[project]` — a section holds the thing it names, and `[project]` is the project's inputs. `[host] project` remains the runtime's slash-command namespace, written from `project.name` on every host-block update; `[project]` is the source of truth and `[host]` the derived view, so they cannot diverge as long as the user edits `[project] name` (the documented way to rename). The earlier "name only in `[host] project`" split was rejected: splitting one concept's answers across two tables to dodge a duplication that `/ductus` already keeps in sync is the wrong trade.
