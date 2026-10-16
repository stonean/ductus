---
section: "The primitive library"
---

# Write-session-primitive

## Context

The session state file at `<repo>/.ductus/session.toml` (repo root, gitignored, TOML) is read by the `dashboard` primitive (`runtime/src/primitives/dashboard.rs`) but written only by the host. [`/{project}:target`](../../../framework/commands/target.md) step 7 and [`/{project}:amend`](../../../framework/commands/amend.md)'s scenario-route step 4 both invoke the host's file-writing tool for the session file, with prose specifying tempfile+rename atomic-write semantics.

On Claude Code this triggers a per-invocation `Write(.ductus/session.toml)` permission prompt that documented `Write(...)` permissions entries in `framework/bootstrap/configure/claude.md` have not been able to suppress reliably across sessions. MCP tool calls go through a separate permission lane, so once a user allows `mcp__ductus__write-session` the prompt is gone for the lifetime of the project.

The "host responsibility — the runtime exposes no session-shaped primitive" wording in `framework/commands/target.md` and `framework/commands/amend.md` is descriptive of the pre-0.9.0 state, not a constitutional constraint. Walking the §runtime-boundary eligibility criteria for the write path: deterministic (atomic-write of a tiny TOML doc is fully mechanical); currently mechanical (the prose already prescribes the exact atomic-write semantics); degradation-not-failure when removed (the markdown-only path keeps the host write). All three criteria pass, and the read path is already in the runtime — the write path is the asymmetry.

Spec 022 establishes the runtime's primitive library and the rule that state-modifying primitives use tempfile+rename. The session file is one of two durable journals named by the spec (markdown + `.ductus/session.toml`, per [plan.md §No data persistence outside session file + markdown](../plan.md#no-data-persistence-outside-session-file--markdown)) — the runtime's write surface ought to cover both.

> **History note.** The first ship of `write-session` (gvrn 0.9.0) wrote `<repo>/.claude/gov-session.json` — a hardcoded path that baked in both the AI CLI's config directory (`.claude/` for Claude Code) and the adopting project's name (`gov-session.json` for this repo). That choice broke adopters whose project or AI CLI didn't match the runtime's baked-in constants (observed against an adopter on Claude whose project name differed from this repo's, so its session would have been `.claude/{project}-session.json`). The 0.10.0 consolidation moved the file to `.govern.session.toml` at the repo root: one path for every adopter, no `{cli-config-dir}` or `{project}` resolution. **That root filename is not where the file lives today, and the retired spelling above is deliberate.** 042 relocated the file under a per-project directory and 049/050 renamed that directory, so the current path is `.ductus/session.toml`; this paragraph records the *sequence* of path choices, so naming 0.10.0's destination by its later name would assert that 0.10.0 did something it did not. A rename sweep did exactly that and it stood until it was caught — the repair is recorded here rather than in a commit message because a sweep must not rewrite the record of the thing being swept, which is the one case [§drift-prevention](../../../framework/constitution.md#drift-prevention) exempts from a blanket substitution.

## Behavior

New primitive: `write-session`. MCP tool name `write-session`; CLI subcommand `ductus write-session`.

Inputs (kebab-case CLI args; kebab-case TOML keys on disk). A call is one of three write shapes — a *target write* (`feature` + `path`), a *host-config write* (`cli-config-dir` alone, `cli-config-dir-per-contributor`), or a *clear write* (`clear`, `scaffolding-primitives`):

- `feature` — string, feature slug (e.g., `022-deterministic-runtime`). Supplied together with `path`, it makes the call a target write; one without the other is a `PrimitiveError::MissingArgument`.
- `path` — string, repo-relative spec directory (e.g., `specs/022-deterministic-runtime`). Named `path` to match the existing convention.
- `scenario` — optional string, scenario slug, on a target write only. Both `scenario` and `scenario-path` MUST be supplied together or both omitted; supplying one without the other is a `PrimitiveError::MissingArgument` naming the absent field. A target write that omits both clears any previously set scenario.
- `scenario-path` — optional string, repo-relative scenario file path (e.g., `specs/022-deterministic-runtime/scenarios/write-session-primitive.md`). Required iff `scenario` is set.
- `cli-config-dir` — optional string, the per-contributor agent config-dir name. Alone, it makes a host-config write that preserves the target; on a target or clear write it overrides the preserved value.
- `clear` — optional boolean. A clear write removes the target while preserving `cli-config-dir`, and is rejected alongside any target field.

Returns:

- `path` — repo-relative path of the shared default written: the active file of the session resolution chain, `.ductus/session.toml` once the layout is migrated; kept on the result for symmetry with other write primitives.
- `own-path` — the per-process file an identified process's target or clear write also wrote (062); absent otherwise. The result carries 062's identity, peer, expiry, unreadable and notice fields beside it.
- `created` — boolean; `true` when the file did not exist before this call, `false` when an existing file was overwritten.

Semantics:

- Writes the shared default, `.ductus/session.toml`, which always holds the working tree's most recent target change. An identified process — one launched with `DUCTUS_SESSION` or a platform session identity — also writes its own target to `.ductus/sessions/{key}.toml` on the `.ductus/` layout (062). Neither path has AI-CLI or project-name variability to parameterize.
- The shared default's top-level keys are `feature`, `path`, optional `scenario`, optional `scenario-path`, `set-at` (ISO 8601 UTC, the primitive's own clock), and optional `cli-config-dir`. Each write shape replaces only its own keys: a target write the target keys and `set-at`, a host-config write `cli-config-dir`, a clear write removes the target keys and `set-at`. The pair `scenario` + `scenario-path` appears only when both inputs are present; their absence means "no scenario targeted." Keys are kebab-case to align with `.ductus/config.toml`'s on-disk format.
- Uses the same tempfile-in-target-directory + atomic rename pattern the runtime's other state-modifying primitives (`mark-task`, `mark-criterion`, `set-status`) already share.
- TOML encoding: `toml::to_string` with field order matching the struct (feature, path, scenario, scenario-path, set-at, cli-config-dir) so the parity byte-equality check on `.ductus/session.toml` is stable.

[`framework/commands/target.md`](../../../framework/commands/target.md) step 7 invokes this primitive on the deterministic path. [`framework/commands/amend.md`](../../../framework/commands/amend.md) scenario-route step 4 of the record block invokes it the same way. Both keep a markdown-only fallback that writes the TOML document by hand with the same atomic-write semantics.

[`framework/runtime-tools.txt`](../../../framework/runtime-tools.txt) carries the `write-session` line. The runtime version bumps to `0.10.0` for the consolidation (breaking: removes the `session-path` capability the runtime never exposed; changes the on-disk format from JSON to TOML; changes the file location). Parity coverage extends to the target command's deterministic path so the byte-equality check on `.ductus/session.toml` is satisfied by the runtime-written file.

## Edge Cases

- **Existing session file.** Merged rather than overwritten in full: each write shape carries forward the keys it does not own (`cli-config-dir-per-contributor`, `scaffolding-primitives`). A key outside the known set is not carried forward, and a default that does not parse is replaced and named in the result's `unreadable`.
- **`scenario` provided without `scenario-path`, or vice versa.** `PrimitiveError::MissingArgument` with a message naming the absent field and the primitive (`write-session`). The pair is atomic by design.
- **Scenario file at `scenario-path` does not exist.** Primitive writes the path regardless; staleness is the caller's concern, matching `dashboard`'s tolerance for stale scenario paths in `load_session_target`.
- **Legacy `.claude/{project}-session.json` on disk.** Ignored by the primitive — `write-session` writes only the session files above. The bootstrap migration (`session-file-consolidate`) translates and deletes legacy files on the next `/ductus` run; adopters who skip the bootstrap will see a stale legacy file alongside the live `.ductus/session.toml` until they delete it by hand.
- **Repo-root resolution.** Same conventions as every other primitive — the runtime resolves the repo root from the invocation context; the primitive does not accept an alternate root argument.
- **Concurrent invocations.** Two agent processes in one working tree are a real model (062): an identified process writes its own target, so one process's target write no longer moves another's, and on the `.ductus/` layout every write holds the `.ductus/sessions/.lock` advisory lock. The shared default records the most recent target change from any process.
- **Filesystem refuses the rename (cross-device, permission).** Surfaces as `PrimitiveError::Io`, halting the call. The prior host-write path had the same failure mode; nothing degraded.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Session file location: parameterize via `session-path` arg, or consolidate onto a single repo-root file?** **Consolidate.** Rationale: the parameterized-path approach (caller passes `{cli-config-dir}/{project}-session.json` to `write-session` and `dashboard`) bakes the host-/project-specific variability into both primitive contracts and every command source that uses them. The host has to remember the path; the wire contract grows a required arg; doc-comments and MCP tool descriptions have to be careful not to commit to specific values. The consolidation has none of that surface area — the path is a constant the runtime knows, the wire contract stays small, and the file is uniform across every adopter. The cost is a one-time migration (`framework/migrations/session-file-consolidate.md`) for adopters who had a pre-0.10.0 session file; the bootstrap handles it on the next `/ductus` run. Rejected: keeping the path host-specific (the original bug); a `[session]` section inside `.ductus/config.toml` (mixes committed config with per-user state in one file, complicates gitignore).
- **TOML or JSON?** **TOML.** Rationale: matches `.ductus/config.toml`'s on-disk format, so adopters see one config language at the project root rather than two. Kebab-case keys (`scenario-path`, `set-at`) align with `.ductus/config.toml`'s convention rather than the legacy camelCase (`scenarioPath`, `setAt`).
