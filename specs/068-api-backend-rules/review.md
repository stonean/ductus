---
spec: 068-api-backend-rules
last-run: 2026-11-20T04:18:14Z
reviewed-against: 110269e04932214a17d8249b3ed0e73c7426c563
diff-base: fa4ae5c75b7d68e907f18cee94eb8f8157f83f52
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 5
unread: []
excluded: 34
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md: 10b60b309ac55e654cd8dbc0d973a585445fef6dc1109797c278b503e9ee988f
criteria-judged: 16
criteria-met: 16
criteria-verdicts:
  - label: AC1
    digest: 7bf40eb0e11fe899864e201c6fc9b17c4cf57802208901681dc49505b1842079
  - label: AC2
    digest: 65967baecc48d6f059f3d024f78134483eccd761927308eadd6a87bb7fcfd81a
  - label: AC3
    digest: 91865c5d06350e2bbfe3f13b7980ca722e8b9b316517b7fa016bf1d82c51057e
  - label: AC4
    digest: ace0b4bdb898bf7586dac2d7a500d7a3f73946c001176f3ca3302c9b33ba628b
  - label: AC5
    digest: 62e205f941c844abf448fe0ac6f303dcfcda89b8a79a8ebc6725577a3146814b
  - label: AC6
    digest: 436aeece11282af3bee3b7252fd83dc2138b8e883ac2451989f52762a8b885e7
  - label: AC7
    digest: 247b0b0e6be5f4b63064cd976f1ccf3b6fb85fa1bc0cfd138f56fc4b7bf0e7b7
  - label: AC8
    digest: ee60ea761f88b1cd76e999d03f3d733980e5ed0ffbe9c8c9778c34726133a33f
  - label: AC9
    digest: 426b55f44fe7cd554efe54e832569a7782e24c053be614157af39c0c44f92d51
  - label: AC10
    digest: 49f429e584d44eea4ed5a18ab7fa1cad8c3be5700c76525a2761c34616ed2781
  - label: AC11
    digest: 4727d327c737063534ebe06b082f9df1dcfede90a81c5dc7ff311ecab79082e2
  - label: AC12
    digest: d728c4de5e7cecdebb061ebdafbd0b1fb3327cafb6f3be42d9cc6322e30b5bf9
  - label: AC13
    digest: f00f02e7a6c9fde6656ad26864ff15ccbe00668f9effefbf763e3e8c2447cb81
  - label: AC14
    digest: c96870b5e3360aa329de813d219ee32cb6976a79407c7a678be6b6925bc86231
  - label: AC15
    digest: a96d695abf014bc94757def0483bf70e12fe44dd917b3583dbfc5917f7393eb3
  - label: AC16
    digest: 50596483e81335d516e2db28fbb836af5434cecd91b04ef2c97f685ea9c6aa60
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "other: round 1's re-read NOTE, that BE-ERR-002's and BE-ERR-004's Not-flagged clauses hand ERRENV a changed code or an envelope \"for an HTTP API\", a term only api-backend.md's header narrows, so a reader of security-backend.md alone could route a GraphQL API's changed code to BE-ERRENV-002 — `framework/rules/security-backend.md`"
    outcome: waived
    reason: "068's Resolved Question on whether ERRENV reaches RPC protocols decided security-backend.md needs no edit: a yield can hand ERRENV only what ERRENV's own Statements cover, which api-backend.md's header now defines, and the file is 008's, outside this spec"
    decided-at: 2026-11-08T22:28:33Z
    decided-by: rule
  - key: "convention: `BE-REPR-003`'s Source holds a clause saying the MUST goes beyond RFC 7493 §2.2 on purpose, which `specs/008-security-rules/data-model.md` words Source as a citation and never an account, and the trailing citation reads loosely after a clause with commas — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the owner decided on 2026-11-19 that the clause sits in the Source, since a Statement carries no reason (scenario `repr-003-bounded-numbers-and-patch-001-format-home`); the data model's Source wording is 008's, outside this spec, and the loose citation changes no requirement
    decided-at: 2026-11-20T04:18:08Z
    decided-by: rule
  - key: "convention: `BE-PATCH-001`'s relative clause \"that defines how an absent field and an explicit `null` each apply\" can read as attaching to the spec or the endpoint rather than the format — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the sentence keeps the owner's wording and the clause's pre-existing shape, and the scenario and the Resolved Question state that the format defines the semantics, so the requirement is as decided
    decided-at: 2026-11-20T04:18:08Z
    decided-by: rule
  - key: "other: the Rule set scope table's REPR row in `specs/068-api-backend-rules/spec.md` does not mention the bounded-integer exemption — `specs/068-api-backend-rules/spec.md`"
    outcome: waived
    reason: the row summarizes the category's Statement, which the exemption leaves as it is, since the exemption narrows only what the Verification flags
    decided-at: 2026-11-20T04:18:08Z
    decided-by: rule
  - key: "other: task 8's done-when asked for bytes before and after per rule and per file, and no artifact in the window recorded them — `specs/068-api-backend-rules/tasks.md`"
    outcome: waived
    reason: the figures are re-derivable from git with `git show <sha>:framework/rules/api-backend.md | wc -c` and the fix commit's message states them (file 31494 to 31746 bytes, `BE-REPR-003` 779 to 986, `BE-PATCH-001` 1037 to 1082), and a ticked task's text is ephemeral
    decided-at: 2026-11-20T04:18:08Z
    decided-by: rule
  - key: "other: the Resolved Question's \"17 MUST and 6 SHOULD rules\" is a count before the 2026-11-08 tier changes, and the file now holds 19 MUST-tier and 4 SHOULD-tier Statements of 23 — `specs/068-api-backend-rules/spec.md`"
    outcome: waived
    reason: the sentence records the state the 2026-11-08 decision started from and says two SHOULDs become MUST (17+2 and 6-2 give today's 19 and 4), so it describes the decision and not the file's current count
    decided-at: 2026-11-20T04:18:08Z
    decided-by: rule
---

# Review — 068-api-backend-rules

## Summary

Review of 068's reopening over `fa4ae5c7..84e090b9` (base-source `reopen`), scope 5, examined 5: `framework/rules/api-backend.md`, `specs/068-api-backend-rules/spec.md`, the scenario `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md` and `specs/068-api-backend-rules/tasks.md` were read whole, and `runtime/CHANGELOG.md` (610 KB) through the window's whole diff (22,443 bytes, which holds the other specs' hunks, read and not judged) and its 068 entry whole; the rest of that file is outside the window. Three readers ran one at a time: the security pass over `framework/rules/security-backend.md`, `framework/rules/security-cross.md` and `framework/rules/security-frontend.md` (193,813 bytes, read whole) found nothing; the reuse, quality, efficiency and simplicity passes over the other nine rule files, `AGENTS.md` Gotchas to Design Principles, `specs/system.md` and `specs/008-security-rules/data-model.md`, each read whole after a first hand-back that had read seven rule files by search only was sent back to finish, found no MUST or SHOULD; the criteria step judged AC1 to AC16 met, each against the tree, with `scripts/lint-rule-ids.sh`, `check-rule-ids`, `discover-rule-files` and `no_shipped_statement_mixes_tiers` run. 0 MUST, 0 SHOULD. Seven observations: two fixed by the rule (the Resolved Question on `BE-IDEMP-001` cited the draft as -07 and expired, which the rule's Source and `a8f419f7` correct, and the 068 changelog entry's heading and its `BE-PATCH-001` clause overstated the change), five waived by it. Fix round 1 (those two edits, `fix(068)` commit) was re-read clean by an independent agent, then the loop ended empty; `specs/068-api-backend-rules/spec.md` was exempt from re-reading. Unchecked, since the run fetched nothing: every external claim of the file's Sources, which rest on the 2026-11-08 re-fetch in `a8f419f7`, and RFC 7493 section 2.2, which rests on the 2026-11-19 fetch the scenario records. A stored decision keyed on `framework/rules/security-backend.md`, outside the scope, is retained. The run executed in a detached worktree of `origin/main` at 84e090b9.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

16 of 16 acceptance criteria judged met.

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

- AC1 — met: header lists the 11 categories of the scope table; scripts/lint-rule-ids.sh exits 0 over 12 files; 23 BE- headings
- AC2 — met: the five other backend headers declare RACE/LOCK/TXN/COORD, METRIC/TRACE/HEALTH, QUERY/CACHE/POOL/PAYLOAD/ASYNC, TIMEOUT/RETRY/BREAKER/DRAIN/BULK and AUTHN/AUTHZ/INPUT/DATA/API/LOG/DEPS/ERR, none shared
- AC3 — met: every rule has an ID-only heading, a block-quoted Statement and a Verification, no Rationale; no_shipped_statement_mixes_tiers passes
- AC4 — met: commit a8f419f7 records every Source re-fetched 2026-11-08 and the 2026-11-19 fetch of RFC 7493 section 2.2 agrees with the BE-REPR-003 Source clause; neither re-fetched in this run
- AC5 — met: every Verification opens on a spec, plan, system.md or errors.md trigger and states what it must commit, the changed BE-REPR-003 and BE-PATCH-001 included
- AC6 — met: no MUST Statement or Verification in api-backend.md requires CI; CI appears only in BE-SCHEMA-003 Not flagged and the SHOULD BE-SCHEMA-004
- AC7 — met: check-rule-ids over api-backend.md finds 32 citations, none missing or deprecated, and every api-backend ID cited from other rule files resolves
- AC8 — met: the changed rules cite siblings and restate no requirement a sibling owns
- AC9 — met: every 2026-10-19 decision holds in api-backend.md: BE-SCHEMA-002, BE-IDEMP-001, BE-STATUS-001, BE-COND-001, BE-PAGE-004, BE-REPR-001 and BE-APIVER-002
- AC10 — met: framework/bootstrap/ductus.md carries the api-backend.md row and discover-rule-files selects it under backend
- AC11 — met: the constitution Rules Lifecycle paragraph has no backfill sentence naming api-backend.md
- AC12 — met: the header states the severity posture, BE-PAGE-003 and BE-REPR-003 are MUST-tier and BE-APIVER-002 requires no future Sunset date
- AC13 — met: BE-APIVER-001 accepts a versionless strategy and BE-COMPAT-001 says such an API makes no breaking change
- AC14 — met: BE-SCHEMA-001 requires the schema served by the API and committed
- AC15 — met: the header and the three ERRENV Statements limit the category to a resource-oriented HTTP API
- AC16 — met: BE-APIVER-002 says a deprecated gRPC method sends deprecation and sunset as response metadata

## Observations

- convention: `BE-REPR-003`'s Source holds a clause saying the MUST goes beyond RFC 7493 §2.2 on purpose, which `specs/008-security-rules/data-model.md` words Source as a citation and never an account, and the trailing citation reads loosely after a clause with commas — `framework/rules/api-backend.md` — **waived**: the owner decided on 2026-11-19 that the clause sits in the Source, since a Statement carries no reason (scenario `repr-003-bounded-numbers-and-patch-001-format-home`); the data model's Source wording is 008's, outside this spec, and the loose citation changes no requirement (decided by the rule)
- convention: `BE-PATCH-001`'s relative clause "that defines how an absent field and an explicit `null` each apply" can read as attaching to the spec or the endpoint rather than the format — `framework/rules/api-backend.md` — **waived**: the sentence keeps the owner's wording and the clause's pre-existing shape, and the scenario and the Resolved Question state that the format defines the semantics, so the requirement is as decided (decided by the rule)
- convention: the Resolved Question on `BE-IDEMP-001` still said the Idempotency-Key draft-07 expired on 18 April 2026, which the rule's Source and the 2026-11-08 re-fetch in `a8f419f7` correct to -08, not expired — `specs/068-api-backend-rules/spec.md` — **fixed** (decided by the rule)
- other: the `runtime/CHANGELOG.md` 068 entry's heading said `BE-REPR-003` flags an unbounded integer only, though a high-precision decimal stays flagged, and said `BE-PATCH-001`'s Verification already passed the `specs/system.md` home, which it names neither — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- other: the Rule set scope table's REPR row in `specs/068-api-backend-rules/spec.md` does not mention the bounded-integer exemption — `specs/068-api-backend-rules/spec.md` — **waived**: the row summarizes the category's Statement, which the exemption leaves as it is, since the exemption narrows only what the Verification flags (decided by the rule)
- other: task 8's done-when asked for bytes before and after per rule and per file, and no artifact in the window recorded them — `specs/068-api-backend-rules/tasks.md` — **waived**: the figures are re-derivable from git with `git show <sha>:framework/rules/api-backend.md | wc -c` and the fix commit's message states them (file 31494 to 31746 bytes, `BE-REPR-003` 779 to 986, `BE-PATCH-001` 1037 to 1082), and a ticked task's text is ephemeral (decided by the rule)
- other: the Resolved Question's "17 MUST and 6 SHOULD rules" is a count before the 2026-11-08 tier changes, and the file now holds 19 MUST-tier and 4 SHOULD-tier Statements of 23 — `specs/068-api-backend-rules/spec.md` — **waived**: the sentence records the state the 2026-11-08 decision started from and says two SHOULDs become MUST (17+2 and 6-2 give today's 19 and 4), so it describes the decision and not the file's current count (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/rules/accessibility-frontend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/reliability-backend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-cross.md`
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
- `specs/069-accessibility-frontend-rules/scenarios/verifications-flag-only-what-the-statement-covers.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`

## Unexamined governance

*None.*
