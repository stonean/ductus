---
status: in-progress
dependencies: []
cross-spec-impact:
  - 022-deterministic-runtime
  - 064-pi-host-support
  - 063-oversized-artifacts-warn-with-a-fix
  - 023-govern-refinement
next-criterion: 30
---

# 080 — Deterministic runtime efficiency

What a `ductus` slash command costs in an agent session — wall-clock time, model tokens, and the size of the tool results the agent reads — and the work of moving what the model did, where the runtime can do it, into the runtime. Every command is reviewed here; `/{project}:status` is the first case.

## Motivation

The runtime made each primitive fast, but a command's cost in a session stayed the model's, because a host shows the user nothing the model did not write. Across 10 `/{project}:status` runs in this repository's Claude Code session transcripts, 2026-10-30 to 2026-11-08, the answer took 41–54 s from prompt to finished reply while its one `dashboard` call took at most 0.5 s. The model spent 10–18 s reading a 37,171–40,454-character result and 26–30 s re-typing the ~9,500-character view, 5,352–6,441 output tokens a run. A prototype `UserPromptExpansion` hook that answered `/{project}:status` from `dashboard` on 2026-11-08 showed the view in 0.17 s with no model turn, in two runs.

Tool results had no size bound either. Claude Code saves a result over 50,000 characters to a file and replaces one over its token cap with an error. Of 3,588 distinct `ductus` tool calls in the same transcripts to 2026-11-08, sized in characters of result text, `read-tasks` had hit both limits four times, and `dashboard`, `read-spec` and `check-corpus-links` had returned results over 36,000 characters.

## Behavior

- **Measuring a command.** A command's cost is its time to first visible output, its time to result, its model output tokens, and its largest tool result in characters, each recorded with its method, units, population and date (§grounding). The deterministic half, result sizes, is measured by a script on a generated fixture corpus with no model. The model half, time and tokens, comes from an on-demand harness that drives the host headlessly. Session transcripts count as supporting evidence only, since they are each contributor's local state.
- **Reducing it**, in order of preference: answer without a model turn where the host allows it; shrink what the model reads; shrink what it writes. Work that needs judgment stays with the model (§runtime-boundary).
- **One case per command.** Each command's review beyond `/{project}:status` is a scenario under this spec.

### `/{project}:status` without a model turn

In Claude Code, `/{project}:status` typed by the user is answered by the runtime through a `UserPromptExpansion` hook. The hook blocks the prompt and shows the pipeline view as its reason, so no model turn runs. It sets `suppressOriginalPrompt`, so the message does not end with the typed command echoed back. The hook's command is the `ductus` binary itself, with no shell script, `jq` or `perl` between, so it runs wherever the runtime runs. `/{project}:configure` installs the hook in Claude Code's `settings.local.json` beside the permissions it already merges. It does so only on Claude Code 2.1.101 or later, since an earlier release ignored a whole settings file over one unrecognized hook event, permissions included. On an older release, or one whose version it cannot read, it writes no hook and says why. The agent does not receive the view, since Claude Code adds a block reason to no context; it calls `dashboard` when it needs the data. The hook answers only the command a user types: one the agent invokes itself takes the model path, since then the agent is the one that needs the view.

In Pi, the `ductus` bridge extension registers the command itself (`registerCommand`), and its handler runs the `ductus` binary and shows the view as a displayed message that triggers no turn (`sendMessage` with `display: true`, `triggerTurn: false`). The run that verifies this records whether that message reached the agent's context. If it did not, and Pi can add text to context without a turn, the bridge also adds the compact form: the target line and the non-done specs grouped by status.

Auggie keeps the model path: its prompt hook cannot block. OpenCode and Antigravity keep it too, as each one's scenario under this spec settles: `opencode-keeps-the-model-path` and `antigravity-keeps-the-model-path`.

### A view for each path

The runtime renders the view two ways from the same data. The hook path shows it in a terminal that renders no markdown, so it gets fixed-width plain text: table columns aligned with two-space gaps, with no pipes, separator row or emphasis. The model path's hosts render markdown in chat, so it keeps a markdown table, which is also the smaller one for the model to re-type. Both put the summary one item per line:

- counts, tags, reopenings and inbox each on a labelled line;
- each unresolved scenario question on its own line beside its spec;
- the `/{project}:target` hint once, at the end.

The view also carries what step 3 of `framework/commands/status.md` had the model compose — the non-done specs grouped by next action and the `/{project}:target` prompt.

On the model path, `/{project}:status` reads the view alone, not `dashboard`'s structured payload, because nothing in the command reads the payload once the view carries the grouping. `/{project}:review --all` still reads `specs[].status` from the full payload.

### Loading tools

Claude Code defers each MCP tool's schema until the agent searches for it. Each search costs an extra model step, while the search itself returns in about 0.01 s.

- **Always loaded:** the tools `framework/commands/target.md`'s steps name, the session-opening command's, set `_meta["anthropic/alwaysLoad"]`. The set is derived from that file, so it follows the file's edits.
- **Batched:** every other command loads the `ductus` tools its steps name, and those the sections it cites invoke, in one search, generated from its files.
- **Measured:** every tool's loaded schema, its description plus input schema, is measured.
- **Revisited:** always-loading every tool was decided again once the schemas were measured, and target's tools stay the only ones loaded with the session (owner, 2026-11-09).

### Bounded tool results

Every primitive whose result grows with the corpus bounds its size or pages it (`BE-PAYLOAD-001`), so no result crosses a host limit as the corpus grows. The bound is the configured read size, `.ductus/config.toml` `[artifacts] read-size-bytes` (50,000 bytes by default), the size spec 063 already uses for one read. A project on a host with a smaller limit lowers it.

## Edge Cases

- **The hook cannot run the runtime.** This covers a missing binary, a timeout, or a `dashboard` that fails. The hook exits with a failure status rather than blocking with an error, so Claude Code reports the failure and runs the command on the model path, which surfaces the error through its own call.
- **Arguments after the command** (`/{project}:status foo`) are ignored, as the model path ignores them.
- **A session started below the project root.** Sessions are expected to start at the project root. Claude Code sets the hook's project directory to the directory the session started in, so below the root the hook cannot find the runtime and takes the failure path above. The model path finds no `ductus` server there either, since `.mcp.json` starts it by a path relative to the root. At the root, the hook's environment carries `CLAUDE_CODE_SESSION_ID` (verified 2026-11-08), so the view names this session's own target.
- **Another `UserPromptExpansion` hook already in `settings.local.json`.** `/{project}:configure` adds its entry beside it and recognizes its own entry by its command, never editing another. Concurrent runs each write the whole file atomically, as `merge-permissions` does, so the file is never half-written.
- **Pi has a prompt template of the same name.** The command the bridge registers must be the one that runs. The verifying run confirms it, and if the template runs instead, `/ductus` stops installing Pi's status template.
- **Pi's handler cannot run the runtime.** It shows the failure as a displayed message, again with no turn.
- **Pi's session identity.** The handler runs the binary with the identity the bridge's MCP server uses, so the view names the same target the agent sees.
- **A view larger than the read size.** At about 105 bytes a row, this is roughly 450 specs. The `done (spec is complete)` rows collapse to one count line first. If the view is still over, the rows past the bound are cut, and a final line says how many specs were not shown, so a partial view never reads as complete (owner, 2026-11-08). Collapsing took the 77-spec table from 7,625 to 4,271 characters. Paging was rejected because the hook path shows one message and cannot page.

## Acceptance Criteria

- [ ] AC1: Typing `/{project}:status` in Claude Code, in a project bootstrapped at this spec's release, shows the pipeline view with no model turn, through a hook whose command is the `ductus` binary.
- [ ] AC2: `/{project}:configure` adds a `UserPromptExpansion` hook matched to `{project}:status` to `settings.local.json`, and running it again leaves exactly one such entry.
- [ ] AC20: `/{project}:configure` writes the hook only when `claude --version` reports 2.1.101 or later. When the version is older or unreadable it writes no hook entry, leaves the permissions it merged intact, and reports that it skipped the hook and why.
- [ ] AC16: The hook's block response sets `suppressOriginalPrompt`, so the shown view does not end with an "Original prompt:" line.
- [ ] AC25: When the hook cannot run the runtime it exits with a failure status, not a block: Claude Code reports the hook failure and runs `/{project}:status` on the model path, which surfaces the runtime's own error rather than showing nothing.
- [ ] AC26: A project bootstrapped before this spec's release gets the hook, and Pi's registered command, the next time `/ductus` or `/{project}:configure` runs, with no manual step.
- [ ] AC9: Typing `/{project}:status` in Pi, in a project bootstrapped at this spec's release, shows the pipeline view with no model turn, through a command the `ductus` bridge registers that runs the `ductus` binary.
- [ ] AC27: The run that verifies the Pi path records in `specs/080-deterministic-runtime-efficiency/research.md` whether the displayed view reached the agent's context and, when it did not, whether Pi can add text to context without a turn.
- [ ] AC28: When `research.md` records that Pi's displayed view did not reach the agent's context and that Pi can add context without a turn, the bridge adds the compact form (the target line and the non-done specs grouped by status), and a bridge harness test shows it.
- [ ] AC10: Auggie, OpenCode and Antigravity run `/{project}:status` as they did before this spec, except where a scenario under this spec has since given one of them a model-free path.
- [ ] AC11: OpenCode and Antigravity each have a scenario under `specs/080-deterministic-runtime-efficiency/scenarios/` that settles its model-free path for `/{project}:status`: OpenCode's by a run on a host version that has the mechanism, Antigravity's recorded without a run, since the installed `agy` lacks it (owner, 2026-11-09).
- [ ] AC17: The view the hook path shows is fixed-width plain text: table columns aligned with two-space gaps, and no markdown table syntax or emphasis.
- [ ] AC18: On both paths the view's summary puts one item per line — counts, tags, reopenings and inbox each on a labelled line, each unresolved scenario question on its own line beside its spec — with the `/{project}:target` hint once at its end.
- [ ] AC19: The model path's view keeps a markdown table, and a test shows both renderings carry the same facts from the same data.
- [ ] AC6: `rendered-markdown` lists the non-done specs grouped by next action, followed by the `/{project}:target` prompt, and `framework/commands/status.md` no longer asks the model to compose either.
- [ ] AC13: The tool result `/{project}:status` reads on the model path carries the view without `dashboard`'s structured payload, and `/{project}:review --all` still reads `specs[].status`.
- [ ] AC21: The tools that `framework/commands/target.md`'s steps name set `_meta["anthropic/alwaysLoad"]`, and no other `ductus` tool does. The set is derived from that file, and a test fails when a tool named there lacks the flag or one not named there has it.
- [ ] AC29: Claude Code's installed copy of every command other than `/{project}:target` loads the `ductus` tools its steps name, and those the sections it cites invoke, in one tool-search call, through a line the generator writes from the command's files. The command sources and other agents' copies are unchanged, and `ductus exec` dispatches no primitive because of that line.
- [ ] AC23: Each `ductus` tool's loaded schema size, its description plus input schema in characters, is recorded in `research.md` with the total across all tools.
- [ ] AC7: Every primitive whose result grows with the corpus bounds or pages its result, and a test drives each past its bound on a generated fixture corpus; `read-tasks` is one.
- [ ] AC24: A view that would exceed the read size collapses its `done (spec is complete)` rows to a count line, then cuts the rows past the bound and ends with a line counting the specs not shown. A test drives a generated fixture corpus past the bound on both renderings.
- [ ] AC8: Each command `/ductus` installs has its cost recorded in `specs/080-deterministic-runtime-efficiency/research.md`, with method, units, population and date.
- [ ] AC14: The result sizes recorded in `research.md` come from a script that runs each primitive on a generated fixture corpus and calls no model.
- [ ] AC15: The time and token figures recorded in `research.md` come from an on-demand harness that drives `claude -p` with stream-json output on a fixture project. No CI job runs it, and its first run records whether `claude -p` executes a custom slash command.

## Applicable Rules

- `BE-PAYLOAD-001` — bounded response payload size

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which agents besides Claude Code can answer a typed command without a model turn, and how?** Claude Code and Pi, both in this spec (owner, 2026-11-08).
  - **Claude Code:** a `UserPromptExpansion` hook whose block reason is shown and added to no context; run twice on 2.1.319.
  - **Pi:** an extension's `registerCommand` handler, showing output through `sendMessage` with `display: true` and `triggerTurn: false`, per pi 1.0.4's extension types. Whether that message reaches the agent's context is untested.
  - **OpenCode 1.17.8:** through a TUI plugin's slash command whose handler runs local code, verified by a run (`research.md`). Its server-side `command.execute.before` hook cannot skip the model. OpenCode keeps the model path all the same, as its scenario `opencode-keeps-the-model-path` records (owner, 2026-11-09).
  - **Antigravity:** from `agy` 1.4.11, through a `PreInvocation` hook returning `errorMessage` with `terminationBehavior: "terminate"`. The installed 1.0.9 lacks it, and whether the hook sees the raw command name rather than the expanded skill is unknown. Antigravity keeps the model path, as its scenario `antigravity-keeps-the-model-path` records without a run (owner, 2026-11-09).
  - **Deferral:** OpenCode and Antigravity were each deferred to a scenario under this spec. Both scenarios settle it, and each defers a model-free path to its own trigger.
  - **Auggie:** its `PromptSubmit` hook cannot block and its commands are prompt templates, so it keeps the model path.
  - **Sources:** the OpenCode and `agy` binaries' strings; `agy changelog`; opencode.ai's plugin docs and its issue #28292; antigravity.google's hooks docs; docs.augmentcode.com's hooks, custom-commands and plugins pages.
- **Does a session started below the project root get the view?** No: sessions are expected to start at the project root (owner, 2026-11-09).
  - **Observed:** on Claude Code 2.1.319, a session started in `runtime/` had `CLAUDE_PROJECT_DIR` set to `runtime/`, so the hook exited 127 without blocking. The model path then had no `ductus` server, since `.mcp.json` names the pointer by a relative path (`research.md`, Verifying runs).
  - **Set aside:** making every command work from any start directory. It would change spec 031's MCP registration for each agent and migrate adopters' committed configs, while no other command supports a start below the root today.
- **Does `UserPromptExpansion` fire when the agent invokes the command itself?** No: it answers only a command the user types, and an agent-invoked one takes the model path (owner, 2026-11-08). Claude Code's hooks reference says the event runs "when a user-typed command expands into a prompt". On 2026-11-08, with the hook matched to `ductus:status`, a typed `/ductus:status` was blocked and one invoked through the Skill tool loaded unblocked.
- **Should a compact form of the view reach the agent's context, and can a blocking hook add context?** Yes, wherever the host can add it without a turn (owner, 2026-11-08).
  - **Size:** the target line plus the non-done specs grouped by status came to 334 characters, against the 38,815-character `dashboard` result the agent otherwise reads on its first follow-up.
  - **Claude Code cannot:** on 2.1.319 a blocking `UserPromptExpansion` hook's `additionalContext` reached no context and was not recorded in the transcript. Tested on 2026-11-08 with a marker string returned beside the block. The agent therefore calls `dashboard` when it needs the data.
  - **Pi is untested:** whether its displayed message reaches the agent's context is settled by the run that verifies the Pi path.
- **What bound do primitive results keep?** The configured read size, `[artifacts] read-size-bytes`, 50,000 bytes by default; a larger result is paged (owner, 2026-11-08).
  - **Claude Code's limits:** its MCP docs put a warning above 10,000 tokens and a 25,000-token default cap (`MAX_MCP_OUTPUT_TOKENS`). A text result over 50,000 characters is saved to a file unless the tool declares `_meta["anthropic/maxResultSizeChars"]`, which allows up to 500,000.
  - **Why this figure:** a byte bound covers that character threshold. In the recorded calls a 53.4 KB `read-tasks` result was saved to a file, so it was under the token cap, while a 67,181-character one was refused for exceeding it.
  - **What it costs now:** at this bound only `read-tasks` had exceeded it.
  - **Precedent and impact:** `framework/commands/review.md` already moves its results' lists to a `lists-file` past this size, a paging pattern the plan can reuse. Spec 063 documents the key as the artifact read size, so widening what it governs is an impact on 063.
  - **Rejected alternatives:**
    - About 25,000 characters, to stay under the warning: eight primitives had exceeded that, so paging would start much sooner.
    - Raising the limit through `maxResultSizeChars`: it bounds nothing, the model still reads the whole result, and it does nothing on other hosts.
    - A separate setting with the same default: a second key and its validation for the same meaning.
- **Does the status path keep `dashboard`'s structured payload?** No: on the model path it reads a view-only result, and `/{project}:review --all` keeps the full payload (owner, 2026-11-08).
  - **Size:** the model read the whole 38,815-character result, of which the view was 9,319 characters, 76% less.
  - **Consumers:** among the command sources only `framework/commands/review.md`'s `--all` enumeration reads the payload; `clarify.md` and `target.md` mention the dashboard only in prose.
  - **Constraint for the plan:** `ductus exec` dispatches a backticked primitive with no per-step arguments, so a flag on `dashboard` would not reach an exec-driven status, and a separate argument-free primitive would.
- **How is a command's cost measured reproducibly?** Split by what needs a model (owner, 2026-11-08).
  - **Sizes:** measured by a script on a generated fixture corpus, the machinery AC7's test needs anyway, so they run anywhere without a model.
  - **Time and tokens:** from an on-demand `claude -p` harness. CI never runs it, because a MUST never requires CI and each run costs a model call.
  - **Transcripts:** supporting evidence only. They are one contributor's state, Claude Code-only, and removed after `cleanupPeriodDays`, 30 days by default.
  - **Verified by the harness's first run:** `claude -p` executes a custom slash command, and a local `UserPromptExpansion` hook answers a `-p` prompt with no model turn (`research.md`).
  - **Which commands:** each command `/ductus` installs. That leaves out the maintainer-only `audit.md`, which reaches no adopter, and takes in `configure`, whose source is `framework/bootstrap/configure/{key}.md` (owner, 2026-11-09).
- **Does Claude Code's block presentation read acceptably?** Yes, with the typed command's echo suppressed (owner, 2026-11-08).
  - **What it looks like:** a block shows as a `level: warning` system message, prefixed "UserPromptExpansion operation blocked by hook:".
  - **The echo:** it ended with "Original prompt: /ductus:status" until the hook returned `suppressOriginalPrompt: true`, which `UserPromptExpansion` honored on 2.1.319 (2026-11-08).
  - **No alternative:** no hook output answers a command without blocking it. `MessageDisplay` transforms only assistant text, and a `systemMessage` without a block lets the prompt reach the model.
- **How does the view read as plain text without growing what the model path re-types?** It is rendered twice from the same data: fixed-width text for the hook path, markdown for the model path, and both put the summary one item per line (owner, 2026-11-08).
  - **Why two renderings:** the table's content read acceptably, but its markdown syntax was noise in a terminal that renders none. The summary's 1,167-character scenario-question line did not read.
  - **Sizes:** measured on the 78-spec view of 2026-11-08, the fixed-width text was 15,477 characters (widest line 174) against the markdown's 9,319. That costs the hook path nothing, but the model path would have re-typed 6,158 more characters, roughly 16–18 s at the measured typing rate.
  - **Rejected alternatives:**
    - One fixed-width format on both paths, inside a code block: the model path would pay that cost.
    - Padding the markdown table: it grew the 77-spec view by 7,065 characters to rows 185 wide.
    - Fewer columns or collapsed `done` rows: these changed content that already read acceptably.
- **What does a Claude Code release older than the `UserPromptExpansion` event do with the hook entry?** It depends on the release, so `/{project}:configure` gates the hook on 2.1.101 (owner, 2026-11-08).
  - **Before 2.1.101:** an unrecognized hook event made Claude Code ignore the entire settings file. The changelog's 2.1.101 entry reads "an unrecognized hook event name in `settings.json` no longer causes the entire file to be ignored"; that `settings.local.json` loads the same way is an assumption.
  - **From 2.1.101 on:** only the entry is ignored, and `/{project}:status` takes the model path.
  - **The event's own release:** unknown, since the changelog never names `UserPromptExpansion`, and `ductus` stated no minimum Claude Code version.
  - **Rejected alternatives:**
    - Documenting a minimum version: it would depend on someone remembering it (§design-principles).
    - Writing the hook unconditionally: it would silently cost a pre-2.1.101 user every permission.
    - A file of its own: Claude Code reads only user, project and local settings, and the project file is the team's.
- **Which `ductus` tools set `anthropic/alwaysLoad`?** The tools `/{project}:target`'s steps name, with every other command batching its own tools into one search (owner, 2026-11-08). Revisited with the measured schemas, and kept (owner, 2026-11-09).
  - **The revisit's figures:** 70 tools load 135,932 characters, description plus compact input schema, by the release build's `tools/list` over stdio on 2026-11-09, `research.md`'s method. Target's seven are 11,308 of them, and no description exceeds the 4,096-character upfront cap. In task 17's round a command's search cost a model step, 1.51 to 1.70 s, only on status's model path, log, specify and configure, where it goes out alone. The other ten sent it beside `resolve-session` or another call, and their next message followed in 0.00 to 0.38 s.
  - **Rejected on revisit:** always-loading all 70, which saves about 1.5 s on those four commands for 124,624 more characters on every model request of every session; and loading none, which saves 11,308 characters but leaves target and the ten with a deferred first call. A deferred tool can go out in the same message as the search that loads it, as fold's `check-unfolded-specs` did, but where a command's first call was deferred the round's search went out alone in 4 of 5 runs.
  - **Cost per search:** a `ToolSearch` returned in 0.01 s (median of 353 calls) but cost an extra model step, 1.0–1.7 s in the 10 `/{project}:status` runs. 100 of this repository's sessions to 2026-11-08 made 463 `ductus` loads, a median of 4 per session.
  - **What a load adds:** a loaded tool adds its description and input schema to context, 132,846 characters across all 68 tools. The output schemas (228,054 characters) never reach the model.
  - **The chosen sets:** target's seven tools add 11,039 characters to a session that would not load them. With per-command batching, the loads drop to 356, attributing each load to the command running at that moment, typed or Skill-invoked.
  - **What stays deferred:** 250 of the 463 loads asked for a tool outside the running command's steps — mostly direct primitive use during framework development, which overstates the share for an adopter.
  - **Rejected alternatives:**
    - Always-loading all 68 tools: zero loads, but 132,846 characters in every session for as long as the flag is set, shrinking only as the schemas do. That is the context cost this work exists to reduce.
    - A measured list of the 12 most-loaded tools: 348 loads, but a hand-kept list drifts (§design-principles).
  - **Constraint:** Claude Code cuts the description of a tool loaded upfront at 4,096 characters, against 16,384 through tool search; the largest `ductus` description was 3,579.
  - **Constraint for the plan:** the batching line goes only into Claude Code's generated copies. `ductus exec` dispatches every backticked primitive name in a command's `## Instructions` with no arguments, so the line must not backtick primitive names where the walker reads them.
