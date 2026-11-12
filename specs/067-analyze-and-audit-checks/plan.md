# 067 — Analyze and Audit Checks Plan

Implements [067 — Analyze and Audit Checks](spec.md).

## Overview

The primitives this spec specifies shipped under 022, and the first work moved their contract here — twenty-five scenarios, the data-model sections for the checks, and four acceptance criteria — and gave `check-corpus-links` the documented shape it never had. Since then this spec's own scenarios have changed the runtime: `check-artifacts`' scenario-consistency history, skipped targets and body-path family and the logical-line reading they share (tasks 5–11); the review rounds' fixes to the derive splices, the reference prefixes and `check-promotion-coverage`'s refusal; `derive-dependencies`' and `derive-references`' `unwritable` list (tasks 14 and 25); the status-gated path families (task 17); `validate-frontmatter`'s row by path (task 18); and `resolve-anchor`'s judgment of references into other documents, with Family 39 gating the whole spec corpus (task 19). Each interface change is classified under `BE-COMPAT-001` below.

## Technical Decisions

### One subject per spec, and the dependency runs one way

067 holds the deterministic checks; [066](../066-audit-records-and-the-completion-gate/spec.md) holds the records their findings feed and the gate that reads them, and 022 the parser, the protocol, the severity tiers and the shared scanners. 067 depends on both, neither links it, and 022 names it only as a backticked slug, so `derive-dependencies` sees `067 → 066 → 022` and no cycle.

### Scenarios move with `git mv`

Each scenario is moved, not copied, so its history follows it and the rename is visible in the log. A scenario changing contracts in two specs moves with the one whose contract its Behavior mostly changes and is not split. Its `section:` is remapped to a heading of this spec, and its links into 022 or 066 become `../../022-deterministic-runtime/` and `../../066-audit-records-and-the-completion-gate/` paths. `scripts/audit/marker-list-parity.sh` reads `criterion-path-existence-family.md` by path, so it changes in the same commit as the move.

### The data model states shapes and field meanings

The sections moved from 022 keep every argument and result shape and every field meaning, including `scenario-consistency`'s referencing-task rule, which the constitution's canonical-sources map names, and drop the rationale their scenarios already state. `check-corpus-links` gains its first documented shape, taken from `runtime/src/schema/primitives.rs`.

### The routing gate keeps offering 022

`derive-routing-candidates` offers the lowest-numbered spec whose plan claims a `runtime/` path (`runtime_home` in `runtime/src/primitives/derive_routing_candidates.rs`), which stays 022. Which spec owns a primitive is the routing decision, stated in `AGENTS.md`'s routing entry. Listing this spec's runtime files below is what scopes its review.

### `check-promotion-coverage` refuses an empty `sections`, a breaking change

Since review round 1 (`2be18da6`) the primitive refuses an empty `sections` on both surfaces rather than reporting zero counts, and the CLI's repeatable `--section` parses as empty when omitted, so an optional argument became required. `BE-COMPAT-001` classes that as breaking; it ships with the pending 0.60.0 minor bump, the pre-1.0 boundary for a breaking change (071's plan). The one in-repo caller, `scripts/audit/promotion-coverage.sh`, passes four `--section` flags.

### A spec records a removed rule's retirement in `retired-rules:`

`check-rule-ids` reads the optional `retired-rules:` frontmatter list of a `spec.md` (`retired_rules_and_body` in `runtime/src/primitives/check_rule_ids.rs`) and reports a citation of a listed ID that no loaded rule file defines as `retired`, never `found` and never `missing`. Choices the scenario left open:

- **The list is not a citation.** The scan reads the body when the file has frontmatter, so a listed ID is checked only by the body citations it exempts and a mistyped entry exempts only its own mistyped citation. Scanning the whole file would report every listed ID `retired` from the list line alone.
- **Only a `spec.md` is read for the list**, so the same list in a `plan.md` exempts nothing, as the scenario's "any other artifact" requires.
- **A list that cannot be read fails closed.** A non-list value, a non-string entry, or frontmatter that does not parse yields no exemption, so the citation stays the blocking `missing`; `validate-frontmatter` treats `retired-rules` as an unknown field the open-schema rule admits and checks its shape nowhere, since a malformed list can only add findings, never hide one.
- **A loaded definition wins.** A listed ID a loaded rule file still defines is `found` (or `deprecated`), as today, so the list exempts nothing until the rule is gone.
- **Retired passes without an unexamined record.** The rule was removed, so there is no text to assess and nothing the run could not read; the step 5 tally counts `missing` alone.

### `check-artifacts` walks the feature directory and skips an outward link

022's walk (`validate_feature_argument`) refuses the whole call on the first link that leaves the project, which would break 067's promise that a linked target is skipped and the run continues. `check-artifacts` therefore calls `OutwardLinks::find` (`runtime/src/primitives/mod.rs`), which runs the same walk, `collect_outward_links`, and returns every such link instead of the first refusal. `validate_feature_contained` stays: `create-feature` and `decisions` call it.

- **Skips, under the family `outward-link`.** The skip's family is a name of its own rather than a borrowed one, because the guard is no check and the link may be a file no family reads; reason `target-unparseable`, path the link.
- **Nothing behind a link is opened.** `read_spec::read_checked_unlinked` takes a predicate and skips a covered scenario and `review.md`; `tasks.md` and `plan.md` are skipped at the call; `scenario_slugs`, `scanned_artifacts` and `body_artifacts` drop a covered path. `artifact-completeness` still stats `plan.md` and `tasks.md` to see that they exist, which opens nothing.
- **A linked `spec.md` ends the run.** There is no status to gate a family on, so the result carries the skip alone, status `unreadable`, `clean: true` with a non-empty `skipped` (partially examined, `QUAL-CLAIM-001`).
- **A feature directory that is itself a link out** is still refused whole, as the argument check refuses it for every primitive.

### A path that leaves the project is `outside-project`, not `root-absent`

Both path families resolve a candidate lexically before any probe (`lexical_in_project`, `resolve_body_path` in `runtime/src/primitives/check_artifacts.rs`) and record one that leaves the project. `root-absent` said "a top-level segment this repository lacks", which sits in the excluded-by-construction class that recurs on every run and is correct, so a suspect citation read as benign. The new reason `outside-project` sits in the *could not be read* class, the one a stored decision is retained against (reference §Unexamined targets, step 18). It is named in the closed sets of this spec's data model and its `check-artifacts-skipped-targets` scenario, in 045's `data-model.md` and 066's class table, and in the compiled docs of `SkippedTarget` and `unexamined_by_reason`; 045's edit reopens it, a cost the owner accepted (2026-11-12). Nothing in the runtime classifies reasons, so no code beyond the two sites changes.

### The integrity checks split between a primitive and step 15

§Spec integrity and §Plan consistency have eight bullets. The deterministic ones run in `check-artifacts` as two families (`check_spec_integrity`, `check_plan_consistency` in `runtime/src/primitives/check_artifacts.rs`), and the three semantic ones (implementation code in the spec, a decision without a rationale, a plan contradicting `system.md`) are added to step 15's host reading rather than a new step, because a new step renumbers steps 16–21, which the runtime's walker, the references and the `done` specs' plans cite by number. Step 15 therefore owns two exec reasons, `grounding-not-checked` and the new `integrity-not-checked`, both recorded once per exec run.

- **A placeholder criterion** is empty, a placeholder word, an ellipsis, or text wholly inside `<>` or `{}`; a longer phrase merely containing `TBD` is a real criterion.
- **A plan references its spec** by a link to its `spec.md`, or by naming its feature directory or number: 13 `done` specs predating the plan template's link name the spec only in the title, and requiring the link would block every analysis of them. The check is a floor, not proof the plan describes that spec.
- **Technical Decisions and Affected Files** are read after dropping comments, single-line ones included, so the template's own guidance comments never satisfy them. Whether a decision has a rationale stays the host's. Affected Files lists a file when it holds a list item or table body row with text, a code span, or a word holding a `/`: the template calls the section a non-exhaustive planning aid, so the check catches a section left empty or holding its scaffold and never judges a spelling, which an earlier rule (a backticked span with a `/` or an extension) did, flagging an unbackticked `- internal/widget.go` or a bare `- Makefile` as blocking (the parent's verification, 2026-11-12).
- **The existing `check_artifacts` tests run through a helper that drops these two families**, since their fixtures are sparse specs and plans written for the family each names; the new tests call the unfiltered one.
- **Measured** at this change over the 77 specs in `specs/` with the release binary: no finding from either family.

### Primitive interface changes, classified under `BE-COMPAT-001`

The rule counts a new value in a response set the schema documents as closed as breaking, as it does a narrowed request or a changed exit status, which is the CLI's machine-readable contract. The runtime is pre-1.0, so a breaking change ships with a minor bump; the unreleased ones ship with 0.60.0. Commits carrying 067's number that change no interface — the logical-line reading and its markers (`0e484e52`, `0a157563`, `cba81b63`, `3b6e6946`, `3b2f0856`, `830e90cb`), the reference-prefix rules of `check-orphaned-references` and `rewrite-spec-links` (`d1853444`, `3fc5755a`, `54715126`, `38fc39f6`), the derive splices (`2be18da6`) and the reference sweeps (`13acc295`, `bd4da459`) — change which findings fire, not the shape or the contract.

| Commit | Primitive | Interface change | Class |
| --- | --- | --- | --- |
| `107e22fb` (task 5, in 0.59.9) | `check-artifacts` | scenario-consistency's message below `done` names the remedy | additive: no field or value changes |
| `49a5b5e1` (task 6, in 0.59.9) | `check-artifacts` | `skipped` carries the existing `no-readable-state` from one more family | additive |
| `abec64ab` (task 7, in 0.59.10) | `check-artifacts` | `family` gains `body-path-existence`, and the closed `reason` set gains `never-existed` | breaking: new values of response sets documented closed, shipped in a patch release without the bump |
| `2be18da6` | `check-promotion-coverage` | an empty `sections` is refused | breaking: an optional request field became required |
| `e38f5c1f` | `check-artifacts` | `disposition-drift` findings move from blocking to advisory | breaking: a response value's meaning changed, so a caller gating on it now passes |
| `77fff147` | `apply-manifest` | the result gains `modes-kept` | additive |
| `d90d2133`, `6b0f96fd` (tasks 14, 25) | `derive-dependencies`, `derive-references` | the result gains `unwritable` and `unwritable-count`, moved into the file form past the read size; both exit non-zero on an unwritable spec | breaking: a run that exited 0 now exits non-zero; the fields are additive |
| `63527403` (task 17) | `check-artifacts` | the closed `reason` set gains `status-gated` and `gitignored`; the path families report at `in-progress` | breaking: new values of a closed response set |
| `286c2c09` (task 18) | `validate-frontmatter` | a path of no schema row is refused; a scenario path takes the scenario row; a `spec.md` run reports its scenarios' findings | breaking: a request accepted before is refused |
| `8651d466` (task 19) | `resolve-anchor` | references gain `unjudged` and the result two counts; a `qualified` reference can be unresolved; a basename no longer names the markers file; a malformed registry is an error; a span citation is a reference | breaking: the fields are additive, the meaning of `qualified` and of the markers match is not |
| (task 22) | `check-rule-ids` | `citations` entries gain `retired` and the result `retired`, both omitted when empty; a `spec.md`'s `retired-rules:` list passes its citation of a removed rule, and the list itself is no citation | additive: a new optional state, no closed set changes, and a spec with no list reads as before |
| (033 task 10) | `check-rule-ids` | `citations` entries gain `not-installed` and the result `not-installed`, both omitted when empty; the closed `reason` set gains `rule-not-installed` | breaking: a new value of a response set documented closed, and a citation that was `missing` is now informational |
| (task 23) | `check-artifacts` | `family` gains `spec-integrity` and `plan-consistency`; the closed `reason` set gains `integrity-not-checked` (recorded by the exec walker) | breaking: new values of response sets documented closed, and a spec the checks flag now carries a blocking finding |

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `specs/067-analyze-and-audit-checks/spec.md` | Create | The spec |
| `specs/067-analyze-and-audit-checks/data-model.md` | Create | The checks' shapes |
| `specs/067-analyze-and-audit-checks/scenarios/*.md` | Move | The twenty-five scenarios moved from 022 |
| `scripts/audit/marker-list-parity.sh` | Modify | Reads `criterion-path-existence-family.md` at its new path |
| `framework/constitution.md` | Modify | The canonical-sources row for the scenario-to-task referencing rule names this data model |
| `runtime/src/primitives/check_artifacts.rs` | Specify | `check-artifacts` |
| `runtime/src/primitives/validate_frontmatter.rs` | Specify | `validate-frontmatter` |
| `runtime/src/primitives/traverse_deps.rs` | Specify | `traverse-deps` |
| `runtime/src/primitives/resolve_anchor.rs` | Specify | `resolve-anchor` |
| `runtime/src/primitives/check_rule_ids.rs` | Specify | `check-rule-ids` |
| `runtime/src/primitives/derive_dependencies.rs` | Specify | `derive-dependencies` |
| `runtime/src/primitives/derive_references.rs` | Specify | `derive-references` |
| `runtime/src/primitives/check_corpus_links.rs` | Specify | `check-corpus-links` |
| `runtime/src/primitives/check_orphaned_references.rs` | Specify | `check-orphaned-references` |
| `runtime/src/primitives/check_command_flags.rs` | Specify | `check-command-flags` |
| `runtime/src/primitives/check_promotion_coverage.rs` | Specify | `check-promotion-coverage` |

## Data Model

[data-model.md](data-model.md) defines the argument and result shapes of `check-artifacts` and its families, `validate-frontmatter`, `traverse-deps`, `resolve-anchor`, `check-rule-ids`, `derive-dependencies` / `derive-references`, `check-corpus-links`, `check-orphaned-references`, `check-command-flags` and `check-promotion-coverage`.

## Trade-offs

- **One new spec for the checks and the records together**, which is keeping the checks with the records in 066 — rejected: the checks have readers the records do not, the `/{project}:audit` families and the pre-commit hook. Size does not decide it: one spec holding both would carry a data model of about 40 KB (the two as built, 19,775 and 20,201 bytes by `wc -c` on 2026-10-15), some 10 KB under the 50,000-byte read size, where two keep each near 20 KB; 022's 44,823 bytes are the same either way.
- **Splitting each mixed scenario across specs** — rejected: it would rewrite each such requirement document to move part of its Behavior, when a pointer serves every reader.
