---
status: planned
dependencies: [008-security-rules, 024-rule-loader, 033-rule-surface-setting]
next-criterion: 12
cross-spec-impact: [008-security-rules, 017-derive-dont-ask]
---

# 077 — Security cross rules

Introduce `framework/rules/security-cross.md`, a security rule file with the `-cross.md` suffix, so its rules install in every project shape: frontend-only, backend-only, cross-only (a command-line tool or a library) and full-stack. It holds the security obligations [008 — Security Rules](../008-security-rules/spec.md) decided belong on every surface: one new rule, and four moved from `security-backend.md`, whose old IDs are deprecated. This is a rule-introducing spec, as constitution §rules requires of a rule file `ductus` ships; 008's task 32 records the decision that created it.

## Motivation

`security-backend.md` and `security-frontend.md` installed only with their surface (`[rules] surfaces`, [033-rule-surface-setting](../033-rule-surface-setting/spec.md)), and no `-cross.md` file carried a security rule. So obligations that hold wherever code runs went unreported in some shapes. 008's re-reads of 2026-10-29 to 2026-11-02 found these cases:

- A sensitive value that is not configuration, such as an access token, a session identifier, a password or sensitive PII, written to a log line, an error-reporting payload or telemetry, was reported only where `security-backend.md` loaded (`BE-LOG-002`, `BE-LOG-008`). A browser app sending the user's access token to its error reporter, and a command-line tool logging an OAuth access token it obtained, had no owner (008 task 32).
- Build inputs the lockfile does not cover, such as CI actions, reusable workflows, container images and downloaded tools, were pinned only by `BE-DEPS-006`. A frontend-only project's deploy workflow using `uses: org/action@v4` was reported by no rule (008 task 40, item 1).
- A non-constant-time comparison of a secret or signature (`BE-AUTHN-009`), and an OAuth or OIDC client's `state`, PKCE, `nonce`, exact `redirect_uri`, grant and response types and refresh-token protection (`BE-AUTHN-013`, `BE-AUTHN-020`), were reported by no rule in a cross-only project. In a frontend-only project, `FE-AUTHN-001` asked an OAuth client only for an allowlist of hosts or paths (008 task 40, items 4 and 5).

## Rule set scope

`security-cross.md` is loaded for every stack through the rule loader's suffix discovery ([024-rule-loader](../024-rule-loader/spec.md)). It follows the canonical rule schema of 008's `data-model.md`. Its rule IDs follow `SEC-{CATEGORY}-{NNN}`, the domain prefix the other `-cross.md` files set (`CFG`, `QUAL`), with three categories: `LOG` (sensitive values in logs and telemetry), `DEPS` (build inputs) and `AUTHN` (authentication). Its header declares them, and this spec's `data-model.md` registers them.

The file carries five rules.

1. **`SEC-LOG-001`: sensitive values in logs, error reports and telemetry (new).**
   - The values `security-backend.md` `BE-LOG-002` bars from logs, and `BE-LOG-008` from telemetry, MUST NOT be written to a log line, an error-reporting payload or telemetry, in any shape. They include passwords, full authentication tokens, session IDs, one-time and recovery codes, raw PII and other sensitive field values.
   - The rule names such a value as `BE-LOG-002`'s in a log line, or as `BE-LOG-008`'s in telemetry or an error-reporting payload, where `security-backend.md` is also loaded.
   - A sensitive configuration value stays `configuration-cross.md` `CFG-ENV-006`'s.
   - Source: the OWASP Logging Cheat Sheet ("Data to exclude"; its event sources include "Client software" and a "JavaScript exception handler"), fetched when the rule is written.
2. **`SEC-DEPS-001`: build-input pinning (moved from `BE-DEPS-006`).**
   - It moves as 008 task 40's first item and task 42 decided: an immutable reference for every build input the lockfile does not cover.
   - A workflow onboarded to a GitHub Actions workflow dependency lockfile pins its actions. A reusable workflow call takes a full commit SHA, and a container image a workflow runs takes a digest.
   - Its yield to `BE-DEPS-002` takes "where `security-backend.md` is also loaded".
3. **`SEC-AUTHN-001`: constant-time comparison (moved from `BE-AUTHN-009`).** Its yield to `BE-API-010` (d) takes the same load condition.
4. **`SEC-AUTHN-002`: OAuth and OIDC client flows (moved from `BE-AUTHN-013`).**
   - The rule covers `state`, PKCE, `nonce` validation and exact `redirect_uri` matching.
   - It includes the native-app loopback port and the pushed-authorization-request relaxation, which covers only a `redirect_uri` the client does not take from the request.
   - Its yield to `BE-API-009` takes the load condition.
5. **`SEC-AUTHN-003`: OAuth grant and response types (moved whole from `BE-AUTHN-020`).** The rule covers the forbidden grants and response types, the PKCE challenge method, and public-client refresh-token protection.

Each moved rule carries its predecessor's Statement and Verification as they stand when it moves, re-pointed but not reworded. Its Sources are re-fetched when it lands.

### Deprecation and re-pointing

- `BE-DEPS-006`, `BE-AUTHN-009`, `BE-AUTHN-013` and `BE-AUTHN-020` are labelled `**DEPRECATED in {version}:**`, naming the release that ships this spec, per constitution §rules. Their removal target is the second minor release after it: deprecated in `0.X.0`, removed in `0.(X+2).0`. Each names its successor as the owner of every case it governed, with no load condition, since a `-cross.md` owner installs in every shape. A deprecated rule binds until it is removed.
- Every yield and Statement that names one of the four as current re-points to the successor:
  - `BE-API-009`, `BE-API-010` (d), `BE-API-011` (including its "Token comparison MUST be constant-time per `BE-AUTHN-009`") and `BE-API-013` in `security-backend.md`;
  - `FE-AUTHN-001` and `FE-STORAGE-003` in `security-frontend.md`.
  The frontend yields drop "where `security-backend.md` is also loaded" (008 `data-model.md` §Verification phrasing, item 6).
- Every rule that names `BE-LOG-002` or `BE-LOG-008` as the owner of a non-configuration sensitive value in a log line or telemetry names the new rule where `security-backend.md` is not loaded. `CFG-ENV-006` is one of them.
- The sweep covers every live artifact that cites one of the four IDs as a current requirement. That includes `framework/templates/ci/adopter-generators.yml`, 008's `data-model.md`, 017's scenario `the-adopter-ci-template-can-run`, 017's and 022's `plan.md`, and the comments in `.github/workflows/framework-checks.yml`, `.github/workflows/runtime-release.yml` and `scripts/check-runtime-currency.sh`. A decision record, a changelog entry, a `review.md` key and an `analysis.md` record keep the old ID.

### Distribution

- The file gets its manifest row in `framework/bootstrap/ductus.md` and its row in `docs/rules.md`.
- `runtime/CHANGELOG.md`'s `## [Unreleased]` section names the four ID changes, their successors, the removal version, and the waivers they expire. A waiver is anchored on its rule ID and file (`framework/reference/review.md`, §Waivers), so one recorded under an old ID stops applying wherever the finding moves to the successor's ID, which then needs a waiver of its own. 008 task 32's follow-on item records this obligation (owner, 2026-11-02).

### Edge cases

- **An adopter spec that cites a deprecated ID.** `/{project}:analyze` reports it as Advisory ("references deprecated rule {ID}; targeted for removal in {version}") until the removal release, and as Blocking ("references unknown rule {ID}") after it. The fix is to re-point the citation to the successor the changelog names.
- **A project that pins `security-backend.md`** (`.ductus/config.toml` `pinned.files`). It keeps the four `BE-` rules undeprecated and without their yields to the successors, while `security-cross.md` still installs beside them. `/{project}:analyze` then reports each moved case under both IDs until the project re-syncs the pinned file. The changelog entry says so; nothing in the shipped files can reach a pinned copy.
- **A project that excludes `security-cross.md` from review.** `/{project}:analyze` still loads it (constitution §rules), so the successors still own their cases there.
- **`/{project}:review` before 020 task 15 lands.** Review checks code against each loaded rule's Statement and applies no Not-flagged routing, so a moved case is reported under both the deprecated ID and its successor until 020's scenario `review-reports-a-defect-under-its-owner` lands or the deprecated rule is removed.
- **A waiver under a deprecated ID.** It is anchored on its rule ID and file, so it stops applying wherever the finding moves to the successor's ID.

## Acceptance Criteria

- [ ] AC1: `framework/rules/security-cross.md` exists in the canonical rule schema. Its header declares the `SEC-{CATEGORY}-{NNN}` format and the categories `LOG`, `DEPS` and `AUTHN`, and it holds the five rules of §Rule set scope, each with a Statement, a Verification and a Source fetched when the rule was written.
- [ ] AC2: `specs/077-security-cross-rules/data-model.md` registers the file's ID prefix and categories, and `/ductus:analyze` loads the file: `check-rule-ids` accepts each of its IDs, and every citation of a successor resolves.
- [ ] AC3: A sensitive value that is not configuration, in a log line, an error-reporting payload or telemetry, is reported by the new log rule in a frontend-only and a cross-only project. In a backend-only and a full-stack project it is reported once, by `BE-LOG-002` or `BE-LOG-008`, which the new rule names as owner where `security-backend.md` is loaded.
- [ ] AC4: The four moved rules carry their predecessors' Statements and Verifications as decided in 008's tasks 40 and 42, and each successor's yield to a backend owner carries "where `security-backend.md` is also loaded".
- [ ] AC5: `BE-DEPS-006`, `BE-AUTHN-009`, `BE-AUTHN-013` and `BE-AUTHN-020` each carry a `**DEPRECATED in {version}:**` label naming the release that ships this spec and a removal target two minor releases later, and each names its successor with no load condition.
- [ ] AC6: Every yield and Statement in `framework/rules/security-backend.md`, `framework/rules/security-frontend.md` and `framework/rules/configuration-cross.md` that names one of the four deprecated IDs, or `BE-LOG-002` or `BE-LOG-008` as owner of a non-configuration sensitive value, names the successor as §Deprecation and re-pointing states.
- [ ] AC7: No live artifact outside a decision record, a changelog entry or an audit record cites one of the four deprecated IDs as a current requirement.
- [ ] AC8: `framework/bootstrap/ductus.md` ships the file in every shape, and `docs/rules.md` lists it. A frontend-only, a backend-only and a cross-only `[rules] surfaces` setting each install it.
- [ ] AC9: `runtime/CHANGELOG.md`'s `## [Unreleased]` section names the four ID changes and the waivers they expire.
- [ ] AC11: `runtime/CHANGELOG.md`'s `## [Unreleased]` section also names each successor, the removal version, and the double report a project pinning `security-backend.md` sees until it re-syncs the file.
- [ ] AC10: A fresh agent's re-read of the new file and of every rule this spec changes, across all four project shapes, finds no MUST or SHOULD.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which ID prefix and categories does `security-cross.md` use?** → `SEC-{CATEGORY}-{NNN}`, with categories `LOG`, `DEPS` and `AUTHN`: `SEC-LOG-001`, `SEC-DEPS-001`, and `SEC-AUTHN-001` to `-003`. It follows the domain prefix the other `-cross.md` files register (`CFG` in 017's `data-model.md`, `QUAL` in 036's), says that the rule is a security rule on every surface, and leaves 008's canonical `{surface}-{category}-{NNN}` format, whose surfaces are `BE` and `FE`, unchanged. 008's `data-model.md` generic grammar (an uppercase prefix of two to five letters) accepts it. A third surface value, `XS`, was rejected: it would change 008's canonical table, drop "security" from the prefix, and collide with any later `-cross.md` file in another domain (owner, 2026-11-02, option a).
- **Which version removes the four deprecated IDs?** → The second minor release after the one that ships this spec: deprecated in `0.X.0`, removed in `0.(X+2).0`. Until removal, an adopter spec citing an old ID gets an Advisory finding naming the removal version (`framework/reference/analyze.md`). After it, the citation becomes a Blocking "unknown rule" finding. A two-release window lets an adopter that skips one release still see the Advisory first. The duplicate findings `/{project}:review` reports meanwhile end when 020 task 15 lands. How often adopters skip releases could not be measured, since their trees are out of reach. The next minor release was rejected because a skipped upgrade would meet the Blocking finding unwarned, and `1.0.0` because it leaves the window open-ended (owner, 2026-11-02, option b).
