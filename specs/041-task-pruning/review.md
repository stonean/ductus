---
spec: 041-task-pruning
last-run: 2026-10-27T04:19:28Z
reviewed-against: 88768fe67738169b56f4f60554eb67b383858398
diff-base: 865e97ae554c65b42bdf842d2a291e69981a9ffe
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 44
scope: 78
skipped-passes: []
reviewed-digest:
  data-model.md: 618d6ecf7942d79fb7f7d74344e4e29e790e0f7991044fac25b32621d0f8bda9
  scenarios/a-plan-edit-does-not-reopen-a-done-spec.md: 0cb04f76649997738f1c988046a93e9b2e9e2c7d000f6e0f732940c89e79acdc
  scenarios/plan-records-the-design-as-it-stands.md: d31c168a226de2a29d27b58f1fa9e188c281387cd4340dd5e68ba3eefbc0e30f
  scenarios/prune-all-walks-every-spec.md: 4b605a7c3d07bff5e6a90bd02e982e93f7360d12f3934f9663a0eb44ebc9fc92
  scenarios/prune-reduces-the-spec-directory.md: 7a987ae34945c37103ef63ff84756a64c3ca30ce8fd373ee66e88def90e5c8f3
blocking: false
dispositions:
  fixed: 13
  routed: 0
  discarded: 10
  undispositioned: 0
decisions:
  - key: "efficiency: exec analyze's step 16 reads HEAD on a done spec for a reopen-required it ignores, and halts when HEAD cannot be read — `runtime/src/interpreter/mod.rs:1078`"
    outcome: discarded
    reason: the cost was accepted in the 2026-10-01 decision recorded in prune-reduces-the-spec-directory's Resolved Questions
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: main.rs and the interpreter keep separate hand-maintained per-primitive lists — `runtime/src/main.rs:689`"
    outcome: discarded
    reason: flat registries pinned by set-equality tests, a design AGENTS.md records
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "contract: the two flattened Option result halves allow neither or both; an untagged enum would keep exactly one — `runtime/src/schema/primitives.rs:2864`"
    outcome: discarded
    reason: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; AGENTS.md records the flattened-halves design
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "efficiency: the prune-plan walk runs inline on the async worker and reads HEAD per done spec, even for specs it then drops — `runtime/src/mcp/server.rs:710`"
    outcome: discarded
    reason: one HEAD read per done spec is not the unbounded revwalk the blocking pool is scoped to, and a spec it drops still needs the answer to know it can be dropped
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "simplicity: check_stuck converts the tasks path to a git path twice — `runtime/src/primitives/check_stuck.rs:91`"
    outcome: discarded
    reason: the helper takes a project-relative path by design; the cost is one string build
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: three sha256-to-hex spellings across analyze_subjects, prune_plan and fetch_archive — `runtime/src/primitives/analyze_subjects.rs:202`"
    outcome: discarded
    reason: consolidating them touches fetch-archive (048) for no change in behavior
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: inbox_standing re-implements the HEAD blob read, and prune_plan imports shared helpers from prune_tasks — `runtime/src/primitives/inbox_standing.rs:191`"
    outcome: discarded
    reason: inbox_standing is the documented raw-repository exception, and where an import comes from changes no behavior
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "edge: split_frontmatter takes a later bare-LF --- as the fence when the frontmatter closes with CRLF — `runtime/src/primitives/mod.rs:857`"
    outcome: discarded
    reason: no such mixed-ending file exists here, and the fix reaches every frontmatter reader for a case not seen
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "contract: SkipScanner recognizes only backtick fences, so a ## inside a ~~~ fence is a section — `runtime/src/primitives/mod.rs:1742`"
    outcome: discarded
    reason: SkipScanner is 022's shared scanner, no ~~~ fence exists in this corpus, and widening it changes every parser built on it
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: the prune-tasks and prune-plan walk skeletons are near-copies — `runtime/src/primitives/prune_plan.rs:210`"
    outcome: discarded
    reason: the walks differ in what they list and how; a shared skeleton would be generic over both results for one loop
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "contract: an applying prune-tasks walk that errors on a later spec returns no record of the reductions it already wrote — `runtime/src/primitives/prune_tasks.rs:176`"
    outcome: discarded
    reason: each write is atomic and idempotent, so re-running the walk resumes; the data model and plan now say so
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "edge: a reset also drops anything above the H1, and its H1 search scans fenced and commented lines — `runtime/src/primitives/prune_tasks.rs:477`"
    outcome: discarded
    reason: a tasks.md opens with its H1 by construction from the template, and no file here has content above it or a heading only inside a fence
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "test: no prune-plan test that a preview on a done spec errors when HEAD cannot be read, or on a malformed decisions list — `runtime/src/primitives/prune_plan.rs:299`"
    outcome: discarded
    reason: the preview and apply share one reopen computation, whose unreadable-HEAD error is tested through an apply, and the decisions reader's refusal is tested where it lives
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "quality: AGENTS.md's 022 routing entry names primitives/mod.rs as if it held every shared parser — `AGENTS.md:69`"
    outcome: discarded
    reason: the entry's catch-all, anything whose contract other specs rest on, already covers rule_sections.rs and parser/mod.rs
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "quality: a remote-tracking ref origin/ci/041-windows-check may survive locally — `AGENTS.md:87`"
    outcome: discarded
    reason: a local remote-tracking ref, not repository content and not 041's work
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: AGENTS.md entries restate 061's rationale and the recommendations rule, and carry dated measurements of 022's record and the file's size — `AGENTS.md:46`"
    outcome: discarded
    reason: each records what a rule cost here at a dated moment, which is what the file is for; classification to the constitution is 050's backlog
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "prose: prune.md's opening and step 5 are long, nested sentences — `framework/commands/prune.md:8`"
    outcome: discarded
    reason: readability of correct procedure text; the content is accurate after this run's fixes
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "claim: AC11 names only the --reset divergence while the design-record heading set has one too — `specs/041-task-pruning/spec.md:56`"
    outcome: discarded
    reason: AC11 is the tasks.md criterion; the plan's bounded divergence is stated in prune.md, analyze.md and the plan-records scenario
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "claim: plan.md narrates the change it makes (gains, is sharpened, is corrected) inside the design record — `specs/041-task-pruning/plan.md:178`"
    outcome: discarded
    reason: a plan describes the change it makes; these sentences state the design, not a history of revisions to it
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: the design-record headings are listed in 041's plan, data model and plan-records scenario beside the template — `specs/041-task-pruning/data-model.md:231`"
    outcome: discarded
    reason: the data model is the contract the drift test holds the runtime constant to, and the scenario records the decision that chose the set
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "stale: 058's spent tasks cite analyze step 16, and 058's plan keeps a now step 19 history sentence and line citations — `specs/058-findings-route-at-discovery/plan.md:351`"
    outcome: discarded
    reason: spent tasks are not rewritten, the history sentence was discarded in 048's review (f7c14890), and path:line citations are provenance AGENTS.md says not to sweep
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "stale: 060's plan cites analyze.md line numbers that moved, and a 058 scenario's Context describes the pre-fix behavior — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md:87`"
    outcome: discarded
    reason: path:line provenance, and a Context section records what motivated its scenario
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "stale: 041's spent task 2 says segmentation uses iter_phase_ranges — `specs/041-task-pruning/tasks.md:18`"
    outcome: discarded
    reason: a spent task's record is not rewritten; the plan and data model state the design
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "other: the plan.md and tasks.md filename literals are repeated with no shared constant (CFG-CONST-001 note) — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: a pre-existing pattern across 14 and 15 modules with no shared constant; introducing one is its own change
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: /amend's reconcile pass offers to reopen a done spec for any uncommitted tasks.md change — `framework/commands/amend.md`"
    outcome: discarded
    reason: it reopens only on the operator's confirmation, matching 023's recorded decision that the operator is the discriminator; 014 owns the delta, and 041's scenario records the boundary
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 072's out-of-scope note says changing the plan rule needs its own spec — `specs/072-governing-documents-stay-within-one-read/spec.md`"
    outcome: discarded
    reason: the note's subject, why 072 did not take the change, still holds
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 023's living-specs and 054 call every factual correction meaningful — `specs/023-govern-refinement/scenarios/living-specs.md`"
    outcome: discarded
    reason: both concern spec.md bodies or audit artifacts, never a plan or task list
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md`"
    outcome: discarded
    reason: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: the replay proof is true by construction for the new binary — `specs/041-task-pruning/tasks.md`"
    outcome: discarded
    reason: its proof is the old binary's half, which reopened all 18; the new half confirms the change landed
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "perf: analyze_freshness re-reads spec.md its callers already parsed — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: one small file read per call, negligible
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 063's Resolved Question says /prune reopens a spec only when it adds owed work — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`"
    outcome: discarded
    reason: "loose, not false, in that Resolved Question: prune reopens when the diff adds owed work"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "edge: an unreadable plan.md or tasks.md on a done spec is dropped like an edit in place and reads current — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: "contrived: no command writes an unreadable plan or task list"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "edge: an in-place plan or tasks edit that could produce a blocking analyze finding is still ignored on a done spec — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: "the chosen design: only a deletion is kept, and the one consumer is the advisory review row"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
---

# Review — 041-task-pruning

## Summary

Five passes over the window 865e97ae..88768fe6, reviewed with `--since 865e97ae`: 041's own diff base, the parent of 9b7ec76a, would pull item 2's already-reviewed work into the window, while every line of `a-plan-edit-does-not-reopen-a-done-spec` is in e3e54b07 and its fix rounds. The window is shared with 047 and 063, which e3e54b07 reopened, and was reviewed once in depth here; 066, reopened by the same commit, stays open for 058's gate change and is reviewed with it. An independent reviewer ran security, reuse, quality, efficiency and simplicity over e3e54b07 and 3ce51eb9 against api-, concurrency-, observability-, performance-, reliability- and security-backend plus configuration- and quality-cross (the runtime is a local CLI): 0 MUST, 0 SHOULD against a loaded rule, and 23 observations across three rounds — 13 fixed, 10 discarded with reasons. Round 1 (fbf2737c) changed code and was re-read independently, which raised one more SHOULD, fixed in round 2 (2f1c33c8); round 2's re-read found no MUST or SHOULD. The 23 stored decisions from 041's earlier windows were retained as unevaluated by this window. Examined: 44 of the 52 window files, every changed file but seven generated command and reference copies, which the reviewer diffed against their sources after substitution rather than read, and 88768fe6's 058 scenario, a clarify commit for separate work outside this review. `scope` counts 78 entries, each one path: the 52 window files, 23 plan Affected Files rows untouched since the base and not read, and the three review records this window writes for 041, 047 and 063, which are the passes' output rather than their subject.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 041's plan, data model and two scenario edge cases still described reopen-required's design-record trigger — `specs/041-task-pruning/plan.md` — **fixed**
- other: 063's Resolved Question ended on /prune reopening a spec when a moved decision edits the design record — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **fixed**
- other: restatements of the plan rule dropped its condition, adding no unchecked task, or said the comparison leaves the files out — `framework/reference/review.md` — **fixed**
- bug: a deleted plan.md or tasks.md on a done spec read current, though analyze reports a missing one as blocking — `runtime/src/primitives/analyze_subjects.rs` — **fixed**
- other: the plan.md and tasks.md filename literals are repeated with no shared constant (CFG-CONST-001 note) — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: a pre-existing pattern across 14 and 15 modules with no shared constant; introducing one is its own change
- other: the review reference said a plan correction leaves the analyze row current, true only of a record already current — `framework/reference/review.md` — **fixed**
- other: /amend's reconcile pass offers to reopen a done spec for any uncommitted tasks.md change — `framework/commands/amend.md` — **discarded**: it reopens only on the operator's confirmation, matching 023's recorded decision that the operator is the discriminator; 014 owns the delta, and 041's scenario records the boundary
- other: 072's out-of-scope note says changing the plan rule needs its own spec — `specs/072-governing-documents-stay-within-one-read/spec.md` — **discarded**: the note's subject, why 072 did not take the change, still holds
- other: 023's living-specs and 054 call every factual correction meaningful — `specs/023-govern-refinement/scenarios/living-specs.md` — **discarded**: both concern spec.md bodies or audit artifacts, never a plan or task list
- other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md` — **discarded**: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
- other: 013's and 016's plans carried further stale constitution claims outside the routed list — `specs/013-text-first-artifacts/plan.md` — **fixed**
- other: the replay proof is true by construction for the new binary — `specs/041-task-pruning/tasks.md` — **discarded**: its proof is the old binary's half, which reopened all 18; the new half confirms the change landed
- other: AGENTS.md's two-disposals entry contradicted the sentence appended to it — `AGENTS.md` — **fixed**
- other: the analyze reference's trim advice read as self-contradictory in one sentence — `framework/reference/analyze.md` — **fixed**
- test: a data-model.md trim at done was unpinned, so widening the plan-trim exemption to it survived — `runtime/src/primitives/check_artifact_size.rs` — **fixed**
- perf: analyze_freshness re-reads spec.md its callers already parsed — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: one small file read per call, negligible
- other: 016's plan item 2 still described a link to 008's data model and an Edge Case the round deleted — `specs/016-cross-cutting-rules/plan.md` — **fixed**
- other: without_plan_edits' doc comment said the commands that read tasks.md reopen a spec for an added task — `runtime/src/primitives/analyze_subjects.rs` — **fixed**
- other: 063's Resolved Question says /prune reopens a spec only when it adds owed work — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: loose, not false, in that Resolved Question: prune reopens when the diff adds owed work
- other: AGENTS.md's disposals entry left a task-list fix that adds a task uncovered — `AGENTS.md` — **fixed**
- other: 016's plan item 4 named cross-references §rules does not carry — `specs/016-cross-cutting-rules/plan.md` — **fixed**
- edge: an unreadable plan.md or tasks.md on a done spec is dropped like an edit in place and reads current — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: contrived: no command writes an unreadable plan or task list
- edge: an in-place plan or tasks edit that could produce a blocking analyze finding is still ignored on a done spec — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: the chosen design: only a deletion is kept, and the one consumer is the advisory review row

## Skipped passes

*None.*

## Unexamined governance

*None.*
