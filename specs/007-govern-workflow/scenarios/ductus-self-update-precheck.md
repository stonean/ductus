---
title: "007-govern-workflow — scenario: ductus-self-update-precheck"
spec-ref: "007-govern-workflow — Distribution Model"
tags: []
---

# Ductus Self-Update Pre-Check

## Context

`/ductus`'s instructions are loaded into the running session at invocation time. When the upstream bootstrap, `framework/bootstrap/ductus.md`, has changed since the installed copy was written, the running command follows stale logic: its manifest, migration, audit and scaffolding rules are whatever shipped with the installed file, not what the run's source now carries.

Running the pipeline on that logic does the work with the wrong rules. A manifest entry the new bootstrap would skip is written, a migration it handles differently is applied, an audit it tightened runs with the loose rules. Swapping to the new instructions mid-run is not feasible, since a slash command cannot reload its own prompt. So the check runs before any of that work, and when it finds a stale copy it replaces the copy on disk and stops, so the next session loads the current instructions and the re-run completes.

## Behavior

- The check runs in the **Pre-flight Phase** (`framework/bootstrap/ductus.md`), after the run's source is resolved and before project inputs are collected, pre-run migrations run, or the framework archive is fetched.
- It makes one small fetch: the upstream bootstrap at the ref the run resolved — the latest release by default, or `main` or a named release tag through `--ref`. It never fetches the archive. A failed fetch aborts the run with an error.
- For each selected agent, it compares the upstream copy with the agent's installed bootstrap at that agent's install path, and records one status:
  - **no installed copy** — the agent has none yet (first run for it). Continue.
  - **current** — the two are byte-identical, whether or not the installed copy is pinned. Continue.
  - **stale** — they differ and the installed copy is not pinned.
  - **pinned-divergent** — they differ and the installed copy is listed in `.ductus/config.toml` `pinned.files`. Continue.
- An Antigravity skill wraps the bootstrap's body in frontmatter of its own, so for that layout the check strips the frontmatter from both sides and compares the bodies.
- For each **stale** agent, the fetched copy is written over the installed file the comparison read, and the write is checked for integrity. Placeholders stay literal, as in every installed copy of the bootstrap.
- Any stale agent then stops the run at the pre-flight abort, before inputs are collected, so no answer the user types is discarded. The notice names the source and lists the stale agents updated, and asks the user to start a new session and re-run the command they can actually invoke. Every section past the pre-flight phase is skipped.
- A **pinned-divergent** agent never stops the run. The post-scaffolding output carries one advisory line for it — `{agent}: ductus.md pinned, upstream has changed` — and only while the pinned copy actually differs from upstream.
- On the next run in a new session, the freshly written copy is what loads, the check records **current**, and the run proceeds.

## Edge Cases

- **First-ever run** — no agent has an installed copy, so every agent records **no installed copy** and the run proceeds; scaffolding installs the copy the next run compares against.
- **Adding an agent on a re-run (`--add-agent`)** — the new agent records **no installed copy**, and an existing agent's copy is still compared. If that copy is stale the run stops, and the new agent's install rides along with the re-run.
- **All selected agents pinned, all matching upstream** — every agent records **current**: no advisory, no stop.
- **All selected agents pinned, at least one divergent** — the divergent ones record **pinned-divergent**. The run proceeds, and the output carries one advisory line per divergent agent.
- **Mixed results across selected agents** — any single **stale** agent stops the run, and the notice lists every stale agent updated.
- **Bootstrap byte-identical across an upstream change** — the comparison is by content, not version, so a release that leaves the bootstrap's bytes alone records **current**.
- **A stale copy in an agent the run did not select** — not compared, not written, and no stop. It is detected on the next run that selects that agent.
- **Stale bootstrap on an adopter who has never wired the runtime** — both pre-flight checks contribute writes, and the abort carries one combined notice, so the user restarts once rather than twice.

## Open Questions

*All open questions resolved. See Resolved Questions below.*

## Resolved Questions

- **Scope of the comparison** — selected agents only. The pre-check diffs only the agents this run will write to; agents that exist in the project but are not in the run's selection are not checked. Reason: the abort exists to prevent destructive work being done with stale rules, and destructive work is per-agent. An unselected stale agent does not affect this run's correctness — it will trip the check on its very next `/ductus` run targeting it, so drift is still detected, just lazily. This keeps the abort message tightly scoped to what the user is actually doing rather than reporting drift in agents they did not ask to touch this run.
- **Pinned `ductus.md` semantics** — pinning suppresses the abort but not the information. A pinned file is still byte-compared to upstream every run: matching → recorded as "current" with no output (the pin had nothing to suppress); divergent → recorded as "pinned-divergent", the run proceeds normally (no abort), and the post-scaffolding output includes one advisory line per divergent agent: `{agent}: ductus.md pinned, upstream has changed`. Reason: the user pinned the file to opt out of automatic updates, not to opt out of knowing when their frozen version has actually drifted from upstream — that is the moment they would want to either review the upstream changes and unpin, or consciously confirm they are staying on the old version. The advisory is silent on runs where the pinned version happens to match upstream, so adopters who are deliberately and indefinitely on an old version see no recurring nag.
- **Aborting after frontmatter migration** — the pre-check moves ahead of migration, so no migration ever runs on stale rules. Migration is exactly the kind of work the new bootstrap might do differently (new fields, tightened patterns, removed migrations), and leaving the old bootstrap's migration changes on disk after a stale abort either commits the user to old migration semantics or forces a `git restore` before re-running. **Refined after it shipped:** the check first ran after the archive fetch and wrote nothing before aborting, so the next session loaded the same stale copy and aborted again, leaving a manual `curl` as the only way out. It now runs ahead of the archive fetch on a small fetch of the bootstrap alone, and writes the fresh copy before stopping, which is what lets the re-run complete.
