---
spec: 018-adopter-owned-pre-commit
last-run: 2026-11-19T14:14:00Z
analyzed-against: 065c45f502059141c75dcfc3c0046deabbf7af46
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 10
analyzed-digest:
  plan.md: 92525d4c58b3ad74f02e9ab6555612fdec2be9917676f629ec198e1e3f652fa2
  review.md: 27a671ba920d9847929f1b45f69e4c5fb2821394d23dc96c7e3fc7a09af28024
  spec.md: 4a9f39760ca61bea68c48a97cca52d70ac7092499c0b08714d6e20b300d368cd
  tasks.md: b4c1325cbe2360c0d24fd2dcafd5e456db6addc7b978369f8069dcc6cf5e6151
unexamined-by-reason:
  never-existed: 1
  not-a-live-claim: 7
  ships-to-adopter: 2
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
blocking: false
dispositions:
  fixed: 8
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 018-adopter-owned-pre-commit

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 10 unexamined target(s). Dispositions: 8 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 7
- ships-to-adopter: 2

## Fixed in this run

- grounding — Edge Cases, executable bits: asserts `update` and `create` strategies write file content but do not chmod, a claim about the manifest's behavior, with no citation to where the strategies are defined. — `specs/018-adopter-owned-pre-commit/spec.md` — **fixed** (decided by the rule)
- grounding — Edge Cases, outer file deleted: asserts that with `core.hooksPath` set git silently skips a missing pre-commit hook, an external-system behavior stated as fact with no citation or hedge. — `specs/018-adopter-owned-pre-commit/spec.md` — **fixed** (decided by the rule)
- grounding — Edge Cases, signpost-link pollution: 'Empirical verification' names six done specs (000, 003, 006, 007, 008, 011) as carrying signpost blockquotes that were rewritten into their frontmatter, with no command or query cited. — `specs/018-adopter-owned-pre-commit/spec.md` — **fixed** (decided by the rule)
- grounding — Migration: asserts no release shipped the spec-017 hook on its own (v0.1.0 and v0.2.0 predate it, the first later tag carries the split), with no tag or git-history citation. — `specs/018-adopter-owned-pre-commit/spec.md` — **fixed** (decided by the rule)
- grounding — Migration: asserts every spec-017 copy carries the sentinel of the project's earlier name, which no `/ductus` trigger has matched since the rename, with no citation to the trigger or the sentinel text. — `specs/018-adopter-owned-pre-commit/spec.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, install.sh deletion: asserts the standalone reinstall path 'was never used in practice' and is 'verifiable by grep through any adopter project's history', with no query cited, and adopter trees are not reachable here. — `specs/018-adopter-owned-pre-commit/plan.md` — **fixed** (decided by the rule)
- grounding — Verification approach and Affected Files: asserts the repo's CI runs the generators, fails on a changed tree and does not boot a fresh adopter project, with no workflow file cited. — `specs/018-adopter-owned-pre-commit/plan.md` — **fixed** (decided by the rule)
- plan-consistency — Technical Decisions, 'Manual integration snippet path change' states the change from the outer to the inner hook and its two callsites but gives no rationale for the change. — `specs/018-adopter-owned-pre-commit/plan.md` — **fixed** (decided by the rule)
