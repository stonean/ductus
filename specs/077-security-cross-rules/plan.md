# 077 — Security cross rules Plan

Implements [077 — Security cross rules](spec.md).

## Overview

Create `framework/rules/security-cross.md` with `SEC-` rules: one new rule (`SEC-LOG-001`), four moved from `security-backend.md`, and `SEC-AUTHN-004`, which 008's task 43 moved from `BE-API-009`'s authorization-server sentences, and `SEC-DATA-001`, which it added. The work then has four parts:

- **Register the prefix** in this spec's `data-model.md`, the rule-ID lint and the analyze reference.
- **Deprecate the four old rules**, each reduced to a yield to its successor so `/{project}:analyze` reports a case once.
- **Re-point every yield and Statement** that names them, and sweep every live citation.
- **Ship the file**: the manifest row, `docs/rules.md`, the README, and `runtime/CHANGELOG.md`'s `## [Unreleased]` section.

No runtime behaviour changes; one corpus test is added (§Verification). The rule-ID regexes in `runtime/src/primitives/check_rule_ids.rs:103` and `runtime/src/primitives/rule_sections.rs:77` already accept any two-to-five-letter prefix.

## Technical Decisions

### Registering `SEC`

There are three registration sites, and each must agree.

- **This spec's `data-model.md`** registers `SEC-{CATEGORY}-{NNN}` and the categories `LOG`, `DEPS`, `AUTHN` and `DATA`, as 036's does for `QUAL` (`specs/036-quality-cross-rules/data-model.md`, §`QUAL` surface). `framework/reference/analyze.md` ("Which ID format governs a file", item 1) reads a file's introducing-spec data-model first, and a file whose IDs fail it is treated as unloadable.
- **`scripts/lint-rule-ids.sh`**'s `id_re` lists `SEC` beside `BE`, `FE`, `CFG` and `QUAL`, the source comment above it names 077's data-model, and the malformed-heading message lists `SEC`.
- **The analyze reference** lists 077's data-model beside 008's and 017's in item 1. 008's `specs/008-security-rules/data-model.md` (§Rule ID format, the paragraph after the category table) names `security-cross.md`'s `SEC` prefix beside `CFG` and `QUAL`.

`security-cross.md`'s header declares the format and its categories, as `quality-cross.md`'s does (`framework/rules/quality-cross.md:3`).

### `SEC-LOG-001` states its own value set

A frontend-only or cross-only project does not install `security-backend.md` (`framework/bootstrap/ductus.md`, "Rule-file surface filter"), so `SEC-LOG-001` cannot rely on `BE-LOG-002` for its value set. Its Statement names the values `BE-LOG-002`'s Statement bars, word for word, and bars them from a log line, an error-reporting payload and telemetry, and, since 008's task 44, product-analytics events and session-replay recordings, in any shape.

Its Verification asks a spec or plan that introduces logging, error reporting, telemetry, product analytics or session replay, client-side or server-side, to commit to excluding or redacting those values. Its Not-flagged clause names three owners:

- the exclusion or redaction commitment for logging and a value in a log line are `BE-LOG-002`'s, and the commitment for error reporting, tracing, metrics, baggage, product analytics or session replay and a value in those carriers `BE-LOG-008`'s, each "where `security-backend.md` is also loaded" (008 `data-model.md` §Verification phrasing, item 6), since both owners' triggers fire on the same missing commitment;
- a sensitive configuration value is `configuration-cross.md` `CFG-ENV-006`'s, with no condition.

Its Sources are the OWASP Logging Cheat Sheet, W3C Trace Context §6 and OpenTelemetry's Baggage security considerations, fetched when the rule is written, and, since 008's task 44, Google Analytics Help's PII guidance.

### Moved rules keep their text

`SEC-DEPS-001`, `SEC-AUTHN-001`, `SEC-AUTHN-002` and `SEC-AUTHN-003` take the Statement and Verification of `BE-DEPS-006`, `BE-AUTHN-009`, `BE-AUTHN-013` and `BE-AUTHN-020` as they stand at the move. Only the IDs and the load conditions change, except that `SEC-DEPS-001`'s reusable-workflow sentence and flag, and `BE-DEPS-006`'s kept Statement, ask a full commit SHA of a reusable workflow called by `{owner}/{repo}/…@{ref}`, in another repository or this one, and state that one called by `./` or `$/` is pinned by the caller's commit (owner, 2026-11-02, recorded on task 8), and, since 008's task 44, that a `$/` local action is pinned by the running commit, a `./` one by a checkout of the running commit or a full commit SHA, and that a GitHub-hosted runner label selecting a GitHub-owned or partner image MAY select the runner, and that `SEC-DEPS-001`'s yield keeps a package a build step's command runs by a mutable reference, such as `npx tool@latest` (below). 008's tasks 43 and 44 later changed `SEC-DEPS-001`, `SEC-AUTHN-002` and `SEC-AUTHN-003` beyond that, as `spec.md`'s §Rule set scope items 2, 4 and 5 record. Each owner a moved rule names that sits on a surface file takes the condition on its file:

- `SEC-DEPS-001`: `BE-DEPS-002` takes "where `security-backend.md` is also loaded", and `FE-DEPS-004` takes "where `security-frontend.md` is also loaded"; the yield keeps a package a build step's command runs by a mutable reference, such as `npx tool@latest`, which `FE-DEPS-004` and `BE-DEPS-002` name as `SEC-DEPS-001`'s, so no rule routes that case to one that routes it back;
- `SEC-AUTHN-001`: `BE-API-010` (d) takes the backend condition;
- `SEC-AUTHN-002`: its yield of an authorization server's `redirect_uri` matching names `SEC-AUTHN-004`, in the same file, so it takes none, and `BE-API-009` and `FE-AUTHN-001` name `SEC-AUTHN-004` with none; since 008's task 44 it also yields, where `security-backend.md` is also loaded, the generator of its values to `BE-DATA-009` and, for a client that verifies an ID token's signature, the algorithm allowlist and key binding to `BE-AUTHN-012` and the key's source to `BE-AUTHN-019`;
- the mutual references between `BE-AUTHN-013` and `BE-AUTHN-020` become `SEC-AUTHN-002` and `SEC-AUTHN-003`, in the same file, so they take none.

The Sources are re-fetched at the move, and any quote that no longer matches is corrected before the rule lands.

### A deprecated rule is a pointer

Each deprecated rule keeps its heading, its ID and its Statement. It gains a `**DEPRECATED in {version}:**` paragraph directly under the heading, the position `runtime/src/primitives/rule_sections.rs`'s `a_deprecated_section_parses_like_any_other` test uses. The paragraph names the successor and the removal version.

Its Verification becomes a pointer, "This rule is deprecated." and a **Not flagged:** clause naming the successor as the owner of every case the rule governed, with no load condition. `/{project}:analyze` therefore reports each case once, under the successor. The exec walker still asks about the rule, since a deprecated section parses like any other, and the host answers `passed`.

The version is the next one `version` would ship, computed when the labels are written: `0.60.0`, since that release carries 076's renamed record and result values (`898a49ec`), a breaking schema change and so a minor bump before 1.0 (`specs/022-deterministic-runtime/data-model.md`, the versioning paragraph). The removal target is the second minor release after it, `0.62.0`, as 077's second Resolved Question decided.

### Re-pointing

These name a deprecated rule and change to name its successor:

- `security-backend.md`: the yields in `BE-API-009`, `BE-API-010`, `BE-API-011` and `BE-API-013`, and the Statements of `BE-API-010` (d) and `BE-API-011`. The latter's "Token comparison MUST be constant-time per `BE-AUTHN-009`" becomes per `SEC-AUTHN-001`.
- `security-frontend.md`: the yields in `FE-AUTHN-001` and `FE-STORAGE-003`, which drop "where `security-backend.md` is also loaded", since a `-cross.md` owner installs in every shape. `FE-DEPS-004` gains a yield, with no condition, of a package a build step's command runs by a mutable reference, such as `npx tool@latest`, to `SEC-DEPS-001`, which a frontend-only project now loads beside it, so the case is reported once, its trigger reaching a spec, a plan and, since 008's task 44, `system.md`; `security-backend.md`'s `BE-DEPS-002` gains the same yield.

`configuration-cross.md`'s `CFG-ENV-006` is the only rule outside the backend surface, other than this spec's `SEC-LOG-001`, that names `BE-LOG-002` or `BE-LOG-008` for a sensitive value (grep of `framework/rules/` for both IDs). For a non-configuration value in a log line it reads "`BE-LOG-002`'s where `security-backend.md` is also loaded and `security-cross.md` `SEC-LOG-001`'s otherwise", and the same for a non-configuration value in propagated trace context, while a sensitive configuration value there is `BE-LOG-008`'s where `security-backend.md` is also loaded and `CFG-ENV-006`'s otherwise, as 008's task 43 decided.

`observability-backend.md`'s pointers to `BE-LOG-008` stay as they are: it installs on the backend surface, beside `security-backend.md`.

### Sweep

A live artifact that cites a deprecated ID as a current requirement names the successor:

- `.github/workflows/framework-checks.yml` and `.github/workflows/runtime-release.yml`, in comments;
- `scripts/check-runtime-currency.sh` and `framework/templates/ci/adopter-generators.yml`;
- 008's `data-model.md`;
- 017's `plan.md` and its scenario `the-adopter-ci-template-can-run`;
- 022's `plan.md`;
- 060's scenario `an-assessment-sees-the-rule-it-asks-about`, whose "`BE-AUTHN-013` to 'all four requirements'" the deprecated rule's new Verification makes untrue.

Each substitution maps a deprecated ID to its current one, so it is mechanical (constitution §spec-lifecycle, case (a)).

These keep the old ID, because they record what was decided or measured: 008's `tasks.md`, 022's `tasks.md` (its ticked task 3, and task 10's unticked item naming a stored `analysis.md` decision key), 008's scenario `rules-are-as-short-as-their-required-content` (checked when swept), the dated assessment in the Context of 017's scenario `the-adopter-ci-template-can-run`, changelog entries, and `review.md` and `analysis.md` records. `.github/` changes run `actionlint`.

### Distribution

- `framework/bootstrap/ductus.md` gains the row `framework/rules/security-cross.md` → `specs/rules/security-cross.md`, between `security-backend.md`'s and `security-frontend.md`'s. The surface filter keeps every `*-cross.md` entry unconditionally (line 836), so every shape installs it.
- `docs/rules.md`'s "What ships" table gains a row.
- `README.md`'s security bullet (line 52) names the cross-surface file.
- `framework/bootstrap/ductus-procedure.md`'s §Security Audit (brownfield) triggers on a newly created `security-cross.md` as on either surface file, loads it, and checks each loaded file's IDs, `SEC-{CATEGORY}-{NNN}` among them, against the format `/{project}:analyze` reads for that file (owner, 2026-11-03, recorded on task 11), so the upgrade that installs it previews its rules' gaps on a project's existing specs; its §Per-rule check applies each rule to a spec as `/{project}:analyze` does, its trigger, commitment, flags and set-asides judged against the spec's `spec.md`, `plan.md`, `data-model.md` and scenarios together, with each cross-spec file the spec or a rule's Verification refers to, one finding per spec and rule (owner, 2026-11-03, recorded on task 11), and honours a yield wherever its Verification states one, judging a load condition against every rule file installed under `specs/rules/`, so a case it yields to a rule in an installed file is left to that rule, as `/{project}:analyze` leaves it: the audit reports the case under the owner's ID when it newly created the owner's file too, and not at all when that file was already installed, on an upgrade or a surface added alike (owner, 2026-11-02, two calls, recorded on task 8). An owner outside the security rule files, in a file the manifest pass newly created, has its file loaded with the same integrity checks and is applied to that case alone, as `/{project}:analyze` applies it — its whole Verification judged, whether or not the spec meets the yielding rule's commitment — so the case is reported under its ID where the yielding rule's trigger fires too and `/{project}:analyze` reports it (owner, 2026-11-03, recorded on task 11). A change under `framework/bootstrap/` is verified by a real adopter bootstrap (tasks 10 and 11).
- `framework/constitution.md`'s Secure principle names the `specs/rules/security-*.md` files, and its canonical-sources row for rule ID conventions points any prefix but `BE` and `FE` at the data-model of the spec that introduced its file, else the file's header, so neither lists files or prefixes one by one, and the constitution stays under `scripts/lint-governing-size.sh`'s 50,000 bytes.

`runtime/CHANGELOG.md`'s `## [Unreleased]` section gains one entry. It names:

- the four ID changes and their successors, and which of the old IDs 0.59.11 shipped (`BE-AUTHN-009` and `BE-AUTHN-013`; `BE-DEPS-006`, `BE-AUTHN-020` and `BE-LOG-008` came after it);
- the deprecation and removal versions;
- the one defect `/{project}:review` reports under two IDs until 020's routing lands, a configuration value's pair lasting past it, and that a waiver under one needs a counterpart under the other, since a waiver is anchored on rule ID and file (`framework/reference/review.md`, §Waivers), the one under the yielding ID expiring once that routing moves the finding to its owner;
- what a pinned or leftover `security-backend.md` or `security-frontend.md` reports under both IDs, or under none, until it is re-synced or deleted.

### Verification

Every rule this spec writes or changes is re-read before the work is called done (AC10). The re-read uses fresh agents, split by cluster: logging, dependencies, authentication, and records. Each probes all four project shapes, under the same severity bar 008's rounds used, repeated until a round has no MUST or SHOULD.

The local gate is a fresh clone of HEAD plus the patch, running `cargo test --release --locked` and every step of `framework-checks.yml`, `generators.yml` and `runtime.yml`. `every_shipped_rule_is_assessable` (`runtime/src/primitives/rule_sections.rs`) parses every file under `framework/rules/` and must pass with the new one. Beside it, `sec_log_001_bars_the_values_be_log_002_bars` compares the two value lists, and `a_barred_value_list_is_read_and_a_differing_one_is_caught` proves its comparison both ways on fixtures.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/security-cross.md` | Create | The `SEC-` rules |
| `specs/077-security-cross-rules/data-model.md` | Create | Registers `SEC` and its categories |
| `framework/rules/security-backend.md` | Modify | Deprecate four rules; re-point `BE-API-009`, `-010`, `-011`, `-013`; `BE-DEPS-002`'s yield to `SEC-DEPS-001` |
| `framework/rules/security-frontend.md` | Modify | Re-point `FE-AUTHN-001` and `FE-STORAGE-003`, dropping their load condition; `FE-DEPS-004`'s yield to `SEC-DEPS-001` |
| `framework/rules/configuration-cross.md` | Modify | `CFG-ENV-006` names `SEC-LOG-001` where `security-backend.md` is not loaded |
| `scripts/lint-rule-ids.sh` | Modify | Allow `SEC` |
| `runtime/src/primitives/rule_sections.rs` | Modify | A test holding `SEC-LOG-001`'s value list to `BE-LOG-002`'s |
| `framework/reference/analyze.md` | Modify | Name 077's data-model as `SEC`'s format source |
| `specs/008-security-rules/data-model.md` | Modify | Name the `SEC` prefix; sweep `BE-DEPS-006`; the schema's Surface, ID-prefix, Category Name and Deprecated entries and its removal rule |
| `framework/constitution.md` | Modify | The Secure principle's rule files and the rule-ID canonical-sources row |
| `framework/bootstrap/ductus.md` | Modify | Manifest row |
| `framework/bootstrap/ductus-procedure.md` | Modify | The brownfield security audit's trigger, loading and ID check reach `security-cross.md`; its §Per-rule check as §Distribution above states; its re-trigger sentence is narrowed |
| `framework/bootstrap/govern.md` | Modify | Byte copy of `ductus.md`, which audit family `transitional-bootstrap-parity` requires |
| `docs/rules.md` | Modify | "What ships" row; what a moved rule's old ID does |
| `README.md` | Modify | Security bullet; ID permanence |
| `runtime/CHANGELOG.md` | Modify | `## [Unreleased]` entry |
| `.github/workflows/framework-checks.yml` | Modify | Comment sweep |
| `.github/workflows/runtime-release.yml` | Modify | Comment sweep |
| `scripts/check-runtime-currency.sh` | Modify | Comment sweep |
| `framework/templates/ci/adopter-generators.yml` | Modify | Comment sweep |
| `specs/017-derive-dont-ask/plan.md` | Modify | Sweep |
| `specs/017-derive-dont-ask/scenarios/the-adopter-ci-template-can-run.md` | Modify | Sweep |
| `specs/017-derive-dont-ask/spec.md` | Modify | Block-quoted signpost discharging 077's impact |
| `specs/022-deterministic-runtime/plan.md` | Modify | Sweep |
| `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-rule-it-asks-about.md` | Modify | Sweep, and its count re-measured over the rule files the sweep changed |
| `specs/008-security-rules/spec.md` | Modify | Block-quoted signpost discharging 077's impact; its opening and Backend category list, the brownfield audit trigger, logic and idempotency, AC26's annotation, the "Neither rule file present" edge case, §Constitution Reference and AC28 |
| `specs/008-security-rules/plan.md` | Modify | The constitution's Secure sentence it quotes; the brownfield audit trigger, loading and per-rule steps |
| `specs/008-security-rules/tasks.md` | Modify | Tick task 32's two items, task 40's first, fourth and fifth items, and task 42's item; task 44 records the gaps the re-read found and closes its two audit-logic items by task 11's decision; task 30's handoff and task 42's note |

## Data Model

`data-model.md` registers the `SEC` surface:

- its prefix, its ID grammar and its categories;
- the rule set, with each rule's tier and, for a moved rule, its predecessor;
- a pointer to 008's `data-model.md` for the rule schema, which it does not restate.

## Trade-offs

- **`SEC-LOG-001` restates `BE-LOG-002`'s value list rather than citing it.** Citing it would leave the rule empty where `security-backend.md` is not installed. The cost is two copies of one list, and a change to either must change both. Neither rule's text points an editor of `BE-LOG-002` at the copy, so `sec_log_001_bars_the_values_be_log_002_bars` (`runtime/src/primitives/rule_sections.rs`) fails when the two lists differ, or when either rule is missing.
- **A deprecated rule's Verification is replaced by a pointer, not kept.** Keeping it would report every moved case under both IDs at `/{project}:analyze` in backend-only and full-stack projects until removal. Its Statement is kept, so `/{project}:review`, which applies Statements and no routing, still reports both until 020 task 15 lands (`spec.md` §Edge cases).
- **`CFG-ENV-006` names both owners directly** rather than naming `SEC-LOG-001` alone and letting that rule yield onward. A direct name reaches the owner in one step, and a chain of yields is what 008's rounds found hardest to read alone.
- **The deprecation version is computed when the labels are written** (`0.60.0`, removed in `0.62.0`). A release cut under another number must correct the four labels, and nothing checks that today; 008's task 43 closed a sibling release-time gap, `scripts/check-release-tag.sh` now refusing a tag while `## [Unreleased]` holds an entry.
- **A project that pins `security-backend.md` sees doubled findings** until it re-syncs. Nothing a shipped file says can reach a pinned copy, so the changelog entry is the remedy (spec §Edge cases).

## Open Questions Resolved

- **The ID prefix** is `SEC`, registered in the three registration sites above.
- **The removal version** is the second minor release after the shipping one, written into each `**DEPRECATED in {version}:**` label.

## Cross-spec impact

- **008-security-rules.** Its rule files change (deprecations and re-pointed yields), and so does its `data-model.md` (the `SEC` prefix and the sweep). Its tasks 32, 40 and 42 close here. Discharged by a block-quoted signpost in 008's `spec.md` linking this spec. The block quote keeps `derive-dependencies` from inducing an `008 → 077` edge, which would close a cycle, since 077 depends on 008.
- **017-derive-dont-ask.** `CFG-ENV-006`, a rule on its surface, changes, and its `plan.md` and scenario are swept. Discharged by a block-quoted signpost in 017's `spec.md`.
- **022 and 060.** 022's swept sentences are mechanical substitutions (constitution §spec-lifecycle, case (a)). 060's scenario also re-measures its count over the rule files this spec changed, a factual update to an in-progress spec, which takes no back-edge. Neither owes anything to 077.
