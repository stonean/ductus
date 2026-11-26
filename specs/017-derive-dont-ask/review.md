---
spec: 017-derive-dont-ask
last-run: 2026-11-26T04:10:20Z
reviewed-against: 93c5e8ab5394948044c66154182f68b7b6d1a124
diff-base: ac842954c5f8815faad0ea43d1e53c4e0abf0b5d
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 11
scope: 11
unread: []
excluded: 492
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: dadd28edc909a348e9b5042e89d03c3f60f1d31c8de3eeb1ee0ca6c9a835574d
  scenarios/a-commit-stages-only-its-own-generated-files.md: 8626b701ad07371d0907c6a1a3a1ad4ee682c4d0d1622b3f503a76ae2bc189dc
  scenarios/a-path-list-commit-leaves-the-shared-index-current.md: c1bb7078add58084d19293a3db3f2b5904ad876b1611c3ca73033532b69e054a
  scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md: 6c52e86d521006fe5655e4a4fd704efad69ad5d9a6eec3bf70701b7962cdecf9
  scenarios/a-throwaway-credential-is-not-a-secret.md: e2167e26e43b1d9f1d6d0a6fb47928594a4c9b41d25d3b55bea4988c1a1e8fa5
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 5be65fb004623287403f9f61b9d13fe8a7b8079d623ff9cd70c85aebf6d0ee65
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 8a6671d9417b0183141f771c438c8a7d985b69b82b23e476e3bd2ec602a0e963
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: f294181444f3bd340d48df644da28f5181e8301531d6ec87276a8894a0961cf5
  scenarios/tracked-specs-not-worktree.md: 8ad8da4efcd159e76d359d978541f585f2e53d63876d218a0c27b15068b19ace
criteria-judged: 28
criteria-met: 28
criteria-verdicts:
  - label: AC1
    digest: 516c490a76b0710f3c17403ceb742cdc47c26ac5490cdb619aa495a65deca6d3
  - label: AC2
    digest: 27203c9d5172217929f6b4c50c0643ef5b765ccc0e081ed6e77321f84db74994
  - label: AC3
    digest: 60d4e6eb963333a0ba6eeb37a54e196ce7f0a771bd47da8eefad696ab907e919
  - label: AC4
    digest: 22007207d06b78225e67c123c0e5cb29433a89a4b6585f6889bd79bf9d81038f
  - label: AC5
    digest: adf809daf6d8bb4ccc9252015cd3265c7fd22ba7905803cbd1339a5591ddf874
  - label: AC6
    digest: 702830c76d20d55f5943bf83d01828e77c63f5b69b14e89e1026f4dfbac9d208
  - label: AC7
    digest: fbf85dc312a1c610bda3ff554432309eb99f85bcee1c8d6538586d22ebe64c39
  - label: AC8
    digest: 423f2a3768f6f4ab03ae8d555a993b84210cb3e74e35fdc7e47531b78c848570
  - label: AC9
    digest: 46f322e6ca0c1c1023b6d6f97183666363ddfdce050d0ac3753599e46d9a483e
  - label: AC10
    digest: ab18ffc3bd33b851dd2a95603730be2970ca873d167062702fd4a45e7f6f5559
  - label: AC11
    digest: 132a9273fb36fe225b77474a217036f412138c5a36ea2f038af75f86dd391751
  - label: AC12
    digest: 69a158a45dd944b2e9857777f0ab3bf79323dd032e238b769a872cc7f9258eaa
  - label: AC13
    digest: 5cb9773b8d0a4d04cc91ed74802baed2d78afaafb65609dd66989ab6196c3453
  - label: AC14
    digest: e2d976dcc7418973998f92b1883c6947d93d7452c6671ec6c4e05a54ed76705c
  - label: AC15
    digest: a05e6ca94106c638ced9c61266756f02aa3ed93073ac8abe6ef7bcc2124bf28d
  - label: AC16
    digest: 7148ef0c96bac309cbe5e920a1d4f89771e263c1b77aab23ca5d139d2068d50f
  - label: AC17
    digest: 679752ee2b2067889ce0b55e10dc08a41bdd4d69f4ceeba95da2e061421f21f3
  - label: AC18
    digest: 60c8f379b41308154c7faf03b185a4a66085a3528ad5a22a315bf3457a33062a
  - label: AC19
    digest: 2ed0b7df6e94b00dae0a21375f84dc639bbe12d855ef78281b63d29df2c6915e
  - label: AC20
    digest: b09362cae55fa36454dc0b4d647b5077962a810ca5ea7344decd3bf1f7a391f7
  - label: AC21
    digest: 62e18ec1db851d35304f688ce0f32db4ee61a5b91b131c6d5775781936341ebc
  - label: AC22
    digest: d5d6c9af91d0fe52ba7277b3430dcae401ae6936786423ff6003999b1a3bf82c
  - label: AC23
    digest: f34d6120992370378749ac88e2aa677cb4306d3f2cd0791a6155b1b6238c9ec7
  - label: AC24
    digest: dbe1d1fe9c7916cdcf522fee75b9c2bb28b30e454658dbdd2dd57c08121d93a8
  - label: AC25
    digest: ea2672bb340785ea7446df5a927ece2a07dc931cd6af9d75af0457984d845d12
  - label: AC26
    digest: 55351d67c4ca5dfe0cf480b31268c38c03c99bd9a39ef0f8bd79462e19f4745a
  - label: AC27
    digest: 4fde8fabf21c606ab327e8861fa28bda669a9a8923cc522e58a77f230d25a259
  - label: AC28
    digest: 256e50c06ce582db3ad9ea074332c075a5634071b090c6715a96766a5fc3fe3b
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 7
  undispositioned: 0
decisions:
  - key: "other: CFG-ENV-001 does not say whether an empty value on an optional value with a default is absent or malformed — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: predates the widening for environment variables and is unchanged by it; which way an empty optional value falls is a new decision, and CFG-ENV-008 already fails a present malformed value
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: \"at startup\" in CFG-ENV-003, CFG-ENV-005 and CFG-ENV-008 does not cover a configuration file a running process reads again — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: a value first read later is validated when it is read, the reading 063 applies on every call, and no spec here reloads a file mid-process
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-008's error names the setting and the value but not the file, which CFG-ENV-003 and CFG-ENV-005 now name for a file key — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: naming the file is a new requirement on a rule today's answers did not widen; the setting and the value already locate the defect
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-CONST-004 places a value's range bounds alongside its default constant, which a required value has none of — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: predates the widening for required environment variables; where a required value's bounds live is a new decision, and the bounds must still be named constants
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 names CFG-ENV-003 and CFG-ENV-008 as owning configuration, not CFG-ENV-005 — `framework/rules/quality-cross.md`"
    outcome: waived
    reason: QUAL-GROUND-001 cedes all configuration to the CFG rules, so the omission changes no finding; the two it names are the presence and type rules its scope decision cited
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-011's Source pairs IEC 80000-13 with a NIST page that credits the binary prefixes to IEC 60027-2 — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: BIPM's VIM attributes the binary prefixes to IEC 80000-13 and NIST to its predecessor IEC 60027-2; together the two citations are accurate
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015's handshake-URL MUST NOT also bars a CSRF token there, which the OWASP WebSocket Security Cheat Sheet suggests sending in the handshake — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the owner's 2026-10-20 answer keeps every token out of the handshake URL, following the same cheat sheet's advice against tokens in URL query strings; (a)'s Origin allowlist is the handshake's cross-site defense, and a CSRF token can still be checked in the first message
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: the pre-signed-URL MAY covers object storage only, so another signed-URL scheme, a CDN's or an application's own HMAC-signed link, falls under BE-API-013 and FE-STORAGE-003 — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the owner's 2026-10-20 answer scoped the MAY to a pre-signed URL for one object and one method; whether other signed-URL schemes fit that scope is a new decision
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-007's Verification asks for legitimate-recovery paths during lockout, which its Statement does not name — `framework/rules/security-backend.md`"
    outcome: waived
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-001's Statement requires frame protection on all HTTP responses while its Verification table sets frame-ancestors on HTML responses only — `framework/rules/security-backend.md`"
    outcome: waived
    reason: predates this window; OWASP's HTTP Headers Cheat Sheet, which BE-API-001 cites, says X-Frame-Options gives no security on a redirect or a JSON API response, while its REST Security Cheat Sheet recommends frame-ancestors 'none' on API responses, so aligning the two decides what the rule requires
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-018's newer-token invalidation would cancel a code issued by one method when a code by another is issued in the same recovery — `framework/rules/security-backend.md`"
    outcome: waived
    reason: a recovery that verifies the first code before issuing the second complies, since the first is already used; NIST SP 800-63B-4 sets no invalidation across codes and nothing in BE-AUTHN-018 forbids that order
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 008's ticked task rows 13, 14 and 15 describe FE-STORAGE-003, FE-DEPS-004 and BE-AUTHN-022 as they read before later edits — `specs/008-security-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005, widened to every source, does not say whether a value read from several sources is range-checked as resolved or at each source — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: the value validated at startup is the one CFG-ENV-007's resolution yields, the reading CFG-ENV-003 states for presence, and no spec here reads a ranged value from two sources
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "reliability: dropping a timed-out client joins its runtime thread, so a slow connect-time lookup can hold a fetch past FETCH_TIMEOUT — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: the GaiResolver it replaced behaves the same way, and in production the lookup is getaddrinfo, which the operating system resolver's own timeouts bound
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "reliability: the initial and redirect-screen lookups run outside the client's timeout — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: unchanged by this fix and bounded the same way, by the operating system resolver's timeouts; FETCH_DEADLINE bounds the download itself
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "security: a redirect hop is looked up twice rather than connected to the address its screen resolved — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: 022's scenario allows an equivalent connect-time re-check, both lookups are screened, and the owner prescribed this resolver and its staged-rebind test
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: two proxy tests each define the same four-line internal-address lookup — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: test fixtures local to each test; sharing one would couple the tests for no gain
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 025's ticked task 2 lists a malformed warning among the step-5 outcomes that task 10 replaced — `specs/025-rule-opt-out/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: waived
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: waived
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs`"
    outcome: waived
    reason: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs`"
    outcome: waived
    reason: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: pre-existing downloads with no named timeout, unchanged in this window: runtime-acquisition.yml:131-132, runtime-release.yml:623 (also retried with a fixed delay and no jitter) and install.sh:124 and :163 — `.github/workflows/runtime-release.yml`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml`"
    outcome: waived
    reason: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "bug: a pinned pre-release would read as current once its stable release ships, since sort -V orders 1.0.0 before 1.0.0-rc.1 — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: No pin in this tree is a pre-release, and the values it is compared with are crates.io's newest stable version and published release tags, so no input reaches the misordering.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: DUCTUS_SHA256 pairs with a DUCTUS_VERSION by file rather than by step, a tab split collapses an empty version field, and cargo install's --version=X, --vers and name@X forms are not parsed — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: Each fails loud, as a hash mismatch or an unpinned report, and the tree has one install step and only the `--version X` form.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: an unknown check name exits 2, the code that also means a check could not run — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: A misspelt argument is a run that examined nothing, which is what exit 2 states, and the message names the cause.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: 022's plan states a provenance exception for crates (BE-DEPS-004) but none for the pip-installed ziglang — `specs/022-deterministic-runtime/plan.md`"
    outcome: waived
    reason: 022's own analysis assesses BE-DEPS-004 against 022's plan, which names the ziglang install; it is outside 017's scope.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: the first weekly pins run will exit 1 on three action pins a major release behind, the template's checkout, upload-artifact and download-artifact — `.github/workflows/runtime-currency.yml`"
    outcome: waived
    reason: Expected and recorded in 022's task 3 measurement; moving those majors is outside task 3 and goes to the owner as its own question.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: the template's version check is close to redundant, since the sha256 pin already fixes the bytes — `framework/templates/ci/adopter-generators.yml`"
    outcome: waived
    reason: Scenario the-adopter-ci-template-can-run requires it, and it catches a mispackaged release the hash cannot.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: the pins check's .yaml globs match no tracked file — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: A guard, so a workflow added as .yaml is not skipped silently.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: pass() and fail() are copied across the four scripts/tests/*.sh — `scripts/tests/test-check-runtime-currency.sh`"
    outcome: waived
    reason: There is no shared test library to reuse, and adding one for two lines is not this change's work.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml`"
    outcome: waived
    reason: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: \"shipped\" still describes the CI template in 017's AC24, plan.md's Affected Files row and 022's task 3 text — `specs/017-derive-dont-ask/spec.md`"
    outcome: waived
    reason: AC24 calls it a shipped example, the Affected Files row says adopters copy it, and 022's task 3 text is a spent task; none claims it is installed.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: cargo's own requests in checks 1, 2 and 4 keep cargo's defaults rather than a timeout the script names — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: Pre-existing checks, unchanged in this window; runtime-currency.yml's timeout-minutes bounds the whole CI run.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: waived
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: waived
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the implement reference's \"non-blocking\" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md`"
    outcome: waived
    reason: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md`"
    outcome: waived
    reason: "pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: waived
    reason: the Context records the state when the scenario was found, and the close is loose, not false
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md`"
    outcome: waived
    reason: "contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "convention: a-status-commit-holds-only-the-transition's Behavior reads as requiring the bullet to say a transition staged alone is lost — `specs/050-constitution/scenarios/a-status-commit-holds-only-the-transition.md`"
    outcome: waived
    reason: the Behavior requires the bullet to state its condition, that the spec file holds nothing but the transition, and its reason, that the pre-commit hook restages the file whole; the bullet states both, and the loss of a transition staged alone is the scenario's own account of the incident
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: claude-md-import-syntax's shared-import bullet contrasts its rewrite with a removal the procedure no longer performs — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: the bullet still guards against de-duplicating an import the adopter added by hand, which removal would drop if the registration were withdrawn; its sense does not depend on the constitution line
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 017's plan and 072's Affected Files still name the old import form — `specs/017-derive-dont-ask/plan.md`"
    outcome: waived
    reason: each records that spec's own edit at the time, a decision record, not a claim about the tree today
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: a CLAUDE.md symlinked to AGENTS.md becomes a copy when constitution-import writes it by temporary file and rename — `framework/migrations/constitution-import.md`"
    outcome: waived
    reason: every bootstrap write of CLAUDE.md already does this, merge-managed-block's managed-block render on every run included, so the migration adds no new case; a symlinked CLAUDE.md is not a layout the framework supports
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 022's constitution-excerpts-as-skill-resources says every CLAUDE.md carries the constitution import — `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`"
    outcome: waived
    reason: this change makes the premise true again, the template's third line being @.ductus/constitution.md; 022 is another session's and owes no edit
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: waived
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: constitution-import accepts a four-space-indented line, misses a match behind a leading byte-order mark, and reports the @import form on a file unpinned after claude-md-import-syntax was skipped — `framework/migrations/constitution-import.md`"
    outcome: waived
    reason: "contrived inputs: claude-md-import-syntax treats indentation the same way, the template writes no byte-order mark, and an operator who unpins a file has the earlier migration's pinned warning in hand"
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "convention: a #fragment containing a dot is cut at the dot on a fold into a scenario — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: GitHub's heading slugs carry no dots, so such a fragment names no heading a link in this corpus can reach; a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: whether Auggie resolves CLAUDE.md's @AGENTS.md import, and so reads the routing block, is unverified — `framework/templates/project/claude-md.md`"
    outcome: waived
    reason: the rule's canonical statement is the constitution's drift-prevention section, which target step 4 loads whenever an import did not, so an Auggie session holds it either way
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: generator-primitives and audit-record-relocate state no report line for their no-op cases — `framework/migrations/generator-primitives.md`"
    outcome: waived
    reason: an observation about the migration machinery outside this window; the host's own wording reported each outcome truthfully
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fetch-archive sends no User-Agent — `runtime/src/primitives/fetch_archive.rs`"
    outcome: deferred
    target: specs/048-govern-acquired-runtime/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a .githooks linked into a host configuration directory inside the project is not judged by host_config::check_destination — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: a contrived layout; the write stays inside the project, and the referrer list is fixed to five files none of which is a host's configuration
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fold's resumption row for step 11 says a re-run rewrites nothing, which a fragment running into a second token contradicts — `framework/commands/fold.md`"
    outcome: waived
    reason: "a contrived input: a fragment that is itself a second spec-root path, which the rewrite now leaves for the orphan check to report"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a root-absolute /specs/ link is neither re-pointed nor reported — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: deferred
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an archive entry whose only mode bits are group or other write lands at the umask default — `runtime/src/primitives/extract_archive.rs`"
    outcome: waived
    reason: "a contrived entry: no archive the framework ships carries a file with neither owner read nor owner write"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: 062 task 23's checked item still gives the fixed-interval rationale for the lock poll — `specs/062-concurrent-session-targets/tasks.md`"
    outcome: waived
    reason: a ticked task records the decision as it was taken, task 28 records its reversal, and tasks.md is ephemeral work tracking that prune may drop
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: installed-file-modes M5's kill depends on ctime resolution finer than 50 ms — `scripts/mutation/installed-file-modes.py`"
    outcome: waived
    reason: disclosed in the list's docstring; APFS and ext4 stamp ctime in nanoseconds, and the test itself passes on any file system
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an @/specs/x import in CLAUDE.md is read as project-relative, where Claude Code reads an absolute path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: an absolute import of a top-level /specs directory is a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a kept-tails entry gives the reference from the spec root, without a prefix kept on the line — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: its path and line locate the reference, and the check reports the same span from the root
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a ./ prefix in the hook or a root markdown file assumes the project root is the work-tree root, which a project in a repository subdirectory breaks — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: waived
    reason: a ./ resolves as the bare paths in those files already do, the assumption every bare reference carries, not one task 30 adds
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a leading / was read from the project root in a project below its repository's root, where a host resolves it from the repository's root — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the framework reads a root-absolute path from the project root, as check-corpus-links reads a root-absolute link (spec 059); reading it from the repository's root here alone split the fold's link and token passes, so 3fc5755a's gate was reverted
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root, /proj/specs/x, the project's own path spelled from the repository's root, is neither checked nor re-pointed — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: a project's README spelling its own spec from the repository's root is a contrived input, and the project-root reading names it nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root the hook runs from the repository's root, so its ./ and bare paths resolve there — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: waived
    reason: the same assumption every bare reference in the hook carries, which task 30 does not add; prefixed and bare agree
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: bare-rule cases outside task 30 — ~/\".ductus/x\", \"$X\"specs/x and ${X}specs/x in the hook, ${X:-./specs/x}, *specs/x*, and @/specs/x in CLAUDE.md — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the bare rule predates task 30 and prefixed and bare agree on each; every input is contrived
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: the leading-/ exception also covers (a)/specs/x, x~/specs/x and a quote after a letter — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: contrived text; requiring $( before ) would add a parser for no real input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: no agreement test puts a path at column 0 in system.md, so a mutant refusing a line-start path only there survives, in the check's tests and the rewrite's — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the scanner has no per-referrer branch, so a defect of that shape is contrived; the binary reports a bare, ../ and / path at column 0 in system.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: the subdirectory test asserts that a spec only the repository's root holds is reported, not that one only the project holds is not — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the realistic regression, reading existence from the repository's root, is killed by M46; skipping only a path present at both roots is contrived, and the binary does not report the project-only spec
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: other — the hook's markdown stage skips silently when only the npx fallback is available or the local linter exits other than 1, so a lint that did not run looks like one that passed (constitution §design-principles); CI step (c) is the gate that catches it — `.githooks/pre-commit:227`
    outcome: waived
    reason: the owner's decision of 2026-11-13 (017 task 46) keeps the npx fallback and a non-1 exit swallowed so a failed fetch never blocks a commit, the header comment says so, and framework-checks step (c) runs the pinned linter on every push; an observation about the pipeline's own machinery
    decided-at: 2026-11-14T03:58:06Z
    decided-by: rule
  - key: other — staged_names, staged_md and the changed-path loop parse `git diff --name-only` output, which core.quotepath quotes for non-ASCII or control characters, while the raw -z list is unquoted; a staged path so quoted would never match, so its scratch-run change would be dropped rather than carried — `.githooks/pre-commit:139`
    outcome: waived
    reason: "latent only: measured 0 of 1304 tracked paths contain a non-ASCII character, and a fix (`--name-only -z` throughout) is a design change to a hook the owner reshaped on 2026-11-11; an observation about the pipeline's own machinery"
    decided-at: 2026-11-14T03:58:06Z
    decided-by: rule
  - key: other — the test holds no case for the hook's stated swallow branches (the npx fallback exiting 1, a local linter exit other than 1); the fixture installs a local stub, so removing the `mdlint_local` condition would break no test — `scripts/tests/test-pre-commit-staging.sh:318`
    outcome: waived
    reason: QUAL-TEST-001 judges a test that exists against the behavior it names, and no test names this behavior; a new case is new work the owner has not asked for, and the branch's blast radius is a lint CI runs again; an observation about the pipeline's own machinery
    decided-at: 2026-11-14T03:58:06Z
    decided-by: rule
  - key: "other — runtime/CHANGELOG.md's [Unreleased] prose says `/review` \"applies Statements and no routing until spec 020's lands\" (introduced by 0481c218, ~lines 962-969; the same future-tense claim also stands at ~787 and ~806, outside the window). Spec 020 task 15 (efe81579, 2026-11-11) landed the routing in framework/reference/review.md §Security pass (\"Report a defect under its owner\"), so the sentence is false at HEAD. It was true when 0481c218 wrote it; 020's commit did not sweep the changelog. — `runtime/CHANGELOG.md:962`"
    outcome: waived
    reason: The claim became stale by 020's later commit, not by 017's hunk; the entry is the Unreleased section whose own preamble says the release commit rewrites it, and a correct rewrite must first decide which pairs the landed routing now moves (configuration-cross.md is applied by no review pass), a judgement for 020's owner rather than a mechanical edit.
    decided-at: 2026-11-14T03:58:06Z
    decided-by: rule
  - key: "other — every_short_help_prints_on_one_line (the window's test) checks a subcommand's `about` and an argument's short help, but not `long_about` or an argument's long help, so a brace-wrapped n in a descriptions.rs constant (which is both `about`'s source and `long_about`) or in the later paragraph of a clap::Args doc comment would still split `--help` with every test green; descriptions.rs's module doc states the ban and nothing enforces it for the long form. No current violation: `{n}` occurs in runtime/src only in that module doc, this test, and three non-clap result-struct doc comments in schema/primitives.rs. — `runtime/src/main.rs:1400-1425`"
    outcome: waived
    reason: An enforcement gap in the pipeline's own runtime machinery, not a defect in 017's code, and no violation exists today (grep of runtime/src); the test's doc comment claims only the short forms and is accurate. Deferring it would reopen a done spec for a hardening nobody has asked 017 to carry.
    decided-at: 2026-11-14T03:58:06Z
    decided-by: rule
  - key: other — hydration of the 21 `--{name}-json` flags is wired by hand in main.rs, and `every_mcp_argument_has_a_cli_form` only proves a flag exists. Only fired-json (process-waivers) and fix-rounds-json/absent (write-review) are driven through a hydrator by a test; no test or script in runtime/tests, runtime/src or scripts/ drives the other write-review fields (findings, observations, applied/expired/new waivers, expired-decisions, decided-by, unread), write-analysis's four, or prune-project's replace through their hydrator, so deleting one hydrate_field line leaves the suite green and a CLI write-review silently drops that argument. — `runtime/src/main.rs:339-415`
    outcome: waived
    reason: "Pre-window code in the pipeline's own machinery; the shared hydrate_field mechanism is tested and each call is one line read in this review (all 21 fields hydrated: 9 write-review, 4 write-analysis, 2 process-waivers/decisions, 3+2 manifests, 1 prune-project). A per-flag test is new work for spec 022's task 24 surface, not a contract break of 017."
    decided-at: 2026-11-14T03:58:06Z
    decided-by: rule
  - key: other — criterion-label-backfill.md Notes cite `700 criteria across 49 specs in this repository's own backfill`; by git, commit eaa3908c assigned 663 labels across 49 spec files, and 700 is the count of labelled criteria in the tree at that commit (663 assigned plus 37 already hand-labelled). The figure states no method, so a reader cannot tell which it counts. — `framework/migrations/criterion-label-backfill.md:24`
    outcome: waived
    reason: Outside the window's hunk (74f6361c changed only step 1), historical illustration in an adopter-facing note that gates nothing, and no loaded rule maps to it; a recorded count's method is a constitution grounding norm, not a rule.
    decided-at: 2026-11-14T03:58:06Z
    decided-by: rule
  - key: quality — Frontmatter Migration walks the literal `specs/` and tests `specs/` existence rather than the configured `[paths] specs-root`, so a project with a renamed spec root and bold-prefix specs is skipped silently — `framework/bootstrap/ductus-procedure.md`
    outcome: waived
    reason: The Frontmatter Migration serves pre-017 projects, which predate `[paths] specs-root` (040), so the literal `specs/` is their spec root; 029's review waived the same by this reason.
    decided-at: 2026-11-19T13:39:22Z
    decided-by: rule
  - key: convention — the new Frontmatter Migration containment screen has the agent run `realpath` by hand, a shell step where a runtime primitive would be the sanctioned form, though `/bin/realpath` exists on macOS and no primitive covers it — `framework/bootstrap/ductus-procedure.md`
    outcome: waived
    reason: No runtime primitive resolves a canonical path today, and the step is one comparison the agent makes with `realpath`; a primitive would be a new feature, not a fix to this change.
    decided-at: 2026-11-19T13:39:22Z
    decided-by: rule
  - key: "other: the throwaway-credential carve-out is scoped to `CFG-ENV-013` only, so `CFG-ENV-001` (an application default), `CFG-ENV-010` (a client artifact) and `BE-DATA-003`, `BE-DATA-008` and `BE-DATA-012` in `framework/rules/security-backend.md` keep the unqualified word secret, and `CFG-ENV-009`'s Not-flagged routes a compiled-in credential to `CFG-ENV-013`; whether the decision reaches those siblings is a new owner decision, and no analysis result flips to a finding. — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: the owner's decision of 2026-11-19 is scoped to CFG-ENV-013 and the siblings flag different placements; extending the carve-out is a separate decision owned by 017 for the CFG-ENV rules and 008 for the BE-DATA rules, so it goes to the owner through the parent, not through this review
    decided-at: 2026-11-20T04:19:48Z
    decided-by: rule
  - key: "other: the fix-round re-read noted that the scenario `a-throwaway-credential-is-not-a-secret` says `CFG-ENV-011` stays a MUST, where `CFG-ENV-013` is evidently meant. — `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md`"
    outcome: waived
    reason: "the sentence is right: task 47's third item records that CFG-ENV-011 stays a MUST, an owner decision of 2026-11-19, and the scenario's second bullet says so beside the encrypted-copy permission"
    decided-at: 2026-11-20T04:19:48Z
    decided-by: rule
  - key: "other: the shipped `framework/bootstrap/hooks/ductus-pre-commit`, in a path-list commit, skips `derive-dependencies --staged` and `derive-references --staged` without a word (the runtime reads the repository's own index, never `GIT_INDEX_FILE`, so the named spec reads as unstaged: a body link to a sibling committed beside `dependencies: []`), and leaves the `label-criteria` rewrite staged in the temporary index only, so the shared index holds the unlabelled blob — `framework/bootstrap/hooks/ductus-pre-commit`"
    outcome: waived
    reason: the shipped hook is spec 018's, outside the code under review, and the fix is a design change to the adopter hook (the carry this spec added to the repository's own hook, plus a staged-set the runtime can read from the temporary index); an observation about the pipeline's own machinery, which the generators CI step also catches for the derivation half
    decided-at: 2026-11-26T04:09:35Z
    decided-by: rule
---

# Review — 017-derive-dont-ask

## Summary

Window ac842954 to 93c5e8ab (diff-base from the last review; base-source last-review): 411 commits examined, 12 claimed by no spec. 11 scope entries, 11 examined: `.githooks/pre-commit`, `scripts/tests/test-pre-commit-staging.sh`, `runtime/tests/common/mod.rs`, `runtime/tests/mechanical_sweep_parity.rs` and `docs/developing-ductus.md` whole; `runtime/CHANGELOG.md`, which joined the scope as the entry the last review left unread, as the hunks 017's commits gave it (da83bf9f and da7bd813), the rest of its 655 KB unread; and `spec.md`, `plan.md`, `tasks.md`, `analysis.md` and the scenario `a-path-list-commit-leaves-the-shared-index-current` whole. Loaded rule files: `configuration-cross.md`, `quality-cross.md`, `security-backend.md`, `security-cross.md` and `security-frontend.md`, each read whole; no pass was skipped. No MUST or SHOULD finding; all 28 criteria judged met, the criteria parts reading the passages each criterion's claim names rather than whole files. Observations: two fixed in the run (the scenario's claim that only this repository's hook has the staged reversal, which the shipped hook reproduces, and the plan's missing account of the shared-index carry), one waived by the rule (the shipped hook's path-list behavior, spec 018's), six stored decisions matched; one expired (the Family 19 parity tests under a hook's inherited `GIT_DIR`, which task 49 and 066's fixture fix removed).

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

12 of 411 commits in the window no spec claimed.

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

- AC1 — met: No `title:` in any of `framework/templates/spec/` (data-model, plan, research, scenario, spec, tasks; grep for title/tags/track/spec-ref found only scenario's `section:`); `spec-and-plan.md` no longer exists
- AC2 — met: Records its own retirement: `framework/templates/spec/spec-and-plan.md` is absent (ls of the directory), so the 'since retired' claim holds and no `track:` survives in any template
- AC3 — met: `framework/templates/spec/scenario.md` frontmatter is `section:` only; no template or command under `framework/commands` and `framework/templates` writes `spec-ref` (only `target.md` reads it as a legacy fallback)
- AC4 — met: Searched `framework/commands`, `framework/reference`, `framework/templates`, `framework/constitution.md`: no `tags` write/prompt/check, no Starter Tag Vocabulary, constitution names it only as a stale field (open-schema paragraph); runtime read claim holds (`runtime/src/primitives/pipeline_view.rs` tags_union line)
- AC5 — met: Searched `framework/commands`, `framework/templates`, `framework/reference`, `framework/constitution.md`, `AGENTS.md`: no `[simple]` marker; `framework/constitution.md` Cost levers section has no tier bullet
- AC6 — met: `framework/commands/groom.md` and `framework/reference/groom.md` carry no `promote-to-rule`; step 2 walks every item in the inbox list each run, so an unmigrated item is re-walked
- AC7 — met: Records the supersession: `framework/templates/spec/plan.md` now has an optional `## Open Questions Resolved` section ('Optional - omit this section when...'), matching the current text's claim
- AC8 — met: `AGENTS.md` has no mirror-constitutions instruction and no commit-time run-gen-claude-commands instruction; the Gotcha 'edit to framework/commands is invisible until the generator re-runs' is the different instruction the criterion names
- AC9 — met: No title is prompted: `framework/commands/specify.md` step 5 `create-feature` derives number/slug/directory from the description, `create-scenario` derives the H1 from the slug (`runtime/src/primitives/create_scenario.rs`), `/plan` copies templates via `create-plan-artifacts`; claim is loosely worded, judged on the no-prompt, filename-derived behavior
- AC10 — met: Records its own retirement: `scripts/gen-readme-table.sh` is absent, `README.md` has no `generated:` marker, the hook and `.github/workflows/*.yml` do not reference it, so 'since retired' holds
- AC11 — met: `framework/commands/help.md` has 5 `generated:*:start` markers; `/bin/bash scripts/gen-help-tables.sh --dry-run` on the tree printed 'No changes (16 command(s) in sync)', exit 0
- AC12 — met: `.githooks/pre-commit` runs `gen-configure-mcp`, `gen-claude-commands`, `gen-help-tables` plus `derive-dependencies`/`derive-references --write`, and exits 1 'the ductus runtime is not reachable' as the current text says; `scripts/install-hooks.sh` sets `core.hooksPath .githooks`; staging scoped to the commit's paths per its header (tasks 48-50 not in this criterion's text)
- AC13 — met: Judged on its Q7 supersession text: `framework/commands/clarify.md` step 4 and `framework/commands/plan.md` sub-step 1.1 invoke `derive-dependencies` report-only (no `--write`) and report drift; `implement.md` sub-step 1.1 does too
- AC14 — met: `framework/commands/implement.md` step 2 invokes `derive-boundary` and says Affected Files is 'a planning aid, not authoritative'; `runtime/src/primitives/derive_boundary.rs` diffs from the spec dir's first commit to HEAD; `framework/templates/spec/plan.md` Affected Files comment reads 'Planning aid only'
- AC15 — met: `framework/commands/analyze.md` has no Fix Mode section; `--fix` only reverts a drifted done spec via set-status (`framework/reference/analyze.md` §Review state drift), and neither file has a checkbox fix entry (searched both for fix mode/fixable/checkbox).
- AC16 — met: No root `constitution.md` in the tree; `CLAUDE.md` imports `@framework/constitution.md`; no gen-root* script in `scripts/`, `.githooks/` or `framework/`.
- AC17 — met: `clarify.md` step 4 (derive-dependencies) and step 8 (sibling links vs dependencies), `plan.md` step 1.1 and `implement.md` step 1.1 run derive-dependencies, and `implement.md` steps 7 and 10 call `diff-cross-spec` and surface `cross-spec-paths` for files outside the spec dir.
- AC18 — met: `framework/rules/configuration-cross.md` holds CFG-CONST-001/002 (shared, module-local), CFG-CONST-003, CFG-ENV-001/003/008 (env defaults, validation) and CFG-ENV-004 (time units), each with Verification; `framework/constitution.md` §constants and §env-vars are one-line pointers.
- AC19 — met: `validate-frontmatter` on `specs/000` to `017` (all 16 present dirs) in the review worktree returns clean:true, 0 findings each; stale fields are open-schema, no analyze check reads them.
- AC20 — met: `specs/017-derive-dont-ask/spec.md` frontmatter holds only status, dependencies, next-criterion and retired-rules; no title or tags.
- AC21 — met: `framework/bootstrap/hooks/pre-commit` ships; `framework/bootstrap/hooks/` has no install.sh; `framework/bootstrap/ductus-procedure.md` §Hook Installation holds the install actions; the inner `ductus-pre-commit` runs only derive-dependencies, derive-references, label-criteria and read-only checks.
- AC22 — met: `framework/bootstrap/ductus-procedure.md` §Hook Installation: detection ladder cases 1-5 (custom hooksPath, `.husky`, lefthook, pre-commit-py skip with snippet), `.githooks/pre-commit` `create` once, inner `ductus-pre-commit` `update`, non-comment-line test, `[pinned] files` section.
- AC23 — met: `.ductus/scripts/` does not exist in the tree; no framework, script or CI file but the `generator-primitives`/rename migration records names gen-spec-deps; `derive-dependencies` and `derive-references` are the runtime primitives the inner hook and CI call.
- AC24 — met: `.github/workflows/generators.yml` runs the generators for real then fails on `git status --porcelain`; `framework/templates/ci/adopter-generators.yml` steps 'Derive generated frontmatter' and 'Fail if any derivation produced a diff' do the same with the release binary at a pinned sha256.
- AC25 — met: `framework/reference/review.md` §Observations: an observation takes a disposition before the record is written (step 10, no separate step), the record counts undispositioned ones, only a broken contract holds done; `framework/commands/review.md` writes no inbox.
- AC26 — met: The groom tree binds beyond groom: `framework/commands/specify.md` step 2 walks the same §Groom decision tree, `amend.md` step 'Walk the bug decision tree', `analyze.md` step 19 and `review.md` step 10 route by it (`framework/reference/groom.md` §Groom decision tree).
- AC27 — met: `framework/rules/configuration-cross.md` header declares `FLAG`; CFG-FLAG-001 MUST, CFG-FLAG-002 SHOULD, CFG-ENV-008 MUST, -009 SHOULD, -010 MUST, -011 MUST, -012 SHOULD each carry Statement, Verification and Source; `specs/017-derive-dont-ask/data-model.md` registers FLAG in the ID-prefix, Category-name and abbreviation table and §Rule set lists all 19 rules with tier.
- AC28 — met: CFG-ENV-013 (MUST) with Statement, Verification and Source is in `framework/rules/configuration-cross.md`; CFG-ENV-009 and -010 and `framework/rules/security-backend.md` BE-DATA-003, -008 and -012 each send the case to it; `specs/017-derive-dont-ask/data-model.md` §Rule set lists it MUST.

## Observations

- convention: the last sentence of the Context of `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md` says only this repository's hook has the staged reversal because the shipped hook re-stages only spec files already in the commit; reproduced false on the HEAD runtime: a path-list commit of a spec with a hand-typed criterion through the shipped `framework/bootstrap/hooks/ductus-pre-commit` commits the labelled spec and leaves `MM` in `git status --porcelain` and the spec in `git diff --cached` — `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md` — **fixed** (decided by the rule)
- other: the shipped `framework/bootstrap/hooks/ductus-pre-commit`, in a path-list commit, skips `derive-dependencies --staged` and `derive-references --staged` without a word (the runtime reads the repository's own index, never `GIT_INDEX_FILE`, so the named spec reads as unstaged: a body link to a sibling committed beside `dependencies: []`), and leaves the `label-criteria` rewrite staged in the temporary index only, so the shared index holds the unlabelled blob — `framework/bootstrap/hooks/ductus-pre-commit` — **waived**: the shipped hook is spec 018's, outside the code under review, and the fix is a design change to the adopter hook (the carry this spec added to the repository's own hook, plus a staged-set the runtime can read from the temporary index); an observation about the pipeline's own machinery, which the generators CI step also catches for the derivation half (decided by the rule)
- convention: the Pre-commit hook architecture paragraph of `specs/017-derive-dont-ask/plan.md` says each changed file is written 'to the real index' and never says a path-list commit also writes it to the shared index (`index.lock`), the design tasks 48 and 50 landed in `.githooks/pre-commit`; the plan's design record omits it — `specs/017-derive-dont-ask/plan.md` — **fixed** (decided by the rule)
- other: AC24 in `specs/017-derive-dont-ask/spec.md` still says 'a shipped example' and the Affected Files row of `specs/017-derive-dont-ask/plan.md` still says 'Shipped template adopters can copy', for a template the plan's CI safety net paragraph says no manifest row installs — `specs/017-derive-dont-ask/spec.md` — **waived**: AC24 calls it a shipped example, the Affected Files row says adopters copy it, and 022's task 3 text is a spent task; none claims it is installed.
- other: the Technical Decisions paragraph and the CLAUDE.md Affected Files row of `specs/017-derive-dont-ask/plan.md` still spell `@import constitution.md`, the old import form — `specs/017-derive-dont-ask/plan.md` — **waived**: each records that spec's own edit at the time, a decision record, not a claim about the tree today
- other: `.githooks/pre-commit` still swallows the npx fallback's exit and any local-linter exit other than 1, so a markdown lint that did not run reads as one that passed; the header comment states it — `.githooks/pre-commit` — **waived**: the owner's decision of 2026-11-13 (017 task 46) keeps the npx fallback and a non-1 exit swallowed so a failed fetch never blocks a commit, the header comment says so, and framework-checks step (c) runs the pinned linter on every push; an observation about the pipeline's own machinery (decided by the rule)
- other: `staged_names`, `staged_md` and the changed-path loop of `.githooks/pre-commit` parse `git diff --name-only` text that `core.quotepath` quotes for a non-ASCII path, while `changes.raw` is `-z`, so such a path would never match and its scratch-run change would be dropped — `.githooks/pre-commit` — **waived**: latent only: measured 0 of 1304 tracked paths contain a non-ASCII character, and a fix (`--name-only -z` throughout) is a design change to a hook the owner reshaped on 2026-11-11; an observation about the pipeline's own machinery (decided by the rule)
- other: `scripts/tests/test-pre-commit-staging.sh` holds no case for the hook's stated swallow branches (the npx fallback, a local linter exit other than 1): its fixture installs a local stub only, and removing the `mdlint_local` condition breaks no case — `scripts/tests/test-pre-commit-staging.sh` — **waived**: QUAL-TEST-001 judges a test that exists against the behavior it names, and no test names this behavior; a new case is new work the owner has not asked for, and the branch's blast radius is a lint CI runs again; an observation about the pipeline's own machinery (decided by the rule)
- other: `runtime/CHANGELOG.md` `[Unreleased]` still says `/review` applies Statements and no routing until spec 020's lands (now at ~2443, one occurrence), though `framework/reference/review.md` §Security pass's 'Report a defect under its owner' landed that routing — `runtime/CHANGELOG.md` — **waived**: The claim became stale by 020's later commit, not by 017's hunk; the entry is the Unreleased section whose own preamble says the release commit rewrites it, and a correct rewrite must first decide which pairs the landed routing now moves (configuration-cross.md is applied by no review pass), a judgement for 020's owner rather than a mechanical edit. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md`
- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.claude/commands/ductus/status.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/prune.md`
- `.ductus/reference/review.md`
- `.gitattributes`
- `.githooks/pre-push`
- `.github/workflows/framework-checks.yml`
- `.github/workflows/runtime-currency.yml`
- `.github/workflows/runtime-release.yml`
- `.github/workflows/runtime.yml`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `.pi/prompts/ductus-status.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `docs/rules.md`
- `docs/shared-constitution.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus-procedure.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/bootstrap/hooks/ductus-pre-commit`
- `framework/bootstrap/hooks/ductus-pre-push`
- `framework/bootstrap/hooks/pre-push`
- `framework/commands/amend.md`
- `framework/commands/analyze.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/groom.md`
- `framework/commands/help.md`
- `framework/commands/implement.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/commands/status.md`
- `framework/constitution.md`
- `framework/migrations.toml`
- `framework/migrations/audit-record-relocate.md`
- `framework/migrations/criterion-label-backfill.md`
- `framework/migrations/project-review-record.md`
- `framework/migrations/remove-legacy-constitutions-region.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/fold.md`
- `framework/reference/groom.md`
- `framework/reference/implement.md`
- `framework/reference/prune.md`
- `framework/reference/review.md`
- `framework/rules/accessibility-frontend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-frontend.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `framework/templates/spec/spec.md`
- `install.sh`
- `runtime/Cargo.lock`
- `runtime/Cargo.toml`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/append_question.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_departures.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_prescriptions.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_stuck.rs`
- `runtime/src/primitives/check_unreviewed_code.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/criterion_verdicts.rs`
- `runtime/src/primitives/dashboard.rs`
- `runtime/src/primitives/decisions.rs`
- `runtime/src/primitives/departures.rs`
- `runtime/src/primitives/derive_references.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/invalidate_review.rs`
- `runtime/src/primitives/label_criteria.rs`
- `runtime/src/primitives/line_citations.rs`
- `runtime/src/primitives/lint_markdown.rs`
- `runtime/src/primitives/mark_criterion.rs`
- `runtime/src/primitives/mark_task.rs`
- `runtime/src/primitives/measure_reopenings.rs`
- `runtime/src/primitives/mechanical_sweep.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/merge_permissions.rs`
- `runtime/src/primitives/merge_status_hook.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/owed_work.rs`
- `runtime/src/primitives/pipeline_view.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/project_parts.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/read_re_read.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/read_tasks.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/remove_inbox_item.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/review_subject.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/set_status.rs`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/moved_scope.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/src/primitives/write_review/seed.rs`
- `runtime/src/schema/config_keys.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/extensions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/src/schema/protocol.rs`
- `runtime/src/schema/registry.rs`
- `runtime/tests/check_artifact_size_cli.rs`
- `runtime/tests/check_unreviewed_code_cli.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/exec_subprocess.rs`
- `runtime/tests/fix_rounds.rs`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/golden/specify-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `runtime/tests/mcp.rs`
- `runtime/tests/measure_reopenings.rs`
- `runtime/tests/merge_status_hook.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/review_command.rs`
- `runtime/tests/size_hook.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/judge-brief.md`
- `scripts/analyze/record.py`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/audit/README.md`
- `scripts/audit/adopter-shell-behavior.sh`
- `scripts/audit/archive-contents.sh`
- `scripts/audit/cross-doc-consistency.sh`
- `scripts/audit/done-spec-criteria.sh`
- `scripts/audit/permission-wildcard-position.sh`
- `scripts/audit/rename-sweep-residue.sh`
- `scripts/audit/run-all.sh`
- `scripts/audit/ssot-invariants.sh`
- `scripts/audit/sweep-target-manifest-parity.sh`
- `scripts/check-release-tag.sh`
- `scripts/lint-tool-coverage.sh`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/artifact-size-bounded.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/compare-and-set-066.py`
- `scripts/mutation/criteria-in-review-066.py`
- `scripts/mutation/criterion-ticks-outside-the-digest.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/departures.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/freshness-results-have-a-bounded-size.py`
- `scripts/mutation/line-citation-strip-066.py`
- `scripts/mutation/loop-inputs-screened.py`
- `scripts/mutation/moved-scope-066.py`
- `scripts/mutation/never-owed-084.py`
- `scripts/mutation/owed-work-084.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/plan-edit-takes-no-back-edge.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/project-parts-087.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/quoted-heading-forms-067.py`
- `scripts/mutation/reachable-trigger-bar-078.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/review-022-task-41.py`
- `scripts/mutation/review-022-task-42.py`
- `scripts/mutation/review-022-task-46.py`
- `scripts/mutation/review-departures-bounded.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/routing-path-claims.py`
- `scripts/mutation/rule-files-left-to-analysis-020.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/site-anchors-020.py`
- `scripts/mutation/status-count-084.py`
- `scripts/mutation/suffixed-task-number.py`
- `scripts/mutation/unreviewed-code-084.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-archive-contents.sh`
- `scripts/tests/test-audit-path-containment.sh`
- `scripts/tests/test-check-release-tag.sh`
- `scripts/tests/test-done-spec-owed-work.sh`
- `scripts/tests/test-run-workflow-steps.sh`
- `specs/000-slash-commands/review.md`
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
- `specs/000-slash-commands/scenarios/scenario-without-task-visibility.md`
- `specs/008-security-rules/analysis.md`
- `specs/008-security-rules/plan.md`
- `specs/008-security-rules/review.md`
- `specs/008-security-rules/scenarios/a-cache-partition-is-purged-when-the-session-ends.md`
- `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md`
- `specs/008-security-rules/scenarios/a-throwaway-credential-owes-no-revocation-path.md`
- `specs/008-security-rules/scenarios/an-absent-origin-on-a-cookie-connection-is-flagged.md`
- `specs/008-security-rules/scenarios/be-deps-005-flags-what-its-statement-requires.md`
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/scenarios/the-tag-check-reads-every-entry-above-the-newest-version.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/010-agent-autonomy/plan.md`
- `specs/011-brownfield-process/spec.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/spec.md`
- `specs/013-text-first-artifacts/plan.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/014-reclarify-backedge/plan.md`
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md`
- `specs/014-reclarify-backedge/spec.md`
- `specs/015-tarball-fetch/analysis.md`
- `specs/015-tarball-fetch/plan.md`
- `specs/015-tarball-fetch/review.md`
- `specs/015-tarball-fetch/spec.md`
- `specs/015-tarball-fetch/tasks.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/018-adopter-owned-pre-commit/spec.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/scenarios/waiver-file-lists.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/021-runtime-boundary/review.md`
- `specs/022-deterministic-runtime/analysis.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/review.md`
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/023-govern-refinement/analysis.md`
- `specs/023-govern-refinement/plan.md`
- `specs/023-govern-refinement/review.md`
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md`
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md`
- `specs/023-govern-refinement/scenarios/living-specs.md`
- `specs/023-govern-refinement/spec.md`
- `specs/023-govern-refinement/tasks.md`
- `specs/024-rule-loader/analysis.md`
- `specs/024-rule-loader/plan.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/025-rule-opt-out/review.md`
- `specs/025-rule-opt-out/spec.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/026-framework-self-audit/scenarios/family-27-done-spec-unchecked-criteria.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/030-cross-service-references/plan.md`
- `specs/030-cross-service-references/spec.md`
- `specs/034-performance-backend-rules/analysis.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md`
- `specs/035-groom-session-target/scenarios/reopen-done-spec-on-scenario.md`
- `specs/037-observability-backend-rules/analysis.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/analysis.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/analysis.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/scenarios/a-plan-edit-does-not-reopen-a-done-spec.md`
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md`
- `specs/041-task-pruning/scenarios/prune-all-walks-every-spec.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/046-scenario-open-question-visibility/plan.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/047-analyze-findings-durability/spec.md`
- `specs/048-govern-acquired-runtime/analysis.md`
- `specs/048-govern-acquired-runtime/plan.md`
- `specs/048-govern-acquired-runtime/review.md`
- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/048-govern-acquired-runtime/tasks.md`
- `specs/049-rename-govern-to-ductus/plan.md`
- `specs/049-rename-govern-to-ductus/review.md`
- `specs/049-rename-govern-to-ductus/spec.md`
- `specs/050-constitution/analysis.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/review.md`
- `specs/050-constitution/scenarios/a-canonical-source-is-pointed-at-not-copied.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/a-retired-filename-leaves-a-decision-record.md`
- `specs/050-constitution/scenarios/adopter-practices-classified-for-promotion.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-pending-fold-never-holds-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/fold-target-checked-before-the-rewrite.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/054-remove-supersession/plan.md`
- `specs/054-remove-supersession/spec.md`
- `specs/055-shared-constitution/analysis.md`
- `specs/055-shared-constitution/plan.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/058-findings-route-at-discovery/analysis.md`
- `specs/058-findings-route-at-discovery/data-model.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/review.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/review.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-write-result-states-what-it-wrote.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/analysis.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md`
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/analysis.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/analysis.md`
- `specs/069-accessibility-frontend-rules/plan.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/070-performance-frontend-rules/analysis.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/073-decided-rule-departures-are-recorded/analysis.md`
- `specs/073-decided-rule-departures-are-recorded/data-model.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/073-decided-rule-departures-are-recorded/tasks.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/analysis.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/review.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md`
- `specs/077-security-cross-rules/analysis.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/review.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/analysis.md`
- `specs/080-deterministic-runtime-efficiency/data-model.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`
- `specs/082-rule-assessments-converge-across-runs/analysis.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/review.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/082-rule-assessments-converge-across-runs/tasks.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/analysis.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/review.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/tasks.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/analysis.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/data-model.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/review.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`
- `specs/088-rule-judgments-cost-only-what-changed/spec.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md`
- `specs/README.md`
- `specs/review.md`
- `version`

## Unexamined governance

*None.*
