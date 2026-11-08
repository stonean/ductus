# 080 — Deterministic runtime efficiency Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Fixture corpus and the result-size script

- [x] Write `runtime/tests/common/corpus.rs`: a git-initialized project of N specs, scaling the spec count and each spec's criteria, tasks and scenarios
- [x] Write `runtime/tests/result_sizes.rs` (`#[ignore]`) and `scripts/measure-result-sizes.sh`. Run every `PRIMITIVE_REGISTRY` name on corpora of two sizes and record the characters of each result's text, or a *not measured* row with its reason
- [x] Record each tool's loaded schema (description plus input schema, in characters, from `ductus mcp` `tools/list` over stdio) and the total
- [x] Create `research.md` holding both tables, each with its method, units, population and date, plus the grows-set and the handoff's three method traps

- **Done when**: `research.md` names every registry primitive as measured or not measured with its reason, names the primitives that grow with the corpus, and records each tool's loaded schema size and the total (AC14, AC23).

## 2. One view model, two renderings

- [ ] Move the view's rendering from `runtime/src/primitives/dashboard.rs` into `runtime/src/primitives/pipeline_view.rs`, built on one model
- [ ] Render markdown and fixed-width text as `data-model.md` defines
- [ ] Put the summary one item per line, with one line per unresolved scenario question, then the `Next actions:` block, then the single closing `/{project}:target` prompt
- [ ] Test that one model rendered both ways carries the same facts

- **Done when**: unit tests show three things. The text rendering aligns its columns with two-space gaps and carries no `|` row syntax or `**`. Both renderings put each summary item on its own line and list the non-done specs by next action, with the `/{project}:target` hint once at the end. Both carry the same facts from one model (AC6's first half, AC17, AC18, AC19).

## 3. The view's bound

- [ ] Collapse the `done (spec is complete)` rows to a count line past `read-size-bytes`, then cut the rows past the bound and end with a line counting the specs not shown
- [ ] Test both renderings on a generated corpus past the bound

- **Done when**: the test passes for both renderings (AC24).

## 4. `render-status`, the status command, and `dashboard`'s dropped field

Blocked on tasks 2 and 3: the primitive returns their view.

- [ ] Register `render-status` at the six sites `docs/developing-ductus.md` lists, then run `scripts/gen-configure-mcp.sh`
- [ ] Rewrite `framework/commands/status.md`: step 1 invokes `render-status`, step 2 emits `rendered-markdown`, and step 3 is removed. Sweep `framework/reference/status.md` with it
- [ ] Drop `rendered-markdown` from `dashboard`, and sweep the name across the plan's list and by meaning
- [ ] Re-bless `runtime/tests/golden/status-basic.jsonl`, reading the diff

- **Done when**: `ductus exec status` and the MCP tool both return the view without `dashboard`'s structured payload, `framework/commands/status.md` composes neither the list nor the prompt, and `rendered-markdown` survives only on `render-status` and in records (AC6, AC13).

## 5. Bound the results that grow with the corpus

Blocked on task 1: its grows-set is this task's population.

- [ ] Return each grows-set primitive through `bounded_lists::place`
- [ ] Add the lists-file branch where a command reads a moved list: `framework/commands/implement.md`, `framework/commands/review.md` `--all`, `framework/commands/target.md` step 3, and any other reader a grep for the moved fields finds
- [ ] Add each member's row to `data-model.md`'s bounded-results table
- [ ] Test each member: drive a generated corpus past the bound and read the whole list back from `lists-file`

- **Done when**: every primitive `research.md` names as growing is bounded and has its test, `read-tasks` among them (AC7).

## 6. The `ductus hook status` surface

Blocked on task 2: it shows the text rendering.

- [ ] Add `runtime/src/hook.rs` and the `hook status` subcommand, and list `hook` in `NON_PRIMITIVE_SUBCOMMANDS` with its reason
- [ ] Test the block output and its `suppressOriginalPrompt`, ignored command arguments, the `CLAUDE_PROJECT_DIR` root, and exit 1 with the error on stderr and nothing on stdout when the view cannot be rendered

- **Done when**: the tests pass (AC16; AC17 and AC25 on the runtime side).

## 7. `merge-status-hook`

- [ ] Register it at the six sites, then run `scripts/gen-configure-mcp.sh`
- [ ] Gate on `claude --version` (2.1.101 or later); recognize ductus's entry by its command; preserve every other key and entry; write atomically
- [ ] Confirm the probe reads the version from inside an MCP server Claude Code spawned. If it cannot, switch to the plan's fallback, in which the host runs the probe and passes its output, and update the plan's decision
- [ ] Test `created`, `updated`, `unchanged`, and `skipped` for an old and an unreadable version. A second run leaves exactly one entry, another `UserPromptExpansion` entry is untouched, and a skip leaves the merged permissions intact

- **Done when**: the tests pass (AC2, AC20).

## 8. Install the hook, and verify it in Claude Code

Blocked on tasks 6 and 7.

- [ ] Add the `merge-status-hook` step to `framework/bootstrap/configure/claude.md` after `merge-permissions`
- [ ] Add it to `framework/bootstrap/ductus.md` for each Claude agent once the runtime is acquired. Add its CLI rule to Claude's seed and sweep the seed's subcommand count
- [ ] Remove the local prototype, `.claude/hooks/status-hook.sh` and its `settings.local.json` entry. Then run `scripts/gen-claude-commands.sh`, since the slash command loads the installed copy, and run `/ductus:configure` in this checkout
- [ ] Type `/ductus:status` twice and confirm the view shows with no model turn and no "Original prompt:" line, then from a session started in a subdirectory, then with the pointer moved aside. Confirm the last exits 1, falls back to the model path, and record whether Claude Code shows a notice

- **Done when**: `research.md` records each run with its Claude Code version, timing and date (AC1, AC16, AC25).

## 9. Step-tool derivation and `alwaysLoad`

- [ ] Write the derivation in `runtime/src/primitives/tool_loading.rs`: served registry names as whole tokens in the parsed `## Instructions` steps' prose
- [ ] Flag the tools of the installed `target.md` that `Host::command_file_candidates` finds with `_meta["anthropic/alwaysLoad"]`, and say on stderr when there is no copy
- [ ] Test in `runtime/tests/mcp.rs`: the flagged set equals the derivation over `framework/commands/target.md`, and each flagged description is at most 4,096 characters
- [ ] Start a Claude Code session in this checkout and confirm target's tools are reached without a `ToolSearch`

- **Done when**: the test passes and `research.md` records the session check (AC21).

## 10. The batched `ToolSearch` line

Blocked on task 9: the line drops the always-loaded set.

- [ ] Add `tool-search-line` to `ManifestEntry` and the insertion to `apply-manifest`
- [ ] Set the field on Claude's command rows in `framework/bootstrap/ductus.md` only
- [ ] Have `scripts/gen-claude-commands.sh` build the Claude copies by driving `apply-manifest`, keeping `--check`'s behaviour
- [ ] Move `.githooks/pre-commit`'s runtime resolution above its generator calls
- [ ] Test that a flagged copy parses to its source's `Procedure`, and that an unflagged entry's output is unchanged

- **Done when**: the tests pass, each `.claude/commands/ductus/` copy whose remaining step tools are non-empty carries the line, and `scripts/gen-claude-commands.sh --check` passes (AC29).

## 11. An adopter bootstrap across the change

Blocked on tasks 8 and 10.

The real `/ductus` run waits for the commits to reach `main`. A version-bumped pin with no tag halts `--ref=main`, so that run names this checkout's release build in `[runtime] path`, which `/ductus`'s acquisition Branch 1 uses with a warning.

- [ ] Bootstrap a scratch adopter, with Claude and Auggie selected, from a release before this spec
- [ ] Before pushing: drive `apply-manifest` and `enforce-manifest` from a staging copy of `framework/`, per `docs/developing-ductus.md` *Exercising an installer change without a push*. Confirm that Claude's copies carry the line and Auggie's are byte-identical to before
- [ ] After pushing: have a subagent run `install.sh` on the scratch adopter, per the same section, with `[runtime] path` naming this checkout's release build. Confirm the hook lands in `.claude/settings.local.json` with no manual step and Auggie's settings are unchanged

- **Done when**: `research.md` records both runs and each check (AC10; AC26, Claude Code's half).

## 12. Pi's registered command

Blocked on task 4: the handler reads `render-status`.

- [ ] Register `{project}-status` in `framework/bootstrap/pi/ductus-bridge.ts`, with `registerCommand` and `sendMessage` added to its `ExtensionAPI` slice
- [ ] Add the cases to `scripts/tests/pi-bridge-harness.mjs`: the view shown with no turn, and a failure shown the same way
- [ ] Run a real `pi` session, and record in `research.md` whether the view reached the agent's context, whether Pi can add context without a turn, whether a displayed message renders markdown, and whether the command or the prompt template ran
- [ ] If markdown does not render, add `render-status`'s `format`. If the template ran, stop `/ductus` installing Pi's status template, and run `classify-spec-edit` on 064's edit

- **Done when**: the harness passes, a real `pi` run shows the view with no turn, and `research.md` records the four outcomes (AC9, AC27, AC26's Pi half).

## 13. Pi's compact form

Blocked on task 12.

- [ ] When `research.md` records that the view did not reach the context and Pi can add context without a turn, add the target line and the non-done specs grouped by status to the agent's context, and a harness test for it

- **Done when**: either `research.md` records that the condition does not hold, or the bridge adds the compact form and its harness test passes (AC28).

## 14. OpenCode's scenario

- [ ] Run OpenCode on a version whose TUI plugin's slash command runs local code, and try `/{project}:status` through it
- [ ] Record the scenario through `/ductus:amend`, with the outcome and the deferral it settles under Resolved Questions, with its trigger

- **Done when**: the scenario exists under `scenarios/`, is settled by the run, and has no open question (AC11's half; AC10 for OpenCode).

## 15. Antigravity's scenario

- [ ] Run `agy` 1.4.11 or later with a `PreInvocation` hook, and record whether it sees the raw command name
- [ ] Record the scenario through `/ductus:amend`, as in task 14

- **Done when**: the scenario exists under `scenarios/`, is settled by the run, and has no open question (AC11's half; AC10 for Antigravity).

## 16. The `claude -p` harness

- [ ] Write `scripts/measure-command-cost.py`, which builds a fixture project and records four things per run: time to first visible output, time to result, output tokens, and the largest tool result in characters
- [ ] Run it once, and record whether `claude -p` executes a custom slash command and how a local `UserPromptExpansion` hook treats a `-p` prompt

- **Done when**: no workflow references the script, and `research.md` records its first run's two findings (AC15).

## 17. Every command's cost

Blocked on tasks 4, 8, 10 and 16: the round measures the commands as changed.

- [ ] Run the harness over every command in `framework/commands/`

- **Done when**: `research.md` records each command's four figures with method, units, population and date (AC8).

## 18. Each command's review

Blocked on task 17.

- [ ] Name in `research.md`, per command, the reduction that applies or why none does
- [ ] Record each reduction as a scenario through `/ductus:amend`

- **Done when**: every command has a reduction or a reason, and each reduction has its scenario.

## 19. Revisit always-loading every tool

Blocked on task 1's schema sizes.

- [ ] Put the measured figures to the owner, and record the decision in `spec.md`'s Resolved Questions

- **Done when**: the spec's Resolved Questions record the owner's decision beside the figures.

## 20. Back-links on 022, 063 and 064

Once this task is ticked the spec owes `/ductus:review`, `/ductus:analyze` and the completion gate, none of which a task schedules.

- [ ] 022: link back to 080 from the `dashboard-primitive` scenario or from a blockquote signpost in its `spec.md`, covering the `hook` surface, `render-status`, `merge-status-hook`, the `alwaysLoad` `_meta` and the bounded results. A link in `data-model.md` discharges nothing, since the gate reads only `spec.md` and `scenarios/*.md`
- [ ] 063: add a signpost that `read-size-bytes` now bounds primitive results and the view
- [ ] 064: add a signpost for the bridge's registered command, and run `classify-spec-edit` on the edit

- **Done when**: `check-review-gate` reports all three `cross-spec-impact:` entries discharged.
