---
spec: 066-audit-records-and-the-completion-gate
last-run: 2026-10-27T06:47:55Z
reviewed-against: 98a2eae0e909d19a50b246d127006f8440535e75
diff-base: 57f82b22e4ca9d7d78c9d481f82f13e72f28b092
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 45
scope: 70
skipped-passes: []
reviewed-digest:
  data-model.md: 8851119617356540be9a3fd4434cdf7eecc04bcb9ae5471322a0c5a93ae1ed77
  scenarios/a-done-spec-has-no-transition-to-gate.md: d4a9f47277ddcd5e8735be7d13ee7f57606827a5f36db608da3fd70a05aaa39a
  scenarios/a-done-specs-plan-edit-leaves-its-analysis-current.md: 3d63a66653413cb90127966cf092b984c71d44dad6d9dda77eba08131f6737e6
  scenarios/a-multi-number-renumber-is-one-rewrite.md: 0b0c25fc2e8446f403bff82cff5b0d64c86cfbc45d98bc492fd5fa824c9fdd5a
  scenarios/a-renamed-file-contributes-its-rewrites.md: 52907e92b668e9d100ab339e0d36953cd6d025104b3650174bf7b806b3631490
  scenarios/a-review-states-what-it-read.md: 333ffc663a0d0f471970dbcdad9b010583dd891f651e626741962cad8265da14
  scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md: 4273f4da9ffa9a0b4b02b06dba0b8d05a2c8ceb4780937dc360c1e72153d57fc
  scenarios/only-gating-findings-hold-done.md: 43012eb89501db9f4abc468448aadf7fc1867f72cd0bb1502bad16f26ea107d0
  scenarios/process-waivers-file-lists.md: 3fb3b9e54f414691d28d984ba76cd48c7e1de4fe2ee67259a44ea8559da447dc
  scenarios/review-base-includes-the-transition-commit.md: 62f1434d3a2d59d62ebbfa37d5c6561d82c4cb343907d7cbe081e0df63801b3f
  scenarios/review-scope-includes-uncommitted-work.md: 49699137b92a8578bb85e00ea0c31f702106b21f1e954ab863e3d9db323a5198
  scenarios/review-scope-plan-affected-table-format.md: 25d9a4b8b89fea9460ea2b134e4184bcf117df678ed501d1ba16efa9f84abe02
  scenarios/review-scope-union.md: e5fa76ef2b48ea246649338d71d8c0a6988d7e48a9202e677ad593657316c0ae
  scenarios/review-staleness-gate.md: 61427d7c8f1393cf72edeae32fd54502c99641499c429cf33b7368d93e0e7131
  scenarios/review-staleness-on-done-specs.md: 18484a327c18ea44c3cfbe24fc22d1b727c5033cf619aa3760c3652fdc1b66f5
  scenarios/the-committed-tree-horizon.md: cac65fb842defe0ce0326a76274794d1402d96c10aa02a2f977fd6d326ab3772
  scenarios/the-cross-spec-impact-gate.md: b85c2d557733a29beb8084823a9156e13b283090a283203aa14dcaa85afed3c1
  scenarios/write-analysis-and-the-second-gate.md: afaaa2f6957547d1f9dce559426353e58a00da287b1066f7eb0af05d51330397
  scenarios/write-review-known-field-quoting.md: 914954823ee3da52dccbb25749851013d5dd3be442dcc85b09661c27c7bea2a9
blocking: false
dispositions:
  fixed: 24
  routed: 0
  discarded: 6
  undispositioned: 0
decisions:
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: discarded
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: discarded
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the implement reference's \"non-blocking\" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md`"
    outcome: discarded
    reason: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md`"
    outcome: discarded
    reason: "pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: discarded
    reason: the Context records the state when the scenario was found, and the close is loose, not false
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md`"
    outcome: discarded
    reason: "contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
---

# Review — 066-audit-records-and-the-completion-gate

## Summary

Shared-window review of 058's `findings-cost-in-proportion-to-risk` change, `--since 57f82b22` (item 3's close): e38f5c1f, the change, with an independent review of its uncommitted form folded in; 84bb4aa1 and 6cf2bb6e, fix rounds 2 and 3, each re-read by an independent agent, round 3's re-read finding no MUST or SHOULD; and 98a2eae0, round 4, a doc comment, a comma and a discharged inbox item, not re-read as a prose and doc-comment round. Rules: the eleven files `discover-rule-files` selects. No rule fires: the gate reads counts from local records, the wire enum and the `blocked-by` value are unchanged, and each gating term has a test that fails without it (mutations M1-M5 killed). 0 MUST, 0 SHOULD; 30 observations, 24 fixed and 6 discarded. Examined 45: the hand-written files the window changed, read as diffs and, for `disposition_block` and its tests, the 058 scenarios and both owner scenarios, whole. The window's 11 generated copies under `.claude/`, `.pi/` and `.ductus/reference/` were compared byte for byte with their sources, not read. The rest of scope went unread: the plan Affected Files the window left unchanged, which earlier records cover, and the five review records this run writes. Stored decisions from earlier windows are retained, their sources lying outside this window. Its own artifacts were read in the window: spec, data model, tasks and the new `only-gating-findings-hold-done` scenario, whose behavior is the gate change above.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: write-analysis's summary said a hard-fail or blocking finding holds done until it is fixed or routed — `runtime/src/primitives/write_analysis.rs` — **fixed**
- other: the constitution's Finding dispositions lead read a review observation as a gating finding — `framework/constitution.md` — **fixed**
- other: "only a gating finding holds done" was false for an out-of-spec implement finding's disposition task — `framework/constitution.md` — **fixed**
- other: 047's §Constitution amendment signpost and AC7 said every analyze finding must get a disposition before done — `specs/047-analyze-findings-durability/spec.md` — **fixed**
- other: the gate's check-11 doc comments still described it as reading the dispositions maps — `runtime/src/primitives/check_review_gate.rs` — **fixed**
- edge: no --fix trigger reverts a done spec whose own analysis is blocking once disposition drift stopped reverting — `framework/reference/analyze.md` — **fixed**
- test: the gate's MUST and analysis terms were reachable only from a hand-edited record and no test exercised them — `runtime/src/primitives/check_review_gate.rs` — **fixed**
- other: 058's first Resolved Question cross-referenced a section that now says the opposite before its supersession note — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- convention: --waive was documented for a MUST violation only, while the gate's guidance directs it at a SHOULD — `framework/commands/review.md` — **fixed**
- other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md` — **discarded**: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
- other: the disposition-drift reference said the report names which findings nobody decided — `framework/reference/analyze.md` — **fixed**
- other: loose "gets a disposition" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md` — **discarded**: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
- other: 058's spec said an undecided finding's decision persists — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- other: the implement reference's "non-blocking" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md` — **discarded**: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
- other: 058's analyze section said the findings disposition drift counts do not hold done, though analysis.md's count includes gating ones — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- other: docs/analyze.md gave the same false reason for --fix not reverting on disposition drift — `docs/analyze.md` — **fixed**
- other: the scenario's first Resolved Question said the gate reads only should-violations — `specs/058-findings-route-at-discovery/scenarios/findings-cost-in-proportion-to-risk.md` — **fixed**
- other: the review reference's §Waivers said a waiver drops a finding from must-violations only — `framework/reference/review.md` — **fixed**
- other: "only an unwaived SHOULD reaches this check" was stated flat, though a hand-edited record reaches it with a MUST — `framework/reference/implement.md` — **fixed**
- other: the drift report sentence understated analysis.md's undispositioned count — `framework/reference/analyze.md` — **fixed**
- other: the analyze-state drift reference bullet said routing anywhere clears a blocking record — `framework/reference/analyze.md` — **fixed**
- other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md` — **discarded**: pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion
- other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md` — **discarded**: the Context records the state when the scenario was found, and the close is loose, not false
- other: §Per-run waiver processing said an expired waiver re-counts toward must-violations and flips blocking, false for a SHOULD — `framework/reference/review.md` — **fixed**
- other: docs/analyze.md's "every other gating count blocks earlier" was the last unscoped sibling — `docs/analyze.md` — **fixed**
- other: 058's data model read as saying analysis.md's undecided findings do not hold done — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- other: docs/analyze.md's scoped clause parsed as "the runtime writes every other gating count" without its commas — `docs/analyze.md` — **fixed**
- other: ReviewSeverity::is_blocking's doc comment said a SHOULD never blocks done, and called it the bucketing predicate — `runtime/src/schema/severity.rs` — **fixed**
- other: the inbox still held "check-review-gate never reads should-violations", which the gate change discharged — `specs/inbox.md` — **fixed**
- edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md` — **discarded**: contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true

## Skipped passes

*None.*

## Unexamined governance

*None.*
