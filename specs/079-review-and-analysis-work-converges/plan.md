# 079 — Review and analysis work converges Plan

Implements [079 — Review and analysis work converges](spec.md).

## Overview

A shared commit-attribution helper gives the review window, scope, fix rounds and `diff-cross-spec` one answer to whose change a commit is. Two new primitives decide from history and from a diff what a person used to judge: `measure-reopenings` reports the outcome, and `classify-spec-edit` says whether an edit reopens. The review and analyze commands stop asking about non-gating observations. Each primitive gets one compiled description, the constitution makes the binary the source of its rules, and the references shrink last, so the cuts cannot hold up the changes that end the cycle.

`specs/system.md`, `specs/events.md` and `specs/errors.md` do not exist in this repository, so there is nothing to conflict with there. This spec declares no dependencies.

## Technical Decisions

### One commit-attribution helper

`runtime/src/primitives/commit_attribution.rs` answers `claims(commit)`: the specs whose directory the commit's diff against its first parent touches, plus those its subject's leading parenthesised scope names, with the first named kept, and the paths the commit contributes. No colon is required after the scope, since `impl(077) task 8 …` is a shape this history holds. Directories and scope tokens resolve through `parse_feature_dir`, so a branch-scoped spec counts and a number no spec has claims nothing. A merge commit claims nothing, since its branch's commits are in the walk, and contributes only the paths differing from every parent, so a clean merge adds none and the branch's work does not return as work no spec claims. History is read through `ProjectRepository`, converting each path with `to_git` and `to_project` (`runtime/src/primitives/mod.rs`). Both tests are needed: 35 commits since 2026-10-01 name a spec without touching a spec directory, and 30 do the reverse. An adopter whose subjects name no spec loses only the first-named step, and AC6 then gives the path to every claimant. Its fixture histories are unit tests, since `git_fixture` is `#[cfg(test)]` and `scripts/mutation/run.py` runs `cargo test --lib`.

### The window

`compute` in `runtime/src/primitives/compute_review_scope.rs` resolves today's base from `find_in_progress_commit` (`runtime/src/primitives/check_stuck.rs`). A new step walks the commits since that transition for the newest one that changed `review.md`'s `last-run`, skipping a record with `skipped-passes`, `budget-stop: true`, `fix-rounds-open`, or frontmatter that does not parse. When it finds one, that commit is the base. `--since`, an uncommitted transition and a spec with no transition keep today's behavior. The commit holding a record also holds that run's fixes, which its fix rounds already re-read, so excluding them is the point.

### The scope

`compute` replaces the union with AC6's rule, walking `base..HEAD` through the helper. A commit the spec alone claims, or no spec claims, contributes every path. A commit several specs claim contributes the spec's directory, its plan's Affected Files, and a path in none of the claimants' plans when the subject names this spec first or names none. Uncommitted changes join whole. A plan entry unchanged in the window stays only when no earlier review record exists in history. Globs match through the plan's existing `read_plan_affected` entries. `modified-since` keeps its meaning, and what the scope leaves out becomes `excluded`, in the bounded lists (`runtime/src/primitives/bounded_lists.rs`). `write-review` derives its denominator from `compute`, so the record follows, and it names each excluded path by the spec that claimed it.

`review-owed` is false when a review record already exists and the scope holds no path outside the spec's directory and no scenario or data-model edit (`is_review_contract`, `runtime/src/primitives/analyze_subjects.rs`). With no record it is true, because `never_reviewed_block` holds `done` on a spec's first pass and AC4 presumes an earlier review. For a reopening with a record, the gate needs no change: its review check is digest-based (`stale_review_block`, `runtime/src/primitives/check_review_gate.rs`), so it already passes. The review command reports `review-owed: false` and stops unless asked to continue, and `/{project}:implement`'s closing guidance reads it.

### Fix rounds

`round_paths` (`runtime/src/primitives/fix_rounds.rs`) gains the filter. Each snapshot's parent is `HEAD` at its time, so the commits between two snapshots' parents are knowable with nothing reported by the agent. A path leaves the round when a commit that only other specs claim changed it, and its content at both snapshots equals its content at that snapshot's parent, so no uncommitted edit sits on top. A commit this spec or no spec claims stays in the round. Another session's uncommitted edits cannot be told apart and stay.

`classify` (`runtime/src/primitives/brief_re_read.rs`) keeps its test per path, so an exempt path leaves the brief and `round.toml` even in a mixed round. A finding on one then counts as out of round through `demote_out_of_round`, which is right: `/{project}:analyze` reads that text. The result returns `exempt` and `excluded`, and `write-review` records them per round.

### The open mark

`write-review` gains `loop-open`, passed by step 11 of `framework/commands/review.md`, which writes `fix-rounds-open: true`. Step 12's last write omits it. An argument, not a default, because under `ductus exec` the walker no-ops step 12 and a default would block every exec run. `fix_round_block` (`runtime/src/primitives/check_review_gate.rs`) blocks on it as a new `ReviewGateBlock` variant, named by `/{project}:review`. `passing_notices` keeps its notice for a record with no loop fields.

### `diff-cross-spec`

`runtime/src/primitives/diff_cross_spec.rs` starts at the same transition parent the window uses, and walks commits through the helper, keeping sibling paths only from commits belonging to the spec, plus uncommitted sibling paths. A spec never `in-progress` keeps the first spec-directory commit, so a planned spec still gets an answer. The no-commit `guidance` stays. `derive-boundary` is unchanged, so its doc comment's claim about a shared window is corrected (`runtime/src/primitives/derive_boundary.rs`).

### Dispositions

Steps 10 of `framework/commands/review.md` and 19 of `framework/commands/analyze.md` change: a non-gating finding follows the rule, with no `gate-confirm`, and only a gating finding's waiver or deferral reaches the operator. A stored decision by a person still wins (`process-decisions`). Under `ductus exec` nothing changes, since with no operator nothing is written.

"Counts as gating" means a contract-breaking observation takes a gating finding's dispositions: it is fixed in the run or deferred to the spec's own task, its deferral reaches the operator, and the record counts it in `contract-breaks`. `check-review-gate` gains no tier: an unchecked task already keeps a spec from `done`, so the MUST, SHOULD, hard-fail and blocking definitions in the constitution, 058, 076 and 066 stay true.

### What reopens

`classify-spec-edit` decides from the diff, since a rule that depends on an agent remembering it would be skipped (§design-principles). It compares `spec.md`'s criteria and open questions through `read-spec`'s parser, unchecked tasks through `read-tasks`', and the changed path set, and returns `correction` only when the diff touches nothing but `spec.md` and `plan.md` and changes no criterion text, question or unchecked task. A label's assignment stays a correction. `/{project}:amend`'s reconcile pass calls it before proposing a reopen, and `check_artifact_size.rs`'s `OnDone` hints follow it. Nothing enforces a reopen today, so this adds no gate. `scripts/audit/cross-doc-consistency.sh` pins the back-edge text and changes with it.

### The measurement

`measure-reopenings` lives in `runtime/src/primitives/measure_reopenings.rs`, because AC8 needs it in an adopter's project and AC19 needs it in `dashboard`. One revwalk reads each `specs/*/spec.md`'s `status:` by blob, memoized as `status_of_commit` does (`runtime/src/primitives/check_stuck.rs`), and delimits reopenings. A run is a distinct `last-run` in the committed record inside a reopening. A second run is over the bound unless the previous record of the same kind in that reopening was `blocking`, or counted hard-fail, blocking, MUST, SHOULD or `contract-breaks`. A review's two writes share step 11's `reviewed-at`, so they are one run. A shallow clone is detected through `Repository::is_shallow` and reports `shallow-history`. The default window keeps `/{project}:status` fast and lets old misses age out; `--since` serves AC20.

The baseline check is a recorded command result at `d62e72f3`, not a test, because a test reading real history would need a full-depth checkout to run at all (§design-principles). A fixture repository proves each state in `runtime/tests/measure_reopenings.rs`.

### One description per primitive

`runtime/src/schema/descriptions.rs` holds one `pub const` per primitive. `#[tool(description = …)]` takes an expression (`rmcp-macros` 3.8.0's `tool.rs`), and clap takes `long_about` and `about` in `runtime/src/main.rs`, so a subcommand's `--help` prints the whole text and the `ductus --help` list its first sentence, which keeps that list readable. The move is byte-identical for MCP: the 66 existing strings become constants, the one CLI-only primitive of the registry's 67 gets one too, and `--help` adopts them, replacing 13,568 bytes of separate doc comments. A test holds each served tool's description equal to its subcommand's `long_about`. Module docs lose only what restates the description; a sweep of them is not owed.

### The constitution

Four edits, net-negative together, since it holds 49,936 of 50,000 bytes:

- §runtime-boundary's host integration and eligibility criterion 3: the binary is the source of a primitive's rules (AC21).
- §spec-lifecycle's third back-edge and its shared test: a correction reopens nothing (AC15).
- §brownfield-inbox's Finding dispositions: "each gating write is confirmed first", and "Scope decides the destination" narrowed to `/{project}:implement` (AC2, AC17).
- The audit-record field list gains the three fields, and §drift-prevention's table gains a row for a primitive's behavior (AC9, AC14).

The bytes come from the sentences these replace and from the two-paths text.

### Prose cuts come last

`research.md` holds the section-by-section estimate. Cuts follow AC21, so a reference can drop a primitive's rules, and they drop text rather than move it. `framework/reference/review.md` is cut first, as it needs the smaller share, and `framework/reference/analyze.md` is re-measured before anything is decided about a split. Every heading keeps its exact text. This takes over `020-code-review`'s task 16, which is closed by a back-link to this spec's result.

### `AGENTS.md`

AC10, AC11 and AC16 landed with their decisions. The window and `--since` entries that AC5 and AC6 retire are removed in the task that lands them, and the fix-round entries are re-read then.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/commit_attribution.rs` | Create | Whose change a commit is, with its fixture histories |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | Window, scope, `excluded`, `review-owed` |
| `runtime/src/primitives/fix_rounds.rs` | Modify | A round drops another spec's commits |
| `runtime/src/primitives/brief_re_read.rs` | Modify | Exempt paths leave a mixed round's brief |
| `runtime/src/primitives/write_review.rs` | Modify | Open mark, `excluded`, `contract-breaks`, `decided-by: rule` |
| `runtime/src/primitives/write_analysis.rs` | Modify | `decided-by: rule` |
| `runtime/src/primitives/check_review_gate.rs` | Modify | Blocks on an open mark |
| `runtime/src/primitives/diff_cross_spec.rs` | Modify | Window and own commits |
| `runtime/src/primitives/derive_boundary.rs` | Modify | Doc comment no longer claims a shared window |
| `runtime/src/primitives/classify_spec_edit.rs` | Create | Correction or reopen, from the diff |
| `runtime/src/primitives/measure_reopenings.rs` | Create | The outcome, from history |
| `runtime/src/primitives/dashboard.rs` | Modify | The counts `/{project}:status` renders |
| `runtime/src/primitives/check_artifact_size.rs` | Modify | `OnDone` hints follow the reopen rule |
| `runtime/src/primitives/mod.rs` | Modify | Module declarations |
| `runtime/src/schema/descriptions.rs` | Create | One description per primitive |
| `runtime/src/schema/primitives.rs` | Modify | Args and results |
| `runtime/src/schema/registry.rs` | Modify | `PRIMITIVE_REGISTRY` |
| `runtime/src/mcp/server.rs` | Modify | Handlers and description constants |
| `runtime/src/interpreter/mod.rs` | Modify | `dispatch_primitive` arms |
| `runtime/src/main.rs` | Modify | Subcommands and `about` constants |
| `runtime/tests/measure_reopenings.rs` | Create | Each measurement state |
| `runtime/tests/fix_rounds.rs` | Modify | Another spec's commit between rounds |
| `runtime/tests/mcp.rs` | Modify | A description equals its subcommand's `about` |
| `runtime/CHANGELOG.md` | Modify | Each change |
| `scripts/audit/cross-doc-consistency.sh` | Modify | The back-edge text it pins |
| `scripts/audit/marker-list-parity.sh` | Modify | The sentences it matches in `analyze.md` |
| `scripts/mutation/scope-attribution.py` | Create | Mutation list for attribution, window and scope |
| `scripts/mutation/reopen-measure.py` | Create | Mutation list for the measurement and the classifier |
| `framework/runtime-tools.txt` | Modify | Two new tools |
| `framework/constitution.md` | Modify | The four edits |
| `framework/commands/review.md` | Modify | Window, scope, rule-decided dispositions, open mark |
| `framework/commands/analyze.md` | Modify | Rule-decided dispositions |
| `framework/commands/implement.md` | Modify | `diff-cross-spec`, `review-owed` |
| `framework/commands/amend.md` | Modify | Reconcile calls `classify-spec-edit` |
| `framework/commands/status.md` | Modify | The counts |
| `framework/commands/help.md` | Modify | A corrected fact stays `done` |
| `framework/reference/review.md` | Modify | The rules, then the cuts |
| `framework/reference/analyze.md` | Modify | The rules, then the cuts |
| `framework/reference/implement.md` | Modify | `diff-cross-spec`, gate check 12 |
| `framework/reference/groom.md` | Modify | Dispositions |
| `docs/analyze.md` | Modify | Dispositions |
| `docs/slash-commands.md` | Modify | Dispositions, reopen |
| `README.md` | Modify | Confirmation claim |
| `AGENTS.md` | Modify | Retired window and fix-round entries |
| `specs/020-code-review/tasks.md` | Modify | Task 16 closes through this spec |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Modify | The largest set of corrected claims |
| `specs/058-findings-route-at-discovery/spec.md` | Modify | Dispositions, with a back-link |
| `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` | Modify | Fix-round claims, with a back-link |
| `specs/{000,011,016,017,023,041,045,046,047,050,054,059,063,075,076}-*/` | Modify | Corrected claims, per `research.md` |

## Data Model

`data-model.md` lists the new record fields, result fields, arguments and constants. A primitive's behavior is canonical in its compiled description (AC9), so the file holds shapes only.

## Trade-offs

- **Attribution by subject alone** was rejected: it depends on a convention nothing ships. **By directory alone** misses the 35 commits that name a spec without touching a spec directory.
- **A marker in a local file** for a cut-off loop was rejected: the gate runs where the file may not exist, and `.ductus/review-rounds/` is gitignored and cleared.
- **Measuring in `scripts/`** was rejected: it does not ship (AC8).
- **`dashboard` measuring all of history** was rejected for the cost on every `/{project}:status`; a rolling window means an old miss ages out of the count, and `--since` still reaches it. What the walk adds to `/{project}:status` is not measured until the primitive exists; if it is noticeable, the window narrows.
- **A new gate tier for contract breaks** was rejected: it falsifies the gating definitions across four specs, and an unchecked task already holds `done`.
- **Sweeping restatements that are still true** stays out, as the spec resolves. A false statement is corrected under AC12, including one that a data model is canonical, while a true copy waits until next edited.
- **Known limitations.** Another session's uncommitted edits stay in scope and in a round. A change of meaning written only as prose does not reopen. A wrong correction waits for the next analysis. An agent's call that a fix is mechanical, or a waiver right, stands until someone reads the record. The cut estimates are unproven until the text is rewritten, so a split stays possible for `framework/reference/analyze.md`. `research.md`'s claim counts are a floor.

## Open Questions Resolved

The spec resolves all of them. Its entry on a reference's rules is why AC9 reaches references at all, and its entry on landing is why every change is in this plan.

## Cross-spec impact

The spec's `cross-spec-impact:` lists what each owes, discharged by a blockquote signpost back-linking this spec, which induces no dependency edge:

| Spec | Status | Surface this changes |
| --- | --- | --- |
| `020-code-review` | in-progress | The review command and reference; its task 16 |
| `022-deterministic-runtime` | in-progress | `diff-cross-spec`, tool descriptions, the shared schema |
| `026-framework-self-audit` | in-progress | `scripts/audit/cross-doc-consistency.sh`, `scripts/audit/marker-list-parity.sh` |
| `050-constitution` | in-progress | The constitution's four edits, under its size ceiling |
| `058-findings-route-at-discovery` | in-progress | Dispositions; its task 5 waits for this |
| `066-audit-records-and-the-completion-gate` | in-progress | Records, gate, scope; its tasks 17–21 rebase on this |
| `076-waived-and-deferred-findings-are-distinct-dispositions` | done | Dispositions; its data model holds a false canonicality claim, so it reopens and owes a review |
| `078-review-fix-rounds-are-re-read-within-a-bound` | in-progress | The loop; its task 15 waits for this |

Correcting false claims also reopens `023-govern-refinement` (a scenario, owing a review) and `059-project-in-a-repository-subdirectory` (a ticked criterion, owing an analysis), and at most 7 more if AC21's text search proves false lines in a scenario or data model. These owe no back-link; each correction names this spec in its commit.
