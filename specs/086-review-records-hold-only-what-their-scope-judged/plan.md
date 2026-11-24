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
- **One walk for many specs.** The walk computes each commit's claims once and splits them for each asked spec that claims it.
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
- **The departure filter goes.** `Departures::in_scope` resolved its own scope, `compute` with `--since` the diff base. That scope leaves out what the run's own window carries in (open findings, unread entries; spec 079). It now reads the `RunScope` `resolve_scope` built.
  - Every finding is then held, so its two filters over a finding's file exclude nothing and are removed. So is `departure_waiver_entries`' sentence that a finding outside the scope "stays counted" (`AGENTS.md`'s no-dead-code rule).
  - A departure entry now decides each finding of its rule anywhere in the scope the run counts, as 073's AC5 states it.

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
- **Owned files.** That branch also adds each path `spec_ownership` gives the spec that exists on disk, is not a link leaving the project (`leaves_project`), and is not in `deleted`. It then removes each such path from `excluded`, as the folded-in paths already leave it. A path another spec's commit also changed stays in scope, by 079's *One path, two commits*.
- **Not examined.** When ownership is not examined, the scope holds the plan entries alone, and the result's `ownership` says `not-examined` with its reason (`QUAL-CLAIM-001`). On a first review the field is always present, and on a later one it is absent.
- **Readers.** `framework/reference/review.md` §Window and coverage says a first review holds the plan's entries as written. It gains the owned files and the field. So does `COMPUTE_REVIEW_SCOPE` (`runtime/src/schema/descriptions.rs`).

### `prune-review` reduces a record between reviews (AC3, AC5)

- **A new primitive.** `prune-tasks` and `prune-plan` each own one file, and their results already carry two flattened halves (**§Adding or removing a runtime primitive** in `docs/developing-ductus.md`). A third file's half on either would mislead by name. It costs the seven registration sites that section counts, an argument type in `runtime/src/schema/primitives.rs`, a description, `framework/runtime-tools.txt`'s line, and each layout's allow entry, which `scripts/gen-configure-mcp.sh` regenerates.
- **Preview.** `feature`, or `all` for every feature's `review.md`, with `{specs-root}/review.md` excluded since it has no spec. With `all` there is one ownership walk. For each record it lists:
  - `drops`: each `decisions:` entry, by `key_anchor`, and each waiver anchor, by its `file`, on a file that does not belong. A deleted file's entry is not dropped here, and it expires at the next unrestricted review (AC4).
  - `rewrites`: each `## Excluded from scope` line of the form ``- `{path}` — {claimants}`` whose tail is a comma-separated list every token of which `parse_feature_dir` reads, as written before `64946fe7`. A line ending `— left out of fix round {n}` is not rewritten.
  - `kept`: each entry on a file with uncommitted changes (git status), which carries until a commit attributes it. A decision a review recorded on uncommitted code is not dropped before that code is committed, and nothing depends on the operator committing first.
  - The record's digest, and `ownership`.
- **Skips and refusals.**
  - A record carrying `fix-rounds-open: true` is skipped with reason `fix-rounds-open`.
  - A record `load_review_record` reads as `Unreadable`, or whose `decisions:` list `read_decisions` refuses, is refused naming the file. That is an error for one `feature`, and a `skipped` entry with its reason in the `all` walk, which goes on.
  - Ownership not examined drops nothing, and the record is reported with its reason.
- **Apply.** `apply` with the previewed digest re-derives the same edits from a fresh `ReadSnapshot` read. It refuses when the digest differs, and writes with `write_if_unchanged`, so a concurrent review's write is refused rather than lost (`BE-TXN-002`; 041's data model, *The write guard*).
- **Byte-identical elsewhere.**
  - The edit splices text: it removes each dropped list item's lines from the frontmatter, and a waiver whose every anchor is dropped goes whole. It also replaces each rewritten line.
  - It then parses the result and compares it with the original record less the dropped entries. On any difference it refuses as an internal error and writes nothing.
  - A record with nothing to drop or rewrite is not written.
- **Status.** A `review.md` edit owes no work, so the spec's status is unchanged. Its analysis goes stale, and no gate reads that on a `done` spec (086's Resolved Questions).

### `/prune` runs it

`framework/commands/prune.md`:

- Its preview step calls `prune-review` beside `prune-tasks` and `prune-plan`.
- The render step shows its drops, rewrites, kept entries and skips.
- Its own `gate-confirm` and apply step follow the plan-section step.
- The artifact table's `review.md` row, the Scope Boundaries' writes, its description and its `ToolSearch` line say so.
- `--project` does not run it.

"Nothing to prune" now also needs no drop and no rewrite.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/spec_ownership.rs` | Create | The files a spec owns; one walk for many specs |
| `runtime/src/primitives/mod.rs` | Modify | Declare the two modules |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | `RunScope` from a resolved scope; owned files on a first review; `ownership` |
| `runtime/src/primitives/write_review.rs` | Modify | Refuse an outcome outside the scope; one `RunScope` for departures |
| `runtime/src/primitives/process_decisions.rs` | Modify | `dropped`, `ownership`, the new retained class |
| `runtime/src/primitives/process_waivers.rs` | Modify | `dropped`, `ownership` |
| `runtime/src/primitives/prune_review.rs` | Create | The record reduction |
| `runtime/src/schema/primitives.rs` | Modify | New fields and `prune-review`'s argument and result types |
| `runtime/src/schema/descriptions.rs` | Modify | Five descriptions |
| `runtime/src/schema/registry.rs` | Modify | Register `prune-review` |
| `runtime/src/mcp/server.rs` | Modify | Its tool |
| `runtime/src/interpreter/mod.rs` | Modify | Its dispatch arm |
| `runtime/src/main.rs` | Modify | Its command |
| `runtime/tests/result_sizes.rs` | Modify | Cases for the new lists and the primitive |
| `framework/runtime-tools.txt` | Modify | List `prune-review` |
| `framework/bootstrap/configure/claude.md` | Modify | Regenerated allow entry |
| `framework/bootstrap/configure/pi.md` | Modify | Regenerated allow entry |
| `framework/commands/review.md` | Modify | Steps 8 and 9 pass and show `dropped` |
| `framework/reference/review.md` | Modify | Window and coverage; per-run waiver processing |
| `framework/commands/prune.md` | Modify | The review-record step, table row, scope, description |
| `runtime/CHANGELOG.md` | Modify | `[Unreleased]` entries |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Modify | Refusal, `dropped`, `ownership`, first-review scope |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Modify | Signpost |
| `specs/041-task-pruning/data-model.md` | Modify | `prune-review`; the write guard's primitives |
| `specs/041-task-pruning/spec.md` | Modify | Body prose on `review.md`; AC9 note; signpost |
| `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md` | Modify | The `review.md` row |
| `specs/079-review-and-analysis-work-converges/spec.md` | Modify | AC24 and AC6 notes; the stored-finding edge case; signpost |
| `specs/058-findings-route-at-discovery/spec.md` | Modify | AC32 note; signpost |
| `specs/020-code-review/spec.md` | Modify | Signpost |
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
- **Rejected: extending `prune-plan` or `prune-tasks`.** It saves seven registration sites, and gives a primitive named for one file a write to another.
- **Rejected: re-serializing the frontmatter after a drop.** That is simpler, but it changes quoting and order in fields the spec requires byte-identical (AC5).
- **Rejected: membership by literal scope entry in the refusal.** It refuses a finding on a file a first review's plan pattern names.
- **Limitations.**
  - The ownership walk costs up to about 9 s per call in this repository, and a review makes at most two such calls. A record whose every unfired entry is on a file the scope holds walks nothing.
  - A decision on code a staging spec folded below `done` belongs to the staging spec's commits, not the target's. It is dropped at the target's next unrestricted review that does not read its file, and asked again if a later review does.
  - A shallow clone drops nothing until it is unshallowed.

## Cross-spec impact

| Spec | What changes | Discharge |
| --- | --- | --- |
| 020 | `/{project}:review` steps 8 and 9 pass and show dropped entries | Signpost |
| 041 | `/{project}:prune` writes `review.md` through `prune-review`; AC9's "nothing to prune" | Signpost; data model; body prose; scenario row; AC9 note |
| 058 | AC32: an unevaluated source's decision is dropped when its file does not belong | AC32 note; signpost |
| 066 | `write-review`'s refusal; `process-decisions`, `process-waivers` and `compute-review-scope` fields | Signpost; data model |
| 079 | AC24's retention, AC6's scope list, the edge case *A stored finding the run did not read* | Notes; edge case; signpost |
| 084 | The project record's review refuses out-of-scope outcomes and inherits no feature decision | Signpost |

- **Notes.** Each note on a ticked criterion reads "Superseded in part (spec 086)", as 084's did. The ticked criterion stays ticked and its spec stays `done` (§spec-lifecycle).
- **Signposts.** Each signpost is a blockquote, which adds no `dependencies:` edge (`derive-dependencies`) and no owed work.
- **Frontmatter.** 086's `cross-spec-impact:` gains 020 and 058.
- **Not impacted.**
  - 073: its AC5 holds, as above.
  - 081 and 083: attribution is unchanged and only read.
  - 020's AC8 and AC19, and 073's AC5, were read and still hold.
