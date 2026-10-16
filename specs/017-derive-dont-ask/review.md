---
spec: 017-derive-dont-ask
last-run: 2026-10-16T00:58:28Z
reviewed-against: c4b14ac5f8bc92a2510a4a9d25f7c9ea00900953
diff-base: 1054b8bd2e2c51c8629b4e6a9d3193103739aa57
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 201
skipped-passes: []
reviewed-digest:
  data-model.md: 6fa23783db77c3e735bd98e99bd6f919cdecd87d5fdb850bc63ee86797b19b14
  scenarios/detect-dependency-cycles.md: d16520a341eb42d0f5ed8d93c4d49593b291270f2a919258757c8e2f43cedf8d
  scenarios/generator-sync-claim-honesty.md: 11effb857afa9e90731671c70535c8c22554895ebc01302faf46543225762ee1
  scenarios/skip-prose-cross-references.md: ff66ff69fb884f8880a4db9d157b9c8ff35981b57fadfd4ba0299157b7a1639f
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

Scoped re-review of the 022 split's reference sweep: six pointers moved from 022-deterministic-runtime to 067-analyze-and-audit-checks, where adopter-generator-promotion and derive-references-unstaged-drift-is-reported now live. examined: 5, each read in full: data-model.md and the scenarios detect-dependency-cycles.md, generator-sync-claim-honesty.md, skip-prose-cross-references.md and tracked-specs-not-worktree.md. The four Superseded mechanism (022) signposts stay as written, since they name the spec that did the superseding and no moved scenario. spec.md (31,024 bytes) was read only as its one-token diff at line 111 and is not counted. The three stored routed decisions are retained: this run did not re-read spec.md, the source of one of them. Base --since=1054b8bd, the commit before the 022 split's move (d852b627): the edits predate any reopen of this spec, which stays done because each is a uniform pointer substitution. The five passes ran over the files counted as examined, against all eleven rule files and AGENTS.md, and found no finding and no observation; each repointed link or name resolves to the scenario, data model or criterion it names in its new home. Not read and not counted: every other scope entry. Those are the plan's Affected Files, unchanged in this window and covered by the previous review, and the files the split changed elsewhere in the repository (git diff --name-only 1054b8bd plus the working tree), which are the subject of 022's, 066's and 067's own reviews.

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
