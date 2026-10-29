---
title: "008-security-rules — tasks"
---

# 008 — Security Rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Write the backend security rules file

Create `framework/rules/security-backend.md` with the v1 starter set. ~35 rules across 8 categories, biased toward MUST/MUST NOT, each with statement, rationale, and verification per `data-model.md`. Coverage targets were selected by walking OWASP Top 10 (2021), OWASP API Security Top 10 (2023), and CWE Top 25.

- [x] Create `framework/rules/` directory
- [x] Create `framework/rules/security-backend.md` with file-level introduction
- [x] **Authentication** category — at least 4 rules: credential storage (deliberately slow password hash); session/token handling; MFA consideration; constant-time comparison of secrets
- [x] **Authorization** category — at least 4 rules: default-deny; explicit permission checks at every entry point; privilege escalation prevention; mass-assignment / over-posting prevention
- [x] **Input validation** category — at least 9 rules: boundary validation; allowlist over denylist; size limits; parameterized queries (SQL/NoSQL/command injection); SSRF prevention on outbound URL fetches; safe deserialization of untrusted data; path-traversal prevention on filesystem ops; XXE prevention on XML parsing; SSTI prevention on template rendering
- [x] **Data protection** category — at least 4 rules: encryption at rest commitment; secrets management; PII handling; key rotation
- [x] **API security** category — at least 7 rules: rate limiting; CORS policy; security headers (generic); HSTS / HTTPS-only enforcement; `Cache-Control: no-store` for sensitive responses; redirect endpoint validation; error responses do not leak internals
- [x] **Logging and audit** category — at least 2 rules: sensitive-data exclusion from logs; audit trail for access control changes
- [x] **Dependency management** category — at least 2 rules: vulnerability scanning; pinned versions
- [x] **Error handling** category — at least 2 rules: no stack traces in production responses; structured error codes
- [x] Every rule heading is a level-3 heading containing only the rule ID
- [x] Every rule has Statement (block quote), Rationale, and Verification fields
- [x] All IDs follow `BE-{CATEGORY}-{NNN}` format and start at `001` per category
- [x] At least one rule per category uses MUST/MUST NOT (errors); SHOULD usage is appropriate where noted
- [x] File passes `npx markdownlint-cli2`

**Done when:** every backend category has at least the targeted rule count, all rules conform to the data-model schema, and the file passes markdownlint.

## 2. Migrate and extend the frontend security rules file

`framework/rules/security-frontend.md` already exists from the framework reorganization (commit `3fc76b7`) but predates 008's clarification. It contains 25 rules across 7 categories whose content is sound but whose **format, three category abbreviations, and Verification phrasing** are out of sync with the post-clarify data-model. Apply the same migration treatment used for the backend file in task 1 — see `framework/rules/security-backend.md` and commit `35502af` as the precedent.

### Migration steps

- [x] Read the existing `framework/rules/security-frontend.md` first; do **not** overwrite without preserving content
- [x] Rename three category abbreviations (preserve suffix numbers — never renumber):
  - `FE-AUTH-*` → `FE-AUTHN-*` (3 rules; matches the data-model's authentication abbreviation, parallel to BE)
  - `FE-DEP-*` → `FE-DEPS-*` (3 rules; data-model uses DEPS, not DEP)
  - `FE-DATA-*` → `FE-PII-*` (3 rules; the "Sensitive data handling" category is abbreviated `PII` per the data-model category table)
- [x] Reformat every rule to match the data-model schema:
  - Level-3 heading contains **only** the rule ID (no title after an em-dash)
  - Statement is a single block-quoted sentence using RFC 2119 keywords (`MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`)
  - `**Rationale:**`, `**Verification:**`, `**Source:**` are paragraphs (not bullet items)
- [x] **Rewrite every Verification field** as an instruction to the validate agent — concrete keywords to search, what artifacts to scan, what commitments must be present, what to flag. Existing Verifications are written for human reviewers ("Code review confirms…") and validate cannot act on them. See backend file's Verification fields for the right phrasing convention.
- [x] Audit every rule for accuracy against the constitution's Technology guiding principles (`framework/constitution.md` §guiding-principles). Backend revealed two real accuracy issues (over-prescriptive `MUST`, missing CSP/Referrer-Policy headers); frontend may have analogous issues — fix them and note each in the commit message.
- [x] Preserve `**Source:**` citations on every existing rule (the field is now in the data-model schema as optional). Add Source citations to any rule that lacks one (OWASP cheat sheet, RFC, MDN, etc.).

### Coverage additions

- [x] **Cross-site scripting (XSS)** — existing 6 rules likely cover output encoding, framework auto-escaping, safe DOM methods, no inline scripts, HTML sanitization, URL validation. Confirm coverage; add rules only if a v1-target gap exists.
- [x] **Cross-site request forgery (CSRF)** — existing 3 rules likely cover token-based protection, SameSite cookies, and no state changes via GET. Confirm coverage.
- [x] **Secure storage** — existing 3 rules cover no secrets in browser storage, cookie security attributes, no sensitive data in URLs. Confirm coverage.
- [x] **Authentication UX (`FE-AUTHN` after rename)** — existing 3 rules cover redirect validation, session expiration UX, logout completeness. Plan called for token storage / session expiration / redirect validation — confirm coverage; the existing rule set is broader than the plan's targets.
- [x] **Content security (`FE-CSP`)** — existing 4 rules cover CSP header required, strict CSP policy, frame protection, form action restriction. Plan called for CSP + SRI; SRI is currently under `FE-DEPS-002`. Decide: keep SRI under DEPS (logical grouping with other dependency-integrity concerns) or move to CSP. Document the decision. **Decision: kept SRI under `FE-DEPS-002` — moving categories would force renumbering, violating the never-renumber discipline; SRI is also conceptually closer to dependency integrity than to CSP directives.**
- [x] **Dependency management (`FE-DEPS` after rename)** — existing 3 rules cover vulnerability scanning, subresource integrity, no dynamic third-party loading. **Add a pinned-versions rule** (`FE-DEPS-004` or next available) matching `BE-DEPS-002`'s posture for npm/pnpm/yarn lockfiles. This is the only confirmed coverage gap.
- [x] **Sensitive data handling (`FE-PII` after rename)** — existing 3 rules cover PII masking, autocomplete control, cache control for sensitive pages. Confirm coverage of the plan's targets (UI masking, URL parameters, browser history).

### Final checks

- [x] No stale IDs remain (`grep -E '^### FE-(AUTH-|DEP-|DATA-)' framework/rules/security-frontend.md` returns nothing)
- [x] Every rule heading is a level-3 heading containing only the rule ID — no em-dash-and-title suffix
- [x] Every rule has Statement (block quote), Rationale, Verification, and Source fields
- [x] All IDs follow `FE-{CATEGORY}-{NNN}` format with categories from `{XSS, CSRF, STORAGE, AUTHN, CSP, DEPS, PII}`
- [x] At least one rule per category uses `MUST`/`MUST NOT`
- [x] File passes `npx markdownlint-cli2`

### Why migrate, not rewrite

The existing rules' *content* (rationale, threat model, OWASP citations) represents real prior thought. Throwing it away to write a fresh ~21-rule starter set loses information and mis-applies the never-renumber discipline. Migration preserves content, fixes format, and lets v1 ship with broader coverage (~26+ rules) than the original plan target.

**Done when:** every frontend rule conforms to the data-model schema, no stale category abbreviations remain, every Verification is written as an agent-actionable instruction, the pinned-versions rule has been added, accuracy issues found during the audit are fixed, and the file passes markdownlint.

## 3. Update ductus manifest

Add the two rule files to `framework/bootstrap/ductus.md`'s **Governance-owned shared files (strategy: update)** table.

- [x] Insert row mapping `framework/rules/security-backend.md` → `specs/rules/security-backend.md`
- [x] Insert row mapping `framework/rules/security-frontend.md` → `specs/rules/security-frontend.md`
- [x] Rows appear adjacent to the other governance-owned shared files
- [x] `framework/bootstrap/ductus.md` passes `npx markdownlint-cli2`

**Done when:** ductus syncs both rule files on its next run, and the file passes markdownlint.

## 4. Add the brownfield security audit to ductus

Add a new top-level **Security audit (brownfield)** section to `framework/bootstrap/ductus.md`, slotted after **Shared Files** and before **Per-Agent Scaffolding**. The section reads each rule file that was newly created by the manifest pass, evaluates each rule's Verification trigger against existing `specs/NNN-*` artifacts, and appends findings to `specs/inbox.md`.

- [x] Insert **Security audit (brownfield)** section between **Shared Files** and **Per-Agent Scaffolding**
- [x] Section's trigger: at least one of `specs/rules/security-backend.md` or `specs/rules/security-frontend.md` was newly **created** by the manifest pass AND the project contains at least one `specs/NNN-*` directory
- [x] Section silently skips when the trigger does not fire (greenfield run, or routine re-run with rule files already present)
- [x] Section loads each newly created rule file using the same integrity checks validate uses (well-formed headings, required fields, valid IDs, no duplicates); on load failure, reports the failure and skips the audit for that file
- [x] Section iterates loaded rules; for each rule whose Verification trigger fires against an existing project artifact (`spec.md`, `spec-and-plan.md`, `plan.md`, scenario files under `specs/NNN-*/`), produces a finding
- [x] Section appends each finding to `specs/inbox.md` as `- [ ] {Rule ID}: {affected artifact path} does not address — {one-line summary}`
- [x] Section deduplicates against existing inbox content: skip any finding whose `- [ ] {Rule ID}: {affected artifact path}` prefix already appears in the inbox
- [x] Add an audit-summary line to **Post-Scaffolding Output**: `{N} security audit items added to specs/inbox.md. Run /{project}:groom to triage.` Omit when N is zero
- [x] `framework/bootstrap/ductus.md` passes `npx markdownlint-cli2`

**Done when:** ductus runs the audit only when the trigger conditions hold, writes deduplicated findings to inbox in the documented format, reports the count, and the file passes markdownlint.

## 5. Extend validate with the security rule check section

Modify `framework/commands/analyze.md` to add a new **Security rules** check section that codifies the spec's edge-case behaviors. Place it after **Cross-spec references (advisory)** and before **Markdown lint (advisory)**.

- [x] Add **Security rules (blocking and advisory)** section
- [x] Section opens by reading `specs/rules/security-backend.md` and `specs/rules/security-frontend.md` (each independently optional — only the present files are loaded)
- [x] Section validates rule file integrity per `data-model.md` (well-formed headings, required fields, ID format) and reports a **blocking** error per malformed rule file (`Malformed security rule file {path} at {location}: {reason}`)
- [x] Section reports a **blocking** error on duplicate IDs within a file (`Duplicate rule ID {ID} in {file}; refusing to load`)
- [x] Section reports an **advisory** warning when neither file is present (`No security rule files found, skipping security checks`)
- [x] When at least one rule file is loaded and well-formed, section iterates over each rule:
  - For each MUST/MUST NOT rule whose Verification trigger fires against any project artifact (spec, plan, `specs/system.md`), execute the Verification check and emit a **blocking** error per failing artifact, including the rule ID
  - For each SHOULD/SHOULD NOT rule whose trigger fires, emit an **advisory** warning per failing artifact, including the rule ID
  - Rules whose Verification trigger does not fire for any artifact produce no finding (silently inert)
- [x] Section reports a **blocking** error when a project artifact references a rule ID that does not exist in the loaded files (`Spec at {path} references unknown rule {ID}`)
- [x] Section reports an **advisory** warning when a project artifact references a `DEPRECATED` rule ID (`Spec at {path} references deprecated rule {ID}; targeted for removal in {version}`)
- [x] Findings are surfaced under the existing severity sections (Hard fail / Blocking / Advisory) in validate's report
- [x] `framework/commands/analyze.md` passes `npx markdownlint-cli2`

**Done when:** validate's check list includes the security rule section with all 7 edge-case behaviors, and the file passes markdownlint.

## 6. Update the constitution

Append the rule-files reference to the "Secure" guiding principle in `framework/constitution.md`.

- [x] Edit the "Secure" bullet under **Guiding Principles → Technology** to read: `**Secure:** protect sensitive data through industry standards and best practices. See \`specs/rules/security-backend.md\` and \`specs/rules/security-frontend.md\` for enforceable rules.`
- [x] No other constitution edits in this task
- [x] `framework/constitution.md` passes `npx markdownlint-cli2`

**Done when:** the "Secure" principle references the rule files by their project paths, and the file passes markdownlint.

## 7. Validate end-to-end and run readiness checks

Run all structural and lint checks; verify each acceptance criterion is satisfied by the produced artifacts.

- [x] `npx markdownlint-cli2` passes on all created/modified files: `framework/rules/security-backend.md`, `framework/rules/security-frontend.md`, `framework/bootstrap/ductus.md`, `framework/commands/analyze.md`, `framework/constitution.md`, and the feature directory (`spec.md`, `plan.md`, `tasks.md`, `data-model.md`)
- [x] Every rule ID in both files matches the `{surface}-{category}-{NNN}` format
- [x] No duplicate rule IDs within either file (check via `grep '^### ' framework/rules/security-*.md | sort | uniq -d`)
- [x] Every rule has Statement, Rationale, and Verification fields present
- [x] Each acceptance criterion in `spec.md` is checked individually against the produced artifacts and marked `- [x]` only if satisfied

**Done when:** all checks pass, the rule files are internally consistent, and every acceptance criterion is satisfied.

## 8. Add FE-DEPS-005 — dependency-originated network egress (reopen)

Reopen surfaced upstream from an adopter: an `FE-DEPS` rule was missing for a dependency's *runtime* network egress (telemetry/analytics/license/update-poll), which every existing `FE-DEPS` rule (load-time, vulnerability, pinning) leaves uncovered. Motivating case: `@mui/x-telemetry` posting to a vendor endpoint on dev-server reloads while silent in production builds.

- [x] Add `FE-DEPS-005` to `framework/rules/security-frontend.md` after `FE-DEPS-004`, before `## FE-PII`
- [x] `specs/008-security-rules/data-model.md` needs no change — `FE`/`DEPS` category already listed; the file carries ID grammar, not a per-ID manifest or rule count
- [x] `npx markdownlint-cli2` passes on `framework/rules/security-frontend.md`
- [x] `grep '^### ' framework/rules/security-frontend.md | sort | uniq -d` reports no duplicate IDs
- [x] Re-review clears the reopen and returns `008` to `done`

**Done when:** `FE-DEPS-005` is present and well-formed, no data-model change is outstanding, and `008` is re-reviewed back to `done`.

## 9. Reconcile mixed-keyword rule Statements with the one-keyword invariant (reopen)

- [x] Implement the behavior described in `scenarios/a-statement-carries-one-obligation-keyword.md`

- **Done when**: the scenario's open questions are resolved, no rule in `framework/rules/` violates the severity-classification invariant as the data models then state it, and `008` is re-reviewed back to `done`.

## 10. Disposition out-of-spec finding: the pre-commit hook restages a whole spec.md, so a status-only commit cannot be split from uncommitted body edits

- [x] `.githooks/pre-commit` and `framework/bootstrap/hooks/ductus-pre-commit` — both run `git add` on every staged `spec.md` from the working tree after `label-criteria`, so a status flip staged alone (by `git update-index` or hunk staging) commits the file's other uncommitted edits with it. Surfaced reopening 017 and 036: the first attempt, `09cd16b5`, committed the signposts under a status-only message and left `status: done`; it was undone before any push. The constitution's §spec-lifecycle advice to commit a status transition as its own step does not say the working file must hold only that change at commit time. — routed to 050-constitution (scenario `a-status-commit-holds-only-the-transition`, task 25; 050 reopened `done` → `in-progress`)

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 11. X-Frame-Options carries one strength across the two rule files

- [x] Implement the behavior described in `scenarios/x-frame-options-carries-one-strength.md`

- **Done when**: the scenario's open question is resolved, `BE-API-001` and `FE-CSP-008` state one strength for `X-Frame-Options`, and `008` is re-reviewed back to `done`.

## 12. Restore analyze's blocking report for a malformed or duplicate-ID rule file

- [x] `framework/commands/analyze.md` §Rules (blocking and advisory): restore the two blocking messages 016 shipped and 022's parseable rewrite (`11aad341`) dropped without a recorded decision — `Malformed rule file {path} at {location}: {reason}` for a missing required field, an ID-format violation, or a malformed heading, and `Duplicate rule ID {ID} in {file}; refusing to load` — each blocking, with no rules from the unloadable file applied
- [x] State the exec-path reduction beside them: under `ductus exec` the walker checks no rule-file integrity, records a rule with no Verification or tier as unexamined (`rule-assessments-not-checked`), and a file it cannot read as `rule-file-unreadable`, so neither message is produced there
- [x] Verify AC18 and AC21 against the restored text and re-tick them; run the runtime suite, since command files are its input

- **Done when**: `framework/commands/analyze.md` states both blocking messages and the exec reduction, AC18 and AC21 are re-verified and ticked, and `008` is re-reviewed and re-analyzed back to `done`.

## 13. Rule-file review 2026-10-19: land new and amended security rules

- [x] `BE-AUTHN-018` (new, MUST) — password-reset and account-recovery flows: a single-use, bounded-lifetime token persisted only as a hash, the reset link built from a configured base URL, existing sessions invalidated on reset, no security questions as the sole recovery factor
- [x] `BE-AUTHN-019` (new, MUST) — JWT verification keys selected only from key material configured in advance; token-supplied `jwk`/`jku`/`x5u`/`x5c` not trusted outside it; `kid` treated as untrusted input
- [x] `BE-AUTHN-020` (new, MUST) — no resource owner password credentials grant and no implicit grant; PKCE uses `S256`, never `plain`; public-client refresh tokens sender-constrained or rotated on every use
- [x] `BE-AUTHN-021` (new, SHOULD) — privileged-account MFA uses a phishing-resistant authenticator (WebAuthn/passkeys, FIDO2 keys, PKI smart cards)
- [x] `BE-INPUT-016` (new, MUST) — `BE-INPUT-007`'s SSRF checks enforced on the connection actually made: validated address pinned, redirects disabled or re-validated per hop, hosts normalized before comparison
- [x] `BE-DATA-012` (new, MUST) — every secret other than `BE-DATA-005`'s keys revocable and replaceable without a source change or rebuild, with a documented procedure for a leaked secret
- [x] `BE-API-013` (new, MUST) — passwords, session IDs, access and refresh tokens, and API keys never accepted in URL query parameters or path segments; single-use, short-lived URL values carved out
- [x] `BE-ERR-005` (new, MUST) — a security control that cannot complete its check denies the request (fail closed)
- [x] `BE-LOG-008` (new, MUST) — the values `BE-LOG-002` bars from logs kept out of trace span attributes and events, metric labels, and error-reporting payloads
- [x] `BE-DEPS-006` (new, MUST) — CI actions and reusable workflows, container base images, and downloaded build tools pinned to an immutable reference (commit SHA, image digest, verified checksum)
- [x] `BE-DEPS-007` (new, SHOULD) — `BE-DEPS-001`'s scan also runs automatically on every change to the dependency manifest or lockfile
- [x] `FE-XSS-010` (new, MUST) — a `postMessage` carrying non-public data names the exact recipient origin; `'*'` never its target origin
- [x] `FE-STORAGE-005` (new, SHOULD) — session and authentication cookies use the `__Host-` name prefix
- [x] `FE-CSP-009` (new, SHOULD) — HTML responses set `Cross-Origin-Opener-Policy: same-origin`, or `same-origin-allow-popups` for a page whose cross-origin popups report back through `window.opener`
- [x] `FE-DEPS-006` (new, SHOULD) — `FE-DEPS-001`'s scan also runs automatically on every change to the frontend dependency manifest or lockfile
- [x] `BE-AUTHN-001` (amended Verification) — cost parameters at or above the OWASP Password Storage Cheat Sheet minimums; a bcrypt choice states how input beyond its 72-byte limit is handled
- [x] `BE-AUTHN-011` (amended Statement) — NIST SP 800-63B-4 minimum length: 15 characters when the password is the sole authenticator, 8 when it is one factor of MFA
- [x] `BE-INPUT-004` (amended Verification) — trigger covers the entry names of an uploaded archive being extracted
- [x] `BE-INPUT-006` (amended Statement) — over-limit request content rejected with `413 Content Too Large`; an over-maximum parameter rejected with a `400`-class error or, for a page size, clamped, no longer `413`
- [x] `BE-INPUT-008` (amended Verification) — `BinaryFormatter` and payload-driven polymorphic type handling flagged; data-only formats acceptable when bound to fixed types
- [x] `BE-API-003` (amended Verification) — echoing the request `Origin` without an allowlist check, or allowlisting the `null` origin, flagged as a wildcard in effect
- [x] `BE-API-011` (amended Statement) — the CSRF defense is a synchronizer or signed double-submit token, a custom header the CORS policy does not admit cross-origin, or `Sec-Fetch-Site` validation with an `Origin` fallback; `SameSite` and `Origin`/`Referer` checks never the sole defense
- [x] `BE-DEPS-001` (amended Statement) — scan before every release and on a recurring schedule between releases; the MUST no longer names CI, and the per-change scan is `BE-DEPS-007`
- [x] `BE-DEPS-004` (amended Statement) — accepted mechanisms name npm provenance and registry signatures checked with `npm audit signatures`, and PyPI digital attestations (PEP 740)
- [x] `FE-XSS-009` (amended Statement) — Trusted Types adoption recommended for every application rendering dynamic HTML in the browser, no longer scoped to supporting browsers
- [x] `FE-CSRF-001` (amended Statement) — scoped to ambient-credential requests, with the same three defenses as `BE-API-011`; `SameSite` and `Origin`/`Referer` checks never the sole defense
- [x] `FE-STORAGE-001` (amended Verification) — encrypting a stored token or credential does not lift the browser-storage prohibition
- [x] `FE-STORAGE-002` (amended Statement) — `SameSite=None` MAY replace `Lax`/`Strict` only under the conditions `FE-CSRF-002` sets
- [x] `FE-STORAGE-003` (amended Statement) — a single-use, short-lived token whose flow requires URL delivery MAY appear in a query parameter when invalidated on first use and the landing page sets `Referrer-Policy: no-referrer`
- [x] `FE-CSP-003` (amended Statement) — a page designed to be embedded MAY list the exact origins permitted to frame it; wildcard `frame-ancestors` sources forbidden
- [x] `FE-CSP-006` (amended Statement) — the `allow-scripts` + `allow-same-origin` prohibition applies to a frame whose document is served from the embedding page's own origin, not to cross-origin frames
- [x] `FE-CSP-007` (amended Statement) — `interest-cohort=()` dropped from the required `Permissions-Policy` denials
- [x] `FE-DEPS-001` (amended Statement) — scan before every release and on a recurring schedule between releases; the MUST no longer names CI, and the per-change scan is `FE-DEPS-006`
- [x] `FE-DEPS-002` (amended Statement) — the no-load-without-integrity clause covers cross-origin scripts referenced from markup or build output; runtime-injected scripts are `FE-DEPS-003`'s
- [x] `FE-DEPS-003` (amended Statement) — a runtime-injected third-party script is permitted only when it carries integrity verification or is gated by a CSP-and-vendor-review process
- [x] `FE-DEPS-004` (amended Verification) — a `pnpm` or `yarn` production install without its frozen-lockfile flag flagged alongside `npm install`

- **Done when**: every listed rule reads as described in `framework/rules/security-backend.md` or `framework/rules/security-frontend.md`, `spec.md`, `plan.md`, `data-model.md`, and `research.md` describe the rule files as they now are, and `008` is re-reviewed back to `done`.

## 14. Rule-file review 2026-10-19 (second pass): land new and amended security rules

- [x] `BE-API-014` (new, MUST) — the application's own management and diagnostic endpoints (metrics, profiling and debug handlers such as Go's `/debug/pprof/`, heap or thread dumps, configuration and environment dumps, framework management endpoints such as Spring Boot Actuator's) not reachable from public networks unless every request to them is authenticated; a separate port or host only internal networks reach satisfies it, and a status-only health endpoint MAY be public
- [x] `FE-CSRF-004` (new, MUST) — client-side CSRF: attacker-controllable input (the URL's path, query string, or fragment, `window.name`, `document.referrer`, `postMessage` data) never determines the endpoint or HTTP method of a state-changing request client-side code sends, other than by selecting from a fixed set of requests the application defines; a value from such input that becomes a request parameter is validated against the expected format and values before the request is sent
- [x] `FE-STORAGE-006` (new, MUST NOT) — service worker code never stores responses carrying sensitive data (tokens, credentials, PII, other user-specific authenticated content) in the Cache API
- [x] `BE-AUTHN-011` (amended Statement and Verification) — each new password screened against a blocklist of commonly used, expected, or compromised passwords, and any password the blocklist contains rejected; no longer only a breached-password corpus
- [x] `BE-INPUT-003` (amended Statement and Verification) — the Statement's NoSQL operator examples are MongoDB `$where`, `$regex`, `$expr`; Verification names type-checked NoSQL query values and flags NoSQL filters that take request values without checking each is the expected scalar type
- [x] `BE-INPUT-004` (amended Verification) — the containment check after canonicalization compares whole path components, not a bare string prefix
- [x] `BE-INPUT-012` (amended Statement) — the trigger characters add line feed (`0x0A`) and the full-width forms `＝`, `＋`, `－`, `＠`
- [x] `BE-API-001` (amended Verification, no obligation change) — the HSTS `preload` paragraph says preload-list inclusion cannot easily be undone, since a removal takes months to reach users, no longer that it is effectively irreversible; the paragraph pointing to `FE-CSP-007` and `FE-CSP-009` for the other HTML-response headers, added in the first pass, was missing from §13
- [x] `BE-LOG-008` (amended Statement and Verification) — the bar covers propagated trace context (W3C `tracestate`, OpenTelemetry baggage entries) as well as span attributes and events, metric labels, and error reports; a correlation value is redacted, truncated, or replaced before it is recorded or propagated, and specs that place tokens or PII in baggage or `tracestate` are flagged
- [x] `BE-DEPS-004` (amended Statement) — verification happens as part of installation, before a dependency is built into or shipped with an artifact, no longer before installation; Go modules are verified against the public checksum database, named as a transparency log of module hashes beside the publisher-signature and provenance paths
- [x] `BE-DEPS-005` (amended Statement and Verification) — (a) for pip, a single `--index-url` that serves the private packages and proxies the public index, never `--extra-index-url` for private names, or a resolver that pins a package to one index (uv `tool.uv.sources`), no longer pip index priority; (b) where the package manager supports it, installs verify against hashes committed to the repository (npm `integrity`, pip `--require-hashes`, `go.sum`, Gradle `verification-metadata.xml`), no longer lockfiles recording each dependency's registry URL; Verification flags pip configurations that reach private packages through `--extra-index-url`
- [x] `FE-XSS-005` (amended Verification, no obligation change) — `bleach` dropped from the example sanitization libraries
- [x] `FE-XSS-008` (amended Statement and Verification) — a reviewed policy named `default` MAY receive the raw strings of legacy code paths while they are migrated to explicit policies; Verification flags raw strings reaching sinks only under an adoption with no reviewed `default` policy to route them through
- [x] `FE-CSP-005` (amended Statement and Verification) — the reporting endpoint keeps reports where the team responsible for the CSP can review them, not discarding them on receipt; no longer that it MUST be monitored so violations surface
- [x] `FE-DEPS-001` (amended Verification, no obligation change) — the Yarn scanner example reads `yarn npm audit` or Yarn 1's `yarn audit`
- [x] `FE-DEPS-004` (amended Verification) — the lockfile-rewriting installs are named (`npm install` rather than `npm ci`, `pnpm install` without `--frozen-lockfile`, a Yarn install without `--immutable`, Yarn 1 without `--frozen-lockfile`), except where pnpm or Yarn 2+ applies that behavior by default on detecting a CI environment
- [x] `FE-PII-002` (amended Statement, no obligation change) — breached-password warnings dropped from the purposes the `autocomplete` tokens serve
- [x] `FE-CSRF-002` (amended Statement in the first pass and missing from §13, no obligation change) — the parenthetical saying modern browsers reject a `SameSite=None` cookie without `Secure` dropped

- **Done when**: every listed rule reads as described in `framework/rules/security-backend.md` or `framework/rules/security-frontend.md`, `spec.md`, `plan.md`, `data-model.md`, and `research.md` describe the rule files as they now are, and `008` is re-reviewed back to `done`.

## 15. Rule-file review 2026-10-19 (owner decisions): CSRF fail-closed option, bcrypt legacy-only, duplicate Content-Length, verification exception, BE-AUTHN-022, WebSocket rule

- [x] `FE-CSRF-001` and `BE-API-011` (amended Statement, Rationale, and Verification) — under `Sec-Fetch-Site` validation, a state-changing request that lacks the header is either rejected (failing closed) or checked against its `Origin` as the fallback, as the OWASP CSRF Prevention Cheat Sheet's server-side policy presents it; the two rules word it identically, and a `Sec-Fetch-Site` check that does not state its treatment of a missing header is flagged
- [x] `BE-AUTHN-001` (amended Statement and Verification, ID kept) — bcrypt acceptable only in a legacy system where neither Argon2id nor scrypt is available, per the OWASP Password Storage Cheat Sheet; a spec choosing bcrypt states why; PBKDF2 unchanged (where FIPS compliance is required)
- [x] `BE-INPUT-013` (amended Statement, Rationale, Verification, and Source) — the edge rejects `Transfer-Encoding` with `Content-Length` and `Content-Length` values that differ (repeated lines or one list); identical values MAY be accepted when collapsed to a single instance before processing or forwarding, per RFC 9112 §6.3 and RFC 9110 §5.3, §8.6
- [x] `BE-DEPS-004` (amended Statement, Rationale, Verification, and Source) — MAY exception for an ecosystem whose standard installer provides no signature or provenance verification (pip: PEP 740 makes no recommendation that installers verify attestations), conditioned on documenting the gap and installing hash-checked (`pip --require-hashes`, per `BE-DEPS-005`); PyPI attestations remain acceptable when a verifier such as `pypi-attestations` runs in the install step
- [x] `BE-AUTHN-022` (new, MUST) — re-authentication with an already-bound authenticator (current password, or a fresh second-factor or passkey step-up) before changing a password or other authenticator, the email or recovery contact, or the MFA settings; a `BE-AUTHN-018` recovery reset MAY proceed on its token
- [x] `BE-API-015` (new, MUST) — a WebSocket endpoint serving non-public data or accepting state-changing messages checks the handshake `Origin` against an exact-match allowlist (`403` otherwise; a handshake without `Origin` MAY be accepted only with a non-ambient credential), authenticates the connection before sending protected data or acting on messages, and subjects every message to `BE-INPUT-001`, `BE-INPUT-006`, `BE-AUTHZ-003`, and `BE-AUTHZ-005`

- **Done when**: every listed rule reads as described in `framework/rules/security-backend.md` or `framework/rules/security-frontend.md`, `spec.md` (category bullets and Resolved Questions 5–10), `plan.md`, and `research.md` describe the rule files as they now are, and `008` is re-reviewed back to `done`.

## 16. Independent-review findings await the owner

- [x] Implement the behavior described in `scenarios/independent-review-findings-await-the-owner.md`
- [x] Owner answers each of the scenario's eight open questions, one at a time
- [x] Apply each answer to security-backend.md / security-frontend.md, fetching any external claim, and record it in Resolved Questions
- [x] Lints, audit suite, and rule-corpus tests pass

- **Done when**: All eight questions are resolved and applied, and 008 is re-reviewed back to done.

## 17. BE-AUTHN-022 and a lost second factor (follow-on to task 16)

- [x] Owner answers the BE-AUTHN-022 lost-factor question in scenarios/independent-review-findings-await-the-owner.md
- [x] Apply the answer to security-backend.md BE-AUTHN-022, fetching any external claim, and record it in Resolved Questions
- [x] Lints, audit suite, and rule-corpus tests pass

- **Done when**: The question is resolved and applied, and 008 is re-reviewed back to done.

## 18. BE-AUTHN-022 saved recovery codes against BE-AUTHN-018 (follow-on to task 17)

- [x] Owner answers the saved-recovery-code question in scenarios/independent-review-findings-await-the-owner.md
- [x] Apply the answer to security-backend.md BE-AUTHN-018 or BE-AUTHN-022, fetching any external claim, and record it in Resolved Questions
- [x] Lints, audit suite, and rule-corpus tests pass

- **Done when**: The question is resolved and applied, and 008 is re-reviewed back to done.

## 19. Rules are as short as their required content

- [x] Implement the behavior described in `scenarios/rules-are-as-short-as-their-required-content.md`
- [x] Second pass, per the scenario's Resolved Questions: cut the three listed leftovers from every file in `framework/rules/`, then an independent per-file review, then re-read each fix round until one has no MUST or SHOULD

The first pass is committed: all eleven files cut, each independently reviewed, a cross-file boundary check, and the fix rounds re-read until clean; every artifact describing the old format is corrected, and 016, 034, 036, 037, 038, 039 and 045 were reopened for it. The files went from 509,421 to 324,390 bytes (`wc -c framework/rules/*.md`, summed; `4e82686d` against the first-pass commit). The second pass cut the three leftovers and added the Rationale-only cases the first pass missed to ten Statements; each file was independently reviewed and each fix round re-read until clean. The files went from 324,390 to 317,523 bytes (same method, `70bb4af5` against the working tree, 2026-10-28). The mismatches that predate the scenario are tasks 24 and 25.

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 20. Disposition out-of-spec finding: FE-SEMHTML-001 and FE-ARIA-001 both flag a div with a widget role and a click listener

- [x] framework/rules/accessibility-frontend.md — `FE-SEMHTML-001`'s Verification flags a click listener on a `<div>`/`<span>` whether or not it carries `role`, and `FE-ARIA-001`'s flags an ARIA widget role on a `<div>`/`<span>` where a native element provides the pattern, so `<div role="button" onclick>` is reported under both IDs. The overlap predates task 19's cut; 008's data model §Verification phrasing item 5 now asks a Verification to name the sibling that owns an overlapping defect, and which rule owns this one is undecided (owning spec: 069). Found by task 19's independent re-read, 2026-10-28. — routed to framework/rules/accessibility-frontend.md: `FE-SEMHTML-001` owns it (owner, 2026-10-28), and `FE-ARIA-001`'s flag list now leaves a click-listening `<div>`/`<span>` that a native `<button>` or `<a href>` would serve to `FE-SEMHTML-001`
- [x] framework/rules/accessibility-frontend.md — a `<div onclick>` with no role that acts as a checkbox or disclosure is flagged by neither rule: `FE-SEMHTML-001`'s Statement names `<input>` and `<details>`, but its flag list covers only `<button>` and `<a href>` stand-ins, and `FE-ARIA-001` needs a role. Found while working this task, 2026-10-28. — discarded: a flag list that checks less than its Statement, the class task 25 discards for the same reason

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 21. Decide the owner of three overlaps task 19's re-reads found in the security rule files

- [x] `BE-AUTHN-010` (MUST) and `BE-AUTHN-021` (SHOULD) both flag SMS as a privileged account's only second factor — name one owner in both Verifications, or record the double report as deliberate with its reason — recorded as deliberate (owner, 2026-10-28): two defects coincide, the SMS dependence (`BE-AUTHN-010`) and the missing phishing-resistant authenticator (`BE-AUTHN-021`), each with a fix the other's does not make, so an author learns both in one analyze run; `BE-AUTHN-021`'s Verification now says so
- [x] `BE-DATA-003` and `BE-DATA-008` both flag a secret in a `Dockerfile` `ENV` directive or an image layer — name one owner, or record it as deliberate — `BE-DATA-008` owns it (owner, 2026-10-28): `BE-DATA-003`'s flag list drops the `Dockerfile` and image-layer cases and names `BE-DATA-008` for a secret in a container image or container-build manifest, and `configuration-cross.md` `CFG-ENV-010`'s boundary sentence now names `BE-DATA-008` for a container-image secret. A secret written in source or committed to version control outside a container-build manifest stays `BE-DATA-003`'s when the image build copies it in (owner, 2026-10-28), so `BE-DATA-008` owns one written in a container-build manifest or placed into the image by its build from outside source and version control, and its Verification names `BE-DATA-003` for the rest
- [x] `BE-DATA-003` and `BE-DATA-012` both flag a secret hardcoded in source (as a secret in source, and as a replacement path that requires editing source) — name one owner, or record it as deliberate — `BE-DATA-003` owns it (owner, 2026-10-28): `BE-DATA-012`'s Verification now leaves a secret that is itself in source or version control to `BE-DATA-003`, and keeps its replacement-path flag for a secret kept elsewhere whose replacement still needs a code change, such as a pinned secret-manager version
- [x] `BE-DATA-008` and `BE-DATA-012` both flag a secret baked into a container image (as an image-embedded secret, and as a replacement path that requires rebuilding an image) — name one owner, or record it as deliberate. Found while working item 2, 2026-10-28. — `BE-DATA-008` owns it (owner, 2026-10-28), by the same `BE-DATA-012` sentence, which leaves a secret in a container image to `BE-DATA-008`
- [x] `BE-DATA-003` and `BE-DATA-012` both reach a secret injected at build time into a non-container artifact (a properties file packaged into a JAR from CI, never committed): `BE-DATA-003` as not sourced at runtime, `BE-DATA-012` as needing an artifact rebuild. Found while working items 3 and 4, 2026-10-28. — `BE-DATA-003` owns it (owner, 2026-10-28): `BE-DATA-012`'s routing sentence now also leaves a secret placed into a build artifact rather than sourced at runtime to `BE-DATA-003`
- [x] `BE-DATA-003` and `configuration-cross.md` `CFG-ENV-010` both claim a secret the build bakes into a client artifact (a browser bundle, a mobile or desktop build): `BE-DATA-003` as not sourced at runtime, `CFG-ENV-010` as what the build bakes into a client artifact; neither names the other. Present before task 19; found while working item 5, 2026-10-28. — name one owner, or record it as deliberate — `CFG-ENV-010` owns it (owner, 2026-10-28): `BE-DATA-003` and `BE-DATA-012` now name `CFG-ENV-010` for a secret the build injects into an artifact delivered to end users unless the secret is itself in source or version control, which `CFG-ENV-010` already leaves to `BE-DATA-003`; `BE-DATA-012`'s routing names the replacement-path defect, so its revocation-path flags still apply to every secret
- [x] `BE-DATA-012`'s flag names a replacement path that requires "rebuilding an image", while its Statement forbids one needing "an artifact rebuild"; a non-embedded secret reference baked into a non-image artifact is flagged by no rule. Present before task 19; found while working item 6, 2026-10-28. — discarded: a flag list that checks less than its Statement, the class task 25 discards for the same reason
- [x] `configuration-cross.md` `CFG-ENV-001` flags "a secret with a hard-coded default" and `BE-DATA-003`'s Statement forbids a secret "stored in environment-variable defaults shipped with the application", so `process.env.API_KEY ?? "sk_…"` is reported under both, with one fix; `CFG-ENV-001`'s Statement says only "see `BE-DATA-003`". Present before task 19; found while working item 6, 2026-10-28. — name one owner, or record it as deliberate — `CFG-ENV-001` owns it (owner, 2026-10-28), since `configuration-cross.md` loads for every stack: `BE-DATA-003` names `CFG-ENV-001` for a secret given an in-application default value (a fallback the code supplies when the configured value is absent), and `BE-DATA-012`, `CFG-ENV-009` and `CFG-ENV-010` name it for the same case
- [x] A frontend-only project (no `security-backend.md` installed) gets no finding for a client secret in source, version control or a container image: `configuration-cross.md` `CFG-ENV-010` routes those to `BE-DATA-003` and `BE-DATA-008`, and `CFG-ENV-009` routes a credential compiled into the code to `BE-DATA-003`; no loaded rule owns them. The routing to `BE-DATA-003` predates task 19; found by the re-read of item 8, 2026-10-28. — owner's call: give `CFG-ENV-010` those cases, or record the gap with its reason — `CFG-ENV-010` owns them (owner, 2026-10-28): it owns any secret that reaches a client artifact, wherever it sits before the build, and its Statement now forbids one whether injected by the build or written in client source, so a literal such as `const STRIPE_SECRET = "sk_live_…"` is flagged in every project; `BE-DATA-003`, `BE-DATA-008`, `BE-DATA-012` and `CFG-ENV-009` name it, and 017's data-model gist matches the widened Statement
- [x] A frontend-only project gets no finding for a secret in version control that never reaches a client artifact (a token in a `vite.config.ts` dev proxy, a deploy token in a committed script): `CFG-ENV-009` and `CFG-ENV-010` leave it to `security-backend.md` `BE-DATA-003`, which that project does not install, and no `-frontend.md` or `-cross.md` rule covers it. Present before task 19; found by the re-read of item 9, 2026-10-28. The same holds for a secret in a container image or container-build manifest that never reaches a client artifact (an `ARG NPM_TOKEN` used only for `npm install` in a SPA build stage, a copied `.npmrc` carrying a registry token, a secret `ENV` in the image that serves the SPA): `CFG-ENV-010` leaves it to `BE-DATA-008`, also not installed there; found by the re-read of item 8's `CFG-ENV-010` widening, 2026-10-28. So does one CI packages into a build artifact that is neither a client artifact nor a container image, which `CFG-ENV-010` leaves to `BE-DATA-003`; found by the next re-read, 2026-10-28. — owner's call: a cross-stack home for it, or record the gap with its reason — `configuration-cross.md` `CFG-ENV-013` owns it (owner, 2026-10-28), per the outcome below

Outcome of the last item (owner, 2026-10-28, option 1): `configuration-cross.md` `CFG-ENV-013` (MUST) owns a secret at rest in every project shape, landed on 017 through its back-edge (scenario `a-secret-at-rest-is-flagged-in-every-project-shape`, task 44, AC28). This supersedes the at-rest half of items 2 and 3: `BE-DATA-003` keeps runtime sourcing and `BE-DATA-008` container runtime injection, for a secret the running application or container uses, and an unnamed injection into a running container the plan describes is `BE-DATA-008`'s (owner, 2026-10-28). Which review pass checks code against `CFG-ENV-013` is recorded on 020's `every-loaded-rule-file-has-a-pass` (owner, 2026-10-29), and a committed secret encrypted to OWASP's Secrets Management Cheat Sheet §3.6 conditions is permitted (owner, 2026-10-29). The `BE-LOG-002` / `CFG-ENV-006` overlap the re-reads found is task 28.

- **Done when**: each overlap is named, in the Verification of the rule that yields it, as its owner's, per data-model §Verification phrasing item 5, or is recorded as deliberate with its reason; the overlaps predate task 19, which the re-reads surfaced on 2026-10-28

## 22. Verify the adoption audit's rule-file check with a real adopter bootstrap

- [x] Task 19 changed `framework/bootstrap/ductus-procedure.md` §Loading rule files (the security-rule audit's integrity check now requires Statement and Verification only). Run `/ductus --ref=main` on an adopter subject that has feature specs and no `specs/rules/security-*.md` yet, so the audit triggers on a newly created rule file; expected: both cut security files load in the audit with no `failed to load` line. A pinned copy is never newly created, so the audit never reads one (§Trigger, item 1): that a pinned pre-cut copy still carrying `**Rationale:**` paragraphs loads is `/{project}:analyze`'s, shown by a runtime test proved to fail without its handling of a Rationale paragraph (owner, 2026-10-29)

Result, 2026-10-29 (owner chose to run it): `/ductus --ref=main` on a first-adoption subject (no prior `ductus`; two feature specs, no `specs/rules/`), run against `main` at `28167f78`, installer and archive both from that commit, runtime `0.59.11` from the store. Both security files were newly created, byte-identical to `28167f78` after `{project}` substitution, with no `**Rationale:**` paragraph; the audit loaded 93 backend and 42 frontend rules with no `failed to load` line and reported `Security audit: 65 gap(s) in 2 existing spec(s) — nothing was written.` The pinned-copy half: `runtime/src/primitives/rule_sections.rs` `a_category_heading_is_not_a_rule_and_ends_the_one_before_it` parses a Verification after a `**Rationale:**` paragraph, and it and `a_deprecated_section_parses_like_any_other` fail when the parser stops at the first field before the Verification (mutation run in the dev profile, reverted). The bootstrap defects the run surfaced are task 29.

- **Done when**: the bootstrap ran and its output matches the expected line, recorded on this task with the subject's starting version and the commit it ran against, or the owner waives the run with a reason written on this task

## 23. Disposition out-of-spec findings: NOTE-level gaps in non-security rule files that predate task 19

- [x] framework/rules/reliability-backend.md — `BE-BREAKER-001` and `BE-BULK-002` both flag an optional, failure-prone dependency with no breaker and no degraded path; neither names the other (owning spec: 039) — fixed (owner, 2026-10-29): recorded as deliberate in `BE-BREAKER-001`'s Verification, two defects with different fixes, the operation failing with the dependency (`BE-BULK-002`) and the calls continuing while it is down (`BE-BREAKER-001`)
- [x] framework/rules/api-backend.md — `BE-LRO-001`'s "failure outside the project error envelope" and `BE-ERRENV-001`'s "ad-hoc error formats" can both fire on a status monitor's failure body (owning spec: 068) — fixed (owner, 2026-10-29): `BE-LRO-001` owns it, since a status monitor reports a failed operation in a success response, which `BE-ERRENV-001`'s Statement on error responses does not cover; `BE-ERRENV-001`, `BE-STATUS-001` and `security-backend.md` `BE-ERR-002` name `BE-LRO-001` for it, `BE-ERRENV-002` names it for a failure reported outside the envelope and keeps the codes of one inside it, and `BE-LRO-001` names `BE-ERRENV-001` for a failure the status monitor reports in an error response; the naked-string overlap the re-reads found is task 30
- [x] framework/rules/accessibility-frontend.md — `FE-A11YMEDIA-002`'s trigger reaches live audio-only media neither Statement obliges; its flag list omits the video-only alternative its Statement requires; its Verification does not name the media-alternative-for-text exemption (owning spec: 069) — fixed in part (owner, 2026-10-29): the trigger now names prerecorded media and a Not-flagged clause names a labelled media alternative for text, removing two blocking false positives; the video-only flag-list gap is discarded, a flag list that checks less than its Statement while the commitment sentence still asks for everything
- [x] framework/rules/accessibility-frontend.md — `FE-KBD-002` routes "too little changed from the unfocused state" to `FE-KBD-005`, whose flag list checks thickness only (owning spec: 069) — discarded (owner, 2026-10-29): `FE-KBD-005`'s commitment sentence asks for the change-of-contrast bar, so the routed case stays its rule's; a flag list that checks less than its Statement while the commitment sentence still asks for everything
- [x] framework/rules/concurrency-backend.md — `BE-COORD-003`'s flag sentence names only commit-then-publish, not a write to a second datastore its Statement covers; `BE-COORD-004` routes a dead-lettered ordered message to `BE-RETRY-005`, whose flag list does not name it (owning specs: 038, 039) — discarded (owner, 2026-10-29): both are a flag list that checks less than its Statement while the commitment sentence still asks for everything, `BE-COORD-003`'s for a second-datastore write and `BE-RETRY-005`'s for a dead-letter that skips a strictly ordered message
- [x] framework/rules/observability-backend.md — `BE-HEALTH-001` and `BE-HEALTH-002` can both fire on a combined probe that checks dependencies; and neither `BE-HEALTH-001` nor `BE-BULK-002` flags a dependency critical to some operation but left out of readiness, though `BE-HEALTH-001`'s Statement requires it (owning specs: 037, 039) — fixed (owner, 2026-10-29): `BE-HEALTH-002` records the combined probe's double report as deliberate, the conflation (`BE-HEALTH-001`) and liveness failing on a dependency (`BE-HEALTH-002`); `BE-HEALTH-001` now flags readiness that leaves out a dependency the plan treats as critical to an operation

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 24. Fix pre-existing Statement/Verification mismatches in the security rule files that task 19's reviews found

- [x] `BE-ERR-001` — the Statement allows verbose errors behind authentication in production, but the Verification flags any internal-detail response on a production path, so a compliant authenticated design is a blocking finding — Statement widened (owner, 2026-10-29): "or behind authentication" is dropped, so production returns no internal detail to any user, matching the cited OWASP Error Handling Cheat Sheet, which returns details to no user in production
- [x] `BE-API-009` — the Verification accepts a signed URL parameter in place of the destination allowlist, which the Statement does not name — Verification narrowed (owner, 2026-10-29): the signed-URL alternative is dropped, since the cited OWASP Unvalidated Redirects and Forwards Cheat Sheet names an allowlist, server-side ID mapping and a confirmation page, not signing
- [x] `BE-ERR-004` — the Verification accepts a documented alternative error format the Statement does not name — Statement widened (owner, 2026-10-29): a structured format documented in `specs/system.md` and used uniformly MAY replace RFC 9457, as `api-backend.md` `BE-ERRENV-003`'s Statement already allows
- [x] `BE-DATA-001` — the Verification requires a loopback exception to be named with its sidecar or proxy; the Statement requires only that it be documented in `specs/system.md` — Verification narrowed (owner, 2026-10-29): a plaintext loopback exception is documented in `specs/system.md`, the sidecar being the Statement's example, not its condition, so a same-host design with no leg off the host can comply
- [x] `BE-DATA-006` — the Verification asks for a documented purpose per field; the Statement asks for a documented purpose — Verification narrowed (owner, 2026-10-29): it asks for the documented purpose, as the Statement and GDPR Article 5(1)(b) do, each field's necessity staying checked by its minimum-data-set item
- [x] `BE-DATA-007` — the Verification's per-environment accounts and separate migration account are not in the Statement — Statement widened (owner, 2026-10-29): each environment has its own application accounts, as the cited OWASP Database Security Cheat Sheet says, and migrations run under an account the application does not use, which the no-schema-modification MUST already implies
- [x] `FE-AUTHN-001` — the Verification accepts signed-token validation, which the Statement does not name — Verification narrowed (owner, 2026-10-29): it names an allowlist of paths or hosts only, as `BE-API-009` now does, on the same OWASP source
- [x] `FE-AUTHN-002` — the Verification requires a data-preservation behavior the Statement does not oblige — Verification narrowed (owner, 2026-10-29): the data-preservation requirement is dropped; the cited OWASP Session Management Cheat Sheet sets none, and preserving work is WCAG 2.2 SC 2.2.5, Level AAA
- [x] `FE-DEPS-004` — the Verification requires the lockfile to be included in the deployment artifact; the Statement does not — Verification narrowed (owner, 2026-10-29): the lockfile is the one the production build installs from, where the Statement places it
- [x] `BE-AUTHN-006` — the trigger names password-reset endpoints; the Statement covers authentication failure responses — Statement widened (owner, 2026-10-29): a password-reset response does not reveal whether the account exists, its responses for an existing and an unknown account indistinguishable in body, status and timing, as the cited OWASP Authentication Cheat Sheet's password-recovery example shows and `BE-AUTHN-018`'s routing assumes
- [x] `BE-API-015` — the trigger reaches every WebSocket endpoint; the Statement covers one that serves non-public data or accepts state-changing messages — trigger narrowed (owner, 2026-10-29) to the Statement's scope, so a public read-only feed is not held to an `Origin` allowlist and authentication
- [x] `FE-STORAGE-002`, `FE-STORAGE-004`, `FE-STORAGE-005` — the trigger "session, authentication, or other privileged cookie" is wider than the Statements' session and authentication cookies — triggers narrowed (owner, 2026-10-29) to a session or authentication cookie, `FE-STORAGE-005`'s flag with them, since "privileged" was undefined and a client-readable double-submit CSRF cookie would have been held to `HttpOnly`
- [x] `FE-PII-003` — the trigger and flag cover every authenticated page; the Statement covers pages displaying sensitive data — trigger and flag narrowed (owner, 2026-10-29) to pages displaying sensitive data, as the Statement and the cited OWASP Session Management Cheat Sheet scope `no-store`
- [x] `FE-STORAGE-002` and `FE-CSRF-002` both flag a missing `SameSite`; `FE-STORAGE-002`'s Not-flagged list routes only a wrong or unjustified value to `FE-CSRF-002` — fixed (owner, 2026-10-29): `FE-CSRF-002` owns every `SameSite` defect, missing, wrong or unjustified, and `FE-STORAGE-002` owns a missing `HttpOnly` or `Secure`, a `SameSite=None` cookie's included, each flag scoped to the session or authentication cookie its trigger names; the backend `BE-AUTHN-014` overlap is task 30
- [x] `BE-API-006`'s per-format-hardening flag and `BE-INPUT-009` (XML parser hardening) or `BE-INPUT-008` (deserialization) both fire on a JSON-plus-XML endpoint with an unhardened XML parser; neither names the other — fixed (owner, 2026-10-29): the format-specific rules own their format, and `BE-API-006` names `BE-INPUT-009` for an unhardened XML parser and `BE-INPUT-008` for any deserialization it flags

- **Done when**: each item is fixed so its Statement and Verification agree — the Statement gains the case in the rule's tier, the Verification stops flagging or asking for it, or a double report is named, in the Verification of the rule that yields it, as its owner's, per data-model §Verification phrasing item 5 — with the direction the owner's call per item and each fix independently reviewed; the mismatches predate task 19, whose reviews found them on 2026-10-28

## 25. Disposition out-of-spec findings: pre-existing Statement/Verification mismatches in non-security rule files that task 19's reviews found

- [x] framework/rules/accessibility-frontend.md — `FE-KBD-001`'s flag ignores the Statement's path-dependent exception, so a freehand-drawing or signature-capture design is flagged (owning spec: 069) — fixed (owner, 2026-10-29): a Not-flagged clause names an interaction whose underlying function, not its input technique, depends on the path of the user's movement, the Statement's own exception, read as W3C Understanding 2.1.1 Note 1 reads it
- [x] framework/rules/accessibility-frontend.md — `FE-CONTRAST-001`'s "pair combinations below threshold" flag ignores the MAY's exceptions (inactive components, decorative content, logotypes, browser-determined colours) (owning spec: 069) — fixed (owner, 2026-10-29): a Not-flagged clause names the pairs the Statement's MAY covers
- [x] framework/rules/accessibility-frontend.md — `FE-KBD-007`'s Escape-dismissal flag does not exempt input-error content, which the Statement exempts (owning spec: 069) — fixed (owner, 2026-10-29): a Not-flagged clause exempts content that communicates an input error from the dismissal flag only, as WCAG 2.2 SC 1.4.13 does
- [x] framework/rules/accessibility-frontend.md — `FE-A11YFORM-007`'s Statement covers a submission that modifies or deletes user-controllable data, but its trigger names deletion only (owning spec: 069) — fixed (owner, 2026-10-29): the trigger names deletion or modification of user-controllable data in storage other than the simple creation or editing of a document or record, as WCAG 2.2 SC 3.3.4 and the Statement's MAY read
- [x] framework/rules/performance-backend.md — `BE-POOL-001`'s Statement covers caches, but its trigger names a database or external service only (owning spec: 034) — fixed (owner, 2026-10-29): the trigger names a database, cache, or external service
- [x] framework/rules/performance-backend.md — `BE-POOL-004`'s Verification accepts a connection proxy, which the Statement does not name (owning spec: 034) — fixed (owner, 2026-10-29): the Statement gains a MAY for a connection proxy, counted in the sum for the pools that go through it as the most backend connections it can hold across all its instances and pools; the re-reads also made the sum count every pool and process of an instance and the other clients' maximum, net of slots the backend reserves for administrative roles, and made a backend downsize fire the trigger (owner, 2026-10-29)
- [x] framework/rules/concurrency-backend.md — `BE-TXN-001`'s Verification asks for the anomalies the isolation level prevents, which the Statement does not oblige (owning spec: 038) — Statement widened (owner, 2026-10-29): it asks for the anomalies the level prevents on the chosen engine, as the rule's sources, Jepsen's MySQL report among them, show a level's name does not
- [x] framework/rules/reliability-backend.md — `BE-RETRY-003`'s flag reaches any downstream throttling signal; its Statement obliges waiting only for `Retry-After` (owning spec: 039) — Statement widened (owner, 2026-10-29) to a `Retry-After` or the downstream's equivalent throttling hint
- [x] framework/rules/reliability-backend.md — `BE-RETRY-001`'s flag list names bound, jitter and idempotency basis but not the backoff its Statement requires, which `BE-TXN-004` and `BE-COORD-002` route to it (owning spec: 039) — discarded (owner, 2026-10-29): the commitment sentence asks for the backoff-with-jitter policy, so a routed backoff case stays this rule's; a flag list that checks less than its Statement while the commitment sentence still asks for everything
- [x] framework/rules/configuration-cross.md — `CFG-FLAG-001`'s Verification flags any evaluation failure, broader than the Statement's three conditions (owning spec: 017) — Statement widened (owner, 2026-10-29) to any evaluation failure, the three conditions kept as examples, as OpenFeature Requirement 1.4.10 reads; 017's data-model gist matches
- [x] framework/rules/configuration-cross.md — `CFG-ENV-004` and `CFG-ENV-011`: the unit-in-name item requires the unit suffix on the default constant's name, while the Statements also accept a typed duration or size constant (owning spec: 017) — Verifications narrowed (owner, 2026-10-29): under unit-in-name the default constant is a typed duration or size or carries the matching suffix, as the Statements allow; and (owner, 2026-10-29) that default-constant check now applies under both patterns, where the variable has a default, as each Statement's default-constant sentence does, flagging a bare-integer default with no unit suffix
- [x] framework/rules/observability-backend.md — `BE-METRIC-002`'s trigger reaches any queue where the Statement names the work queue that feeds the resource, and no flag checks its error count (owning spec: 037) — fixed in part (owner, 2026-10-29): the trigger names a bounded resource pool, a worker set, or the work queue that feeds one; the error-count flag gap is discarded, a flag list that checks less than its Statement while the commitment sentence still asks for everything
- [x] framework/rules/accessibility-frontend.md — `FE-POINTER-002`'s MAY lets "signature capture" stay drag- or path-only without qualification, while W3C Understanding 2.5.1 says "entering your signature may be inherently path-based (although acknowledging something or confirming your identity need not be)", and `FE-KBD-001` now flags a signature that only acknowledges something; found by task 25's fourth re-read, 2026-10-29 (owning spec: 069) — fixed (owner, 2026-10-29): the MAY's example is a signature whose drawn mark is itself the function, not one that only acknowledges something or confirms identity, as W3C Understanding 2.5.1 reads and `FE-KBD-001` now does

The same reviews found 31 rules whose flag list checks less than the Statement while the Verification's commitment sentence still asks for everything; they are discarded, not recorded as items (owner, 2026-10-28). The commitment sentence already binds the artifact to each obligation, and data-model §Verification phrasing does not require a flag list to enumerate them. 21 are in 008's files, 4 in 068's, 3 in 037's, 2 in 017's and 1 in 038's. Discarded with them, for the same reason: `FE-STORAGE-002` and `FE-CSRF-002` do not name `BE-AUTHN-014` back.

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 26. Implement scenario: a-custom-credential-header-does-not-follow-a-redirect

- [x] Implement the behavior described in `scenarios/a-custom-credential-header-does-not-follow-a-redirect.md` — `BE-DATA-013` (MUST), scoped by origin (owner, 2026-10-29); a credential in a request body is task 31

- **Done when**: security-backend.md carries the rule, its ID registered under 008's data-model conventions, every claim it makes about library behavior fetched from the library's documentation or source and cited in its Source (data-model § Rule entry fields), and check-rule-ids accepts the file.

## 27. Implement scenario: single-host-appliance-rule-fit

- [x] Implement the behavior described in `scenarios/single-host-appliance-rule-fit.md` — `BE-DATA-003`, `BE-DATA-010` and `BE-DATA-011` amended in place with IDs kept, as the scenario's Resolved Questions read (owner, 2026-10-29)

- **Done when**: The scenario's open questions are resolved, BE-DATA-003, BE-DATA-010 and (per the policy call) BE-DATA-011 are amended in place in framework/rules/security-backend.md with their IDs kept, and check-rule-ids accepts the file.

## 28. Decide the owner of a sensitive configuration value in a log line or error report

- [ ] `configuration-cross.md` `CFG-ENV-006` and `BE-LOG-002` both reach a sensitive configuration value in a log line — a configuration object holding `dbPassword` logged at startup, a single `DB_PASSWORD` value logged, a `CFG-ENV-008` malformed-value error printed unredacted: `CFG-ENV-006` hands "any other sensitive value in a log line" to `BE-LOG-002`, which frontend-only and cross-only projects do not install, and `BE-LOG-002` names neither case, so a full-stack project can report one defect under both IDs and the other shapes can report it under neither. Present before task 21; found by the re-read of task 21's last item, 2026-10-29. — owner's call: put the boundary at the configuration value (`CFG-ENV-006` owns a sensitive configuration value in any operator-facing output, alone or in a configuration object, and `BE-LOG-002` names it), or record the overlap as deliberate with its reason
- [ ] `BE-LOG-008` and `configuration-cross.md` `CFG-ENV-006` both reach a configuration object in an error-reporting payload or span attribute (`Sentry.setContext("config", config)`): `BE-LOG-008`'s Not-flagged list yields only a log line, to `BE-LOG-002`. Same re-read, 2026-10-29. — owner's call, with the item above

- **Done when**: each overlap is named, in the Verification of the rule that yields it, as its owner's, per data-model §Verification phrasing item 5, with the owner installed wherever the yielding rule is, or is recorded as deliberate with its reason; a fresh agent's re-read of the changed rules finds no MUST or SHOULD

## 29. Disposition out-of-spec findings: defects task 22's adopter bootstrap surfaced

- [ ] framework/bootstrap/ductus-procedure.md — on a brownfield run whose specs carry unlabelled criteria, the Pre-run Migrations' `criterion-label-backfill` writes labels under `specs/`, and the Frontmatter Migration's precheck then refuses ("Migration requires a clean working tree under `specs/`"), so the Frontmatter Migration cannot run on that adopter's first run; its "Exit before any modifications" also does not say whether it ends the section or the run (candidate owners: 027, 017)
- [ ] framework/migrations/criterion-label-backfill.md — step 1 runs "the labelling pass in its reporting form", which `label-criteria` does not have (its only argument is the feature); separately, `label-criteria --help` splits its first line as `Assign stable AC` / `labels` (candidate owners: 013, 022)
- [ ] framework/bootstrap/ductus.md and ductus-procedure.md — neither half says where the archive half's sections (Pre-run Migrations, Frontmatter Migration, the security audit, Hook Installation) fall among the walker's steps 4–10, so the order is the host's guess (candidate owner: 056)
- [ ] framework/bootstrap/ductus-procedure.md — a first adoption into a project whose `specs/` already exists is treated as an update and prints the update-mode completion message, so it never shows the first-run next steps such as `/{project}:configure` (candidate owners: 002, 003)

- **Done when**: each item is fixed, routed to its owning spec as a scenario or task that back-links this one, or discarded with its reason written here

## 30. Decide the owner of a naked-string error response and a missing correlation ID

- [ ] `api-backend.md` `BE-ERRENV-001` ("naked strings"), `BE-ERRENV-002` ("only by message text without a stable code") and `BE-ERR-002` ("only a string message") all flag an endpoint whose error response is a bare string such as `500 {"error":"boom"}`, on any endpoint; `BE-ERR-002`'s Not-flagged clause yields only an error code's schema and stability to `BE-ERRENV-002`, and `BE-ERRENV-001` yields only the correlation ID to `BE-ERR-002`, so one defect is reported under up to three IDs in a full-stack project. Present before task 19; found by the second re-read of task 23, 2026-10-29. — owner's call: name one owner in the others' Verifications, or record the overlap as deliberate with its reason
- [ ] `BE-LOG-006` flags an error response that does not show the correlation ID "per `BE-ERR-002`", which flags the same omission. Same re-read. — owner's call, with the item above
- [ ] `BE-ERR-004` and `api-backend.md` `BE-ERRENV-003` both warn on an error format that is neither RFC 9457 nor documented in `specs/system.md`; `BE-ERR-004` says only that `BE-ERRENV-003` "states the same preference", and neither yields. Found while working task 24's `BE-ERR-004` item, 2026-10-29. — owner's call, with the items above
- [ ] `BE-AUTHN-014` and `security-frontend.md` `FE-STORAGE-002` / `FE-CSRF-002` both flag a session or authentication cookie's missing `HttpOnly`, `Secure` or `SameSite` in a full-stack project; `BE-AUTHN-014` says only that the frontend rules "state the same attributes", and none yields. Found while working task 24's `SameSite` item, 2026-10-29. — owner's call, with the items above
- [ ] `BE-API-015` and `BE-API-013` both flag a token in a WebSocket's opening-handshake URL (`wss://…/chat?access_token=…`); neither yields, and both sit in `security-backend.md`. The discard recorded in the `decisions:` of 008's, 017's, 022's and 063's `review.md` gives as one reason that a sentence "matches `BE-API-015`'s unscoped Verification", which task 24 scoped, so the next review of each re-decides that entry rather than carrying it byte-identical. Found by task 24's second re-read, 2026-10-29. — owner's call, with the items above
- [ ] `BE-DATA-001` and `BE-API-008` both flag a database connection whose spec names no TLS, and neither yields; since task 24 a plaintext loopback database connection documented in `specs/system.md` meets `BE-DATA-001`'s MAY while `BE-API-008`'s "authenticated, encrypted channels" MUST still blocks it, the single-host case task 27 concerns. Same re-read. — owner's call, with the items above and task 27
- [ ] Backend and frontend rules both flag one defect in a full-stack project, neither naming the other: `BE-API-009` and `FE-AUTHN-001` (a post-login `?next=` redirect with no allowlist, now with word-for-word the same obligation), `BE-API-001` and `FE-PII-003` (a sensitive page without `Cache-Control: no-store`), and `BE-DEPS-002` and `FE-DEPS-004` (`^` ranges in a frontend production build). Same re-read. — owner's call, with the `BE-AUTHN-014` item above
- [ ] `accessibility-frontend.md` `FE-KBD-007` and `FE-A11YFORM-002` both flag an input-error tooltip, referenced by `aria-describedby`, that a timer hides while its field keeps focus; neither yields, and both sit in one file. Found by task 25's second re-read, 2026-10-29. — owner's call, with the items above
- [ ] `performance-backend.md` `BE-POOL-001` and `configuration-cross.md` `CFG-CONST-003` both flag a pool size written as a bare literal (`new Pool({ max: 20 })`); `BE-LOCK-002` already yields the same case to `CFG-CONST-003`, which installs wherever `BE-POOL-001` does. Same re-read. — owner's call, with the items above

- **Done when**: each overlap is named, in the Verification of the rule that yields it, as its owner's, per data-model §Verification phrasing item 5, with the owner installed wherever the yielding rule is, or is recorded as deliberate with its reason; a fresh agent's re-read of the changed rules finds no MUST or SHOULD

## 31. Decide whether a credential in a request body resent on a 307 or 308 redirect needs a rule

- [ ] A credential sent in a request body — OAuth `client_secret_post`, a form-posted token — is resent with the body when a 307 or 308 redirect sends the request to another host: Go's `net/http` keeps the method and body on a 307 or 308 when `GetBody` is defined, and Python `requests` keeps the body on them. `BE-DATA-013` governs a credential in a request header only, and no rule covers the body. Found by the re-read of `BE-DATA-013`, 2026-10-29. — owner's call: widen `BE-DATA-013` to a credential in the body, add a rule, or record the gap with its reason

- **Done when**: the gap is closed by a rule whose every library claim is fetched and cited, with a fresh agent's re-read finding no MUST or SHOULD, or it is recorded with its reason on this task
