---
spec: 083-commits-are-attributed-to-specs-without-a-subject-convention
last-run: 2026-11-24T01:14:44Z
analyzed-against: f5d624fd6898b16206ad5929b60683abf3b4d06f
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  plan.md: 31a8e36f6174ff984c0cc223b9a086e14284e431bf360d090981520acc42251b
  review.md: d019bc916a439bbf34dc6bec4b5241ad625defc8bb161fcc45643b879c1d4c1c
  spec.md: f9dd8f550e45585f6bd79a4b59dfa398d8f06208b5f1b479794554951be71a54
  tasks.md: 997a559b05319824d615101b65a3ab8cc1dc3f67008dcbb4855a5bcc8757d151
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
  "plan.md": e3b0c44298fc1c14
  "plan.md#083 — Commits are attributed to specs without a subject convention Plan": 337eedf1d8cc11d9
  "plan.md#Affected Files": c793ebd1968074a1
  "plan.md#Counting in the window walk": c815187c26edb5cb
  "plan.md#Cross-spec impact": 92ff257fe41032ba
  "plan.md#Overview": 2877275f2abdc772
  "plan.md#Result fields": 816e2b2ef9c5d983
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The Summary line": 48136ff4bc3cdcee
  "plan.md#The commit step": abc4f3980c87eded
  "plan.md#Trade-offs": 95afe7f3957288ae
  "plan.md#Where the shape is documented": cd195519c96386ca
  "spec.md": 3ef38c5f717fe93a
  "spec.md#083 — Commits are attributed to specs without a subject convention": 2d2086b3c05bd5d2
  "spec.md#Acceptance Criteria": dd5d4523b8e26921
  "spec.md#Applicable Rules": 2fadc9a870f17d68
  "spec.md#Behavior": 8dc28e428e1d8629
  "spec.md#Edge Cases": a96cb4423277fc3a
  "spec.md#Motivation": 8d0044e64abd54ef
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": bcbdd54e57350a97
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Result fields"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#Result fields"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
blocking: false
dispositions:
  fixed: 10
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 083-commits-are-attributed-to-specs-without-a-subject-convention

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 10 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

*None — every target was examined.*

## Fixed in this run

- grounding — Counting in the window walk / The commit step: the claim that `ductus exec`'s walker makes no commit 'as before this spec' is stated as fact about current behavior with no citation (§Grounding). — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md` — **fixed** (decided by the rule)
- grounding — The Summary line: the claim that 020's data model lists `## Summary` among body sections without fixing its lines names the artifact but no section or quoted phrase (§Grounding). — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md` — **fixed** (decided by the rule)
- grounding — Motivation, 'The miss was silent': the claim that no result counted the commits no spec claimed describes existing compute-review-scope output and cites no symbol or command. — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, 'The commit step': the claim that none of the primitives in PRIMITIVE_NAMES makes a commit outside its tests is stated as fact with no query or enumeration behind it. — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md` — **fixed** (decided by the rule)
- decision-reasons — Resolved Questions, 'Where does the count appear?': the frontmatter-field option is rejected partly because it would reopen 020 (and edit 066's data model), a reopen cost. — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, Where the shape is documented: asserts that check-review-gate's cross-spec-impact check reads the spec.md body or a scenario, citing no file-and-symbol, command or artifact for it (Grounding, advisory). — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, Where the shape is documented: asserts that derive-dependencies reads no link on a blockquote-prefixed line, citing no primitive description or source for it (Grounding, advisory). — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md` — **fixed** (decided by the rule)
- grounding — Edge Cases, 'A squash merge on the host': the claim that a merge commit contributes only what it changed itself, as before, cites no source (found by the host closing step 15's class). — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md` — **fixed** (decided by the rule)
- grounding — Resolved Questions, 'Where does the count appear?': the claim that write-review already appends lines it derives itself to the Summary cites no symbol (found by the host closing step 15's class). — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md` — **fixed** (decided by the rule)
- rule-assessment — BE-COMPAT-001: The plan adds commits-examined and commits-unclaimed to compute-review-scope's result but never classifies the change as additive or breaking, nor names the release it ships in, as 019, 020 and 066's plans do for primitive results. — `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md` — **fixed**
