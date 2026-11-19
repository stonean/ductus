---
spec: 000-slash-commands
last-run: 2026-11-19T02:10:10Z
reviewed-against: 37636dd2b0758da0c9dfd91f12f69a5de845c2be
diff-base: e0de7982d041ee342057a5137f36dbeb7045f655
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 4
unread: []
excluded: 64
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/a-done-stop-says-the-request-is-not-built.md: a578e1a0f2c9dfbaee173a31734aa64e2d42afd5d10f41df72fd9c47cd9b3402
  scenarios/analyze-reads-the-layout-derived-command-set.md: 134abf405d47afc5d4b055bd91105293db6ba2b434a82d0c1ea2a908a1f2e4d7
  scenarios/clarify-one-at-a-time.md: 74531377a3ba6862619c2a4aaf861084da5b53d69ac09a39f28740176f5c078d
  scenarios/command-autocomplete-summary.md: de4ef40b8d2a8587be508d333d502198cae4c312639b282ce363d8bb7253fa66
  scenarios/criterion-route-after-draft.md: 7fa00d35986c315ec41a85794276370181240fd34f822374a03cf816cacd7063
  scenarios/dashboard-dependencies-column.md: d1ce2860d34e9be0e4f34bb67651645e92b709bca1a1572833259923b9da0428
  scenarios/implement-skips-planned-prompt.md: adeb014c4192e84543d733fcf27f12907c2cada0d4950af9782a6168a6fac1c3
  scenarios/implement-stops-below-planned.md: 4e74cbbf5c625ee2b7415a3bc682f996d1b8356b2d25cb3203e8dcf08d9e0f81
  scenarios/scenario-without-task-visibility.md: a28dc9524faff8ff2ebca2c58c3ef125536694b9fe24efd88555703d13d81b34
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
  fixed: 3
  deferred: 0
  waived: 1
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
  - key: rule QUAL-CLAIM-001 at runtime/src/primitives/check_artifacts.rs
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/tasks.md
    task: "31"
    reason: "the owner deferred it to 067's task 31 on 2026-11-14: check-artifacts is 067's primitive, and whether an unreadable plan.md is a skip or a blocking finding is that task's call"
    decided-at: 2026-11-14T03:18:40Z
    decided-by: andy@stone.dev
  - key: other — criterion-route-after-draft's second Resolved Question quotes §spec-lifecycle's third back-edge and its exemption (c) in wording the constitution no longer holds — `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
    outcome: waived
    reason: A decision record resolved 2026-08-17 quoting the constitution as it stood then, and a sweep must not rewrite the record of the thing being swept (AGENTS.md Gotchas); its conclusion still holds, since framework/constitution.md §Spec lifecycle's 'Backward via meaningful body edit' reopens a done spec on an edit that changes a criterion.
    decided-at: 2026-11-19T02:10:10Z
    decided-by: rule
---

# Review — 000-slash-commands

## Summary

Window e0de7982 (base-source reopen, 066 task 27e) to 37636dd2. The scope holds four artifacts, each read whole by the passes (spec.md 11,539 bytes; scenarios criterion-route-after-draft 6,223, validation-gates 2,567, dashboard-dependencies-column 3,918), so 4 of 4 examined; dashboard-dependencies-column.md is in scope through the last record's reviewed-digest (4d681fad), which predates the one sentence 97d854b5 added. The nine backend and cross rule files were read whole (285,795 bytes) and all five passes ran: no finding. The criteria step judged all 15 criteria one by one against the tree and history: AC10 and AC12 were first judged not met, their annotations having been made false by later work (prune --project and consolidate write a README; 062 added per-process session targets), and were corrected by owner decision and judged met on their new text. Three observations were fixed (stale claims 066 task 27e's sweep left in two scenarios) and one waived by rule. Nine stored decisions are retained, each keyed to a file outside the scope. Unexamined: the scenarios' point-in-time Context claims, and dashboard's specs[] bound, read from its description only.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- AC1 — met: Annotation holds: git ls-tree 2c838b20 shows 13 files under commands/ (about, clarify, create, implement, plan, question, scenario, setup, specify, status, target, triage, validate = nine of the ten named plus create, question, scenario, triage); `git log --all -- '*next.md'` is empty; framework/commands/ plus the Slash commands manifest in framework/bootstrap/ductus.md exist (16 rows plus configure; maintainer-only audit.md is withheld by design).
- AC2 — met: scripts/gen-claude-commands.sh substitutes exactly {project} and {cli-config-dir} (sed lines); framework/commands/*.md carry only {project} as an installer placeholder ({cli-config-dir} lives in framework/bootstrap/configure/claude.md); 012 added the second; 000 spec.md Parameterization records both.
- AC3 — met: specify creates at status draft (framework/templates/spec/spec.md); clarify.md Gate table draft->clarified via set-status; plan.md requires clarified, set-status clarified->planned; implement.md step 4 stops on draft/clarified/done, planned->in-progress via set-status, in-progress->done only after check-review-gate and approval; all match constitution Spec lifecycle.
- AC4 — met: framework/commands/help.md Scope Boundaries: prints a fixed guide, reads no spec files, lists no directories, invokes no primitive, command calls no runtime; tables are pre-generated by scripts/gen-help-tables.sh. (Typed help on Claude Code/Pi is answered by the runtime's render-help, which reads the installed help copy; that is spec 080's no-model-turn path, not the command.)
- AC5 — met: framework/commands/target.md step 8 invokes write-session (target write), step 9 displays feature status, artifacts, deps and next step; driven write-session on a scratch fixture and it wrote .ductus/session.toml and .ductus/sessions/abc123.toml.
- AC6 — met: framework/commands/status.md step 1 invokes render-status; ran runtime/target/release/ductus render-status: rendered-markdown holds a Feature/Status/Plan/Tasks/Data-model/Scenarios/Dependencies/Next Action table of every spec dir.
- AC7 — met: Annotation holds: no next command in framework/commands/ and `git log --all -- '*next.md'` is empty; status.md Status -> next action table and render-status Next Action column, and target.md step 9 next pipeline step, do the job.
- AC8 — met: analyze.md steps 1-17 cover them: validate-frontmatter (step 2), traverse-deps and derive-dependencies (3, 6), check-artifacts (8: spec integrity, artifact completeness, plan consistency, task consistency per its description), resolve-references and check-orphaned-references (9, 13), resolve-anchor (4).
- AC9 — met: framework/bootstrap/configure/claude.md canonical allow set: Bash(git add *), Bash(git status*), Bash(npx markdownlint-cli2 *), Bash(ls *), Edit/Write, mcp__ductus__* reads, additionalDirectories for the spec root; shell file-content parsers (cat/head/grep/awk/for) are intentionally absent since fa78f79c, so the file-reads clause is covered only by ls, MCP read tools and additionalDirectories (weak clause).
- AC10 — met: 9847647b (feat(003), 2026-06-10) removed specify's README sentence with scripts/gen-readme-table.sh; framework/commands/specify.md step 5 invokes create-feature, which a fixture run numbered 004-hello-world after 000 and 003; neither specify.md nor runtime/src/primitives/create_feature.rs names a README (grep), the clause corrected from 'no command source updates a README', which prune --project (ab5eaa49) and consolidate (3b359742) made false (owner decision)
- AC11 — met: runtime/src/primitives/mod.rs template_candidates resolves {specs-root}/templates/{file} first, framework/templates/spec/{file} only as the ductus repo's own layout; create-feature on a fixture returned template specs/templates/spec.md; specify.md step 5 and create-plan-artifacts use resolve_template. Spec root is configurable.
- AC12 — met: 2c838b20's commands name {cli-config-dir}/{project}-session.json, installed as .claude/...; 4c5c4aea (022.40, 2026-05-24) replaced it with the root .govern.session.toml; 000's Session State section states today's files (062's .ductus/sessions/ beside the shared .ductus/session.toml); the clause corrected from 'session state is .ductus/session.toml for every adopter' (owner decisions)
- AC13 — met: analyze.md step 7 invokes lint-markdown; runtime/src/primitives/lint_markdown.rs resolve_markdownlint prefers repo-local node_modules/.bin/markdownlint-cli2 else `npx markdownlint-cli2`; cargo test --release lint_markdown: 14 passed, 0 failed.
- AC14 — met: Annotation holds: framework/templates/spec/ has no spec-and-plan.md (deleted in dd8f7b41, 2026-05-16, 023 AC1); no framework/commands/*.md mentions qualifying questions or lightweight track; 023 spec AC1 records the deletion.
- AC15 — met: Annotation holds: grep finds no spec-and-plan in framework/commands/ (023 AC10); runtime/src/primitives/mod.rs is_spec_path still matches "spec.md" | "spec-and-plan.md" (read tier, also in the spec-file walk).

## Observations

- convention — criterion-route-after-draft's second Resolved Question says in the present tense that nothing detects an unchecked criterion on a `done` spec and that `/{project}:implement`'s criterion-verification step verifies criteria, both reversed by 066's `criteria-are-verified-in-review` — `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — **fixed** (decided by the rule)
- convention — validation-gates' Edge Cases say an already-ticked criterion is left as-is and not re-verified, which 066's `criteria-are-verified-in-review` reversed: `/{project}:review`'s criteria step judges every criterion on each run — `specs/000-slash-commands/scenarios/validation-gates.md` — **fixed** (decided by the rule)
- convention — criterion-route-after-draft's Context says in the present tense that no command writes a criterion after `/{project}:clarify` and that `/{project}:amend` has two routes, though amend has carried the criterion route this scenario specifies since its delivery — `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — **fixed** (decided by the rule)
- other — criterion-route-after-draft's second Resolved Question quotes §spec-lifecycle's third back-edge and its exemption (c) in wording the constitution no longer holds — `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — **waived**: A decision record resolved 2026-08-17 quoting the constitution as it stood then, and a sweep must not rewrite the record of the thing being swept (AGENTS.md Gotchas); its conclusion still holds, since framework/constitution.md §Spec lifecycle's 'Backward via meaningful body edit' reopens a done spec on an edit that changes a criterion. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/audit.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `.ductus/reference/review.md` — 066-audit-records-and-the-completion-gate
- `.github/workflows/runtime-acquisition.yml` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime-release.yml` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime.yml` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `.pi/prompts/ductus-amend.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `docs/developing-ductus.md` — 066-audit-records-and-the-completion-gate
- `docs/slash-commands.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/bootstrap/ductus-procedure.md` — 027-bootstrap-migration-registry
- `framework/bootstrap/ductus.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `framework/bootstrap/govern.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `framework/commands/amend.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/constitution.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/migrations.toml` — 027-bootstrap-migration-registry
- `framework/reference/audit.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/reference/review.md` — 066-audit-records-and-the-completion-gate
- `runtime/CHANGELOG.md` — 066-audit-records-and-the-completion-gate, 073-decided-rule-departures-are-recorded
- `runtime/src/interpreter/payload.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/check_review_gate.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/compute_review_scope.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/line_citations.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/schema/descriptions.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/schema/primitives.rs` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/review-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `scripts/audit/README.md` — 066-audit-records-and-the-completion-gate
- `scripts/audit/done-spec-criteria.sh` — 066-audit-records-and-the-completion-gate
- `scripts/audit/migration-coverage.sh` — 027-bootstrap-migration-registry
- `scripts/mutation/criteria-in-review-066.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/line-citation-strip-066.py` — 066-audit-records-and-the-completion-gate
- `scripts/tests/test-migration-coverage.sh` — 027-bootstrap-migration-registry
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/027-bootstrap-migration-registry/plan.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/review.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/a-projects-own-constitution-is-not-relocated.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/spec.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/048-govern-acquired-runtime/data-model.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/analysis.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/review.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/073-decided-rule-departures-are-recorded/plan.md` — 073-decided-rule-departures-are-recorded

## Unexamined governance

*None.*
