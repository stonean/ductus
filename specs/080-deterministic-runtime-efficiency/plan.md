# 080 — Deterministic runtime efficiency Plan

Implements [080 — Deterministic runtime efficiency](spec.md).

<!-- This plan records the design as it currently stands, in the sections below
     and no others — they are its design record (§plan-phase). When
     implementation changes a decision, edit that decision's entry to say what
     is now true; never append a note that it changed, because git history
     already keeps what the plan said when. Everything else has a home that is
     not the plan: verification evidence in the commit that lands the task,
     contributor knowledge where §drift-prevention's "Shared knowledge stays
     in git" routes it, a finding's decision where the finding was
     dispositioned, and handoff notes on the pending task in tasks.md. `/{project}:analyze`
     reports any other `##` section as outside the design record. -->

## Overview

Measurement comes first. A generated fixture corpus and a model-free size script settle which primitives grow with the corpus, and so which ones AC7 bounds, before anything is bounded. The pipeline view then gets one model and two renderings, and a new argument-free primitive, `render-status`, serves it to the model path. A non-primitive `ductus hook status` surface serves it to Claude Code's `UserPromptExpansion` hook, and a new primitive, `merge-status-hook`, installs that hook from `/{project}:configure` and `/ductus`. Tool loading is derived from the commands' parsed steps in one runtime function. The MCP server uses that function to flag `/{project}:target`'s tools always-loaded, and `apply-manifest` uses it to write a batched `ToolSearch` line into Claude Code's command copies. Pi gets a registered command in the bridge. OpenCode and Antigravity each get a scenario. A `claude -p` harness then records every command's cost, and that record is the input to each command's review.

## Technical Decisions

### A generated corpus and a size script measure before anything is bounded

`runtime/tests/common/corpus.rs` generates a git-initialized project of N specs. It scales both the spec count and each spec's size (criteria, tasks, scenarios), because `read-tasks` grows with one file and `dashboard` with the count. The AC7 and AC24 tests use it, and so does the size script.

`scripts/measure-result-sizes.sh` runs `runtime/tests/result_sizes.rs`, an `#[ignore]`d test that CI never runs, over corpora of two sizes (AC14):

- It calls every name in `PRIMITIVE_REGISTRY` (`runtime/src/schema/registry.rs`) and records the characters of each result's text at each size.
- A primitive it cannot run on a fixture, such as `fetch-archive`, which needs the network, gets a *not measured* row with its reason, never an omitted row (§design-principles).
- It records each tool's loaded schema from `ductus mcp` `tools/list` over stdio as description plus input schema in characters, with the total (AC23).

A primitive **grows with the corpus** when its result at the larger corpus exceeds its result at the smaller one. That measured set is AC7's population, so no hand-kept enumeration exists. `research.md` records it, and carries the handoff's three method traps:

- sizes are characters of result text, not `json.dumps` of the content;
- Skill-invoked commands leave no `<command-name>` tag in a transcript;
- output schemas never reach the model.

### One view model, two renderings, one bound

The rendering in `dashboard.rs` (`render_markdown` and its helpers, `runtime/src/primitives/dashboard.rs:127-460`) moves to a new module, `runtime/src/primitives/pipeline_view.rs`. The module builds one model and renders it two ways (AC17, AC19). The model holds the target line, notices, rows, summary items, next-action groups and the closing prompt.

- **Markdown:** the existing table, with the target's row in bold.
- **Text:** each column padded to its widest cell plus two spaces, with no pipes, separator row or emphasis; the target's row is marked by a leading `→`.

Both renderings put the summary one item per line (AC18). `Counts:`, `Tags:`, `Reopenings:` and `Inbox:` are labelled lines. The scenario-question callout becomes a header line followed by one `{spec}: {scenario}` line per scenario. Its `Run /{project}:target …` hint moves to the end. A `Next actions:` block follows the summary, with one line per next action listing its non-done specs, excluding the target and `done (spec is complete)`. The block ends with the single `Run /{project}:target <feature> to select one.` line (AC6). The other callouts keep their own guidance on one line each.

**Bound (AC24).** Each rendering is measured in bytes against `resolve_threshold`, the read size `bounded_lists.rs:23` already reads. When it is over:

1. the `done (spec is complete)` rows collapse to one `done (spec is complete): N spec(s)` line;
2. if it is still over, the rows past the bound are cut, and a final line counts the specs not shown.

The AC19 test renders one model both ways and compares the facts each carries. The AC24 test drives a generated corpus past the bound on both renderings.

### `render-status` serves the view, and `dashboard` stops carrying it

`render-status` is a new, argument-free primitive. Its result is `{rendered-markdown, project}` (AC13). It takes no arguments because `ductus exec` dispatches a step's backticked primitive with none (spec Resolved Questions, constraint for the plan; `runtime/src/parser/mod.rs:18-19`). `project` is there for Pi's registered command.

`framework/commands/status.md` changes three ways:

- step 1 invokes `render-status`;
- step 2 emits `rendered-markdown`;
- step 3, which composed the non-done list and the prompt, is removed.

`dashboard` drops `rendered-markdown`. Once status reads `render-status`, nothing reads it: `/{project}:review --all` reads `specs[].status`, and `framework/commands/target.md` step 3 reads `specs[].slug`. An unread field is dead code (`AGENTS.md`). The rename sweep covers `framework/commands/status.md`, `runtime/src/schema/primitives.rs`, `runtime/src/schema/descriptions.rs`, `runtime/tests/measure_reopenings.rs`, and 022's `data-model.md` with its `dashboard-primitive` and `coverage-expansion-primitives` scenarios. 062's `plan.md` and 079's records are swept by meaning, since each is `in-progress` and its claims describe the view. `runtime/tests/golden/status-basic.jsonl` is re-blessed, and the bless diff is read.

### Claude Code's hook runs `ductus hook status`, a host surface, not a primitive

`hook` is a subcommand beside `mcp` and `exec`. `hook status` is its one member. It is listed in `NON_PRIMITIVE_SUBCOMMANDS` (`runtime/src/main.rs:1093-1108`) with its reason: it adapts a host's hook protocol, and the capability it adapts is the view, which `render-status` exposes on MCP (§runtime-boundary principle 5). A cli-only primitive would need one of principle 5's three exemptions, and rendering a view meets none.

- **Input.** The hook reads Claude Code's hook input from stdin and uses no field of it, so `command_args` is ignored (the spec's arguments edge case). The project root is `CLAUDE_PROJECT_DIR`, which Claude Code exports to command hooks (code.claude.com/docs/en/hooks), else the working directory. The identity comes from `CLAUDE_CODE_SESSION_ID` in the hook's environment, verified 2026-11-08.
- **Success.** It prints `{"decision":"block","reason":<text view>,"hookSpecificOutput":{"hookEventName":"UserPromptExpansion","suppressOriginalPrompt":true}}` and exits 0 (AC1, AC16). That placement of `suppressOriginalPrompt` is the prototype's, honored on 2.1.319. The hooks reference documents the field only for `UserPromptSubmit`, in the same place.
- **Failure.** It writes the error to stderr and exits 1 (AC25). The reference makes exit 2 block the expansion, and any other non-zero exit a non-blocking error after which the action goes ahead. The exit is therefore 1, never 2. *Assumption:* Claude Code shows the user a notice when a `UserPromptExpansion` hook fails; the reference does not say, so the hook task verifies it with a run.
- **Hook entry command.** `"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus" hook status`, the pointer (`framework/bootstrap/ductus.md:308`), with `ductus.exe` on Windows. The timeout is 10 s, as in the prototype, against a measured `dashboard` of at most 0.5 s. A timeout takes the failure path. *Assumptions:* `CLAUDE_PROJECT_DIR` is the directory whose `.claude/settings.local.json` registered the hook, so a session started below it still resolves the pointer; and Claude Code on Windows runs hook commands through a POSIX shell. The hook task's runs check the first.

### `merge-status-hook` installs the hook and gates it on the version

`merge-status-hook` is a new primitive that takes the settings file's path (AC2, AC20).

- **Version gate.** It runs `claude --version` itself, with a timeout, as `lint-markdown` shells out to `npx`. That works from configure's MCP call, from `/ductus`'s CLI path and under `ductus exec`, and needs no new permission entry. *Assumption:* the probe runs inside an MCP server that Claude Code spawned. Task 7 checks this, and if it fails, the host runs the probe and passes its output instead (see Trade-offs). A version of 2.1.101 or later merges the entry. An older or unreadable one returns `skipped` with its reason and writes nothing. The permissions are a separate `merge-permissions` call, so a skip leaves them intact.
- **Merge.** It works under `hooks.UserPromptExpansion` with matcher `{project}:status`, taking the project from `Host::load` (`runtime/src/host.rs:81`). It recognizes its own entry by the command string, adds the entry when absent, reports `unchanged` when present, and never edits another entry. It writes the whole file atomically and preserves every other key, as `merge-permissions` does.
- **Callers.** `framework/bootstrap/configure/claude.md` calls it after `merge-permissions`. `framework/bootstrap/ductus.md` calls it for each Claude agent once the runtime is acquired, so an earlier adopter gets the hook on its next `/ductus` with no manual step (AC26). Claude's seed gains the subcommand's CLI rule, and the seed's count of CLI subcommands is swept.

### Tool loading derives from each command's parsed steps

A command's steps **name** a tool when a served registry name occurs as a whole token in the prose of a numbered step of its parsed `## Instructions`. Backticked and bare mentions both count, since per-item primitives are written unbackticked (`AGENTS.md`). CLI-only names are excluded, since a host cannot load them. One function in `runtime/src/primitives/tool_loading.rs` computes it, and both uses call it. It gives target seven tools: `dashboard`, `derive-dependencies`, `read-spec`, `resolve-constitutions`, `resolve-feature`, `resolve-session` and `write-session`. They measure 11,000 characters today, against the spec's 11,039, which was taken at an earlier commit.

**Always loaded (AC21).** The MCP server derives the set when it starts, from the installed `target.md` that `Host::command_file_candidates("target")` (`runtime/src/host.rs:171`) finds. It sets `_meta["anthropic/alwaysLoad"]: true` on those tools. rmcp 3.9.0's `Tool.meta` serializes as `_meta`, and the server post-processes its router in `GovRuntimeServer::new` as `strip_nonstandard_formats` does (`runtime/src/mcp/server.rs:104-106`).

- *Why at run time:* the crate is published from `runtime/` alone (`.github/workflows/runtime-release.yml:495-498`), so a build cannot embed `framework/commands/target.md`. A constant held equal to the file by a test would not follow a pinned, adopter-edited `target.md`.
- *No installed copy:* when none is found, no tool is flagged, which is the pre-080 behaviour, and the server says so on stderr.
- *The test:* over a fixture holding the generated `target.md`, the flagged set equals the derivation over `framework/commands/target.md`. Every flagged tool's description is at most 4,096 characters, the upfront cut (spec Resolved Questions).
- *Verification:* a Claude Code session started in this checkout reaches target's tools without a `ToolSearch`.

**Batched (AC29).** A `ManifestEntry` gains a per-entry field, `tool-search-line`. When the field is set, `apply-manifest` inserts one line after substitution, as the last paragraph of the `## Instructions` preamble blockquote. The line names the command's step tools minus the always-loaded set, as one `select:mcp__ductus__…,…` code span. The always-loaded set is derived from the `commands/target.md` under `apply-manifest`'s source root, the staged framework. That copy equals the installed one once the install completes.

- *Why the walker ignores it:* the parser dispatches only a registry name inside a step, and treats a preamble code span as a reference (`runtime/src/parser/mod.rs:351-370`). The span's text is no registry name in any case.
- *When there is no line:* when the remainder is empty. Once status's steps name `render-status`, that is help and link, which name no tool; audit, whose `run-generator` is CLI-only; and target, whose tools are the always-loaded set.
- *Which copies:* `framework/bootstrap/ductus.md` sets the field on Claude's command rows only. Auggie shares the `claude-style` layout, so the field is keyed by agent, not by layout (AC10).
- *This repo's copies:* `scripts/gen-claude-commands.sh` builds them by driving `apply-manifest` through the release binary, with its JSON in a `mktemp -d` directory. The CLI's staging roots are the temp directory (`runtime/src/primitives/mod.rs:1876-1910`), and `generators.yml` builds the binary first (`.github/workflows/generators.yml:48-54`). This repo's copy and an adopter's therefore come from one code path. `.githooks/pre-commit` runs the generator before it resolves the binary (`.githooks/pre-commit:72-92`), so the hook resolves the binary first. The Pi pass is unchanged.
- *The test:* a flagged copy parses to the same `Procedure` as its source, and an unflagged entry's output is byte-identical to its output before this change.

**Revisited.** Once AC23's schema sizes are recorded, whether to always-load every tool goes back to the owner with those figures (spec Behavior, *Revisited*).

### Pi registers the command in the bridge

The bridge calls `render-status` once at load and registers `{project}-status` through `registerCommand`. Its handler calls `render-status` through the same server process, so it uses the same identity (the spec's Pi identity edge case). It shows `rendered-markdown` through `sendMessage` with `display: true` and `triggerTurn: false`, and shows a failure the same way (AC9). The `ExtensionAPI` slice gains `registerCommand` and `sendMessage`, per pi 1.0.4's types. The bridge stays verbatim and dependency-free, as 064's AC6 requires. It is copied by every `/ductus` with strategy `update` (`framework/bootstrap/ductus.md:471`), so an earlier Pi adopter gets the command on its next `/ductus` (AC26).

The verifying run settles three assumptions and records them in `research.md` (AC27):

- **Context.** Whether the displayed message reaches the agent's context. If it does not, and Pi can add context without a turn, the bridge adds the compact form (AC28).
- **Markdown.** Whether a displayed message renders markdown. If it does not, `render-status` gains a `format` argument whose `text` value the bridge passes.
- **Templates.** Whether the registered command runs instead of the `{project}-status` prompt template. If the template wins, `/ductus` stops installing it.

### Corpus-growing results are bounded through `bounded_lists`

Each primitive in `research.md`'s grows-set returns through `bounded_lists::place` (`runtime/src/primitives/bounded_lists.rs:53`) (AC7). The result is inline while it fits `read-size-bytes`. Past that, its lists go to the file `lists-file` names, with their counts inline, as with `compute-review-scope` (`runtime/src/schema/primitives.rs:752`).

- **Known members.** `read-tasks`, which crossed both host limits, and `dashboard`: 38,815 characters at 78 specs, so past 50,000 near 100.
- **Consumers.** Each command that reads one of the lists gains a branch for the lists file: `framework/commands/implement.md` for `read-tasks`, and `framework/commands/review.md` `--all` and `framework/commands/target.md` step 3 for `dashboard`. The exec walker's own `read-tasks` call (`runtime/src/interpreter/payload.rs:1054`) uses the unbounded computation, not the tool's result.
- **Tests.** Each member has a test that drives a generated corpus past the bound and reads the whole list back from the file.

### A `claude -p` harness records each command's cost

`scripts/measure-command-cost.py` drives `claude -p --output-format stream-json --verbose` on a fixture project: a generated corpus, the generated Claude copies, `.mcp.json`, and the pointer to the release binary (AC15).

- **What it records per run:** time to first visible output, time to result, model output tokens from the result event's usage, and the largest tool result in characters of result text.
- **When it runs:** on demand only. No workflow references it, since a MUST never requires CI and each run is a model call.
- **Its first run** records whether `claude -p` executes a custom slash command, and how a local `UserPromptExpansion` hook treats a `-p` prompt.
- **Why Python:** stream parsing, and dropping the prototype's `jq` dependency is part of what this spec does.

It runs one round after the view, the hook and tool loading land. That round records every command's cost in `research.md` with its method, units, population and date (AC8). There is no separate baseline round: the spec's transcript figures are status's before-figure. Each command's review then reads that record. `research.md` names, per command, the reduction that applies or why none does, and each reduction becomes a scenario through `/{project}:amend` (spec Behavior, *One case per command*).

### Auggie, OpenCode and Antigravity

None of the three gets the hook, the batching line or a bridge change (AC10). OpenCode's scenario and Antigravity's are recorded through `/{project}:amend` after a run on a version that has the mechanism: an OpenCode TUI plugin's slash command, and `agy` 1.4.11 or later with a `PreInvocation` hook (AC11). Each scenario records its outcome, and the deferral it settles, under Resolved Questions with its trigger. An open question there would hold 080 short of `done`.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/tests/common/corpus.rs` | Create | Generated fixture corpus |
| `runtime/tests/result_sizes.rs` | Create | Ignored size measurement the script runs |
| `scripts/measure-result-sizes.sh` | Create | Model-free result-size and schema-size script |
| `scripts/measure-command-cost.py` | Create | On-demand `claude -p` cost harness |
| `specs/080-deterministic-runtime-efficiency/research.md` | Create | Measurements, method traps, verifying-run records |
| `runtime/src/primitives/pipeline_view.rs` | Create | View model, two renderings, bound |
| `runtime/src/primitives/render_status.rs` | Create | `render-status` primitive |
| `runtime/src/primitives/merge_status_hook.rs` | Create | `merge-status-hook` primitive |
| `runtime/src/primitives/tool_loading.rs` | Create | Step-tool derivation and the batching line |
| `runtime/src/hook.rs` | Create | `ductus hook status` surface |
| `runtime/src/primitives/dashboard.rs` | Modify | Rendering moves out; `rendered-markdown` dropped; bounded |
| `runtime/src/primitives/read_tasks.rs` | Modify | Bounded result |
| `runtime/src/primitives/apply_manifest.rs` | Modify | `tool-search-line` entry field |
| `runtime/src/primitives/mod.rs` | Modify | Module registration |
| `runtime/src/mcp/server.rs` | Modify | New tools; `anthropic/alwaysLoad` |
| `runtime/src/schema/registry.rs` | Modify | Registry entries |
| `runtime/src/schema/descriptions.rs` | Modify | Descriptions |
| `runtime/src/schema/primitives.rs` | Modify | Args and results; `ManifestEntry` field |
| `runtime/src/interpreter/mod.rs` | Modify | Dispatch arms |
| `runtime/src/main.rs` | Modify | Subcommands; `NON_PRIMITIVE_SUBCOMMANDS` |
| `runtime/tests/mcp.rs` | Modify | Always-load test |
| `runtime/tests/golden/status-basic.jsonl` | Modify | Re-blessed status golden |
| `framework/runtime-tools.txt` | Modify | Two new tools |
| `framework/commands/status.md` | Modify | Reads `render-status`; step 3 removed |
| `framework/commands/implement.md` | Modify | `read-tasks` lists-file branch |
| `framework/commands/review.md` | Modify | `--all` lists-file branch |
| `framework/commands/target.md` | Modify | Not-found enumeration lists-file branch |
| `framework/bootstrap/configure/claude.md` | Modify | Calls `merge-status-hook` |
| `framework/bootstrap/configure/auggie.md` | Modify | Regenerated allow block |
| `framework/bootstrap/configure/opencode.md` | Modify | Regenerated allow block |
| `framework/bootstrap/configure/antigravity.md` | Modify | Regenerated allow block |
| `framework/bootstrap/ductus.md` | Modify | Claude rows' field; hook install; seed rule |
| `framework/bootstrap/pi/ductus-bridge.ts` | Modify | Registered status command |
| `scripts/gen-claude-commands.sh` | Modify | Claude pass drives `apply-manifest` |
| `.githooks/pre-commit` | Modify | Resolves the binary before the generator runs |
| `scripts/tests/pi-bridge-harness.mjs` | Modify | Registered-command cases |
| `specs/022-deterministic-runtime/data-model.md` | Modify | `rendered-markdown` moves to `render-status` |
| `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md` | Modify | Same, with the back-link to 080 |
| `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md` | Modify | Same rename |
| `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` | Modify | Back-link: the read size bounds results |
| `specs/064-pi-host-support/spec.md` | Modify | Back-link: the bridge's registered command |

## Data Model

[data-model.md](data-model.md) defines:

- `render-status`'s result;
- the view's line grammar, its collapse line and its cut line;
- the hook's input, output and exit statuses;
- the `settings.local.json` hook entry and how it is recognized;
- `merge-status-hook`'s arguments and result;
- the `ManifestEntry` field and the batching line;
- the `_meta` key;
- how a bounded result is split.

## Trade-offs

- **Paging the view:** rejected, because the hook shows one message (spec Edge Cases). It collapses and cuts instead.
- **A `format` flag on `dashboard`:** rejected, because `ductus exec` passes no arguments (the spec's constraint), so the flag could not reach an exec-driven status.
- **Keeping `rendered-markdown` on `dashboard`:** rejected, because nothing would read it.
- **A cli-only `status-hook` primitive:** rejected, because principle 5's exemptions do not cover it.
- **A shell or `jq` hook:** rejected by the spec. The hook's command is the binary.
- **Host-run `claude --version` passed to the primitive:** this is the fallback, used only if the probe cannot run inside the MCP server. Otherwise it is rejected: it would need a permission entry in two seeds, and it would not run under `ductus exec`.
- **Always-load as a test-pinned constant:** rejected, because a pinned, edited `target.md` would not move it. Embedding at build time is impossible for a crate published from `runtime/` alone.
- **Always-loading every tool, or a measured top-12 list:** rejected by the owner (spec Resolved Questions). Every tool is revisited after AC23.
- **Steps only:** the derivation reads steps, as the spec says. A tool named only outside them still loads through its own search. Today that is prune's `prune-project` and `classify-spec-edit`; consolidate's `check-corpus-links`, `check-orphaned-references` and `classify-spec-edit`; fold's `check-orphaned-references`; amend's `label-criteria`; clarify's `append-question`; and status's `resolve-references`.
- **A batching line in the command sources:** rejected, because AC29 keeps the sources and other agents' copies unchanged.
- **Driving `apply-manifest` from the generator:** costs the generator a built binary, which `generators.yml` already builds. It buys one code path for this repo's copies and adopters'.
- **Pi's load-time `render-status` call:** costs one view computation per Pi session start, to learn the namespace. The alternatives were parsing `.ductus/config.toml` in TypeScript, or substituting `{project}` into a bridge 064 requires to be verbatim.
- **No baseline round of the harness:** status's before-figures come from transcripts (spec Motivation). For the other commands there is no before-figure, so a reduction found later is measured against the round 080 records.
- **The settings entry's path:** it names the repository pointer and assumes `CLAUDE_PROJECT_DIR` is the registering root. A per-machine absolute path, as in the prototype, would not travel.

## Open Questions Resolved

- **`ductus exec` passes no per-step arguments.** The status path is the argument-free `render-status`, not a `dashboard` flag.
- **The batched-load line must not backtick primitive names where the walker reads them.** The line sits in the Instructions preamble. Its one code span is `select:mcp__ductus__…`, which is no registry name, and the AC29 test checks that the generated copy parses to the source's `Procedure`.

## Cross-spec impact

- **022 — deterministic runtime**, `in-progress`, owner of the runtime core. It owes the new `hook` surface, `render-status`, `merge-status-hook`, the `alwaysLoad` `_meta`, `dashboard`'s dropped field and the bounded results. Discharged by the edits to its data model and its `dashboard-primitive` scenario, which links back to 080.
- **063 — oversized artifacts warn with a fix**, `in-progress`. `read-size-bytes` now also bounds primitive results and the view. Discharged by a signpost in its `spec.md` linking 080.
- **064 — Pi host support**, `done`. The bridge registers a command. A blockquote signpost in its `spec.md` is a correction and does not reopen it. Its D1 command surface changes only if the template loses to the command. If `/ductus` then stops installing `{project}-status`, 064's criteria change, and `classify-spec-edit` decides the reopen.
