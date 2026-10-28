---
title: "008-security-rules — data-model"
---

# 008 — Security Rules Data Model

## Rule file structure

A rule file is a markdown document with the following structure:

```markdown
# Security Rules — {Surface}

{Introduction: the file's scope and how its rules are enforced; the ID format and category list; a pointer to constitution §rules for severity, ID permanence, and how review selects or excludes the file.}

## {ID prefix} — {Category Name}

### {Rule ID}

> {Rule statement using RFC 2119 keywords.}

**Verification:** {Instruction to `/{project}:analyze` on how to check the rule.}

### {Rule ID}

…
```

- **Surface** is `Backend` or `Frontend`.
- **ID prefix** is the `{surface}-{category}` pair the section's rules share (`BE-AUTHN`, `FE-XSS`), so the heading carries the abbreviation a reader needs to cite a rule under it.
- **Category Name** is the section's human-readable title in title case (`Authentication`, `Input Validation`, `Cross-Site Scripting Prevention`). It **names the same category** as the spec's `## Two Rule Files` list and the abbreviation table below, but is **not byte-identical** to either: the shipped headings title-case the words and several are longer than the spec's label (`Cross-Site Scripting Prevention` for `Cross-site scripting (XSS)`, `Secure Client-Side Storage` for `Secure storage`, `Content Security Policy` for `Content security`). The binding identity is the **abbreviation**, not the prose title — an earlier wording of this bullet claimed the titles matched the spec's list "verbatim", which was never true of 12 of the 15.
- **Rule ID** appears as a level-3 heading and is the only level-3 heading content (no surrounding text). This makes rules grep-able by ID.

## Rule ID format

```text
{surface}-{category}-{NNN}
```

| Element | Type | Constraints |
| --- | --- | --- |
| `surface` | string | `BE` (backend) or `FE` (frontend). Uppercase. |
| `category` | string | Short uppercase alphanumeric abbreviation (`[A-Z][A-Z0-9]*`) drawn from the per-surface set below or extended by a later rule-introducing spec. |
| `NNN` | integer | Zero-padded sequence number, starting at `001`. Never renumbered. Never reused after removal. |

### Category abbreviations

| Surface | Category | Abbreviation |
| --- | --- | --- |
| BE | Authentication | `AUTHN` |
| BE | Authorization | `AUTHZ` |
| BE | Input validation | `INPUT` |
| BE | Data protection | `DATA` |
| BE | API security | `API` |
| BE | Logging and audit | `LOG` |
| BE | Dependency management | `DEPS` |
| BE | Error handling | `ERR` |
| FE | Cross-site scripting (XSS) | `XSS` |
| FE | Cross-site request forgery (CSRF) | `CSRF` |
| FE | Secure storage | `STORAGE` |
| FE | Authentication UX | `AUTHN` |
| FE | Content security | `CSP` |
| FE | Dependency management | `DEPS` |
| FE | Sensitive data handling | `PII` |

`AUTHN` (authentication) and `AUTHZ` (authorization) are deliberately distinct abbreviations to prevent collision; both surfaces use `AUTHN` for their authentication-related rules. `AUTHN` and `DEPS` are shared across surfaces (`BE-AUTHN` and `FE-AUTHN` are different namespaces). The full ID always includes the surface prefix to disambiguate.

The table is the category set of the two security files, `security-backend.md` and `security-frontend.md`. Every other rule file declares its ID prefix and categories in its own header (constitution §rules): `api-backend.md` declares `SCHEMA`, `APIVER`, and the rest; `configuration-cross.md` uses the `CFG` prefix registered in `specs/017-derive-dont-ask/data-model.md`, and `quality-cross.md` the `QUAL` prefix registered in `specs/036-quality-cross-rules/data-model.md`. Every rule ID, in any file, follows one generic grammar: an uppercase prefix of two to five letters, an `[A-Z][A-Z0-9]*` category abbreviation, and a zero-padded sequence number of three or four digits. `check-rule-ids` resolves citations against that shape, requiring a category of at least two characters, which every shipped category has (`specs/036-quality-cross-rules/data-model.md` records the difference).

## Rule entry fields

| Field | Required | Format | Notes |
| --- | --- | --- | --- |
| Rule ID | yes | Level-3 heading (`### {ID}`) | Matches the format above. The heading contains nothing but the ID. |
| Statement | yes | Block quote (`> …`) | The rule's obligation, stated with RFC 2119 keywords of one tier — MUST and MUST NOT, or SHOULD and SHOULD NOT — and optionally MAY for a permission or scope boundary. See **Severity classification** below. |
| Verification | yes | Paragraph beginning `**Verification:**` | Instruction to `/{project}:analyze` — see **Verification phrasing** below. |
| Source | no | Paragraph beginning `**Source:**` | Citation to authoritative origin (e.g., OWASP cheat sheet name, RFC number, NIST publication, CWE entry), never an account of how the rule came about. Optional but recommended. A constitution section is named in prose (`constitution §grounding`), never linked: a rule file ships to `specs/rules/` and the constitution to `.ductus/constitution.md`, so a repo-relative link dangles in an adopter tree. Every claim a rule makes about an external standard, specification, or product behavior — in any field, not only Source — is checked against the cited document's current text when the rule is written or amended: fetched, never recalled. A claim that cannot be verified that way is rewritten to what can be, or removed; it is not landed on recall. Nothing enforces this mechanically; it is a discipline for rule authors and for anyone reviewing a rule file. |
| Deprecated | no | Paragraph beginning `**DEPRECATED in {version}:**` | Present only on deprecated rules. Includes the removal target version. The rule remains in the file with this label until removed. |

## Verification phrasing

The Verification field is read by `/{project}:analyze` during an analyze run. It must:

1. Identify the project artifacts in scope (typically: feature specs, plans, `specs/system.md`).
2. Describe the trigger that makes the rule applicable to a given artifact (e.g., "any spec that introduces credential storage", "any plan that handles file uploads"). A rule whose trigger does not fire for any artifact is silently inert (no finding emitted).
3. State what the artifact MUST or SHOULD include when the trigger fires (e.g., "specify the hashing algorithm by name", "describe the rate-limit threshold").
4. Distinguish documentation commitments from code patterns when the rule's enforcement happens outside the repository (runtime config, infrastructure, deployment).
5. Name the sibling rule that owns an overlapping defect, where one exists, so a finding is reported under one ID.

### Examples

**Code-pattern Verification:**

```text
Verification: Any spec or plan that introduces credential storage MUST
specify the hashing algorithm by name. /{project}:analyze searches feature
specs and plans for credential/password/auth keywords; for each match,
flags the artifact if it does not name a deliberately slow
password-hashing algorithm (Argon2id, scrypt, bcrypt, or PBKDF2) and
its cost parameters.
```

**Documentation-commitment Verification (runtime/infra):**

```text
Verification: specs/system.md or a deployment-related spec MUST describe
how TLS is terminated (load balancer, application, or sidecar) and the
expected protocol/cipher policy. /{project}:analyze flags the absence of
any TLS handling commitment in the project's specs.
```

## Severity classification

The Statement's RFC 2119 keyword determines the severity `/{project}:analyze` reports:

| Keyword | Severity | Reporting |
| --- | --- | --- |
| MUST, MUST NOT | Error | Blocking |
| SHOULD, SHOULD NOT | Warning | Non-blocking |

A Statement's obligation keywords MUST come from one tier: MUST and MUST NOT, or SHOULD and SHOULD NOT. Keywords of both tiers in one Statement (e.g., "MUST … SHOULD") are not permitted, because a review waiver and cross-pass dedup both key on the rule ID and would treat a blocking clause and an advisory one as one finding. Split such a rule into two entries: the MUST clauses keep the existing ID, and the SHOULD clauses take the next unused sequence number in the category. A SHOULD clause that its own MUST clause already implies is dropped rather than split off, since the rule it would mint could never fail on its own.

MAY is permitted beside either tier, to state a permission or a scope boundary (e.g., "offset pagination MAY be used only for append-only collections"). It states no obligation and carries no tier, so a Statement using MAY alone has no tier and cannot be assessed.

The shipped rule files are held to this by the runtime's corpus test (`no_shipped_statement_mixes_tiers` in `runtime/src/primitives/rule_sections.rs`). A rule file a project authors, or a pinned copy of a shipped one, is not checked; `/{project}:analyze` reads a Statement there that carries both tiers as MUST-tier.

A MUST or MUST NOT obligation never requires continuous integration — neither that a project run CI nor that a check run in it. Whether a project has CI, and what it runs there, is its own choice: `ductus` ships its own CI checks only as a template an adopter may copy (`framework/templates/ci/adopter-generators.yml`), and a spec or plan cannot show how a project's CI behaves, so `/{project}:analyze` has no artifact against which to verify such an obligation. The MUST states the outcome instead, and where running a check in automation on every change is worth recommending, the recommendation is a SHOULD of its own, under the next unused number in the category — `BE-SCHEMA-003` (the committed schema must match the implementation) and `BE-SCHEMA-004` (that check should run automatically on every change) are the pattern. A rule MAY still constrain the inputs a project's build or CI uses when it has one — `BE-DEPS-006` requires CI actions and base images to be pinned to immutable references — because that governs what runs, not whether CI exists. Decided by the project owner, 2026-10-19. Nothing enforces this mechanically; it is a discipline for rule authors and for anyone reviewing a rule file.

## ID stability invariants

These invariants are enforced by `/{project}:analyze` (as edge cases in `spec.md`) and are also a discipline for rule authors:

- Once an ID is assigned, the rule retains that ID for life. Editing the Statement or moving the rule within the file does not change its ID.
- Deprecated rules retain their ID. They are removed only after the deprecation window has passed and references in adopting projects have been updated.
- Sequence numbers are never reused after a rule is fully removed. New rules in a category get the next unused number.
- Two rules in the same file MUST NOT share an ID. `/{project}:analyze` refuses to load a file with duplicate IDs.

## Rule file integrity

A rule file is considered well-formed if every rule heading, statement, and verification field is present and the ID format is satisfied. A rule carries no Rationale: its Statement stands without a stated reason, and a boundary with a sibling rule is stated in its Verification. A `**Rationale:**` paragraph in a project-authored or pinned file is ignored, never malformed. Missing fields, malformed IDs, or unparseable content cause `/{project}:analyze` to refuse to load the file (per the spec's edge-case decisions). The file is then treated as absent for matching purposes, but the parse failure itself is a hard error.
