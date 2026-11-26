# 088 — Rule judgments cost only what changed Plan

Implements [088 — Rule judgments cost only what changed](spec.md).

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

Three changes, two of them through 082's one currency decision, `rule_verdicts::standing` (`runtime/src/primitives/rule_verdicts.rs`):

- **AC1.** `write-analysis` counts, in the pass that already sorts every assessable rule into judged, carried and unjudged, the rules its run judged and the verdicts it carried, and writes both into the record.
- **AC2.** `standing` compares a changed rule file with its copy at the record's `analyzed-against`, one rule at a time, once that copy is proven to be the text the run judged. So an edit to one rule re-judges that rule and each rule naming it, and a copy it cannot prove re-judges the whole file, naming why.
- **AC3.** A corpus-wide primitive, `backfill-section-digest`, writes `section-digest` into each record proven current, and a migration runs it once per repository.

## Technical Decisions

### The narrowing lives in `standing`, which every caller shares

`standing` is the decision its three callers must not disagree on, by its module doc:

- `discover-rule-files`' `judge` (`verdicts_block` in `runtime/src/primitives/discover_rule_files.rs`);
- the exec walker's skip (`runtime/src/interpreter/payload.rs`);
- `write-analysis`' carry (`derive_rule_fields` in `runtime/src/primitives/write_analysis.rs`).

Narrowing `discover-rule-files` alone would leave `derive_rule_fields` putting each rule still `Stale` that the host did not judge into `unjudged-rules`, which holds `done` at the gate's `unjudged-rules` check. `LoadedRuleFile` gains the file's path from the project root, set where each caller builds one, and `standing` takes the project root.

### A changed rule file is compared rule by rule at `analyzed-against` (AC2)

- **When.** Once the loaded rule-file set is the one the record's `rule-digest` names, for a file whose digest, by `section_digests` (`runtime/src/primitives/rule_sections.rs`), differs from its entry. Until 088 every rule of such a file is `Stale`.
- **The copy.** The file's text is read from the commit the record's `analyzed-against` names:
  - the repository opened by `ProjectRepository::discover` from the project root;
  - the sha resolved by `revparse_single` on the trimmed value, peeled to its commit, as `exempt_renames` (`runtime/src/primitives/analyze_subjects.rs`), that field's one reader before 088, resolves it;
  - the commit's tree read by `blob_text` at `to_git` of the file's project-relative path, as `ProjectRepository::read_at_head` reads one at `HEAD` (`runtime/src/primitives/mod.rs`).

  None of the three changes. Every one of the 79 records at `1ffaf951` holds a full 40-character sha, by the length of each record's `analyzed-against` value, the form `write-analysis` is given, and the fixtures use that form. A lookup that finds no such object is `commit-not-held`; any other failure is `history-unreadable`.
- **The guard.** The copy stands in for the text the run judged only when its own file digest equals the record's entry for the file. Two facts make the guard necessary:
  - `analyzed-against` is a caller argument, written verbatim (`args.analyzed_against` in `write_analysis.rs`);
  - an analysis may run before its commit (§grounding, "An analysis record is judged by a digest of what it read, so it may precede its commit").

  So only the digest ties that commit's copy to what was judged. Without it, a run that judged uncommitted rule text, later reverted, would carry verdicts no run gave the text that stands.
- **Per rule.** Both copies are parsed by `parse_rule_sections`. A rule's own text is the fields `section_digests` hashes for it, the same ones its file digest hashes: its ID, which is its whole heading, its Statement, and its Verification with whether it has one.
  - A rule whose own text is unchanged stands as a rule of an unchanged file stands, subject to 082's other conditions: the sections, `unjudged-rules`, and both files for a rule two files define.
  - A rule whose own text changed, or whose ID the old copy lacks (a new rule, or one moved in from another file), is `Stale`.
  - A file whose digest moved while no rule's text did, a reorder or a removal, keeps every remaining rule's verdict, unless the next bullet holds for it.
- **The rules naming one.** A loaded rule is `Stale` when its Statement or Verification names a changed rule, in any loaded file (the operator's decision, 2026-11-26, in the spec's Resolved Questions). A changed rule here is one whose own text changed, one new to its file, one its file no longer holds, or any rule of a file whose copy cannot stand in.
  - A name is a token matching the rule-ID grammar `rule_sections` reads headings by, other than the rule's own ID, that names a changed rule: a loaded one, or one the changed file held at `analyzed-against`.
  - Only direct names count: a rule naming one only through another reads that other's text, which did not change.
  - Each `AssessableRule` carries the names its Statement and Verification hold, taken when `LoadedRuleFile::read` parses the file, so the scan reads no further file.
- **When the copy cannot stand in.** Every rule of the file is `Stale`, as before 088, and so is each rule naming one of them. `standing` names the file with one of the closed set of reasons in [data-model.md](data-model.md), and `discover-rule-files` reports it under `verdicts.rejudged-files`. A shallow clone that lacks the commit reads as `commit-not-held`.
- **The stale doc comments.** `AnalyzeBlock::analyzed_against` (`runtime/src/schema/primitives.rs`) says it is "Read for exactly one thing", which this makes two. Three other texts state the per-file rule, and each says what is now true:
  - point 3 of `rule_verdicts`' module doc;
  - `NoReuse::RuleFilesChanged`'s "keeps the per-file rule";
  - `DISCOVER_RULE_FILES`' "a file whose content alone changed makes only its own rules non-current" (`runtime/src/schema/descriptions.rs`).

### `write-analysis` records the two counts (AC1)

- **Where they are counted.** `derive_rule_fields` walks `standing.rules()`, one entry per distinct assessable rule, and sends each into a stored or carried verdict, nothing (not fired) or `unjudged-rules`.
  - A rule in the run's `judged` set (`judged-rules` and the `rule-verdicts` keys) counts toward `rules-judged`. The exception is an `addressed` verdict whose citation does not resolve, which goes to `unjudged-rules`.
  - A rule owed a judgment against the changed sections only, and judged, counts as judged.
  - A rule not judged whose verdict stands counts toward `rules-carried`: a `Fired` one, except a `waived` one whose entry is gone, and a `NotFired` one.
  - So the two counts and `unjudged-rules` partition the assessable rules, as the spec's Edge Cases say.
- **How they are written.** `render_rule_fields` writes both on every record, `0` when no rule loaded. The exec path agrees, since its tally passes `judged-rules` (`runtime/src/interpreter/analyze_tally.rs`).
- **No reader.** `AnalyzeBlock` does not refuse an unknown key (its serde attributes carry no `deny_unknown_fields`) and no gate reads the counts, so the type gains no field for them.
- **The schema.** By 082's AC4 precedent the fields join the constitution's analyze-record row (§text-first-artifacts), and 066's data model points at [data-model.md](data-model.md).

### The backfill is one corpus-wide primitive (AC3)

- **The walk.** `backfill-section-digest` walks `list_feature_dirs`, the corpus form `check-audit-records` uses. Its outcomes per record are those [data-model.md](data-model.md) names.
- **Pinned records.** Its one argument, `pinned`, lists the paths from the project root that the project pins, and a record named there is reported and left untouched. A pinned file is opted out of framework writes, and the migration is one: both backfill precedents, `framework/migrations/criterion-label-backfill.md` and `framework/migrations/audit-record-relocate.md`, skip a pinned spec. The runtime holds no reader of `[pinned] files`, which the host reads and passes to `apply-manifest` and `enforce-manifest` (`framework/bootstrap/ductus.md`, the walker context's pinned-list), so the procedure passes it here the same way.
- **The match.** A record matches as it stands by `design_subjects_match` (`rule_verdicts.rs`). Failing that, it tries each other status of `ALLOWED_STATUSES` in order, digesting the design subjects with `spec.md`'s text replaced by the splice `set-status` writes (`split_frontmatter_with_offset` and `locate_status_field`, `runtime/src/primitives/set_status.rs`).
- **One digest path.** `analyze_subjects` exposes its per-file hash and `section_digest` with `spec.md`'s text supplied by the caller. So the backfill and every reader digest the same text the same way, never by a second copy of the walk (`QUAL-DELEG-001`).
- **The write.** The `section-digest:` block comes from the one renderer `write-analysis` uses, moved out of `render_rule_fields`, and is spliced before the frontmatter's closing fence. It is guarded as `set-status` writes: `guarded::read_required`, then `write_guarded`, refused when the file changed since it was read.
- **The surface.** It is an MCP tool and a CLI command, since it writes only inside the project (§runtime-boundary principle 5). The CLI exits non-zero on a refused write, which a second run repairs.

### The migration and this repository's records

- **The entry.** `framework/migrations/section-digest-backfill.md` is registered in `framework/migrations.toml` at `introduced_in` 0.65.0, the release after the `0.64.0` that `version` names, where 084's entry and 087's sit.
  - Its id sorts after `report-project-file-links-into-docs` on the tie-break. A `--ref=main` adopter whose `last_applied` is that entry still runs it, as that entry's registry comment says for its own predecessor.
  - It declares no `sunset_after`. Its `target_paths` are `specs/*/analysis.md`.
- **The procedure.**
  - **Run.** It runs `.ductus/bin/ductus backfill-section-digest` through the CLI with a `--pinned` per `[pinned] files` entry, never the MCP tool, for the reason `framework/migrations/report-project-file-links-into-docs.md` gives: the server keeps the binary it started with.
  - **Not checked.** A call that fails, or a binary with no such subcommand, is reported as not checked, naming the command, never as nothing to backfill.
  - **Report.** It reports the count written and names each record left, which gains the digest at its spec's next analysis.
  - **Status.** It leaves each `done` spec `done`, since it writes only `analysis.md` and leaves no unchecked box or unticked criterion (§spec-lifecycle).
- **The seed.** The claude row's `settings_template` gains `Bash(.ductus/bin/ductus backfill-section-digest *)` and its `.exe` twin, in `framework/bootstrap/ductus.md`, its byte-identical copy `framework/bootstrap/govern.md` and `install.sh`'s copy, since `framework/bootstrap/ductus.md` §Permission Setup seeds one rule per CLI subcommand the bootstrap and its migrations run (the agent's recommendation, adopted by the operator, 2026-11-26).
- **Exercised.** A scratch-adopter bootstrap run exercises the entry, as AGENTS.md requires for a change under `framework/migrations/` (`.ductus/docs/agents/developing-ductus.md`, Exercising an installer change).
- **This repository.** A task runs the primitive over this corpus and commits the records it writes, as `framework/migrations/criterion-label-backfill.md` records that this repository was swept when its primitive landed.

### Tests: each behavior fails with its condition removed

- **AC1.** A fixture run carries one verdict and judges one rule, and records `rules-judged: 1` and `rules-carried: 1`. A carried not-fired rule counts as carried, an unresolved `addressed` counts in neither, and the counts and `unjudged-rules` partition a mixed run.
- **AC2.** Git fixtures run two analyses, with a commit between them that edits one rule's Verification:
  - `judge` holds that rule and each rule naming it, in its own file and in another, alone, and `write-analysis` carries the other rules rather than listing them unjudged;
  - a rule naming an unedited rule, or the edited one only through another rule, keeps its verdict;
  - each rule naming a rule of a whole-file re-judge, or one the edit removed, is judged;
  - a shallow clone whose boundary lies past `analyzed-against` re-judges the whole file, naming `commit-not-held`;
  - `judged-copy-differs`, in both directions;
  - a record with no `analyzed-against`;
  - a reorder keeps every verdict, and a rule moved in from another file is judged;
  - a project in a subdirectory (`git_fixture::subdirectory_project`).
- **AC3.**
  - A record matching as recorded gains the digest.
  - One matching at another status gains the digest of that status's text, equal to `section_digest` of the spec at that status.
  - A stale one is untouched and named, and an unreadable one untouched.
  - A second run writes nothing.
  - The written record, with the block removed, is byte-identical to the original.
  - A changed record is refused.
  - A pinned record is reported and untouched.
  - The project-in-a-subdirectory case runs too.
- **Mutants.** Each behavior has a mutant its test turns red, listed in `scripts/mutation/rule-judgments-088.py`.

### What a run costs, and what it retires (089, AC4)

Measured at a commit on `origin/main` once the code lands, in these units:

- **AC1.** The two record lines' bytes, by `wc -c` over the records a run writes.
- **AC2.**
  - Judgments owed per rule-file commit. Over the 22 commits that touched `framework/rules/` after `a3629a95`, 082's per-file rule owed 1,009, and the per-rule one with each rule naming a changed one 104. That figure was measured at `1ffaf951` by dumping `discover-rule-files --all --rules --verification` at each commit and its parent in a clone.
  - The added read: one commit lookup and one blob read per changed rule file per call, timed by `/usr/bin/time -p`.
  - The tally of `rejudged-files` reasons over this corpus, since one dominant reason would be an AC2 defect, not a cost.
- **AC3.**
  - The tool's description and input schema in bytes, by `scripts/measure-result-sizes.sh`.
  - A run's wall time over this corpus and the records it writes.

It retires no primitive, check or record field. The counts hold what no field held, since `judged-rules` is an argument never stored. AC2 replaces a rule, not a field. The backfill runs once per repository, and no existing primitive writes a record field outside a run. The figures go in the spec's body beside a blockquoted signpost to [089](../089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md), which induces no dependency edge.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/rule_verdicts.rs` | Modify | `standing` takes the project root; the copy at `analyzed-against`, its guard, the per-rule comparison and the reasons; `LoadedRuleFile`'s path; module doc |
| `runtime/src/primitives/discover_rule_files.rs` | Modify | Its `LoadedRuleFile` paths; `verdicts.rejudged-files`, bounded |
| `runtime/src/interpreter/payload.rs` | Modify | Its `standing` call |
| `runtime/src/primitives/write_analysis.rs` | Modify | The two counts; the `section-digest` renderer moved out for the backfill; its `standing` call |
| `runtime/src/primitives/analyze_subjects.rs` | Modify | The per-file hash and `section_digest` with `spec.md`'s text supplied |
| `runtime/src/primitives/backfill_section_digest.rs` | Create | The primitive |
| `runtime/src/primitives/mod.rs` | Modify | Declare the module |
| `runtime/src/schema/primitives.rs` | Modify | The backfill's arguments and result; `verdicts`' `rejudged-files`; `analyzed_against`'s doc comment |
| `runtime/src/schema/descriptions.rs` | Modify | `DISCOVER_RULE_FILES`, `WRITE_ANALYSIS`, and the backfill's description |
| `runtime/src/schema/registry.rs` | Modify | `PRIMITIVE_REGISTRY` entry |
| `runtime/src/mcp/server.rs` | Modify | Its `#[tool]` |
| `runtime/src/interpreter/mod.rs` | Modify | Its `dispatch_primitive` arm |
| `runtime/src/main.rs` | Modify | Its `Command` and exit rule |
| `runtime/tests/result_sizes.rs` | Modify | Its measured `case` |
| `framework/runtime-tools.txt` | Modify | The tool's line |
| `framework/bootstrap/configure/*.md` | Modify | Generated allow blocks (`scripts/gen-configure-mcp.sh`) |
| `framework/migrations/section-digest-backfill.md` | Create | The procedure |
| `framework/migrations.toml` | Modify | Its entry |
| `framework/bootstrap/ductus.md` | Modify | The claude seed gains the backfill's two entries |
| `framework/bootstrap/govern.md` | Modify | The same, byte-identical to `ductus.md` |
| `install.sh` | Modify | Its copy of the claude seed gains them |
| `framework/constitution.md` | Modify | The analyze-record row gains `rules-judged` and `rules-carried` |
| `scripts/mutation/rule-judgments-088.py` | Create | Each behavior's mutant |
| `runtime/CHANGELOG.md` | Modify | The counts, the narrowing, the primitive and the migration |
| `specs/*/analysis.md` | Modify | This corpus's records the backfill writes |
| `specs/088-rule-judgments-cost-only-what-changed/spec.md` | Modify | The cost and what it retires, beside a signpost to 089 |
| `specs/082-rule-assessments-converge-across-runs/data-model.md` | Modify | Relations and the `verdicts` section state the per-rule rule and point here |
| `specs/082-rule-assessments-converge-across-runs/spec.md` | Modify | The per-file Resolved Question annotated as superseded; a signpost back-linking 088 |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Modify | `write-analysis`' rule assessments gain the two counts |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Modify | Signpost back-linking 088 |
| `specs/050-constitution/spec.md` | Modify | Signpost back-linking 088 |

## Data Model

[data-model.md](data-model.md) defines the two record fields, `discover-rule-files`' `rejudged-files` and its reasons, and `backfill-section-digest`'s walk, write and result.

## Trade-offs

- **Rejected: a rule's own text alone.** Over the 22 commits above it would have owed 59 judgments, against 104 with the rules naming a changed one. But it keeps the verdict of a rule whose Verification rests on a changed rule's content, which 4 of the first 12 names in `framework/rules/accessibility-frontend.md` do (the spec's Resolved Questions).
- **Known limitation: only a direct name counts.** A rule naming another only through a third is not judged again. The third's own text, which the first reads, did not change.
- **Rejected: trusting the copy at `analyzed-against` without the digest guard.** It saves no read, and it carries verdicts over rule text no run judged whenever an analysis ran before its commit.
- **Rejected: narrowing in `discover-rule-files` alone.** `write-analysis` would then list each narrowed rule the host did not judge as unjudged, and the gate blocks `done` on that list.
- **Rejected: a backfill per feature.** A per-feature primitive, as `relocate-audit-records` is, costs a migration one call per spec, 79 records in this corpus, where one corpus call reports the same outcomes.
- **Rejected: a backfill mode on `write-analysis`.** It records a run and refuses to record before every task is done (spec 079), and a backfill is no run.
- **Rejected: ignoring `[pinned] files`.** `write-analysis` does not read it, but it writes a run, while the backfill is a migration's framework write, which both backfill precedents keep off a pinned file.
- **Rejected: the runtime reading `[pinned] files` itself.** No primitive reads it, and the host already passes it to the manifest primitives, so a second reader would be a copy of that read.
- **Known limitation: a record matches only when `spec.md`'s `status:` alone moved.** Any other edit since its analysis leaves the record to gain the digest at its spec's next analysis: 36 of the 49 records without one at `1ffaf951`.
- **Rejected: a third count splitting full judgments from narrowed ones.** AC1 records two counts, and the narrowed share is what 088's Motivation derived from the reuse rule (2,402 of 11,364 judgments).

## Cross-spec impact

- **050** owns the constitution's contents. Its analyze-record row gains `rules-judged` and `rules-carried`, in the change that writes them (task 1). The constitution held 49,869 bytes at `1ffaf951` by `wc -c`, 131 under the 50,000 that `scripts/lint-governing-size.sh` holds it to, and the other lane edits it, so the size is measured again at landing. A blockquoted signpost in 050's `spec.md` back-linking 088 discharges it (task 5).
- **066** owns the record's schema and `write-analysis`' contract. Its data model's rule-assessment bullet under `write-analysis` gains the two counts by [data-model.md](data-model.md), in task 1. Nothing binds the written keys to that bullet or to the constitution's row (no test under `runtime/tests` or `scripts/audit` names them). A signpost in 066's `spec.md` discharges it (task 5).
- **082** owns verdict currency and `discover-rule-files`' `verdicts` (AGENTS.md: "a feature-specific primitive lives with its feature").
  - Its data model's Relations ("a verdict is current while its rule's `rule-digest` entry matches the file") and its `discover-rule-files` section state the per-rule rule and point here for `rejudged-files`.
  - Its Resolved Question keeping the hash "per rule file, so a rule an upgrade edits is judged again with the rest of its file" is annotated as superseded by spec 088, named in prose: a link there would induce an edge from 082 to 088, a cycle with 088's own dependency on 082.
  - These land in the change that makes the per-file rule false (task 2), and a signpost in its `spec.md` discharges it (task 5). They change a data model and prose only, so 082 stays `done`.
- **060** owes nothing. Its walker asks about each rule "without a current stored verdict (spec 082)", which stays true as currency narrows.
- **022** owes nothing. Its data model points at 082's for the `verdicts` block (082's plan, Cross-spec impact), and `ProjectRepository` and `blob_text` are called, not changed.
- **089**'s AC4 binds 088: the cost statement and a signpost back-linking 089 in 088's body (task 6).
