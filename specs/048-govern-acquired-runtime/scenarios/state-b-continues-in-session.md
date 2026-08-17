---
section: "Follow-on scenarios"
---

# State-b-continues-in-session

## Context

The Pre-flight Phase's State B — runtime not live — acquires the binary, wires
MCP, seeds permissions, then **aborts** so the next session runs the
deterministic path. The abort exists so the expensive remainder (migrations, the
archive fetch, scaffolding) is not spent on the markdown path.

But after acquisition the binary is **already on disk** at `.ductus/bin/ductus`.
The MCP server is not live this session; the **CLI is**. Every primitive is
reachable as `./.ductus/bin/ductus <primitive>` — the same deterministic code,
a different invocation surface (spec 022 AC1 ships both). The abort therefore
buys nothing the CLI could not already give this run.

The route is even pre-authorized: the Agent Registry's Claude
`settings_template` already grants `Bash(.ductus/bin/ductus *)` and
`Bash(~/.ductus/bin/ductus *)`, and the Permission Setup seed writes them before
the probe. Nothing needs a new grant.

Measured 2026-08-16 on the real adopter bootstrap (048 AC10, the adopter project): **three**
restarts.

1. the installed pre-rename `govern.md` self-updates itself — inherent, since a
   copy predating the Pre-flight Phase cannot run the combined abort;
2. pre-flight acquires and wires, then aborts;
3. migrations (including the rename to `ductus.md`) and scaffolding run.

Only the second is avoidable. `AC10` promises *"one `/ductus` run plus one
restart"*, which today holds only for an adopter whose bootstrap is already
current — every other adopter pays one more than the criterion states.

## Behavior

**State B continues in the same session through the CLI.** After acquiring the
binary, materializing the pointer, wiring MCP and seeding permissions, the run
does **not** stop. It proceeds through Collect Project Inputs, Pre-run
Migrations, the archive fetch, Shared Files, Per-Agent Scaffolding and the rest,
invoking each primitive as `{pointer-path} <primitive>` rather than as an MCP
tool.

**The single abort moves to the end.** One restart is still required, because
the MCP server this run just registered loads only on a fresh session — so the
*next* session calls primitives as tools rather than through the CLI. The abort
is deferred to after the scaffolding it used to precede, and its message says
what it now means: the work is done, and the restart is for the tool surface.

**Two primitives needed a CLI argument surface before this was possible.**
`apply-manifest`'s `entries` / `pinned` / `substitutions` and
`enforce-manifest`'s `expected` / `pinned` are arrays and maps of objects,
which clap cannot express as flags; they were `#[arg(skip)]`, reachable only
through the JSON context. Moving the abort without fixing that would have
carried the run **past Shared Files having written nothing** — silently, since
an empty manifest is a legal manifest and the primitive reports success over
it. Each field gains a sibling `--{field}-json PATH` flag, `serde(skip)` so the
MCP tool contract is unchanged. An unreadable or malformed file is an error,
never an empty default: the silent-empty path is the entire hazard.

**The self-update restart is untouched.** A stale `ductus.md` still aborts
before anything else, because the run must not proceed on instructions it is
about to replace. An adopter carrying a bootstrap that predates the Pre-flight
Phase still pays that hop; it cannot be collapsed, since the installed copy
cannot execute a phase it does not contain.

**Nothing changes for State A.** A run whose runtime is already live is
unaffected — it never entered State B.

## Edge Cases

- **A step with no CLI equivalent**: it stays on the step's documented
  markdown-only specification for that step only, exactly as State A's
  primitive-error fallback already does — the run does not abandon the
  deterministic path wholesale.
- **The acquired binary will not execute** after install: that is already an
  acquisition failure and halts the run, before this scenario applies.
- **A migration that needs the runtime** (`criterion-label-backfill` calls
  `label-criteria`): satisfied by the CLI, which is what makes moving the
  migrations ahead of the restart possible at all.
- **The wiring was skipped** because the MCP file is not valid JSON: acquisition
  still happened and the pointer exists, so the run still continues by CLI; only
  the closing restart notice changes to say the registration was skipped.
- **A `surface-instruction` agent** (Auggie, Antigravity) never gets an MCP file
  written by `/ductus` at all, so the closing abort keeps surfacing the one-line
  registration command — the run still completes its work first.

## Open Questions

*None — see Resolved Questions.*

## Resolved Questions

- **Does any step between the pre-flight phase and the end of scaffolding
  genuinely require the MCP surface rather than the CLI?** **Yes — two did, and
  the expectation recorded here was wrong.** Resolved 2026-08-17 by walking the
  primitives, which is the check this question asked for.

  Eight primitives run after the pre-flight phase: `label-criteria`,
  `check-orphaned-references`, `merge-managed-block`, `fetch-archive`,
  `extract-archive`, `write-session`, `apply-manifest`, and
  `enforce-manifest`. All eight have CLI subcommands, so *availability* was
  never the constraint. **Arguments** were: `apply-manifest`'s `entries` — the
  manifest itself — plus `pinned` and `substitutions`, and
  `enforce-manifest`'s `expected` and `pinned`, were all `#[arg(skip)]`,
  documented "Set via JSON context — not exposed as CLI flags."

  Called from the CLI as it stood, `apply-manifest` would have received an
  empty manifest and copied nothing, **reporting success** — an empty manifest
  is a legal one. The run would have continued through Per-Agent Scaffolding
  on a project that never received a single shared file. That is precisely the
  "strands the run halfway" outcome this question was written to prevent, and
  the question is the only reason it was caught before the abort moved.

  The fix is the `--{field}-json PATH` surface described in Behavior, which
  reverses the original `arg(skip)` decision deliberately: that decision was
  taken when the CLI surface of these two primitives was "a debug entry point,
  not the production path", and State B makes the CLI *the* production path
  for one whole run.
