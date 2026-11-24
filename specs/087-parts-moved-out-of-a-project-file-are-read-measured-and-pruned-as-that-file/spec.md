---
status: draft
dependencies: [041-task-pruning, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 075-agents-md-holds-what-every-session-needs, 081-reviews-and-checks-follow-what-changed]
cross-spec-impact: [041-task-pruning, 050-constitution, 063-oversized-artifacts-warn-with-a-fix]
next-criterion: 11
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

A project file is an **index** when it names parts. A part holds content that belongs to the project file by that file's §spec-phase definition, moved out so the index stays within one read. Where parts live, what makes a file a part, and which files may have them are open (Open Questions).

## The parts are the file

- **Reading.** Where `/plan`, `/analyze`, `/implement` or `/review` reads a project file, its read scope includes that file's parts. A check that compares against the project file names the parts it read and the parts it did not, and a part not read is reported as unexamined, never as no conflict.
- **Measuring.** `check-artifact-size`'s project-file subjects include each part, measured against the same threshold, staged and not. A part over the threshold is reported like its project file, and the shipped hook refuses a commit that grows one past it. [063](../063-oversized-artifacts-warn-with-a-fix/spec.md) owns the primitive, and this spec declares the impact.
- **Pruning.** `/prune --project` walks the parts as it walks the project file, and routes an entry to the part it belongs in. [041](../041-task-pruning/spec.md) owns the command, and this spec declares the impact.
- **Sweeping.** §drift-prevention's sweeps count a part as a live artifact wherever they count its project file.
- **Integrity.** An index entry naming a part that does not exist, and a part no index names, are each a finding, never clean.
- **The constitution** names the index and its parts in §spec-phase's tree, in §drift-prevention's routing and in its canonical-sources table. [050](../050-constitution/spec.md) owns that text, and this spec declares the impact.
- **Every agent in the registry** gets the same behavior: the change lands in command sources and the runtime, which each layout's generated copies carry.

## Edge Cases

- **A project file with no parts** behaves as it did before this spec.
- **A part that cannot be read** is reported as not examined, by the size check and by each command that reads it.
- **A link in a project file to a file that is not a part** — a spec, a README, a source file — is not a part and is not measured as one.

## Acceptance Criteria

- [ ] AC1: Over a fixture with an index `system.md` and one part, `/plan`'s and `/analyze`'s read scope includes the part, and the plan-against-`system.md` check reports the part as read.
- [ ] AC2: Over that fixture with the part unreadable, the plan-against-`system.md` check reports it as unexamined and does not report *no conflict*.
- [ ] AC3: `framework/reference/implement.md` and `framework/reference/review.md` state that following `system.md` includes its parts.
- [ ] AC4: `check-artifact-size` with `project-files` reports a part one byte over the threshold with its path, size, threshold and fix, and does not report a part at the threshold.
- [ ] AC5: With `staged`, `check-artifact-size` refuses a commit whose staged copy of a part is over the threshold and larger than its `HEAD` copy or has none, and passes one that shrinks an over-threshold part or leaves it unchanged.
- [ ] AC6: An index entry naming a missing part is reported as a finding, and so is a part no index names.
- [ ] AC7: `/prune --project` over a fixture index and part walks the part's entries, and proposes the part as the home of an architecture entry that belongs in it.
- [ ] AC8: A fixture project whose four project files name no parts gets the same `check-artifact-size`, `prune-project` and read-scope results as before this spec.
- [ ] AC9: The constitution's §spec-phase tree, §drift-prevention routing and canonical-sources table name a project file's index and its parts.
- [ ] AC10: Each test above fails when the behavior it names is removed.

## Applicable Rules

- `QUAL-CLAIM-001` — a part that was not read or could not be measured is reported as such, never as clean.
- `QUAL-TEST-001` — each test fails when its behavior is removed.

## Open Questions

- Where do parts live? Candidates: `.ductus/docs/{file}/` (`.ductus/docs/system/`, `.ductus/docs/agents/`), the owner's proposal, 2026-11-24, which cannot collide with a project's own `docs/` and falls inside 081's artifact set, so a part edit owes artifact checks and no review; one directory named by a `[paths]` key, the adopter's proposal, which costs a setting; or a directory under the spec root beside the file (`specs/system/`), also inside the artifact set. Unsettled for `.ductus/docs/`: whether any `/ductus` step removes files there. The lines of `framework/bootstrap/ductus.md` naming `enforce-manifest` run it over each agent's command directory and `.ductus/reference/` only; the file is 166,133 bytes and was not read whole.
- What makes a file a part: every file under the parts directory for that project file, or only the files the index's relative links name? The first needs no parse of the index and makes *a part no index names* a directory listing; the second lets a directory hold other files.
- Which files may have parts? The four project files are the report's subject. The owner also named the constitution and a spec's artifacts. The framework constitution installs whole, with `.ductus/reference/` as its on-demand content, so it has no pointers of this kind. A shared constitution does: adopting projects register ones that carry pointers to content moved out of them (owner, 2026-11-24). It lives in its own repository, so its parts do too, and nothing in the adopting project can move them; `resolve-constitutions` measures `constitution.md` alone, so those parts are unmeasured and no command is told to read them. 063 rules out sub-files for `data-model.md`, because the review's contract digest tracks it by exact name, and scenarios already elaborate a spec.
- May a part have parts of its own?
- Does a command read every part on every run, or only the parts the work touches? Reading all of them returns the read the split removed (90 KB in the report). Reading some needs a way to choose, and the check then reports the parts it did not read as unexamined.
- Is a procedure moved out of `AGENTS.md` a part? 075 routes it to `docs/`, a README or a skill, read when the task comes up, and it is not content of `AGENTS.md` by that file's definition. If it is a part, it moves to the parts directory and is measured; if not, `docs/` stays its home and only verbatim sections of a project file are parts.
- How do existing pointers move to the chosen location: a migration entry in the registry, `/prune --project` proposing each move, or by hand? In this repository `docs/developing-ductus.md` is named by 80 files across `framework/`, `scripts/`, `runtime/src`, `runtime/tests`, `.github/`, `docs/`, `README.md`, `AGENTS.md` and `specs/` (`grep -rl`, 2026-11-24), and `docs/` also holds adopter-facing documentation that is not a part.
- Does the staged size check gain a per-file waiver, and does it belong in this spec or as a scenario on 063? The report asks for one with a recorded reason, honoured by the staged check and reported as waived, never as passed. Today a project that must grow an over-limit file can only raise `read-size-bytes`, which moves the limit for every artifact and every primitive's result form, or pass `--no-verify`. 063's waivers live in a feature's `analysis.md`, and the staged check runs with no feature.
