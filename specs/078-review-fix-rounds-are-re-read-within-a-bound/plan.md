# 078 — Review fix rounds are re-read within a bound Plan

Implements [078 — Review fix rounds are re-read within a bound](spec.md).

## Overview

Four feature-specific runtime primitives carry every bound the spec derives: `start-fix-rounds` snapshots the base, `brief-re-read` derives a round's paths, classifies it and writes its brief, `read-re-read` reads the re-read's result file, and `check-prescriptions` runs the diff check. `framework/commands/review.md` gains a loop step between dispositioning and its final `write-review` that calls them and holds the budget. The record fields and the gate check belong to 066, which owns the audit records and the completion gate, and land through its scenario `the-fix-round-loop-is-recorded-and-gated`. A `[review] re-reads-off` key switches the loop off with a reason. The shapes are in [data-model.md](data-model.md).

## Technical Decisions

### Four primitives, owned by this feature

AGENTS.md routes "a feature-specific primitive" to its feature, and these four exist only for the loop. Each is registered at the six sites `docs/developing-ductus.md` names (`PRIMITIVE_REGISTRY`, its description constant, a `#[tool]`, a `dispatch_primitive` arm, a `main.rs` `Command`, a `framework/runtime-tools.txt` line); all four are MCP tools, since each writes only inside the project and its `.git` (§runtime-boundary principle 5). `framework/bootstrap/configure/*.md` allow blocks follow from `scripts/gen-configure-mcp.sh`.

### The snapshot is an unreferenced commit object

`compute-review-scope` peels `--since` to a commit and diffs that commit's tree to the index and working tree, untracked files included (`runtime/src/primitives/compute_review_scope.rs`, `run` and `diff_since`), so a base must be a commit. The snapshot is built from an in-memory copy of the repository's index: add every non-ignored working-tree file, apply deletions, write the tree objects, and create a commit with `HEAD` as parent and no ref update. Writing the tree through git2 leaves the index file untouched as long as the index is never written back: `start_fix_rounds`' `the_snapshot_moves_nothing` asserts the index file's bytes, `HEAD` and every ref unchanged. libgit2's `add_all` applies the working tree's deletions itself (`runtime/src/primitives/start_fix_rounds.rs`, `the_snapshot_is_the_working_tree`, deletes a file and asserts the snapshot lacks it). An unreferenced object outlives a run, since git's default `gc.pruneExpire` is two weeks (git-config(1), `gc.pruneExpire`: `git gc` calls `prune --expire 2.weeks.ago`).

### A round's paths are a tree-to-tree diff

`brief-re-read` snapshots the tree first and diffs the base's tree against the snapshot's. `compute-review-scope`'s `modified-since` cannot serve: it diffs from the base through the index, and a snapshot base holds unstaged edits the index lacks, so it reports every one of them in every round. The diff is bounded to the project by `project_diff_options` and `project_paths`, extracted from `diff_since` rather than copied (`QUAL-DELEG-001`), so the two bound a project alike.

### Exemption is a path predicate the runtime evaluates

A path is exempt, and a round with nothing else exempt (spec 079), when it is an analyze subject of the reviewed feature and not a durable contract, reusing `is_analyze_subject` and `is_review_contract` (`runtime/src/primitives/analyze_subjects.rs`). The predicate follows the reason the exemption exists: an edit to an analyze subject stales `analysis.md`, which the gate holds `done` on. So `analysis.md` itself, which is not an analyze subject, is not exempt, and nor is a non-markdown file in the feature directory.

### The runtime writes the brief

A brief the host writes can carry scope the fixer chose, so `brief-re-read` renders it whole: the derived paths, the diff command, the severity bar and the result-file contract. The bar's text is a constant in `brief_re_read.rs`, stated as a requirement in the spec's *The loop* section; `framework/reference/review.md` points at the primitive rather than restating it. The host hands the re-read the brief's path and nothing else.

### The round directory ignores itself

`.ductus/review-rounds/` writes its own `.gitignore` of `*`, as `.ductus/sessions/` does (`runtime/src/session.rs`, `ignore_everything`). Its files then never enter a snapshot, and so no round's paths, including in an adopter whose `.gitignore` predates 078, so no gitignore migration is needed. One subdirectory per round is named by its base sha, and `start-fix-rounds` removes earlier ones.

### The re-read reports through a file the runtime reads

The re-read writes `result.json` last, with `complete: true`. `read-re-read` classifies it, so whether a re-read found a MUST or SHOULD is not the fixer's reading of a report, and a cut-off agent, which writes no complete file, cannot read as clean (`QUAL-CLAIM-001`). A host without subagents uses the same files: it prints the brief's path, the operator runs it in a fresh session of any host, and that session writes `result.json`.

### The loop's end and its budget

The loop ends at the first round with nothing changed since its base: after a clean re-read, or a `matched` diff check whose `open` is zero, every MUST and SHOULD of the re-read that prescribed the edits having carried an edit, each applied as written. A NOTE fixed afterwards is a change, so it is a round of its own. A matched check over fully prescribed findings stands for that re-read's clean result, since the spec's diff check replaces another re-read; a finding without an edit keeps the loop open. The host counts re-reads with a MUST or SHOULD; at the budget it stops and reports the open findings, the rounds, and, where the host reports it, the cost. A diff-checked round is not a re-read and does not count. Once the budget is reached, every further round waits on the operator. The budget's value — two, by AC6 — lives once, as `RE_READ_BUDGET` in `runtime/src/primitives/fix_rounds.rs`: `start-fix-rounds` reports it as `budget` for the host's loop, `write-review` derives `budget-stop` from it, and the command and reference name the result field rather than the number (`CFG-CONST-001`). An open MUST or SHOULD left at a stop is recorded as an observation, its severity in its text, with the operator's deferral or waiver: a re-read finding maps to no loaded rule, which is 066's definition of an observation (the **Write `review.md`** section of `framework/reference/review.md`, under Observations).

### The record and the gate are 066's

066 owns "the audit records and the completion gate" (AGENTS.md): the loop's fields and gate check landed as its scenario and task 15, and the Summary's account of a supplied Summary in this spec (task 16, owner, 2026-11-09), whose `cross-spec-impact:` holds `done` until 066 links back. `write-review` takes the per-round `fix-rounds` list and derives every count, so no caller can supply a count that disagrees with its rounds, and appends their account to the Summary, whether the host supplied it or `write-review` generated it; an absent list writes no field, which keeps a run that stated nothing apart from a zero, as `examined` does (`runtime/src/schema/primitives.rs`, `WriteReviewArgs.examined`). `check-review-gate` gains `fix-round-not-re-read`, after `undispositioned-findings`. The constitution's audit-record table gains the fields, which `scripts/lint-governing-size.sh` holds to 50,000 bytes, so that task cuts as much as it adds.

The change is **breaking** under `BE-COMPAT-001` in two places: `check-review-gate`'s `blocked-by` gains `fix-round-not-re-read` and, from spec 079, `fix-round-loop-open`, values its schema (`runtime/src/schema/primitives.rs`, `ReviewGateBlock`) does not document as open; and past the read size `brief-re-read`, `read-re-read` and `check-prescriptions` stop populating their lists inline, naming a list file instead (scenario `fix-round-results-have-a-bounded-size`). The rest is additive: four new tools, `write-review`'s optional `fix-rounds` argument, and record fields written only when it is passed. The runtime is pre-1.0, where Cargo's reading of SemVer makes the minor version the compatibility boundary (the Cargo Book, *Specifying Dependencies*, Caret requirements), so the release that carries 078 is a minor bump, as 063's plan reasons, and every consumer of the gate's result ships in that same release (§runtime-boundary).

### The switch is read by the runtime

`[review] re-reads-off` is one key whose value is the reason, validated as `[[review.disabled-rule-files]]`' reason is (`runtime/src/primitives/discover_rule_files.rs`, `validate_disabled_entry` and `MIN_REASON_LEN`), through one shared reader. `brief-re-read` reports it, so the host starts no agent. `write-review` applies its reason to every un-re-read round without one, as it resolves the constitution registry itself rather than taking it as an argument (`runtime/src/primitives/write_review.rs`, `render_unexamined_governance`), so the host cannot leave the waiver out. The key joins `runtime/src/schema/config_keys.rs` `SECTIONS` and `framework/bootstrap/ductus.md` §Project Configuration, which `the_installer_documents_the_registry` holds together.

### The exec path makes no round

`ductus exec` dispatches each backticked primitive with no per-step arguments (AGENTS.md, Workflow: a primitive is backticked in an `## Instructions` step only when the walker can supply its arguments), and the loop's primitives need a base, so the loop step names them unbackticked behind `<!-- audit:ignore-promotion -->`. The walker no-ops step 10's fixes (`framework/commands/review.md` step 12, its last sentence), so an exec run has no round, passes no `fix-rounds`, and its record carries no loop field.

### Every agent gets the same procedure

The loop step says what to do when the host can start an independent agent and when it cannot, and names no host, since the agent registry records no host's subagent support. The generated command copies for each agent change with their source. The implementing task walks `framework/bootstrap/ductus.md` §Derived values' layout-derived rows and records each layout's impact in its commit.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/fix_rounds.rs` | Create | Round directory, self-ignore, the `re-reads-off` reader, `result.json` parsing |
| `runtime/src/primitives/start_fix_rounds.rs` | Create | `start-fix-rounds` |
| `runtime/src/primitives/brief_re_read.rs` | Create | `brief-re-read` and the brief's text |
| `runtime/src/primitives/read_re_read.rs` | Create | `read-re-read` |
| `runtime/src/primitives/check_prescriptions.rs` | Create | `check-prescriptions` |
| `runtime/src/primitives/mod.rs` | Modify | Module declarations |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | Its project bound extracted for `brief-re-read`'s diff |
| `runtime/src/schema/primitives.rs` | Modify | Args and results of the four primitives |
| `runtime/src/schema/registry.rs` | Modify | `PRIMITIVE_REGISTRY` |
| `runtime/src/mcp/server.rs` | Modify | Four `#[tool]` handlers |
| `runtime/src/interpreter/mod.rs` | Modify | Four `dispatch_primitive` arms |
| `runtime/src/main.rs` | Modify | Four `Command` variants |
| `runtime/src/schema/config_keys.rs` | Modify | `[review] re-reads-off` |
| `runtime/src/primitives/discover_rule_files.rs` | Modify | `rendered` and `toml_type_name` shared with the `re-reads-off` reader |
| `runtime/tests/fix_rounds.rs` | Create | The four primitives driven through one loop |
| `runtime/tests/review_command.rs` | Modify | Step 12 classified as prose in the command's step table |
| `scripts/mutation/declined-note-edits.py` | Create | Mutation list: a declined NOTE edit forces no re-read |
| `scripts/mutation/fix-round-results-bounded.py` | Create | Mutation list: the loop results' bound |
| `scripts/mutation/loop-inputs-screened.py` | Create | Mutation list: `feature` and the switch screened before any write |
| `scripts/mutation/out-of-round-findings.py` | Create | Mutation list: a finding outside its round is a NOTE |
| `scripts/mutation/own-record-is-no-round.py` | Create | Mutation list: the review's own record is no round's path |
| `scripts/mutation/re-read-budget.py` | Create | Mutation list: the re-read budget's one home |
| `scripts/mutation/fix-round-record.py` | Create | Through 066's scenario task: the record fields' and the gate's fix-round check's mutation list |
| `runtime/tests/line_ending_discipline.rs` | Modify | `brief_re_read.rs` exempt: it writes generated state |
| `runtime/tests/golden/review-basic.jsonl` | Modify | Re-blessed for the changed command |
| `runtime/CHANGELOG.md` | Modify | Unreleased entry |
| `framework/runtime-tools.txt` | Modify | Four tool lines |
| `framework/bootstrap/configure/claude.md` | Modify | Generated allow block |
| `framework/bootstrap/configure/auggie.md` | Modify | Generated allow block |
| `framework/bootstrap/ductus.md` | Modify | §Project Configuration documents `re-reads-off` |
| `framework/commands/review.md` | Modify | The loop step and the Writes line |
| `framework/reference/review.md` | Modify | The loop's walk, the host fallback and the record fields' meaning |
| `runtime/src/primitives/write_review.rs` | Modify | Through 066's scenario task: `fix-rounds` and the record fields |
| `runtime/src/primitives/check_review_gate.rs` | Modify | Through 066's scenario task: the gate's fix-round check |
| `framework/constitution.md` | Modify | Through 066's scenario task: the audit-record table |
| `AGENTS.md` | Modify | The re-read rule points at the framework (AC9) |
| `README.md` | Modify | Documents `[review] re-reads-off` |
| `docs/slash-commands.md` | Modify | The review command's entry describes the loop |

## Data Model

[data-model.md](data-model.md) defines the round directory, the four primitives' arguments and results, the re-read's `result.json`, `write-review`'s `fix-rounds` argument and the record fields it derives, the gate's fix-round check, and the `[review] re-reads-off` key.

## Trade-offs

- **One primitive for the whole loop** — rejected: it would orchestrate a workflow and spawn nothing it could finish, and §runtime-boundary rules out a runtime that orchestrates. Four primitives each answer one deterministic question.
- **The host writing the brief or classifying the report** — rejected: both put the fixer between the bound and its input, which the spec exists to remove.
- **Record fields implemented under this spec** — rejected under AGENTS.md's routing rule as it read when they landed, which sent the audit records to 066; 066's `done` waits on its scenario's task, and this spec's AC12 and AC13 are verified against that task's work. The supplied Summary's account landed here (task 16) under the rule as it now reads.
- **Clearing the round directory never** — rejected: it would grow by a directory per round for the life of the checkout. Clearing it at the run's start means two concurrent reviews of one feature in one tree clear each other's rounds; §concurrent-features already answers a shared tree with `git worktree`.
- **Known limitation:** a snapshot stores every untracked, non-ignored file as a blob, so an unignored build directory makes each snapshot large until git prunes it.

## Cross-spec impact

- **066** owes the record fields and the gate's fix-round check, through its scenario `the-fix-round-loop-is-recorded-and-gated` and that scenario's task, which back-link this spec. The task's completion is what this spec's AC12 and AC13 are verified against.
- **020** owes its data model's review-record table a pointer to these fields. A blockquoted signpost in 020's spec body is the back-link, since a plain link there would add an edge from 020 to this spec, which already depends on 020.
