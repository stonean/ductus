# 033 — Explicit project rule-surface setting Plan

Implements [033 — Explicit project rule-surface setting](spec.md).

## Overview

The feature threads one new `.ductus/config.toml` setting, `[rules] surfaces`, through the two places that already act on rule-file surface, and a runtime primitive resolves the citations the setting leaves unresolved (the Technical Decision on `/ductus:analyze` and citation resolution):

1. **`framework/bootstrap/ductus.md`** (the installer) — resolve `[rules] surfaces` as a project input (prompt when unset, persist), then filter the host-built rule-file manifest entries by the configured surface(s) before `apply-manifest` runs.
2. **`framework/commands/review.md`** (the `/ductus:review` loader, spec 024) — consult `[rules] surfaces` in `framework/reference/review.md` §Resolve target and scope item 5; when set, it replaces the detected-stack filter; when unset, stack derivation remains the fallback.

`/ductus:analyze` is deliberately untouched: it already loads *every* rule file regardless of stack, and that load-all behavior is what makes cross-surface citation resolution work — so surface selection must not reach it.

## Technical Decisions

### Setting schema — `[rules] surfaces`

`.ductus/config.toml` gains a `[rules]` table with one key:

```toml
[rules]
surfaces = ["backend"]   # list; members ∈ {"backend", "frontend"}; full-stack = both
```

- A **list**, not an enum — composes with the existing suffix model (`-backend`/`-frontend`/`-cross`) and admits future surfaces without a schema change.
- Members are validated against `{"backend", "frontend"}`. `"cross"` is rejected: `-cross.md` files are unconditional and not a selectable surface.
- **Unset** is a first-class state meaning "derive" — the loader falls back to 024's stack detection and the installer installs all rule files (today's behavior), so existing adopters are unaffected until `/ductus` next prompts.
- An unrecognized key in `[rules]` is named, with the section and the file, and the run continues (`CFG-ENV-012`), as `specs/019-config-decisions/plan.md` §Unknown keys are reported by name, never dropped decides for every `ductus`-owned section; `runtime/src/schema/config_keys.rs` registers `surfaces` as this section's one key.

### Installer (`framework/bootstrap/ductus.md`)

- **Input resolution.** Add `[rules] surfaces` to `framework/bootstrap/ductus.md` §Collect Project Inputs as a resolved input: read from `.ductus/config.toml` `[rules] surfaces`; if absent, prompt ("Which rule surfaces does this project need? backend / frontend / both"); persist the answer into `.ductus/config.toml` `[rules]`, preserving every other section (same pattern as `[project] name/description/languages`). On a routine re-run the value is present, so no prompt fires.
- **Manifest filter.** The host already builds `manifest-entries`. Filter the rule-file entries (`framework/rules/*.md` → adopter `specs/rules/*.md`) to those whose suffix matches a configured surface, **plus every `*-cross.md` unconditionally**, before calling `apply-manifest`. Entries for unconfigured surfaces are simply omitted from the manifest — never added to any prune/enforce set, so an already-installed file for a now-unconfigured surface is **left in place** (rule files are not in `enforce-directories`, which holds each agent's slash-command directory and `.ductus/reference/` only: `framework/bootstrap/ductus.md` §Instructions step 8).
- **Contradiction notice.** When `surfaces` excludes a surface that `[project] languages` clearly implies (e.g., `surfaces=["backend"]` but a frontend language is listed), emit one advisory line; the explicit setting still wins. No prompt.

### Review loader (`framework/commands/review.md`)

`framework/reference/review.md` §Resolve target and scope item 5 currently: discover by suffix → filter by detected stack → apply disabled-files filter. Insert a surface source ahead of the stack filter:

- Read `.ductus/config.toml` `[rules] surfaces`. **If set**, keep rule files whose surface is in `surfaces`, plus every `*-cross.md` and every unrecognized-suffix file; this *replaces* the detected-stack filter. **If unset**, run the detected-stack filter exactly as today.
- The `framework/commands/review.md` §Inputs section documents `[rules] surfaces` alongside `[review] tech-stack-verified` and `[[review.disabled-rule-files]]`.
- The 025 disabled-files filter runs after, unchanged. A file already excluded by surface needs no opt-out entry.

### `/ductus:analyze` and citation resolution

`/ductus:analyze` continues to load every discovered rule file regardless of `surfaces` (the constitution's "loads every discovered file regardless of stack" invariant), and `[rules] surfaces` scopes `/ductus:review` enforcement only. A citation to a rule on a surface whose file was never installed resolves from an index of the shipped rule IDs: `check-rule-ids` (owned by [067](../067-analyze-and-audit-checks/spec.md)) reports it `not-installed`, informational and never missing, and analyze records it unexamined under `rule-not-installed`, since the rule's text was not available.

**The index is compiled into the binary** (`runtime/src/primitives/shipped_rule_ids.txt`, `include_str!`ed by `check_rule_ids.rs`), generated from `framework/rules/*.md`, and `shipped_rule_index_matches_the_framework_rules` fails when it drifts (regenerate with `BLESS_RULE_INDEX=1`). The runtime ships in lockstep with the framework, so the binary holds the IDs of the version an adopter runs with no file in the adopter's tree, and no second shipped file needs a manifest entry, a path in an adopter's checkout or a generator workflow step. A generated file shipped beside the rules would need all three, and its drift check would run in the same test suite anyway.

### Documentation

`README.md` (and any `.ductus/config.toml` reference) documents `[rules] surfaces`: accepted values, the derive-when-unset fallback, and its precedence over stack detection.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Modify | Resolve/prompt/persist `[rules] surfaces`; filter manifest rule-file entries by surface (+ always `-cross`); contradiction notice |
| `framework/commands/review.md` | Modify | `framework/reference/review.md` §Resolve target and scope item 5 consults `[rules] surfaces` (replaces stack filter when set, falls back when unset); §Inputs documents the setting |
| `framework/reference/analyze.md` | Modify | Note that `surfaces` scopes review enforcement only, not analyze citation resolution |
| `runtime/src/primitives/check_rule_ids.rs` | Modify | The not-installed state, read from the compiled index |
| `runtime/src/primitives/shipped_rule_ids.txt` | Create | The index of the shipped rule IDs |
| `runtime/src/interpreter/analyze_tally.rs` | Modify | Record `rule-not-installed` unexamined |
| `README.md` | Modify | Document the `[rules] surfaces` setting and its precedence |

`.claude/commands/ductus/*.md` regenerate from the `framework/commands/*` sources via the pre-commit hook — not edited by hand.

## Trade-offs

- **List vs. single enum** — chose list. An enum (`backend|frontend|fullstack`) is simpler to validate but does not compose with the suffix model or future surfaces; the list mirrors how files are already classified.
- **Prompt-in-`/ductus` vs. replace derivation everywhere** — chose: derivation stays the default, the prompt lives only in `/ductus`, and the persisted value is an override. Forcing an "ask" into every command would reverse [017-derive-dont-ask](../017-derive-dont-ask/spec.md); confining it to the installer keeps zero-config projects zero-config.
- **Leave vs. prune on surface removal** — chose leave-in-place. Deleting a now-irrelevant rule file is destructive and could drop adopter-pinned edits; omitting it from the manifest (no update, no delete) is reversible and consistent with how `[pinned]` already protects files. A future explicit prune can be added if demand appears.
- **Analyze load-all vs. surface-scoped** — chose load-all. Scoping analyze to `surfaces` would make a legitimate cross-surface citation (a backend spec citing a frontend rule) report as an unknown rule; resolution must stay global while only enforcement is scoped. Because installation is scoped, load-all alone does not resolve a citation whose file was never installed; the shipped-ID index above does (the rejected alternatives, installing every file and resolving silently as found, are in the scenario `an-out-of-surface-citation-resolves`).
- **Known limitation** — the contradiction check is heuristic (compares `surfaces` against `[project] languages`), advisory only, and will not catch every mismatch; the operator's explicit choice is always honored.

## Cross-spec impact

The implementation edits `framework/commands/review.md` (conceptually owned by [020-code-review](../020-code-review/spec.md) and [024-rule-loader](../024-rule-loader/spec.md)) and `framework/bootstrap/ductus.md`. The design is **additive**: when `[rules] surfaces` is unset, 024's stack derivation and 020's review behavior are byte-for-byte unchanged. Because 033 layers a new source ahead of an unchanged fallback rather than altering the existing contracts, neither 024 nor 020 was edited, and neither carries a back-link to 033. The cross-spec impact that holds is 067's, through `cross-spec-impact:` and its signpost.
