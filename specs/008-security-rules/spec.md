---
title: "008-security-rules — spec"
status: in-progress
dependencies: [007-govern-workflow]
tags: [security, format]
next-criterion: 29
cross-spec-impact: [016-cross-cutting-rules, 017-derive-dont-ask, 034-performance-backend-rules, 036-quality-cross-rules, 037-observability-backend-rules, 038-concurrency-backend-rules, 039-reliability-backend-rules, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules]
---

# 008 — Security Rules

> **Signpost:** 008 defines the *security instance* of the general rules tier later formalized in [016 — Cross-Cutting Rules](../016-cross-cutting-rules/spec.md). The rule-file format, ID conventions, and `/{project}:analyze` enforcement defined here remain the canonical reference for any future rule file (observability, performance, accessibility, etc.). See §rules in `framework/constitution.md` for the general framing of rules as a cross-cutting artifact tier alongside specs and scenarios.

Comprehensive, enforceable security rules for backend and frontend development. Distributed to adopting projects via the ductus command as two files — `security-backend.md` and `security-frontend.md`. These are rules, not guidelines: `/{project}:analyze` checks implementation against them.

## Motivation

The constitution lists "Secure" as a guiding principle but does not operationalize it. Projects adopting `ductus` have no concrete security rules to follow or validate against. Each project reinvents its own security posture, leading to inconsistency and gaps.

Security rules belong at the governance level because they are cross-cutting — the same rules apply regardless of language, framework, or domain. Project-specific security decisions (which auth provider, which encryption library) belong in the project's `system.md` or feature specs. Governance defines *what* must be secured and *how* to think about it; projects decide the implementation.

## Two Rule Files

Security rules are split by attack surface:

### Backend (`security-backend.md`)

Rules for server-side code, APIs, data persistence, and infrastructure integration.

**Categories:**

- **Authentication** — credential storage (with bcrypt only in a legacy system where neither Argon2id nor scrypt is available), password policy, session management, token handling, password-reset and account-recovery flows, re-authentication before a password or other authenticator, the email or recovery contact, or the MFA settings change, JWT verification (algorithms, claims, and key material), OAuth/OIDC flows and the grant types they must not use, multi-factor authentication (required for privileged accounts, with phishing-resistant factors recommended for them), constant-time comparison of secrets
- **Authorization** — permission enforcement, privilege escalation prevention, default-deny posture, mass-assignment / over-posting prevention
- **Input validation** — sanitization at system boundaries, reject-by-default, allowlisting over denylisting, explicit size and count limits on input, parameterized queries (SQL/NoSQL/command) with each NoSQL query value type-checked so an object cannot inject an operator, safe deserialization, path-traversal prevention through a containment check on whole path components, XXE/SSTI prevention, formula escaping in CSV and spreadsheet exports, HTTP request smuggling prevention at the edge (ambiguous framing rejected, `Content-Length` values that differ included, identical repeats collapsed to one value), SSRF prevention on outbound URL fetches — checked on the connection actually made, not only on the submitted URL
- **Data protection** — encryption at rest and in transit, secrets management, revocation and replacement of leaked secrets, PII handling, key rotation
- **API security** — security headers (HSTS / HTTPS enforcement, CSP, frame protection, referrer policy, cache-control for sensitive responses, an explicit content type) and suppression of version-revealing headers, CORS policy, rate limiting, HTTP method and `Content-Type` restriction, infrastructure management interfaces and database ports kept off public networks, the application's own management and diagnostic endpoints (metrics, profiling, dumps, framework management endpoints) kept off them unless every request is authenticated, redirect validation, signed and replay-protected webhooks, server-side CSRF enforcement for ambient credentials (a synchronizer or signed double-submit token, a required custom header, or a `Sec-Fetch-Site` check rejecting a request that lacks the header or falling back to `Origin`), no credentials in URLs, WebSocket endpoints with an allowlisted handshake `Origin`, an authenticated connection, and every message validated and authorized as an HTTP request would be
- **Logging and audit** — security-relevant events must be logged, sensitive data must never appear in logs, in other telemetry (trace attributes, metric labels, error reports), or in propagated trace context (`tracestate`, baggage), audit trails for access control changes
- **Dependency management** — known-vulnerability scanning before release and on a schedule, pinned versions, a build-time SBOM, signature, provenance, or (for Go modules) checksum-database verification of registry packages as part of installation — or, where the ecosystem's standard installer provides no verification (pip), a documented gap and hash-checked installs — dependency-confusion and typosquatting defenses including integrity-verified installs against committed hashes, immutable pinning of build inputs the lockfile does not cover (CI actions, base images, downloaded tools)
- **Error handling** — no stack traces or internal details in production responses, structured error codes, security controls that fail closed when their check cannot complete

### Frontend (`security-frontend.md`)

Rules for browser-side code, UI rendering, and client-server interaction.

**Categories:**

- **Cross-site scripting (XSS)** — output encoding, Content Security Policy, Trusted Types (enforced once adopted, with a reviewed `default` policy permitted to take legacy code paths' raw strings while they migrate), no inline scripts, `postMessage` origin checks on both the receiving and the sending side
- **Cross-site request forgery (CSRF)** — a token, custom-header, or Fetch Metadata (`Sec-Fetch-Site`) defense for ambient-credential requests, a request lacking `Sec-Fetch-Site` either rejected or checked against `Origin`, with SameSite cookie attributes as defense in depth rather than the sole defense; no state change over `GET`; and client-side CSRF, where attacker-controlled input (the URL, `window.name`, `document.referrer`, `postMessage` data) never chooses the endpoint or method of a state-changing request the application's own script sends
- **Secure storage** — no secrets, tokens, or credentials in localStorage, sessionStorage, or IndexedDB; no sensitive or authenticated responses in a service worker's Cache API; cookie security attributes (HttpOnly, Secure, SameSite) and the `__Host-` name prefix; no sensitive data in URLs
- **Authentication UX** — redirect validation after login, session expiration behavior, logout that invalidates the server-side session
- **Content security** — CSP headers with violation reports kept where the responsible team can review them, frame protection, iframe sandboxing, `Permissions-Policy`, `Cross-Origin-Opener-Policy`
- **Dependency management** — known-vulnerability scanning before release and on a schedule, subresource integrity, pinned versions with production installs that cannot rewrite the lockfile, no dynamically loaded third-party scripts without integrity checks, no dependency-originated network egress
- **Sensitive data handling** — mask or redact PII in the UI, `autocomplete` tokens on credential, multi-factor, and payment forms, no caching of pages that display sensitive data, eviction of client-side artifacts at logout

## Rule Format

Each rule within a file follows a consistent structure:

- **Rule ID** — short identifier formatted as `{surface}-{category}-{NNN}` (e.g., `BE-AUTHN-001`, `FE-XSS-001`) for reference in specs, plans, and `/{project}:analyze` output. See **ID stability** below.
- **Rule statement** — a block quote declaring what must or must not happen, with RFC 2119 keywords of one tier (see `data-model.md` § Severity classification)
- **Verification** — how `/{project}:analyze` or a reviewer checks compliance (code pattern, test requirement, configuration check, or documentation commitment — see **Verification phrasing** below), naming any sibling rule that owns an overlapping defect

A rule carries no rationale: its statement stands alone (scenario [`rules-are-as-short-as-their-required-content`](scenarios/rules-are-as-short-as-their-required-content.md)).

Rules use RFC 2119 language: MUST, MUST NOT, SHOULD, SHOULD NOT. MUST/MUST NOT rules are reported as errors; SHOULD/SHOULD NOT rules are flagged as warnings. MAY may appear beside either tier to state a permission or a scope boundary, and carries no tier of its own.

### ID stability

Rule IDs are permanent. Once an ID is assigned, it must never be renumbered, even if the rule is moved within the file, edited, or deprecated. New rules get the next available sequence number for their category. Deprecated rules retain their ID with a `DEPRECATED` label and a removal target version. Sequence numbers are not reused after a rule is fully removed. Reading a file top-to-bottom may show non-sequential IDs after deprecations and reorders — IDs are anchors, not a reading order.

### Verification phrasing

Rules whose enforcement happens *outside* the code repository (runtime configuration, infrastructure, deployment) phrase their **Verification** field as a documentation commitment rather than a code pattern. Example:

```text
BE-DATA-001 — TLS for host-crossing traffic

Verification: Any spec or plan covering network communication, edge
configuration, or system.md MUST commit to TLS 1.2+ across all
host-crossing surfaces (HTTPS, gRPC, message brokers, database
connections). /{project}:analyze flags specs that allow plaintext
protocols for host-to-host traffic, that name TLS 1.0/1.1, or that omit
the TLS version question entirely. Plaintext loopback exceptions MUST be
named explicitly with the sidecar/proxy that provides the encrypted
host-boundary leg.
```

> **Signpost:** this example cited `BE-API-002` from the spec's first draft until 2026-09-13. It never matched: `BE-API-002` has been the `Server` / `X-Powered-By` version-header suppression rule since the file was written, so a reader following the ID landed on an unrelated rule. The example now quotes `BE-DATA-001`'s shipped Verification verbatim, so it can be checked against `framework/rules/security-backend.md`.

`/{project}:analyze` does not probe running infrastructure or parse deployment configs — it confirms the project has documented its approach. Code-pattern Verification (e.g., "every handler accepting user input MUST call a validator before persisting") remains the norm for rules whose enforcement lives in the repository.

## Ductus Integration

Both files are added to the ductus file manifest with `update` strategy — `ductus`-owned, always overwritten with the latest version on re-run.

| Source Path | Destination Path |
| --- | --- |
| `framework/rules/security-backend.md` | `specs/rules/security-backend.md` |
| `framework/rules/security-frontend.md` | `specs/rules/security-frontend.md` |

Source files live in the `ductus` framework under `framework/rules/`, alongside the constitution and other ship-everything artifacts. Destination is the project's `specs/` directory, alongside `system.md`, `errors.md`, and `events.md` — the other cross-cutting global specs. A project selects which surfaces it needs through `.ductus/config.toml` `[rules] surfaces` (spec `033-rule-surface-setting`), which is the operator-set source of truth: a `surfaces` list omitting `frontend` means `specs/rules/security-frontend.md` is never installed or updated, and one omitting `backend` does the same to the backend file. A copy already on disk is left in place, and `/{project}:analyze` still loads it, since it loads every rule file present regardless of `[rules] surfaces`. Pinning either file in `[pinned] files` still works and is the way to keep a locally diverged copy. An earlier wording here said backend rules "apply to all projects", which 033 made false.

**Local edits will be overwritten.** Because both files use the `update` strategy, any local edits to `specs/rules/security-backend.md` or `specs/rules/security-frontend.md` are discarded on the next `/ductus` run. To diverge from the governance-owned ruleset, pin the file in `.ductus/config.toml` — pinned files are never updated. Editing rule files directly without pinning is a path to losing work.

## Analyze Integration

`/{project}:analyze` gains security rule checking:

- During an analyze run, `/{project}:analyze` reads the applicable security rule files
- For each MUST/MUST NOT rule, `/{project}:analyze` checks whether the implementation complies
- Violations are reported as errors (blocking)
- SHOULD/SHOULD NOT violations are reported as warnings (non-blocking)
- Rule IDs are included in the report output for traceability

`/{project}:analyze` does not perform static analysis or code scanning. It checks whether the spec, plan, and implementation *address* the applicable rules — for example, whether a spec that handles user input includes input validation acceptance criteria, or whether a plan that stores credentials specifies hashed storage.

Rules apply **contextually** based on what the spec or plan actually addresses. A rule that no spec or plan content exercises is silently inert — no finding emitted, no opt-out required. A project with no database, for example, naturally produces no findings for data-at-rest rules because no spec mentions data persistence.

Rules with a **runtime or infrastructure dimension** (e.g., "TLS must be enabled in production") are not a separate check category. They are verified the same way every other rule is — by checking that a spec, plan, or `system.md` documents how the project addresses the rule. `/{project}:analyze` does not probe a running server or parse Terraform/Helm/Ansible; it confirms that the project has *thought about* the rule and recorded its approach. Rules whose enforcement happens outside the code repository should phrase their **Verification** field as a documentation commitment (e.g., "system.md MUST describe how TLS is terminated and the expected protocol/cipher policy"). The trade-off is deliberate: a spec that *says* TLS is enabled but where production is actually misconfigured will pass. The findings are necessary but not sufficient — runtime enforcement is the job of deployment tooling, infra-as-code review, and observability.

## Brownfield Adoption

> **Changed by [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md).**
> The audit no longer writes to `specs/inbox.md`. It reports each finding
> under the spec it belongs to and writes nothing, and `/{project}:analyze`
> dispositions each gap when it next runs on that spec. The trigger and the
> audit logic are unchanged. AC22, AC25, AC26, and AC27 are annotated in place.

When `/ductus` installs the security rule files in a project that already has feature specs, the adopter inherits a backlog: existing specs were written before these rules existed and almost certainly do not address all of them. To avoid dumping that backlog directly on `/{project}:analyze` (where it would block the next pipeline gate on every legacy spec at once), the audit reports it at adoption and writes nothing. Each gap names its spec, and `/{project}:analyze` finds it again, and dispositions it, when that spec is next worked on.

### Trigger

Ductus runs a one-time security audit when **both** conditions hold after the file manifest has been processed:

- Either `specs/rules/security-backend.md` or `specs/rules/security-frontend.md` was newly **created** by the manifest pass (i.e., not already present, not just updated).
- The project contains at least one feature spec directory under `specs/`, in either numbering form — the membership rule lives in `framework/constitution.md` §numbering and is not restated here, because a second copy is how the sequential and branch-scoped forms drift apart.

When neither condition holds — greenfield adoption with no existing specs, or a routine re-run where the rule files already exist — the audit is silently skipped. There is no per-run audit; the trigger is "rule file newly installed in a project with existing specs."

### Audit logic

For each newly created rule file:

1. Load the rule file, applying the same integrity checks `/{project}:analyze` uses (well-formed headings, required fields, valid IDs, no duplicates). If the file fails to load, ductus reports the load failure and skips the audit for that file — same posture as `/{project}:analyze`.
2. For each MUST/MUST NOT and SHOULD/SHOULD NOT rule whose Verification trigger fires against any artifact under a feature spec directory (`spec.md`, `plan.md`, scenario files), produce a finding.
3. Report each finding under the spec it belongs to. The audit writes no file.

### Finding format

Each finding is reported as one line, grouped under its spec:

```text
{Rule ID}: {affected artifact path} does not address — {one-line summary}
```

Examples:

```text
BE-AUTHN-001: specs/004-user-login/spec.md does not name a password-hashing algorithm or its cost parameters
FE-XSS-001: specs/007-comment-rendering/spec.md does not specify an output encoding strategy
```

Prefixing every line with the rule ID makes related findings group naturally and gives the adopter a stable handle for cross-referencing.

### Idempotency

The audit writes nothing, so it is safe to re-run if a user deletes and re-installs a rule file or otherwise re-triggers the "newly created" path: a re-run reports the gaps again and duplicates nothing.

### Reporting

After the audit completes, ductus's post-scaffolding output gains a block headed `Security audit: {N} gap(s) in {M} existing spec(s) — nothing was written.` It lists every finding under its spec, with no cap, because a truncated list reads as the complete one. It then says that each gap resurfaces when `/{project}:analyze` next runs on its spec, and that `/{project}:log` tracks one sooner. Every command is rendered in the adopted agent's own invocation form.

When `N == 0` (no findings), the block is omitted.

### Why not block the analyze gate instead?

A simpler alternative would be: `/{project}:analyze` runs on existing specs after ductus adoption and emits errors as usual. Rejected — for brownfield projects, that produces an immediate analyze failure that blocks every pipeline gate until the adopter fixes dozens of legacy specs. Reporting at adoption shows the adopter the whole backlog, and each gap then reaches `/{project}:analyze` only when its spec is next worked on, so adoption spreads through the areas being touched rather than stalling every spec at once. The accepted cost is that a legacy spec nobody touches is never re-checked.

The first version of this section wrote each finding to `specs/inbox.md`, so that one place held the backlog and `/{project}:groom` processed it. 058 reversed that: no gate reads the inbox, so a finding parked there could wait while its spec reached `done`. An adopter who wants a specific finding tracked before its spec is next worked on can `/{project}:log` it.

## Constitution Reference

The "Secure" principle in the constitution gains a reference to the security rule files:

> Protect sensitive data through industry standards and best practices. See `specs/rules/security-backend.md` and `specs/rules/security-frontend.md` for enforceable rules.

This connects the principle to its operational detail without duplicating content.

## Versioning and Evolution

- Rules are added, modified, or deprecated in the `ductus` repo
- Adopting projects receive updates on the next `/ductus` re-run
- Deprecated rules are marked with a `DEPRECATED` label and removal target version rather than deleted immediately, giving projects time to adjust
- New rules are announced in `ductus` commit messages so adopters can review changes

## Edge Cases

How `/{project}:analyze` behaves when the inputs are unusual:

- **Neither rule file present.** If a project has no `specs/rules/security-backend.md` and no `specs/rules/security-frontend.md` (e.g., a cross-only `[rules] surfaces = []` kept both from ever being installed, or both were deleted by hand), `/{project}:analyze` applies no security rules and continues. When the rule-file directory holds no rule file of any kind, it also emits an advisory warning — `No rule files found, skipping rule checks`, generalized from this spec's original security-only wording by 016; a project that still has other rule files (the `-cross.md` files a cross-only setting installs, for instance) gets no such warning. The rule check is non-blocking in either case.
- **Only one file present.** If a project has only one of the two files (e.g., a backend-only project whose `[rules] surfaces` omitted `frontend` before the frontend file was ever installed, or vice versa), `/{project}:analyze` runs over the present file and emits no finding for the missing one. This is the common case for non-fullstack projects.
- **Malformed rule file.** A rule file is malformed if any rule is missing a required field (ID, statement, verification), if any rule's ID does not match the `{surface}-{category}-{NNN}` format, or if the file fails to parse. `/{project}:analyze` **blocks** with an error: `Malformed rule file {path} at {location}: {reason}` (generalized from this spec's original `Malformed security rule file` wording by 016). The accompanying file is treated as unloadable; no rules from that file are applied. Rationale: the findings must rest on accurate rule data — partial or guessed-at parsing produces unreliable findings.
- **Stale rule reference.** A spec or plan references a rule ID that does not exist in the current rule files (the rule was removed upstream after `/ductus` updated the file). `/{project}:analyze` **blocks** with an error: `Spec at {path} references unknown rule {ID}.` The adopter must update or remove the reference before the gate will pass. Rationale: stale references silently rot if tolerated.
- **Reference to DEPRECATED rule.** A spec or plan references a rule ID that exists but is marked `DEPRECATED`. `/{project}:analyze` emits a warning (not an error): `Spec at {path} references deprecated rule {ID}; targeted for removal in {version}.` The reference still satisfies the rule for the duration of the deprecation window. Rationale: deprecation needs a real grace window between the label landing and references becoming invalid; a hard block at deprecation collapses the window to zero.
- **Local edits overwritten by `/ductus`.** The ductus command overwrites `specs/rules/security-backend.md` and `specs/rules/security-frontend.md` on every run because they use the `update` strategy. This is normal ductus behavior, not a security-rules-specific concern, but is called out in **Ductus Integration** above so adopters know to use `.ductus/config.toml` pinning rather than local edits when they need to diverge.
- **Duplicate rule IDs in a file.** If two rules in the same file share an ID (a botched edit broke the never-renumber discipline), `/{project}:analyze` **blocks** with an error: `Duplicate rule ID {ID} in {file}; refusing to load.` The whole file is skipped to prevent ambiguous references — it cannot tell which of the two rules a spec's reference points to. Rationale: same as malformed file — accurate rule data is non-negotiable.

## Acceptance Criteria

### Rule Files

- [x] AC1: `framework/rules/security-backend.md` exists in the `ductus` framework with categorized, numbered rules
- [x] AC2: `framework/rules/security-frontend.md` exists in the `ductus` framework with categorized, numbered rules
- [ ] AC3: Every rule has an ID, a statement, and a verification method, and no rule carries a Rationale paragraph
- [x] AC4: Rule IDs follow the format `{surface}-{category}-{NNN}` with `{surface}` ∈ `{BE, FE}` and `{NNN}` zero-padded starting at `001`
- [x] AC5: Rules use RFC 2119 language to distinguish enforced (MUST/MUST NOT) from advisory (SHOULD/SHOULD NOT)

### Ductus Integration

- [x] AC6: Both files appear in the ductus file manifest with `update` strategy
- [x] AC7: The ductus command fetches `framework/rules/security-backend.md` and writes it to `specs/rules/security-backend.md` in the project
- [x] AC8: The ductus command fetches `framework/rules/security-frontend.md` and writes it to `specs/rules/security-frontend.md` in the project
- [x] AC9: Re-running ductus updates both files to the latest `ductus` version
- [x] AC10: Projects can pin either file in `.ductus/config.toml` to skip updates

### Analyze Integration

- [x] AC11: `/{project}:analyze` reads `specs/rules/security-backend.md` and `specs/rules/security-frontend.md` when present in the project
- [x] AC12: MUST/MUST NOT violations are reported as errors (blocking)
- [x] AC13: SHOULD/SHOULD NOT violations are reported as warnings (non-blocking)
- [x] AC14: Rule IDs appear in `/{project}:analyze` output for each finding
- [x] AC15: Rules apply contextually — a rule that no spec or plan content exercises produces no finding

### Edge-case behavior

- [x] AC16: `/{project}:analyze` emits an advisory warning and continues when no rule files are present
- [x] AC17: `/{project}:analyze` runs only over the present file when only one of the two is present (e.g., the other's surface was never selected in `[rules] surfaces`), with no finding for the missing file
- [x] AC18: `/{project}:analyze` blocks with an error on a malformed rule file (missing required field, ID format violation, parse failure)
- [x] AC19: `/{project}:analyze` blocks with an error on a spec/plan reference to an unknown rule ID
- [x] AC20: `/{project}:analyze` emits a warning (not an error) on a spec/plan reference to a `DEPRECATED` rule ID
- [x] AC21: `/{project}:analyze` blocks with an error when a rule file contains duplicate IDs

### Brownfield Adoption

- [x] AC22: On a ductus run where a security rule file is newly created AND the spec root holds at least one feature spec directory (either numbering form per §numbering), ductus audits the existing specs against the rule and writes one inbox item per finding to `specs/inbox.md` — **Superseded by 058**: the audit still runs on this trigger, and it reports each finding under its spec instead of writing an inbox item. Nothing is written.
- [x] AC23: On a greenfield run (no existing feature spec directories in the spec root), the audit is silently skipped
- [x] AC24: On a routine re-run (rule files already present), the audit is silently skipped
- [x] AC25: Inbox items follow the format `- [ ] {Rule ID}: {affected artifact path} does not address — {one-line summary}` — **Superseded by 058**: no inbox item is written. Each reported finding keeps this format without its leading checkbox.
- [x] AC26: Audit findings are deduplicated against existing inbox content (no duplicate items emitted on re-trigger) — **Superseded by 058**: the audit writes nothing, so a re-trigger re-reports its findings and duplicates nothing. There is no inbox content to deduplicate against.
- [x] AC27: Ductus's post-scaffolding output reports the count of new audit items added (omitted when zero) — **Superseded by 058**: no item is added. The output reports how many gaps were found and in how many specs, lists each finding under its spec, and is still omitted when there are none.

### Constitution Reference

- [x] AC28: The "Secure" principle references `specs/rules/security-backend.md` and `specs/rules/security-frontend.md`

## Open Questions

None — all resolved during clarification.

## Resolved Questions

1. **Severity levels beyond MUST/SHOULD** — No new tiers. Keep the RFC 2119 MUST/SHOULD distinction only. MUST/MUST NOT violations are errors (blocking); SHOULD/SHOULD NOT violations are warnings (non-blocking). A "critical / blocks merge" tier adds nothing over MUST since CI gating on `/{project}:analyze` already blocks. An "acknowledge" tier is a documentation requirement, better expressed as a SHOULD whose rationale says "if you choose not to follow this, record the deviation in the spec." Trade-off accepted: all SHOULDs are equal in v1; if prioritization becomes painful, address it in the report output, not by adding tiers.

2. **Rule ID granularity** — Per-rule IDs with a never-renumber policy. Format: `{surface}-{category}-{NNN}` where `{surface}` is `BE` or `FE`, `{category}` is a short uppercase abbreviation (backend: `AUTHN`, `AUTHZ`, `INPUT`, `DATA`, `API`, `LOG`, `DEPS`, `ERR`; frontend: `XSS`, `CSRF`, `STORAGE`, `AUTHN`, `CSP`, `DEPS`, `PII`), and `{NNN}` is zero-padded starting at `001`. `AUTHN` covers authentication, `AUTHZ` covers authorization — distinct abbreviations to prevent collision. Numbering is per-category, not global. Once an ID is assigned it is permanent for the lifetime of the rule — reorganization moves rules but never renumbers them. Deprecated rules keep their ID with a `DEPRECATED` label and removal target. Sequence numbers are never reused, even after a rule is fully removed. Trade-off accepted: reading the file top-to-bottom may show non-sequential IDs after deprecations; IDs are anchors, not a reading order.

3. **Per-category opt-out** — No category-level opt-out for v1. Surface selection handles the case where an entire surface does not apply. At the time this question was resolved that meant whole-file pinning (e.g., backend-only projects pin `security-frontend.md`); spec `033-rule-surface-setting` later made `.ductus/config.toml` `[rules] surfaces` the operator-set source of truth for it, which supersedes pinning for this purpose without reopening the decision below. Within a surface, rules apply *contextually* — a rule that no spec or plan content exercises is silently inert, so a project with no database naturally produces no data-at-rest findings without needing to opt out. Adding category-level opt-outs would require a declaration syntax, gate logic to honor it, and a migration story when the project's stack evolves — substantial surface area for a use case that does not yet exist. Trade-off accepted: if `/{project}:analyze` produces a false-positive finding because contextual matching catches something the project doesn't actually do, there is no opt-out — fix the matching, not the policy.

4. **Runtime/infrastructure rules** — No special handling. Runtime rules (e.g., "TLS must be enabled") are verified the same way every other rule is — `/{project}:analyze` checks that the project's spec, plan, or `system.md` documents how the rule is addressed. It does not probe running infrastructure or parse deployment configs; it confirms the project has *thought about* the rule and recorded its approach. Such rules should phrase their **Verification** field as a documentation commitment (e.g., "system.md MUST describe how TLS is terminated"). Trade-off accepted: a spec that says TLS is enabled but where production is misconfigured will pass — runtime enforcement is the job of deployment tooling, infra-as-code review, and observability, not of a text-based pipeline gate.

5. **A request without `Sec-Fetch-Site` may be rejected instead of falling back to `Origin` (decided by the owner 2026-10-19)** — `FE-CSRF-001` and `BE-API-011` accept either treatment of a state-changing request that lacks the header: reject it (failing closed) or check its `Origin` as the fallback. Both rules had required the `Origin` fallback. The current OWASP CSRF Prevention Cheat Sheet's server-side policy offers both, with fail-safe blocking recommended for sensitive endpoints. Trade-off accepted: a project that fails closed turns away the outdated or embedded browsers that send no Fetch Metadata, and requests whose headers an intermediary strips.

6. **bcrypt only in legacy systems (decided by the owner 2026-10-19)** — `BE-AUTHN-001` accepts bcrypt only in a legacy system where neither Argon2id nor scrypt is available, per the OWASP Password Storage Cheat Sheet, and a spec choosing it states why. The tightened MUST keeps its ID. PBKDF2 keeps its place, as the choice where FIPS compliance is required (OWASP; NIST SP 800-63B-4 pointing to SP 800-132). Trade-off accepted: a new system that would have picked bcrypt for library familiarity now fails the rule.

7. **Identical duplicate `Content-Length` values are not rejected (decided by the owner 2026-10-19)** — `BE-INPUT-013` requires the edge to reject only `Content-Length` values that differ, whether as repeated header lines or one comma-separated list. A request whose values are all identical MAY be accepted with the values collapsed to one, which RFC 9112 §6.3 and RFC 9110 §8.6 permit. The rule had rejected every duplicate. Trade-off accepted: the edge must normalize identical repeats before forwarding, since RFC 9110 forbids forwarding the list as received.

8. **An installer without verification is a documented gap, not a violation (decided by the owner 2026-10-19)** — `BE-DEPS-004` gains a MAY for an ecosystem whose standard installer provides no signature or provenance verification. pip is the example: PEP 740 makes no recommendation that installing clients verify attestations, and PyPI documents verification only through the separate `pypi-attestations` tool. Such a project documents the gap and installs with the integrity checking its ecosystem offers (`pip --require-hashes`, which `BE-DEPS-005` already names). Trade-off accepted: hash-checked installs pin bytes without identifying the publisher, so the gap stays visible rather than closed.

9. **Re-authentication before account-security changes (decided by the owner 2026-10-19)** — new `BE-AUTHN-022` (MUST): changing a password or other authenticator, the email or recovery contact, or the MFA settings requires re-authentication with an already-bound authenticator, not the existing session alone. A password reset through `BE-AUTHN-018`'s recovery flow MAY proceed on its reset token. Grounded in the OWASP Authentication and Multifactor Authentication Cheat Sheets and NIST SP 800-63B-4 §4.1.2.1. Trade-off accepted: an extra prompt on every such change, even moments after sign-in. **Narrowed 2026-10-20 for MFA changes (owner decision, scenario `independent-review-findings-await-the-owner`):** enrolling another factor beside an enrolled one, replacing or removing an enrolled second factor, or turning MFA off requires a fresh verification with an enrolled second factor or passkey, and the current password alone no longer authorizes it, per the OWASP Multifactor Authentication Cheat Sheet's "Require reauthentication with an existing enrolled factor before allowing changes". The password still authorizes the other changes, and enrolling a second factor when none is enrolled. **Recovery exception added 2026-10-20 (owner decision, same scenario):** a user who has lost every enrolled second factor MAY bind a replacement through account recovery meeting NIST SP 800-63B-4 §4.2's AAL2 bar — two recovery codes obtained by different methods, one code together with a single-factor authenticator bound to the account such as the current password, or repeated identity proofing — with the user notified independently; the password alone still never replaces a second factor. `BE-AUTHN-018`'s expiry, newer-token invalidation and reset sweep reach only the tokens a recovery flow issues and uses, so a saved recovery code stays valid until used, when it is invalidated and replaced.

10. **A WebSocket rule (decided by the owner 2026-10-19)** — new `BE-API-015` (MUST): a WebSocket endpoint serving non-public data or accepting state-changing messages allowlists the handshake's `Origin`, authenticates the connection, and validates and authorizes every message as an HTTP request would be. It cites `BE-API-003`, `BE-API-011`, `BE-INPUT-001`/`-006`, and `BE-AUTHZ-003`/`-005` rather than restating them. MUST rather than SHOULD because the OWASP WebSocket Security Cheat Sheet states these requirements without condition; RFC 6455 §10.2's SHOULD covers exactly the servers this rule scopes to, those not meant to take input from any web page. A handshake with no `Origin`, which RFC 6455 says should not be read as coming from a browser, MAY be accepted when the connection uses a credential the browser does not attach automatically. Trade-off accepted: the rest of the WebSocket surface (connection limits, session expiry on long-lived connections, compression) stays deferred.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [007-govern-workflow](../007-govern-workflow/spec.md)
