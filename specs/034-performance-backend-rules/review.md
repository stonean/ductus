---
spec: 034-performance-backend-rules
last-run: 2026-10-20T00:34:28Z
reviewed-against: 91ec570c05bd21ae56864e1f25db599a05191dff
diff-base: 37c8219c8730fd23b5fbac7629c11fd2d6bc5b3b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 83
scope: 87
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 58
  routed: 10
  discarded: 14
  undispositioned: 0
decisions:
  - key: "security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: routed
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: routed
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: routed
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: routed
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: routed
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: discarded
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: discarded
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
---

# Review — 034-performance-backend-rules

## Summary

Five-pass review of 034 over the window `37c8219c..91ec570c` plus the working tree: the 2026-10-19 rule-file campaign, whose 87 scope entries are every file that campaign touched (034's plan names two of them). **Method.** Eight read-only reviewer passes ran in parallel over slices of the scope — one to three rule files each, the rule-owning specs, 008, 017 and the three draft rule specs, and the non-rule changes — and every observation they reported was re-verified against the cited lines before it was recorded; the fixed tree's whole diff was then re-read once. The 11 rule files and the owning specs' spec, plan, tasks and data model were read at their full current text; every other entry as its in-window diff with surrounding context. **Examined: 83 of the 87 scope entries.** The four generated command mirrors (`.claude/commands/ductus/analyze.md`, `.claude/commands/ductus/review.md`, `.pi/prompts/ductus-analyze.md`, `.pi/prompts/ductus-review.md`) were not read; `scripts/gen-claude-commands.sh --check` reported them in sync with their sources. **Findings: none.** No loaded rule's trigger fires on rule prose or on these spec artifacts, so every defect is an observation: 82 recorded, 58 fixed, 10 routed, 14 discarded. **Statement-level changes (what a rule requires):** `BE-TXN-003` (the serializable guard covers every transaction touching the invariant); `BE-COORD-004` (the producer must not reorder a scope on retry); `BE-METRIC-004` (per-instance summaries barred only on a multi-instance service); `BE-ASYNC-001` (the asynchronous acknowledgment is owed when the request's own outcome waits on the job — beside the owner's 2026-10-19 scope decision, which is unchanged); `FE-TIMING-001` (WCAG 2.2.1's real-time exception wording); `FE-A11YMEDIA-001`/`-002`/`-003` (FE-ARIA-002's naming methods; the media-alternative-for-text exception; MAYs for two-way calls and technique G203); `BE-PAGE-001` (a default and a maximum page size); `BE-AUTHN-001` (bcrypt's 72-byte handling); `BE-API-001` (an explicit content type, with a charset on HTML); `BE-DATA-004` (ECB never; a justified legacy non-AEAD deviation MAY remain); `BE-LOG-008` (BE-LOG-002's list as examples); `FE-LOAD-007` (what a short HTML `max-age` buys); `CFG-ENV-001` (empty-counts-as-unset scoped to the optional-no-default declaration); `CFG-ENV-011` (`TB`, `k` and `T` added). Every other fix is Rationale, Verification, header, or spec wording; `FE-TIMING-001`'s Rationale now says outright that a non-essential absolute session lifetime of 20 hours or less is a limit the rule governs, which its Statement already required. **External sources fetched for the claims acted on:** WCAG 2.2 (§1.2.1, §1.2.2, §2.1.2, §2.2.1), W3C Understanding 1.2.4 and 1.2.5, PostgreSQL *Transaction Isolation*, Kafka producer configs, the Yarn berry CHANGELOG, Spring `DataUnit` Javadoc, Kubernetes `Quantity`, WebKit `CachedResourceRequest.cpp` and `PlatformHave.h`, the OWASP HTTP Headers and Logging cheat sheets, and AWS's S3 presigned-URL guide. **Routed:** two new owner questions on `BE-API-013` versus `FE-STORAGE-003` (pre-signed URLs; where a single-use token may sit) to 008's owner scenario, plus observations already covered by open owner questions and four task extensions in 019, 025, 033 and 063. **Also in this run:** 045's `plan.md`, edited on a done spec without a reopen, was restored to its pre-sweep bytes (its recorded analysis digest matches again), and 050's task 34 item was ticked against the constitution. **034's acceptance criteria:** all nine verified against the tree — 18 rules, 8 MUST and 10 SHOULD; every AC6 citation present and resolving (`check-rule-ids`, 11 rule files examined, none missing); the manifest row present and `govern.md` identical to `ductus.md`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 034 plan's verification-style example named the retired validate command — `specs/034-performance-backend-rules/plan.md` — **fixed**
- other: 034 spec's severity posture listed the MUST cases without BE-QUERY-001 (N+1) and BE-POOL-003 (leaked connections) — `specs/034-performance-backend-rules/spec.md` — **fixed**
- other: 034 plan sent background-job idempotency to reliability-backend.md; its home is concurrency-backend.md BE-COORD-002 — `specs/034-performance-backend-rules/plan.md` — **fixed**
- other: BE-QUERY-002's warning condition omitted joins, which its Statement and trigger cover — `framework/rules/performance-backend.md` — **fixed**
- other: BE-CACHE-001's last sentence could be read as banning only caches lacking expiry, invalidation and eviction together — `framework/rules/performance-backend.md` — **fixed**
- other: BE-POOL-001's Verification required a pool-size constant even for a shared multiplexed client — `framework/rules/performance-backend.md` — **fixed**
- other: BE-PAYLOAD-003 had no warning condition for an unprotected secret in a compressed reflective response — `framework/rules/performance-backend.md` — **fixed**
- other: BE-ASYNC-001 read literally required a 202 for a request that offloads only a side effect, where BE-STATUS-001 requires 201 — `framework/rules/performance-backend.md` — **fixed**
- other: FE-TIMING-001's Verification demanded a warning lead time for every option and flagged turn-off and adjust designs — `framework/rules/accessibility-frontend.md` — **fixed**
- other: FE-TIMING-001's real-time exemption dropped WCAG 2.2.1's required-part and no-alternative conditions — `framework/rules/accessibility-frontend.md` — **fixed**
- other: FE-TIMING-001's Rationale left the BE-AUTHN-005 absolute lifetime's status under the rule vague — `framework/rules/accessibility-frontend.md` — **fixed**
- other: FE-A11YMEDIA-003's Verification accepted W3C technique G203 and scoped live captions to broadcast while its Statement did neither — `framework/rules/accessibility-frontend.md` — **fixed**
- other: FE-A11YMEDIA-001 restated FE-ARIA-002 with a narrower list of naming methods and flagged the same unnamed SVG control twice — `framework/rules/accessibility-frontend.md` — **fixed**
- other: FE-A11YMEDIA-002 lacked the media-alternative-for-text exception WCAG 1.2.1 and 1.2.2 carry — `framework/rules/accessibility-frontend.md` — **fixed**
- other: FE-ARIA-003's Verification asked to justify an override the Statement forbids — `framework/rules/accessibility-frontend.md` — **fixed**
- other: FE-CONTRAST-001's Verification flagged any hard-coded color at MUST tier, beyond its contrast Statement — `framework/rules/accessibility-frontend.md` — **fixed**
- other: FE-KBD-003's Rationale called Escape the only way a modal trap meets WCAG 2.1.2 — `framework/rules/accessibility-frontend.md` — **fixed**
- other: BE-TXN-003 accepted serializable isolation without requiring every transaction touching the invariant to run at it — `framework/rules/concurrency-backend.md` — **fixed**
- other: BE-RACE-003's Rationale used where for whereas, so the sentence did not parse — `framework/rules/concurrency-backend.md` — **fixed**
- other: BE-COORD-004's routing branch did not keep the producer from reordering a scope on retry — `framework/rules/concurrency-backend.md` — **fixed**
- other: BE-DRAIN-001 and BE-DRAIN-002 counted a pre-stop delay inside and outside the grace period — `framework/rules/reliability-backend.md` — **fixed**
- other: BE-METRIC-004's Statement banned per-instance summaries outright while its Verification exempted a single-instance service — `framework/rules/observability-backend.md` — **fixed**
- other: 037's spec claimed citations of BE-LOG-005 and a required scrape interval, and its Motivation promised SLOs and alerting — `specs/037-observability-backend-rules/spec.md` — **fixed**
- other: 036's AC7 and plan named a manifest slot and a count that no longer hold — `specs/036-quality-cross-rules/spec.md` — **fixed**
- other: 036's data model said a rule ID is never moved within its file — `specs/036-quality-cross-rules/data-model.md` — **fixed**
- other: FE-DEPS-004 credited Yarn 2 with immutable installs on CI; Yarn's changelog places that default in 3.0.0 — `framework/rules/security-frontend.md` — **fixed**
- other: FE-CSP-002's Verification required a style nonce or hash where the Statement allows style-src 'self' — `framework/rules/security-frontend.md` — **fixed**
- other: FE-STORAGE-002's Rationale said SameSite blocks CSRF, contradicting FE-CSRF-001 — `framework/rules/security-frontend.md` — **fixed**
- other: FE-DEPS-003's commitment list omitted the runtime integrity option its Statement allows — `framework/rules/security-frontend.md` — **fixed**
- other: FE-XSS-009's Rationale claimed any feature-checking fallback still sanitizes — `framework/rules/security-frontend.md` — **fixed**
- other: wording fixes in FE-DEPS-002, FE-CSP-006 and FE-CSRF-004 (a keyword in a Rationale, an overclaim, a garbled clause) — `framework/rules/security-frontend.md` — **fixed**
- other: CFG-ENV-001's empty-counts-as-unset clause read as covering optional variables with defaults — `framework/rules/configuration-cross.md` — **fixed**
- other: CFG-ENV-003's flag condition named any env var rather than required ones — `framework/rules/configuration-cross.md` — **fixed**
- other: CFG-ENV-011's ambiguous-unit list omitted TB, k and T — `framework/rules/configuration-cross.md` — **fixed**
- other: 017's data model gave a one-paragraph intro template and called BE and FE IDs security-only — `specs/017-derive-dont-ask/data-model.md` — **fixed**
- other: FE-IMAGE-001's Rationale said Accept negotiation serves Safari WebP; WebKit advertises image/avif except in Lockdown Mode — `framework/rules/performance-frontend.md` — **fixed**
- other: FE-LOAD-007 offered a short max-age as a way for each navigation to revalidate HTML — `framework/rules/performance-frontend.md` — **fixed**
- other: FE-LOAD-002's Verification narrowed its commitment to non-essential scripts; FE-VITALS section title lagged the header — `framework/rules/performance-frontend.md` — **fixed**
- other: BE-PAGE-001's Verification required default and maximum page sizes its Statement never set, which BE-INPUT-006 relies on — `framework/rules/api-backend.md` — **fixed**
- other: api-backend.md's header narrowed every trigger to interface changes and its RPC-reach list omitted BE-APIVER-001 — `framework/rules/api-backend.md` — **fixed**
- other: BE-AUTHN-001's Verification required bcrypt's 72-byte handling its Statement did not state — `framework/rules/security-backend.md` — **fixed**
- other: BE-API-001's table required a Content-Type with charset on every response, absent from its Statement — `framework/rules/security-backend.md` — **fixed**
- other: BE-API-014's Verification flagged an authenticated debug handler the Statement allows — `framework/rules/security-backend.md` — **fixed**
- other: BE-LOG-003's Verification never checked the monitoring its Statement requires and had no flag sentence — `framework/rules/security-backend.md` — **fixed**
- other: BE-LOG-008's parenthetical narrowed the BE-LOG-002 list it cites — `framework/rules/security-backend.md` — **fixed**
- other: BE-DATA-004 implied single-block ECB was allowed and left its legacy deviation out of the Statement — `framework/rules/security-backend.md` — **fixed**
- other: BE-INPUT-006 cited BE-PAGE-001 without naming its file — `framework/rules/security-backend.md` — **fixed**
- other: 008's plan abridged BE-AUTHN-001's Verification by dropping its bcrypt clause without an ellipsis — `specs/008-security-rules/plan.md` — **fixed**
- other: 008's data model miscounted its title mismatches and did not say its category table covers only the security files or state the generic ID grammar — `specs/008-security-rules/data-model.md` — **fixed**
- other: 008's owner questions did not cite the resolved decisions they would amend, and Q4's premise overstated BE-API-013's carve-out — `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md` — **fixed**
- other: 008's one-tier scenario and API-security summary lagged later changes (BE-DEPS-001 supersession, the three CSRF defenses, content type) — `specs/008-security-rules/spec.md` — **fixed**
- other: 017's owner question relied on CFG-ENV-001 for file keys and flags, and said flag where it meant a command-line flag — `specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md` — **fixed**
- other: 050's governance-precedence task item was unticked though the constitution carries the exception — `specs/050-constitution/tasks.md` — **fixed**
- other: 045's plan was edited on a done spec without a reopen, sweeping two path-line provenance citations — `specs/045-decision-state-drift-detection/plan.md` — **fixed**
- other: 060's corpus measurements carried no commit or date — `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — **fixed**
- other: 022's DNS-rebinding scenario overstated BE-INPUT-016's scope and offered resolve_to_addrs as sufficient — `specs/022-deterministic-runtime/scenarios/fetch-archive-dns-rebinding.md` — **fixed**
- other: 063's scenario cited CFG-ENV-005, which names environment variables only, for a config-file key — `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/an-invalid-threshold-fails-fast.md` — **fixed**
- other: 019's scenario said two readers state the slice idiom and left adopter fields in ductus-owned entries unaddressed — `specs/019-config-decisions/scenarios/an-unknown-config-key-is-named.md` — **fixed**
- security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md` — **routed** to `specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md`
- other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md` — **routed** to `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md` — **routed** to `specs/025-rule-opt-out/tasks.md`
- other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — **routed** to `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`
- other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md` — **routed** to `specs/033-rule-surface-setting/tasks.md`
- other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md` — **routed** to `specs/019-config-decisions/tasks.md`
- other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md` — **discarded**: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
- other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md` — **discarded**: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
- other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md` — **discarded**: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
- other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md` — **discarded**: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
- other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md` — **discarded**: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
- other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md` — **discarded**: both splits are recorded as intended in 068; low
- other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md` — **discarded**: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
- other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md` — **discarded**: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
- other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md` — **discarded**: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
- other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md` — **discarded**: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
- other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md` — **discarded**: the repeated sentences are accurate and not contradictory; each rule stands alone
- other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md` — **discarded**: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
- other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md` — **discarded**: a resolved question records the reasoning at the time; rewriting it records a fiction

## Skipped passes

*None.*

## Unexamined governance

*None.*
