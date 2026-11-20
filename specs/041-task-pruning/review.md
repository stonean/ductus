---
spec: 041-task-pruning
last-run: 2026-11-20T00:43:51Z
reviewed-against: 45c0665bef4e7b2522bbaf2443059a089df7cd65
diff-base: ef2fb4e8403b886743ea20960b40ed50f03ac1d5
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 17
scope: 21
unread:
  - runtime/CHANGELOG.md
  - runtime/src/schema/primitives.rs
  - runtime/src/primitives/prune_project.rs
  - scripts/mutation/prune-project.py
excluded: 91
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 8da08307a7f78ceeea3992437bbb46b31f0a399ee4150449387a1bbb4da60b77
  scenarios/a-plan-edit-does-not-reopen-a-done-spec.md: d212af7304352b7080514a22d93fa67f76e350b7989670730c16045c50dc2051
  scenarios/a-pruned-task-number-is-never-reissued.md: 7d97f1b790d2b12d3b1630cc78f24eac2aff167c9ab932b436964f0f50205d06
  scenarios/a-suffixed-task-number-is-a-task.md: 362cf9e7efe74c12c39d20cd82d2347148695310611c9895b5d7694dad611f17
  scenarios/an-open-fenced-block-or-comment-is-not-examined.md: 4f9252380ba21bb4bd80e3f0ca308a75cc3fe001b7f688ecae7d1a84f08a00c5
  scenarios/plan-records-the-design-as-it-stands.md: b3bfd54321552fcc9e32f91dfa5b24a21a923685937f661a38e78b8ed67d5658
  scenarios/prune-all-walks-every-spec.md: 0bcf1f2b448c089721deab6e96dba269082c23a092c0b8aaca56ee90244aefab
  scenarios/prune-reduces-the-spec-directory.md: c485de78348784233c118c59c649bc38d6b33ba4b6d9a05e0a8daa9c656f57c0
  scenarios/prune-walks-the-project-files.md: 8cb1a20f84195213bb7ddac0789d75a3a557a3872fdc480cf868115d06125a3e
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
    digest: 0b50354ae4870989ed4c3eec5e6d1d1303ee184fecc83caae4fc9623a589c88f
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
    digest: 7251092d6a5e4b037fc47925840cd204e7856c469a11ec99be7e2532f62d95a3
  - label: AC16
    digest: f7919d0e9a33d3590016a4d662665b92f7fede79775f87baa21d4a118a98b398
  - label: AC17
    digest: e58fb58b1d607efc59e5ce0202f08c76dafe2c997cf45ce4b646b00ce5d9527b
  - label: AC18
    digest: d2032eb9382fc36598f77ef4907b1afc8281c807bf4215b1f7c0f135f0345642
  - label: AC19
    digest: 0971bd8e7f193fad51836a14d7fec145ecb2036ba5447b8eb72af14628449c6e
  - label: AC20
    digest: de03f1ce630ff02e0501143c417418bde819d7a320b0520dc34456d737c529aa
  - label: AC21
    digest: 72c83d3baf107c61f90402e59a68b10ce4e27720284bfd284aba11cc5a488c35
blocking: false
dispositions:
  fixed: 9
  deferred: 0
  waived: 5
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
---

# Review — 041-task-pruning

## Summary

Review of 041-task-pruning, diff-base ef2fb4e8, scope 21 entries, 17 examined. Passes read by four parts: security (three security rule files, whole), the other four passes (nine rule files, AGENTS.md Gotchas to the end, system.md), the criteria step (AC1 to AC21, each judged met by two readers after reading the spec, plan, data model and all eight scenarios), and a fourth part that finished the whole reads of framework/commands/analyze.md, framework/reference/analyze.md and runtime/src/schema/descriptions.rs. Read in part and so unread: runtime/CHANGELOG.md (the 041 entries only), runtime/src/schema/primitives.rs (the prune types and the 5b20aaa4 hunks), runtime/src/primitives/prune_project.rs (non-test code and the fix hunk, tests not read) and scripts/mutation/prune-project.py (the new mutant's hunk; run.py --check finds 0 invalid anchors). Installed copies were read by diff against their sources, which were read whole (prune.md) or whole by the fourth part (analyze.md and its reference), with the generator in sync. Fixed in the run: the open-region wording in prune_tasks.rs, the two walk doc comments, the UnexaminedTail and TailCause docs, three data-model enumerations, prune.md step 5's nothing-to-prune exit (19887 to 19947 bytes), spec Edge Cases, the plan-record sentence in framework/reference/analyze.md that contradicted AC17, and prune-project's heading entry running through an open comment's tail (test extended, mutant M22 killed). Two fix rounds, both re-read clean. The stored decision on prune_tasks.rs's H1 search no longer holds and is dropped; twelve in-scope decisions carry.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- AC1 — met: framework/commands/prune.md steps 1-3 resolve the session target and preview prune-tasks and prune-plan on it with no argument; runtime/tests/walker.rs prune_command_gate_blocks_on_the_exec_path walks steps 1-3.
- AC2 — met: prune_tasks.rs reduce_keep_pending drops spent sections; fixture cut tasks.md 251 to 163 bytes; test keep_pending_drops_spent_preserves_pending_and_prose.
- AC3 — met: Block::classification and reduce_keep_pending keep Pending and NoCheckbox blocks verbatim; fixture kept a 1-of-2 ticked section byte-for-byte; tests keep_pending_drops_spent_preserves_pending_and_prose, suffixed_task_pending_stays_with_its_phase.
- AC4 — met: Driven: keep-pending (flat and phased) and reset output start with the next-task frontmatter then the H1 and lint with 0 issues; check-artifacts reports no task-consistency finding; tests keep_pending_phased_drops_spent_and_empty_phase, reset_output_parses_as_zero_tasks.
- AC5 — met: prune.md step 6 invokes gate-confirm before step 7's apply; runtime/tests/walker.rs prune_command_gate_blocks_on_the_exec_path (denial leaves tasks.md byte-identical) and prune_command_applies_after_a_confirmed_gate.
- AC6 — met: reduce_reset emits frontmatter, the existing H1 and CANONICAL_EMPTY_TASKS_BODY; fixture reset gave tasks-count 0 and a second reset nothing-to-prune; tests reset_produces_template_state_when_done, a_reset_keeps_the_frontmatter_block, canonical_empty_body_matches_template.
- AC7 — met: The only write path is ReadSnapshot::write_if_unchanged (tempfile and rename) via Reduction::write; no backup code; the fixture tree held only the 0-byte gitignored compare-and-set lock files, which hold no pruned content.
- AC8 — met: prune_tasks.rs reduce sets PruneGate::BlockedNeedsForce when status is not done and force is absent; fixture reset on in-progress returned blocked-needs-force and wrote nothing, --force wrote; tests reset_blocked_on_non_done_without_force, reset_forced_on_non_done_writes.
- AC9 — met: nothing_to_prune is new_content equal to content and applied requires it false; fixture second apply returned nothing-to-prune; prune.md step 5 stops with nothing to prune except for a result carrying unexamined-tail; tests keep_pending_no_op_when_nothing_spent, a_prune_that_removes_nothing_writes_no_counter.
- AC10 — met: prune.md step 1 directs to /target with no session target, step 2 for a missing feature directory, step 3 directs to /plan when neither tasks.md nor plan.md exists; tests missing_tasks_file_errors and walker.rs's missing-plan walk; the no-target stop is command prose with no test.
- AC11 — met: prune_tasks.rs run and Reduction::write produce and write the reduction inside the runtime and PruneTasksSummary has no body field; reset writes the compiled CANONICAL_EMPTY_TASKS_BODY pinned by canonical_empty_body_matches_template; no markdown-only fallback remains in framework/commands/prune.md.
- AC12 — met: framework/constitution.md §Tasks Phase states tasks.md is an ephemeral work-tracking artifact distinct from the durable spec.md, data-model.md, scenarios and rules, stated directly.
- AC13 — met: SkipScanner (primitives/mod.rs) is used by parse_tasks in read_tasks.rs, mark_task.rs and tasks_counter.rs; driven: the shipped template gives read-tasks tasks-count 0 and append-task task-number 1; test reset_output_parses_as_zero_tasks.
- AC14 — met: check_artifacts.rs check_scenario_consistency flags a scenario only when no revision of tasks.md ever named it (ever_tasked_slugs); driven: a done spec whose scenario tasks were pruned gives no finding; tests a_task_pruned_by_keep_pending_satisfies_the_mapping, a_task_removed_by_a_reset_satisfies_the_mapping.
- AC15 — met: framework/commands/prune.md step 8 writes the moves first, confirms a section's moves and removal together through gate-confirm, leaves a declined section byte-for-byte and routes knowledge as the criterion says; host procedure, the primitive side is tested by a_digest_mismatch_refuses_the_whole_apply.
- AC16 — met: prune_plan.rs DESIGN_RECORD, is_design_record (ASCII case-insensitive), segment() through OpenRegion, PlanSection with no text, claim() refusing the whole apply on stale-sections; driven on a fixture; tests a_heading_inside_a_fence_or_a_comment_is_not_a_section, headings_compare_case_insensitively, a_digest_mismatch_refuses_the_whole_apply.
- AC17 — met: prune.md step 4 keeps a decided section; prune_plan.rs outside_record sets decided only for a person's well-formed waiver under the exact key; driven; tests a_stored_waiver_sets_decided, a_waiver_the_rule_made_does_not_set_decided; framework/reference/analyze.md Plan record now agrees.
- AC18 — met: framework/commands/analyze.md step 16 records every listed section as an Advisory at planned or later under the result's own finding; plan_section_finding in prune_plan.rs builds the message, test a_preview_reports_each_section_outside_the_record_and_writes_nothing; the step is host procedure.
- AC19 — met: Driven on a done fixture: reopen-required false after a plan-only edit and after a checked box, true after an added unchecked box; reopen_required and adds_unchecked_box in read_tasks.rs; prune.md steps 5, 8, 9; tests an_added_unchecked_task_reopens, a_changed_design_record_section_does_not_reopen.
- AC20 — met: framework/constitution.md §Plan Phase states the plan records the design as it stands with the template's own sections as the design record and homes for the rest; framework/templates/spec/plan.md names every section and the tasks template carries the working-notes guidance; test design_record_matches_the_plan_template.
- AC21 — met: tasks_counter.rs and prune_tasks.rs write max(held, highest+1) when a section is removed and a reset keeps the frontmatter; append_task.rs next_task_number takes the larger of the counter and highest+1 and advances it only when a counter exists; driven; tests a_pruned_top_number_is_not_reissued_by_the_next_append, a_file_without_a_counter_gains_none.

## Observations

- quality: the lines after an open region sit inside the block holding the opener, which the comment above `let applied` in `reduce` in `runtime/src/primitives/prune_tasks.rs` and the data model's segmentation text called "in no block" — `runtime/src/primitives/prune_tasks.rs` — **fixed** (decided by the rule)
- quality: the doc comments on `walk` in `runtime/src/primitives/prune_tasks.rs` and `runtime/src/primitives/prune_plan.rs` omitted the `unexamined-tail` listing condition — `runtime/src/primitives/prune_plan.rs` — **fixed** (decided by the rule)
- quality: the schema doc comments on `UnexaminedTail`, `TailCause` and the two summaries said "in no entry", which describes `prune-project` only, in `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- quality: `specs/041-task-pruning/data-model.md` enumerations omitted `UnexaminedTail`, `TailCause`, the `PruneTasksLine` field and the open-region case of `applied` — `specs/041-task-pruning/data-model.md` — **fixed** (decided by the rule)
- quality: `framework/commands/prune.md` step 5's nothing-to-prune exit could read a result carrying `unexamined-tail` as clean — `framework/commands/prune.md` — **fixed** (decided by the rule)
- quality: `framework/reference/analyze.md` Plan record said a waiver leaves a prune still proposing the section, contradicting AC17 — `framework/reference/analyze.md` — **fixed** (decided by the rule)
- quality: `specs/041-task-pruning/spec.md` Edge Cases said prune reports nothing to prune whenever nothing is spent, with no exception for an `unexamined-tail` — `specs/041-task-pruning/spec.md` — **fixed** (decided by the rule)
- quality: the installed copies `.claude/commands/ductus/prune.md` and `.pi/prompts/ductus-prune.md` lacked the step 5 clause until generated — `.claude/commands/ductus/prune.md` — **fixed** (decided by the rule)
- reuse: the four-line `unexamined_count` computation is repeated in both walks in `runtime/src/primitives/prune_plan.rs` — `runtime/src/primitives/prune_plan.rs` — **waived**: the walks differ in what they list and how; a shared skeleton would be generic over both results for one loop
- test: `scripts/mutation/prune-open-region.py` has no mutant for the frontmatter line offset of `OpenRegion::tail` or for prune-plan's `.tail(0)` — `scripts/mutation/prune-open-region.py` — **waived**: the tests the_tail_line_counts_the_frontmatter and the plan tail test assert the figure, so a mutant would be redundant (decided by the rule)
- pipeline-machinery: `/analyze` step 16's sentence about `unexamined-tail` has no record field to land in, since `UNEXAMINED_REASONS` in `runtime/src/primitives/write_analysis.rs` has no reason for a partly examined plan, in `framework/commands/analyze.md` — `framework/commands/analyze.md` — **waived**: the analysis record's closed reason set belongs to 066 and 067, and the step reports the partial examination in the run (decided by the rule)
- quality: each prune apply leaves 0-byte gitignored lock files under `.ductus/locks/`, which AC7's no gitignored sidecar might be read to exclude, from the write guard in `runtime/src/primitives/mod.rs` — `runtime/src/primitives/mod.rs` — **waived**: the lock files hold no pruned content, and AC7 bars a backup of it (decided by the rule)
- quality: `prune-project` kept a heading entry open across a comment that never closes, so an entry covered the unexamined tail and an apply could rewrite text no one examined, against the data model's segmentation, in `runtime/src/primitives/prune_project.rs` — `runtime/src/primitives/prune_project.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- quality: the last sentence of `PRUNE_PROJECT` in `runtime/src/schema/descriptions.rs` can be read as saying a later rewrite's re-check writes nothing, which only the home-to-create check guarantees — `runtime/src/schema/descriptions.rs` — **waived**: low confidence: the sentence's nearest antecedent is the home to create, which it states correctly, and the wording predates this window (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/review.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-review.md`
- `framework/commands/review.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/plan.md`
- `framework/reference/review.md`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/main.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_command_flags.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/spec_links.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/derive_dependencies_golden.rs`
- `runtime/tests/derive_references_golden.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `scripts/audit/README.md`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/review.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/analysis.md`
- `specs/057-analyze-artifact-and-record-relocation/data-model.md`
- `specs/057-analyze-artifact-and-record-relocation/review.md`
- `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
