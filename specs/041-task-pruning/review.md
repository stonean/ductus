---
spec: 041-task-pruning
last-run: 2026-11-16T01:54:12Z
reviewed-against: 1edf32d27401567636dce89224e2f9b28d0a5214
diff-base: c2d62cf2dbabfe7dc6e4c731b30d61865435f4ee
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 9
scope: 22
unread:
  - AGENTS.md
  - framework/commands/prune.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/mod.rs
  - runtime/src/primitives/prune_plan.rs
  - runtime/src/primitives/prune_project.rs
  - runtime/src/primitives/prune_tasks.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/paths.rs
  - runtime/src/schema/primitives.rs
  - runtime/src/session.rs
  - specs/022-deterministic-runtime/plan.md
  - specs/022-deterministic-runtime/tasks.md
excluded: 70
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: a0070f57393f898ae607087b2de92769bee86dc9a3071d022f29b77485ce7e32
  scenarios/a-plan-edit-does-not-reopen-a-done-spec.md: d212af7304352b7080514a22d93fa67f76e350b7989670730c16045c50dc2051
  scenarios/a-pruned-task-number-is-never-reissued.md: 7d97f1b790d2b12d3b1630cc78f24eac2aff167c9ab932b436964f0f50205d06
  scenarios/a-suffixed-task-number-is-a-task.md: 362cf9e7efe74c12c39d20cd82d2347148695310611c9895b5d7694dad611f17
  scenarios/plan-records-the-design-as-it-stands.md: b3bfd54321552fcc9e32f91dfa5b24a21a923685937f661a38e78b8ed67d5658
  scenarios/prune-all-walks-every-spec.md: 0bcf1f2b448c089721deab6e96dba269082c23a092c0b8aaca56ee90244aefab
  scenarios/prune-reduces-the-spec-directory.md: c485de78348784233c118c59c649bc38d6b33ba4b6d9a05e0a8daa9c656f57c0
  scenarios/prune-walks-the-project-files.md: 8cb1a20f84195213bb7ddac0789d75a3a557a3872fdc480cf868115d06125a3e
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 4
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
  - key: "other — BE-COMPAT-001 classification lives in the CHANGELOG only: the new skip reason is a new value of an existing response enum (breaking under the rule) and written and refused are new fields (additive), while the commit message calls the two fields the breaking part; 041's plan and data model state no classification. The CHANGELOG classifies the change BREAKING for the enum value and the new error, which agrees with the rule. — `runtime/CHANGELOG.md:13`"
    outcome: waived
    reason: every runtime change carries its classification in the CHANGELOG by this project's convention, the CHANGELOG entry agrees with BE-COMPAT-001, and a commit message is history, not a live artifact
    decided-at: 2026-11-16T01:54:12Z
    decided-by: rule
  - key: perf — reduce copies tasks.md out of its snapshot (text().map(str::to_string)), so each feature in an all walk holds the file three times (snapshot bytes, content, new_content) until write_walk runs. — `runtime/src/primitives/prune_tasks.rs:406`
    outcome: waived
    reason: one copy of a task list of a few KB per spec, held for the length of one walk on a local call
    decided-at: 2026-11-16T01:54:12Z
    decided-by: rule
  - key: rule BE-TXN-002 at runtime/src/primitives/mod.rs
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    task: "38"
    reason: 022 task 38 puts append-task, mark-task and merge-managed-block on the guard, with 051 task 37 for rewrite-spec-links, so every runtime writer of a prune's files takes the compare-and-set lock (owner, 2026-11-16)
    decided-at: 2026-11-16T01:54:12Z
    decided-by: andy@stone.dev
---

# Review — 041-task-pruning

## Summary

041's review of task 27, the window from `c2d62cf2`, 041's last review, to `1edf32d2`: task 27's guard (`3e3707fa`), a `§` fix (`1edf32d2`), 041's hunk of `40df6e8b`, task 27's text and `7ab30b27`'s two AGENTS.md rules. One agent ran all five passes and read all nine loaded backend and cross rule files whole. It read 9 of the 22 scope entries whole: 041's spec, plan, data model and tasks, the three mutation lists and the two generated prune copies, each checked equal to its source; and the seven scenarios, outside the scope. The rest were read at the window's hunks with every function or section they touch whole: `ReadSnapshot`, `Stage`, `read_observed`, `write_within` and the new tests in `mod.rs`; `reduce`, `Reduction`, `plan_walk` and `write_walk`; the prune-plan and prune-project applies; the lock functions in `session.rs`; the three descriptions; the CHANGELOG entry; prune.md steps 5 to 10 and the Project files section; and AGENTS.md's hunk. It also read 022's plan §Compare before persist as part of the contract. One MUST, BE-TXN-002, deferred by the owner to 022 task 38: the compare-and-set is atomic only among writers that take its lock, and append-task, mark-task, merge-managed-block and rewrite-spec-links still write a prune's files without it; 022 task 38 and 051 task 37 put them on the guard. Its contract break, fixed in the run: 041's spec, data model, the helper's doc comment, the three descriptions, the CHANGELOG and 022's plan claimed atomicity among runtime writers; each now states the guard's own property, atomic among the writers that go through it, with any writer that does not as the residual. Three chores fixed: the plan's bounded-lists sentence miscounted its lists; the uncalled `lock_compare_and_set` wrapper is folded into one function taking the timeout; and the `changed-since-read` wire string is asserted, with mutant M14. The rule waived four observations with their reasons. Of the stored decisions on files the run read, 20 stand and are carried; three no longer fire and expire.

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

### DEFERRED: BE-TXN-002 — the compare-and-set is atomic only among writers that take the compare-and-set lock; runtime writers that do not (append-task, mark-task, mark-criterion, set-status) can still land between the re-read and the rename, and the stated lock order is held by no caller

- **File**: `runtime/src/primitives/mod.rs:1316-1450`
- **Rule**: A concurrent read-modify-write of persisted state MUST prevent lost updates — via an optimistic version/ETag check, a `SELECT … FOR UPDATE` row lock, an atomic in-place or conditional write (`UPDATE … SET n = n + 1`, compare-and-set), or an isolation level that detects the conflict and aborts one writer.
- **Finding**: write_within holds .ductus/locks/compare-and-set.lock from the re-read through the rename. append_task.rs line 90 takes only lock_append_task and writes with write_atomic; mark_task.rs, mark_criterion.rs and set_status.rs take no lock and write with write_atomic; no prune applier takes lock_append_task (grep of session::lock across runtime/src/primitives: only append_task.rs and mod.rs). So a prune-tasks apply and an append-task on one tasks.md are both runtime writers that exclude nothing between them: an append landing after the re-read and before the rename is replaced, and a prune landing between append-task's read and write is replaced by it. ReadSnapshot's doc comment, the PRUNE_TASKS, PRUNE_PLAN and PRUNE_PROJECT descriptions, 041 data-model.md The write guard, 041 plan.md and 022 plan.md Compare before persist all state the compare and rename are atomic among runtime writers and that a caller holding append-task's lock takes this one second, naming only a writer outside the runtime as the residual; no caller holds both locks, so that order and the BE-LOCK-002 order claim are vacuous, and the residual is wider than stated.
- **Auto-fixable**: no
- **Deferred**: to `specs/022-deterministic-runtime/tasks.md` task 38

## Observations

- bug — 041's own contract overclaims what the guard does: spec.md Edge Cases (Concurrent invocations) says that among runtime writers one project-wide lock makes the compare and the rename atomic, and tasks.md task 27 says a prune never drops an append-task, mark-task or other writer landing in the window. append-task, mark-task, mark-criterion and set-status are runtime writers that take no compare-and-set lock, so the code does not meet the sentence (see the BE-TXN-002 finding). The fix lands in 041's spec.md, data-model.md and plan.md, the runtime descriptions and doc comments, and 022's plan.md Compare before persist, which is another spec's directory. — `specs/041-task-pruning/spec.md:79` (breaks the spec's contract) — **fixed** (decided by the rule)
- other — plan.md's bounded-lists sentence says Three lists are not held to the read size but names four (stale-sections, stale-entries, created, kept); a count stated beside an enumeration drifts, and the constitution prefers a set stated without a count. — `specs/041-task-pruning/plan.md:279` — **fixed** (decided by the rule)
- other — dead code: session.rs lock_compare_and_set (pub) has no caller; ReadSnapshot::write_within calls lock_compare_and_set_within directly because it needs the test timeout. Only doc comments and 022's plan name it. — `runtime/src/session.rs:449` — **fixed** (decided by the rule)
- other — test gap: nothing asserts the wire string of the new skip reason, though prune.md step 7, data-model.md and the CHANGELOG tell the host to look for reason changed-since-read in skipped; the walk test compares the enum, so a serde rename of the variant would pass every test and break the host contract. — `runtime/src/schema/primitives.rs:8759` — **fixed** (decided by the rule)
- other — prune.md's Scope Boundaries and AC7 say prune writes no gitignored sidecar; every apply now also creates the gitignored lock file .ductus/locks/compare-and-set.lock (no content), beside the list files already waived. The lock file holds no pruned content, so recovery is still git history. — `framework/commands/prune.md:35` — **waived**: AC7 and the Scope Boundaries sentence bar a backup of pruned content; the list file holds each section's heading, size, digest and finding, never its text (PlanSection has no text field, checked in schema/primitives.rs), so pruned content stays recoverable only from git (decided by the rule)
- perf — the lock hold is stated as one re-read and one rename, but a path that has become a FIFO between the first read and the re-read would block std::fs::read while the project-wide compare-and-set lock is held, stalling every guarded writer until the 10 s wait elapses for each; a regular file is bounded. — `runtime/src/primitives/mod.rs:1439` — **waived**: contrived (an operator-placed FIFO at a project document in a local single-operator tool), pre-existing, and BE-TIMEOUT-001 leaves local file I/O alone (decided by the rule)
- other — BE-COMPAT-001 classification lives in the CHANGELOG only: the new skip reason is a new value of an existing response enum (breaking under the rule) and written and refused are new fields (additive), while the commit message calls the two fields the breaking part; 041's plan and data model state no classification. The CHANGELOG classifies the change BREAKING for the enum value and the new error, which agrees with the rule. — `runtime/CHANGELOG.md:13` — **waived**: every runtime change carries its classification in the CHANGELOG by this project's convention, the CHANGELOG entry agrees with BE-COMPAT-001, and a commit message is history, not a live artifact (decided by the rule)
- perf — reduce copies tasks.md out of its snapshot (text().map(str::to_string)), so each feature in an all walk holds the file three times (snapshot bytes, content, new_content) until write_walk runs. — `runtime/src/primitives/prune_tasks.rs:406` — **waived**: one copy of a task list of a few KB per spec, held for the length of one walk on a local call (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `.ductus/reference/fold.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `.pi/prompts/ductus-analyze.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `docs/analyze.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `framework/bootstrap/ductus-procedure.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `framework/commands/analyze.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `framework/reference/fold.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/interpreter/mod.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/main.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/bounded_lists.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_artifacts.rs` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_command_flags.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_corpus_links.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_departures.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_orphaned_references.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_promotion_coverage.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_review_gate.rs` — 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_rule_ids.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_unfolded_specs.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_dependencies.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_references.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/read_spec.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/resolve_anchor.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/rewrite_spec_links.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/traverse_deps.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/validate_frontmatter.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/audit/command-flag-hint-parity.sh` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/audit/deprecation-removal.sh` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/audit/promotion-coverage.sh` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/audit/shipped-anchor-resolution.sh` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/mutation/containment-and-bounds-067.py` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/mutation/corpus-link-containment.py` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/mutation/gate-cross-spec-containment.py` — 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `scripts/mutation/outward-link-reads-067.py` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/mutation/owner-reviews-067-checks.py` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/mutation/owner-reviews-067.py` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/mutation/reopen-measure.py` — 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `scripts/mutation/root-prefixed-references.py` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `scripts/tests/test-anchor-and-label-families.sh` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/026-framework-self-audit/tasks.md` — 026-framework-self-audit
- `specs/050-constitution/analysis.md` — 050-constitution
- `specs/050-constitution/plan.md` — 050-constitution
- `specs/050-constitution/review.md` — 050-constitution
- `specs/050-constitution/scenarios/the-constitution-fits-one-read.md` — 050-constitution
- `specs/050-constitution/spec.md` — 050-constitution
- `specs/051-branch-scoped-spec-numbering/spec.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/051-branch-scoped-spec-numbering/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/057-analyze-artifact-and-record-relocation/spec.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/057-analyze-artifact-and-record-relocation/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/review.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-cross-spec-impact-gate.md` — 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/data-model.md` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/plan.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/review.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/body-path-existence-family.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-historical-roots.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/shared-constitution-references-are-judged.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/sibling-link-grammar-is-the-shared-one.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/spec.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/080-deterministic-runtime-efficiency/analysis.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
