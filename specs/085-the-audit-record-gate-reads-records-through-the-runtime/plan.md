# 085 — The audit-record gate reads records through the runtime Plan

Implements [085 — The audit-record gate reads records through the runtime](spec.md).

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

A new corpus-wide primitive, `check-audit-records`, reads every `done` spec's review and analyze record by the read and parse of the loaders `check-review-gate` uses, each record file once, holds each never-run set against the committed high-water marks, and reports each finding with its project-relative path and the counts. The adopter CI template's step "Audit-record gate on done specs" runs it and turns its report into annotations, as the step "No code is left unreviewed" does `check-unreviewed-code`'s, so the step parses no frontmatter or markdown. Each property the step's inline python enforced is kept, decision by decision below (`QUAL-DELEG-001`).

## Technical Decisions

### One corpus-wide primitive with no arguments

`check-audit-records` takes no argument: its subject is the corpus under the configured spec root (`[paths] specs-root`). It is an MCP tool and a CLI command, since it runs no project command and writes nothing but its lists file under `.ductus/results/` (§runtime-boundary principle 5). The name follows `check-owed-work`, the other corpus check the template runs.

### The walk is `check-owed-work`'s corpus form

The primitive enumerates with `list_feature_dirs` and `list_untracked_specs` (`runtime/src/primitives/mod.rs`), as `corpus_owed` and `judge` in `runtime/src/primitives/check_owed_work.rs` do, so the corpus is §numbering's one membership rule: a directory without a feature name is not a spec.

- **A link leaving the project.** `validate_feature_argument` (`runtime/src/primitives/mod.rs`) contains each feature directory, walking it for outward links through `validate_feature_path`, so `spec.md`, `review.md` and `analysis.md`, opened under fixed names beneath it, are covered. Its `InvalidPath` is a finding naming the link, never a skip.
- **Untracked, or no `spec.md`.** An untracked spec is skipped with its reason and counted. So is a feature directory holding no `spec.md`, which the step's own walk never enumerated.
- **The spec's frontmatter.** A `spec.md` that cannot be read, or whose frontmatter `frontmatter_status` (`runtime/src/primitives/mod.rs`) cannot read a `status` from, is a finding, since whether it is `done` is unknown.
- **Examined, and done.** Every other spec counts as examined. A `done` one has its records read.
- **An unlistable spec root is an error.** `list_feature_dirs` returns an empty list for a root `read_dir` cannot open, by its doc comment and its `read_dir` arm in `runtime/src/primitives/mod.rs`, so the primitive opens the root itself first. A root that is absent or cannot be listed fails the call naming it, never zero specs examined. With that, the template's derive step no longer resolves the root for the gate.

### Records through the gate's loaders, with a probe for `blocking`

- **Three states.** Each record file is read once by `read_record` and parsed by `parse_record` (`runtime/src/primitives/mod.rs`), the two halves of `load_record`, which `load_review_record` and `load_analyze_record` run, so its `RecordLoad` is the step's three states:
  - `Absent` joins its record's exempt set;
  - `Unreadable` is a finding carrying the loader's reason;
  - `Present` is checked.
- **`last-run`.** A present record whose `last-run` is null, absent or blank once trimmed is a finding. YAML's null spellings (`null`, `Null`, `NULL`, `~`, an empty value) deserialize to `None`, each a `no-last-run` finding in the test `each_record_finding_fails_with_its_condition_and_passes_without_it` (`RECORD_CASES` in `runtime/src/primitives/check_audit_records.rs`). The blank string is checked by the primitive, as the step checked it.
- **`blocking`.** `ReviewBlock` and `AnalyzeBlock` (`runtime/src/schema/primitives.rs`) default an absent `blocking` to `false` (`#[serde(default)]`). So for a present record the primitive also parses the text it read through `parse_record` into a probe holding only `blocking` as an optional raw value, so the file is read once, and an absent key is a finding (absent is not `false`). A probe that does not parse reads as no key, a finding too, never a pass. `blocking: true` is a finding.
- **Messages.** Each finding keeps the step's message and names the command to run.

### The runtime reads the baseline file

- **Where.** The primitive reads `.ductus/audit-record-baseline.txt`, contained by `validate_contained`, a link leaving the project being a finding naming it.
- **Format.** The format is that of the step's `baselines()`, read at `ea691221`: text after `#` is a comment, a line without `:` is ignored, keys are `review-exempt` and `analyze-exempt`, and a key the primitive does not know is ignored.
- **Absent, malformed or unreadable.** An absent file, or a key it omits, holds a mark of zero. A value that is not a whole number, or a file that cannot be read, is a finding naming the file. Its marks read so far stand, and the exempt sets are still counted and reported.
- **Over its mark.** An exempt set over its mark is a finding per exempt spec, on its `spec.md`, saying the completion gate was bypassed and how to raise the mark deliberately, as the step's did.

### The report and the exit status

- **The result.** The result carries the spec root, the examined, `done` and skipped counts, each exempt set with its count, its mark and whether it is over, the baseline's state, and the findings and skipped specs. Past the read size the lists move to its lists file, as `check-owed-work`'s do (`bounded_lists::place` with `ALL_SPECS`).
- **The CLI's report.** As `check-unreviewed-code`'s CLI does (`unreviewed_code_lines` and `emit_result_gated` in `runtime/src/main.rs`), the CLI prints the whole answer on stderr before the result is bounded: one line per finding, with its path and message, then the counts line, one line per exempt set with its mark, and a line saying nothing was examined when no spec was, so the step can annotate that failure too. That holds on a clean run too.
- **The exit status.** The CLI exits non-zero when any finding stands or when no spec was examined.

### The template's step runs the primitive

- **The step.** The step runs `ductus check-audit-records`, prints its report, and maps each finding line to `::error file=` with the project's path from the work tree (`git rev-parse --show-prefix`, as today, spec 059), escaping `%` in the message. The nothing-examined line becomes an error of its own, and so does a check that fails with no line to annotate, naming its exit status. Its exit status is the CLI's.
- **No interpolation.** It holds no `${{ }}` expression, so `scripts/run-workflow-steps.py` can run it, and it parses no frontmatter or markdown.
- **The derive step.** Its spec-root extraction and `specs-root` output go, since the gate was their only reader.
- **The header.** The template's header item 2 says the step runs the primitive.

### The pin moves in the release that ships the primitive

- **Why the pin waits.** The template's pinned runtime predates the primitive, and `DUCTUS_SHA256` is known only once the release's assets publish. So the pin moves in the release that ships the primitive, as it did for 084's two steps (the header of `scripts/tests/test-done-spec-owed-work.sh`).
- **What catches a stale pin.** `scripts/check-runtime-currency.sh` reports a pin behind the latest release.
- **Not a task here.** No task of this spec waits on the tag, since the tag waits on every spec being `done`.

### Tests: the primitive and the step, each finding both ways

- **The primitive's unit tests.** They hold each finding failing with its condition present and passing without it, each of the baseline's states the data model names, an exempt set at and past its mark, an untracked spec, a link leaving the project, a spec root that cannot be listed, and the empty corpus. They run from the repository root and from a project in a subdirectory (`git_fixture::subdirectory_project`). Each behavior has a mutant its test turns red, listed in `scripts/mutation/audit-records-085.py`.
- **The step's test.** `scripts/tests/test-audit-record-gate.sh` copies the template, without its install step, into fixture repositories with this tree's release build on `PATH`, as `scripts/tests/test-done-spec-owed-work.sh` does. It runs the gate step through `scripts/run-workflow-steps.py`, from a project at the root and from one in a subdirectory, the copy setting the job's `working-directory`.
  - It asserts the step ran, never skipped.
  - It asserts each finding fails the step and is annotated on its file with the project's prefix.
  - It asserts the clean corpus passes with the counts reported.
  - It asserts an empty corpus and a spec root that cannot be listed each fail the step with an error of its own, and that a `%` in a message is escaped.
- **Where it runs.** `framework-checks.yml` gains a step running it after the runtime build, since a test that shells out to a repository script is a change to the job that runs it (§design-principles).

### What a run costs, and what it retires (089, AC4)

A run is one `check-audit-records` call. It reads each feature's `spec.md` and the two records of each `done` spec. It is measured over this repository's corpus by wall time (`/usr/bin/time -p`, seconds) and by bytes read (`wc -c` of the files it opens).

It also adds one MCP tool, its description measured in bytes. Claude Code lists only its name in each session's deferred tools and loads its description and input schema when the tool is fetched, as the section "Target's tools loaded with the session (task 9)" of `specs/080-deterministic-runtime-efficiency/research.md` observed for a tool the `target` command does not call, and Pi's bridge registers it with both at load. It retires the step's inline python, 159 lines at `d4fa51ba` between `python3 - <<'PY'` and `PY`, and the derive step's spec-root extraction. It retires no primitive, check or record field, since the step it replaces held its own reader, not one the runtime had.

The measured figures go in the spec's body, where 089's Behavior puts them, beside a blockquoted signpost to 089, which induces no dependency edge (task 3).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/check_audit_records.rs` | Create | The primitive: walk, records, probe, baseline, findings |
| `runtime/src/primitives/mod.rs` | Modify | Declare the module; split `load_record` into `read_record` and `parse_record` |
| `runtime/src/primitives/bounded_lists.rs` | Modify | `IRREGULAR_COUNTS` rows for `examined-count` and `done-count`, counts of no list |
| `runtime/src/schema/primitives.rs` | Modify | Its arguments and result types |
| `runtime/src/schema/descriptions.rs` | Modify | `CHECK_AUDIT_RECORDS`, its one description |
| `runtime/src/schema/registry.rs` | Modify | `PRIMITIVE_REGISTRY` entry |
| `runtime/src/mcp/server.rs` | Modify | Its `#[tool]` |
| `runtime/src/interpreter/mod.rs` | Modify | Its `dispatch_primitive` arm |
| `runtime/src/main.rs` | Modify | Its `Command`, stderr report and exit rule |
| `runtime/tests/result_sizes.rs` | Modify | Its measured `case` |
| `runtime/tests/check_audit_records_cli.rs` | Create | The CLI's lines and exit status, from a run of the binary |
| `framework/runtime-tools.txt` | Modify | The tool's line |
| `framework/bootstrap/configure/claude.md` | Modify | Generated allow block (`scripts/gen-configure-mcp.sh`) |
| `framework/templates/ci/adopter-generators.yml` | Modify | The gate step runs the primitive; the derive step's root extraction goes; header item 2 |
| `scripts/tests/test-audit-record-gate.sh` | Create | The step's test, root and subdirectory |
| `.github/workflows/framework-checks.yml` | Modify | A step running that test |
| `scripts/mutation/audit-records-085.py` | Create | Each behavior's mutant |
| `runtime/CHANGELOG.md` | Modify | The new primitive and the step's change |
| `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md` | Modify | The measured cost and what it retires, with a signpost to 089 |
| `specs/057-analyze-artifact-and-record-relocation/spec.md` | Modify | Signpost back-linking 085 |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Modify | Signpost back-linking 085 |

## Data Model

[data-model.md](data-model.md) defines the primitive's result and the baseline file's format.

## Trade-offs

- **Rejected: an optional `blocking` in the record types.** Making `ReviewBlock` and `AnalyzeBlock`'s `blocking` optional would carry absent-is-not-false into every reader. But it changes the shared types 066 owns and `check-review-gate`'s verdict on a record with no field, outside this spec. The probe reads the key's presence through the same `parse_record`, from the text the record was parsed from.
- **Known limitation: the local gate reads an absent `blocking` as `false`.** So a hand-written record without the field passes the local gate and fails this CI step, which is where such a hand edit is caught, as before this spec.
- **Rejected: a JSON parse in the step.** The template's step "Done specs owe no work" parses `check-owed-work`'s JSON with python. This step maps the CLI's report lines and takes its exit status, as the step "No code is left unreviewed" does, so all its logic lives in the runtime, where its tests are.
- **Rejected: a CLI-only primitive.** Principle 5 leaves off MCP only a primitive that runs a project command, writes outside the project, or writes a host's settings. This one does none of these.
- **Where the reader is narrower than the step's** (`QUAL-DELEG-001`). It differs from the step's parse, read at `ea691221`, in four places:
  - A directory without a feature name is no longer read, because §numbering holds one membership rule. In an adopter that is `specs/templates/`, whose `spec.md` is a template.
  - An untracked spec is skipped, as `check-owed-work` skips one. A CI checkout holds none.
  - A quoted `"null"` `last-run` is a string, as the gate reads it, where the step's parse stripped the quotes.
  - A blank `last-run` stays a finding.
- **Where it is stricter.** A `spec.md` whose frontmatter names no `status`, or does not parse as spec frontmatter (`frontmatter_status` reads it whole, so a list field holding a scalar is a finding), and a directory holding a link leaving the project are findings, and a spec root that cannot be listed is an error.

## Cross-spec impact

- **057** owns the step's properties (its AC17). AC17 stays true, since the step still reads the two records and bounds the exempt set with a committed mark. A blockquoted signpost in 057's body back-linking 085 discharges it, leaving 057 `done` (task 3).
- **066** owns the audit records and their loaders. `load_record` splits into its read and parse halves, `read_record` and `parse_record`, which it composes, so its callers see no change, and the halves gain a caller. A blockquoted signpost in 066's body back-linking 085 discharges it (task 3).
- **089**'s AC4 binds 085: the cost statement and a signpost back-linking 089 in 085's body (task 3).
