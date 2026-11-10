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
- [x] Record each reduction as a scenario through `/ductus:amend`

- **Done when**: every command has a reduction or a reason, and each reduction has its scenario.

## 19. Revisit always-loading every tool

Blocked on task 1's schema sizes.

- [x] Put the measured figures to the owner, and record the decision in `spec.md`'s Resolved Questions

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

- [x] `AGENTS.md` — the Gotchas entry "Pi has no MCP client — its tool surface is the `.pi/extensions/ductus.ts` bridge, and never \"fix\" Pi by registering an MCP server for it", and the header of `framework/bootstrap/pi/ductus-bridge.ts` ("Pi has no built-in MCP client (by design)"), against pi 1.0.4's `pi mcp` command, `--no-mcp` flag and `docs/mcp.md`, which connect stdio servers configured per project in `.pi/mcp.json`; its CHANGELOG has MCP entries from 1.0.0 (2026-10-01) (task 12's run, 2026-11-09) — fixed: the claim is corrected in `AGENTS.md`, the constitution's host-integration paragraph, `ductus.md` and its `govern.md` copy, `configure/pi.md`, the bridge's header, `README.md`, and 064's `spec.md` and `plan.md`, and a real pi 1.5.0 run loads the edited bridge; moving Pi to its MCP client waived: the bridge also registers `/{project}-status`, which no MCP registration can, both at once would serve every tool twice, and no pi account is at hand to verify a turn (owner, 2026-11-09; 064's Resolved Questions)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 28. Pi's `/{project}-status` view names commands Pi can run

- [x] The view `render-status` returns spells its hints `/{project}:<name>` (each row's next action, the `Next actions:` block, the closing `/{project}:target` prompt). On Pi a command is `/{project}-<name>` (064's D1), and task 12's run showed `/scratch:status` reaching the model as plain text, so on Pi every hint names no command. Decide where the Pi spelling comes from before writing it (owner): the bridge rewriting the view's `/{project}:` to `/{project}-` before it shows it, or `render-status` taking the hint form

OpenCode (`/{project}/<name>`) and Antigravity (`/{project}-<name>`) have the same gap on their model paths, which their scenarios' Edge Cases record; decide with Pi's whether the fix covers them.

- **Done when**: the view Pi's registered command shows spells each hint as a command Pi runs, with a bridge harness case for it.

## 29. Disposition out-of-spec finding: a stale ductus.md of an agent that is not running aborts the run, and the notice says the running session is stale

- [x] `framework/bootstrap/ductus.md` — §Self-update check marks a selected agent `stale` when its installed copy differs from upstream, "The running session is using instructions other than the resolved source's", and §Pre-flight abort stops the run on any stale agent. An upgrade started with `install.sh claude --ref=main` refreshes Claude's copy only, so with Auggie also selected the run found Claude `current` and Auggie `stale`, overwrote Auggie's, and aborted with "the running session is using the older instructions", though the running copy matched upstream; every multi-agent upgrade that way costs a second session (task 11's update run, 2026-11-09) — deferred: 007's task 8, since 007's `ductus-self-update-precheck` scenario specifies the abort and its reason argues for the narrower one, and its verifying adopter run is 007's work (owner, 2026-11-09)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 30. An adopter's Claude copies get the ToolSearch line: `apply-manifest` finds `target.md` under the bootstrap's source root

- [x] `always_loaded_tools` in `runtime/src/primitives/apply_manifest.rs` reads `{source-root}/commands/target.md`, the generator's layout (`--source-root framework`, sources `commands/*.md`). `/ductus` passes `--source-root {framework-root}`, the extracted archive's root, with sources `framework/commands/*.md`, so on every adopter upgrade Claude's `apply-manifest` call halts before writing (`invalid 'entries': an entry sets tool-search-line, and the source root holds no commands/target.md`) and the 17 copies stay as they were (task 11's second update run, 2026-11-09). Decide the lookup before writing it (owner)
- [x] Sweep the claim with it: `APPLY_MANIFEST` in `runtime/src/schema/descriptions.rs`, the plan's *Batched (AC29)* decision, and `docs/developing-ductus.md` *Exercising an installer change without a push*, whose staging copy is the generator's layout, which is why task 11's staging check passed
- [x] Test `apply-manifest` in the bootstrap's layout, then re-run `/ductus --ref=main` on task 11's scratch adopter with the fixed build and confirm Claude's 14 copies carry the line

- **Done when**: a test drives `apply-manifest` with the bootstrap's source root and sources, and a real adopter `/ductus` run writes the line into Claude's copies (AC29).

## 31. Disposition out-of-spec finding: Auggie's surfaced MCP command names the store when `[runtime] path` names another binary

- [x] `framework/bootstrap/ductus.md` — §MCP registration's Auggie row surfaces `auggie mcp add ductus --command ~/.ductus/bin/ductus --args "mcp"`, the store path, while Runtime acquisition Branch 1 points a project with `[runtime] path` at that binary instead ("The pointer resolves to this path rather than to the store"), so an Auggie session would serve the store's runtime, not the one the project chose (task 11's second update run, 2026-11-09) — fixed: `ductus.md`'s Branch 1 and its `surface-instruction` paragraph, with `govern.md` copied, now say `[runtime] path` reaches only the agents whose registration names the pointer, while Auggie and Antigravity serve the store's runtime, since 048's Resolved Questions make the store the one correct entry in a per-machine config (owner, 2026-11-09)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 32. Scenario: OpenCode keeps the model path

- [x] Implement the behavior described in `scenarios/opencode-keeps-the-model-path.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 33. Scenario: Antigravity keeps the model path

- [x] Implement the behavior described in `scenarios/antigravity-keeps-the-model-path.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 34. Scenario: help answers with no model turn

Design it in `plan.md` before any code: the plan has none yet. `merge-status-hook` recognizes its entry by command and matches `{project}:status`, so help is a second entry or a widened matcher, and the plan chooses. On Pi the guide takes task 28's respelling in the bridge. The `hook` surface and `merge-status-hook` are 022's, so 022's task-20 signpost gains help. Verify as task 8 did, by typing it in Claude Code, and with `scripts/measure-command-cost.py --round --only help --hook --permissions adopter`. On Pi, verify with the bridge harness and a real `pi --mode rpc` run, as `658c0ffc`'s message records. A runtime change follows `docs/developing-ductus.md` *Landing a runtime change*.

- [x] Implement the behavior described in `scenarios/help-answers-with-no-model-turn.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 35. Scenario: configure types no permission entries

Design it in `plan.md` first. `merge-permissions` holds the canonical allow set as its ceiling (`CANONICAL_ALLOW_STATIC` and the served tools, `runtime/src/primitives/merge_permissions.rs`), while the deny and retired sets live only in `framework/bootstrap/configure/claude.md` steps 3 and 4, and step 5's `additionalDirectories` write moves into the primitive. `merge-permissions` is a 022 surface, so 022's signpost gains it. Verify with `--round --only configure --permissions adopter`: no entry typed and no denial.

- [x] Implement the behavior described in `scenarios/configure-types-no-permission-entries.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 36. Scenario: one search covers referenced sections

36 and 40 both change the generated `ToolSearch` line (`tool-search-line` in `runtime/src/primitives/apply_manifest.rs`, the derivation in `runtime/src/primitives/tool_loading.rs`), so design them together in `plan.md`'s *Batched (AC29)* decision and land them as one runtime change. Verify with `--round --only plan,status,log,specify,configure --permissions adopter`: one search per run.

- [x] Implement the behavior described in `scenarios/one-search-covers-referenced-sections.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 37. Scenario: reference reads follow the cited sections

Put the mechanism to the owner before code, with each option's cost: no primitive returns a section's text, since `resolve-anchor` only checks that a `§` anchor resolves. Verify with `--round --only analyze,clarify,groom,plan,fold,review --permissions adopter`: no whole-reference read.

- [x] Implement the behavior described in `scenarios/reference-reads-follow-the-cited-sections.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 38. Scenario: rules load without shell reads

Put the mechanism to the owner before code, with each option's cost: extend `discover-rule-files`, or a primitive returning each loaded rule's ID, tier, Statement and Verification, named by review's §Load rules and analyze's steps 11–12. Verify with `--round --only review,analyze --permissions adopter`: no shell read of a rule file.

- [x] Implement the behavior described in `scenarios/rules-load-without-shell-reads.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 39. Scenario: groom derives routing candidates

The smallest of the seven: `framework/commands/groom.md`'s routing step calls `derive-routing-candidates` before `routeInboxItem`, as `framework/commands/specify.md` step 1 does. Verify with `--round --only groom --permissions adopter`: no `grep` of spec bodies. That change landed with tasks 37 and 38. Its first round ran the no-candidate Edge Case, since no item can match the generated corpus's slugs. The owner chose to leave the body search to the model there, and to verify the main path with a seeded spec whose slug shares the item's words (2026-11-10). `plan.md`'s *groom derives its routing candidates* holds the design, and `research.md`'s *Routing candidates on the main path* holds both runs.

- [x] Implement the behavior described in `scenarios/groom-derives-routing-candidates.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 40. Scenario: the search goes out with the first call

Lands with task 36 (see its note). Verify that each run's first call goes out in the search's own message, read from the raw stream as `research.md`'s *The search step* does. Once 34–40 are ticked, task 41 re-measures, and once 41 is ticked the spec owes `/ductus:review`, `/ductus:analyze` and the completion gate, none of which a task schedules. Each harness run is a model call on the owner's account; `research.md`'s *Command cost (AC8)* table gives each command's price.

- [x] Implement the behavior described in `scenarios/the-search-goes-out-with-the-first-call.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 41. Re-measure result and schema sizes after 34–40's new tools

- [x] Re-run `scripts/measure-result-sizes.sh` once tasks 34–40 have landed, and replace `research.md`'s *Result sizes (AC14)* and *Loaded schema sizes (AC23)* tables, each with its method, units, population, date and commit. Task 34 adds `render-help`, and 35 and 38 may add or change tools, so the recorded tables no longer cover every registry primitive and every tool, and the total no longer adds up. Re-derive the grows-set from the new run, and bound any new member, as task 5 did

- **Done when**: `research.md` names every registry primitive as measured or not measured with its reason, and records each tool's loaded schema size and the total, at a commit after task 40 (AC14, AC23).

## 42. `merge-permissions`' canonical-directory tests on Windows

- [x] Make the `dir` helper in `runtime/src/primitives/merge_permissions.rs`'s tests join its relative path one component at a time, as `canonical_directories` does. The runtime workflow's Windows build + test failed `a_canonical_call_writes_the_canonical_sets_and_both_directories` and `a_canonical_call_retires_names_and_keeps_the_adopters_entries` at daf517be (run 42713267146): the primitive writes `…\.claude\commands\acme`, and the helper joined `.claude/commands/acme` whole, so it expected `…\.claude/commands/acme`

- **Done when**: the runtime workflow's Windows build + test job passes at the commit that lands the fix, since a local pass proves only the Unix branch.

## 43. An adopter bootstrap across task 36's reference read

- [x] Run a real `/ductus` bootstrap on a Claude Code adopter, as task 11 did, by `docs/developing-ductus.md` *Exercising an installer change without a push*. Confirm that its installed `plan`, `implement` and `prune` copies' lines name `derive-boundary`, `append-task` and `prune-project`, and that each Claude copy's line equals the one this repository's generator writes, the project name substituted, as the staging check at 44284f1a found for all 17. The subject is an upgrade from `ductus-v0.59.11`, Claude only, since tasks 34, 35 and 36 changed `framework/bootstrap/` after task 11's last real run (owner, 2026-11-10). `apply-manifest` now reads each reference a command names from the archive's `framework/reference/`. That read was exercised by the cost harness's fixture, which runs `apply-manifest` from the archive's layout, and by `a_cited_reference_sections_tool_joins_the_line_in_both_layouts`, but not through `/ductus`'s own staging. It precedes the release tag, not the review

- **Done when**: `research.md` records the bootstrap's installed lines, with its method, date and commit.

## 44. The view's bound holds the whole result

Tasks 44 to 54 hold what the first review's passes found. Those passes ran on 2026-11-10 at 6f2e314f over the 168 scope entries from base 150b4dc1, and no record was written. The owner chose to fix every gating item rather than defer it (2026-11-10). Once 44 to 54 are ticked, the spec owes `/ductus:review`, its first record, then `/ductus:analyze` and the completion gate. A task whose prose names a design question puts it to the owner before code, one question at a time, with each option's cost, and writes the choice into `plan.md`.

`render()` in `runtime/src/primitives/pipeline_view.rs` collapses and cuts only table rows. `Next actions:`, the scenario-question lines and the Blocked, Recovery and Folds lists stay whole. With 1,701 planned specs on 31-character slugs, `render-status` returned 56,752 characters and `hook status` 56,766, against the 50,000 default (BE-PAYLOAD-001, MUST; AC24, AC7). The bound also measures the rendering rather than the serialized result. How the bound cuts the other blocks is a design question, for `plan.md`'s *One view model, two renderings, one bound*.

- [x] Hold every block, and the serialized result, to `[artifacts] read-size-bytes` on both renderings. Add a test that drives the text rendering through the configured read size on disk, too
- [x] Correct the claims the defect falsifies: `RENDER_STATUS` in `runtime/src/schema/descriptions.rs`, step 1 of `framework/commands/status.md`, `research.md`'s grows-set line for `render-status`, and `data-model.md:50`
- [x] `Counts:` renders only when a spec exists (`pipeline_view.rs:290`). Correct `data-model.md:39` and the doc comment at `pipeline_view.rs:281-282`
- [x] Flatten tags and an out-of-set status before they join a view line, as the disabled-rule-file line does, so a newline cannot forge one (`pipeline_view.rs:322-324`, and 118, 448-466 and 600-606). Remove the no-op `gamma.blocked_by = Vec::new();` at `pipeline_view.rs:719`
- [x] `plan.md:24` and `runtime/tests/common/corpus.rs:2` say the AC24 tests use `corpus.rs`; none does. Correct both

- **Done when**: the on-disk test holds both renderings' results to the read size past the bound, and fails with the new bound removed.

## 45. `read-spec`'s file form is held to the read size

`read-spec`'s file form moves only its five lists, so `review` and `analyze`, with their digest maps, stay inline (`runtime/src/primitives/read_spec.rs:36-37`). A review record with 474 digest entries gave a 60,591-byte `form: file` result (BE-PAYLOAD-001, MUST; AC7). `bounded_members` in `runtime/tests/mcp.rs` bounds at one byte and asserts no size. What moves to the file is a design question.

- [x] Move what grows into the list file in the file form, and say so in `READ_SPEC`'s description and in `data-model.md`'s *Bounded results* table
- [x] Assert a file-form result's size at a realistic read size

- **Done when**: a test holds a file-form `read-spec` result with large digests to the read size, and fails with the move removed.

## 46. Tool loading: pinned copies, one derivation, shared constants

- [x] A pinned `target.md`: `tool_search_context` (`runtime/src/primitives/apply_manifest.rs:572-628`) derives the always-loaded set from the manifest's source, while the MCP server reads the installed, pinned copy. A tool the source names and the pinned copy does not then loads from neither, which breaks AC29. The lookup is a design question
- [x] One derivation for both uses: `AlwaysLoaded::resolve` (`tool_loading.rs:450-459`) uses `parser::parse` with `step_tools`, while `apply_manifest.rs:615` uses `text_step_tools`. A legacy-prose `target.md` therefore derives two different sets, against the plan's *Tool loading derives from each command's parsed steps*
- [x] CFG-CONST-001 (MUST), twice:
  - build `apply_manifest.rs:581`'s `commands/target.md` from `tool_loading::ALWAYS_LOADED_COMMAND`, moving `scripts/mutation/tool-loading.py`'s M15 anchor with it;
  - add one shared constant for the `mcp__ductus__` prefix at `tool_loading.rs:318` and `merge_permissions.rs:268` and `:318`, moving `scripts/mutation/destination-checks.py`'s M8 anchor, which already matches twice.
- [x] Skip fenced blocks and HTML comments in a cited section's body (`tool_loading.rs:282`, M17's anchor) and in `atx_headings` (`tool_loading.rs:381-407`, M12's anchor), or reuse `resolve_anchor::heading_lines`
- [x] Docs:
  - move `read_and_substitute`'s doc comment back above it (`apply_manifest.rs:538-545`);
  - name the invoked-anywhere rule in `apply_manifest.rs:33-39`, `tool_loading.rs:291-293`, `primitives.rs:3051-3055`, `data-model.md:127` and `framework/bootstrap/ductus.md:987`, with `govern.md` copied;
  - in `APPLY_MANIFEST`, name the same rule and correct "halts before any write";
  - `plan.md:101`: audit's copy carries a line;
  - `plan.md:96`: the citation resolution order (the code's order is right);
  - `scripts/gen-claude-commands.sh:20-23`: `--check` also exits 1 when the runtime cannot build the copies
- [x] Add the runtime sources the generator now depends on to `.github/workflows/generators.yml`'s `pull_request` paths. Add a test that drives the MCP server's stderr notice (`mcp/server.rs:151-154`). Fix `destination-checks.py`'s M3 anchor, which still names `always_loaded`. Then run `run.py --check` on every list touched

- **Done when**:
  - a test shows a pinned target's tools loading through the line or the server;
  - one function derives the always-loaded set for both uses;
  - `run.py --check` passes on `tool-loading.py` and `destination-checks.py`.

## 47. Pi's commands register when the load-time view fails

`framework/bootstrap/pi/ductus-bridge.ts:351-352` calls `render-status` at load to learn the namespace. Any view error, such as malformed spec frontmatter or a malformed read size, then rejects registration. Neither `{project}-status` nor `{project}-help` registers, and the session notice blames the pointer (scenario `help-answers-with-no-model-turn`). How the bridge learns the namespace is a design question.

- [x] Register both commands when the load-time call fails, with a notice naming the real error, and add a case for it to `scripts/tests/pi-bridge-harness.mjs`
- [x] Docs. Name `/{project}-help` beside `/{project}-status` in:
  - the harness header (`pi-bridge-harness.mjs:9-10`);
  - `AGENTS.md`'s Pi gotcha;
  - `framework/bootstrap/ductus.md:103`, with `govern.md` copied.

- **Done when**: the harness case passes, and a real `pi --mode rpc` run, as task 12's, shows both commands registered while the view fails.

## 48. Rule loading keeps analyze's integrity checks

The first item's done-when, that analyze reports a malformed rule heading, has not been observed. `discover-rule-files` lists the heading under `other-headings`, and §Rules says to report it, but only an analyze run shows the model doing so. That run is a model call on the owner's account: price it from `research.md`'s *Command cost (AC8)* analyze row ($1.072 on the 40-spec fixture), and ask the owner before running it, on a fixture holding a malformed rule heading.

`framework/reference/analyze.md` §Rules now checks rule headings "from the rules the result lists". But `rule_sections::heading_id_regex` (`^#{2,4}\s+[A-Z]{2,5}-…`) drops exactly the malformed headings those checks exist to catch (QUAL-DELEG-001, SHOULD). How the checks see a malformed heading is a design question.

- [x] Let analyze report a malformed rule heading. Record what the shared parser gives up both at the call site and in `rule_sections.rs`
- [x] QUAL-TEST-001 (SHOULD): add a test asserting the `should` tier (`discover_rule_files.rs:186`), with a SHOULD rule added to the test's `RULES`
- [x] Docs:
  - `discover_rule_files.rs:72-81`'s `# Errors` and its module doc, and `primitives.rs:510-511`;
  - `merge_permissions.rs:89-106` ("before any filesystem read") and `framework/bootstrap/configure/claude.md:18` (what a created file holds);
  - `framework/commands/review.md:39`: steps 1, 2, 9 and 12 write lists files;
  - `framework/commands/analyze.md:69`: `; The` becomes `. The`;
  - the `configure/claude.md` comment in `.github/workflows/runtime.yml:66-68`: it covers steps 3 and 4 too;
  - analyze's Scope Boundaries: name the lists file and the rule files.
- [x] Fix each of these, or waive it with its reason:
  - `all` still validates `[rules] surfaces` and the disabled entries — fixed: under `all` neither is validated;
  - `extra-files` and `verification` are silently ignored without `rules` — fixed: either without `rules` is an error;
  - `read-section`'s `headings` is required on the CLI but may be empty over MCP — fixed: an empty list is an error on every surface;
  - merge-permissions' canonical revoke-overlap message names `--allow` — fixed: it names the allow and revoke sets.

- **Done when**: analyze reports a malformed rule heading, and the tier test fails with the mapping removed.

## 49. Bounded results: the walker, writes before a failure, mutation kills

- [x] `restore_lists` (`runtime/src/interpreter/payload.rs:417`) marks every key in the list file restored, including `compute-review-scope`'s non-list `diff-base`. So past the read size, `diff-base` leaves `performReview`'s legacy tail, against 022's `data-model.md:615`. Filter out the keys the result carries inline, and add a test
- [x] Add a test of the walker's halt when `restore_lists` errs (`interpreter/mod.rs:266-282`)
- [x] QUAL-TEST-001 (SHOULD): `scripts/mutation/scope-results-bounded.py`'s M4 survives its named test. Add a `diff_cross_spec` unit test of the file form, and point M4 at it
- [x] `prune-tasks --apply` and `rewrite-spec-links` parse `[artifacts] read-size-bytes` only after they write (`prune_tasks.rs:126-158`, `rewrite_spec_links.rs:497-528`). Resolve it before any write, and add tests
- [x] Docs:
  - `payload.rs:84-90`: `ListsFileUnreadable`'s causes;
  - `bounded_lists.rs`' module doc (its members and keys) and its `IRREGULAR_COUNTS` test claim;
  - "one entry per line", which holds only for lists of strings, in `data-model.md:147` and `bounded_lists.rs:82-84`;
  - the `# Errors` of `check_artifacts.rs:138-152`, `check_review_gate.rs:30-46` and `dashboard.rs:46-55`.
- [x] The audit scripts' lists-file branches read a missing list as empty (`scripts/audit/lib.sh:146-150`, `broken-relative-links.sh:96-116`, `manifest-destination-links.sh:170-179`). Halt instead, or waive with the reason

- **Done when**:
  - each new test fails with its behavior removed;
  - `run.py --check` passes on `scope-results-bounded.py` and M4 runs killed.

## 50. AC7's population covers routing, drift and cycles

`runtime/tests/result_sizes.rs:165-167` measures `derive-routing-candidates` with a description that can never match the corpus's `NNN-feature-NNN` slugs, and the corpus never drifts its frontmatter. So two results that grow with the corpus measure as flat:

- `derive-routing-candidates`, with 60,530 characters and no `form` at 300 matching specs;
- `derive-dependencies`, with 31,395 characters at 801 drifted specs.

`traverse-deps`' `cycles` is never seeded either. AC7 fails as it stands.

- [x] Seed the corpus with routing-matchable slugs, a drifted variant and a cycle, then measure. Bound each new member through `bounded_lists`, with its `bounded_members` row in `runtime/tests/mcp.rs` and its `data-model.md` row
- [x] QUAL-TEST-001 (SHOULD, low confidence): `the_corpus_scales_every_dimension` (`result_sizes.rs:552-613`) builds 4 specs, so the per-ten branch-scoped scaling goes untested
- [x] BE-TIMEOUT-001 (MUST, low confidence): `result_sizes.rs:272-287` reads the MCP server with no deadline. Fix it, or waive with the reason
- [x] Small items:
  - the `traverse-deps` comment at `result_sizes.rs:189-190`;
  - `corpus.rs:84-96`'s unreachable `clarified` arm;
  - `CorpusSize.defects`, which is always true, so its false branches are dead;
  - `scripts/measure-result-sizes.sh:33-41`, which reads the sha after the run and checks only `runtime/` for uncommitted changes.
- [x] After every description change in tasks 44 to 52, re-run `scripts/measure-result-sizes.sh` and replace `research.md`'s two tables, as task 41 did. Run it once those changes are committed, on a clean tree: the script records the tree's state, and fails when HEAD moves during the run. Then replace the figures that cite the tables: the 11,347 characters in `plan.md`'s *Tool loading derives from each command's parsed steps*, the grows-set lines in *Grows with the corpus*, and AC23's total

- **Done when**: `research.md`'s grows-set names every member, and each member is bounded with its test.

## 51. The cost harness and the generator's environment

- [x] BE-TIMEOUT-001 (MUST): put a timeout on every child process `scripts/measure-command-cost.py` waits on (lines 85-86, 373-386 and 434)
- [x] The rest of the harness:
  - send stderr to a temporary file, so a full pipe cannot deadlock (379-380, 440);
  - have `child_env` drop, by name, the Claude Code session variables it now passes (357-361);
  - have each row record the commit, the dirty state and the Claude Code version (493-495);
  - check the docstring's `enabledMcpjsonServers` claim against `ductus.md` (23-28, 186-197);
  - replace `int(d.name[:3])` at 266 (task 39's seed), which breaks AGENTS.md's feature-number gotcha.
- [x] CFG-CONST-001 is task 46's. CFG-ENV-002 (MUST, low confidence): `DUCTUS_BIN` (`scripts/gen-claude-commands.sh:76`, `scripts/tests/test-gen-claude-commands.sh:28`) goes into `docs/runtime.md`'s inventory, or that inventory states that maintainer-script variables are out of its scope

- **Done when**: the harness waits on nothing without a timeout, and `--help` and one model-free fixture build still run.

## 52. Descriptions, docs, and the flaky probe test

- [x] Descriptions:
  - `DASHBOARD` (`descriptions.rs:49`) is no longer the status surface;
  - `DERIVE_ROUTING_CANDIDATES` (`:57`): groom calls it too;
  - `InboxStanding`'s docs (`primitives.rs:5921-5931`).
- [x] `runtime/src/main.rs`:
  - `NON_PRIMITIVE_SUBCOMMANDS`' reason (1161-1167) names `hook help`;
  - `HookCommand`'s doc (45) says one per slash command;
  - the `Hook` variant's maintainer note (93-96) becomes a `//` comment.
- [x] `merge_status_hook::tests::the_probe_reads_a_version_and_names_each_failure` flakes on macOS: it times out at 5 s on its freshly written `prints` script. The session closing 047 and 026 measured two whole-suite failures and one in five lone runs at b70246f7. Give each probe that must answer a timeout the first run of a new file cannot reach
- [x] `merge_status_hook.rs:133-136`: the matcher is unanchored and `[host] project` is not validated (low confidence). Fix it, or waive with the reason — waived: `/ductus` validates `[host] project` as lowercase letters, digits and hyphens before writing it (`framework/bootstrap/ductus.md`), and every name of that shape is a literal regular expression. An unanchored matcher could match only a command whose name starts `{project}:status` or `{project}:help`, and `/ductus` installs none. Escaping or anchoring the matcher would change what Claude Code is given on every project, which only a Claude Code run verifies, for a case only a hand-edited config reaches. Validating the name here would refuse a project with no `[host] project`, whose name falls back to its directory's basename.
- [x] `docs/developing-ductus.md:7-14`: registering a primitive now takes seven sites, the seventh being `result_sizes.rs`' case

- **Done when**: the probe test passes in ten lone runs on macOS, and each description edit is in task 50's re-measure.

## 53. 080's own artifacts

- [x] `data-model.md`: add a `discover-rule-files` row to *Bounded results*, and the shapes of `read-section`'s arguments and result and of `discover-rule-files`' `rules`, `verification`, `all` and `extra-files` arguments and `rule-files` and `unreadable` fields
- [x] `framework/reference/groom.md:22` forbids shell searches of spec bodies with no exception for an item that matches nothing. That contradicts the scenario's amended Edge Case and `plan.md`'s *An item that matches nothing* (task 39). Add the exception
- [x] `plan.md`:
  - `:32`: the grows definition is twice the small result and 200 characters more, as `result_sizes.rs` has it;
  - `:64`: 062 and 079 are `done`;
  - `:72`: the assumption is settled by `research.md`'s task 8 run;
  - `:85`: the figure is 11,347 characters at HEAD by the same method;
  - `:234-239`: four new tools, not three, and the rows that name lists-file branches the plan rejected;
  - `:254-265`: the Data Model list lacks render-help, `hook help` and the canonical call;
  - `:309`: the condition is reversed.
- [x] `spec.md`:
  - `:70` and AC24's "final line": the count line closes the table block, and the data model is right;
  - `:136`: `target.md`'s step 3 reads `specs[].slug`.
- [x] `research.md`: task 43's record says "runtime source unchanged since 56a92b87", which is false, since other specs changed `write_analysis.rs` and `render_status.rs` in between. Correct it
- [x] The scenario Contexts that describe the pre-change state in the present tense: `configure-types-no-permission-entries.md:9` and `groom-derives-routing-candidates.md:9`
- [x] Owner decides:
  - AC27, which cannot be ticked as written, since `research.md` records the context check as waived — decided: reworded so that, with no model turn to run, the answer comes from pi's documentation and the record says so; `research.md`'s task 12 record now states it (owner, 2026-11-10);
  - the OpenCode and Antigravity `/{project}:` hint gap, which tasks 28, 32 and 33 and those scenarios' Edge Cases leave without a disposition — waived: it predates 080, since `/ductus` substitutes only `{project}` and every installed command's prose on those hosts already spells the colon form, so it is not the view's alone, and AC10 holds; recorded in the plan's *Auggie, OpenCode and Antigravity* and both scenarios' Edge Cases (owner, 2026-11-10).

- **Done when**: each item is fixed, or decided with its reason written on this task.

## 54. Other specs' claims that 080's work left stale

Each edit goes through `classify-spec-edit` first. An edit to 022's scenarios or data model stales 022's review.

- [x] 022 (`in-progress`):
  - `scenarios/dashboard-primitive.md:39`, a field list 080 wrote, omits `specs-count`, `tags-union-count`, `form`, `lists-file` and `reopenings`. Its `:47` says read-only, and its `:25` and `:58` say `null` where the key is absent;
  - `scenarios/coverage-expansion-primitives.md:32` names the render field;
  - `scenarios/framework-list-dedup.md:24` gives the path a default;
  - `data-model.md`: `:152` (the count and form fields), `:142` and `:148` (the callers of `command_file_candidates`), `:615` (the legacy tail, and `lists-file-unreadable`), `:873` (the absolute paths);
  - `spec.md:97`: "two interfaces".
- [x] 064 (`done`):
  - AC12 (`spec.md:207`) still says "Pi has no MCP client". Its fix reopens 064 (`classify-spec-edit` measured `reopens`, `criterion`), and the owner decides;
  - `:44` and `:59`, "extension-only": the `:44` edit measured as a correction;
  - `:71-73`, the Compatibility Contract;
  - `:59` and `:223`, the unsourced "pi 1.0.0".

  Made in bdd2032a, each a correction, so 064 stays `done`: `:44` and `:59`, the Compatibility Contract, which also gains the registered commands as a permitted deviation, and the source, which dates pi's MCP client to 0.99.0 (pi-coding-agent's CHANGELOG, 2026-09-29), swept across the seven live artifacts that said 1.0.0. AC12: corrected in 4b6c5fe4, which reopens 064 alone, by the owner's decision to reopen it and run its review, analysis and gate in this session (2026-11-10).
- [x] 063: `spec.md:38`'s signpost says 080 made the read size a bound on tool results, where 080 extended the bound 066 and 078 already applied
- [x] 062 (`done`): `plan.md:42-44`'s callers omit `render-status` and `hook status`
- [x] 066: `data-model.md:51`. The walker now halts at the `compute-review-scope` step
- [x] 007: in `tasks.md:55-56`, the scenario edit should come before the bootstrap edit (low confidence)
- [x] 046 (`done`): correct AC24 and the *Many scenarios carrying questions* edge case so that they except the pipeline view past the read size. There the view counts the question lines it does not show, and on the model path it names the file holding them all. Back-link 080, which discharges 080's `cross-spec-impact:` entry. `classify-spec-edit` reads a criterion edit as `reopens`, so 046 goes back to `in-progress` through `set-status` and owes a review, an analysis and its gate (owner, 2026-11-10; plan *Bound (AC24)*). Land it after the runtime change it describes, in a commit touching only 046's directory, so its review window holds that one file
- [x] 066 (`in-progress`): remove `scenarios/write-analysis-and-the-second-gate.md`'s edge case on `skip_serializing_if` on `ReadSpecResult.analyze`, since `read-spec` no longer carries either audit record (owner, 2026-11-10; plan *Corpus-growing results are bounded through `bounded_lists`*). Back-link 080, which discharges 080's `cross-spec-impact:` entry. Land it after the runtime change, in its own commit. The edit adds one file to 066's owed review

- **Done when**: each edit is made or waived with its reason, and each reopen it starts has the owner's decision.

## 55. Disposition out-of-spec finding: command-reference-split.py's M2 and M5 anchor on code that is gone

- [x] `scripts/mutation/command-reference-split.py` — `run.py --check` reports M2 (`headings inside fenced blocks and comments are collected`) and M5 (`the line rule accepts a heading that ends inside a word`) with anchors that occur nowhere in `runtime/src/primitives/resolve_anchor.rs`, which 080's work did not change, so both mutants were already invalid at HEAD and run nothing — fixed: f9991d60, 080's tasks 36 and 40, is what moved both anchors, so the fix was 080's own; re-anchored on `heading_lines`' enumerated filter and `cited_heading`'s word-end check, and M2 and M5 run killed (e66f79c2)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 56. Disposition out-of-spec finding: four artifacts say `dashboard` returns `session-target: null`, where the key is absent

- [x] `framework/commands/target.md` step 2 ("the dashboard's documented \"session file → session-target: null\" reset state holds"), `specs/000-slash-commands/scenarios/target-clear-flag.md` (Context) and `scenarios/dashboard-dependencies-column.md` (item 2), and `specs/022-deterministic-runtime/scenarios/cli-config-dir-per-contributor.md` ("reports `session-target: null`") — `DashboardResult.session_target` has been `skip_serializing_if = "Option::is_none"` since the primitive landed (07371334), so with no target the result has no `session-target` key. Task 54's edit of 022's `dashboard-primitive` scenario, the claim's source, now says so, which leaves target.md quoting a state the scenario no longer documents (task 54's 022 item, 2026-11-10) — fixed: the five occurrences, two of them in `target-clear-flag.md`, now say the result has no `session-target`; target.md's `dashboard` stays unbackticked, since `ductus exec` dispatches a backticked primitive and the target-basic golden caught a backticked one as a new step

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 57. analyze reads the rules' list file and a rule file through the shell, against §Rules

- [ ] Task 48's run (research.md, *Analyze's malformed-rule-heading check (task 48)*): after reading `discover-rule-files`' list file whole with the host's reader, analyze queried it with `jq` three times, one denied as compound, and read the malformed heading's lines with `git show 149e3cf -- specs/rules/quality-cross.md`. `framework/reference/analyze.md` §Rules says neither the list file nor a rule file is read through the shell, and the `rules-load-without-shell-reads` scenario rules out a shell read of a rule file. `other-headings` gives each heading's line and text only, while §Rules asks the model to judge each against the file's structure, which the model read from git. How analyze sees the text under such a heading without the shell is a design question for the owner, priced before code

- **Done when**: an analyze run of the `analyze-malformed-rule-heading` check reads neither the list file nor a rule file through the shell, or the owner waives it with its reason written on this task.
