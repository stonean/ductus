---
spec: 023-govern-refinement
last-run: 2026-11-22T04:14:13Z
reviewed-against: c9657c8892d11716a3f25d6e284ec2e079a37078
diff-base: d77a904dc8ad4e09cbefffd6f07854e4b9d44c41
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 12
scope: 18
unread:
  - framework/bootstrap/govern.md
  - runtime/CHANGELOG.md
  - framework/reference/audit.md
  - runtime/src/primitives/merge_permissions.rs
  - specs/013-text-first-artifacts/plan.md
  - specs/016-cross-cutting-rules/spec.md
excluded: 3
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/configure-dedup-permissions.md: bf3690f13886de1650ad52685c636082dbb34e89028f6f81a674e6206d843a41
  scenarios/configure-inert-write-path-entries.md: 6548c68140f0b3363c38b9ae228b264aa85041aa0fe1537437b735bf8320af65
  scenarios/configure-permission-pattern-safety.md: 23794a3651ae508412681c6754aa282157e15322e972ca3a881a6879d7678738
  scenarios/configure-retires-formerly-canonical-entries.md: 0edf0a2eeed43837628c59de7c539e2ce1119fd9ab088066baf5b46a8d2eccca
  scenarios/extend-existing-scenario-task.md: 70df3af27fc39ca140538dbe470a13112dc0ca007913b4f07bb2baf9dbb617e6
  scenarios/living-specs.md: 0331b227b72b54edbcb383f8f27c994505d62ab792c6a1d58272c210297d7feb
criteria-judged: 38
criteria-met: 38
criteria-verdicts:
  - label: AC1
    digest: 730fe028a9ef3c4205a22daefcd556a51d8ce34b28dc5c4d9c47fe8b7e82bd77
  - label: AC2
    digest: 6e981a249e6b7d567712084a6504019f55c88a6e9514b6024cf9fb3f00324e09
  - label: AC3
    digest: 2ec13cfcfe28f5d29250de1dd8f977565996d79d46d86a5aaa429b6ea76590f9
  - label: AC4
    digest: dd7f1ba3c96131e18bc9b846440462c2a4cff72bf5b2e2f31ee6b8d5c7f01132
  - label: AC5
    digest: 83b45f2e9e5b467d88b07eeae55aee38864561c606b1eace94cd2f8e42e9b8bf
  - label: AC6
    digest: ac812f9671cc3e5276ce3ff09e2ae479f6c0887dae0f34f7df57eb72a17be568
  - label: AC7
    digest: 33c9b9ac27984ee8fee29bf1c2702421e774194f84bae9f0f098057d969e6cc9
  - label: AC8
    digest: 40510d7948407b4a98e7fb6a037948a0c19a5c370a24e4cfcbdd02792ad2ba03
  - label: AC9
    digest: 31e325841b139e6fd4f9e3fba0985368549083301e5ca6c52ecb3ad5bda413be
  - label: AC10
    digest: ef075cadaa9e9d55a99fe209d9508a9976a27fede225de5fe922fef9eab9ab19
  - label: AC11
    digest: 70c3b78829333e5a10d1e82a49c9c406df7780e6bb78f2ec12ea69f199d3d0b3
  - label: AC12
    digest: b3c48490078177b2256c03298b0ceb165e8ba2168b67588ca7c39f571ff15835
  - label: AC14
    digest: db4c7836edb0d2c2528c8ab6d5dc686aaa5fdac21e7d6f67a0b9ba409ab695d4
  - label: AC16
    digest: cce0c7c5ec0fdf481b5e8316729faac1a3259076d1ea2fcc1fc4eb1d5075e13a
  - label: AC17
    digest: 944cc8bbc731639c3550e7f36556241a5e77afd1dd222ffcf0d4c133feb6b4cd
  - label: AC18
    digest: 99e540ff9a828246e00201c8ca22cb57cd765b711ab50d9c410d18cce21a977e
  - label: AC19
    digest: af4f89c66dd2ddbe7c999e2f8e3a7464283e61e504a98bf6607b35a82b572474
  - label: AC20
    digest: 0b2f2b7ea07342447448429d3467155d9e798e4c443560944d6ceca6a3a38fe8
  - label: AC21
    digest: 55eaba6157399830658562fd032732d4513e2bccd3ee14e9cc50278fd5921cff
  - label: AC22
    digest: dd70dd74f8e8cfa19c371022bc19fcf06941c71305282570b699c36f3c9eae3f
  - label: AC23
    digest: 35131a5a74dc8305042e2840d1fdca28bc2e71f387efba31481b3665fd775de0
  - label: AC24
    digest: 0dd76e55cd321b78300568cdc5f0b51cbd1fefa9aadd77106ad2f4603123be02
  - label: AC25
    digest: 4193bee69a5bac0bbe769ba40afd422dc3f42ccf7d88e1b56ae100c76d0dfc25
  - label: AC26
    digest: 316baa8bbea5e024a54ca1dc385dbfdcf212792d0afb7582e12269d4504a5cbb
  - label: AC27
    digest: d01b5f36f0d8828d9dcee41f069a437918c00b90ed2637cdb1350a70249cc7a9
  - label: AC28
    digest: fb55454067188bd2dd9016bac2cc8dfbab36481524caceff8abb0838269f0c8d
  - label: AC29
    digest: 3e6ce4157037df75ce262dbf807e0842f18d744ce5e4b1d3487a80933f82240b
  - label: AC30
    digest: 6d20ff7e2a024eea23fe8b55d6c6067289c46ed53a2afd027220064df28882c0
  - label: AC31
    digest: 1614010093e014c0cadb591025f76e34e4eec49519f5d3cbd17d585678c1f128
  - label: AC32
    digest: 3460b359afec595cfbbde1ed81b3c89e59178708b934db6879793191ede4f7c5
  - label: AC33
    digest: 8358b3fc33c8668e00b3b0fdbc5419cead52c53fbba1025acdd002e991b39262
  - label: AC34
    digest: efc83fec5f0d51b252dbb5c0d23ec311956dbe2da99ead620b404626e81a1bd6
  - label: AC35
    digest: 3bf0100f5b4df8a883a0ca85e102ad7e2bc6a423fa12ed383556e2da8f1b7e01
  - label: AC36
    digest: a9334b4a09d33d1ec7ce2b6db86f9899b3dc21cdad28484fac82d6eb440f60ab
  - label: AC37
    digest: e5ebbd8f4336f4bd50fcd7415ab00932f2be68e86f4fa09f70a7c1bb5b999cec
  - label: AC38
    digest: 70ee0adba2e6eb0779d086e84e38fca9cb106a758fa0646687b1c57569009050
  - label: AC39
    digest: 5ad23f16411257eaedf39dc9d972217cbb7fffe41243446225ec57b16c4f5e78
  - label: AC40
    digest: abe7584a01fa6eb1a064e1b3d6ac8eadc33d1a16d9f15b4765d059038013e131
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 7
  undispositioned: 0
decisions:
  - key: "convention: Adding or removing a runtime primitive names six sites but neither /configure step 4 retired list, and no check catches a removed tool missing from them — `docs/developing-ductus.md`"
    outcome: waived
    reason: "the pipeline's own maintainer procedure: a doc line would rest the next removal on the remover's memory, and a check deriving retired MCP entries from history is a new audit family, not a mechanical fix; this run retired the three entries the gap left"
    decided-at: 2026-11-07T23:21:03Z
    decided-by: rule
  - key: "convention: 016's AC8 names its spec, template and validate changes by the command's old name — `specs/016-cross-cutting-rules/spec.md`"
    outcome: waived
    reason: the phrase records the changes 016 made under the command's name at the time, outside AC21's four-form claim, and editing the criterion would reopen 016 for one token
    decided-at: 2026-11-22T04:14:13Z
    decided-by: rule
  - key: "convention: the Context sections of the dedup, inert-write, retires and pattern-safety scenarios state their problem in the present tense as it stood when each was written — `specs/023-govern-refinement/scenarios/configure-dedup-permissions.md`"
    outcome: waived
    reason: a scenario's Context is the problem it was written against; each Behavior, its superseded annotations and the criteria state the current shape
    decided-at: 2026-11-22T04:14:13Z
    decided-by: rule
  - key: "convention: 048's AC22 annotation says the rest of the acquisition sequence prompts for nothing on Claude, which a reader may take to cover the Windows [runtime] path digest — `specs/048-govern-acquired-runtime/spec.md`"
    outcome: waived
    reason: 048's own review reads its spec in this window, and the claim concerns the acquisition sequence, which no Windows host runs since 048's AC25
    decided-at: 2026-11-22T04:14:13Z
    decided-by: rule
  - key: "convention: Permission Setup's closing rationale against a prompt on every bootstrap stands beside the Windows configured binary's per-run digest prompt — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the agent's recommendation, adopted by the operator, 2026-11-21: that prompt over a trailing wildcard or no entry, and the same paragraph states it
    decided-at: 2026-11-22T04:14:13Z
    decided-by: rule
  - key: "other: the pinned certutil -hashfile .ductus/bin/ductus.exe SHA256, a Git Bash relative path, is unmeasured on Windows, where the measurement harness passes a cygpath -w path — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the procedure passed Git Bash paths to certutil before this change too, and no Windows runner exists since 048's AC25 (owner, 2026-11-19)
    decided-at: 2026-11-22T04:14:13Z
    decided-by: rule
  - key: "bug: mid_wildcard does not flag a pattern whose first token is a wildcard, such as Bash(* ls) — `scripts/audit/permission-wildcard-position.sh`"
    outcome: waived
    reason: the predicate predates this change and serves both subjects, and no allow entry either set ships has that shape
    decided-at: 2026-11-22T04:14:13Z
    decided-by: rule
  - key: "other: Family 29's seed parse is a third copy of the Agent Registry row read that Families 32c and 14 also carry — `scripts/audit/permission-wildcard-position.sh`"
    outcome: waived
    reason: each family is a standalone script by the audit contract and lib.sh holds bash helpers only, so a shared reader is a new surface rather than a mechanical fix; a broken parse reports findings, never a pass
    decided-at: 2026-11-22T04:14:13Z
    decided-by: rule
---

# Review — 023-govern-refinement

## Summary

Five passes over the 14 scope entries the window held at the start (diff base d77a904d, the reopen; 048's three files excluded as its own) against the five rule files review loads, each read whole: 12 read whole, framework/bootstrap/govern.md established byte-identical to ductus.md with cmp rather than read, and runtime/CHANGELOG.md (623961 bytes) read in part, its [Unreleased] section and the commit's hunk. The record's scope of 18 adds the four files this run's fixes touched (framework/reference/audit.md, runtime/src/primitives/merge_permissions.rs, specs/013-text-first-artifacts/plan.md, specs/016-cross-cutting-rules/spec.md), which the passes did not read and the fix round's independent re-read read, clean; all six are listed unread. No rule violation. All 38 criteria judged one by one: AC11's and AC37's annotations were stale against the tree (027's review archived spec-and-plan-sunset; merge-permissions' path checks and canonical config read now precede the overlap check), so both were corrected in this run and judged met against the corrected text. Five observations fixed in the run, seven waived by the rule.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

38 of 38 acceptance criteria judged met.

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

- AC1 — met: framework/templates/spec/ lists data-model, plan, research, scenario, spec, tasks; no spec-and-plan.md.
- AC2 — met: grep -i 'lightweight|spec-and-plan' framework/constitution.md returns nothing.
- AC3 — met: grep for /capture, /elaborate, capture.md, elaborate.md in framework/constitution.md returns nothing.
- AC4 — met: framework/commands/ and .claude/commands/ductus/ hold no capture.md.
- AC5 — met: framework/commands/ and .claude/commands/ductus/ hold no elaborate.md.
- AC6 — met: framework/commands/specify.md has no qualifying-question step; step 5 create-feature copies the spec template (spec.md); Purpose says richness scales with input.
- AC7 — met: framework/commands/amend.md Classify the input, Refine (scenario route) walks bug decision tree, Record: create-scenario then append-task.
- AC8 — met: amend.md Classify the input lists question signals, scenario signals and Status tiebreaker under ## Instructions; Approval gate prints the route and accepts flip, which redrafts under the other route.
- AC9 — met: amend.md Purpose and Record the input: question route append-question performs clarified+ -> draft; scenario route set-status done -> in-progress.
- AC10 — met: grep spec-and-plan in framework/commands/ finds nothing (read or write side).
- AC11 — met: met once corrected in this run: the check has not fired since 0.10.0, and c061c1f0 (027's review, 2026-11-17) archived spec-and-plan-sunset: absent from framework/migrations.toml, recipe at CHANGELOG.md ### spec-and-plan-sunset
- AC12 — met: claude.md step 2 holds Edit(.ductus/session.toml) and Edit(.ductus/config.toml); Write(path) entries appear only in step 4's retired list (AC34); claim of AC34 supersession and path move are true.
- AC14 — met: 73 tools in framework/runtime-tools.txt equal the 73 mcp__ductus__ entries in claude.md (python set diff empty); entries unconditional.
- AC16 — met: scripts/gen-configure-mcp.sh --dry-run exits 0 'in sync'; it is run by .githooks/pre-commit and generators.yml, and tested in framework-checks.yml.
- AC17 — met: specs/022-deterministic-runtime/scenarios/ask-consolidation.md exists introducing create-scenario and append-task; spec 022 status is done.
- AC18 — met: runtime/target/release/ductus create-scenario --help and append-task --help work as CLI; both names in framework/runtime-tools.txt (MCP tools, now under the ductus namespace).
- AC19 — met: amend.md scenario route calls create-scenario and append-task; no host-side prose fallback remains (retirement claim consistent: no fallback text in file).
- AC20 — met: framework/commands/validate.md is absent and framework/commands/analyze.md exists with H1 '# Analyze'.
- AC21 — met: Token search (/validate, /ductus:validate, /{project}:validate, validate.md) over framework, scripts, .github, docs, README.md, AGENTS.md, specs, runtime finds only the subject uses (introducing-drift.sh catalog, 023's own spec/plan/scenario prose, analysis.md quotes); the recorded bare-prose sweep is history that holds, though three bare-prose residues survive in older specs (016 spec lines on 'validate plumbing'/'validate changes', 013 plan 'The validate command's instructions') - outside the four-form claim.
- AC22 — met: scripts/gen-help-tables.sh pipeline_entries lists /{project}:analyze with $CMD_DIR/analyze.md; 'gen-help-tables.sh --dry-run' reports 'No changes (16 command(s) in sync)', exit 0.
- AC23 — met: specs/README.md has no 'Past Renames' section (grep empty); its line 9 says historical renames live in git history, matching the annotation.
- AC24 — met: analyze.md description opens verbatim 'Audit artifacts against each other — spec, plan, tasks, scenarios, frontmatter, dependencies, rule IDs.' and its tail is the longer 'Detection never modifies...' form, exactly as the annotation states; help.md row 55 carries the same text.
- AC25 — met: framework/commands/review.md description reads exactly 'Audit code against rules — security, reuse, quality, efficiency, simplicity. Writes review.md; blocks done on MUST violations.'
- AC26 — met: Both analyze.md and review.md descriptions begin 'Audit artifacts' / 'Audit code' verbatim; help.md rows 54-55 render them.
- AC27 — met: specs/023-govern-refinement/analysis.md (last-run 2026-11-18, analyzed-against 9cae484c) records hard-fail 0, blocking-findings 0, blocking false; since then only c9657c88 touched the spec dir (a scenario, tasks.md, status), and a live 'ductus check-artifacts --feature 023-govern-refinement' shows 2 advisory findings only (both waived in the record) and validate-frontmatter on spec.md is clean; no fresh /analyze run was done after c9657c88.
- AC28 — met: No /capture, /elaborate or lightweight-track references in README.md, AGENTS.md, framework/ (bar the migration procedure's record of an old Track field), docs/ or help.md; gen-help-tables --dry-run is in sync; docs/introduction.md is absent as annotated; specs/README.md line 9 still names the old verbs, but only as a rename record pointing at git history.
- AC29 — met: framework/constitution.md Brownfield Process keeps 'Capture → incremental growth → promotion' with step 1 '/specify with whatever description exists; sparse acceptance criteria are expected and valid for brownfield use', and the <!-- §brownfield-process --> marker is present.
- AC30 — met: target.md's done row reads 'complete for what it specifies; run /{project}:amend to record new work'; status.md's reads 'done (spec is complete)' and 'git show 670a3181:framework/commands/status.md' shows it read that already, so each annotated statement holds.
- AC31 — met: scripts/lint-tool-coverage.sh exits 0 (17 command files examined); `ductus parse --check` returns 0 for framework/commands/specify.md and 2 (LegacyProse, on runtime/legacy-prose-commands.txt, never Invalid) for amend.md, and scripts/lint-procedure-parseability.sh exits 0 over both.
- AC32 — met: As annotated: .github/workflows/markdown-only-pipeline.yml is absent (ls .github/workflows: framework-checks, generators, runtime*, only), the annotation's replacement framework-checks.yml exists, and the runtime CI half holds: .github/workflows/runtime.yml runs `cargo test --release --locked` and `cargo clippy ... -D warnings`; no CI verdict was read here.
- AC33 — met: framework/commands/amend.md §Re-open precondition and reconcile pass: no-input only, done/planned/in-progress only (draft/clarified skip), per-slug match in heading/subtask/Done-when matching specs/067-analyze-and-audit-checks/data-model.md's canonical referencing-task rule, pending task skipped, checked tasks named in the offer, per-scenario prompt, append-task with existing slug on confirm, set-status done->in-progress folded into step 4; linked scenario extend-existing-scenario-task.md exists.
- AC34 — met: framework/bootstrap/configure/claude.md step 2 holds no `-`Write(` bullet (python scan of steps 2-3), keeps Edit(.ductus/session.toml), Edit(.ductus/config.toml) and bare Edit/Write; the rationale paragraph names both removed entries; .claude/commands/ductus/configure.md differs from the source only by the generator's ToolSearch line; CANONICAL_ALLOW_STATIC in runtime/src/primitives/merge_permissions.rs agrees; the Write(.ductus/...) strings sit only in step 4's retired list.
- AC35 — met: Ran scripts/audit/permission-entry-shape.sh: clean on the tree (28 entries); on scratchpad copies a `Write(.ductus/x.toml)` bullet in allow and one in deny each exit 1 with a finding; bare Edit/Write and the rationale prose are not reported (base copy clean); the script examines only configure/claude.md; the empty-extraction finding is in the code (entry_count -eq 0 emit) but was not driven.
- AC36 — met: claude.md step 4 'Retired permissions.allow entries' lists the seven git -C entries and two Write(.ductus/*.toml) entries grouped by reason, says step 1 passes them as revoke, the Scope Boundaries confine retirement to that list, and 'Why this list lives here and not in a /ductus migration' gives the last_applied reason; RETIRED_ALLOW[0..9] in merge_permissions.rs equals them (now 31 entries, per the criterion's own superseded-in-part note).
- AC37 — met: met once corrected in this run: run() in runtime/src/primitives/merge_permissions.rs calls validate_revoke_disjoint before ReadSnapshot::read of the settings file, after validate_contained, check_no_protected and canonical_directories; at d1c56d42 it followed only the lexical traversal check; merge-permissions driven on scratch settings files
- AC38 — met: Scratchpad run with all nine retired entries (one doubled), the near miss `Bash(git -C ~/mine status *)`, deny `Bash(git -C * rm *)`, ask/defaultMode siblings and model/env keys: first run 'updated' with the nine named in `revoked`, retired entries gone, near miss/deny/siblings/top-level keys intact; second run 'unchanged' with mtime_ns identical; a malformed file exits 1 with a JSON parse error and its bytes unchanged; claude.md step 7 names each removed entry in the closing report.
- AC39 — met: permission-entry-shape.sh 32b: on scratchpad copies a retired entry also canonical (`Bash(ls *)` added to step 4) exits 1 naming ConflictingRevoke; a retired section with no bullets exits 1 ('holds no bullet entries'); with the section removed 32b prints 'no retired-entries section present, so no overlap was examined' and skips (the exit-1 there is 32c's own finding that claude.md must carry a retired list); the real tree passes with 31 retired entries checked.
- AC40 — met: specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md's 'does not retroactively strip' bullet is struck through, followed by a **Superseded** note citing AC36-AC38 and linking configure-retires-formerly-canonical-entries.md, and kept rather than deleted; that scenario file exists.

## Observations

- convention: framework/reference/audit.md's Family 29 entry still says one host is checked, claude.md, after the family began reading the /ductus seed — `framework/reference/audit.md` — **fixed** (decided by the rule)
- convention: configure-inert-write-path-entries's Edge Case says an adopter keeps both Write entries until a 027 migration runs, though /configure now retires them (AC36-AC38) — `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md` — **fixed** (decided by the rule)
- convention: merge_permissions.rs's module comment points at spec 027's retired-permission-entry-cleanup scenario, which does not exist — `runtime/src/primitives/merge_permissions.rs` — **fixed** (decided by the rule)
- convention: 023's §1 prose says the spec-and-plan-sunset entry and its procedure file survive, though 027's review archived both, and points at Open Questions for a Resolved Question — `specs/023-govern-refinement/spec.md` — **fixed** (decided by the rule)
- convention: bare-prose validate names survive in done specs' prose, 016's spec twice and 013's plan once — `specs/016-cross-cutting-rules/spec.md` — **fixed** (decided by the rule)
- convention: 016's AC8 names its spec, template and validate changes by the command's old name — `specs/016-cross-cutting-rules/spec.md` — **waived**: the phrase records the changes 016 made under the command's name at the time, outside AC21's four-form claim, and editing the criterion would reopen 016 for one token (decided by the rule)
- convention: the Context sections of the dedup, inert-write, retires and pattern-safety scenarios state their problem in the present tense as it stood when each was written — `specs/023-govern-refinement/scenarios/configure-dedup-permissions.md` — **waived**: a scenario's Context is the problem it was written against; each Behavior, its superseded annotations and the criteria state the current shape (decided by the rule)
- convention: 048's AC22 annotation says the rest of the acquisition sequence prompts for nothing on Claude, which a reader may take to cover the Windows [runtime] path digest — `specs/048-govern-acquired-runtime/spec.md` — **waived**: 048's own review reads its spec in this window, and the claim concerns the acquisition sequence, which no Windows host runs since 048's AC25 (decided by the rule)
- convention: Permission Setup's closing rationale against a prompt on every bootstrap stands beside the Windows configured binary's per-run digest prompt — `framework/bootstrap/ductus.md` — **waived**: the agent's recommendation, adopted by the operator, 2026-11-21: that prompt over a trailing wildcard or no entry, and the same paragraph states it (decided by the rule)
- other: the pinned certutil -hashfile .ductus/bin/ductus.exe SHA256, a Git Bash relative path, is unmeasured on Windows, where the measurement harness passes a cygpath -w path — `framework/bootstrap/ductus.md` — **waived**: the procedure passed Git Bash paths to certutil before this change too, and no Windows runner exists since 048's AC25 (owner, 2026-11-19) (decided by the rule)
- bug: mid_wildcard does not flag a pattern whose first token is a wildcard, such as Bash(* ls) — `scripts/audit/permission-wildcard-position.sh` — **waived**: the predicate predates this change and serves both subjects, and no allow entry either set ships has that shape (decided by the rule)
- other: Family 29's seed parse is a third copy of the Agent Registry row read that Families 32c and 14 also carry — `scripts/audit/permission-wildcard-position.sh` — **waived**: each family is a standalone script by the audit contract and lib.sh holds bash helpers only, so a shared reader is a new surface rather than a mechanical fix; a broken parse reports findings, never a pass (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/048-govern-acquired-runtime/tasks.md`

## Unexamined governance

*None.*
