---
status: in-progress
dependencies: [056-bootstrap-archive-boundary-split, 067-analyze-and-audit-checks]
cross-spec-impact: [026-framework-self-audit, 067-analyze-and-audit-checks]
next-criterion: 25
---

# 071 — Command reference split

Move each pipeline command's `## Markdown-only reference` out of the command
file and into a reference file the agent reads only when a step names a section
there. The command keeps its frontmatter, its Purpose / Context / Scope
sections and its `## Instructions`. The reference ships beside the
constitution, one copy for every agent.

## Motivation

Invoking a pipeline command loaded its whole body into the agent's context,
every time, on every agent. The large commands were mostly reference:
`/{project}:analyze` loaded 111,742 bytes, roughly 28k tokens, and 74,883 of
them were its `## Markdown-only reference`. A runtime-backed run reaches into
only a handful of those sections, through a step that names one.

The size had a second cost. On Claude Code, compaction keeps only the first
5,000 tokens of an invoked skill. Its documentation states that rule for
skills; command files load the same way, so this spec assumes it applies to
them too. For `analyze.md` the cut fell near Instructions step 14, so a
compacted session kept neither the later steps nor any of the reference.

The precedent is
[056 — Bootstrap archive-boundary split](../056-bootstrap-archive-boundary-split/spec.md),
which split `/ductus` itself into the installed `ductus.md` and the
`ductus-procedure.md` read from the archive "when the run reaches these
sections". This spec reuses its conventions. It differs in one respect: 056's
second half is read from the framework archive a `/ductus` run has just
extracted, but a pipeline command runs with no archive present, so this
reference has to be **installed**.

## The boundary

### What moves

Every command's `## Markdown-only reference` section, and nothing else, with
one exception: `review.md`'s top-level sections after its reference — Blocking
semantics, Blocking message, Waivers, Auto-fix scope, Output, Idempotency and
Notes for adopters — move with it. They are policy that `write-review` and
`process-waivers` already apply, read on the `--waive`, `--fix` and
blocking-message paths. `audit.md`'s two short sections after its reference
(*Boundary with `/ductus:analyze`* and *Output*) are not reference and stay.

The rule is uniform rather than size-based, so it is auditable: after the
split no file under `framework/commands/` contains `## Markdown-only
reference`. `amend`, `target`, `link` and `help` have no such section and are
untouched. `framework/bootstrap/ductus.md` is out of scope; 056 already split
it.

### Measured

Measured 2026-10-25 at `bb372676`, in bytes, before the split. *Kept* is what
stays in the command file, *Moved* is what leaves it.

| Command | Total | Kept | Moved |
| --- | --- | --- | --- |
| analyze | 111,742 | 36,859 | 74,883 |
| review | 64,647 | 18,937 | 45,710 |
| implement | 44,828 | 18,261 | 26,567 |
| audit | 37,267 | 4,817 | 32,450 |
| clarify | 30,694 | 18,556 | 12,138 |
| fold | 22,711 | 16,439 | 6,272 |
| specify | 21,884 | 12,752 | 9,132 |
| groom | 20,206 | 10,594 | 9,612 |
| prune | 19,879 | 15,102 | 4,777 |
| status | 19,529 | 6,545 | 12,984 |
| plan | 16,954 | 7,718 | 9,236 |
| consolidate | 16,141 | 12,920 | 3,221 |
| log | 4,414 | 3,776 | 638 |
| **Total** | **430,896** | **183,276** | **247,620** (57.5%) |

These figures are the estimate. The plan records the figures measured after the
split, since rewriting a pointer changes the kept side by a few bytes.

### Where the content lives

- **Source:** `framework/reference/<command>.md`. It sits outside
  `framework/commands/`, so nothing that globs that directory sees it: not the
  generator, not the parser, not any audit family keyed on command sources.
- **Adopter:** `.ductus/reference/<command>.md`, committed beside
  `.ductus/constitution.md`. One copy serves all five agents.
- **Never under an agent's commands directory.** Every `.md` file there
  registers as a command. A reference file installed there would become a
  command of its own.

### How a step points

Every step that pointed at "the markdown-only reference below" names a section
in a file instead, for example "apply **§Grounding** in
`.ductus/reference/analyze.md`". The `§name` is a real heading in the named
file, so a pointer can be checked by tooling and not only by reading it.

Each reference file opens with a **Where a named section lives** paragraph, as
`ductus-procedure.md` does: a section name with no heading in the reference
file is in the command file. That direction needs no read, because the command
is already loaded whenever its reference is.

The reverse direction does need a pointer. Other documents cite a command's
sections by file, as `analyze.md` §Rules or `review.md` §Notes for adopters.
After the move those headings are no longer in the command file, so each
command file states, once, near the top of its Instructions, that its
markdown-only reference is in `.ductus/reference/<command>.md` and that a
section it or another document names with no heading in the command file is
there. A citation of `<command>.md` §X then resolves in two hops, the way 056
left citations of `ductus.md` sections that moved to `ductus-procedure.md`.

A reference file is shared by every agent, so it cannot take a per-agent
`{cli-config-dir}`. The one occurrence names the agent running the command
through the `cli-config-dir` recorded in `.ductus/session.toml` instead.
`{project}` is unaffected: the installer substitutes it in every file it
copies, shared or not.

`$ARGUMENTS` stays in the command file. A host substitutes the invocation's
arguments into the command body it loads, and Pi substitutes them nowhere
else, so an `$ARGUMENTS` that moves stops being substituted. A moved sentence
that used it to name the argument names it in words instead. The move also
ends a quieter defect: a host substitutes every `$ARGUMENTS` in the body,
including one that only documents the token, so reference prose describing it
was rewritten with whatever the user typed.

## Behavior

### What the adopter installs

`/ductus` installs one `.ductus/reference/<command>.md` per shipped command
that has a reference. The rows are in the **Shared Files** manifest
(`ductus`-owned, strategy `update`), so the files are written in the same
`apply-manifest` call that writes the rewritten commands. The maintainer-only
`audit` command is withheld, and so is its reference.

### Cleanup

`.ductus/reference/` is an enforced directory: a `*.md` file there that no
manifest row produces, and that `[pinned] files` does not list, is removed.
A reference file a later release renames or retires is pruned rather than left
for an agent to read.

### No migration

`framework/migrations.toml` records convention removals, and nothing is removed
from an adopter here. The next `/ductus` run writes the rewritten commands and
the new reference files together. An adopter who has **pinned** a command keeps
its old, self-contained file, and that file stays consistent because its
pointers still say "below".

### This repository

The repository dogfoods the adopter layout. `scripts/gen-claude-commands.sh`
writes `.ductus/reference/*.md` from `framework/reference/*.md` with the same
`{project}` substitution, and its `--check` mode covers them. This repository's
pointers then resolve to the same paths an adopter's do. The repository has no
`.ductus/constitution.md` and leans on a runtime fallback for the constitution;
nothing in the runtime reads a reference file, so there is no fallback to lean
on, and the files have to exist.

## What the split touches

Checked 2026-10-25 by reading each consumer.

### Consumers that break, fixed in the same change

- **Family 18, `scripts/audit/marker-list-parity.sh`.** It parses the sentence
  "A criterion carrying any of … closed phrases is exempted whole" out of
  `framework/commands/analyze.md`. The sentence is in the reference, so the
  family would report that it could not locate it. It is pointed at
  `framework/reference/analyze.md`.
- **Family 28, `scripts/audit/audit-family-parity.sh`.** It reads the numbered
  family list out of `framework/commands/audit.md`, and that list is audit's
  reference. The family would report no documented families. It is pointed at
  `framework/reference/audit.md`.
- **`check-corpus-links`.** The moved text holds `](../constitution.md)` links.
  Installed at `.ductus/reference/` they resolve to `.ductus/constitution.md`,
  which is correct in an adopter. This repository's generated copies have no
  such file, so the links break there. The primitive excludes `.ductus/reference/`
  from the repository scope the way it excludes every agent's generated
  copies, because the files are framework-managed and only a release can
  repair them. Family 39 checks them at the source. This applies to adopters
  too, which is deliberate.
- **The installer's pointers into review.** `framework/bootstrap/ductus.md`
  cites `review.md` §Behavior step 5 for rule-surface selection, four times.
  That heading had already gone before this change. The selection it meant is
  item 5 of the reference's *Resolve target and scope* section, which
  Instructions step 2 carries out through `discover-rule-files`. Each pointer is
  re-pointed at the section that holds it after the move.

### Checks that would silently stop covering the moved text

- **`scripts/audit/placeholder-roundtrip.sh`** scans `framework/commands/*.md`.
  It extends to `framework/reference/*.md` with a stricter rule: no
  `{cli-config-dir}` and no hardcoded agent directory, since a shared file has
  no per-agent value.
- **Manifest parity (Family 16, `installer-command-parity.sh`).** Every
  `framework/reference/*.md` has a Shared Files row, maintainer-only commands
  excepted, and every row has a source.
- **Cross-file pointers.** Family 39 covers all of `framework/` from git, so
  the reference files' own constitution references stay checked. A pointer
  that names another file is *qualified*, though, and `resolve-anchor` counts a
  qualified reference without judging it. The primitive gains a bounded
  judgment. When a line names exactly one document, and the Shared Files
  manifest maps that document to a framework source, each `§name` on the line
  resolves if it names a heading in that source or a marker in the markers
  file, and is reported unresolved otherwise. A line naming several documents,
  or one the manifest does not map, stays counted and unjudged, as does a
  mapped source that cannot be read. The marker fallback is required: measured
  2026-10-25, the corpus held 16 references on lines naming exactly one mapped
  document, and every one was a constitution marker beside a path such as
  `specs/inbox.md`. A headings-only rule would have reported all 16; this one
  reports none of them.
- **CI.** `.github/workflows/runtime.yml` triggers on `framework/commands/*.md`
  and gains `framework/reference/*.md`.
- **The install fixture.** The parity install fixture gains a reference file,
  to prove both the install to `.ductus/reference/` and the pruning of a stale
  one.

### Consumers checked and unaffected

- Readers of `## Instructions` alone: the parser and `ductus exec`,
  `check-step-references`, `primitive-promotion-candidates`, and
  `lint-procedure-parseability`, which globs `framework/commands/` and
  `framework/bootstrap/` only.
- `check-command-flags` (Family 30): every Flags table is in the kept part.
- The `writeCode` payload: it reads the command's `Reference:` line, which is
  in the kept part of every command.
- `check-artifact-size`: its subjects are spec artifacts only.
- `adopter_destinations` in `check_artifacts.rs`: it derives destinations from
  the Shared Files table, so the new rows register by themselves, and a
  `.ductus/reference/…` path then counts as shipped.
- The name and frontmatter checks: Family 16's command rows,
  `readme-command-parity`, `gen-help-tables`, and `host-namespace-parity`.
- `ssot-invariants`: clarify's §Gate is in the kept part.
- `lint-tool-coverage.sh` skips the reference section. After the move that
  skip has nothing to skip, and it is removed as dead code.

### Prose that describes the old location

Comments and descriptions that say the reference is in the command file go
false with the move, and an identifier sweep does not find them. They are
corrected in the same change: the audit family comments that cite a command
file's reference (`sibling-coupling.sh`, `template-alignment.sh`,
`marker-list-parity.sh`, `audit-family-parity.sh`), `scripts/audit/README.md`,
the runtime doc comments that point at "`analyze.md`'s markdown-only
reference", and the `check-step-references` description in the MCP server and
its schema.

### Per-layout walk

- Command and skill paths, invocation, the `ductus` install path, and the
  command cleanup glob are unchanged for every layout.
- Antigravity's SKILL.md wrapper and Pi's prompt copies carry the same pointers
  to the same shared path.
- Settings and permissions are unchanged: every agent already reads
  `.ductus/constitution.md` from the same directory.
- Native rules are unaffected.

## Edge Cases

- **A reference file is missing when a step names it** — deleted by hand, or
  left behind by a run that failed after writing the commands. The agent
  reports the path it could not read, as unexamined, and does not improvise
  the section from memory (§grounding: a partial read is not a read). The next
  `/ductus` run writes it again.
- **A command is pinned and its reference is not.** The pinned command is the
  old, self-contained file, and its pointers say "below". The installed
  reference file is current and unread. Nothing is inconsistent.
- **A reference is pinned and its command is not.** The updated command can
  name a section the pinned reference lacks. The step then meets the missing
  section the way it meets a missing file: reported, not improvised. Pinning
  one half of a pair is the adopter's choice, and `[pinned] files` already
  documents that a pinned file stops receiving updates.
- **A file an adopter put in `.ductus/reference/` by hand.** The directory is
  `ductus`-owned and enforced, so it is removed unless pinned, the same rule
  the commands directory follows.
- **A command with no reference.** `amend`, `target`, `link` and `help` get no
  reference file and no location line.
- **`/{project}:configure`.** Its sources are
  `framework/bootstrap/configure/{key}.md`, which carry no markdown-only
  reference, so it is untouched.
- **The maintainer-only `audit` command.** Its reference is dogfooded here at
  `.ductus/reference/audit.md` and never shipped.

## Acceptance Criteria

- [ ] AC1: No file under `framework/commands/` contains a `## Markdown-only reference` heading.
- [ ] AC2: Each of the thirteen commands measured above has a `framework/reference/<command>.md` holding its moved sections, `review.md`'s trailing sections included, and opening with a **Where a named section lives** paragraph.
- [ ] AC3: Every step pointer that read "the markdown-only reference below", in any wording, names a `§` heading and a `.ductus/reference/<command>.md` file, and each named heading exists in that file.
- [ ] AC4: No `framework/reference/*.md` file contains `{cli-config-dir}`, a hardcoded agent config directory, or `$ARGUMENTS` used as the argument token, and `placeholder-roundtrip.sh` fails when one is introduced.
- [ ] AC5: `framework/bootstrap/ductus.md`'s Shared Files manifest carries one `update` row per shipped reference file, mapping `framework/reference/<command>.md` to `.ductus/reference/<command>.md`, with `audit` withheld, and Family 16 fails when a reference file lacks a row or a row lacks a source.
- [ ] AC6: `.ductus/reference/` is in the installer's enforce-directories list and in its markdown-only cleanup, so a reference file no manifest row produces is removed unless pinned.
- [ ] AC7: `framework/bootstrap/govern.md` is byte-identical to `framework/bootstrap/ductus.md`.
- [ ] AC8: No reference file is written under any agent's commands, command, skills or prompts directory, for any of the five layouts.
- [ ] AC9: `scripts/gen-claude-commands.sh` writes `.ductus/reference/*.md` from `framework/reference/*.md` with `{project}` substituted and prunes one with no source, its `--check` mode reports drift in either, and the pre-commit hook stages the directory.
- [ ] AC10: Family 18 reads its marker restatement from `framework/reference/analyze.md` and still reports a finding when the sentence is absent.
- [ ] AC11: `check-corpus-links` excludes `.ductus/reference/` from the repository scope and counts the exclusion, with a test that fails when the exclusion is removed.
- [ ] AC12: `resolve-anchor` judges a `§name` on a line naming exactly one document that the Shared Files manifest maps to a framework source: resolved when it names a heading in that source or a markers-file marker, unresolved otherwise. A line naming several documents or an unmapped one, and a mapped source that cannot be read, stay counted and unjudged. Each case has a test that fails when its behavior is removed, and the rule reports no unresolved reference in this repository's `framework/` or spec corpus that the previous rule did not.
- [ ] AC13: The installer's pointers at `review.md` §Behavior step 5 name a section that exists in `.ductus/reference/review.md`.
- [ ] AC14: `.github/workflows/runtime.yml` triggers on `framework/reference/*.md` wherever it triggers on `framework/commands/*.md`.
- [ ] AC15: The parity install fixture installs a reference file to `.ductus/reference/` and prunes a stale one there, and the test fails when either step is removed.
- [ ] AC16: `framework/constitution.md` §drift-prevention's canonical-sources row for pipeline command behavior names `framework/reference/<command>.md`, and §runtime-host-integration says where a command's markdown-only prose lives.
- [ ] AC17: `AGENTS.md`'s entry on wiring a new slash command names the reference file and its manifest row.
- [ ] AC18: `scripts/audit/run-all.sh` reports no findings, and the three generators plus `derive-dependencies` and `derive-references` report no drift.
- [ ] AC19: The whole local gate passes: `npx markdownlint-cli2`, the six `lint-*.sh` scripts, `scripts/tests/*.sh`, `shellcheck -S warning` over the tracked shell set, and under `runtime/` `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings` and `cargo test --release --locked`.
- [ ] AC20: The bytes each command loads per invocation, before and after the split, are recorded in the plan from a measurement taken after the split.
- [ ] AC21: Family 28 reads its documented family list from `framework/reference/audit.md` and still reports a finding when the list is absent.
- [ ] AC22: Each split command file states once, in its Instructions preamble, that its markdown-only reference is in `.ductus/reference/<command>.md`, and the four commands with no reference carry no such line.
- [ ] AC23: No live comment, description or `docs/` page outside `specs/` still says a command's markdown-only reference is in its command file.
- [ ] AC24: 026 carries the changed subjects of Families 4, 16, 18, 28 and 39, and 067 carries the `resolve-anchor` and `check-corpus-links` changes, each as a scenario back-linking to this spec, so the `cross-spec-impact:` declaration is discharged.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Where does the reference live?** Resolved: `framework/reference/<command>.md`
  at the source and `.ductus/reference/<command>.md` in an adopter. Under
  `framework/commands/` every reader of that directory would see it, and under
  an agent's commands directory it would register as a command. Operator
  decision, 2026-10-25.
- **Which commands are split, and by what rule?** Resolved: all thirteen that
  have a `## Markdown-only reference`, including the small ones (`log`,
  `consolidate`, `prune`) and the maintainer-only `audit`. A size threshold
  would leave nothing to audit; the uniform rule leaves one invariant. `audit`'s
  reference stays in this repository, because `audit` is not shipped. Operator
  decision, 2026-10-25.
- **Do review's sections after its reference move too?** Resolved: yes. They
  are policy the primitives already apply, and the steps that need them —
  `--waive`, `--fix` and the blocking message — get named pointers. Review's
  command file ends near 19KB. Operator decision, 2026-10-25.
- **Does the change need a migration entry?** Resolved: no. Nothing is removed
  from an adopter, and the rewritten commands and the new reference files land
  in one `apply-manifest` call. A pinned command keeps its self-contained file.
- **Who owns the runtime half?** Resolved: the `check-corpus-links` exclusion
  and the `resolve-anchor` extension change checks that
  [067 — Analyze and audit checks](../067-analyze-and-audit-checks/spec.md)
  owns, so each lands as a scenario under 067 through the back-edge, with
  067's data model updated. This spec keeps the requirement, the constitution
  amendment and the criteria above, so it cannot reach `done` before 067's
  scenarios do.
- **How are pointers into a reference file checked?** Resolved: `resolve-anchor`
  gains the bounded judgment described under *Cross-file pointers*. This
  reverses a decision recorded in 067's `anchor-reference-kinds` scenario,
  which rejected resolving qualified references because it made the primitive
  a link resolver with a file-loading policy, an error surface for unreadable
  targets, and a question about what counts as a document root. The manifest
  answers all three: it is the loading policy, a mapped source that cannot be
  read stays unjudged, and the source path is the root. The alternatives were
  a section-pointer finding in `check-step-references`, which reads only
  `## Instructions` and would miss pointers in Context, Scope, the Gate table
  and the installer, and no check at all. Operator decision, 2026-10-25; the
  reversal is recorded in 067's scenario.
- **Are the done specs that cite a moved section by its command file
  reopened?** Resolved: no, with two exceptions. Measured 2026-10-25, about 67
  lines across the corpus cite `<command>.md` together with a moved section's
  name. Most are spent `tasks.md` entries and regenerated review and analysis
  records, which stay as written. The rest are navigational, and the command
  file's location line resolves them in two hops, which is the footing 056
  left its own inbound citations on. The exceptions are claims about a check's
  behavior that this change alters: 026's Families 18 and 28 read a different
  file, and 067's `resolve-anchor` judges what it used to only count. Those are
  recorded in 026 and 067, which is the `cross-spec-impact:` this spec
  declares.
- **Which changelog records the change?** Resolved: the release that ships
  it, in `runtime/CHANGELOG.md`. The root `CHANGELOG.md` records removed
  conventions, and nothing is removed here. `runtime/CHANGELOG.md` takes a
  section per release, and Family 20 holds its newest heading to the version
  pin, so an entry written ahead of the release would fail the audit. Measured
  2026-10-25: the pin is 0.59.11 and 238 commits sit unreleased since
  `ductus-v0.59.11`.
- **Is the commands' colon-form invocation in prose fixed here?** Resolved: no.
  It is wrong for non-claude layouts today and is unchanged by this move, so it
  is its own change. Trimming `analyze`'s Instructions below the compaction
  window, and splitting `/ductus` further, are out of scope for the same
  reason.
