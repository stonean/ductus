---
spec: 000-slash-commands
last-run: 2026-11-25T19:25:07Z
reviewed-against: 5f7be3fb4ec2d8ab01528f48107f5ed8f0e72b21
diff-base: 25bbd053a0ad1852d7ee117d716dbda76b9c6b0c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 8
unread:
  - .claude/commands/ductus/help.md
  - .pi/prompts/ductus-help.md
  - framework/commands/help.md
  - specs/000-slash-commands/analysis.md
  - specs/000-slash-commands/plan.md
  - specs/000-slash-commands/scenarios/criterion-route-after-draft.md
  - specs/000-slash-commands/scenarios/scenario-without-task-visibility.md
excluded: 651
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/a-done-stop-says-the-request-is-not-built.md: a578e1a0f2c9dfbaee173a31734aa64e2d42afd5d10f41df72fd9c47cd9b3402
  scenarios/analyze-reads-the-layout-derived-command-set.md: 134abf405d47afc5d4b055bd91105293db6ba2b434a82d0c1ea2a908a1f2e4d7
  scenarios/clarify-one-at-a-time.md: 74531377a3ba6862619c2a4aaf861084da5b53d69ac09a39f28740176f5c078d
  scenarios/command-autocomplete-summary.md: de4ef40b8d2a8587be508d333d502198cae4c312639b282ce363d8bb7253fa66
  scenarios/criterion-route-after-draft.md: cd3b4edc59fbb63cbf8b567fd230a510475c91776ef1eccba144b722e5bac87c
  scenarios/dashboard-dependencies-column.md: d1ce2860d34e9be0e4f34bb67651645e92b709bca1a1572833259923b9da0428
  scenarios/implement-skips-planned-prompt.md: adeb014c4192e84543d733fcf27f12907c2cada0d4950af9782a6168a6fac1c3
  scenarios/implement-stops-below-planned.md: 4e74cbbf5c625ee2b7415a3bc682f996d1b8356b2d25cb3203e8dcf08d9e0f81
  scenarios/scenario-without-task-visibility.md: 0b576aefdc3d0e90823258250cd156ff6861ad79f9894c256cf830fcc3cbed9c
  scenarios/target-argument-parsing.md: b91e1ec533ba1730f5b3760219ef3817c6238298ae8c351670a6edc9b6bb6db3
  scenarios/target-clear-flag.md: 1a64e209badd9131c6874a3786b01cc599e422965d21d9e7fac72acde67b114b
  scenarios/target-names-the-step-the-gate-would-name.md: 0d3ff3d98704f5df2d560838f193b1daadf073a62e5039ce9b215259b41bd31b
  scenarios/validation-gates.md: ad40135b38a38f59e616f28469869744f79196b625742bb013111475d72e4a07
criteria-judged: 15
criteria-met: 15
criteria-verdicts:
  - label: AC1
    digest: bdd10debe62849434565a03b9d3f91c44677ac3629fac62a05edf36ebdfd88db
  - label: AC2
    digest: 297e225796c7554f08ed3ff5d668644ca7b835925a7281b9b3ac40e357a12948
  - label: AC3
    digest: 5337c3fa693b2e486533412d9b547bde2441bb0bd82c7ad18d6f20e64262a604
  - label: AC4
    digest: 8a6bebfef4747e7b533836172ef9430ab110474694647138d018b840fadc6ab7
  - label: AC5
    digest: a38ad9953a7e84ec84482bde029a5fa07cb7b244e5fd8fc34a36b097b0c7d94a
  - label: AC6
    digest: 39b7f8202473c408dc28a01ceb30a37ef64e98b906c638362dc9c0d439866ed2
  - label: AC7
    digest: 5ec4954cc12cfe753d45bc6dbe81eae673ff1a6058d0c854e0204002c2e0d665
  - label: AC8
    digest: 3987003fb46682d99ddf88f94a73351a210538a4b903dabe466813c3f2d460e8
  - label: AC9
    digest: 5f77f55252a76cd71d5423ba913f3a76120cca5c804b59780fe55743b39d0af9
  - label: AC10
    digest: d5819cfe126c60892bbd942298e816982d92b31d95588bf64e717564fbf63d7b
  - label: AC11
    digest: 18fa05122504cd6a76ec4f5dfa5e0d6b8562792e00ea9318f8d36f5c31873a9a
  - label: AC12
    digest: 08dff4fea26afdd5953c2a972b580d0d29fcb900a981f4f6c75bfb99e000ee2b
  - label: AC13
    digest: a0d24be3c64db49317c0837500dd4a540395b410214115e51dfe2d054c691c6e
  - label: AC14
    digest: 1ca31ea648a3d040e261b6628878d3d2de019d576c104709250f59bc66b8ec79
  - label: AC15
    digest: 48421283d9aa72f8ac8116d136494e0f5dc05a8b0959786df881b3e60868eef0
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
decisions:
  - key: "other — b1d319f1's comment edits left uneven wraps: the MergePermissionsArgs doc in runtime/src/schema/primitives.rs breaks after \"Host-supplied from the\" with a short line, the WriteSessionArgs cli_config_dir doc runs past 100 columns, and check_artifacts.rs's registry doc comment lines are uneven. Cosmetic only, no behavior or claim affected. — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: Doc comments in schema/primitives.rs render as clap --help and MCP tool descriptions, which parity goldens pin; a rewrap is an unasked change to a shared surface for no behavioral gain. Pipeline-machinery-adjacent cosmetic, no loaded rule maps to it.
    decided-at: 2026-11-14T03:18:40Z
    decided-by: rule
  - key: perf — check-artifacts reads and parses framework/bootstrap/ductus.md twice per run, once in check_criterion_path_existence and once in check_body_path_existence, though the comment says it is read once (predates the window) — `runtime/src/primitives/check_artifacts.rs:1790`
    outcome: waived
    reason: Two reads of one local file per run, no input bound at stake, and the fix touches the done spec 067's checks for no behavior change; no loaded performance rule fires against a local stdio process.
    decided-at: 2026-11-14T03:18:40Z
    decided-by: rule
  - key: bug — judge() refuses a path under any directory a root `.augment`, `.agents` or `.opencode` link leads to, before the CONFIG_DIRS loop runs, so a project whose `.augment` (or `.agents`, `.opencode`) links to `.claude` has every `/ductus` command write under `.claude/commands` refused with a message naming the other tool's directory. Before b1d319f1 such a link was judged as a config directory and the command row passed. Fails closed, and the tests cover `.claude` linking outward but not another tool's directory linking to the project's own config directory. — `runtime/src/primitives/host_config.rs:460`
    outcome: waived
    reason: The refusal of other tools' configuration paths is the owner's decision recorded in b1d319f1's message (2026-11-11); the case fails closed with a refusal that names the link, and the dropped hosts leave no supported project with such a link. Observation about the runtime's own machinery made reviewing framework work (AGENTS.md Workflow).
    decided-at: 2026-11-14T03:18:40Z
    decided-by: rule
  - key: "other — `ductus exec` silently drops any argument that is not `key=value`: `ductus exec target 062` runs the walker with no overlay and no signal. The code says overlay `key=value` overrides; the clap help for `args` says `Arguments forwarded to the command`. — `runtime/src/main.rs:900`"
    outcome: waived
    reason: Predates the window and the walker takes no positional argument (spec 022's exec protocol, ductus exec dispatches a step's primitive with none); a positional would be an authoring error caught by the command's own steps. Observation about the pipeline's own machinery made reviewing framework work (AGENTS.md Workflow).
    decided-at: 2026-11-14T03:18:40Z
    decided-by: rule
  - key: "other — the generated copy .ductus/reference/audit.md item 35 (Family 35b) says the Slash commands entries' destinations \"carry `{config_dir}` and `ductus` placeholders\"; the source framework/reference/audit.md says `{project}`, and the generator's blind {project} -> ductus substitution made it name a placeholder called `ductus`. Predates the window (b1d319f1 did not touch that sentence); maintainer-only reference, and the sentence still reads as the intended claim. — `.ductus/reference/audit.md:55`"
    outcome: waived
    reason: An observation about the pipeline's own machinery (the command/reference generator) made while reviewing framework work, predating the window; the generated copy equals its source under the generator's substitution, so the only fix is a source rewording that adds no requirement and reaches no adopter (audit is maintainer-only).
    decided-at: 2026-11-14T03:18:40Z
    decided-by: rule
  - key: "convention — amend.md heading 'Approval gate (both routes)' predates the criterion route; the gate now covers question, scenario and criterion. Not a chore: 014's done AC3 cites the heading by name ('§Approval gate (both routes)'), so a rename reaches a done spec's criterion text and a reopen decision. — `framework/commands/amend.md:163`"
    outcome: waived
    reason: Renaming the heading would have to sweep 014 AC3's citation of it, a done spec's criterion text; the parenthetical misleads no step, since the gate's own text lists all three routes.
    decided-at: 2026-11-14T03:18:40Z
    decided-by: rule
  - key: other — the Pi prompt copies of the five commands (.pi/prompts/ductus-*.md) carry the claude-style invocation (`/ductus:target`, `/ductus:clarify`, ...) in their bodies, while Pi invokes `/ductus-<name>` (ductus.md §Derived values, Invocation row). The copies are faithful to their sources under the 064 AC5 rule (verbatim body, `{project}` substituted), so this is the pipeline's own design, not a defect of 000's edits. — `.pi/prompts/ductus-target.md:17`
    outcome: waived
    reason: "Pipeline machinery observed while reviewing framework work: 064 AC5 decided the Pi copies install verbatim with `{project}` substituted, and 000's spec records that the colon form is claude-style alone; it breaks no 000 contract and makes no result of this review wrong."
    decided-at: 2026-11-14T03:18:40Z
    decided-by: rule
  - key: "convention — AGENTS.md's new bullet calls the step \"a spec's pre-done verification\", a term no framework artifact defines (framework/reference/implement.md §Completion gate names the 'Verify each criterion' steps and, separately, the 'Pre-done gate' = check-review-gate); a reader must infer which is meant, and the bullet's 687K-token figure states its method only in commit 4d681fad's message, not in the bullet — `AGENTS.md:53`"
    outcome: waived
    reason: Wording of a project-file bullet the owner decided (2026-11-14). The ordering it states (review, then criteria verification, then analysis) agrees with framework/reference/implement.md §Completion gate, and the measurement method is recorded in the commit message. No behavior, rule or contract is involved.
    decided-at: 2026-11-14T03:18:40Z
    decided-by: rule
---

# Review — 000-slash-commands

## Summary

Review of 000's code changed since its last review, from `25bbd053` (`last-review`) to `5f7be3fb`, for 084's task 19. The five passes read the spec's own change, the hunks of `a2c58abc` (`chore(000)`) in `framework/commands/help.md` and its two generated copies, against the five loaded rule files, each read whole (231,962 bytes): no finding and no observation. The criteria step judged all 15 criteria met against the tree, running `scripts/gen-claude-commands.sh --check`, `render-help` and `render-status`, and the `create_feature` and plan-artifact tests. Stored decisions: 8 retained on files this scope does not hold, and 2 expired. The deferral of `QUAL-CLAIM-001` at `runtime/src/primitives/check_artifacts.rs` to 067's task 31 was recorded before sites, so it expires. The waived observation that scenario `criterion-route-after-draft`'s second Resolved Question quotes a back-edge the constitution no longer holds no longer stands, judged by the host: 084's sweep added "Superseded in part (spec 084)" beneath that answer, stating the two back-edges in force, and the stored reason cited the retired one. Examined 1 of 8: `specs/000-slash-commands/spec.md`, read whole by the criteria step. Unread, by the operator's decision to read each spec's own change (084's `tasks.md`, task 19): `framework/commands/help.md`, `.claude/commands/ductus/help.md` and `.pi/prompts/ductus-help.md`, read at the slice's hunks, the two copies identical by diff; and `specs/000-slash-commands/analysis.md`, `plan.md` and its two scenarios, not opened by the passes. The host read the scenario `criterion-route-after-draft.md`'s Resolved Questions to check the expired decision.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

10 of 664 commits in the window no spec claimed.

15 of 15 acceptance criteria judged met.

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

## Acceptance criteria

- AC1 — met: `git show 2c838b20 --stat` lists 13 files under `commands/` (no `next.md`), `git log --all --oneline -- '*next.md'` is empty, and `framework/bootstrap/ductus.md` §Slash commands manifest lists the live set (16 commands plus `configure`) matching the 17 files in `framework/commands/` less the maintainer-only `audit.md`; the claim holds as its own text qualifies it.
- AC2 — met: `scripts/gen-claude-commands.sh` `substitute()` rewrites only `{project}` and `{cli-config-dir}` (`scripts/gen-claude-commands.sh --check` exits 0); the remaining braces in `framework/commands/*.md` are runtime display slots, not substituted placeholders.
- AC3 — met: Gate tables in `framework/commands/clarify.md` (Gate table), `framework/commands/plan.md` (## Gate: draft to clarify, planned/in-progress to implement, done to amend) and `framework/commands/implement.md` step 4 (draft to clarify, clarified to plan, done to amend, planned to in-progress without prompt) match the constitution's lifecycle; `specify` creates `draft` via `create-feature` (`cargo test --lib create_feature`: 27 passed).
- AC4 — met: `framework/commands/help.md` Scope Boundaries forbid reading files or invoking primitives and print a pre-generated fixed guide; driven `runtime/target/release/ductus render-help` returns the guide from the installed help copy only, read-only per its description.
- AC5 — met: `framework/commands/target.md` step 8 invokes `write-session` (feature, path, scenario) and step 9 displays feature status, artifacts, dependencies, open questions and next step; no runtime test driven, judged from the command source and the `write-session` primitive description.
- AC6 — met: `framework/commands/status.md` steps 1-2 call `render-status`; driving `runtime/target/release/ductus render-status` in the review worktree returned the table of all spec directories with Status, Plan, Tasks, Data-model, Scenarios, Dependencies and Next Action columns.
- AC7 — met: Claim is that `next` was never delivered and its job is done by `status`'s Next Action column and `target`'s next-step line: `git log --all -- '*next.md'` is empty, `framework/commands/` holds no `next.md`, and the `render-status` output carries a Next Action column and `framework/commands/target.md` step 9 names the next step.
- AC8 — met: `framework/commands/analyze.md` steps 2 (`validate-frontmatter`), 3 (`traverse-deps`), 4 (`resolve-anchor`), 8 (`check-artifacts` families spec-integrity, plan-consistency, task consistency, artifact completeness, cross-spec impact) and 13 (cross-service references) cover every named area, confirmed in the `check-artifacts --help` family list.
- AC9 — met: `framework/bootstrap/configure/claude.md` canonical allow set holds `Bash(git add *)`, `Bash(git status *)`, `Bash(npx markdownlint-cli2 *)`, `Bash(ls *)` and the read-only `mcp__ductus__*` tools (generated from `framework/runtime-tools.txt`); file reads go through the host's Read tool and the runtime primitives, as the file states.
- AC10 — met: `framework/commands/specify.md` step 5 invokes `create-feature`, which numbers max+1 and creates the directory (`runtime/src/primitives/create_feature.rs`, tests `numbers_from_max_existing_prefix_plus_one` and `never_touches_existing_feature_directories` pass); no README write exists in `specify.md` or `create_feature.rs`, and `9847647b` dropped the README table generator.
- AC11 — met: `framework/commands/specify.md` step 5 and `runtime/src/primitives/create_feature.rs` `resolve_template` read `{specs-root}/templates/spec.md` first (test `installed_template_wins_over_framework_source` passes), and `create_plan_artifacts.rs` copies from `specs/templates/` (`scaffolds_plan_and_tasks_from_templates` passes); the `framework/templates/spec/` fallback is only the ductus repo's own layout.
- AC12 — met: Criterion states the `{project}-session.json` reference was superseded by 022: no live command or generator source in `framework/commands/*.md`, `framework/bootstrap/configure/claude.md` or `runtime/src` references it (only `framework/migrations/session-file-consolidate.md` and `framework/migrations.toml` name it as the legacy file to migrate), `4c5c4aea` is the consolidation commit, and `framework/commands/target.md` writes `.ductus/session.toml` via `write-session`.
- AC13 — met: `framework/commands/analyze.md` step 7 invokes `lint-markdown` on the feature's markdown files, and `runtime/target/release/ductus lint-markdown --help` states it wraps `npx markdownlint-cli2` (preferring the local install per `AGENTS.md`); no lint run driven.
- AC14 — met: Criterion records removal by 023: `framework/commands/specify.md` asks no qualifying questions (step 6 writes the spec body directly) and `grep -rn spec-and-plan framework/commands framework/templates` finds nothing.
- AC15 — met: Criterion records removal by 023 and the legacy read tier: `framework/commands/*.md` carry no two-filename fallback, while `runtime/src/primitives/mod.rs` `is_spec_path` still accepts `spec-and-plan.md` as the criterion states.

## Observations

*None.*

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md`
- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/clarify.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/link.md`
- `.claude/commands/ductus/plan.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.claude/commands/ductus/status.md`
- `.claude/commands/ductus/target.md`
- `.ductus/config.toml`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/consolidate.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/prune.md`
- `.ductus/reference/review.md`
- `.ductus/reference/specify.md`
- `.ductus/reference/status.md`
- `.gitattributes`
- `.githooks/pre-commit`
- `.githooks/pre-push`
- `.github/workflows/framework-checks.yml`
- `.github/workflows/runtime-acquisition.yml`
- `.github/workflows/runtime-currency.yml`
- `.github/workflows/runtime-release.yml`
- `.github/workflows/runtime.yml`
- `.pi/extensions/ductus.ts`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-clarify.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-link.md`
- `.pi/prompts/ductus-plan.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `.pi/prompts/ductus-status.md`
- `.pi/prompts/ductus-target.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `docs/developing-ductus.md`
- `docs/rules.md`
- `docs/runtime.md`
- `docs/shared-constitution.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus-procedure.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/bootstrap/hooks/ductus-pre-push`
- `framework/bootstrap/hooks/pre-push`
- `framework/bootstrap/pi/ductus-bridge.ts`
- `framework/commands/amend.md`
- `framework/commands/analyze.md`
- `framework/commands/clarify.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/groom.md`
- `framework/commands/implement.md`
- `framework/commands/link.md`
- `framework/commands/plan.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/commands/status.md`
- `framework/commands/target.md`
- `framework/constitution.md`
- `framework/migrations.toml`
- `framework/migrations/audit-record-relocate.md`
- `framework/migrations/criterion-label-backfill.md`
- `framework/migrations/project-review-record.md`
- `framework/migrations/remove-legacy-constitutions-region.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/consolidate.md`
- `framework/reference/fold.md`
- `framework/reference/groom.md`
- `framework/reference/implement.md`
- `framework/reference/plan.md`
- `framework/reference/prune.md`
- `framework/reference/review.md`
- `framework/reference/specify.md`
- `framework/reference/status.md`
- `framework/rules/accessibility-frontend.md`
- `framework/rules/api-backend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/reliability-backend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-cross.md`
- `framework/rules/security-frontend.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `framework/templates/spec/spec.md`
- `install.sh`
- `runtime/CHANGELOG.md`
- `runtime/Cargo.lock`
- `runtime/Cargo.toml`
- `runtime/README.md`
- `runtime/src/host.rs`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/io.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/flag_help.rs`
- `runtime/src/mcp/mod.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/append_question.rs`
- `runtime/src/primitives/apply_manifest.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_command_flags.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_departures.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_prescriptions.rs`
- `runtime/src/primitives/check_promotion_coverage.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/check_stuck.rs`
- `runtime/src/primitives/check_unreviewed_code.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/criterion_verdicts.rs`
- `runtime/src/primitives/dashboard.rs`
- `runtime/src/primitives/decisions.rs`
- `runtime/src/primitives/departures.rs`
- `runtime/src/primitives/derive_references.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/host_config.rs`
- `runtime/src/primitives/invalidate_review.rs`
- `runtime/src/primitives/label_criteria.rs`
- `runtime/src/primitives/line_citations.rs`
- `runtime/src/primitives/lint_markdown.rs`
- `runtime/src/primitives/mark_criterion.rs`
- `runtime/src/primitives/mark_task.rs`
- `runtime/src/primitives/measure_reopenings.rs`
- `runtime/src/primitives/mechanical_sweep.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/merge_permissions.rs`
- `runtime/src/primitives/merge_status_hook.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/owed_work.rs`
- `runtime/src/primitives/pipeline_view.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_project.rs`
- `runtime/src/primitives/prune_tasks.rs`
- `runtime/src/primitives/read_re_read.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/read_tasks.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/remove_inbox_item.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/review_subject.rs`
- `runtime/src/primitives/rewrite_spec_links.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/set_status.rs`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `runtime/src/primitives/spec_links.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/moved_scope.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/src/primitives/write_review/seed.rs`
- `runtime/src/schema/config_keys.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/extensions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/src/schema/protocol.rs`
- `runtime/src/schema/registry.rs`
- `runtime/src/schema/severity.rs`
- `runtime/src/session.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/check_unreviewed_code_cli.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/derive_dependencies_golden.rs`
- `runtime/tests/derive_references_golden.rs`
- `runtime/tests/exec_stderr_gone.rs`
- `runtime/tests/exec_subprocess.rs`
- `runtime/tests/fix_rounds.rs`
- `runtime/tests/fixtures/ductus-basic/framework/bootstrap/install.md`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/plan-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/golden/specify-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `runtime/tests/mcp.rs`
- `runtime/tests/measure_reopenings.rs`
- `runtime/tests/mechanical_sweep_parity.rs`
- `runtime/tests/merge_status_hook.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/review_command.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/judge-brief.md`
- `scripts/analyze/record.py`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/audit/README.md`
- `scripts/audit/adopter-shell-behavior.sh`
- `scripts/audit/archive-contents.sh`
- `scripts/audit/cross-doc-consistency.sh`
- `scripts/audit/done-spec-criteria.sh`
- `scripts/audit/permission-wildcard-position.sh`
- `scripts/audit/promotion-coverage.sh`
- `scripts/audit/rename-sweep-residue.sh`
- `scripts/audit/run-all.sh`
- `scripts/audit/ssot-invariants.sh`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/audit/sweep-target-manifest-parity.sh`
- `scripts/check-release-tag.sh`
- `scripts/measure-replace-under-server.sh`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/compare-and-set-066.py`
- `scripts/mutation/criteria-in-review-066.py`
- `scripts/mutation/criterion-ticks-outside-the-digest.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/departures.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/freshness-results-have-a-bounded-size.py`
- `scripts/mutation/installed-file-modes.py`
- `scripts/mutation/json-flag-help-022.py`
- `scripts/mutation/line-citation-strip-066.py`
- `scripts/mutation/loop-inputs-screened.py`
- `scripts/mutation/moved-scope-066.py`
- `scripts/mutation/never-owed-084.py`
- `scripts/mutation/owed-work-084.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/plan-edit-takes-no-back-edge.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/prune-open-region.py`
- `scripts/mutation/prune-project.py`
- `scripts/mutation/prune-review-fixes-041.py`
- `scripts/mutation/quoted-heading-forms-067.py`
- `scripts/mutation/reachable-trigger-bar-078.py`
- `scripts/mutation/record-path-quoting-066.py`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/resolve-constitutions-size.py`
- `scripts/mutation/review-022-fixes-2026-11-19.py`
- `scripts/mutation/review-022-task-41.py`
- `scripts/mutation/review-022-task-42.py`
- `scripts/mutation/review-022-task-44.py`
- `scripts/mutation/review-022-task-46.py`
- `scripts/mutation/review-departures-bounded.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/rewrite-spec-links.py`
- `scripts/mutation/routing-path-claims.py`
- `scripts/mutation/rule-files-left-to-analysis-020.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/mutation/run.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/site-anchors-020.py`
- `scripts/mutation/status-count-084.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/mutation/suffixed-task-number.py`
- `scripts/mutation/unreviewed-code-084.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/tests/pi-bridge-harness.mjs`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-archive-contents.sh`
- `scripts/tests/test-audit-path-containment.sh`
- `scripts/tests/test-check-release-tag.sh`
- `scripts/tests/test-done-spec-owed-work.sh`
- `scripts/tests/test-pre-commit-staging.sh`
- `scripts/tests/test-run-workflow-steps.sh`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/008-security-rules/analysis.md`
- `specs/008-security-rules/plan.md`
- `specs/008-security-rules/review.md`
- `specs/008-security-rules/scenarios/a-cache-partition-is-purged-when-the-session-ends.md`
- `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md`
- `specs/008-security-rules/scenarios/a-throwaway-credential-owes-no-revocation-path.md`
- `specs/008-security-rules/scenarios/an-absent-origin-on-a-cookie-connection-is-flagged.md`
- `specs/008-security-rules/scenarios/be-deps-005-flags-what-its-statement-requires.md`
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/scenarios/the-tag-check-reads-every-entry-above-the-newest-version.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/010-agent-autonomy/plan.md`
- `specs/011-brownfield-process/spec.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/spec.md`
- `specs/013-text-first-artifacts/plan.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/014-reclarify-backedge/plan.md`
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md`
- `specs/014-reclarify-backedge/spec.md`
- `specs/015-tarball-fetch/analysis.md`
- `specs/015-tarball-fetch/plan.md`
- `specs/015-tarball-fetch/review.md`
- `specs/015-tarball-fetch/spec.md`
- `specs/015-tarball-fetch/tasks.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/data-model.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md`
- `specs/017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/analysis.md`
- `specs/018-adopter-owned-pre-commit/plan.md`
- `specs/018-adopter-owned-pre-commit/review.md`
- `specs/018-adopter-owned-pre-commit/spec.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/scenarios/waiver-file-lists.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/analysis.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/review.md`
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md`
- `specs/022-deterministic-runtime/scenarios/append-inbox-comment-aware-write.md`
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`
- `specs/022-deterministic-runtime/scenarios/done-when-authoring-forms.md`
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md`
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/host-protocol-conformance.md`
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md`
- `specs/022-deterministic-runtime/scenarios/installed-files-are-not-group-writable.md`
- `specs/022-deterministic-runtime/scenarios/lint-markdown-tool-resolution.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/severity-is-a-closed-set-not-a-string.md`
- `specs/022-deterministic-runtime/scenarios/skipscanner-inline-code-exemption.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
- `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md`
- `specs/022-deterministic-runtime/scenarios/unchecked-done-when-clause-tally.md`
- `specs/022-deterministic-runtime/scenarios/writecode-payload-bundling.md`
- `specs/022-deterministic-runtime/scenarios/writecode-payload-canonicalize-paths.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/023-govern-refinement/analysis.md`
- `specs/023-govern-refinement/plan.md`
- `specs/023-govern-refinement/review.md`
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md`
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md`
- `specs/023-govern-refinement/scenarios/living-specs.md`
- `specs/023-govern-refinement/spec.md`
- `specs/023-govern-refinement/tasks.md`
- `specs/024-rule-loader/analysis.md`
- `specs/024-rule-loader/plan.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/025-rule-opt-out/review.md`
- `specs/025-rule-opt-out/spec.md`
- `specs/026-framework-self-audit/analysis.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/026-framework-self-audit/review.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/027-bootstrap-migration-registry/analysis.md`
- `specs/027-bootstrap-migration-registry/plan.md`
- `specs/027-bootstrap-migration-registry/spec.md`
- `specs/030-cross-service-references/plan.md`
- `specs/030-cross-service-references/spec.md`
- `specs/034-performance-backend-rules/analysis.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md`
- `specs/035-groom-session-target/scenarios/reopen-done-spec-on-scenario.md`
- `specs/037-observability-backend-rules/analysis.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/analysis.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/039-reliability-backend-rules/analysis.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/scenarios/expendable-messages-may-be-dropped-when-recorded.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
- `specs/040-configurable-specs-dir/spec.md`
- `specs/041-task-pruning/analysis.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/review.md`
- `specs/041-task-pruning/scenarios/a-plan-edit-does-not-reopen-a-done-spec.md`
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md`
- `specs/041-task-pruning/scenarios/an-open-fenced-block-or-comment-is-not-examined.md`
- `specs/041-task-pruning/scenarios/prune-all-walks-every-spec.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/041-task-pruning/tasks.md`
- `specs/046-scenario-open-question-visibility/plan.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/047-analyze-findings-durability/spec.md`
- `specs/048-govern-acquired-runtime/analysis.md`
- `specs/048-govern-acquired-runtime/data-model.md`
- `specs/048-govern-acquired-runtime/plan.md`
- `specs/048-govern-acquired-runtime/review.md`
- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/048-govern-acquired-runtime/tasks.md`
- `specs/049-rename-govern-to-ductus/plan.md`
- `specs/049-rename-govern-to-ductus/spec.md`
- `specs/050-constitution/analysis.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/review.md`
- `specs/050-constitution/scenarios/a-canonical-source-is-pointed-at-not-copied.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/a-retired-filename-leaves-a-decision-record.md`
- `specs/050-constitution/scenarios/adopter-practices-classified-for-promotion.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/051-branch-scoped-spec-numbering/analysis.md`
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/review.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-definition-and-a-title-follow-the-link-grammar.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-pending-fold-never-holds-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/fold-target-checked-before-the-rewrite.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/rewrites-preserve-line-endings.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/054-remove-supersession/plan.md`
- `specs/054-remove-supersession/spec.md`
- `specs/055-shared-constitution/analysis.md`
- `specs/055-shared-constitution/plan.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/analysis.md`
- `specs/057-analyze-artifact-and-record-relocation/data-model.md`
- `specs/057-analyze-artifact-and-record-relocation/review.md`
- `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/058-findings-route-at-discovery/analysis.md`
- `specs/058-findings-route-at-discovery/data-model.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/review.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/061-updates-track-the-latest-release-tag/plan.md`
- `specs/062-concurrent-session-targets/analysis.md`
- `specs/062-concurrent-session-targets/data-model.md`
- `specs/062-concurrent-session-targets/plan.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/062-concurrent-session-targets/scenarios/first-writers-create-the-ignore-file-once.md`
- `specs/062-concurrent-session-targets/spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/064-pi-host-support/analysis.md`
- `specs/064-pi-host-support/plan.md`
- `specs/064-pi-host-support/review.md`
- `specs/064-pi-host-support/scenarios/colon-form-commands-run-on-pi.md`
- `specs/064-pi-host-support/spec.md`
- `specs/064-pi-host-support/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-write-result-states-what-it-wrote.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/record-paths-are-quoted-only-when-they-need-it.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/analysis.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-plan-heading-is-matched-without-regard-to-case.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-results-are-bounded.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphan-check-skips-ignored-local-state.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/scenarios/promotion-coverage-header-without-a-key.md`
- `specs/067-analyze-and-audit-checks/scenarios/shared-constitution-references-are-judged.md`
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/analysis.md`
- `specs/068-api-backend-rules/plan.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/069-accessibility-frontend-rules/analysis.md`
- `specs/069-accessibility-frontend-rules/plan.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/scenarios/verifications-flag-only-what-the-statement-covers.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`
- `specs/070-performance-frontend-rules/analysis.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/071-command-reference-split/analysis.md`
- `specs/071-command-reference-split/plan.md`
- `specs/071-command-reference-split/review.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/073-decided-rule-departures-are-recorded/analysis.md`
- `specs/073-decided-rule-departures-are-recorded/data-model.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/073-decided-rule-departures-are-recorded/tasks.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/analysis.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/review.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md`
- `specs/077-security-cross-rules/analysis.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/review.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/analysis.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/a-step-gates-on-a-bounded-list-by-its-count.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`
- `specs/082-rule-assessments-converge-across-runs/analysis.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/review.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/082-rule-assessments-converge-across-runs/tasks.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/analysis.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/review.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/tasks.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/data-model.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`
- `specs/README.md`
- `specs/inbox.md`
- `specs/review.md`
- `version`

## Unexamined governance

*None.*
