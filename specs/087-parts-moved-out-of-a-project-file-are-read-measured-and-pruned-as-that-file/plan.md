# 087 — Parts moved out of a project file are read, measured and pruned as that file Plan

Implements [087 — Parts moved out of a project file are read, measured and pruned as that file](spec.md).

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

One runtime function defines a project file's parts and the integrity findings over them. `check-artifact-size`, `prune-project` and `resolve-constitutions` take their parts from it. `check-artifact-size` with `project-files` is the list `/plan` and `/analyze` read before judging a plan against a project file. The size check, the shipped hook and `/analyze`'s findings read the same result. `prune-project` walks parts as subjects and can move a pointer's target into a parts directory. The migration entry and this repository's own procedure file both use that move. The rest is text: command sources and references, the constitution, 075's routing table and the memory-routing block. Each owning spec's surface changes here, in the spec that requires it, and each owner links back (`AGENTS.md`, the shared-runtime-surface rule, owner 2026-11-04).

## Technical Decisions

### Implementation follows 084's `done`

084's Resolved Questions, *Landing order*: every other open spec's implementation follows 084 reaching `done`, and a clarification may precede it (owner, 2026-11-24). Planning writes no code either. 084's pending tasks also change surfaces this plan changes: `check_artifact_size.rs` (its task 17 removes `OnDone`), `framework/commands/review.md`, `framework/reference/review.md`, `framework/bootstrap/` and `framework/migrations/` (084's implementing session, 2026-11-24). Task 1 carries the wait. This plan cites `check_artifact_size.rs` only by symbols 084 leaves in place.

### One definition of the part set

A new module, `runtime/src/primitives/project_parts.rs`, is the only enumeration of parts, so no two primitives can disagree about what a part is.

- **Parts directories.** `.ductus/docs/agents/`, `.ductus/docs/system/`, `.ductus/docs/events/` and `.ductus/docs/errors/` are under the project root, not the spec root. Each is keyed by the `ProjectFileKind` that `project_subjects` (`runtime/src/primitives/check_artifact_size.rs`) pairs with its file. `project_subjects` stays the four files.
- **A part** is a `*.md` regular file directly under one of them, listed in `scenario_name_cmp` order, the order `check-artifact-size` uses for scenarios. Any other file there is neither measured nor reported (AC11). A part that is, or lies under, a symbolic link leaving the project is `unreadable` and never opened, as a project file is (`leaves_project`; `BE-INPUT-004`).
- **Linkage.** Each project file's inline markdown links outside fenced blocks and HTML comments are resolved relative to that file. A part is linked when its own project file links it, since the index names each part (spec, *Parts*). A link from another project file is an ordinary link (spec, Edge Cases). The scan is `check-corpus-links`': `link_targets`, `link_file_part`, `link_base` and `lexical_join` in `runtime/src/primitives/check_corpus_links.rs`, the middle two taken out of its `classify` so a fragment, a scheme, a shape and a root-absolute target are judged once. Those helpers become `pub(crate)`, and are not copied. They run here the same way, except that a link in an HTML comment is skipped as one in a fence is, so nothing else widens (`QUAL-DELEG-001`).
- **Integrity findings.** Each carries a kind, a path and a runtime-worded message:
  - `missing-part`: a project file's link resolves into a parts directory, to no file.
  - `unlinked-part`.
  - `nested-file`: a markdown file at any depth in a subdirectory of a parts directory.
  - `not-a-parts-directory`: a directory under `.ductus/docs/` other than the four and `constitution/`, which is a shared constitution's parts directory in its own repository (the agent's recommendation, adopted by the operator, 2026-11-25).
  - `not-a-part`: a markdown file directly under `.ductus/docs/` (AC20).
- **What could not be looked at.** A directory under `.ductus/docs/` that is, or lies under, a symbolic link leaving the project, or that cannot be listed, is returned as unlisted, never as one holding no parts. A part whose project file exists but cannot be read has unknown linkage, never an `unlinked-part` finding (`QUAL-CLAIM-001`).
- **A shared constitution's directory.** `{checkout}/.ductus/docs/constitution/` is listed by the same directory function. It gets no linkage and no integrity findings, because no command here judges another repository's index (AC17).
- **No parts.** With no `.ductus/docs/`, the module returns no part and no finding.

### `check-artifact-size` supplies the list (AC1, AC2, AC4–AC6, AC11, AC12, AC20)

With `project-files`:

- **Two new lists.** `project-files` stays exactly four records (063's data model, **Size**). Two new top-level lists, `parts` and `part-findings`, each with an inline count, join `oversized`, `skipped` and `notices` in the bounded set. They are inline while the result fits `[artifacts] read-size-bytes`, and in `lists-file` past it, since both grow with the project (`BE-PAYLOAD-001`).
- **A part record** carries its path, its project file's path, `state` (`measured` or `unreadable`), `bytes` when measured, and `linked`. When staged it also carries `head-bytes` and `refused`, as a project file does (`measure_project_files`, `head_bytes`). `examined` counts measured parts.
- **A part over the threshold** joins `oversized` with a new `ArtifactKind`, `project-part`. Its one fix, a new `FixKind`, is splitting it into sibling parts, each named by its index (spec, *Parts*). Its `message` is the `artifact-size` finding message (`finding_message`), so a stored waiver covers it by the decided rule (`decide`). An `unreadable` part goes to `skipped` under `artifact-unreadable`.
- **Staged.** The part set is the index's entries directly under each parts directory, read through `ProjectRepository::discover` with `to_git` (spec 059). A part deleted in the commit is therefore absent and passes, and its index link becomes the `missing-part` finding of the next unstaged call (AC5; spec, Edge Cases). `part-findings` is absent under `staged`, not empty: the hook refuses on size alone (spec, Resolved Questions), and absent says the list was not computed.
- **`refused` and `refused-count`.** Staged only, `refused` lists the path of each refused project file and part, bounded with the other lists, and `refused-count` is its count, inline in both forms, which the CLI's exit reads. A count with no list of its own would have to be an `IRREGULAR_COUNTS` entry, which the shared test helper holds present in every file-form result, a feature-only one included (the agent's recommendation, adopted by the operator, 2026-11-25). `main.rs` names each refused project file and part, and each unmeasurable subject, on stderr from the result before it is bounded (`check_artifact_size::whole`), as it names a broken link from `check-corpus-links`' whole scan, so it reads no lists file. The shipped hook's call, `check-artifact-size --project-files --staged`, is unchanged.
- **A feature-only call** carries none of this, so `/clarify` and `/plan`'s step 8 get the same bytes as before.

### `/plan` and `/analyze` read every part before judging the plan (AC1, AC2)

**`/plan`.**

- In `framework/reference/plan.md` §Cross-spec context checklist, *Always read* becomes `system.md` plus every part listed under it by `check-artifact-size` invoked with `project-files` and no feature. `events.md` and `errors.md` are read with their parts under their existing conditions. The run names the parts it read.
- `framework/commands/plan.md`'s Scope Boundaries names the parts beside `specs/system.md`.
- In `framework/reference/plan.md`, §Create the plan step 3 and §Validation gate's *Plan does not conflict with `system.md`* cover the file and its parts. An `unreadable` part is reported in the gate prompt as unexamined, never as no conflict.
- The call is host prose, not a numbered step. The judgment it feeds is host work, which the exec walker no-ops (Exec-path scope), so a backticked dispatch would buy nothing.

**`/analyze`.**

- Step 15 judges *the plan does not contradict `system.md`*. Before that judgment it invokes `check-artifact-size` with `project-files` and no feature, and reads every part the result lists. Step 17's call is unchanged.
- Step 15 already carries `audit:ignore-promotion`. Under `ductus exec` it records `integrity-not-checked` as before (`framework/reference/analyze.md` §Unexamined targets (informational)).
- An `unreadable` part is not judged against. In `framework/reference/analyze.md`, §Plan consistency (blocking if plan exists), *How each runs*, records the judgment unexamined for that part, under `artifact-unreadable`. Step 17's skip of the same part already records that reason, so the part is counted once and no reason joins the closed set.
- `framework/commands/analyze.md`'s Scope Boundaries names the parts beside the three files.

### `/implement` and `/review` read by pointer (AC3)

`framework/reference/implement.md` step 4 says to follow conventions in `AGENTS.md` and `specs/system.md`. `framework/reference/review.md` says to cross-reference `specs/system.md`. Each adds two things: the parts those files' pointers name for the work in hand, and that the run says which parts it read. Neither command calls a primitive, because the pointer says when to read a part (spec, Resolved Questions, *Does a command read every part*).

### `/analyze` records the integrity findings (AC6, AC12, AC20)

- Step 17 records each `part-findings` entry as an Advisory finding, family `project-parts`, with its `message` and `path`. `framework/reference/analyze.md` §Artifact size (advisory) states them.
- `write-analysis` refuses only an empty family (`write_analysis.rs`, the finding validation), so no closed set changes.
- Under `ductus exec`, the tally's `check-artifact-size` arm (`runtime/src/interpreter/analyze_tally.rs`) adds `part-findings-count` to the advisory tier, as it adds `oversized`. That module is 022's runtime core: 022's spec says it owns `analyze_tally.rs`, in its signpost on 067.
- Advisory findings never hold `done` (spec 058).

### `prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)

- **Subjects.** The four project files come first, then each `measured` part from `project_parts`. Each part is segmented by its project file's grammar (041's data model, §`prune-project`, Segmentation), so a part of `events.md` carries catalog identifiers and joins the literal search. `files` stays the four project files. Each part gets the same record, with `part-of` naming its project file, in a new list, `part-files`. `check_args` accepts a part's path for `remove` and `replace`. The result carries `part-findings` for the preview to show.
- **Bound.** `part-files`, `part-findings`, `move-candidates` and a move's referrers each grow with the project. They join `entries` among the lists moved to `lists-file` past the read size, each with its count inline (041's data model §`prune-project`, `form`; `BE-PAYLOAD-001`).
- **Routing** (host; `framework/commands/prune.md`, *Project files*). An architecture entry goes to `system.md` or to the part it belongs in. A procedure leaving `AGENTS.md` goes to a part under `.ductus/docs/agents/`, a README beside the code, or a named skill. The host writes a new part before the apply, as it writes any home.
- **Candidates.** A preview lists `move-candidates`: each distinct markdown target inside the project that a project file or part links to, outside every parts directory, the spec root and `.ductus/`, with the files linking it. The host judges which target is a part (spec, Resolved Questions, *How do existing pointers move*).
- **Walk order.** The move proposals come before the entry walk, and the operator may end the run after them. The report then names each entry left unjudged as not examined, never as kept.
- **Search.** A preview passing `move: [{from, to}]` runs the literal search for `from`'s project-relative path. The search covers every file the project's repository tracks, the spec root included, except `from` itself (`tracked_project_paths`). That scope differs from the catalog search's, which leaves out the spec root (`search` in `prune_project.rs`); here the spec root holds 69 of the 81 referrers. A file that is, or lies under, a link leaving the project is named and not read, as the catalog search names one. A match needs a boundary before the path, so a longer path ending in it does not match. The result lists each referrer with its occurrence count. `to` must be a new `*.md` path directly under a parts directory.
- **Apply.** An apply passes the same `move` with the referrers the operator confirmed. A referrer the search did not find is refused. A found one left out is kept, so a decision record keeps the old name (§drift-prevention, the sweep rule). The apply then:
  - writes `to` create-only (`write_new_atomic`);
  - rewrites each confirmed referrer's occurrences to `to`'s path, keeping the prefix before each, so `../../docs/x.md` becomes `../../.ductus/docs/agents/x.md`;
  - rewrites the moved file's own relative links for its new directory, which §drift-prevention's *No dead references in live artifacts* requires in the same change;
  - removes `from`.

  Each file is written under the write guard (041's data model, *The write guard*). A referrer that changed since the read is refused and named.
- **Scope.** Writing referrers outside the four project files widens `--project`'s writes, as 075 widened them for homes, and `framework/commands/prune.md`'s Scope Boundaries says so.

### The migration entry announces the move (AC15)

- **Entry.** The procedure is `framework/migrations/project-file-links-into-docs.md`, registered in `framework/migrations.toml`. Its `introduced_in` is read from `version` at implement time, as the release that carries this spec.
- **Procedure.** It runs `prune-project` as a preview, through the CLI at the pointer path that Pre-flight brought to the pinned version. It does not use the session's MCP server, which in an upgrade can be an older binary (`framework/bootstrap/ductus.md`, the agent-runtimes note: MCP when registered, otherwise the CLI). It reports each `move-candidates` target under `docs/`, with the project file linking it and `/{project}:prune --project`, and writes nothing. With no such target it reports nothing. A result without `move-candidates` is reported as not checked, naming the command, never as no link.
- **One list.** The same list serves AC14 and AC15.
- **Installer change.** Its task therefore includes a scratch-adopter bootstrap run (`AGENTS.md`'s rule; **§Exercising an installer change without a push** in `docs/developing-ductus.md`).
- **Not an impact on 027.** The entry changes none of 027's contract, and no spec has declared 027 for adding one (`git grep '^cross-spec-impact:.*027' -- 'specs/*/spec.md'` finds none).

### A shared constitution's parts (AC16, AC17)

- **Measured.** `resolve-constitutions`' measured entry gives each `loaded` record `parts` and `parts-count`. Each part carries `path`, `bytes`, `over-limit` and `warning`, measured as the document is (`measure` and `over_limit_warning` in `runtime/src/primitives/resolve_constitutions.rs`). A part that cannot be opened carries no `bytes` and reads as not measured.
- **Skipped.** A `skipped` record carries no parts, and `/target` step 4 reports its parts as not measured along with it.
- **Unmeasured callers.** The unmeasured function, which `write-review`, `write-analysis` and `/ductus` use, reads no part. So neither record nor the imports block changes, and `/analyze` records no part finding (AC17).
- **Bound.** The parts lists are bounded as `check-artifact-size`'s are, because a checkout's directory sets their length, not the project's registry (`BE-PAYLOAD-001`). This changes 055's data model, **Bound**.
- **Departure.** 055's `BE-INPUT-004` departure reason adds the fixed `.ductus/docs/constitution/` beside `constitution.md`.

### Text surfaces (AC9, AC13, AC19)

- **Constitution.** Four edits:
  - §spec-phase's tree gains a `.ductus/docs/{file}/` line naming an index's parts;
  - §drift-prevention's routing sends a procedure to `.ductus/docs/agents/`;
  - *No dead references* names project files and their parts among the live artifacts;
  - the canonical-sources table gains a row for a project file split into an index and parts.

  The drafted additions measure 240 bytes (`wc -c` of the drafted strings, 2026-11-24). The headroom is 481 bytes: `framework/constitution.md` is 49,519 bytes at `b9d75c27` (`wc -c`) against the 50,000-byte ceiling in `scripts/lint-governing-size.sh`. The task re-measures, and cuts a restatement in the same commit if another change has taken the headroom.
- **Routing.** 075's routing-table row *A procedure for one kind of task* names `.ductus/docs/agents/` in place of `docs/`, and so does `framework/bootstrap/ductus.md`'s `ductus:memory-routing` line. The line is copied to `framework/bootstrap/govern.md`, which must stay byte-identical (audit Family 21). Every adopter receives it on its next `/ductus`.
- **Configuration text.** `README.md`'s `[artifacts]` entry and the commented schema in `framework/bootstrap/ductus.md` §Project Configuration say `read-size-bytes` limits parts too. This keeps 075's AC16 claim true.

### This repository's procedure file moves

`docs/developing-ductus.md` → `.ductus/docs/agents/developing-ductus.md`, in 087's last task, by running `/ductus:prune --project` here (the agent's recommendation, adopted by the operator, 2026-11-24).

- **Why in 087.** A real tree proves AC14's move before `done`, so a defect it finds lands in this spec. The tree holds 98 occurrences in 81 tracked files (`git grep -l` and `-c`, 2026-11-24): 69 specs, 7 scripts, 2 `runtime/src`, 1 workflow, `README.md` and `AGENTS.md`.
- **The run is the move.** The walk stops after the move proposal (*Walk order*, above). Judging the rest of `AGENTS.md`'s entries is not part of this task.
- **Records.** Every referrer is confirmed, this spec's own files included, so the citations of the procedure's headings follow the file. The substitution turns a line recording the move into `X → X`, and a recorded measurement into a claim about the new path. The next commit restores each such line to the old path in the `` ` → ` `` form, or rewords it into non-path form. Body path existence exempts that form, so it does not read such a line as a dead path (`framework/reference/analyze.md`).
- **Commit shape.** The rename and the referrer substitution land in one commit, apart from every other edit, so that each spec it touches reads it as a mechanical sweep (§drift-prevention). The sweep-targets edit and the record restoration follow in a commit of their own.
- **Sweep targets.** `AGENTS.md`'s sweep-targets list gains `.ductus/docs/` (spec, *Sweeping*).
- **`AGENTS.md` size.** The file names the path 4 times (`git grep -c`, 2026-11-24). Each rewrite adds 15 bytes and the sweep-targets entry adds 17, so the file grows by 77 bytes against its 50,000-byte ceiling (`scripts/lint-governing-size.sh`). It measured 49,427 bytes, a figure that includes another session's uncommitted edits. The task re-measures before committing.

### Every registry layout (AC18)

No primitive is added and no tool is renamed, so neither layout's allow list changes. Each layout takes the command changes through its generated copies: `.claude/commands/ductus/*.md` for `claude-style` and `.pi/prompts/ductus-*.md` for `pi`, which the hook regenerates. Each commit that ships a command or runtime surface records both layouts' impact in its message.

### No regression without parts (AC8)

"The same results" means four things:

- A feature-only call's JSON is byte-identical.
- A `project-files` call keeps every earlier key and value, with `parts` and `part-findings` empty and their counts 0. They are present, not absent, so an unstaged call's empty list never reads like a staged call's uncomputed one.
- `prune-project` keeps every earlier key and value, with its new lists empty.
- The read scope adds no file.

A fixture of four project files with no `.ductus/docs/` asserts each.

### Tests fail without their behavior (AC10)

Each test is proven to fail with its behavior removed (`QUAL-TEST-001`; **§Proving a test fails** in `docs/developing-ductus.md`). The task that adds a test records the proof in its commit.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/project_parts.rs` | Create | The part set, linkage and integrity findings |
| `runtime/src/primitives/mod.rs` | Modify | Declare the module |
| `runtime/src/primitives/check_corpus_links.rs` | Modify | Link scanner helpers made `pub(crate)` |
| `runtime/src/primitives/check_artifact_size.rs` | Modify | Parts, `part-findings`, `refused-count`, staged parts |
| `runtime/src/primitives/prune_project.rs` | Modify | Parts as subjects, `move-candidates`, `move` |
| `runtime/src/primitives/resolve_constitutions.rs` | Modify | A loaded constitution's parts |
| `runtime/src/schema/primitives.rs` | Modify | New fields, kinds and arguments |
| `runtime/src/schema/descriptions.rs` | Modify | The three primitives' descriptions |
| `runtime/src/main.rs` | Modify | Staged stderr for a refused part; `move` on the CLI |
| `runtime/src/interpreter/analyze_tally.rs` | Modify | Count `part-findings` as advisory |
| `runtime/tests/check_artifact_size_cli.rs` | Modify | A refused part's exit and stderr |
| `runtime/tests/size_hook.rs` | Create | The shipped hook refuses a grown part |
| `runtime/tests/result_sizes.rs` | Modify | Measured cases for the new lists |
| `framework/commands/plan.md` | Modify | Scope names the parts |
| `framework/reference/plan.md` | Modify | Checklist, create-the-plan and gate read parts |
| `framework/commands/analyze.md` | Modify | Scope; step 15 reads parts; step 17 records findings |
| `framework/reference/analyze.md` | Modify | Plan consistency and artifact-size sections |
| `framework/reference/implement.md` | Modify | Read the parts a pointer names |
| `framework/reference/review.md` | Modify | Read the parts a pointer names |
| `framework/commands/prune.md` | Modify | Parts, routing, the move, scope |
| `framework/commands/target.md` | Modify | Step 4 reports a constitution's parts |
| `framework/constitution.md` | Modify | Tree, routing, live artifacts, canonical source |
| `framework/bootstrap/ductus.md` | Modify | Memory-routing line; `read-size-bytes` comment |
| `framework/bootstrap/govern.md` | Modify | Byte-identical copy of `ductus.md` |
| `framework/migrations.toml` | Modify | Register the entry |
| `framework/migrations/project-file-links-into-docs.md` | Create | Report project-file links into `docs/` |
| `README.md` | Modify | `[artifacts]` limits parts; the moved file's path |
| `.ductus/docs/agents/developing-ductus.md` | Create | This repository's procedure part: `docs/developing-ductus.md` → `.ductus/docs/agents/developing-ductus.md` |
| `AGENTS.md` | Modify | Pointer to the part; sweep targets |
| `.github/workflows/framework-checks.yml` | Modify | The moved file's path |
| `runtime/src/parser/mod.rs` | Modify | The moved file's path |
| `runtime/src/primitives/check_owed_work.rs` | Modify | The moved file's path |
| `scripts/analyze/check.py` | Modify | The moved file's path |
| `scripts/analyze/combine.py` | Modify | The moved file's path |
| `scripts/mutation/exec-arguments.py` | Modify | The moved file's path |
| `scripts/mutation/run.py` | Modify | The moved file's path |
| `scripts/mutation/stderr-warnings-022.py` | Modify | The moved file's path |
| `scripts/stale-literals.sh` | Modify | The moved file's path |
| `scripts/tests/test-stale-literals.sh` | Modify | The moved file's path |
| `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md` | Modify | Parts in the result |
| `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md` | Modify | `/plan`'s read call |
| `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` | Modify | Signpost |
| `specs/041-task-pruning/data-model.md` | Modify | Parts as subjects; the move |
| `specs/041-task-pruning/spec.md` | Modify | Signpost |
| `specs/055-shared-constitution/data-model.md` | Modify | A loaded record's parts; bound |
| `specs/055-shared-constitution/spec.md` | Modify | Signpost; departure reason |
| `specs/050-constitution/spec.md` | Modify | Signpost |
| `specs/075-agents-md-holds-what-every-session-needs/spec.md` | Modify | Routing row; signpost |
| `specs/022-deterministic-runtime/spec.md` | Modify | Signpost |
| `specs/067-analyze-and-audit-checks/data-model.md` | Modify | `plan-consistency` judges the parts too |
| `specs/067-analyze-and-audit-checks/spec.md` | Modify | Signpost |

The 69 spec files that name the procedure file's old path are re-pointed by the move too.

## Data Model

This spec adds no `data-model.md`, because each shape belongs to its primitive's owner, as 075 decided (075's plan, *Data Model*). Each owner's data model changes in the task that changes the primitive:

- 063's `data-model.md` gains `parts`, `part-findings`, `refused-count`, `project-part` and its fix.
- 041's gains parts as subjects, `move-candidates` and `move`.
- 055's gains a loaded record's `parts` and their bound.

## Trade-offs

- **Rejected: a new primitive listing parts.** It needs seven registration sites (**§Adding or removing a runtime primitive** in `docs/developing-ductus.md`), an allow entry per registry layout, and an entry in two commands' `ToolSearch` lines. It would be a second tool serving a list `check-artifact-size` must produce anyway, for AC4–AC6.
- **Rejected: renumbering `/analyze` so step 17's call precedes step 15.** Up to 54 files cite those step numbers (`git grep -l -E 'step 1[5-9]|step 2[0-2]|steps? 1[5-9]'` over `framework/`, `runtime/`, `scripts/`, `docs/` and `specs/`, 2026-11-24; an upper bound, since the pattern also matches other commands' steps). Reading step 17's result in step 15 would reverse the walk's order. The chosen route costs one more call per interactive `/analyze`, whose result with no parts is 631 bytes (063's data model, **Size**).
- **Rejected: nesting parts inside each `project-files` record.** It makes the four-record list unbounded, and the staged CLI reads that list inline.
- **Rejected: the host moving the file and editing each referrer.** Here that is 98 occurrences in 81 files, and a mechanical rewrite is runtime-eligible (§runtime-boundary, *Eligibility criteria*).
- **Rejected: a search that also resolves links written relative to another directory.** The spec names a literal search (*Moving what exists*). Such a link — `docs/b.md` linking `x.md` — is not found, and the preview says the search was literal, as the catalog search does.
- **Rejected: a migration that parses links itself.** That is a second parse of the list `prune-project` already returns. Its cost is the preview's catalog search, run once per repository.
- **Rejected: scenarios on the owning specs, the route 075 took.** It reopens six `done` specs (022, 041, 050, 055, 063 and 075), each then owing a review and an analysis. It also moves the work out of the spec that requires it, against `AGENTS.md`'s shared-surface rule.
- **Limitations.**
  - Integrity findings are not computed under `staged`.
  - A nested file in a shared constitution's parts directory is not reported, since nothing here judges a checkout.

## Cross-spec impact

| Spec | What changes | Discharge |
| --- | --- | --- |
| 022 | Exec tally arm; link scanner made `pub(crate)` | Signpost in its `spec.md` |
| 041 | `prune-project` parts, candidates and move; `/prune --project` text | Signpost; data model §`prune-project` |
| 050 | Constitution tree, routing, live artifacts, canonical sources | Signpost |
| 055 | `resolve-constitutions` parts; `/target` step 4; departure reason | Signpost; data model; departure reason |
| 063 | `check-artifact-size` parts and findings; `/analyze` steps 15 and 17 | Signpost; data model; scenario sentence |
| 067 | `plan-consistency`'s judgment of `system.md` covers its parts | Signpost; data-model sentence |
| 075 | Routing-table row; memory-routing line | Row edit; signpost |

- **Signposts.** Each is a blockquote. A blockquote line induces no `dependencies:` edge (`derive-dependencies`), so no cycle forms. It adds no criterion or task, so each owner stays `done` (§spec-lifecycle).
- **Read scopes.** No spec's criterion or scenario states the read scopes 087 widens. A `git grep` of `specs/*/spec.md` and `specs/*/scenarios/*.md` for *Cross-spec context checklist*, *Always read* and *Plan does not contradict* finds only 071's kept-section marker, and that section stays.
- **067.** Its data model's `plan-consistency` entry states the judgment as "a contradiction of `system.md`", which this spec widens to the file and its parts. 067's AC7, the step that judges against `specs/system.md`, still holds.
- **Not impacted.** 027 (see the migration decision); 071; and 081, since `.ductus/` is already among the artifacts.
