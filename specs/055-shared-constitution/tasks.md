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

- [x] Add the schema to `framework/bootstrap/ductus.md` §Project Configuration, pointing at `specs/055-shared-constitution/data-model.md` as canonical rather than restating it
- [x] Validate each entry at configuration time: alias is a bare TOML key, `repo` is URL-shaped, `path` is non-empty (AC6)
- [x] Warn — never reject — on a `path` that does not resolve, matching `link.md:65`
- [x] `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md`

- **Done when**: a malformed entry is rejected with a clear message, a non-resolving `path` only warns, and `scripts/audit/run-all.sh` Family 21 reports the two bootstrap files byte-identical.

## 3. Extend `/ductus:target` to load registered constitutions

- [x] Edit `framework/commands/target.md` step 4 to invoke `resolve-constitutions` and load every `loaded` document alongside `.ductus/constitution.md`
- [x] Report the sources loaded, and any in `skipped` with its reason (AC4, AC5)
- [x] Update the command's Scope Boundaries to cover reading the resolved documents
- [x] Run `cargo test --release --locked --test parity`; if `implement-basic` fails, re-bless and confirm the diff is shas only (`AGENTS.md:116`)

- **Done when**: a session targeted in a project with a registered constitution has that document's rules in effect for subsequent commands with no per-command opt-in (AC2), multiple entries all load (AC11), and the parity suite is green.

## 4. Maintain the layout-derived managed block

- [x] Add a `merge-managed-block` step to `framework/bootstrap/ductus.md` §Per-Agent Scaffolding writing the block into each agent's derived native rules file
- [x] Emit `@import {path}` for `claude-style`; emit a `See [alias]({path})` link for the `antigravity` and `opencode` layouts, which read `AGENTS.md` and have no import directive
- [x] Add the marker to `framework/templates/project/claude-md.md` and `framework/templates/project/agents.md`
- [x] `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md`

- **Done when**: re-running `/ductus` updates only the managed region, the rest of the adopter's file is byte-identical, and `.ductus/constitution.md` is unmodified (AC9).

## 5. Report unresolved sources as unexamined in the findings commands

- [x] Edit `framework/commands/review.md` so a run with a `skipped` source records it as unexamined and never folds it into a clean count
- [x] Edit `framework/commands/analyze.md` the same way
- [x] Ensure the report text names the source and the reason (AC5)

- **Done when**: a `/ductus:review` run in a project whose registered checkout is missing says so in the report rather than reporting clean (AC8).

## 6. State the precedence order once

- [x] Add the framework-as-floor statement (project > shared > framework where the framework is silent) to the constitution as §governance-precedence, together with the explicit fact that nothing enforces it
- [x] Point the config schema and `/ductus:target` step 4 at it rather than restating it

- **Done when**: the precedence order appears exactly once as a normative statement, carries its own unenforced caveat, and no second copy exists (AC10).

## 7. Verify the unset and additive paths against a fixture adopter tree

- [x] Build a `/tmp` adopter fixture with no `[constitutions]` entry; run the pipeline and confirm no source resolution, no new prompt, and no behavior change (AC1)
- [x] Repeat with a present-but-empty `[constitutions]` table and confirm it is indistinguishable from absent (AC1)
- [x] Confirm AC3 by inspection plus the bootstrap parity tests — the `framework/constitution.md → .ductus/constitution.md` manifest row (strategy `update`) and `[pinned]` handling are untouched by this change, and `ductus_basic_post_run_filesystem_state_matches_expectations` still passes. A live `/ductus` run needs network and the full installer, so it is not executed here
- [x] Confirm two fixtures with identical entries and checkout content load identical documents in identical order (AC7)
- [x] Exercise the **shipped** copies, not this repo's dogfooded ones (`AGENTS.md:108`)

- **Done when**: AC1, AC3, and AC7 are each demonstrated against a fixture tree shaped like an adopter's, with the commands run recorded.

## 8. Regenerate and run the full check suite

- [x] `scripts/gen-claude-commands.sh`
- [x] `scripts/audit/run-all.sh`
- [x] `npx markdownlint-cli2` over the changed markdown
- [x] `cargo test --release --locked`

- **Done when**: all four are green, and `.claude/commands/ductus/` matches the sources under `framework/commands/`.

## 9. Document the feature for adopters

- [x] Write `docs/shared-constitution.md` — the deep reference: registering, what travels, how it loads, resolution outcomes, and what the feature deliberately does not do
- [x] Add a brief `## Shared constitutions` section to `README.md` linking it, matching the shape of the existing Cross-service references section
- [x] Add a `[constitutions]` bullet to the README's `## Configuration` list and the new file to the `docs/` line under `## Repository layout`

- **Done when**: `docs/shared-constitution.md` exists and is linked from README's new section, its Configuration list, and its Repository layout; markdownlint is clean and no link is broken.

## 10. Add authoring guidelines and a setup walkthrough to the manual

- [x] Add a **What belongs in a shared constitution** section — the tier test against `AGENTS.md` below and the framework above, the reword test, and what cannot work (loosening a framework rule, rule-file-shaped content)
- [x] Add a **Setting one up** section — create the governance repo, agree a checkout layout, register in each project, run `/ductus`, verify
- [x] State the committed-`path` constraint: `.ductus/config.toml` is team-shared, so a relative sibling path is required and an absolute one resolves on one machine only

- **Done when**: the manual tells a reader both what to write in a shared constitution and how to put it into use across projects, including why the checkout path must be relative; markdownlint clean.

## 11. Reattach the doc comment task 5 orphaned in `write_review.rs`

- [x] Move `/// Render skipped passes as a list, or`*None.*`when empty.` back above `render_skipped` — task 5 inserted `render_unexamined_governance` between that line and the function it documents, so the new function's rustdoc summary describes a different function and `render_skipped` carries none
- [x] Re-read the doc comment on every other function the task-5 diff touched, per `AGENTS.md` §Workflow's prose-claim sweep entry
- [x] `cargo test --release --locked` and `cargo clippy --release --all-targets --locked -- -D warnings`

- **Done when**: `render_skipped` carries its own doc comment again, `render_unexamined_governance`'s summary line describes what it does, and the runtime suite plus clippy are green. Found by `/ductus:review` on 2026-09-11; routed here rather than to `specs/inbox.md` because it is a defect in what this in-progress spec built (constitution §brownfield-inbox).
