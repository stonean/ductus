---
spec: 068-api-backend-rules
last-run: 2026-11-22T00:17:59Z
reviewed-against: 5a71a350670fd6424f6a3c3a13cb9a2a0fdba3d1
diff-base: 2983d2a34c07c501545bb76512b6c7c9555f11a5
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 1
unread: []
excluded: 71
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
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
    digest: fa943e985d7e148bc759bb3a656b33fe3203abaea366bc34099090abdb5ecd67
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
  fixed: 1
  deferred: 0
  waived: 2
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

Five-pass review of 068's reopening by 925c6905 (020's task 18, AC17), diff-base 2983d2a3, base-source reopen, at HEAD 5a71a350. Its window is 925c6905's edit of `specs/068-api-backend-rules/spec.md`. Passes (part P1, one reviewer for 020 and the eight specs 925c6905 reopened) read that hunk and the file whole against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding, and one observation, fixed in this run. 925c6905's rewrite of the Enforcement bullet left "How a project narrows that" with no antecedent; it now reads "How a project narrows which rule files load", and P1 re-ran its passes over the fix and reported nothing. The two stored waivers of the REPR row and the dated rule count still apply. The criteria step (part C7) judged every criterion individually: 16 of 16 met. AC4 was judged from 42 fetched documents covering all 23 rules, with no mismatch; RFC 9110 was read to character 500,000 of 565,815 (its sections from 18 on, which no rule cites, unread), and two pages to character 100,000, each confirming the one-sentence claim its rule makes. The seven rule files whose header names only analysis are judged by analysis alone, so the efficiency pass ran on its own definition. Examined 1 of 1; nothing unread.

1 fix round(s): 0 re-read, 0 diff-checked, 1 exempt, 0 not re-read. Exempt: round 1.

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

- AC1 — met: `api-backend.md` header lists SCHEMA,APIVER,ERRENV,STATUS,PAGE,IDEMP,COMPAT,PATCH,COND,LRO,REPR = the scope table's 11; 23 rule IDs; `scripts/lint-rule-ids.sh` exit 0 (308 headings, 12 files)
- AC2 — met: headers of concurrency, observability, performance, reliability, security-backend declare disjoint abbreviations (checked by script over each header's Categories list)
- AC3 — met: 23 rules each with ### ID, > Statement, **Verification:**; grep Rationale = 0; no Statement mixes MUST and SHOULD (script); runtime test `no_shipped_statement_mixes_tiers` in `rule_sections.rs` reads all shipped files (not run, no cargo)
- AC4 — met: all 23 rules' cited documents fetched (42 distinct URLs) and no claim contradicted; fetches were summarised by WebFetch's small model, and RFC 9110 was read in 100k-char windows to 500k of 565,815 chars
- AC5 — met: all 23 Verifications open 'Any spec or plan' and commit to what a `spec/plan/system.md/errors.md` must state (script over every rule)
- AC6 — met: 'CI' appears only in BE-SCHEMA-003's Not-flagged clause (as a permitted choice) and BE-SCHEMA-004 (SHOULD); no MUST Statement or Verification requires CI
- AC7 — met: every ID cited from `api-backend.md` (BE-ASYNC-001, AUTHZ-006, `ERR-001/002/004`, INPUT-014, TXN-002, CFG-CONST-003, CFG-ENV-006) exists and its text supports the claim; every api ID cited by other rule files exists (script)
- AC8 — met: Boundaries read against the rules: BE-ERRENV, BE-LRO, BE-COND, BE-PAGE, BE-IDEMP each cite the sibling owner (`BE-ERR-001/002`, BE-ASYNC-001, BE-TXN-002, CFG-CONST-003) and restate none
- AC9 — met: BE-SCHEMA-002 both directions; BE-IDEMP-001 PATCH, `422/409`, RPC request field and error channel; BE-STATUS-001 WWW-Authenticate; `BE-COND-001/PAGE-004/REPR-001` MUST (script); header RPC sentence; COND `etag/ABORTED`; REPR Timestamp; APIVER-002 GraphQL mark alone
- AC10 — met: `framework/bootstrap/ductus.md` Shared Files table lists `api-backend.md`; driven discover-rule-files on a fixture: review selects only `quality-cross.md` with notice 'judged by analysis alone', --all selects `api-backend.md` (narrowing by 020 AC17 holds)
- AC11 — met: grep -c backfill `framework/constitution.md` = 0 and no `api-backend.md` mention there
- AC12 — met: header states the severity posture verbatim in substance; BE-PAGE-003 and BE-REPR-003 Statements are MUST; BE-APIVER-002 requires Sunset not earlier than Deprecation and allows a past date
- AC13 — met: BE-APIVER-001 accepts versionless and commits to none of BE-COMPAT-001's breaking changes; BE-COMPAT-001 last sentence says deprecated stays served until a strategy is adopted
- AC14 — met: BE-SCHEMA-001 requires serving by the API itself (own-origin URL, or gRPC reflection / OpenRPC rpc.discover) and committing to the repo
- AC15 — met: header and `BE-ERRENV-001/002/003` Statements each say 'resource-oriented HTTP API'; header names BE-ERR-002 and BE-ERR-004 for RPC errors
- AC16 — met: BE-APIVER-002 Statement: deprecated gRPC method sends deprecation and sunset as custom response metadata carried in `HTTP/2` response headers

## Observations

- convention: the Enforcement bullet's "How a project narrows that" lost its antecedent when 925c6905 removed the review-criteria clause — `specs/068-api-backend-rules/spec.md` — **fixed** (decided by the rule)
- other: the Rule set scope table's REPR row in `specs/068-api-backend-rules/spec.md` does not mention the bounded-integer exemption — `specs/068-api-backend-rules/spec.md` — **waived**: the row summarizes the category's Statement, which the exemption leaves as it is, since the exemption narrows only what the Verification flags (decided by the rule)
- other: the Resolved Question's "17 MUST and 6 SHOULD rules" is a count before the 2026-11-08 tier changes, and the file now holds 19 MUST-tier and 4 SHOULD-tier Statements of 23 — `specs/068-api-backend-rules/spec.md` — **waived**: the sentence records the state the 2026-11-08 decision started from and says two SHOULDs become MUST (17+2 and 6-2 give today's 19 and 4), so it describes the decision and not the file's current count (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-review.md`
- `docs/developing-ductus.md`
- `docs/rules.md`
- `framework/commands/analyze.md`
- `framework/commands/review.md`
- `framework/reference/analyze.md`
- `framework/reference/review.md`
- `runtime/CHANGELOG.md`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/step15-brief.md`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
