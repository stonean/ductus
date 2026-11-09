# 080 — Deterministic runtime efficiency Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Fixture corpus and the result-size script

- [x] Write `runtime/tests/common/corpus.rs`: a git-initialized project of N specs, scaling the spec count and each spec's criteria, tasks and scenarios
- [x] Write `runtime/tests/result_sizes.rs` (`#[ignore]`) and `scripts/measure-result-sizes.sh`. Run every `PRIMITIVE_REGISTRY` name on corpora of two sizes and record the characters of each result's text, or a *not measured* row with its reason
- [x] Record each tool's loaded schema (description plus input schema, in characters, from `ductus mcp` `tools/list` over stdio) and the total
- [x] Create `research.md` holding both tables, each with its method, units, population and date, plus the grows-set and the handoff's three method traps

- **Done when**: `research.md` names every registry primitive as measured or not measured with its reason, names the primitives that grow with the corpus, and records each tool's loaded schema size and the total (AC14, AC23).

## 2. One view model, two renderings

- [x] Move the view's rendering from `runtime/src/primitives/dashboard.rs` into `runtime/src/primitives/pipeline_view.rs`, built on one model
- [x] Render markdown and fixed-width text as `data-model.md` defines
- [x] Put the summary one item per line, with one line per unresolved scenario question, then the `Next actions:` block, then the single closing `/{project}:target` prompt
- [x] Test that one model rendered both ways carries the same facts

- **Done when**: unit tests show three things. The text rendering aligns its columns with two-space gaps and carries no `|` row syntax or `**`. Both renderings put each summary item on its own line and list the non-done specs by next action, with the `/{project}:target` hint once at the end. Both carry the same facts from one model (AC6's first half, AC17, AC18, AC19).

## 3. The view's bound

- [x] Collapse the `done (spec is complete)` rows to a count line past `read-size-bytes`, then cut the rows past the bound and end with a line counting the specs not shown
- [x] Test both renderings on a generated corpus past the bound

- **Done when**: the test passes for both renderings (AC24).

## 4. `render-status`, the status command, and `dashboard`'s dropped field

Blocked on tasks 2 and 3: the primitive returns their view.

- [x] Register `render-status` at the six sites `docs/developing-ductus.md` lists, then run `scripts/gen-configure-mcp.sh`
- [x] Rewrite `framework/commands/status.md`: step 1 invokes `render-status`, step 2 emits `rendered-markdown`, and step 3 is removed. Sweep `framework/reference/status.md` with it
- [x] Drop `rendered-markdown` from `dashboard`, and sweep the name across the plan's list and by meaning
- [x] Re-bless `runtime/tests/golden/status-basic.jsonl`, reading the diff

- **Done when**: `ductus exec status` and the MCP tool both return the view without `dashboard`'s structured payload, `framework/commands/status.md` composes neither the list nor the prompt, and `rendered-markdown` survives only on `render-status` and in records (AC6, AC13).

## 5. Bound the results that grow with the corpus

Blocked on task 1: its grows-set is this task's population.

The population is what `research.md`'s grows-set lists as not bounded yet. The first measurement found 11: `read-spec`, `read-tasks`, `derive-boundary`, `lint-markdown`, `prune-tasks`, `dashboard`, `check-review-gate`, `check-artifacts`, `check-corpus-links`, `check-unfolded-specs` and `rewrite-spec-links`. Measuring each argument mode that widens a read added `prune-plan`'s `all` walk. `render-status` was added after that measurement and is held to the read size by the view's own bound (task 3). The other readers a result's new shape reaches, besides the commands, were mapped at `57a63c35`, before any was changed:

- Rust callers of `run`, which need an unbounded path such as a `compute` function, the pattern `compute-review-scope` uses: `read_tasks::run` from `runtime/src/interpreter/payload.rs`, `prune_tasks.rs` and `mark_task.rs`, and `lint_markdown::run` from `check_review_gate.rs`.
- The exec walker unions `derive-boundary`'s `boundary` into the writeCode validator (`runtime/src/interpreter/mod.rs`, the `name == "derive-boundary"` arm), so a boundary moved to a file has to reach it.
- `scripts/audit/lib.sh` and `scripts/audit/review-freshness.sh` enumerate the corpus from `dashboard`'s JSON, and `scripts/audit/broken-relative-links.sh` and `scripts/audit/manifest-destination-links.sh` parse `check-corpus-links`'. Both pre-commit hooks call `check-corpus-links` for its exit status only, which the CLI now gates on the counts.
- Each primitive's description in `runtime/src/schema/descriptions.rs` states its file form, as `diff-cross-spec`'s does, since the binary is the source of a primitive's rules (§runtime-host-integration).

- [x] Return each grows-set primitive through `bounded_lists::place`
- [x] State each member's file form in its description, the source a host reads a moved list by, and sweep the earlier per-step branches (owner, 2026-11-08); give each script that parses a moved list a lists-file branch
- [x] Add each member's row to `data-model.md`'s bounded-results table
- [x] Test each member: drive a generated corpus past the bound and read the whole list back from `lists-file`

- **Done when**: every primitive `research.md` names as growing is bounded and has its test, `read-tasks` among them (AC7).

## 6. The `ductus hook status` surface

Blocked on task 2: it shows the text rendering.

- [x] Add `runtime/src/hook.rs` and the `hook status` subcommand, and list `hook` in `NON_PRIMITIVE_SUBCOMMANDS` with its reason
- [x] Test the block output and its `suppressOriginalPrompt`, ignored command arguments, the `CLAUDE_PROJECT_DIR` root, and exit 1 with the error on stderr and nothing on stdout when the view cannot be rendered

- **Done when**: the tests pass (AC16; AC17 and AC25 on the runtime side).

## 7. `merge-status-hook`

- [x] Register it at the six sites, then run `scripts/gen-configure-mcp.sh`
- [x] Gate on `claude --version` (2.1.101 or later); recognize ductus's entry by its command; preserve every other key and entry; write atomically
- [x] Confirm the probe reads the version from inside an MCP server Claude Code spawned. If it cannot, switch to the plan's fallback, in which the host runs the probe and passes its output, and update the plan's decision
- [x] Test `created`, `updated`, `unchanged`, and `skipped` for an old and an unreadable version. A second run leaves exactly one entry, another `UserPromptExpansion` entry is untouched, and a skip leaves the merged permissions intact

- **Done when**: the tests pass (AC2, AC20).

## 8. Install the hook, and verify it in Claude Code

`/ductus:configure` has installed the hook in this checkout, and two typed `/ductus:status` runs showed the view with no model turn, in the installing session. A session started in `runtime/` showed no view on either path, since Claude Code sets `CLAUDE_PROJECT_DIR` to the session's start directory. Sessions are expected to start at the project root (owner, 2026-11-09), which the spec, the plan and `docs/runtime.md` now say, with `runtime/src/hook.rs`'s comment left to task 23. With the pointer moved aside, the hook exited 127 without blocking and Claude Code showed `UserPromptExpansion hook error`. In the installing session the still-running MCP server then rendered the view. In a session started with the pointer moved aside, the model reported the missing server, the missing-runtime case of AC25's second half. `research.md` records every run.

- [x] Add the `merge-status-hook` step to `framework/bootstrap/configure/claude.md` after `merge-permissions`
- [x] Add it to `framework/bootstrap/ductus.md` for each Claude agent once the runtime is acquired. Add its CLI rule to Claude's seed and sweep the seed's subcommand count
- [x] Remove the local prototype, `.claude/hooks/status-hook.sh` and its `settings.local.json` entry. Then run `scripts/gen-claude-commands.sh`, since the slash command loads the installed copy, and run `/ductus:configure` in this checkout
- [x] Type `/ductus:status` twice and confirm the view shows with no model turn and no "Original prompt:" line. Then type it from a session started in a subdirectory, and with the pointer moved aside. Confirm each fails without blocking (the shell exits 127 for the missing pointer; only exit 2 blocks) and falls back to the model path, and record the status Claude Code reports and whether it shows a notice

- **Done when**: `research.md` records each run with its Claude Code version, timing and date (AC1, AC16, AC25).

## 9. Step-tool derivation and `alwaysLoad`

- [x] Write the derivation in `runtime/src/primitives/tool_loading.rs`: served registry names as whole tokens in the parsed `## Instructions` steps' prose
- [x] Flag the tools of the installed `target.md` that `Host::command_file_candidates` finds with `_meta["anthropic/alwaysLoad"]`, and say on stderr when there is no copy
- [x] Test in `runtime/tests/mcp.rs`: the flagged set equals the derivation over `framework/commands/target.md`, and each flagged description is at most 4,096 characters
- [x] Start a Claude Code session in this checkout and confirm target's tools are reached without a `ToolSearch`

- **Done when**: the test passes and `research.md` records the session check (AC21).

## 10. The batched `ToolSearch` line

Blocked on task 9: the line drops the always-loaded set.

- [x] Add `tool-search-line` to `ManifestEntry` and the insertion to `apply-manifest`
- [x] Set the field on Claude's command rows in `framework/bootstrap/ductus.md` only
- [x] Have `scripts/gen-claude-commands.sh` build the Claude copies by driving `apply-manifest`, keeping `--check`'s behaviour
- [x] Move `.githooks/pre-commit`'s runtime resolution above its generator calls
- [x] Test that a flagged copy parses to its source's `Procedure`, and that an unflagged entry's output is unchanged

- **Done when**: the tests pass, each `.claude/commands/ductus/` copy whose remaining step tools are non-empty carries the line, and `scripts/gen-claude-commands.sh --check` passes (AC29).

## 11. An adopter bootstrap across the change

Tasks 8 and 10 have landed and reached `main` (a62d2328, restored in 9bf198f5), so the second subtask's staging-copy check runs against the pushed tree. A version-bumped pin with no tag halts `--ref=main`, so the real `/ductus` run names this checkout's release build in `[runtime] path`, which `/ductus`'s acquisition Branch 1 uses with a warning.

- [x] Bootstrap a scratch adopter, with Claude and Auggie selected, from a release before this spec
- [x] Before pushing: drive `apply-manifest` and `enforce-manifest` from a staging copy of `framework/`, per `docs/developing-ductus.md` *Exercising an installer change without a push*. Confirm that Claude's copies carry the line and Auggie's are byte-identical to before
- [x] After pushing: have a subagent run `install.sh` on the scratch adopter, per the same section, with `[runtime] path` naming this checkout's release build. Confirm the hook lands in `.claude/settings.local.json` with no manual step and Auggie's settings are unchanged

- **Done when**: `research.md` records both runs and each check (AC10; AC26, Claude Code's half).

## 12. Pi's registered command

Blocked on task 4: the handler reads `render-status`.

- [x] Register `{project}-status` in `framework/bootstrap/pi/ductus-bridge.ts`, with `registerCommand` and `sendMessage` added to its `ExtensionAPI` slice
- [x] Add the cases to `scripts/tests/pi-bridge-harness.mjs`: the view shown with no turn, and a failure shown the same way
- [x] Run a real `pi` session, and record in `research.md` whether the view reached the agent's context, whether Pi can add context without a turn, whether a displayed message renders markdown, and whether the command or the prompt template ran — the context check waived: no pi account to run a model turn with (owner, 2026-11-09)
- [x] If markdown does not render, add `render-status`'s `format`. If the template ran, stop `/ductus` installing Pi's status template, and run `classify-spec-edit` on 064's edit

- **Done when**: the harness passes, a real `pi` run shows the view with no turn, and `research.md` records the four outcomes (AC9, AC27, AC26's Pi half).

## 13. Pi's compact form

Blocked on task 12.

- [x] When `research.md` records that the view did not reach the context and Pi can add context without a turn, add the target line and the non-done specs grouped by status to the agent's context, and a harness test for it

- **Done when**: either `research.md` records that the condition does not hold, or the bridge adds the compact form and its harness test passes (AC28).

## 14. OpenCode's scenario

- [x] Run OpenCode on a version whose TUI plugin's slash command runs local code, and try `/{project}:status` through it
- [x] Record the scenario through `/ductus:amend`, with the outcome and the deferral it settles under Resolved Questions, with its trigger

- **Done when**: the scenario exists under `scenarios/`, is settled by the run, and has no open question (AC11's half; AC10 for OpenCode).

## 15. Antigravity's scenario

- [x] Run `agy` 1.4.11 or later with a `PreInvocation` hook, and record whether it sees the raw command name — waived: the installed `agy` is 1.0.9, and the scenario is recorded without a run (owner, 2026-11-09)
- [x] Record the scenario through `/ductus:amend`, as in task 14

- **Done when**: the scenario exists under `scenarios/`, is settled by the run, and has no open question (AC11's half; AC10 for Antigravity).

## 16. The `claude -p` harness

- [x] Write `scripts/measure-command-cost.py`, which builds a fixture project and records four things per run: time to first visible output, time to result, output tokens, and the largest tool result in characters
- [x] Run it once, and record whether `claude -p` executes a custom slash command and how a local `UserPromptExpansion` hook treats a `-p` prompt

- **Done when**: no workflow references the script, and `research.md` records its first run's two findings (AC15).

## 17. Every command's cost

Blocked on tasks 4, 8, 10 and 16: the round measures the commands as changed. Those have landed, so it is unblocked. Run `python3 scripts/measure-command-cost.py --specs 40 --raw <dir> --out <file> "/fixture:<name> <args>" …` (its docstring states every figure and flag; `research.md` *The `claude -p` harness's first run* is the method to repeat), once per command, with the hook absent so status's figure is the model path's. Its fixture allows only `mcp__ductus__*`, so a command whose steps need `Read`, `Bash` or `Edit` meets permission denials under `-p`: decide per command before the round (owner), and say in the record which ran to completion. Each run is a model call on the owner's account; the round is 17 commands (`audit.md` excluded, as `/ductus` installs no copy of it).

- [x] Run the harness over every command in `framework/commands/`

- **Done when**: `research.md` records each command's four figures with method, units, population and date (AC8).

## 18. Each command's review

Blocked on task 17.

- [x] Name in `research.md`, per command, the reduction that applies or why none does
- [ ] Record each reduction as a scenario through `/ductus:amend`

- **Done when**: every command has a reduction or a reason, and each reduction has its scenario.

## 19. Revisit always-loading every tool

Blocked on task 1's schema sizes.

- [ ] Put the measured figures to the owner, and record the decision in `spec.md`'s Resolved Questions

- **Done when**: the spec's Resolved Questions record the owner's decision beside the figures.

## 20. Back-links on 022, 063 and 064

Once this task is ticked the spec owes `/ductus:review`, `/ductus:analyze` and the completion gate, none of which a task schedules.

The 022 edit also corrects task 4's sweep residue in 022's `config-readers-name-unknown-keys` scenario, found at task 5. Its Behavior says `dashboard`'s rendering adds the unknown-keys callout, and an Edge Case says `dashboard` names a key in that callout, but since task 4 the callout is in `render-status`'s view, and `dashboard` no longer loads `[host]` or `[services]`. `runtime/tests/config_unknown_keys.rs` was corrected at `ece1ed5a`. A scenario edit stales 022's review, so `classify-spec-edit` decides it with the back-link.

- [x] 022: link back to 080 from the `dashboard-primitive` scenario or from a blockquote signpost in its `spec.md`, covering the `hook` surface, `render-status`, `merge-status-hook`, the `alwaysLoad` `_meta` and the bounded results. A link in `data-model.md` discharges nothing, since the gate reads only `spec.md` and `scenarios/*.md`
- [x] 063: add a signpost that `read-size-bytes` now bounds primitive results and the view
- [x] 064: add a signpost for the bridge's registered command, and run `classify-spec-edit` on the edit

- **Done when**: `check-review-gate` reports all three `cross-spec-impact:` entries discharged.

## 21. `merge-status-hook`'s created test on Windows

- [x] Make `no_file_is_created_holding_the_entry` in `runtime/src/primitives/merge_status_hook.rs` expect each platform's own literal, `ductus.exe` on Windows, rather than compare against `hook_command()`. The runtime workflow's Windows build + test failed it at 3e7b3828 (run 42549616407): the primitive appends `EXE_SUFFIX`, as the plan and `data-model.md` specify, and the test pinned the Unix string
- [x] Say in `MERGE_STATUS_HOOK`'s description (`runtime/src/schema/descriptions.rs`) that the command names `ductus.exe` on Windows

- **Done when**: the runtime workflow's Windows build + test job passes at the commit that lands the fix, since a local pass proves only the Unix branch.

## 22. Disposition out-of-spec finding: `.mcp.json`'s relative command needs a session started at the project root

- [x] `framework/bootstrap/ductus.md` — §MCP registration writes Claude's `.mcp.json` with the command `.ductus/bin/ductus`, relative to the root, so a session started below the project root has no `ductus` server (task 8's `runtime/` run) — waived: sessions are always expected to start at the project root (owner, 2026-11-09; spec Resolved Questions)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 23. `hook.rs`'s `CLAUDE_PROJECT_DIR` comment

- [x] Correct `PROJECT_DIR_VAR`'s doc comment in `runtime/src/hook.rs`: Claude Code sets `CLAUDE_PROJECT_DIR` to the directory the session started in, and sessions start at the project root, so the claim that a session started below the root still shows the root's view goes (task 8's `runtime/` run; spec Resolved Questions)

- **Done when**: the comment states the observed behavior, landed with the next runtime change rather than on its own.

## 24. Re-anchor `scope-results-bounded.py`'s M6 and M9, which task 5 moved

- [x] Point M6 at `restore_lists`' file-form check in `runtime/src/interpreter/payload.rs`, with the renamed killing test `a_file_form_result_gets_its_lists_back_from_its_list_file`, and M7 and M8 at the renamed `an_unreadable_list_file_is_an_error_not_an_empty_list`. Task 5 (ece1ed5a) replaced the walker's scope-only read with `restore_lists`: `run.py --check` reports M6's and M9's anchors invalid, and M7's and M8's filters name no test, which `--check` does not see
- [x] Point M9 at task 25's restored check, after task 25

- **Done when**: `run.py --check` passes on every list in `scripts/mutation/`, and M6 and M9 run killed.

## 25. A list file missing a counted list halts the walk again

- [x] Restore the walker's halt on a list file that lacks a list its result counts, which task 5 (ece1ed5a) dropped when `restore_lists` replaced `scope_paths`. Since then a file-form `compute-review-scope` result whose list file holds no `scope` reads as an empty scope, which 066's scenario `scope-results-have-a-bounded-size` rules out ("a counts-only result never reads as an empty scope"). Derive the required lists from the bounded result, minding that `diff-cross-spec` counts `cross-spec-paths` as `cross-spec-count`
- [x] Test it in `runtime/src/interpreter/payload.rs`: a file-form result whose file lacks a counted list is `ListsFileUnreadable`, naming the list

- **Done when**: the test passes and fails with the check removed, and task 24's M9 anchors on the restored check.

## 26. `gen-claude-commands.sh` halts on a runtime that predates `tool-search-line`

- [x] Refuse a runtime whose `apply-manifest --help` does not name `tool-search-line`. 81285165 was committed from a worktree whose release binary predated a62d2328: that `apply-manifest` ignored the unknown field, so the hook's regeneration dropped the line from 14 Claude copies and nothing halted (restored in 9bf198f5). A stale runtime must halt the generator, and so the commit, rather than rewrite the copies
- [x] Test it in `scripts/tests/`, with a stub runtime that lacks the field and one that has it, and add the test to the workflow that runs the generator's scripts

- **Done when**: the generator halts, naming the runtime, with a binary that lacks the field and passes with one that has it, and the test fails with the check removed.

## 27. Disposition out-of-spec finding: pi 1.0.0 and later ship an MCP client, which the Pi bridge's premise and AGENTS.md's Pi gotcha deny

- [ ] `AGENTS.md` — the Gotchas entry "Pi has no MCP client — its tool surface is the `.pi/extensions/ductus.ts` bridge, and never \"fix\" Pi by registering an MCP server for it", and the header of `framework/bootstrap/pi/ductus-bridge.ts` ("Pi has no built-in MCP client (by design)"), against pi 1.0.4's `pi mcp` command, `--no-mcp` flag and `docs/mcp.md`, which connect stdio servers configured per project in `.pi/mcp.json`; its CHANGELOG has MCP entries from 1.0.0 (2026-10-01) (task 12's run, 2026-11-09)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 28. Pi's `/{project}-status` view names commands Pi can run

- [ ] The view `render-status` returns spells its hints `/{project}:<name>` (each row's next action, the `Next actions:` block, the closing `/{project}:target` prompt). On Pi a command is `/{project}-<name>` (064's D1), and task 12's run showed `/scratch:status` reaching the model as plain text, so on Pi every hint names no command. Decide where the Pi spelling comes from before writing it (owner): the bridge rewriting the view's `/{project}:` to `/{project}-` before it shows it, or `render-status` taking the hint form

OpenCode (`/{project}/<name>`) and Antigravity (`/{project}-<name>`) have the same gap on their model paths, which their scenarios' Edge Cases record; decide with Pi's whether the fix covers them.

- **Done when**: the view Pi's registered command shows spells each hint as a command Pi runs, with a bridge harness case for it.

## 29. Disposition out-of-spec finding: a stale ductus.md of an agent that is not running aborts the run, and the notice says the running session is stale

- [ ] `framework/bootstrap/ductus.md` — §Self-update check marks a selected agent `stale` when its installed copy differs from upstream, "The running session is using instructions other than the resolved source's", and §Pre-flight abort stops the run on any stale agent. An upgrade started with `install.sh claude --ref=main` refreshes Claude's copy only, so with Auggie also selected the run found Claude `current` and Auggie `stale`, overwrote Auggie's, and aborted with "the running session is using the older instructions", though the running copy matched upstream; every multi-agent upgrade that way costs a second session (task 11's update run, 2026-11-09)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 30. An adopter's Claude copies get the ToolSearch line: `apply-manifest` finds `target.md` under the bootstrap's source root

- [x] `always_loaded_tools` in `runtime/src/primitives/apply_manifest.rs` reads `{source-root}/commands/target.md`, the generator's layout (`--source-root framework`, sources `commands/*.md`). `/ductus` passes `--source-root {framework-root}`, the extracted archive's root, with sources `framework/commands/*.md`, so on every adopter upgrade Claude's `apply-manifest` call halts before writing (`invalid 'entries': an entry sets tool-search-line, and the source root holds no commands/target.md`) and the 17 copies stay as they were (task 11's second update run, 2026-11-09). Decide the lookup before writing it (owner)
- [x] Sweep the claim with it: `APPLY_MANIFEST` in `runtime/src/schema/descriptions.rs`, the plan's *Batched (AC29)* decision, and `docs/developing-ductus.md` *Exercising an installer change without a push*, whose staging copy is the generator's layout, which is why task 11's staging check passed
- [x] Test `apply-manifest` in the bootstrap's layout, then re-run `/ductus --ref=main` on task 11's scratch adopter with the fixed build and confirm Claude's 14 copies carry the line

- **Done when**: a test drives `apply-manifest` with the bootstrap's source root and sources, and a real adopter `/ductus` run writes the line into Claude's copies (AC29).

## 31. Disposition out-of-spec finding: Auggie's surfaced MCP command names the store when `[runtime] path` names another binary

- [ ] `framework/bootstrap/ductus.md` — §MCP registration's Auggie row surfaces `auggie mcp add ductus --command ~/.ductus/bin/ductus --args "mcp"`, the store path, while Runtime acquisition Branch 1 points a project with `[runtime] path` at that binary instead ("The pointer resolves to this path rather than to the store"), so an Auggie session would serve the store's runtime, not the one the project chose (task 11's second update run, 2026-11-09)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 32. Scenario: OpenCode keeps the model path

- [x] Implement the behavior described in `scenarios/opencode-keeps-the-model-path.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 33. Scenario: Antigravity keeps the model path

- [x] Implement the behavior described in `scenarios/antigravity-keeps-the-model-path.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.
