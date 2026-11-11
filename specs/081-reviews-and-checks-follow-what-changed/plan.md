# 081 — Reviews and checks follow what changed Plan

Implements [081 — Reviews and checks follow what changed](spec.md).

## Overview

A review is owed for code alone, where code is every path outside the project's artifacts. A commit naming a spec has its code reviewed by the specs it names. The runtime change is in `compute-review-scope`'s owed rule and window attribution, and in `check-review-gate`, which stops blocking on a stale review digest. `check-review-freshness` and the self-audit's Family 19 asked only that staleness question, so both retire. In this repository the two workflows that run on a spec-only push reuse a cached runtime binary instead of compiling one. The constitution, the review and implement references, the descriptions and `AGENTS.md` say the new rule, and 079, 066, 026, 047 and 050 are corrected to match.

## Technical Decisions

### The artifact set is one predicate

A single function in `runtime/src/primitives/mod.rs`, `is_project_artifact(path, specs_root)`, holds the set: a path under the configured spec root, under `.ductus/`, or equal to `AGENTS.md` or `CLAUDE.md`. Paths are project-relative, as every `compute-review-scope` path already is (`ProjectRepository::to_project`), so a configured spec root and a project in a repository subdirectory need no case of their own. Everything that asks "is this code" calls it.

### What makes a review owed

`paths_owing_a_review` (`runtime/src/primitives/compute_review_scope.rs`) keeps a path when it is in scope, or is a deletion that is this spec's, and is not a project artifact. The `is_review_contract(own) && !contracts_current` branch goes, so a spec's own scenarios and data model never owe a review. Paths the record at `HEAD` leaves open still owe one. `record_is_held` keeps `holds_done_until_reviewed` and drops its `RecordFreshness::Stale` clause. A spec with no review record still owes a review (`run.is_none_or(...)`, unchanged).

`contracts_against_digest` stays. A contract changed since the review is still put in the scope of a review that runs, so a review owed for code reads the contracts that changed beside it. `reviewed-digest` therefore keeps a reader and stays in the record schema, `reviewed-against` keeps its one reader (the rename exemption inside `review_freshness`), and no adopter's record changes shape.

### Attribution

`CommitClaims` (`runtime/src/primitives/commit_attribution.rs`) gains `named`: every spec the subject's scope names, in subject order. `first_named` becomes `named.first()`. `attribute_window` changes only for a commit whose subject names a spec, for the spec under review:

| The commit | This spec's scope gets |
| --- | --- |
| names this spec | every path outside the other claimants' directories |
| names other specs, and touched this spec's directory | the paths in this spec's directory |
| names no spec | what 079's rule gives it, unchanged: every path when no spec claims the commit or this spec claims it alone, and otherwise its directory, its plan's paths and the paths no claimant's plan lists |

Anything else goes to `excluded` under the claimants' names, as now. A commit naming no spec stays 079's because the pipeline prescribes no subject convention. An adopter's code commit usually names none, and plans are not exhaustive, so narrowing it would leave unplanned code unreviewed (spec, Resolved Questions). Uncommitted work stays in scope. Fix rounds and `diff-cross-spec` read `CommitClaims` but not `attribute_window`, so they are unchanged.

### The gate

`check-review-gate` loses check 6, `review is stale` (`stale_review_block`), and `ReviewGateBlock::ReviewStale`. It also loses the review half of `passing_notices`, the notice that a record with no `reviewed-digest` could not be compared. The analyze half's freshness check (`AnalyzeStale`) and every other check are unchanged. Losing a variant narrows the `blocked-by` value set, which 066's data model documents.

### Retiring `check-review-freshness` and Family 19

Family 19 (`scripts/audit/review-freshness.sh`) asked `check-review-freshness` whether a `done` spec's durable contracts still matched its `reviewed-digest`. Under the reversal, a contract changed after a review is no defect, so the family's finding would fail the release audit on correct states. Its other case, a record that could not be compared, depended on the same digest. `check-review-freshness` has no other caller. The primitive's seven sites go, per [`docs/developing-ductus.md`](../../docs/developing-ductus.md) §Adding or removing a runtime primitive:

- `PRIMITIVE_REGISTRY`, its description, its `#[tool]`, its `dispatch_primitive` arm and its `Command`;
- its `framework/runtime-tools.txt` line;
- its `result_sizes.rs` case.

Its allow entry joins the retired list in `framework/bootstrap/configure/claude.md`, so `merge-permissions` revokes it from adopters' settings, and the generated allow block is regenerated. Family 19 leaves `run-all.sh`, `framework/reference/audit.md` and `scripts/audit/README.md`, its number spent as Family 3's is. `scripts/mutation/family-19-calls-the-runtime.py` goes with it.

### Check selection in this repository's CI

`.github/workflows/framework-checks.yml` and `.github/workflows/generators.yml` each cache `runtime/target/release/ductus` under a key built from `hashFiles('runtime/Cargo.lock', 'runtime/Cargo.toml', 'runtime/rust-toolchain.toml', 'runtime/src/**')`. Their build step runs only on a cache miss, and prints that no runtime built from those sources was cached. The key covers what the binary is compiled from: `runtime/src` holds no `include_str!` or `include_bytes!`, and the crate has no `build.rs` (checked at `8dd03c22`). The existing registry cache stays for a miss. `runtime.yml`'s path filter already excludes `specs/` and stays as it is. A push whose changes are confined to artifacts then compiles nothing whenever the sources match a cached build.

### The governing text

- **`framework/constitution.md`.**
  - §grounding's blast-radius rule gains the division and the artifact set: a change confined to the project's artifacts owes the artifact checks, and no build, test, code lint or code CI.
  - Its "a review or analysis record is judged by a digest of what it read" becomes a statement about the analysis record.
  - The file is 49,914 bytes against its 50,000-byte ceiling, so the same edit cuts at least what it adds.
- **`AGENTS.md`.**
  - The local-checks rule gives an artifacts-only change the hook and the self-audit, with no build and no code workflow.
  - The gotcha "Only a spec's durable contracts stale its review" goes.
- **`framework/reference/review.md`.** §Record the run and §Window and coverage say what owes a review, the attribution table, and what the digest is still for.
- **`framework/reference/implement.md`.** Step 5's freshness paragraph is corrected.
- **`framework/reference/fold.md`.** The folded-in-scenario bullet is corrected.
- **`docs/analyze.md`.** Its review-freshness paragraph is corrected.
- **Descriptions.** `compute-review-scope`'s and `check-review-gate`'s compiled descriptions state the rule, since the binary is a primitive's canonical source.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/mod.rs` | Modify | `is_project_artifact` |
| `runtime/src/primitives/commit_attribution.rs` | Modify | `CommitClaims::named` |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | Owed rule, attribution, tests |
| `runtime/src/primitives/check_review_gate.rs` | Modify | Remove check 6 and its notice, tests |
| `runtime/src/primitives/check_review_freshness.rs` | Delete | Retired primitive |
| `runtime/src/primitives/analyze_subjects.rs` | Modify | Doc comments naming the retired callers |
| `runtime/src/schema/registry.rs`, `runtime/src/schema/primitives.rs`, `runtime/src/schema/descriptions.rs` | Modify | Unregister, describe |
| `runtime/src/mcp/server.rs`, `runtime/src/interpreter/mod.rs`, `runtime/src/main.rs` | Modify | Unregister |
| `runtime/tests/result_sizes.rs`, `runtime/tests/*.rs`, `runtime/tests/golden/*` | Modify | Cases, fixtures, goldens |
| `runtime/CHANGELOG.md` | Modify | Unreleased entry |
| `framework/runtime-tools.txt`, `framework/bootstrap/configure/claude.md` | Modify | Tool list, retired allow entry, regenerated block |
| `framework/constitution.md` | Modify | Division and check selection |
| `framework/reference/review.md`, `framework/reference/implement.md`, `framework/reference/fold.md`, `framework/reference/audit.md` | Modify | Prose |
| `scripts/audit/review-freshness.sh`, `scripts/mutation/family-19-calls-the-runtime.py` | Delete | Retired family |
| `scripts/audit/run-all.sh`, `scripts/audit/README.md` | Modify | Unregister Family 19 |
| `.github/workflows/framework-checks.yml`, `.github/workflows/generators.yml` | Modify | Binary cache keyed on runtime sources |
| `AGENTS.md`, `docs/analyze.md` | Modify | Local checks, prose |
| `specs/079-review-and-analysis-work-converges/spec.md` | Modify | AC4, AC6 and two Resolved Questions point here |
| `specs/066-audit-records-and-the-completion-gate/` | Modify | Gate checks and `blocked-by` values |
| `specs/026-framework-self-audit/` | Modify | Family 19 retired |
| `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md` | Modify | Family 19 row |
| `specs/050-constitution/` | Modify | Back-link for the constitution change |

## Trade-offs

- **Retiring `reviewed-digest`.** Rejected. `contracts_against_digest` still reads it to put changed contracts in a running review's scope, and adopters' records carry it, so dropping it was a schema change for no behavior gained.
- **Repointing Family 19 at code changed since a `done` spec's review.** Rejected here. Family 19's header records that scoping by a plan's Affected Files flagged 42 of 48 specs, so a code-based family is new work needing its own measurement. What retiring gives up: Family 19 was built after `gvrn-v0.26.1` and `gvrn-v0.26.2` shipped with 022 scenarios added after its review. Under the reversal, a scenario added through `/{project}:amend` carries a task whose code owes the review at the gate. A hand-added one is checked against the code by nothing, as the spec's Resolved Questions accept.
- **Keeping Family 19 for unparseable records.** Rejected. `/{project}:analyze` already hard-fails a record that does not parse.
- **A released binary in CI instead of a cached build.** Rejected. A release lags `main`'s schemas, 1,083 commits at `6158d7df`.
- **Caching `runtime/target` under a source-hash key.** Rejected for the single binary. The binary is what the steps run, and caching it makes a hit skip the compile outright.
- **Known limitation: a cache miss still compiles.** On eviction or a first push after a runtime change, a spec-only push compiles once, and the run says so.

## Cross-spec impact

- **079.** AC4's annotation, AC6 and the Resolved Questions on scenario edits and attribution change. That reopens 079, which then owes an analysis and, under this rule, no review.
- **066.** Its data model and scenarios list the gate's checks and `blocked-by` values; it is `in-progress`.
- **026.** Its criteria and scenarios describe Family 19; it is `in-progress`.
- **047.** Scenario `analyze-run-durability` names Family 19 as a reader of review records. It is `done`, so the edit reopens it, and it owes an analysis.
- **050.** It owns the constitution's content and gains the back-link.

Each affected spec links back here, which discharges 081's `cross-spec-impact:`, and 047 joins that list.
