---
spec: 026-framework-self-audit
last-run: 2026-10-16T01:28:01Z
reviewed-against: 3be5cda9f869af21fbc6e75f1758a76a66ce5331
diff-base: 1054b8bd2e2c51c8629b4e6a9d3193103739aa57
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 189
skipped-passes: []
reviewed-digest:
  scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
  scenarios/audit-script-refactors.md: 035bb7ef52c236135d791f35c3ec407d7908c8d51a169483bce40bc26c7a46c8
  scenarios/family-10-migration-coverage.md: 8684e643e4ea938cbeeed6d1342aef27efc84a7bf7559b111b6a12cacacfa02b
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: 603be582414b2cf49d14019afbd8fe1c0e9cdef49373d08b4a6c918ecfc755ec
  scenarios/family-19-mechanical-sweep-exemption.md: 1aed9678cb90da55fb314f9f8bd26addb3baebdbad04bda48fb11deeb95c4089
  scenarios/family-19-review-freshness.md: 2347d993d37163036d9cf592c2e3a28bf14498cf262c5eac5d86ca6b9bd569d6
  scenarios/family-19-says-what-it-examined.md: 2ee343d1295dfc755ea19ae453e5edd61dfb7cdb3d1eecdd46865d54be48621d
  scenarios/family-22-adopter-shell-behavior.md: 2eac3b7db9587dd354168941be8a1816d11a9e6f7fb27ecd8b8f9c9f8f719754
  scenarios/family-23-sweep-target-manifest-parity.md: 9dfa1299cdeb27ad691c66d6e9adeb68187c7c60039576cafd6ba55f29274b38
  scenarios/family-24-rename-sweep-residue.md: 41d6c829f0851a3caab11650d24b7100992667969ee3f21454141b482819e132
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 60627b24354610f3e0f819abe7d0603faa9644c27e8cee5163a452804bb94b6c
  scenarios/family-27-done-spec-unchecked-criteria.md: d164ba1fedaababcb88f5a4062b150ae8bd7e017699d7ac6d310292a25489bf9
  scenarios/family-28-audit-family-registry-parity.md: 0d7b15a3c103b50b3aeafa3145941910582c16dbda08148ebd804346da4ae15a
  scenarios/family-34-step-reference-integrity.md: 4d24ed6a08c54ae98f135ad108d3b8a7b85f782f5af7edf26c67d92aa509f067
  scenarios/family-35-manifest-destination-links.md: 8e94dce4172e2b326612da806327af31777d99b8c6ce89642c9d10a98dc148b3
  scenarios/family-36-self-url-resolution.md: 23716b0f5c3810890bab089c65258008842869e35616a71b86b663ed231bde18
  scenarios/family-39-shipped-anchor-resolution.md: 5a1600cb44670deb556bc0669717b1ef70122e1cb6072879cfdcca39a5a9f4b2
  scenarios/host-namespace-parity.md: 05714fe6b728391f699ed7328e1aea252a48259ae489fa9c06e0d0609dc1d376
  scenarios/link-check-consolidation.md: edaa15335c49241c5565a0b4063a3f59a5233b5b4a0c6362cbe9f4356b82812f
  scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 1
  undispositioned: 0
decisions:
  - key: "convention: family-19-review-freshness's Behavior still describes a `review:` block in spec.md and a commit-diff staleness check, which spec 057's relocation to review.md and the digest arm replaced — `specs/026-framework-self-audit/scenarios/family-19-review-freshness.md`"
    outcome: discarded
    reason: relocation residue from spec 057, outside the 022 split's window this re-review covers; the sibling family-19-says-what-it-examined states the digest arm, so a reader of 026 meets the current rule (operator decision, 2026-10-16)
    decided-at: 2026-10-16T01:28:01Z
    decided-by: andy@stone.dev
---

# Review — 026-framework-self-audit

## Summary

Scoped re-review of the 022 split's reference sweep: 026's family-18-marker-list-parity, family-19-review-freshness, family-39-shipped-anchor-resolution and link-check-consolidation scenarios name the moved scenarios (criterion-path-existence-family, criterion-negated-creation-phrasing, criterion-non-assertion-phrasings and adopter-corpus-link-integrity in 067, review-staleness-gate in 066) and 022's former AC23, now 067's AC2, at their new homes. examined: 4, each read in full: those four scenarios. spec.md (45,053 bytes) was read only as its two-token diff and is not counted. Base --since=1054b8bd, the commit before the 022 split's move (d852b627): the edits predate any reopen of this spec, which stays done because each is a pointer repoint. The five passes ran over the four files against all eleven rule files and AGENTS.md: no finding. One observation, discarded by the operator: family-19-review-freshness still describes the pre-057 review: block and commit-diff check, which is relocation residue outside this window, and its sibling family-19-says-what-it-examined states the digest arm. Not read and not counted: every other scope entry. Those are the plan's Affected Files, unchanged in this window and covered by the previous review, and the files the split changed elsewhere in the repository (git diff --name-only 1054b8bd plus the working tree), which are the subject of 022's, 066's and 067's own reviews.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: family-19-review-freshness's Behavior still describes a `review:` block in spec.md and a commit-diff staleness check, which spec 057's relocation to review.md and the digest arm replaced — `specs/026-framework-self-audit/scenarios/family-19-review-freshness.md` — **discarded**: relocation residue from spec 057, outside the 022 split's window this re-review covers; the sibling family-19-says-what-it-examined states the digest arm, so a reader of 026 meets the current rule (operator decision, 2026-10-16)

## Skipped passes

*None.*

## Unexamined governance

*None.*
