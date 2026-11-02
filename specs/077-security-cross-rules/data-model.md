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

## Rule set

| ID | Tier | Subject | Predecessor |
| --- | --- | --- | --- |
| `SEC-LOG-001` | MUST | A sensitive value that is not configuration is kept out of a log line, an error-reporting payload and telemetry | none (new; 008 task 32) |
| `SEC-DEPS-001` | MUST | Build inputs the lockfile does not cover are pinned to an immutable reference | `BE-DEPS-006` |
| `SEC-AUTHN-001` | MUST | Secrets and signatures are compared in constant time | `BE-AUTHN-009` |
| `SEC-AUTHN-002` | MUST | OAuth and OIDC client flows use `state`, PKCE, `nonce` validation and an exact `redirect_uri` | `BE-AUTHN-013` |
| `SEC-AUTHN-003` | MUST | OAuth grant and response types, the PKCE method and public-client refresh-token protection | `BE-AUTHN-020` |

Each predecessor is labelled `**DEPRECATED in {version}:**` in `security-backend.md`, with the second minor release after the shipping one as its removal target (077's Resolved Questions). It names its successor as the owner of every case it governed until it is removed.
