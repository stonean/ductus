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
