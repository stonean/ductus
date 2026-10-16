---
spec: 067-analyze-and-audit-checks
last-run: 2026-10-16T19:13:57Z
analyzed-against: 7756775c292389a6dffb49a5a9015f73a4e82426
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 50
analyzed-digest:
  data-model.md: 4ed42a5f083d8f4fc453cd4ebd4962e0e4ab734aec59c4e22bab395dbfc74b71
  plan.md: cf9223eac3d1aaeaa8c3ee1f008039bc21287f6d4f6ce194c50d52530bf7494a
  review.md: 84a816c37c49348f69afc5a45458197c85251e3a5794c177ac31cc10b4d48e8d
  scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md: 4057057a18a969d37b8c503f0022b277754d9c41d34f96b443a3dc1e8df7e858
  scenarios/adopter-corpus-link-integrity.md: 7950b5560f15986c7bed5cc134034ed46fb0d2cbc517e4b030eb55ac114da3e9
  scenarios/adopter-generator-promotion.md: 54be234cf50bbed566015a3e35017998fcd916e5ce4e7038007a2fb7a9a7d2e5
  scenarios/analyze-artifact-checks.md: 4375259e5872ad058d9c8169e7b8981ad550a681be03f16e3bc5813ede982dc5
  scenarios/anchor-reference-kinds.md: 27e72e3aa41d7a6eca6fa128d6b86aae518adb9b1212ace06bf5bb6015455a45
  scenarios/argument-hint-needs-a-token.md: 130b949e9ca72ab34b48e43d2179fbed5d21ef111b40213180c72ae41bc28ae7
  scenarios/block-element-scanner.md: 0b96c1dc41bd91f898e535c17c8964ef9da5ccf39411d888d8e244c82afe9c38
  scenarios/body-path-existence-family.md: 87635ccc65f84ee18c0e12c47e8896a77189d944eff9ff220fd43ca9bd46917a
  scenarios/check-artifacts-skipped-targets.md: 6a418c79ad180d47529bc174c398097fb0862360df6ed345008aee5cdcbfb7e2
  scenarios/criterion-adopter-scope-destinations.md: fcc75cb399c447fbb98b03c786e1f104006649f84724f104e6c36eae6094e0ec
  scenarios/criterion-negated-creation-phrasing.md: e0b5baf33ef555c5590aa88334079698b88a13c2e95c2892e1868e132259db74
  scenarios/criterion-non-assertion-phrasings.md: 5626be63a44cf409f82e3f5def18223129fe3dbde20c68d22eb9fe96b7032536
  scenarios/criterion-path-existence-family.md: e90f11630b03867e4fee306c1f1c9f02a0f01320ab88ac2871557754d82fcdbe
  scenarios/derive-references-unstaged-drift-is-reported.md: dc9f33da659dd20c971b5691d46127ba9673af3141cede8b7bc5481e1b9e4da0
  scenarios/derive-unparseable-frontmatter-is-reported.md: 062193d4b54ba56a6b791a086b1b72c6b63b86b6f7bc991bea5fe978a7b2747b
  scenarios/every-agents-generated-copies-are-excluded.md: cfbb050341906864fe67031fe1cc1c8928966221adf2e2346079b8e784d30541
  scenarios/frontmatter-severity-tiers-on-both-sides.md: 517961da7a2118869f494179041a5e1b3dcdf90060828595411c6f35eb386a85
  scenarios/link-adjacent-drift-family.md: 41814ebc48e3744fdb053d09ad024bb9da2bc998d3a4767d3b6a2338b4fd7b44
  scenarios/orphan-check-adopter-authored-references.md: f489b8688ad92e92846bc0c5d74d68239c7478a47aedd954aaac38dbd09c7340
  scenarios/orphaned-reference-check.md: 6c990e4c4668e432381107b3d0bc184204a5888f01004868b4f22d55f9012ee0
  scenarios/orphaned-reference-historical-roots.md: c876b849247585e9deff19b7ec91bbc80ec3ab292849a38a842765cf52ed32a7
  scenarios/per-agent-scaffold-paths-ship-to-adopter.md: b9d60c57c3ef5f4a6fc443cd25dbfd0decf189f47aa5571da036be0c90ea6e19
  scenarios/scenario-consistency-reads-tasks-history.md: 34f4ec8f9142207add588fa357e77dc1a1e2261b9f5068c64220b255a98e4f6f
  scenarios/sibling-link-grammar-is-the-shared-one.md: a186aab513ad32d2be12f541e36e5780a1d11b47e87940d1639004236f8d3d4a
  scenarios/sibling-symlink-trust-boundary.md: 752fabdf39eba625cf83e6c7ba3a57d593d7d5661af9eca800c852ca620c2ff7
  scenarios/the-promotion-coverage-line.md: 0aca63478ef99e42077f4460680846592ddf60a8b5bec891cf4d0fc05e97efe3
  scenarios/traverse-deps-cycle-check.md: 67a6f8c2b451c2db1ec85abaf5d3f5516b38f7618ce607f23fa318a49dc7f09e
  spec.md: 0dde97bf8590dbccfc710dc70623394fb720488f86488f8d9a936e3ae9995e46
  tasks.md: 9e83406e6d3609e55c4d6959093817e5c42d15f0cd459d8cfea68c2262640354
unexamined-by-reason:
  never-existed: 11
  not-a-live-claim: 15
  root-absent: 15
  ships-to-adopter: 9
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 2
  undispositioned: 0
decisions:
  - key: "body-path-existence — names `scripts/lib/`, which existed in this repository and no longer resolves: \"- **Managed roots** are `.ductus/`, `.githooks/`, the configured spec root, and the **historical** roots `.govern/`, `scripts/gen-` and `scripts/lib/`, since the orphan a migration leaves carries the root before the move (scenario [`orphaned-reference-historical-roots`](scenarios/orphaned-reference-historical-roots.md)). `matched-prefixes` bounds a clean result by scope, as `examined` bounds it by subject.\""
    outcome: discarded
    reason: "Correct by design: `scripts/lib/` is named as a historical prefix root the orphaned-reference check matches on purpose, not as a file that should exist. Operator-approved discard (2026-10-16)."
    decided-at: 2026-10-16T19:13:57Z
    decided-by: andy@stone.dev
  - key: "body-path-existence — names `scripts/lib/`, which existed in this repository and no longer resolves: \"**The pre-042 entries are prefixes, not a directory.** `scripts/gen-` and `scripts/lib/` rather than `scripts/`, because the framework owned those and never the whole directory. A `scripts/` root would make every unresolved adopter script a finding — noise this check must not produce, and the reason the current-roots list was scoped in the first place.\""
    outcome: discarded
    reason: "Correct by design: the scenario explains why the historical roots are the prefixes `scripts/gen-` and `scripts/lib/`; naming a retired root is its subject. Operator-approved discard (2026-10-16)."
    decided-at: 2026-10-16T19:13:57Z
    decided-by: andy@stone.dev
---

# Analysis — 067-analyze-and-audit-checks

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 50 unexamined target(s). Dispositions: 0 fixed, 0 routed, 2 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `scripts/lib/`, which existed in this repository and no longer resolves: "- **Managed roots** are `.ductus/`, `.githooks/`, the configured spec root, and the **historical** roots `.govern/`, `scripts/gen-` and `scripts/lib/`, since the orphan a migration leaves carries the root before the move (scenario [`orphaned-reference-historical-roots`](scenarios/orphaned-reference-historical-roots.md)). `matched-prefixes` bounds a clean result by scope, as `examined` bounds it by subject." — `specs/067-analyze-and-audit-checks/data-model.md` — **discarded**: Correct by design: `scripts/lib/` is named as a historical prefix root the orphaned-reference check matches on purpose, not as a file that should exist. Operator-approved discard (2026-10-16).
- body-path-existence — names `scripts/lib/`, which existed in this repository and no longer resolves: "**The pre-042 entries are prefixes, not a directory.** `scripts/gen-` and `scripts/lib/` rather than `scripts/`, because the framework owned those and never the whole directory. A `scripts/` root would make every unresolved adopter script a finding — noise this check must not produce, and the reason the current-roots list was scoped in the first place." — `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-historical-roots.md` — **discarded**: Correct by design: the scenario explains why the historical roots are the prefixes `scripts/gen-` and `scripts/lib/`; naming a retired root is its subject. Operator-approved discard (2026-10-16).

## Unexamined targets

- never-existed: 11
- not-a-live-claim: 15
- root-absent: 15
- ships-to-adopter: 9

## Fixed in this run

*None.*
