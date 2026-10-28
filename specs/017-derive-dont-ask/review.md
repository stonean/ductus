---
spec: 017-derive-dont-ask
last-run: 2026-10-28T20:22:24Z
reviewed-against: c1cf51ffa655635ef1b56b977023762f596c1aaa
diff-base: af99793c0593bbf1e7915b7ee96e131e3d558750
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 26
scope: 165
skipped-passes: []
reviewed-digest:
  data-model.md: 1564ce26c444116e8378804cdfb3182671007492a51e8ad0291576cbc14bf05d
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 1bd156f5abd11ebb6c854241092e5c428cec4d3601f2a9fb4c4545c997aa1fc9
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 873eeab49a775f68b5283737ad21952b98e771c5409c018df418f573b89b2041
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: 2d197440840c8229dc3dda3f93df9ebd73f2af5b491b9991db2c070b1f5b6c4c
  scenarios/tracked-specs-not-worktree.md: e3808ab1ae9a4d6b1b2103c85537c6236e99898b7569536eb9ac2bfcdde684bf
blocking: false
dispositions:
  fixed: 76
  routed: 4
  discarded: 27
  undispositioned: 0
decisions:
  - key: "other: CFG-ENV-001 does not say whether an empty value on an optional value with a default is absent or malformed — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: predates the widening for environment variables and is unchanged by it; which way an empty optional value falls is a new decision, and CFG-ENV-008 already fails a present malformed value
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: \"at startup\" in CFG-ENV-003, CFG-ENV-005 and CFG-ENV-008 does not cover a configuration file a running process reads again — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: a value first read later is validated when it is read, the reading 063 applies on every call, and no spec here reloads a file mid-process
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-008's error names the setting and the value but not the file, which CFG-ENV-003 and CFG-ENV-005 now name for a file key — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: naming the file is a new requirement on a rule today's answers did not widen; the setting and the value already locate the defect
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005's error names the offending value with no redaction exception like CFG-ENV-008's — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: CFG-ENV-006's MUST redaction at the config-loading layer still applies, so both rules are met together
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-CONST-004 places a value's range bounds alongside its default constant, which a required value has none of — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: predates the widening for required environment variables; where a required value's bounds live is a new decision, and the bounds must still be named constants
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 names CFG-ENV-003 and CFG-ENV-008 as owning configuration, not CFG-ENV-005 — `framework/rules/quality-cross.md`"
    outcome: discarded
    reason: QUAL-GROUND-001 cedes all configuration to the CFG rules, so the omission changes no finding; the two it names are the presence and type rules its scope decision cited
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-011's Source pairs IEC 80000-13 with a NIST page that credits the binary prefixes to IEC 60027-2 — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: BIPM's VIM attributes the binary prefixes to IEC 80000-13 and NIST to its predecessor IEC 60027-2; together the two citations are accurate
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "security: FE-STORAGE-003 does not repeat BE-API-013's WebSocket-handshake exclusion — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: "its MAY requires a landing page that sets Referrer-Policy: no-referrer, which a WebSocket handshake has no page to do, so a handshake ticket already falls under its MUST NOT; the owner's answer placed the narrowing in BE-API-013 and BE-API-015"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015's handshake-URL MUST NOT also bars a CSRF token there, which the OWASP WebSocket Security Cheat Sheet suggests sending in the handshake — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the owner's 2026-10-20 answer keeps every token out of the handshake URL, following the same cheat sheet's advice against tokens in URL query strings; (a)'s Origin allowlist is the handshake's cross-site defense, and a CSRF token can still be checked in the first message
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: the pre-signed-URL MAY covers object storage only, so another signed-URL scheme, a CDN's or an application's own HMAC-signed link, falls under BE-API-013 and FE-STORAGE-003 — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the owner's 2026-10-20 answer scoped the MAY to a pre-signed URL for one object and one method; whether other signed-URL schemes fit that scope is a new decision
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-007's Verification asks for legitimate-recovery paths during lockout, which its Statement does not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-001's Statement requires frame protection on all HTTP responses while its Verification table sets frame-ancestors on HTML responses only — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: predates this window; OWASP's HTTP Headers Cheat Sheet, which BE-API-001 cites, says X-Frame-Options gives no security on a redirect or a JSON API response, while its REST Security Cheat Sheet recommends frame-ancestors 'none' on API responses, so aligning the two decides what the rule requires
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-013's handshake sentence says BE-API-015 keeps every token out of the handshake URL, while BE-API-015's Statement covers only endpoints serving non-public data or accepting state-changing messages — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence is the narrowing the owner's 2026-10-20 answer placed in both rules and matches BE-API-015's unscoped Verification; a token in a handshake URL authenticates the connection, so its endpoint is not public in practice; scoping it made the two rules disagree and was reverted
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-018's newer-token invalidation would cancel a code issued by one method when a code by another is issued in the same recovery — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: a recovery that verifies the first code before issuing the second complies, since the first is already used; NIST SP 800-63B-4 sets no invalidation across codes and nothing in BE-AUTHN-018 forbids that order
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 008's ticked task rows 13, 14 and 15 describe FE-STORAGE-003, FE-DEPS-004 and BE-AUTHN-022 as they read before later edits — `specs/008-security-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005, widened to every source, does not say whether a value read from several sources is range-checked as resolved or at each source — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: the value validated at startup is the one CFG-ENV-007's resolution yields, the reading CFG-ENV-003 states for presence, and no spec here reads a ranged value from two sources
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "reliability: dropping a timed-out client joins its runtime thread, so a slow connect-time lookup can hold a fetch past FETCH_TIMEOUT — `runtime/src/primitives/fetch_archive.rs`"
    outcome: discarded
    reason: the GaiResolver it replaced behaves the same way, and in production the lookup is getaddrinfo, which the operating system resolver's own timeouts bound
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "reliability: the initial and redirect-screen lookups run outside the client's timeout — `runtime/src/primitives/fetch_archive.rs`"
    outcome: discarded
    reason: unchanged by this fix and bounded the same way, by the operating system resolver's timeouts; FETCH_DEADLINE bounds the download itself
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "security: a redirect hop is looked up twice rather than connected to the address its screen resolved — `runtime/src/primitives/fetch_archive.rs`"
    outcome: discarded
    reason: 022's scenario allows an equivalent connect-time re-check, both lookups are screened, and the owner prescribed this resolver and its staged-rebind test
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: two proxy tests each define the same four-line internal-address lookup — `runtime/src/primitives/fetch_archive.rs`"
    outcome: discarded
    reason: test fixtures local to each test; sharing one would couple the tests for no gain
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 025's ticked task 2 lists a malformed warning among the step-5 outcomes that task 10 replaced — `specs/025-rule-opt-out/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: discarded
    reason: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: routed
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: routed
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: discarded
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: discarded
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs`"
    outcome: discarded
    reason: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs`"
    outcome: discarded
    reason: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
    decided-at: 2026-10-20T16:20:20Z
    decided-by: andy@stone.dev
  - key: "other: pre-existing downloads with no named timeout, unchanged in this window: runtime-acquisition.yml:131-132, runtime-release.yml:623 (also retried with a fixed delay and no jitter) and install.sh:124 and :163 — `.github/workflows/runtime-release.yml`"
    outcome: routed
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml`"
    outcome: discarded
    reason: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "bug: a pinned pre-release would read as current once its stable release ships, since sort -V orders 1.0.0 before 1.0.0-rc.1 — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: No pin in this tree is a pre-release, and the values it is compared with are crates.io's newest stable version and published release tags, so no input reaches the misordering.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: DUCTUS_SHA256 pairs with a DUCTUS_VERSION by file rather than by step, a tab split collapses an empty version field, and cargo install's --version=X, --vers and name@X forms are not parsed — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: Each fails loud, as a hash mismatch or an unpinned report, and the tree has one install step and only the `--version X` form.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: an unknown check name exits 2, the code that also means a check could not run — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: A misspelt argument is a run that examined nothing, which is what exit 2 states, and the message names the cause.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: 022's plan states a provenance exception for crates (BE-DEPS-004) but none for the pip-installed ziglang — `specs/022-deterministic-runtime/plan.md`"
    outcome: discarded
    reason: 022's own analysis assesses BE-DEPS-004 against 022's plan, which names the ziglang install; it is outside 017's scope.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: the first weekly pins run will exit 1 on three action pins a major release behind, the template's checkout, upload-artifact and download-artifact — `.github/workflows/runtime-currency.yml`"
    outcome: discarded
    reason: Expected and recorded in 022's task 3 measurement; moving those majors is outside task 3 and goes to the owner as its own question.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: the template's version check is close to redundant, since the sha256 pin already fixes the bytes — `framework/templates/ci/adopter-generators.yml`"
    outcome: discarded
    reason: Scenario the-adopter-ci-template-can-run requires it, and it catches a mispackaged release the hash cannot.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: the pins check's .yaml globs match no tracked file — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: A guard, so a workflow added as .yaml is not skipped silently.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: pass() and fail() are copied across the four scripts/tests/*.sh — `scripts/tests/test-check-runtime-currency.sh`"
    outcome: discarded
    reason: There is no shared test library to reuse, and adding one for two lines is not this change's work.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml`"
    outcome: discarded
    reason: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: \"shipped\" still describes the CI template in 017's AC24, plan.md's Affected Files row and 022's task 3 text — `specs/017-derive-dont-ask/spec.md`"
    outcome: discarded
    reason: AC24 calls it a shipped example, the Affected Files row says adopters copy it, and 022's task 3 text is a spent task; none claims it is installed.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: cargo's own requests in checks 1, 2 and 4 keep cargo's defaults rather than a timeout the script names — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: Pre-existing checks, unchanged in this window; runtime-currency.yml's timeout-minutes bounds the whole CI run.
    decided-at: 2026-10-20T19:37:31Z
    decided-by: andy@stone.dev
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: discarded
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: discarded
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the implement reference's \"non-blocking\" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md`"
    outcome: discarded
    reason: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md`"
    outcome: discarded
    reason: "pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: discarded
    reason: the Context records the state when the scenario was found, and the close is loose, not false
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md`"
    outcome: discarded
    reason: "contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "convention: a-status-commit-holds-only-the-transition's Behavior reads as requiring the bullet to say a transition staged alone is lost — `specs/050-constitution/scenarios/a-status-commit-holds-only-the-transition.md`"
    outcome: discarded
    reason: the Behavior requires the bullet to state its condition, that the spec file holds nothing but the transition, and its reason, that the pre-commit hook restages the file whole; the bullet states both, and the loss of a transition staged alone is the scenario's own account of the incident
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: claude-md-import-syntax's shared-import bullet contrasts its rewrite with a removal the procedure no longer performs — `framework/migrations/claude-md-import-syntax.md`"
    outcome: discarded
    reason: the bullet still guards against de-duplicating an import the adopter added by hand, which removal would drop if the registration were withdrawn; its sense does not depend on the constitution line
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 017's plan and 072's Affected Files still name the old import form — `specs/017-derive-dont-ask/plan.md`"
    outcome: discarded
    reason: each records that spec's own edit at the time, a decision record, not a claim about the tree today
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: a CLAUDE.md symlinked to AGENTS.md becomes a copy when constitution-import writes it by temporary file and rename — `framework/migrations/constitution-import.md`"
    outcome: discarded
    reason: every bootstrap write of CLAUDE.md already does this, merge-managed-block's managed-block render on every run included, so the migration adds no new case; a symlinked CLAUDE.md is not a layout the framework supports
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 022's constitution-excerpts-as-skill-resources says every CLAUDE.md carries the constitution import — `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`"
    outcome: discarded
    reason: this change makes the premise true again, the template's third line being @.ductus/constitution.md; 022 is another session's and owes no edit
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: discarded
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: constitution-import accepts a four-space-indented line, misses a match behind a leading byte-order mark, and reports the @import form on a file unpinned after claude-md-import-syntax was skipped — `framework/migrations/constitution-import.md`"
    outcome: discarded
    reason: "contrived inputs: claude-md-import-syntax treats indentation the same way, the template writes no byte-order mark, and an operator who unpins a file has the earlier migration's pinned warning in hand"
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "convention: a #fragment containing a dot is cut at the dot on a fold into a scenario — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: GitHub's heading slugs carry no dots, so such a fragment names no heading a link in this corpus can reach; a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: whether Auggie resolves CLAUDE.md's @AGENTS.md import, and so reads the routing block, is unverified — `framework/templates/project/claude-md.md`"
    outcome: discarded
    reason: the rule's canonical statement is the constitution's drift-prevention section, which target step 4 loads whenever an import did not, so an Auggie session holds it either way
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "bug: a run the Frontmatter Migration precheck halts after generator-primitives leaves a pre-0.30 hook calling the scripts that migration deleted, so the commit the refusal asks for fails — `framework/bootstrap/ductus-procedure.md`"
    outcome: routed
    target: specs/013-text-first-artifacts/scenarios/the-precheck-ignores-what-the-same-run-wrote.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: generator-primitives and audit-record-relocate state no report line for their no-op cases — `framework/migrations/generator-primitives.md`"
    outcome: discarded
    reason: an observation about the migration machinery outside this window; the host's own wording reported each outcome truthfully
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fetch-archive sends no User-Agent — `runtime/src/primitives/fetch_archive.rs`"
    outcome: routed
    target: specs/048-govern-acquired-runtime/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a .githooks linked into a host configuration directory inside the project is not judged by host_config::check_destination — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: a contrived layout; the write stays inside the project, and the referrer list is fixed to five files none of which is a host's configuration
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fold's resumption row for step 11 says a re-run rewrites nothing, which a fragment running into a second token contradicts — `framework/commands/fold.md`"
    outcome: discarded
    reason: "a contrived input: a fragment that is itself a second spec-root path, which the rewrite now leaves for the orphan check to report"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: the spec-root walk drops a directory it cannot list, so it is neither counted nor skipped — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: a directory the process cannot list inside a repository it writes is contrived, and the walk's filter predates this window
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a root-absolute /specs/ link is neither re-pointed nor reported — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: routed
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an archive entry whose only mode bits are group or other write lands at the umask default — `runtime/src/primitives/extract_archive.rs`"
    outcome: discarded
    reason: "a contrived entry: no archive the framework ships carries a file with neither owner read nor owner write"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: legacy-constitutions-region was edited in place after a --ref=main push, so an adopter who ran the earlier text keeps its trailing blank lines — `framework/migrations/legacy-constitutions-region.md`"
    outcome: discarded
    reason: the migration is unreleased, reached only --ref=main adopters for the hours between f056b209 and 47e68233, and the residue is trailing blank lines after a removed comment region, which change no behavior
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: 062 task 23's checked item still gives the fixed-interval rationale for the lock poll — `specs/062-concurrent-session-targets/tasks.md`"
    outcome: discarded
    reason: a ticked task records the decision as it was taken, task 28 records its reversal, and tasks.md is ephemeral work tracking that prune may drop
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: installed-file-modes M5's kill depends on ctime resolution finer than 50 ms — `scripts/mutation/installed-file-modes.py`"
    outcome: discarded
    reason: disclosed in the list's docstring; APFS and ext4 stamp ctime in nanoseconds, and the test itself passes on any file system
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an @/specs/x import in CLAUDE.md is read as project-relative, where Claude Code reads an absolute path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: an absolute import of a top-level /specs directory is a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a kept-tails entry gives the reference from the spec root, without a prefix kept on the line — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: its path and line locate the reference, and the check reports the same span from the root
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a ./ prefix in the hook or a root markdown file assumes the project root is the work-tree root, which a project in a repository subdirectory breaks — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: discarded
    reason: a ./ resolves as the bare paths in those files already do, the assumption every bare reference carries, not one task 30 adds
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: the Family 19 parity tests run git in their fixtures under a hook's inherited GIT_DIR, so a commit from a linked worktree fails four of them — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: routed
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a leading / was read from the project root in a project below its repository's root, where a host resolves it from the repository's root — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the framework reads a root-absolute path from the project root, as check-corpus-links reads a root-absolute link (spec 059); reading it from the repository's root here alone split the fold's link and token passes, so 3fc5755a's gate was reverted
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root, /proj/specs/x, the project's own path spelled from the repository's root, is neither checked nor re-pointed — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: a project's README spelling its own spec from the repository's root is a contrived input, and the project-root reading names it nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root the hook runs from the repository's root, so its ./ and bare paths resolve there — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: discarded
    reason: the same assumption every bare reference in the hook carries, which task 30 does not add; prefixed and bare agree
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: bare-rule cases outside task 30 — ~/\".ductus/x\", \"$X\"specs/x and ${X}specs/x in the hook, ${X:-./specs/x}, *specs/x*, and @/specs/x in CLAUDE.md — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the bare rule predates task 30 and prefixed and bare agree on each; every input is contrived
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: the leading-/ exception also covers (a)/specs/x, x~/specs/x and a quote after a letter — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: contrived text; requiring $( before ) would add a parser for no real input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: no agreement test puts a path at column 0 in system.md, so a mutant refusing a line-start path only there survives, in the check's tests and the rewrite's — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the scanner has no per-referrer branch, so a defect of that shape is contrived; the binary reports a bare, ../ and / path at column 0 in system.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: the subdirectory test asserts that a spec only the repository's root holds is reported, not that one only the project holds is not — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the realistic regression, reading existence from the repository's root, is killed by M46; skipping only a path present at both roots is contrived, and the binary does not report the project-only spec
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: two ticked items in 017's tasks.md name a rule's Rationale, which 70bb4af5 removed from the format — `specs/017-derive-dont-ask/tasks.md`"
    outcome: discarded
    reason: tasks.md is ephemeral work tracking whose ticked items record what was built then (§tasks-phase); the format's canonical statement, 017's data-model.md, is current
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
---

# Review — 017-derive-dont-ask

## Summary

One in-depth review of the window af99793c..c1cf51ff, recorded in each spec it touched. An independent five-pass review read af99793c..fb26d602 (3b359742, 55220b20, 5471eb02, fb26d602) in full against the eleven rule files discover-rule-files selects, and its findings were fixed in f056b209. Independent re-reads followed each fix round that changed code or a durable contract, each re-running the earlier probes: of f056b209 (5 SHOULD, 11 NOTE, fixed in 47e68233); of 47e68233 (5 SHOULD, 9 NOTE, fixed in 77fff147 beside f33096a5's analysis-round fixes); of f33096a5, 77fff147, 38fc39f6 (051 task 30) and 4e82686d with db6ae819 (050 task 41), 10 SHOULD and 8 NOTE, fixed in 54715126; of 54715126 (1 SHOULD, fixed in 3fc5755a); of 3fc5755a (3 SHOULD, fixed in d1853444); of d1853444 (2 SHOULD, both test gaps, fixed in c1cf51ff); and of c1cf51ff, 0 MUST, 0 SHOULD and 4 NOTE: the two comment sentences it left unclear or incomplete were fixed in a prose-only round that was not re-read, and its two test gaps were discarded as contrived. The 107 observations below are those findings and the reviewers' notes: 76 fixed, 4 routed, 27 discarded with their reasons. Mutation lists for the window's runtime changes ran whole and every mutation was killed. Examined counts the 26 in-scope files the passes read in full, of 165 in scope. Not counted: files read in part, among them the large runtime modules' test sections and the 021, 062 and 066 analysis records; the generated .claude/, .pi/ and .ductus/ copies, checked only against their sources; another session's rules sweep, 70bb4af5, inside this window, whose files the passes read only where it touched this spec: its hunks to spec.md and plan.md with their context, checked against configuration-cross.md at HEAD (data-model.md, read in full, is counted); and plan Affected Files the window did not touch. Stored decisions are carried byte-identical through process-decisions' restricted mode: this window's passes did not re-read the artifacts they concern.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: a referrer that is not UTF-8 text stopped rewrite-spec-links after the spec-root walk had written files, where check-orphaned-references skips the same file — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- bug: a referrer that is a symbolic link was skipped with nothing in the result naming it (QUAL-CLAIM-001) — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- bug: a referrer reached through a directory linked outside the project was rewritten there — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- bug: a reference re-pointed at a feature-only target kept a tail such as plan.md and landed on the target's file of that name, which the fold never wrote, while the orphan check stayed clean, and fold step 11 said inbound links land on the file the content reached — `framework/commands/fold.md` — **fixed**
- bug: a referrer line whose first token's fragment ran into a second token panicked rewrite-spec-links after the spec root was written — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- bug: the ductus:memory-routing block no longer said the framework's constitution is upstream, so an agent could file a learning into .ductus/constitution.md, which the next run replaces — `framework/bootstrap/ductus.md` — **fixed**
- stale: knowledge-routes-by-population-not-by-kind said the corrected routing reaches new adoptions only, and named four places where its table lists two — `specs/050-constitution/scenarios/knowledge-routes-by-population-not-by-kind.md` — **fixed**
- stale: 051's plan said the rewrite is scoped to markdown under the spec root, though it now rewrites the root referrers too — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- convention: a #fragment containing a dot is cut at the dot on a fold into a scenario — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: GitHub's heading slugs carry no dots, so such a fragment names no heading a link in this corpus can reach; a contrived input
- stale: 050's plan recorded the window base's constitution character counts and a total matching neither commit — `specs/050-constitution/plan.md` — **fixed**
- stale: memory-routing-lives-in-agents-md described the migration's match as byte-identical and its bound as the next second-level heading — `specs/050-constitution/scenarios/memory-routing-lives-in-agents-md.md` — **fixed**
- convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md` — **discarded**: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
- convention: whether Auggie resolves CLAUDE.md's @AGENTS.md import, and so reads the routing block, is unverified — `framework/templates/project/claude-md.md` — **discarded**: the rule's canonical statement is the constitution's drift-prevention section, which target step 4 loads whenever an import did not, so an Auggie session holds it either way
- bug: 3b359742's mutations were never registered, so its 11-of-11 claim could not be re-run — `scripts/mutation/rewrite-spec-links.py` — **fixed**
- bug: memory-routing-leaves-claude-md ran the section to the end of the file, so a ductus:constitutions block an earlier run appended after it made every shipped section read as edited — `framework/migrations/memory-routing-leaves-claude-md.md` — **fixed**
- bug: every file apply-manifest installs landed group-writable, the pre-commit hook and Pi's bridge among them, because extract-archive kept the tarball's 0664 and 0775 — `runtime/src/primitives/extract_archive.rs` — **fixed**
- bug: adopters from v0.47.0 to v0.59.11 keep a ductus:constitutions comment region whose comment says ductus maintains it, beside the block ductus does maintain — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- bug: a run the Frontmatter Migration precheck halts after generator-primitives leaves a pre-0.30 hook calling the scripts that migration deleted, so the commit the refusal asks for fails — `framework/bootstrap/ductus-procedure.md` — **routed** to `specs/013-text-first-artifacts/scenarios/the-precheck-ignores-what-the-same-run-wrote.md`
- convention: the bootstrap's memory-routing step left the host to tell an append from a fill — `framework/bootstrap/ductus.md` — **fixed**
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **discarded**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: generator-primitives and audit-record-relocate state no report line for their no-op cases — `framework/migrations/generator-primitives.md` — **discarded**: an observation about the migration machinery outside this window; the host's own wording reported each outcome truthfully
- convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md` — **discarded**: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
- convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md` — **discarded**: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **discarded**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md` — **discarded**: the template has frontmatter, so the walk skips it with a report line and writes nothing
- convention: fetch-archive sends no User-Agent — `runtime/src/primitives/fetch_archive.rs` — **routed** to `specs/048-govern-acquired-runtime/tasks.md`
- convention: a .githooks linked into a host configuration directory inside the project is not judged by host_config::check_destination — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: a contrived layout; the write stays inside the project, and the referrer list is fixed to five files none of which is a host's configuration
- bug: a scenario-route fold re-pointed a reference to one of the staged spec's carried scenarios at the routed scenario, listing nothing — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- stale: 051 AC22 said the orphan check reports clean afterwards save for skipped referrers, though a kept tail naming a file the target lacks is reported — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed**
- stale: 051's data-model said skipped holds referrers outside the spec root, though system.md is listed too — `specs/051-branch-scoped-spec-numbering/data-model.md` — **fixed**
- stale: legacy-constitutions-region's report and Background said the BEGIN/END block carries the imports in AGENTS.md, which a claude-only project never gets — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- bug: the mode fix reached only files a run rewrote, since apply-manifest returned unchanged before re-applying the mode, so an installed hook stayed group-writable — `runtime/src/primitives/apply_manifest.rs` — **fixed**
- bug: a carried scenario whose slug was already taken upstream was reported and the fold went on, so step 11 re-pointed its references at an unrelated scenario and step 12 deleted the only copy — `framework/commands/fold.md` — **fixed**
- convention: fold's resumption row for step 11 says a re-run rewrites nothing, which a fragment running into a second token contradicts — `framework/commands/fold.md` — **discarded**: a contrived input: a fragment that is itself a second spec-root path, which the rewrite now leaves for the orphan check to report
- edge: legacy-constitutions-region did not decide a region followed only by blank lines — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- stale: a rewrite-spec-links test doc called a CLAUDE.md symlinked to AGENTS.md a common layout — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- stale: fold step 14 said the orphan check reports what a skipped referrer still names, untrue of a file that is not UTF-8 — `framework/commands/fold.md` — **fixed**
- stale: installed-files-are-not-group-writable described the old rwxrwxrwx mask in the present tense — `specs/022-deterministic-runtime/scenarios/installed-files-are-not-group-writable.md` — **fixed**
- test: the read_referrers arm for a referrer behind a path component that is a regular file had no test — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- edge: the spec-root walk drops a directory it cannot list, so it is neither counted nor skipped — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: a directory the process cannot list inside a repository it writes is contrived, and the walk's filter predates this window
- edge: a root-absolute /specs/ link is neither re-pointed nor reported — `runtime/src/primitives/check_orphaned_references.rs` — **routed** to `specs/051-branch-scoped-spec-numbering/tasks.md`
- edge: an archive entry whose only mode bits are group or other write lands at the umask default — `runtime/src/primitives/extract_archive.rs` — **discarded**: a contrived entry: no archive the framework ships carries a file with neither owner read nor owner write
- stale: 050's plan dated its character figures to the last commit to change the three files, which the next AGENTS.md edit ends — `specs/050-constitution/plan.md` — **fixed**
- bug: apply-manifest's re-applied mode was fatal on a file the runner may not chmod, stopping the walk mid-manifest — `runtime/src/primitives/apply_manifest.rs` — **fixed**
- bug: fold step 7's recovery, renaming one of two colliding scenarios, re-pointed the old slug's links silently at the unrelated upstream scenario — `framework/commands/fold.md` — **fixed**
- stale: the rewrite-spec-links MCP tool description said a scenario target replaces every tail — `runtime/src/mcp/server.rs` — **fixed**
- stale: 051 AC22's kept-tails clause held only for the root referrers, and AC6 named only the skipped exception — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed**
- bug: the session-lock poll and 062's plan stated no maximum attempt count, which BE-RETRY-001's Statement asks for beside the deadline — `runtime/src/session.rs` — **fixed**
- stale: scope-results-have-a-bounded-size credited the review reference with the redirect instruction the review command's step 1 gives — `specs/066-audit-records-and-the-completion-gate/scenarios/scope-results-have-a-bounded-size.md` — **fixed**
- stale: session-lock-backoff.py claimed mutation coverage of the WouldBlock-only retry it has no mutation for — `scripts/mutation/session-lock-backoff.py` — **fixed**
- stale: 071 task 23's evidence named the 12-case walk after the 15-case re-walk — `specs/071-command-reference-split/tasks.md` — **fixed**
- edge: legacy-constitutions-region was edited in place after a --ref=main push, so an adopter who ran the earlier text keeps its trailing blank lines — `framework/migrations/legacy-constitutions-region.md` — **discarded**: the migration is unreleased, reached only --ref=main adopters for the hours between f056b209 and 47e68233, and the residue is trailing blank lines after a removed comment region, which change no behavior
- convention: legacy-constitutions-region step 7 said the blank lines before the start line, meaning the adjacent run — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- stale: legacy-constitutions-region's Background said a file no selected agent reads keeps no block, untrue after another agent's run wrote one — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- stale: 062 task 23's checked item still gives the fixed-interval rationale for the lock poll — `specs/062-concurrent-session-targets/tasks.md` — **discarded**: a ticked task records the decision as it was taken, task 28 records its reversal, and tasks.md is ephemeral work tracking that prune may drop
- stale: rewrite_spec_links's panic-guard comment said a scenario target always drops the fragment — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- test: installed-file-modes M5's kill depends on ctime resolution finer than 50 ms — `scripts/mutation/installed-file-modes.py` — **discarded**: disclosed in the list's docstring; APFS and ext4 stamp ctime in nanoseconds, and the test itself passes on any file system
- stale: the rewrite-spec-links `to` argument doc said a scenario target lists nothing, which 77fff147's listing of an unheld scenario tail made false — `runtime/src/schema/primitives.rs` — **fixed**
- stale: the check-orphaned-references MCP description listed four referrers where the check reads five — `runtime/src/mcp/server.rs` — **fixed**
- bug: the CLI help gave check-corpus-links the orphan check's line and check-orphaned-references its Args doc, since a variant was inserted between that doc comment and its own — `runtime/src/main.rs` — **fixed**
- edge: a leading / after ~, } or % was read as naming the project root, so ~/.ductus/bin/ductus was checked and re-pointed as a project path — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- security: create-feature did not contain its derived directory, so a spec root linked outside the project received the new spec (BE-INPUT-004), though 051's plan said it was validated — `runtime/src/primitives/create_feature.rs` — **fixed**
- test: create-feature-claim M3 was declared a survivor reached only by a race, though a dangling link at the derived name reaches the lost claim every time (QUAL-TEST-001) — `scripts/mutation/create-feature-claim.py` — **fixed**
- stale: the cut sentence naming the checks the completion gate composes with is one 046's Resolved Question rests on, though the plan said no tracked file quoted a cut — `framework/constitution.md` — **fixed**
- stale: apply-manifest's Errors doc said an unchangeable mode is an Io error, where PermissionDenied names the file in modes-kept — `runtime/src/primitives/apply_manifest.rs` — **fixed**
- convention: the completion message's list had no modes-kept line, which bootstrap step 4 promises — `framework/bootstrap/ductus-procedure.md` — **fixed**
- stale: five shipped statements said every tail but a held scenario's collapses on a scenario target, where any scenarios/{name} tail is kept — `framework/reference/fold.md` — **fixed**
- stale: rewrite-spec-links' kept-tail comments named only the feature-target predicate — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- convention: a test's doc comment was split from its test by an inserted test — `runtime/src/primitives/apply_manifest.rs` — **fixed**
- stale: the revision-qualified-path scenario stated the token rule without the root-prefix exception — `specs/067-analyze-and-audit-checks/scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md` — **fixed**
- stale: create-feature's comment and 051's plan said concurrent creation under one identifier refuses the loser, which holds for one name only — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- edge: an @/specs/x import in CLAUDE.md is read as project-relative, where Claude Code reads an absolute path — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: an absolute import of a top-level /specs directory is a contrived input
- convention: a kept-tails entry gives the reference from the spec root, without a prefix kept on the line — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: its path and line locate the reference, and the check reports the same span from the root
- edge: a ./ prefix in the hook or a root markdown file assumes the project root is the work-tree root, which a project in a repository subdirectory breaks — `specs/067-analyze-and-audit-checks/data-model.md` — **discarded**: a ./ resolves as the bare paths in those files already do, the assumption every bare reference carries, not one task 30 adds
- stale: 062 task 28's ticked item still said its mutation run was owed — `specs/062-concurrent-session-targets/tasks.md` — **fixed**
- stale: the session lock's docs named only the timeout as a bound on the attempts — `runtime/src/session.rs` — **fixed**
- convention: 050's plan narrated the search behind the cut, verification evidence the commit holds — `specs/050-constitution/plan.md` — **fixed**
- stale: the analyze reference listed the managed roots without the historical .govern/, scripts/gen- and scripts/lib/ — `framework/reference/analyze.md` — **fixed**
- test: the Family 19 parity tests run git in their fixtures under a hook's inherited GIT_DIR, so a commit from a linked worktree fails four of them — `runtime/tests/mechanical_sweep_parity.rs` — **routed** to `specs/066-audit-records-and-the-completion-gate/tasks.md`
- edge: the round-3 fix's opener allowlist refused a prefix after |, >, & or a typographic quote, though the bare spelling is found, so a fold left such a reference dangling and the check silent — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- edge: a quoted or backticked variable before a prefix ("$HOME"/, `pwd`/) was read as the project root — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test: seven of the eleven opener characters had no test, and removing any one passed both modules — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- edge: a leading / was read from the project root in a project below its repository's root, where a host resolves it from the repository's root — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the framework reads a root-absolute path from the project root, as check-corpus-links reads a root-absolute link (spec 059); reading it from the repository's root here alone split the fold's link and token passes, so 3fc5755a's gate was reverted
- stale: 051 task 31's ticked items recorded the mutation lists before round 4 changed them — `specs/051-branch-scoped-spec-numbering/tasks.md` — **fixed**
- stale: 050's plan attributed the round-4 constitution figures to the tree that added the length rule — `specs/050-constitution/plan.md` — **fixed**
- edge: 3fc5755a's closing-quote rule read an opening backtick after emphasis or strikethrough as closing a variable, so **`./specs/x`** was missed though **`specs/x`** is found — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test: a prefix at the start of a line had no test, so the arms deciding it survived their mutations (QUAL-TEST-001) — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- stale: the shipped references, the module doc and the mutation list listed a leading / unconditionally while 3fc5755a dropped it below the repository root — `framework/reference/fold.md` — **fixed**
- edge: ~~./specs/x~~ and (a)./specs/x were missed though their bare spellings are found — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- edge: in a project below its repository's root, system.md's link pass re-pointed a leading / that its token pass left — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- edge: in a project below its repository's root, /proj/specs/x, the project's own path spelled from the repository's root, is neither checked nor re-pointed — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: a project's README spelling its own spec from the repository's root is a contrived input, and the project-root reading names it nothing
- test: two members of 3fc5755a's rule, the typographic closing quotes and the prefixed-only guard, had no killing test — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- stale: 051 task 31 did not record 3fc5755a's runs — `specs/051-branch-scoped-spec-numbering/tasks.md` — **fixed**
- test: the rewrite test for markup and shell forms carried only prefixed spellings, and the check's blockquote pair sat on two lines — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- edge: in a project below its repository's root the hook runs from the repository's root, so its ./ and bare paths resolve there — `specs/067-analyze-and-audit-checks/data-model.md` — **discarded**: the same assumption every bare reference in the hook carries, which task 30 does not add; prefixed and bare agree
- edge: bare-rule cases outside task 30 — ~/".ductus/x", "$X"specs/x and ${X}specs/x in the hook, ${X:-./specs/x}, *specs/x*, and @/specs/x in CLAUDE.md — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the bare rule predates task 30 and prefixed and bare agree on each; every input is contrived
- test: no test had a bare managed path at a line's start, so a mutant refusing only that survived (QUAL-TEST-001) — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test: the subdirectory test held its spec at neither root, so it could not tell a project-root reading from a repository-root one — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test: no fixture put % or a digit before a closing quote — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- edge: the leading-/ exception also covers (a)/specs/x, x~/specs/x and a quote after a letter — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: contrived text; requiring $( before ) would add a parser for no real input
- stale: the root-prefixed mutation list's docstring omitted two functions and two killing tests and misgrouped M12 — `scripts/mutation/root-prefixed-references.py` — **fixed**
- convention: a rewrite test's doc comment broke mid-phrase onto a one-word line — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- unclear: the root-prefixed mutation list's docstring could read as if a leading / ending another directory's name were a reference — `scripts/mutation/root-prefixed-references.py` — **fixed**
- stale: the agreement test's comment omitted the hook's line-start lines, among them the bare-first line the round added — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test gap: no agreement test puts a path at column 0 in system.md, so a mutant refusing a line-start path only there survives, in the check's tests and the rewrite's — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the scanner has no per-referrer branch, so a defect of that shape is contrived; the binary reports a bare, ../ and / path at column 0 in system.md
- test gap: the subdirectory test asserts that a spec only the repository's root holds is reported, not that one only the project holds is not — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the realistic regression, reading existence from the repository's root, is killed by M46; skipping only a path present at both roots is contrived, and the binary does not report the project-only spec
- stale: two ticked items in 017's tasks.md name a rule's Rationale, which 70bb4af5 removed from the format — `specs/017-derive-dont-ask/tasks.md` — **discarded**: tasks.md is ephemeral work tracking whose ticked items record what was built then (§tasks-phase); the format's canonical statement, 017's data-model.md, is current

## Skipped passes

*None.*

## Unexamined governance

*None.*
