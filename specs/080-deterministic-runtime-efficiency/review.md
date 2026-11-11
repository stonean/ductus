---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-11T00:12:04Z
reviewed-against: b0ad74d582596f0f29bb069067c45b8a81d31e0f
diff-base: ba5166e3f10165d83fe5aeabbc2c1d536a74745f
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 9
scope: 9
excluded: 14
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
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
  fixed: 1
  deferred: 0
  waived: 7
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
  - key: "other: for a namespace holding a tab or a line break, Pi's bridge respells the status view with render-help's unflattened namespace, so the view's flattened hints keep their colon form — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: Such a namespace names no command on any host, since Pi registers `{project}-status` from it and every installed command file is named by it, so no respelled hint could run; aligning the bridge would change 064's verbatim bridge, not a mechanical edit.
    decided-at: 2026-11-11T00:12:04Z
    decided-by: rule
  - key: "other: a `|` in a slug, status or tag adds a cell to the model path's markdown table, since view_text replaces only control characters and tabs — `runtime/src/primitives/pipeline_view.rs`"
    outcome: waived
    reason: No loaded rule covers a printable character, the pre-080 dashboard's table rendered the same cells raw (150b4dc1), and escaping each cell changes the table the model path re-types, which is not a mechanical edit.
    decided-at: 2026-11-11T00:12:04Z
    decided-by: rule
---

# Review — 080-deterministic-runtime-efficiency

## Summary

A review of the window this session's analysis fixes opened, from ba5166e3, 080's closing review, to b0ad74d5. It is not a second closing review. The scope is the 9 files 080's commits 75dc3d96 and 36a4ce86 changed, and every pass read 080's own change in each, with its context:

- pipeline_view.rs: every changed hunk with its function, and the non-test code's other view values, `inbox.path`, the config items and the reopenings line.
- rule_sections.rs: whole but for the tier tests.
- render_status.rs: its run and tests.
- dashboard.rs: `view_model` and `Loaded::model`.
- schema/primitives.rs: the `LoadedRule` struct. The window's other 52 changed lines are 067's d90d2133 and were not read; they are 067's change.
- gen-claude-commands.sh: lines 70-142 and the apply-manifest call.
- test-gen-claude-commands.sh: its header and case E.
- docs/runtime.md: the inventory table.
- plan.md: whole at 75dc3d96, then each later hunk.

Rules: the 9 backend and cross files by discover-rule-files, all 1,267 lines of its list read.

Checks run: cargo fmt --check, cargo clippy --all-targets -D warnings, cargo test --lib (2,708), tests/mcp.rs, tests/hook_status.rs, tests/parity.rs, and scripts/tests/test-gen-claude-commands.sh. Each new test was shown failing with its behavior reverted.

No MUST or SHOULD violation. Five stored decisions on files in scope still stand and matched. One doc observation was fixed: the bridge claim in dashboard.rs, render_status.rs and plan.md. Two were waived by the rule. The fix's one round was re-read clean by an independent agent.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- doc: 080's plan.md Affected Files omits files 080's commits changed, among them read_spec.rs, payload.rs, bounded_lists.rs and the bounded members — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: The plan's bounded-results decision covers those files, which rows to add is a judgment, and check-artifact-size's advisory on this plan asks for finished rows to be trimmed rather than added. (decided by the rule)
- other: fix round 1's re-read found that `bounded()`'s watchdog can leave its `sleep` running when the call ends before the watchdog sets its trap — `scripts/gen-claude-commands.sh` — **waived**: Nothing hangs: an orphaned sleep holds only /dev/null and exits at the bound, 3 in 200 calls in the re-read's count, and closing the window needs a second signal handshake, not a mechanical edit. (decided by the rule)
- other: fix round 1's re-read found that the dashboard test drives only `session-unreadable` past the bound, so removing the `session-notices` move fails no test — `runtime/src/primitives/dashboard.rs` — **waived**: Both lists move by the same branch, which the test drives for one of them; a fixture that puts a session notice past the bound needs co-targeting session records, which is new test design, not a chore. (decided by the rule)
- other: fix round 2's re-read found no generator-test case for the all-zero timeout refusal — `scripts/tests/test-gen-claude-commands.sh` — **waived**: The refusal was driven by hand for 0, 00 and abc, each refused naming the variable, and 05 accepted; a case per refused value is new test design, not a chore. (decided by the rule)
- other: fix round 2's re-read found that a timeout of 20 or more digits overflows `[`'s comparison and is let through, as it was before the round — `scripts/gen-claude-commands.sh` — **waived**: A contrived value a maintainer sets by hand; `sleep` then rejects it, so every runtime call is stopped at once and the generator fails, which is no silent pass. (decided by the rule)
- doc: the comments on render-status's namespace say Pi's bridge respells every hint with it, but a runtime that serves render-help, as this one does, hands the bridge render-help's unflattened namespace — `runtime/src/primitives/dashboard.rs` — **fixed** (decided by the rule)
- other: for a namespace holding a tab or a line break, Pi's bridge respells the status view with render-help's unflattened namespace, so the view's flattened hints keep their colon form — `runtime/src/primitives/dashboard.rs` — **waived**: Such a namespace names no command on any host, since Pi registers `{project}-status` from it and every installed command file is named by it, so no respelled hint could run; aligning the bridge would change 064's verbatim bridge, not a mechanical edit. (decided by the rule)
- other: a `|` in a slug, status or tag adds a cell to the model path's markdown table, since view_text replaces only control characters and tabs — `runtime/src/primitives/pipeline_view.rs` — **waived**: No loaded rule covers a printable character, the pre-080 dashboard's table rendered the same cells raw (150b4dc1), and escaping each cell changes the table the model path re-types, which is not a mechanical edit. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/bootstrap/ductus.md` — 071-command-reference-split
- `framework/bootstrap/govern.md` — 071-command-reference-split
- `runtime/src/main.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_dependencies.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_references.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/spec_links.rs` — 067-analyze-and-audit-checks
- `runtime/src/schema/descriptions.rs` — 067-analyze-and-audit-checks
- `runtime/tests/derive_unwritable_cli.rs` — 067-analyze-and-audit-checks
- `scripts/mutation/owner-reviews-067.py` — 067-analyze-and-audit-checks
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 060-exec-analyze-assesses-each-loaded-rule
- `specs/067-analyze-and-audit-checks/data-model.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks
- `specs/071-command-reference-split/tasks.md` — 071-command-reference-split

## Unexamined governance

*None.*
