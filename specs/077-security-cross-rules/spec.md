---
status: in-progress
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
   - The rule names such a value, and the exclusion or redaction commitment for its carrier, as `BE-LOG-002`'s in a log line, or as `BE-LOG-008`'s in telemetry or an error-reporting payload, where `security-backend.md` is also loaded.
   - A sensitive configuration value stays `configuration-cross.md` `CFG-ENV-006`'s.
   - Source: the OWASP Logging Cheat Sheet ("Data to exclude"; its event sources include "Client software" and a "JavaScript exception handler"), W3C Trace Context §6 and OpenTelemetry's Baggage security considerations, fetched when the rule is written.
2. **`SEC-DEPS-001`: build-input pinning (moved from `BE-DEPS-006`).**
   - It moves as 008 task 40's first item and task 42 decided: an immutable reference for every build input the lockfile does not cover.
   - A workflow onboarded to a GitHub Actions workflow dependency lockfile pins its actions. A reusable workflow called by `{owner}/{repo}/…@{ref}`, in another repository or this one, takes a full commit SHA, while one called by `./` or `$/` runs from the caller's commit, which pins it (owner, 2026-11-02, recorded on this spec's task 8), and a container image a workflow runs takes a digest.
   - Its yield to `BE-DEPS-002` takes "where `security-backend.md` is also loaded".
3. **`SEC-AUTHN-001`: constant-time comparison (moved from `BE-AUTHN-009`).** Its yield to `BE-API-010` (d) takes the same load condition.
4. **`SEC-AUTHN-002`: OAuth and OIDC client flows (moved from `BE-AUTHN-013`).**
   - The rule covers `state`, PKCE, `nonce` validation and exact `redirect_uri` matching.
   - It includes the native-app loopback port and the relaxation for a client that authenticates to the authorization server, in a pushed authorization request or a signed request object, which covers only a `redirect_uri` the client does not take from the request.
   - Its yield to `BE-API-009` takes the load condition.
5. **`SEC-AUTHN-003`: OAuth grant and response types (moved whole from `BE-AUTHN-020`).** The rule covers the forbidden grants and response types, the PKCE challenge method, and public-client refresh-token protection.

Each moved rule carries its predecessor's Statement and Verification as they stand when it moves, re-pointed but not reworded, except that `SEC-DEPS-001`'s reusable-workflow sentence and flag, and `BE-DEPS-006`'s kept Statement, draw the reusable-workflow line by how the call is written, as item 2 states, and `SEC-DEPS-001`'s yield keeps a package a build step's command runs by a mutable reference, such as `npx tool@latest`, as §Deprecation and re-pointing states. Its Sources are re-fetched when it lands.

### Deprecation and re-pointing

- `BE-DEPS-006`, `BE-AUTHN-009`, `BE-AUTHN-013` and `BE-AUTHN-020` are labelled `**DEPRECATED in {version}:**`, naming the release that ships this spec, per constitution §rules. Their removal target is the second minor release after it: deprecated in `0.X.0`, removed in `0.(X+2).0`. Each names its successor as the owner of every case it governed, with no load condition, since a `-cross.md` owner installs in every shape. A deprecated rule binds until it is removed.
- Every yield and Statement that names one of the four as current re-points to the successor:
  - `BE-API-009`, `BE-API-010` (d), `BE-API-011` (including its "Token comparison MUST be constant-time per `BE-AUTHN-009`") and `BE-API-013` in `security-backend.md`;
  - `FE-AUTHN-001` and `FE-STORAGE-003` in `security-frontend.md`.
  The frontend yields drop "where `security-backend.md` is also loaded" (008 `data-model.md` §Verification phrasing, item 6).
- `FE-DEPS-004` and `BE-DEPS-002` name a package a build step's command runs by a mutable reference, such as `npx tool@latest`, in a spec or plan, as `SEC-DEPS-001`'s, with no load condition, since every shape now loads `SEC-DEPS-001` beside whichever of them it loads and one fix, pinning the tool, clears both. `SEC-DEPS-001` keeps that case, so no rule yields it to another that yields it back.
- Every rule that names `BE-LOG-002` or `BE-LOG-008` as the owner of a non-configuration sensitive value in a log line or telemetry names the new rule where `security-backend.md` is not loaded. `CFG-ENV-006` is one of them.
- The sweep covers every live artifact that cites one of the four IDs as a current requirement. That includes `framework/templates/ci/adopter-generators.yml`, 008's `data-model.md`, 017's scenario `the-adopter-ci-template-can-run`, 017's and 022's `plan.md`, and the comments in `.github/workflows/framework-checks.yml`, `.github/workflows/runtime-release.yml` and `scripts/check-runtime-currency.sh`. A decision record, a changelog entry, a `review.md` key and an `analysis.md` record keep the old ID.

### Distribution

- The file gets its manifest row in `framework/bootstrap/ductus.md` and its row in `docs/rules.md`.
- `/ductus`'s brownfield security audit (`framework/bootstrap/ductus-procedure.md`, §Security Audit (brownfield)) also runs when `security-cross.md` is newly created, as the upgrade that ships this spec creates it in every adopter's tree, so a project with feature specs sees its rules' gaps then, before `/{project}:analyze` blocks on them. A case a rule's **Not flagged:** clause yields to a rule in an installed file is left to that rule, as `/{project}:analyze` leaves it, since the audit judges the yield's load condition against every rule file installed: the audit reports the case under the owner's ID when it newly created the owner's file too, and not at all when that file was already installed (owner, 2026-11-02, recorded on task 8). An owner outside the security rule files, such as `configuration-cross.md` `CFG-ENV-006`, is loaded and applied to that case alone (owner, 2026-11-03, recorded on task 11).
- `runtime/CHANGELOG.md`'s `## [Unreleased]` section names the four ID changes, their successors, the removal version, and the waivers they expire. A waiver is anchored on its rule ID and file (`framework/reference/review.md`, §Waivers), so one recorded under an old ID stops applying wherever the finding moves to the successor's ID, which then needs a waiver of its own. 008 task 32's follow-on item records this obligation (owner, 2026-11-02).

### Edge cases

- **An adopter spec that cites a deprecated ID.** `/{project}:analyze` reports it as Advisory ("references deprecated rule {ID}; targeted for removal in {version}") until the removal release, and as Blocking ("references unknown rule {ID}") after it. The fix is to re-point the citation to the successor the changelog names.
- **A project that pins `security-backend.md`** (`.ductus/config.toml` `pinned.files`). The pinned copy keeps the moved rules it has undeprecated and without their yields to the successors, while `security-cross.md` still installs beside them, so `/{project}:analyze` reports each such case under both IDs until the project re-syncs the file. A copy taken from `main` after 0.59.11 keeps whichever moved rules it had not yet deprecated, all four before `ad8a36f9`. A copy taken from 0.59.11 has `BE-AUTHN-009` and `BE-AUTHN-013`, but not `BE-DEPS-006` or `BE-AUTHN-020`, which were added after it. Nor has it `BE-LOG-008`, so `SEC-LOG-001`'s yield of a sensitive value in an error report, a span, a metric label or propagated trace context, and of the redaction commitment for those carriers, reaches no rule until the file is re-synced. A pinned `security-frontend.md` keeps `FE-DEPS-004`'s case of a package a build step's command runs by a mutable reference, such as `npx tool@latest`, beside `SEC-DEPS-001` the same way, and a pinned `security-backend.md` keeps `BE-DEPS-002`'s. A pinned `security-frontend.md`'s `FE-AUTHN-001` and `FE-STORAGE-003` keep their OAuth cases beside `SEC-AUTHN-002` and `SEC-AUTHN-003` in a project that loads no `security-backend.md`, or in any project when the copy predates `9b067399`, 0.59.11's among them, whose two rules yield neither case. The changelog entry says so; nothing in the shipped files can reach a pinned copy.
- **A `security-backend.md` or `security-frontend.md` left from a surface the project no longer configures.** The surface filter never prunes it, so it stops receiving updates (`framework/bootstrap/ductus.md`, "Rule-file surface filter") while `/{project}:analyze` still loads it. Its consequences are a pinned copy's, until the file is deleted or its surface configured again. The changelog entry says so.
- **A project that excludes `security-cross.md` from review.** `/{project}:analyze` still loads it (constitution §rules), so the successors still own their cases there.
- **`/{project}:review` before 020 task 15 lands.** Review checks code against each loaded rule's Statement and applies no Not-flagged routing, so until 020's scenario `review-reports-a-defect-under-its-owner` lands it reports one defect under two IDs:
  - a moved case under the deprecated ID and its successor, until the deprecated rule is removed, if that comes first;
  - where `security-backend.md` is loaded, a sensitive value in a log line, an error report or telemetry under `SEC-LOG-001` beside `BE-LOG-002` or `BE-LOG-008`;
  - where `security-frontend.md` is loaded, a client's `redirect_uri` under `SEC-AUTHN-002` beside `FE-AUTHN-001`, and a token an authorization response returns in a URL fragment under `SEC-AUTHN-003` beside `FE-STORAGE-003`;
  - a package a build step's command runs by a mutable reference, such as `npx tool@latest`, under `SEC-DEPS-001` beside `BE-DEPS-002` or `FE-DEPS-004`.

  A sensitive configuration value in a log line, an error report, a span or a metric label stays under both `SEC-LOG-001` and `BE-LOG-002` or `BE-LOG-008` after that scenario lands, since each yields it to `configuration-cross.md` `CFG-ENV-006`, which no review pass applies until 020's scenario `every-loaded-rule-file-has-a-pass` gives that file one.
- **A waiver under one of those IDs.** A waiver is anchored on its rule ID and file, so at `/{project}:review` each pair above needs a waiver under both IDs. One under the yielding ID — a deprecated ID, `SEC-LOG-001`, `FE-AUTHN-001`, `FE-STORAGE-003`, `BE-DEPS-002` or `FE-DEPS-004` — expires once 020's routing moves the finding to its owner, and one under a deprecated ID also when that ID is removed.

## Acceptance Criteria

- [ ] AC1: `framework/rules/security-cross.md` exists in the canonical rule schema. Its header declares the `SEC-{CATEGORY}-{NNN}` format and the categories `LOG`, `DEPS` and `AUTHN`, and it holds the five rules of §Rule set scope, each with a Statement, a Verification and a Source fetched when the rule was written.
- [ ] AC2: `specs/077-security-cross-rules/data-model.md` registers the file's ID prefix and categories, and `/ductus:analyze` loads the file: `check-rule-ids` accepts each of its IDs, and every citation of a successor resolves.
- [ ] AC3: A sensitive value that is not configuration, in a log line, an error-reporting payload or telemetry, is reported by the new log rule in a frontend-only and a cross-only project. In a backend-only and a full-stack project `/{project}:analyze` reports it once, under `BE-LOG-002` or `BE-LOG-008`, which the new rule names as owner where `security-backend.md` is loaded.
- [ ] AC4: The four moved rules carry their predecessors' Statements and Verifications as decided in 008's tasks 40 and 42, `SEC-DEPS-001`'s reusable-workflow sentence and flag as §Rule set scope item 2 states them and its yield as §Deprecation and re-pointing states it, and each successor's yield to a backend owner carries "where `security-backend.md` is also loaded".
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
- **Why split from 008-security-rules:** 008 delivers `security-backend.md` and `security-frontend.md` and owns the rule schema every rule file follows; this spec delivers `security-cross.md`, a further rule file `ductus` ships, which constitution §rules gives an introducing spec of its own, as 008's task 32 decided (owner, 2026-10-30). The two share the files the move touches: the deprecated rules and re-pointed yields in `security-backend.md` and `security-frontend.md`, 008's `data-model.md`, the manifest row and the constitution's rule-file lines.
