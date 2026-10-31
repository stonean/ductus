---
spec: 013-text-first-artifacts
last-run: 2026-10-31T01:16:01Z
reviewed-against: afb8a865b3d62ef7a02ff52fa64b3234bb7cefe6
diff-base: d5de121cdca984d6280dce8613e266bee5a20e0b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 352
skipped-passes: []
reviewed-digest:
  data-model.md: 351907a6128e8e325f6685f7bfedeed26efff4b46b3f76918a8bbca74e180dd8
  scenarios/criterion-identifiers.md: 2a6b871e588d7abb6f4ba6bb5a38e81562e30b82c611d2e3f36ba72da86be931
  scenarios/past-tense-motivation-convention.md: 04b9583c6f2f4d56efa0cd7b544afd9811bd7925121ede083fbed9b6f2cdc1d4
  scenarios/the-precheck-ignores-what-the-same-run-wrote.md: f2bae94f49334f88d3b9889eefb40f2b7e19df126a7e64e0edba73ed8ce155d6
blocking: false
dispositions:
  fixed: 7
  routed: 0
  discarded: 6
  undispositioned: 0
decisions:
  - key: "other: the pre-run status's claim that nothing earlier in the run writes under specs/ holds only if Pre-run Migrations run before walker step 4's Shared Files apply-manifest, which ductus.md §Instructions does not state — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: already recorded as 056's scenario the-walker-places-every-archive-half-section, whose task gates 056's done; the old fresh status refused the same way, so this is no regression
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "other: spec step 1 scopes the precheck to specs/ and framework/templates/spec/, while the procedure checks specs/ only — `specs/013-text-first-artifacts/spec.md`"
    outcome: discarded
    reason: the spec's own parenthetical covers it, and the plan's step 1 explains why an adopter has no framework/templates/spec/
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "other: the plan's list of Pre-run Migrations that write under specs/ omits strip-inbox-guidance and audit-record-relocate — `specs/013-text-first-artifacts/plan.md`"
    outcome: discarded
    reason: the list is illustrative, not a closed set
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "edge: Pre-run Migrations step 1's abort does not say whether the section or the run stops — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: "contrived: it needs a malformed archive, and the pre-run status is taken before step 1, so either reading is safe"
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "edge: a file the adopter had dirtied that a migration also rewrote is named only by the refusal, not as changed by a migration — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: "decided in the plan and in the scenario's first edge case: the file is the adopter's to commit, and the refusal names it"
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "other: the repeated refusal ends with The rest of this run continues after the run has finished — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: wording only; the sentence is still true of the run it describes
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
---

# Review — 013-text-first-artifacts

## Summary

Five passes over 013's window since its reopen, the default base `d5de121c` (the parent of `a750d8a8`, which recorded scenario `the-precheck-ignores-what-the-same-run-wrote`). The window spans three days of other specs' work, and the base was not overridden: the reopen is what the scenario's task answers. An independent agent did the passes and re-read the fix round; its claims were re-checked here.

Examined 7 of 352 scope entries: the scenario (its `a750d8a8` text, the edge cases `3b359742`, `f056b209` and `74f6361c` added, and `9d79988e`'s Behavior and Resolved Question), 013's `spec.md`, `plan.md`, `tasks.md` and `data-model.md`, and `framework/bootstrap/ductus-procedure.md` in full, including the other specs' hunks of `3b359742`, `54715126`, `70bb4af5` and `2ce9e62d`, none of which touches Pre-run Migrations or the Frontmatter Migration; `framework/bootstrap/ductus.md` in part, to check that nothing before Pre-run Migrations writes under `specs/` (lines 20-49, 113-609 and 740-939 read, 48-112 scanned for writes, the config block 610-739 and 940 onward not read). Not read, 345 entries: 343 that are other specs' work, each reviewed by its own record, among them 058's deliverable and its `review.md`; 013's own `review.md`, which this pass writes; and `resume.txt`, an untracked file another session left at the root.

Rules: every rule in the eleven files was loaded by heading, Statement and trigger; `quality-cross.md`, BE-INPUT-003 and BE-INPUT-004 were read in full as the ones whose triggers reach a host-walked procedure that runs `git status` and rewrites files under `specs/`, and none fires.

Checked against the procedure as it reads: each of the scenario's five edge cases and three Behavior bullets is closed by `ductus-procedure.md`'s text, and its Resolved Question matches the Precheck and the plan; the second status's difference is what Post-Scaffolding Output promises. A run that aborts at Pre-run Migrations step 1, one that declines the migrations, a greenfield run, and a host placing the Frontmatter Migration before or after Shared Files all judge the pre-run status correctly. The real adopter bootstrap at `89e6f02b` (`09b2a408`) verified the no-refusal path, the refusal path and the hook replacement on three subjects.

Observations: thirteen, each confirmed by the operator. Seven fixed in the run: project-relative paths for a subdirectory project, the procedure's "the walk above", and stale `ductus.md` pointers in the spec's edge case, the data model, the plan's Overview, Affected Files and Resolved Question summary, and ticked AC17, which still holds against `ductus-procedure.md`'s post-run output. Six discarded with reasons. The fix round was prose-only, so it is not re-read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: the pre-run status's claim that nothing earlier in the run writes under specs/ holds only if Pre-run Migrations run before walker step 4's Shared Files apply-manifest, which ductus.md §Instructions does not state — `framework/bootstrap/ductus-procedure.md` — **discarded**: already recorded as 056's scenario the-walker-places-every-archive-half-section, whose task gates 056's done; the old fresh status refused the same way, so this is no regression
- edge: in a project that is a subdirectory of its repository, git status --porcelain prints paths from the repository root, so the refusal's {paths} and each changed-by-a-migration line named a path outside the project's own form — `framework/bootstrap/ductus-procedure.md` — **fixed**
- other: the procedure's partially-migrated edge case called the Walk's first-line check the precheck above — `framework/bootstrap/ductus-procedure.md` — **fixed**
- other: the spec's partially-migrated edge case said the migration completes the conversion, while the procedure skips the file and attempts no mixed-state recovery — `specs/013-text-first-artifacts/spec.md` — **fixed**
- other: data-model.md named framework/bootstrap/ductus.md §Frontmatter Migration, which 056 moved to ductus-procedure.md — `specs/013-text-first-artifacts/data-model.md` — **fixed**
- other: the plan's Affected Files row for ductus.md described a migration step and a viewer tip that 056 moved to ductus-procedure.md — `specs/013-text-first-artifacts/plan.md` — **fixed**
- other: the plan's Overview said migration logic lives in framework/bootstrap/ductus.md — `specs/013-text-first-artifacts/plan.md` — **fixed**
- other: ticked AC17 and the plan's Resolved Question summary named ductus.md's post-run output, which 056 moved to ductus-procedure.md — `specs/013-text-first-artifacts/spec.md` — **fixed**
- other: spec step 1 scopes the precheck to specs/ and framework/templates/spec/, while the procedure checks specs/ only — `specs/013-text-first-artifacts/spec.md` — **discarded**: the spec's own parenthetical covers it, and the plan's step 1 explains why an adopter has no framework/templates/spec/
- other: the plan's list of Pre-run Migrations that write under specs/ omits strip-inbox-guidance and audit-record-relocate — `specs/013-text-first-artifacts/plan.md` — **discarded**: the list is illustrative, not a closed set
- edge: Pre-run Migrations step 1's abort does not say whether the section or the run stops — `framework/bootstrap/ductus-procedure.md` — **discarded**: contrived: it needs a malformed archive, and the pre-run status is taken before step 1, so either reading is safe
- edge: a file the adopter had dirtied that a migration also rewrote is named only by the refusal, not as changed by a migration — `framework/bootstrap/ductus-procedure.md` — **discarded**: decided in the plan and in the scenario's first edge case: the file is the adopter's to commit, and the refusal names it
- other: the repeated refusal ends with The rest of this run continues after the run has finished — `framework/bootstrap/ductus-procedure.md` — **discarded**: wording only; the sentence is still true of the run it describes

## Skipped passes

*None.*

## Unexamined governance

*None.*
