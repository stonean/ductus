# 055 — Shared Constitution Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Add the `resolve-constitutions` primitive and wire every site

- [x] Add `ResolveConstitutionsArgs` / `ResolveConstitutionsResult` to `runtime/src/schema/primitives.rs`
- [x] Write `runtime/src/primitives/resolve_constitutions.rs`, modelling the registry read on `load_services` (`resolve_references.rs:94`) and the checkout resolution on `classify` (`:107`)
- [x] Distinguish `not-checked-out` from `no-constitution-document` in the skip reason
- [x] Order `loaded` deterministically by alias (`BTreeMap`) — config order does not survive a TOML reformat, so it cannot satisfy AC7
- [x] Unit tests: absent table, empty table, resolved entry, missing checkout, checkout without `constitution.md`, two entries sharing a path, multiple entries
- [x] Wire every site in this same change: `schema/constitutions.rs` + `schema/mod.rs`, `primitives/mod.rs`, `schema/registry.rs` (`PRIMITIVE_REGISTRY`, which defines both `PRIMITIVE_NAMES` and `TOOL_NAMES`), `mcp/server.rs` (`#[tool]`), `interpreter/mod.rs` (`dispatch_primitive`), `main.rs` (`Command` + dispatch), `framework/runtime-tools.txt`

- **Done when**: `cargo test --release --locked` passes, `ductus resolve-constitutions` runs from the CLI, and `lint-tool-coverage` reports the new name covered.

## 2. Document and validate the `[constitutions.*]` schema in `/ductus`

- [ ] Add the schema to `framework/bootstrap/ductus.md` §Project Configuration, pointing at `specs/055-shared-constitution/data-model.md` as canonical rather than restating it
- [ ] Validate each entry at configuration time: alias is a bare TOML key, `repo` is URL-shaped, `path` is non-empty (AC6)
- [ ] Warn — never reject — on a `path` that does not resolve, matching `link.md:65`
- [ ] `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md`

- **Done when**: a malformed entry is rejected with a clear message, a non-resolving `path` only warns, and `scripts/audit/run-all.sh` Family 21 reports the two bootstrap files byte-identical.

## 3. Extend `/ductus:target` to load registered constitutions

- [ ] Edit `framework/commands/target.md` step 4 to invoke `resolve-constitutions` and load every `loaded` document alongside `.ductus/constitution.md`
- [ ] Report the sources loaded, and any in `skipped` with its reason (AC4, AC5)
- [ ] Update the command's Scope Boundaries to cover reading the resolved documents
- [ ] Run `cargo test --release --locked --test parity`; if `implement-basic` fails, re-bless and confirm the diff is shas only (`AGENTS.md:116`)

- **Done when**: a session targeted in a project with a registered constitution has that document's rules in effect for subsequent commands with no per-command opt-in (AC2), multiple entries all load (AC11), and the parity suite is green.

## 4. Maintain the layout-derived managed block

- [ ] Add a `merge-managed-block` step to `framework/bootstrap/ductus.md` §Per-Agent Scaffolding writing the block into each agent's derived native rules file
- [ ] Emit `@import {path}` for `claude-style`; emit a `See [alias]({path})` link for the `antigravity` and `opencode` layouts, which read `AGENTS.md` and have no import directive
- [ ] Add the marker to `framework/templates/project/claude-md.md` and `framework/templates/project/agents.md`
- [ ] `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md`

- **Done when**: re-running `/ductus` updates only the managed region, the rest of the adopter's file is byte-identical, and `.ductus/constitution.md` is unmodified (AC9).

## 5. Report unresolved sources as unexamined in the findings commands

- [ ] Edit `framework/commands/review.md` so a run with a `skipped` source records it as unexamined and never folds it into a clean count
- [ ] Edit `framework/commands/analyze.md` the same way
- [ ] Ensure the report text names the source and the reason (AC5)

- **Done when**: a `/ductus:review` run in a project whose registered checkout is missing says so in the report rather than reporting clean (AC8).

## 6. State the precedence order once

- [ ] Add the framework-as-floor statement (project > shared > framework where the framework is silent) to the config-schema documentation, together with the explicit fact that nothing enforces it
- [ ] Add the corresponding constitution line and its pointer

- **Done when**: the precedence order appears exactly once as a normative statement, carries its own unenforced caveat, and no second copy exists (AC10).

## 7. Verify the unset and additive paths against a fixture adopter tree

- [ ] Build a `/tmp` adopter fixture with no `[constitutions]` entry; run the pipeline and confirm no source resolution, no new prompt, and no behavior change (AC1)
- [ ] Repeat with a present-but-empty `[constitutions]` table and confirm it is indistinguishable from absent (AC1)
- [ ] With an entry registered and unpinned, run `/ductus` and confirm `.ductus/constitution.md` still receives its manifest update (AC3)
- [ ] Confirm two fixtures with identical entries and checkout content load identical documents in identical order (AC7)
- [ ] Exercise the **shipped** copies, not this repo's dogfooded ones (`AGENTS.md:108`)

- **Done when**: AC1, AC3, and AC7 are each demonstrated against a fixture tree shaped like an adopter's, with the commands run recorded.

## 8. Regenerate and run the full check suite

- [ ] `scripts/gen-claude-commands.sh`
- [ ] `scripts/audit/run-all.sh`
- [ ] `npx markdownlint-cli2` over the changed markdown
- [ ] `cargo test --release --locked`

- **Done when**: all four are green, and `.claude/commands/ductus/` matches the sources under `framework/commands/`.
