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

A primitive **grows with the corpus** when its result at the larger corpus is at least twice its result at the smaller one and at least 200 characters more, as `runtime/tests/result_sizes.rs` has it. That measured set is AC7's population, so no hand-kept enumeration exists. `research.md` records it, and carries the handoff's three method traps:

- sizes are characters of result text, not `json.dumps` of the content;
- Skill-invoked commands leave no `<command-name>` tag in a transcript;
- output schemas never reach the model.

### One view model, two renderings, one bound

`runtime/src/primitives/pipeline_view.rs` holds the view: one model, built from the data `dashboard.rs` loads, rendered two ways (AC17, AC19). The model holds the target line, notices, rows, summary items, next-action groups and the closing prompt.

- **Markdown:** the existing table, with the target's row in bold.
- **Text:** each column padded to its widest cell plus two spaces, with no pipes, separator row or emphasis; the target's row is marked by a leading `→`.

The two renderings differ only in the table and its emphasis, and every other line is the same in both. Both put the summary one item per line (AC18), each item a labelled line, with the reopenings line keeping the form spec 079 gave it. The scenario-question callout becomes a header line followed by one indented `{spec} · {scenario}` line per unresolved question, so a scenario with three questions has three lines. The question's text stays out of the view, because the model path re-types it: on this repository's 18 questions it took the view from 10,904 characters to 16,434 (owner, 2026-11-08). The view model carries each question's scenario beside `DashboardSpec`, whose payload is unchanged. The callout's `Run /{project}:target …` hint moves to the end. A `Next actions:` block follows the summary, with one line per next action listing its non-done specs, excluding the target and `done (spec is complete)`. The block ends with the single `Run /{project}:target <feature> to select one.` line (AC6). The other callouts keep their own guidance on one line each. Every value the view takes from a file name or from frontmatter, a spec's directory name, a scenario's file name and the namespace, which falls back to the project directory's name (`Host::load`, `runtime/src/host.rs`), included, passes through `view_text` (`runtime/src/primitives/pipeline_view.rs`) in both renderings. It turns each tab, and each character `is_line_hazard` names (`runtime/src/primitives/mod.rs`), into a space, so no value can add a view line, carry an escape sequence or shift a fixed-width column (`BE-INPUT-011`, as `specs/025-rule-opt-out/analysis.md` applied it to the view). `flatten_line` itself keeps the tab, since 058's decision keys normalize through it.

**Bound (AC24).** Each surface measures the result it serializes, `render-status`'s `{rendered-markdown, project, form}`, with `lists-file` past the bound, and the hook's block response, in bytes against `resolve_threshold`, the read size `bounded_lists.rs:23` already reads. A result exactly at the bound fits. When it is over, each step below applies in turn until it fits, and after each later step the rows, then the next-action names, are fitted again (owner, 2026-11-10):

1. the `done (spec is complete)` rows collapse to one `done (spec is complete): N spec(s)` line;
2. the rows past the bound are cut, and a line counts the specs not shown;
3. with no row left, the `Next actions:` block keeps the leading names that fit. A group cut short ends `… N more`, and a group with none left reads `{next-action}: N spec(s)`. A name costs about a third of a row, so at 1,701 planned specs this names 1,498 specs where keeping rows would name 515;
4. each other list collapses to its labelled count, marked as not shown, in this order: the References block, Disabled rule files, Unknown config keys, Tags, Folds, Recovery, Blocked, the scenario-question lines, the out-of-set statuses in `Counts:`, the `Next actions:` block's count lines, then the Notice and Unreadable lines.

The scenario-question lines are cut like the rest, so 046's AC24, which listed them with no cap, now excepts the view past the read size, and 046 reopens for it (owner, 2026-11-10). Without that exception the view would pass the read size at about 660 unresolved questions on this repository's slugs.

What is left is the floor: the target line, the table header, the `done (spec is complete)` line, the count lines, the reopenings and inbox lines, the `Scenario questions:` header, one `not shown` line for each collapsed list and the prompt. On the model path each `not shown` line also names the list file. A read size below it gets the floor, over the bound, as `bounded_lists::place` returns its file form without measuring it again.

**What the agent gets past the bound.** On the model path, `render-status` writes the whole view's lines to a list file through `bounded_lists::place`, under the key `view`, one line per entry. The result then carries `form: file` and `lists-file` beside the cut view, and every not-shown line names that file, so the agent can read every fact the cut left out (owner, 2026-11-10). At the rich 1,701-spec corpus that file is 279,572 bytes, six reads, where `dashboard`'s list file, the other source, is 791,817 bytes and lacks the references readout. The key is not `rendered-markdown`, so the exec walker's `restore_lists`, which merges every key of the file into the payload, never replaces the cut view. The hook path writes no file, since it shows one message to a person and the view never reaches the model there.

The AC19 test renders one model both ways and compares the facts each carries. The AC24 test drives a generated corpus past the bound on both renderings.

### `render-status` serves the view, and `dashboard` stops carrying it

`render-status` is a new, argument-free primitive. Its result is `{rendered-markdown, project, form}`, with `lists-file` past the bound (AC13). It takes no arguments because `ductus exec` dispatches a step's backticked primitive with none (spec Resolved Questions, constraint for the plan; `runtime/src/parser/mod.rs:18-19`). `project` is there for Pi's bridge, which reads it on a runtime that serves no `render-help`. It is the namespace as the view's hints spell it, through `view_text`, so the bridge's respelling matches each hint.

`framework/commands/status.md` changes three ways:

- step 1 invokes `render-status`;
- step 2 emits `rendered-markdown`;
- step 3, which composed the non-done list and the prompt, is removed.

`dashboard` drops `rendered-markdown`. Once status reads `render-status`, nothing reads it: `/{project}:review --all` reads `specs[].status`, and `framework/commands/target.md` step 3 reads `specs[].slug`. An unread field is dead code (`AGENTS.md`). The rename sweep covers `framework/commands/status.md`, `runtime/src/schema/primitives.rs`, `runtime/src/schema/descriptions.rs`, `runtime/tests/measure_reopenings.rs`, and 022's `data-model.md` with its `dashboard-primitive` and `coverage-expansion-primitives` scenarios. 062's `plan.md` and 079's records are swept by meaning, since their claims describe the view. `runtime/tests/golden/status-basic.jsonl` is re-blessed, and the bless diff is read.

**Compatibility (`BE-COMPAT-001`).** Three result changes here are breaking: `dashboard` drops `rendered-markdown`, `read-spec` drops `review` and `analyze`, and each bounded result stops carrying its lists inline past the read size. 022's data model (*Versioning of these schemas*) takes a breaking schema change before 1.0 as a minor version bump, as 0.53.0's removals were, and the lockstep pin of §runtime-boundary ships each runtime with the command files that read it. So the release that ships this spec is 0.60.0, not a patch release. Every other change is additive: four new tools, `_meta` on target's tools in `tools/list`, new optional arguments (`canonical`, `rules`, `verification`, `all`, `extra-files` and the manifest entry's `tool-search-line`), and new result fields, among them `discover-rule-files`' `unreadable` and `merge-permissions`' `revoked` and `directories-added`.

### Claude Code's hook runs `ductus hook status`, a host surface, not a primitive

`hook` is a subcommand beside `mcp` and `exec`, with two members, `hook status` and `hook help` (*Help answers through the status surfaces*). It is listed in `NON_PRIMITIVE_SUBCOMMANDS` (`runtime/src/main.rs:1093-1108`) with its reason: it adapts a host's hook protocol, and the capability it adapts is the view, which `render-status` exposes on MCP (§runtime-boundary principle 5). A cli-only primitive would need one of principle 5's three exemptions, and rendering a view meets none.

- **Input.** The hook reads Claude Code's hook input from stdin and uses no field of it, so `command_args` is ignored (the spec's arguments edge case). The project root is `CLAUDE_PROJECT_DIR`, which Claude Code exports to command hooks (code.claude.com/docs/en/hooks), else the working directory. The variable is optional, a directory path, read once when the hook starts, empty counting as unset (`hook::project_root`, which `runtime/src/main.rs` gives the variable's value). Its fallback is the process's own working directory, so no constant holds it. A root that cannot render takes the failure path, never the fallback. The event name and the variable's name are `HOOK_EVENT` and `PROJECT_DIR_VAR` in `runtime/src/hook.rs`, which `merge-status-hook` reads rather than restating (`CFG-CONST-001`). The identity comes from `CLAUDE_CODE_SESSION_ID` in the hook's environment, verified 2026-11-08.
- **Success.** It prints `{"decision":"block","reason":<text view>,"hookSpecificOutput":{"hookEventName":"UserPromptExpansion","suppressOriginalPrompt":true}}` and exits 0 (AC1, AC16). That placement of `suppressOriginalPrompt` is the prototype's, honored on 2.1.319. The hooks reference documents the field only for `UserPromptSubmit`, in the same place.
- **Failure.** It writes the error to stderr and exits 1 (AC25). The reference makes exit 2 block the expansion, and any other non-zero exit a non-blocking error after which the action goes ahead. The exit is therefore 1, never 2. Claude Code shows the user the failure as a `UserPromptExpansion hook error` notice and then runs the command on the model path, as `research.md`'s task 8 runs with the pointer moved aside recorded.
- **Hook entry command.** `"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus" hook status`, the pointer (`framework/bootstrap/ductus.md:308`), with `ductus.exe` on Windows. The command is fixed text from `hook_command` (`runtime/src/primitives/merge_status_hook.rs`) and holds no caller-supplied value: its one variable, `$CLAUDE_PROJECT_DIR`, sits inside double quotes (`BE-INPUT-003`). The timeout is `HOOK_TIMEOUT_SECONDS`, 10 s, in the same file, as in the prototype, against a measured `dashboard` of at most 0.5 s. A timeout takes the failure path. Sessions start at the project root (spec Resolved Questions): Claude Code sets `CLAUDE_PROJECT_DIR` to the directory the session started in, so below the root the command fails and takes the failure path, as `research.md` records. *Assumption:* Claude Code on Windows runs hook commands through a POSIX shell.

### `merge-status-hook` installs the hook and gates it on the version

`merge-status-hook` is a new primitive that takes the settings file's path (AC2, AC20), which passes 022's path screen (`validate_contained`, called from `runtime/src/primitives/merge_status_hook.rs`; 022's plan, *Path arguments: a lexical screen, then canonical containment*).

- **Version gate.** It runs `claude --version` itself under `PROBE_TIMEOUT`, 10 s (`runtime/src/primitives/merge_status_hook.rs`), as `lint-markdown` shells out to `npx`. `probe_program` passes `--version` as the one argument of an argument vector, with no caller-supplied value (`BE-INPUT-003`). No shell runs it, except that Windows' `claude.cmd` fallback runs through `cmd.exe`, as Rust's standard library runs any batch file (`command_prompt()` with `make_bat_command_line`, in 1.97.0's `library/std/src/sys/process/windows.rs`), which the fixed argument keeps safe. That works from configure's MCP call, from `/ductus`'s CLI path and under `ductus exec`, and needs no new permission entry. The probe reads the version from inside an MCP server Claude Code spawned, as `research.md` records, so the host never runs it (see Trade-offs). The probe closes its stdin, which carries the server's protocol, and tries `claude.cmd` after `claude` on Windows. A version of 2.1.101 or later merges the entry. An older or unreadable one returns `skipped` with its reason and writes nothing. The permissions are a separate `merge-permissions` call, so a skip leaves them intact.
- **Merge.** It works under `hooks.UserPromptExpansion` with two entries, matcher `{project}:status` running `hook status` and matcher `{project}:help` running `hook help`, taking the project from `Host::load` (`runtime/src/host.rs:81`). It recognizes each entry by its own command string and adds each one that is absent. It reports `created` when there was no file, `updated` when it added either entry, and `unchanged` when both were present. It never edits another entry. It writes the whole file atomically and preserves every other key, as `merge-permissions` does. A file that does not parse as JSON, or whose top level, `hooks` or `hooks.UserPromptExpansion` is not the object or array expected, is refused with an error naming the file before anything is written (`BE-INPUT-001`; `merge` in `runtime/src/primitives/merge_status_hook.rs`). It takes no lock: a concurrent run's entries can be lost, and the next run restores them, since both merges are idempotent (spec Edge Cases).
- **Callers.** `framework/bootstrap/configure/claude.md` calls it after `merge-permissions`. `framework/bootstrap/ductus.md` calls it for each Claude agent once the runtime is acquired, so an earlier adopter gets the hook on its next `/ductus` with no manual step (AC26). Claude's seed gains the subcommand's CLI rule, and the seed's count of CLI subcommands is swept.

### Tool loading derives from each command's parsed steps

A command's steps **name** a tool when a served registry name occurs as a whole token in the prose of a numbered step of its parsed `## Instructions`. Backticked and bare mentions both count, since per-item primitives are written unbackticked (`AGENTS.md`). CLI-only names are excluded, since a host cannot load them. The parser ends the section at its first sub-heading, so a command whose steps all sit under sub-headings, as amend's do, parses as legacy prose. Its steps name the tools in its whole `## Instructions` section instead, past the preamble and up to the next heading of the same or a higher level (owner, 2026-11-09). One function in `runtime/src/primitives/tool_loading.rs` computes it, and both uses call it. It gives target seven tools: `dashboard`, `derive-dependencies`, `read-spec`, `resolve-constitutions`, `resolve-feature`, `resolve-session` and `write-session`. They measure 12,028 characters in `research.md`'s *Loaded schema sizes (AC23)* table, against the spec's 11,039, which was taken at an earlier commit.

**Always loaded (AC21).** The MCP server derives the set when it starts, from the installed `target.md` that `Host::command_file_candidates("target")` (`runtime/src/host.rs:171`) finds. It sets `_meta["anthropic/alwaysLoad"]: true` on those tools. rmcp 3.9.0's `Tool.meta` serializes as `_meta`, and the server post-processes its router in `GovRuntimeServer::new` as `strip_nonstandard_formats` does (`runtime/src/mcp/server.rs:104-106`).

- *Why at run time:* the crate is published from `runtime/` alone (`.github/workflows/runtime-release.yml:495-498`), so a build cannot embed `framework/commands/target.md`. A constant held equal to the file by a test would not follow a pinned, adopter-edited `target.md`.
- *No installed copy:* when none is found, no tool is flagged, which is the pre-080 behaviour, and the server says so on stderr. It says so too for a copy that does not parse or names no tool.
- *The test:* over a fixture holding the generated `target.md`, the flagged set equals the derivation over `framework/commands/target.md`. Every flagged tool's description is at most 4,096 characters, the upfront cut (spec Resolved Questions).
- *Verification:* a Claude Code session started in this checkout reaches target's tools without a `ToolSearch`.

**Batched (AC29).** A `ManifestEntry` gains a per-entry field, `tool-search-line`. When the field is set, `apply-manifest` inserts one line after substitution, as the last paragraph of the `## Instructions` preamble blockquote. The line names the command's run tools minus the always-loaded set, as one `select:mcp__ductus__…,…` code span. The always-loaded set is derived from the `target.md` the manifest itself installs, the entry whose source ends in `commands/target.md`, so it is the copy the MCP server reads once the install completes. That holds in `/ductus`'s layout, the archive's root with sources `framework/commands/*.md`, and the generator's, `framework/` with sources `commands/*.md`, alike (owner, 2026-11-09). When that entry is pinned, `apply-manifest` skips it and the installed copy stays, so the set is derived from the copy at the entry's destination instead, which is the one the server reads. A pinned destination that is missing or does not parse gives the empty set, since the server then flags nothing (owner, 2026-11-10). Both derive through `text_step_tools`, the one function.

- *The run tools:* the step tools, and each served name the command file or a section it cites invokes (scenario `one-search-covers-referenced-sections`; owner, 2026-11-09, over every name in the cited sections, every name in the command's reference, and naming each call in a step). A section is cited when a line of the command file holds a `§` followed by its heading, or a bold span equal to its heading, matched by the rules `resolve_anchor.rs` already applies: longest heading first, ending where a word ends, outside fenced blocks and HTML comments. A line naming `.ductus/reference/<name>.md` resolves only against the references it names. Any other line resolves against its own file, then the command file, then the command's own reference, the one its text names as `.ductus/reference/<command>.md`. A cited section's own citations are followed in turn, against its own file first. Inside a cited section, a name counts only where it is written `invoke`, `invokes` or `invoking`, an optional `the`, then the backticked name. A section that names a tool without invoking it, as most do, adds nothing. Measured at 65e7acdb, the rule adds plan's `derive-boundary`, implement's `append-task` and prune's `prune-project`, 7,513 characters of schema in all, and no tool a run does not call. It needs §Validation gate in plan's reference and §Walk through tasks in implement's to be worded as invocations.
- *Where a reference is read:* `.ductus/reference/<name>.md` is read from `reference/<name>.md` beside the `commands/` directory of the `commands/target.md` entry, with the entry's substitutions applied, so both layouts resolve it alike. A named reference missing from the source root is an error naming it, never a line that silently lacks its tools, and its path passes `validate_contained` against the source root (`runtime/src/primitives/apply_manifest.rs`), so no name leaves it.
- *A call the rule misses:* a reference that calls a tool in other words costs that run one more search, as before this change.
- *The search goes out with the first call:* the line asks for the command's first `ductus` call to go in the search's own message, and says a tool can be called beside the search that loads it (scenario `the-search-goes-out-with-the-first-call`; owner, 2026-11-09, over deriving that call and naming it). It names no exception for a call that needs an earlier result, since the search's own schema reads as one. The sentence is the same in every copy.
- *Why the walker ignores it:* the parser dispatches only a registry name inside a step, and treats a preamble code span as a reference (`runtime/src/parser/mod.rs:351-370`). The span's text is no registry name in any case.
- *When there is no line:* when the remainder is empty. Once status's steps name `render-status`, that is help and link, which name no tool, and target, whose tools are the always-loaded set. Audit's `run-generator` is CLI-only, but its *Where a named section lives* paragraph invokes `read-section`, so its copy carries a line.
- *Which copies:* `framework/bootstrap/ductus.md` sets the field on Claude's command rows only. Auggie shares the `claude-style` layout, so the field is keyed by agent, not by layout (AC10).
- *This repo's copies:* `scripts/gen-claude-commands.sh` builds them by driving `apply-manifest` through `DUCTUS_BIN`, else the release binary, with its JSON in a `mktemp -d` directory. Both of its variables are optional, read once when the script starts, empty counting as unset, and listed in `docs/runtime.md`'s inventory:
  - `DUCTUS_BIN`, a path to the runtime, defaults to `DEFAULT_DUCTUS_BIN`, the checkout's `runtime/target/release/ductus`. A path that does not run, or a runtime whose `apply-manifest` lacks `tool-search-line`, stops the script, naming the path and the rebuild command.
  - `DUCTUS_GEN_TIMEOUT_SECONDS`, a whole number of seconds, defaults to `DEFAULT_GEN_TIMEOUT_SECONDS`, 120, and bounds each runtime call (`BE-TIMEOUT-001`). A value that is not a positive whole number stops the script, naming the variable.

  Each call passes its arguments as separate words, never a built command string (`BE-INPUT-003`). `--check` stages the sources and `framework/reference/` in its temp directory and runs there, since `apply-manifest` holds both roots inside the directory it runs in. `generators.yml` builds the binary first (`.github/workflows/generators.yml:48-54`), and `.githooks/pre-commit` resolves it before its generators and passes it as `DUCTUS_BIN`. This repo's copy and an adopter's therefore come from one code path. The Pi pass is unchanged.
- *The test:* a flagged copy parses to the same `Procedure` as its source, and an unflagged entry's output is byte-identical to its output before this change. Over fixtures, a tool a cited section invokes joins the line, through a `§` citation, a bold one, a named reference and a second-level citation alike; one it only names does not; and a named reference missing from the source root is an error.
- *Verification:* `scripts/measure-command-cost.py --round --only plan,status,log,specify,configure,fold --permissions adopter`. Each run makes one search. For each run whose first call is a deferred tool, the raw stream shows whether that call went in the search's own message, read as `research.md`'s *The search step* reads it.

**Revisited.** With AC23's schema sizes and task 17's search costs measured, the owner kept target's tools as the only ones loaded with the session (spec Resolved Questions, 2026-11-09).

### Pi registers the command in the bridge

The bridge learns the namespace at load from `render-help`'s `project`. That call reads the installed help copy and no spec, so a view that cannot be rendered, as with malformed spec frontmatter or a malformed read size, never stops the registration (owner, 2026-11-10). A load whose `render-help` call fails, as with a missing or malformed help copy, registers neither command, and the session notice names that error rather than the pointer. A runtime that serves no `render-help` gives the namespace through `render-status` at load, as before. The bridge registers `{project}-status` through `registerCommand`. Its handler calls `render-status` through the same server process, so it uses the same identity (the spec's Pi identity edge case), and each call is bounded by 064's `REQUEST_TIMEOUT_MS`, 120 s (`framework/bootstrap/pi/ductus-bridge.ts`). It shows `rendered-markdown` through `sendMessage` with `display: true` and `triggerTurn: false`, and shows a failure the same way (AC9). Before showing the view, it respells each `/{project}:<name>` hint as `/{project}-<name>`, the name Pi runs the command by (064's D1), so every hint names a command Pi can run (owner, 2026-11-09). The `ExtensionAPI` slice gains `registerCommand` and `sendMessage`, per pi 1.0.4's types. The bridge stays verbatim and dependency-free, as 064's AC6 requires. It is copied by every `/ductus` with strategy `update` (`framework/bootstrap/ductus.md:471`), so an earlier Pi adopter gets the command on its next `/ductus` (AC26).

The verifying run settles three assumptions and records them in `research.md` (AC27):

- **Context.** Whether the displayed message reaches the agent's context. If it does not, and Pi can add context without a turn, the bridge adds the compact form (AC28). Without a model turn to observe, the answer comes from pi's own documentation, and the record says which source it is (owner, 2026-11-10): no pi account was at hand to run a turn, so `research.md` takes it from `docs/message-types.md`.
- **Markdown.** Whether a displayed message renders markdown. If it does not, `render-status` gains a `format` argument whose `text` value the bridge passes.
- **Templates.** Whether the registered command runs instead of the `{project}-status` prompt template. If the template wins, `/ductus` stops installing it.

### Help answers through the status surfaces

The scenario `help-answers-with-no-model-turn` puts `/{project}:help` on the paths status already has, so each surface gains a second member rather than a new mechanism (owner, 2026-11-09, over rendering the guide's tables as text and over one entry with a widened matcher).

- **The guide.** A function in `runtime/src/primitives/render_help.rs` reads the installed help copy that `Host::command_file_candidates("help")` finds first, as the MCP server finds `target.md`. The guide therefore carries the project's name, as the model path's copy does, and follows a pinned or edited `help.md`. The crate cannot embed `framework/commands/help.md`, for the reason *Always loaded (AC21)* gives. The guide is the text between the first line after the `## Instructions` heading that is exactly `---` and the last such line. Each line that is wholly an HTML comment (the generator's `<!-- generated:… -->` markers) is removed, and the blank lines that leaves are collapsed. On this repo's copy that is 6,110 characters; the model path printed 6,146 in the cost round. A copy that is missing, that has no `## Instructions` heading, that holds fewer than two such lines after it, or that holds nothing but blank lines and comments between them, is an error.
- **`render-help`.** A new, argument-free primitive that returns `{guide, project}`: the guide from that function, and the namespace `Host::load` resolves, which Pi's bridge names its commands from (*Pi registers the command in the bridge*). Principle 5 puts the capability `hook help` adapts on MCP, as `render-status` serves `hook status`'s, and Pi's bridge reaches the guide through it. `help.md`'s steps name no tool, so its model path gains no `ToolSearch` line and no call.
- **`ductus hook help`.** It answers a typed `/{project}:help` as `hook status` answers status: the same input handling, the `CLAUDE_PROJECT_DIR` root, a block response with `suppressOriginalPrompt`, and exit 1 on failure. Its reason is the guide as written. The scenario asks for the guide the model path prints, so its five pipe tables (28 lines, the longest 480 characters) show as markdown source in the terminal. The `hook` group sets clap's `disable_help_subcommand`, since clap otherwise generates `ductus hook help` itself. `ductus hook --help` still prints the group's help.
- **The entry.** `merge-status-hook` merges help's entry beside status's, under the same version gate and in the same write (*`merge-status-hook` installs the hook and gates it on the version*). A matcher holding `:` is an unanchored regular expression on the command name (code.claude.com/docs/en/hooks), for both entries alike.
- **Pi.** When the runtime serves `render-help`, the bridge also registers `{project}-help`. Its handler calls `render-help` and shows the guide as a displayed message that triggers no turn, respelled as status's view is, with a failure shown the same way.
- **The agent's own invocation.** Claude Code fires `UserPromptExpansion` only for a command the user types (code.claude.com/docs/en/hooks), so a `/{project}:help` the agent invokes takes the model path, as status's does.

### Configure's canonical sets come from the runtime

The scenario `configure-types-no-permission-entries` has Claude Code's `/{project}:configure` pass `merge-permissions` no entries. In the cost round its model typed 132 of them, 3,897 characters in a 13.28 s step, and step 5's host-side `Edit` of the settings file was denied (`research.md`, R2). The design below is the owner's choice, over reading the sets from the installed configure copy and over reopening 023 (2026-11-09).

- **`canonical`.** `merge-permissions` gains a boolean argument, `canonical`, which is `false` when absent. When it is `true`, the call merges Claude's canonical allow set, its canonical deny set and its retired set as `revoke`, together with any entries the caller passes, which the ceiling still bounds. Configure's step 1 passes `path` and `canonical: true` and nothing else. A call without it behaves as before, so the explicit form stays the contract 022's `framework-list-dedup` scenario and 023's scenarios describe.
- **Where the sets live.** The deny set and the retired set join the allow ceiling as compiled constants in `runtime/src/primitives/merge_permissions.rs`, `CANONICAL_DENY` and `RETIRED_ALLOW`. Two tests hold them equal to steps 3 and 4 of `framework/bootstrap/configure/claude.md`, as `the_ceiling_is_configure_claudes_canonical_allow_set` holds the allow set to step 2. Those steps stay the reviewed record that Families 29 and 32 read. A change to either side alone fails its test, and the constants reach adopters in lockstep with the command.
- **`additionalDirectories`.** The canonical call also ensures that `permissions.additionalDirectories` holds two absolute paths without a trailing slash, as this checkout's file holds them: the configured spec root, and `{config-dir}/commands/{project}`. `{config-dir}` is the settings file's own directory and `{project}` is the namespace `Host::load` resolves. An entry is compared by its exact string, a missing one is appended, and every other entry stays where it is. An `additionalDirectories` that is present but not an array is refused, naming the file, before anything is written (`BE-INPUT-001`; `ensure_directories` in `runtime/src/primitives/merge_permissions.rs`). Step 5's host-side write goes, and with it the approval prompt it raised.
- **The result.** It gains `directories-added`, the paths appended, and `revoked`, the retired entries removed, in the retired set's order. Step 7 can then name each removed entry, which a count alone did not allow. Every existing count stays.
- **The harness.** `scripts/measure-command-cost.py --permissions adopter` passes `--canonical` instead of the sets it parsed from configure's steps 2 and 3, so its own parse goes.

### Reference sections are read through `read-section`

The scenario `reference-reads-follow-the-cited-sections` has a run read only the sections of a reference that its path's steps point at. The design is the owner's choice (2026-11-09), over a line index in each installed copy read through the host's own reader, over splitting each reference into one file per section, and over retiring the scenario.

- **`read-section`.** A new primitive taking a repo-relative markdown `path` and a list of `headings`. It returns each section found, from its heading line to the next heading of the same or a higher level, in the order asked, and lists the headings it did not find. A heading matches by exact text. Sections are cut by the rules `tool_loading::cited_tools` uses, from `resolve_anchor.rs`'s heading index, outside fenced blocks and HTML comments, so the reader and the line agree on what a section is. A section past `[artifacts] read-size-bytes` moves to a file, as every bounded result does (AC7). A path outside the project root is refused by 022's path screen (`validate_contained`, called from `runtime/src/primitives/read_section.rs`).
- **The commands.** Each of the 13 commands' "Where a named section lives" paragraphs tells the model to invoke `read-section` with that reference and every section a step points at, in one call, rather than read the whole file. A section that points at another is read in the same call when the step names both, or in a second call when only the first names it.
- **The line.** The paragraph is neither a step nor a cited section, so `cited_tools` also counts a name invoked anywhere in the command file. Measured at f9991d60, the only names invoked outside the steps are `resolve-session`, which loads with the session, and prune's `prune-project`, which the line already holds. Every line for a command with a reference therefore gains `read-section`.
- **Measured before code**, by the sections each cost-round preset's steps reach (an estimate, not a run): clarify reads about 7,000 of 11,581 bytes, groom 6,300 of 10,625, analyze 43,600 of 48,933, fold 3,900 of 5,095 and plan 8,400 of 9,559.

### Rules reach the model through `discover-rule-files`

The scenario `rules-load-without-shell-reads` has the rules a review or an analysis loads reach the model as tool results. The design is the owner's choice (2026-11-09), over a separate `read-rules` primitive, which would cost review a second call that waits on the first's selection.

- **`rules`.** `discover-rule-files` gains a boolean `rules`. When it is set, the result also lists each selected file's rules, in heading order: each rule's ID, its tier, and its Statement, parsed by `rule_sections::parse_rule_sections`, the reading `check-rule-ids` and the exec walker share. `verification` adds each rule's Verification: its paragraph, and a pipe table directly after it, one row per line, as BE-API-001's header values follow its paragraph, so the listing carries all of it and analyze reads no rule file. A rule whose Statement carries no RFC 2119 keyword has no tier, as `rule_sections` reads it.
- **`all`.** `all` selects every rule file in the directory, whatever its suffix, `[rules] surfaces` and `disabled-rule-files` say, as analyze loads them (§rules). Its notices name what it loaded, as review's do.
- **`extra-files`.** Repo-relative rule files outside the directory, which review loads when `AGENTS.md` references them, are parsed and listed the same way, each passing the same path screen (`validate_contained`, called from `runtime/src/primitives/discover_rule_files.rs`).
- **Unreadable.** A file that cannot be read, or is not UTF-8 text, is listed under `unreadable` with its reason, and a file holding no rule heading lists no rules. Neither is dropped silently.
- **The bound.** The rules list is bounded through `bounded_lists`. By an estimate replicating the parser in Python, the 12 shipped files hold 310 rules, about 138,000 characters with their Statements and about 341,000 with their Verifications. Both are past the read size, so the list moves to a file, which the model reads as it reads any bounded result.
- **The commands.** review's §Load rules and analyze's §Rules read the rules from that result instead of reading the files. analyze's directory walk becomes the primitive's `all`.
- **Malformed rule headings.** The parser opens a rule only at a heading the rule-ID grammar matches, so a malformed rule heading is no rule there. Under `all`, therefore, each listed file also lists its `other-headings`, every heading that opens no rule, with its line and its `body`, the lines under it up to the next heading. analyze's §Rules judges from that body which of them were meant as rules and reports each as malformed (owner, 2026-11-10), reading no rule file. The body was added after task 48's run, where a model with the heading alone read its lines with `git show` (owner, 2026-11-10, over a prose-only fix and a waiver). A model that queries the list file itself with `jq`, as both verifying runs did, reads a tool result, not a rule file, and that is waived (owner, 2026-11-10). That catches what a looser pattern cannot, such as `### BE_AUTHN_001` or a prose heading, at 84 headings on the 12 shipped files: 2,939 characters of heading lines, each with its newline, and since task 57 12,601 more of their bodies, by the release build's `discover-rule-files` with `all` at 12a7cede. Reading the files whole was rejected at 407,292 bytes. Review's call passes no `all`, so its result is unchanged. Under `all`, `[rules] surfaces` and the disabled entries select nothing, so they are not validated there, and a review setting cannot halt an analysis.

### groom derives its routing candidates

The scenario `groom-derives-routing-candidates` has groom call `derive-routing-candidates` for each inbox item before its routing judgment, as specify's step 1 does. groom's step 2 names it bare, since the item's text is a per-item argument the exec walker cannot supply, so the walker dispatches nothing there and the line still loads it. The decision tree's Step 2 in groom's reference starts from those candidates where the command derived them. implement, review and analyze walk the same tree for findings and derive none, so for them the step reads as before.

- **An item that matches nothing.** It takes the scenario's Edge Case: the round trip runs as before, and whether the model also searches spec bodies is its call (owner, 2026-11-10). A slug list that matches nothing says nothing of what the bodies cover, and §grounding has the model consult a reachable source before reasoning without it.
- **Verified on the main path.** The cost harness's groom preset seeds a spec whose slug shares the item's words, since no item can match the generated corpus's `NNN-feature-N` slugs: `feature` is a stopword. A shown candidate is the path a real corpus takes. Every one of the 390 items `specs/inbox.md`'s history added matches at least one of this repository's slugs (`research.md`, *Routing candidates on the main path*).

### Corpus-growing results are bounded through `bounded_lists`

Each primitive in `research.md`'s grows-set returns through `bounded_lists::place` (`runtime/src/primitives/bounded_lists.rs:53`) (AC7). The result is inline while it fits `read-size-bytes`. Past that, its lists go to the file `lists-file` names, with their counts inline, as with `compute-review-scope` (`runtime/src/schema/primitives.rs:752`).

- **Known members.** `read-tasks`, which crossed both host limits, and `dashboard`: 38,815 characters at 78 specs, so past 50,000 near 100.
- **Consumers.** A command reads a moved list by its primitive's description, which states the file form. A tool's description is where the constitution places a primitive's rules (§runtime-host-integration), and a host loads it before it can call the tool. The description replaces a branch per reading step. Those readers sit in 12 command sources and 5 references, and the 8 branches the earlier bounded results carried in `analyze.md`, `implement.md` and `review.md` are swept (owner, 2026-11-08). A rule in the constitution was weighed and set aside: the constitution sat 12 bytes under its 50,000-byte ceiling (owner, 2026-11-08). A script that parses a member's JSON reads the file itself. A Rust caller of a member's `run` calls its unbounded `compute` instead. The exec walker's own `read-tasks` call (`runtime/src/interpreter/payload.rs`, the `writeCode` task lookup) is one of them, and the CLI's `check-corpus-links` reports every broken link from the whole scan before bounding it.
- **The walker.** `ductus exec` reads a moved list back from `lists-file` when the result arrives, before its tally and its merge (`restore_lists`), so every step of a walk reads each bounded result's lists whole. It halts when the file lacks a list the result counts, since that list would read as empty: each `{list}-count` names its list, and `bounded_lists`' `IRREGULAR_COUNTS` names the exceptions, `diff-cross-spec`'s `cross-spec-count` of `cross-spec-paths` and the counts of no list in `prune-tasks` and `measure-reopenings` (owner, 2026-11-09). The context holds one `lists-file` key, so a reader that waited would find a later result's file in its place: `implement` runs five bounded primitives in one walk. The walker emits no result to a host, so the read size bounds nothing there (owner, 2026-11-08).
- **Tests.** Each member has a test that drives a generated corpus past the bound and reads the whole list back from the file.
- **`read-spec` carries no audit record.** Its result dropped `review` and `analyze`, the records composed from `review.md` and `analysis.md`, since no command, walker step or script read them. In the file form they had stayed inline, so records holding 474 digest entries each gave a 112,121-byte file form, where without them it is 424 bytes (owner, 2026-11-10). `check-artifacts` still reads the review record through the unbounded `compute`, and a caller wanting a record reads its artifact.

### A `claude -p` harness records each command's cost

`scripts/measure-command-cost.py` drives `claude -p --output-format stream-json --verbose` on a fixture project (AC15). The fixture holds a generated corpus, the tree `/ductus` installs for Claude, each file list read from `framework/bootstrap/ductus.md`'s own tables, `.mcp.json`, and the pointer to the release binary.

- **What it records per run:** time to first visible output, time to result, model output tokens from the result event's usage, and the largest tool result in characters of result text.
- **When it runs:** on demand only. No workflow references it, since a MUST never requires CI and each run is a model call.
- **Its waits:** each child it runs is an argument list, never a shell (`BE-INPUT-003`), under a named deadline in `scripts/measure-command-cost.py`: `PROBE_TIMEOUT_S` (30 s), `BUILD_TIMEOUT_S` (180 s), `CLAUDE_RUN_TIMEOUT_S` (20 min), then `KILL_REAP_TIMEOUT_S` (10 s) to reap a killed one (`BE-TIMEOUT-001`).
- **Its first run** records whether `claude -p` executes a custom slash command, and how a local `UserPromptExpansion` hook treats a `-p` prompt.
- **Why Python:** stream parsing, and dropping the prototype's `jq` dependency is part of what this spec does.

It runs one round after the view, the hook and tool loading land. That round records the cost of each command `/ductus` installs in `research.md` with its method, units, population and date (AC8). Each command runs once on a fixture of its own, seeded with the least state its main path needs, under an adopter's permissions: the bootstrap seed, then `/{project}:configure`'s canonical sets (owner, 2026-11-09). There is no separate baseline round: the spec's transcript figures are status's before-figure. Each command's review then reads that record. `research.md` names, per command, the reduction that applies or why none does, and each reduction becomes a scenario through `/{project}:amend` (spec Behavior, *One case per command*).

### Auggie, OpenCode and Antigravity

None of the three gets the hook, the batching line or a bridge change (AC10). OpenCode's scenario and Antigravity's are recorded through `/{project}:amend` after a run on a version that has the mechanism: an OpenCode TUI plugin's slash command, and `agy` 1.4.11 or later with a `PreInvocation` hook (AC11). Each scenario records its outcome, and the deferral it settles, under Resolved Questions with its trigger. An open question there would hold 080 short of `done`.

On OpenCode and Antigravity the view's `/{project}:<name>` hints name no command, since those hosts run one as `/{project}/<name>` and `/{project}-<name>`. That gap is waived for this spec (owner, 2026-11-10): it predates 080, since `/ductus` substitutes `{project}` and leaves the colon after it (`framework/bootstrap/ductus.md`, step 4's substitutions map), so every installed command's prose on those hosts already spells the colon form, as status's model-composed prompt did before; it is not the view's alone; and AC10 holds.

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
| `runtime/src/primitives/render_help.rs` | Create | The guide's extraction; `render-help` primitive |
| `runtime/src/primitives/merge_status_hook.rs` | Create | `merge-status-hook` primitive |
| `runtime/tests/merge_status_hook.rs` | Create | The version probe's `PATH` lookup, driven through the binary |
| `scripts/mutation/merge-status-hook.py` | Create | `merge-status-hook`'s mutations, each with its killing test |
| `runtime/src/primitives/tool_loading.rs` | Create | Step-tool derivation, cited sections' invoked tools, and the batching line |
| `runtime/src/primitives/resolve_anchor.rs` | Modify | Heading rules shared with the batching line's citations and `read-section` |
| `runtime/src/primitives/read_section.rs` | Create | `read-section` primitive |
| `runtime/src/primitives/discover_rule_files.rs` | Modify | `rules`, `verification`, `all` and `extra-files` |
| `framework/commands/*.md` | Modify | Each "Where a named section lives" paragraph invokes `read-section` |
| `framework/reference/review.md` | Modify | §Load rules reads the rules from `discover-rule-files` |
| `framework/reference/analyze.md` | Modify | §Rules reads every file's rules from `discover-rule-files` with `all` |
| `runtime/src/primitives/rule_sections.rs` | Modify | Each rule section keeps its heading line, and its Verification the table directly after it |
| `scripts/mutation/reference-and-rule-reads.py` | Create | `read-section`'s and the rules listing's mutations |
| `framework/commands/groom.md` | Modify | Step 2 derives routing candidates per item |
| `framework/reference/groom.md` | Modify | The decision tree's Step 2 starts from those candidates |
| `framework/reference/plan.md` | Modify | §Validation gate invokes `derive-boundary` |
| `framework/reference/implement.md` | Modify | §Walk through tasks invokes `append-task` |
| `runtime/src/hook.rs` | Create | `ductus hook status` and `ductus hook help` surface |
| `runtime/tests/hook_status.rs` | Create | The hooks' output, root and exits, driven through the binary |
| `docs/runtime.md` | Modify | `CLAUDE_PROJECT_DIR`, `DUCTUS_BIN` and `DUCTUS_GEN_TIMEOUT_SECONDS` in the environment-variable inventory (`CFG-ENV-002`) |
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
| `framework/runtime-tools.txt` | Modify | Four new tools: `render-status`, `merge-status-hook`, `render-help`, `read-section` |
| `framework/commands/status.md` | Modify | Reads `render-status`; step 3 removed |
| `framework/commands/help.md` | Modify | Scope Boundaries name the runtime's answer to a typed help |
| `framework/commands/implement.md` | Modify | Its `read-tasks` lists-file branch swept, since the description states the file form |
| `framework/commands/review.md` | Modify | Its `--all` lists-file branch swept, as above |
| `framework/commands/target.md` | Modify | Its not-found enumeration's lists-file branch swept, as above |
| `framework/bootstrap/configure/claude.md` | Modify | Calls `merge-status-hook`; step 1 passes `canonical`, step 5's host write goes |
| `runtime/src/primitives/merge_permissions.rs` | Modify | `canonical`: the deny and retired sets, `additionalDirectories` |
| `framework/bootstrap/configure/auggie.md` | Modify | Regenerated allow block |
| `framework/bootstrap/ductus.md` | Modify | Claude rows' field; hook install; seed rule |
| `framework/bootstrap/pi/ductus-bridge.ts` | Modify | Registered status and help commands |
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

- `render-status`'s result, and `render-help`'s;
- the view's line grammar, its collapse line and its cut lines;
- each hook's input, output and exit statuses, `ductus hook status` and `ductus hook help`;
- `merge-permissions`' canonical call;
- the `settings.local.json` hook entries and how each is recognized;
- `merge-status-hook`'s arguments and result;
- the `ManifestEntry` field and the batching line;
- the `_meta` key;
- how a bounded result is split.

## Trade-offs

- **Paging the view:** rejected for the hook, which shows one message (spec Edge Cases), so the view collapses and cuts instead. The model path keeps the cut view too, and its file holding the whole view is what the agent pages (owner, 2026-11-10).
- **A `format` flag on `dashboard`:** rejected, because `ductus exec` passes no arguments (the spec's constraint), so the flag could not reach an exec-driven status.
- **Keeping `rendered-markdown` on `dashboard`:** rejected, because nothing would read it.
- **A cli-only `status-hook` primitive:** rejected, because principle 5's exemptions do not cover it.
- **A shell or `jq` hook:** rejected by the spec. The hook's command is the binary.
- **Host-run `claude --version` passed to the primitive:** rejected, since the probe reads the version from inside the MCP server. It would need a permission entry in two seeds, and it would not run under `ductus exec`.
- **Always-load as a test-pinned constant:** rejected, because a pinned, edited `target.md` would not move it. Embedding at build time is impossible for a crate published from `runtime/` alone.
- **Always-loading every tool, or a measured top-12 list:** rejected by the owner (spec Resolved Questions), and rejected again on the revisit after AC23.
- **Every name in a cited section, or in the command's whole reference:** rejected (owner, 2026-11-09). Measured at 65e7acdb, the first would load 20 tools that no run calls, 140,823 characters of schema across 9 commands, and the second 15 tools, 67,274 characters across 8. Most of it is `write-review` and `write-analysis`, which a reference names when describing another command's record, and it is paid to save a search of about 1.6 s.
- **Naming each call in a step instead:** rejected (owner, 2026-11-09). It adds no tool a run does not call either. But every call a reference gains would need its step edited too, or it costs a search again.
- **The invoked form's reach:** a name outside the steps that no cited section invokes still loads through its own search. Measured at 65e7acdb, every such name in a command file is a mention: prune's `classify-spec-edit`; consolidate's `check-corpus-links`, `check-orphaned-references` and `classify-spec-edit`; fold's `check-orphaned-references`; amend's `label-criteria`; clarify's `append-question`; and status's `resolve-references`.
- **A line index in each installed copy, read through the host's own reader:** rejected (owner, 2026-11-09). It costs about 50 characters per heading in every run, read or not, analyze's about 1,500, and needs every agent's reader shown to take an offset.
- **One file per reference section:** rejected (owner, 2026-11-09). About 110 files would replace 13, every `§` pointer, manifest row, anchor check and audit family would change, and adopters would need a migration.
- **Retiring `reference-reads-follow-the-cited-sections`:** rejected (owner, 2026-11-09). clarify and groom would keep reading about 4,000 bytes each that their paths do not use, and nothing else bounds a reference: analyze's is 49,186 bytes against the 50,000-byte read size.
- **A separate `read-rules` primitive:** rejected (owner, 2026-11-09). review would make two calls, the second waiting on the first's selection, and analyze would keep its host-side directory walk.
- **Leaving every body search to the model:** rejected (owner, 2026-11-10). It would verify nothing on the main path, and it gives up the 6.99 s and 3 of 7 model requests the failed round's three `grep`s cost, one of them denied.
- **Forbidding the shell search in groom's step 2:** rejected (owner, 2026-11-10). Where the slugs carry no subject, it either contradicts §grounding or moves the model to reading each spec.
- **Matching titles or bodies in the primitive's spec-corpus source:** rejected (owner, 2026-11-10). The seeded item matches nothing on the generated corpus by slug, title or body alike. Over this repository's corpus, titles match a median of 22.5 specs per historical inbox item against 20.5 for slugs, and bodies match all 78 for every item (`research.md`, *Routing candidates on the main path*).
- **Naming the command's first call in the line:** rejected (owner, 2026-11-09). The first tool the steps name is not the first call in review, implement or groom, so the name would need the parser's dispatch order, and a wrong one points the model at the wrong call. Whether a fixed sentence moves the model as far as a named call would is what the verifying round measures.
- **A batching line in the command sources:** rejected, because AC29 keeps the sources and other agents' copies unchanged.
- **Driving `apply-manifest` from the generator:** costs the generator a built binary, which `generators.yml` already builds. It buys one code path for this repo's copies and adopters'.
- **Pi's load-time `render-help` call:** costs one read of the installed help copy per Pi session start, to learn the namespace, with `render-status` read instead on a runtime that serves no `render-help`. The alternatives were parsing `.ductus/config.toml` in TypeScript, or substituting `{project}` into a bridge 064 requires to be verbatim.
- **No baseline round of the harness:** status's before-figures come from transcripts (spec Motivation). For the other commands there is no before-figure, so a reduction found later is measured against the round 080 records.
- **One entry for status and help, with a widened matcher:** rejected. It needs a `hook` member that reads `command_name` from the input, a field this project has not observed, and it needs every adopter's `hook status` entry retired, which would otherwise fire beside it. Two entries need neither.
- **Renaming `merge-status-hook` once it installs help too:** rejected. The name is in 28 live files (115 occurrences, 7 specs), and every adopter's permission entries for the old name would need retiring.
- **The bridge reading Pi's help copy itself:** rejected. It would be a second copy of the guide's extraction rule, in TypeScript, and the capability `hook help` adapts belongs on MCP in any case.
- **Reading configure's sets from its installed copy at run time:** rejected. It would follow a pinned copy's deny and retired edits. But it needs a runtime parser of three steps, errors for a copy that is missing or malformed, and an allow set that the compiled ceiling would still refuse past, so a pinned copy that adds an allow entry would fail the whole call. The allow side already ignores such edits.
- **Making the canonical sets the default when no entry is passed:** rejected. Over MCP an omitted list and an empty one arrive alike, so a call meant to dedup alone would install the sets. The explicit argument says what the call does.
- **The settings entry's path:** it names the repository pointer and assumes `CLAUDE_PROJECT_DIR` is the registering root. A per-machine absolute path, as in the prototype, would not travel.

## Open Questions Resolved

- **`ductus exec` passes no per-step arguments.** The status path is the argument-free `render-status`, not a `dashboard` flag.
- **The batched-load line must not backtick primitive names where the walker reads them.** The line sits in the Instructions preamble. Its one code span is `select:mcp__ductus__…`, which is no registry name, and the AC29 test checks that the generated copy parses to the source's `Procedure`.

## Cross-spec impact

- **022 — deterministic runtime**, `in-progress`, owner of the runtime core. It owes the new `hook` surface with its `help` member, `render-status`, `render-help`, `merge-status-hook` with help's entry, `merge-permissions`' `canonical` argument, the `alwaysLoad` `_meta`, `dashboard`'s dropped field and the bounded results. Discharged by the edits to its data model and its `dashboard-primitive` scenario, which links back to 080.
- **023 — govern refinement**, `done`. Its scenario `configure-retires-formerly-canonical-entries` says configure passes the retired list to `merge-permissions` as `revoke`. Under `canonical` the primitive holds that list itself. A blockquote signpost in its `spec.md` is a correction and does not reopen it, and `cross-spec-impact:` gains 023 until that signpost links back.
- **063 — oversized artifacts warn with a fix**, `in-progress`. `read-size-bytes` now also bounds primitive results and the view. Discharged by a signpost in its `spec.md` linking 080.
- **046 — scenario open-question visibility**, reopened to `in-progress` (37693d4d). Its AC24 and its *Many scenarios carrying questions* edge case now except the pipeline view past the read size, which counts the question lines it does not show and on the model path names the file holding them all. Discharged by that edge case's link back to 080.
- **066 — audit records and the completion gate**, `in-progress`. Its scenario `write-analysis-and-the-second-gate` drops the edge case on `ReadSpecResult.analyze`, since `read-spec` carries no audit record. Discharged by that scenario's signpost linking 080.
- **064 — Pi host support**, `in-progress`, reopened for its AC12 by the owner's decision (4b6c5fe4). The bridge registers two commands, status and help. A blockquote signpost in its `spec.md` is a correction and does not reopen it. Its D1 command surface would change only if the template won over the command, and `research.md`'s task 12 run found the registered command runs. Had `/ductus` stopped installing `{project}-status`, 064's criteria would have changed, and `classify-spec-edit` would have decided the reopen.
