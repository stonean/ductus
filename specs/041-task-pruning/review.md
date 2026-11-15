---
spec: 041-task-pruning
last-run: 2026-11-15T21:51:31Z
reviewed-against: 09a6e973095e52d5b0963fee591074e3e2f9844c
diff-base: 6672d2f67b628ad149c3f0f6b4e55295b09f3a51
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 19
unread:
  - .claude/commands/ductus/analyze.md
  - .claude/commands/ductus/prune.md
  - .pi/prompts/ductus-analyze.md
  - .pi/prompts/ductus-prune.md
  - framework/commands/analyze.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/prune_tasks.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - scripts/mutation/prune-result-keys.py
  - specs/080-deterministic-runtime-efficiency/data-model.md
  - specs/080-deterministic-runtime-efficiency/spec.md
excluded: 71
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 9739cc9cc54c4a091386b593df35e4f752309f67dfa3ccc3cafe7dc63f3c5bb0
  scenarios/a-plan-edit-does-not-reopen-a-done-spec.md: d212af7304352b7080514a22d93fa67f76e350b7989670730c16045c50dc2051
  scenarios/a-pruned-task-number-is-never-reissued.md: 7d97f1b790d2b12d3b1630cc78f24eac2aff167c9ab932b436964f0f50205d06
  scenarios/a-suffixed-task-number-is-a-task.md: 362cf9e7efe74c12c39d20cd82d2347148695310611c9895b5d7694dad611f17
  scenarios/plan-records-the-design-as-it-stands.md: b3bfd54321552fcc9e32f91dfa5b24a21a923685937f661a38e78b8ed67d5658
  scenarios/prune-all-walks-every-spec.md: 0bcf1f2b448c089721deab6e96dba269082c23a092c0b8aaca56ee90244aefab
  scenarios/prune-reduces-the-spec-directory.md: c485de78348784233c118c59c649bc38d6b33ba4b6d9a05e0a8daa9c656f57c0
  scenarios/prune-walks-the-project-files.md: 8cb1a20f84195213bb7ddac0789d75a3a557a3872fdc480cf868115d06125a3e
blocking: false
dispositions:
  fixed: 9
  deferred: 0
  waived: 10
  undispositioned: 0
decisions:
  - key: "contract: the two flattened Option result halves allow neither or both; an untagged enum would keep exactly one — `runtime/src/schema/primitives.rs:2864`"
    outcome: waived
    reason: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; AGENTS.md records the flattened-halves design
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "efficiency: the prune-plan walk runs inline on the async worker and reads HEAD per done spec, even for specs it then drops — `runtime/src/mcp/server.rs:710`"
    outcome: waived
    reason: one HEAD read per done spec is not the unbounded revwalk the blocking pool is scoped to, and a spec it drops still needs the answer to know it can be dropped
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "simplicity: check_stuck converts the tasks path to a git path twice — `runtime/src/primitives/check_stuck.rs:91`"
    outcome: waived
    reason: the helper takes a project-relative path by design; the cost is one string build
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: three sha256-to-hex spellings across analyze_subjects, prune_plan and fetch_archive — `runtime/src/primitives/analyze_subjects.rs:202`"
    outcome: waived
    reason: consolidating them touches fetch-archive (048) for no change in behavior
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: inbox_standing re-implements the HEAD blob read, and prune_plan imports shared helpers from prune_tasks — `runtime/src/primitives/inbox_standing.rs:191`"
    outcome: waived
    reason: inbox_standing is the documented raw-repository exception, and where an import comes from changes no behavior
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "edge: split_frontmatter takes a later bare-LF --- as the fence when the frontmatter closes with CRLF — `runtime/src/primitives/mod.rs:857`"
    outcome: waived
    reason: no such mixed-ending file exists here, and the fix reaches every frontmatter reader for a case not seen
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: the prune-tasks and prune-plan walk skeletons are near-copies — `runtime/src/primitives/prune_plan.rs:210`"
    outcome: waived
    reason: the walks differ in what they list and how; a shared skeleton would be generic over both results for one loop
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "contract: an applying prune-tasks walk that errors on a later spec returns no record of the reductions it already wrote — `runtime/src/primitives/prune_tasks.rs:176`"
    outcome: waived
    reason: each write is atomic and idempotent, so re-running the walk resumes; the data model and plan now say so
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "edge: a reset also drops anything above the H1, and its H1 search scans fenced and commented lines — `runtime/src/primitives/prune_tasks.rs:477`"
    outcome: waived
    reason: a tasks.md opens with its H1 by construction from the template, and no file here has content above it or a heading only inside a fence
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "test: no prune-plan test that a preview on a done spec errors when HEAD cannot be read, or on a malformed decisions list — `runtime/src/primitives/prune_plan.rs:299`"
    outcome: waived
    reason: the preview and apply share one reopen computation, whose unreadable-HEAD error is tested through an apply, and the decisions reader's refusal is tested where it lives
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "quality: AGENTS.md's 022 routing entry names primitives/mod.rs as if it held every shared parser — `AGENTS.md:69`"
    outcome: waived
    reason: the entry's catch-all, anything whose contract other specs rest on, already covers rule_sections.rs and parser/mod.rs
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "quality: a remote-tracking ref origin/ci/041-windows-check may survive locally — `AGENTS.md:87`"
    outcome: waived
    reason: a local remote-tracking ref, not repository content and not 041's work
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: AGENTS.md entries restate 061's rationale and the recommendations rule, and carry dated measurements of 022's record and the file's size — `AGENTS.md:46`"
    outcome: waived
    reason: each records what a rule cost here at a dated moment, which is what the file is for; classification to the constitution is 050's backlog
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "prose: prune.md's opening and step 5 are long, nested sentences — `framework/commands/prune.md:8`"
    outcome: waived
    reason: readability of correct procedure text; the content is accurate after this run's fixes
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "claim: AC11 names only the --reset divergence while the design-record heading set has one too — `specs/041-task-pruning/spec.md:56`"
    outcome: waived
    reason: AC11 is the tasks.md criterion; the plan's bounded divergence is stated in prune.md, analyze.md and the plan-records scenario
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "claim: plan.md narrates the change it makes (gains, is sharpened, is corrected) inside the design record — `specs/041-task-pruning/plan.md:178`"
    outcome: waived
    reason: a plan describes the change it makes; these sentences state the design, not a history of revisions to it
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: the design-record headings are listed in 041's plan, data model and plan-records scenario beside the template — `specs/041-task-pruning/data-model.md:231`"
    outcome: waived
    reason: the data model is the contract the drift test holds the runtime constant to, and the scenario records the decision that chose the set
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "stale: 058's spent tasks cite analyze step 16, and 058's plan keeps a now step 19 history sentence and line citations — `specs/058-findings-route-at-discovery/plan.md:351`"
    outcome: waived
    reason: spent tasks are not rewritten, the history sentence was discarded in 048's review (f7c14890), and path:line citations are provenance AGENTS.md says not to sweep
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "stale: 060's plan cites analyze.md line numbers that moved, and a 058 scenario's Context describes the pre-fix behavior — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md:87`"
    outcome: waived
    reason: path:line provenance, and a Context section records what motivated its scenario
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "other: the plan.md and tasks.md filename literals are repeated with no shared constant (CFG-CONST-001 note) — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: waived
    reason: a pre-existing pattern across 14 and 15 modules with no shared constant; introducing one is its own change
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: /amend's reconcile pass offers to reopen a done spec for any uncommitted tasks.md change — `framework/commands/amend.md`"
    outcome: waived
    reason: it reopens only on the operator's confirmation, matching 023's recorded decision that the operator is the discriminator; 014 owns the delta, and 041's scenario records the boundary
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 072's out-of-scope note says changing the plan rule needs its own spec — `specs/072-governing-documents-stay-within-one-read/spec.md`"
    outcome: waived
    reason: the note's subject, why 072 did not take the change, still holds
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 023's living-specs and 054 call every factual correction meaningful — `specs/023-govern-refinement/scenarios/living-specs.md`"
    outcome: waived
    reason: both concern spec.md bodies or audit artifacts, never a plan or task list
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "perf: analyze_freshness re-reads spec.md its callers already parsed — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: waived
    reason: one small file read per call, negligible
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 063's Resolved Question says /prune reopens a spec only when it adds owed work — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`"
    outcome: waived
    reason: "loose, not false, in that Resolved Question: prune reopens when the diff adds owed work"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "edge: an unreadable plan.md or tasks.md on a done spec is dropped like an edit in place and reads current — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: waived
    reason: "contrived: no command writes an unreadable plan or task list"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "edge: an in-place plan or tasks edit that could produce a blocking analyze finding is still ignored on a done spec — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: waived
    reason: "the chosen design: only a deletion is kept, and the one consumer is the advisory review row"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other — reuse: tasks_counter::frontmatter_end hand-rolls the opening and closing fence split that `split_frontmatter_with_offset` (mod.rs:1024) provides (`content.len() - body.len()` is the same end offset). Not replaced, because the two differ on a closing fence followed by trailing whitespace (tasks_counter accepts `--- `, the shared one does not) and on the error (MalformedTasks naming the counter's cause, against UnclosedFrontmatter), and the data-model's malformed-tasks row and the compiled descriptions pin the first. — `runtime/src/primitives/tasks_counter.rs:19`"
    outcome: waived
    reason: the shared splitter differs in the two ways named, both of which the data model's malformed-tasks row and the descriptions fix for tasks.md; delegating would move behavior a caller reads, and the helper is 15 lines with its own tests
    decided-at: 2026-11-15T19:22:01Z
    decided-by: rule
  - key: other — prune-tasks reads and replaces tasks.md without append-task's advisory lock (022 task 35, BE-TXN-002), so a prune that read before an append and wrote after it drops the appended task and writes a counter from the stale read. 041's Edge Cases (`Concurrent invocations`) accepts last-writer-wins for a prune, and a feature's pipeline is serial within one agent process; 022's lock covers append-task against append-task. Read `lock_append_task` and the scenario before judging; mark-task, which also replaces tasks.md unlocked, is outside the window's hunks. — `runtime/src/primitives/prune_tasks.rs:297`
    outcome: waived
    reason: 041's Edge Cases (Concurrent invocations) states last-writer-wins for a prune as for every primitive that rewrites a spec artifact, and 022's ask-consolidation scenario scopes the lock to append-task against itself; BE-TXN-002 reads design-time commitments about shared persisted state, and the commitment is stated
    decided-at: 2026-11-15T19:22:01Z
    decided-by: rule
  - key: other — AC21 and the scenario bind `append-task`'s numbering, so after a `--reset` the counter is kept but `/plan`, which the spec's re-derivation contract has the operator run to repopulate, writes `## 1.` onward by hand and so assigns numbers the reset removed. The counter then sits above the new numbers (a gap, never a duplicate for a later append). The scenario's Behavior states `a task number in use before the prune is never assigned to a new task` generally, while its mechanism and AC21 bind only append-task. — `specs/041-task-pruning/scenarios/a-pruned-task-number-is-never-reissued.md:13`
    outcome: waived
    reason: AC21 and the owner's 2026-11-12 resolution bind append-task's numbering and rest on the counter, which derives from the artifact; /plan's hand-numbered tasks are outside both, and a reset the operator chose with --reset or --force is the stated destructive path
    decided-at: 2026-11-15T19:22:01Z
    decided-by: rule
  - key: "other — a symbolic-link loop at a project file reads `outside-project` though it leaves nothing. `validate_contained` answers InvalidPath for a link it cannot resolve, and `read_subject` and `search` map every InvalidPath to the outside state. Run on a fixture where AGENTS.md is a link to itself, the built binary printed state `outside-project`. The descriptions, data model and prune.md say \"resolving outside the project\". — `runtime/src/primitives/prune_project.rs:243`"
    outcome: waived
    reason: the file is neither read nor written either way, and prune.md has the host report it as not examined with its path, so the safe direction holds; a link loop is no file the walk could examine
    decided-at: 2026-11-15T19:22:01Z
    decided-by: rule
  - key: perf — `search` calls `validate_contained` for every candidate, and each call re-resolves the project root (`canonical_base`) and lstats each path component. On a fixture repository with 20,002 tracked files the whole preview took 0.59 s wall (0.05 s user, 0.54 s system), so the per-file check is not the cost that matters beside the read it guards. — `runtime/src/primitives/prune_project.rs:600`
    outcome: waived
    reason: measured on the built binary at 20,002 tracked files; `rewrite_spec_links` already calls the same function per referrer, and hoisting the root's resolution needs `check_links_on`, which is private to primitives/mod.rs
    decided-at: 2026-11-15T19:22:01Z
    decided-by: rule
  - key: "reuse — three sites map `validate_contained`'s InvalidPath to an 'outside the project' outcome and decide the other errors differently: `read_subject` (other errors read `unreadable`), `search` (other errors fail the call) and `rewrite_spec_links` (other errors fail the call). — `runtime/src/primitives/prune_project.rs:243`"
    outcome: waived
    reason: each site pairs the test with a different outcome and different handling of the other errors, so a shared helper would take a closure per caller for a two-arm match
    decided-at: 2026-11-15T19:22:01Z
    decided-by: rule
  - key: other — the search's `let Ok(bytes) = std::fs::read(..) else { continue; }` skips a tracked file that cannot be read (deleted from the worktree, unreadable) without naming it, so its identifiers' hits read as 'no literal occurrence found' for a file never read. `outside-project` now names the links; this skip stays silent. — `runtime/src/primitives/prune_project.rs:608`
    outcome: waived
    reason: pre-existing in 2f61d67d, which 075's review read; `files-searched` is the count of what was examined that QUAL-CLAIM-001 names as not flagged
    decided-at: 2026-11-15T19:22:01Z
    decided-by: rule
  - key: "bug — a FIFO at one of the four project files hangs `prune-project`: `std::fs::read` blocks on it, with no check that the path is a regular file. Run on a fixture where AGENTS.md was a FIFO, the built binary did not return before a 4 s alarm (exit 142). `rewrite_spec_links` refuses a non-regular file before opening it for the same reason. task 25's containment does not reach it. — `runtime/src/primitives/prune_project.rs:250`"
    outcome: waived
    reason: contrived (an operator-placed FIFO at a project document in a local single-operator tool), pre-existing, and BE-TIMEOUT-001 leaves local file I/O alone
    decided-at: 2026-11-15T19:22:01Z
    decided-by: rule
  - key: "perf — prune-plan's stale-sections is a list the result returns and the bound does not reach: it echoes one heading per entry of the caller's own remove list, so the result grows with the request, never with the corpus or the plan. Listing every collection the call returns: sections is bounded by place; stale-sections is bounded by the request; size-before and size-after are fixed; a walk's features and skipped are bounded whole by bound_walk. Task 26's done-when and the plan's enumeration read as if every list were held, though the plan's parenthesis names only three. — `runtime/src/primitives/prune_plan.rs:311`"
    outcome: waived
    reason: stale-sections holds at most one heading per remove entry the caller sent, so its size is set by the request the host built from the sections it had read; nothing in the corpus or the plan grows it, and moving it to a file would hide the one list the host must see on a refused apply
    decided-at: 2026-11-15T21:49:49Z
    decided-by: rule
  - key: other — 041's AC7 and prune.md's Scope Boundaries say prune writes no backup file or gitignored sidecar. With prune-plan's single-feature result bounded, prune now writes a list file under the gitignored results directory past the read size, as prune-tasks has since spec 080 and prune-project since 075. The wording is literally false for those runs. — `specs/041-task-pruning/spec.md:54`
    outcome: waived
    reason: AC7 and the Scope Boundaries sentence bar a backup of pruned content; the list file holds each section's heading, size, digest and finding, never its text (PlanSection has no text field, checked in schema/primitives.rs), so pruned content stays recoverable only from git
    decided-at: 2026-11-15T21:49:49Z
    decided-by: rule
  - key: perf — the single-feature bounding builds an owned serde_json value of every section through json! on every call, including the common inline one, before place measures the summary; place takes any Serialize, so the copy is only needed when the file is written. prune_tasks does the same. — `runtime/src/primitives/prune_plan.rs:181`
    outcome: waived
    reason: one copy of a list bounded by the plan's section count, a few hundred bytes a section, on a local call made a handful of times per run
    decided-at: 2026-11-15T21:49:49Z
    decided-by: rule
---

# Review — 041-task-pruning

## Summary

041's second review of its reopening. The window runs from `6672d2f6`, 041's last review, to `09a6e973`. Its commits are `749a5c08` (task 26: prune-plan's single-feature result is bounded) and two to `tasks.md` and `spec.md`. One agent ran all five passes over the scope, reading all nine loaded backend and cross rule files whole. It read 7 of the 19 scope entries whole: the contract (spec, plan, data model, tasks), prune-plan, the prune command and the new mutation list. The two generated prune copies were checked against their source, and the CHANGELOG, descriptions and schema were read at the window's hunks. The other unread entries are the files only this run's fix round changed, which its re-read reads. Fixed in the run: two contract breaks. Analyze step 16 read no prune-plan file form, and now does. The prune results' two flattened halves shared `form` and `lists-file`, so a file-form walk did not deserialize back. By the owner's decision of 2026-11-15 both keys are hoisted to the result, with round-trip tests in both forms; the bound is judged on the result as sent; prune-plan walk entries no longer carry `form`. Also fixed: prune step 2's file-form read, the data model's samples and errors table, PRUNE_PLAN's and PRUNE_TASKS' malformed-read-size halt, and the plan's Affected Files. The bounding copy is now one shared `bound_result`. 080's data model, which task 26 had left false, is corrected and 080 reopened (owner approval). Three observations were waived by the rule, and seven stored decisions still stand and are carried.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- bug — task 26's first criterion says every reader of prune-plan's result states or reads the file form, but analyze's step 16, the one other interactive reader, does not. It invokes prune-plan and records every section the result lists as an advisory finding, yet never says to read sections from the file lists-file names when form is file. In the file form sections is absent, so a host following step 16 as written records no plan-record finding for a plan large enough to take the file form, and nothing in the result says the list was not read. Steps 4, 5.1 and 17 of the same command each carry the clause. The exec walker is unaffected: restore_lists gives the lists back before analyze_tally counts them. Checked on a fixture with read-size-bytes 300: the built binary printed form file, sections-count 2, no sections key. — `framework/commands/analyze.md:79` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — PrunePlanSummary now carries form and lists-file, and PrunePlanResult flattens it beside PruneWalk, which carries the same two keys. 041's data model (Walking every spec, last paragraph) says the two halves share one key space so no key may appear in both, and docs/developing-ductus.md line 17 says the same and adds that serde's flatten consumes a shared key even when that half fails. So deserializing a file-form prune-plan walk gives the summary half the form and lists-file keys, which it then fails on (path is missing), and the walk half comes back as inline with no lists-file. Serialization, the wire and the walker are unaffected, and no runtime code deserializes a result; prune-tasks has had the same shape since spec 080. No test round-trips a file-form walk or a file-form summary, so nothing shows it. Reasoned from the project's own documented serde behavior and the struct definitions; not run, since cargo was off limits. — `runtime/src/schema/primitives.rs:4973` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — framework/commands/prune.md step 2 invokes prune-tasks and step 5 renders one line per task section, and under all a table from the walk, but step 2 never says to read sections, features and skipped from the file lists-file names when form is file. Step 3 gained that clause for prune-plan in this window, so the two steps now disagree. prune-tasks has returned the file form since spec 080: a corpus of about 190 specs or a tasks.md of a few hundred sections reaches it. A host following step 2 as written sees the lists absent and renders a clean preview. — `framework/commands/prune.md:47` — **fixed** (decided by the rule)
- other — 041's data model sample for prune-plan's result (the JSON block under Request/response schema) omits sections-count and form, which the binary now prints on every single-feature call and on every walk entry; the bullet below it describes them. The prune-tasks sample at the top of its schema section omits the same two keys, left from spec 080. Checked against the built binary on a fixture: sections-count and form inline follow the sections key and the stale-sections key respectively. — `specs/041-task-pruning/data-model.md:314` — **fixed** (decided by the rule)
- other — 041's data model prune-plan Operational errors table says the primitive writes nothing when any error fires and lists no read-size error, but this window added one: a malformed read size halts any call, an applying one before its removal. One exception to writes-nothing also exists: a list file that cannot be written after an applied removal fails the call and the removal stands, which prune_plan.rs's doc comment states and the prune-project section of the same data model states for itself. Checked by running the binary on a fixture whose config holds a malformed read-size-bytes: apply, preview and a missing feature each exit 1 with the invalid-read-size text, and plan.md was unchanged. — `specs/041-task-pruning/data-model.md:389` — **fixed** (decided by the rule)
- other — PRUNE_PLAN's compiled description does not state that a malformed read-size-bytes halts the call, though this window made that so, and the constitution makes a primitive's compiled description the canonical source of its behavior. derive-dependencies, derive-references and discover-rule-files each state it in theirs. PRUNE_TASKS lacks the sentence too, from spec 080. A change to the description needs the parity and golden checks the runtime workflow runs. — `runtime/src/schema/descriptions.rs:80` — **fixed** (decided by the rule)
- perf — prune-plan's stale-sections is a list the result returns and the bound does not reach: it echoes one heading per entry of the caller's own remove list, so the result grows with the request, never with the corpus or the plan. Listing every collection the call returns: sections is bounded by place; stale-sections is bounded by the request; size-before and size-after are fixed; a walk's features and skipped are bounded whole by bound_walk. Task 26's done-when and the plan's enumeration read as if every list were held, though the plan's parenthesis names only three. — `runtime/src/primitives/prune_plan.rs:311` — **waived**: stale-sections holds at most one heading per remove entry the caller sent, so its size is set by the request the host built from the sections it had read; nothing in the corpus or the plan grows it, and moving it to a file would hide the one list the host must see on a refused apply (decided by the rule)
- other — 041's AC7 and prune.md's Scope Boundaries say prune writes no backup file or gitignored sidecar. With prune-plan's single-feature result bounded, prune now writes a list file under the gitignored results directory past the read size, as prune-tasks has since spec 080 and prune-project since 075. The wording is literally false for those runs. — `specs/041-task-pruning/spec.md:54` — **waived**: AC7 and the Scope Boundaries sentence bar a backup of pruned content; the list file holds each section's heading, size, digest and finding, never its text (PlanSection has no text field, checked in schema/primitives.rs), so pruned content stays recoverable only from git (decided by the rule)
- reuse — prune_plan's run repeats prune_tasks's single-feature bounding block: build a lists object from the summary's sections, call place, and on a file set form, clear sections and set lists_file. Only the walk half was shared, as bound_walk. — `runtime/src/primitives/prune_plan.rs:180` — **fixed** (decided by the rule)
- perf — the single-feature bounding builds an owned serde_json value of every section through json! on every call, including the common inline one, before place measures the summary; place takes any Serialize, so the copy is only needed when the file is written. prune_tasks does the same. — `runtime/src/primitives/prune_plan.rs:181` — **waived**: one copy of a list bounded by the plan's section count, a few hundred bytes a section, on a local call made a handful of times per run (decided by the rule)
- convention — 041's plan Affected Files table lists the other mutation lists it created but not scripts/mutation/prune-plan-bounded.py, nor runtime/src/schema/descriptions.rs, which task 26 and the earlier tasks edit, so a review that selects its scope from the table's patterns does not name them. — `specs/041-task-pruning/plan.md:481` — **fixed** (decided by the rule)
- other — task 26 left 080's data model false: its Bounded results row for prune-plan named only the walk's lists, and the paragraph after the table said prune-plan's one-feature summary carries neither form nor lists-file (found by the parent while dispositioning the shared-keys contract break) — `specs/080-deterministic-runtime-efficiency/data-model.md` — **fixed** (decided by the rule)
- contract: the two flattened Option result halves allow neither or both; an untagged enum would keep exactly one — `runtime/src/schema/primitives.rs:2864` — **waived**: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; AGENTS.md records the flattened-halves design
- reuse: the prune-tasks and prune-plan walk skeletons are near-copies — `runtime/src/primitives/prune_plan.rs:210` — **waived**: the walks differ in what they list and how; a shared skeleton would be generic over both results for one loop
- test: no prune-plan test that a preview on a done spec errors when HEAD cannot be read, or on a malformed decisions list — `runtime/src/primitives/prune_plan.rs:299` — **waived**: the preview and apply share one reopen computation, whose unreadable-HEAD error is tested through an apply, and the decisions reader's refusal is tested where it lives
- prose: prune.md's opening and step 5 are long, nested sentences — `framework/commands/prune.md:8` — **waived**: readability of correct procedure text; the content is accurate after this run's fixes
- claim: AC11 names only the --reset divergence while the design-record heading set has one too — `specs/041-task-pruning/spec.md:56` — **waived**: AC11 is the tasks.md criterion; the plan's bounded divergence is stated in prune.md, analyze.md and the plan-records scenario
- claim: plan.md narrates the change it makes (gains, is sharpened, is corrected) inside the design record — `specs/041-task-pruning/plan.md:178` — **waived**: a plan describes the change it makes; these sentences state the design, not a history of revisions to it
- reuse: the design-record headings are listed in 041's plan, data model and plan-records scenario beside the template — `specs/041-task-pruning/data-model.md:231` — **waived**: the data model is the contract the drift test holds the runtime constant to, and the scenario records the decision that chose the set

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/analyze.md` — 067-analyze-and-audit-checks
- `.ductus/reference/audit.md` — 067-analyze-and-audit-checks
- `.githooks/pre-commit` — 067-analyze-and-audit-checks
- `.github/workflows/framework-checks.yml` — 067-analyze-and-audit-checks
- `framework/reference/analyze.md` — 067-analyze-and-audit-checks
- `framework/reference/audit.md` — 067-analyze-and-audit-checks
- `runtime/src/interpreter/payload.rs` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `runtime/src/primitives/bounded_lists.rs` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_artifacts.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/check_corpus_links.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/check_departures.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/mod.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/read_deprecation_labels.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/read_section.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/resolve_anchor.rs` — 067-analyze-and-audit-checks
- `runtime/src/schema/extensions.rs` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/tests/walker.rs` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `scripts/audit/README.md` — 026-framework-self-audit, 067-analyze-and-audit-checks
- `scripts/audit/analyze-record-backlog.sh` — 026-framework-self-audit
- `scripts/audit/deprecation-removal.sh` — 026-framework-self-audit, 067-analyze-and-audit-checks
- `scripts/audit/done-spec-criteria.sh` — 026-framework-self-audit
- `scripts/audit/feature-dir-grammar-parity.sh` — 067-analyze-and-audit-checks
- `scripts/audit/fixture-session-shape.sh` — 026-framework-self-audit
- `scripts/audit/host-namespace-parity.sh` — 026-framework-self-audit
- `scripts/audit/installer-command-parity.sh` — 026-framework-self-audit
- `scripts/audit/introducing-drift.sh` — 026-framework-self-audit
- `scripts/audit/lib.sh` — 026-framework-self-audit
- `scripts/audit/manifest-destination-links.sh` — 026-framework-self-audit
- `scripts/audit/migration-coverage.sh` — 026-framework-self-audit
- `scripts/audit/placeholder-roundtrip.sh` — 026-framework-self-audit
- `scripts/audit/primitive-promotion-candidates.sh` — 026-framework-self-audit
- `scripts/audit/readme-command-parity.sh` — 026-framework-self-audit
- `scripts/audit/rename-sweep-residue.sh` — 026-framework-self-audit
- `scripts/audit/run-all.sh` — 067-analyze-and-audit-checks
- `scripts/audit/shipped-anchor-resolution.sh` — 026-framework-self-audit, 067-analyze-and-audit-checks
- `scripts/audit/sibling-coupling.sh` — 026-framework-self-audit
- `scripts/lint-frontmatter.sh` — 067-analyze-and-audit-checks
- `scripts/mutation/anchor-and-label-results-bounded.py` — 067-analyze-and-audit-checks
- `scripts/mutation/corpus-link-containment.py` — 067-analyze-and-audit-checks
- `scripts/mutation/criterion-body-path-links.py` — 067-analyze-and-audit-checks
- `scripts/mutation/exec-requests-bounded.py` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `scripts/mutation/scope-results-bounded.py` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `scripts/tests/test-anchor-and-label-families.sh` — 067-analyze-and-audit-checks
- `scripts/tests/test-audit-path-containment.sh` — 026-framework-self-audit
- `scripts/tests/test-deprecation-removal.sh` — 026-framework-self-audit
- `scripts/tests/test-feature-dir-grammar-parity.sh` — 067-analyze-and-audit-checks
- `scripts/tests/test-migration-coverage.sh` — 026-framework-self-audit
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `specs/026-framework-self-audit/analysis.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/review.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-10-migration-coverage.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-24-rename-sweep-residue.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-27-done-spec-unchecked-criteria.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-35-manifest-destination-links.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-39-shipped-anchor-resolution.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/host-namespace-parity.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/readme-command-parity.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/tasks.md` — 026-framework-self-audit
- `specs/066-audit-records-and-the-completion-gate/analysis.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate
- `specs/067-analyze-and-audit-checks/data-model.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/plan.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks

## Unexamined governance

*None.*
