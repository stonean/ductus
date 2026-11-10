# 080 — Deterministic runtime efficiency Research

The measurements and verifying runs spec 080's criteria point at. Each figure states its method, units, population and date (§grounding), and each section names the command that reproduces it.

## Method traps

Ways a measurement of this work went wrong, each of which inflates or misattributes a figure:

- **Size a tool result as the characters of its result text, not as `json.dumps` of the content.** In a session transcript the content is a JSON value, and serializing it again counts its escaping: for `dashboard` that read 41–44 KB against the true 37,171–40,454 characters.
- **A command the agent starts through the Skill tool leaves no `<command-name>` tag in a transcript.** Attributing each `ToolSearch` to the running command takes the Skill `tool_use` entries as well as the tags.
- **A loaded tool reaches the model as its description and input schema only.** The output schemas, 228,054 of 366,331 schema characters when the spec was clarified, never do, so a loaded-schema size counts description plus input schema, measured from `ductus mcp` `tools/list` over stdio, never from a transcript.
- **A `Read` result is not the file's size.** Claude Code's `Read` prefixes each line with its number, so analyze's 49,132-byte reference came back as 50,087 characters of result: over the 50,000-byte read size by that count, under it by the file's. Compare a file with the read size by its bytes (`wc -c`).

## Result sizes (AC14)

**Method.** `scripts/measure-result-sizes.sh` runs the ignored `result_sizes` test in `runtime/tests/result_sizes.rs`. It generates two corpora with `runtime/tests/common/corpus.rs`, sized as the report's first line says, and calls every name in `PRIMITIVE_REGISTRY` through the built binary's MCP server over stdio. Each call runs on a fresh copy of its corpus, so a writing primitive changes nothing the next one reads. Per-feature primitives are pointed at the subject spec, `001-corpus-feature-001`, which is `in-progress` with every task ticked; `traverse-deps` is pointed at the last spec, whose chain runs through every other. Every slug holds the word `corpus`, so a `derive-routing-candidates` description naming it is routed to every spec. Every spec carries one markdown-lint violation and one broken link to a file outside the spec root. A primitive that returns findings grows with the defects in what it reads, and on a defect-free corpus that growth would not show. For each ten specs the corpus also holds one branch-scoped spec with a `folds-into:`, so the staging inventory scales as well. `check-orphaned-references` reads only a fixed set of adopter-owned files (`CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit` and the spec root's `system.md`), not the spec corpus, so its result cannot grow with the corpus by its contract.

An argument that widens what a primitive reads gets a row of its own, since a result can grow in a mode the default call never takes: `read-spec` with `include-body`, the `all` walks of `prune-tasks` and `prune-plan`, `check-corpus-links` with `scope: repository`, and `discover-rule-files` with `rules`, as review calls it, and with `all`, `rules` and `verification`, as analyze calls it. The corpus is in sync and acyclic, so what grows only with drift or a cycle has rows measured on a copy that has it: `derive-dependencies` with every spec's `dependencies:` emptied while its body still links its predecessor, and `derive-dependencies` and `traverse-deps` with the chain closed into one cycle through every spec. The corpus holds one rule file, `quality-cross.md`, at every size, so those two rows measure that file's rules, not the spec corpus. Every other optional argument narrows a read, names a write, or carries the host's input. `render-help` is called after Claude's help copy is installed in the corpus, as `/ductus` installs it, since its result is that copy's guide. The Small and Large columns are measured with `[artifacts] read-size-bytes` raised past every result, so a bounded result is measured by what it carries rather than by its file form. The third column measures the large corpus at the default read size.

**Units.** Characters of the result's text, the `text` of the `tools/call` result's content, which is what a host reads.

**Population.** Every registry name, and each argument mode above. A name the fixture cannot exercise has a *not measured* row with its reason.

**Grows with the corpus.** A result grows when its size on the large corpus is at least twice its size on the small one and at least 200 characters more. A count's extra digits pass neither test, and a list that scales with the corpus passes both. *Bounded* means the result carries `bounded_lists`' `form` field, so it moves its lists to a file past `read-size-bytes`; `render-status` carries it too, beside the view it cuts to the read size (AC24).

### Measurement

Measured by `scripts/measure-result-sizes.sh` at e48033c659fa739acb8caa4ce671606b6a482537, with a clean tree, at 2026-11-10T20:31:42Z.

Small corpus: 10 specs, each with 5 criteria, 5 tasks of 3 subtasks and 2 scenarios, one markdown-lint violation and one broken link; 5 inbox items. Large corpus: 100 specs, each with 50 criteria, 50 tasks of 3 subtasks and 20 scenarios, one markdown-lint violation and one broken link; 50 inbox items. Small and Large are measured at a read size of 1000000000 bytes, past every result; Large at the default read size is the same corpus at 50,000.

| Primitive | Small (chars) | Large (chars) | Large at the default read size (chars) | Grows | Bounded | Note |
| --- | --- | --- | --- | --- | --- | --- |
| `read-spec` | 1145 | 6174 | 6174 | yes | yes | |
| `read-spec` with `include-body` | 2380 | 18140 | 18140 | yes | yes | |
| `read-tasks` | 1785 | 17287 | 17287 | yes | yes | |
| `mark-task` | 79 | 79 | 79 | no | no | |
| `mark-criterion` | 79 | 79 | 79 | no | no | |
| `set-status` | 89 | 89 | 89 | no | no | |
| `derive-boundary` | 421 | 2717 | 2717 | yes | yes | |
| `discover-rule-files` | 110 | 110 | 110 | no | no | |
| `discover-rule-files` with `rules` | 2606 | 2606 | 2606 | no | yes | |
| `discover-rule-files` with `all`, `rules` and `verification` | 9344 | 9344 | 9344 | no | yes | |
| `process-waivers` | 54 | 54 | 54 | no | no | |
| `process-decisions` | 172 | 172 | 172 | no | yes | |
| `compute-review-scope` | 1631 | 35970 | 35970 | yes | yes | |
| `write-review` | 359 | 360 | 360 | no | no | |
| `write-analysis` | 172 | 172 | 172 | no | no | |
| `check-step-references` | 255 | 255 | 255 | no | no | |
| `check-stuck` | 101 | 101 | 101 | no | no | |
| `validate-frontmatter` | 28 | 28 | 28 | no | no | |
| `resolve-anchor` | 85 | 85 | 85 | no | no | |
| `traverse-deps` | 196 | 192 | 192 | no | yes | |
| `traverse-deps` with a cycle through every spec | 448 | 2694 | 2694 | yes | yes | |
| `check-rule-ids` | 58 | 58 | 58 | no | no | |
| `run-generator` | — | — | — | not measured | — | CLI-only: it runs a project script and is no MCP tool, so no host reads its result as a tool result |
| `lint-markdown` | 1442 | 13783 | 13783 | yes | yes | |
| `gate-confirm` | 70 | 70 | 70 | no | no | |
| `fetch-archive` | — | — | — | not measured | — | it needs the network |
| `extract-archive` | — | — | — | not measured | — | it needs a release archive, which a spec corpus does not hold |
| `apply-manifest` | — | — | — | not measured | — | its result follows the manifest the host builds and needs a staged framework tree; a spec corpus holds neither |
| `enforce-manifest` | — | — | — | not measured | — | its result follows the expected-file list the host passes, not the spec corpus |
| `merge-managed-block` | 163 | 163 | 163 | no | no | |
| `merge-permissions` | 245 | 245 | 245 | no | no | |
| `merge-status-hook` | 61 | 61 | 61 | no | no | |
| `migrate-session-file` | 114 | 114 | 114 | no | no | |
| `relocate-audit-records` | 68 | 68 | 68 | no | no | |
| `create-scenario` | 66 | 66 | 66 | no | no | |
| `append-task` | 96 | 97 | 97 | no | no | |
| `label-criteria` | 96 | 97 | 97 | no | no | |
| `prune-tasks` | 947 | 6928 | 6928 | yes | yes | |
| `prune-tasks` with `all` | 1420 | 13563 | 13563 | yes | yes | |
| `prune-plan` | 235 | 235 | 235 | no | no | |
| `prune-plan` with `all` | 150 | 658 | 658 | yes | yes | |
| `prune-project` | 424 | 424 | 420 | no | no | |
| `dashboard` | 3820 | 34932 | 34932 | yes | yes | |
| `render-status` | 2140 | 12881 | 12881 | yes | yes | |
| `render-help` | 6223 | 6223 | 6223 | no | no | |
| `read-section` | 555 | 4012 | 4012 | yes | yes | |
| `write-session` | 46 | 46 | 46 | no | no | |
| `resolve-session` | 60 | 60 | 60 | no | no | |
| `retarget-sessions` | 46 | 46 | 46 | no | no | |
| `resolve-references` | 147 | 147 | 147 | no | yes | |
| `resolve-constitutions` | 60 | 60 | 60 | no | no | |
| `resolve-feature` | 134 | 134 | 134 | no | no | |
| `create-feature` | 130 | 130 | 130 | no | no | |
| `create-plan-artifacts` | 217 | 217 | 217 | no | no | |
| `check-review-freshness` | 93 | 93 | 93 | no | no | |
| `check-review-gate` | 1000 | 6628 | 6628 | yes | yes | |
| `classify-spec-edit` | 28 | 28 | 28 | no | no | |
| `measure-reopenings` | 541 | 4315 | 4315 | yes | yes | |
| `append-question` | 142 | 142 | 142 | no | no | |
| `diff-cross-spec` | 490 | 4046 | 4046 | yes | yes | |
| `append-inbox` | 56 | 57 | 57 | no | no | |
| `remove-inbox-item` | 60 | 61 | 61 | no | no | |
| `check-artifacts` | 698 | 5199 | 5199 | yes | yes | |
| `check-artifact-size` | 231 | 232 | 228 | no | no | |
| `derive-routing-candidates` | 2287 | 20554 | 20554 | yes | yes | |
| `check-corpus-links` | 4703 | 147494 | 295 | yes | yes | |
| `check-corpus-links` with `scope: repository` | 4702 | 147493 | 295 | yes | yes | |
| `check-orphaned-references` | 181 | 181 | 181 | no | no | |
| `check-command-flags` | 263 | 263 | 263 | no | no | |
| `check-promotion-coverage` | — | — | — | not measured | — | it audits the framework's own rule file against command sections, which a spec corpus does not hold |
| `derive-dependencies` | 294 | 295 | 295 | no | yes | |
| `derive-dependencies` with every spec drifted | 643 | 4155 | 4155 | yes | yes | |
| `derive-dependencies` with a cycle through every spec | 545 | 2796 | 2796 | yes | yes | |
| `derive-references` | 168 | 169 | 169 | no | no | |
| `check-unfolded-specs` | 174 | 1177 | 1177 | yes | yes | |
| `rewrite-spec-links` | 991 | 7744 | 7744 | yes | yes | |
| `retire-feature` | 54 | 54 | 54 | no | no | |
| `invalidate-review` | 69 | 69 | 69 | no | no | |
| `start-fix-rounds` | 73 | 75 | 75 | no | no | |
| `brief-re-read` | 1055 | 18111 | 18111 | yes | yes | |
| `read-re-read` | 323 | 323 | 323 | no | yes | |
| `check-prescriptions` | 413 | 413 | 413 | no | yes | |

### Grows with the corpus

Of the 73 registry names, 21 grow with the corpus by the test above, in 25 of the table's rows, and every one is bounded.

- **Bounded through `bounded_lists` by this spec, so AC7's population (16):** `read-spec`, `read-tasks`, `read-section`, `derive-boundary`, `lint-markdown`, `prune-tasks`, `prune-plan`, `dashboard`, `check-review-gate`, `check-artifacts`, `check-corpus-links`, `check-unfolded-specs`, `rewrite-spec-links`, `derive-routing-candidates`, `derive-dependencies`, `traverse-deps`. The first measurement, at 2b4eaddb, called each primitive in its default mode only and found 11 of them; `prune-plan` grows only in its `all` walk. `read-section`, which task 37 added bounded, joined at task 41's re-measure, and the last three at task 50's: routing grows only with a description the slugs match, and `derive-dependencies` and `traverse-deps` only over a corpus that drifts or holds a cycle.
- **Bounded through `bounded_lists` before this spec (4):** `compute-review-scope`, `measure-reopenings`, `diff-cross-spec`, `brief-re-read`.
- **Bounded in a shape of its own (1):** `render-status`, which cuts the view to the read size and writes the whole view to its list file (AC24). Before task 44 its bound cut only the table rows and measured the rendering, so 1,701 planned specs on 31-character slugs returned 56,752 characters.

## Loaded schema sizes (AC23)

**Method.** The same run lists the tools with `tools/list` and sizes each as its description's characters plus its input schema serialized as compact JSON, which is the form the server sends. Rows are in descending order of the loaded size.

**Reconciliation with the spec's figure.** Spec 080's Resolved Questions record 132,846 characters across 68 tools. The first measurement, at 2b4eaddb, totalled 129,531 for the same 68 tools in the compact form. The same `tools/list`, read from `runtime/target/release/ductus mcp` over stdio on 2026-11-08, also totalled 129,531 compact, and 132,353 when Python's `json.dumps` serializes the input schemas with its default separators (`", "` and `": "`). The spec's figure was therefore taken with the default separators, and the remaining 493 characters are description changes made since it was taken. The measurement at 4294b79a added `render-status` (task 4) and the sentence each bounded primitive's description gained on its file form (task 5), for 134,535 characters across 70 tools. The measurement at 4025732d, after tasks 34 to 40, totalled 140,524 across 72, 5,989 more:

- `render-help` (task 34) and `read-section` (task 37) are new, at 855 and 825.
- Four tools changed with this spec's tasks: `discover-rule-files` by 1,391 (task 38), `apply-manifest` by 1,378 (the ToolSearch line, tasks 10, 30 and 36), `merge-permissions` by 820 (task 35) and `merge-status-hook` by 216 (help's entry, task 34).
- The other 504 are on six tools other specs' work changed since 4294b79a, by `git log` of `runtime/src/schema/`: 058 changed `write-analysis`, 062 `dashboard`, 078 `brief-re-read`, `check-prescriptions` and `write-review`'s input schema, and 079 `check-review-freshness` and `write-review`'s description.

The measurement at a11079af, after tasks 44 to 52, totalled 142,809 across the same 72, 2,285 more, every change this spec's own (e66f79c2, the only commit to `runtime/src/schema/` since 4025732d): `discover-rule-files` by 408 (task 48), `apply-manifest` by 405 (task 46), `derive-dependencies` by 398, `derive-routing-candidates` by 263 and `traverse-deps` by 223 (each bounded by task 50, routing's naming groom too by task 52), `render-status` by 273 (task 44), `dashboard` by 119 (task 52), `render-help` by 106 (the namespace Pi's bridge reads, task 47) and `read-spec` by 102 (task 45), less 4 each on `process-decisions`, `check-prescriptions` and `read-re-read`, whose descriptions now call their list files pretty-printed rather than one entry per line (task 49). The one below, at 12a7cede, totals 56 more, all on `discover-rule-files`, whose `other-headings` entries gained `body` (task 57); 022's 92120f45, the other commit to `runtime/src/schema/` since a11079af, changed no tool's loaded schema. Of the result rows, only `discover-rule-files` with `all`, `rules` and `verification` moved, from 8,762 to 9,344 characters.

The measurement at e48033c6, after the closing review's fixes (567e6399), totals 143,069 across the same 72, 204 more, each a description the review corrected: `dashboard` by 62, `render-help` by 42, `check-corpus-links` and `check-unfolded-specs` by 41 each, and `render-status` by 18.

### Measurement

Measured by `scripts/measure-result-sizes.sh` at e48033c659fa739acb8caa4ce671606b6a482537, with a clean tree, at 2026-11-10T20:31:42Z.

72 tools, 143069 characters in all.

| Tool | Description | Input schema | Loaded |
| --- | --- | --- | --- |
| `write-review` | 3577 | 17010 | 20587 |
| `write-analysis` | 1565 | 8721 | 10286 |
| `rewrite-spec-links` | 2614 | 1486 | 4100 |
| `apply-manifest` | 1336 | 2667 | 4003 |
| `compute-review-scope` | 3217 | 704 | 3921 |
| `process-decisions` | 1563 | 1964 | 3527 |
| `append-task` | 234 | 3222 | 3456 |
| `check-corpus-links` | 2076 | 1365 | 3441 |
| `retire-feature` | 1522 | 1638 | 3160 |
| `prune-project` | 986 | 2089 | 3075 |
| `write-session` | 970 | 2097 | 3067 |
| `merge-permissions` | 554 | 2324 | 2878 |
| `derive-references` | 1927 | 735 | 2662 |
| `derive-dependencies` | 1577 | 1063 | 2640 |
| `discover-rule-files` | 1264 | 1209 | 2473 |
| `measure-reopenings` | 1980 | 460 | 2440 |
| `prune-plan` | 1173 | 1242 | 2415 |
| `check-review-gate` | 2109 | 211 | 2320 |
| `check-prescriptions` | 1904 | 368 | 2272 |
| `check-artifacts` | 1990 | 211 | 2201 |
| `create-feature` | 330 | 1749 | 2079 |
| `derive-routing-candidates` | 1303 | 742 | 2045 |
| `retarget-sessions` | 588 | 1443 | 2031 |
| `process-waivers` | 544 | 1438 | 1982 |
| `brief-re-read` | 1555 | 350 | 1905 |
| `classify-spec-edit` | 1560 | 339 | 1899 |
| `resolve-constitutions` | 1471 | 427 | 1898 |
| `dashboard` | 1677 | 74 | 1751 |
| `read-re-read` | 1316 | 314 | 1630 |
| `prune-tasks` | 573 | 973 | 1546 |
| `append-question` | 666 | 852 | 1518 |
| `check-promotion-coverage` | 138 | 1360 | 1498 |
| `render-status` | 1384 | 74 | 1458 |
| `invalidate-review` | 1159 | 291 | 1450 |
| `diff-cross-spec` | 1200 | 211 | 1411 |
| `create-plan-artifacts` | 514 | 860 | 1374 |
| `check-step-references` | 1293 | 74 | 1367 |
| `check-artifact-size` | 1094 | 211 | 1305 |
| `merge-status-hook` | 987 | 299 | 1286 |
| `resolve-references` | 1007 | 243 | 1250 |
| `enforce-manifest` | 66 | 1089 | 1155 |
| `check-unfolded-specs` | 1013 | 74 | 1087 |
| `resolve-feature` | 442 | 622 | 1064 |
| `check-orphaned-references` | 969 | 74 | 1043 |
| `check-command-flags` | 938 | 74 | 1012 |
| `create-scenario` | 193 | 810 | 1003 |
| `render-help` | 929 | 74 | 1003 |
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
| `read-section` | 506 | 319 | 825 |
| `lint-markdown` | 263 | 527 | 790 |
| `check-review-freshness` | 552 | 211 | 763 |
| `read-spec` | 443 | 300 | 743 |
| `append-inbox` | 258 | 407 | 665 |
| `extract-archive` | 72 | 546 | 618 |
| `remove-inbox-item` | 224 | 373 | 597 |
| `gate-confirm` | 289 | 273 | 562 |
| `set-status` | 136 | 330 | 466 |
| `traverse-deps` | 281 | 180 | 461 |
| `read-tasks` | 246 | 195 | 441 |
| `check-rule-ids` | 64 | 352 | 416 |
| `label-criteria` | 163 | 211 | 374 |
| `validate-frontmatter` | 55 | 187 | 242 |

## Command cost (AC8)

**Method.** On 2026-11-09 from 17:13:01Z to 17:21:23Z, `python3 scripts/measure-command-cost.py --round --permissions adopter --specs 40 --raw <dir> --out <file>` at 14c88044 ran each of the 17 commands `/ductus` installs for Claude Code once, through Claude Code 2.1.319 (`claude -p`, `stream-json`, model `claude-opus-5-5`), against this checkout's release build. That build was current: no `runtime/` change landed after it. Each command ran on a fixture of its own. Each fixture is the harness's generated 40-spec corpus, 8 specs per status, holding the tree `/ductus` installs for Claude, read from `ductus.md`'s own tables, with the status hook absent, so status's figure is the model path's. Its `.claude/settings.local.json` holds an adopter's permissions: the bootstrap seed, then `/ductus:configure`'s canonical sets (owner, 2026-11-09). On that fixture sat the command's preset in the script's `ROUND`, its prompt, session target and seed, the least state its main path needs. The child ran with stdin closed, without this shell's Claude Code session variables, with setting sources `project,local` and `--strict-mcp-config`. Whether the fixture's `CLAUDE.md`, which imports `.ductus/constitution.md` and `AGENTS.md`, loaded in the child is not measured. The stream's init event names only the auto-memory path, and Claude Code's docs tie `--setting-sources` to `CLAUDE.local.md` and project rules, not to the root `CLAUDE.md`. `audit.md` is not measured: `/ductus` installs no copy of it, so the fixture has no `/fixture:audit`.

**Units.** First output and result are seconds of wall time from launch, to the first assistant text delta and to the result event. Output tokens are the result's `usage.output_tokens`. Largest tool result is the most characters of text any one tool result carried. Input tokens are the result's `input_tokens`, `cache_creation_input_tokens` and `cache_read_input_tokens` summed over the run's model requests. Turns are its `num_turns`. Cost is its `total_cost_usd`, at list price. Under `-p` a command cannot be answered, so a run that reaches a gate or a question ends there. *Ended at* says where each run ended, from its final reply and its last tool call. Every run's result subtype was `success`, and Claude Code listed every command among its slash commands.

| Command | Preset (prompt; target; seed) | Ended at | First output (s) | Result (s) | Output tokens | Largest tool result (chars) | Input tokens | Turns | Cost (USD) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| help | none | its end, the guide printed | 2.841 | 16.653 | 2,204 | 0 | 43,450 | 1 | 0.270 |
| status | none | its end, the view emitted | 13.626 | 20.036 | 2,134 | 3,853 | 131,190 | 3 | 0.300 |
| target | `3` | its end, the target written | 11.663 | 11.680 | 684 | 822 | 186,722 | 8 | 0.297 |
| log | one item | its end, the item appended | 9.590 | 9.605 | 357 | 81 | 171,350 | 4 | 0.256 |
| specify | one description | step 9's gate, after writing `041`'s `spec.md` | 7.618 | 38.966 | 3,133 | 5,709 | 398,177 | 11 | 0.443 |
| clarify | 005 (draft), one open question | step 6, asking the question | 4.663 | 15.030 | 1,112 | 11,859 | 204,281 | 7 | 0.364 |
| plan | 001 (clarified) | step 9's gate, after writing `plan.md` and `tasks.md` | 5.782 | 42.648 | 3,606 | 9,781 | 468,306 | 18 | 0.486 |
| implement | 002 (planned), a concrete task and a `src/` commit | the walk's end, the task written, `make test` passing, nothing committed | 5.502 | 28.088 | 2,118 | 1,061 | 403,475 | 17 | 0.412 |
| review | 003, taken to done, reopened and implemented | its end, `review.md` written, not blocking | 6.919 | 91.777 | 6,286 | 7,808 | 1,502,864 | 27 | 0.925 |
| analyze | 004 (done) | step 19's first gating question, before `write-analysis` | 32.766 | 80.132 | 6,446 | 50,087 | 1,266,279 | 27 | 1.072 |
| amend | 003, one question | the approval gate, nothing recorded | 14.601 | 14.618 | 847 | 387 | 155,170 | 5 | 0.342 |
| groom | none; one inbox item | step 3's gate on the item | 25.728 | 25.744 | 1,860 | 10,775 | 361,247 | 11 | 0.416 |
| prune | 004 (done) | step 6's gate | 5.608 | 14.531 | 954 | 401 | 195,366 | 6 | 0.326 |
| consolidate | `005-feature-5 --into 001-feature-1` | step 3's gate | 16.456 | 16.472 | 1,363 | 1,025 | 199,291 | 8 | 0.350 |
| fold | `cost.1-csv-export`; a branch-scoped spec folding into 001 | step 5's gate | 15.346 | 22.529 | 1,967 | 5,210 | 264,759 | 9 | 0.411 |
| link | one alias, repo and path | its end, the service registered | 11.400 | 11.412 | 870 | 239 | 131,718 | 4 | 0.268 |
| configure | none | its end, step 5 not written | 28.847 | 28.859 | 3,047 | 7,172 | 252,649 | 6 | 0.414 |

The round cost $7.352. analyze's population effect is the corpus's: on the generated `done` spec, its own reading found three blocking findings (placeholder criteria, and a plan with neither Technical Decisions nor Affected Files), so it stopped at the first one's deferral question. Its largest result is `.ductus/reference/analyze.md` read whole in one call.

**Denied calls.** Six calls in four runs, from `permission_denials`:

- review: a `for` loop running `cat -n` on each file, which no rule approves as a compound command, and `rm -r src/__pycache__`, which the deny set refuses. Neither stopped the run.
- analyze: a `grep … | wc` pipeline and a `for` loop of `grep -c`, both compound. Neither stopped the run.
- groom: `grep -il …; echo "exit=$?"`, compound. It did not stop the run.
- configure: step 5's `Edit` of `.claude/settings.local.json`, though the file allows `Edit`. The run reported the step not written.

**The search step.** The `ToolSearch` line's call costs a model step only when the call goes out alone. status's, log's and specify's did: 1.51 s, 1.52 s and 1.70 s from its result to the next model message, by the stream's `assistant` and `user` timestamps. The other ten sent it in the same message as their first step's call, so their next message followed in 0.00 to 0.38 s. That call was `resolve-session`, which loads with the session, for eight of them. consolidate's was two `resolve-feature` calls, and fold's was `check-unfolded-specs` beside `resolve-feature`. `check-unfolded-specs` is deferred, and its call in the search's own message returned its result, so a deferred tool can go out with the search that loads it. Where a command's first call was a deferred tool, the search went out alone in 4 of 5 runs: status's, log's, specify's and configure's. configure's 13.28 s is the model composing `merge-permissions`' 132 entries, not the search. plan made a second search, for `derive-boundary`, which its reference's §Validation gate names and its steps do not, at 1.68 s. help, target and link made none.

### Each command's reduction (task 18)

The reductions the round's figures support, in the spec's order of preference: no model turn, then less read, then less written.

- **R1. help answers with no model turn.** Its whole cost is retyping a static guide: 6,146 characters of reply, 2,204 output tokens, 16.7 s and $0.270, with no tool call.
- **R2. configure's model types no permission entries.** It passed `merge-permissions` 132 entries, 99 to allow, 20 to deny and 13 to revoke, in 3,897 characters of arguments and a 13.28 s step. The primitive already holds the canonical allow set as its ceiling. Its step-5 `Edit` of `.claude/settings.local.json` was denied with `Edit` allowed, so an interactive session is asked on every run.
- **R3. One search covers the tools a command's referenced sections call.** plan searched a second time for `derive-boundary`, which its reference's §Validation gate calls and its steps do not name: 1.68 s and a model step. A static diff of each installed copy's line against the primitive names in its reference finds such names for 11 commands, most of them prose mentions. Which ones a run calls depends on its path, and the round ran one path per command.
- **R4. A command reads only the reference sections its path cites.** analyze read its whole reference, 49,132 bytes as installed, in one call returning 50,087 characters with the reader's line numbers, its fourth of 15 model messages, so the 11 after it carried it. clarify read 11,859 characters, groom 10,775, plan 9,781 and fold 5,210, each whole. review listed the headings and read 3,603.
- **R5. The rules a review or an analysis loads reach the model without shell reads.** review read the rule files in four shell calls: their sizes, their heading counts, their headings, and the Statements of five rules. analyze read them in three, and its `grep` of every rule's Verification line came back as a 44,805-character saved output, read back whole. review needs each rule's ID, tier and Statement; analyze also needs its Verification.
- **R6. groom finds the covering spec with the primitive specify uses.** It searched spec bodies with three `grep` calls, one denied, where specify's `derive-routing-candidates` derives candidates for the same decision tree.
- **R7. The search goes out with the first call.** Where a command's first call was a deferred tool, the search went out alone in 4 of 5 runs, and status's, log's and specify's searches each cost a model step of 1.51 to 1.70 s, by *The search step* above. fold's deferred first call went out in the search's own message and returned its result, so the step is the model's choice, not the host's. Found while verifying task 19, and recorded with the six (owner, 2026-11-09).

A *none* below holds for the path measured. For a run that ended at a gate, the work past the gate was not measured.

| Command | Reduction | Why, from the round |
| --- | --- | --- |
| help | R1 | the guide retyped, as above |
| status | R7, beyond AC1 | the hook answers with no model turn; on the model path, kept for hosts without one, its search went out alone |
| target | none | 11.7 s and 684 output tokens, its tools loaded with the session, and loading the constitutions into the model's context is the reason it runs |
| log | R7 | 9.6 s and 357 output tokens, its search alone; its one judgment is whether the item is clear enough to act on later |
| specify | R7 | its search went out alone; most of its 3,133 output tokens are the spec it writes, 4,459 characters of tool arguments |
| clarify | R4 | its question is judgment; measured to step 6 |
| plan | R3, R4 | the plan and tasks it writes are its output |
| implement | none | its output is the code and the test, with `make test` run once; it read no reference |
| review | R4, R5 | 91.8 s, 1,502,864 input tokens over 27 turns |
| analyze | R4, R5 | 80.1 s, 1,266,279 input tokens; measured to step 19's gate |
| amend | none measured | 14.6 s to its approval gate, its recording unmeasured |
| groom | R4, R6 | measured to step 3's gate |
| prune | none measured | 14.5 s to step 6's gate |
| consolidate | none measured | 16.5 s to step 3's gate |
| fold | R4 | measured to step 5's gate |
| link | none | 11.4 s and 870 output tokens for one additive write |
| configure | R2, R7 | as above |

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

**Command or template.** `get_commands` listed `scratch-status` twice, once with source `extension` (the bridge) and once with source `prompt` (the template). `/scratch-status` returned disposition `handled` and emitted one `custom` message, `customType` `ductus-status`, `display: true`, holding the view, with no `agent_start` or turn after it. So the registered command runs, not the same-named template, and `/ductus` keeps installing the template. `/scratch:status`, the colon form the view's hints spell, returned `started` and went to the model as plain user text, so on Pi the hints named no command, until task 28's bridge respelled each as `/{project}-<name>` (owner, 2026-11-09).

**Markdown.** The TUI drew the message under a `[ductus-status]` label, with the table rendered as a box-drawn table of aligned columns and the summary lines as paragraphs, so a displayed message renders markdown and `render-status` needs no `format` argument. The footer's cost stayed `$0.000`.

**Context.** Waived, not measured: the owner has no pi account to run a model turn with, and Pi support was contributed (owner, 2026-11-09). Both attempts ended before a model request with `stopReason` `error`, pi's stored Anthropic OAuth refresh token having expired (`invalid_grant`): the first above, and one at 16:00:38Z on pi 1.5.0, to which pi had updated itself, where `/scratch-status` again returned `handled` and showed the view as a `ductus-status` custom message with no turn. So no model turn observed whether the view joins the agent's context, and the answer is pi's documentation's (AC27, owner 2026-11-10): it does. pi's `docs/message-types.md` says a custom message's content is converted to a user message before the next model request and `display` governs only rendering, which is also how `sendMessage` with `triggerTurn: false` adds context without a turn. AC28's compact form is conditional on this record showing the view did not reach the context, which it does not, so the bridge adds none (task 13).

### Pi's commands while the view fails (task 47)

**Method.** On 2026-11-10 at about 15:15 UTC, pi 1.5.0 ran `pi --mode rpc --approve --no-session`, without this session's `CLAUDE_CODE_SESSION_ID`. Its scratch project held one spec whose frontmatter does not parse (`status: [`), so `render-status` fails, and `[host] project = "scratch"`. It also held `cli-config-dir = ".pi"` in `.ductus/session.toml`, a `.ductus/bin/ductus` pointer to this checkout's release build, and the working-tree bridge as `.pi/extensions/ductus.ts`. `help.md` and `status.md` were installed as `.pi/prompts/scratch-help.md` and `scratch-status.md`, `{project}` substituted, as `/ductus` installs them. The tree was HEAD c0090a22 with task 47's uncommitted bridge and runtime changes. The run sent `get_commands`, then the prompts `/scratch-status` and `/scratch-help`.

**Result.** `get_commands` listed `scratch-status` and `scratch-help` each with source `extension`, beside the two templates, so both commands registered though the view fails. `/scratch-status` returned disposition `handled` and emitted one `ductus-status` custom message with `display: true`, reading `ductus: scratch-status could not show the view: YAML parse error in …/specs/001-broken/spec.md`. `/scratch-help` returned `handled` and emitted the guide as a `ductus-help` message. Neither emitted `agent_start` or a turn, and no notice was raised.

### OpenCode's TUI-plugin slash command (task 14)

**Method.** On 2026-11-09 from 16:19:45 to 16:22:33 UTC, OpenCode 1.17.8 ran its TUI under `expect` (160 by 60) in a scratch project under `/tmp` holding one draft spec, `[host] project = "scratch"`, a `.ductus/bin/ductus` pointer to this checkout's release build, and `framework/commands/status.md` installed as `.opencode/command/scratch/status.md`, as `/ductus` installs it. `.opencode/tui.json` listed one TUI plugin, a module of `@opencode-ai/plugin` 1.17.8's `TuiPluginModule` shape (`{ id, tui }`), whose `tui` registered through `api.command.register` a command named `ductus-status-probe` with the slash aliases `scratch:status` and `scratch/status`. Its `onSelect` ran `.ductus/bin/ductus render-status`, wrote a marker file holding the time and the view, and showed the view's first lines with `api.ui.toast`. Typing `/scratch` listed `/scratch/status` with status.md's description and `/ductus-status-probe`, so OpenCode spells a namespaced command with a slash.

**Result.** Typing `/scratch:status` and Enter ran the plugin's handler, its marker stamped 16:20:13.638Z: the toast showed `ductus status` and the view's opening lines, the TUI stayed on its home prompt, and no session was created, so a TUI plugin's slash command answers with local code and no model turn. Typing `/scratch/status` instead, with an unknown model configured so that a turn could reach no provider, did not run the plugin, its marker unchanged: Enter selected the template's entry, and a second Enter submitted it, opening a new session holding status.md's prompt body. So an installed command keeps its name over a plugin's alias, and the view's `/scratch:` hints name no OpenCode command, as on Pi before task 28's bridge respelled them. The API 1.17.8 offers for a slash command, `api.command`, is marked deprecated in its types ("Remove in v2", in favor of `api.keymap.registerLayer`), and the TUI-plugin API opencode.ai documents now (`@opencode/plugin/tui`, `Plugin.define`, `cli.json`) is absent from the 1.17.8 binary.

### The `claude -p` harness's first run (task 16)

**Method.** On 2026-11-09 at 16:44:04Z and 16:44:23Z, `scripts/measure-command-cost.py --specs 40 /fixture:status` ran Claude Code 2.1.319 (`claude -p`, `stream-json`, model `claude-opus-5-5`) twice on its generated 40-spec fixture, whose command copies `apply-manifest` wrote in `/ductus`'s layout from this checkout's `framework/`, against this checkout's release build: first with no hook, then with `--hook`, which installs the status hook through `merge-status-hook`. The child ran with stdin closed and without this shell's Claude Code session variables, with setting sources `project,local`.

**A custom slash command under `-p`.** It runs: the init event listed `fixture:status` among the slash commands, and the model's first call was `ToolSearch` with the query `select:mcp__ductus__render-status`, the installed copy's ToolSearch line exactly, then `render-status`, and the result was the view. The stream carries no prompt, so the calls the body prescribes are the evidence that it was expanded. Three turns, first text at 11.878 s, result at 18.430 s, 2,147 output tokens, and a largest tool result of 3,893 characters.

**The status hook under `-p`.** It answers the prompt with no model turn: the result was `UserPromptExpansion operation blocked by hook:` followed by the text-rendered view, with 0 turns, 0 output tokens and no tool call, at 0.362 s. The stream carried no hook event, so the block is visible only in the result. A first run without stdin closed waited 3 s for piped input inside its timings, which the harness now closes.

### Help with no model turn (task 34)

**Method.** On 2026-11-09, at bfd9c6b0 with task 34's change uncommitted, this checkout's release build of that change, stamped 20:51:15Z, ran three ways:

- **The harness.** At 20:53:39Z, `python3 scripts/measure-command-cost.py --round --only help --hook --permissions adopter --specs 40` ran Claude Code 2.1.321 (`claude -p`, `stream-json`, model `claude-opus-5-5`) on its generated fixture, where `--hook` installs both hook entries through `merge-status-hook`.
- **pi.** At 20:53Z, pi 1.5.0 ran in RPC mode (`pi --mode rpc --approve --no-session`, without this shell's Claude Code session variables) in a scratch project under `/tmp`. The project held one draft spec, `[host] project = "scratch"`, `cli-config-dir = ".pi"` in its session file, a `.ductus/bin/ductus` pointer to the build, the working-tree bridge as `.pi/extensions/ductus.ts`, and `help.md` and `status.md` installed as `.pi/prompts/scratch-help.md` and `scratch-status.md`, `{project}` substituted. The run sent `get_commands`, the prompt `/scratch-help`, then `get_messages`.
- **This checkout's settings.** `runtime/target/release/ductus merge-status-hook --path .claude/settings.local.json` ran on this checkout's file, which held status's entry alone, then ran again.

**Result.** The harness's result was `UserPromptExpansion operation blocked by hook:` followed by the guide, with 0 turns, 0 output tokens, no tool call and $0 at 0.356 s. On the model path the round measured 16.653 s, 2,204 output tokens and $0.270. In pi, `get_commands` listed `scratch-help` twice, with source `extension` (the bridge) and source `prompt` (the template). `/scratch-help` returned disposition `handled` and emitted one `custom` message, `customType` `ductus-help`, `display: true`, holding the guide in 6,146 characters, with no turn after it. The guide spelled every command `/scratch-<name>`, with no `/scratch:` hint and no generator marker, so the registered command runs in place of the same-named template, as status's does. On this checkout's settings, the first call returned `updated` with `claude-version` `2.1.321`. It appended help's entry, matcher `ductus:help`, command `"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus" hook help`, timeout 10, and left status's entry and every other key as they were. The second call returned `unchanged`.

**Typed `/ductus:help`.** At 21:03:11Z the owner typed `/ductus:help` at this checkout's root, in the Claude Code 2.1.321 session whose settings that call had just updated. The session transcript holds one entry for it, at 21:03:11.145Z: a `system` entry with subtype `informational`, `level: warning` and `preventContinuation: true`. Its content is `UserPromptExpansion operation blocked by hook:` followed by exactly the guide `render-help` returns from the same build, 6,110 characters, with no "Original prompt:" line. The transcript holds no model request between the previous turn's end at 21:01:11.982Z and the next prompt at 21:03:13.547Z. The help hook therefore fires in the session that installed it, as status's did. Its failure path is the same exit 1 that task 8's runs took for status on this event, and `help_with_no_installed_copy_exits_1_with_the_error_on_stderr_alone` holds it on the runtime's side.

### Configure with no entries typed (task 35)

**Method.** On 2026-11-09 at 22:22:22Z, at 3965f6f3 with task 35's change uncommitted, `python3 scripts/measure-command-cost.py --round --only configure --permissions adopter --specs 40` ran Claude Code 2.1.321 (`claude -p`, `stream-json`, model `claude-opus-5-5`) against this checkout's release build of that change. The adopter fixture's settings now come from the bootstrap seed and then `merge-permissions --canonical`, as configure's step 1 now makes the call. Beforehand, the same build's canonical call ran twice on a copy of this checkout's `.claude/settings.local.json`, in a scratch project named `ductus`.

**Result.** The run's one `merge-permissions` call passed `{"path": ".claude/settings.local.json", "canonical": true}`, 58 characters of arguments, and returned `unchanged` with empty `revoked` and `directories-added`. The run had no permission denial and no host write of the settings file. It took 8.197 s to its result, 420 output tokens, 4 turns and 192,703 input tokens, at $0.428, its largest tool result 257 characters. The round's configure run had taken 28.859 s, 3,047 output tokens, 6 turns and 252,649 input tokens, at $0.414. This run wrote 48,803 input tokens to the prompt cache, and cache writes are billed above cache reads. On the copy of this checkout's file, the first call returned `updated`: one allow entry added, task 34's `mcp__ductus__render-help`, no retired entry, and both scratch directories appended. The second call returned `unchanged`.

### One search, sent with the first call (tasks 36 and 40)

**Method.** On 2026-11-09, at 9ebca7b0 with tasks 36 and 40's change uncommitted, `python3 scripts/measure-command-cost.py --round --only <commands> --permissions adopter --specs 40 --bin <copy>` ran Claude Code 2.1.321 (`claude -p`, `stream-json`, model `claude-opus-5-5`). Each run used a copy of this checkout's release build of the change. The fixture's copies were made by `apply-manifest` from the archive's layout, with `framework/reference/` staged. Two rounds ran:

- **Round 1**, 23:51:21Z to 23:54:19Z. It ran plan, status, log, specify, configure and fold, with the line worded "…in one `ToolSearch` call, sent in the same message as the command's first `ductus` call unless that call needs an earlier result: …". The binary's sha256 begins `9c74bcfc82612b8d`.
- **Round 2**, 23:58:48Z to 00:00:28Z on 2026-11-10. It ran the five whose first `ductus` call is a deferred tool, with the wording the generator now writes, "…in one `ToolSearch` call, and send the command's first `ductus` call in that same message, since a tool can be called beside the search that loads it: …". The binary's sha256 begins `328bd93a6c3579aa`.

Each run's searches, and the tools sent in each search's message, were read from its raw stream, grouping `assistant` events by message id. A lone search's step is the seconds from its result to the next `assistant` event.

**One search (task 36).** All 11 runs made one `ToolSearch` each. plan's line named `derive-boundary`, and plan called it at §Validation gate with no second search. In the cost round, plan had searched again for it and paid 1.68 s. Every result subtype was `success`, and no call was denied.

**The search with the first call (task 40).** These are the runs whose first `ductus` call is a deferred tool, and the tool sent in the search's message, if any.

| Command | First deferred call | Cost round (14c88044) | Round 1 | Round 2 |
| --- | --- | --- | --- | --- |
| status | `render-status` | alone | alone, 3.66 s | alone, 1.48 s |
| log | `append-inbox` | alone | alone, 3.03 s | alone, 3.33 s |
| specify | `derive-routing-candidates` | alone | alone, 2.92 s | with it |
| configure | `merge-permissions` | alone | alone, 3.07 s | alone, 2.46 s |
| fold | `check-unfolded-specs` | with it | `resolve-feature` only | with it, and `resolve-feature` |

That is 1 of 5, 0 of 5 and 2 of 5, from one run per command each time. In round 2, specify's and fold's calls sent in the search's message both returned their results, with no error. The cost-round figures come from *The search step* above, which measured a lone search's step at 1.51 to 1.70 s, by the stream's `assistant` and `user` timestamps. Round 1 cost $2.333 and round 2 $1.659.

### Sections, rules and routing candidates as tool results (tasks 37, 38 and 39)

**Method.** From 01:10:40Z to 01:15:31Z on 2026-11-10, `python3 scripts/measure-command-cost.py --round --only analyze,clarify,groom,plan,fold,review --permissions adopter --specs 40 --bin <copy>` ran Claude Code 2.1.321 (`claude -p`, `stream-json`, model `claude-opus-5-5`) at 696c04c0 with the three tasks' change uncommitted. It ran against a copy of this checkout's release build of the change. Each run's calls were read from its raw stream, and its denials from `permission_denials`. The round ran before review's step 2 and analyze's §Rules gained the sentence on reading the `lists-file` with the host's reader, so that sentence's effect is not measured.

**Result.** It cost $4.211. Every result subtype was `success`, and each run made one `ToolSearch`.

- **Task 37.** No run read a whole reference. clarify asked `read-section` for one section, which returned 4,892 characters against the cost round's 11,859 for the whole file. fold asked for one, 1,473 against 5,210. plan asked for six, 7,370 against 9,781. review asked for two, 3,707. groom and analyze read nothing of their references, and analyze's largest tool result was 2,189 characters, against 50,087.
- **Task 38.** No run read a rule file. analyze called `discover-rule-files` with `all`, `rules` and `verification`, and review with `rules`. Each result moved its rules to its `lists-file`. analyze read that file with `jq`, with one call denied as compound. review read it with the host's reader after one `python3` call was denied.
- **Task 39.** groom called `derive-routing-candidates`, which matched nothing for the item, then ran three `grep`s of spec bodies, one of them denied as compound. Its verifying criterion, no `grep` of spec bodies, was not met.
- **Other denials.** review had five more, which the cost round's runs also showed: `make test` chained, `rm -r`, `git checkout`, a chained `git diff`, and `git -C`. analyze had one more, a `grep` of its own saved output.

### Routing candidates on the main path (task 39)

**Why the earlier round could not pass.** No inbox item can match a slug of the harness's generated corpus. Its specs are `NNN-feature-N`, `feature` is a `derive-routing-candidates` stopword, and a number is under the four-character minimum. So the failed round above ran the scenario's no-candidate Edge Case. There, groom's three `grep`s took 6.99 s, from the primitive's result at 01:12:08.952Z to the last `grep`'s result at 01:12:15.942Z, and 3 of the run's 7 model requests, by message id.

**Lexical matching, measured.** The method is a Python copy of the primitive's tokenizer (`tokenize`, `STOPWORDS`, `MIN_TOKEN_LEN` 4 in `runtime/src/primitives/derive_routing_candidates.rs`). The population is the 390 distinct bullet lines added across `specs/inbox.md`'s git history, a rewritten item counting again. They were matched against the 78 `specs/NNN-*/spec.md` at 56a92b87 by three subjects: the slug's words after its number, the H1 title, and the body below the frontmatter. Each figure is the number of specs an item shares a token with.

| Subject | Median specs matched | Items matching none |
| --- | --- | --- |
| slug, as the primitive matches today | 20.5 | 0 of 390 |
| title | 22.5 | 0 of 390 |
| body | 78, every spec, for every item | 0 of 390 |

By token count, the 1 item of at most 8 tokens matched 4 slugs, the 6 of 9 to 20 a median of 6, the 111 of 21 to 50 a median of 11, and the 272 of more than 50 a median of 29.5. On a generated 40-spec corpus, the seeded item matches no spec by slug, title or body.

**The run.** From 01:32:02Z to 01:32:30Z on 2026-11-10, `python3 scripts/measure-command-cost.py --round --only groom --permissions adopter --specs 40 --bin <copy>` ran Claude Code 2.1.321 (`claude -p`, `stream-json`, model `claude-opus-5-5`) at 56a92b87, with this task's harness change uncommitted. The copy was of this checkout's release build of 56a92b87's runtime source, whose sha256 begins `596ff1b15fcc5af4`. groom's preset now seeds `041-status-view`, a `done` spec whose slug shares `status` and `view` with the item, beside the inbox item.

- **Result.** One `ToolSearch`, no `grep` and no other shell read of a spec, and no denial. Its one `Bash` call listed the config, the spec root and the inbox. It read 041 with `read-spec` and three sections of its reference with `read-section`, then routed the item to a spec edit of 041 at step 3's gate. It took 27.04 s, 9 turns and 2,060 output tokens, and cost $0.403. Its largest tool result was the three sections, 9,863 characters.
- **`routed-by`.** groom's first `derive-routing-candidates` call passed `routed-by: "groom"`, which derives nothing and returns `gate-required: false`. Its second call, without it, returned 041. Step 2 of `framework/commands/groom.md` said "as `/{project}:specify` does", and specify passes `routed-by` when groom has already routed. The argument's schema doc also gives `groom` as its example. Step 2 now says to pass no `routed-by`, which marks a tree that has already run.
- **Step 2's wording, verified.** From 01:36:34Z to 01:37:02Z the same command ran once more, on the same copy, with step 2's new wording. groom's one `derive-routing-candidates` call passed no `routed-by` and returned 041. It made no `grep` and no denial. It read 041's `spec.md` with the host's reader, listed that spec's directory, and routed the item to a scenario under 041 at step 3's gate. It took 26.73 s, 10 turns and 1,838 output tokens, and cost $0.405.

### An adopter bootstrap across task 36's reference read (task 43)

**Method.** The subject was an upgrade from `ductus-v0.59.11`, Claude only (owner, 2026-11-10), in a fresh scratch repository under `/tmp`. Each run was a subagent with no `ductus` MCP tools and no nested CLI. Both ran on `claude-sonnet-5-5`, the general-purpose agent's default, where task 11's four ran on `claude-opus-5-5`.

- **Before pushing.** The staging check ran on 2026-11-10 at 44284f1a. It installed Claude's 17 copies with the cost harness's `install_framework`, from a copy of `framework/` in the archive's layout, through a copy of this checkout's release build. Each copy's line equalled the one this repository's generator writes, the project name substituted.
- **Run 1, 12:30:17Z to 12:34:34Z.** It ran that release's `install.sh claude --ref=ductus-v0.59.11`, then its bootstrap as `/ductus --agents=claude --ref=ductus-v0.59.11`. None of its 17 copies carries the line. It departed from the procedure twice, both corrected before its commit. It first passed `enforce-manifest` destination paths rather than basenames, which removed the 17 copies, so it installed them again. It also wrote `AGENTS.md` and `CLAUDE.md` with the `create` strategy.
- **Run 2, 12:35:15Z to 12:44:04Z.** It added `[runtime] path` naming a copy of this checkout's release build, whose sha256 begins `1854721bd3798678`, built at 4025732d. Between 56a92b87 and that build, other specs' commits changed runtime source (bf1072ce `render_status.rs` and c871a550 `write_analysis.rs`, and the descriptions and schema), none of them `apply-manifest` or the tool-loading derivation this run exercised. Then it ran `main`'s `install.sh claude --ref=main` and carried out `main`'s bootstrap as `/ductus --ref=main`.
  - Branch 1 took the configured binary with no warning, since it and the pin both report 0.59.11. The self-update check found Claude's copy `current`, and five migrations ran.
  - Claude's `apply-manifest` call updated all 17 copies.
  - `merge-status-hook` returned `{"action":"updated","claude-version":"2.1.322","reason":null}`, adding the `scratch:status` and `scratch:help` entries.
  - Its departures changed no result. Permission Setup's edit was written with the host's file tool after a scripted edit was denied. `last_applied` was written once, after the five migrations. `write-session` ran with the session-identity variables unset. One stray write to `/tmp/x` was removed.

**Result.** 14 of the 17 copies carry the line, all but help, link and target. Each of the 17 lines equals this repository's generated copy's line with `/ductus:` read as `/scratch:`. plan's names `derive-boundary`, implement's `append-task`, and prune's `prune-project`. All 17 copies are byte-identical to a staging install, by the same binary, of this checkout's `framework/` at 60c28d26, whose `framework/` is 44284f1a's. That install used run 2's own entries and substitutions, from a `mktemp -d -t ductus-XXXXXX` source root. So the bootstrap's own staging reads each cited reference as the generator does.

**Usage.** Run 1 made 32 model requests, with 16,060 output tokens, 4,738,677 cache-read tokens and 187,329 cache-write tokens. Run 2 made 78, with 35,866, 13,592,331 and 219,164. Every cache write was a 5-minute one. Sonnet 5.5's rates are not derived here, since the 18 harness runs that priced Opus 5.5 ran on Opus, so neither run is priced.

### Analyze's malformed-rule-heading check (task 48)

**Method.** At 2026-11-10T17:28:10Z, `python3 scripts/measure-command-cost.py --round --only analyze-malformed-rule-heading --permissions adopter --specs 40 --keep --raw <dir> --out <file>` at a7160c04 ran Claude Code 2.1.322 (`claude -p`, `stream-json`, model `claude-opus-5-5`) once, against this checkout's release build of e66f79c2's runtime, the owner having approved the run (2026-11-10). The tree was dirty only by the untracked `scripts/__pycache__/`. The preset is analyze's from the round, on `004-feature-4`, with the seed `seed_malformed_rule_heading`: one rule under the QUAL-CLAIM category of the fixture's `specs/rules/quality-cross.md`, headed `### qual-claim-099`, which the rule-ID grammar refuses, so `discover-rule-files` lists it under `other-headings`, line 35, and not among the file's five rules (checked without a model on the seeded file). The run's calls were read from its raw stream.

**Result.** It cost $0.933 over 79.9 s and 27 turns, result subtype `success`. Its final reply named the heading as a blocking finding: `specs/rules/quality-cross.md:35`, `### qual-claim-099`, "a real rule, with a MUST Statement … But the parser only recognises uppercase rule IDs, so it skips this rule entirely", and asked whether to defer it. So analyze reports a malformed rule heading. It also treated the whole of `quality-cross.md` as unloadable and applied none of its rules, as §Rules has it when any of its checks fails.

**Shell reads.** After reading the rules' list file whole with the host's reader, 11,224 characters, the run queried that file with `jq` three times, the last denied as compound, and read the heading's added lines with `git show 149e3cf -- specs/rules/quality-cross.md`. §Rules says neither the list file nor a rule file is read through the shell, so its sentence on reading the list file with the host's reader, unmeasured at task 38's round, did not hold on this run (task 57).

**After task 57.** At 2026-11-10T17:49:56Z the same check ran at 12a7cede, where each `other-headings` entry carries its `body`, against the release build of that tree, the owner having approved the run with task 57's design (2026-11-10). It cost $0.897 over 85.9 s and 31 turns, result subtype `success`. It again named `specs/rules/quality-cross.md:35`, `### qual-claim-099`, as a blocking finding, and it read no rule file: it took the heading's lines from the entry's `body`. It still queried the list file through the shell, with `jq` five times, the first denied as compound, beside one `Read` of the file with a line limit, so the list-file half of task 57's done-when is not met.

### A Pi adopter upgrade across the change (AC26, Pi's half)

**Method.** On 2026-11-10 a subagent on `claude-opus-5-5`, with no `ductus` MCP tools and no nested CLI, upgraded a fresh scratch adopter, project `scratch`, as tasks 11 and 43 did, with Pi selected (owner, 2026-11-10). pi 1.5.0 ran only as `pi --mode rpc --approve --no-session --offline`, through a driver that sends a slash command only when `get_commands` lists it from an extension and stops pi on any turn event, so no login was used and no model request made.

- **Run 1, 19:25:20Z to 19:33:21Z.** `ductus-v0.59.11`'s `install.sh claude --ref=ductus-v0.59.11`, then its bootstrap as `/ductus --agents=claude,pi --ref=ductus-v0.59.11`. The bridge installed was that release's, and `get_commands` listed `scratch-status` and `scratch-help` only as prompt templates.
- **Run 2a, 19:35:00Z to 19:36:53Z.** With `[runtime] path` naming a copy of this checkout's release build, `main`'s (`ce6c5884`) `install.sh claude --ref=main`, then `/ductus --ref=main`. The self-update check found Pi's `ductus.md` copy stale and stopped at Pre-flight abort, as task 29 records for every multi-agent upgrade.
- **Run 2b, 19:40:29Z to 19:45:12Z.** `/ductus --ref=main` again. Pi's `apply-manifest` call installed the bridge with `substitutions-applied: 12`: the bridge 080 shipped held `{project}` inside its template literals, and walker step 4 installs it through Pi's per-agent call, under Pi's substitutions, while its Bridge step promises a verbatim copy of a file holding no placeholders. So pi listed `$scratch-status` and `$scratch-help` from the extension, and AC26's Pi half did not hold.

**The fix.** The bridge now spells every name by concatenation and every comment's command as `/<project>-…`, so it holds no substituted key, which `apply_manifest`'s `the_pi_bridge_survives_pis_substitutions` asserts of the shipped file. The step that failed was then run again on a copy of run 2b's tree, at 19:59:47Z to 19:59:57Z: Pi's `apply-manifest` entry for the bridge, with Pi's substitutions, applied 0 and wrote a bridge byte-identical to the source, `b6990d2f…`. `get_commands` then listed `scratch-status` and `scratch-help` from the extension, and each ran `handled`, showing a `ductus-status` or `ductus-help` custom message with no turn and an empty stderr. Every other step of the upgrade is run 2b's, unchanged by the fix.
