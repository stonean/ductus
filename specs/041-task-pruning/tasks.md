# 041 — Spec Directory Pruning Tasks

Tasks derived from the [plan](plan.md). Complete in order. Phase A ships the
`prune-tasks` runtime primitive; Phase B adds the `/{project}:prune` command on
top and registers it. The primitive (Phase A) must build and pass its parity
test before the command (Phase B) wires to it.

## Phase A — `prune-tasks` runtime primitive

### 1. Schema types

- [x] Add `PruneTasksArgs`, `PruneTasksResult`, the per-section record, and the `PruneMode` / `Classification` enums to `runtime/src/schema/primitives.rs`, following the kebab-case serde + `clap::Args` pattern (`feature`, `reset`, `force`, `apply` flags; the result excludes the file body).
- [x] Add a `prune_tasks_round_trip` serde test alongside the existing round-trip tests.
- **Done when**: `cargo build` compiles the new types and the round-trip test passes.

### 2. Segmentation, classification, and rebuild

- [x] Create `runtime/src/primitives/prune_tasks.rs` with `run(&PruneTasksArgs, &Path) -> Result<PruneTasksResult>`: segment via `detect_tasks_structure` / `parse_atx_heading` / `iter_phase_ranges`, and classify each task section `Spent` / `Pending` / `NoCheckbox` using `checkbox::find_checkbox_line`.
- [x] Implement keep-pending rebuild: drop `Spent` sections, drop phase containers with no surviving task section, preserve the preamble verbatim, normalize seams to one blank line, and set `nothing-to-prune` when nothing is spent.
- [x] Implement reset rebuild: existing first `# …` heading + a `CANONICAL_EMPTY_TASKS_BODY` constant, with a test asserting the constant equals `framework/templates/spec/tasks.md` minus its H1.
- [x] Implement the `--reset` status gate (read `spec.md` status; `allowed` / `blocked-needs-force`; `force` override; domain-outcome, not error) and the `apply` write via `write_atomic`.
- [x] Add `TasksFileMissing` and `MalformedTasks` variants to `PrimitiveError` and `pub mod prune_tasks;` in `runtime/src/primitives/mod.rs`.
- [x] Add inline `#[cfg(test)]` tests + fixtures under `runtime/tests/fixtures/primitives/` covering flat and phased spent-section removal, pending / no-checkbox preservation, empty-phase drop, keep-pending no-op, reset target, reset gate (done vs non-done vs `force`), missing `tasks.md`, and malformed `tasks.md`.
- **Done when**: all `prune_tasks` unit tests pass and preview output carries no file body.

### 3. Wire the primitive through CLI, MCP, interpreter, parser

- [x] `runtime/src/main.rs`: import the args, add `PruneTasks(PruneTasksArgs)` to `Command`, add the dispatch arm.
- [x] `runtime/src/mcp/server.rs`: add `"prune-tasks"` to `TOOL_NAMES` and a `#[tool(name = "prune-tasks", …)]` method.
- [x] `runtime/src/interpreter/mod.rs`: add `"prune-tasks" => call!(PruneTasksArgs, prune_tasks)`.
- [x] `runtime/src/parser/mod.rs`: add `"prune-tasks"` to `PRIMITIVE_NAMES`.
- **Done when**: the CLI subcommand, the MCP tool, and a `ductus exec` procedure step all resolve the primitive.

### 4. Canonical manifest, generated config, release metadata

- [x] Add `prune-tasks` to `framework/runtime-tools.txt`.
- [x] Run `scripts/gen-configure-mcp.sh` and `scripts/gen-claude-commands.sh`; commit the regenerated configure allow-blocks.
- [x] Add a `runtime/CHANGELOG.md` `### Added` entry (new tool; list grows N→N+1) and bump `runtime/Cargo.toml` to the next minor version.
- **Done when**: the `runtime/tests/mcp.rs` parity test passes (`TOOL_NAMES` ↔ `runtime-tools.txt` ↔ served) and the generators report no drift.

### 5. Green gate

- [x] `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo fmt --check` are all clean.
- **Done when**: the runtime workspace is green.

## Phase B — `/{project}:prune` command

### 6. Author the command source

- [x] Create `framework/commands/prune.md` with `description:` frontmatter and `{project}:` / `{cli-config-dir}/` placeholders. Instructions: resolve the session target; call `prune-tasks` preview; render the compact summary; route confirmation through `gate-confirm`; call `prune-tasks` apply; surface `blocked-needs-force`, `tasks-file-missing` (→ `/{project}:plan`), and no-target (→ `/{project}:target`). Annotate each step with a primitive name, an `<!-- llm:* -->` marker, or `<!-- audit:ignore-promotion -->`, and reference §runtime-host-integration once.
- **Done when**: the command documents both the runtime and markdown-only paths and states the single-artifact (`tasks.md`-only) scope.

### 7. Parseability

- [x] Run `ductus parse --check framework/commands/prune.md`. If it parses cleanly, leave it; otherwise add `framework/commands/prune.md` to `runtime/legacy-prose-commands.txt`.
- **Done when**: `scripts/lint-procedure-parseability.sh` passes for `prune.md`.

### 8. Register and regenerate

- [x] Add the `'/{project}:prune' "$CMD_DIR/prune.md"` row to the appropriate group in `scripts/gen-help-tables.sh` (add a new `generated:commands-<group>` marker pair in `help.md` and a splice-loop entry only if prune warrants its own group).
- [x] Run the generators (or the pre-commit hook) to regenerate `framework/commands/help.md` and materialize `.claude/commands/ductus/prune.md`; commit both.
- **Done when**: `gen-help-tables.sh --dry-run` and `gen-claude-commands.sh --check` report in-sync.

### 9. Docs

- [x] Add a `/prune` row to the hand-maintained `## Commands` section of `README.md`.
- **Done when**: the README lists prune in the correct group.

### 10. Full audit gate

- [x] Run `scripts/audit/run-all.sh` (check-zero + all families) and resolve any findings.
- **Done when**: the audit reports zero findings and the CI-equivalent checks pass.

## Phase C — Framework consistency (tasks.md is ephemeral tracking)

Surfaced by the pre-`done` durability review: `tasks.md` must be treated as disposable tracking end to end, not durable information.

### 11. Make the shared `tasks.md` parsers ignore HTML comments

- [x] In `runtime/src/primitives/mod.rs`, teach the shared line-walkers to skip content inside `<!-- … -->` HTML comments (single- and multi-line) exactly as they skip fenced blocks: `iter_task_numbers_at_levels`, `iter_phase_ranges`, and `section_lines` (so `detect_tasks_structure` follows).
- [x] Apply the same comment-skipping in `read_tasks.rs`, `mark_task.rs`, `check_stuck.rs`, and `prune_tasks.rs::segment` so every tasks parser agrees.
- [x] Add a regression test proving a reset (template-state) `tasks.md` parses to zero tasks and `append-task` returns number 1.
- **Done when**: `ductus read-tasks` on a `--reset` file returns 0 tasks; `cargo test` / clippy / fmt clean.

### 12. Codify `tasks.md` as an ephemeral tracking artifact in the constitution

- [x] Add a canonical statement (in `framework/constitution.md`, §tasks-phase or §text-first-artifacts) classifying `tasks.md` as an ephemeral work-tracking artifact — a view of what is left to do, safe to prune — distinct from the durable spec / scenarios / rules, with `plan.md` / `data-model.md` as design records.
- [x] Update `framework/commands/prune.md` and this spec to cite that classification directly rather than by analogy to §bug-handling; reconcile the AGENTS.md artifact grouping so `tasks` is not read as a durable source of truth.
- **Done when**: the constitution names tasks.md's durability class explicitly; `resolve-anchor` and the framework audit are clean.

### 13. Relax `/{project}:analyze` scenario-consistency for pruned tasks

- [x] Update `framework/commands/analyze.md` so the scenario-consistency check does not require a scenario's implementing task to persist in `tasks.md` after the scenario is implemented (a `done` spec with pruned scenario tasks is not a drift finding); regenerate the materialized command.
- **Done when**: analyze reports no false scenario-consistency finding for a `done` spec whose scenario tasks were pruned; the framework audit is clean.

## Phase D — Follow-on scenarios

### 14. A plan records the design as it stands

- [x] Implement the rule half of the behavior described in `scenarios/plan-records-the-design-as-it-stands.md` — the constitution and both templates; its advisory half is task 15's analyze subtask
- [x] Constitution: §plan-phase states the rule — the design record is the template's `##` sections and the only place a plan's claims live, and where every other kind of content goes; §tasks-phase's closing sentence drops "none of which `/{project}:prune` touches" for `plan.md` and names handoff notes on the pending task; §implement-phase's review-body rule gains the triage clause
- [x] `framework/templates/spec/plan.md`: Trade-offs always; Data Model, Open Questions Resolved and Cross-spec impact optional, each marked to be omitted when it does not apply; a guidance comment on what stays out of a plan and where it goes
- [x] `framework/templates/spec/tasks.md`: the working-notes guidance and both placement rules; update `prune-tasks`' `CANONICAL_EMPTY_TASKS_BODY` in the same change so its drift test passes
- [x] Prose-claim sweep for "prune touches only `tasks.md`" and "single-artifact" across `framework/`, `docs/`, `README.md` and the spec corpus, by meaning, not by token

The advisory half of this scenario — `/{project}:analyze` firing every section outside the record — needs `prune-plan`, so it is task 15's last subtask, not this task's.

- **Done when**: the constitution and both templates state the rule, the reset-constant drift test passes against the new tasks template, and every live claim that prune touches only `tasks.md` is either corrected or owned by the task that changes the behavior it describes — a claim about prune's current command stays true until task 15 changes it, so rewriting it here would make it false in the interim.

### 15. Prune reduces every prunable artifact in the spec directory

- [x] Implement the behavior described in `scenarios/prune-reduces-the-spec-directory.md`
- [x] `data-model.md`: `prune-plan`'s segmentation, classification, finding key, schema, the `stale-sections` outcome and `reopen-required`
- [x] Promote `read_blob_at_head` from `check_stuck.rs` into `mod.rs` on `ProjectRepository`; `check_stuck.rs` calls it
- [x] `primitives/prune_plan.rs`: fence- and comment-aware `##` segmentation; the design-record constant pinned to the plan template by a drift test; `plan_section_finding`; `decided` from `read_decisions` + `same_key`; digest-guarded removal with seam normalization; `reopen-required` from the HEAD diff; a missing plan reported as `missing`, not an error
- [x] Wire `prune-plan` through the seven registration sites, `framework/runtime-tools.txt`, then `scripts/gen-configure-mcp.sh` and `scripts/gen-claude-commands.sh`; `cargo test --test mcp` first
- [x] Tests that fail when their behavior is removed (`QUAL-TEST-001`), each proven red by mutation: a `##` in a fence or comment is not a section; headings compare case-insensitively; a digest mismatch refuses the whole apply; a stored discard sets `decided`; each reopen trigger fires and a removal-only diff does not
- [x] `framework/commands/prune.md`: spec-directory scope, stored discards, the reopen named at confirmation and asked about when unannounced, declines run-only, a markdown-only reference for plan segmentation
- [x] `framework/commands/analyze.md`: a step invoking `prune-plan` on `planned`+ specs that records every section outside the record, decided or not, as a `plan-record` advisory (`scenarios/plan-records-the-design-as-it-stands.md`); re-bless `analyze-basic` filtered to that golden and read the diff line by line
- [x] Body corrections: *Scope confirmation* resolution, Behavior's "scope is `tasks.md` only", AC1, *Framework consistency* gains `plan.md`'s classification, title broadened; new criteria added unlabelled and labelled by `label-criteria`
- [x] Docs: `docs/slash-commands.md`'s `/prune` entry, `docs/analyze.md`'s advisory, `README.md`'s `/prune` line, prune's description in `scripts/gen-help-tables.sh`

- **Done when**: `/{project}:prune` on a spec with a journal section moves its durable pieces home and removes it, a stored discard keeps a section from being proposed, analyze reports every section outside the record, and the full local gate is green.

### 16. Prune --all walks every spec

- [x] Implement the behavior described in `scenarios/prune-all-walks-every-spec.md`
- [x] `all` on `prune-tasks` and `prune-plan`: exactly one of `feature` and `all`; `force` with `all` and `apply` with `all` on `prune-plan` refused through `InvalidArgument`; walk in `feature_dir_cmp` order; `features`, `skipped` and `examined` in the result; `data-model.md` updated
- [x] Tests proven red by mutation: the walk order, a skipped feature named with its reason, the refusals, and a per-spec reset gate
- [x] `framework/commands/prune.md`: the `--all` flow — one corpus preview, one confirmation for every `tasks.md` reduction, plans one spec at a time
- [x] `docs/slash-commands.md`: sharpen the flag-versus-command rule so a batch flag repeating a one-spec operation (`/{project}:analyze --all`, `/{project}:prune --all`) is distinct from a two-spec operation
- [x] Discharge the declared `cross-spec-impact` on `052-spec-supersession-and-consolidation`: its body states the same rule ("`amend`, `prune`, `clarify`, `plan`, and `implement` each write one"), which `--all` falsifies for `prune` as written. Reopen 052 with `set-status` in its own commit, sharpen the sentence to the batch-versus-two-spec distinction with a `> **Signpost:**` blockquote linking back to 041, re-run `/{project}:analyze` on 052 (a `spec.md` edit, so no re-review), and return it to `done`

- **Done when**: `/{project}:prune --all` previews the corpus, applies every `tasks.md` reduction under one confirmation, walks plans spec by spec, and the refusals hold.

### 17. Release

Released: `ductus-v0.56.0` at `62cc9490`, with `framework-checks`, `runtime`, `generators` and `runtime-release` all concluding `success` (`verify-published` and `verify-installer` included) and the GitHub release carrying its 13 assets.

- [x] The full local gate, as `AGENTS.md` §Workflow lists it
- [x] `runtime/CHANGELOG.md` `### Added` and a minor bump across `version`, `runtime/Cargo.toml` and the changelog heading, in one commit
- [x] `/{project}:review` and `/{project}:analyze` on this spec, then the completion gate to `done`
- [x] Tag `ductus-v<version>` in the same sitting and read every workflow run for that sha

- **Done when**: 041 is `done`, the tag is pushed, and every workflow run for the release commit has concluded successfully.

### 18. Disposition out-of-spec finding: 062's concurrent-session test fails on Windows, racing on .ductus/sessions/.gitignore

- [x] `runtime/tests/concurrent_sessions.rs:201` — `concurrent_writers_each_keep_their_own_target` fails on windows-latest (runtime run 36653211031, at `52cf2f15`) with `I/O error on …\.ductus/sessions\.gitignore: Access is denied. (os error 5)`; it passes on ubuntu-latest and macos-latest. 062's runtime commits were first pushed with `52cf2f15`, so this is the test's first Windows run and it has never passed there — routed to `062-concurrent-session-targets` scenario `first-writers-create-the-ignore-file-once` (062 task 26): the ignore file was written before the session lock, so every first writer replaced it by rename, which Windows refuses

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task.

### 19. Correct 041's contracts and prose from 048's review

- [x] `prune_plan.rs`: an apply writes `plan.md` before `reopen_required` can fail (no repository, an unborn HEAD, an unreadable `tasks.md`), against the plan's "each error writes nothing" — compute the reopen from the in-memory text before the write, update `data-model.md`'s "computed after the write", and prove the test red
- [x] `prune_tasks.rs`: an `all` + `apply` walk rewrites a spec's `tasks.md` before `read_status` can fail — read the status first, and prove the test red
- [x] `prune_plan.rs` rebuilds the stored-decision key inline — share one builder with `write_analysis::finding_key`
- [x] Decide with the operator: `prune.md` and `docs/slash-commands.md` say the reopen is named before anything is written, while step 9 asks after the moves about a reopen from pre-run edits (computing `reopen-required` before the write may let it be asked first)
- [x] Decide with the operator: AC19 and the scenario say a diff "adds an unchecked checkbox", while the code compares counts against HEAD, so adding one box and checking another reopens nothing
- [x] `constitution.md` §tasks-phase says prune changes nothing the plan asserts, while the scenario's moves edit a design-record entry in place; it and `consolidate.md` leave out `--reset`
- [x] `prune.md` says `spec.md` is never touched, but step 9's `set-status` writes its frontmatter
- [x] Contributor knowledge: `prune.md`, the plan template, `docs/slash-commands.md` and the scenario name two destinations — point at §drift-prevention's *Shared knowledge stays in git*, which has three
- [x] The design-record heading list is restated in `prune.md`, `analyze.md`'s Plan record and step 16, and §plan-phase — point at the plan template and add a canonical-sources row; `analyze.md`'s Plan record contradicts itself on a customized template
- [x] `analyze.md` step 16 does not say `prune-plan` refuses a malformed `analysis.md` decisions list, which halts an exec walk; `prune.md` step 2's "note it and continue" on a missing `tasks.md` halts under exec, undocumented
- [x] `constitution.md`: §plan-phase's "detected, not remembered" overstates a `##`-only check; §implement-phase's triage clause reads as covering review MUST/SHOULD findings, which keep fix-or-waive; §plan-phase's "update them when decisions change" can read as recording reversals
- [x] The tasks template restates §tasks-phase's note-placement rule without citing it and drops the free-standing-heading case
- [x] `spec.md`: the Resolved Questions *Relationship to groom*, *Re-derivation contract* and *Runtime eligibility* state premises this reopen falsified; *Framework consistency*'s "Four alignments" sits under a `tasks.md`-only heading; AC17's grammar
- [x] `plan.md`: "which the template omits today" is now false; a narration of this reopen sits inside the design record; "a seventh registration set"; no Cross-spec impact section for 052. `data-model.md`: `tasks-file-missing` directs to `/plan` only when there is no plan either

- **Done when**: every item is fixed, or put to the operator and decided; `data-model.md`, the scenarios and the code agree; and `cargo test` passes.

### 20. Correct what the review beside 0.56.0's found

- [x] Exec /prune wrote nothing after a confirmed gate (QUAL-STUB-001): routed to 022 scenario a-confirmed-gate-authorizes-the-writes-after-it, task 128, which binds apply at a confirmed gate
- [x] AC19, the Behavior paragraph, prune.md's Scope Boundaries and the scenario: each reopen is named before the write that causes it, not before anything is written
- [x] spec.md's Runtime eligibility names both compiled-in divergences, the reset body and the design-record heading set
- [x] plan.md: keep-pending's status is omitted (a walk reads it), the step-renumbering narration, the Title and slug heading, Affected Files for write_analysis.rs and the hex move
- [x] Scenario resolutions: the reopen-cost resolution points at the preview resolution; plan-records says the template then omitted Trade-offs
- [x] Code and tests from the review: CRLF prune tests assert the write; the fenced-H1 reset refusal; the PruneTasksLine mapping test; doc comments on TasksFileMissing, split_blocks, PruneTasksLine, prune-plan's errors and analysis.md's readers

- **Done when**: every correction is on disk, the code changes' tests fail with their behavior reverted, and 041's review and analyze record this reopen

### 21. A plan edit does not reopen a done spec

- [x] Implement the behavior described in `scenarios/a-plan-edit-does-not-reopen-a-done-spec.md`
- [x] Under the new rule, correct the `done`-spec plan claims about the constitution's layout that `050-constitution`'s cut to one read left stale: `013` plan (the schema table's Allowed values column), `016` plan (§rules carries the three-tier table), `024` plan (Filename suffix names `scripts/lint-rule-filenames.sh`), `036` plan (one canonical-sources row per rule surface), `045` plan (Decision resolution defers to `/{project}:analyze`), `060` plan (the review window as §implement-phase's), `072` plan (the constitution's ceiling as its size when the check landed, and its fix message naming the owed cut) — each a plan edit that leaves its spec `done` (routed from 050 task 35)

Start with `/ductus:target 041-task-pruning/a-plan-edit-does-not-reopen-a-done-spec` and `/ductus:clarify`: the scenario's open questions block `done`, and the owner pre-authorized their `Recommended:` answers (2026-10-26). The §spec-lifecycle case must fit under the constitution's ceiling in `scripts/lint-governing-size.sh` (then the file's exact size), so the amendment removes as many bytes as it adds. The proof replays the 2026-10-16 `amend(NNN)` plan commits ending at `8191e695`.

- **Done when**: the scenario's described behavior is correctly implemented and tested, and the routed plan claims are corrected without reopening their specs
