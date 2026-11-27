---
spec: 041-task-pruning
last-run: 2026-11-27T18:44:48Z
reviewed-against: 6bdca096442f3bdd2719d682beb6f23d29a4a4f2
diff-base: 8dc7d231d1046f9fb82c38b790380047ddff85bf
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 11
scope: 13
unread:
  - runtime/CHANGELOG.md
  - runtime/src/schema/primitives.rs
excluded: 634
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: e712af6d72f75fbe6462598afad7e179e75408b47ee8f4cef8e738c7f1d2de4f
  scenarios/a-plan-edit-does-not-reopen-a-done-spec.md: 24af928b2434d298b9d3b1e9fe389193e4488a61ce1af643ac3bfb4094e7ac72
  scenarios/a-pruned-task-number-is-never-reissued.md: 7d97f1b790d2b12d3b1630cc78f24eac2aff167c9ab932b436964f0f50205d06
  scenarios/a-suffixed-task-number-is-a-task.md: 1e9ba09f8f209040b965e43d0276f8037baa46c811c0ea60b811738f7a6b5181
  scenarios/an-open-fenced-block-or-comment-is-not-examined.md: 4f9252380ba21bb4bd80e3f0ca308a75cc3fe001b7f688ecae7d1a84f08a00c5
  scenarios/plan-records-the-design-as-it-stands.md: b3bfd54321552fcc9e32f91dfa5b24a21a923685937f661a38e78b8ed67d5658
  scenarios/prune-all-walks-every-spec.md: cf3a0fea6a054194b629cc0fa7e81251eeadddf73c39952608790444209f7712
  scenarios/prune-reduces-the-spec-directory.md: 7f72e5a288972a1164f79ed2cebe6a22e1f96cdc05687f5143d1155ac5d6cd6f
  scenarios/prune-walks-the-project-files.md: 381e0753fb99acac13e8a2af2c4ed79fd948f037dc7d87b7867a72402ac848e8
criteria-judged: 21
criteria-met: 21
criteria-verdicts:
  - label: AC1
    digest: 9f050a47d6b52ba69ad98dca64013b58fcec10897a64db63cb5d081b007c4944
  - label: AC2
    digest: c80b5006d5e9027514b4398d238a32fdeea50bf46757e161c919607a7a78f524
  - label: AC3
    digest: 34c9290f480390def128cd8b87f020cfa5626b9f643e1d50ea4e21894b15aaa0
  - label: AC4
    digest: eec18ed499c9a988f5eb4c4bf3e640c19b197768b6dfef087a45e2a2092211e0
  - label: AC5
    digest: 6fd5f15fc8ec2788feb3b7184e7ca7ba376c148c5c4135ea3614ecf7e9c52865
  - label: AC6
    digest: 9705c5b6ee10f7cf1096fdd2dd4132a5c206ddbdaf74a12427e2eaf046c4082c
  - label: AC7
    digest: c80bf0600e0deb0dded57acb4c29cf8fb9f7fa74d88ba177bb62a2f4152833d5
  - label: AC8
    digest: bd4e3a8c6875707047e32acb7086f8af49ad631fb9af85417af8f83a2cdf1704
  - label: AC9
    digest: 10b94c63e5e4a76948e822e8daff5cadd734b80851e4a0bdaa63c2b77caca706
  - label: AC10
    digest: e8b21b31ef579797acfecd644debbf3063aed9702a265a2fbb7ca91bc11f2835
  - label: AC11
    digest: cf3886c3b6bcadfb22577c9fbf2779a3139265ab911bb3c1663bc8efc89ba08f
  - label: AC12
    digest: 7585d9d0936462194fbe2300536325059eb61a1e3f3e92b6a92fd578102d5082
  - label: AC13
    digest: 1a1f907788205a43f9f4b0953c19a8954ea2f0f567fbc3832a4f4271d76797a6
  - label: AC14
    digest: a61d9089be169aea9fee866b809d6adaf92a1070c8733aa13953285e4272082c
  - label: AC15
    digest: fc32216d371a64363fc012302974f73b671d335fa2689c5f5b68df476a05033e
  - label: AC16
    digest: f7919d0e9a33d3590016a4d662665b92f7fede79775f87baa21d4a118a98b398
  - label: AC17
    digest: e58fb58b1d607efc59e5ce0202f08c76dafe2c997cf45ce4b646b00ce5d9527b
  - label: AC18
    digest: d2032eb9382fc36598f77ef4907b1afc8281c807bf4215b1f7c0f135f0345642
  - label: AC19
    digest: 98b829cb1da1d8de585b30b61afb1e2f3424968d840986329ec33ffb553c5555
  - label: AC20
    digest: de03f1ce630ff02e0501143c417418bde819d7a320b0520dc34456d737c529aa
  - label: AC21
    digest: 72c83d3baf107c61f90402e59a68b10ce4e27720284bfd284aba11cc5a488c35
blocking: false
dispositions:
  fixed: 6
  deferred: 0
  waived: 12
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
  - key: "perf: analyze_freshness re-reads spec.md its callers already parsed — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: waived
    reason: one small file read per call, negligible
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
  - key: "test: `scripts/mutation/prune-open-region.py` has no mutant for the frontmatter line offset of `OpenRegion::tail` or for prune-plan's `.tail(0)` — `scripts/mutation/prune-open-region.py`"
    outcome: waived
    reason: the tests the_tail_line_counts_the_frontmatter and the plan tail test assert the figure, so a mutant would be redundant
    decided-at: 2026-11-20T00:43:45Z
    decided-by: rule
  - key: "pipeline-machinery: `/analyze` step 16's sentence about `unexamined-tail` has no record field to land in, since `UNEXAMINED_REASONS` in `runtime/src/primitives/write_analysis.rs` has no reason for a partly examined plan, in `framework/commands/analyze.md` — `framework/commands/analyze.md`"
    outcome: waived
    reason: the analysis record's closed reason set belongs to 066 and 067, and the step reports the partial examination in the run
    decided-at: 2026-11-20T00:43:45Z
    decided-by: rule
  - key: "quality: each prune apply leaves 0-byte gitignored lock files under `.ductus/locks/`, which AC7's no gitignored sidecar might be read to exclude, from the write guard in `runtime/src/primitives/mod.rs` — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: the lock files hold no pruned content, and AC7 bars a backup of it
    decided-at: 2026-11-20T00:43:45Z
    decided-by: rule
  - key: "quality: the last sentence of `PRUNE_PROJECT` in `runtime/src/schema/descriptions.rs` can be read as saying a later rewrite's re-check writes nothing, which only the home-to-create check guarantees — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: "low confidence: the sentence's nearest antecedent is the home to create, which it states correctly, and the wording predates this window"
    decided-at: 2026-11-20T00:43:45Z
    decided-by: rule
  - key: "reuse: `PruneReviewWalk` repeats the fields of `PruneWalk<T>` — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: its skipped entries are `ReviewSkipped` and it carries no unexamined count, so sharing `PruneWalk` would take a second type parameter for its one caller; the type is spec 086's `prune-review`'s
    decided-at: 2026-11-27T18:42:31Z
    decided-by: rule
---

# Review — 041-task-pruning

## Summary

041's review since its 2026-11-19 run, over the window from `8dc7d231` (`base-source: last-review`) to `6bdca096`: a scope of 13 entries, four of them code its last review left unread. Two passes parts, by behaviour (prune-project; prune-tasks, prune-plan and 041 as specified), each ran all five passes against the five rule files `discover-rule-files` selected, read whole (231,962 bytes), and one criteria part judged all 21 criteria against the code wherever it lies, driving the binary on clones of `6bdca096`; AC9, AC15 and AC19, whose text 086 and 084 changed, owed this review. Read whole: `runtime/src/primitives/prune_project.rs`, `scripts/mutation/prune-project.py`, `spec.md`, `plan.md`, `data-model.md`, `analysis.md` and the five changed scenarios. Read in part, and so unread: `runtime/src/schema/primitives.rs` (574,382 bytes; read 041's 13 blamed ranges with 8 lines of context and the `prune-review` types, 86,136 bytes) and `runtime/CHANGELOG.md` (665,252 bytes; read the Unreleased section and 041's 15 blamed ranges in released history, 81,540 bytes), the rest of both being other specs' code (the agent's recommendation, adopted by the operator, 2026-11-27). Two paths deleted since the base, `docs/developing-ductus.md` and `runtime/src/primitives/classify_spec_edit.rs`, could not be read by any pass. No MUST or SHOULD finding; 21 of 21 criteria met. The Unreleased section had no entry for spec 087's runtime changes, and by the owner's decision (the agent's recommendation, adopted by the operator, 2026-11-27) this run writes one, an Added entry and a BREAKING Changed entry, classified as 087's plan classifies them. Of the 41 stored decisions, 11 on in-scope files matched, their issues still standing, 24 on files outside the scope that 041 owns were retained, 5 on files 041 does not own were dropped, and 1, a deferral recorded before sites, expired. `AGENTS.md`'s Gotchas to Design Principles were read from the reviewers' loaded project instructions, not re-read. Its one fix round, the changelog entries, the data model and two scenarios, was re-read clean by an independent agent that read the spec, data model and every scenario whole; the round's `plan.md` edits were exempt, an artifact edit `/ductus:analyze` reads again.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

18 of 577 commits in the window no spec claimed.

21 of 21 acceptance criteria judged met.

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

## Acceptance criteria

- AC1 — met: `framework/commands/prune.md` exists; steps 1 to 3 resolve the session target and call `prune-tasks` and `prune-plan` on its `tasks.md` and `plan.md`.
- AC2 — met: Probe clone: `prune-tasks` with `apply` dropped 2 spent sections of 3, 232 to 156 bytes, and its preview on `specs/041-task-pruning/tasks.md` read 6,653 to 122 bytes; test `keep_pending_drops_spent_preserves_pending_and_prose`.
- AC3 — met: Probe clone: the section with an unchecked box stayed byte-for-byte (`reduce_keep_pending` in `prune_tasks.rs` keeps every block not spent); tests `keep_pending_drops_spent_preserves_pending_and_prose` and `keep_pending_keeps_a_phase_it_did_not_empty`.
- AC4 — met: Probe clone: the output starts with the `next-task:` frontmatter, then the template's heading; `markdownlint-cli2` reports 0 issues on it and `check-artifacts` no task-consistency finding.
- AC5 — met: `framework/commands/prune.md` steps 7, 9 and 10 call `gate-confirm` before each apply, and a decline leaves the file unchanged; `prune-tasks` without `apply` writes nothing (test `preview_does_not_write`).
- AC6 — met: Probe clone: `--reset` on a `done` spec left the frontmatter (`next-task: 4`), the heading and the guidance comment with no task entry; tests `reset_produces_template_state_when_done` and `canonical_empty_body_matches_template`.
- AC7 — met: No backup is written (`prune_tasks.rs` writes only `tasks.md`); the one gitignored file is the result's lists file under `.ductus/results/`, holding headings, sizes and digests, never text (`bound_result`), its wording covered by 041's stored waiver.
- AC8 — met: Probe clone: `--reset` with `apply` on an `in-progress` spec returned `blocked-needs-force` and left `tasks.md` unchanged, and with `--force` wrote; `framework/commands/prune.md` step 6 names the status; tests `reset_blocked_on_non_done_without_force` and `reset_forced_on_non_done_writes`.
- AC9 — met: On its whole text: `reduce()` reports nothing to prune and applies nothing when the content is unchanged, and `framework/commands/prune.md` step 6 stops so only when `tasks.md`, the plan and `review.md` all have nothing; test `a_record_with_nothing_to_drop_or_rewrite_is_not_written`.
- AC10 — met: `framework/commands/prune.md` step 1 stops with no target and directs to `/{project}:target`, and steps 2 and 3 with neither file direct to `/{project}:plan`; `prune_tasks.rs` returns distinct `FeatureNotFound` and `TasksFileMissing` errors.
- AC11 — met: `prune-tasks` and `prune-plan` write through `apply` with no body in any result; a reset writes the compiled `CANONICAL_EMPTY_TASKS_BODY` (test `canonical_empty_body_matches_template`), never the project's template; no markdown-only fallback remains.
- AC12 — met: `framework/constitution.md` §tasks-phase: `tasks.md` is an ephemeral work-tracking artifact, distinct from the durable `spec.md`, `data-model.md`, scenarios and rules.
- AC13 — met: Probe clone: `read-tasks` on a reset `tasks.md` returned no task despite the commented examples; test `reset_output_parses_as_zero_tasks` has `append-task` number 1 with no counter.
- AC14 — met: `check_artifacts.rs`' scenario consistency reads `tasks.md` history, so a pruned task satisfies its scenario (tests `a_task_pruned_by_keep_pending_satisfies_the_mapping`, `a_task_removed_by_a_reset_satisfies_the_mapping`); `framework/reference/analyze.md` agrees.
- AC15 — met: On its whole text: `framework/commands/prune.md` step 9, the plan-section step since 086's renumbering, proposes each piece's home, confirms the moves and the removal together through `gate-confirm`, and keeps a declined section byte-for-byte.
- AC16 — met: Probe clone: a `prune-plan` preview listed a section with its lines, bytes and digest and no text, and an apply with a wrong digest returned `stale-sections` and wrote nothing; tests `a_heading_inside_a_fence_or_a_comment_is_not_a_section`, `headings_compare_case_insensitively`, `a_digest_mismatch_refuses_the_whole_apply`.
- AC17 — met: `prune_plan.rs` `outside_record` marks a section decided only for a person's stored waiver under its plan-record key, and `framework/commands/prune.md` step 5 lists it as kept; tests `a_stored_waiver_sets_decided` and `a_waiver_the_rule_made_does_not_set_decided`.
- AC18 — met: `framework/commands/analyze.md` step 16 invokes `prune-plan` and records every listed section at `planned` or later as an advisory finding, its message built by `plan_section_finding` in `prune_plan.rs`.
- AC19 — met: On its whole current text: probe clone, a plan section's removal on a `done` spec gave `reopen-required` false and an appended unchecked task true, and `check-owed-work` owed; `framework/commands/prune.md` step 11 reopens only on owed work, naming it first.
- AC20 — met: `framework/constitution.md` §plan-phase states the design-as-it-stands rule and the design record; `framework/templates/spec/plan.md` holds every section `/{project}:plan` fills; `framework/templates/spec/tasks.md`'s comment allows working notes as prose on the task that must wait.
- AC21 — met: Probe clone: after a prune left `next-task: 4`, `append-task` assigned 4 then 5 and advanced the counter; tests `a_counter_past_the_highest_number_is_assigned_and_advanced`, `a_counter_at_or_below_the_highest_number_never_duplicates`, `a_reset_keeps_the_counter_and_a_file_without_one_gains_none`.

## Observations

- contract: `PruneTasksResult` and `PrunePlanResult` flatten two optional halves, so neither or both can be built — `runtime/src/schema/primitives.rs` — **waived**: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; AGENTS.md records the flattened-halves design
- claim: AC11 names only the `--reset` divergence from a customized template, not the plan template's heading-set divergence — `specs/041-task-pruning/spec.md` — **waived**: AC11 is the tasks.md criterion; the plan's bounded divergence is stated in prune.md, analyze.md and the plan-records scenario
- claim: `plan.md`'s Constitution, templates and docs section still narrates the change it makes (gains, is sharpened) — `specs/041-task-pruning/plan.md` — **waived**: a plan describes the change it makes; these sentences state the design, not a history of revisions to it
- reuse: the design-record headings are listed in 041's plan, data model and plan-records scenario beside the plan template — `specs/041-task-pruning/data-model.md` — **waived**: the data model is the contract the drift test holds the runtime constant to, and the scenario records the decision that chose the set
- other: a symbolic-link loop at a project file still reads `outside-project` — `runtime/src/primitives/prune_project.rs` — **waived**: the file is neither read nor written either way, and prune.md has the host report it as not examined with its path, so the safe direction holds; a link loop is no file the walk could examine (decided by the rule)
- perf: `search` still calls `validate_contained` once per candidate, re-resolving the project root each time — `runtime/src/primitives/prune_project.rs` — **waived**: measured on the built binary at 20,002 tracked files; `rewrite_spec_links` already calls the same function per referrer, and hoisting the root's resolution needs `check_links_on`, which is private to primitives/mod.rs (decided by the rule)
- reuse: the sites mapping `validate_contained`'s `InvalidPath` to an outside-the-project outcome still decide the other errors each its own way — `runtime/src/primitives/prune_project.rs` — **waived**: each site pairs the test with a different outcome and different handling of the other errors, so a shared helper would take a closure per caller for a two-arm match (decided by the rule)
- other: `search` still skips a tracked file it cannot read without naming it — `runtime/src/primitives/prune_project.rs` — **waived**: pre-existing in 2f61d67d, which 075's review read; `files-searched` is the count of what was examined that QUAL-CLAIM-001 names as not flagged (decided by the rule)
- edge: a FIFO at a project file still has no regular-file check before `std::fs::read` — `runtime/src/primitives/prune_project.rs` — **waived**: contrived (an operator-placed FIFO at a project document in a local single-operator tool), pre-existing, and BE-TIMEOUT-001 leaves local file I/O alone (decided by the rule)
- other: AC7 still says prune writes no gitignored sidecar, while each prune result writes a lists file past the read size — `specs/041-task-pruning/spec.md` — **waived**: AC7 and the Scope Boundaries sentence bar a backup of pruned content; the list file holds each section's heading, size, digest and finding, never its text (PlanSection has no text field, checked in schema/primitives.rs), so pruned content stays recoverable only from git (decided by the rule)
- other: 041's `BE-COMPAT-001` classification still lives only in `runtime/CHANGELOG.md` — `runtime/CHANGELOG.md` — **waived**: every runtime change carries its classification in the CHANGELOG by this project's convention, the CHANGELOG entry agrees with BE-COMPAT-001, and a commit message is history, not a live artifact (decided by the rule)
- convention: the project-files scenario said the catalog search covers tracked files outside the spec root, while `search` in `prune_project.rs` also sets the project files and their parts aside; the scenario now says so — `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md` — **fixed** (decided by the rule)
- convention: the Unreleased section of `runtime/CHANGELOG.md` carried no entry for spec 087's runtime changes; an Added and a BREAKING Changed entry now state them, classified as 087's plan classifies them — `runtime/CHANGELOG.md` — **fixed**
- convention: `plan.md` cited `prune.md` step 8 twice for the plan-section routing, step 9 since 086's renumbering, and analyze's steps 17 to 21 for what are now 17 to 22; corrected — `specs/041-task-pruning/plan.md` — **fixed** (decided by the rule)
- convention: `plan.md` gave rmcp 2.2.0's refusal of a non-object output schema as the reason for the flattened result halves, while the locked rmcp 3.11.0 allows one and strips it only for a peer on an older protocol; the reason is restated — `specs/041-task-pruning/plan.md` — **fixed** (decided by the rule)
- convention: `plan.md`'s Overview and write-guard paragraph and `data-model.md`'s opening counted two or three prune primitives where the tree and the data model's later sections hold four; corrected, with no `prune_review.rs` row added to Affected Files, since 086's plan holds it — `specs/041-task-pruning/plan.md` — **fixed** (decided by the rule)
- convention: `prune-reduces-the-spec-directory.md` closed with an implementation instruction, done, in the present tense; it is put in the past tense — `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md` — **fixed** (decided by the rule)
- reuse: `PruneReviewWalk` repeats the fields of `PruneWalk<T>` — `runtime/src/schema/primitives.rs` — **waived**: its skipped entries are `ReviewSkipped` and it carries no unexamined count, so sharing `PruneWalk` would take a second type parameter for its one caller; the type is spec 086's `prune-review`'s (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md`
- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/plan.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.claude/commands/ductus/status.md`
- `.claude/commands/ductus/target.md`
- `.ductus/docs/agents/developing-ductus.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/prune.md`
- `.ductus/reference/review.md`
- `.gitattributes`
- `.githooks/pre-commit`
- `.githooks/pre-push`
- `.github/workflows/framework-checks.yml`
- `.github/workflows/runtime-currency.yml`
- `.github/workflows/runtime-release.yml`
- `.github/workflows/runtime.yml`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-plan.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `.pi/prompts/ductus-status.md`
- `.pi/prompts/ductus-target.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `docs/rules.md`
- `docs/shared-constitution.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus-procedure.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/bootstrap/hooks/ductus-pre-commit`
- `framework/bootstrap/hooks/ductus-pre-push`
- `framework/bootstrap/hooks/pre-push`
- `framework/commands/amend.md`
- `framework/commands/analyze.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/groom.md`
- `framework/commands/help.md`
- `framework/commands/implement.md`
- `framework/commands/plan.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/commands/status.md`
- `framework/commands/target.md`
- `framework/constitution.md`
- `framework/migrations.toml`
- `framework/migrations/audit-record-relocate.md`
- `framework/migrations/criterion-label-backfill.md`
- `framework/migrations/project-review-record.md`
- `framework/migrations/remove-legacy-constitutions-region.md`
- `framework/migrations/report-project-file-links-into-docs.md`
- `framework/migrations/section-digest-backfill.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/fold.md`
- `framework/reference/groom.md`
- `framework/reference/implement.md`
- `framework/reference/plan.md`
- `framework/reference/prune.md`
- `framework/reference/review.md`
- `framework/rules/accessibility-frontend.md`
- `framework/rules/api-backend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/reliability-backend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-cross.md`
- `framework/rules/security-frontend.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `framework/templates/spec/spec.md`
- `install.sh`
- `runtime/Cargo.lock`
- `runtime/Cargo.toml`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/append_question.rs`
- `runtime/src/primitives/backfill_section_digest.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_audit_records.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_departures.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_prescriptions.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_rule_ids.rs`
- `runtime/src/primitives/check_stuck.rs`
- `runtime/src/primitives/check_unreviewed_code.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/criterion_verdicts.rs`
- `runtime/src/primitives/dashboard.rs`
- `runtime/src/primitives/decisions.rs`
- `runtime/src/primitives/departures.rs`
- `runtime/src/primitives/derive_references.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/invalidate_review.rs`
- `runtime/src/primitives/label_criteria.rs`
- `runtime/src/primitives/line_citations.rs`
- `runtime/src/primitives/lint_markdown.rs`
- `runtime/src/primitives/mark_criterion.rs`
- `runtime/src/primitives/mark_task.rs`
- `runtime/src/primitives/measure_reopenings.rs`
- `runtime/src/primitives/mechanical_sweep.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/merge_permissions.rs`
- `runtime/src/primitives/merge_status_hook.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/owed_work.rs`
- `runtime/src/primitives/pipeline_view.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/project_parts.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_project/moves.rs`
- `runtime/src/primitives/prune_review.rs`
- `runtime/src/primitives/read_re_read.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/read_tasks.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/remove_inbox_item.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/review_subject.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/set_status.rs`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `runtime/src/primitives/spec_ownership.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/moved_scope.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/src/primitives/write_review/seed.rs`
- `runtime/src/schema/config_keys.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/extensions.rs`
- `runtime/src/schema/paths.rs`
- `runtime/src/schema/protocol.rs`
- `runtime/src/schema/registry.rs`
- `runtime/src/session.rs`
- `runtime/tests/check_artifact_size_cli.rs`
- `runtime/tests/check_audit_records_cli.rs`
- `runtime/tests/check_unreviewed_code_cli.rs`
- `runtime/tests/common/mod.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/exec_subprocess.rs`
- `runtime/tests/fix_rounds.rs`
- `runtime/tests/fixtures/review-basic/specs/007-review/plan.md`
- `runtime/tests/fixtures/review-basic/specs/007-review/spec.md`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/golden/specify-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `runtime/tests/mcp.rs`
- `runtime/tests/measure_reopenings.rs`
- `runtime/tests/mechanical_sweep_parity.rs`
- `runtime/tests/merge_status_hook.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/review_command.rs`
- `runtime/tests/size_hook.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/analyze_lib.py`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/combine.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/judge-brief.md`
- `scripts/analyze/record.py`
- `scripts/analyze/scenario-brief.md`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/audit/README.md`
- `scripts/audit/adopter-shell-behavior.sh`
- `scripts/audit/archive-contents.sh`
- `scripts/audit/cross-doc-consistency.sh`
- `scripts/audit/done-spec-criteria.sh`
- `scripts/audit/permission-wildcard-position.sh`
- `scripts/audit/rename-sweep-residue.sh`
- `scripts/audit/run-all.sh`
- `scripts/audit/ssot-invariants.sh`
- `scripts/audit/sweep-target-manifest-parity.sh`
- `scripts/check-release-tag.sh`
- `scripts/lint-governing-size.sh`
- `scripts/lint-tool-coverage.sh`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/artifact-size-bounded.py`
- `scripts/mutation/audit-records-085.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/compare-and-set-041.py`
- `scripts/mutation/compare-and-set-066.py`
- `scripts/mutation/criteria-in-review-066.py`
- `scripts/mutation/criterion-ticks-outside-the-digest.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/departures.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/first-review-scope-086.py`
- `scripts/mutation/freshness-results-have-a-bounded-size.py`
- `scripts/mutation/installed-file-modes.py`
- `scripts/mutation/line-citation-strip-066.py`
- `scripts/mutation/loop-inputs-screened.py`
- `scripts/mutation/moved-scope-066.py`
- `scripts/mutation/never-owed-084.py`
- `scripts/mutation/outcomes-outside-the-scope-086.py`
- `scripts/mutation/owed-work-084.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/plan-edit-takes-no-back-edge.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/process-decisions-retained.py`
- `scripts/mutation/project-parts-087.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/prune-project-containment.py`
- `scripts/mutation/prune-review-086.py`
- `scripts/mutation/quoted-heading-forms-067.py`
- `scripts/mutation/reachable-trigger-bar-078.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/resolve-constitutions-size.py`
- `scripts/mutation/review-022-fixes-2026-11-19.py`
- `scripts/mutation/review-022-task-41.py`
- `scripts/mutation/review-022-task-42.py`
- `scripts/mutation/review-022-task-46.py`
- `scripts/mutation/review-departures-bounded.py`
- `scripts/mutation/review-drops-unowned-086.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/routing-path-claims.py`
- `scripts/mutation/rule-files-left-to-analysis-020.py`
- `scripts/mutation/rule-judgments-088.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/mutation/run.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/site-anchors-020.py`
- `scripts/mutation/spec-ownership-086.py`
- `scripts/mutation/status-count-084.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/suffixed-task-number.py`
- `scripts/mutation/unreviewed-code-084.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/stale-literals.sh`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-archive-contents.sh`
- `scripts/tests/test-audit-path-containment.sh`
- `scripts/tests/test-audit-record-gate.sh`
- `scripts/tests/test-check-release-tag.sh`
- `scripts/tests/test-done-spec-owed-work.sh`
- `scripts/tests/test-lint-governing-size.sh`
- `scripts/tests/test-pre-commit-staging.sh`
- `scripts/tests/test-run-workflow-steps.sh`
- `scripts/tests/test-stale-literals.sh`
- `specs/000-slash-commands/review.md`
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
- `specs/000-slash-commands/scenarios/scenario-without-task-visibility.md`
- `specs/002-project-scaffolding/analysis.md`
- `specs/003-bootstrap-automation/analysis.md`
- `specs/003-bootstrap-automation/review.md`
- `specs/003-bootstrap-automation/tasks.md`
- `specs/008-security-rules/analysis.md`
- `specs/008-security-rules/plan.md`
- `specs/008-security-rules/review.md`
- `specs/008-security-rules/scenarios/a-cache-partition-is-purged-when-the-session-ends.md`
- `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md`
- `specs/008-security-rules/scenarios/a-throwaway-credential-owes-no-revocation-path.md`
- `specs/008-security-rules/scenarios/an-absent-origin-on-a-cookie-connection-is-flagged.md`
- `specs/008-security-rules/scenarios/be-deps-005-flags-what-its-statement-requires.md`
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/scenarios/the-tag-check-reads-every-entry-above-the-newest-version.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/009-scenario-targeting/analysis.md`
- `specs/009-scenario-targeting/review.md`
- `specs/010-agent-autonomy/plan.md`
- `specs/011-brownfield-process/spec.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/spec.md`
- `specs/012-multi-agent-govern/tasks.md`
- `specs/013-text-first-artifacts/plan.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/014-reclarify-backedge/plan.md`
- `specs/014-reclarify-backedge/review.md`
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md`
- `specs/014-reclarify-backedge/spec.md`
- `specs/015-tarball-fetch/analysis.md`
- `specs/015-tarball-fetch/plan.md`
- `specs/015-tarball-fetch/review.md`
- `specs/015-tarball-fetch/spec.md`
- `specs/015-tarball-fetch/tasks.md`
- `specs/016-cross-cutting-rules/review.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/data-model.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/review.md`
- `specs/018-adopter-owned-pre-commit/spec.md`
- `specs/019-config-decisions/analysis.md`
- `specs/019-config-decisions/review.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/scenarios/waiver-file-lists.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/021-runtime-boundary/review.md`
- `specs/022-deterministic-runtime/analysis.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/review.md`
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md`
- `specs/022-deterministic-runtime/scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md`
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md`
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/parser-walker-conventions.md`
- `specs/022-deterministic-runtime/scenarios/resolve-references-cli-exec-wiring.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`
- `specs/022-deterministic-runtime/scenarios/scaffolding-primitives.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/skipscanner-inline-code-exemption.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/023-govern-refinement/analysis.md`
- `specs/023-govern-refinement/plan.md`
- `specs/023-govern-refinement/review.md`
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md`
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md`
- `specs/023-govern-refinement/scenarios/living-specs.md`
- `specs/023-govern-refinement/spec.md`
- `specs/023-govern-refinement/tasks.md`
- `specs/024-rule-loader/analysis.md`
- `specs/024-rule-loader/plan.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/025-rule-opt-out/review.md`
- `specs/025-rule-opt-out/spec.md`
- `specs/026-framework-self-audit/analysis.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/026-framework-self-audit/scenarios/family-27-done-spec-unchecked-criteria.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/027-bootstrap-migration-registry/analysis.md`
- `specs/027-bootstrap-migration-registry/review.md`
- `specs/027-bootstrap-migration-registry/tasks.md`
- `specs/029-bootstrap-runtime-autowire/review.md`
- `specs/029-bootstrap-runtime-autowire/spec.md`
- `specs/030-cross-service-references/plan.md`
- `specs/030-cross-service-references/spec.md`
- `specs/033-rule-surface-setting/review.md`
- `specs/034-performance-backend-rules/analysis.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/035-groom-session-target/review.md`
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md`
- `specs/035-groom-session-target/scenarios/reopen-done-spec-on-scenario.md`
- `specs/036-quality-cross-rules/analysis.md`
- `specs/036-quality-cross-rules/review.md`
- `specs/037-observability-backend-rules/analysis.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/analysis.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/039-reliability-backend-rules/analysis.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/scenarios/expendable-messages-may-be-dropped-when-recorded.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/analysis.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/review.md`
- `specs/043-workflows-sunset/review.md`
- `specs/045-decision-state-drift-detection/plan.md`
- `specs/045-decision-state-drift-detection/review.md`
- `specs/046-scenario-open-question-visibility/plan.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/047-analyze-findings-durability/review.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/047-analyze-findings-durability/spec.md`
- `specs/048-govern-acquired-runtime/analysis.md`
- `specs/048-govern-acquired-runtime/plan.md`
- `specs/048-govern-acquired-runtime/review.md`
- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/048-govern-acquired-runtime/tasks.md`
- `specs/049-rename-govern-to-ductus/plan.md`
- `specs/049-rename-govern-to-ductus/review.md`
- `specs/049-rename-govern-to-ductus/spec.md`
- `specs/050-constitution/analysis.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/review.md`
- `specs/050-constitution/scenarios/a-canonical-source-is-pointed-at-not-copied.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/a-retired-filename-leaves-a-decision-record.md`
- `specs/050-constitution/scenarios/adopter-practices-classified-for-promotion.md`
- `specs/050-constitution/scenarios/findings-route-by-scope.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-pending-fold-never-holds-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/fold-target-checked-before-the-rewrite.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/052-spec-supersession-and-consolidation/analysis.md`
- `specs/052-spec-supersession-and-consolidation/review.md`
- `specs/054-remove-supersession/plan.md`
- `specs/054-remove-supersession/review.md`
- `specs/054-remove-supersession/spec.md`
- `specs/055-shared-constitution/analysis.md`
- `specs/055-shared-constitution/data-model.md`
- `specs/055-shared-constitution/plan.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/056-bootstrap-archive-boundary-split/analysis.md`
- `specs/056-bootstrap-archive-boundary-split/review.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/058-findings-route-at-discovery/analysis.md`
- `specs/058-findings-route-at-discovery/data-model.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/review.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/059-project-in-a-repository-subdirectory/review.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/review.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/rule-judging-runs-from-repo-scripts.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/tasks.md`
- `specs/061-updates-track-the-latest-release-tag/review.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/064-pi-host-support/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-write-result-states-what-it-wrote.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-base-includes-the-transition-commit.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/analysis.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md`
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/analysis.md`
- `specs/068-api-backend-rules/plan.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/069-accessibility-frontend-rules/analysis.md`
- `specs/069-accessibility-frontend-rules/plan.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/scenarios/verifications-flag-only-what-the-statement-covers.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`
- `specs/070-performance-frontend-rules/analysis.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/071-command-reference-split/analysis.md`
- `specs/071-command-reference-split/plan.md`
- `specs/071-command-reference-split/review.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/072-governing-documents-stay-within-one-read/spec.md`
- `specs/073-decided-rule-departures-are-recorded/analysis.md`
- `specs/073-decided-rule-departures-are-recorded/data-model.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/073-decided-rule-departures-are-recorded/tasks.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/075-agents-md-holds-what-every-session-needs/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/spec.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/analysis.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/review.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md`
- `specs/077-security-cross-rules/analysis.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/review.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/analysis.md`
- `specs/080-deterministic-runtime-efficiency/data-model.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/research.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`
- `specs/082-rule-assessments-converge-across-runs/analysis.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/review.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/082-rule-assessments-converge-across-runs/tasks.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/analysis.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/review.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/tasks.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/analysis.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/data-model.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/review.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/analysis.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/data-model.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/plan.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/review.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/tasks.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/analysis.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/review.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/analysis.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/review.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`
- `specs/088-rule-judgments-cost-only-what-changed/analysis.md`
- `specs/088-rule-judgments-cost-only-what-changed/data-model.md`
- `specs/088-rule-judgments-cost-only-what-changed/plan.md`
- `specs/088-rule-judgments-cost-only-what-changed/review.md`
- `specs/088-rule-judgments-cost-only-what-changed/spec.md`
- `specs/088-rule-judgments-cost-only-what-changed/tasks.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/analysis.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/review.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/tasks.md`
- `specs/README.md`
- `specs/review.md`
- `version`

## Unexamined governance

*None.*
