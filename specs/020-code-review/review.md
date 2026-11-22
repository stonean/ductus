---
spec: 020-code-review
last-run: 2026-11-22T21:32:51Z
reviewed-against: c2e78f7510147fa154648f1abdd58b15a84b03e5
diff-base: 0400e970752f57b249d77f7ad16bba7d230eb614
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 18
scope: 37
unread:
  - .claude/commands/ductus/review.md
  - .ductus/reference/review.md
  - .pi/prompts/ductus-review.md
  - runtime/CHANGELOG.md
  - runtime/src/interpreter/mod.rs
  - runtime/src/primitives/mod.rs
  - runtime/src/primitives/write_analysis.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/extensions.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/mcp.rs
  - runtime/tests/walker.rs
  - scripts/mutation/scope-attribution.py
  - scripts/mutation/site-anchors-020.py
  - specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md
  - specs/025-rule-opt-out/spec.md
  - specs/058-findings-route-at-discovery/data-model.md
  - specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md
excluded: 8
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 1
diff-checked: 1
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 2b2765b2d9c821ff57a660e6616eeb317e27f271b133e106b9cb2854a618150c
  scenarios/every-loaded-rule-file-has-a-pass.md: 1f684e5b4c4c3d6aa764a4744f27fda185952ca95ae7743de1190e042337b78b
  scenarios/review-flag-parsing-is-specified.md: a19fe0f4f50c954363f7cca4d913a81d96374535d31a986792243be5263631c1
  scenarios/review-reports-a-defect-under-its-owner.md: ef91f01e38d96520853db62f5ee62b9b8893d41582f055f4f01a25d5a796b665
  scenarios/waiver-expiry.md: 626f92a6e7ee0063e17e4a8135fc21e72a555e6caa0b731401acd29212337b42
  scenarios/waiver-file-lists.md: 3a76e64b910f1a1e0be02c6b603909803a959b70c71b1c50ff5ed12c2420684c
criteria-judged: 19
criteria-met: 19
criteria-verdicts:
  - label: AC1
    digest: ce5aa4324d3c5074cc992c77c77fc6544943098ec77457d676bcc21ec6a66c22
  - label: AC2
    digest: 22cf445c79f82fa98f79b09465401f9155f3431b05aed6c19633625eadb29bf3
  - label: AC3
    digest: 438dcd405d7ba95436ed06918be09abda1b2afff3bfa076dfb3be8fbaea3dd96
  - label: AC4
    digest: 907261b65da49141dcbb3021c17ecac2c1adcce25188827f89c2341f00a806ad
  - label: AC5
    digest: 0d8c8bc9088e35ddcf0e4b57f678f1e97831ed39f0821096ab56f9389aacf9a9
  - label: AC6
    digest: fe089c02065e87955465569fdb1a5c91c34ac6507c9c0fa40e17363b81bc4c5d
  - label: AC7
    digest: 4146048a1a2e934fdea9509467d09e41c8e2a23f443ba2b1955055b807d9865b
  - label: AC8
    digest: bbb3164a35920f2fb5ba6e7156315b981715fd2342ff5e815608887c5d41cddb
  - label: AC9
    digest: 5fd3f43cf7b723d5919e1941b4d25b0d78ce13e4b9778b1a3038f4b76b97ee31
  - label: AC10
    digest: c0e2b6fb7c2e4452b23eba54a1f40047298aae8bff0e1f0d917a8460dceccc4a
  - label: AC11
    digest: 5dcb8b369ecbc9ea769332a320af5b9d1d83a98e5ff4ff16814bca49124acf93
  - label: AC12
    digest: 287844dca2caa9884aa8e95b6718c9bdbbcb008945a74cbdcffa23fd9abf8897
  - label: AC13
    digest: d65e39e5269cd6b3df15f21a545fd33d20330d0ef750754e3628b3db9c983841
  - label: AC14
    digest: ad54c90154031e3864972c5bbd26b7ec784d0c2a25fe1f895f71e6d80b803c73
  - label: AC15
    digest: 1f49d9de76d6223d63204b1ebe837a831e706d6b4a51c87badc7a5da6a1ee3ef
  - label: AC16
    digest: 08d9a0ed5efc065e5ccfca189c8b754468e218e3eefaafa73f2dc44b49b8dca6
  - label: AC17
    digest: c162674202583e492ffdb1b8fe4905bb3fd82ab6ca0cb01c11b2a62716c22c26
  - label: AC18
    digest: de7805675a6786eee063eb29aea997db67f39ab6335f6db645cdf769ecb58429
  - label: AC19
    digest: 893cfa2d724a3b9976fc5376e6133a8071ace1c649b189da5b2ce2051afb1edd
blocking: false
dispositions:
  fixed: 14
  deferred: 0
  waived: 8
  undispositioned: 0
decisions:
  - key: "other — the Context of scenarios every-loaded-rule-file-has-a-pass and review-reports-a-defect-under-its-owner states in the present tense what task 14 and task 15 changed: that the reference names a rule file in only one pass, that the quality pass's definition does not say it enforces `quality-cross.md`, that `/review` applies no Not-flagged clause, and that 18 yields sit in files no pass applies today. — `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md:9`"
    outcome: waived
    reason: Each Context records the defect as found on a stated date (2026-10-19, 2026-11-02, anchored to ab2ad085 in the Resolved Questions), the Resolved Questions and Behavior state the requirement, and rewording the found-state changes no requirement.
    decided-at: 2026-11-15T15:51:01Z
    decided-by: rule
  - key: other — plan.md's first Technical Decision says `/ductus:review` has no new code and no new runtime, and its Overview says the same, while the plan's later decisions record the runtime half (`process-waivers`, `review_pass_for_rule_file`) and §Plan records the design as it stands. — `specs/020-code-review/plan.md:9`
    outcome: waived
    reason: Rewriting the Overview and the first decision into the runtime-backed design is a design-record edit, not a mechanical chore, and the plan's later decisions state the runtime half.
    decided-at: 2026-11-15T15:51:01Z
    decided-by: rule
  - key: "other — the unapplied-file naming in the Summary on a restricted run rests on the reviewer writing it: `write-review`'s `## Skipped passes` lists passes only, and nothing derives the rule files the skipped passes would have applied, although `review_pass_for_rule_file` maps them. — `framework/commands/review.md:125`"
    outcome: waived
    reason: The record's `skipped-passes` plus the §Load rules mapping let a reader derive the unapplied files, so the record is not indistinguishable from a clean one; deriving them in write-review is new runtime requirement, not a chore.
    decided-at: 2026-11-15T15:51:01Z
    decided-by: rule
  - key: other — spec.md's Tasks list still directs edits to a `review:` block in spec frontmatter and the spec template, which 057 removed — `specs/020-code-review/spec.md`
    outcome: waived
    reason: The list records what 020 delivered, and the relocation is stated beside it, in the 057 note and the Frontmatter schema section; rewording a delivered task list changes no requirement and makes no claim about the tree.
    decided-at: 2026-11-19T02:29:42Z
    decided-by: rule
  - key: "other: discover-rule-files' header test runs only over the rule-file directory's selection; a file an `AGENTS.md` reference adds under `extra-files` (outside the directory) is never header-tested, so one whose header names analyze alone still gets the quality pass, which AC17's stated purpose (the two audits never judge one rule differently) would not predict. The spec's AC17 speaks of the review's selection only, so no criterion is broken; low confidence that anyone intended it. — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: An extra-files entry is a rule file the project's AGENTS.md names by path, outside the rule-file directory AC17 governs; the header test would only drop a file a review now reads, so leaving it in costs one more pass and loses nothing.
    decided-at: 2026-11-22T00:22:28Z
    decided-by: rule
  - key: "other: the count of findings at a (rule, file, site) anchor is computed in four places (`Pass::place` in process_waivers.rs, `lower_covers`, `findings_at_key` and `new_waiver_entries` in write_review.rs), each over `collapse_at_sites` with its own site trim, so the guarantee that both sides count alike rests on the four agreeing — `runtime/src/primitives/process_waivers.rs`"
    outcome: waived
    reason: The four share collapse_at_sites and agree today (a waiver write-review recorded applied unexpired on the next run when driven), and folding them into one counter is a refactor of the pipeline's own machinery with no defect to fix.
    decided-at: 2026-11-22T21:32:51Z
    decided-by: rule
  - key: "other: write-review lowers a matched deferral's `covers` on a restricted run too, so a restricted run whose skipped pass produced findings of a rule another pass also fires at one site lowers it, and the next full run expires the deferral with the count notice — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: It needs one rule's findings at one site from two passes, one skipped; the expiry it causes prints the count notice and asks for one repeated decision, so it is never silent.
    decided-at: 2026-11-22T21:32:51Z
    decided-by: rule
  - key: "other: write-review keeps a matched stored deferral's `covers` when a run carries more findings at its key than it covers and the host left that decision out of `expired-decisions` — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: Only a host that omits a count-expired decision from expired-decisions, against step 11, reaches it, and the next process-decisions expires the decision with the count notice, so it corrects itself and is never silent.
    decided-at: 2026-11-22T21:32:51Z
    decided-by: rule
  - key: "other: data-model.md's frontmatter table gives `skipped-passes` permitted values that write-review does not check, writing each entry as given — `specs/020-code-review/data-model.md`"
    outcome: waived
    reason: The host passes the skipped dimensions from its own flag parsing, which names only the five passes, so the writer's callers hold the claim; the table describes what a record holds.
    decided-at: 2026-11-22T21:32:51Z
    decided-by: rule
  - key: "security: process-decisions passes the file `decisions::key_anchor` reads from a stored key to `RunScope::why_unread`, which probes it with `symlink_metadata` under the project root with no containment check, so a hand-edited key naming an absolute or `..` path probes outside the project — `runtime/src/primitives/process_decisions.rs`"
    outcome: waived
    reason: The probe reads only whether a path named in the project's own review record exists and writes nothing; such a key decides only whether its decision is retained or expired, and the record is the project's own file.
    decided-at: 2026-11-22T21:32:51Z
    decided-by: rule
---

# Review — 020-code-review

## Summary

Five-pass review of 020's reopening for AC19 (diff-base 0400e970, base-source reopen) at HEAD c2e78f75, whose window holds task 20's commits c8c0357b, 8ef7d0e5, b9d5d4fe and 83b2aad2 and tasks 21-23's 1ab1fbf3 and c2e78f75. The passes ran in parts split by behavior, each driving the release binary on fixtures under $TMPDIR: R1 waiver anchoring, R2 violation deferrals with the CLI and schema surface, R3 the prose contract with the tests and the mutation harness, and C the criteria step; a fifth part, M, re-derived AC19's guard-removal clause in a clone. The five rule files discover-rule-files selected were applied from its 99,611-byte rules list, read whole by R2 and R3 and as 77,329 bytes of rendered Statement text by R1, each rule file opened where a rule might apply. Four broken contracts were found and fixed in the run: the cross-pass dedup merged a rule's findings across sites, so a covered finding absorbed an uncovered one at another site (it now matches the site, and AC13's parenthetical says so: the agent's recommendation, adopted by the operator, 2026-11-22); a violation's decision-key keyed its finding to any stored deferral and exempted a site-less one from the --defer refusal (it is now the finding's own key or refused: the agent's recommendation, adopted by the operator, 2026-11-22); key_anchor misread a violation key whose heading site holds an em dash and a backtick; and process-waivers read a departure entry holding anchors by those anchors. R2's QUAL-TEST-001 SHOULD on fired_keys forwarding an untested decision-key no longer fires: the forwarding is gone and the refusal that replaced it is tested. The re-derived guard-removal list, scripts/mutation/site-anchors-020.py, found one guard no fixture proved (a --waive beside a stored waiver of its rule at another site), now tested; its 44 mutations, run in a clone, killed 42, M19 and M39 being redundant guards. Ten chores were fixed by the rule, among them stale claims in the reference, the scenario, the plan, the data model and 025's Motivation (a correction, so 025 stays done), and the prose-claim sweep of this run's own fixes corrected 022's review-runtime-acceleration scenario and 058's and 076's data-model rows, all three specs already in-progress. Five observations were waived by the rule, and three stored decisions matched. The fix-round re-read of the 22 paths the fixes changed found one SHOULD, four doc comments in runtime/src/schema/primitives.rs still describing a host-matched decision-key as replacing the finding's own key, fixed by its four prescribed edits and diff-checked; the next round was empty. The criteria step judged every criterion individually: 19 of 19 met, AC13 and AC19 re-judged on the fixed tree. Examined 18 of 37, each read whole by a part: the window's 32 entries, and five this run's fixes added as uncommitted work, which the passes did not read and step 12's re-read reads (scripts/mutation/site-anchors-020.py, 022's scenario, 025's spec.md, and 058's and 076's data models). Unread besides those five: the three generated mirrors (in sync by scripts/gen-claude-commands.sh --check, then regenerated after the fixes); and, read only in their window hunks and the units those touch, runtime/CHANGELOG.md, runtime/src/interpreter/mod.rs, runtime/src/primitives/mod.rs, write_analysis.rs and write_review.rs (its production half read whole by R1, its test module in part), runtime/src/schema/descriptions.rs, extensions.rs and primitives.rs, runtime/tests/mcp.rs and walker.rs, and scripts/mutation/scope-attribution.py. R1 applied the passes as the shared brief stated them and did not read the reference's Load rules, Run review passes, Criteria step and Write review.md sections. Two stored decisions on files outside the scope were retained.

2 fix round(s): 1 re-read, 1 diff-checked, 0 exempt, 0 not re-read.

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

- AC1 — met: `framework/commands/review.md` is the verb-named command (argument-hint, Instructions steps 1-12); generated `.claude/commands/ductus/review.md` and `.pi/prompts/ductus-review.md` exist; `runtime/tests/review_command.rs` `review_command_parses_to_the_expected_procedure` parses it; generator parity is R3's check.
- AC2 — met: Drove write-review on a fixture with a MUST, a SHOULD and a low-confidence MUST: `specs/001-x/review.md` rendered MUST, SHOULD and Low-confidence sections (counts 1/1/1, blocking true); render in `runtime/src/primitives/write_review.rs`.
- AC3 — met: `framework/commands/review.md` steps 3-7 run the five passes; `review_pass_for_rule_file` in `runtime/src/interpreter/payload.rs` maps security-* files to the security pass; `discover-rule-files` selects security-backend/frontend/cross (read the code and ran it on a fixture rule dir, not the real tree, to leave the project's results dir alone); `Stack/surface` filtering is the Resolved Question's.
- AC4 — met: Drove write-review: `review.md` frontmatter carries last-run, must-violations, should-violations, blocking and waivers while `spec.md` stays status/dependencies only; validate-frontmatter on a spec with a residual `review:` block returned a blocking finding 'review: no longer belongs in spec frontmatter'.
- AC5 — met: Drove check-review-gate on fixtures: blocked-by must-violations (blocking record), not-reviewed (no review.md), record-unreadable (unparseable record, undeterminable); analyze-drift and CI-step halves read at `framework/commands/implement.md`/`framework/reference/analyze.md`/`framework/templates/ci/adopter-generators.yml` but not re-driven this run.
- AC6 — met: Two write-review calls with identical arguments and reviewed-at 01:00 vs 05:00 produced `review.md` files whose only diff was the `last-run` line.
- AC7 — met: `framework/reference/review.md` 'Apply --fix' and 'Auto-fix scope' state conservative auto-fixes and never auto-applying behavior changes; applied by command step 11; host-judgment contract with no runtime path to drive, judged by reading.
- AC8 — met: Drove write-review new-waivers (waiver with anchors, reason, waived-at, waived-by landed in `review.md`), process-waivers (expired with the 'waiver expired:' notice when the rule no longer fired at its site; applied otherwise) and a hand-added `co-waived-by` field survived a re-render (`RawWaiverFull` `#[serde(flatten)]` in `write_review.rs`); records waivers in `review.md`.
- AC9 — met: Every write-review result carried `exit-code` 1 with blocking true and 0 otherwise while the CLI process still exited 0, as the criterion's parenthetical says; CI gates on the record (AC5).
- AC10 — met: `README.md` Commands section lists `/review` under '- **Pipeline — advance state**' with a one-line purpose, linking `docs/slash-commands.md` heading '### `/review` — `review.md`, and a hold on `done` while violations stand', which exists.
- AC11 — met: `framework/commands/review.md` step 1 and `framework/reference/review.md` 'Resolve target and scope' halt on a missing/empty or inconsistent Tech Stack and honor `[review] tech-stack-verified` (key registered in `runtime/src/schema/config_keys.rs`); host-judgment step, judged by reading, no fixture.
- AC12 — met: write-review --empty-scope on a fixture with a one-file window wrote a `review.md` with 0 MUST/SHOULD, blocking false and result exit-code 0; the refusal over an earlier record counting a finding was also seen ('invalid empty-scope').
- AC13 — met: `dedup_findings` in `runtime/src/primitives/write_review.rs` keeps the highest-ranked of findings sharing rule, file, site and an overlapping range: `cross_pass_dedup_highest_severity_wins` collapses a SHOULD 10-20 and a MUST 15-25 at one site to 1 MUST, and `the_dedup_keeps_findings_at_different_sites_apart` keeps overlapping findings at two sites apart in either order and behind a whole-file one; the parenthetical names the site since this run (the operator's adoption, 2026-11-22).
- AC14 — met: `framework/commands/review.md` 'Parsing $ARGUMENTS' specifies every flag, a bare `--since` and an unrecognized `--flag` are reported and stop; its argument-hint names --all --fix --security --simplicity --quality --since --waive --defer (--target --task --work-item --reason); check-command-flags on a fixture holding the command returned findings 0, with-flags-table and with-argument-hint 1 each.
- AC15 — met: write-review recorded `examined: 1` against a self-derived `scope: 1` in `review.md`; with --examined omitted the field was absent from the record, so absent is not zero; the superseded half is stated in the criterion.
- AC16 — met: `framework/commands/review.md` step 3 and `framework/reference/review.md` 'Passes in a subagent' state the fresh-subagent run of steps 3-7.2 and the hand-back and short-read rule; `runtime/tests/review_command.rs` `review_command_parses_to_the_expected_procedure`, `interpreter/mod.rs` `perform_review_emits_one_llm_request_per_pass_step` and `the_review_walk_judges_every_criterion_and_writes_its_tick` show exec asks each pass then the criteria (read, not re-run).
- AC17 — met: Drove discover-rule-files --rules on a fixture (analyze-only header, header naming both, header naming neither): only the analyze-only file was left out, with the notice 'rule file a-backend.md is judged by analysis alone ...'; `discover_rule_files.rs` test `a_review_leaves_out_each_rule_file_whose_header_names_analyze_alone` holds a fixture of each kind.
- AC18 — met: `framework/reference/review.md` '## Run review passes' states a finding, observation or claim is recorded only when its trigger is shown possible from code, configuration, history or deployment shape and its mechanism checked, a case shown only by building it being a false positive recorded nowhere and fixed in no round; 'Passes in a subagent' requires the hand-back to carry the evidence and drops one without it; prose contract, no test.
- AC19 — met: Met after this run's fixes: process-waivers, process-decisions and write-review cover only the findings at a (rule, file, site) anchor up to `covers`, and a finding at another site, a surplus at the site and any finding under an anchor recorded without a site each count with the scenario's notice, driven for a waiver and a deferral on fixtures (parts R1, R2, C) and held by tests such as `a_waiver_covers_only_the_findings_at_its_site`, `more_findings_than_it_covers_expire_the_decision_with_the_count_notice`, `the_dedup_keeps_findings_at_different_sites_apart` and `a_violations_decision_key_that_is_not_its_own_is_refused`; --waive and --defer refuse a site-less finding whatever its decision-key; `scripts/mutation/site-anchors-020.py`, 44 guard removals run in a clone: 42 killed, M19 and M39 redundant guards.

## Observations

- security: process-decisions passes the file `decisions::key_anchor` reads from a stored key to `RunScope::why_unread`, which probes it with `symlink_metadata` under the project root with no containment check, so a hand-edited key naming an absolute or `..` path probes outside the project — `runtime/src/primitives/process_decisions.rs` — **waived**: The probe reads only whether a path named in the project's own review record exists and writes nothing; such a key decides only whether its decision is retained or expired, and the record is the project's own file. (decided by the rule)
- bug: write-review's cross-pass dedup (`dedup_findings`) merged a rule's overlapping findings across sites, so a waived or released-deferred finding at one site absorbed an uncovered finding of its rule at another, by input order and always behind a whole-file finding, breaking AC19's count at another site for waivers and deferrals alike; it now matches rule, file, site and overlapping range, as `collapse_at_sites` counts, AC13's parenthetical gaining the site (the agent's recommendation, adopted by the operator, 2026-11-22) — `runtime/src/primitives/write_review.rs` (breaks the spec's contract) — **fixed**
- bug: a violation's `decision-key` was used verbatim as its key and exempted a finding naming no site from the --defer refusal, so a host's match could key a finding at another site, or a site-less one, to a stored deferral, the host match AC19 and the scenario waiver-expiry's Resolved Questions reject; write-review and process-decisions now refuse a decision-key that is not the finding's own key (`decisions::foreign_violation_key`) and the site-less refusal holds whatever key it carries (the agent's recommendation, adopted by the operator, 2026-11-22) — `runtime/src/primitives/decisions.rs` (breaks the spec's contract) — **fixed**
- bug: `decisions::key_anchor` read a key's observation shape first, so a violation key whose site is a heading holding an em dash and a backtick, as framework/constitution.md's `Review record — review.md` heading does, named a file ending in a backtick and process-decisions expired a deferral on a file the run had not read instead of retaining it; a rule ID's sited violation key is now read first — `runtime/src/primitives/decisions.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: process-waivers read a departure-marked entry holding `anchors` by those anchors, while plan.md's AC19 decision and write-review's `refresh_departure_waivers` read a departure waiver by its `file` at every site whatever else it holds, so the two primitives classified one entry differently; `read_anchors` now reads a departure waiver by its `file` — `runtime/src/primitives/process_waivers.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: AC19's last clause, that a fixture of each case fails with its guard removed, was proven for task 20's tests only by the session that wrote them and recorded in no artifact, and the re-derivation left one guard unproven (a --waive beside a stored waiver of its rule at another site: `holds_anchor` matching by file alone survived); a test now covers it and `scripts/mutation/site-anchors-020.py` holds the guard-removal list — `scripts/mutation/site-anchors-020.py` — **fixed** (decided by the rule)
- convention: framework/reference/review.md §Per-run waiver processing said the `waiver expired:` notice says which cause expired the anchor, but process-waivers prints one text for a file gone and a rule no longer firing, and names only the count and recorded-before-sites causes, as the scenario waiver-expiry gives them — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention: the scenario waiver-expiry and the reference's §Run review passes said no stored waiver or deferral covers a finding naming no site, but a departure waiver covers every finding of its rule in its files, rule 5's exception; both now say so — `specs/020-code-review/scenarios/waiver-expiry.md` — **fixed** (decided by the rule)
- convention: spec.md §Frontmatter schema showed a waiver keyed by `file` expiring when its file goes or its rule stops firing there, with no AC19 note, though such an entry is now one recorded before sites that expires on the next review; a note states the `anchors:` shape — `specs/020-code-review/spec.md` — **fixed** (decided by the rule)
- convention: framework/commands/review.md step 3 listed a finding's fields without `site`, which the reference's §Security pass lists and --waive and --defer refuse a finding for lacking — `framework/commands/review.md` — **fixed** (decided by the rule)
- convention: plan.md, tasks.md task 22 and a process-decisions test's doc said `check_rule_ids::is_a_rule_id`'s category takes two characters, where its grammar takes two or more — `specs/020-code-review/plan.md` — **fixed** (decided by the rule)
- convention: specs/025-rule-opt-out/spec.md Motivation said waivers are per-`(rule, file)`, which AC19 made `(rule, file, site)`; a spec.md correction outside the criteria, so 025 stays done — `specs/025-rule-opt-out/spec.md` — **fixed** (decided by the rule)
- convention: process-decisions' module doc and the `ProcessDecisionsResult.expired` field doc said an expired decision is one whose key did not fire on an unrestricted run, omitting the count and recorded-before-sites expiries this window added, which hold on a restricted run too — `runtime/src/primitives/process_decisions.rs` — **fixed** (decided by the rule)
- convention: `StoredAnchor::usable` defined a local `nonblank` identical to `decisions::nonblank`, the blank test its readers share — `runtime/src/primitives/mod.rs` — **fixed** (decided by the rule)
- convention: data-model.md's Waiver record said `rule` must reference a known rule at write time, which write-review does not check, and its `decisions` row omitted the `contract-break: true` field the writer emits — `specs/020-code-review/data-model.md` — **fixed** (decided by the rule)
- other: the count of findings at a (rule, file, site) anchor is computed in four places (`Pass::place` in process_waivers.rs, `lower_covers`, `findings_at_key` and `new_waiver_entries` in write_review.rs), each over `collapse_at_sites` with its own site trim, so the guarantee that both sides count alike rests on the four agreeing — `runtime/src/primitives/process_waivers.rs` — **waived**: The four share collapse_at_sites and agree today (a waiver write-review recorded applied unexpired on the next run when driven), and folding them into one counter is a refactor of the pipeline's own machinery with no defect to fix. (decided by the rule)
- other: write-review lowers a matched deferral's `covers` on a restricted run too, so a restricted run whose skipped pass produced findings of a rule another pass also fires at one site lowers it, and the next full run expires the deferral with the count notice — `runtime/src/primitives/write_review.rs` — **waived**: It needs one rule's findings at one site from two passes, one skipped; the expiry it causes prints the count notice and asks for one repeated decision, so it is never silent. (decided by the rule)
- other: write-review keeps a matched stored deferral's `covers` when a run carries more findings at its key than it covers and the host left that decision out of `expired-decisions` — `runtime/src/primitives/write_review.rs` — **waived**: Only a host that omits a count-expired decision from expired-decisions, against step 11, reaches it, and the next process-decisions expires the decision with the count notice, so it corrects itself and is never silent. (decided by the rule)
- other: data-model.md's frontmatter table gives `skipped-passes` permitted values that write-review does not check, writing each entry as given — `specs/020-code-review/data-model.md` — **waived**: The host passes the skipped dimensions from its own flag parsing, which names only the five passes, so the writer's callers hold the claim; the table describes what a record holds. (decided by the rule)
- other — plan.md's first Technical Decision says `/ductus:review` has no new code and no new runtime, and its Overview says the same, while the plan's later decisions record the runtime half (`process-waivers`, `review_pass_for_rule_file`) and §Plan records the design as it stands. — `specs/020-code-review/plan.md:9` — **waived**: Rewriting the Overview and the first decision into the runtime-backed design is a design-record edit, not a mechanical chore, and the plan's later decisions state the runtime half. (decided by the rule)
- other — the unapplied-file naming in the Summary on a restricted run rests on the reviewer writing it: `write-review`'s `## Skipped passes` lists passes only, and nothing derives the rule files the skipped passes would have applied, although `review_pass_for_rule_file` maps them. — `framework/commands/review.md:125` — **waived**: The record's `skipped-passes` plus the §Load rules mapping let a reader derive the unapplied files, so the record is not indistinguishable from a clean one; deriving them in write-review is new runtime requirement, not a chore. (decided by the rule)
- other — spec.md's Tasks list still directs edits to a `review:` block in spec frontmatter and the spec template, which 057 removed — `specs/020-code-review/spec.md` — **waived**: The list records what 020 delivered, and the relocation is stated beside it, in the 057 note and the Frontmatter schema section; rewording a delivered task list changes no requirement and makes no claim about the tree. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/022-deterministic-runtime/spec.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md`

## Unexamined governance

*None.*
