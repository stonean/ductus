---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-27T04:19:28Z
reviewed-against: 88768fe67738169b56f4f60554eb67b383858398
diff-base: 865e97ae554c65b42bdf842d2a291e69981a9ffe
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 44
scope: 72
skipped-passes: []
reviewed-digest:
  data-model.md: 42dc6d741a76d613b5a1424a86725d08c2d7cb52b2f3901ced2cd736559fa426
  scenarios/a-plan-trim-does-not-reopen-a-done-spec.md: 981fb71fcb69757942c915c7b000474cfbe278bd7b9e89aefbfda7307e1e28fa
  scenarios/an-invalid-threshold-fails-fast.md: 59bceffaed0e278952f69acb4f847a3639b7390314cf212dfbef405624c37f23
  scenarios/an-unknown-artifacts-key-is-named.md: 912ce563e772f628884c9096a8fafb00526379e935c99f40d7ca96090d986e1a
blocking: false
dispositions:
  fixed: 13
  routed: 0
  discarded: 10
  undispositioned: 0
decisions:
  - key: "other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: discarded
    reason: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
    decided-at: 2026-10-20T12:25:46Z
    decided-by: andy@stone.dev
  - key: "perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs`"
    outcome: discarded
    reason: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
    decided-at: 2026-10-20T12:25:46Z
    decided-by: andy@stone.dev
  - key: "other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs`"
    outcome: discarded
    reason: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
    decided-at: 2026-10-20T12:25:46Z
    decided-by: andy@stone.dev
  - key: "perf: write-review checks its feature and compute-review-scope checks it again inside it, and the writeCode payload walks the feature directory once per request — `runtime/src/primitives/write_review.rs`"
    outcome: discarded
    reason: measured under 2 ms per walk on this corpus's largest feature directory (read-spec on 022, 81 entries, 5.63 ms best of 30 against 3.44 ms for --version), so the repeat costs nothing a caller sees
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "perf: write-review loads [paths] three times per call, once itself and once each through compute-review-scope and its analyze-freshness read — `runtime/src/primitives/write_review.rs`"
    outcome: discarded
    reason: "pre-existing composition this window did not add to: each composed reader loaded the config before the change too, and a malformed config prints one line per reader"
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "perf: dashboard and the --all prune walks now walk every feature directory — `runtime/src/primitives/dashboard.rs`"
    outcome: discarded
    reason: measured dashboard 0.23-0.25 s, prune-plan --all 0.04 s and prune-tasks --all 0.01 s on this corpus, the walks' share at most about 6 ms; refusing the whole command is the scenario's stated edge case
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "perf: a link inside a feature directory to an ancestor makes every call on that feature walk the whole project and refuse on any outward link anywhere in it — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: only a feature directory holding a link to an ancestor reaches it, no ductus artifact is authored as a link (067's sibling-symlink-trust-boundary), the visited set walks each directory once, and following inside links is what keeps a read through two links from passing unseen
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: the error for an outward link prints the absolute path the link resolves to — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: the runtime answers the operator's own agent over stdio, so no caller sits across a trust boundary, and the resolved path is what the operator needs to find the link
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: canonicalize-and-contain logic exists in both interpreter/payload.rs and primitives/mod.rs — `runtime/src/interpreter/payload.rs`"
    outcome: discarded
    reason: "the two differ on purpose: the payload readers skip a missing or outside path and give None for a non-UTF-8 name, where the check refuses with an error and judges a missing path, so one helper would carry both behaviors"
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs`"
    outcome: discarded
    reason: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: validate_write_code_resolution takes its resolver as a closure with one caller — `runtime/src/schema/extensions.rs`"
    outcome: discarded
    reason: the closure keeps filesystem access out of the schema module, which otherwise holds shapes and their checks
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: fifteen primitives that can return InvalidPath list none in their docs — `runtime/src/primitives/read_spec.rs`"
    outcome: discarded
    reason: "pre-existing: none listed it for the lexical screen they already ran, and the change gives them a new cause of that error, not a new error"
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: verify-published fetches the just-uploaded assets with no retry, unlike verify-installer — `.github/workflows/runtime-acquisition.yml`"
    outcome: discarded
    reason: "all 40 of the most recent runtime-release runs concluded success, so the 404 window has not failed it, and it gates nothing: both halves of the release are out when it runs"
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: verify-installer ignores a Retry-After of 2^63 or more and reads a redirect hop's Retry-After when the final hop sends none — `.github/workflows/runtime-release.yml`"
    outcome: discarded
    reason: an out-of-range value falls back to the jittered wait, and GitHub's release-asset redirect sends no Retry-After, so neither changes a run
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: the bootstrap's own curls in framework/bootstrap/ductus.md carry no timeout and no --proto or --tlsv1.2, though runtime-acquisition.yml calls itself their executable copy — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: "pre-existing and outside this window: the bootstrap's curls are steps the host agent runs, 017's review routed only the workflows' and the installer's to 022 task 6, and bounding them changes the shipped bootstrap and 048's acquisition plan, a decision of its own"
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-001 does not say whether an empty value on an optional value with a default is absent or malformed — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: predates the widening for environment variables and is unchanged by it; which way an empty optional value falls is a new decision, and CFG-ENV-008 already fails a present malformed value
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: \"at startup\" in CFG-ENV-003, CFG-ENV-005 and CFG-ENV-008 does not cover a configuration file a running process reads again — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: a value first read later is validated when it is read, the reading 063 applies on every call, and no spec here reloads a file mid-process
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-008's error names the setting and the value but not the file, which CFG-ENV-003 and CFG-ENV-005 now name for a file key — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: naming the file is a new requirement on a rule today's answers did not widen; the setting and the value already locate the defect
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005's error names the offending value with no redaction exception like CFG-ENV-008's — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: CFG-ENV-006's MUST redaction at the config-loading layer still applies, so both rules are met together
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: CFG-CONST-004 places a value's range bounds alongside its default constant, which a required value has none of — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: predates the widening for required environment variables; where a required value's bounds live is a new decision, and the bounds must still be named constants
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 names CFG-ENV-003 and CFG-ENV-008 as owning configuration, not CFG-ENV-005 — `framework/rules/quality-cross.md`"
    outcome: discarded
    reason: QUAL-GROUND-001 cedes all configuration to the CFG rules, so the omission changes no finding; the two it names are the presence and type rules its scope decision cited
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-011's Source pairs IEC 80000-13 with a NIST page that credits the binary prefixes to IEC 60027-2 — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: BIPM's VIM attributes the binary prefixes to IEC 80000-13 and NIST to its predecessor IEC 60027-2; together the two citations are accurate
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "security: FE-STORAGE-003 does not repeat BE-API-013's WebSocket-handshake exclusion — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: "its MAY requires a landing page that sets Referrer-Policy: no-referrer, which a WebSocket handshake has no page to do, so a handshake ticket already falls under its MUST NOT; the owner's answer placed the narrowing in BE-API-013 and BE-API-015"
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015's handshake-URL MUST NOT also bars a CSRF token there, which the OWASP WebSocket Security Cheat Sheet suggests sending in the handshake — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the owner's 2026-10-20 answer keeps every token out of the handshake URL, following the same cheat sheet's advice against tokens in URL query strings; (a)'s Origin allowlist is the handshake's cross-site defense, and a CSRF token can still be checked in the first message
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: the pre-signed-URL MAY covers object storage only, so another signed-URL scheme, a CDN's or an application's own HMAC-signed link, falls under BE-API-013 and FE-STORAGE-003 — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the owner's 2026-10-20 answer scoped the MAY to a pre-signed URL for one object and one method; whether other signed-URL schemes fit that scope is a new decision
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-007's Verification asks for legitimate-recovery paths during lockout, which its Statement does not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-001's Statement requires frame protection on all HTTP responses while its Verification table sets frame-ancestors on HTML responses only — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: predates this window; OWASP's HTTP Headers Cheat Sheet, which BE-API-001 cites, says X-Frame-Options gives no security on a redirect or a JSON API response, while its REST Security Cheat Sheet recommends frame-ancestors 'none' on API responses, so aligning the two decides what the rule requires
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-013's handshake sentence says BE-API-015 keeps every token out of the handshake URL, while BE-API-015's Statement covers only endpoints serving non-public data or accepting state-changing messages — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence is the narrowing the owner's 2026-10-20 answer placed in both rules and matches BE-API-015's unscoped Verification; a token in a handshake URL authenticates the connection, so its endpoint is not public in practice; scoping it made the two rules disagree and was reverted
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-018's newer-token invalidation would cancel a code issued by one method when a code by another is issued in the same recovery — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: a recovery that verifies the first code before issuing the second complies, since the first is already used; NIST SP 800-63B-4 sets no invalidation across codes and nothing in BE-AUTHN-018 forbids that order
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: 008's ticked task rows 13, 14 and 15 describe FE-STORAGE-003, FE-DEPS-004 and BE-AUTHN-022 as they read before later edits — `specs/008-security-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005, widened to every source, does not say whether a value read from several sources is range-checked as resolved or at each source — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: the value validated at startup is the one CFG-ENV-007's resolution yields, the reading CFG-ENV-003 states for presence, and no spec here reads a ranged value from two sources
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: routed
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: discarded
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml`"
    outcome: discarded
    reason: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "bug: a pinned pre-release would read as current once its stable release ships, since sort -V orders 1.0.0 before 1.0.0-rc.1 — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: No pin in this tree is a pre-release, and the values it is compared with are crates.io's newest stable version and published release tags, so no input reaches the misordering.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: DUCTUS_SHA256 pairs with a DUCTUS_VERSION by file rather than by step, a tab split collapses an empty version field, and cargo install's --version=X, --vers and name@X forms are not parsed — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: Each fails loud, as a hash mismatch or an unpinned report, and the tree has one install step and only the `--version X` form.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: an unknown check name exits 2, the code that also means a check could not run — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: A misspelt argument is a run that examined nothing, which is what exit 2 states, and the message names the cause.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: 022's plan states a provenance exception for crates (BE-DEPS-004) but none for the pip-installed ziglang — `specs/022-deterministic-runtime/plan.md`"
    outcome: discarded
    reason: 022's own analysis assesses BE-DEPS-004 against 022's plan, which names the ziglang install; it is outside 017's scope.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: the template's version check is close to redundant, since the sha256 pin already fixes the bytes — `framework/templates/ci/adopter-generators.yml`"
    outcome: discarded
    reason: Scenario the-adopter-ci-template-can-run requires it, and it catches a mispackaged release the hash cannot.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: the pins check's .yaml globs match no tracked file — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: A guard, so a workflow added as .yaml is not skipped silently.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: pass() and fail() are copied across the four scripts/tests/*.sh — `scripts/tests/test-check-runtime-currency.sh`"
    outcome: discarded
    reason: There is no shared test library to reuse, and adding one for two lines is not this change's work.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml`"
    outcome: discarded
    reason: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: \"shipped\" still describes the CI template in 017's AC24, plan.md's Affected Files row and 022's task 3 text — `specs/017-derive-dont-ask/spec.md`"
    outcome: discarded
    reason: AC24 calls it a shipped example, the Affected Files row says adopters copy it, and 022's task 3 text is a spent task; none claims it is installed.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: cargo's own requests in checks 1, 2 and 4 keep cargo's defaults rather than a timeout the script names — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: Pre-existing checks, unchanged in this window; runtime-currency.yml's timeout-minutes bounds the whole CI run.
    decided-at: 2026-10-21T00:27:35Z
    decided-by: andy@stone.dev
  - key: "other: the plan.md and tasks.md filename literals are repeated with no shared constant (CFG-CONST-001 note) — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: a pre-existing pattern across 14 and 15 modules with no shared constant; introducing one is its own change
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: /amend's reconcile pass offers to reopen a done spec for any uncommitted tasks.md change — `framework/commands/amend.md`"
    outcome: discarded
    reason: it reopens only on the operator's confirmation, matching 023's recorded decision that the operator is the discriminator; 014 owns the delta, and 041's scenario records the boundary
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 072's out-of-scope note says changing the plan rule needs its own spec — `specs/072-governing-documents-stay-within-one-read/spec.md`"
    outcome: discarded
    reason: the note's subject, why 072 did not take the change, still holds
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 023's living-specs and 054 call every factual correction meaningful — `specs/023-govern-refinement/scenarios/living-specs.md`"
    outcome: discarded
    reason: both concern spec.md bodies or audit artifacts, never a plan or task list
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md`"
    outcome: discarded
    reason: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: the replay proof is true by construction for the new binary — `specs/041-task-pruning/tasks.md`"
    outcome: discarded
    reason: its proof is the old binary's half, which reopened all 18; the new half confirms the change landed
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "perf: analyze_freshness re-reads spec.md its callers already parsed — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: one small file read per call, negligible
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 063's Resolved Question says /prune reopens a spec only when it adds owed work — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`"
    outcome: discarded
    reason: "loose, not false, in that Resolved Question: prune reopens when the diff adds owed work"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "edge: an unreadable plan.md or tasks.md on a done spec is dropped like an edit in place and reads current — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: "contrived: no command writes an unreadable plan or task list"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "edge: an in-place plan or tasks edit that could produce a blocking analyze finding is still ignored on a done spec — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: "the chosen design: only a deletion is kept, and the one consumer is the advisory review row"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
---

# Review — 063-oversized-artifacts-warn-with-a-fix

## Summary

Five passes over the window 865e97ae..88768fe6 — 063's diff base, the parent of e3e54b07, which reopened it through the scenario back-edge for `a-plan-trim-does-not-reopen-a-done-spec`: `check-artifact-size` now advises that a `plan.md` trim on a `done` spec does not reopen it. The window is shared with 041 and 047 and was reviewed once in depth; this record carries that review, read from the same passes, with identical observations and dispositions. An independent reviewer ran security, reuse, quality, efficiency and simplicity over e3e54b07 and 3ce51eb9 against api-, concurrency-, observability-, performance-, reliability- and security-backend plus configuration- and quality-cross: 0 MUST, 0 SHOULD against a loaded rule, and 23 observations across three rounds — 13 fixed, 10 discarded with reasons. For 063 the rounds corrected its Resolved Question, its lead's condition and its plan, and pinned a `data-model.md` trim's effect with a test that kills mutation M5. Round 1 (fbf2737c) and round 2 (2f1c33c8) were each re-read independently; round 2's re-read found no MUST or SHOULD. The 49 stored decisions from 063's earlier windows were retained as unevaluated by this window. Examined: 44 of the 52 window files, every changed file but seven generated copies diffed against their sources and 88768fe6's 058 scenario, outside this review. `scope` counts 72 entries, each one path: the 52 window files, 17 plan Affected Files rows untouched since the base and not read, and the three review records this window writes for 041, 047 and 063, which are the passes' output rather than their subject.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 041's plan, data model and two scenario edge cases still described reopen-required's design-record trigger — `specs/041-task-pruning/plan.md` — **fixed**
- other: 063's Resolved Question ended on /prune reopening a spec when a moved decision edits the design record — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **fixed**
- other: restatements of the plan rule dropped its condition, adding no unchecked task, or said the comparison leaves the files out — `framework/reference/review.md` — **fixed**
- bug: a deleted plan.md or tasks.md on a done spec read current, though analyze reports a missing one as blocking — `runtime/src/primitives/analyze_subjects.rs` — **fixed**
- other: the plan.md and tasks.md filename literals are repeated with no shared constant (CFG-CONST-001 note) — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: a pre-existing pattern across 14 and 15 modules with no shared constant; introducing one is its own change
- other: the review reference said a plan correction leaves the analyze row current, true only of a record already current — `framework/reference/review.md` — **fixed**
- other: /amend's reconcile pass offers to reopen a done spec for any uncommitted tasks.md change — `framework/commands/amend.md` — **discarded**: it reopens only on the operator's confirmation, matching 023's recorded decision that the operator is the discriminator; 014 owns the delta, and 041's scenario records the boundary
- other: 072's out-of-scope note says changing the plan rule needs its own spec — `specs/072-governing-documents-stay-within-one-read/spec.md` — **discarded**: the note's subject, why 072 did not take the change, still holds
- other: 023's living-specs and 054 call every factual correction meaningful — `specs/023-govern-refinement/scenarios/living-specs.md` — **discarded**: both concern spec.md bodies or audit artifacts, never a plan or task list
- other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md` — **discarded**: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
- other: 013's and 016's plans carried further stale constitution claims outside the routed list — `specs/013-text-first-artifacts/plan.md` — **fixed**
- other: the replay proof is true by construction for the new binary — `specs/041-task-pruning/tasks.md` — **discarded**: its proof is the old binary's half, which reopened all 18; the new half confirms the change landed
- other: AGENTS.md's two-disposals entry contradicted the sentence appended to it — `AGENTS.md` — **fixed**
- other: the analyze reference's trim advice read as self-contradictory in one sentence — `framework/reference/analyze.md` — **fixed**
- test: a data-model.md trim at done was unpinned, so widening the plan-trim exemption to it survived — `runtime/src/primitives/check_artifact_size.rs` — **fixed**
- perf: analyze_freshness re-reads spec.md its callers already parsed — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: one small file read per call, negligible
- other: 016's plan item 2 still described a link to 008's data model and an Edge Case the round deleted — `specs/016-cross-cutting-rules/plan.md` — **fixed**
- other: without_plan_edits' doc comment said the commands that read tasks.md reopen a spec for an added task — `runtime/src/primitives/analyze_subjects.rs` — **fixed**
- other: 063's Resolved Question says /prune reopens a spec only when it adds owed work — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: loose, not false, in that Resolved Question: prune reopens when the diff adds owed work
- other: AGENTS.md's disposals entry left a task-list fix that adds a task uncovered — `AGENTS.md` — **fixed**
- other: 016's plan item 4 named cross-references §rules does not carry — `specs/016-cross-cutting-rules/plan.md` — **fixed**
- edge: an unreadable plan.md or tasks.md on a done spec is dropped like an edit in place and reads current — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: contrived: no command writes an unreadable plan or task list
- edge: an in-place plan or tasks edit that could produce a blocking analyze finding is still ignored on a done spec — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: the chosen design: only a deletion is kept, and the one consumer is the advisory review row

## Skipped passes

*None.*

## Unexamined governance

*None.*
