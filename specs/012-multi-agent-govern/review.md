---
spec: 012-multi-agent-govern
last-run: 2026-11-25T19:20:44Z
reviewed-against: c9257d47c3c56a7b3112072558c641851b8a447a
diff-base: 067d94f6892feb3e5a7796f248b329c46dfbc869
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 12
unread:
  - .claude/commands/ductus/consolidate.md
  - .claude/commands/ductus/fold.md
  - .github/workflows/runtime-release.yml
  - .github/workflows/runtime.yml
  - .pi/prompts/ductus-consolidate.md
  - .pi/prompts/ductus-fold.md
  - framework/bootstrap/govern.md
  - runtime/CHANGELOG.md
  - runtime/src/schema/primitives.rs
  - specs/012-multi-agent-govern/analysis.md
  - specs/012-multi-agent-govern/plan.md
excluded: 686
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 3811330ca711759e48b2e1658edced9b1c93bb1a85d7ea641d371ff33e3a64b0
  scenarios/a-dropped-hosts-leftover-tree-is-named.md: b90dd4b9460182627500c70781228121fc32704ea3e4501e01c42b751726e735
  scenarios/settings-template-bash-allowlist.md: 6fb0d4346bb688e9b02a14ca19f6e9f515ab5d927483d5e4bc505f0c551349ac
criteria-judged: 19
criteria-met: 19
criteria-verdicts:
  - label: AC1
    digest: b98680b9a11c294b052203dac4f2c337d9b257e5e02925afa5b14dc8195761ad
  - label: AC2
    digest: abc3b18fe3c740609e25b1826979b56a00045d9db9153c55e9a2d5c6a25f25ef
  - label: AC3
    digest: 5716864920a904115db2ea63f08ff176eb6020a8e916a4deada8dfeb14cac621
  - label: AC4
    digest: da18e7102b40069e06d7f77a7059ad5f4bf8e4a485354757c4b06ae5f37e57f2
  - label: AC5
    digest: b30d4b45a3a89424dd26256339231d3567c61e4769e8cb8225ca336551c9ab4d
  - label: AC6
    digest: 9c9d083f9a380aaec06f2490f722bb54a00423a7842d21aa2645b99e1e678524
  - label: AC7
    digest: 303aa1a0442a741f58e14b71825fb757fde96722cbec7d7ac4d00b12f603728b
  - label: AC8
    digest: c129dae3813cab13f57890b27038ac3e8ffbdc809b765db90ba704c9e40e366f
  - label: AC9
    digest: 8e153093f9960cbdc687098f4746bbc40f307f63b5fd3684031eab304f5648a1
  - label: AC10
    digest: b1478529ede76f921d3cd49c339607d8ad0f63234bb97df96cfa3387bd055e27
  - label: AC11
    digest: 04001c0f49458f802e85c44b12b05c0404fa487b8c66c21e5837b0c33c987836
  - label: AC12
    digest: f8ca2627eaebf752a71a7a81b64e9ec0dec0857df5b34766031f2634cc017166
  - label: AC13
    digest: 63bdb59a0d4e8d29c088c565459ecc5fedc34346ea2e48cf5423d59a6b79bbe5
  - label: AC14
    digest: 8961b44a36dd1fb1df071234685d657569c9eefa1a92303428b5d4f5d517bb77
  - label: AC15
    digest: f61e04505881a9046c1acf59e0b1cb60a126ea1141feabe0936aebbf89d61abe
  - label: AC16
    digest: 0b0ff7290bb91e475013528a1466b38338bad697dccdce0d0562ae8c6f2ff1aa
  - label: AC17
    digest: 46afb81d8fde3d6bf005b1f053faddf601550a178b2b98c016507eb1ff137b10
  - label: AC18
    digest: 9f1f256902f467a532ab423063e61b9141cf470c95f4c1b762298310f3b75c5a
  - label: AC19
    digest: 10d280e11602ee7671161751d3b2003b43199464a4a2d6d946e7f79923ede3bc
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
decisions:
  - key: "convention: the Layout-derived row label `Command/skill path` names the skill layout the dropped hosts used — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: renaming it is a sweep across ductus.md, the analyze reference and done specs 000, 067 and 071 for a name no step misreads; 048's and 063's reviews waived it too
    decided-at: 2026-11-18T20:46:33Z
    decided-by: rule
  - key: "other: the MCP registration `mechanism` column holds one value since b1d319f1 — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: it is the registry's per-agent field that the Adding a new agent steps fill, and no step reads it wrong
    decided-at: 2026-11-18T20:46:33Z
    decided-by: rule
  - key: "security: the runtime binary is authenticated only by a same-origin sidecar digest, and the framework archive by none — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: 048's and 061's stated trade-off, recorded in the file
    decided-at: 2026-11-18T20:46:33Z
    decided-by: rule
  - key: "security: a committed `[runtime] path` runs the binary it names — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "048's design: the project supplies its own binary, with the trust a repository hook already has"
    decided-at: 2026-11-18T20:46:33Z
    decided-by: rule
  - key: "convention: AGENTS.md's Tech Stack omits the Python scripts and the Node harness, and Project Structure omits ductus-procedure.md, hooks/ and pi/ — `AGENTS.md`"
    outcome: waived
    reason: the pre-commit hook refuses a commit that grows AGENTS.md past its 50,000-byte one-read limit, and it stands at 49,979 bytes, so adding them needs a cut that is the owner's to choose
    decided-at: 2026-11-18T20:46:33Z
    decided-by: rule
  - key: "convention: task 12's unexercised Source resolution step 6 and archive abort message are named only in its closing commit — `specs/012-multi-agent-govern/tasks.md`"
    outcome: waived
    reason: task 12's own text (owner, 2026-11-11) asked the closing commit to name what no run could reach
    decided-at: 2026-11-18T20:46:33Z
    decided-by: rule
---

# Review — 012-multi-agent-govern

## Summary

Review of 012's code changed since its last review, from `067d94f6` (`last-review`) to `c9257d47`, for 084's task 19. The five passes read the spec's own change, the hunks of `598fe9ca` (`chore(048, 070, 068, 012)`) in `runtime/CHANGELOG.md`, against the five loaded rule files, each read whole (231,962 bytes): no finding and no observation. The criteria step judged 18 of 19 criteria met against the tree and AC18 not met: `framework/bootstrap/ductus.md` §Permission Setup's step 2 has removed exact-match retired seed entries from `permissions.allow`, an adopter's identical entry included, since `df474a1f` (022's task 18), which the criterion's text did not say. The owner decided the code is correct (2026-11-25), so AC18's text was corrected to state that exception, its tick and label kept, and it is recorded judged met. The criteria part's observation on the same gap is no observation, since the corrected criterion settles it. Stored decisions: all 6 retained, on files this scope does not hold. Examined 1 of 12: `specs/012-multi-agent-govern/spec.md`, read whole by the criteria step. Unread, by the operator's decision to read each spec's own change (084's `tasks.md`, task 19): `runtime/CHANGELOG.md`, read at the slice's hunks and the entries they touch; `framework/bootstrap/govern.md`, compared byte for byte with `framework/bootstrap/ductus.md` and found identical, not read; and the entries an older record carried in, not opened: `.claude/commands/ductus/consolidate.md`, `.claude/commands/ductus/fold.md`, `.pi/prompts/ductus-consolidate.md`, `.pi/prompts/ductus-fold.md`, `.github/workflows/runtime-release.yml`, `.github/workflows/runtime.yml`, `runtime/src/schema/primitives.rs`, `specs/012-multi-agent-govern/analysis.md` and `plan.md`.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

10 of 738 commits in the window no spec claimed.

19 of 19 acceptance criteria judged met.

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

- AC1 — met: `framework/bootstrap/ductus.md` is the only installed bootstrap (`framework/bootstrap/` holds no per-CLI file); `framework/bootstrap/govern.md` is byte-identical (`cmp` rc 0, both 166938 bytes) and `bash scripts/audit/transitional-bootstrap-parity.sh` (Family 21) exits 0; `framework/bootstrap/ductus-procedure.md` is the archive half of the same procedure, not a second bootstrap.
- AC2 — met: `framework/bootstrap/ductus.md` §Agent Selection rule 2 scaffolds every registry agent whose `config_dir` exists, silently, when neither `--agents=` nor `--add-agent` is present; a procedure read by the agent, no test drives it.
- AC3 — met: `framework/bootstrap/ductus.md` §Agent Selection rule 3 prompts one yes/no `AskUserQuestion` per registry agent on `--add-agent`, pre-selecting Yes where `config_dir` exists; procedure text only, no test drives it.
- AC4 — met: `framework/bootstrap/ductus.md` §Agent Selection rule 3 prompts when no agent dir is detected and pre-selects the agent whose `config_dir` holds the running `ductus.md` (`.claude/commands/ductus.md`, `.pi/prompts/ductus.md`); procedure text only, no test drives it.
- AC5 — met: `framework/bootstrap/ductus.md` §Agent Selection rule 1 scaffolds exactly the `--agents=` keys with no prompt, and §Inputs lists the flag; procedure text only, no test drives it.
- AC6 — met: `framework/bootstrap/ductus.md` §Agent Selection rule 3 rejects a zero-agent confirmation with "At least one agent must be selected." and scaffolds nothing; procedure text only, no test drives it.
- AC7 — met: The four surviving legs hold in `framework/bootstrap/ductus.md` §Per-Agent Scaffolding (commands at `{config_dir}/commands/{project}/`, `configure/{key}.md` written as `configure.md`, native settings file per §Permission Setup, verbatim `ductus` self-install at the derived install path, `pi` via the Pi layout section); the session leg is replaced by `.ductus/session.toml` via `write-session` (§Instructions step 7, §Session state) exactly as the criterion's own annotation says; `bash scripts/tests/test-install.sh` passes.
- AC8 — met: `framework/bootstrap/ductus.md` Agent Registry: `claude` `settings_template` is `{ "permissions": { "allow": [...] } }` in `{config_dir}/settings.local.json`, and the `pi` row's `settings_template` is `{}` with Permission Setup step 1 skipped for `pi`; `bash scripts/audit/installer-registry-parity.sh` exits 0.
- AC9 — met: `framework/bootstrap/ductus.md` §Re-Run Behavior, "Re-run adding a new agent", and §Per-Agent Scaffolding, one `apply-manifest` call per selected agent with its own `cli-config-dir` (§Instructions step 4d), leave an unselected existing agent's tree untouched; the session-JSON leg is moot as the criterion says; procedure text, no test drives the add-agent run.
- AC10 — met: `framework/bootstrap/ductus.md` §Per-Agent Scaffolding marks the slash commands and `configure.md` strategy `update` and §Shared Files marks `specs/system.md`, `errors.md`, `events.md`, `inbox.md`, `.githooks/pre-commit` and `pre-push` strategy `create`, so a same-selection re-run rewrites the former and skips the latter (§Re-Run Behavior); procedure text, `apply-manifest` strategies are the mechanism.
- AC11 — met: `framework/bootstrap/ductus.md` §Shared Files is applied once per run (§Instructions step 4a) independent of agent selection, and holds every file the criterion lists (constitution, `specs/templates/*`, `.markdownlint-cli2.jsonc`, `AGENTS.md`, `CLAUDE.md` claude-style only, `.gitignore`, `specs/system.md`, `errors.md`, `events.md`, `inbox.md`); the criterion itself defers the canonical list to that section; `bash scripts/audit/manifest-parity.sh` exits 0.
- AC12 — met: `framework/bootstrap/ductus.md` §Adding a new agent states a new agent is a registry row, an MCP registration row, `framework/bootstrap/configure/{key}.md` and an `install.sh` arm with a README row, creating no top-level ductus file; `bash scripts/audit/installer-registry-parity.sh` exits 0 holding registry and `install.sh` (`claude)`, `pi)` arms) in step; the criterion's three-value list is wider now (MCP row, installer arm) but the no-new-top-level-file claim holds.
- AC13 — met: `specs/012-multi-agent-govern/spec.md` §Migration from 007's Multi-File Model documents the one-time curl, `/ductus` run and permission command and records the migration as spent; the later re-run path is `framework/bootstrap/ductus.md` §Self-update check; the curl is now superseded by `install.sh` (003), which `bash scripts/tests/test-install.sh` passes.
- AC14 — met: `README.md` §Installing (per agent) has the per-agent table (Claude `claude`/omitted to `.claude/commands/ductus.md`, Pi `pi` to `.pi/prompts/ductus.md`) and states "re-run `/ductus --add-agent` from either later to add the other", so no second install is needed; the wording is implicit rather than the literal phrase "second curl", since `install.sh` replaced the curl snippets.
- AC15 — met: `specs/007-govern-workflow/spec.md` opens with the blockquote "Superseded in part by [012-multi-agent-govern]" naming the single registry-driven installer, and its AC1, AC3 and AC14 are annotated to match.
- AC16 — met: No "Ductus File Parity" or parity rule remains in `CLAUDE.md`, `AGENTS.md` or `framework/templates/project/claude-md.md` (grep for `Parity`/`parity` returns only unrelated `AGENTS.md` lines about audit scripts).
- AC17 — met: `framework/bootstrap/ductus.md` §Agent Selection rule 1 stops before any scaffolding with "Unknown agent key: `{key}`. Valid keys: ..." and "Do not partially scaffold", and §Inputs says reject unknown keys; procedure text only, no test drives it.
- AC18 — met: `framework/bootstrap/ductus.md` §Permission Setup: step 3 adds missing `settings_template` entries without overwriting others, and step 2 removes only exact-match retired seed entries from `permissions.allow`, the exception the criterion's text now states (corrected, the owner deciding the code is correct, 2026-11-25); `scripts/audit/installer-registry-parity.sh` exits 0
- AC19 — met: `npx markdownlint-cli2` run in the review worktree at `394740b3` linted 920 files: "Summary: 0 issues in 0 files" (exit 0).

## Observations

*None.*

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md`
- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/clarify.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/help.md`
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
- `.pi/extensions/ductus.ts`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-clarify.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-help.md`
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
- `.ductus/docs/agents/developing-ductus.md`
- `docs/rules.md`
- `docs/runtime.md`
- `docs/shared-constitution.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus-procedure.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/hooks/ductus-pre-push`
- `framework/bootstrap/hooks/pre-push`
- `framework/bootstrap/pi/ductus-bridge.ts`
- `framework/commands/amend.md`
- `framework/commands/analyze.md`
- `framework/commands/clarify.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/groom.md`
- `framework/commands/help.md`
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
- `runtime/tests/fixtures/implement-basic/.govern.session.toml`
- `runtime/tests/fixtures/implement-basic/stdin.jsonl`
- `runtime/tests/fixtures/review-basic/stdin.jsonl`
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
- `scripts/audit/migration-coverage.sh`
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
- `scripts/mutation/fix-round-record.py`
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
- `scripts/tests/test-migration-coverage.sh`
- `scripts/tests/test-pre-commit-staging.sh`
- `scripts/tests/test-run-workflow-steps.sh`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/000-slash-commands/analysis.md`
- `specs/000-slash-commands/plan.md`
- `specs/000-slash-commands/review.md`
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
- `specs/000-slash-commands/scenarios/scenario-without-task-visibility.md`
- `specs/000-slash-commands/scenarios/validation-gates.md`
- `specs/000-slash-commands/spec.md`
- `specs/003-bootstrap-automation/analysis.md`
- `specs/003-bootstrap-automation/plan.md`
- `specs/003-bootstrap-automation/spec.md`
- `specs/007-govern-workflow/analysis.md`
- `specs/007-govern-workflow/plan.md`
- `specs/007-govern-workflow/review.md`
- `specs/007-govern-workflow/scenarios/ductus-self-update-precheck.md`
- `specs/007-govern-workflow/spec.md`
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
- `specs/013-text-first-artifacts/analysis.md`
- `specs/013-text-first-artifacts/plan.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/013-text-first-artifacts/spec.md`
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
- `specs/026-framework-self-audit/scenarios/family-10-migration-coverage.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/027-bootstrap-migration-registry/analysis.md`
- `specs/027-bootstrap-migration-registry/plan.md`
- `specs/027-bootstrap-migration-registry/review.md`
- `specs/027-bootstrap-migration-registry/scenarios/a-first-adoption-runs-no-migration.md`
- `specs/027-bootstrap-migration-registry/scenarios/a-projects-own-constitution-is-not-relocated.md`
- `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md`
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md`
- `specs/027-bootstrap-migration-registry/scenarios/seed-files-keep-no-literal-placeholder.md`
- `specs/027-bootstrap-migration-registry/spec.md`
- `specs/027-bootstrap-migration-registry/tasks.md`
- `specs/029-bootstrap-runtime-autowire/analysis.md`
- `specs/029-bootstrap-runtime-autowire/plan.md`
- `specs/029-bootstrap-runtime-autowire/review.md`
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md`
- `specs/029-bootstrap-runtime-autowire/spec.md`
- `specs/029-bootstrap-runtime-autowire/tasks.md`
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
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md`
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
- `specs/056-bootstrap-archive-boundary-split/analysis.md`
- `specs/056-bootstrap-archive-boundary-split/plan.md`
- `specs/056-bootstrap-archive-boundary-split/review.md`
- `specs/056-bootstrap-archive-boundary-split/spec.md`
- `specs/056-bootstrap-archive-boundary-split/tasks.md`
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
