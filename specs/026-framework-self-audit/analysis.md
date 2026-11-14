---
spec: 026-framework-self-audit
last-run: 2026-11-14T02:33:55Z
analyzed-against: c9da403a5ecb11fc6f0c6c4d71cecf40dbbeb1ca
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 43
analyzed-digest:
  plan.md: 4cd19df4749a98205f5445b5235979ac0880bb04e91b8a1cdda7968bbf4256ff
  review.md: da760eb3a64bfd43d2905410436eb92c269cbdc987a4a139841ab8b334623a27
  scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
  scenarios/audit-script-refactors.md: 4d4a93fd968898006fa4780beb8088e48616ed1ceae54574ac38a3a02ad9583b
  scenarios/families-follow-the-command-reference-split.md: 2827003a57acd578cd14add059be8c7fcdee0ae80389acad64f282d8c9ffc1ea
  scenarios/family-10-migration-coverage.md: d2071390e33f8598daed4a07d0e08a52d8569f25b69fa4bb260c85ae77b3d515
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: 296f55313954ec984b5260945b92639d15b8458f707f90e94e0bd1bbc0f052c0
  scenarios/family-22-adopter-shell-behavior.md: 3959304113aa956aa8cb70a78d703b7145521ea97d7f6e8f2f33c803c1a1fce5
  scenarios/family-23-sweep-target-manifest-parity.md: fabc89386b8c36156f41c09b5ae28e5f0cd3130c08abc284fa90dcd37507ebb1
  scenarios/family-24-rename-sweep-residue.md: 41d6c829f0851a3caab11650d24b7100992667969ee3f21454141b482819e132
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 60627b24354610f3e0f819abe7d0603faa9644c27e8cee5163a452804bb94b6c
  scenarios/family-27-done-spec-unchecked-criteria.md: d164ba1fedaababcb88f5a4062b150ae8bd7e017699d7ac6d310292a25489bf9
  scenarios/family-28-audit-family-registry-parity.md: bf80f1d64718deb87a7dec79c69e9248e8c1ab4a41e65c638488fdd9fcd5e529
  scenarios/family-34-step-reference-integrity.md: 4175e05f88fcec7a03a63150bf652a3dd352a4e2d48b228a51a3839c0d867656
  scenarios/family-35-manifest-destination-links.md: 8e94dce4172e2b326612da806327af31777d99b8c6ce89642c9d10a98dc148b3
  scenarios/family-36-self-url-resolution.md: 23716b0f5c3810890bab089c65258008842869e35616a71b86b663ed231bde18
  scenarios/family-39-shipped-anchor-resolution.md: c00188dfd875ce2aeda794be6e023221173dd712c949b58f60ac34a80a5e17ca
  scenarios/family-40-deprecated-rules-are-removed-on-time.md: b0f97a19013851cdabf3172954b6c0a8da12ccc06a46f0183c93bbf50f0227bc
  scenarios/host-namespace-parity.md: 5d20df476bcd94b0f843e3c44919b19215f5dcc9ddba18af435f2b46e5b194ed
  scenarios/link-check-consolidation.md: edaa15335c49241c5565a0b4063a3f59a5233b5b4a0c6362cbe9f4356b82812f
  scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
  spec.md: 1a648a0d7fa2745b48000778efe8eafba879fcef6ff3cf8162cd96ddc2f69cfb
  tasks.md: a89dc6842ebea77e560254167bc11d587a541b544483539843bd46562010c690
unexamined-by-reason:
  never-existed: 4
  not-a-live-claim: 24
  root-absent: 10
  ships-to-adopter: 5
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
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#CI integration", message: "Plan adds audit steps to framework-checks.yml and runtime-release.yml (CI configuration) but commits to no automated vulnerability scan on dependency-set changes."}
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: Plan adds audit steps to framework-checks.yml and runtime-release.yml (CI configuration) but commits to no automated vulnerability scan on dependency-set changes."
    outcome: waived
    reason: "advisory and no chore: 026's CI work adds `bash scripts/audit/run-all.sh` steps (framework-checks.yml step (h), runtime-release.yml's audit job) that run repo-local scripts and change no dependency set; cargo audit runs in runtime.yml on every runtime/** change (as 024's analysis waived the same finding)"
    decided-at: 2026-11-14T02:33:55Z
    decided-by: rule
  - key: scenario-consistency — family-26-broken-relative-links Behavior still describes the family's own code-span stripping and line-by-line fence toggling, while link-check-consolidation and family-35 record Family 26 as an entry point delegating to check-corpus-links
    outcome: waived
    reason: the requirements the Behavior states, code-span stripping and line-by-line fence toggling, still hold; since link-check-consolidation the family meets them by delegating to check-corpus-links, so the requirement is unchanged and only its implementer moved
    decided-at: 2026-11-14T02:33:55Z
    decided-by: rule
---

# Analysis — 026-framework-self-audit

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 43 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: Plan adds audit steps to framework-checks.yml and runtime-release.yml (CI configuration) but commits to no automated vulnerability scan on dependency-set changes. — `specs/026-framework-self-audit/plan.md` — **waived**: advisory and no chore: 026's CI work adds `bash scripts/audit/run-all.sh` steps (framework-checks.yml step (h), runtime-release.yml's audit job) that run repo-local scripts and change no dependency set; cargo audit runs in runtime.yml on every runtime/** change (as 024's analysis waived the same finding) (decided by the rule)
- scenario-consistency — family-26-broken-relative-links Behavior still describes the family's own code-span stripping and line-by-line fence toggling, while link-check-consolidation and family-35 record Family 26 as an entry point delegating to check-corpus-links — `specs/026-framework-self-audit/scenarios/family-26-broken-relative-links.md` — **waived**: the requirements the Behavior states, code-span stripping and line-by-line fence toggling, still hold; since link-check-consolidation the family meets them by delegating to check-corpus-links, so the requirement is unchanged and only its implementer moved (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 4
- not-a-live-claim: 24
- root-absent: 10
- ships-to-adopter: 5

## Fixed in this run

- plan-consistency — Technical Decision 'Per-family script designs' states the shared contract (stdout row format, exit 0/1, read-only, idempotent) and the Family 1, 2, 4 and 5 designs with no rationale; only Families 6, 7 and 8 and the sibling decisions carry a reason — `specs/026-framework-self-audit/plan.md` — **fixed**
- scenario-consistency — audit-script-refactors Edge Cases still describe the sketched signature (`audit_emit`, a `$(dirname "$0")/lib.sh` source path) that its own Behavior says did not ship (`emit LOCATION MESSAGE FIX`, `${BASH_SOURCE[0]}`) — `specs/026-framework-self-audit/scenarios/audit-script-refactors.md` — **fixed** (decided by the rule)
