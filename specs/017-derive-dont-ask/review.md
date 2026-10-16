---
spec: 017-derive-dont-ask
last-run: 2026-10-16T19:36:00Z
reviewed-against: ec72c234476d97f7057f68d83c6e92f9e88eb94b
diff-base: 34b137d57993ed10a995c6510ea7f3d0956de037
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 60
skipped-passes: []
reviewed-digest:
  data-model.md: 6fa23783db77c3e735bd98e99bd6f919cdecd87d5fdb850bc63ee86797b19b14
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 1bd156f5abd11ebb6c854241092e5c428cec4d3601f2a9fb4c4545c997aa1fc9
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/tracked-specs-not-worktree.md: e3808ab1ae9a4d6b1b2103c85537c6236e99898b7569536eb9ac2bfcdde684bf
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
decisions:
  - key: "convention: AC25 said recording a review observation is what writes it to the inbox, which 058 stopped; its guarantee holds and only the mechanism is dated — `specs/017-derive-dont-ask/spec.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/spec.md
    decided-at: 2026-09-28T00:02:48Z
    decided-by: andy@stone.dev
  - key: "convention: data-model prescribed `## {Category Name}` rule-file headings while configuration-cross.md and 008's corrected twin schema use `## {ID prefix} — {Category Name}` — `specs/017-derive-dont-ask/data-model.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/data-model.md
    decided-at: 2026-09-28T00:02:48Z
    decided-by: andy@stone.dev
  - key: "convention: data-model said hook scripts use the `.sh` extension; git invokes hooks by name and none has ever had one — `specs/017-derive-dont-ask/data-model.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/data-model.md
    decided-at: 2026-09-28T00:02:48Z
    decided-by: andy@stone.dev
---

# Review — 017-derive-dont-ask

## Summary

Scoped re-review of the body-path round's rewrite to current (58c1942e). examined: 5: the scenarios detect-dependency-cycles.md, generator-sync-claim-honesty.md and skip-prose-cross-references.md in full, spec.md as its window diff, and runtime/src/primitives/derive_dependencies.rs as its window diff - one test, a_cycle_beside_an_acyclic_subgraph_reports_only_the_cycle, shown to fail when acyclic components are reported. Each rewritten claim was checked against the code it now names: derive_dependencies.rs (the cycle check over every tracked spec; the result fields examined, untracked-skipped and unwritten and the tests asserting them), main.rs (a cycle exits non-zero with cycle: lines on stderr), spec_links.rs (the See-also rule at level 1 or 2), ductus-procedure.md Hook Installation, and both pre-commit hooks. The three stored routed decisions are retained: they are settled, and two concern data-model.md, which this run did not re-read. The five passes ran over the files counted as examined, against all eleven rule files and AGENTS.md, and found no finding and no observation. Not read and not counted: every other scope entry - the plan's Affected Files, unchanged in this window, and the files other specs' amendments and 067's tasks 10-11 changed after the base (including runtime/src/primitives/derive_dependencies.rs unless named above), which are the subject of their own reviews.

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
