---
spec: 077-security-cross-rules
last-run: 2026-11-20T04:36:00Z
reviewed-against: 146fc9129e9a581f705a66df87c10834c4eeda83
diff-base: fa4ae5c75b7d68e907f18cee94eb8f8157f83f52
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 7
unread:
  - runtime/CHANGELOG.md
excluded: 32
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 61f20f04a9f2478e73532be6b6679def811041c53c6774c1503528c018a8b7f8
  scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md: f36db867d7d1e0ecc3adebf96594bc156ae358d0039bd7b483d60a19664ae7d5
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
    digest: ef517b12750a9f6351cc3eedcd56623c022dd6ecdbb16ffd0ff0db5f4e7f89ac
  - label: AC6
    digest: 08a95afa8d586daf6fbf5550f0d90abd5609843487bacc3f102834f22f97ecda
  - label: AC7
    digest: 9bdb22b5fc00d018e19d0658133816a28a9c4ab86caeb32b5615be510a627ad7
  - label: AC8
    digest: 71cde3328d48597c054988722db24490b1afeb2332736a58fe92db62005681d0
  - label: AC9
    digest: 80e2255fb6b5efae1d4e6dee22960a18beba8649a57720d493447e793810c530
  - label: AC11
    digest: 1175f1ff8948596b813394c57bff5b4b537566f47a49ba0bac8eff6818b993d1
  - label: AC10
    digest: 7d22810e1dd35cc506898c0d6ec6fa2ef8cce80037bf243b5ad1299b1bcfb514
blocking: false
dispositions:
  fixed: 6
  deferred: 1
  waived: 3
  undispositioned: 0
decisions:
  - key: "other: once 0.62.0 removes the four deprecated rules, check-rule-ids will report a Blocking unknown-rule finding for each record that must keep the old ID (077's spec, 008's and 017's signposts) — `specs/077-security-cross-rules/spec.md`"
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md
    decided-at: 2026-11-03T19:11:22Z
    decided-by: andy@stone.dev
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
---

# Review — 077-security-cross-rules

## Summary

0 MUST and 0 SHOULD violations, 0 low-confidence findings; all 11 criteria judged met. Run in a clean worktree of origin/main at 84e090b9 over base fa4ae5c7 (the reopen, measured, not overridden), window 7 scope entries. Three read-only readers, one at a time: A, the security pass over the three security rule files read whole (24,283, 50,152 and 119,378 bytes) and the window's hunks; B, the other four passes over the nine other rule files, AGENTS.md's Gotchas to Design Principles, specs/system.md, the five 077 spec files and the window's CHANGELOG hunks (11,021 bytes) read whole, with a verdict on each of the ten stored decisions; C, the criteria step, driving check-rule-ids (12 files examined, 0 missing), discover-rule-files, read-deprecation-labels and a grep of the live-artifact set for the four old IDs. examined 6 of 7: runtime/CHANGELOG.md was read by its window hunks and by every 077 entry whole, but not its 610,176 bytes whole, so it is listed unread. Claims about RFC 9700 section 2.1 and 2.1.1, RFC 8058 section 3.1, CWE-598 and the OWASP REST Security Cheat Sheet were not re-fetched and are unchecked; the artifacts record the RFC fetches of 2026-11-19. 7 observations from the passes: 6 fixed by the rule in this run (the SEC-DATA-001 summaries in the data model and spec item 7, the scenario and spec prose on PII in a path or fragment, three plan slips, task 13's unrecorded byte measurements) and 1 waived (no test drives the security-cross.md install); three stored decisions carried, two waived by the rule and one deferred to 067's a-record-cites-a-removed-rule. Seven stored decisions name files this run did not read and are retained. One fix round, re-read clean by a fresh agent, and the next brief was empty. Fixes landed as three fix(077) commits.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- AC1 — met: security-cross.md holds the 7 rules of Rule set scope with Statement, Verification and Source; header declares SEC and LOG, DEPS, AUTHN, DATA; discover-rule-files --all --rules lists the 7 IDs
- AC2 — met: data-model.md registers the prefix and categories; check-rule-ids over the 12 rule files examined 12, missing 0; scripts/lint-rule-ids.sh exits 0 over 308 headings
- AC3 — met: SEC-LOG-001 states BE-LOG-002's value list and yields to BE-LOG-002 and BE-LOG-008 only where security-backend.md is loaded; test sec_log_001_bars_the_values_be_log_002_bars in runtime/src/primitives/rule_sections.rs
- AC4 — met: SEC-AUTHN-001 is byte-equal to BE-AUTHN-009's Statement and its yields carry the backend load condition; SEC-DEPS-001, SEC-AUTHN-002 and SEC-AUTHN-003 read as the spec's items 2, 4 and 5 state
- AC5 — met: read-deprecation-labels returns BE-AUTHN-009 and BE-AUTHN-013, deprecated in 0.60.0, removal 0.62.0, each naming its successor with no load condition, in security-backend.md
- AC6 — met: the only occurrences of BE-AUTHN-009 and BE-AUTHN-013 in framework/rules are their own headings; BE-API-009/010/011/013, FE-AUTHN-001, FE-STORAGE-003 and CFG-ENV-006 name the successors
- AC7 — met: grep of the live-artifact set for the four IDs finds only records, the deprecated rules' own sections, retired-rules lists and 077's past-tense account of the move
- AC8 — met: manifest row in framework/bootstrap/ductus.md, its surface filter keeps every *-cross.md, docs/rules.md lists the file; discover-rule-files selects it for backend, frontend and cross-only (judged on text, no install test exists)
- AC9 — met: runtime/CHANGELOG.md [Unreleased] move entry names the four ID changes and the waivers they expire; 008's Removed entry names the two removals
- AC11 — met: the same entry names each successor, 0.62.0 for BE-AUTHN-009 and BE-AUTHN-013, the removal of the other two in this release, and the pinned-copy double report
- AC10 — met: readers A and B and the fix-round re-read found no MUST or SHOULD in the rules this spec writes or changes across the four shapes

## Observations

- other: the scenario's edge case and the spec's PII Resolved Question said a PII-bearing path or fragment is no one's flag, false where `security-frontend.md` is loaded, since `FE-STORAGE-003` bars it there; both now say where it holds — `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md` — `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md` — **fixed** (decided by the rule)
- other: `SEC-DATA-001`'s row in `specs/077-security-cross-rules/data-model.md` and spec §Rule set scope item 7 said a credential or PII in a URL, not the query-only PII bar task 13 made — `specs/077-security-cross-rules/data-model.md` — `specs/077-security-cross-rules/data-model.md` — **fixed** (decided by the rule)
- convention: plan §Moved rules keep their text said in the present tense that `BE-DEPS-006`'s kept Statement asks a full commit SHA, a rule 008 removed — `specs/077-security-cross-rules/plan.md` — `specs/077-security-cross-rules/plan.md` — **fixed** (decided by the rule)
- convention: plan §Distribution cited `README.md`'s security bullet by a line number, which drifts — `specs/077-security-cross-rules/plan.md` — `specs/077-security-cross-rules/plan.md` — **fixed** (decided by the rule)
- other: plan §Distribution said `runtime/CHANGELOG.md` gains one entry, while 077 now has the move's entry and task 13's — `specs/077-security-cross-rules/plan.md` — `specs/077-security-cross-rules/plan.md` — **fixed** (decided by the rule)
- other: task 13's done-when item on bytes measured per rule and per file recorded no figure — `specs/077-security-cross-rules/tasks.md` — `specs/077-security-cross-rules/tasks.md` — **fixed** (decided by the rule)
- other: no automated test drives the install of `framework/rules/security-cross.md` under the three `[rules] surfaces` settings; the surface filter is prose in `framework/bootstrap/ductus.md` and `runtime/src/primitives/discover_rule_files.rs`'s tests name no `security-cross.md` — `framework/bootstrap/ductus.md` — `framework/bootstrap/ductus.md` — **waived**: An observation about the pipeline's own machinery made while reviewing framework work, not a defect of 077's rule file: the filter keeps every `*-cross.md` entry unconditionally by prose, `discover-rule-files` selects the file for each surface setting, and a test of the install is a change to spec 033's or 022's surface, not a fix here. (decided by the rule)
- other: once 0.62.0 removes the four deprecated rules, check-rule-ids will report a Blocking unknown-rule finding for each record that must keep the old ID (077's spec, 008's and 017's signposts) — `specs/077-security-cross-rules/spec.md` — `specs/077-security-cross-rules/spec.md` — **deferred** to `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- other: the tasks 43 and 44 changelog entry does not name BE-API-013's explicit bar on a credential or PII in a link sent to a person — `runtime/CHANGELOG.md` — `runtime/CHANGELOG.md` — **waived**: Not mechanical, and not untrue: BE-API-013's earlier Not-flagged clause already kept a URL sent in an email or another message as its own, so the entry, which lists new obligations, omits a clarification rather than a change. (decided by the rule)
- other: a security-frontend.md copied from main between afb8a865 and 0481c218^ hands over only a server the project serves, so it keeps an authorization-server library's redirect_uri case beside SEC-AUTHN-004 — `runtime/CHANGELOG.md` — `runtime/CHANGELOG.md` — **waived**: A contrived input: both windows are unreleased main, the second about an hour long, and a project pinning such a copy re-syncs it as the changelog's pinned-copy sentence already says. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/rules/accessibility-frontend.md`
- `framework/rules/api-backend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/reliability-backend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-frontend.md`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `scripts/audit/README.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/038-concurrency-backend-rules/scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/scenarios/expendable-messages-may-be-dropped-when-recorded.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/069-accessibility-frontend-rules/scenarios/verifications-flag-only-what-the-statement-covers.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`

## Unexamined governance

*None.*
