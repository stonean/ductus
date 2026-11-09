# 079 — Review and analysis work converges Plan

Implements [079 — Review and analysis work converges](spec.md).

## Overview

A shared commit-attribution helper gives the review window, scope, fix rounds and `diff-cross-spec` one answer to whose change a commit is. Two new primitives decide from history and from a diff what a person used to judge: `measure-reopenings` reports the outcome, and `classify-spec-edit` says whether an edit reopens. The review and analyze commands stop asking about non-gating observations. Each primitive gets one compiled description, the constitution makes the binary the source of its rules, and the references shrink last, so the cuts cannot hold up the changes that end the cycle.

`specs/system.md`, `specs/events.md` and `specs/errors.md` do not exist in this repository, so there is nothing to conflict with there. This spec declares no dependencies.

## Technical Decisions

### One commit-attribution helper

`runtime/src/primitives/commit_attribution.rs` answers `claims(commit)`: the specs whose directory the commit's diff against its first parent touches, plus those its subject's leading parenthesised scope names, with the first named kept, and the paths the commit contributes. No colon is required after the scope, since `impl(077) task 8 …` is a shape this history holds. Directories and scope tokens resolve through `parse_feature_dir`, so a branch-scoped spec counts and a number no spec has claims nothing. A merge commit claims nothing, since its branch's commits are in the walk, and contributes only the paths differing from every parent, so a clean merge adds none and the branch's work does not return as work no spec claims. History is read through `ProjectRepository`, converting each path with `to_git` and `to_project` (`runtime/src/primitives/mod.rs`). Both tests are needed: 35 commits since 2026-10-01 name a spec without touching a spec directory, and 30 do the reverse. An adopter whose subjects name no spec loses only the first-named step, and AC6 then gives the path to every claimant. Its fixture histories are unit tests, since `git_fixture` is `#[cfg(test)]` and `scripts/mutation/run.py` runs `cargo test --lib`.

### The window

`compute` in `runtime/src/primitives/compute_review_scope.rs` resolves today's base from `find_in_progress_commit` (`runtime/src/primitives/check_stuck.rs`), which finds where the reopening started: where the spec last left `done` or, never `done`, where it first entered `in-progress`, a trip through `draft` or a commit with no readable status staying in it. A new step walks the commits after that transition, newest first, for the one that finished a run: its record names a `last-run` its first parent's does not, or its parent's write of that run still had `fix-rounds-open`. A record with `skipped-passes`, `budget-stop: true`, `fix-rounds-open`, or frontmatter that does not parse finishes nothing; nor does one `holds_done_until_reviewed` holds, or a re-run would see an empty scope and clear an unfixed MUST (owner, 2026-11-05: at `6aef0472` this changes the window of 0 of 47 `in-progress` specs, built from a clean worktree, and 1 of the 126 closings since 2026-10-01 at `d62e72f3` took a second review after a gating finding; carrying open findings forward was rejected, since the record would hold findings its passes never derived), and neither does a commit moving a run's record out of `spec.md`'s frontmatter (`run_in_spec_block`) or a merge carrying a run in, judged against every parent, since neither is a review. When it finds one, that commit is the base. `--since`, an uncommitted transition and a spec with no transition keep today's behavior. The commit holding a record also holds that run's fixes, which its fix rounds already re-read, so excluding them is the point.

The lookup finds a spec's files by number when its name is absent from a commit (`feature_file_in_tree`, `runtime/src/primitives/check_stuck.rs`), so a rename keeps the window and moves no run. A commit restoring a stamp an older commit finished starts the window at that older commit, so what lay between has no standing, and one restoring the run the reopening carried in finished nothing here, so the window stays at the reopen. No record at `HEAD` also starts at the reopen, while `last-run: null`, which a fold's `invalidate-review` writes, keeps the start, since what the fold brought joins by its own rule. On a shallow clone, a walk reaching its cut below `done` with no later commit leaving `done` is `ShallowHistory`: an error from `compute-review-scope`, which `/{project}:review` stops on, and guidance from `diff-cross-spec` and `check-stuck`.

### The scope

`compute` replaces the union with AC6's rule, walking `base..HEAD` through the helper. A commit the spec alone claims, or no spec claims, contributes every path. A commit several specs claim contributes the spec's directory, its plan's Affected Files, and a path in none of the claimants' plans when the subject names this spec first or, outside the other claimants' directories, names none. Uncommitted changes join whole. A plan entry unchanged in the window stays only when the record at `HEAD` names no run, so a run's own uncommitted record does not change the answer between its two writes. Each path that record lists in `open-findings` joins too, wherever the window starts and out of `excluded`, so a re-run reads what a finding names (owner, 2026-11-06), and so does a contract that record's digest says changed, so one edited before the window is read before its digest refreshes. A branch-scoped spec a commit in the window, or the working tree, removed below `done` with `folds-into` naming this spec brings the paths of every commit it claimed, outside any spec's directory and still on disk, from all of history, since a target reopened by the fold starts after them (owner, 2026-11-06). The status and target are read at the removing commit's parent, the window's walk finds it, and only then does `FoldTest` walk history, testing each commit's subject and tree entry before any diff. A list in frontmatter, not the report's body, since structured data lives there (§text-first-artifacts). Globs match through the plan's existing `read_plan_affected` entries. `modified-since` keeps its meaning, and what the scope leaves out becomes `excluded`, in the bounded lists (`runtime/src/primitives/bounded_lists.rs`). `write-review` derives its denominator from `compute`, so the record follows, and it names each excluded path by the spec that claimed it.

`review-owed` is false when a review record names a run and the scope holds no path outside a spec's directory and no edit to this spec's scenarios or data model (`is_review_contract`, `runtime/src/primitives/analyze_subjects.rs`), since a path in another spec's directory is that spec's change and its review reads it. With no such record it is true, because `never_reviewed_block` holds `done` on a spec's first pass and AC4 presumes an earlier review. It is true too while the record holds `done` until a review runs, through `holds_done_until_reviewed` (`runtime/src/primitives/check_review_gate.rs`): the gate blocks a gating count, no `dispositions:` map, the open mark or an unwaived round, and calling a review not owed would leave `/{project}:review` stopping on what the gate asks for. A stale digest counts too, since a contract edited before the reopen lies outside the window, and so does a deletion the scope's rule gives this spec, since it has left the scope. A record the gate does not hold passes a text-only reopening without a new run, since its review check is digest-based (`stale_review_block`). The review command reports `review-owed: false` and stops unless asked to continue, and `/{project}:implement`'s closing guidance reads it.

### Fix rounds

`round_paths` (`runtime/src/primitives/fix_rounds.rs`) gains the filter. Each snapshot's parent is `HEAD` at its time, so the commits between two snapshots' parents are knowable with nothing reported by the agent. A path leaves the round when a commit that only other specs claim changed it, and its content at both snapshots equals its content at that snapshot's parent, so no uncommitted edit sits on top. A commit this spec or no spec claims stays in the round. Another session's uncommitted edits cannot be told apart and stay.

`classify` (`runtime/src/primitives/brief_re_read.rs`) keeps its test per path, so an exempt path leaves the brief and `round.toml` even in a mixed round. A finding on one then counts as out of round through `demote_out_of_round`, which is right: `/{project}:analyze` reads that text. The result returns `exempt` and `excluded`, and `write-review` records them per round.

### The open mark

`write-review` gains `loop-open`, passed by step 11 of `framework/commands/review.md`, which writes `fix-rounds-open: true`. Step 12's last write omits it. An argument, not a default, because under `ductus exec` the walker no-ops step 12 and a default would block every exec run. `fix_round_block` (`runtime/src/primitives/check_review_gate.rs`) blocks on it as a new `ReviewGateBlock` variant, named by `/{project}:review`. `passing_notices` keeps its notice for a record with no loop fields.

### The gate reads what a review owes

`check-review-gate` blocks on `review-owed`, naming what changed (`ReviewScope::owed_by`), and every review check runs ahead of the analyze checks, so `review_half_block` (`runtime/src/primitives/check_review_gate.rs`) holds the map, the gating count, the fix-round check and the owed test, and `disposition_block` takes one record at a time. The order matters because `review.md` is one of an analysis's subjects, so an analysis run first is stale when the review writes. It clears once the record is committed, because the window moves only at that commit. A listed break blocks from the record itself, so it does not depend on the window. A window that cannot be read passes with a notice, as a gate blocking on its own inability to check is one people route around, and the fix for a shallow clone is that item of task 16.

### `diff-cross-spec`

`runtime/src/primitives/diff_cross_spec.rs` starts at the same reopening's parent the window uses, and walks commits through the helper, keeping sibling paths only from commits belonging to the spec, plus uncommitted sibling paths. A spec never `in-progress` keeps the first spec-directory commit, so a planned spec still gets an answer. The no-commit `guidance` stays. `derive-boundary` is unchanged, so its doc comment's claim about a shared window is corrected (`runtime/src/primitives/derive_boundary.rs`).

### Dispositions

Steps 10 of `framework/commands/review.md` and 19 of `framework/commands/analyze.md` change: a non-gating finding follows the rule, with no `gate-confirm`, and only a gating finding's waiver or deferral reaches the operator. A stored decision by a person still wins (`process-decisions`). Under `ductus exec` nothing changes, since with no operator nothing is written.

"Counts as gating" means a contract-breaking observation takes a gating finding's dispositions: it is fixed in the run or deferred to the spec's own task, its deferral reaches the operator, and the record counts it in `contract-breaks`. A stored decision on one is marked `contract-break: true`, so a later run matching it holds the same without a host marking it again. `check-review-gate` gains no tier, so the MUST, SHOULD, hard-fail and blocking definitions in the constitution, 058, 076 and 066 stay true: a break the run did not fix, a person did not waive and no deferral releases is listed in `open-findings`, and the gate blocks on the list as `review-owed` (owner, 2026-11-06). That holds `holds_done_until_reviewed` true too, so its record does not move the window's start. `contract-breaks` still counts a waived break, which the measurement reads as a gating finding left by that run.

`process-waivers` and `process-decisions` resolve the run's scope once, through `RunScope` in `runtime/src/primitives/compute_review_scope.rs`, when a stored finding on a file did not fire. One on a file still on disk that the scope does not hold is retained, so a narrower window cannot expire a person's decision (AC24). The primitive reads the scope itself rather than taking a host's word for each anchor (§design-principles), and takes `since` only to match a run that took `--since`.

### What reopens

`classify-spec-edit` decides from the diff, since a rule that depends on an agent remembering it would be skipped (§design-principles). It compares `spec.md`'s criteria and open questions through `read-spec`'s parser, unchecked boxes through `unchecked_lines` (`runtime/src/primitives/read_tasks.rs`), the one test of what `tasks.md` owes, by a line's text under any heading, which `prune-plan`'s `reopen-required`, `measure-reopenings`' `waiting`, the gate and `check-artifacts` share (owner, 2026-11-06), and the changed path set, and returns `correction` only when the diff touches nothing but `spec.md`, `plan.md` and `tasks.md` and changes no criterion text, question or unchecked task, since a `tasks.md` edit adding no unchecked task takes no back-edge (§spec-lifecycle). A label's assignment stays a correction, and a run's record is no edit. `/{project}:amend`'s reconcile pass calls it before proposing a reopen, and `check_artifact_size.rs`'s `OnDone` hints follow it, so trimming `spec.md` reopens only when a criterion or question changes and trimming `data-model.md` always does. Nothing enforces a reopen today, so this adds no gate. `scripts/audit/cross-doc-consistency.sh` pins only the transitions, so it does not change.

A contract edit, or one to any other file under the spec, reads `mechanical` when `ReferenceScrub` (`runtime/src/primitives/derive_references.rs`, on the predicate `derive-references` harvests by) leaves both sides equal, or when `SweepIndex` over a snapshot of the working tree finds it a sweep, so the classifier and the gate's staleness share one sweep rule and its blind spot (owner, 2026-11-06). They read a reference-only contract edit apart: `mechanical` here and stale there, since exempting references from the digest is a change of its own, with 0 occurrences while no spec here carries `references:`. Sweeps committed one spec at a time show the index one file each, and read `reopens`.

Taking a tick back reads `reopens`, and a `done` spec holding an unticked criterion or an unchecked box reports `review-state-drift` from `check-artifacts`, reusing that family so no closed set grows and `--fix` already reverts it, and the gate blocks `done` over an unchecked box (owner, 2026-11-06).

### The measurement

`measure-reopenings` lives in `runtime/src/primitives/measure_reopenings.rs`, because AC8 needs it in an adopter's project and AC19 needs it in `dashboard`. One revwalk, newest first, reads each feature directory's `status:` by blob, memoized as `status_of_commit` does (`runtime/src/primitives/check_stuck.rs`), judges each commit against its own parents, and delimits reopenings, a trip through `draft` staying in one (`History::opened_earlier`): every commit is walked, so a closing on a merged branch counts, and every directory seen, so a closing survives its spec's retirement. It skips a commit past the window while no closing awaits its start. A run is a distinct `last-run` in the committed record inside a reopening. A second run is over the bound unless the previous record of the same kind in that reopening was `blocking`, or counted hard-fail, blocking, MUST, SHOULD or `contract-breaks`, or still carried `fix-rounds-open`, `budget-stop: true` or a skipped pass, each a run the gate has re-run, read from that run's last write. A review's two writes share step 11's `reviewed-at`, so they are one run. `decisions-by-person` counts the record's `decisions:` entries stamped with a run's time whose `decided-by` is not `rule`, and its `waivers:` and `round-waivers:` stamped alike, a round the config file waived aside, and `run-decisions` holds that count per run. A decline with no reason stores nothing, so it counts as none (owner, 2026-11-06). A shallow clone is detected through `Repository::is_shallow` and reports `shallow-history`. The default window keeps `/{project}:status` fast and lets old misses age out; `--since` serves AC20.

The baseline check is a recorded command result at `d62e72f3`, not a test, because a test reading real history would need a full-depth checkout to run at all (§design-principles). A fixture repository proves each state in `runtime/tests/measure_reopenings.rs`.

A run whose last write still carries `fix-rounds-open` counts as leaving a gating finding, since the gate demands the re-run (owner, 2026-11-06). Only the mark is added, not the gate's whole predicate, whose no-map arm would count an older record as gating.

### A run waits for the work

`refuse_while_tasks_are_open` (`runtime/src/primitives/read_tasks.rs`) refuses a record while `tasks.md` holds a box by `describe_unchecked`, the gate's own test, and `write-review` and `write-analysis` call it after resolving the feature, so a run made before implementation is complete cannot record, under `ductus exec` too. `/{project}:review` and `/{project}:analyze` say so at their first step, so a host stops before the passes rather than at the write. The tasks template and `/{project}:plan`'s reference schedule no run, and each task list that did now says in prose what its spec owes (owner, 2026-11-07).

### One description per primitive

`runtime/src/schema/descriptions.rs` holds one `pub const` per primitive. `#[tool(description = …)]` takes an expression (`rmcp-macros` 3.8.0's `tool.rs`), and clap takes `long_about` and `about` in `runtime/src/main.rs`, so a subcommand's `--help` prints the whole text and the `ductus --help` list its first sentence, or what precedes its first colon when that comes sooner, since some first sentences run to a thousand bytes listing what follows one. The move is byte-identical for MCP but for what one text requires: `gate-confirm`, which behaves differently on each surface, says so, and `AC{n}`, which clap's help renderer turns into a newline, becomes `AC<n>`. The CLI-only primitive gets a constant too, and `--help` adopts them, replacing the separate doc comments. A test holds each served tool's description equal to its subcommand's `long_about`. Module docs lose only what restates the description; a sweep of them is not owed.

### The constitution

Four edits, net-negative together, since it holds 49,936 of 50,000 bytes:

- §runtime-boundary's host integration and eligibility criterion 3: the binary is the source of a primitive's rules, and the server is required, so a command stops without it (AC21; owner, 2026-11-06).
- §spec-lifecycle's third back-edge and its shared test: a correction reopens nothing (AC15).
- §brownfield-inbox's Finding dispositions: "each gating write is confirmed first", and "Scope decides the destination" narrowed to `/{project}:implement` (AC2, AC17).
- The audit-record field list gains the three fields, and §drift-prevention's table gains a row for a primitive's behavior (AC9, AC14).

The bytes come from the sentences these replace, the two-paths text, and sentences stating one primitive's behavior, which move to its compiled description. §cross-spec-impact also follows AC15: a signpost on a `done` spec is a correction, and `check-review-gate`'s guidance says so.

`framework/bootstrap/ductus.md`'s shell text stays this bootstrap's own contract, since it runs before the binary exists: it carries the steps before acquisition and 029's per-step fallback when a primitive call errors, and everything else runs through the tool or `{pointer-path}` (owner, 2026-11-05). `framework/bootstrap/govern.md` stays byte-identical to it.

### Prose cuts come last

`research.md` holds the section-by-section estimate. Cuts follow AC21, so a reference drops a primitive's rules, and they drop text rather than move it. Neither `framework/reference/review.md` nor `framework/reference/analyze.md` needed a split. What stays beyond a step's result is what an author or operator acts on and no description states, which counts as policy (owner, 2026-11-05): `framework/reference/analyze.md`'s closed phrase lists, which Family 18 holds to the code, and `framework/reference/review.md`'s rule-file selection and waiver lifecycle. Every heading keeps its exact text. `framework/reference/status.md` loses what `dashboard` renders, and `derive-boundary`, `mark-task` and `check-stuck` state their rules in their descriptions (owner, 2026-11-06). `framework/reference/log.md` stays though it only restates its command's step 4, since 071's AC2 and its entry on which commands are split name `log`. This took over `020-code-review`'s task 16, closed by a pointer to the result.

### `AGENTS.md`

AC10, AC11 and AC16 landed with their decisions. The window and `--since` entries that AC5 and AC6 retire are removed in the task that lands them, and the fix-round entries are re-read then.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/commit_attribution.rs` | Create | Whose change a commit is, with its fixture histories |
| `runtime/src/primitives/invalidate_review.rs` | Modify | Doc comment: the next review's scope holds what a fold below `done` brought |
| `runtime/src/primitives/process_waivers.rs` | Modify | Retains a waiver on a file the run did not read |
| `runtime/src/primitives/process_decisions.rs` | Modify | Retains a decision on a file the run did not read |
| `runtime/src/primitives/check_stuck.rs` | Modify | Where a reopening starts, shared with the window and `diff-cross-spec` |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | Window, scope, `excluded`, `open-findings`, `review-owed` |
| `runtime/src/primitives/fix_rounds.rs` | Modify | A round drops another spec's commits |
| `runtime/src/primitives/brief_re_read.rs` | Modify | Exempt paths leave a mixed round's brief |
| `runtime/src/primitives/write_review.rs` | Modify | Open mark, `excluded`, `contract-breaks`, `open-findings`, `decided-by: rule` |
| `runtime/src/primitives/write_analysis.rs` | Modify | `decided-by: rule`; refuses while a task is open |
| `framework/templates/spec/tasks.md` | Modify | Schedules no run |
| `framework/reference/plan.md` | Modify | No task schedules a run |
| `runtime/src/primitives/check_review_gate.rs` | Modify | Blocks on an open mark, a break left open and an owed review, review checks first |
| `runtime/src/primitives/check_prescriptions.rs` | Modify | Shares the round's filter |
| `runtime/src/primitives/check_artifacts.rs` | Modify | Review-state drift reports a broken contract left open |
| `runtime/src/primitives/decisions.rs` | Modify | `decided-by: rule`; a person's stored decision wins |
| `runtime/src/primitives/diff_cross_spec.rs` | Modify | Window and own commits |
| `runtime/src/primitives/derive_boundary.rs` | Modify | Doc comment no longer claims a shared window |
| `runtime/src/primitives/classify_spec_edit.rs` | Create | Correction or reopen, from the diff |
| `runtime/src/primitives/measure_reopenings.rs` | Create | The outcome, from history |
| `runtime/src/primitives/dashboard.rs` | Modify | The counts `/{project}:status` renders |
| `runtime/src/primitives/check_artifact_size.rs` | Modify | `OnDone` hints follow the reopen rule |
| `runtime/src/primitives/prune_plan.rs` | Modify | Only a person's waiver keeps a plan section; `reopen-required` takes the shared test of an added box |
| `runtime/src/primitives/read_tasks.rs` | Modify | The one test of what `tasks.md` owes, and the refusal both writers share |
| `runtime/src/primitives/mod.rs` | Modify | Module declarations |
| `runtime/src/schema/descriptions.rs` | Create | One description per primitive |
| `runtime/src/schema/primitives.rs` | Modify | Args and results |
| `runtime/src/schema/registry.rs` | Modify | `PRIMITIVE_REGISTRY` |
| `runtime/src/mcp/server.rs` | Modify | Handlers and description constants |
| `runtime/src/interpreter/mod.rs` | Modify | `dispatch_primitive` arms |
| `runtime/src/main.rs` | Modify | Subcommands and `about` constants |
| `runtime/tests/measure_reopenings.rs` | Create | Each measurement state |
| `runtime/tests/fix_rounds.rs` | Modify | Another spec's commit between rounds |
| `runtime/tests/review_command.rs` | Modify | Only a gating deferral reaches `gate-confirm` |
| `runtime/tests/walker.rs` | Modify | A bounded-result fixture's commit names its spec |
| `runtime/tests/mcp.rs` | Modify | A description equals its subcommand's `about` |
| `runtime/CHANGELOG.md` | Modify | Each change |
| `scripts/audit/marker-list-parity.sh` | Modify | A comment names the reference |
| `scripts/mutation/scope-attribution.py` | Create | Mutation list for attribution, window and scope |
| `scripts/mutation/reopen-measure.py` | Create | Mutation list for the measurement and the classifier |
| `scripts/mutation/one-description.py` | Create | Mutation list for a description's summary |
| `scripts/mutation/run.py` | Modify | Ids that select no mutation fail |
| `scripts/mutation/fix-round-record.py` | Modify | An anchor follows `passing_notices` |
| `runtime/tests/cross_service.rs` | Modify | What its second assertion now proves |
| `scripts/mutation/plan-edit-takes-no-back-edge.py` | Modify | The hints' renamed test and value |
| `runtime/tests/common/mod.rs` | Modify | One path to the release binary, following its build |
| `runtime/tests/{parity,exec_subprocess,manifest_cli_args,concurrent_sessions}.rs` | Modify | Use it |
| `runtime/tests/golden/implement-basic.jsonl` | Modify | Its fixture's commit shas move with `implement.md` |
| `framework/runtime-tools.txt` | Modify | Two new tools |
| `framework/constitution.md` | Modify | The four edits |
| `framework/commands/review.md` | Modify | Window, scope, rule-decided dispositions, open mark |
| `framework/commands/analyze.md` | Modify | Rule-decided dispositions |
| `framework/commands/implement.md` | Modify | `diff-cross-spec`, `review-owed` |
| `framework/commands/amend.md` | Modify | Reconcile calls `classify-spec-edit` |
| `framework/commands/status.md` | Modify | The counts |
| `framework/commands/help.md` | Modify | A corrected fact stays `done` |
| `framework/commands/*.md` | Modify | The host-integration pointer |
| `framework/migrations/{audit-record-relocate,criterion-label-backfill}.md` | Modify | A label's assignment is a correction |
| `framework/migrations/session-file-consolidate.md` | Modify | Runs through the CLI, with no hand-walk |
| `framework/bootstrap/{ductus,govern}.md` | Modify | The shell text is the bootstrap's own contract |
| `framework/bootstrap/configure/{claude,auggie}.md` | Modify | No hand-walk |
| `framework/reference/review.md` | Modify | The rules, then the cuts |
| `framework/reference/analyze.md` | Modify | The rules, then the cuts |
| `framework/reference/implement.md` | Modify | `diff-cross-spec`, the gate's fix-round and review-owed checks |
| `framework/reference/groom.md` | Modify | Dispositions |
| `framework/reference/status.md` | Modify | Drops what `dashboard` renders |
| `docs/analyze.md` | Modify | Dispositions |
| `docs/developing-ductus.md` | Create | Procedures moved out of `AGENTS.md` |
| `docs/slash-commands.md` | Modify | Dispositions, reopen |
| `README.md` | Modify | Confirmation claim |
| `AGENTS.md` | Modify | Retired window and fix-round entries |
| `specs/020-code-review/tasks.md` | Modify | Task 16 closes through this spec |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Modify | The largest set of corrected claims |
| `specs/058-findings-route-at-discovery/spec.md` | Modify | Dispositions, with a back-link |
| `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` | Modify | Fix-round claims, with a back-link |
| `specs/{000,010,011,014,016,017,019,020,022,023,026,029,030,032,035,041,045,046,047,048,049,050,051,054,055,057,059,060,062,063,064,067,071,072,075,076}-*/` | Modify | Corrected claims and signposts; `research.md`'s count was a floor |
| `specs/021-runtime-boundary/spec.md` | Modify | The eligibility criterion it quotes |

## Data Model

`data-model.md` lists the new record fields, result fields, arguments and constants. A primitive's behavior is canonical in its compiled description (AC9), so the file holds shapes only.

## Trade-offs

- **Attribution by subject alone** was rejected: it depends on a convention nothing ships. **By directory alone** misses the 35 commits that name a spec without touching a spec directory.
- **A marker in a local file** for a cut-off loop was rejected: the gate runs where the file may not exist, and `.ductus/review-rounds/` is gitignored and cleared.
- **Measuring in `scripts/`** was rejected: it does not ship (AC8).
- **`dashboard` measuring all of history** was rejected for the cost on every `/{project}:status`; a rolling window means an old miss ages out of the count, and `--since` still reaches it.
- **An unclaimed commit not making a review owed** was rejected (owner, 2026-11-05), though it is why 12 of the 47 `in-progress` specs with no scenario or data-model edit read `review-owed: true` at `37ec7496`, by `compute-review-scope` on each from a clean tree. Counting only an unclaimed commit on the plan's Affected Files frees 1 of them (015), and counting none frees 3 (015, 059, 061), while an adopter whose commits name no spec could then change a spec's code and reach `done` unread, since the gate compares no code. By a commit-claims estimate over the 126 reopenings since 2026-10-01 at `d62e72f3`, 12 owed a review only for this, holding 3 of 261 runs.
- **A person's lasting keep of a plan section** was dropped (owner, 2026-11-05): `/{project}:analyze` asks nobody about an advisory finding and has no `--waive`, so the keep is folding the section under a design-record heading, and a person's waiver stored earlier still holds. 3 of 70 analysis records held any `plan-record` decision. Having `/{project}:prune`'s decline store a waiver was rejected: rewriting the record reads as a second analysis run to `measure-reopenings`, so it needs a new write path. Letting the rule's waiver keep a section was rejected: `/{project}:prune` would then almost never propose one once a spec is analyzed, against §plan-phase.
- **`/{project}:implement` halting on `review-owed` beside the gate** was rejected (owner, 2026-11-06), though it is one sentence: the gate would still say `passed: true` over an owed review, and only a host following the step would stop. The spec's entry holds the numbers.
- **A new gate tier for contract breaks** was rejected: it falsifies the gating definitions across four specs. A break left open reuses `review-owed` instead, which `/{project}:review` clears.
- **A gate notice for a contract break**, chosen on 2026-11-05, was replaced (owner, 2026-11-06): it let a known broken contract reach `done` with no decision recorded. The spec's entry holds the options and their numbers.
- **Sweeping restatements that are still true** stays out, as the spec resolves. A false statement is corrected under AC12, including one that a data model is canonical, while a true copy waits until next edited.
- **Known limitations.** `measure-reopenings` reads a slug changed mid-reopening as starting there, undercounting that reopening's earlier runs; 0 of this history's 5 renames fell inside one, and it reports nothing as over the bound it was not. A consolidated spec's commits touching its directory stay excluded, named by number, so a spec consolidated below `done` leaves that code unread, as `/{project}:consolidate` claims no review; a commit that only named a spec gone from disk reads as no spec's, so a spec overlapping it owes a review of that path. Another session's uncommitted edits stay in scope and in a round. A change of meaning written only as prose does not reopen. A wrong correction waits for the next analysis. Code committed between a run and its record is read by nothing (spec's Resolved Questions). A record restored from before the one the transition holds still reads as a finish. A claim carried only in a reference's label reads `mechanical`, since the scrub removes label and target by its stated design. A person running `write-review` by hand writes what they give it, since the CLI cannot pass findings. An agent's call that a fix is mechanical, or a waiver right, stands until someone reads the record. `research.md`'s claim counts are a floor.

## Open Questions Resolved

The spec resolves all of them. Its entry on a reference's rules is why AC9 reaches references at all, and its entry on landing is why every change is in this plan.

## Cross-spec impact

The spec's `cross-spec-impact:` lists what each owes, discharged by a blockquote signpost back-linking this spec, which induces no dependency edge:

| Spec | Surface this changes |
| --- | --- |
| `020-code-review` | The review command and reference; its task 16 |
| `022-deterministic-runtime` | `diff-cross-spec`, tool descriptions, the shared schema |
| `026-framework-self-audit` | Families 1c and 18, which pin the text this amends; neither script's logic changed |
| `041-task-pruning` | Only a person's waiver keeps a plan section from `/{project}:prune`, since the disposition rule waives every `plan-record` finding |
| `050-constitution` | The constitution's four edits, under its size ceiling |
| `051-branch-scoped-spec-numbering` | What the review after a fold reads, which its AC24 requires |
| `058-findings-route-at-discovery` | Dispositions; its task 5 waits for this |
| `063-oversized-artifacts-warn-with-a-fix` | Which trims `check-artifact-size`'s warning says reopen a `done` spec, read as `classify-spec-edit` reads the result |
| `067-analyze-and-audit-checks` | A condition added to review-state drift |
| `066-audit-records-and-the-completion-gate` | Records, gate, scope; its tasks 17–21 rebase on this |
| `076-waived-and-deferred-findings-are-distinct-dispositions` | Dispositions; its data model held a false canonicality claim |
| `078-review-fix-rounds-are-re-read-within-a-bound` | The loop; its task 15 waits for this |

Correcting false claims also reopened `010-agent-autonomy`, `014-reclarify-backedge`, `019-config-decisions`, `023-govern-refinement`, `030-cross-service-references`, `032-opencode-agent` and `035-groom-session-target` (a scenario or data model), and `059-project-in-a-repository-subdirectory` and `062-concurrent-session-targets` (a criterion), each with a task naming what its reopening owed. These owe no back-link; the correction names this spec in its commit. Task 19 rewrote the boxes that scheduled a run, or held a note as a box, in the task lists of 008, 010, 014, 019, 023, 026, 030, 032, 035, 048, 054, 058, 059, 062, 076 and 078 as prose, and task 26 those of 022 and 047, which task 19 missed, a correction to each that reopens nothing.
