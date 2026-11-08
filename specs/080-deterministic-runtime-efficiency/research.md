# 080 — Deterministic runtime efficiency Research

The measurements and verifying runs spec 080's criteria point at. Each figure states its method, units, population and date (§grounding), and each section names the command that reproduces it.

## Method traps

Three ways an earlier measurement of this work went wrong, each of which inflates or misattributes a figure:

- **Size a tool result as the characters of its result text, not as `json.dumps` of the content.** In a session transcript the content is a JSON value, and serializing it again counts its escaping: for `dashboard` that read 41–44 KB against the true 37,171–40,454 characters.
- **A command the agent starts through the Skill tool leaves no `<command-name>` tag in a transcript.** Attributing each `ToolSearch` to the running command takes the Skill `tool_use` entries as well as the tags.
- **A loaded tool reaches the model as its description and input schema only.** The output schemas, 228,054 of 366,331 schema characters when the spec was clarified, never do, so a loaded-schema size counts description plus input schema, measured from `ductus mcp` `tools/list` over stdio, never from a transcript.

## Result sizes (AC14)

**Method.** `scripts/measure-result-sizes.sh` runs the ignored `result_sizes` test in `runtime/tests/result_sizes.rs`. It generates two corpora with `runtime/tests/common/corpus.rs`, sized as the report's first line says, and calls every name in `PRIMITIVE_REGISTRY` through the built binary's MCP server over stdio. Each call runs on a fresh copy of its corpus, so a writing primitive changes nothing the next one reads. Per-feature primitives are pointed at the subject spec, `001-feature-001`, which is `in-progress` with every task ticked; `traverse-deps` is pointed at the last spec, whose chain runs through every other. Every spec carries one markdown-lint violation and one broken link to a file outside the spec root. A primitive that returns findings grows with the defects in what it reads, and on a defect-free corpus that growth would not show. For each ten specs the corpus also holds one branch-scoped spec with a `folds-into:`, so the staging inventory scales as well. `check-orphaned-references` reads only a fixed set of adopter-owned files (`CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit` and the spec root's `system.md`), not the spec corpus, so its result cannot grow with the corpus by its contract.

**Units.** Characters of the result's text, the `text` of the `tools/call` result's content, which is what a host reads.

**Population.** Every registry name. A name the fixture cannot exercise has a *not measured* row with its reason.

**Grows with the corpus.** A result grows when its size on the large corpus is at least twice its size on the small one and at least 200 characters more. A count's extra digits pass neither test, and a list that scales with the corpus passes both. *Bounded already* means the result carries `bounded_lists`' `form` field, so it moves its lists to a file past `read-size-bytes`.

### Measurement

Measured by `scripts/measure-result-sizes.sh` at 2b4eaddb79b820e140f4fbc0da05a69c2a7bd8f4, with these uncommitted under runtime/: runtime/tests/common/mod.rs, runtime/tests/common/corpus.rs, runtime/tests/result_sizes.rs, at 2026-11-08T19:30:56Z.

Small corpus: 10 specs, each with 5 criteria, 5 tasks of 3 subtasks and 2 scenarios, one markdown-lint violation and one broken link; 5 inbox items. Large corpus: 100 specs, each with 50 criteria, 50 tasks of 3 subtasks and 20 scenarios, one markdown-lint violation and one broken link; 50 inbox items.

| Primitive | Small (chars) | Large (chars) | Grows | Bounded already | Note |
| --- | --- | --- | --- | --- | --- |
| `read-spec` | 978 | 6006 | yes | no | |
| `read-tasks` | 1746 | 17247 | yes | no | |
| `mark-task` | 72 | 72 | no | no | |
| `mark-criterion` | 72 | 72 | no | no | |
| `set-status` | 82 | 82 | no | no | |
| `derive-boundary` | 350 | 2330 | yes | no | |
| `discover-rule-files` | 110 | 110 | no | no | |
| `process-waivers` | 54 | 54 | no | no | |
| `process-decisions` | 172 | 172 | no | yes | |
| `compute-review-scope` | 1491 | 34570 | yes | yes | |
| `write-review` | 345 | 346 | no | no | |
| `write-analysis` | 165 | 165 | no | no | |
| `check-step-references` | 255 | 255 | no | no | |
| `check-stuck` | 101 | 101 | no | no | |
| `validate-frontmatter` | 28 | 28 | no | no | |
| `resolve-anchor` | 85 | 85 | no | no | |
| `traverse-deps` | 133 | 129 | no | no | |
| `check-rule-ids` | 58 | 58 | no | no | |
| `run-generator` | — | — | not measured | — | CLI-only: it runs a project script and is no MCP tool, so no host reads its result as a tool result |
| `lint-markdown` | 1334 | 13044 | yes | no | |
| `gate-confirm` | 70 | 70 | no | no | |
| `fetch-archive` | — | — | not measured | — | it needs the network |
| `extract-archive` | — | — | not measured | — | it needs a release archive, which a spec corpus does not hold |
| `apply-manifest` | — | — | not measured | — | its result follows the manifest the host builds and needs a staged framework tree; a spec corpus holds neither |
| `enforce-manifest` | — | — | not measured | — | its result follows the expected-file list the host passes, not the spec corpus |
| `merge-managed-block` | 163 | 163 | no | no | |
| `merge-permissions` | 209 | 209 | no | no | |
| `migrate-session-file` | 114 | 114 | no | no | |
| `relocate-audit-records` | 61 | 61 | no | no | |
| `create-scenario` | 59 | 59 | no | no | |
| `append-task` | 89 | 90 | no | no | |
| `label-criteria` | 89 | 90 | no | no | |
| `prune-tasks` | 905 | 6885 | yes | no | |
| `prune-plan` | 228 | 228 | no | no | |
| `prune-project` | 420 | 420 | no | no | |
| `dashboard` | 5375 | 43430 | yes | no | |
| `write-session` | 46 | 46 | no | no | |
| `resolve-session` | 60 | 60 | no | no | |
| `retarget-sessions` | 46 | 46 | no | no | |
| `resolve-references` | 140 | 140 | no | yes | |
| `resolve-constitutions` | 60 | 60 | no | no | |
| `resolve-feature` | 120 | 120 | no | no | |
| `create-feature` | 130 | 130 | no | no | |
| `create-plan-artifacts` | 196 | 196 | no | no | |
| `check-review-freshness` | 86 | 86 | no | no | |
| `check-review-gate` | 886 | 6198 | yes | no | |
| `classify-spec-edit` | 28 | 28 | no | no | |
| `measure-reopenings` | 520 | 4133 | yes | yes | |
| `append-question` | 135 | 135 | no | no | |
| `diff-cross-spec` | 434 | 3360 | yes | yes | |
| `append-inbox` | 56 | 57 | no | no | |
| `remove-inbox-item` | 60 | 61 | no | no | |
| `check-artifacts` | 617 | 4991 | yes | no | |
| `check-artifact-size` | 213 | 214 | no | no | |
| `derive-routing-candidates` | 209 | 209 | no | no | |
| `check-corpus-links` | 4211 | 130619 | yes | no | |
| `check-orphaned-references` | 181 | 181 | no | no | |
| `check-command-flags` | 263 | 263 | no | no | |
| `check-promotion-coverage` | — | — | not measured | — | it audits the framework's own rule file against command sections, which a spec corpus does not hold |
| `derive-dependencies` | 156 | 157 | no | no | |
| `derive-references` | 168 | 169 | no | no | |
| `check-unfolded-specs` | 132 | 1071 | yes | no | |
| `rewrite-spec-links` | 819 | 6941 | yes | no | |
| `retire-feature` | 47 | 47 | no | no | |
| `invalidate-review` | 62 | 62 | no | no | |
| `start-fix-rounds` | 73 | 75 | no | no | |
| `brief-re-read` | 971 | 17397 | yes | yes | |
| `read-re-read` | 316 | 316 | no | yes | |
| `check-prescriptions` | 406 | 406 | no | yes | |

### Grows with the corpus

Of the 69 registry names, 15 grow with the corpus by the test above.

- **Not bounded yet, so AC7's population (11):** `read-spec`, `read-tasks`, `derive-boundary`, `lint-markdown`, `prune-tasks`, `dashboard`, `check-review-gate`, `check-artifacts`, `check-corpus-links`, `check-unfolded-specs`, `rewrite-spec-links`.
- **Bounded already through `bounded_lists` (4):** `compute-review-scope`, `measure-reopenings`, `diff-cross-spec`, `brief-re-read`.

## Loaded schema sizes (AC23)

**Method.** The same run lists the tools with `tools/list` and sizes each as its description's characters plus its input schema serialized as compact JSON, which is the form the server sends. Rows are in descending order of the loaded size.

**Reconciliation with the spec's figure.** Spec 080's Resolved Questions record 132,846 characters across 68 tools. The measurement below totals 129,531 in the compact form. The same `tools/list`, read from `runtime/target/release/ductus mcp` over stdio on 2026-11-08, also totals 129,531 compact, and 132,353 when Python's `json.dumps` serializes the input schemas with its default separators (`", "` and `": "`). The spec's figure was therefore taken with the default separators, and the remaining 493 characters are description changes made since it was taken.

### Measurement

Measured by `scripts/measure-result-sizes.sh` at 2b4eaddb79b820e140f4fbc0da05a69c2a7bd8f4, with these uncommitted under runtime/: runtime/tests/common/mod.rs, runtime/tests/common/corpus.rs, runtime/tests/result_sizes.rs, at 2026-11-08T19:30:56Z.

68 tools, 129531 characters in all.

| Tool | Description | Input schema | Loaded |
| --- | --- | --- | --- |
| `write-review` | 3579 | 16929 | 20508 |
| `write-analysis` | 1356 | 8721 | 10077 |
| `compute-review-scope` | 3217 | 704 | 3921 |
| `rewrite-spec-links` | 2371 | 1486 | 3857 |
| `process-decisions` | 1567 | 1964 | 3531 |
| `append-task` | 234 | 3222 | 3456 |
| `retire-feature` | 1522 | 1638 | 3160 |
| `prune-project` | 986 | 2089 | 3075 |
| `check-corpus-links` | 1704 | 1365 | 3069 |
| `write-session` | 970 | 2097 | 3067 |
| `derive-references` | 1927 | 735 | 2662 |
| `measure-reopenings` | 1980 | 460 | 2440 |
| `derive-dependencies` | 1179 | 1063 | 2242 |
| `apply-manifest` | 60 | 2160 | 2220 |
| `prune-plan` | 941 | 1242 | 2183 |
| `check-prescriptions` | 1794 | 368 | 2162 |
| `check-review-gate` | 1877 | 211 | 2088 |
| `create-feature` | 330 | 1749 | 2079 |
| `merge-permissions` | 97 | 1961 | 2058 |
| `retarget-sessions` | 588 | 1443 | 2031 |
| `process-waivers` | 544 | 1438 | 1982 |
| `check-artifacts` | 1770 | 211 | 1981 |
| `classify-spec-edit` | 1560 | 339 | 1899 |
| `resolve-constitutions` | 1471 | 427 | 1898 |
| `brief-re-read` | 1534 | 350 | 1884 |
| `derive-routing-candidates` | 1040 | 742 | 1782 |
| `read-re-read` | 1320 | 314 | 1634 |
| `append-question` | 666 | 852 | 1518 |
| `check-promotion-coverage` | 138 | 1360 | 1498 |
| `invalidate-review` | 1159 | 291 | 1450 |
| `diff-cross-spec` | 1200 | 211 | 1411 |
| `dashboard` | 1313 | 74 | 1387 |
| `create-plan-artifacts` | 514 | 860 | 1374 |
| `check-step-references` | 1293 | 74 | 1367 |
| `check-artifact-size` | 1094 | 211 | 1305 |
| `prune-tasks` | 309 | 973 | 1282 |
| `resolve-references` | 1007 | 243 | 1250 |
| `enforce-manifest` | 66 | 1089 | 1155 |
| `resolve-feature` | 442 | 622 | 1064 |
| `check-orphaned-references` | 969 | 74 | 1043 |
| `check-command-flags` | 938 | 74 | 1012 |
| `create-scenario` | 193 | 810 | 1003 |
| `merge-managed-block` | 76 | 921 | 997 |
| `start-fix-rounds` | 763 | 224 | 987 |
| `check-stuck` | 657 | 316 | 973 |
| `relocate-audit-records` | 698 | 223 | 921 |
| `fetch-archive` | 64 | 847 | 911 |
| `mark-task` | 420 | 478 | 898 |
| `migrate-session-file` | 342 | 555 | 897 |
| `resolve-anchor` | 266 | 611 | 877 |
| `resolve-session` | 791 | 74 | 865 |
| `mark-criterion` | 57 | 807 | 864 |
| `check-unfolded-specs` | 765 | 74 | 839 |
| `check-review-freshness` | 535 | 211 | 746 |
| `derive-boundary` | 555 | 180 | 735 |
| `append-inbox` | 258 | 407 | 665 |
| `discover-rule-files` | 179 | 439 | 618 |
| `extract-archive` | 72 | 546 | 618 |
| `remove-inbox-item` | 224 | 373 | 597 |
| `lint-markdown` | 52 | 527 | 579 |
| `gate-confirm` | 289 | 273 | 562 |
| `set-status` | 136 | 330 | 466 |
| `check-rule-ids` | 64 | 352 | 416 |
| `label-criteria` | 163 | 211 | 374 |
| `read-spec` | 41 | 300 | 341 |
| `validate-frontmatter` | 55 | 187 | 242 |
| `read-tasks` | 45 | 195 | 240 |
| `traverse-deps` | 58 | 180 | 238 |
