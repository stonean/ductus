# 017 — Derive, Don't Ask Data Model

Defines the structures introduced or modified by this spec: the configuration rule file format, the updated spec/scenario frontmatter schema, and the marker-comment convention for generated content blocks.

## Configuration rule file structure

A configuration rule file is a markdown document with the following structure (mirrors the security-rule file precedent declared in `specs/008-security-rules/data-model.md`):

```markdown
# Configuration Rules

{Introduction, one short paragraph each: the file's scope; the RFC 2119 severity mapping; the ID format and category list; how the rules are enforced; how a project's configuration selects or excludes the file.}

## {ID prefix} — {Category Name}

### {Rule ID}

> {Rule statement using RFC 2119 keywords.}

**Rationale:** {Threat the rule mitigates.}

**Verification:** {Instruction to `/{project}:analyze` on how to check the rule.}

**Source:** {Optional citation to authoritative origin.}

### {Rule ID}

…
```

- **ID prefix** is `CFG-CONST`, `CFG-ENV`, or `CFG-FLAG` — the `CFG-{category}` pair the section's rules share, so the heading carries the abbreviation a reader needs to cite a rule under it.
- **Category Name** is `Constants`, `Environment and configuration values`, or `Feature flags`.
- **Rule ID** appears as a level-3 heading and is the only level-3 heading content (no surrounding text). This makes rules grep-able by ID.

## Configuration rule ID format

```text
CFG-{category}-{NNN}
```

| Element | Type | Constraints |
| --- | --- | --- |
| `CFG` | literal | Always uppercase. The fixed prefix for configuration rules. |
| `category` | string | Short uppercase abbreviation drawn from the set below. |
| `NNN` | integer | Zero-padded sequence number, starting at `001`. Never renumbered. Never reused after removal. |

### Category abbreviations

| Category | Abbreviation |
| --- | --- |
| Constants | `CONST` |
| Environment and configuration values | `ENV` |
| Feature flags | `FLAG` |

The full ID always includes the `CFG-` prefix to disambiguate from the `BE-` (backend) and `FE-` (frontend) rule IDs of the surface rule files.

## Configuration rule entry fields

| Field | Required | Format | Notes |
| --- | --- | --- | --- |
| Rule ID | yes | Level-3 heading (`### {ID}`) | Matches the format above. The heading contains nothing but the ID. |
| Statement | yes | Block quote (`> …`) | The rule's obligation, stated with RFC 2119 keywords of one tier — MUST and MUST NOT, or SHOULD and SHOULD NOT — and optionally MAY for a permission or scope boundary. See **Severity classification** below. |
| Rationale | yes | Paragraph beginning `**Rationale:**` | Brief explanation of the threat or risk the rule mitigates. |
| Verification | yes | Paragraph beginning `**Verification:**` | Instruction to `/{project}:analyze` — see **Verification phrasing** below. |
| Source | no | Paragraph beginning `**Source:**` | Citation to authoritative origin (e.g., 12-Factor App, NIST SP 800, ISO 80000-3 for units of time). Optional but recommended. |
| Deprecated | no | Paragraph beginning `**DEPRECATED in {version}:**` | Present only on deprecated rules. Includes the removal target version. The rule remains in the file with this label until removed. |

## Verification phrasing

Same rules as the security-rule file (see `specs/008-security-rules/data-model.md` § Verification phrasing). The Verification field must:

1. Identify the project artifacts in scope (typically: feature plans, source code via spec-stated affected files, `specs/system.md` configuration sections).
2. Describe the trigger that makes the rule applicable to a given artifact.
3. State what the artifact MUST or SHOULD include when the trigger fires.
4. Distinguish documentation commitments from code patterns when the rule's enforcement happens outside the repository.

## Severity classification

Same as security rules:

| Keyword | Severity | Reporting |
| --- | --- | --- |
| MUST, MUST NOT | Error | Blocking |
| SHOULD, SHOULD NOT | Warning | Non-blocking |

A Statement's obligation keywords MUST come from one tier: MUST and MUST NOT, or SHOULD and SHOULD NOT. Keywords of both tiers in one Statement are not permitted; split such rules into two entries. MAY may appear beside either tier as a permission or scope boundary, and carries no tier. The canonical statement, including how a split assigns IDs, is `specs/008-security-rules/data-model.md` § Severity classification.

## ID stability invariants

Same as security rules. Once an ID is assigned, the rule retains that ID for life. Editing the Statement or moving the rule within the file does not change its ID. Deprecated rules retain their ID. Sequence numbers are never reused after a rule is fully removed. Two rules in the same file MUST NOT share an ID.

## Rule set

`framework/rules/configuration-cross.md` shipped its first version with seven rules (`CFG-CONST-001`–`003`, `CFG-ENV-001`–`004`). `CFG-CONST-004` and `CFG-ENV-005` were added on 2026-05-09 and `CFG-ENV-006` and `CFG-ENV-007` on 2026-05-16, both times without this data model being updated; they are registered here as of 2026-10-19. The rule-file review of 2026-10-19 added `CFG-ENV-008`–`010` and the `FLAG` category with `CFG-FLAG-001` and `CFG-FLAG-002`, through this spec's back-edge per [§rules](../../framework/constitution.md#rules); a second pass the same day added `CFG-ENV-011` and `CFG-ENV-012` to the existing `ENV` category and amended `CFG-ENV-001`, `002`, `004` and `008`. On 2026-10-20 the owner widened `CFG-ENV-001`'s declaration and `CFG-ENV-003`'s startup validation from environment variables to configuration values from any source ([required-config-values-from-any-source](scenarios/required-config-values-from-any-source.md)), and `CFG-ENV-005`'s range check the same way; it widened `CFG-ENV-002`'s inventory to configuration-file keys, leaving command-line flags to their help output ([ranged-and-inventoried-config-values-from-any-source](scenarios/ranged-and-inventoried-config-values-from-any-source.md)); and the file's header now says a command-line flag is a configuration value only when it supplies a setting that could equally come from an environment variable, a configuration file, or a default, leaving an argument that names what one invocation acts on, or how, to the interface's help output ([required-config-values-from-any-source](scenarios/required-config-values-from-any-source.md)). Numbering is permanent. This list enumerates the shipped set and is therefore a claim of its own — extend it whenever a rule is added, per [§grounding](../../framework/constitution.md#grounding).

### CFG-CONST namespace (Constants category)

- `CFG-CONST-001` (MUST) — a value used by more than one module lives in one centralized location idiomatic to the project's language, not duplicated across modules
- `CFG-CONST-002` (MUST) — a constant used by one module lives within that module (its own constants file, or the top of the file that uses it), not in the shared location
- `CFG-CONST-003` (MUST) — every operator-tunable value (timeout, retry count, batch size, threshold, rate limit, expiry duration) is backed by a named constant or by a configuration value declared per `CFG-ENV-001`, never a bare literal in business logic
- `CFG-CONST-004` (MUST) — a tunable value's valid-range bounds are named constants defined alongside its default constant

### CFG-ENV namespace (Environment and configuration values category)

- `CFG-ENV-001` (MUST) — every configuration value, whether an env var, a configuration-file key or a command-line flag, is declared one of three ways: required, with no default; optional, with a default fallback that is a named constant; or optional with no default, where an absent or empty value disables a feature or integration the declaration names; secrets carry no in-application default, so a secret is required, or optional with no default when its absence disables the integration it authenticates; every env var is read once at startup and cached
- `CFG-ENV-002` (MUST) — a canonical inventory, kept with the source, lists every env var and every configuration-file key the application reads, with its purpose, whether it is required, and a safe placeholder or default — one inventory per kind of source or one for both, never two for one source; its form follows the deployment model (`.env.example` where a dotenv-style loader reads configuration, a Helm chart's `values.yaml` for defaults with its `values.schema.json` for types and required keys, a commented example of the configuration file, a typed config module, or an equivalent checked-in manifest); command-line flags are left to the interface's help output
- `CFG-ENV-003` (MUST) — required configuration values, from any source, are validated at startup; one that is absent — an env var unset or empty, a file key missing or empty, a flag not given or empty, and for a value read from several sources absent at every one of them — fails fast, exiting non-zero with an error naming the setting, and the file for a file key
- `CFG-ENV-004` (MUST) — a time-valued env var states its unit unambiguously, either in the value (a duration string whose parser rejects a non-zero value without a unit — preferred) or in the name (a `_MS`, `_SECONDS`, `_MINUTES` or `_HOURS` suffix on an integer value); its default constant is a typed duration or carries the matching suffix
- `CFG-ENV-005` (MUST) — a configuration value from any source, env var, configuration-file key or command-line flag, with a documented valid range (bounds named per `CFG-CONST-004`) is checked against it at startup; an out-of-range value fails fast, naming the setting, and the file for a file key, the value, and the violated bound
- `CFG-ENV-006` (MUST) — sensitive configuration values are redacted at the config-loading layer before they reach logs, error messages, diagnostic dumps, health-check responses, or any other operator-facing output
- `CFG-ENV-007` (MUST) — configuration precedence is documented and uniform: command-line flag, then environment variable, then configuration file, then in-code default constant; a deviation is justified in `specs/system.md`
- `CFG-ENV-008` (MUST) — every configuration value, whether from an env var, a file, or a flag, is parsed into its declared type at startup; a present but malformed value fails fast, naming the setting and the value (redacted per `CFG-ENV-006` when sensitive), and never falls back to the default or is coerced to zero, empty, or `false`
- `CFG-ENV-009` (SHOULD) — a value that varies between deployments (backing-service locators, external endpoints, per-deploy hosts and ports) is supplied at deploy time rather than compiled in, and application code does not branch on the deployment environment's name; build tooling that keys on a standard mode variable may still read it
- `CFG-ENV-010` (MUST) — no secret is injected into an artifact delivered to end users (a browser bundle, a mobile app binary, a distributed desktop or CLI build), and the build exposes to client code only the variables explicitly designated for it, never the whole build environment; values their issuer designs to be public may be injected
- `CFG-ENV-011` (MUST) — a data-size env var (a byte count such as a buffer, request-body, upload, cache, or memory limit) states its unit unambiguously, either in the value (a size string whose parser rejects a non-zero value without a unit) or in the name (a unit suffix such as `_BYTES`, `_KIB` or `_MIB` on an integer value); under either pattern a unit whose multiplier differs between parsers (`KB`, `MB`, `GB`, `TB`, `k`, `M`, `G`, `T`) has its multiplier — powers of 1000 or of 1024 — stated in the variable's `CFG-ENV-002` inventory entry; its default constant is a typed size or carries the matching suffix
- `CFG-ENV-012` (SHOULD) — an unrecognized key in a configuration file, within the settings the application owns, fails startup with an error naming the key and the file; where one file is read by more than one version of the application at once (a rolling deploy, older binaries reading the same file), it instead produces a startup warning naming the key; an unrecognized key is never silently ignored, and sections of a shared file that belong to other tools may pass unexamined

### CFG-FLAG namespace (Feature flags category)

- `CFG-FLAG-001` (MUST) — every feature flag has a declared default value, and evaluation resolves to it — never throwing, blocking indefinitely, or yielding an undefined state — when the flag source is unreachable, times out, or returns a value of the wrong type
- `CFG-FLAG-002` (SHOULD) — a flag gating a release, rollout, or experiment is recorded with an owner and a removal criterion when introduced, and its removal is tracked as a work item; a long-lived operational flag declared permanent may omit the removal criterion

## Frontmatter schema (after this spec)

Updates the schema declared in `framework/constitution.md` §text-first-artifacts.

### Spec files (`spec.md`, `spec-and-plan.md`)

| Field | Required | Type | Allowed values | Notes |
| --- | --- | --- | --- | --- |
| `status` | yes | string | `draft`, `clarified`, `planned`, `in-progress`, `done` | Spec lifecycle state |
| `dependencies` | yes | list of strings | spec slugs (e.g., `002-events`); empty list permitted | **Generated** from body inline links; not hand-authored. Delivered here as `gen-spec-deps.sh`; since `067-analyze-and-audit-checks`'s `adopter-generator-promotion` it is the `derive-dependencies` runtime primitive (AC23) |

Removed: `tags` (deleted from schema and from `/{project}:analyze`). The `track` field on `spec-and-plan.md` is also removed; track is inferred from the filename.

### Scenario files (`scenarios/{slug}.md`)

| Field | Required | Type | Notes |
| --- | --- | --- | --- |
| `section` | yes | string | The parent spec section the scenario elaborates. Parent feature is implicit in the file path. |

Removed: `spec-ref` (replaced by `section`). `tags` removed.

### Other artifact files

`plan.md`, `tasks.md`, `data-model.md`, and `research.md` have no required frontmatter fields. The `title` field is removed from all template defaults.

### Open-schema rule (unchanged)

Additional fields beyond those listed above are permitted and ignored by uninterested consumers. Stale fields in done specs (`title`, `tags`, `spec-ref`, `track`) remain valid under this rule and produce no `/{project}:analyze` findings after this spec.

## Marker-comment convention for generated content blocks

Generated content blocks inside otherwise hand-authored files are bounded by HTML marker comments:

```markdown
<!-- generated:{name}:start -->
{generated content}
<!-- generated:{name}:end -->
```

Where `{name}` identifies the block type. The generator scripts find the markers, splice the regenerated content between them, and leave everything outside untouched. If the markers are absent, the generator emits a warning and exits non-zero — it never appends or guesses.

### Initial marker names

| Marker name | Where it lives | Generated by |
| --- | --- | --- |
| `feature-specs` | `README.md` | `scripts/gen-readme-table.sh` — **retired** with the generated table (AC10) |
| `commands-pipeline` | `framework/commands/help.md` | `scripts/gen-help-tables.sh` |
| `commands-refine` | `framework/commands/help.md` | `scripts/gen-help-tables.sh` |
| `commands-brownfield` | `framework/commands/help.md` | `scripts/gen-help-tables.sh` |
| `commands-orient` | `framework/commands/help.md` | `scripts/gen-help-tables.sh` |
| `commands-bootstrap` | `framework/commands/help.md` | `scripts/gen-help-tables.sh` |

The refine group was named `commands-elaborate` here when this spec shipped, after the `/elaborate` command it rendered; `023-govern-refinement` consolidated that command into `/amend` and the marker became `commands-refine`. The name above is the one `gen-help-tables.sh` and `help.md` actually carry.

## Hook sentinel comment

The shipped adopter pre-commit hook contains a single sentinel line near the top:

```bash
# managed-by: ductus
```

`/ductus`'s hook-installation logic uses the presence of this sentinel to distinguish a ductus-installed hook from a hand-rolled one. When the sentinel is present, `/ductus` treats the file as `update`-strategy and overwrites it on subsequent runs (subject to `.ductus/config.toml` `[pinned] files`). When the sentinel is absent, `/ductus` skips installation and warns the user.

**Post-018:** the sentinel still exists and still decides this, but it moved with the ownership split described in the signpost at the top of `spec.md`. `018-adopter-owned-pre-commit` made `framework/bootstrap/hooks/pre-commit` an adopter-owned `create`-strategy outer stub carrying **no** sentinel, and put the ductus-owned orchestration in `.githooks/ductus-pre-commit`, which carries `# managed-by: ductus` on line 2. `/ductus` also reads that line to recognize a spec-017-era hook and migrate it. Pinning is `.ductus/config.toml`.

## File extension and shebang conventions

All generator scripts and hook scripts:

- Generator scripts use the `.sh` extension; hook scripts carry the name git invokes them by (`pre-commit`, and the adopter's `ductus-pre-commit`), with no extension
- Begin with `#!/usr/bin/env bash`
- Follow `set -euo pipefail` for fail-fast behavior
- Are executable (`chmod +x`)
