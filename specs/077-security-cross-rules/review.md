---
spec: 077-security-cross-rules
last-run: 2026-11-22T12:41:37Z
reviewed-against: f54f17d111cbfd241d7df4a80c176b9efa5757d0
diff-base: 07061c96d34d085e3db3271d09d4246856a895f8
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 8
unread:
  - runtime/CHANGELOG.md
  - runtime/src/primitives/shipped_rule_ids.txt
excluded: 4
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 83dd2b54037204cd50870705a4abfd7a3e0e7e416d43a4ef7915bbd578657416
  scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md: 5a85f93c9f4244f5a7616e1a2b6141143ff66140831c46830e43be32b3acbe8c
criteria-judged: 11
criteria-met: 11
criteria-verdicts:
  - label: AC1
    digest: a306e1badf3cf037dafea2e21207d5a61920cd8c1b4535d32175f6e86119c17b
  - label: AC2
    digest: e0545301a0c2c7b17190f64f666835ee096add3196fb0be62941b6346e17d170
  - label: AC3
    digest: c65e8b4797c9557481c15996cc2da3057cdc1cd7239ffd0540ce2de4e804220f
  - label: AC4
    digest: 3c87ef82f0b84b5ce99670b939becf9cabed022e46ba064ee4ea8237138c6600
  - label: AC5
    digest: b9c708afcd9024b8a166350b5fc704cd0646fc4f9027b3e355b6c6a477de78f5
  - label: AC6
    digest: 08a95afa8d586daf6fbf5550f0d90abd5609843487bacc3f102834f22f97ecda
  - label: AC7
    digest: 9bdb22b5fc00d018e19d0658133816a28a9c4ab86caeb32b5615be510a627ad7
  - label: AC8
    digest: 71cde3328d48597c054988722db24490b1afeb2332736a58fe92db62005681d0
  - label: AC9
    digest: df3b55fe51fab2677857a44567635f32f4b66ad21f7a9dc941f84f4239745690
  - label: AC11
    digest: fc543be10e58dd5a59a860f3891932fd34bd444a4e586e75bd4737a38e5de0b1
  - label: AC10
    digest: 7d22810e1dd35cc506898c0d6ec6fa2ef8cce80037bf243b5ad1299b1bcfb514
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: rule QUAL-GROUND-001 at scripts/audit/deprecation-removal.sh
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/tasks.md
    task: "21"
    decided-at: 2026-11-03T19:11:22Z
    decided-by: andy@stone.dev
  - key: "convention: step 2's parenthetical in `.ductus/reference/analyze.md` §Rules (blocking and advisory) can read as describing the checks rather than naming the heading — `framework/bootstrap/ductus-procedure.md:198`"
    outcome: waived
    reason: "Style only: the checks applied are the same either way, and Family 39 needs the full heading (owner, 2026-11-03, option a)."
    decided-at: 2026-11-04T03:41:31Z
    decided-by: andy@stone.dev
  - key: "other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924`"
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag."
    decided-at: 2026-11-04T03:41:31Z
    decided-by: andy@stone.dev
  - key: "other: the tasks 43 and 44 changelog entry does not name BE-API-013's explicit bar on a credential or PII in a link sent to a person — `runtime/CHANGELOG.md`"
    outcome: waived
    reason: "Not mechanical, and not untrue: BE-API-013's earlier Not-flagged clause already kept a URL sent in an email or another message as its own, so the entry, which lists new obligations, omits a clarification rather than a change."
    decided-at: 2026-11-08T06:04:10Z
    decided-by: rule
  - key: "other: at /review, BE-API-009's Statement, which no longer carries the MAY for an unregistered redirect_uri a pushed authorization request or signed request object sends, can be read to reach a case SEC-AUTHN-004 permits — `framework/rules/security-backend.md:569`"
    outcome: waived
    reason: "Not mechanical: a pushed redirect_uri comes from a client that authenticated to the server, not a user-controlled destination, and an authorization server's redirect is SEC-AUTHN-004's, which BE-API-009's Not-flagged clause names; until 020 task 15's routing lands /review applies Statements with no routing, the overlap 077's §Edge cases already records for moved cases, and the two other Statements named predate this window."
    decided-at: 2026-11-08T06:04:10Z
    decided-by: rule
  - key: "other: 060's scenario an-assessment-sees-the-rule-it-asks-about quotes SEC-AUTHN-002 committing to all four requirements, now each requirement — `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-rule-it-asks-about.md:9`"
    outcome: waived
    reason: The quote is the example of a measurement dated 2026-11-02, which records its own method; SEC-AUTHN-002 still points at its Statement, so the scenario's behaviour, every request carrying the Statement, is unchanged, and rewriting a dated count's example would stale 060's review for no change of claim.
    decided-at: 2026-11-08T06:04:10Z
    decided-by: rule
  - key: "other: BE-ERR-002's Verification reaches an MCP service over stdio, while AGENTS.md's gotcha says an MCP tool error carries text and no code, MCP defining none for a tool-execution error — `framework/rules/security-backend.md:637`"
    outcome: waived
    reason: "Not mechanical: dropping MCP's name from the examples removed the explicit claim, but by meaning an MCP service over stdio is still an API returning errors to callers outside its process, as fix round 1's re-read found; AGENTS.md's gotcha is this repository's reading of the rule against its own local runtime, naming the parts that assume a deployed service and that an MCP tool error carries no code, while the rule states what an adopter's API owes, and the two meet at the gotcha's 'Name the boundary first'."
    decided-at: 2026-11-08T06:04:10Z
    decided-by: rule
  - key: "other: task 44 item 10's acceptance weighs only option a's cost, while a security-cross.md rule adapted from BE-DEPS-002 could close the cross-only part with no deprecation — `specs/008-security-rules/tasks.md:602`"
    outcome: waived
    reason: "Not mechanical: a new security-cross.md rule is a rule decision of its own, not a correction of this review's fix, and item 10's option b accepted the cross-only gap with its reason; a later re-read or an adopter's need can reopen it as an item on 008, the rule surface's home."
    decided-at: 2026-11-08T06:04:10Z
    decided-by: rule
  - key: "other: a security-frontend.md copied from main between afb8a865 and 0481c218^ hands over only a server the project serves, so it keeps an authorization-server library's redirect_uri case beside SEC-AUTHN-004 — `runtime/CHANGELOG.md`"
    outcome: waived
    reason: "A contrived input: both windows are unreleased main, the second about an hour long, and a project pinning such a copy re-syncs it as the changelog's pinned-copy sentence already says."
    decided-at: 2026-11-08T06:04:10Z
    decided-by: rule
  - key: "other: no automated test drives the install of `framework/rules/security-cross.md` under the three `[rules] surfaces` settings; the surface filter is prose in `framework/bootstrap/ductus.md` and `runtime/src/primitives/discover_rule_files.rs`'s tests name no `security-cross.md` — `framework/bootstrap/ductus.md` — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while reviewing framework work, not a defect of 077's rule file: the filter keeps every `*-cross.md` entry unconditionally by prose, `discover-rule-files` selects the file for each surface setting, and a test of the install is a change to spec 033's or 022's surface, not a fix here."
    decided-at: 2026-11-20T04:36:00Z
    decided-by: rule
  - key: "other: 026's scenario family-40-deprecated-rules-are-removed-on-time says in the present tense that the changelog, docs/rules.md and 008's data-model.md 'repeat the promise' of the BE-AUTHN-009/-013 removal in 0.62.0, but docs/rules.md and 008's data-model.md no longer mention either ID now that both rules are removed; 067's scenario a-record-cites-a-removed-rule likewise says 077 'deprecates' them for removal in 0.62.0 — `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`"
    outcome: waived
    reason: "a scenario's Context records the state that prompted it: 026's and 067's Context paragraphs describe the deprecation labels as they stood when each scenario was written, and neither scenario's Behavior or Edge Cases depends on the labels still existing"
    decided-at: 2026-11-22T12:41:37Z
    decided-by: rule
---

# Review — 077-security-cross-rules

## Summary

The window runs from 07061c96, the parent of 077's reopen 81805f64 (base-source reopen), to f54f17d1, which removes BE-AUTHN-009 and BE-AUTHN-013 from security-backend.md at their 0.62.0 target. 008's, 026's and 067's retired-rules: corrections are excluded as theirs. Two subagent reviewers read it. One ran the five passes against the four rule files review loads (configuration-cross.md, quality-cross.md, security-backend.md, security-cross.md; 180,906 bytes), each read whole, and grepped security-frontend.md for the removed IDs. The other judged the 11 criteria. It built a four-shape fixture for AC3 and AC8, ran check-rule-ids for AC2 and AC5, and judged AC10 on the 2026-11-20 fresh re-read, after a diff from that review's commit showed the rules it covered unchanged apart from this removal and a one-word CFG-ENV-013 edit. It re-read the changed rules itself. The passes read 6 of the 8 scope entries whole: security-backend.md, and 077's spec.md, plan.md, data-model.md, tasks.md and scenario. Two are listed unread. Of runtime/CHANGELOG.md the reviewers read the [Unreleased] and [0.60.0] sections. shipped_rule_ids.txt was compared by script with every heading in framework/rules/*.md: the two lists agree, and the removed IDs are in neither. No rule violation was found. AC9 and AC11 were first judged not met, since they named the CHANGELOG's ## [Unreleased] section, which the 0.60.0 cut renamed. Both now name ## [0.60.0] and are judged met. Of four new observations, three were fixed in the run. 077's spec.md and plan.md still said the two rules stay deprecated. Task 15 had waived 008's scenario sentence partly on the cost of reopening 008, which §design-principles rules out as a waiver's reason. That sentence is now in the past tense, which reopened 008 (c69e2ec2): the agent's recommendation, adopted by the operator, 2026-11-22. The present tense in 026's and 067's scenario Context paragraphs is waived by the rule. Four stored decisions on in-scope files were checked against their text, unchanged in the window, and still stand. One expires: 067's retired-rules: list resolved it.

1 fix round(s): 0 re-read, 0 diff-checked, 1 exempt, 0 not re-read. Exempt: round 1.

11 of 11 acceptance criteria judged met.

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

- AC1 — met: framework/rules/security-cross.md read whole: header declares SEC-{CATEGORY}-{NNN} and LOG, DEPS, AUTHN, DATA; 7 rules (SEC-LOG-001, SEC-DEPS-001, SEC-AUTHN-001 to -004, SEC-DATA-001) each with Statement, Verification, Source; scripts/lint-rule-ids.sh exits 0; Source-fetch dates rest on the 2026-11-20 record, not re-fetched
- AC2 — met: data-model.md registers SEC prefix and the four categories; check-rule-ids over all 12 rule files on spec.md: missing 0, retired 4 (the four old IDs); every SEC-* citation in spec/data-model/plan/tasks and the rule files resolves (only the 4 old IDs missing in plan/tasks/data-model, as records)
- AC3 — met: SEC-LOG-001 Not-flagged yields log-line values to BE-LOG-002 and the other carriers to BE-LOG-008 only where security-backend.md is loaded; discover-rule-files fixture: frontend-only and [] (cross-only) select security-cross.md without backend, backend shapes select both; BE-LOG-002/008 own the cases there; CFG-ENV-006 names SEC-LOG-001 otherwise
- AC4 — met: SEC-AUTHN-001 Statement/Verification equal BE-AUTHN-009 at ad8a36f9^ plus the backend-load yield; SEC-AUTHN-002/003 carry the 008 task 43/44 changes per items 4 and 5, SEC-DEPS-001 per item 2 and the npx yield; every backend-owner yield reads 'where security-backend.md is also loaded'
- AC5 — met: Spec annotates 'Superseded in part'; BE-AUTHN-009 and BE-AUTHN-013 absent from framework/rules/*.md (grep 0 hits), both in retired-rules: of 077, check-rule-ids reports them retired; version is 0.61.0 and CHANGELOG [Unreleased] Removed records the 0.62.0-target removal
- AC6 — met: No old ID remains in security-backend/frontend/configuration-cross; BE-API-009/010(d)/011/013 name SEC-AUTHN-002/-004/-001/SEC-DATA-001/-003, FE-AUTHN-001 SEC-AUTHN-002/-004, FE-STORAGE-003 SEC-AUTHN-003, CFG-ENV-006 names SEC-LOG-001 where backend not loaded
- AC7 — met: git grep of the live-artifact set for the four IDs finds only records: audit records, retired-rules lists, 008/017 signposts and Q-records, a test fixture ID in scripts/tests/test-audit-path-containment.sh; no current-requirement citation (CHANGELOG excluded)
- AC8 — met: framework/bootstrap/ductus.md manifest row 'framework/rules/security-cross.md -> specs/rules/security-cross.md' and the Rule-file surface filter keeps every *-cross.md unconditionally; docs/rules.md row 21 lists it; discover-rule-files in a fixture selects security-cross.md for surfaces [frontend], [backend], [] and [backend,frontend]
- AC9 — met: runtime/CHANGELOG.md ## [0.60.0], the security-cross entry: the four moves (BE-DEPS-006, BE-AUTHN-009, -013, -020 to SEC-DEPS-001, SEC-AUTHN-001 to -003) and the waivers that expire (one under the yielding or old ID, an old ID's also when it is removed); criterion corrected from [Unreleased], which the 0.60.0 cut renamed
- AC11 — met: runtime/CHANGELOG.md ## [0.60.0], the same entry: each successor, removal in 0.62.0 for BE-AUTHN-009 and BE-AUTHN-013, and a pinned security-backend.md reporting each case under both IDs; criterion corrected from [Unreleased]
- AC10 — met: Relied on the 2026-11-20 review.md re-read (rules unchanged 84e090b9..146fc912); git diff 146fc912 HEAD -- framework/rules/ changes only the CFG-ENV-013 Verification ('though' to 'while', meaning unchanged), the two BE-AUTHN removals, and accessibility-frontend.md (not 077's); security-cross.md and security-frontend.md unchanged; I re-read security-cross.md whole and those changed rules across all four shapes: no MUST/SHOULD, no dangling yield to a removed ID (check-rule-ids missing 0)

## Observations

- drift: spec.md §Deprecation and re-pointing still says BE-AUTHN-009 and BE-AUTHN-013 "stay deprecated for removal in 0.62.0", present tense, after f54f17d1 removed both from security-backend.md; task 14's sweep reworded AC5, data-model, scenario and plan §A deprecated rule is a pointer but not this sentence; mechanical fix: "were deprecated, and 0.62.0 removed them (task 14)" — `specs/077-security-cross-rules/spec.md` — **fixed** (decided by the rule)
- drift: plan.md Overview still says "BE-AUTHN-009 and BE-AUTHN-013 stay deprecated" after f54f17d1 removed them; the same plan's §A deprecated rule is a pointer and Trade-offs were updated but this bullet was missed; mechanical fix: name task 14's removal — `specs/077-security-cross-rules/plan.md` — **fixed** (decided by the rule)
- drift: task 15 waives 008's scenario sentence that BE-AUTHN-009 and BE-AUTHN-013 "remain" and "keep their labels"; neither is in any framework rule file now, so the sentence is false against the tree, and the waiver's reason leans on the cost of reopening 008 (constitution §design-principles: waive on the requirement that puts it out of scope, not its cost); the waiver stands only on the historical-tense reading — `specs/077-security-cross-rules/tasks.md` — **fixed**
- other: 026's scenario family-40-deprecated-rules-are-removed-on-time says in the present tense that the changelog, docs/rules.md and 008's data-model.md 'repeat the promise' of the BE-AUTHN-009/-013 removal in 0.62.0, but docs/rules.md and 008's data-model.md no longer mention either ID now that both rules are removed; 067's scenario a-record-cites-a-removed-rule likewise says 077 'deprecates' them for removal in 0.62.0 — `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md` — **waived**: a scenario's Context records the state that prompted it: 026's and 067's Context paragraphs describe the deprecation labels as they stood when each scenario was written, and neither scenario's Behavior or Edge Cases depends on the labels still existing (decided by the rule)
- other: the tasks 43 and 44 changelog entry does not name BE-API-013's explicit bar on a credential or PII in a link sent to a person — `runtime/CHANGELOG.md` — **waived**: Not mechanical, and not untrue: BE-API-013's earlier Not-flagged clause already kept a URL sent in an email or another message as its own, so the entry, which lists new obligations, omits a clarification rather than a change. (decided by the rule)
- other: at /review, BE-API-009's Statement, which no longer carries the MAY for an unregistered redirect_uri a pushed authorization request or signed request object sends, can be read to reach a case SEC-AUTHN-004 permits — `framework/rules/security-backend.md:569` — **waived**: Not mechanical: a pushed redirect_uri comes from a client that authenticated to the server, not a user-controlled destination, and an authorization server's redirect is SEC-AUTHN-004's, which BE-API-009's Not-flagged clause names; until 020 task 15's routing lands /review applies Statements with no routing, the overlap 077's §Edge cases already records for moved cases, and the two other Statements named predate this window. (decided by the rule)
- other: BE-ERR-002's Verification reaches an MCP service over stdio, while AGENTS.md's gotcha says an MCP tool error carries text and no code, MCP defining none for a tool-execution error — `framework/rules/security-backend.md:637` — **waived**: Not mechanical: dropping MCP's name from the examples removed the explicit claim, but by meaning an MCP service over stdio is still an API returning errors to callers outside its process, as fix round 1's re-read found; AGENTS.md's gotcha is this repository's reading of the rule against its own local runtime, naming the parts that assume a deployed service and that an MCP tool error carries no code, while the rule states what an adopter's API owes, and the two meet at the gotcha's 'Name the boundary first'. (decided by the rule)
- other: a security-frontend.md copied from main between afb8a865 and 0481c218^ hands over only a server the project serves, so it keeps an authorization-server library's redirect_uri case beside SEC-AUTHN-004 — `runtime/CHANGELOG.md` — **waived**: A contrived input: both windows are unreleased main, the second about an hour long, and a project pinning such a copy re-syncs it as the changelog's pinned-copy sentence already says. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/spec.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md` — left out of fix round 1
- `specs/008-security-rules/spec.md` — left out of fix round 1

## Unexamined governance

*None.*
