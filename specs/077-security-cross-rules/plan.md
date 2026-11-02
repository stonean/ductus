# 077 — Security cross rules Plan

Implements [077 — Security cross rules](spec.md).

## Overview

Create `framework/rules/security-cross.md` with five `SEC-` rules: one new rule (`SEC-LOG-001`) and four moved from `security-backend.md`. The work then has four parts:

- **Register the prefix** in this spec's `data-model.md`, the rule-ID lint and the analyze reference.
- **Deprecate the four old rules**, each reduced to a yield to its successor so `/{project}:analyze` reports a case once.
- **Re-point every yield and Statement** that names them, and sweep every live citation.
- **Ship the file**: the manifest row, `docs/rules.md`, the README, and `runtime/CHANGELOG.md`'s `## [Unreleased]` section.

No runtime code changes. The rule-ID regexes in `runtime/src/primitives/check_rule_ids.rs:103` and `runtime/src/primitives/rule_sections.rs:77` already accept any two-to-five-letter prefix.

## Technical Decisions

### Registering `SEC`

There are three registration sites, and each must agree.

- **This spec's `data-model.md`** registers `SEC-{CATEGORY}-{NNN}` and the categories `LOG`, `DEPS` and `AUTHN`, as 036's does for `QUAL` (`specs/036-quality-cross-rules/data-model.md`, §`QUAL` surface). `framework/reference/analyze.md` ("Which ID format governs a file", item 1) reads a file's introducing-spec data-model first, and a file whose IDs fail it is treated as unloadable.
- **`scripts/lint-rule-ids.sh`** allows only `^(BE|FE|CFG|QUAL)-` (line 45), so `SEC` joins the allowlist. 077's data-model joins the source comment above it (lines 16–18), and the message on line 76 is updated.
- **The analyze reference** lists 077's data-model beside 008's and 017's in item 1. 008's `data-model.md` (§Rule ID format, the paragraph after the category table) names `security-cross.md`'s `SEC` prefix beside `CFG` and `QUAL`.

`security-cross.md`'s header declares the format and the three categories, as `quality-cross.md`'s does (`framework/rules/quality-cross.md:3`).

### `SEC-LOG-001` states its own value set

A frontend-only or cross-only project does not install `security-backend.md` (`framework/bootstrap/ductus.md`, "Rule-file surface filter"), so `SEC-LOG-001` cannot rely on `BE-LOG-002` for its value set. Its Statement names the values `BE-LOG-002` bars, word for word: passwords, full authentication tokens, session IDs, encryption keys, one-time codes, recovery codes, security-question answers, raw PII, full payment card numbers and other sensitive field values. It bars them from a log line, an error-reporting payload and telemetry, in any shape.

Its Verification asks a spec or plan that introduces logging, error reporting or telemetry, client-side or server-side, to commit to excluding or redacting those values. Its Not-flagged clause names three owners:

- a value in a log line is `BE-LOG-002`'s, and one in telemetry, an error-reporting payload or propagated trace context is `BE-LOG-008`'s, each "where `security-backend.md` is also loaded" (008 `data-model.md` §Verification phrasing, item 6);
- a sensitive configuration value is `configuration-cross.md` `CFG-ENV-006`'s, with no condition.

Its Source is the OWASP Logging Cheat Sheet, fetched when the rule is written.

### Moved rules keep their text

`SEC-DEPS-001`, `SEC-AUTHN-001`, `SEC-AUTHN-002` and `SEC-AUTHN-003` take the Statement and Verification of `BE-DEPS-006`, `BE-AUTHN-009`, `BE-AUTHN-013` and `BE-AUTHN-020` as they stand at the move. Only the IDs and the load conditions change. Each owner a moved rule names that sits on a surface file takes the condition on its file:

- `SEC-DEPS-001`: `BE-DEPS-002` takes "where `security-backend.md` is also loaded", and `FE-DEPS-004` takes "where `security-frontend.md` is also loaded";
- `SEC-AUTHN-001`: `BE-API-010` (d) takes the backend condition;
- `SEC-AUTHN-002`: `BE-API-009` takes the backend condition;
- the mutual references between `BE-AUTHN-013` and `BE-AUTHN-020` become `SEC-AUTHN-002` and `SEC-AUTHN-003`, in the same file, so they take none.

The Sources are re-fetched at the move, and any quote that no longer matches is corrected before the rule lands.

### A deprecated rule is a pointer

Each deprecated rule keeps its heading, its ID and its Statement. It gains a `**DEPRECATED in {version}:**` paragraph directly under the heading, the position `runtime/src/primitives/rule_sections.rs`'s `a_deprecated_section_parses_like_any_other` test uses. The paragraph names the successor and the removal version.

Its Verification becomes one sentence naming the successor as the owner of every case the rule governed, with no load condition. `/{project}:analyze` therefore reports each case once, under the successor. The exec walker still asks about the rule, since a deprecated section parses like any other, and the host answers `passed`.

The version is the next one `version` would ship, computed when the labels are written: `0.59.12`, since a release with no primitive change is a patch (`runtime/CHANGELOG.md`, the 0.59.11 entry). The removal target is the second minor release after it, `0.61.0`, as 077's second Resolved Question decided.

### Re-pointing

These name a deprecated rule and change to name its successor:

- `security-backend.md`: the yields in `BE-API-009`, `BE-API-010`, `BE-API-011` and `BE-API-013`, and the Statements of `BE-API-010` (d) and `BE-API-011`. The latter's "Token comparison MUST be constant-time per `BE-AUTHN-009`" becomes per `SEC-AUTHN-001`.
- `security-frontend.md`: the yields in `FE-AUTHN-001` and `FE-STORAGE-003`, which drop "where `security-backend.md` is also loaded", since a `-cross.md` owner installs in every shape.

`configuration-cross.md`'s `CFG-ENV-006` names a non-configuration sensitive value in a log line as `BE-LOG-002`'s, and one in propagated trace context as `BE-LOG-008`'s. It is the only rule outside the backend surface that does so (grep of `framework/rules/` for both IDs). It then reads "`BE-LOG-002`'s where `security-backend.md` is also loaded, otherwise `security-cross.md` `SEC-LOG-001`'s", and the same for `BE-LOG-008`. The trace-context case stays task 43's open item 8 on 008.

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

These keep the old ID, because they record what was decided or measured: 008's `tasks.md`, 022's ticked `tasks.md` items, 008's scenario `rules-are-as-short-as-their-required-content` (checked when swept), changelog entries, and `review.md` and `analysis.md` records. `.github/` changes run `actionlint`.

### Distribution

- `framework/bootstrap/ductus.md` gains the row `framework/rules/security-cross.md` → `specs/rules/security-cross.md` beside `configuration-cross.md`'s (line 858). The surface filter keeps every `*-cross.md` entry unconditionally (line 836), so every shape installs it.
- `docs/rules.md`'s "What ships" table gains a row.
- `README.md`'s security bullet (line 52) names the cross-surface file.

`runtime/CHANGELOG.md`'s `## [Unreleased]` section gains one entry. It names:

- the four ID changes and their successors;
- the deprecation and removal versions;
- that a waiver under an old ID stops applying where the finding moves, since a waiver is anchored on rule ID and file (`framework/reference/review.md`, §Waivers);
- that a project pinning `security-backend.md` sees each moved case under both IDs until it re-syncs the file.

### Verification

Every rule this spec writes or changes is re-read before the work is called done (AC10). The re-read uses fresh agents, split by cluster: logging, dependencies, authentication, and records. Each probes all four project shapes, under the same severity bar 008's rounds used, repeated until a round has no MUST or SHOULD.

The local gate is a fresh clone of HEAD plus the patch, running `cargo test --release --locked` and every step of `framework-checks.yml`, `generators.yml` and `runtime.yml`. `every_shipped_rule_is_assessable` (`runtime/src/primitives/rule_sections.rs`) parses every file under `framework/rules/` and must pass with the new one.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/security-cross.md` | Create | The five `SEC-` rules |
| `specs/077-security-cross-rules/data-model.md` | Create | Registers `SEC` and its categories |
| `framework/rules/security-backend.md` | Modify | Deprecate four rules; re-point `BE-API-009`, `-010`, `-011`, `-013` |
| `framework/rules/security-frontend.md` | Modify | Re-point `FE-AUTHN-001` and `FE-STORAGE-003`, dropping their load condition |
| `framework/rules/configuration-cross.md` | Modify | `CFG-ENV-006` names `SEC-LOG-001` where `security-backend.md` is not loaded |
| `scripts/lint-rule-ids.sh` | Modify | Allow `SEC` |
| `framework/reference/analyze.md` | Modify | Name 077's data-model as `SEC`'s format source |
| `specs/008-security-rules/data-model.md` | Modify | Name the `SEC` prefix; sweep `BE-DEPS-006` |
| `framework/bootstrap/ductus.md` | Modify | Manifest row |
| `framework/bootstrap/govern.md` | Modify | Byte copy of `ductus.md`, which audit family `transitional-bootstrap-parity` requires |
| `docs/rules.md` | Modify | "What ships" row |
| `README.md` | Modify | Security bullet |
| `runtime/CHANGELOG.md` | Modify | `## [Unreleased]` entry |
| `.github/workflows/framework-checks.yml` | Modify | Comment sweep |
| `.github/workflows/runtime-release.yml` | Modify | Comment sweep |
| `scripts/check-runtime-currency.sh` | Modify | Comment sweep |
| `framework/templates/ci/adopter-generators.yml` | Modify | Comment sweep |
| `specs/017-derive-dont-ask/plan.md` | Modify | Sweep |
| `specs/017-derive-dont-ask/scenarios/the-adopter-ci-template-can-run.md` | Modify | Sweep |
| `specs/017-derive-dont-ask/spec.md` | Modify | Block-quoted signpost discharging 077's impact |
| `specs/022-deterministic-runtime/plan.md` | Modify | Sweep |
| `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-rule-it-asks-about.md` | Modify | Sweep |
| `specs/008-security-rules/spec.md` | Modify | Block-quoted signpost discharging 077's impact |
| `specs/008-security-rules/tasks.md` | Modify | Tick task 32's follow-on, task 40's first, fourth and fifth items, and task 42's item |

## Data Model

`data-model.md` registers the `SEC` surface:

- its prefix, its ID grammar and its three categories;
- the rule set, with each rule's tier and, for a moved rule, its predecessor;
- a pointer to 008's `data-model.md` for the rule schema, which it does not restate.

## Trade-offs

- **`SEC-LOG-001` restates `BE-LOG-002`'s value list rather than citing it.** Citing it would leave the rule empty where `security-backend.md` is not installed. The cost is two copies of one list, and a change to either must change both. The rule's Not-flagged clause and this plan name the pair, so a reviewer of one finds the other.
- **A deprecated rule's Verification is replaced by a pointer, not kept.** Keeping it would report every moved case under both IDs at `/{project}:analyze` in backend-only and full-stack projects until removal. Its Statement is kept, so `/{project}:review`, which applies Statements and no routing, still reports both until 020 task 15 lands (spec §Edge cases).
- **`CFG-ENV-006` names both owners directly** rather than naming `SEC-LOG-001` alone and letting that rule yield onward. A direct name reaches the owner in one step, and a chain of yields is what 008's rounds found hardest to read alone.
- **The deprecation version is computed when the labels are written** (`0.59.12`, removed in `0.61.0`). A release cut under another number must correct the four labels, and nothing checks that today. 008's task 43 item on the `## [Unreleased]` rename names the same release-time gap.
- **A project that pins `security-backend.md` sees doubled findings** until it re-syncs. Nothing a shipped file says can reach a pinned copy, so the changelog entry is the remedy (spec §Edge cases).

## Open Questions Resolved

- **The ID prefix** is `SEC`, registered in `data-model.md` and in the three registration sites above.
- **The removal version** is the second minor release after the shipping one, written into each `**DEPRECATED in {version}:**` label.

## Cross-spec impact

- **008-security-rules.** Its rule files change (deprecations and re-pointed yields), and so does its `data-model.md` (the `SEC` prefix and the sweep). Its tasks 32, 40 and 42 close here. Discharged by a block-quoted signpost in 008's `spec.md` linking this spec. The block quote keeps `derive-dependencies` from inducing an `008 → 077` edge, which would close a cycle, since 077 depends on 008.
- **017-derive-dont-ask.** `CFG-ENV-006`, a rule on its surface, changes, and its `plan.md` and scenario are swept. Discharged by a block-quoted signpost in 017's `spec.md`.
- **022 and 060.** Their swept sentences are mechanical substitutions (constitution §spec-lifecycle, case (a)), owe nothing to 077, and take no back-edge.
