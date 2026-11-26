---
spec: 017-derive-dont-ask
last-run: 2026-11-26T04:23:54Z
analyzed-against: 49a075e0f1fb558bfa56a8249d1b05bcc729c92b
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 32
analyzed-digest:
  data-model.md: dadd28edc909a348e9b5042e89d03c3f60f1d31c8de3eeb1ee0ca6c9a835574d
  plan.md: 7cb4fb66cc7b174e69a5c798bda66e2fa32b2e64eb59a49a4ba1df5ac7b032a8
  review.md: 5ff91148098cdc425b13c0fb51e418aec726d72d8ffba703a6331f104b3a3210
  scenarios/a-commit-stages-only-its-own-generated-files.md: 8626b701ad07371d0907c6a1a3a1ad4ee682c4d0d1622b3f503a76ae2bc189dc
  scenarios/a-path-list-commit-leaves-the-shared-index-current.md: c1bb7078add58084d19293a3db3f2b5904ad876b1611c3ca73033532b69e054a
  scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md: 6c52e86d521006fe5655e4a4fd704efad69ad5d9a6eec3bf70701b7962cdecf9
  scenarios/a-throwaway-credential-is-not-a-secret.md: e2167e26e43b1d9f1d6d0a6fb47928594a4c9b41d25d3b55bea4988c1a1e8fa5
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 5be65fb004623287403f9f61b9d13fe8a7b8079d623ff9cd70c85aebf6d0ee65
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 8a6671d9417b0183141f771c438c8a7d985b69b82b23e476e3bd2ec602a0e963
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: f294181444f3bd340d48df644da28f5181e8301531d6ec87276a8894a0961cf5
  scenarios/tracked-specs-not-worktree.md: 8ad8da4efcd159e76d359d978541f585f2e53d63876d218a0c27b15068b19ace
  spec.md: d56a54aec5cd2eb8f9bb72d6b49a40f5f8676d3e7a6194821a93f816d10a8f91
  tasks.md: d2089033dc26f64e556806f26dfbf1484e4d98251e12ac76ebba27360b78d06f
unexamined-by-reason:
  anchor-target-unjudgeable: 5
  never-existed: 1
  no-readable-state: 1
  not-a-live-claim: 20
  root-absent: 2
  ships-to-adopter: 3
rule-digest:
  accessibility-frontend.md: 20fb956e35bb749f061090adf631329bafe3e37662a4e47b0076b697784bbf50
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: c9d90cf6bfa6e4f5492e6d1a49e073fbee467ba08c7788bf6c7224ae8bd5d411
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 9a200916d11a5597a6860d1538840bf7d9aa573bddce953a0bef604379d1451b
section-digest:
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#017 — Derive, Don't Ask Data Model": 2c6317957e38561d
  "data-model.md#CFG-CONST namespace (Constants category)": a5c2e06e152b59b0
  "data-model.md#CFG-ENV namespace (Environment and configuration values category)": 6ef1c6e176c9bcda
  "data-model.md#CFG-FLAG namespace (Feature flags category)": 520ccfadfc25087a
  "data-model.md#Category abbreviations": a06ad230f0ebc999
  "data-model.md#Configuration rule ID format": d112147fccca8db1
  "data-model.md#Configuration rule entry fields": e09fd3904cbcbfbb
  "data-model.md#Configuration rule file structure": 36aed48bcedab772
  "data-model.md#File extension and shebang conventions": 694db381c77b3b8a
  "data-model.md#Frontmatter schema (after this spec)": fa91583bfb41120e
  "data-model.md#Hook sentinel comment": dacd08e8a38939eb
  "data-model.md#ID stability invariants": 65c234b9e44055c5
  "data-model.md#Initial marker names": ef68713ffe28f31c
  "data-model.md#Marker-comment convention for generated content blocks": 459e75e05d7d1efd
  "data-model.md#Open-schema rule (unchanged)": 07ab83b7b52b49ef
  "data-model.md#Other artifact files": bf319ccaf56c6490
  "data-model.md#Rule set": a5dcf8635362bf24
  "data-model.md#Scenario files (`scenarios/{slug}.md`)": 3d5e9a37fa1991d3
  "data-model.md#Severity classification": cf880f82df419966
  "data-model.md#Spec files (`spec.md`, `spec-and-plan.md`)": 70075d3cb14b07ca
  "data-model.md#Verification phrasing": 346b6d8d61c2f3df
  "plan.md": e3b0c44298fc1c14
  "plan.md#017 — Derive, Don't Ask Plan": a39d6ca35f2a7bf6
  "plan.md#Adopter hook surface and `/ductus` integration": ddb3f72b0dc34e80
  "plan.md#Affected Files": 5f1e34e8f7c47768
  "plan.md#Body inline links are authoritative for `dependencies`": 7937c4ba38fd90c9
  "plan.md#Bootstrap installer": 84a26620a426c7fb
  "plan.md#CI": a8eb60aad3555e6a
  "plan.md#CI safety net": 2e27c68e0c16ec9e
  "plan.md#Commands": 7dc46a1c38ed77ee
  "plan.md#Configuration rule file": d79d05f160a5baff
  "plan.md#Constitution": 36d3944844c3f723
  "plan.md#Data Model": c4b0c723456bb3e6
  "plan.md#Existing dogfood spec migration": b73f06c2cdd24d84
  "plan.md#Generated artifacts use marker comments": 6b46b691504a0922
  "plan.md#Generators (new)": a5f0b4225d9d754e
  "plan.md#Hooks (new)": 547c62617c8753a4
  "plan.md#Migration of existing dogfood specs": 09086f87801e22cf
  "plan.md#New rule file": b781edd5c093eba9
  "plan.md#Overview": 1167c34e06d5b136
  "plan.md#Pre-commit hook architecture": d8ebdbda9c62354b
  "plan.md#Project root files": 2f33f05372bd1fd1
  "plan.md#Single canonical constitution at `framework/constitution.md`": 93ffab203e22d468
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Templates": 2a032aebe9962a65
  "plan.md#This spec's own artifacts": c0c2b64c8c003e87
  "plan.md#Trade-offs": 76784365de67ccfe
  "plan.md#`--fix` mode removed entirely from `/validate`": 6e478c3f8cf0bafb
  "plan.md#`/validate` schema and check changes": d0f8946bad69f295
  "scenarios/a-commit-stages-only-its-own-generated-files.md": f6aef6ffc7ecfb80
  "scenarios/a-commit-stages-only-its-own-generated-files.md#A-commit-stages-only-its-own-generated-files": 8de3eac7efd1dff1
  "scenarios/a-commit-stages-only-its-own-generated-files.md#Behavior": 04567c422aa07ea5
  "scenarios/a-commit-stages-only-its-own-generated-files.md#Context": 51fdd7ca38807beb
  "scenarios/a-commit-stages-only-its-own-generated-files.md#Edge Cases": f07d6380705677ca
  "scenarios/a-commit-stages-only-its-own-generated-files.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-commit-stages-only-its-own-generated-files.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/a-path-list-commit-leaves-the-shared-index-current.md": f6aef6ffc7ecfb80
  "scenarios/a-path-list-commit-leaves-the-shared-index-current.md#A-path-list-commit-leaves-the-shared-index-current": 94651e8f62690f0e
  "scenarios/a-path-list-commit-leaves-the-shared-index-current.md#Behavior": cbf1e9730c4406d5
  "scenarios/a-path-list-commit-leaves-the-shared-index-current.md#Context": 48724f0faecdb5b5
  "scenarios/a-path-list-commit-leaves-the-shared-index-current.md#Edge Cases": 2bbb0ecc92bf4334
  "scenarios/a-path-list-commit-leaves-the-shared-index-current.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-path-list-commit-leaves-the-shared-index-current.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md": 1843a1f8bb3f6abe
  "scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md#A-secret-at-rest-is-flagged-in-every-project-shape": 58b9d88d3cf6684d
  "scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md#Behavior": b5bbdd540e03030f
  "scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md#Context": c17d406b43a2c72a
  "scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md#Edge Cases": c768db4607abfaaa
  "scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/a-throwaway-credential-is-not-a-secret.md": 1843a1f8bb3f6abe
  "scenarios/a-throwaway-credential-is-not-a-secret.md#A-throwaway-credential-is-not-a-secret": 7fb5f0c1e1058c4c
  "scenarios/a-throwaway-credential-is-not-a-secret.md#Behavior": 6741a02395d7b462
  "scenarios/a-throwaway-credential-is-not-a-secret.md#Context": 92a78e034c51bd00
  "scenarios/a-throwaway-credential-is-not-a-secret.md#Edge Cases": 1c3f6c57d6fa8e85
  "scenarios/a-throwaway-credential-is-not-a-secret.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-throwaway-credential-is-not-a-secret.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/detect-dependency-cycles.md": f6aef6ffc7ecfb80
  "scenarios/detect-dependency-cycles.md#Behavior": 82692ff9fdc088a0
  "scenarios/detect-dependency-cycles.md#Context": 5a849c7fb243590c
  "scenarios/detect-dependency-cycles.md#Detect-dependency-cycles": 00e1ffe790522490
  "scenarios/detect-dependency-cycles.md#Edge Cases": 8f1b2fcb7eb6867c
  "scenarios/detect-dependency-cycles.md#Open Questions": e4bf715f08f38a86
  "scenarios/detect-dependency-cycles.md#Resolved Questions": 5f7d1d0c9b57778f
  "scenarios/generator-sync-claim-honesty.md": 50d3794a575f214e
  "scenarios/generator-sync-claim-honesty.md#Behavior": 9beb021600cb8d91
  "scenarios/generator-sync-claim-honesty.md#Context": 99daa3e7fac66c2b
  "scenarios/generator-sync-claim-honesty.md#Edge Cases": 33e855e0d9f911d2
  "scenarios/generator-sync-claim-honesty.md#Generator-sync-claim-honesty": f453de84ad80df01
  "scenarios/generator-sync-claim-honesty.md#Open Questions": 9fc5979d590cdb97
  "scenarios/generator-sync-claim-honesty.md#Outcome of that assessment (2026-08-16)": 8fc0c1bee6f1392d
  "scenarios/generator-sync-claim-honesty.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/ranged-and-inventoried-config-values-from-any-source.md": f4036d4aa3763d17
  "scenarios/ranged-and-inventoried-config-values-from-any-source.md#Behavior": e8999e59207720bf
  "scenarios/ranged-and-inventoried-config-values-from-any-source.md#Context": ae0ee68327667312
  "scenarios/ranged-and-inventoried-config-values-from-any-source.md#Edge Cases": af97d58f096041f0
  "scenarios/ranged-and-inventoried-config-values-from-any-source.md#Open Questions": 20edda8881516e4e
  "scenarios/ranged-and-inventoried-config-values-from-any-source.md#Ranged-and-inventoried-config-values-from-any-source": 7210674d2d0a37b4
  "scenarios/ranged-and-inventoried-config-values-from-any-source.md#Resolved Questions": 8ac3a495f46142ff
  "scenarios/required-config-values-from-any-source.md": f4036d4aa3763d17
  "scenarios/required-config-values-from-any-source.md#Behavior": b4be97a6c85fb89c
  "scenarios/required-config-values-from-any-source.md#Context": 80331493c70d97c8
  "scenarios/required-config-values-from-any-source.md#Edge Cases": 5abafb6b8219f0c2
  "scenarios/required-config-values-from-any-source.md#Open Questions": 20edda8881516e4e
  "scenarios/required-config-values-from-any-source.md#Required-config-values-from-any-source": b0d7be533d05310f
  "scenarios/required-config-values-from-any-source.md#Resolved Questions": ebcb454fbf8c3a83
  "scenarios/skip-prose-cross-references.md": f6aef6ffc7ecfb80
  "scenarios/skip-prose-cross-references.md#Behavior": d3c857e3b48409a5
  "scenarios/skip-prose-cross-references.md#Context": 53f37185ae518576
  "scenarios/skip-prose-cross-references.md#Edge Cases": e8d5b3581d71d9e5
  "scenarios/skip-prose-cross-references.md#Open Questions": e4bf715f08f38a86
  "scenarios/skip-prose-cross-references.md#Resolved Questions": 8d7e1a0b2603a6db
  "scenarios/skip-prose-cross-references.md#Skip-prose-cross-references": eb88101f0079e246
  "scenarios/the-adopter-ci-template-can-run.md": f6aef6ffc7ecfb80
  "scenarios/the-adopter-ci-template-can-run.md#Behavior": 486c630b8c78ed9c
  "scenarios/the-adopter-ci-template-can-run.md#Context": 91b19561da78cb60
  "scenarios/the-adopter-ci-template-can-run.md#Edge Cases": 2dcd0b2b028d7eec
  "scenarios/the-adopter-ci-template-can-run.md#Open Questions": 545eadd502d01a7c
  "scenarios/the-adopter-ci-template-can-run.md#Resolved Questions": 8649438b888b7bec
  "scenarios/the-adopter-ci-template-can-run.md#The-adopter-ci-template-can-run": 0fb310dc470789d8
  "scenarios/tracked-specs-not-worktree.md": f6aef6ffc7ecfb80
  "scenarios/tracked-specs-not-worktree.md#Behavior": 48a4ba59ee3115e4
  "scenarios/tracked-specs-not-worktree.md#Context": ba8cab926093eb02
  "scenarios/tracked-specs-not-worktree.md#Edge Cases": 6434adecd0fc4eca
  "scenarios/tracked-specs-not-worktree.md#Tracked-specs-not-worktree": e546e148407924f0
  "spec.md": b8337bc5f3ed469f
  "spec.md#017 — Derive, Don't Ask": 3138516abc4397b0
  "spec.md#Acceptance Criteria": 39e696f817857e27
  "spec.md#Adopter projects' pre-commit hook": a30ed1b925572552
  "spec.md#CI safety net": 17a4ec58d1d9040d
  "spec.md#Delete entirely (no viable derivation, not load-bearing)": a67ef3c927a55383
  "spec.md#Derive (replace the input with a scaffold/generator/hook)": dd47dc988e36ca31
  "spec.md#Generators and Hooks": c41cee3180164969
  "spec.md#Migration": 74f3c87811fd33e6
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Promote to rule file (the framework's own enforceable-rule mechanism)": 2f57ef42b08e7bea
  "spec.md#Replace with structural enforcement (Bucket 3 — real designs)": 24418c694f5dd0d4
  "spec.md#Resolved Questions": 6d408fe9543bdbb8
  "spec.md#Schema Changes": 558d2e5e1f4e57db
  "spec.md#Violation Inventory": 0ea96d81edff07d5
  "spec.md#ductus repo's pre-commit hook": 90d7e170766dcbb4
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Behavior"}
  BE-DATA-001: {verdict: addressed, cite: "plan.md#CI safety net"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#CI safety net"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#Pre-commit hook architecture"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Behavior"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#Pre-commit hook architecture"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#Pre-commit hook architecture"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#CI safety net"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#CI safety net"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#CI safety net"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Behavior"}
  QUAL-GROUND-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Outcome of that assessment (2026-08-16)"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Pre-commit hook architecture"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#CI safety net"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-006: plan.md introduces a CI workflow and a shipped CI template without committing to immutable pinning of their actions or to an update process for the pins"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-DEPS-007: plan.md's CI configuration commits to no automated dependency scan"
    outcome: waived
    reason: 017 introduces no manifest or lockfile and its workflows check generator drift, not dependency changes; the repository's per-change scan is committed in 022's plan, cargo audit in runtime.yml
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-DEPS-006: plan.md's CI configuration commits to no frontend dependency scan"
    outcome: waived
    reason: ductus has no frontend dependency set for a scan to cover; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-BUNDLE-003: plan.md's CI configuration includes no automated bundle-budget check that fails on exceedance"
    outcome: waived
    reason: ductus ships no frontend bundle for a budget check to measure; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T20:14:13Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-A11YTEST-003: plan.md's CI configuration includes no automated accessibility check on each change"
    outcome: waived
    reason: ductus ships no frontend interface for an accessibility check to exercise; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T20:14:13Z
    decided-by: andy@stone.dev
  - key: "scenario-consistency — ranged-and-inventoried-config-values-from-any-source: the Behavior section describes process ('each open question below is answered by the owner...') and states no requirement; the behavior lives only in Resolved Questions"
    outcome: waived
    reason: "advisory and no chore: the contract is carried by the scenario's Resolved Questions (CFG-ENV-005 widened to any source; CFG-ENV-002 widened to configuration-file keys, flags left to help output) and by data-model.md's Rule set, so no requirement is lost; rewriting a scenario's Behavior is a requirement edit, not a mechanical chore"
    decided-at: 2026-11-14T13:41:18Z
    decided-by: rule
  - key: rule-citation — Spec at specs/017-derive-dont-ask/spec.md references deprecated rule BE-DEPS-006; targeted for removal in 0.62.0
    outcome: waived
    reason: "advisory and no chore: 017's spec records SEC-DEPS-001 taking over the deprecated BE-DEPS-006 beside 077's obligation, a record of the move that keeps the old ID by design, as 008's analysis waived the same citation; its treatment once 0.62.0 removes the rule is 067's scenario a-record-cites-a-removed-rule"
    decided-at: 2026-11-14T13:41:18Z
    decided-by: rule
  - key: "grounding — Resolved Questions, Q4: states that prose references to other specs are common and would flood the proposal with false positives, with nothing cited (waived in round 1 as decision reasoning; still unsourced)."
    outcome: waived
    reason: "A Resolved Question's premise, recorded with the decision it was taken on: the reasoning for treating bare slug mentions as commentary, not a claim about the system as it stands. Re-deriving how common such mentions are is a measurement, not a mechanical edit."
    decided-at: 2026-11-19T14:13:24Z
    decided-by: rule
  - key: "grounding — Resolved Questions, Q4: states that 'always write proper links' is a convention authors already follow, with no source cited (waived in round 1; still unsourced)."
    outcome: waived
    reason: "A Resolved Question's premise, recorded with the decision it was taken on: the reasoning for scanning inline links only, not a claim about the system as it stands."
    decided-at: 2026-11-19T14:13:24Z
    decided-by: rule
  - key: "grounding — Resolved Questions, Q3: states the first-commit diff window is equivalent to the planned-to-in-progress transition for clean pipelines, with no command or artifact cited (waived in round 1; still unsourced)."
    outcome: waived
    reason: "A Resolved Question's reasoning, which carries its argument inline: pre-implement work touches only the spec directory, which the filter excludes. The window it describes is `derive-boundary`'s, whose compiled description states it."
    decided-at: 2026-11-19T14:13:24Z
    decided-by: rule
  - key: "grounding — Trade-offs, 'Symmetric hooks': states drift between commits is a real failure mode, with no observed instance or artifact cited (waived in round 1; still unsourced)."
    outcome: waived
    reason: A trade-off's rationale naming its failure scenario (a teammate reads stale dependencies from an out-of-date branch without running a ductus command); it is the reason for the decision, not a claim about the system as it stands.
    decided-at: 2026-11-19T14:13:24Z
    decided-by: rule
  - key: grounding — §Migration states as fact that 'a meaningful body edit reopened a spec through the back-edge, so a bulk retroactive migration would have reopened every one of them', citing no commit, command or artifact and carrying no assumption marker.
    outcome: waived
    reason: "advisory and no chore: a decision's premise recorded with the decision, stating the reopen rule in force when 017 landed (a meaningful body edit reopened a done spec, since replaced by spec 084's owed-code rule), not a claim about the system as it stands; re-sourcing it is history research, not a mechanical edit"
    decided-at: 2026-11-26T04:23:54Z
    decided-by: rule
  - key: grounding — §Pre-commit hook architecture states that 'scripts/tests/test-pre-commit-staging.sh holds each case, and each fails under a mutation that removes its behavior' without naming the run or command that showed the mutation failures.
    outcome: waived
    reason: "advisory and no chore: a claim about the test suite whose re-sourcing is a measurement, a mutant run per case, which is research rather than a chore; 017's review of 2026-11-26 ran case J's, L's and M's mutants through HOOK_UNDER_TEST and each turned red"
    decided-at: 2026-11-26T04:23:54Z
    decided-by: rule
---

# Analysis — 017-derive-dont-ask

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 32 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — §Migration states as fact that 'a meaningful body edit reopened a spec through the back-edge, so a bulk retroactive migration would have reopened every one of them', citing no commit, command or artifact and carrying no assumption marker. — `specs/017-derive-dont-ask/spec.md` — **waived**: advisory and no chore: a decision's premise recorded with the decision, stating the reopen rule in force when 017 landed (a meaningful body edit reopened a done spec, since replaced by spec 084's owed-code rule), not a claim about the system as it stands; re-sourcing it is history research, not a mechanical edit (decided by the rule)
- grounding — §Resolved Questions, Q4 states as fact that '"always write proper links" is a markdown convention authors already follow', with no citation or measurement. — `specs/017-derive-dont-ask/spec.md` — **waived**: A Resolved Question's premise, recorded with the decision it was taken on: the reasoning for scanning inline links only, not a claim about the system as it stands. (decided by the rule)
- grounding — §Resolved Questions, Q4 states as fact that prose references to other specs are common and would flood the proposal with false positives, with no citation or measurement. — `specs/017-derive-dont-ask/spec.md` — **waived**: A Resolved Question's premise, recorded with the decision it was taken on: the reasoning for treating bare slug mentions as commentary, not a claim about the system as it stands. Re-deriving how common such mentions are is a measurement, not a mechanical edit. (decided by the rule)
- grounding — §Pre-commit hook architecture states that 'scripts/tests/test-pre-commit-staging.sh holds each case, and each fails under a mutation that removes its behavior' without naming the run or command that showed the mutation failures. — `specs/017-derive-dont-ask/plan.md` — **waived**: advisory and no chore: a claim about the test suite whose re-sourcing is a measurement, a mutant run per case, which is research rather than a chore; 017's review of 2026-11-26 ran case J's, L's and M's mutants through HOOK_UNDER_TEST and each turned red (decided by the rule)
- scenario-consistency — ranged-and-inventoried-config-values-from-any-source: the Behavior section describes process ('each open question below is answered by the owner...') and states no requirement; the behavior lives only in Resolved Questions — `specs/017-derive-dont-ask/scenarios/ranged-and-inventoried-config-values-from-any-source.md` — **waived**: advisory and no chore: the contract is carried by the scenario's Resolved Questions (CFG-ENV-005 widened to any source; CFG-ENV-002 widened to configuration-file keys, flags left to help output) and by data-model.md's Rule set, so no requirement is lost; rewriting a scenario's Behavior is a requirement edit, not a mechanical chore (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 5
- never-existed: 1
- no-readable-state: 1
- not-a-live-claim: 20
- root-absent: 2
- ships-to-adopter: 3

## Fixed in this run

*None.*
