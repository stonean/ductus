---
spec: 017-derive-dont-ask
last-run: 2026-10-20T19:37:31Z
reviewed-against: fa489f6ae4a08bb10b95fde274c72fd4363f6a35
diff-base: 37c8219c8730fd23b5fbac7629c11fd2d6bc5b3b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 19
scope: 184
skipped-passes: []
reviewed-digest:
  data-model.md: 59aa84e8bc06cb7526e20597cbeb672066cfbe1e8fa9e70a0f3db6f97b4a756b
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 1bd156f5abd11ebb6c854241092e5c428cec4d3601f2a9fb4c4545c997aa1fc9
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 873eeab49a775f68b5283737ad21952b98e771c5409c018df418f573b89b2041
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: 2d197440840c8229dc3dda3f93df9ebd73f2af5b491b9991db2c070b1f5b6c4c
  scenarios/tracked-specs-not-worktree.md: e3808ab1ae9a4d6b1b2103c85537c6236e99898b7569536eb9ac2bfcdde684bf
blocking: false
dispositions:
  fixed: 8
  routed: 3
  discarded: 53
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
---

# Review — 017-derive-dont-ask

## Summary

Re-review of 017 after 022 task 3's CI pinning, which 017's plan covers through `generators.yml` and the adopter CI template, and after the owner's answer (b) to the scenario `the-adopter-ci-template-can-run`. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking.

**Base.** `37c8219c`, the parent of the reopen (`3d7f20e9`). The newest record, `6855e5bf`, was reviewed against `8dd72c61`; this review reads what changed in 017's scope since `8dd72c61`.

**Read** (`examined: 19`). The 21 in-scope files changed since `8dd72c61`, less `review.md` and `analysis.md`, which are regenerated whole and were read only for their decisions and counts. Two independent passes at `b85e4907` (security and quality; reuse, efficiency, simplicity and prose accuracy) and an independent re-read of the first fix round at `02a5ac13` read in full `scripts/check-runtime-currency.sh`, `scripts/tests/test-check-runtime-currency.sh`, `framework/templates/ci/adopter-generators.yml`, the six workflows, `runtime/rust-toolchain.toml` and the new scenario. The host read 017's `spec.md` and `plan.md` in full, and the changed sections of `AGENTS.md`, `framework/migrations/generator-primitives.md`, 017's `tasks.md`, 022's `plan.md` and `tasks.md`, and `specs/inbox.md`. Every rule's Statement and Verification in the eleven rule files was applied, and the template's steps ran in a linux/amd64 container against two adopter-shaped fixtures. Not read in this run: the other 163 scope entries, unchanged since `8dd72c61`, and 017's `data-model.md` and five older scenarios, also unchanged.

**Fixed in the run.** BE-TIMEOUT-001 (MUST, twice): neither the template's download nor the pins check's lookups had a timeout. Both are now bounded by named values, the GitHub API goes through the same bounded `curl` in place of `gh api`, and the weekly job sets `timeout-minutes`. BE-RETRY-001: the template retried without jitter and no longer retries. A test gap: no fixture wrote a step-key `uses:`, and no case asserted a pin's location or check 3's comparison. Efficiency: a per-line subprocess loop (32 s) became one grep prefilter and a per-repository cache (6 s). Prose: the template's architecture comment, the migration's orphaned-reference claim, "shipped" read as installed, and two task notes. The token now reaches `curl` from a 0600 file. 27 test assertions pass, and mutating each fixed behavior turns a case red.

**Routed.** Pre-existing downloads with no named timeout, unchanged in this window (`runtime-acquisition.yml`, `runtime-release.yml:623`, `install.sh`), to 022's task 6.

**Carried.** The 43 standing decisions (2 routed, 41 discarded), byte-identical; every path they name is unchanged since `8dd72c61`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: CFG-ENV-001 does not say whether an empty value on an optional value with a default is absent or malformed — `framework/rules/configuration-cross.md` — **discarded**: predates the widening for environment variables and is unchanged by it; which way an empty optional value falls is a new decision, and CFG-ENV-008 already fails a present malformed value
- other: "at startup" in CFG-ENV-003, CFG-ENV-005 and CFG-ENV-008 does not cover a configuration file a running process reads again — `framework/rules/configuration-cross.md` — **discarded**: a value first read later is validated when it is read, the reading 063 applies on every call, and no spec here reloads a file mid-process
- other: CFG-ENV-008's error names the setting and the value but not the file, which CFG-ENV-003 and CFG-ENV-005 now name for a file key — `framework/rules/configuration-cross.md` — **discarded**: naming the file is a new requirement on a rule today's answers did not widen; the setting and the value already locate the defect
- other: CFG-ENV-005's error names the offending value with no redaction exception like CFG-ENV-008's — `framework/rules/configuration-cross.md` — **discarded**: CFG-ENV-006's MUST redaction at the config-loading layer still applies, so both rules are met together
- other: CFG-CONST-004 places a value's range bounds alongside its default constant, which a required value has none of — `framework/rules/configuration-cross.md` — **discarded**: predates the widening for required environment variables; where a required value's bounds live is a new decision, and the bounds must still be named constants
- other: QUAL-GROUND-001 names CFG-ENV-003 and CFG-ENV-008 as owning configuration, not CFG-ENV-005 — `framework/rules/quality-cross.md` — **discarded**: QUAL-GROUND-001 cedes all configuration to the CFG rules, so the omission changes no finding; the two it names are the presence and type rules its scope decision cited
- other: CFG-ENV-011's Source pairs IEC 80000-13 with a NIST page that credits the binary prefixes to IEC 60027-2 — `framework/rules/configuration-cross.md` — **discarded**: BIPM's VIM attributes the binary prefixes to IEC 80000-13 and NIST to its predecessor IEC 60027-2; together the two citations are accurate
- security: FE-STORAGE-003 does not repeat BE-API-013's WebSocket-handshake exclusion — `framework/rules/security-frontend.md` — **discarded**: its MAY requires a landing page that sets Referrer-Policy: no-referrer, which a WebSocket handshake has no page to do, so a handshake ticket already falls under its MUST NOT; the owner's answer placed the narrowing in BE-API-013 and BE-API-015
- security: BE-API-015's handshake-URL MUST NOT also bars a CSRF token there, which the OWASP WebSocket Security Cheat Sheet suggests sending in the handshake — `framework/rules/security-backend.md` — **discarded**: the owner's 2026-10-20 answer keeps every token out of the handshake URL, following the same cheat sheet's advice against tokens in URL query strings; (a)'s Origin allowlist is the handshake's cross-site defense, and a CSRF token can still be checked in the first message
- other: the pre-signed-URL MAY covers object storage only, so another signed-URL scheme, a CDN's or an application's own HMAC-signed link, falls under BE-API-013 and FE-STORAGE-003 — `framework/rules/security-backend.md` — **discarded**: the owner's 2026-10-20 answer scoped the MAY to a pre-signed URL for one object and one method; whether other signed-URL schemes fit that scope is a new decision
- other: BE-AUTHN-007's Verification asks for legitimate-recovery paths during lockout, which its Statement does not name — `framework/rules/security-backend.md` — **discarded**: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
- other: BE-API-001's Statement requires frame protection on all HTTP responses while its Verification table sets frame-ancestors on HTML responses only — `framework/rules/security-backend.md` — **discarded**: predates this window; OWASP's HTTP Headers Cheat Sheet, which BE-API-001 cites, says X-Frame-Options gives no security on a redirect or a JSON API response, while its REST Security Cheat Sheet recommends frame-ancestors 'none' on API responses, so aligning the two decides what the rule requires
- other: BE-API-013's handshake sentence says BE-API-015 keeps every token out of the handshake URL, while BE-API-015's Statement covers only endpoints serving non-public data or accepting state-changing messages — `framework/rules/security-backend.md` — **discarded**: the sentence is the narrowing the owner's 2026-10-20 answer placed in both rules and matches BE-API-015's unscoped Verification; a token in a handshake URL authenticates the connection, so its endpoint is not public in practice; scoping it made the two rules disagree and was reverted
- security: BE-AUTHN-018's newer-token invalidation would cancel a code issued by one method when a code by another is issued in the same recovery — `framework/rules/security-backend.md` — **discarded**: a recovery that verifies the first code before issuing the second complies, since the first is already used; NIST SP 800-63B-4 sets no invalidation across codes and nothing in BE-AUTHN-018 forbids that order
- other: 008's ticked task rows 13, 14 and 15 describe FE-STORAGE-003, FE-DEPS-004 and BE-AUTHN-022 as they read before later edits — `specs/008-security-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: CFG-ENV-005, widened to every source, does not say whether a value read from several sources is range-checked as resolved or at each source — `framework/rules/configuration-cross.md` — **discarded**: the value validated at startup is the one CFG-ENV-007's resolution yields, the reading CFG-ENV-003 states for presence, and no spec here reads a ranged value from two sources
- reliability: dropping a timed-out client joins its runtime thread, so a slow connect-time lookup can hold a fetch past FETCH_TIMEOUT — `runtime/src/primitives/fetch_archive.rs` — **discarded**: the GaiResolver it replaced behaves the same way, and in production the lookup is getaddrinfo, which the operating system resolver's own timeouts bound
- reliability: the initial and redirect-screen lookups run outside the client's timeout — `runtime/src/primitives/fetch_archive.rs` — **discarded**: unchanged by this fix and bounded the same way, by the operating system resolver's timeouts; FETCH_DEADLINE bounds the download itself
- security: a redirect hop is looked up twice rather than connected to the address its screen resolved — `runtime/src/primitives/fetch_archive.rs` — **discarded**: 022's scenario allows an equivalent connect-time re-check, both lookups are screened, and the owner prescribed this resolver and its staged-rebind test
- other: two proxy tests each define the same four-line internal-address lookup — `runtime/src/primitives/fetch_archive.rs` — **discarded**: test fixtures local to each test; sharing one would couple the tests for no gain
- other: 025's ticked task 2 lists a malformed warning among the step-5 outcomes that task 10 replaced — `specs/025-rule-opt-out/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs` — **discarded**: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
- other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md` — **routed** to `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md` — **routed** to `specs/033-rule-surface-setting/tasks.md`
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
- perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs` — **discarded**: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
- other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs` — **discarded**: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
- other: the pins check's comments implied it bounds every lookup and reads every CI download; it bounds the lookups it makes itself and reads six forms, and now says so — `scripts/check-runtime-currency.sh` — **fixed**
- other: the template said an arm runner fails the hash check; its URL is always the x86_64 archive, so the step fails on running the binary, and the comment now says so — `framework/templates/ci/adopter-generators.yml` — **fixed**
- other: the generator-primitives migration said the orphaned-reference check reports a stale copied workflow; that check reads no workflow file, and the step now says the copy fails on its first run — `framework/migrations/generator-primitives.md` — **fixed**
- other: 017's spec and AGENTS.md called the CI template shipped, which reads as installed, AGENTS.md's Renovate comparison lacked its qualifier, and 017's task 42 and 022's task 3 note overstated; each corrected — `AGENTS.md` — **fixed**
- perf: the pins check spawned subprocesses for every workflow line and fetched a repository's latest release once per pin (32 s); one grep prefilter and a per-repository cache bring it to 6 s with the same report — `scripts/check-runtime-currency.sh` — **fixed**
- other: three copies of one sed, two of one reference count, an inline version comparison beside ver_lt, and a missing-gh message worded as a registry failure; replaced by helpers, ver_lt, and the GitHub API through the bounded fetch — `scripts/check-runtime-currency.sh` — **fixed**
- bug: no fixture wrote a uses: as a step key, and no case asserted where a pin is reported or exercised check 3's comparison; cases U, X, Y and Z cover them — `scripts/tests/test-check-runtime-currency.sh` — **fixed**
- security: the GitHub token went on curl's argument list and into the test stub's log; it is passed from a 0600 file, and the test unsets GH_TOKEN — `scripts/check-runtime-currency.sh` — **fixed**
- other: pre-existing downloads with no named timeout, unchanged in this window: runtime-acquisition.yml:131-132, runtime-release.yml:623 (also retried with a fixed delay and no jitter) and install.sh:124 and :163 — `.github/workflows/runtime-release.yml` — **routed** to `specs/022-deterministic-runtime/tasks.md`
- other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml` — **discarded**: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
- bug: a pinned pre-release would read as current once its stable release ships, since sort -V orders 1.0.0 before 1.0.0-rc.1 — `scripts/check-runtime-currency.sh` — **discarded**: No pin in this tree is a pre-release, and the values it is compared with are crates.io's newest stable version and published release tags, so no input reaches the misordering.
- other: DUCTUS_SHA256 pairs with a DUCTUS_VERSION by file rather than by step, a tab split collapses an empty version field, and cargo install's --version=X, --vers and name@X forms are not parsed — `scripts/check-runtime-currency.sh` — **discarded**: Each fails loud, as a hash mismatch or an unpinned report, and the tree has one install step and only the `--version X` form.
- other: an unknown check name exits 2, the code that also means a check could not run — `scripts/check-runtime-currency.sh` — **discarded**: A misspelt argument is a run that examined nothing, which is what exit 2 states, and the message names the cause.
- other: 022's plan states a provenance exception for crates (BE-DEPS-004) but none for the pip-installed ziglang — `specs/022-deterministic-runtime/plan.md` — **discarded**: 022's own analysis assesses BE-DEPS-004 against 022's plan, which names the ziglang install; it is outside 017's scope.
- other: the first weekly pins run will exit 1 on three action pins a major release behind, the template's checkout, upload-artifact and download-artifact — `.github/workflows/runtime-currency.yml` — **discarded**: Expected and recorded in 022's task 3 measurement; moving those majors is outside task 3 and goes to the owner as its own question.
- other: the template's version check is close to redundant, since the sha256 pin already fixes the bytes — `framework/templates/ci/adopter-generators.yml` — **discarded**: Scenario the-adopter-ci-template-can-run requires it, and it catches a mispackaged release the hash cannot.
- other: the pins check's .yaml globs match no tracked file — `scripts/check-runtime-currency.sh` — **discarded**: A guard, so a workflow added as .yaml is not skipped silently.
- other: pass() and fail() are copied across the four scripts/tests/*.sh — `scripts/tests/test-check-runtime-currency.sh` — **discarded**: There is no shared test library to reuse, and adding one for two lines is not this change's work.
- other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml` — **discarded**: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
- other: "shipped" still describes the CI template in 017's AC24, plan.md's Affected Files row and 022's task 3 text — `specs/017-derive-dont-ask/spec.md` — **discarded**: AC24 calls it a shipped example, the Affected Files row says adopters copy it, and 022's task 3 text is a spent task; none claims it is installed.
- other: cargo's own requests in checks 1, 2 and 4 keep cargo's defaults rather than a timeout the script names — `scripts/check-runtime-currency.sh` — **discarded**: Pre-existing checks, unchanged in this window; runtime-currency.yml's timeout-minutes bounds the whole CI run.

## Skipped passes

*None.*

## Unexamined governance

*None.*
