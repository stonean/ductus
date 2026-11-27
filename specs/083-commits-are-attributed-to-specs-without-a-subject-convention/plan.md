# 083 — Commits are attributed to specs without a subject convention Plan

Implements [083 — Commits are attributed to specs without a subject convention](spec.md).

## Overview

Two independent changes. `/{project}:implement`'s per-task commit holds the task's code and its `tasks.md` tick, so the attribution rule that already reads spec directories claims it. `compute-review-scope` counts the commits its window walk examined and those no spec claimed, and `write-review` renders both in `review.md`'s `## Summary`. No attribution rule changes: `Attributor::claims` and `subject_scope_forms` in `runtime/src/primitives/commit_attribution.rs` are untouched.

## Technical Decisions

### The commit step

`framework/reference/implement.md` §Walk through tasks (per task, in order) step 8 becomes the commit step. After step 7's tick, the host stages exactly the paths the task edited and `tasks.md`, never the whole tree, so another session's files stay out, and commits them in one commit. With `--auto` it commits on its own. Without it, it shows the staged paths and a proposed subject and commits once the operator confirms, taking any subject the operator gives instead. A declined commit commits nothing and the walk goes on to step 9's next-step offer. A commit the hook refuses halts the walk, reporting the refusal, with the task's changes left on disk. Nothing is pushed. The proposed subject follows no form the framework names, since it prescribes none (AC14). In `framework/commands/implement.md`, the `--auto` Flags bullet and step 7's "remind the user to commit" point at that step instead of a reminder.

The commit is host work with no primitive, like the rest of step 7's summary, so `ductus exec`'s walker makes no commit, as before this spec: it no-ops a step's prose (`Step::Prose` in `runtime/src/interpreter/mod.rs`) and runs only the primitives a step names (`PRIMITIVE_NAMES` in `runtime/src/parser/mod.rs`), none of which commits to a branch: a search of `runtime/src` for `.commit(` and `"commit"` finds each in test code but `snapshot` in `runtime/src/primitives/fix_rounds.rs`, whose fix-round snapshot is a commit object no ref points at.

Why the tick decides: `attribute_window` in `runtime/src/primitives/compute_review_scope.rs` gives a commit whose subject names no spec, and which touches one spec's directory, to that spec alone, every changed path in its scope and none in another's (079's AC6 and 081's rule, read at `0db66fcc` and after `c55f7222`).

### Counting in the window walk

`attribute_window` counts each commit its walk visits as examined. It counts one as unclaimed when `CommitClaims::claimants` is empty and `CommitClaims::paths` is not, before its paths are placed. An unclaimed commit's paths go to `excluded`. A clean merge has no paths, since `Attributor::own_paths` keeps only what a merge changed itself, so it is examined and never unclaimed. The counts travel on `WindowClaims` into `ReviewScope` as an `Option`. It is `None` on the branch of `compute` that skips `attribute_window` because `diff_base` is empty, and `Some` with zeros for a window that walks no commit, such as a `HEAD` base.

### Result fields

`ComputeReviewScopeResult` in `runtime/src/schema/primitives.rs` gains `commits-examined` and `commits-unclaimed`. Both are `Option<usize>`, omitted when `None`, and carried inline in both forms like `scope-count`. They are counts, not lists, so the lists file is unchanged. The `COMPUTE_REVIEW_SCOPE` description in `runtime/src/schema/descriptions.rs` states both fields and their absence without a window.

**`BE-COMPAT-001`:** additive. `compute-review-scope`'s result gains two optional response fields, absent with no window, and `write-review`'s regenerated `## Summary` gains one line; no field is removed, retyped or made required, so a consumer reading either as before reads it unchanged. It ships in the next `ductus-v` release, as the unreleased entry in `runtime/CHANGELOG.md` records. **`BE-SCHEMA-002`:** the result's published schema is derived from `ComputeReviewScopeResult` itself (`#[derive(JsonSchema)]`, served through `runtime/src/mcp/mod.rs`), its single source.

### The Summary line

`write-review`'s `resolve_scope` in `runtime/src/primitives/write_review.rs` already calls `compute_review_scope::compute` with the run's diff base, and returns early when that base is empty. It returns the counts from that call alongside the scope count and `excluded`. `Record` carries them, and `report_summary` appends one line on both its branches, as it appends `loop_summary`: `{unclaimed} of {examined} commits in the window no spec claimed.`, or `The scope had no window, so no commit was examined.` when the counts are `None`. The record's frontmatter gains no field. 020's data model lists `## Summary` among the body sections without fixing its lines (its **Body sections (in order)** table's row reads only `Always`), so it needs no edit. The `WRITE_REVIEW` description states the line.

### Where the shape is documented

066's data model, section `compute-review-scope`, is the result's documented host contract, so the two fields are documented there, not in a data model of 083's own. Its **Size** paragraph's measured file-form bytes grow with the two inline counts, so they are re-measured with the release binary by the method the paragraph states. A blockquoted signpost in 066's `spec.md` links back to 083, which is the link `check-review-gate`'s cross-spec-impact check reads (`spec.md` body or a scenario: `links_back` in `runtime/src/primitives/check_review_gate.rs`, and its test `a_blockquoted_signpost_discharges_the_obligation`). As a blockquote it induces no `dependencies:` edge, since `derive-dependencies` reads no link on a blockquote-prefixed line (`DERIVE_DEPENDENCIES` in `runtime/src/schema/descriptions.rs`).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/reference/implement.md` | Modify | Step 8 becomes the commit step |
| `framework/commands/implement.md` | Modify | The `--auto` bullet and step 7 point at the commit step |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | Count examined and unclaimed commits; fixtures for AC11, AC12, AC13 |
| `runtime/src/schema/primitives.rs` | Modify | `commits-examined` and `commits-unclaimed` on the result |
| `runtime/src/schema/descriptions.rs` | Modify | `COMPUTE_REVIEW_SCOPE` and `WRITE_REVIEW` state the counts |
| `runtime/src/primitives/write_review.rs` | Modify | Summary line, and its tests, including those asserting the Summary's exact text |
| `runtime/CHANGELOG.md` | Modify | Unreleased entry |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Modify | The two fields and the re-measured Size figures |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Modify | Signpost linking back to 083 |

## Trade-offs

- **A data model of 083's own** was rejected. 066's data model would still need its Size figures re-measured, so 066 reopens either way, and the result's contract would sit in two specs.
- **Counting after credit** was rejected (Resolved Questions): the count would fall to zero after one crediting review.
- **Limitation:** a task's code committed before its tick, by hand, through `ductus exec`, or outside `/{project}:implement`, is still unclaimed. The count reports it, and since spec 084 it owes the project record's review.
- **Limitation:** staging only the task's paths cannot keep another session's hunks out of a file both sessions edited. Isolating edits stays `git worktree`'s job (§concurrent-features).

## Cross-spec impact

- **066 — Audit Records and the Completion Gate** owes the data-model sync above. An edit to a data model leaves a `done` spec `done` (spec 084), and because the change is confined to artifacts it owes an analysis and no review (081). Its signpost discharges 083's `cross-spec-impact:` entry.
