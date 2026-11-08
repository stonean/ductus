# 077 — Security cross rules Data Model

Registers the `SEC` rule-ID surface and its three categories. It does not restate the rule-file schema, which `specs/008-security-rules/data-model.md` defines once: Statement and Verification, `### {ID}` headings, permanent IDs, one obligation tier per Statement, and the `**DEPRECATED in {version}:**` label.

## `SEC` surface

`SEC` is a rule-ID surface beside `BE` and `FE` (`specs/008-security-rules/data-model.md`), `CFG` (`specs/017-derive-dont-ask/data-model.md`) and `QUAL` (`specs/036-quality-cross-rules/data-model.md`).

| Property | Value |
| --- | --- |
| Rule file | `framework/rules/security-cross.md`, installed in every project shape |
| Surface prefix | `SEC` (always uppercase) |
| ID grammar | `SEC-{CATEGORY}-{NNN}`, accepted by `scripts/lint-rule-ids.sh`'s surface allowlist (extended to `SEC` by this spec) and by the runtime's `check-rule-ids` harvester |

The prefix disambiguates `SEC-` IDs from `BE-` and `FE-` IDs, so a rule on every surface never shares a namespace with a security rule on one surface.

## `SEC` rule ID format

```text
SEC-{category}-{NNN}
```

| Element | Type | Constraints |
| --- | --- | --- |
| `SEC` | literal | Always uppercase. The fixed prefix for security rules that apply on every surface. |
| `category` | string | One of the abbreviations below, or one a later rule-introducing change registers here. |
| `NNN` | integer | Zero-padded sequence number from `001`, per category. Never renumbered, never reused after removal. |

### Category abbreviations

| Category | Abbreviation |
| --- | --- |
| Logging, error reporting and telemetry | `LOG` |
| Build inputs and dependencies | `DEPS` |
| Authentication and OAuth/OIDC client flows | `AUTHN` |
| Sensitive values in URLs | `DATA` |

## Rule set

| ID | Tier | Subject | Predecessor |
| --- | --- | --- | --- |
| `SEC-LOG-001` | MUST | A sensitive value is kept out of a log line, an error-reporting payload and telemetry, a configuration value being `CFG-ENV-006`'s at `/{project}:analyze` | none (new; 008 task 32) |
| `SEC-DEPS-001` | MUST | Build inputs the lockfile does not cover are pinned to an immutable reference | `BE-DEPS-006` |
| `SEC-AUTHN-001` | MUST | Secrets and signatures are compared in constant time | `BE-AUTHN-009` |
| `SEC-AUTHN-002` | MUST | OAuth and OIDC client flows use a bound `state`, PKCE, ID-token validation, an exact `redirect_uri`, unguessable transaction-bound values, a mix-up defence and an external user-agent | `BE-AUTHN-013` |
| `SEC-AUTHN-003` | MUST | OAuth grant and response types, the PKCE method and its enforcement, public-client refresh-token protection, and an authorization server's redirect of a credential-carrying request | `BE-AUTHN-020` |
| `SEC-AUTHN-004` | MUST | An OAuth authorization server matches each `redirect_uri` it receives exactly | `BE-API-009`'s authorization-server sentences (008 task 43) |
| `SEC-DATA-001` | MUST | A credential or PII stays out of a URL the project's code requests, sends or renders, with `BE-API-013`'s carve-outs | none (new; 008 task 43) |

Each predecessor is labelled `**DEPRECATED in {version}:**` in `security-backend.md`, with the second minor release after the shipping one as its removal target (077's Resolved Questions). It names its successor as the owner of every case it governed until it is removed.
