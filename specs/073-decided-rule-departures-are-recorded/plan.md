# 073 — Decided Rule Departures Are Recorded Plan

Implements [073 — Decided Rule Departures Are Recorded](spec.md).

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

One shared reader, `runtime/src/primitives/departures.rs`, parses a spec's departure entries under `## Applicable Rules`, validates their structure, and derives each entry's `decided-by` and `decided-at` from git history. Four consumers call it, so none restates the grammar or the attribution rule: a new primitive `check-departures` (AC1, and the host's list of entries), `write-analysis` and the exec walker's tally (AC2 to AC4), and `write-review` with `process-waivers` (AC5, AC8).

At analyze, a rule an entry names is still judged for whether its trigger fires, as any rule is, but a fired outcome is recorded as a third rule-verdict shape, `waived`, never `finding`, so no finding exists to count or to refuse. At review, `write-review` turns each entry into per-file waivers for the files its rule fires at, marked as departures so that `process-waivers` expires them when the entry is gone.

## Technical Decisions

### The entry grammar is a nested bullet

Decided by the owner, 2026-11-12, of three options (nested bullets, one line, a fenced YAML block); the grammar is **§Departure entry** in `data-model.md`. An entry is a list item under `## Applicable Rules` whose text is the literal key `departs:` and one backticked rule ID. Its children are `reason:` (required, non-empty) and `cited:` (optional, one `§anchor` reference); any other child key is malformed. HTML comments and fenced code are skipped, so the template's commented example is no entry and passes by default (§drift-prevention, its template-rule alignment). A `§` citation, not a markdown link, keeps the entry from creating a `dependencies:` edge (§text-first-artifacts). Measured 2026-11-12: 9 of the 76 specs carry `## Applicable Rules`, holding 20 bullets, all `` - `ID` — reason ``; none opens with `departs:`, so no existing citation reads as a departure.

### One reader, one attribution rule, four consumers

`departures.rs` exposes the entry parser (rule, reason, cited, line, structural problems) and the attribution. One module follows the pattern where a shared predicate is the only decision several callers consult: `runtime/src/primitives/rule_verdicts.rs` (its header states the callers must not disagree) and `runtime/src/primitives/commit_attribution.rs`. Repository access goes through `ProjectRepository::discover` with `to_git` and `to_project`, since the project root is not the work tree's root. The module is `pub mod` with `pub` items taking the project root as a `&Path`, as the primitives' `run` functions do, so no consumer-less item reads as dead code before task 2 calls it and the pub(crate) `ProjectRepository` stays inside it. `parse(content)` returns `entries` (the well-formed ones, so a malformed entry waives nothing) and `problems` (a line and a kind: no rule ID, a second rule ID, no or empty `reason:`, another or a repeated child key, and a rule two entries name). **A rule two entries name is reported by the reader**, as `DuplicateRule` on the later entry, with both entries kept in `entries`; `check-departures` turns each problem into its Blocking finding and so owns no second duplicate check. A `cited:` child with no text names no citation. A reason wrapped onto a continuation line, indented or lazily unindented as CommonMark renders it (directly after the child's text, no blank line between), is joined with a space. `attribute(project_root, spec_rel, rules)` returns each rule's `Attribution`, `Attributed{decided_by, decided_at}` or `Unattributed`; the walk follows first parents from `HEAD`, so a merge's side branch is not part of the run, and a blob already read is not parsed again.

### Attribution is the first commit of the entry's unbroken run

`git blame` names the commit that last touched a line, and the Edge Cases keep an edited entry's introducing author and date. The reader walks commits from `HEAD` (`revwalk`), parses each revision of `spec.md` (`tree.get_path`), and takes, per rule, the oldest commit of the unbroken run ending at `HEAD` in which an entry for that rule exists; an entry removed and added back is introduced by the re-adding commit. `decided-by` is the commit author's email, the identity `write-review` already records as `waived-by` from `git config user.email`; `decided-at` is the author time in UTC to the second.

`decided-by: unattributed`, a state of its own with `decided-at` absent, when the repository is shallow (`Repository::is_shallow`, as `paths_in_history` in `runtime/src/primitives/check_artifacts.rs` refuses to answer on one, and as `oldest_bullet_date` in `runtime/src/primitives/inbox_standing.rs` documents a shallow boundary reading as a confident answer), when there is no repository, when the entry is absent from `HEAD`'s `spec.md` (not yet committed), or when a revision cannot be read. A spec directory rename is followed with `Diff::find_similar` on the commit that adds `spec.md`, and only within one spec: the renamed-from directory must parse (`parse_feature_dir`) to the same `FeatureForm` as the new one, since similarity detection pairs any deleted spec with a similar added one and a spec number is never reissued. Where none is found, or the pair is another spec, the add is the introduction. The walk follows first parents, so an entry brought in by a merge is introduced by the merge commit and its side branch's authors are not named. Fixtures in `departures.rs` prove each.

### `check-departures` is a new primitive, owned by this spec

`write-analysis` refuses a `blocking-findings` count below the MUST-tier `Finding` verdicts the record holds (`derive_rule_fields` in `runtime/src/primitives/write_analysis.rs`), so the host must know which rules carry an entry before it answers steps 11 and 12, and `check-artifacts`' result carries findings and skipped targets, never a list of entries (`CheckArtifactsArgs`, `CHECK_ARTIFACTS`). `check-departures` takes `feature` and returns `entries` (rule, tier, reason, cited, decided-by, decided-at), `findings` (family `departure-entries`, tier, message, path) and `unexamined` (a citation it could not judge, reason `anchor-target-unjudgeable` or `anchor-target-unreadable`, never a pass). It judges each `cited:` through `resolve_anchor::run` with `markers-path` set to `paths::constitution_path(repo)`, as the exec walker already resolves the constitution (`runtime/src/interpreter/mod.rs`), keeping the results on entry lines. A feature-specific primitive lives with its feature (AGENTS.md). It is registered at the seven sites `docs/developing-ductus.md` §Adding or removing a runtime primitive lists, with its name in `framework/runtime-tools.txt`.

### Finding tiers

Decided by the owner, 2026-11-12: a missing `reason`, an unknown or retired rule ID, and a rule two entries name are **Blocking**; a deprecated rule and an unresolvable citation are **Advisory**. This matches step 5 of `framework/commands/analyze.md` (`check-rule-ids`: a missing ID blocks, a deprecated one is advisory) and step 4 (an unresolved `§` reference is advisory). An entry naming a rule the project never installed (`check-rule-ids`' `not-installed`) is recorded unexamined under `rule-not-installed`, as step 5 records the citation. A malformed entry waives nothing, so its rule is assessed as any other.

`check-rule-ids` scans every ID token in the body (`citation_regex` in `runtime/src/primitives/check_rule_ids.rs`), so it reports an entry's unknown or deprecated ID too, and one defect yields two findings, one naming the ID and one the entry. Both are fixed by one edit; teaching `check-rule-ids` to skip entry lines would add a file and a grammar copy for no change in what blocks.

### `waived` is a third verdict shape, which the host cannot fabricate

`RuleVerdict` (`runtime/src/schema/primitives.rs`) is `Addressed{cite}` or `Finding{tier, at, message}`; it gains `Waived{reason, decided-by, decided-at?, cited?}`. The host supplies the bare `{verdict: waived}` for a rule `check-departures` listed whose trigger fired, since firing is the host's semantic judgment (steps 11 and 12). `write-analysis` fills the fields from the entry, refuses `waived` for a rule with no entry, and refuses `finding` or `addressed` for a rule with one: a refused write leaves no record, so the host corrects it, and nothing depends on its remembering (§design-principles). The exec walker does the same in `AnalyzeTally::record_assessment` (`runtime/src/interpreter/analyze_tally.rs`): for a rule with an entry, any fired answer records `Waived` and counts into no tier, and a pass records nothing, as 082's data model defines a rule that does not fire. A rule that fires and is addressed is still `waived` (Edge Cases).

A stored `waived` verdict is reused like any fired verdict while its subjects are unchanged (`Currency::Fired` in `rule_verdicts.rs`), and `spec.md` is a subject, so an entry edit makes every verdict stale. Attribution depends on history rather than on a digest, so `write-analysis` recomputes a carried `waived` verdict's fields on every write: a record written `unattributed` before the commit is attributed after it. Every exhaustive match on `RuleVerdict` gains an arm: `rule_verdicts.rs` (the currency and `contradiction`), `interpreter/mod.rs` (the reused-findings filter), `write_analysis.rs` (the body renderer and the finding floor) and `analyze_tally.rs`.

### The record and the report list waived rules apart

`write-analysis` renders a `## Waived rules` section in `analysis.md`'s body, MUST-tier rules first (the tier from `AssessableRule` in `rule_verdicts.rs`), and its result gains `waived`, bounded as `contradictions` is (`bounded_lists::place`: `waived-count` inline, `lists-file` past the read size). Step 22 of `framework/commands/analyze.md` reports it before the passed rules and the advisory findings. No count field joins the record (Resolved Questions). A waived rule is not a finding, so `dispositions.waived` is untouched and 058's AC30 and the refusal of a waiver on a blocking finding stand.

### Review: `write-review` derives the waivers, `process-waivers` expires them

`new_waiver_entries` (`runtime/src/primitives/write_review.rs`) already turns a `--waive` (`NewWaiver{rule, reason}`) into one entry listing the files the rule fires at that no waiver holds. It gains a second source: each well-formed entry, of any tier, becomes a new waiver for its rule, so AC8's SHOULD takes the same path as a MUST. The stored waiver carries `waived-by` and `waived-at` from the entry, its `reason`, and `departure: true` in the open-schema `extra` map, which survives verbatim (`framework/reference/review.md` §Waivers). Each path still applies, expires and is retained on its own, so a file added later where the rule fires is covered with no second decision; 020's rule that a waiver never extends to another location gains one exception sentence, that a departure is bounded by the spec's own scope. An `unattributed` entry stores `waived-by: unattributed` and `waived-at` the run's own time, since `process-waivers` treats a blank `waived-at` as malformed (`REQUIRED_FIELDS` in `runtime/src/primitives/process_waivers.rs`). A malformed entry derives no waiver, and the run says so.

Without the marker, a removed entry would leave its waivers applying while the rule fires, which the Edge Case "an entry removed" forbids. `process-waivers` reads the entries through `departures.rs` and expires a marked waiver whose rule has no entry, with its existing `waiver expired:` notice. A hand `--waive` on the same rule and file stays the first claim (`framework/reference/review.md` §Malformed and duplicate waivers).

### The constitution gains one bullet and loses two restated sentences

`framework/constitution.md` is 49,831 bytes against the 50,000-byte ceiling `scripts/lint-governing-size.sh` enforces. Finding dispositions gains, before "Detect, decide, re-check, record." (176 bytes):

> - **A departure entry waives its rule before any finding exists.** Analyze records the rule `waived`, never failing, apart from those that passed; review reads the same entry.

and "Three dispositions" loses " The inbox is not a disposition. A question is deferred with its trigger." (73 bytes). Both are copies: §design-principles says "The inbox is none of the three", and §spec-requirements that a deferred decision "is resolved *with* its trigger". Nothing asserts either sentence (grep of the live artifacts, 2026-11-12). Result: 49,934 bytes. The analysis-record table's `rule-verdicts` row already reads "map of rule ID → verdict" and needs no edit; AC6's record row is the `rule-verdicts` bullet of `framework/reference/analyze.md` §Record fields, 066's data model and `docs/analyze.md`.

### The test that reads history runs in the job that has it

AC7's unit tests build their own repositories in a temp directory, so they run in `runtime.yml`'s depth-1 matrix, whose comment says no test there reads this repository's history. The full-depth job AC7 names is `framework-checks` in `.github/workflows/framework-checks.yml`, which checks out with `fetch-depth: 0`, has no `paths:` filter and builds the release binary. A new step runs `scripts/tests/test-departure-attribution.sh`: it commits to a repository it builds, runs the built `ductus check-departures` for the attributed, edited, uncommitted and re-added cases, runs it against a `git clone --depth 1` of that repository for the shallow case, and fails loudly when its own checkout is shallow (§design-principles: a check that reads history must be given it).

### Every agent

The runtime and the reference prose are agent-neutral, and the command sources are generated per layout. Walking `framework/bootstrap/ductus.md` §Derived values' Layout-derived rows: the edited steps name commands by `{project}`, so both layouts regenerate from one source; the new tool's allow entry is generated into `framework/bootstrap/configure/claude.md` by `scripts/gen-configure-mcp.sh`; pi gates no tool and its bridge (`framework/bootstrap/pi/ductus-bridge.ts`) lists tools through `tools/list`, checked with `node scripts/tests/pi-bridge-harness.mjs`; the native rules file is unchanged. The `analyze-basic` and `review-basic` goldens move with the command sources and are re-blessed with the diff read.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/departures.rs` | Create | Entry reader, structural validation, attribution |
| `runtime/src/primitives/check_departures.rs` | Create | The `check-departures` primitive (AC1) |
| `runtime/src/primitives/mod.rs` | Modify | Register both modules |
| `runtime/src/schema/primitives.rs` | Modify | `RuleVerdict::Waived`; `check-departures` arguments and result; `write-analysis`' `waived` |
| `runtime/src/schema/registry.rs`, `runtime/src/schema/descriptions.rs`, `runtime/src/mcp/server.rs`, `runtime/src/main.rs`, `runtime/src/interpreter/mod.rs`, `runtime/tests/result_sizes.rs` | Modify | The registration sites for `check-departures` |
| `runtime/src/primitives/rule_verdicts.rs`, `runtime/src/interpreter/analyze_tally.rs` | Modify | `Waived` currency, contradiction, exec tally |
| `runtime/src/primitives/write_analysis.rs` | Modify | Derive and refuse `waived`; `## Waived rules`; result list |
| `runtime/src/primitives/write_review.rs`, `runtime/src/primitives/process_waivers.rs` | Modify | Departure waivers and their expiry (AC5, AC8) |
| `runtime/CHANGELOG.md` | Modify | `[Unreleased]` entry |
| `scripts/mutation/departures.py` | Create | A mutant per refusal and per attribution state |
| `scripts/tests/test-departure-attribution.sh`, `.github/workflows/framework-checks.yml` | Create, Modify | AC7 over real history, a depth-1 clone included |
| `framework/runtime-tools.txt`, `framework/bootstrap/configure/claude.md` | Modify | Tool name; generated allow entry |
| `framework/commands/analyze.md`, `framework/commands/review.md` | Modify | The `check-departures` step; steps 11, 12 and 22; the review waiver step |
| `framework/reference/analyze.md`, `framework/reference/review.md`, `docs/analyze.md` | Modify | `Departure entries (blocking)`; the `rule-verdicts` row; §Waivers |
| `framework/templates/spec/spec.md`, `scripts/audit/template-alignment.sh` | Modify | The grammar in the Applicable Rules comment; the new blocking heading judged |
| `framework/constitution.md` | Modify | One bullet added, two restated sentences cut |
| `specs/073-decided-rule-departures-are-recorded/data-model.md` | Create | Entry grammar, `waived` shape, `check-departures` result, the review marker |
| `specs/082-rule-assessments-converge-across-runs/data-model.md` | Modify | The third verdict shape (reopens 082) |
| `specs/066-audit-records-and-the-completion-gate/data-model.md`, `specs/020-code-review/scenarios/waiver-expiry.md`, `specs/022-deterministic-runtime/data-model.md` | Modify | The `rule-verdicts` description; the exception sentence; the `assessSpecQuality` answer |
| `specs/050-constitution/spec.md`, `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`, `specs/067-analyze-and-audit-checks/spec.md` | Modify | Signposts back-linking 073 |

## Data Model

[data-model.md](data-model.md) defines the entry grammar, the `waived` verdict, `check-departures`' result and the review waiver's `departure` marker. 082's data model and 066's record schema carry the verdict shape in the record (**§The analyze waiver** in `spec.md`); 073's data model owns the entry and the primitive's result and points at them.

## Trade-offs

- **A primitive rather than a `check-artifacts` family**: seven registration sites and a generated allow entry against about three files, chosen because the host needs the entries before steps 11 and 12 and `check-artifacts` returns none.
- **`write-review` derives the departure waivers**: the host has no step to forget, at the cost of `write-review` reading `spec.md` and history, which it already reads for scope.
- **The `departure` marker** is the one state that lets an entry's removal expire its waivers: one open-schema field per waiver.
- **The double report** of an entry's unknown or deprecated ID (above) is accepted.
- **Known limits**: a rename `find_similar` does not detect reads as creation, attributing the renaming commit's author; a squash or rebase attributes to its author, which no check can see; `decided-by` is who committed the entry, who accepted it for the spec, not necessarily who decided the departure, which a `cited:` document records.
- **Measured, 2026-11-13:** `attribute` over a temporary repository of 5,000 first-parent commits, the entry present from the first commit, in a release build (`cargo test --release`, one run each): 319 ms when `spec.md` is unchanged in every commit, 794 ms when every commit rewrites it (parsed each time). Wall time of the one call, on the author's machine; the worst case, since a walk ends at the first commit without the entry.
- **Not measured**: the compile time a `cargo test` step would add to `framework-checks`, whose `timeout-minutes` is 15; the script step avoids it.

## Open Questions Resolved

- *Settled before any finding exists* (Resolved Questions 1): `waived` is a verdict the runtime fills and the host cannot turn into a finding, so 058's AC30 stands.
- *Every file in review scope* (2): `write-review` derives per-file waivers each run from the run's own findings.
- *008 unchanged* (3): no tier is added.
- *`decided-by` from history* (4): `departures.rs`' attribution, `unattributed` as its own state, the test in `framework-checks`' full-depth job.
- *No count field* (5): `waived-count` is the result's derived figure, not a record field.
- *SHOULD rules too* (6): the review path is tier-agnostic, and AC8's fixture proves a SHOULD.

## Cross-spec impact

- **082 (`done`) reopens** `done → in-progress` when its `data-model.md` gains the third verdict shape, a data-model edit that is no correction (`classify-spec-edit`, §spec-lifecycle); it then owes the analysis its return to `done` requires, and a review only if `compute-review-scope` owes one. The back-link to 073 sits in that data model.
- **066, 020, 050** (`in-progress`): the `rule-verdicts` description in 066's data model; the exception sentence in 020's `waiver-expiry` scenario; a signpost in 050 for the constitution bullet and cut. No reopen.
- **022, 060, 067** (`in-progress`) join `cross-spec-impact:` with this plan, by AGENTS.md's rule that a change to a shared runtime surface holds `done` until the surface's owner links back: the exec walker and the `assessSpecQuality` answer (022, 060) and the analyze checks and report (067). Each is discharged by a back-link; no reopen.
- **058 is unchanged**: AC30 and the refusal stand by decision.
