---
spec: 062-concurrent-session-targets
last-run: 2026-11-14T17:38:26Z
analyzed-against: 46668bfb8f4f2667cb1e10a9558c95245e2b950c
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 2
analyzed-digest:
  data-model.md: 0f1edd3db663df560a438fd4d43607e39f4c1a3f9339b29203638dd3c21b96a9
  plan.md: 9d232e4abd9260c276543f05014f9b447fc082935c1c69592e04bec11bc7065f
  review.md: 8fcb169d49bab581eb6fa0db348689ed3fd72eaf75bf8119333cafffd3428554
  scenarios/first-writers-create-the-ignore-file-once.md: 150b95396db7f7e774f4da1726618b953070531f1810c4272a5c080f7e47c0b8
  spec.md: 4300a10626518bb7f758c08ef4bf1d4a96d90cb1663a29c1edefe4b9449ce73f
  tasks.md: d206ce917d4e0734ab8dc0daa2f6c7bc1418ac275c97c06554c807a20731afc3
unexamined-by-reason:
  root-absent: 2
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "data-model.md#`write-session` result additions"}
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#`retarget-sessions`"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Identity"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Identity"}
  BE-INPUT-004: {verdict: finding, tier: must, at: "plan.md#Storage: confine each process to its own file", message: "Session files open under a sanitized identity and a fixed default name with no canonical containment (validate_contained); the plan itself states the gap, deferred to 022 task 34."}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#Storage: confine each process to its own file"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#`dashboard` and the exec seed"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#Storage: confine each process to its own file"}
  BE-RACE-003: {verdict: addressed, cite: "scenarios/first-writers-create-the-ignore-file-once.md#Behavior"}
  BE-REPR-001: {verdict: addressed, cite: "plan.md#`write-session`: own file plus default"}
  BE-REPR-002: {verdict: finding, tier: should, at: "data-model.md#`write-session` result additions", message: "New tool fields (seen-peers, last-used, session-identity) follow kebab-case but no casing convention is declared in specs/system.md (file absent)."}
  BE-RETRY-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-RETRY-003: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#`write-session`: own file plus default"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#One session core, four callers"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#One session core, four callers"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Identity"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#One session core, four callers"}
blocking: false
dispositions:
  fixed: 1
  deferred: 1
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — plan.md says the lock poll's jitter comes from the standard library's randomly seeded hasher, with no source
    outcome: waived
    reason: a claim about the Rust standard library, not about the system 062 changes, whose grounding needs a documentation fetch this run did not make; the jitter's behavior is held by the session-lock-backoff mutants, not by this sentence
    decided-at: 2026-11-10T02:08:31Z
    decided-by: rule
  - key: "rule-assessment — BE-INPUT-004: Session files open under a sanitized identity and a fixed default name with no canonical containment (validate_contained); the plan itself states the gap, deferred to 022 task 34."
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    task: "34"
    reason: the owner deferred it (2026-11-14) to 022, whose plan's Path arguments section is the containment contract and leaves the session files out of the sites that open a path; task 34 routes every open through validate_contained
    decided-at: 2026-11-14T17:38:26Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-REPR-002: New tool fields (seen-peers, last-used, session-identity) follow kebab-case but no casing convention is declared in specs/system.md (file absent)."
    outcome: waived
    reason: "the new fields follow one convention, kebab-case, and this project declares none: it has no specs/system.md"
    decided-at: 2026-11-14T17:38:26Z
    decided-by: rule
  - key: grounding — spec.md Motivation states which commands wrote the session file as a side effect (specify, groom, amend's scenario route, fold, consolidate) with no file-and-symbol citation
    outcome: waived
    reason: history motivating the behavior this spec replaces; it governs no behavior, and the command sources are the authority for what each writes
    decided-at: 2026-11-14T17:38:26Z
    decided-by: rule
---

# Analysis — 062-concurrent-session-targets

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 2 unexamined target(s). Dispositions: 1 fixed, 1 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-REPR-002: New tool fields (seen-peers, last-used, session-identity) follow kebab-case but no casing convention is declared in specs/system.md (file absent). — `specs/062-concurrent-session-targets/data-model.md` — **waived**: the new fields follow one convention, kebab-case, and this project declares none: it has no specs/system.md (decided by the rule)
- grounding — plan.md says the lock poll's jitter comes from the standard library's randomly seeded hasher, with no source — `specs/062-concurrent-session-targets/plan.md` — **waived**: a claim about the Rust standard library, not about the system 062 changes, whose grounding needs a documentation fetch this run did not make; the jitter's behavior is held by the session-lock-backoff mutants, not by this sentence
- grounding — spec.md Motivation states which commands wrote the session file as a side effect (specify, groom, amend's scenario route, fold, consolidate) with no file-and-symbol citation — `specs/062-concurrent-session-targets/spec.md` — **waived**: history motivating the behavior this spec replaces; it governs no behavior, and the command sources are the authority for what each writes (decided by the rule)

## Deferred to other specs

- rule-assessment — BE-INPUT-004: Session files open under a sanitized identity and a fixed default name with no canonical containment (validate_contained); the plan itself states the gap, deferred to 022 task 34. — `specs/062-concurrent-session-targets/plan.md` — **deferred** to `specs/022-deterministic-runtime/tasks.md` task 34

## Contradictions

*None.*

## Unexamined targets

- root-absent: 2

## Fixed in this run

- plan-consistency — plan.md Technical Decision 'Command files' (line 310) states what changes but carries no rationale of its own — `specs/062-concurrent-session-targets/plan.md` — **fixed**
