# 049 — Rename govern to ductus Plan

Implements [049 — Rename govern to ductus](spec.md).

## Overview

The rename is one substitution applied to four distinct token families, plus one genuine
behavior change and one registry entry. The bulk is mechanical and lands as a uniform sweep
across live artifacts (§spec-lifecycle case (a), so `done` specs stay `done`). The
non-mechanical parts are small and enumerated here:

1. **Path resolution gains a third tier.** The runtime today resolves the project config and
   session file new-first-then-legacy across two locations; the new per-project directory
   makes it three. This is new runtime behavior, so it lands as a scenario under
   [022](../022-deterministic-runtime/spec.md) per `AGENTS.md`'s runtime-routing rule.
2. **A migration entry** converges adopter state the sweep cannot reach.
3. **The bootstrap renames its own entry point mid-run**, which is what makes the cutover
   self-completing rather than a flag day.

Ordering is forced by dogfooding: the runtime must understand both layouts *before* this
repo's own `.govern/` moves, or the binary stops finding its config mid-rename.

## Technical Decisions

### The substitution table, and the order it must be applied in

The sweep is not one find-and-replace. Four token families change, and they nest — `.govern/`
contains `govern`, `mcp__gvrn__` contains `gvrn` — so a shorter rule applied first corrupts a
longer one. Apply longest-first:

| # | Old | New | Reaches |
| --- | --- | --- | --- |
| 1 | `.govern/` | `.ductus/` | per-project directory (`config.toml`, `session.toml`, `scripts/`, `constitution.md`) |
| 2 | `mcp__gvrn__` / `mcp:gvrn:` / `mcp(gvrn/` / `gvrn*` | `mcp__ductus__` / `mcp:ductus:` / `mcp(ductus/` / `ductus*` | per-agent permission grammars (four of them) |
| 3 | `gvrn` | `ductus` | binary, crate, lib, MCP server key, tag prefix, store filename |
| 4 | `/gov:` and `commands/gov/` | `/ductus:` and `commands/ductus/` | command namespace and installed command dir |
| 5 | `govern` (bare) | `ductus` | project name, `/govern` bootstrap command, repo URL, `bootstrap/govern.md` |

**The must-not-touch list is the load-bearing half.** Five classes survive verbatim; a sweep
that misses any one of them ships a lie or breaks an adopter:

- **Legacy path constants.** `runtime/src/schema/paths.rs:46` (`LEGACY_CONFIG_FILE =
  ".govern.toml"`) and `:56` (`LEGACY_SESSION_FILE = ".govern.session.toml"`) name files that
  exist in adopter checkouts right now. Rewriting them silently drops the pre-042 fallback.
  `.govern/config.toml` and `.govern/session.toml` join them as a middle tier (see below)
  rather than being rewritten.
- **Version- and tag-adjacent occurrences** — the 69 the spec measured (`gvrn-v0.23.0`,
  `gvrn 0.24.0`, `introduced_in = "0.22.0"`). These name published artifacts.
- **CHANGELOG entries describing past releases**, and the historical `gvrn-v*` tags.
- **Historical migration bodies.** `framework/migrations/govern-dir-consolidate.md` and
  `governance-config-rename.md` describe migrations that genuinely write `.govern/` and
  `.govern.toml`. See the dedicated decision below.
- **Legacy-layout test fixtures.** `runtime/tests/fixtures/**/.govern.session.toml` and
  `.govern.toml` exist to prove the legacy fallback still works;
  `scripts/audit/fixture-session-shape.sh:48` asserts on exactly those names.

### Path resolution becomes a three-tier chain

Grounded in `runtime/src/schema/paths.rs:40-56` and `:143-153`. Today:

```rust
pub(crate) const CONFIG_FILE: &str = ".govern/config.toml";        // new (042)
pub(crate) const LEGACY_CONFIG_FILE: &str = ".govern.toml";        // legacy (pre-042)
```

with `active_path(repo, new, legacy)` implementing new-wins-then-legacy for writes and
`config_path` / `session_path` doing the same for reads.

The rename inserts a tier rather than replacing one: `.ductus/config.toml` becomes primary,
`.govern/config.toml` demotes to a middle tier, `.govern.toml` stays oldest. `active_path`
generalizes from a `(new, legacy)` pair to an ordered slice, first-existing-wins, defaulting
to the primary for a fresh project. The same shape applies to the session file. This preserves
042's guarantee one level up — an adopter who upgrades the binary before re-running the
bootstrap is never broken, on either of the two prior layouts.

This is the one **behavior** change in the spec, so per `AGENTS.md` it lands as a scenario
under 022 with `specs/022-deterministic-runtime/data-model.md` updated, and 022 takes the
`done → in-progress` back-edge. The sweep of `runtime/src/` prose is mechanical and rides
along without reopening anything.

### `govern-dir-consolidate` keeps writing `.govern/`; the new migration completes the second hop

`framework/migrations/govern-dir-consolidate.md:1-39` is a shipped contract that moves an
adopter's root files *into* `.govern/`. Rewriting its body to name `.ductus/` would make it
claim to do something it does not, and would put its `Introduced in: gvrn 0.22.0` line
(which the spec's Resolved Questions cites by name as a must-survive occurrence) next to
prose describing a release that never existed under that name.

So it is left as written, under AC1's "recording history" carve-out, and composition does the
work. The registry sorts by `introduced_in` ascending
(`framework/migrations.toml:15-17`), which gives three adopter populations one deterministic
path each:

| Adopter's `last_applied` | Runs | Ends at |
| --- | --- | --- |
| < 0.22.0 | `govern-dir-consolidate`, then the rename migration | `.ductus/` |
| ≥ 0.22.0, pre-rename | the rename migration only | `.ductus/` |
| already renamed | neither (idempotent no-op) | `.ductus/` |

Two moves in one run for the oldest population is the cost, and it is paid once. Retargeting
`govern-dir-consolidate` would save that hop at the price of rewriting a published contract —
a bad trade. This is what AC11 asserts.

### The bootstrap renames its own entry point mid-run

`framework/bootstrap/govern.md` → `framework/bootstrap/ductus.md`, and the `/govern` command
becomes `/ductus`. The self-update fetch inside the bootstrap is what makes this converge
without a flag day: an adopter still holding the old bootstrap fetches through GitHub's rename
redirect, receives the *new* bootstrap carrying the new URLs and paths, and the self-update
check writes it (`framework/bootstrap/govern.md` §Self-update check, lines 245-306). Their next
run is already the new command. The redirect only has to survive until each adopter's next run
— which is why the retired repo name can never be recreated (already recorded in `AGENTS.md`).

Slash-command cleanup then removes the old `commands/gov/` copies, and the migration installs
the new namespace. AC5 is satisfied by the redirect plus the migration's own summary line
naming `/ductus`.

### `[host] project` default and the basename fallback

Grounded in `runtime/src/host.rs:38` (`FALLBACK_PROJECT = "gov"`),
`runtime/src/host.rs:147-152` (project defaults to the repo directory basename), and
`scripts/gen-claude-commands.sh:39` (`PROJECT="gov"`). All three become `ductus`.

This repo's own `.govern/config.toml` pins `project = "gov"` with a comment explaining that the
basename fallback would otherwise yield `govern`. After the rename the checkout basename *is*
`ductus`, so the explicit pin and the fallback agree for the first time — but the pin stays,
because Family 17 (`host-namespace-parity`) enforces agreement between the config value and the
installed command directory, and an implicit value is one directory rename away from drifting.
The stale comment gets corrected rather than deleted.

### Two migration entries, not one

The spec's §Sequencing with 048 leaves this open ("they must compose — or 048's is authored
already knowing the new names, and there is only one"). Because 049 lands first, 048's entry
does not exist yet, so it is authored afterward against final names and the two are disjoint:
049's rewrites the MCP server **key**, the permission entries, the per-project directory, and
the installed command files; 048's rewrites the MCP **command path** to the acquired-runtime
store. They touch the same file and different keys, in registry order. Recorded here so 048's
task 8 is written knowing it owns only the path.

### Goldens are re-blessed, never hand-edited

`runtime/tests/parity.rs:722-739` supports `BLESS=1` to overwrite goldens from captured stdout.
The sweep changes command names and paths inside all 9 goldens under `runtime/tests/golden/`,
so they are regenerated by running the suite with `BLESS=1` after the source sweep — AC8's
explicit requirement. Hand-editing them would defeat the check they exist to perform. Note the
version line is a `{{runtime-version}}` placeholder (`AGENTS.md` runtime-release entry), so the
version bump itself needs no bless.

### The retired-name guard gets the new tokens

`scripts/audit/introducing-drift.sh:48-58` holds the rename catalog that flags backticked old
names in `done` spec bodies. The rename adds its tokens there. 049 itself is the *introducing*
spec — the old names are the subject of its prose — so it carries the file-scope marker
`<!-- audit:ignore-introducing-drift:file -->` the script documents for exactly this case
(lines 28-32). That is the family's designed exemption for an introducing spec, not a disabled
family, so AC7 is unaffected (AC7 names the installer, registry, namespace, and host-namespace
families).

### No `data-model.md`

The rename introduces no domain entity and no new primitive result shape. It changes two path
*constants* and adds one registry row. The resolution-order contract is the one structural
change, and its canonical home is `specs/022-deterministic-runtime/data-model.md`, which the
022 back-edge updates. Adding a second record here would create the divergent-copy problem
`AGENTS.md`'s canonical-table rule exists to prevent.

## Affected Files

Planning aid; the write boundary is derived from git history at implement time.

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/schema/paths.rs` | Modify | Three-tier config/session resolution; new constants |
| `runtime/src/host.rs` | Modify | `FALLBACK_PROJECT` → `ductus` |
| `runtime/Cargo.toml` | Modify | Crate, bin, lib name; description; repository URL |
| `runtime/src/**` | Modify | Server key, `TOOL_NAMES` prose, doc comments, error strings |
| `runtime/tests/golden/*.jsonl` (9) | Modify | Re-blessed via `BLESS=1` |
| `runtime/tests/fixtures/**` | Modify | New-layout fixtures renamed; legacy-layout fixtures untouched |
| `framework/bootstrap/govern.md` | Rename + modify | → `ductus.md`; self-update URL, MCP shapes, permission seeds |
| `framework/commands/*.md` | Modify | Namespace placeholders, primitive prefixes |
| `framework/constitution.md` | Modify | Name, directory, runtime references |
| `framework/migrations.toml` | Modify | New `[[migrations]]` entry |
| `framework/migrations/ductus-rename.md` | Create | Migration procedure body |
| `framework/migrations/govern-dir-consolidate.md` | **Unchanged** | Records history (AC1 carve-out) |
| `scripts/audit/*.sh` | Modify | Hardcoded names; `introducing-drift` catalog |
| `scripts/gen-*.sh` | Modify | `PROJECT`, permission-string emitters |
| `.github/workflows/runtime-release.yml` | Modify | Tag trigger `gvrn-v*` → `ductus-v*`; asset names |
| `README.md`, `AGENTS.md`, `CLAUDE.md` | Modify | Name, install route, contributor-local checklist |
| `specs/NNN-*/**` | Modify | Sweep minus the published-artifact exception |
| `specs/022-deterministic-runtime/` | Modify | Back-edge scenario + `data-model.md` |
| `.govern/` → `.ductus/` | Rename | Dogfooding |
| `.mcp.json` | Modify | Server key and command |
| `.claude/commands/gov/` → `.claude/commands/ductus/` | Rename | Regenerated, not hand-moved |

## Trade-offs

**Considered and rejected: retargeting `govern-dir-consolidate` to `.ductus/`.** Would spare
the oldest adopters a second directory hop. Rejected because it rewrites a shipped contract to
describe behavior it never had, and strands its `Introduced in: gvrn 0.22.0` line against
prose naming a release that does not exist. Composition costs one extra move, once.

**Considered and rejected: a compatibility shim keeping `gvrn` on `PATH` as an alias.**
Rejected because it keeps the retired name in daily use indefinitely and gives adopters no
signal to converge, which is precisely what the deprecation release on the retired crate does
better — it reaches them at the moment they next look.

**Considered and rejected: sweeping spec bodies wholesale, including version-adjacent
occurrences.** Rejected in the spec's Resolved Questions; recorded here because it is the
tempting simplification. It would make `framework/migrations/govern-dir-consolidate.md` claim
a `ductus 0.22.0` release that was never published.

**Known limitation: the sweep cannot verify the GitHub rename or the crates.io publish.** Both
are out-of-repo operator actions. AC12 covers the crate; the repo rename is verified only by
the adopter-facing URLs resolving, and its one failure mode (recreating the retired repo name)
has no detection — which is why it is recorded as a permanent constraint in `AGENTS.md` rather
than as a check.

**Known limitation: `/ductus:implement`'s derived write boundary is wrong for this spec.**
`derive-boundary` returns `["AGENTS.md", "specs/049-rename-govern-to-ductus/**"]` from the spec
directory's commit history, while the rename touches the whole repo. Implementation seeds an
explicit `write-boundary` in the session rather than letting the derived one block every edit.
