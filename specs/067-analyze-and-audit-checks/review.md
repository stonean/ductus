---
spec: 067-analyze-and-audit-checks
last-run: 2026-10-16T19:13:16Z
reviewed-against: 4ae8f4b99dde44319d54ab8a134bd9449e9b1eea
diff-base: 1e0b05d57b9d1f51cf62abf9b8e8ce9c7c885f2e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 14
scope: 59
skipped-passes: []
reviewed-digest:
  data-model.md: 4ed42a5f083d8f4fc453cd4ebd4962e0e4ab734aec59c4e22bab395dbfc74b71
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
blocking: false
dispositions:
  fixed: 4
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 067-analyze-and-audit-checks

## Summary

Review of 067's body-path-existence round, tasks 7-11. Base 1e0b05d5, the parent of the done-to-in-progress reopen (ebe611c2). examined: 14, each read as its full window diff or in full: runtime/src/primitives/check_artifacts.rs (the 708-line window diff), runtime/src/primitives/mod.rs (split_blocks_reading, written in this window and mutation-tested rule by rule), runtime/src/primitives/derive_dependencies.rs (one test, 017's case (c)), runtime/src/schema/primitives.rs, runtime/src/mcp/server.rs, scripts/audit/marker-list-parity.sh, framework/commands/analyze.md and its two generated mirrors (each mirror's change compared with the source's), 067's spec.md, data-model.md and check-artifacts-skipped-targets.md as diffs, and body-path-existence-family.md and 045's data-model.md in full. The five passes ran over them against all eleven rule files and AGENTS.md: no rule finding. Every test added in the window was shown to fail with the behavior it names disabled (QUAL-TEST-001). Four observations, each fixed before this record. Not read and not counted: the plan's runtime files this window did not change (validate_frontmatter.rs, traverse_deps.rs, resolve_anchor.rs, check_rule_ids.rs, derive_references.rs, check_corpus_links.rs, check_orphaned_references.rs, check_command_flags.rs, check_promotion_coverage.rs), framework/constitution.md, tasks.md, 045's spec.md, and the six 067 scenarios whose edits in this window were authored rather than re-read (adopter-generator-promotion, criterion-adopter-scope-destinations, criterion-negated-creation-phrasing, criterion-non-assertion-phrasings, criterion-path-existence-family, orphaned-reference-historical-roots), whose content is the subject of 067's analyze. Also not counted: the other specs' amendments in scope (007, 014, 017, 018, 020, 023, 024, 026, 030, 042, 043, 054, 057), the subject of their own reviews and analyses.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other — `logical_lines` re-implemented `split_blocks` with a different policy and diverged where the original was better: it read paths inside one-line HTML comments, took any `#`-led line for a heading, and carried its own list-item test. Folded into one splitter, `split_blocks_reading` with a `BlockReading` policy (task 11, 3b6e6946). — `runtime/src/primitives/check_artifacts.rs` — **fixed**
- bug — task 11's reading split let a plain line lazily continue a quote block, against the rule block-element-scanner records ("the text after resumes as a new paragraph"); both readings now resume a paragraph after a quote (3b2f0856). — `runtime/src/primitives/mod.rs` — **fixed**
- convention — the body family's doc comment carried an overlong line left by task 8; rewrapped (task 11). — `runtime/src/primitives/check_artifacts.rs` — **fixed**
- convention — the logical-line unit list omitted headings in analyze.md, the body family's doc comment and 067's body scenario, and that scenario still said the criterion family has fourteen markers after task 10 made it fifteen; all three corrected (830e90cb). — `specs/067-analyze-and-audit-checks/scenarios/body-path-existence-family.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
