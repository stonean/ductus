---
status: in-progress
dependencies: [041-task-pruning, 050-constitution, 055-shared-constitution, 063-oversized-artifacts-warn-with-a-fix, 075-agents-md-holds-what-every-session-needs, 081-reviews-and-checks-follow-what-changed]
cross-spec-impact: [022-deterministic-runtime, 041-task-pruning, 050-constitution, 055-shared-constitution, 063-oversized-artifacts-warn-with-a-fix, 067-analyze-and-audit-checks, 075-agents-md-holds-what-every-session-needs]
next-criterion: 22
---

# 087 — Parts moved out of a project file are read, measured and pruned as that file

A project file that has outgrown its size limit may keep an index and move its sections into **parts**. Every command that reads, checks, measures or routes the project file treats its parts as the file, so splitting one never turns a check into one that passes having read nothing.

## Motivation

- **The size limit pushed content out, and nothing followed it.** [075](../075-agents-md-holds-what-every-session-needs/spec.md) held `AGENTS.md`, `system.md`, `events.md` and `errors.md` to `[artifacts] read-size-bytes`, and the shipped pre-commit hook refused a commit that grew one past it. An adopting project's `system.md` was 90 KB. It moved each `##` section verbatim to a file of its own under `docs/system/`, leaving every heading in `system.md` with a one-line link to its part, and the file fell to 5.7 KB (the adopter's report, 2026-11-24; its tree is not reachable from here).
- **The commands bound their reads to the file by name.** `/plan`'s Scope Boundaries read "the target spec, `specs/system.md`, and cross-spec files", and its reference's *Always read* named `specs/system.md` alone. `/analyze`'s Scope Boundaries read `specs/system.md`, `specs/events.md` and `specs/errors.md`. A run that obeyed them never opened a part.
- **Two checks then passed having read no architecture.** The readiness check *Plan does not conflict with `system.md`* and `/analyze`'s *Plan does not contradict `specs/system.md`* compared a plan against pointers. That is a check that could not run reading as one that passed (§design-principles).
- **Two commands left the pointer to judgment.** `/implement` said to follow conventions in `specs/system.md`, and `/review` to cross-reference it; neither said to follow a link out of it.
- **Parts were unmeasured and unrouted.** `check-artifact-size`'s project-file subjects were a fixed four paths (`project_subjects` in `runtime/src/primitives/check_artifact_size.rs`), so a part could grow without bound. `/prune --project` routed architecture to `system.md` itself, never to a part of it.
- **Moved content had no one home.** `/prune --project` sent a procedure to `docs/`, a directory a project also uses for its own documentation, and [081](../081-reviews-and-checks-follow-what-changed/spec.md) counts every path outside the spec root, `.ductus/`, `AGENTS.md` and `CLAUDE.md` as code. A part under `docs/` was therefore code: editing it owed a review, and no artifact check read it.

## Parts

A **part** is a markdown file directly under a project file's parts directory, and a project file with at least one part is an **index**:

| File | Parts directory |
| --- | --- |
| `AGENTS.md` | `.ductus/docs/agents/` |
| `system.md` | `.ductus/docs/system/` |
| `events.md` | `.ductus/docs/events/` |
| `errors.md` | `.ductus/docs/errors/` |
| a registered shared constitution | `.ductus/docs/constitution/` under its registry entry's `path`, beside the `constitution.md` read from there |

- A part of `system.md`, `events.md` or `errors.md` holds content that belongs to that file by its §spec-phase definition, moved out so the index stays within one read.
- A part of `AGENTS.md` holds a procedure an agent reads when its task comes up, which 075 routed to `docs/`. A README beside the code and a skill stay as homes for one, and documentation written for people stays in `docs/`.
- The index names each part with a relative link and says when to read it. A part is a part whether or not the index links it.
- Parts do not nest. A part over the limit is split into sibling parts.
- A spec's artifacts and the framework constitution have no parts.

## The parts are the file

- **Reading.** `/plan` and `/analyze`, which each check a plan against a project file, read that file and every part of it. That check is the host's judgment (`framework/reference/analyze.md`, *How each runs*; `framework/reference/plan.md`, *Create the plan*), so the runtime supplies the list of parts with each one's state, and the command sources direct the run to read every part listed and to name those it read. `/implement` and `/review` read the index and the parts its pointers name for the work in hand, and say which they read. A part a check could not read is reported as unexamined, never as no conflict.
- **Measuring.** `check-artifact-size`'s project-file subjects include each part, measured against the same threshold, staged and not. A part over the threshold is reported like its project file, with splitting it into sibling parts as its fix, and the shipped hook refuses a commit that grows one past it. [063](../063-oversized-artifacts-warn-with-a-fix/spec.md) owns the primitive, and this spec declares the impact.
- **Pruning.** `/prune --project` walks the parts as it walks the project file, and routes an entry to the part it belongs in. It routes a procedure leaving `AGENTS.md` to `.ductus/docs/agents/`. [041](../041-task-pruning/spec.md) owns the command and 075 the routing table, and this spec declares both impacts.
- **Moving what exists.** For a project-file pointer whose target is a markdown file outside the parts directory, `/prune --project` proposes moving the target into it and re-pointing every referrer its literal search finds, shows that search, and writes nothing the operator has not confirmed. One migration entry reports, at an adopter's next `/ductus`, each project-file link to a markdown file under `docs/`, names `/prune --project`, and moves nothing.
- **Sweeping.** §drift-prevention's sweeps count a part as a live artifact wherever they count its project file.
- **Integrity.** Each of these is reported, never clean: an index link into the parts directory whose file does not exist; a part no index links; a markdown file in a subdirectory of a parts directory; a directory under `.ductus/docs/` that is no project file's parts directory; and a markdown file directly under `.ductus/docs/`. `check-artifact-size`'s project-files result lists them, `/analyze` records each as an advisory finding, and `/prune --project`'s preview shows them. None holds a spec's `done` or stops a commit.
- **Routing text.** The `ductus:memory-routing` block in `framework/bootstrap/ductus.md`, the constitution's §drift-prevention routing and `/prune`'s routing each name `.ductus/docs/agents/` where they named `docs/` as a procedure's home.
- **A shared constitution's parts** are measured by `resolve-constitutions` and reported by `/target` beside the document's own size, one over the limit named with its owning repository as the fix. They are never an `/analyze` finding, and no command here writes them. [055](../055-shared-constitution/spec.md) owns that surface, and this spec declares the impact.
- **The constitution** names the index and its parts in §spec-phase's tree, in §drift-prevention's routing and in its canonical-sources table. [050](../050-constitution/spec.md) owns that text, and this spec declares the impact.
- **Every agent in the registry** gets the same behavior: the change lands in command sources and the runtime, which each layout's generated copies carry.

## Edge Cases

- **A project file with no parts** behaves as it did before this spec.
- **A part that cannot be read** is reported as not examined, by the size check and by each command that reads it.
- **A link in a project file to a file that is not a part** — a spec, a README, a source file — is not a part and is not measured as one.
- **A file in a parts directory that is not markdown** is not a part: it is neither measured nor reported.
- **An empty or missing parts directory** means the project file has no parts.
- **A parts directory whose project file is absent** — `.ductus/docs/events/` with no `events.md` — has its parts measured, each reported as unlinked, beside the file's own `absent` state.
- **An index that is itself over the limit** is reported as it was before this spec, whatever its parts measure.
- **A part exactly at the limit** passes; one byte over is reported.
- **A commit that deletes a part** passes the hook, and the index link it leaves is the missing-part finding.
- **A part linked from two project files** is a part of the file whose directory holds it; the other link is an ordinary link.
- **A shared constitution that is not checked out** has its parts reported as not measured, never as under the limit.
- **A pinned project file** is still an index when its directory holds parts: pinning governs what `/ductus` writes, not what is measured.

## Cost

> **Signpost:** [089 — The pipeline's own cost is stated and bounded](../089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md) asks this spec to state what a run of what it adds costs, in a measurable unit, and what it retires.

Measured at `4e5ee251` with the release build of its `runtime/`, over this repository, on a Darwin arm64 machine (Apple M4 Max). A result's size is the bytes `wc -c` counts on stdout, compact JSON and a newline. A wall time is five runs after a warm-up, by `time.perf_counter` around the call, with a warm file cache.

- **`check-artifact-size` with `project-files` and no feature**, the call `/analyze` step 15 and `/plan`'s checklist each add once a run: a 527-byte result, 70 bytes of it 087's `parts` and `part-findings` with their counts, in 0.009 s each run. This repository holds no part at that commit, so both lists are empty.
- **The parts `/plan` and `/analyze` then read**, each part listed under `system.md`, and under their conditions `events.md` and `errors.md`, read whole: 0 bytes here. Each part is held to `[artifacts] read-size-bytes`, 50,000 bytes by default, so a run reads at most that per part. In the adopter's report the parts of one `system.md` were about 90 KB (Resolved Questions, *Does a command read every part*). `/implement` and `/review` read only the parts a pointer names for the work in hand.
- **`prune-project`'s preview**, which `/prune --project` and the migration entry make, the entry once per repository: a 33,657-byte result in 0.011 to 0.012 s, 438 bytes of it 087's `move-candidates` (four here), `part-files`, `part-findings` and `parts-unlisted` with their counts.
- **The move** `docs/developing-ductus.md` → `.ductus/docs/agents/developing-ductus.md`:
  - its preview, a 40,764-byte result in 0.100 to 0.109 s, whose literal search read 1,478 tracked files and found 132 occurrences in 92;
  - its apply, confirming all 92, a 38,291-byte result in 0.568 to 0.637 s, writing 93 files, timed once in each of three `git clone --local` copies at that commit, since task 12 makes the move in this tree.
- **`resolve-constitutions`**, which `/target` step 4 reports: 61 bytes in 0.009 s, since this repository registers no shared constitution. Each part of a registered one adds one record to `parts`.
- **Descriptions**, loaded when a Claude Code session fetches the tool and by Pi's bridge at load. 087's five commits to `runtime/src/schema/descriptions.rs` grew `check-artifact-size`'s by 1,502 characters, `prune-project`'s by 2,366 and `resolve-constitutions`' by 735, each commit's change the literal's length less its length at the parent. With their input schemas the three load 5,334, 8,616 and 2,688 characters at that commit, by `scripts/measure-result-sizes.sh`.
- **Bound.** Over that script's large fixture corpus at that commit, with unlinked parts, or move candidates and referrers, past the read size, `check-artifact-size` with `project-files` returns 75,094 characters unbounded and 564 at the default read size, `prune-project` 78,095 and 720, and `prune-project` with `move` 270,984 and 860, the rest in `lists-file`.

**What it retires.** Nothing. 087 widens what existing commands read, measure and route, and none of the result fields, findings, the move or the migration entry it adds stands in for a primitive, check, step or record field that existed. A procedure's home moves from `docs/` to `.ductus/docs/agents/`, which changes where it lives and retires nothing.

## Acceptance Criteria

- [ ] AC1: Over a fixture with an index `system.md` and one part, a runtime primitive's result lists the part under its project file with its state, and `framework/commands/plan.md`, `framework/reference/plan.md` and `framework/commands/analyze.md` direct the run to read every part that result lists under the file before judging the plan against the file, and to name the parts it read.
- [ ] AC2: Over that fixture with the part unreadable, the result gives the part a state distinct from a readable one, and `framework/reference/analyze.md` has the plan-against-`system.md` judgment recorded as unexamined for that part, never as no contradiction.
- [ ] AC3: `framework/reference/implement.md` and `framework/reference/review.md` state that following `system.md` includes the parts its pointers name for the work in hand, and that the run says which parts it read.
- [ ] AC4: `check-artifact-size` with `project-files` reports a part one byte over the threshold with its path, size, threshold and fix, and does not report a part at the threshold.
- [ ] AC5: With `staged`, `check-artifact-size` refuses a commit whose staged copy of a part is over the threshold and larger than its `HEAD` copy or has none, and passes one that shrinks an over-threshold part or leaves it unchanged.
- [ ] AC6: `check-artifact-size` with `project-files` lists an index link naming a missing part, and a part no index links, and `/analyze` records each as an advisory finding that does not hold `done`.
- [ ] AC7: `/prune --project` over a fixture index and part walks the part's entries, and proposes the part as the home of an architecture entry that belongs in it.
- [ ] AC8: A fixture project whose four project files name no parts gets the same `check-artifact-size`, `prune-project` and read-scope results as before this spec.
- [ ] AC9: The constitution's §spec-phase tree, §drift-prevention routing and canonical-sources table name a project file's index and its parts.
- [ ] AC10: Each test a criterion in this section names fails when the behavior it names is removed.
- [ ] AC11: A markdown file directly under each of `.ductus/docs/agents/`, `.ductus/docs/system/`, `.ductus/docs/events/` and `.ductus/docs/errors/` is measured as a part of its project file whether or not the index links it, and a file there that is not markdown is neither measured nor reported.
- [ ] AC12: A markdown file in a subdirectory of a parts directory, and a directory under `.ductus/docs/` that is no project file's parts directory, are each listed by `check-artifact-size` with `project-files` and recorded by `/analyze` as an advisory finding.
- [ ] AC20: A markdown file directly under `.ductus/docs/` is listed by `check-artifact-size` with `project-files` and recorded by `/analyze` as an advisory finding.
- [ ] AC13: `/prune --project` proposes `.ductus/docs/agents/` as the home of a procedure leaving `AGENTS.md`, and the routing table in `specs/075-agents-md-holds-what-every-session-needs/spec.md` and the constitution's §drift-prevention routing name that directory where they named `docs/`.
- [ ] AC19: The `ductus:memory-routing` block in `framework/bootstrap/ductus.md` names `.ductus/docs/agents/` as a procedure's home where it named `docs/`.
- [ ] AC14: Over a fixture `AGENTS.md` whose pointer names a markdown file under `docs/` that a second file also links, `/prune --project` proposes moving it into `.ductus/docs/agents/`, shows the search that found the second file, and on confirmation moves the file and re-points both links; declined, it writes nothing.
- [ ] AC15: A migration entry reports each project-file link to a markdown file under `docs/` and names `/prune --project`, moves and rewrites nothing, and reports nothing for a project with no such link.
- [ ] AC16: `/target`'s report of a loaded shared constitution gives each part under its checkout's `.ductus/docs/constitution/` with its bytes, names a fixture part one byte over the limit with its owning repository as the fix, reports the parts of one not checked out as not measured, and the run continues.
- [ ] AC17: `/analyze` records no finding for a shared constitution's part.
- [ ] AC18: The change that ships the command and runtime surfaces records each registry layout's impact (`claude-style`, `pi`).
- [ ] AC21: `/prune --project` makes its move proposals before the entry walk, and a run the operator ends after them reports each entry it did not judge as not examined, never as kept.

## Applicable Rules

- `QUAL-CLAIM-001` — a part that was not read or could not be measured is reported as such, never as clean.
- `QUAL-TEST-001` — each test fails when its behavior is removed.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Where do parts live?** Under `.ductus/docs/{file}/`, one directory per project file: `.ductus/docs/system/`, `.ductus/docs/agents/` (the owner's proposal, recommended by the agent and chosen by the owner, 2026-11-24). It costs no setting, cannot collide with a project's own `docs/`, and is inside 081's artifact set, so a part edit owes artifact checks and no review. `/ductus` leaves it alone: of the 1,069 lines of `framework/bootstrap/ductus.md`, those that name both `.ductus` and a removal cover only the top-level `*.md` files of `.ductus/reference/` and the runtime pointer, and the shipped `.gitignore` template ignores `session.toml`, `sessions/`, `bin/` and `results/` under `.ductus/` and nothing else. Of the 13 files under `framework/migrations/` that mention `.ductus/`, the lines naming both it and a removal cover `.ductus/scripts/`, the legacy session file and the emptied `.govern/`, never `.ductus/docs/`; the files were searched by line, not read whole. What this gives up: a hidden directory is less visible to someone browsing the repository. Rejected: a `[paths]` key, which costs a setting and leaves a part under `docs/` as code unless 081's set reads the key, the configurable list 081 rejected; and a directory under the spec root, which leaves `AGENTS.md`, at the project root, without a home beside it.
- **What makes a file a part?** Each markdown file under the project file's directory, linked or not (the agent's recommendation, adopted by the operator, 2026-11-24). The part set is a directory listing, so a part whose pointer was deleted is still read and measured while it is reported as unnamed. A file there that is not markdown is not a part and is not measured. Rejected: only the files the index links, under which deleting a pointer takes its part out of every check but the unnamed-part finding. Both read the index's links, since a pointer to a missing part is a finding either way.
- **Which files may have parts?** The four project files, and each registered shared constitution (the agent's recommendation, adopted by the operator, 2026-11-24). Adopting projects register shared constitutions that carry pointers to content moved out of them (owner, 2026-11-24). One lives in its own repository, so its parts sit in its checkout at `.ductus/docs/constitution/`; `resolve-constitutions` measured `constitution.md` alone, so those parts were unmeasured. They are measured and reported where its size already is, by `/target`, and never as an `/analyze` finding, since nothing in the adopting project can fix one, as 075 decided for its size. [055](../055-shared-constitution/spec.md) owns that surface, and this spec declares the impact. The framework constitution has no parts: it installs whole, with `.ductus/reference/` as its on-demand content. A spec's artifacts have none either: 063 names their fixes (split, trim, promote a scenario) and bars sub-files for `data-model.md`, whose name the review's contract digest tracks. Not computed: how many readers of a spec's files parts would reach, so that option was not costed.
- **May a part have parts of its own?** No (the agent's recommendation, adopted by the operator, 2026-11-24). Parts are the markdown files directly under the project file's directory, and a part over the limit is split into sibling parts, each named by the index. A markdown file in a subdirectory there is a finding, never silently skipped. Rejected: nesting, under which every reader, the size check, `/prune` and the integrity findings walk a tree, and a command follows two or more pointers to reach content.
- **Does a command read every part, or only some?** `/plan` and `/analyze`, which each check a plan against the project file, read the index and every part; `/implement` and `/review` read the index and the parts its pointers name for the work in hand, and say which they read (the agent's recommendation, adopted by the operator, 2026-11-24). The limit exists so that each file is read whole in one call (063), and every part is within it whichever way a command reads. In the adopter's report the index was 5.7 KB and the parts about 90 KB, roughly 22,500 tokens at four bytes a token. Rejected: every command reading every part, which charges that read to each `/implement` session and each `/review`; and every command reading by pointer, under which the two consistency checks report unexamined parts on most runs.
- **Is a procedure moved out of `AGENTS.md` a part?** Yes, under `.ductus/docs/agents/` (the owner's proposal, recommended by the agent and chosen by the owner, 2026-11-24). A procedure an agent reads, leaving `AGENTS.md` behind a pointer, is a part: measured against the limit, inside the artifact set, and read when its pointer says. 075's routing sent one to `docs/`, where it was unmeasured, counted as code, and shared a directory with the project's own documentation; that row changes to this directory, a README beside the code and a skill stay as homes, and documentation written for people stays in `docs/`. 075 owns that table, and this spec declares the impact. Rejected: leaving `docs/` as the home, which changes nothing in 075 and leaves those three faults.
- **How do existing pointers move?** `/prune --project` moves them, and one migration entry announces it (the agent's recommendation, adopted by the operator, 2026-11-24). Which file under `docs/` is a part and which is documentation for people is a judgment, so the move belongs to the walk that already judges each entry and confirms each write: it proposes moving a pointer's target into the parts directory and re-pointing every referrer its literal search finds, and shows that search. The migration entry runs once at an adopter's next `/ductus`, reports each project-file link into `docs/`, names the command, and moves nothing, so an adopter who has already split a file, and is under the limit, is told without having to remember. A shared constitution's parts move by hand in its own repository, since no command here writes there. In this repository the procedure file this spec moves was named, at its old path under `docs/`, by 80 files across `framework/`, `scripts/`, `runtime/src`, `runtime/tests`, `.github/`, `docs/`, `README.md`, `AGENTS.md` and `specs/` (`grep -rl`, 2026-11-24). Rejected: `/prune --project` alone, which reaches such an adopter only through release notes; and a migration that performs the moves, which puts the move and its referrer sweep in two places.
- **Does the staged size check gain a per-file waiver?** No (the agent's recommendation, adopted by the operator, 2026-11-24). The adopter's report asked for one, with a recorded reason, for a project that must grow an over-limit file before this spec lands. A waiver would ship no earlier than this spec, and once it ships an over-limit file has an exit, which is to split it into parts. 075 chose refusing growth and rejected an exit that depends on someone choosing to take it, and 063 declined an off switch as permanent configuration surface. Until then such a project raises `read-size-bytes` or keeps the split it has. Rejected: a waiver here, which costs a configuration table and partly reverses 075's refusal; and one as a scenario on 063, which reopens it.
- **Who reports the integrity findings, and do they hold `done`?** `check-artifact-size`'s project-files result lists them, `/analyze` records each as an advisory finding, and `/prune --project`'s preview shows them; none holds `done` or stops a commit (the agent's recommendation, adopted by the operator, 2026-11-24). The fault is in a project file, not in the spec being analyzed, as 075 decided for an over-limit project file. Rejected: blocking, under which one project-level fault holds every open spec; and a hook refusal for a staged link to a missing part, a second refusal beyond size growth.
- **Is another directory under `.ductus/docs/` a finding?** Yes, advisory, naming the directory (the agent's recommendation, adopted by the operator, 2026-11-24). `.ductus/docs/` holds parts alone, so a misnamed parts directory such as `.ductus/docs/sytem/` cannot hide unmeasured content. Rejected: ignoring it, which leaves that directory unmeasured with no signal.
- **Is a markdown file directly under `.ductus/docs/` a finding?** Yes, advisory, naming the file (the agent's recommendation, adopted by the operator, 2026-11-24). It is no part, so it is otherwise neither measured nor reported, and `.ductus/docs/` holds parts alone, the reason a stray directory there is a finding. The listing of `.ductus/docs/` that finds a stray directory finds it too. Rejected: leaving it unreported, as this spec first read, under which such a file is unmeasured with no signal.
