---
status: clarified
dependencies: [063-oversized-artifacts-warn-with-a-fix, 072-governing-documents-stay-within-one-read]
cross-spec-impact: [041-task-pruning, 050-constitution, 055-shared-constitution, 063-oversized-artifacts-warn-with-a-fix]
next-criterion: 17
---

# 075 — AGENTS.md holds what every session needs

An adopter's project-level files — `AGENTS.md`, `specs/system.md`, `specs/events.md` and `specs/errors.md` — each hold only what their readers need. `AGENTS.md` is defined by when its content is needed as well as by who it is true for: it holds what an agent must know before acting in any session of the project, and everything else goes to an artifact the agent reads when the work it governs happens. A size check, shipped to adopters, measures each file, and `/prune` reduces one that has outgrown its definition.

## Motivation

- **Routing asked only who a learning was true for.** The constitution's §drift-prevention *Shared knowledge stays in git* sent a project-only learning to "its `AGENTS.md` or a rule file". The `ductus:memory-routing` block `/ductus` wrote into every adopter's `AGENTS.md` (`framework/bootstrap/ductus.md`, Memory routing) said "this project alone, this file or a rule file". The template header (`framework/templates/project/agents.md`) told the agent to append a project-only rule to its matching section. No surface named another project artifact, and none removed an entry.
- **The file loaded whole in every session.** `claude-style` agents loaded it through `CLAUDE.md`'s `@AGENTS.md` import; Antigravity, OpenCode and Pi read it natively. Claude Code's [memory documentation](https://code.claude.com/docs/en/memory), read 2026-10-29, said an import does not reduce context cost and recommended under 200 lines per file. It also said a multi-step procedure, or a rule for one part of the codebase, should move to something that loads on demand.
- **The fix stopped at this repository.** [072](../072-governing-documents-stay-within-one-read/spec.md) gave ductus's own `AGENTS.md` four exits (copies of constitution rules, stories of how a lesson was learned, lessons a check already enforces, lessons no longer true) and a 50,000-byte ceiling. Its plan called that ceiling a maintainer lint, never shipped, and left an adopter ceiling to a spec of its own, naming an extension of [063](../063-oversized-artifacts-warn-with-a-fix/spec.md)'s `check-artifact-size` as the route.
- **The other project files had no definition and no exit.** `check-artifact-size` measured a spec's own artifacts only. No command wrote `specs/system.md`, `specs/events.md` or `specs/errors.md` after adoption seeded them, though `/plan` read `system.md` on every run and `/analyze` and `/review` checked against all three. `/prune` moved a plan section's project-only knowledge into `AGENTS.md` (`framework/commands/prune.md`, step 8), so the command that reduced spec artifacts grew the project's largest file.
- **One adopter's file showed the result.** Measured 2026-10-29, it was 716 lines and 131,607 bytes, roughly 33,000 tokens at four bytes a token. Its Gotchas and Boundaries sections held 74% of the bytes. Each of its 70 top-level entries and sections was read and assigned one destination. Mixed entries took their main one, so the split is a judgment, not a mechanical classification. Bytes were counted per `##` section and per top-level bullet, nested children included:

| Destination | Bytes | Share |
| --- | --- | --- |
| A procedure for one kind of task | 38,038 | 28.9% |
| Architecture, or one feature's decision | 36,490 | 27.7% |
| ductus behavior true for every adopter | 32,399 | 24.6% |
| Needed in every session, this project only | 14,766 | 11.2% |
| A code comment, or a test that already existed | 5,010 | 3.8% |
| The organization's shared constitution | 4,262 | 3.2% |
| Template residue and headings | 643 | 0.5% |

The every-session share still held dated accounts of how lessons were learned, which were not measured separately.

## What AGENTS.md holds

An entry belongs in `AGENTS.md` when all four hold:

- it is true for this project alone — the population question, unchanged;
- an agent needs it before acting in any session, whatever the task;
- no artifact the agent reads while doing the work it governs would tell it;
- no check, test or hook already reports the failure it warns about.

The second and third questions choose among this project's artifacts only. They never move a learning between population tiers: a rule true for every adopter goes upstream however rarely it is needed, as `050-constitution`'s scenario `knowledge-routes-by-population-not-by-kind` requires. 072's four exits apply to an adopter's entries as they do to this repository's.

## What the other project files hold

Each records what is true now, never how it came to be, and is as short as the constitution's §text-first-artifacts requires. The constitution's §spec-phase directory tree states each definition in one line, and the template headers and `/prune`'s reference point at it:

- **`specs/system.md`** — the architecture as it stands: system-wide constraints, configuration, lifecycle, shared infrastructure and module patterns. A decision that concerns one feature belongs in that feature's spec.
- **`specs/events.md`** — the envelope and one entry per event the code publishes or consumes.
- **`specs/errors.md`** — the error-response conventions and the codes the code returns.

## Where everything else goes

| Content | Home | Read when |
| --- | --- | --- |
| Architecture, system-wide constraints, deployment shape | `specs/system.md`, `specs/events.md`, `specs/errors.md` | `/plan`, `/implement`, `/review`, `/analyze` |
| A decision about one feature | that spec's Resolved Questions, or a scenario | when the spec is worked |
| A procedure for one kind of task | `docs/`, a README beside the code, or a skill | when the task comes up |
| A trap in one file or module | a comment at the site, or a test | when the file is opened |
| ductus behavior true for every adopter | upstream, reported to ductus | — |
| A rule true across the organization's projects | the shared constitution registered under `[constitutions.*]` | every session |
| How a lesson was learned | the commit that added it | never |

An entry that leaves `AGENTS.md` for an on-demand home may leave one pointer line naming the home and when to read it. A pointer counts toward the file's size like any other line.

## Surfaces

- **The constitution** states the `AGENTS.md` test and the homes in its routing, and its §spec-phase tree defines the other three files; `050-constitution` owns that text, and this spec declares the impact. Its scenario `artifacts-are-as-short-as-their-required-content` says existing artifacts are not swept, which stops holding for the four project files once `/prune` walks them.
- **The `ductus:memory-routing` block** carries them, naming each home and when to read it — before changing architecture, configuration or deployment, `specs/system.md` — so every existing adopter receives them on its next `/ductus` run and a session that runs no pipeline command still meets them.
- **The templates' headers** point at those definitions — `AGENTS.md`'s at the constitution's test, the other three at its §spec-phase tree — and reach new adoptions only, since `AGENTS.md` installs with the `skip` strategy and the other three with `create`. The `AGENTS.md` template's Project Structure section drops the copy of the spec directory tree that the constitution's §spec-phase states.
- **`/prune`** reduces the four files; `041-task-pruning` owns it, and this spec declares the impact.
- **`check-artifact-size`** measures them; `063-oversized-artifacts-warn-with-a-fix` owns it, and this spec declares the impact.
- **`/target`'s report of loaded shared constitutions** carries each one's size; `055-shared-constitution` owns it, and this spec declares the impact.
- **Every agent in the registry** — `claude` and `auggie` (`claude-style`), `antigravity`, `opencode` and `pi` — gets the same definitions and homes, and the size check applies whichever agents a project selected.

## The size check

A runtime check measures each of the four files' bytes on disk against `[artifacts] read-size-bytes`, 50,000 bytes by default, and reports a file over it by name, size, limit and fix. The fix is `/prune --project`. A missing or unreadable file gets a result of its own, never a pass.

- **The shipped pre-commit hook** refuses a commit that grows a file already over its limit, or takes one past it: the staged copy is over the limit and larger than the copy at `HEAD`, or has no copy at `HEAD`. A commit that leaves an over-limit file the same size or smaller passes, so an oversized file shrinks over time and never blocks unrelated work.
- **`/analyze`** reports an over-limit file as an advisory finding naming `/prune --project`.
- **`/target`** reports each loaded shared constitution's bytes on disk beside its alias, and names one over `read-size-bytes` as over its limit, with the fix in the repository that owns it. That report never blocks and is never an `/analyze` finding, since nothing in the project can fix, route or discard it.

## `/prune` reduces the project files

`/prune --project` walks the four project files, needs no session target, and leaves `--all`'s walk over every spec unchanged. It walks a project file's entries, proposes each one's home by the file's definition and the table under *Where everything else goes*, and moves an entry only after the operator confirms it. A moved entry's content lands in its home, with a pointer line left in `AGENTS.md` when the home is read on demand. An entry removed as no longer true carries its reason in the run's report. For an `events.md` or `errors.md` entry, `/prune` searches the project's source for the entry's event type or error code and shows that search with any proposal to remove it; a search with no hits is reported as *no literal occurrence found*, never as proof the code no longer has it, since an identifier built at runtime does not appear literally. Reading source to run that search is the one widening of `/prune`'s scope this spec makes.

An `AGENTS.md` entry true for every adopter is checked against the framework the project has installed — `.ductus/constitution.md`, `.ductus/reference/` and the runtime. One the installed framework already states becomes a one-line pointer to that section; one no longer true at the installed version is deleted with its reason; one still true stays, trimmed to one line, and `/prune` drafts an upstream report for the operator to file with the project's own names and details removed. `/prune` files nothing upstream itself. When `/prune` moves a plan section's knowledge, it routes it by the same definitions rather than to `AGENTS.md` by default. The project-file walk ships no later than the size check, so an adopter over a limit when the check lands is offered `/prune --project` rather than met with only a failure.

## Edge Cases

- **A pinned `AGENTS.md`** still gets the managed block, as today, and the size check applies to it like any other: pinning governs what `/ductus` writes, not what the project's file may hold.
- **A file exactly at its limit** passes; the limit is a maximum.
- **A commit that deletes a project file** passes the hook: there is no staged copy to be over the limit.
- **A project file that does not exist** is reported as absent by the size check and by `/prune --project`, never as clean, so a walk over nothing reads differently from a walk that found nothing to move.
- **An entry changed between `/prune --project`'s preview and its move** is not moved; the walk previews it again, as `prune-plan` does for a plan section whose digest changed.
- **A registered shared constitution that is not checked out** is already reported as skipped by `/target`; its size is reported as not measured, never as under the limit.
- **A project with no `specs/system.md`** — `/prune` names the home and creates it from the template only on confirmation.
- **An entry whose home is in another repository**, such as a sibling service's spec — `/prune` names it and writes nothing outside the project.
- **An organization with no registered shared constitution** — its rules stay in `AGENTS.md` and count toward its size, as `050-constitution` already states for the middle tier.
- **`specs/inbox.md`** is not a subject: `/groom` drains it.

## Acceptance Criteria

- [ ] AC1: The constitution's §drift-prevention *Shared knowledge stays in git* states the four-part test under *What AGENTS.md holds* in this spec and names the homes under *Where everything else goes*, and says the added questions never move a learning between population tiers.
- [ ] AC2: The memory-routing line in `framework/bootstrap/ductus.md` names `AGENTS.md` only for what every session needs, and names each other project home with when to read it.
- [ ] AC3: The header of `framework/templates/project/agents.md` points at the constitution's four-part test without restating it, and no longer tells the agent to append a project-only rule to its matching section.
- [ ] AC4: `framework/templates/project/agents.md` carries no copy of the spec directory tree the constitution's §spec-phase states.
- [ ] AC5: A shipped check reports each of `AGENTS.md`, `specs/system.md`, `specs/events.md` and `specs/errors.md` over its limit with the file, its size, the limit and `/prune --project` as the fix; a fixture one byte over fails, a fixture at the limit passes, and a missing or unreadable file gets a result distinct from both.
- [ ] AC6: Run over fixture project files holding one entry for each row of the table under *Where everything else goes*, `/prune --project` proposes that row's home for each, writes nothing an operator has not confirmed, and lands each confirmed entry's content in its home.
- [ ] AC7: The release that ships the size check also ships `/prune`'s project-file walk, and the check's message names `/prune --project`.
- [ ] AC8: Each registry layout's impact (`claude-style`, `antigravity`, `opencode`, `pi`) is recorded in the change that ships the block, the templates and the size check.
- [ ] AC9: The headers of `framework/templates/project/system.md`, `framework/templates/project/events.md` and `framework/templates/project/errors.md` each point at the constitution's §spec-phase directory tree for what that file holds, and restate none of it.
- [ ] AC10: `framework/commands/prune.md` routes a plan section's project-only knowledge by the definitions in this spec, and no longer names `AGENTS.md` as its default home.
- [ ] AC11: Run over a fixture `specs/events.md` holding one entry whose event type occurs in the fixture's source and one whose type does not, `/prune --project` proposes removing only the second, shows the search behind the proposal, words its result as *no literal occurrence found*, and removes nothing the operator has not confirmed.
- [ ] AC12: Run over a fixture `AGENTS.md` holding three entries true for every adopter — one the fixture's installed constitution already states, one its installed framework no longer exhibits, one still true — `/prune --project` proposes a pointer for the first, a deletion with its reason for the second, and a one-line entry plus a drafted upstream report for the third, and sends nothing outside the project.
- [ ] AC13: The constitution's §spec-phase directory tree gives `system.md`, `events.md` and `errors.md` each a one-line definition matching *What the other project files hold* in this spec.
- [ ] AC14: The shipped pre-commit hook refuses a commit whose staged copy of one of the four files is over its limit and larger than its `HEAD` copy or has none, naming the file, its size, the limit and `/prune --project`; a fixture commit that shrinks an over-limit file, or leaves it unchanged, passes.
- [ ] AC15: `/target`'s report of loaded shared constitutions gives each one's bytes on disk; a fixture shared constitution one byte over `read-size-bytes` is named as over its limit with its owning repository as the fix, and the run continues.
- [ ] AC16: The README's `[artifacts]` entry and the commented schema in `framework/bootstrap/ductus.md` §Project Configuration state that `read-size-bytes` also limits the four project files, `AGENTS.md` among them though it loads in every session rather than through a file read.

## Applicable Rules

- `QUAL-CLAIM-001` — the size check distinguishes a file it could not measure from one under its limit.
- `QUAL-TEST-001` — the size-check and `/prune` tests fail when the behavior they name is removed.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **What is each file's limit?** `[artifacts] read-size-bytes`, 50,000 bytes by default, for all four files (owner, 2026-10-29). At the 184–221 bytes a line measured on two `AGENTS.md` files (716 lines and 131,607 bytes; 220 lines and 48,546 bytes, `wc -lc`), it is about 226–272 lines, near Claude Code's 200, and it is the figure 072 holds this repository's `AGENTS.md` to. An `AGENTS.md` key of its own defaulting to 40,000 would save at most 10,000 bytes a session. A session's every-session load is then the framework constitution (49,993 bytes, `wc -c`, 2026-10-29), `AGENTS.md` (at most 50,000) and each registered shared constitution, which nothing limits.
- **Does a file over its limit block a commit?** Only a commit that grows it or takes it past the limit, plus an advisory `/analyze` finding naming `/prune` (owner, 2026-10-29). Blocking every commit while a file is over, as 072's ceiling does here, would stop an adopter's unrelated work on upgrade; one 131,607-byte `AGENTS.md` would need 81,607 bytes moved before anything else committed. An advisory alone is the exit 072 rejected, one that depends on someone choosing to take it. Refusing growth applies 072's exit where the writing happens: an addition past the limit retires something in the same change.
- **Does the check extend `check-artifact-size` or become its own primitive?** It extends `check-artifact-size`, with the four project files as a second subject set and a staged-against-`HEAD` mode for the hook (owner, 2026-10-29). The primitive already reads the limit, refuses a malformed one, words each kind's fix and matches stored discards; a new primitive or a shell check would each carry a second copy of that parse, which §runtime-boundary rules out for a capability that is runtime-eligible.
- **Is a file measured on disk or as a host loads it?** On disk, for all four (owner, 2026-10-29). That is an upper bound for every registry agent and what a file reader returns for the three files commands read. Claude Code strips block-level HTML comments, but whether Antigravity, OpenCode and Pi do is unverified. Measured as shipped, block comments are 4,252 of the `agents.md` template's 6,261 bytes, and 1,621 to 2,342 bytes of the other three templates (lines from `<!--` through `-->`, outside fences), so a file keeping its template comments spends at most 8.5% of its limit on them.
- **Where is each file's definition canonical?** The constitution's §spec-phase directory tree, one line each, with the template headers and `/prune`'s reference pointing at it (owner, 2026-10-29). Every agent reads the constitution in every session and every adopter receives it on each `/ductus` run, so the definition is in front of whoever writes to the file; `/prune`'s reference is read only once a file has grown, and the templates reach new adoptions only (`framework/bootstrap/ductus.md`, Shared Files: the constitution and references install with `update`, the three templates with `create`). Sharpening the three tree lines costs about 64 bytes, which `050-constitution` cuts in the same change as AC1's growth; the constitution measured 49,993 bytes against its 50,000-byte ceiling (`wc -c`, 2026-10-29).
- **Does `/prune` read source to judge a catalog entry no longer true?** Yes: it searches the source for the entry's identifier, shows the search, reports no hits as *no literal occurrence found*, and the operator confirms each removal (owner, 2026-10-29). §grounding requires consulting a reachable source before answering from the conversation, and asking the operator instead would make the result depend on their memory, which §design-principles rules out. Nothing else catches such an entry: `/analyze` checks a spec's events against `events.md` and `/review` checks code against `errors.md`, but neither reports an entry the code no longer has.
- **Does `/prune --all` walk the project files?** No: the project-file walk is `/prune --project`, and `--all` keeps walking every spec (owner, 2026-10-29). `041-task-pruning`'s scenario `prune-all-walks-every-spec` defines `--all` as the same reduction over every spec, and the project-file walk confirms each entry, so folding it into `--all` would add a per-entry walk to every corpus clean-up. A flag of its own gives the size check one exact command to name.
- **What does `/prune` do with an entry true for every adopter?** It checks the entry against the installed framework: already stated becomes a pointer, no longer true is deleted with its reason, still true stays as one line with a drafted upstream report the operator files (owner, 2026-10-29). These are 072's copy and no-longer-true exits, applied at the version the adopter runs; deleting a still-true workaround would leave agents without it until ductus changes, and filing a report publishes it, so the operator does that.
- **How does a session that runs no pipeline command reach the homes?** The `ductus:memory-routing` block names each home and when to read it, and `/prune` leaves a pointer line for an entry it moves to a home only this project has (owner, 2026-10-29). The block reaches every adopter's `AGENTS.md`, which all five registry agents load in every session. Claude Code's path-scoped `.claude/rules/` files and skills load on demand, but a `paths:` glob must name which code is "architecture", which nothing in a project derives, and the mechanism would need a counterpart per layout.
- **Is a shared constitution measured?** Yes, and reported only: `/target` gives each loaded one's size and names one over `read-size-bytes`, with the fix in its owning repository (owner, 2026-10-29). It loads in every session beside the framework constitution and `AGENTS.md`, so leaving it unmeasured hides the one unlimited file in that load. It is not in the project's index, so the hook has no staged copy to compare, and nothing in the project can dispose of an `/analyze` finding about it.
- **Which command reduces the project files?** `/prune`, extended to them (owner, 2026-10-29). Its purpose is already that each artifact says only what it should, decided per file by its durability class, and it already moved plan knowledge into `AGENTS.md`; one command lets that knowledge go straight to its home. A separate command would run the same walk while `/prune` kept filling the file it drained.
