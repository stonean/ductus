# 080 — Deterministic runtime efficiency Research

The measurements and verifying runs spec 080's criteria point at. Each figure states its method, units, population and date (§grounding), and each section names the command that reproduces it.

## Method traps

Three ways an earlier measurement of this work went wrong, each of which inflates or misattributes a figure:

- **Size a tool result as the characters of its result text, not as `json.dumps` of the content.** In a session transcript the content is a JSON value, and serializing it again counts its escaping: for `dashboard` that read 41–44 KB against the true 37,171–40,454 characters.
- **A command the agent starts through the Skill tool leaves no `<command-name>` tag in a transcript.** Attributing each `ToolSearch` to the running command takes the Skill `tool_use` entries as well as the tags.
- **A loaded tool reaches the model as its description and input schema only.** The output schemas, 228,054 of 366,331 schema characters when the spec was clarified, never do, so a loaded-schema size counts description plus input schema, measured from `ductus mcp` `tools/list` over stdio, never from a transcript.

## Result sizes (AC14)

**Method.** `scripts/measure-result-sizes.sh` runs the ignored `result_sizes` test in `runtime/tests/result_sizes.rs`. It generates two corpora with `runtime/tests/common/corpus.rs`, sized as the report's first line says, and calls every name in `PRIMITIVE_REGISTRY` through the built binary's MCP server over stdio. Each call runs on a fresh copy of its corpus, so a writing primitive changes nothing the next one reads. Per-feature primitives are pointed at the subject spec, `001-feature-001`, which is `in-progress` with every task ticked; `traverse-deps` is pointed at the last spec, whose chain runs through every other. Every spec carries one markdown-lint violation and one broken link to a file outside the spec root. A primitive that returns findings grows with the defects in what it reads, and on a defect-free corpus that growth would not show. For each ten specs the corpus also holds one branch-scoped spec with a `folds-into:`, so the staging inventory scales as well. `check-orphaned-references` reads only a fixed set of adopter-owned files (`CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit` and the spec root's `system.md`), not the spec corpus, so its result cannot grow with the corpus by its contract.

An argument that widens what a primitive reads gets a row of its own, since a result can grow in a mode the default call never takes: `read-spec` with `include-body`, the `all` walks of `prune-tasks` and `prune-plan`, and `check-corpus-links` with `scope: repository`. Every other optional argument narrows a read, names a write, or carries the host's input. The Small and Large columns are measured with `[artifacts] read-size-bytes` raised past every result, so a bounded result is measured by what it carries rather than by its file form. The third column measures the large corpus at the default read size.

**Units.** Characters of the result's text, the `text` of the `tools/call` result's content, which is what a host reads.

**Population.** Every registry name, and each argument mode above. A name the fixture cannot exercise has a *not measured* row with its reason.

**Grows with the corpus.** A result grows when its size on the large corpus is at least twice its size on the small one and at least 200 characters more. A count's extra digits pass neither test, and a list that scales with the corpus passes both. *Bounded* means the result carries `bounded_lists`' `form` field, so it moves its lists to a file past `read-size-bytes`. `render-status` carries none: its view is held to the read size by its own bound (AC24), so its row reads *no*.

### Measurement

Measured by `scripts/measure-result-sizes.sh` at 4294b79a15383c1c5cf700d62a0f05b2fadee23d, with a clean runtime/, at 2026-11-09T01:51:06Z.

Small corpus: 10 specs, each with 5 criteria, 5 tasks of 3 subtasks and 2 scenarios, one markdown-lint violation and one broken link; 5 inbox items. Large corpus: 100 specs, each with 50 criteria, 50 tasks of 3 subtasks and 20 scenarios, one markdown-lint violation and one broken link; 50 inbox items. Small and Large are measured at a read size of 1000000000 bytes, past every result; Large at the default read size is the same corpus at 50,000.

| Primitive | Small (chars) | Large (chars) | Large at the default read size (chars) | Grows | Bounded | Note |
| --- | --- | --- | --- | --- | --- | --- |
| `read-spec` | 1138 | 6167 | 6167 | yes | yes | |
| `read-spec` with `include-body` | 2373 | 18133 | 18133 | yes | yes | |
| `read-tasks` | 1778 | 17280 | 17280 | yes | yes | |
| `mark-task` | 72 | 72 | 72 | no | no | |
| `mark-criterion` | 72 | 72 | 72 | no | no | |
| `set-status` | 82 | 82 | 82 | no | no | |
| `derive-boundary` | 386 | 2367 | 2367 | yes | yes | |
| `discover-rule-files` | 110 | 110 | 110 | no | no | |
| `process-waivers` | 54 | 54 | 54 | no | no | |
| `process-decisions` | 172 | 172 | 172 | no | yes | |
| `compute-review-scope` | 1491 | 34570 | 34570 | yes | yes | |
| `write-review` | 345 | 346 | 346 | no | no | |
| `write-analysis` | 165 | 165 | 165 | no | no | |
| `check-step-references` | 255 | 255 | 255 | no | no | |
| `check-stuck` | 101 | 101 | 101 | no | no | |
| `validate-frontmatter` | 28 | 28 | 28 | no | no | |
| `resolve-anchor` | 85 | 85 | 85 | no | no | |
| `traverse-deps` | 133 | 129 | 129 | no | no | |
| `check-rule-ids` | 58 | 58 | 58 | no | no | |
| `run-generator` | — | — | — | not measured | — | CLI-only: it runs a project script and is no MCP tool, so no host reads its result as a tool result |
| `lint-markdown` | 1372 | 13083 | 13083 | yes | yes | |
| `gate-confirm` | 70 | 70 | 70 | no | no | |
| `fetch-archive` | — | — | — | not measured | — | it needs the network |
| `extract-archive` | — | — | — | not measured | — | it needs a release archive, which a spec corpus does not hold |
| `apply-manifest` | — | — | — | not measured | — | its result follows the manifest the host builds and needs a staged framework tree; a spec corpus holds neither |
| `enforce-manifest` | — | — | — | not measured | — | its result follows the expected-file list the host passes, not the spec corpus |
| `merge-managed-block` | 163 | 163 | 163 | no | no | |
| `merge-permissions` | 209 | 209 | 209 | no | no | |
| `merge-status-hook` | 61 | 61 | 61 | no | no | |
| `migrate-session-file` | 114 | 114 | 114 | no | no | |
| `relocate-audit-records` | 61 | 61 | 61 | no | no | |
| `create-scenario` | 59 | 59 | 59 | no | no | |
| `append-task` | 89 | 90 | 90 | no | no | |
| `label-criteria` | 89 | 90 | 90 | no | no | |
| `prune-tasks` | 940 | 6921 | 6921 | yes | yes | |
| `prune-tasks` with `all` | 1350 | 12863 | 12863 | yes | yes | |
| `prune-plan` | 228 | 228 | 228 | no | no | |
| `prune-plan` with `all` | 150 | 658 | 658 | yes | yes | |
| `prune-project` | 424 | 424 | 420 | no | no | |
| `dashboard` | 3680 | 33469 | 33469 | yes | yes | |
| `render-status` | 1991 | 11570 | 11570 | yes | no | |
| `write-session` | 46 | 46 | 46 | no | no | |
| `resolve-session` | 60 | 60 | 60 | no | no | |
| `retarget-sessions` | 46 | 46 | 46 | no | no | |
| `resolve-references` | 140 | 140 | 140 | no | yes | |
| `resolve-constitutions` | 60 | 60 | 60 | no | no | |
| `resolve-feature` | 120 | 120 | 120 | no | no | |
| `create-feature` | 130 | 130 | 130 | no | no | |
| `create-plan-artifacts` | 196 | 196 | 196 | no | no | |
| `check-review-freshness` | 86 | 86 | 86 | no | no | |
| `check-review-gate` | 951 | 6264 | 6264 | yes | yes | |
| `classify-spec-edit` | 28 | 28 | 28 | no | no | |
| `measure-reopenings` | 520 | 4133 | 4133 | yes | yes | |
| `append-question` | 135 | 135 | 135 | no | no | |
| `diff-cross-spec` | 434 | 3360 | 3360 | yes | yes | |
| `append-inbox` | 56 | 57 | 57 | no | no | |
| `remove-inbox-item` | 60 | 61 | 61 | no | no | |
| `check-artifacts` | 670 | 5045 | 5045 | yes | yes | |
| `check-artifact-size` | 217 | 218 | 214 | no | no | |
| `derive-routing-candidates` | 209 | 209 | 209 | no | no | |
| `check-corpus-links` | 4283 | 130694 | 295 | yes | yes | |
| `check-corpus-links` with `scope: repository` | 4282 | 130693 | 295 | yes | yes | |
| `check-orphaned-references` | 181 | 181 | 181 | no | no | |
| `check-command-flags` | 263 | 263 | 263 | no | no | |
| `check-promotion-coverage` | — | — | — | not measured | — | it audits the framework's own rule file against command sections, which a spec corpus does not hold |
| `derive-dependencies` | 156 | 157 | 157 | no | no | |
| `derive-references` | 168 | 169 | 169 | no | no | |
| `check-unfolded-specs` | 167 | 1107 | 1107 | yes | yes | |
| `rewrite-spec-links` | 914 | 7037 | 7037 | yes | yes | |
| `retire-feature` | 47 | 47 | 47 | no | no | |
| `invalidate-review` | 62 | 62 | 62 | no | no | |
| `start-fix-rounds` | 73 | 75 | 75 | no | no | |
| `brief-re-read` | 971 | 17397 | 17397 | yes | yes | |
| `read-re-read` | 316 | 316 | 316 | no | yes | |
| `check-prescriptions` | 406 | 406 | 406 | no | yes | |

### Grows with the corpus

Of the 71 registry names, 17 grow with the corpus by the test above, in 20 of the table's rows.

- **Bounded through `bounded_lists` by this spec, so AC7's population (12):** `read-spec`, `read-tasks`, `derive-boundary`, `lint-markdown`, `prune-tasks`, `prune-plan`, `dashboard`, `check-review-gate`, `check-artifacts`, `check-corpus-links`, `check-unfolded-specs`, `rewrite-spec-links`. The first measurement, at 2b4eaddb, called each primitive in its default mode only and found the other 11; `prune-plan` grows only in its `all` walk.
- **Bounded through `bounded_lists` before this spec (4):** `compute-review-scope`, `measure-reopenings`, `diff-cross-spec`, `brief-re-read`.
- **Held to the read size by its own bound (1):** `render-status`, whose view collapses and cuts past it (AC24).

## Loaded schema sizes (AC23)

**Method.** The same run lists the tools with `tools/list` and sizes each as its description's characters plus its input schema serialized as compact JSON, which is the form the server sends. Rows are in descending order of the loaded size.

**Reconciliation with the spec's figure.** Spec 080's Resolved Questions record 132,846 characters across 68 tools. The first measurement, at 2b4eaddb, totalled 129,531 for the same 68 tools in the compact form. The same `tools/list`, read from `runtime/target/release/ductus mcp` over stdio on 2026-11-08, also totalled 129,531 compact, and 132,353 when Python's `json.dumps` serializes the input schemas with its default separators (`", "` and `": "`). The spec's figure was therefore taken with the default separators, and the remaining 493 characters are description changes made since it was taken. The measurement below adds `render-status` (task 4) and the sentence each bounded primitive's description gained on its file form (task 5).

### Measurement

Measured by `scripts/measure-result-sizes.sh` at 4294b79a15383c1c5cf700d62a0f05b2fadee23d, with a clean runtime/, at 2026-11-09T01:51:06Z.

70 tools, 134535 characters in all.

| Tool | Description | Input schema | Loaded |
| --- | --- | --- | --- |
| `write-review` | 3579 | 16929 | 20508 |
| `write-analysis` | 1356 | 8721 | 10077 |
| `rewrite-spec-links` | 2614 | 1486 | 4100 |
| `compute-review-scope` | 3217 | 704 | 3921 |
| `process-decisions` | 1567 | 1964 | 3531 |
| `append-task` | 234 | 3222 | 3456 |
| `check-corpus-links` | 2035 | 1365 | 3400 |
| `retire-feature` | 1522 | 1638 | 3160 |
| `prune-project` | 986 | 2089 | 3075 |
| `write-session` | 970 | 2097 | 3067 |
| `derive-references` | 1927 | 735 | 2662 |
| `measure-reopenings` | 1980 | 460 | 2440 |
| `prune-plan` | 1173 | 1242 | 2415 |
| `check-review-gate` | 2109 | 211 | 2320 |
| `derive-dependencies` | 1179 | 1063 | 2242 |
| `apply-manifest` | 60 | 2160 | 2220 |
| `check-artifacts` | 1990 | 211 | 2201 |
| `check-prescriptions` | 1794 | 368 | 2162 |
| `create-feature` | 330 | 1749 | 2079 |
| `merge-permissions` | 97 | 1961 | 2058 |
| `retarget-sessions` | 588 | 1443 | 2031 |
| `process-waivers` | 544 | 1438 | 1982 |
| `classify-spec-edit` | 1560 | 339 | 1899 |
| `resolve-constitutions` | 1471 | 427 | 1898 |
| `brief-re-read` | 1534 | 350 | 1884 |
| `derive-routing-candidates` | 1040 | 742 | 1782 |
| `read-re-read` | 1320 | 314 | 1634 |
| `prune-tasks` | 573 | 973 | 1546 |
| `append-question` | 666 | 852 | 1518 |
| `dashboard` | 1432 | 74 | 1506 |
| `check-promotion-coverage` | 138 | 1360 | 1498 |
| `invalidate-review` | 1159 | 291 | 1450 |
| `diff-cross-spec` | 1200 | 211 | 1411 |
| `create-plan-artifacts` | 514 | 860 | 1374 |
| `check-step-references` | 1293 | 74 | 1367 |
| `check-artifact-size` | 1094 | 211 | 1305 |
| `resolve-references` | 1007 | 243 | 1250 |
| `render-status` | 1093 | 74 | 1167 |
| `enforce-manifest` | 66 | 1089 | 1155 |
| `merge-status-hook` | 771 | 299 | 1070 |
| `resolve-feature` | 442 | 622 | 1064 |
| `check-unfolded-specs` | 972 | 74 | 1046 |
| `check-orphaned-references` | 969 | 74 | 1043 |
| `check-command-flags` | 938 | 74 | 1012 |
| `create-scenario` | 193 | 810 | 1003 |
| `merge-managed-block` | 76 | 921 | 997 |
| `start-fix-rounds` | 763 | 224 | 987 |
| `check-stuck` | 657 | 316 | 973 |
| `derive-boundary` | 762 | 180 | 942 |
| `relocate-audit-records` | 698 | 223 | 921 |
| `fetch-archive` | 64 | 847 | 911 |
| `mark-task` | 420 | 478 | 898 |
| `migrate-session-file` | 342 | 555 | 897 |
| `resolve-anchor` | 266 | 611 | 877 |
| `resolve-session` | 791 | 74 | 865 |
| `mark-criterion` | 57 | 807 | 864 |
| `lint-markdown` | 263 | 527 | 790 |
| `check-review-freshness` | 535 | 211 | 746 |
| `append-inbox` | 258 | 407 | 665 |
| `read-spec` | 341 | 300 | 641 |
| `discover-rule-files` | 179 | 439 | 618 |
| `extract-archive` | 72 | 546 | 618 |
| `remove-inbox-item` | 224 | 373 | 597 |
| `gate-confirm` | 289 | 273 | 562 |
| `set-status` | 136 | 330 | 466 |
| `read-tasks` | 246 | 195 | 441 |
| `check-rule-ids` | 64 | 352 | 416 |
| `label-criteria` | 163 | 211 | 374 |
| `validate-frontmatter` | 55 | 187 | 242 |
| `traverse-deps` | 58 | 180 | 238 |

## Verifying runs

### The version probe under an MCP server's environment (task 7)

**Method.** On 2026-11-08, on macOS with Claude Code 2.1.319, the `ductus mcp` server that this session's `claude` process spawned was read for its environment with `ps -E` and for its working directory with `lsof`. `claude --version` was then run with exactly that environment, replacing rather than inheriting one, in that working directory, with stdin from `/dev/null`, as `merge-status-hook`'s probe runs it. The environment held 65 variables, `CLAUDECODE`, `CLAUDE_CODE_SESSION_ID` and `CLAUDE_PROJECT_DIR` among them, and its `PATH` held `~/.local/bin`, where the native installer puts `claude`.

**Result.** Exit 0, `2.1.319 (Claude Code)` on stdout, nothing on stderr, so the probe stays in the runtime and the host-run fallback is not used. This replays the server's environment rather than probing from inside the server process; task 8's `/ductus:configure` run is that check.

### The probe inside the MCP server, and the hook's install (task 8)

**Method.** On 2026-11-09 at 02:11 UTC, on macOS with Claude Code 2.1.319, `/ductus:configure` ran in this checkout. It ran through the `ductus mcp` server this session's `claude` process spawned, which `lsof` showed running `runtime/target/release/ductus`, the release build of 4294b79a's runtime. No `runtime/` change landed after that build. Beforehand, `.claude/settings.local.json` had no `hooks` key.

**Result.** Step 6's `merge-status-hook` returned `updated` with `claude-version` `2.1.319` and a null `reason`, so the probe reads the version from inside the server process. The file then held one `UserPromptExpansion` entry: matcher `ductus:status`, command `"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus" hook status`, timeout 10. It held no other hook event, and its other top-level keys were unchanged. A second call returned `unchanged` and left the one entry.

### Typed `/ductus:status` in Claude Code (task 8)

**First run.** On 2026-11-09, on macOS with Claude Code 2.1.319, the owner typed `/ductus:status` at this checkout's root, in the session that installed the hook at 02:11 UTC. The session transcript (`~/.claude/projects/<slug>/<session-id>.jsonl`) holds one entry for it, at 02:18:00.156Z. It is a `system` entry with subtype `informational`, `level: warning` and `preventContinuation: true`, and its content is `UserPromptExpansion operation blocked by hook:` followed by the view, with no "Original prompt:" line. No model request falls between the previous turn's end at 02:17:25Z and the next prompt at 02:18:04Z, and that next turn's context did not carry the view. The hook therefore fires in the session that installed it, with no `/hooks` visit or fresh session.

**Second run.** At 02:23:03.679Z the same session holds a second entry of the same subtype, level and size, 16,888 characters, again with no "Original prompt:" line. No model request falls between the previous turn's end at 02:19:27Z and the next prompt at 02:23:05Z.

**Subdirectory run.** At 02:31:24Z the owner typed `/ductus:status` in a fresh Claude Code 2.1.319 session started in `runtime/` (session `3bffc5dd`). The hook fired, so that session read the root's `.claude/settings.local.json`. Claude Code set `CLAUDE_PROJECT_DIR` to the session's start directory, though, not to the directory whose settings registered the hook. Its transcript's `hook_non_blocking_error` attachment holds `/bin/sh: …/ductus/runtime/.ductus/bin/ductus: No such file or directory`, with `exitCode` 127 and `durationMs` 15. The command then ran on the model path, where the `ductus` MCP server was not connected, since `.mcp.json` names the pointer by the relative path `.ductus/bin/ductus`, and the model said so and stopped, 22.3 s after the prompt (02:31:24.427Z to the turn's end at 02:31:46.733Z). The owner's copy of what that session showed holds only the model's reply, which names the unconnected server and not the hook's failure. So the plan's assumption about `CLAUDE_PROJECT_DIR` does not hold, and neither path shows the view in a session started below the root.

**Pointer moved aside.** At 02:56:27Z the owner typed `/ductus:status` at the root, in the installing session, with `.ductus/bin/ductus` moved aside. The transcript's `hook_non_blocking_error` attachment holds `Failed with non-blocking status code: /bin/sh: …/ductus/.ductus/bin/ductus: No such file or directory`, with `exitCode` 127 and `durationMs` 17. That error did not reach the model's context. The command then ran on the model path. This session's MCP server, started before the move, was still up, so `render-status` returned the view, and the model emitted it after 3 model requests, 44.6 s after the prompt (to the turn's end at 02:57:11.883Z). The run therefore exercises AC25's first half, a failure status rather than a block followed by the model path. It does not exercise the second, the model path surfacing the runtime's own error.

**Pointer moved aside, fresh session.** At 03:10:13Z the owner typed `/ductus:status` in a session started at the root with the pointer moved aside (session `d8f7f5bc`), so its `ductus` MCP server never started. Claude Code showed the owner `UserPromptExpansion hook error` with `Failed with non-blocking status code: /bin/sh: …/ductus/.ductus/bin/ductus: No such file or directory`, from the owner's copy of the screen. The transcript's attachment holds `exitCode` 127 and `durationMs` 12. The command then ran on the model path. After 2 model requests, 14.9 s after the prompt (to the turn's end at 03:10:28.696Z), the model reported that the server had not started, why, and how to recover, rather than showing nothing. This is the missing-runtime case of AC25's second half. A runtime that runs and fails is exercised only by task 6's unit test, on the hook's side.

**Timing.** The transcript records when the view arrived but not when the prompt was submitted, and a block entry carries no duration, so the run's latency cannot be derived from it. Run by hand, the hook command alone took 162 to 167 ms of wall time, median 165 ms, and wrote 17,076 characters. That was ten runs at 2a28bc58, with the hook's command line under `sh`, `CLAUDE_PROJECT_DIR` set to this checkout (78 specs), and a minimal `UserPromptExpansion` input on stdin.

### Target's tools loaded with the session (task 9)

**Method.** On 2026-11-09, on macOS with Claude Code 2.1.319, the owner started a fresh session at this checkout's root (session `902ac95a`). Its MCP server ran this checkout's release build of task 9's change. Over stdio, that build's `tools/list` served 70 tools and set `_meta["anthropic/alwaysLoad"]` on 7: `dashboard`, `derive-dependencies`, `read-spec`, `resolve-constitutions`, `resolve-feature`, `resolve-session` and `write-session`. The owner then typed `/ductus:target 080-deterministic-runtime-efficiency`, and the session transcript was read.

**Result.** At the prompt, 12:10:50.440Z, the transcript's `deferred_tools_delta` attachment listed 99 deferred tools, 63 of them `ductus` tools, and none of the 7. The session made no `ToolSearch` call. It called `resolve-session` 3.1 s after the prompt, then `resolve-feature`, `resolve-constitutions`, `derive-dependencies`, `read-spec` and `write-session`, and the turn ended 19.8 s after the prompt. Claude Code 2.1.319 therefore honors the flag, and target reaches its tools without a search.

### An adopter bootstrap across the change (task 11)

**Staging copy, before pushing.** On 2026-11-09, finished by 14:01:08Z, at 46c449d5, `framework/` was copied into a `mktemp -d` directory per `docs/developing-ductus.md` *Exercising an installer change without a push*. Each agent's `--entries-json` was built from that copy's `bootstrap/ductus.md` *Slash commands* table, 17 rows, with `tool-search-line: true` on Claude's rows only, as the table's paragraph says, and the substitutions `project` `scratch` and the agent's `cli-config-dir`. Two runtimes drove `apply-manifest`, then `enforce-manifest` with the 17 basenames, each in its own copy holding a planted `obsolete.md`. One was this checkout's release build, stamped 13:07:53Z after 81285165, whose runtime change is unit tests alone, so its `apply-manifest` is a62d2328's. The other was the store's `ductus` 0.59.11, the release before this spec.

**Result.** Auggie's 17 copies are byte-identical between the two runtimes, so the field leaves the `claude-style` layout's other agent as it was (AC10). Of Claude's 17, 14 differ from 0.59.11's output only by one added paragraph, `> **Claude Code:** load this command's …`, which closes the `## Instructions` preamble's blockquote: amend, analyze, clarify, configure, consolidate, fold, groom, implement, log, plan, prune, review, specify and status. help, link and target carry no line and are byte-identical, as the plan's *When there is no line* expects. In all four runs `enforce-manifest` removed `obsolete.md` and kept the 17. The procedure stages `framework/` itself as the source root, with sources `commands/*.md`, the generator's layout; the bootstrap's is the archive's root, with sources `framework/commands/*.md`, so this check did not exercise the layout an adopter's run uses, and the upgrade's second run below shows that layout failing.

**The release before this spec.** On 2026-11-09, between the scratch repository's commits at 14:01:06Z and 14:13:19Z, a subagent, with no `ductus` MCP tools and no nested CLI, bootstrapped a scratch adopter under `/tmp` from `ductus-v0.59.11`: that release's `install.sh claude --ref=ductus-v0.59.11`, then its bootstrap carried out as `/ductus --agents=claude,auggie --ref=ductus-v0.59.11`. The store already held 0.59.11, so nothing was downloaded, and the pointer linked to it. It installed 17 command copies for each agent, none carrying the ToolSearch paragraph, and left `.claude/settings.local.json` with no `hooks` key. Auggie's settings file is `.augment/settings.local.json`, and Auggie's MCP registration is the surfaced `auggie mcp add` instruction, which was not run.

**The upgrade, first run.** After 01b97a87 reached `main`, a second subagent added `[runtime] path` naming this checkout's release build to the adopter's config, ran `main`'s `install.sh claude --ref=main`, and carried out `main`'s bootstrap as `/ductus --ref=main`, both agents auto-detected. Branch 1 took the configured binary with no warning, since it and the pin both report 0.59.11, and the pointer was relinked to it. Permission Setup merged each agent's seed, removing the retired entries and adding the current ones: 13 removed and 39 added on Claude, 10 and 25 on Auggie, every changed entry a seed entry, with no hook and no MCP entry. The self-update check then found Claude's `ductus.md` `current` and Auggie's `stale`, since `install.sh` refreshes only the agent it names; it overwrote Auggie's and stopped at Pre-flight abort, asking for a new session and a re-run, before Per-Agent Scaffolding and `merge-status-hook`; the adopter's commits for the two steps are at 14:17:49Z and 14:24:41Z. Task 29 records that abort.

**The upgrade, second run.** A third subagent, as the new session the abort asked for, carried out `main`'s bootstrap again as `/ductus --ref=main`. Both `ductus.md` copies were `current`, Permission Setup found nothing to change, Branch 1 again took the configured binary with no warning, and five migrations ran, committed at 14:45:02Z, among them `remove-legacy-constitutions-region`, which removed the 0.59.11 template's unmaintained `<!-- ductus:constitutions -->` region from `CLAUDE.md`. Step 10's `merge-status-hook` returned `{"action":"updated","claude-version":"2.1.319","reason":null}`, and `.claude/settings.local.json` gained one `UserPromptExpansion` entry, matcher `scratch:status`, command `"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus" hook status`, timeout 10; nothing else had written that file since the first run. So the hook lands on an upgrade with no manual step beyond the re-run every bootstrap change asks for (AC26, Claude Code's half). `.augment/settings.local.json` was byte-identical to its state after the first run and has no `hooks` key, so the hook stays Claude's; its only changes across the upgrade are the first run's seed entries. Auggie's 17 copies were updated, none carrying the ToolSearch paragraph (AC10). Claude's `apply-manifest` call halted before any write, `invalid 'entries': an entry sets tool-search-line, and the source root holds no commands/target.md naming the tools that load with the session`, because the lookup reads `{source-root}/commands/target.md` and the bootstrap's source root holds it at `framework/commands/target.md`. So Claude's 17 copies stayed 0.59.11's, and AC29 does not hold on an adopter's upgrade; task 30 fixes it.

**The upgrade, third run, with task 30's fix.** After `apply-manifest` came to take target's tools from the `commands/target.md` its manifest installs, this checkout's release build was rebuilt with the fix, and a fourth subagent carried out `main`'s bootstrap once more as `/ductus --ref=main`, committed in the adopter at 15:19:30Z. Claude's `apply-manifest` call updated all 17 copies; 14 carry the ToolSearch paragraph, help, link and target none, and all 17 are byte-identical to the staging check's output above, so the two layouts now give one result (AC29). Auggie's 17 were `unchanged`, none carrying the line. `merge-status-hook` returned `unchanged` with the one entry in place, and both settings files were byte-identical to their state before the run.

### Pi's registered `/{project}-status` (task 12)

**Method.** On 2026-11-09 at 14:21 to 14:24 UTC, pi 1.0.4 ran in a scratch project under `/tmp` holding one draft spec, `[host] project = "scratch"`, a `.ductus/bin/ductus` pointer to this checkout's release build, the working-tree bridge as `.pi/extensions/ductus.ts`, and `framework/commands/status.md` installed as the prompt template `.pi/prompts/scratch-status.md`, as `/ductus` installs it. Both runs passed `--approve` and `--no-session` and ran without this session's `CLAUDE_CODE_SESSION_ID`. The first drove `pi --mode rpc`: `get_commands`, the prompts `/scratch-status` and `/scratch:status`, then `get_messages`. The second drove the interactive TUI under `expect` (`--tui-mode regular`, 160 by 60), typed `/scratch-status`, and logged every byte pi wrote.

**Command or template.** `get_commands` listed `scratch-status` twice, once with source `extension` (the bridge) and once with source `prompt` (the template). `/scratch-status` returned disposition `handled` and emitted one `custom` message, `customType` `ductus-status`, `display: true`, holding the view, with no `agent_start` or turn after it. So the registered command runs, not the same-named template, and `/ductus` keeps installing the template. `/scratch:status`, the colon form the view's hints spell, returned `started` and went to the model as plain user text, so on Pi the hints name no command.

**Markdown.** The TUI drew the message under a `[ductus-status]` label, with the table rendered as a box-drawn table of aligned columns and the summary lines as paragraphs, so a displayed message renders markdown and `render-status` needs no `format` argument. The footer's cost stayed `$0.000`.

**Context.** Not measured. The one model run, from `/scratch:status`, ended with `stopReason` `error`: pi's stored Anthropic OAuth refresh token had expired (`invalid_grant`), so no request reached a model and nothing shows whether the custom message joined its context. pi 1.0.4's `docs/message-types.md` says a custom message's content is converted to a user message before the next model request, and `display` governs only rendering; that is the claim a run with working credentials is to confirm.
