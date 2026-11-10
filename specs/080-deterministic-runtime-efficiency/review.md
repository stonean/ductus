---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-10T21:33:12Z
reviewed-against: c41d69660c7e14960ec26a3dc5f3f5ed9628f108
diff-base: 150b4dc1cd3c0e121acbb0d6b6477715965a8fc4
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 144
scope: 183
excluded: 178
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: e341b9c089dbcd966a03da2f8d45634e40a3d3ee083ba1091b640111db55a1a6
  scenarios/antigravity-keeps-the-model-path.md: 06e76b9b8d9c68a59edca631b88bfbfaf33ae313974ae52f6a27a894c6fe6be1
  scenarios/configure-types-no-permission-entries.md: 32fa7bcfc2ba3bb064fe3f3bcd6c23ed637340b8319a8b231d1eb3be62ec62ad
  scenarios/groom-derives-routing-candidates.md: 516690b45261a38cad315305e8b1ca19fa51a072137a7bfbb1a12c9217fe42da
  scenarios/help-answers-with-no-model-turn.md: be093aa1eadf2df589f65651f73c79537222b659bdf109c4f07820188bfa5bf5
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/opencode-keeps-the-model-path.md: 753136883e25ef235d7279e3c696f5e8fce5683f4db0dea1717870a85f6083f8
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
blocking: false
dispositions:
  fixed: 64
  deferred: 0
  waived: 15
  undispositioned: 0
decisions:
  - key: "other: with a pinned `target.md`, a tool can load from neither side when the session's `cli-config-dir` names another agent — `runtime/src/primitives/apply_manifest.rs`"
    outcome: waived
    reason: "Inside the two-agent-CLI bound the constitution's §concurrent-features names: the runtime reads command files from the agent `/ductus` last recorded. Within one agent's layout every case agrees."
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "convention: resolve-anchor's `heading_lines` and tool_loading's `atx_headings` read ATX headings by different rules — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: waived
    reason: No shipped command, reference or configure source holds a heading the two read differently, and unifying them changes every reader of one, which is not a mechanical edit.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: no test holds each bounded member's top-level `-count` keys to the lists its file holds, which the walker's halt reads — `runtime/tests/mcp.rs`"
    outcome: waived
    reason: Every member's counts name a list its file holds today, checked struct by struct; a guard for a future member is new test design, not a chore.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "doc: ductus.md's State A version check probes `[runtime] path` for Auggie and Antigravity, whose servers task 31 says serve the store's runtime — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Which binary State A probes per agent is a design question for the bootstrap's runtime acquisition, not a mechanical edit, and probing the configured binary still detects a stale pin.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "doc: the bootstrap's step 10 runs merge-status-hook through the CLI, which only Claude's seed allows, so a non-Claude host bootstrapping with Claude selected meets an unmatched command — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: It affects only a multi-agent State-B run hosted by another agent, and adding a CLI rule to other agents' seeds is a permission change for the owner, not a mechanical edit.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: read-section's file form holds each section's text as one line, so a host reading by line offset cannot split a section — `runtime/src/primitives/read_section.rs`"
    outcome: waived
    reason: No shipped section reaches the default read size (the largest is 7,934 bytes), and a read size below one section is a host configuration this spec does not size for.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: past the bound the MCP response carries the result twice, as text (49,965 bytes for render-status on 700 specs) and as `structuredContent` (53,614) — `runtime/src/mcp/server.rs`"
    outcome: waived
    reason: Claude Code hands the model one copy, as every `mcp__ductus__` result in this run shows; the bound measures the result, and changing how the server's JSON wrapper serializes is 022's surface and not a mechanical edit.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "doc: 080's plan.md Affected Files omits files 080's commits changed, among them read_spec.rs, payload.rs, bounded_lists.rs and the bounded members — `specs/080-deterministic-runtime-efficiency/plan.md`"
    outcome: waived
    reason: The plan's bounded-results decision covers those files, which rows to add is a judgment, and check-artifact-size's advisory on this plan asks for finished rows to be trimmed rather than added.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "doc: 023's AC36 and AC38 count nine retired entries where there are 13, and AC37 says the revoke conflict is raised before any filesystem read — `specs/023-govern-refinement/spec.md`"
    outcome: waived
    reason: The counts were false at 150b4dc1 too, and AC37 was never strictly true; correcting a done spec's criteria reopens it, a reopen that is 023's to spend.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: no 080 artifact records that the next release must be a minor version, since 080 changes tool results — `specs/022-deterministic-runtime/data-model.md`"
    outcome: waived
    reason: The release step reads 022's versioning section when it chooses the version, so nothing of 080's holds it.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: fix round 1's re-read found that `bounded()`'s watchdog can leave its `sleep` running when the call ends before the watchdog sets its trap — `scripts/gen-claude-commands.sh`"
    outcome: waived
    reason: "Nothing hangs: an orphaned sleep holds only /dev/null and exits at the bound, 3 in 200 calls in the re-read's count, and closing the window needs a second signal handshake, not a mechanical edit."
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 1's re-read found that the dashboard test drives only `session-unreadable` past the bound, so removing the `session-notices` move fails no test — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: Both lists move by the same branch, which the test drives for one of them; a fixture that puts a session notice past the bound needs co-targeting session records, which is new test design, not a chore.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 2's re-read found the unrecognized-suffix notice and the `loading rule files:` line printing a rule file's name unflattened — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: Both lines predate this spec and print the names of files the project put in its own rule directory; flattening every notice name is a sweep beyond 080's change, not its fix.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 2's re-read found no test feeding the non-file notice a name holding a line hazard — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: The flattening is `flatten_line`'s, whose own tests and the pipeline view's cover it; a symlink named with a newline is a new fixture, not a chore.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 2's re-read found no generator-test case for the all-zero timeout refusal — `scripts/tests/test-gen-claude-commands.sh`"
    outcome: waived
    reason: The refusal was driven by hand for 0, 00 and abc, each refused naming the variable, and 05 accepted; a case per refused value is new test design, not a chore.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 2's re-read found that a timeout of 20 or more digits overflows `[`'s comparison and is let through, as it was before the round — `scripts/gen-claude-commands.sh`"
    outcome: waived
    reason: A contrived value a maintainer sets by hand; `sleep` then rejects it, so every runtime call is stopped at once and the generator fails, which is no silent pass.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
---

# Review — 080-deterministic-runtime-efficiency

## Summary

080's closing review, its first record. The window starts at the reopen base 150b4dc1. The passes read it at ec6c2088, which seven read-only agents on claude-opus-5-5 pinned with `git show`, one per behavior slice. The fixes landed in 567e6399, 080's own, with each other spec's edits in a commit of its own (32eb5abf, c03f2ba1, 67bac134, 5b622012, e48033c6). The re-measure its description edits required is c41d6966, and the fix rounds' own edits are committed with this record. Every item was re-derived from the code before it was recorded. All findings are fixed, so none is counted: 0 MUST, 0 SHOULD, 0 low-confidence.

Found and fixed:

- CFG-CONST-001 (MUST, merge_status_hook.rs): the hook event and project-dir variable were duplicated literals.
- BE-PAYLOAD-001 (MUST, dashboard.rs): the file form kept session-unreadable inline.
- QUAL-TEST-001 (SHOULD, pi-bridge-harness.mjs): a case passed with its guard removed.
- QUAL-DELEG-001 (SHOULD, rule_sections.rs): a tab-separated rule heading appeared nowhere in the result.
- Five low-confidence findings were settled. BE-INPUT-004 (apply_manifest.rs), BE-TIMEOUT-001 (gen-claude-commands.sh) and QUAL-CLAIM-001 (discover_rule_files.rs) were judged to apply and fixed. BE-TXN-002 and BE-LOCK-001, on merge-status-hook's settings write, were dropped: concurrency-backend.md scopes its rules to server-side shared state, and 048's and 051's records drop BE-TXN-002 on a local file's read-then-write. spec.md and plan.md state that write's strategy all the same.
- Each new test was shown to fail with its behavior removed: eight run.py mutants, all killed.

Three contract breaks are fixed. The largest came from the AC26 run the owner chose (2026-11-10): a real Pi adopter upgrade from ductus-v0.59.11 showed that /ductus's Pi apply-manifest call substituted 12 {project} tokens 080 had put in the bridge. The bridge now holds none, a test asserts it, and a re-install through Pi's call registered both commands (research.md). The owner also chose AC7's wording (2026-11-10).

Coverage: scope is 183 entries, 182 files and the pattern framework/commands/*.md. examined is 144:

- the agents' 143 entries, 78 read whole, 60 by their full window diff with context, and 5 in part:
  - apply_manifest.rs, its tests at 860-2300 unread;
  - mcp/server.rs, its wrappers at 320-1230 read only where the diff touches them;
  - derive_dependencies.rs past line 170;
  - derive_routing_candidates.rs past 190;
  - traverse_deps.rs's detect_cycles and tests;
- runtime/src/host.rs, which the fix commit brought into scope, read in its module doc and the command_file_candidates callers.

The other 39 entries are generated mirrors and were not opened. They are generated from sources read in full: gen-claude-commands.sh --check exited 0 at the pin and after the fixes, govern.md is a byte copy of ductus.md, and .pi/extensions/ductus.ts of the bridge source. After the pin, the only in-scope path that moved was runtime/src/main.rs, a doc comment from 055's 3602c737, which I read.

Rules: every slice read the backend and cross ranges of the twelve rule files with the reader, and only the TypeScript and JavaScript slice read the frontend ranges. The two spec-prose slices judged the code rules inapplicable from their statements.

Left unexamined:

- descriptions.rs's 079-era edits (BRIEF_RE_READ, CHECK_PRESCRIPTIONS, CHECK_REVIEW_FRESHNESS, WRITE_ANALYSIS, WRITE_REVIEW), not checked against their code;
- 022's data-model sections on the AST, protocol, severity tiers, mark-task, mark-criterion, extension schemas and versioning, read but not checked;
- 023's spec.md beyond its signpost and AC36-AC40;
- ductus.md's Source resolution, Self-update, Pre-flight abort, Collect Project Inputs and File Fetching sections beyond the window diff;
- the workflows' new action SHAs against their tags, and 064's pi 0.99.0 CHANGELOG claim, both needing a network request;
- research.md's recorded model runs.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- contract: the Pi bridge 080 shipped held `{project}` in its template literals, so `/ductus`'s Pi apply-manifest call substituted 12 and the installed bridge registered `$scratch-status` and `$scratch-help`, against AC26 and the Bridge step's verbatim copy; it now holds no substituted key, `the_pi_bridge_survives_pis_substitutions` asserts it, and a re-install through Pi's call registered both commands — `framework/bootstrap/pi/ductus-bridge.ts` (breaks the spec's contract) — **fixed** (decided by the rule)
- contract: a single-line HTML comment's invocations and `§` citations joined the ToolSearch line, against plan *Batched (AC29)*'s "outside fenced blocks and HTML comments" — `runtime/src/primitives/tool_loading.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- contract: data-model.md *`merge-permissions`' canonical call* said `allow` follows step 2's order, which the code never did: it writes the static entries in step 2's order, then the served tools in registry order; the claim and the doc comment now say so — `specs/080-deterministic-runtime-efficiency/data-model.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: the pipeline view put the session's feature and scenario, a scenario's `section`, a `folds-into`, a dependency with no sequential number and a references line into view lines unflattened, so a newline forged a `Notice:` line on both surfaces — `runtime/src/primitives/pipeline_view.rs` — **fixed** (decided by the rule)
- bug: render-help's malformed-copy reason named only fewer than two `---` lines, though a copy with no `## Instructions` heading or no text between the lines fails the same way — `runtime/src/primitives/render_help.rs` — **fixed** (decided by the rule)
- bug: `ConflictingRevoke` rendered "1 entry appear"; reworded so the count needs no agreement — `runtime/src/primitives/mod.rs` — **fixed** (decided by the rule)
- bug: the bridge's `piSpelling` passed a replacement string, so a `$` in the namespace expanded as a pattern — `framework/bootstrap/pi/ductus-bridge.ts` — **fixed** (decided by the rule)
- bug: measure-command-cost.py named the 1200 s bound when the 10 s reap after its child closed its output timed out, and left the killed child unreaped — `scripts/measure-command-cost.py` — **fixed** (decided by the rule)
- other: measure-command-cost.py's `git status` provenance read could rewrite `.git/index`; it passes `--no-optional-locks` — `scripts/measure-command-cost.py` — **fixed** (decided by the rule)
- convention: merge-status-hook's MCP handler ran its `claude --version` probe on an async worker; it takes the blocking pool, as a subprocess shell-out does — `runtime/src/mcp/server.rs` — **fixed** (decided by the rule)
- convention: `if !args.all` around the disabled-rule-files filter was dead under `all`, whose list is empty, which made reference-and-rule-reads.py's M7 an equivalent mutant; the guard and M7 are removed — `runtime/src/primitives/discover_rule_files.rs` — **fixed** (decided by the rule)
- convention: the unrecognized-suffix notice was written twice in discover-rule-files; one helper builds it — `runtime/src/primitives/discover_rule_files.rs` — **fixed** (decided by the rule)
- convention: derive-dependencies had no test that a malformed read size halts a writing call before it rewrites a spec; `a_malformed_read_size_halts_a_writing_call_before_any_write` is added — `runtime/src/primitives/derive_dependencies.rs` — **fixed** (decided by the rule)
- other: the merge-status-hook integration test probed a freshly written `claude` under the 10 s timeout task 52's flake reached; it runs the stub once first — `runtime/tests/merge_status_hook.rs` — **fixed** (decided by the rule)
- other: generators.yml's `pull_request` paths named `runtime/Cargo.lock` and not `runtime/Cargo.toml` — `.github/workflows/generators.yml` — **fixed** (decided by the rule)
- doc: RENDER_STATUS, CHECK_CORPUS_LINKS and CHECK_UNFOLDED_SPECS, and the module docs of check_corpus_links.rs, check_unfolded_specs.rs and derive_routing_candidates.rs, said read-only though each writes a list file past the read size — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- doc: dashboard.rs's module doc called `dashboard` the status surface, and `view_model`'s `# Errors` claimed `bounded_lists::place`'s errors — `runtime/src/primitives/dashboard.rs` — **fixed** (decided by the rule)
- doc: tool_loading.rs's module doc named `step_tools` where both uses derive through `text_step_tools` — `runtime/src/primitives/tool_loading.rs` — **fixed** (decided by the rule)
- doc: apply-manifest's `# Errors` and module doc misstated a pinned `commands/target.md`, which is read where installed and never halts the call — `runtime/src/primitives/apply_manifest.rs` — **fixed** (decided by the rule)
- doc: `dispatch_blocking`'s doc omitted `render-status` — `runtime/src/mcp/server.rs` — **fixed** (decided by the rule)
- doc: bounded_lists.rs pointed at a data-model table for the list-file keys, which a bullet above it holds — `runtime/src/primitives/bounded_lists.rs` — **fixed** (decided by the rule)
- doc: merge-permissions' `# Ceiling` said an outside entry is refused before anything is read, and `run`'s `# Errors` omitted the canonical call's config errors — `runtime/src/primitives/merge_permissions.rs` — **fixed** (decided by the rule)
- doc: discover-rule-files' `# Errors` omitted `InvalidSpecsRoot` and the config read's `Io`, its test doc said two rules, and `DiscoverRuleFilesResult.selected`'s doc omitted `all` — `runtime/src/primitives/discover_rule_files.rs` — **fixed** (decided by the rule)
- doc: `InboxStanding.state`'s doc said `dashboard` renders the Inbox rows — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- doc: derive-routing-candidates' `# Errors` said every failure but an empty description is a skipped source, though a config that does not parse or an invalid spec root is an error — `runtime/src/primitives/derive_routing_candidates.rs` — **fixed** (decided by the rule)
- doc: host.rs said two callsites resolve command files; 080 added `tool_loading::AlwaysLoaded::resolve` and `render_help::guide` — `runtime/src/host.rs` — **fixed** (decided by the rule)
- doc: payload.rs's `RESTORED_LISTS` and 022's data model said the legacy tail carries each bounded result's `lists-file`, where `form` and `lists-file` are the walk's last result's and a restored list reaches a request only through the typed prefix — `runtime/src/interpreter/payload.rs` — **fixed** (decided by the rule)
- doc: gen-claude-commands.sh's `--help` printed lines 2-24 of a header now 26 lines long, stopping mid-sentence — `scripts/gen-claude-commands.sh` — **fixed** (decided by the rule)
- doc: measure-command-cost.py's docstring said all 17 Claude copies carry the tool-search line; 14 do — `scripts/measure-command-cost.py` — **fixed** (decided by the rule)
- doc: pi-bridge-harness.mjs said CI pins Node 20; CI runs the runner image's own Node — `scripts/tests/pi-bridge-harness.mjs` — **fixed** (decided by the rule)
- doc: status.md still called the view's summary lines callouts — `framework/commands/status.md` — **fixed** (decided by the rule)
- doc: analyze.md said detection writes nothing, though its primitives write list files past the read size — `framework/commands/analyze.md` — **fixed** (decided by the rule)
- doc: review.md's Scope Boundaries said review reads the selected rule files, and named only steps 1, 2, 9 and 12 as writing list files, omitting `dashboard` under `--all` and `read-section` — `framework/commands/review.md` — **fixed** (decided by the rule)
- doc: ductus.md's Pi Bridge step, and its govern.md copy, said Pi has no MCP config file, which task 27's sweep missed — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- doc: AC7 covered every growing primitive while research.md names its population as the 16 this spec bounds; AC7 reworded and a Resolved Question added (owner, 2026-11-10) — `specs/080-deterministic-runtime-efficiency/spec.md` — **fixed**
- doc: spec.md's *Batched* behavior and AC29 omitted the tools the command file invokes anywhere — `specs/080-deterministic-runtime-efficiency/spec.md` — **fixed** (decided by the rule)
- doc: spec.md's Resolved Questions called Pi's context untested, though the AC27 decision settled it from pi's documentation — `specs/080-deterministic-runtime-efficiency/spec.md` — **fixed** (decided by the rule)
- doc: plan.md and data-model.md put the view's floor at about 600 bytes with no method; both now name what the floor holds — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- doc: plan.md gave render-status's result as `{rendered-markdown, project}`, without `form` and `lists-file` — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- doc: plan.md's Trade-offs and data-model.md described Pi's load-time namespace read as render-status's, which task 47 made render-help's — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- doc: plan.md's Cross-spec impact omitted 046 and 066 and called 064 `done` — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- doc: data-model.md said `render-status` may gain `format`, which the Pi verifying run made unnecessary — `specs/080-deterministic-runtime-efficiency/data-model.md` — **fixed** (decided by the rule)
- doc: data-model.md's bounded-results paragraph said grows-set beside a table holding discover-rule-files, and its list of writing calls omitted derive-dependencies with `write` — `specs/080-deterministic-runtime-efficiency/data-model.md` — **fixed** (decided by the rule)
- doc: research.md named ff466dec among the commits before the 4025732d build, which it followed — `specs/080-deterministic-runtime-efficiency/research.md` — **fixed** (decided by the rule)
- doc: the rules-load-without-shell-reads scenario's Context described the earlier rule loading in the present tense — `specs/080-deterministic-runtime-efficiency/scenarios/rules-load-without-shell-reads.md` — **fixed** (decided by the rule)
- convention: tasks 46 and 49 put their done-when in nested bullets under an empty line, which read-tasks reads as empty — `specs/080-deterministic-runtime-efficiency/tasks.md` — **fixed** (decided by the rule)
- doc: tasks 48 and 34 described as owed the analyze run and the design that research.md and plan.md record done — `specs/080-deterministic-runtime-efficiency/tasks.md` — **fixed** (decided by the rule)
- doc: 022's dashboard-primitive scenario said the primitive writes only its list file past the read size, though `place` also writes the directory's `.gitignore` and prunes stale list files — `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md` — **fixed** (decided by the rule)
- doc: 022's dashboard-primitive scenario said `scenario-detail` comes with `scenario`, and gave `config` without `unknown-keys` and with a callout rule the view no longer has — `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md` — **fixed** (decided by the rule)
- doc: 022's data model gave render-status's example without `form`, said every bounded result carries a count, and named only `directories-added` as an absolute payload path — `specs/022-deterministic-runtime/data-model.md` — **fixed** (decided by the rule)
- doc: 022's spec.md signpost omitted the tools a command file invokes anywhere from what `tool-search-line` loads — `specs/022-deterministic-runtime/spec.md` — **fixed** (decided by the rule)
- doc: 022's cli-config-dir-per-contributor and 000's target-clear-flag scenarios said `--clear` deletes the session file, which it leaves empty, and cited ductus.md step 6 for step 7's write — `specs/022-deterministic-runtime/scenarios/cli-config-dir-per-contributor.md` — **fixed** (decided by the rule)
- doc: 000's target-clear-flag scenario quoted a no-target line 080's view no longer prints — `specs/000-slash-commands/scenarios/target-clear-flag.md` — **fixed** (decided by the rule)
- doc: 000's dashboard-dependencies-column scenario put the table in status.md's removed step 3 and misdescribed the dependency cell — `specs/000-slash-commands/scenarios/dashboard-dependencies-column.md` — **fixed** (decided by the rule)
- doc: 022's framework-list-dedup scenario said any path is accepted, omitted `revoke`, and gave the created file without a canonical call's `additionalDirectories` — `specs/022-deterministic-runtime/scenarios/framework-list-dedup.md` — **fixed** (decided by the rule)
- doc: 022's config-resolution-single-probe scenario and 058's plan named `render_markdown` and `render_callouts`, which 080 removed — `specs/022-deterministic-runtime/scenarios/config-resolution-single-probe.md` — **fixed** (decided by the rule)
- doc: 062's plan said session.rs is called by nothing else, and counted `dashboard`'s session lists among those bounded only by the project, which now move to its list file — `specs/062-concurrent-session-targets/plan.md` — **fixed** (decided by the rule)
- convention: 064's spec.md linked 080 in prose, which made 064 depend on 080, and it and its plan kept AGENTS.md's retired "never fix Pi" wording — `specs/064-pi-host-support/spec.md` — **fixed** (decided by the rule)
- other: with a pinned `target.md`, a tool can load from neither side when the session's `cli-config-dir` names another agent — `runtime/src/primitives/apply_manifest.rs` — **waived**: Inside the two-agent-CLI bound the constitution's §concurrent-features names: the runtime reads command files from the agent `/ductus` last recorded. Within one agent's layout every case agrees. (decided by the rule)
- convention: resolve-anchor's `heading_lines` and tool_loading's `atx_headings` read ATX headings by different rules — `runtime/src/primitives/resolve_anchor.rs` — **waived**: No shipped command, reference or configure source holds a heading the two read differently, and unifying them changes every reader of one, which is not a mechanical edit. (decided by the rule)
- other: no test holds each bounded member's top-level `-count` keys to the lists its file holds, which the walker's halt reads — `runtime/tests/mcp.rs` — **waived**: Every member's counts name a list its file holds today, checked struct by struct; a guard for a future member is new test design, not a chore. (decided by the rule)
- doc: ductus.md's State A version check probes `[runtime] path` for Auggie and Antigravity, whose servers task 31 says serve the store's runtime — `framework/bootstrap/ductus.md` — **waived**: Which binary State A probes per agent is a design question for the bootstrap's runtime acquisition, not a mechanical edit, and probing the configured binary still detects a stale pin. (decided by the rule)
- doc: the bootstrap's step 10 runs merge-status-hook through the CLI, which only Claude's seed allows, so a non-Claude host bootstrapping with Claude selected meets an unmatched command — `framework/bootstrap/ductus.md` — **waived**: It affects only a multi-agent State-B run hosted by another agent, and adding a CLI rule to other agents' seeds is a permission change for the owner, not a mechanical edit. (decided by the rule)
- other: read-section's file form holds each section's text as one line, so a host reading by line offset cannot split a section — `runtime/src/primitives/read_section.rs` — **waived**: No shipped section reaches the default read size (the largest is 7,934 bytes), and a read size below one section is a host configuration this spec does not size for. (decided by the rule)
- other: past the bound the MCP response carries the result twice, as text (49,965 bytes for render-status on 700 specs) and as `structuredContent` (53,614) — `runtime/src/mcp/server.rs` — **waived**: Claude Code hands the model one copy, as every `mcp__ductus__` result in this run shows; the bound measures the result, and changing how the server's JSON wrapper serializes is 022's surface and not a mechanical edit. (decided by the rule)
- doc: 080's plan.md Affected Files omits files 080's commits changed, among them read_spec.rs, payload.rs, bounded_lists.rs and the bounded members — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: The plan's bounded-results decision covers those files, which rows to add is a judgment, and check-artifact-size's advisory on this plan asks for finished rows to be trimmed rather than added. (decided by the rule)
- doc: 023's AC36 and AC38 count nine retired entries where there are 13, and AC37 says the revoke conflict is raised before any filesystem read — `specs/023-govern-refinement/spec.md` — **waived**: The counts were false at 150b4dc1 too, and AC37 was never strictly true; correcting a done spec's criteria reopens it, a reopen that is 023's to spend. (decided by the rule)
- other: no 080 artifact classified the removed result fields as a breaking change or committed the release that ships them to a minor version; plan.md now does, citing 022's versioning (0.60.0), which the analysis found too — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- doc: fix round 1's re-read found the analyze reference's §Finding dispositions still saying detection writes nothing — `framework/reference/analyze.md` — **fixed** (decided by the rule)
- doc: fix round 1's re-read found plan.md's *The guide* naming only fewer than two `---` lines among render-help's malformed copies — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- bug: fix round 1's re-read found the non-file rule-entry notice printing the entry's name unflattened, unlike the disabled-rule-file notice beside it — `runtime/src/primitives/discover_rule_files.rs` — **fixed** (decided by the rule)
- bug: fix round 1's re-read found `DUCTUS_GEN_TIMEOUT_S=00` passing the generator's validation as a zero-second bound; an all-zero value is now refused, naming the variable — `scripts/gen-claude-commands.sh` — **fixed** (decided by the rule)
- doc: fix round 1's re-read found dashboard.rs's module doc calling the primitive read-only, though its file form writes a list file — `runtime/src/primitives/dashboard.rs` — **fixed** (decided by the rule)
- other: fix round 1's re-read found that `bounded()`'s watchdog can leave its `sleep` running when the call ends before the watchdog sets its trap — `scripts/gen-claude-commands.sh` — **waived**: Nothing hangs: an orphaned sleep holds only /dev/null and exits at the bound, 3 in 200 calls in the re-read's count, and closing the window needs a second signal handshake, not a mechanical edit. (decided by the rule)
- other: fix round 1's re-read found that the dashboard test drives only `session-unreadable` past the bound, so removing the `session-notices` move fails no test — `runtime/src/primitives/dashboard.rs` — **waived**: Both lists move by the same branch, which the test drives for one of them; a fixture that puts a session notice past the bound needs co-targeting session records, which is new test design, not a chore. (decided by the rule)
- other: fix round 2's re-read found the unrecognized-suffix notice and the `loading rule files:` line printing a rule file's name unflattened — `runtime/src/primitives/discover_rule_files.rs` — **waived**: Both lines predate this spec and print the names of files the project put in its own rule directory; flattening every notice name is a sweep beyond 080's change, not its fix. (decided by the rule)
- other: fix round 2's re-read found no test feeding the non-file notice a name holding a line hazard — `runtime/src/primitives/discover_rule_files.rs` — **waived**: The flattening is `flatten_line`'s, whose own tests and the pipeline view's cover it; a symlink named with a newline is a new fixture, not a chore. (decided by the rule)
- other: fix round 2's re-read found no generator-test case for the all-zero timeout refusal — `scripts/tests/test-gen-claude-commands.sh` — **waived**: The refusal was driven by hand for 0, 00 and abc, each refused naming the variable, and 05 accepted; a case per refused value is new test design, not a chore. (decided by the rule)
- other: fix round 2's re-read found that a timeout of 20 or more digits overflows `[`'s comparison and is let through, as it was before the round — `scripts/gen-claude-commands.sh` — **waived**: A contrived value a maintainer sets by hand; `sleep` then rejects it, so every runtime call is stopped at once and the generator fails, which is no silent pass. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/audit.md` — 029-bootstrap-runtime-autowire
- `.pi/prompts/ductus-configure.md` — 064-pi-host-support
- `.pi/prompts/ductus-help.md` — 022-deterministic-runtime, 064-pi-host-support
- `docs/shared-constitution.md` — 050-constitution, 055-shared-constitution
- `docs/slash-commands.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `framework/bootstrap/configure/pi.md` — 064-pi-host-support
- `framework/constitution.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 079-review-and-analysis-work-converges
- `framework/reference/audit.md` — 029-bootstrap-runtime-autowire
- `framework/rules/accessibility-frontend.md` — 069-accessibility-frontend-rules
- `framework/rules/api-backend.md` — 068-api-backend-rules
- `framework/rules/performance-frontend.md` — 070-performance-frontend-rules
- `runtime/CHANGELOG.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/append_inbox.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/append_question.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/brief_re_read.rs` — 022-deterministic-runtime, 052-spec-supersession-and-consolidation, 055-shared-constitution, 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_artifact_size.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_orphaned_references.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_prescriptions.rs` — 022-deterministic-runtime, 052-spec-supersession-and-consolidation, 055-shared-constitution, 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_review_freshness.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_stuck.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/classify_spec_edit.rs` — 022-deterministic-runtime, 050-constitution, 055-shared-constitution, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `runtime/src/primitives/compute_review_scope.rs` — 022-deterministic-runtime, 052-spec-supersession-and-consolidation, 055-shared-constitution, 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/create_feature.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/create_plan_artifacts.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/decisions.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/derive_references.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/fix_rounds.rs` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/inbox_standing.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/invalidate_review.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/label_criteria.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/mark_criterion.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/process_decisions.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/process_waivers.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/prune_project.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/read_re_read.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/relocate_audit_records.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/remove_inbox_item.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/resolve_constitutions.rs` — 050-constitution, 055-shared-constitution
- `runtime/src/primitives/resolve_feature.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/resolve_references.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/retire_feature.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/set_status.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/start_fix_rounds.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/write_analysis.rs` — 022-deterministic-runtime, 050-constitution, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `runtime/src/primitives/write_review.rs` — 022-deterministic-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 055-shared-constitution, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/schema/config_keys.rs` — 050-constitution, 055-shared-constitution, 078-review-fix-rounds-are-re-read-within-a-bound
- `runtime/src/schema/constitutions.rs` — 050-constitution, 055-shared-constitution
- `runtime/src/schema/paths.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/session.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/tests/concurrent_sessions.rs` — 062-concurrent-session-targets
- `runtime/tests/exec_subprocess.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/tests/fixtures/review-basic/specs/007-review/spec.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `runtime/tests/measure_reopenings.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/tests/specify_command.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/tests/walker.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `scripts/audit/README.md` — 026-framework-self-audit, 029-bootstrap-runtime-autowire
- `scripts/audit/marker-list-parity.sh` — 026-framework-self-audit
- `scripts/audit/rename-sweep-residue.sh` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `scripts/audit/runtime-probe-parity.sh` — 029-bootstrap-runtime-autowire
- `scripts/audit/step-reference-integrity.sh` — 026-framework-self-audit
- `scripts/mutation/analysis-standing-decision.py` — 058-findings-route-at-discovery
- `scripts/mutation/declined-note-edits.py` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/fix-round-record.py` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/loop-inputs-screened.py` — 078-review-fix-rounds-are-re-read-within-a-bound
- `scripts/mutation/own-record-is-no-round.py` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/resolve-constitutions-size.py` — 050-constitution, 055-shared-constitution
- `scripts/mutation/scope-attribution.py` — 022-deterministic-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 055-shared-constitution, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/015-tarball-fetch/analysis.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/review.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/spec.md` — 015-tarball-fetch
- `specs/022-deterministic-runtime/scenarios/config-resolution-single-probe.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/026-framework-self-audit/analysis.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/review.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-19-mechanical-sweep-exemption.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-19-review-freshness.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-19-says-what-it-examined.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-28-audit-family-registry-parity.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/028-antigravity-agent/tasks.md` — 028-antigravity-agent
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/archive-fetch-direct-codeload.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/project-inputs-asked-once.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/runtime-probe-parity-audit.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 029-bootstrap-runtime-autowire
- `specs/045-decision-state-drift-detection/analysis.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/review.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/spec.md` — 045-decision-state-drift-detection
- `specs/046-scenario-open-question-visibility/spec.md` — 046-scenario-open-question-visibility
- `specs/047-analyze-findings-durability/analysis.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/plan.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/review.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/spec.md` — 047-analyze-findings-durability
- `specs/047-analyze-findings-durability/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/050-constitution/plan.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/050-constitution/spec.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/050-constitution/tasks.md` — 050-constitution, 055-shared-constitution
- `specs/052-spec-supersession-and-consolidation/tasks.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/054-remove-supersession/analysis.md` — 054-remove-supersession
- `specs/054-remove-supersession/plan.md` — 054-remove-supersession
- `specs/054-remove-supersession/review.md` — 054-remove-supersession
- `specs/054-remove-supersession/spec.md` — 054-remove-supersession
- `specs/054-remove-supersession/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/055-shared-constitution/analysis.md` — 055-shared-constitution
- `specs/055-shared-constitution/data-model.md` — 022-deterministic-runtime, 050-constitution, 055-shared-constitution, 062-concurrent-session-targets
- `specs/055-shared-constitution/plan.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/review.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/scenarios/a-registered-source-is-named-with-its-description.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/scenarios/target-reports-each-constitutions-size.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/spec.md` — 022-deterministic-runtime, 050-constitution, 055-shared-constitution, 062-concurrent-session-targets
- `specs/055-shared-constitution/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/058-findings-route-at-discovery/analysis.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/data-model.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/plan.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/review.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/scenarios/analysis-drift-judges-the-record-it-writes.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/scenarios/analyze-findings-match-decisions-by-host-judgment.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/scenarios/findings-cost-in-proportion-to-risk.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/spec.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/061-updates-track-the-latest-release-tag/analysis.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/plan.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/review.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/spec.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/tasks.md` — 061-updates-track-the-latest-release-tag
- `specs/062-concurrent-session-targets/analysis.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/data-model.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/review.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/scenarios/first-writers-create-the-ignore-file-once.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/spec.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 007-govern-workflow, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-supplied-summary-carries-the-loops-account.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/write-analysis-and-the-second-gate.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/068-api-backend-rules/analysis.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/plan.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/review.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/spec.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/tasks.md` — 068-api-backend-rules
- `specs/069-accessibility-frontend-rules/analysis.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/plan.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/review.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/spec.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/tasks.md` — 069-accessibility-frontend-rules
- `specs/070-performance-frontend-rules/analysis.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/plan.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/review.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/spec.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/tasks.md` — 070-performance-frontend-rules
- `specs/071-command-reference-split/tasks.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md` — 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/scenarios/a-reviews-own-record-is-no-fix-round.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/analysis.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/data-model.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 052-spec-supersession-and-consolidation, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/review.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges

## Unexamined governance

*None.*
