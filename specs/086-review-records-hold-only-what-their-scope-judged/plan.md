# 086 — Review records hold only what their scope judged Plan

Implements [086 — Review records hold only what their scope judged](spec.md).

<!-- This plan records the design as it currently stands, in the sections below
     and no others — they are its design record (§plan-phase). When
     implementation changes a decision, edit that decision's entry to say what
     is now true; never append a note that it changed, because git history
     already keeps what the plan said when. Everything else has a home that is
     not the plan: verification evidence in the commit that lands the task,
     contributor knowledge where §drift-prevention's "Shared knowledge stays
     in git" routes it, a finding's decision where the finding was
     dispositioned, and handoff notes on the pending task in tasks.md. `/{project}:analyze`
     reports any other `##` section as outside the design record. -->

## Overview

One new runtime module says which files belong to a spec: those its plan names, and those a commit attributed to it changed. It walks history once for any number of specs. Four surfaces read it or the run's scope:

- `write-review` refuses an outcome on a file the run's scope does not hold (AC1).
- `process-decisions` and `process-waivers` drop, on an unrestricted run, a stored entry that did not fire, on a file the scope does not hold and that does not belong, and report it as dropped (AC2–AC4).
- `compute-review-scope` puts the files a spec's attributed commits changed into a first review's scope (AC6).
- A new primitive, `prune-review`, lets `/prune` drop the same entries between reviews and rewrite old Excluded from scope entries (AC3, AC5).

Each owning spec's surface changes here, in the spec that requires it, and each owner links back (`AGENTS.md`'s shared-surface rule, owner 2026-11-04).

## Technical Decisions

### Implementation follows 084's `done`

084's Resolved Questions, *Landing order*: every other open spec's implementation follows 084 reaching `done` (owner, 2026-11-24). Planning writes no code. 084's open tasks 8 to 10 also change the surfaces this plan changes. The working tree at `a6cba0be` held their uncommitted edits (`git diff HEAD -- runtime/src`, 2026-11-24):

- `write-review` gains a project subject and an optional `feature` (`WriteReviewArgs`, `resolve_scope`, `record_snapshot`).
- `RunScope::resolve` takes an optional feature.
- `ProcessDecisionsArgs` changes with it.

This plan cites only symbols that diff leaves in place, at the signatures it gives them. Task 1 carries the wait.

### One definition of "belongs"

A new module, `runtime/src/primitives/spec_ownership.rs`, is the only answer to whether a file belongs to a spec. Review, prune and the first review's scope read it, so no two of them can disagree.

- **Plan half.** The spec's Affected Files entries matched by `PlanEntries::names` (`compute_review_scope.rs`), the match `compute-review-scope` applies to a plan entry (spec, Behavior). The plan is read from disk, as `Attributor::plan_of` reads it.
- **History half.** Every commit reachable from `HEAD`:
  - Each commit is attributed once by `Attributor::claims` and divided by `SpecShare::split` (`commit_attribution.rs`). That is the per-path rule a review's window applies (`attribute_window` in `compute_review_scope.rs`).
  - A path a commit gave the spec is the spec's. So a commit claimed by several specs gives each only its share, as it does in a review's scope.
  - 086's Resolved Questions counted whole commits instead, and call that an approximation of the runtime's attribution.
- **One spec, cheaply.** The walk tests each commit with `Attributor::may_claim` first. That reads the spec root's listing and the subject, with no diff, and names no spec for a merge. Only a commit it names an asked spec for is diffed by `Attributor::claims` and then split. `may_claim` names every spec `claims` names (its test), so a walk over one spec gives it the full walk's answer (the agent's recommendation, adopted by the operator, 2026-11-26). `Attributor::belongs_to` would not, since a commit that only renames the spec's directory keeps the directory's tree id.
  - The cost was measured at `6dd9e0f6` on 2026-11-26, a release-profile build on Darwin arm64, over 5,196 commits. Each range is five runs after a warm-up. 089 alone, 3 commits diffed, took 0.114 to 0.115 s; 087 alone, 33 commits, 0.269 to 0.270 s; 022 alone, 910 commits, 2.44 to 2.52 s; one walk over all 83 specs took 7.51 to 7.85 s.
- **One walk for many specs.** The walk computes a commit's claims at most once and splits them for each asked spec that claims it.
  - The cost was measured at `a6cba0be` on 2026-11-24 with the release build. `compute-review-scope --feature 022-deterministic-runtime --since d3980837`, 022's first commit, walked 4,911 commits in 8.73 s real (`/usr/bin/time -p`).
  - Walking each spec's life separately would cover 301,702 commits over 81 specs, against 5,066 for the whole history. Each life was counted with `git rev-list --count` from the spec directory's first commit to `HEAD`, plus that commit.
- **States.**
  - Examined: it carries the plan entries and the owned paths.
  - Not examined, with a reason:
    - In a shallow repository the walk ends at a commit whose parents are missing. That is the cut `check_stuck` reads (`is_shallow` and a parentless commit; its fixture `shallow_at`).
    - A spec directory `parse_feature_dir` does not recognize is also not examined, since no commit can be attributed to it.
  - A spec with no commit yet is examined and owns its plan's files alone.
- **No spec.** 084's project record has no spec, so nothing here judges its entries by ownership.

### `write-review` refuses an outcome outside the scope (AC1)

- **Where.** In `resolve_scope`, beside `refuse_unread_outside_the_scope`. That is before `write_record`, so a refusal writes nothing (`run`'s doc: every refusal lands before any write).
- **The test** is `RunScope::holds`: a scope entry, or a file that a plan entry among the scope's entries names. A first review's scope holds a plan's pattern as one entry, so membership by literal entry alone would refuse a finding on a file the pattern names.
- **One scope.** `RunScope` gains a constructor from a resolved scope, which `RunScope::resolve` then calls. `write-review` builds its `RunScope` from the scope `resolve_scope` resolved, so the refusal tests the scope the record counts with no second `compute`.
- **Subjects.**
  - Each `findings[].file`.
  - Each `observations[].path`, without its `:line` suffix (`path_without_lines`).
  - An empty `path` is exempt (spec, Edge Cases). Criteria verdicts carry no file and are untouched.
- **Error.** `InvalidArgument` naming `findings[{idx}].file` or `observations[{idx}].path`, the file, and that the run's scope does not hold it. `an_open_break_names_its_file` names `observations[{idx}].path` the same way.
- **Empty diff base.** `resolve_scope` gives an empty `diff-base` no entries, so every anchored outcome is refused there. The reason says to pass the diff base `compute-review-scope` returned, as the `unread` refusal does.
- **The project record.** 084's project review writes through the same path, so the refusal holds there too.
- **An outcome outside the scope** (spec, Edge Cases). `framework/commands/review.md` step 10 takes each hand-back outcome on a file the scope does not hold out of the run before step 11's write.
  - A gating one (a MUST or SHOULD violation, or a broken contract) is fixed in the run when the fix is mechanical. Otherwise it is deferred to the home **§Groom decision tree** in `.ductus/reference/groom.md` chooses, put to the operator one at a time. `spec_ownership` names the specs whose files they are.
  - Any other outcome is named in the run's report and recorded nowhere.
  - **§Passes in a subagent** in `framework/reference/review.md` tells the passes to mark each outcome on a file outside the scope as such, so the host sorts the hand-back before step 11 rather than meeting the refusal there.
- **Departures are unchanged.** `Departures::in_scope` resolves `compute` with `--since` the diff base. That scope is the one AC1 tests, less what the run's own window carries in (open findings, unread entries; spec 079), so its filter still excludes a finding on a carried path. 073 is not impacted.

### Review drops what does not belong (AC2, AC3, AC4)

- **Which entries.** Only a stored entry the run carries in, that did not fire, on a file the scope does not hold and that is still on disk: today's `Retained::NotRead` case in `process-decisions` and the `why_unread` branch of `Pass::place` in `process-waivers`.
  - An entry that fired was read by the run and is kept.
  - One on a file the scope holds expires as it does today.
  - One on a file gone from disk expires as today, ahead of any ownership test, so AC4's case reads as expired, never as dropped.
  - An outcome the run records itself is never judged by this rule. AC1 makes its file one the scope holds.
- **Unrestricted only.** A run with `restricted` (dimension flags, or no pass run), or with `skipped-passes`, retains as today. So does a run given `since` (spec, Edge Cases). The ownership walk runs only when an entry reaches the test, lazily as `RunScope` is resolved, so a run with no such entry walks nothing.
- **Outcomes.**
  - When the file belongs, the entry is retained as `NotRead`.
  - When the file does not belong, the entry joins a new `dropped` list, never `expired` (`QUAL-CLAIM-001`). Its notice opens `decision dropped:` (`waiver dropped:` for an anchor), then names the key and the file and says that no plan entry names the file and no commit attributed to the spec changed it.
  - When ownership is not examined, the entry is retained under a new `Retained` class whose summary line names the reason.
- **Results.**
  - Each result gains `dropped` and `dropped-count`, bounded with its other lists in `lists-file` (`BE-PAYLOAD-001`).
  - Each also gains `ownership`, absent when no entry reached the test, and otherwise `examined`, or `not-examined` with `ownership-reason`.
- **The write.** `framework/commands/review.md` steps 8 and 9 pass each `dropped` entry to `write-review` with the expired ones, in `expired-waivers` and `expired-decisions`, so `write-review` gains no argument. They also show each dropped entry's notice, which names it (AC2).
- **Unclaimed code.** Under 084 a path no spec claims leaves the spec's scope. A decision stored on it before then therefore reaches this test, belongs to no spec, and is dropped. Nothing writes it into `{specs-root}/review.md` (AC3; 086's Resolved Questions).
- **Several records.** One key in several specs' records is judged in each record against that record's spec (spec, Edge Cases), since each call names one feature.

### A first review reads what the spec's commits changed (AC6)

- **Branch.** `compute`'s branch for a spec whose record at `HEAD` names no run (`recorded_run`) keeps adding the plan's Affected Files entries.
- **Owned files.** That branch also adds each path `spec_ownership` gives the spec that exists on disk, is not a link leaving the project (`leaves_project`), and is not in `deleted`. It then removes each such path from `excluded`, as the folded-in paths already leave it. A path another spec's commit also changed stays in scope, by 079's *One path, two commits*, and so does one with uncommitted edits that another open spec's plan holds: 084's AC23 leaves such a path out only when neither the spec's plan nor its own commits hold it, and on a first review its own commits are all of history, not the window's alone.
- **Not examined.** When ownership is not examined, the scope holds the plan entries alone, and the result's `ownership` says `not-examined` with its reason (`QUAL-CLAIM-001`). On a first review the field is always present, and on a later one it is absent.
- **Readers.** `framework/reference/review.md` §Window and coverage says a first review holds the plan's entries as written. It gains the owned files and the field. So does `COMPUTE_REVIEW_SCOPE` (`runtime/src/schema/descriptions.rs`).

### `prune-review` reduces a record between reviews (AC3, AC5)

- **A new primitive.** `prune-tasks` and `prune-plan` each own one file, and their results already carry two flattened halves (**§Adding or removing a runtime primitive** in `.ductus/docs/agents/developing-ductus.md`). A third file's half on either would mislead by name. It costs the seven registration sites that section counts, an argument type in `runtime/src/schema/primitives.rs`, a description, `framework/runtime-tools.txt`'s line, and the `claude-style` layout's allow entry, which `scripts/gen-configure-mcp.sh` regenerates; pi's bridge lists the server's tools when it loads, so it takes no entry.
- **Preview.** `feature`, or `all` for every feature's `review.md`, with `{specs-root}/review.md` excluded since it has no spec. With `all` there is one ownership walk. For each record it lists:
  - `drops`: each `decisions:` entry, by `key_anchor`, and each waiver anchor, read by `process_waivers::anchor_files` as the classifier reads it, on a file that does not belong. A malformed entry is never judged. The on-disk test is the one a review's run applies (`still_on_disk`, which `RunScope::why_unread` calls), so a prune drops what an unrestricted review would: a deleted file's entry is not dropped here, and it expires at the next unrestricted review (AC4).
  - `rewrites-count`: each `## Excluded from scope` line, outside fenced blocks and HTML comments, of the form ``- `{path}` — {claimants}`` whose tail is a comma-separated list every token of which `parse_feature_dir` reads, as written before `64946fe7`. A line ending `— left out of fix round {n}` is not rewritten. A count, never a list: over this repository at `c0cdd75e` the 10,640 paths were 546,974 of the walk's 688,549 compact bytes, and nothing reads them, since the digest covers the record (the agent's recommendation, adopted by the operator, 2026-11-26).
  - The record's digest, the sha256 of the record as read and of its drops, so a commit that changes what belongs refuses an apply as a change to the record does; and `ownership`.
- **Skips and refusals.**
  - A record carrying `fix-rounds-open: true` is skipped with reason `fix-rounds-open`.
  - A record whose frontmatter does not parse as a review record, or whose `decisions:` or `waivers:` list does not, is refused naming the file, held by `fix-rounds-open: true` or not. A record behind a link leaving the project is refused as an invalid path. A record whose lists will not splice line by line, as one in flow style, is refused with `UnsplicableRecord`. Each is an error for one `feature`, and a `skipped` entry in the `all` walk, which goes on, with reason `record-unreadable`, `outside-project` or `unsplicable` and the error as `detail`. The walk's skip reasons are a type of their own, so `SkipReason`, which `prune-tasks` and `prune-plan` share, does not widen.
  - Ownership not examined drops nothing, and the record is reported with its reason.
- **Apply.** `apply` with the previewed digest re-derives the same edits from a fresh `ReadSnapshot` read. It refuses when the digest differs, and writes with `write_if_unchanged`, so a concurrent review's write is refused rather than lost (`BE-TXN-002`; 041's data model, *The write guard*).
- **An applying walk.** `all` with `apply` takes `digests`, each listed record's digest keyed by its feature (on the CLI `--digests-json FILE`), and makes one ownership walk. It writes each record whose edits its digest still names. One whose edits differ, or that changed before its write, is skipped as `changed-since-read` and the others are written, as `prune-tasks`' walk does. One it holds no digest for is listed unwritten, and a key naming no feature directory refuses the call before any write (the agent's recommendation, adopted by the operator, 2026-11-26).
- **Byte-identical elsewhere.**
  - The edit splices text: it removes each dropped list item's lines from the frontmatter, and a waiver whose every anchor is dropped goes whole. It also replaces each rewritten line.
  - It then reads the result back against the original record less the drops: every other frontmatter field compared as a YAML value, an open-schema field included, and both lists through their readers. On any difference it refuses with `UnsplicableRecord` and writes nothing.
  - A record with nothing to drop or rewrite is not written.
- **Status.** A `review.md` edit owes no work, so the spec's status is unchanged. Its analysis goes stale, and no gate reads that on a `done` spec (086's Resolved Questions).

### `/prune` runs it

`framework/commands/prune.md`:

- Its preview step calls `prune-review` beside `prune-tasks` and `prune-plan`.
- The render step shows its drops, rewrites and skips.
- Its own `gate-confirm` and apply step follow the plan-section step.
- The artifact table's `review.md` row, the Scope Boundaries' writes, its description and its `ToolSearch` line say so.
- `--project` does not run it.

"Nothing to prune" now also needs no drop and no rewrite.

### Compatibility

**`BE-COMPAT-001`**, within major version zero, as 087's plan classifies its changes (`version` reads 0.64.0; the agent's recommendation, adopted by the operator, 2026-11-26).

- **Breaking:**
  - `write-review` refuses a finding or an observation on a file outside the scope, which it accepted before.
  - `process-decisions` and `process-waivers` drop an unfired entry on a file that does not belong, which they retained before.
  - A first review's `compute-review-scope` scope gains the files the spec's commits changed.
- **Additive:**
  - `dropped`, `dropped-count`, `ownership` and `ownership-reason` on both classifiers.
  - `ownership` and `ownership-reason` on `compute-review-scope`.
  - `prune-review`, a new primitive.
- No field is removed, renamed or retyped.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/spec_ownership.rs` | Create | The files a spec owns; one walk for many specs |
| `runtime/src/primitives/mod.rs` | Modify | Declare the two modules; `recorded_list_in`; `UnsplicableRecord` |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | `RunScope` from a resolved scope; owned files on a first review; `ownership`; `still_on_disk` |
| `runtime/src/primitives/write_review.rs` | Modify | Refuse an outcome outside the scope; its tests' window |
| `runtime/src/primitives/write_review/departure_waivers.rs` | Modify | A departure's tests where an outcome outside the scope is refused |
| `runtime/src/primitives/write_review/project_record.rs` | Modify | The project record refuses the same way |
| `runtime/src/primitives/process_decisions.rs` | Modify | `dropped`, `ownership`, the new retained class |
| `runtime/src/primitives/process_waivers.rs` | Modify | `dropped`, `ownership`; `anchor_files` |
| `runtime/src/primitives/prune_review.rs` | Create | The record reduction |
| `runtime/src/schema/primitives.rs` | Modify | New fields and `prune-review`'s argument and result types |
| `runtime/src/schema/descriptions.rs` | Modify | Five descriptions |
| `runtime/src/schema/registry.rs` | Modify | Register `prune-review` |
| `runtime/src/mcp/server.rs` | Modify | Its tool |
| `runtime/src/interpreter/mod.rs` | Modify | Its dispatch arm; the exec review tests' window; prune's step numbers in two comments |
| `runtime/src/main.rs` | Modify | Its command |
| `runtime/tests/result_sizes.rs` | Modify | Cases for the new lists and the primitive |
| `runtime/tests/crlf_preservation.rs` | Modify | `prune-review` keeps CRLF |
| `runtime/tests/mcp.rs` | Modify | The MCP write's window; a read size above a first review's file form |
| `runtime/tests/walker.rs` | Modify | The exec waiver tests' window; prune's twelve steps and its review-record preview |
| `runtime/tests/golden/review-basic.jsonl` | Modify | Re-blessed for the review command's edit and a first review's scope |
| `runtime/tests/parity.rs` | Modify | The review fixture's harness committed apart from its spec |
| `runtime/tests/fixtures/review-basic/specs/007-review/plan.md` | Modify | The fixture's two commits |
| `scripts/mutation/spec-ownership-086.py` | Create | The ownership walk's mutants |
| `scripts/mutation/first-review-scope-086.py` | Create | The first review's owned files' mutants |
| `scripts/mutation/outcomes-outside-the-scope-086.py` | Create | The refusal's mutants |
| `scripts/mutation/project-record-084.py` | Modify | An anchor `RunScope::of` moved |
| `scripts/mutation/scope-attribution.py` | Modify | Anchors the refusal, `RunScope::of` and `still_on_disk` moved |
| `scripts/mutation/review-drops-unowned-086.py` | Create | The classifiers' drop's mutants |
| `scripts/mutation/prune-review-086.py` | Create | `prune-review`'s mutants |
| `scripts/mutation/process-decisions-results-bounded.py` | Modify | An anchor the `dropped` list moved |
| `scripts/mutation/process-decisions-retained.py` | Modify | Anchors the drop moved |
| `scripts/mutation/waivers-bounded.py` | Modify | An anchor the `dropped` list moved |
| `framework/runtime-tools.txt` | Modify | List `prune-review` |
| `framework/bootstrap/configure/claude.md` | Modify | Regenerated allow entry |
| `framework/commands/review.md` | Modify | Steps 8 and 9 pass and show `dropped`; step 10 sorts out outcomes outside the scope |
| `framework/reference/review.md` | Modify | Window and coverage; per-run waiver processing; the passes' brief |
| `framework/commands/prune.md` | Modify | The review-record steps, table row, scope, description |
| `framework/reference/prune.md` | Modify | The reopen's step number |
| `docs/slash-commands.md` | Modify | The prune entry names `review.md` |
| `README.md` | Modify | The prune entry and its anchor |
| `runtime/CHANGELOG.md` | Modify | `[Unreleased]` entries |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Modify | Refusal, `dropped`, `ownership`, first-review scope |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Modify | Signpost |
| `specs/066-audit-records-and-the-completion-gate/scenarios/review-base-includes-the-transition-commit.md` | Modify | A first review with no base holds the owned files |
| `specs/079-review-and-analysis-work-converges/data-model.md` | Modify | `modified-since`'s accounting on a first review |
| `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md` | Modify | The scope's first-review entries |
| `specs/022-deterministic-runtime/scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md` | Modify | prune's steps, and its review-record preview before the gate |
| `specs/022-deterministic-runtime/scenarios/parser-walker-conventions.md` | Modify | The gate step named, not numbered |
| `specs/022-deterministic-runtime/data-model.md` | Modify | prune's step numbers |
| `specs/041-task-pruning/data-model.md` | Modify | `prune-review`; the write guard's primitives |
| `specs/041-task-pruning/spec.md` | Modify | Body prose on `review.md`; AC9 note; signpost; AC15's step number |
| `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md` | Modify | The `review.md` row; prune's step numbers |
| `specs/041-task-pruning/plan.md` | Modify | prune's step numbers |
| `specs/075-agents-md-holds-what-every-session-needs/spec.md` | Modify | prune's step numbers |
| `specs/075-agents-md-holds-what-every-session-needs/plan.md` | Modify | prune's step numbers |
| `specs/084-reopen-for-owed-code-review-every-code-change/plan.md` | Modify | prune's step numbers |
| `specs/079-review-and-analysis-work-converges/spec.md` | Modify | AC24 and AC6 notes; the stored-finding edge case; signpost |
| `specs/058-findings-route-at-discovery/spec.md` | Modify | AC32 note; signpost |
| `specs/020-code-review/spec.md` | Modify | Signpost |
| `specs/020-code-review/scenarios/waiver-expiry.md` | Modify | Rule 4 names the drop |
| `specs/084-reopen-for-owed-code-review-every-code-change/spec.md` | Modify | Signpost |

Each layout's generated copies of the two commands, `.claude/commands/ductus/*.md` and `.pi/prompts/ductus-*.md`, follow from the hook.

## Data Model

This spec adds no `data-model.md`: each shape belongs to its primitive's owner, as 087 and 075 decided. The owner's data model changes in the task that changes the primitive:

- 066's gains `write-review`'s refusal, the `dropped` lists and `ownership` field of `process-decisions` and `process-waivers`, and `compute-review-scope`'s first-review scope and `ownership`.
- 041's gains a `prune-review` section, and its write guard names four primitives.

## Trade-offs

- **Rejected: a commit's whole paths for every claimant.** This is the method 086's counts used. It gives a multi-spec commit's every path to each spec that claims it, which a review's scope does not, so a decision would be kept on a file its spec's review never reads.
- **Rejected: a walk cut at each spec directory's first commit.** It is shorter for a young spec. A shared walk would then need an ancestry test per commit and spec to keep one definition.
- **Rejected: one walk per spec in `prune-review --all`.** It covers 301,702 commits against 5,066 (above).
- **Rejected: a preview-only walk, each record applied alone.** At `c0cdd75e`, with task 5's code uncommitted, in a `git clone --local` copy with the release build: the walk's preview took 8.0 to 8.2 s over 83 specs (three runs) and listed 61 records, while applying those 61 one at a time took 33.9 s summed (median 0.46 s, 022 the slowest at 2.66 s), one ownership walk and one call each. The applying walk costs one walk, a `digests` argument and a skip reason.
- **Rejected: extending `prune-plan` or `prune-tasks`.** It saves seven registration sites, and gives a primitive named for one file a write to another.
- **Rejected: re-serializing the frontmatter after a drop.** That is simpler, but it changes quoting and order in fields the spec requires byte-identical (AC5).
- **Rejected: membership by literal scope entry in the refusal.** It refuses a finding on a file a first review's plan pattern names.
- **Rejected: `prune-review` keeping an entry on a file with uncommitted changes.** It saves at most one question, for a prune run between a review and the commit of the code it read, at the cost of a status read and a narrower AC5 than the spec states.
- **Rejected: one `RunScope` for departures.** No criterion here needs it, and it would widen which findings a departure entry decides, which is 073's behavior.
- **Limitations.**
  - The ownership walk runs only when an answer is needed. A first review's `compute` needs it on every call: the primitive, each `write-review` write, each `RunScope::resolve` and `check-review-gate`'s `review-owed`. A classifier needs it only for an unfired entry outside the scope. Each single-spec call diffs only the commits `may_claim` names the spec for, and `prune-review --all` diffs each commit some spec may claim once, 7.5 to 7.9 s here over all 83 specs (measured above).
  - A decision a review recorded on code not yet committed does not belong until that commit. A `/prune` run in between drops it, and the next review that reads its file asks again.
  - A decision on code a staging spec folded below `done` belongs to the staging spec's commits, not the target's. It is dropped at the target's next unrestricted review that does not read its file, and asked again if a later review does.
  - A shallow clone drops nothing until it is unshallowed.

## Cross-spec impact

| Spec | What changes | Discharge |
| --- | --- | --- |
| 020 | `/{project}:review` steps 8 and 9 pass and show dropped entries | Signpost |
| 022 | Scenario `review-runtime-acceleration` states a first review's scope; its data model and two scenarios cite prune's step numbers | Each sentence corrected where it stands; nothing owed |
| 075 | Its spec and plan cite prune's step numbers | Corrected where they stand; nothing owed |
| 041 | `/{project}:prune` writes `review.md` through `prune-review`; AC9's "nothing to prune" | Signpost; data model; body prose; scenario row; AC9 note |
| 058 | AC32: an unevaluated source's decision is dropped when its file does not belong | AC32 note; signpost |
| 066 | `write-review`'s refusal; `process-decisions`, `process-waivers` and `compute-review-scope` fields; a first review's scope | Signpost; data model; scenario `review-base-includes-the-transition-commit` |
| 079 | AC24's retention, AC6's scope list, the edge case *A stored finding the run did not read*, `modified-since`'s accounting | Notes; edge case; data model; signpost |
| 084 | The project record's review refuses out-of-scope outcomes and inherits no feature decision; AC23's exclusion yields to a first review's owned files; its plan cites prune's step number | Signpost; an AC23 note if the owner asks for one (task 7) |

- **Notes.** Each note on a ticked criterion reads "Superseded in part (spec 086)", as 084's did. The ticked criterion stays ticked and its spec stays `done` (§spec-lifecycle).
- **Signposts.** Each signpost is a blockquote, which adds no `dependencies:` edge (`derive-dependencies`) and no owed work.
- **Frontmatter.** 086's `cross-spec-impact:` gains 020 and 058.
- **Not impacted.**
  - 073: its AC5 holds, as above.
  - 081 and 083: attribution is unchanged and only read.
  - 020's AC8 and AC19, and 073's AC5, were read and still hold.
