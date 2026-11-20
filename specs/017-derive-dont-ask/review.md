---
spec: 017-derive-dont-ask
last-run: 2026-11-20T04:19:58Z
reviewed-against: da7bd813e09e3b6a3da1d9ce65a3dac241abad80
diff-base: fa4ae5c75b7d68e907f18cee94eb8f8157f83f52
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 6
unread:
  - runtime/CHANGELOG.md
excluded: 34
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
  fixed: 3
  deferred: 0
  waived: 2
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
  - key: "test: the Family 19 parity tests run git in their fixtures under a hook's inherited GIT_DIR, so a commit from a linked worktree fails four of them — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
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
---

# Review — 017-derive-dont-ask

## Summary

Window fa4ae5c7 to the commit before this record, base-source reopen: the owner decisions of 2026-11-19 reopened 017 (task 47: CFG-ENV-013 sets aside a throwaway local or test credential; the BE-DEPS-006 signpost corrected; retired-rules lists BE-DEPS-006). From a clean worktree of origin/main at 84e090b9. Scope 6 entries (the five of the decision commits plus `specs/017-derive-dont-ask/data-model.md`, which this run fixed), examined 5: `framework/rules/configuration-cross.md`, the scenario `a-throwaway-credential-is-not-a-secret`, `spec.md`, `tasks.md` and the data model were read whole by the passes. `runtime/CHANGELOG.md` (610,176 bytes) is unread as a file: the passes read the window diff whole (11,021 bytes) and the 017-bearing entries in part, so it is named unread, not counted. The security pass read the three security rule files whole (193,813 bytes) and found no violation; the other passes read the nine other rule files whole (216,167 bytes), AGENTS.md Gotchas to Design Principles and `specs/system.md`, and 017 spec, plan, data model and all ten scenarios whole, and found none. The criteria step judged all 28 criteria met against the tree; AC19 was judged on validate-frontmatter over specs 000 to 019 and the analyze reference without re-driving a full analysis. Five observations: three fixed in one commit (the data model records the carve-out; the Verification says a secret, not a throwaway credential, owes the encrypted-copy conditions; Q10 names every place the removed ID appears), and two waived by the rule (the carve-out is scoped to CFG-ENV-013 only, an owner question about its siblings; the scenario sentence CFG-ENV-011 stays a MUST is right). One fix round, re-read clean with one NOTE waived. Stored decisions: eight still hold and are carried, one no longer fires (the header decision, discharged by 020), 91 name files outside this scope and are retained. Not examined: the fetched sources behind CFG-ENV-013 Source, settled for this round.

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

- AC1 — met: framework/templates/spec holds spec, plan, tasks, data-model, research, scenario (spec-and-plan absent); grep finds no title: in any
- AC2 — met: met on its annotation (delivered, since retired): framework/templates/spec/spec-and-plan.md is absent from the tree
- AC3 — met: framework/templates/spec/scenario.md frontmatter holds only section:, no spec-ref
- AC4 — met: no tags instruction in framework/templates, commands or reference; constitution names tags only as a stale field (text-first-artifacts), no Starter Tag Vocabulary; runtime/src/primitives/pipeline_view.rs still reads it as annotated
- AC5 — met: grep for the [simple] marker across framework/ finds nothing, and the constitution cost-levers section has no such bullet
- AC6 — met: no promote-to-rule anywhere under framework/ (commands/groom.md, reference/groom.md included)
- AC7 — met: met on its annotation (delivered, then superseded by 041): framework/templates/spec/plan.md now carries the optional ## Open Questions Resolved design-record section
- AC8 — met: met on its annotation: AGENTS.md has no mirror-constitutions instruction; its generator line is the later run-before-invoking gotcha for scripts/gen-claude-commands.sh
- AC9 — met: runtime/src/primitives/create_scenario.rs writes only section: and derives its H1 from the slug (title_from_slug), and create_feature.rs derives the slug from its argument, so no title: is written or prompted
- AC10 — met: met on its annotation (since retired): scripts/gen-readme-table.sh is absent and README.md carries no Feature Specs marker or table
- AC11 — met: driven: bash scripts/gen-help-tables.sh --dry-run reports No changes (16 command(s) in sync) exit 0; framework/commands/help.md carries marker pairs (commands-pipeline etc.)
- AC12 — met: met on its annotation: .githooks/pre-commit runs gen-configure-mcp, gen-claude-commands, gen-help-tables and derive-dependencies/derive-references --write and halts on an unreachable runtime; scripts/install-hooks.sh sets core.hooksPath; scripts/tests/test-pre-commit-staging.sh driven, all PASS
- AC13 — met: met on its annotation (superseded in part by Q7): clarify.md step 4, plan.md sub-step 1.1, implement.md sub-step 1.1, amend.md step 3 and target.md step 5 invoke report-only derive-dependencies (no --write)
- AC14 — met: framework/commands/implement.md step 2 invokes derive-boundary (runtime/src/primitives/derive_boundary.rs; cargo test --lib derive_boundary 10 passed); framework/templates/spec/plan.md marks Affected Files Planning aid only
- AC15 — met: no checkbox fix or Fix Mode heading in framework/commands/analyze.md or framework/reference/analyze.md; the surviving --fix is the guarded done-spec status revert, not checkbox state
- AC16 — met: root constitution.md absent; CLAUDE.md holds @framework/constitution.md; no root-constitution generator under scripts/
- AC17 — met: clarify.md step 8 lists sibling-linked specs; plan.md sub-step 1.1 with reference/plan.md Cross-spec impact check; implement.md sub-step 1.1 and step 10 diff-cross-spec surface files outside the spec dir
- AC18 — met: framework/rules/configuration-cross.md holds 19 rules each with a Verification (scripted check), covering the listed topics; framework/constitution.md constants and env-vars sections are one-line pointers to it
- AC19 — met: driven: ductus validate-frontmatter on specs 000 to 019 spec.md returned clean true, 0 findings each; framework/reference/analyze.md raises no title/tags/spec-ref finding; the full analyze run was not re-driven
- AC20 — met: specs/017-derive-dont-ask/spec.md frontmatter carries status, dependencies, next-criterion and retired-rules, no title: or tags:
- AC21 — met: met on its annotation: framework/bootstrap/hooks/pre-commit ships, install.sh is absent, ductus-procedure.md Hook Installation exists, stub calls ductus-pre-commit
- AC22 — met: met as annotated: ductus.md manifest has hooks/pre-commit as create and hooks/ductus-pre-commit as update; ductus-procedure.md Hook Installation holds the detection ladder
- AC23 — met: met on its supersession annotation: .ductus/scripts/ is absent and derive-dependencies/derive-references run as runtime primitives (runtime/tests/derive_dependencies_golden.rs)
- AC24 — met: met on its annotation: .github/workflows/generators.yml runs the generators and both derivations with --write then fails on git status --porcelain; framework/templates/ci/adopter-generators.yml downloads a sha256-verified release binary and does the same
- AC25 — met: met on its annotation: framework/reference/review.md Observations routes an observation to fix, defer or waive with an undispositioned state, and no review step writes the inbox
- AC26 — met: framework/reference/groom.md Groom decision tree is also walked by specify.md step 2, reference/analyze.md, reference/review.md and reference/implement.md disposition handling, not only by groom
- AC27 — met: configuration-cross.md header declares FLAG; CFG-FLAG-001 MUST, CFG-FLAG-002 SHOULD, CFG-ENV-008 MUST, 009 SHOULD, 010 MUST, 011 MUST (still MUST at HEAD), 012 SHOULD each carry Statement, Verification and Source; data-model.md registers FLAG and enumerates all 19 rules with matching tiers (scripted check)
- AC28 — met: CFG-ENV-013 is MUST with Statement, Verification and Source and the throwaway carve-out leaves the case it owns intact; CFG-ENV-009/010 and security-backend.md BE-DATA-003, BE-DATA-008 and BE-DATA-012 each name CFG-ENV-013 (verified per rule body); data-model.md Rule set lists it as MUST

## Observations

- other: `specs/017-derive-dont-ask/data-model.md` Rule set enumerated `CFG-ENV-013` without the 2026-11-19 throwaway-credential carve-out the Statement now carries; the bullet and the preface now state it. — `specs/017-derive-dont-ask/data-model.md` — **fixed** (decided by the rule)
- other: `CFG-ENV-013`'s new Verification clause said a copy committed encrypted owes every condition, attaching the permission's conditions to a throwaway credential that is not a secret; reworded to say a secret committed encrypted owes them, the owner's as-strict-as-before intent, with the `[Unreleased]` entry in `runtime/CHANGELOG.md` saying so. — `framework/rules/configuration-cross.md` — **fixed** (decided by the rule)
- other: Q10 in `specs/017-derive-dont-ask/spec.md` said only its own entry is why `retired-rules:` lists `BE-DEPS-006`, but task 47 and the scenario `the-adopter-ci-template-can-run` name the ID too; the clause now names all three. — `specs/017-derive-dont-ask/spec.md` — **fixed** (decided by the rule)
- other: the throwaway-credential carve-out is scoped to `CFG-ENV-013` only, so `CFG-ENV-001` (an application default), `CFG-ENV-010` (a client artifact) and `BE-DATA-003`, `BE-DATA-008` and `BE-DATA-012` in `framework/rules/security-backend.md` keep the unqualified word secret, and `CFG-ENV-009`'s Not-flagged routes a compiled-in credential to `CFG-ENV-013`; whether the decision reaches those siblings is a new owner decision, and no analysis result flips to a finding. — `framework/rules/configuration-cross.md` — **waived**: the owner's decision of 2026-11-19 is scoped to CFG-ENV-013 and the siblings flag different placements; extending the carve-out is a separate decision owned by 017 for the CFG-ENV rules and 008 for the BE-DATA rules, so it goes to the owner through the parent, not through this review (decided by the rule)
- other: the fix-round re-read noted that the scenario `a-throwaway-credential-is-not-a-secret` says `CFG-ENV-011` stays a MUST, where `CFG-ENV-013` is evidently meant. — `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md` — **waived**: the sentence is right: task 47's third item records that CFG-ENV-011 stays a MUST, an owner decision of 2026-11-19, and the scenario's second bullet says so beside the encrypted-copy permission (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/rules/accessibility-frontend.md`
- `framework/rules/api-backend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/reliability-backend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-cross.md`
- `framework/rules/security-frontend.md`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `scripts/audit/README.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/038-concurrency-backend-rules/scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/scenarios/expendable-messages-may-be-dropped-when-recorded.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/069-accessibility-frontend-rules/scenarios/verifications-flag-only-what-the-statement-covers.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`

## Unexamined governance

*None.*
