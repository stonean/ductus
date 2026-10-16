---
spec: 067-analyze-and-audit-checks
last-run: 2026-10-16T01:06:47Z
reviewed-against: 40d0eb2a733f90e97e4228cc8d212fc89eedf4cb
diff-base: 08f0b1993e00eb4d6bad3fa645de8516985ed9f3
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 30
scope: 177
skipped-passes: []
reviewed-digest:
  data-model.md: 98c97abac04c4b38af0781f87a6f9ecb568a7285c01260e28ad2460d9b5c8c46
  scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md: 4057057a18a969d37b8c503f0022b277754d9c41d34f96b443a3dc1e8df7e858
  scenarios/adopter-corpus-link-integrity.md: 7950b5560f15986c7bed5cc134034ed46fb0d2cbc517e4b030eb55ac114da3e9
  scenarios/adopter-generator-promotion.md: 386beb5d136f78ea8f3d8542209cfd9381ce7d29c1c5ea8a2871880bc802cbca
  scenarios/analyze-artifact-checks.md: 0dbaa7167a55eb7c373cbe74ad5fee99cc79175ab75c416f8657b7699247867a
  scenarios/anchor-reference-kinds.md: 27e72e3aa41d7a6eca6fa128d6b86aae518adb9b1212ace06bf5bb6015455a45
  scenarios/argument-hint-needs-a-token.md: 130b949e9ca72ab34b48e43d2179fbed5d21ef111b40213180c72ae41bc28ae7
  scenarios/block-element-scanner.md: 0b96c1dc41bd91f898e535c17c8964ef9da5ccf39411d888d8e244c82afe9c38
  scenarios/check-artifacts-skipped-targets.md: 8997c7853ae7f954b05d19ab649ec44f2303a1c148ece7da51898f03550a1be7
  scenarios/criterion-adopter-scope-destinations.md: 3787689a96528f3dfa1e526bb18bda86ba3ab0c3446902d570186de6a97c8ad8
  scenarios/criterion-negated-creation-phrasing.md: 9182066874fbfdba889487da4f41fdd30d329691b9e5e0daca565e7aa94740cd
  scenarios/criterion-non-assertion-phrasings.md: 788a9b38d41c67496635ae93d8df4293c5faedda2c94d63933a6938e992b091e
  scenarios/criterion-path-existence-family.md: 1ea335467cd06aff79abce86ebaee0aa560a886e04ab21c64f28f9e561c25350
  scenarios/derive-references-unstaged-drift-is-reported.md: dc9f33da659dd20c971b5691d46127ba9673af3141cede8b7bc5481e1b9e4da0
  scenarios/derive-unparseable-frontmatter-is-reported.md: 062193d4b54ba56a6b791a086b1b72c6b63b86b6f7bc991bea5fe978a7b2747b
  scenarios/every-agents-generated-copies-are-excluded.md: cfbb050341906864fe67031fe1cc1c8928966221adf2e2346079b8e784d30541
  scenarios/frontmatter-severity-tiers-on-both-sides.md: 517961da7a2118869f494179041a5e1b3dcdf90060828595411c6f35eb386a85
  scenarios/link-adjacent-drift-family.md: 41814ebc48e3744fdb053d09ad024bb9da2bc998d3a4767d3b6a2338b4fd7b44
  scenarios/orphan-check-adopter-authored-references.md: f489b8688ad92e92846bc0c5d74d68239c7478a47aedd954aaac38dbd09c7340
  scenarios/orphaned-reference-check.md: 6c990e4c4668e432381107b3d0bc184204a5888f01004868b4f22d55f9012ee0
  scenarios/orphaned-reference-historical-roots.md: 969cc86318c764f9297c199287e68c104ac186d1160ade68829f03d9b839bc3b
  scenarios/per-agent-scaffold-paths-ship-to-adopter.md: b9d60c57c3ef5f4a6fc443cd25dbfd0decf189f47aa5571da036be0c90ea6e19
  scenarios/sibling-link-grammar-is-the-shared-one.md: a186aab513ad32d2be12f541e36e5780a1d11b47e87940d1639004236f8d3d4a
  scenarios/sibling-symlink-trust-boundary.md: 752fabdf39eba625cf83e6c7ba3a57d593d7d5661af9eca800c852ca620c2ff7
  scenarios/the-promotion-coverage-line.md: 0aca63478ef99e42077f4460680846592ddf60a8b5bec891cf4d0fc05e97efe3
  scenarios/traverse-deps-cycle-check.md: 67a6f8c2b451c2db1ec85abaf5d3f5516b38f7618ce607f23fa318a49dc7f09e
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 067-analyze-and-audit-checks

## Summary

First review of 067, which the 022 split created. Base 08f0b199, the parent of the planned-to-in-progress transition 1054b8bd, so the window holds the move (d852b627), the sweep (13acc295, bd4da459, c4b14ac5) and this spec's own fixes. examined: 30, each read in full: spec.md, plan.md, tasks.md, data-model.md, all twenty-five scenarios, and framework/constitution.md, whose canonical-sources row and scenario-link bullet this window changed (the bullet's claim checked against is_spec_path in runtime/src/primitives/mod.rs). The five passes ran over them against all eleven rule files and AGENTS.md: no finding. Reading them surfaced five references the move re-aimed, each fixed before this record as this spec's own work: adopter-generator-promotion's spec.md section reference (twice) and AC13 were 022's, sibling-symlink-trust-boundary's AC8 is 045's, derive-unparseable-frontmatter-is-reported called this data model the registry of every primitive's shapes, and analyze-artifact-checks said check-artifacts takes --all, which it does not (task 4). Not read in full, and not counted: scripts/audit/marker-list-parity.sh and the plan's runtime files check_artifacts.rs, validate_frontmatter.rs, traverse_deps.rs, resolve_anchor.rs, check_rule_ids.rs, derive_dependencies.rs, derive_references.rs, check_corpus_links.rs, check_orphaned_references.rs, check_command_flags.rs and check_promotion_coverage.rs. Their window diffs were read in full and are doc-comment and path repoints only; no runtime behavior changed in this window, and the code is 022's earlier reviews' subject. Also not counted: every other scope entry, the files the split changed elsewhere in the repository (git diff --name-only 08f0b199 plus the working tree), which are the subject of 022's, 066's and the done specs' own reviews.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
