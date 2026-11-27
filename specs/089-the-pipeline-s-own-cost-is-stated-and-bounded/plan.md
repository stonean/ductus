# 089 — The pipeline's own cost is stated and bounded Plan

Implements [089 — The pipeline's own cost is stated and bounded](spec.md).

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

- **AC1.** A §design-principles bullet states the cost rule. It lands in the commit that cuts the constitution, since the constitution sits 89 bytes under its 50,000-byte ceiling.
- **AC3.** Four files are cut to fit, in this order: `AGENTS.md`, whose entries can route into a part; the constitution, whose rules can move into a reference; then `framework/reference/analyze.md` and `framework/reference/review.md`.
- **AC2.** `scripts/lint-governing-size.sh` then gains its lowered ceilings and two glob rows, all in the commit where every governed file already fits.
- **AC4.** Closes once 086 states its cost, since 085, 087 and 088 already do.

## Technical Decisions

### The cost rule is a §design-principles bullet (AC1)

- **Placement.** Beside *Use what a task needs*, so the section's preamble makes it a hard filter (spec, Resolved Questions). It reads, in substance: a spec that adds a check, an audit-record field or a primitive states what a run of it costs, in a unit it can measure (model turns, bytes loaded, commits or wall time), and which check, field or primitive it retires, or why none.
- **Who it binds.** It binds every spec not `done` when it lands (spec, Resolved Questions). At planning those are 060, 066, 086 and 089 (`status:` of every `specs/*/spec.md`, 2026-11-27):
  - **086** is AC4's, and is its task 8.
  - **089** adds ceilings on 30 files to a check, so it states its own cost.
  - **060**'s task 15 adds a judgment to `scripts/analyze/`, so it is bound once this lands, which its task 15's own ordering puts before it.
  - **066**'s open task 32 changes when an existing primitive counts a criterion as owing a review. It adds no check, field or primitive, so it is not bound.

### The byte ledger, cut in feeding order (AC3)

Bytes are by `wc -c`, or by a Python split on headings outside fences summing to `wc -c`, at `d963a791`. A reference's figures come from an inventory that read each file whole: 386 of 386 lines and 838 of 838. Each section's bytes were classed by route against the compiled descriptions in `runtime/src/schema/descriptions.rs` and the command files. `AGENTS.md`'s and the constitution's come from the 2026-11-25 inventory in the spec's Resolved Questions, which no cut has verified.

| File | Now | End (estimated) | Must end at or under | Ceiling |
| --- | --- | --- | --- | --- |
| `AGENTS.md` | 49,820 | 32,400 to 37,000 | 49,000 | the cut's size rounded up to the next 1,000 |
| `framework/constitution.md` | 49,911 | about 48,700 to 49,000 | 49,000 | the cut's size rounded up to the next 1,000 |
| `framework/reference/analyze.md` | 69,387 | 49,748 (cut) | 50,000 | 50,000 |
| `framework/reference/review.md` | 65,138 | about 48,500, plus 086 task 4's addition | 50,000 | 50,000 |
| `framework/reference/implement.md` | 14,055 | 14,055 (unchanged) | 50,000 | 50,000 |

A ceiling is computed from the file's size at the commit that sets it: ⌈size ÷ 1,000⌉ × 1,000. AC2 holds each governing ceiling below 50,000, so each governing document must end at or under 49,000.

### `AGENTS.md` is cut by `/ductus:prune --project`

It is the project's command for reducing a project file (075), so no hand-rolled walk stands in for it. The operator confirms each move. Homes follow its routing:

- A constitution rule's entry stays a one-line pointer.
- A procedure goes to `.ductus/docs/agents/developing-ductus.md` (30,826 bytes), or to a sibling part once that part would pass 50,000 (087).
- A lesson a check reports leaves, with the fix added to the check's message (072).
- A lesson's history goes to its commit.

An entry true for every adopter that the constitution does not state stays as one line, with prune's drafted upstream report put to the owner. 089 promotes none: promotion is 050's subject, and the constitution's cut cannot pay for it.

### The constitution keeps every rule, and must end at or under 49,000

It needs at least 1,156 bytes out: 49,911, plus 244 for AC1's bullet, less 49,000. The cut ends at 48,999 (`wc -c`):

- **Pointed at `framework/reference/implement.md`.** §brownfield-inbox's two `/implement` bullets, *In `/{project}:implement`, scope decides the destination* (315 bytes) and *Record, do not derail* (107), become one pointer to the reference's *Walk through tasks* step 5. That step already states both, so the reference does not change. 050's AC15 says §brownfield-inbox states the tiers, so AC15 gains a *Superseded in part by 089* note, which owes 050 one review (the agent's recommendation, adopted by the operator, 2026-11-27). 050's scenario `findings-route-by-scope` signpost and `AGENTS.md`'s entry on a finding made during implementation are re-pointed, and 050's plan gains that entry's row under *Relocated to a command reference*.
- **Copies of a primitive's rules cut to a pointer.**
  - *An anchor reference resolves by line*, to `resolve-anchor`.
  - *Detect, decide, re-check, record*, to the analyze reference's *Finding dispositions*, whose order bullet task 3 keeps.
  - The last sentence of *Of a run record's findings, only a gating one holds `done`*, which `check-review-gate`'s description states (the dispositions-map block). The rest of that bullet stays, since 076's plan describes its tiers.
- **Tightened** under §text-first-artifacts' as-short-as rule:
  - §runtime-host-integration's host paragraph;
  - *Prefer the source to the recollection*, which §grounding's opening states;
  - §brownfield-inbox's backlog-drains sentence;
  - §drift-prevention's sentence that a reworded line is not mechanical, which its definition of a mechanical sweep implies;
  - *Each capability arrives in its own feature spec*, which §pipeline-boundaries' bar on implementing without a spec covers.

These candidates stay where they are:

- The constitution's Validation Severity subsection is the canonical tier assignment `validate-frontmatter` follows (067's scenario `frontmatter-severity-tiers-on-both-sides`), so it is not moved.
- No other single-command passage moves into the analyze or review reference, since their margins belong to their own cuts.
- §spec-lifecycle's sentence that the gate composes with `/analyze` and the shipped CI template stays, since 046's Resolved Questions quote it.

Rules for every cut:

- A passage another spec quotes stays verbatim, or its quoting spec is corrected through its own back-edge (050's scenario `the-constitution-fits-one-read`, Edge Cases). Each cut passage is searched for in `specs/*/spec.md` and `specs/*/scenarios/*.md` before it goes.
- Every anchor stays (050's AC10).
- If the cut cannot reach 49,000 without dropping required content, the ceiling returns to the owner as an open question through `/ductus:amend`, as it did in that scenario, rather than shipping a ceiling of 50,000 that AC2 does not allow.

### The analyze reference moves the primitives' rules into their descriptions

The agent's recommendation, adopted by the owner, 2026-11-27. Cutting copies and duplicates alone leaves it at about 51,100. The constitution makes the binary the source of a primitive's rules (§runtime-host-integration), and these spans are the primitives' detection rules.

- **Copies and duplicates out.** The reference goes from 69,387 to 49,748 bytes. The inventory's spans cut are:
  - rules a description already states, such as `validate-frontmatter`'s row selection, `check-artifacts`' family conditions, `discover-rule-files`' judge and rejudge sets, and `write-analysis`' refusals;
  - restatements of a `framework/commands/analyze.md` step;
  - one rule another command applies (*Spec integrity*'s clarify and amend paths).
- **Moved into descriptions** (2,968 bytes). Before it lands in a description, each moved sentence is checked against the code that enforces it.
  - `CHECK_ARTIFACTS` gains 2,319 bytes:
    - the placeholder tokens;
    - the plan-title link rule;
    - *Acceptance-criterion path existence*'s scope, candidate grammar and `root-absent`, `ships-to-adopter` and `gitignored` rules, plus the unbackticked-path note;
    - *Body path existence*'s path reading and history rule;
    - *Line-number citations*' scope;
    - *Link-adjacent decision drift*'s coverage, exemptions and message shape.
  - `CHECK_ARTIFACT_SIZE` gains 248: bytes as a proxy, and that a zero or non-numeric value is refused, so none switches the check off.
  - `WRITE_ANALYSIS` gains 401: `constitution-unresolved` and `constitution-registry-unreadable`, the two reasons it adds itself.
  - The reference keeps one pointer line per section it moves text out of.
- **Kept verbatim.**
  - Family 18's phrase, negator and verb lists, which `scripts/audit/marker-list-parity.sh` reads from this file (071's AC10).
  - *Record fields*, which the constitution's audit-records subsection names as field semantics' home.
  - *Severity tiers*.
  - Every heading `framework/commands/analyze.md` points at, `scripts/audit/template-alignment.sh` enumerates or `check_step_references`' exempt list names.
  - Text the ticked criteria of 016, 033, 045, 046, 060, 071, 084 and 087 name, or that 027's, 000's and 067's scenarios and 068's and 069's specs name ("Which ID format governs a file").
  - *Cross-service references*' class definitions, since `resolve-references`' description defines only `duplicate-repos`.
  - The managed roots under *Project-level consistency*, which `check-orphaned-references`' description does not name.
  - *Line-number citations*' scope line, which 067's AC5 says the reference states.
  - *Finding dispositions*' order line, which the constitution's *Detect, decide, re-check, record* points at.
- **Its cost.** Every `/analyze` run loads all three descriptions (the generated `.claude/commands/ductus/analyze.md`'s `select:` line), so a run reads 2,968 more bytes. No golden holds a description, and `runtime/tests/mcp.rs`' description test compares each tool's text with its subcommand's `long_about`, which share the constant. The 4,096-character bound there applies to `/target`'s always-loaded tools, none of these three.

### The review reference cuts copies only

- **Out** (about 16,600 net), all text the inventory found that a description, the constitution or `framework/commands/review.md` already states, with no description growing.
- **Blocking semantics** keeps its heading and a one-line pointer to §spec-lifecycle's gate, evaluated by `check-review-gate`. So `framework/commands/review.md`'s pointer at that section, `check_step_references`' exempt list and 071's scenario `reference-sections-are-reached-by-a-step` stay true.
- **Kept verbatim.**
  - Text the ticked criteria of 020 (AC16, AC18), 024 (AC8), 071 (AC13), 073 (AC14), 076 (AC6, AC16), 081 (AC4, AC11) and 087 (AC3) name.
  - The `## Load rules` lines `review_reference_names_the_rule_file_mapping` (`runtime/src/interpreter/payload.rs`) reads.
  - Every heading a command, `framework/constitution.md`, `AGENTS.md`, `framework/reference/implement.md`, `framework/reference/analyze.md` or `framework/bootstrap/ductus.md` points at.
- **If it ends over 50,000 for any reason**, 086 task 4's addition to *Window and coverage* included: its four spans whose rule no description states yet (about 2,200 bytes) move into `discover-rule-files`' and `process-waivers`' descriptions, the route the owner chose for analyze.
- ***Window and coverage*'s cut** (2,585 bytes, the largest) rests on `compute-review-scope`'s description, read whole, which states the window's start, its base sources and the attribution rule. One line stating attribution and the artifact set stays (081's AC11).

### The lint governs four rows, two of them globs (AC2)

- **Rows.** `CEILINGS` holds:
  - `AGENTS.md` and `framework/constitution.md` at their computed ceilings, each with the fix it names now;
  - `framework/commands/*.md` and `framework/reference/*.md` at 50,000, whose fix names the routes: retire a copy, move a primitive's rule into its description, or move a rule another command applies into that command's reference (spec 089).
- **Globs.** A row whose path holds `*` is enumerated from the index with `git ls-files -- ':(glob)PATTERN'`, so `*` stops at `/`, and each match is measured as staged (`git cat-file -s :PATH`), as a single path is. A new command or reference file is governed from its first commit with no row added, and a glob matching no indexed file fails with a message of its own, never passing (§design-principles).
- **Comments and messages.** These name 089 beside 072 and state that only the glob rows are `DEFAULT_READ_SIZE_BYTES` (`runtime/src/primitives/check_artifact_size.rs`): the script's header and table comments, `.githooks/pre-commit`'s, and `framework-checks.yml` step (o)'s.
- **The test** (`scripts/tests/test-lint-governing-size.sh`).
  - Every case's tree is seeded with one small command and one small reference file, so no case meets the empty-glob failure by accident. Case A copies every indexed command and reference file.
  - For each of the four rows, a file one byte over fails naming its file, size, ceiling and fix, and a file exactly at the ceiling passes.
  - Two cases are new: a command file the table never names is governed from its first commit, and a glob matching no indexed file fails.
  - Case H now asserts that the glob rows equal `DEFAULT_READ_SIZE_BYTES` and that each governing ceiling is below it and a multiple of 1,000.
  - Each case is shown to fail with its row removed (`QUAL-TEST-001`).
- **Where it runs.** The hook and `framework-checks.yml` already run both scripts on every commit and push, with no path filter.

### 089 states its own cost

It is bound by AC1, so its spec gains a `## Cost` section as 085, 087 and 088 have:

- the lint's added git processes per commit and their wall time, measured at the commit;
- the 3,000-odd bytes each `/analyze` run reads from the grown descriptions;
- that it retires no check, field or primitive, since it widens an existing check's subject.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | AC1's bullet; the cut; the two `/implement` bullets replaced by a pointer to the implement reference |
| `framework/reference/analyze.md` | Modify | Cut to 50,000 or under |
| `framework/reference/review.md` | Modify | Cut to 50,000 or under |
| `runtime/src/schema/descriptions.rs` | Modify | `CHECK_ARTIFACTS`, `CHECK_ARTIFACT_SIZE`, `WRITE_ANALYSIS` take the moved rules |
| `runtime/CHANGELOG.md` | Modify | `[Unreleased]`: the description changes |
| `AGENTS.md`, `.ductus/docs/agents/*.md` | Modify | `/ductus:prune --project`'s confirmed moves |
| `scripts/lint-governing-size.sh` | Modify | Lowered ceilings, glob rows, empty-glob failure |
| `scripts/tests/test-lint-governing-size.sh` | Modify | Per-row over and at cases, new-file and empty-glob cases, H re-aimed |
| `.githooks/pre-commit`, `.github/workflows/framework-checks.yml` | Modify | Comments name the governed set |
| `.ductus/reference/*.md`, `.claude/commands/ductus/*.md`, `.pi/prompts/*.md` | Regenerate | Generator outputs of the reference and command edits |
| `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md` | Modify | `## Cost`; `cross-spec-impact:` gains 063, 066, 067 and 079 |
| `specs/{050,063,066,067,072,075,079}-*/spec.md` | Modify | Blockquoted signposts to 089; 050's AC15 note; 079's AC18 note |
| `specs/050-constitution/plan.md` | Modify | §Classification follows task 1's moves and task 2's re-pointed entry (Family 38) |
| `specs/050-constitution/scenarios/findings-route-by-scope.md` | Modify | Its signpost names the implement reference as the moved bullet's home |
| `specs/060-exec-analyze-assesses-each-loaded-rule/tasks.md` | Modify | Task 15 states its cost (AC1) |

## Trade-offs

- **Splitting the analyze reference, or giving it its own ceiling.** Each keeps every `/analyze` run's load unchanged but needs an amend back-edge. A split also changes 071's one-reference-per-command contract and re-points each section pointer to a moved section; its own ceiling leaves one reference over the read size. Rejected by the owner, 2026-11-27.
- **Moving the review reference's four conditional spans now.** This would add about 2,200 bytes to two descriptions that every `/review` run loads, which the cut does not need unless 086 task 4 pushes the file over.
- **Removing *Blocking semantics*' heading.** It would save about 100 bytes more, at the cost of an edit to `check_step_references`' test, to `framework/commands/review.md` and to 071's scenario.
- **Moving Family 18's lists and *Record fields*.** These are 1,970 bytes the cut does not need. The move would re-aim `marker-list-parity.sh` at a Rust string, reword 071's ticked AC10, and amend the constitution's audit-records subsection, "Field semantics: each command's reference".
- **A ratchet test holding each governing ceiling to its file's rounded size.** AC2 sets the ceiling at the cut, and a ratchet would fail an unrelated commit that shrank a file by 1,000 bytes.
- **Known limitations.**
  - `AGENTS.md`'s and the constitution's end sizes are estimates until the cuts run; their ceilings are computed from the result, not the estimate.
  - 089's AC4 closes only after the other lane's 086 task 8.

## Cross-spec impact

Each discharge is a blockquoted signpost back-linking 089, a blockquote so it adds no `dependencies:` edge and closes no cycle (089 links 050 and 072).

- **050** owns the constitution's content: the cost bullet, the lowered ceiling, and the two `/implement` bullets now a pointer to the implement reference. AC15's note owes 050 one review, before the release tag.
- **072** owns the lint: the lowered ceilings and the two glob rows. Its AC1 and AC10 still hold, since a lower ceiling still keeps each file at most 50,000 bytes and no higher than its size when the check was added.
- **075**: `AGENTS.md`'s ceiling is below the read size and its cut ran through `/prune --project`.
- **079** (added to `cross-spec-impact:`): 089 holds each reference to the read size, which 079's Resolved Question on AC18 rejected (2026-11-16) before the copies were measured.
  - That question gets a signpost.
  - AC18's "later specs' requirements may grow them past it" gets a *Superseded in part* note, worded as the answer 086 task 7 records for 084's correction rule says, or asked of the owner if none is recorded yet.
  - A reworded ticked criterion owes a review, so the note costs one review of 079 (`compute-review-scope` reads it `review-owed`). It lands with or after 086 task 7's notes on 079's AC6 and AC24, so one review of 079 covers all three. That review is owed before the release tag, as every open spec's is.
- **063, 066, 067** (added to `cross-spec-impact:`): own `check-artifact-size`, `write-analysis` and `check-artifacts`, whose descriptions take the moved rules. No behavior changes.
- **The other lane.** 066 is its spec, and 086 task 7 edits 079's spec. So the other lane is told, through `SendMessage`, before this lane edits either and once the edit lands.
- **060** gets a checkbox on its task 15 rather than a declaration. That task runs after 089 closes, so a declaration would hold 089's `done` on later work.
