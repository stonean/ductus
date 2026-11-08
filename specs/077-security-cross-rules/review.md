---
spec: 077-security-cross-rules
last-run: 2026-11-08T06:04:10Z
reviewed-against: 5dfc9c2d1c249e11a5c5aa909f86e2f01b19eca7
diff-base: 5a38fdd11c11154ea8281f956f16608831a96c6d
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 14
scope: 15
excluded: 13
contract-breaks: 0
skipped-passes: []
fix-rounds: 3
re-reads: 3
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 7c47616784eef9e266659f9323cbfea1a710519df6d959dc8f056c2a026efce6
blocking: false
dispositions:
  fixed: 21
  deferred: 1
  waived: 6
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
---

# Review — 077-security-cross-rules

## Summary

0 MUST and 0 SHOULD violations counted, 0 low-confidence findings. The five passes ran on 2026-11-08 over base `5a38fdd1`, the parent of 077's reopen (`6e2b3fcd`), measured and not overridden, in three read-only agent slices divided by behavior as §Run review passes asks: OAuth and authorization; URL carriage, configuration and telemetry; build inputs with the rest of the scope, 077's data model checked whole against the rule files. Each slice read its changed rules whole at `5dfc9c2d` and its other files by the window's hunks; the five re-read rounds 008's task 43 records had judged the rule text across the four project shapes, which the passes did not repeat. One SHOULD, `QUAL-GROUND-001` on `plan.md` §Moved rules keep their text, which said the moved OAuth rules changed only their IDs and load conditions, was fixed in this run, so it is not counted. `examined: 14` counts the in-scope files the passes read, of the 15 the scope resolves: `specs/008-security-rules/tasks.md` was read by the task 43 and 44 items each slice's behavior names, the window's hunks, and `runtime/CHANGELOG.md` by its `[Unreleased]` entries for 077 and for 008's tasks 43 and 44; `docs/rules.md`, the fifteenth, joins the scope as this run's own fix, read by fix round 1's re-read. 28 observations, the passes' 20 and the fix rounds' 8 NOTEs: 21 fixed by the rule, 6 waived by the rule with their reasons, and 1 deferred under the stored decision it matched, the 0.62.0 records deferred to 067's scenario `a-record-cites-a-removed-rule`. Three stored decisions are retained, this run reading none of their files: `QUAL-GROUND-001` at `scripts/audit/deprecation-removal.sh`, deferred to 067 task 21; the style waiver at `framework/bootstrap/ductus-procedure.md:198`; and the `--unmeasured` waiver at `framework/bootstrap/ductus.md:924`, which 008's real adopter bootstrap of 2026-11-08 met again. Three fix rounds, each re-read by a fresh agent from `start-fix-rounds`' base: round 1, over the passes' fixes, found 2 SHOULD and 7 NOTE; round 2 found 1 SHOULD and 2 NOTE, which reached the budget of 2; the loop went on by the owner's delegation of 2026-11-08, since only a clean re-read closes it without owing another review; round 3, the prescribed fix alone, was clean, and the next brief was empty. 13 paths only other specs' commits changed are excluded and named below.

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

- convention: docs/rules.md's security-cross.md row named neither SEC-AUTHN-004 nor SEC-DATA-001's credentials and PII kept out of URLs — `docs/rules.md:21` — **fixed** (decided by the rule)
- convention: §Rule set scope item 1's yield bullet named only telemetry and error reports as BE-LOG-008's, not the two carriers 008's task 44 added — `specs/077-security-cross-rules/spec.md:28` — **fixed** (decided by the rule)
- convention: the data model's SEC-LOG-001 row left out product-analytics events and session-replay recordings — `specs/077-security-cross-rules/data-model.md:42` — **fixed** (decided by the rule)
- other: the double report that survives 020's routing was listed only for a configuration value in a log line, an error report, a span or a metric label, not in analytics, session replay or a URL — `specs/077-security-cross-rules/spec.md:77` — **fixed** (decided by the rule)
- other: the tasks 43 and 44 changelog entry does not name BE-API-013's explicit bar on a credential or PII in a link sent to a person — `runtime/CHANGELOG.md` — **waived**: Not mechanical, and not untrue: BE-API-013's earlier Not-flagged clause already kept a URL sent in an email or another message as its own, so the entry, which lists new obligations, omits a clarification rather than a change. (decided by the rule)
- convention: SEC-AUTHN-004 was missing from the /review double-report lists, BE-API-009 from the IDs whose waivers expire, and a pinned security-frontend.md older than 86f46805 from the pinned-copy cases — `specs/077-security-cross-rules/spec.md:67` — **fixed** (decided by the rule)
- other: at /review, BE-API-009's Statement, which no longer carries the MAY for an unregistered redirect_uri a pushed authorization request or signed request object sends, can be read to reach a case SEC-AUTHN-004 permits — `framework/rules/security-backend.md:569` — **waived**: Not mechanical: a pushed redirect_uri comes from a client that authenticated to the server, not a user-controlled destination, and an authorization server's redirect is SEC-AUTHN-004's, which BE-API-009's Not-flagged clause names; until 020 task 15's routing lands /review applies Statements with no routing, the overlap 077's §Edge cases already records for moved cases, and the two other Statements named predate this window. (decided by the rule)
- other: 060's scenario an-assessment-sees-the-rule-it-asks-about quotes SEC-AUTHN-002 committing to all four requirements, now each requirement — `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-rule-it-asks-about.md:9` — **waived**: The quote is the example of a measurement dated 2026-11-02, which records its own method; SEC-AUTHN-002 still points at its Statement, so the scenario's behaviour, every request carrying the Statement, is unchanged, and rewriting a dated count's example would stale 060's review for no change of claim. (decided by the rule)
- convention: the reasons SEC-AUTHN-002 and SEC-AUTHN-003 are stricter than RFC 9700 lived only in 008's tasks.md, which prune may drop — `specs/008-security-rules/tasks.md:553` — **fixed** (decided by the rule)
- convention: the tasks 43 and 44 changelog entry omitted SEC-AUTHN-002's and SEC-AUTHN-003's change triggers and (c)'s key the issuer provides — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- other: BE-ERR-002's Verification reaches an MCP service over stdio, while AGENTS.md's gotcha says an MCP tool error carries text and no code, MCP defining none for a tool-execution error — `framework/rules/security-backend.md:637` — **waived**: Not mechanical: dropping MCP's name from the examples removed the explicit claim, but by meaning an MCP service over stdio is still an API returning errors to callers outside its process, as fix round 1's re-read found; AGENTS.md's gotcha is this repository's reading of the rule against its own local runtime, naming the parts that assume a deployed service and that an MCP tool error carries no code, while the rule states what an adopter's API owes, and the two meet at the gotcha's 'Name the boundary first'. (decided by the rule)
- other: 008 task 44 item 10 left a cross-only command-line application's floating ranges and SEC-DEPS-001's conditional yield of a lockfile-covered version undecided — `specs/008-security-rules/tasks.md:598` — **fixed** (decided by the rule)
- convention: item 10's decision was spliced in after "so no rule reports:" instead of after the owner's call that ends the item — `specs/008-security-rules/tasks.md:598` — **fixed** (decided by the rule)
- other: task 43 item 5's record said BE-API-008 matches BE-API-007's list, which names no coordination service — `specs/008-security-rules/tasks.md:557` — **fixed** (decided by the rule)
- other: task 43 item 16's record said 048's fetch-archive scenario cites BE-INPUT-016, which it does not — `specs/008-security-rules/tasks.md:568` — **fixed** (decided by the rule)
- other: task 43 item 23's record gave a count of entry lines under [Unreleased] with no commit — `specs/008-security-rules/tasks.md:575` — **fixed** (decided by the rule)
- other: §Trade-offs said task 43's [Unreleased] item names the same release-time gap, which its decision closed — `specs/077-security-cross-rules/plan.md:151` — **fixed** (decided by the rule)
- convention: FE-DEPS-007's Source shortened the OWASP Dependency Graph & SBOM Best Practices Cheat Sheet's title — `framework/rules/security-frontend.md:327` — **fixed** (decided by the rule)
- convention: task 44's intro said every item was a re-read's gap, while its last box is a verification its decisions owe — `specs/008-security-rules/tasks.md:587` — **fixed** (decided by the rule)
- other: once 0.62.0 removes the four deprecated rules, check-rule-ids will report a Blocking unknown-rule finding for each record that must keep the old ID (077's spec, 008's and 017's signposts) — `specs/077-security-cross-rules/spec.md` — **deferred** to `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- convention: task 43's SSRF item listed two of the artifacts moving BE-INPUT-007 and BE-INPUT-016 would re-point, 022's plan.md and fetch_archive.rs's comments among the rest — `specs/008-security-rules/tasks.md:568` — **fixed** (decided by the rule)
- convention: §Edge cases said a pinned security-frontend.md older than 86f46805 keeps the authorization server's redirect_uri case in every project, true only before afb8a865 or without security-backend.md — `specs/077-security-cross-rules/spec.md:67` — **fixed** (decided by the rule)
- convention: §Rule set scope item 2 left out SEC-DEPS-001's Not-flagged rewrite, that it governs no lockfile-covered version — `specs/077-security-cross-rules/spec.md:35` — **fixed** (decided by the rule)
- convention: SEC-DEPS-001's phrase a library's lockfile binding no consumer explained only the library half of the cross-only case — `framework/rules/security-cross.md` — **fixed** (decided by the rule)
- convention: docs/rules.md's security-cross.md row named product analytics but not session replay — `docs/rules.md:21` — **fixed** (decided by the rule)
- convention: the changelog's SEC-AUTHN-003 summary hung public clients' refresh tokens on the PKCE and 307 asks — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- other: task 44 item 10's acceptance weighs only option a's cost, while a security-cross.md rule adapted from BE-DEPS-002 could close the cross-only part with no deprecation — `specs/008-security-rules/tasks.md:602` — **waived**: Not mechanical: a new security-cross.md rule is a rule decision of its own, not a correction of this review's fix, and item 10's option b accepted the cross-only gap with its reason; a later re-read or an adopter's need can reopen it as an item on 008, the rule surface's home. (decided by the rule)
- other: a security-frontend.md copied from main between afb8a865 and 0481c218^ hands over only a server the project serves, so it keeps an authorization-server library's redirect_uri case beside SEC-AUTHN-004 — `runtime/CHANGELOG.md` — **waived**: A contrived input: both windows are unreleased main, the second about an hour long, and a project pinning such a copy re-syncs it as the changelog's pinned-copy sentence already says. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/analyze.md` — 008-security-rules, 017-derive-dont-ask
- `framework/bootstrap/ductus-procedure.md` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `framework/rules/api-backend.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `framework/rules/performance-frontend.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `framework/templates/project/errors.md` — 008-security-rules
- `runtime/src/primitives/prune_project.rs` — 008-security-rules
- `scripts/check-release-tag.sh` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `scripts/tests/test-check-release-tag.sh` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `specs/008-security-rules/data-model.md` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `specs/008-security-rules/plan.md` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `specs/017-derive-dont-ask/data-model.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review
- `specs/020-code-review/scenarios/review-reports-a-defect-under-its-owner.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `specs/048-govern-acquired-runtime/spec.md` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules

## Unexamined governance

*None.*
