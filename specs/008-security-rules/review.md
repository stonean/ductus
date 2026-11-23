---
spec: 008-security-rules
last-run: 2026-11-23T17:01:12Z
reviewed-against: 1d4f0afaff82e28302154c5452fa99a3898661fa
diff-base: e92f9adb4f6d1077ea5a83dc1debda5c5e2b189d
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 14
scope: 15
unread:
  - runtime/CHANGELOG.md
excluded: 5
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: b3c50297287cbc973c443e03f8f40f62dd8921d274b613aabf22bc87cdbc7acc
  scenarios/a-cache-partition-is-purged-when-the-session-ends.md: 482b7e2099f5bae522f49820d1830f6cc99d5afeb41ff2ed2cfff756910ef535
  scenarios/a-custom-credential-header-does-not-follow-a-redirect.md: 2c25f447f5731eb05192956205e08b600cc355376fd35492fc47573d2ebe0b2f
  scenarios/a-statement-carries-one-obligation-keyword.md: 25130f7eb66ca03216927ef7293bbb0a1e55d9e26697be629c143b176d687e11
  scenarios/a-throwaway-credential-owes-no-revocation-path.md: 255adbbe25663fa352e74fa91395fa8c8971c882ffddf52c0afe7c2703825a9d
  scenarios/an-absent-origin-on-a-cookie-connection-is-flagged.md: c8eaad9079d0165017055265c821745084c7c448be8e742132702422d5283499
  scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md: db3d868a816c78dd41f62dd5bb08b44da41e0d694ea28097e8de53dcba753011
  scenarios/be-deps-005-flags-what-its-statement-requires.md: f8ac2b621a3cdfb580d1499f4739ad6ef9412f4fe4d74a8106a5ce0941dc7493
  scenarios/independent-review-findings-await-the-owner.md: 356a0ea3225449ddeff7e75966ade95c35af36f83e3187baf9ca05db8eed5c30
  scenarios/rules-are-as-short-as-their-required-content.md: f6afe0da29359d2c2ae5ef8f81930ddf7b82cb8fc02d53c7dcaaf237d1455705
  scenarios/security-rules-flag-what-their-sources-support.md: 06bd4aeb2c2ce217df0452da7d6310ed4247b6572296fbfcdbeddd99c427f6f8
  scenarios/single-host-appliance-rule-fit.md: 8f352bb3397cb2ccd81c841a72c74a19f6f125f8171485948969d337b1441a14
  scenarios/the-tag-check-reads-every-entry-above-the-newest-version.md: 9996dcc1c45968e29903cd2d1c85f9e5b8169d5a4581ef3a412543112ab65d0a
  scenarios/x-frame-options-carries-one-strength.md: 3f003bce779297c8d2af9d604d36f9d197534e28cfe7e835a2c4d5463ce5ee06
criteria-judged: 28
criteria-met: 28
criteria-verdicts:
  - label: AC1
    digest: b4d09fa7c145f62be251e9ca3f8c2b301cd6538d46af68e6494294d48be039c3
  - label: AC2
    digest: cabce85d72d75007ddb7adb8220beedadf423c7bf431ff55cfe36612eca0d508
  - label: AC3
    digest: 686cd8d2edc031ef0978942ecd2a0b6fde996eb94f9db4ab887e1180941c6bf5
  - label: AC4
    digest: a8efad0b674277e744889cb038379de81de0bf5c7898cdf97a0f5c281a52674c
  - label: AC5
    digest: c9965486b58795d539d5250687efec00b3b01e896afe169125078132d8caafda
  - label: AC6
    digest: 532dc30c596fc3f7e87e03b04edd580c9248608f390341fe95e76d9f08760fc3
  - label: AC7
    digest: e420a2d39fb5082d1ec0488ce237e92aa8d5c28b64ee1f88127f66ba3a5190dc
  - label: AC8
    digest: 3b87d77d1ef6ded42149adcdc51da4088e4e62a5bcd1a83a2a9a7734c626c7db
  - label: AC9
    digest: 81a8549a522954c99351d2b94ec4eba31cf946f704af2e444f42ea6b4f4e5916
  - label: AC10
    digest: 119afdaf7e3aa75e18ca129f25e7e4cf28030ecbf41fb36c574c6c99ccd0452c
  - label: AC11
    digest: 6714297c571183bf5078f5c695991864643b5d5d2baec8647f654ff93fea08e6
  - label: AC12
    digest: bef424f509ad791b6b03517943d26e3e1aa9ab11fe0be5fde715016bd33a9c8d
  - label: AC13
    digest: 8d13f6ae853e453b6c9df2a60105f192f41450de19526a2e0ab80b620e8c6297
  - label: AC14
    digest: a37593eb5dbfc8f96cf0f0143680788e7bcaf8837318e70e7895d1e1040c42d1
  - label: AC15
    digest: f208034be72f0cba47d97dfe484709fa0e344da46dc180ae96b0782b5a8e0ecd
  - label: AC16
    digest: 7b344e1695e811aed5fe45c572a58c3d0f465f90d9358999c069263141faac8a
  - label: AC17
    digest: 987a7132b8338980e7081af9f3270ac5636998074a73324a60cd6044c163fe98
  - label: AC18
    digest: 952ed7971b94dfa8ff24090549a9a90a0eb952d8326c7cb5ccac36788ea07444
  - label: AC19
    digest: 3e2677aaffa390a1dbb5136102f4f08c6531ee4d753b8e3e2c3a49d591708e42
  - label: AC20
    digest: 308e8ed9d279071db0925ab0062112dbbfe755efa088b9f36395c7078892324f
  - label: AC21
    digest: 15af43f4da9954d57b97229ac411ffda847ff83f0c5124a07dbfa3880d6cdf7d
  - label: AC22
    digest: 3e3c0ef1410d0af2ee607eba0433a540c09a407e1c79e3386dbadea2265d67f2
  - label: AC23
    digest: e664d67684d43efd8ac8a948e27e1d203841c8dc40d90f9e592db1aa29afd25d
  - label: AC24
    digest: bb6c0595e9c95a31fe1c7e44a9414129459b2e0b044aefe2947e3d5feef2ee79
  - label: AC25
    digest: 1d1c12ae3d4430fbceddb576bc5b9957340b936ab49ba27138725be55edbac70
  - label: AC26
    digest: 3ea5f2eafd6ddb31b6a1577f61132ec530cfae35f3e6f18c4f99d891e621911a
  - label: AC27
    digest: 63081acd6e4ce73494ec93e84123f7aadc60fa12a9cb35a2480068d6beaab9ab
  - label: AC28
    digest: ac30ba8ac19ef5bae2aaaf7ee5fa79fff615e28e02c01ab8befbb5309014421f
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 27
  undispositioned: 0
decisions:
  - key: "security: BE-API-015's handshake-URL MUST NOT also bars a CSRF token there, which the OWASP WebSocket Security Cheat Sheet suggests sending in the handshake — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the owner's 2026-10-20 answer keeps every token out of the handshake URL, following the same cheat sheet's advice against tokens in URL query strings; (a)'s Origin allowlist is the handshake's cross-site defense, and a CSRF token can still be checked in the first message
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: the pre-signed-URL MAY covers object storage only, so another signed-URL scheme, a CDN's or an application's own HMAC-signed link, falls under BE-API-013 and FE-STORAGE-003 — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the owner's 2026-10-20 answer scoped the MAY to a pre-signed URL for one object and one method; whether other signed-URL schemes fit that scope is a new decision
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-007's Verification asks for legitimate-recovery paths during lockout, which its Statement does not name — `framework/rules/security-backend.md`"
    outcome: waived
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-001's Statement requires frame protection on all HTTP responses while its Verification table sets frame-ancestors on HTML responses only — `framework/rules/security-backend.md`"
    outcome: waived
    reason: predates this window; OWASP's HTTP Headers Cheat Sheet, which BE-API-001 cites, says X-Frame-Options gives no security on a redirect or a JSON API response, while its REST Security Cheat Sheet recommends frame-ancestors 'none' on API responses, so aligning the two decides what the rule requires
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-018's newer-token invalidation would cancel a code issued by one method when a code by another is issued in the same recovery — `framework/rules/security-backend.md`"
    outcome: waived
    reason: a recovery that verifies the first code before issuing the second complies, since the first is already used; NIST SP 800-63B-4 sets no invalidation across codes and nothing in BE-AUTHN-018 forbids that order
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005, widened to every source, does not say whether a value read from several sources is range-checked as resolved or at each source — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: the value validated at startup is the one CFG-ENV-007's resolution yields, the reading CFG-ENV-003 states for presence, and no spec here reads a ranged value from two sources
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "reliability: dropping a timed-out client joins its runtime thread, so a slow connect-time lookup can hold a fetch past FETCH_TIMEOUT — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: the GaiResolver it replaced behaves the same way, and in production the lookup is getaddrinfo, which the operating system resolver's own timeouts bound
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "reliability: the initial and redirect-screen lookups run outside the client's timeout — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: unchanged by this fix and bounded the same way, by the operating system resolver's timeouts; FETCH_DEADLINE bounds the download itself
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "security: a redirect hop is looked up twice rather than connected to the address its screen resolved — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: 022's scenario allows an equivalent connect-time re-check, both lookups are screened, and the owner prescribed this resolver and its staged-rebind test
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: two proxy tests each define the same four-line internal-address lookup — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: test fixtures local to each test; sharing one would couple the tests for no gain
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: 025's ticked task 2 lists a malformed warning among the step-5 outcomes that task 10 replaced — `specs/025-rule-opt-out/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: deferred
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: waived
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs`"
    outcome: waived
    reason: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs`"
    outcome: waived
    reason: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
    decided-at: 2026-10-20T15:42:12Z
    decided-by: andy@stone.dev
  - key: "bug: FE-CSRF-003's Statement bars every state-changing GET while its Not-flagged clause sets aside an OAuth client's GET redirection endpoint, so /review, which applies Statements and no routing, reports a MUST on a compliant callback /analyze sets aside — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: "Not mechanical: the clause names its owner, security-cross.md SEC-AUTHN-002 (a), and how /review applies a Not-flagged clause that names an owner is what 008 task 43's item gave to 020's scenario review-reports-a-defect-under-its-owner; this is one more case of it."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: BE-AUTHN-017 and security-frontend.md FE-STORAGE-004 both warn on a session cookie's unjustified Domain attribute in a full-stack project, and neither names the other — `framework/rules/security-backend.md`"
    outcome: waived
    reason: "Not mechanical: both warn alike at SHOULD, so the double report gives no wrong result, and it predates this window; choosing an owner is an overlap decision of task 30's kind."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: BE-AUTHN-018 (d)'s 'asking the user instead does not comply' is stricter than its OWASP source, a reason only its pre-cut Rationale recorded — `framework/rules/security-backend.md`"
    outcome: waived
    reason: The rule format states no reason for a Statement (scenario rules-are-as-short-as-their-required-content, owner, 2026-10-28), and the cut kept (d)'s obligation; recording why a rule is stricter than its source is a new record of the kind 008 task 43's first item made by an owner's call.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: external claims in the authentication and credential rules have no fetched copy to re-check them against: Go net/http and crypto/tls, Python requests, RFC 7143, the Amazon EBS encryption guide, and OWASP's Authentication, Forgot Password and CSRF Prevention cheat sheets — `framework/rules/security-backend.md`"
    outcome: waived
    reason: Each was fetched and cited when its rule was written, or kept verbatim by the cut, which rewords no claim from recall (scenario rules-are-as-short-as-their-required-content); re-fetching every source is outside the passes, and the Summary names these as not re-checked.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: BE-ERR-002's Verification reaches a request-response protocol a process serves over its stdio, while AGENTS.md's gotcha says BE-ERR-002's correlation ID never fires against the runtime and an MCP tool error carries no code — `AGENTS.md`"
    outcome: waived
    reason: "Not mechanical, and 077's review waived the same tension by the rule: AGENTS.md's gotcha is this repository's reading of the rule against its own local runtime, naming the parts that assume a deployed service, while the rule states what an adopter's API owes, and the two meet at the gotcha's 'Name the boundary first'."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: CFG-ENV-006 and FE-STORAGE-003 yield a sensitive configuration value the build injects into a client artifact to CFG-ENV-010, which bars only secrets, so a build-injected PII value has no owner — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: "Not mechanical: CFG-ENV-010 holds every injected value public and neither Statement names a PII value in a build, so barring one is a new obligation, decided with its sources as tasks 43 and 44 decided theirs."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: BE-ERR-003's Statement requires a structured 5xx while its Verification reaches an API that is not an HTTP API, which has no 5xx — `framework/rules/security-backend.md`"
    outcome: waived
    reason: Its Verification holds a non-HTTP API to the handler's existence and to no unhandled exception reaching the client, as its last Not-flagged sentence says, and leaves the response's shape to BE-ERR-002; the 5xx names the HTTP case, and restating the Statement is a rule decision.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: the errors.md template's Error Codes example does not say the codes are declared in the published schema, which BE-ERRENV-002's commitment asks of specs/errors.md's envelope — `framework/templates/project/errors.md`"
    outcome: waived
    reason: The template is an example a project replaces, and no BE-ERRENV-002 flag trips on it; whether its commitment reaches an unedited template is a new question of the kind task 43 item 7 answered for BE-ERR-002.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: BE-API-013 and FE-STORAGE-003 more than doubled in the window, their flag sentences restating the Statements' MAYs where a pointer would carry them — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: "Not mechanical: each flag sentence names the carve-outs a reviewer acts on at the flag, compliant cases the cut keeps, and replacing them with a pointer rewrites two rules five re-read rounds settled."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: external claims the window added have no fetched copy to re-check them against: OpenID Connect RP-Initiated Logout 1.0 §2, JSON-RPC 2.0 §5.1, gRPC's error model, Stripe's API keys page, OWASP Secrets Management §3.6 and the Sealed Secrets README — `framework/rules/security-backend.md`"
    outcome: waived
    reason: Each was fetched and cited when tasks 43 and 44 wrote it, as their records state; re-fetching every source is outside the passes, and the Summary names these as not re-checked.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "security: BE-API-001 does not cover Permissions-Policy or Cross-Origin-Opener-Policy, leaving them to FE-CSP-007 and FE-CSP-009, so a backend-only project serving HTML is asked for neither — `framework/rules/security-backend.md`"
    outcome: waived
    reason: "Not mechanical: a single-surface coverage gap of the kind task 40 decided rule by rule, and adding the headers to a backend rule is a new obligation."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: BE-API-006 requires validating Content-Type on incoming requests, while its OWASP REST source requires it only of a request with a body — `framework/rules/security-backend.md`"
    outcome: waived
    reason: Predates the window, and narrowing the Statement to requests with a body is a rule decision.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "security: BE-INPUT-016's MAY exempts only the lookup of an operator proxy's host by name, so a connection to a proxy given as an internal IP literal is exempted by no sentence of the Statement — `framework/rules/security-backend.md`"
    outcome: waived
    reason: "Low confidence, and the MAY makes the operator's configured proxy the fetch's trust boundary: an IP-literal proxy is never looked up, so there is no lookup to exempt, and the Verification sets aside a fetch through a forward proxy."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: 022's fetch-archive-dns-rebinding Context says in the present tense that reqwest re-resolves the host at connect time, the design before the fix, and its proxy edge case counts a fetch URL naming the proxy's host in the rebinding residual, though that host is pinned — `specs/022-deterministic-runtime/scenarios/fetch-archive-dns-rebinding.md`"
    outcome: waived
    reason: "Not mechanical: the Context records the design the scenario was written against, and correcting the residual takes a reading of reqwest's override order that is 022's; 022 owes a review that reads its scenarios whole."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: 048's fetch-archive-reads-its-proxy-once and docs/runtime.md say a direct connection reaches only a screened address, leaving out the proxy host's unscreened connect-time lookup when a fetch URL names that host — `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md`"
    outcome: waived
    reason: "Not mechanical: the case is the proxy-host lookup BE-INPUT-016's MAY exempts, which 008's and 022's scenarios name, and wording 048's account of it is 048's; 048 owes a review that reads its scenario whole."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: four claims are pending, undecided, on 048's tasks.md task 30: the certificate variables read again at the first fetch, the scenario's socks5 history, the opencode clause in AC22, and the symlink flag wording — `specs/048-govern-acquired-runtime/tasks.md`"
    outcome: waived
    reason: Already pending on 048's task 30, whose closing decides them.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: external claims in the dependency, input and header rules have no fetched copy to re-check them against: RFC 9110 §9.3.6, OWASP SSRF Prevention, RFC 9111, Cloudflare Cache Rules, W3C CSP and Referrer Policy, OWASP Forgot Password and HTTP Headers, PostgreSQL, Redis ACL, PgBouncer, Cloud SQL Auth Proxy, OWASP Deserialization, CSP3 on default-src, the Topics API explainer, Go net/http, Python requests and curl --proto — `framework/rules/security-backend.md`"
    outcome: waived
    reason: Each was fetched and cited when its rule was written, or kept verbatim by the cut, which rewords no claim from recall; re-fetching every source is outside the passes, and the Summary names these as not re-checked.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: BE-TXN-004's Verification restates its Statement's rollback-before-retry clause as a fact with a reason — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: "Not a restatement to cut: the sentence is the Verification's only check of the clause the cut moved into the Statement."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: FE-A11YFORM-002's Not-flagged sentence nests its exception inside its carve-out and is hard to parse — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: Accurate as written; rewording it for readability is a judgment edit, and 069, a draft, owns the file's next pass.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: x-frame-options-carries-one-strength's Non-HTML edge case cites OWASP's HTTP Headers sheet on JSON API responses, while the fetched OWASP REST Security sheet recommends X-Frame-Options on browser-consumed API responses — `specs/008-security-rules/scenarios/x-frame-options-carries-one-strength.md`"
    outcome: waived
    reason: "Not a misquote: BE-API-012 cites the HTTP Headers and Clickjacking sheets, which the scenario's Resolved Question read; the REST sheet's compatibility advice is a scope question for BE-API-012."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "convention: 008's data-model introduction template leaves out the tier-posture sentence five rule-file headers carry — `specs/008-security-rules/data-model.md`"
    outcome: waived
    reason: Kept on purpose, as 70bb4af5's message records, because 068's and 070's open questions cite those headers' tier sentences.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "convention: BE-API-001's header table follows its Verification paragraph, so the ductus exec walker's assessSpecQuality request, which carries the Verification paragraph, sends BE-API-001 without the header values — `framework/rules/security-backend.md`"
    outcome: waived
    reason: 060's scenario an-assessment-sees-the-rule-it-asks-about requires a request's Verification to run to the rule's next field, BE-API-001's header table included, so 060's pending work owns it; /analyze's host path reads each rule from its file whole.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: external claims in accessibility-frontend.md, performance-frontend.md and concurrency-backend.md have no fetched copy to re-check them against: WCAG 2.2 and its Understanding documents, ARIA in HTML, the APG, web.dev, MDN, database vendors' locking documentation, Kafka, SQS, RabbitMQ, Kubernetes and Kleppmann — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: Each was fetched and cited when its rule was written, or kept verbatim by the cut, which rewords no claim from recall; re-fetching every source is outside the passes, and the Summary names these as not re-checked.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "bug: 038's lock-the-parent scenario requires a MUST-tier lock-ordering rule, against 038's Resolved Question that lock ordering stays SHOULD, and says no rule covers the case BE-LOCK-002 asks for — `specs/038-concurrency-backend-rules/scenarios/lock-the-parent-before-rewriting-its-children.md`"
    outcome: waived
    reason: "038's own requirement: 038, next in the closing order, owes a review that reads its scenario whole and decides its tier against its Resolved Questions."
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: 013's the-precheck-ignores-what-the-same-run-wrote Context says in the present tense that the precheck's exit line leaves section-versus-run open and pre-0.53 adopters meet its refusal, both of which 013 task 25 removed — `specs/013-text-first-artifacts/scenarios/the-precheck-ignores-what-the-same-run-wrote.md`"
    outcome: waived
    reason: 013 is done, and correcting its scenario reopens it for a review and an analysis to change a Context paragraph whose Behavior already resolves the case.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: 008's first Resolved Question expresses an acknowledge tier as a SHOULD whose rationale says to record the deviation, a mechanism task 19 removed with every Rationale paragraph — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: A resolved question records the decision as taken, and 073, a draft, owns how a decided rule departure is now recorded, its open question 3 citing this one.
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: 000 task 22's pending Done-when still names the markdown-only reference, a label 079 retired — `specs/000-slash-commands/tasks.md`"
    outcome: waived
    reason: 10302897's message records leaving a 'markdown-only reference' label as read until its line is next edited (079 AC9's Resolved Questions).
    decided-at: 2026-11-08T14:04:07Z
    decided-by: rule
  - key: "other: task 45's done-when asks for bytes measured per rule and per file, and no artifact records them; `specs/008-security-rules/tasks.md` — `specs/008-security-rules/tasks.md`"
    outcome: waived
    reason: the figures are measured and stated, with their method, in this review's Summary; tasks.md is ephemeral and the measurement is a method of work, not a requirement
    decided-at: 2026-11-20T04:28:13Z
    decided-by: rule
  - key: "other: `BE-API-013`'s Statement says the server MUST NOT send a credential or PII to a client in a URL, naming no list where its first sentence and Verification list passwords, session identifiers, tokens and API keys; low confidence; `framework/rules/security-backend.md` — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the scenario `security-rules-flag-what-their-sources-support` defines credentials as that list and the owner's decision names no change to it; the edit would re-judge the rule file for every spec's next analysis for a reading the Verification already bounds
    decided-at: 2026-11-20T04:28:13Z
    decided-by: rule
  - key: "other: the lifecycle sentence in `specs/008-security-rules/data-model.md` that deprecated rules are removed in the release their label names has no early-removal path, and two labelled rules were removed before their target — `specs/008-security-rules/data-model.md`"
    outcome: waived
    reason: Resolved Question 16 records the owner's one-off for two rules no tag carried; the sentence states the rule for a released rule, and editing a canonical data model reopens 008 and the specs restating it
    decided-at: 2026-11-20T04:28:13Z
    decided-by: rule
  - key: "other: `AC21`'s duplicate-ID blocking message is produced only by the host reading `discover-rule-files` output and is absent under `ductus exec`, and only `scripts/lint-rule-ids.sh` enforces uniqueness in CI; `specs/008-security-rules/spec.md` — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: an observation about the analyze walker's own machinery made while reviewing framework work; `framework/reference/analyze.md` states the exec reduction
    decided-at: 2026-11-20T04:28:13Z
    decided-by: rule
  - key: "other: `AC22` to `AC27` (the brownfield audit) are procedure text in `framework/bootstrap/ductus-procedure.md` only and no test exercises the audit's output; `specs/008-security-rules/spec.md` — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: the audit is a deliberate procedural step with no host-level enforcement of the deterministic path (AGENTS.md Boundaries; spec 029) and 058 owns its contract
    decided-at: 2026-11-20T04:28:13Z
    decided-by: rule
  - key: "other: BE-API-015's new Verification flag fires only on a spec that says the endpoint accepts a handshake with no Origin on a cookie-authenticated connection; its MUST-name list (a) asks only for the Origin allowlist, so a cookie-authenticated spec that names an allowlist and says nothing about an absent Origin passes analyze though Statement (a) refuses that handshake. This is inside the owner's 2026-11-23 decision (flag acceptance, Statement unchanged); recorded as a residual gap, not a defect in the change. — `framework/rules/security-backend.md`"
    outcome: waived
    reason: "within the owner's decision of 2026-11-23 (008's Resolved Question 24): the new flag catches a spec that accepts a handshake with no Origin on a cookie-authenticated connection, and requiring every such spec to state its absent-Origin handling is a further commitment the owner did not decide"
    decided-at: 2026-11-23T17:01:12Z
    decided-by: rule
---

# Review — 008-security-rules

## Summary

Review of 008's reopening for the owner's decisions of 2026-11-23 (tasks 46 to 50) over the window e92f9adb..1d4f0afa (base source: the reopen; no override). Three readers ran the passes: A the security pass, reading security-backend.md (118,308 bytes), security-frontend.md (50,341) and security-cross.md (24,283) whole; B the reuse, quality, efficiency and simplicity passes, reading quality-cross.md (9,060) and configuration-cross.md (29,932) whole and driving scripts/tests/test-check-release-tag.sh; C the criteria step over all 28 criteria, each judged met against the tree, with 13 named runtime tests and scripts/lint-rule-ids.sh run (AC9's fetch of the latest release itself was not driven). Of the 15 scope entries, 14 were read whole by the security pass; runtime/CHANGELOG.md (634,370 bytes) was read in part, its ## [Unreleased] section and the window's diff, and is listed unread. The quality pass read the two security rule files, spec.md, plan.md and tasks.md through the window's diff and the changed rules' sections. No MUST or SHOULD violation and no broken contract. Four observations: three fixed in the run by the rule (scripts/check-release-tag.sh -h printed a fixed line range the window's longer header outgrew, and now prints the header comment whole; BE-DEPS-005's Verification now flags a missing (b) only where the package manager supports it, as its Statement binds (b); the changelog's BE-DEPS-005 entry now says its Statement's (c) changed), and one waived by the rule (BE-API-015's MUST-name list does not ask a cookie-authenticated spec to state its absent-Origin handling, which Resolved Question 24 left as decided). Of the stored decisions, 26 whose issues stand in text the window did not change were matched again, 1 expired (the FE-DEPS hand-off note task 47 resolved), and 37 were retained, their files outside the scope. Measured with wc -c and each rule's section, the bytes after its ### heading line up to the next heading, after the fixes: security-backend.md 117,631 to 118,346 (BE-DATA-012 1,540 to 1,958, BE-DEPS-005 1,890 to 2,037, BE-API-015 2,995 to 3,145); security-frontend.md 50,152 to 50,341 (FE-STORAGE-006 1,059 to 1,248). Not re-checked: the external sources the rule files cite, of which no fetched copy was at hand, as the matched decisions record; no rule's new text makes an external claim.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

28 of 28 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: framework/rules/security-backend.md exists with 90 numbered '### BE-' blocks (script count today); lint-rule-ids.sh exit 0
- AC2 — met: framework/rules/security-frontend.md exists with 45 numbered '### FE-' blocks (script count today); lint-rule-ids.sh exit 0
- AC3 — met: Script over 135 blocks in both files: each has a '> ' Statement and '**Verification:**', zero '**Rationale', zero duplicate IDs; rule_sections test every_shipped_rule_is_assessable passes
- AC4 — met: Every heading matches (BE|FE)-CAT-NNN, each category starts at 001; scripts/lint-rule-ids.sh examined 12 files, 306 headings, exit 0
- AC5 — met: Every Statement carries an RFC 2119 keyword; rule_sections tests no_shipped_statement_mixes_tiers and a_statement_s_keyword_gives_its_tier pass
- AC6 — met: framework/bootstrap/ductus.md '`ductus`-owned shared files (strategy: update)' table lists security-backend.md and security-frontend.md
- AC7 — met: ductus.md manifest row security-backend.md -> specs/rules/security-backend.md; apply_manifest.rs update_creates_when_dest_absent passes
- AC8 — met: ductus.md manifest row security-frontend.md -> specs/rules/security-frontend.md; same apply_manifest update path and test
- AC9 — met: apply_manifest.rs update_updates_when_dest_differs passes (stale dest rewritten); latest-version fetch itself not driven
- AC10 — met: apply_manifest.rs pinned_short_circuits_before_any_io_regardless_of_strategy passes; ductus.md Procedural fidelity names [pinned] files as the opt-out
- AC11 — met: framework/reference/analyze.md Rules section loads discovered files; discover_rule_files.rs all_selects_every_file_past_surfaces_and_the_disabled_filter passes
- AC12 — met: framework/commands/analyze.md step 11 puts MUST-tier findings in Blocking; analyze_tally.rs a_single_tier_rule_counts_in_its_own_tier_and_a_mixed_rule_keeps_the_label passes
- AC13 — met: framework/commands/analyze.md step 12 puts SHOULD-tier findings in Advisory; same analyze_tally test passes
- AC14 — met: analyze.md steps 11/12 findings carry rule-id and step 22 renders each finding; rendering is host-side with no runtime test
- AC15 — met: analyze.md steps 11/12 assess only rules whose Verification trigger fires, 'passed: true' when it does not; host-judged, no runtime test of the inert path
- AC16 — met: reference/analyze.md emits 'No rule files found, skipping rule checks'; discover_rule_files.rs no_rule_directory_yields_empty_selection passes
- AC17 — met: analyze loads every present file, no finding for absent; discover_rule_files.rs surfaces_backend_keeps_backend_and_cross_drops_frontend passes
- AC18 — met: reference/analyze.md 'Malformed rule file {path} at {location}: {reason}' blocking; discover_rule_files.rs all_lists_each_files_headings_that_open_no_rule passes
- AC19 — met: reference/analyze.md 'references unknown rule {ID}' Blocking; check_rule_ids.rs missing_rule_is_flagged passes
- AC20 — met: reference/analyze.md 'references deprecated rule' Advisory; check_rule_ids.rs deprecated_rule_is_flagged passes
- AC21 — met: reference/analyze.md 'Duplicate rule ID {ID} in {file}' Blocking; scripts/lint-rule-ids.sh exit 0 is the CI proof; no runtime test
- AC22 — met: ductus-procedure.md 'Security Audit (brownfield)' triggers on newly created rule file plus an existing spec dir, reports and writes nothing (:235), matching the 058 note
- AC23 — met: ductus-procedure.md 'Greenfield adoption' bullet: no feature spec dir, audit skipped
- AC24 — met: ductus-procedure.md trigger 1 and 'Routine re-run' bullet: updated/unchanged file does not trigger the audit
- AC25 — met: ductus-procedure.md Reporting format '{Rule ID}: {path} does not address — {summary}' without checkbox, as the 058 note states
- AC26 — met: ductus-procedure.md: audit writes no file (inbox write is out-of-scope at :302), one finding per spec per rule (Per-rule check item 5)
- AC27 — met: ductus-procedure.md summary block 'Security audit: {N} gap(s) in {M} existing spec(s) — nothing was written.', omitted when none
- AC28 — met: Superseded by 050-constitution AC25: framework/constitution.md has no 'Secure' principle (grep empty) and §rules names specs/rules/{rule-set}.md

## Observations

- security: BE-API-015's handshake-URL MUST NOT also bars a CSRF token there, which the OWASP WebSocket Security Cheat Sheet suggests sending in the handshake — `framework/rules/security-backend.md` — **waived**: the owner's 2026-10-20 answer keeps every token out of the handshake URL, following the same cheat sheet's advice against tokens in URL query strings; (a)'s Origin allowlist is the handshake's cross-site defense, and a CSRF token can still be checked in the first message
- security: BE-AUTHN-018's newer-token invalidation would cancel a code issued by one method when a code by another is issued in the same recovery — `framework/rules/security-backend.md` — **waived**: a recovery that verifies the first code before issuing the second complies, since the first is already used; NIST SP 800-63B-4 sets no invalidation across codes and nothing in BE-AUTHN-018 forbids that order
- security: BE-API-001 does not cover Permissions-Policy or Cross-Origin-Opener-Policy, leaving them to FE-CSP-007 and FE-CSP-009, so a backend-only project serving HTML is asked for neither — `framework/rules/security-backend.md` — **waived**: Not mechanical: a single-surface coverage gap of the kind task 40 decided rule by rule, and adding the headers to a backend rule is a new obligation. (decided by the rule)
- security: BE-INPUT-016's MAY exempts only the lookup of an operator proxy's host by name, so a connection to a proxy given as an internal IP literal is exempted by no sentence of the Statement — `framework/rules/security-backend.md` — **waived**: Low confidence, and the MAY makes the operator's configured proxy the fetch's trust boundary: an IP-literal proxy is never looked up, so there is no lookup to exempt, and the Verification sets aside a fetch through a forward proxy. (decided by the rule)
- bug: scripts/check-release-tag.sh -h prints `sed -n '2,27p'` of its own header, and the window's three added header lines moved the `Consumed by:` and `Tested by:` lines past line 27, so --help no longer shows them (it did at e92f9adb); the range should end at the header's last comment line — `scripts/check-release-tag.sh` — **fixed** (decided by the rule)
- convention: the BE-DEPS-005 entry under `## [Unreleased]` in runtime/CHANGELOG.md reads as garbled prose (`in every project, and (c) requires the plan to name where ...`; `omits either or names no such place` after naming three items) and never says the Statement's (c) itself gained the `the plan MUST name where a new dependency's name is checked` clause, though the BE-API-015 entry states the Statement is unchanged by contrast; an adopter reading it cannot tell the Statement moved — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- convention: BE-DEPS-005's Verification flags a spec that omits (b) with no condition, but its Statement binds (b) only 'where the package manager supports it', and the scenario be-deps-005-flags-what-its-statement-requires (edge case) and the changelog entry both say (b) still binds only where the package manager supports it; FE-DEPS-009 has no such condition in its Statement, so the 'same as FE-DEPS-009' wording does not carry it over. A spec for a package manager that commits no hashes (a Bundler or Composer project, say) can be read as flagged for omitting (b). Suggested edit: 'omit (b) where the package manager supports it'. — `framework/rules/security-backend.md` — **fixed** (decided by the rule)
- other: BE-API-015's new Verification flag fires only on a spec that says the endpoint accepts a handshake with no Origin on a cookie-authenticated connection; its MUST-name list (a) asks only for the Origin allowlist, so a cookie-authenticated spec that names an allowlist and says nothing about an absent Origin passes analyze though Statement (a) refuses that handshake. This is inside the owner's 2026-11-23 decision (flag acceptance, Statement unchanged); recorded as a residual gap, not a defect in the change. — `framework/rules/security-backend.md` — **waived**: within the owner's decision of 2026-11-23 (008's Resolved Question 24): the new flag catches a spec that accepts a handshake with no Origin on a cookie-authenticated connection, and requiring every such spec to state its absent-Origin handling is a further commitment the owner did not decide (decided by the rule)
- other: the pre-signed-URL MAY covers object storage only, so another signed-URL scheme, a CDN's or an application's own HMAC-signed link, falls under BE-API-013 and FE-STORAGE-003 — `framework/rules/security-backend.md` — **waived**: the owner's 2026-10-20 answer scoped the MAY to a pre-signed URL for one object and one method; whether other signed-URL schemes fit that scope is a new decision
- other: BE-AUTHN-007's Verification asks for legitimate-recovery paths during lockout, which its Statement does not name — `framework/rules/security-backend.md` — **waived**: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
- other: BE-API-001's Statement requires frame protection on all HTTP responses while its Verification table sets frame-ancestors on HTML responses only — `framework/rules/security-backend.md` — **waived**: predates this window; OWASP's HTTP Headers Cheat Sheet, which BE-API-001 cites, says X-Frame-Options gives no security on a redirect or a JSON API response, while its REST Security Cheat Sheet recommends frame-ancestors 'none' on API responses, so aligning the two decides what the rule requires
- other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md` — **waived**: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
- other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md` — **waived**: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
- other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md` — **waived**: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
- other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md` — **waived**: a resolved question records the reasoning at the time; rewriting it records a fiction
- bug: FE-CSRF-003's Statement bars every state-changing GET while its Not-flagged clause sets aside an OAuth client's GET redirection endpoint, so /review, which applies Statements and no routing, reports a MUST on a compliant callback /analyze sets aside — `framework/rules/security-frontend.md` — **waived**: Not mechanical: the clause names its owner, security-cross.md SEC-AUTHN-002 (a), and how /review applies a Not-flagged clause that names an owner is what 008 task 43's item gave to 020's scenario review-reports-a-defect-under-its-owner; this is one more case of it. (decided by the rule)
- other: BE-AUTHN-017 and security-frontend.md FE-STORAGE-004 both warn on a session cookie's unjustified Domain attribute in a full-stack project, and neither names the other — `framework/rules/security-backend.md` — **waived**: Not mechanical: both warn alike at SHOULD, so the double report gives no wrong result, and it predates this window; choosing an owner is an overlap decision of task 30's kind. (decided by the rule)
- other: BE-AUTHN-018 (d)'s 'asking the user instead does not comply' is stricter than its OWASP source, a reason only its pre-cut Rationale recorded — `framework/rules/security-backend.md` — **waived**: The rule format states no reason for a Statement (scenario rules-are-as-short-as-their-required-content, owner, 2026-10-28), and the cut kept (d)'s obligation; recording why a rule is stricter than its source is a new record of the kind 008 task 43's first item made by an owner's call. (decided by the rule)
- other: external claims in the authentication and credential rules have no fetched copy to re-check them against: Go net/http and crypto/tls, Python requests, RFC 7143, the Amazon EBS encryption guide, and OWASP's Authentication, Forgot Password and CSRF Prevention cheat sheets — `framework/rules/security-backend.md` — **waived**: Each was fetched and cited when its rule was written, or kept verbatim by the cut, which rewords no claim from recall (scenario rules-are-as-short-as-their-required-content); re-fetching every source is outside the passes, and the Summary names these as not re-checked. (decided by the rule)
- other: BE-ERR-003's Statement requires a structured 5xx while its Verification reaches an API that is not an HTTP API, which has no 5xx — `framework/rules/security-backend.md` — **waived**: Its Verification holds a non-HTTP API to the handler's existence and to no unhandled exception reaching the client, as its last Not-flagged sentence says, and leaves the response's shape to BE-ERR-002; the 5xx names the HTTP case, and restating the Statement is a rule decision. (decided by the rule)
- other: BE-API-013 and FE-STORAGE-003 more than doubled in the window, their flag sentences restating the Statements' MAYs where a pointer would carry them — `framework/rules/security-frontend.md` — **waived**: Not mechanical: each flag sentence names the carve-outs a reviewer acts on at the flag, compliant cases the cut keeps, and replacing them with a pointer rewrites two rules five re-read rounds settled. (decided by the rule)
- other: external claims the window added have no fetched copy to re-check them against: OpenID Connect RP-Initiated Logout 1.0 §2, JSON-RPC 2.0 §5.1, gRPC's error model, Stripe's API keys page, OWASP Secrets Management §3.6 and the Sealed Secrets README — `framework/rules/security-backend.md` — **waived**: Each was fetched and cited when tasks 43 and 44 wrote it, as their records state; re-fetching every source is outside the passes, and the Summary names these as not re-checked. (decided by the rule)
- other: BE-API-006 requires validating Content-Type on incoming requests, while its OWASP REST source requires it only of a request with a body — `framework/rules/security-backend.md` — **waived**: Predates the window, and narrowing the Statement to requests with a body is a rule decision. (decided by the rule)
- other: external claims in the dependency, input and header rules have no fetched copy to re-check them against: RFC 9110 §9.3.6, OWASP SSRF Prevention, RFC 9111, Cloudflare Cache Rules, W3C CSP and Referrer Policy, OWASP Forgot Password and HTTP Headers, PostgreSQL, Redis ACL, PgBouncer, Cloud SQL Auth Proxy, OWASP Deserialization, CSP3 on default-src, the Topics API explainer, Go net/http, Python requests and curl --proto — `framework/rules/security-backend.md` — **waived**: Each was fetched and cited when its rule was written, or kept verbatim by the cut, which rewords no claim from recall; re-fetching every source is outside the passes, and the Summary names these as not re-checked. (decided by the rule)
- convention: BE-API-001's header table follows its Verification paragraph, so the ductus exec walker's assessSpecQuality request, which carries the Verification paragraph, sends BE-API-001 without the header values — `framework/rules/security-backend.md` — **waived**: 060's scenario an-assessment-sees-the-rule-it-asks-about requires a request's Verification to run to the rule's next field, BE-API-001's header table included, so 060's pending work owns it; /analyze's host path reads each rule from its file whole. (decided by the rule)
- other: 008's first Resolved Question expresses an acknowledge tier as a SHOULD whose rationale says to record the deviation, a mechanism task 19 removed with every Rationale paragraph — `specs/008-security-rules/spec.md` — **waived**: A resolved question records the decision as taken, and 073, a draft, owns how a decided rule departure is now recorded, its open question 3 citing this one. (decided by the rule)
- other: task 45's done-when asks for bytes measured per rule and per file, and no artifact records them; `specs/008-security-rules/tasks.md` — `specs/008-security-rules/tasks.md` — **waived**: the figures are measured and stated, with their method, in this review's Summary; tasks.md is ephemeral and the measurement is a method of work, not a requirement (decided by the rule)
- other: `BE-API-013`'s Statement says the server MUST NOT send a credential or PII to a client in a URL, naming no list where its first sentence and Verification list passwords, session identifiers, tokens and API keys; low confidence; `framework/rules/security-backend.md` — `framework/rules/security-backend.md` — **waived**: the scenario `security-rules-flag-what-their-sources-support` defines credentials as that list and the owner's decision names no change to it; the edit would re-judge the rule file for every spec's next analysis for a reading the Verification already bounds (decided by the rule)
- other: `AC21`'s duplicate-ID blocking message is produced only by the host reading `discover-rule-files` output and is absent under `ductus exec`, and only `scripts/lint-rule-ids.sh` enforces uniqueness in CI; `specs/008-security-rules/spec.md` — `specs/008-security-rules/spec.md` — **waived**: an observation about the analyze walker's own machinery made while reviewing framework work; `framework/reference/analyze.md` states the exec reduction (decided by the rule)
- other: `AC22` to `AC27` (the brownfield audit) are procedure text in `framework/bootstrap/ductus-procedure.md` only and no test exercises the audit's output; `specs/008-security-rules/spec.md` — `specs/008-security-rules/spec.md` — **waived**: the audit is a deliberate procedural step with no host-level enforcement of the deterministic path (AGENTS.md Boundaries; spec 029) and 058 owns its contract (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/rules/accessibility-frontend.md`
- `framework/templates/ci/adopter-generators.yml`
- `specs/017-derive-dont-ask/spec.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/069-accessibility-frontend-rules/plan.md`

## Unexamined governance

*None.*
