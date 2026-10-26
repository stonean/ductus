# 072 — Governing documents stay within one read Plan

Implements [072 — Governing documents stay within one read](spec.md).

## Overview

Three changes land in order, and only the last two are enforcement:

1. **The cut.** Every `AGENTS.md` entry is classified once, by the rubric under §Entry dispositions, and the guide is rewritten from that classification to fit the ceiling. Lessons a check already reports leave for the check. Where a check's message does not name the fix, the fix is added to the message or as a comment at the enforcement site.
2. **The ceiling.** `scripts/lint-governing-size.sh` measures each governed file as it sits in the git index and fails when one is over its ceiling, missing or unreadable. `.githooks/pre-commit` and CI run it, and a test proves it fails.
3. **The import.** `CLAUDE.md` imports `AGENTS.md` as `@AGENTS.md` and drops what it copied from the guide. A tool-less `claude -p` run proves the guide loads.

No constitution content, runtime source or shipped file changes, so nothing here needs a release.

## Technical Decisions

### Entry dispositions

Each entry under `AGENTS.md`'s `##` sections gets exactly one disposition:

- **pointer**: the rule's canonical text is in `framework/constitution.md`. The entry becomes one line: its bold lead, kept verbatim, then the `§anchor` link and nothing else. Grepping the rule's distinctive phrasing then finds one statement and one pointer, which is the shape `050-constitution` AC3 describes.
- **trimmed**: a project-only rule stays. It keeps the rule, plus how this repository applies it where that is not obvious. The incident account goes ("Reason: surfaced …", the session, the task, what went wrong). An owner decision keeps one clause, `(owner, YYYY-MM-DD)`.
- **moved**: a test, hook, audit family or CI check in this repository reports the failure the entry warns about. The entry is removed, and the row names the enforcement site. Where that site's message does not name the fix, it is added in the same change.
- **deleted**: the entry is obsolete (it names a file, flag or behavior the repository no longer has), or it duplicates another entry and is merged into it. The row gives the reason, or the entry it merged into.

Four constraints bind every disposition, because existing checks read this file:

- **Bold leads stay verbatim** on every surviving entry. `/ductus:audit` Family 38 (`scripts/audit/promotion-coverage.sh`) matches each row of `050-constitution`'s plan §Classification to an entry by its lead phrase, and reports an unmatched key as a finding. Trimming applies to the body; a lead that is itself a story is re-keyed in 050's table in the same change.
- **A removed entry's 050 row goes with it.** When a **moved** or **deleted** entry has a row in 050's §Classification table, that row is removed or re-keyed to the entry it merged into, in the same change. 050 is `in-progress` and the table is its record of the guide, so the edit opens nothing.
- **Content a family reads survives.** The `<!-- audit:sweep-targets:begin -->` … `end` list inside the dead-references entry is Family 23's subject and is kept whole. `unbalanced-inline-markup` scans the file, so markup stays balanced.
- **The prose outside entries is held to the same rules.** That is about 9.4 KB of plain bullets, mostly §Gotchas and §Project Structure, and about 2.5 KB of section prose. It counts toward the ceiling, and its stories go too.

The table below is filled while the cut is made. The lead column is abridged to about 40 characters, so the table stays near 25 KB. It has one row per entry, in file order, cited by section and position as the file stood at `5cb01fdb`. Entries added after that commit get rows too, under the spec's *Entries added meanwhile* edge case.

| § | # | Entry (abridged lead) | Disposition | Destination or reason |
| --- | --- | --- | --- | --- |

### Size target

The ceiling is 50,000 bytes. The cut aims for 40,000–45,000, so the first lesson added after this spec closes does not fire the check. A ceiling met with zero headroom would force a retirement on the very next commit, before the exit has been exercised once.

### Fan-out for classification

The 164 entries are independent of one another, so classification fans out to parallel agents by section chunk. Each agent gets the rubric above, the four constraints, its chunk's text, and the byte arithmetic, not an adjective. A pointer is about 250 bytes, because leads run long, so the 56 cost about 14 KB. Headings and prose take about 5 KB. That leaves 21–26 KB for the trimmed set, so a trimmed entry's body is capped at about 350 bytes: the rule, and one sentence of how this repository applies it. Each agent returns a proposed disposition, the replacement text and, for **moved**, the enforcement site and whether its message names the fix. The assembly, the table, every write to `AGENTS.md` and the commit are serial (`AGENTS.md` §Workflow, the fan-out entry). Every **moved** and **deleted** proposal is verified against the tree before it is accepted. An agent's claim that a check covers a failure is a claim, and it is read before it is trusted.

### Enforcement-site edits stop short of `runtime/src/`

A message or comment added for a **moved** entry lands in `scripts/`, `scripts/tests/`, `.githooks/`, `.github/workflows/` or `runtime/tests/`. When the only enforcement site is in `runtime/src/` and its message lacks the fix, the entry stays **trimmed** instead. A change under `runtime/src/` reaches adopters only at a `ductus-v*` release and runs the cargo gate on every commit, and neither cost buys anything for a documentation move.

### The ceiling check is a script, not a runtime primitive

`scripts/lint-governing-size.sh` follows `scripts/lint-rule-filenames.sh`: a maintainer lint over this repository's own files, run by CI and never shipped. It parses no markdown, frontmatter or configuration. It reads two blob sizes with `git cat-file -s`, so §runtime-boundary principle 3's prohibition, which covers shell pipelines that parse frontmatter or markdown structure, does not reach it. It also fails eligibility criterion 2 (currently mechanical), because no LLM step or script performs this check today. The nearest primitive, `check-artifact-size`, measures one feature's spec artifacts. Giving it a governing-document subject means new argument and result types, CLI, MCP, registry and exec wiring (063's plan lists about twenty files for the primitive), plus a release, all to replace two `git cat-file` calls.

- **Ceilings** live in one table at the top of the script: `AGENTS.md 50000`, and `framework/constitution.md` at its byte size in the commit that adds the script. A comment on the `AGENTS.md` row names `DEFAULT_READ_SIZE_BYTES` in `runtime/src/primitives/check_artifact_size.rs` as the source of 50,000. The constitution row says `050-constitution`'s `the-constitution-fits-one-read` lowers it.
- **What is measured.** `git -C "$ROOT" cat-file -s ":$path"` reads the index. In the hook that is the staged file, so an unstaged edit neither fails nor passes a commit. In CI, the index is the checkout. One code path serves both, as AC9 requires.
- **Three failures, three messages**, each line naming the file:
  - Over the ceiling: `{file}: {size} bytes, over its {ceiling}-byte ceiling — retire, merge, or move an entry to the check that enforces it (spec 072)`. The constitution's line names its fix as the cut `050-constitution` owes.
  - Missing from the index: `{file}: not in the index, so its size cannot be measured`.
  - Unreadable (`cat-file` fails on a path the index holds): `{file}: could not be read from the index`.

  Exit 1 on any failure and 0 only when every file was measured and is at or under its ceiling (AC11). A file exactly at its ceiling passes.
- **Raising a ceiling** means editing the script. The comment above the table says that is an owner decision, recorded in this spec through its back-edge.

### Wiring

- **`.githooks/pre-commit`** runs `bash scripts/lint-governing-size.sh` unconditionally, after `check-corpus-links`, and its header comment gains a paragraph for it. Two `cat-file` calls cost nothing, so a path filter would buy nothing.
- **`.github/workflows/framework-checks.yml`** gains step `(o) Governing-document size ceilings`. It runs the lint and then `bash scripts/tests/test-lint-governing-size.sh`, following step `(k)`'s pairing of a guard with the test that proves it fails.
- **The wiring lands after the cut.** Wiring first would fail every commit until the guide fit.

### The test proves both directions

`scripts/tests/test-lint-governing-size.sh` follows `test-lint-release-ordering.sh`. Each case copies the lint into a throwaway tree, runs `git init`, stages sized files and runs the lint. The cases:

- (a) the committed `AGENTS.md` and constitution pass;
- (b) `AGENTS.md` one byte over fails, and the message names the file, the size, the ceiling and the fix;
- (c) exactly at the ceiling passes;
- (d) the constitution over its ceiling fails;
- (e) `AGENTS.md` absent from the index fails with the missing message, not a pass;
- (f) back under the ceiling after (b) passes.

### `CLAUDE.md`

The rewritten file has three parts:

- `# CLAUDE.md`.
- One import line, `@AGENTS.md`.
- The **Auto-Memory Routing** section `050-constitution` AC20 requires, reduced to a pointer at §drift-prevention's two questions plus the one fact this repository adds: it keeps no auto-memory at all.

The **Non-negotiables** section goes. Its trunk-based rule is an `AGENTS.md` entry and its truncation rule is §grounding, both now loaded through the import. The `@import framework/constitution.md` line goes rather than being converted, because at 119,262 bytes the constitution is more than one read (AC6). The pipeline commands keep reading it as they do today. No sentence states another file's size (AC8).

### Proving the import loads

AC7 is verified with `claude -p` at the repository root and the file tools disallowed (`--disallowedTools Read,Grep,Glob,Bash`), asking a question only `AGENTS.md` answers, with `--output-format stream-json --verbose` so the run's tool use is visible. Because the run has no file tools, a correct answer can only come from the loaded context. The same run against the pre-change `CLAUDE.md` is the negative control: it shows the probe fails when the guide is not loaded. Two things can make the control pass for the wrong reason, and both are checked before it counts:

- Claude Code's **Project instructions** setting, when it is `claude-md-and-agents-md`, loads `AGENTS.md` beside `CLAUDE.md` without any import.
- A nested `claude -p` that refuses to start says something about the environment, not about the import.

The probe question is picked after the cut, from an entry that survives it.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `AGENTS.md` | Modify | The cut: every entry reclassified and rewritten to fit the ceiling |
| `CLAUDE.md` | Modify | `@AGENTS.md` import; Non-negotiables and the constitution import removed; routing section reduced to a pointer |
| `scripts/lint-governing-size.sh` | Create | The ceiling check |
| `scripts/tests/test-lint-governing-size.sh` | Create | Proves the check fails and passes |
| `.githooks/pre-commit` | Modify | Runs the check on every commit |
| `.github/workflows/framework-checks.yml` | Modify | Step `(o)`: the check and its test |
| `scripts/`, `scripts/tests/`, `.githooks/`, `.github/workflows/`, `runtime/tests/` | Modify | Enforcement-site messages or comments for **moved** entries whose report did not name the fix |
| `specs/072-governing-documents-stay-within-one-read/plan.md` | Modify | The entry-disposition table (AC5) |
| `specs/050-constitution/plan.md` | Modify | §Classification rows for removed entries removed or re-keyed, so Family 38 still matches |

## Trade-offs

- **A runtime primitive for the ceiling.** Rejected for this spec: see *The ceiling check is a script*. If adopters later want a ceiling on their own guide, extending `check-artifact-size` is the route, and that is a spec of its own.
- **Deleting the 56 mirrors outright.** Rejected: `050-constitution` resolved that a promoted rule keeps a one-line pointer, so the index of how to work here does not silently lose a rule. A one-line pointer costs about 100 bytes.
- **Splitting `AGENTS.md` into topic files imported on demand.** Rejected: an import loads its file whole, so a split saves nothing unless the agent chooses what to read, which is a diligence dependency. Claude Code's path-scoped rules load by file path, and most entries here are about workflow rather than a path.
- **Warning instead of failing at the ceiling.** Rejected: a warning is an exit that depends on someone choosing to take it.
- **Known limitation.** The ceiling bounds bytes, not quality. A guide can be within its ceiling and still carry a story, and only review catches that. The ceiling makes the cost of a story visible, because every byte competes with a rule.
- **Known limitation.** Hosts that read `AGENTS.md` natively (Antigravity and OpenCode, per `framework/bootstrap/ductus.md`) get the smaller guide too. That is the intent, and nothing here tests it on those hosts.

## Cross-spec impact

- **`050-constitution`** owes the constitution's cut to one read (`scenarios/the-constitution-fits-one-read.md`, task 35). That scenario's link back to this spec discharges the `cross-spec-impact` declaration. When the cut lands, it lowers this spec's constitution ceiling in `scripts/lint-governing-size.sh`.
- **`050-constitution` plan §Classification** loses the rows of entries this spec removes, or re-keys them to the entry they merged into, so Family 38 keeps matching. That is a sync of 050's own record while it is `in-progress`.
- **`050-constitution` AC3** (each promoted entry's `AGENTS.md` line points without restating) is false against the tree at `5cb01fdb`, and the **pointer** disposition makes it true again. No edit to 050 is owed for that, because the criterion already states the shape this spec restores.
