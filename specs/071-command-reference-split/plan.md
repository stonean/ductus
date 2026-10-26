# 071 — Command reference split Plan

Implements [071 — Command reference split](spec.md).

## Overview

A script cuts each command's `## Markdown-only reference` (and review's
trailing sections) into `framework/reference/<command>.md` byte for byte, so
the move itself is provable by diff. A separate, reviewable pass then repairs
the prose the move falsifies: the step pointers, the two directional
paragraphs, `$ARGUMENTS` and `{cli-config-dir}`, and intra-file links whose
target stayed behind. The installer ships the files through the Shared Files
manifest and enforces their directory. The generator dogfoods them. Five audit
families and two runtime primitives follow the content to its new file, and
the primitive changes are recorded as scenarios under the specs that own them.

## Technical Decisions

### The cut is mechanical and verified before any prose changes

A script copies each moved range out of the command file at the commit the
task starts from, and the first check is that the reference body equals that
range, modulo the heading promotion below. Prose repairs are a separate step
on top. This is 056's discipline: a move mixed with edits cannot be reviewed
as a move. The ranges are the spec's: `## Markdown-only reference` to end of
file for every command, except `audit` (it stops before `## Boundary with
/ductus:analyze`, which stays). `review`'s range runs to end of file and so
takes its trailing sections, per the spec's What moves. The ranges are cut by
line, after reading where each ends. `review.md` holds `## Summary` through
`## Unexamined governance` inside a fenced `markdown` block
(`framework/commands/review.md:398-447`), so a cut keyed on level-2 headings alone would
end in the wrong place.

### Shape of a reference file

- No frontmatter. It is not a command, and the command-frontmatter checks
  must not see it as one.
- An H1, `# <command> — markdown-only reference`.
- The moved section's own lead paragraph, which says what the reference is
  for.
- A **Where a named section lives** paragraph: a section this file names with
  no heading here is in the command file, which is already loaded whenever
  this one is read.
- The moved sections, each heading promoted one level (`###` → `##`, `####` →
  `###`), so the file opens H1 → H2 as markdownlint's MD001 requires. Lines
  inside fenced blocks are never touched: review's record skeleton is a fenced
  `markdown` block whose headings are content, not structure. Review's
  trailing `##` sections keep their level and become siblings of the promoted
  ones.
- Numbered headings lose their number (`### 1. Resolve target and scope` →
  `## Resolve target and scope`). `resolve-anchor`'s reference grammar is
  `§[A-Za-z]…` (`runtime/src/primitives/resolve_anchor.rs:232-235`), so a
  heading starting with a digit cannot be named by a `§` pointer.

### How a step points

A pointer names the full heading and the file, as in "apply
**§Grounding (advisory)** in `.ductus/reference/analyze.md`". The full heading is
required because the intra-document match is a prefix match of the text after
`§` against each heading (`resolve_anchor.rs:211-216`). `§Rules` does not match
the heading `Rules (blocking and advisory)`; the pointer spells the whole
heading. A pointer line names that one document and no other, so the bounded
`resolve-anchor` judgment applies to it.

Every wording counts as a pointer, not only "markdown-only reference below".
Measured at `a55af0cd`, the kept parts carry 29 occurrences of the phrase
across 11 files, against the 28 in 12 the operator's plan estimated, and
`implement.md` and `status.md` carry none although they have 26,567 and 12,984
bytes of reference. So the repair reads each kept part for "below", "detailed
walk", "the … section" and bolded section names, rather than grepping one
phrase.

### The location line

Each split command gets one line, placed after the *For agent runtimes*
blockquote at the head of `## Instructions`: its markdown-only reference is
`.ductus/reference/<command>.md`, and a section it or another document names
with no heading in the command is there. This resolves the inbound citations
in other specs (`analyze.md` §Rules, `review.md` §Notes for adopters) in two
hops, as `ductus.md`'s `## The archive half` does for 056
(`framework/bootstrap/ductus.md:798-813`). The four commands with no reference
get no line.

### Prose the move falsifies

- **`$ARGUMENTS`.** One occurrence moves: `specify.md`'s reference, step 1 of
  *Resolve feature number and slug*. It becomes "the command's argument".
  `analyze.md`'s reference documents the token (files whose body documents an
  `$ARGUMENTS` parameter) and keeps it literal; in a reference file a host no
  longer substitutes it, which is the point.
- **`{cli-config-dir}`.** One occurrence moves, `analyze.md`'s reference
  (`framework/commands/analyze.md:449`): "the §Agent Registry row with
  `config_dir` `{cli-config-dir}`" becomes the row whose `config_dir` is the
  `cli-config-dir` recorded in `.ductus/session.toml` for the agent running
  the command.
- **Intra-file links and directions.** A moved link to a heading that stayed
  (review's `[Inputs](#inputs)`) becomes a bolded section name, which the
  *Where a named section lives* paragraph resolves. "Above" and "below" are
  rewritten wherever their target changed files.
- **Constitution links.** The moved `](../constitution.md…)` links resolve from
  `framework/reference/` exactly as from `framework/commands/`, and from
  `.ductus/reference/` in an adopter. They stay.

### The installer

- **Rows.** Twelve rows in Shared Files → `ductus`-owned shared files
  (strategy: update), `framework/reference/<command>.md` →
  `.ductus/reference/<command>.md`, `audit` withheld per
  `scripts/maintainer-only-commands.txt`.
- **Cleanup.** `.ductus/reference/` joins the enforce-directories list in
  `ductus.md`'s Instructions step 1 and step 8 (`enforce-manifest`, glob `*.md`, expected
  set the twelve destinations, pinned honored). A **Reference cleanup**
  paragraph under Shared Files gives the markdown-only path's equivalent of
  its **Slash command cleanup**.
- **`govern.md`.** Copied from `ductus.md` in the same change; Family 21 holds
  them byte-identical.

### The generator dogfoods the adopter layout

`scripts/gen-claude-commands.sh` gains a pass writing
`.ductus/reference/<name>.md` from `framework/reference/<name>.md` with the
same `substitute` function, pruning a file with no source and checking both in
`--check`. All thirteen are written, `audit` included: the maintainer-only
rule withholds files from adopters, and this repository dogfoods `audit`
already (`.claude/commands/ductus/audit.md`). `.githooks/pre-commit` gains
`git add .ductus/reference/` beside its other generator outputs
(`.githooks/pre-commit:94-95`). `.gitignore` ignores only `session.toml`,
`sessions/` and `bin/` under `.ductus/`, so the directory is tracked.

### Audit families

- **Family 18** (`marker-list-parity.sh`) and **Family 28**
  (`audit-family-parity.sh`) read their subject from
  `framework/reference/analyze.md` and `framework/reference/audit.md`, with
  their labels and fixes renamed. Each keeps its fail-closed path on a missing
  file and on an empty extraction.
- **Family 4** (`placeholder-roundtrip.sh`) also scans
  `framework/reference/*.md`, where `{cli-config-dir}` and a hardcoded agent
  directory are both findings: a shared file has no per-agent value.
- **Family 16** (`installer-command-parity.sh`) gains a reference arm: every
  `framework/reference/<name>.md` has a Shared Files row unless `<name>` is
  maintainer-only, every reference row has a source, and a maintainer-only
  reference has no row.
- **Family 39** (`shipped-anchor-resolution.sh`) reports the new
  `cross-document` count on stderr with the others.
- **`lint-tool-coverage.sh`** loses its markdown-only-reference skip, dead once
  no command file has the heading.

### Runtime

- **`check-corpus-links`.** `.ductus/reference/` joins the repository-scope
  exclusions in `is_excluded`
  (`runtime/src/primitives/check_corpus_links.rs:150-178`), counted like the
  agent directories, when the tree carries `framework/reference/`, the
  sources the copies are generated from. Without them the files are the
  installed ones and are checked, which keeps them in Family 35a's synthetic
  adopter tree.
- **`resolve-anchor`.** A line naming exactly one document, where the
  `## Shared Files` section of `framework/bootstrap/ductus.md` maps that
  document to a source, is judged: each `§` resolves when it names a heading
  in the source (longest-first prefix, the intra-document rule), a heading in
  the citing file, or a marker in the markers file, and is unresolved
  otherwise. Those references get a new
  kind, `cross-document`, and a count of their own, so `qualified` keeps
  meaning *counted, not judged*. A pointer written as an explicit pair,
  `**§X** in` a shipped document, is judged against that document's headings
  alone, whatever else its line names: 59 of the 100 pointers share a line
  with `plan.md`, `spec.md` or another file. Otherwise a line naming several
  documents, an unmapped one, or a mapped source that cannot be read stays
  `qualified`. The map is
  empty where `framework/bootstrap/ductus.md` is absent, which is every
  adopter, so the rule engages only in a framework checkout. That is the same
  discriminator `adopter_destinations` uses
  (`runtime/src/primitives/check_artifacts.rs:1370-1377`), but this map reads
  the Shared Files section alone and keeps each row's source, which
  `manifest_destinations` discards.
- **Doc comments** that place the reference in `analyze.md`
  (`check_artifacts.rs:9`, `schema/primitives.rs:4417`) and the
  `check-step-references` description (`mcp/server.rs`,
  `schema/primitives.rs`) are corrected.

### Ownership

The runtime half routes by `AGENTS.md`'s ownership rule: `check-corpus-links`
and `resolve-anchor` are 067's, so each change is a 067 scenario back-linking
to 071, with 067's `data-model.md` updated for the new kind and count and a
matching task in 067's `tasks.md`. The audit families are 026's, so their
changed subjects are one 026 scenario, and the lines in 026's Family 18 and
Family 28 scenarios and its AC21 that name the old file are corrected under
the same reopen. Both owners are `done` and reopen `done → in-progress`. 071
keeps the requirement, the constitution amendment and its criteria.

### Constitution

The §drift-prevention canonical-sources row for pipeline command behavior
names `framework/reference/<command>.md` beside the command source.
§runtime-host-integration gains one sentence: a command's markdown-only prose
lives in its reference file, `.ductus/reference/<command>.md` in an adopter,
read when a step names a section there. Spec 048's AC17 still holds: the
reference sections remain, in another file.

### The gate is the whole CI surface

Every command file changes, so the `implement-basic` parity golden's commit
shas move (`AGENTS.md` Gotchas). It is re-blessed filtered to that test, and
the diff is confirmed to be shas only. No `## Instructions` step is added or
removed, so no step number moves and no other golden should. The install
golden moves with the fixture change and is re-blessed filtered the same way.

### Bytes loaded per invocation, measured after the split

Measured 2026-10-25 over the tree that lands the split, against the spec's
figures at `bb372676`. *Before* is the whole command file, which every
invocation loaded; *After* is the command file alone, which is now all an
invocation loads; *Reference* is read only when a step names a section in it.

| Command | Before | After | Change | Reference |
| --- | --- | --- | --- | --- |
| analyze | 111,742 | 38,862 | −65.2% | 75,492 |
| review | 64,647 | 19,839 | −69.3% | 46,458 |
| implement | 44,828 | 19,034 | −57.5% | 27,104 |
| audit | 37,267 | 5,207 | −86.0% | 33,026 |
| clarify | 30,694 | 19,520 | −36.4% | 12,638 |
| fold | 22,711 | 16,796 | −26.0% | 6,735 |
| specify | 21,884 | 13,146 | −39.9% | 9,660 |
| groom | 20,206 | 11,119 | −45.0% | 10,072 |
| prune | 19,879 | 15,434 | −22.4% | 5,240 |
| status | 19,529 | 7,019 | −64.1% | 13,465 |
| plan | 16,954 | 8,145 | −52.0% | 9,711 |
| consolidate | 16,141 | 13,264 | −17.8% | 3,693 |
| log | 4,414 | 4,104 | −7.0% | 1,097 |
| **Total** | **430,896** | **191,489** | **−55.6%** | **254,391** |

Per invocation the commands load 239,407 fewer bytes in total, 55.6%. The
spec's estimate kept 183,276; the 8,213-byte difference is what the repair
added back: the location line in each of the thirteen command files and the
pointers, each of which names a full heading and a file where it used to say
"below". `analyze`, the case that motivated the change, drops from 111,742 to
38,862 bytes. That is still above the ~20KB compaction window, which the spec
leaves to a follow-up.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/reference/*.md` | Create | The thirteen reference files |
| `framework/commands/*.md` | Edit | Cut the reference; rewrite pointers; add the location line |
| `framework/constitution.md` | Edit | Canonical-sources row; §runtime-host-integration sentence |
| `framework/bootstrap/ductus.md` | Edit | Shared Files rows; enforce-directories; reference cleanup; review pointers |
| `framework/bootstrap/govern.md` | Edit | Byte copy of `ductus.md` |
| `scripts/gen-claude-commands.sh` | Edit | Write, prune and check `.ductus/reference/` |
| `.githooks/pre-commit` | Edit | Stage `.ductus/reference/` |
| `.ductus/reference/*.md` | Create | Generated dogfood copies |
| `scripts/audit/marker-list-parity.sh` | Edit | Family 18 subject |
| `scripts/audit/audit-family-parity.sh` | Edit | Family 28 subject |
| `scripts/audit/placeholder-roundtrip.sh` | Edit | Family 4 extends to reference files |
| `scripts/audit/installer-command-parity.sh` | Edit | Family 16 reference arm |
| `scripts/audit/shipped-anchor-resolution.sh` | Edit | Family 39 reports `cross-document` |
| `scripts/audit/sibling-coupling.sh` | Edit | Comment |
| `scripts/audit/template-alignment.sh` | Edit | Comment |
| `scripts/audit/README.md` | Edit | Family descriptions |
| `scripts/lint-tool-coverage.sh` | Edit | Remove the dead skip |
| `.github/workflows/runtime.yml` | Edit | Path filters |
| `runtime/src/primitives/check_corpus_links.rs` | Edit | Exclusion and test |
| `runtime/src/primitives/resolve_anchor.rs` | Edit | Bounded judgment and tests |
| `runtime/src/schema/primitives.rs` | Edit | `cross-document` count; doc comments |
| `runtime/src/mcp/server.rs` | Edit | Tool descriptions |
| `runtime/src/primitives/check_artifacts.rs` | Edit | Doc comment |
| `runtime/tests/parity.rs` | Edit | Install fixture assertions |
| `runtime/tests/fixtures/ductus-basic/` | Edit | A reference file and a stale one |
| `runtime/tests/golden/ductus-basic.jsonl` | Edit | Re-blessed |
| `runtime/tests/golden/implement-basic.jsonl` | Edit | Re-blessed, shas only |
| `specs/067-analyze-and-audit-checks/scenarios/*.md` | Create | The two runtime scenarios |
| `specs/067-analyze-and-audit-checks/data-model.md` | Edit | `resolve-anchor` kind and count |
| `specs/026-framework-self-audit/scenarios/*.md` | Create | The family-subject scenario |
| `AGENTS.md` | Edit | The new-slash-command entry |

## Trade-offs

- **A size threshold** would have split only the large commands. Rejected:
  it leaves nothing auditable, while "no file under `framework/commands/` holds
  the heading" is one grep.
- **Converting commands to skills**, which load supporting files on demand
  natively, was out of scope: it changes command names, install paths and
  layouts on five agents.
- **Installing the reference under each agent's commands directory** was
  rejected: every `.md` file there registers as a command.
- **Reading the reference from the archive, as 056 does,** cannot work: a
  pipeline command runs with no archive present.
- **Checking pointers in `check-step-references`** would leave the pointers in
  Context, Scope, the Gate table and the installer unchecked. The operator chose
  the bounded `resolve-anchor` judgment instead (spec, Resolved Questions).
- **Known limitation: a pointer into `audit`'s reference would not be
  judged.** `audit` is not in the Shared Files manifest, so a `§` pointer
  naming `.ductus/reference/audit.md` would stay `qualified`. None exists: the
  maintainer-only command names its reference file as a whole.
- **Known limitation: a `§` pointer must spell its heading in full**, because
  the match is a prefix match. A shortened pointer is reported unresolved,
  which is the safe direction.

## Cross-spec impact

- **067 — Analyze and audit checks.** Owes two scenarios: `check-corpus-links`
  excludes `.ductus/reference/` on the repository scope, and `resolve-anchor`
  judges a reference on a line naming one manifest-mapped document. The second
  reverses the resolved question in `anchor-reference-kinds`, which is
  corrected to point at it. Its `data-model.md` gains the kind and the count.
  Discharged when both scenarios link back to 071.
- **026 — Framework self-audit.** Owes one scenario recording the changed
  subjects of Families 4, 16, 18, 28 and 39, with the lines in its Family 18
  and Family 28 scenarios and its AC21 that name the old file corrected.
  Discharged when the scenario links back to 071.
- **016, 024 and the other done specs** that cite a moved section by its
  command file are not reopened; the location line resolves those citations
  (spec, Resolved Questions).
