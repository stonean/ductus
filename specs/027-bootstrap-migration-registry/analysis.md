---
spec: 027-bootstrap-migration-registry
last-run: 2026-11-19T02:35:13Z
analyzed-against: 4230cbc29a88bfbfb061e1399a8c38fd73cb8f7f
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 17
analyzed-digest:
  plan.md: 0327d239e2bea84bd7db35cb46b8165485d64f4b034cdc8f67a4a6cbd84df18a
  review.md: 84daf5622aecd7281064d059feb2b58be02c8f20bd524de869353fa9bc5fa207
  scenarios/a-first-adoption-runs-no-migration.md: 72dea9489a50c0db0b1f95b2e61a0ccaa499495a4ef1e4cadde8a5a826c48bce
  scenarios/a-projects-own-constitution-is-not-relocated.md: f8abe06d0c71fb56767340d373cb44f2c1d401ab309997307bf6a578c760cd82
  scenarios/migration-chain-reference-integrity.md: cd416ac0962c23dd9e3a675208b6bee5f522544ecf4f788770cd59100f1edee6
  scenarios/migrations-apply-once-per-repo.md: ecff31c3d2c1a80ad8c3ff8390033bc704cc7cee57428322934f6677243003c2
  scenarios/seed-files-keep-no-literal-placeholder.md: 58bbb1208e1c7b11f873f00c193bfdc18f9c9664e72042b59c90767c440ca43a
  spec.md: b28993bd95ff82bdaac82e12e0bffba0de8cacb07cb67ed9ba54fcee4fc76a2c
  tasks.md: bc141b9b62663b3c63e17ec035ad90539b5808eff5a830f4c4b1e4eb39f8c589
unexamined-by-reason:
  never-existed: 2
  not-a-live-claim: 8
  root-absent: 6
  ships-to-adopter: 1
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
rule-verdicts:
  BE-INPUT-001: {verdict: addressed, cite: "spec.md#Bootstrap Loop"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Edge cases and failure modes"}
  BE-INPUT-004: {verdict: addressed, cite: "spec.md#Bootstrap Loop"}
  BE-INPUT-008: {verdict: addressed, cite: "spec.md#Resolved Questions"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#`.ductus/config.toml` `[migrations]` section"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-008: {verdict: addressed, cite: "spec.md#Edge cases and failure modes"}
  CFG-ENV-012: {verdict: addressed, cite: "spec.md#Adopter State"}
blocking: false
dispositions:
  fixed: 17
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: plan-record — plan.md §Known Limitations is outside the design record
    outcome: waived
    reason: "deliberately part of this plan's design: it records the v1 limits the registry shipped with (uniform back-fill sunsets, discipline-only removal coverage and CHANGELOG append, a loosely specified procedure-file shape), the case analyze's Plan record section names as a discard (operator decision, 2026-10-16)"
    decided-at: 2026-10-16T02:24:44Z
    decided-by: andy@stone.dev
---

# Analysis — 027-bootstrap-migration-registry

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 17 unexamined target(s). Dispositions: 17 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- plan-record — plan.md §Known Limitations is outside the design record — `specs/027-bootstrap-migration-registry/plan.md` — **waived**: deliberately part of this plan's design: it records the v1 limits the registry shipped with (uniform back-fill sunsets, discipline-only removal coverage and CHANGELOG append, a loosely specified procedure-file shape), the case analyze's Plan record section names as a discard (operator decision, 2026-10-16)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 2
- not-a-live-claim: 8
- root-absent: 6
- ships-to-adopter: 1

## Fixed in this run

- rule-assessment — BE-INPUT-004: the Bootstrap Loop's guard named only a slug allowlist and an exact-path match for the path it turns a registry `id` into, and pointed at no containment mechanism; step 1 now points at `extract-archive`'s, which extracts no link entry — `specs/027-bootstrap-migration-registry/spec.md` — **fixed** (decided by the rule)
- plan-consistency — The Bootstrap loop placement decision said where the loop sits and gave no reason; it now says why, citing `framework/bootstrap/ductus.md` §Instructions (§Plan consistency (blocking if plan exists)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- plan-consistency — The CHANGELOG.md seed format decision gave the seed text and its ordering with no reason, describing a seed and a most-recent-first order the file no longer holds; it now states the file's shape as it stands, with its reason (§Plan consistency (blocking if plan exists)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The DSL trade-off said five of six existing migrations have twists, describing migrations since archived, with no source; it now names each back-filled migration's twist from its recipe in `CHANGELOG.md` (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — Known Limitations said no non-comment line under `runtime/src` names `framework/migrations/`, a measurement with no command; it now gives the `grep` and its date (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — Known Limitations said `c061c1f0` archived five entries each with `sunset_after` 0.10.0, though `workflows-sunset` had 0.25.0; it now gives each, citing the `CHANGELOG.md` headings and `git show c061c1f0:version` (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The note after Affected Files said the pre-commit hook regenerates a README spec-status table, which no generator or hook does now; the clause is gone (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The `apply-migrations` trade-off said a primitive would carry user-interaction protocol design that exists nowhere else in the runtime, citing nothing, though `gate-confirm` surfaces prompts; it now says a primitive reaches the operator only through a gate the host routes (`ductus gate-confirm --help`) (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The git-archaeology trade-off said a history check breaks the current-state-only contract every other family follows, though 10d and other families read history; that reason is gone and 10d's one-file history read is named (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The Overview and the procedure-file shape said every procedure file's first step is a target-presence check, though nine of the fifteen open otherwise; both now state what each file's first step shows and point at AC21 (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The SemVer decision said the loop's filter is SemVer compare's only consumer, though Family 10's 10d and 10e compare versions; it now scopes the claim to the bootstrap and names 10d and 10e (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The `target_paths` field said Family 10 holds each path absent from `framework/`, though 10b checks only `framework/`-prefixed ones; it now says so, citing the script's header (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The Family 10 wiring said `/audit` invokes `run-all.sh` via `run-generator`, citing nothing; it now cites `framework/commands/audit.md` step 1 (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The out-of-scope diff-check bullet said the check would couple `/audit` to git history shape, though `/audit` already reads history in 10d; it now says the check itself would be coupled (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/spec.md` — **fixed** (decided by the rule)
- anchor — The `apply-migrations` trade-off's §Pre-run Migrations reference named no heading of `plan.md` (Family 39); it now names `framework/bootstrap/ductus-procedure.md` — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
- grounding — The spec's Out of scope list, its `/audit` detection Resolved Question and the plan's Known Limitations said the removal diff-check was captured as a follow-on spec or scenario, though none exists; by the owner's decision all three now say it is out of scope with no follow-on (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/spec.md` — **fixed**
- grounding — The git-archaeology trade-off still called the pre-commit diff hook a captured out-of-scope follow-on after the owner's decision; it now says the hook is out of scope with no follow-on (§Grounding (advisory)). — `specs/027-bootstrap-migration-registry/plan.md` — **fixed** (decided by the rule)
