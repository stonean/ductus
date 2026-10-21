---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-21T00:27:35Z
reviewed-against: a680b2c1756f356088b5e6b1ade165d5c0fb4a63
diff-base: 8eecb53608af1b51f7eff5e513fa26bdafbf50ef
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 66
scope: 144
skipped-passes: []
reviewed-digest:
  data-model.md: b0684b84bc923f52fbfd751cfeafaef3bb0289081e57ba61de3176269c983f1a
  scenarios/an-invalid-threshold-fails-fast.md: 59bceffaed0e278952f69acb4f847a3639b7390314cf212dfbef405624c37f23
  scenarios/an-unknown-artifacts-key-is-named.md: 912ce563e772f628884c9096a8fafb00526379e935c99f40d7ca96090d986e1a
blocking: false
dispositions:
  fixed: 23
  routed: 1
  discarded: 48
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
---

# Review — 063-oversized-artifacts-warn-with-a-fix

## Summary

Five-pass re-review of 063 after 022 task 4 (path canonicalization, which 063's analysis routed there) and the CI work since. The window since this spec's last record (`e7044738`) is the one the shared-window chain of 008's, 022's and 017's reviews read, whose newest record is 017's at `742db59b`, so this run read 063's own artifacts and every in-scope file changed since `742db59b`: the path-containment change (`fd7e1d4e`, `fd42d4e9`), the action-pin, lockfile and download-bound commits (`b58db0b8`, `d58fd275`, `a016b9d4`), and their prose. Three independent passes read them at `a016b9d4` — security and quality; reuse, efficiency and simplicity; all five over the CI and installer changes — with all eleven rule files read in full between them. **Read** (`examined: 66`): 063's eight artifacts in full; the 55 in-scope files changed since `742db59b`, as diffs or in full; the in-scope rule files; and 017's `analysis.md`. **Not read** (78): files whose changes predate `742db59b`, which 017's review read in that shared window — 008's, 017's, 019's, 022's and 036's spec artifacts, `scripts/check-runtime-currency.sh` and its test, four workflows, `README.md`, `docs/`, the framework command and bootstrap sources, and the runtime files of 019's unknown-key change — plus the ten generated mirrors under `.claude/` and `.pi/` (`scripts/gen-claude-commands.sh --check` in sync) and plan-affected files nothing in the window changed. **Findings: none remain.** Two MUST findings were fixed in the run. `BE-INPUT-004`: the resolver judged a dangling `missing/../escape` by its text, so one `apply-manifest` call could write outside its target root, probed end to end. `BE-DATA-001` and `BE-DATA-011`: `runtime-acquisition.yml`'s archive fetches had no TLS floor, and the plan committed to none. The first fix made resolution exponential in a crafted chain of links; an independent re-read caught it, and a hop budget shared across the whole resolution closed it (a fan-out-8 chain refused in 0.43 s, measured past 60 s before). **Observations: 72.** 23 fixed, among them the `BE-RETRY-003` SHOULD on the release retry and three `QUAL-CLAIM-001` corrections; 12 discarded with their reasons, one of them the bootstrap's own unbounded curls in `framework/bootstrap/ductus.md`, pre-existing and 048's surface; 37 carried byte-identically from 017's standing decisions (36 discards, 1 route), 4 of which match this record's own stored decisions. Two independent re-reads verified the fix rounds (`404db2ac`, `a680b2c1`), and each fix's new test fails under the mutation that removes it. One stored decision expires: the pre-commit hook's three stderr lines, whose file is outside this scope.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- security: the resolver appended a name after a `..` past a missing directory without asking whether that name is a link, so one apply-manifest call could create the directory with an earlier entry and write outside its target root with a later one — `runtime/src/primitives/mod.rs` — **fixed**
- perf: the resolver counted its 40-link cap per branch, so a chain of links that each name the next twice cost time exponential in its length — `runtime/src/primitives/mod.rs` — **fixed**
- perf: each feature-argument call read and parsed the config twice, so a malformed config printed its warning twice per call and N+1 times in an --all walk — `runtime/src/primitives/mod.rs` — **fixed**
- other: check-artifacts rebuilt the no-walk feature check inline, and validate_feature_path canonicalized the base a second time — `runtime/src/primitives/check_artifacts.rs` — **fixed**
- perf: the walk called symlink_metadata per entry where DirEntry::file_type answers without another system call — `runtime/src/primitives/mod.rs` — **fixed**
- other: resolve_symlinks_within carried a match arm no input reaches — `runtime/src/primitives/mod.rs` — **fixed**
- other: resolve_contained_dir carried an error branch no input reaches after its starts_with check — `runtime/src/primitives/enforce_manifest.rs` — **fixed**
- other: resolve_path's doc named validate_no_traversal where the primitives it lists call validate_contained — `runtime/src/primitives/mod.rs` — **fixed**
- other: validate_write_code_resolution tested resolved != edit.path, which its only caller's earlier boundary check makes redundant — `runtime/src/schema/extensions.rs` — **fixed**
- other: the interpreter's negative pass ran every case after retire-feature had removed the clean feature, so seven cases passed without testing anything — `runtime/src/interpreter/mod.rs` — **fixed**
- other: walk_write_code repeated the setup of out_of_boundary_write_code_edit_emits_error — `runtime/src/interpreter/mod.rs` — **fixed**
- other: twenty primitives' InvalidPath docs named only the lexical conditions, not a link that leaves the project — `runtime/src/primitives/check_artifact_size.rs` — **fixed**
- other: merge-managed-block's and merge-permissions' docs said every primitive that opens a path runs the BE-INPUT-004 check, which the operator-local readers do not — `runtime/src/primitives/merge_managed_block.rs` — **fixed**
- other: the walk's limit on a directory it cannot list was unstated, so a link inside a traversable but unreadable directory passed unseen with nothing saying so — `runtime/src/primitives/mod.rs` — **fixed**
- security: runtime-acquisition.yml's archive fetches carried no --proto '=https' --tlsv1.2, and 022's plan committed to no TLS floor or certificate check for them (BE-DATA-001, BE-DATA-011) — `.github/workflows/runtime-acquisition.yml` — **fixed**
- security: runtime-acquisition.yml spliced inputs.version and the pin output into its scripts' text — `.github/workflows/runtime-acquisition.yml` — **fixed**
- other: verify-installer's retry retried every failure and ignored Retry-After, and full jitter could wait less than the old fixed 50 s (BE-RETRY-003) — `.github/workflows/runtime-release.yml` — **fixed**
- other: 022's plan said every download on the release path is bounded, while only its curl downloads are — `specs/022-deterministic-runtime/plan.md` — **fixed**
- other: 022's plan said 32 feature-argument sites go through validate_feature_argument or validate_feature_path, while check-artifacts' does not — `specs/022-deterministic-runtime/plan.md` — **fixed**
- other: test-install.sh case V accepted a zero bound and depended on flag order — `scripts/tests/test-install.sh` — **fixed**
- other: AGENTS.md's shared-window entry listed AGENTS.md among the paths the 13 newest decisions cite, which none does — `AGENTS.md` — **fixed**
- other: install.sh's comment and 061's plan gave main's bootstrap size, not the latest release's — `install.sh` — **fixed**
- other: task 7 tied download-artifact's skip-decompress input to files that are not zips — `specs/022-deterministic-runtime/tasks.md` — **fixed**
- perf: write-review checks its feature and compute-review-scope checks it again inside it, and the writeCode payload walks the feature directory once per request — `runtime/src/primitives/write_review.rs` — **discarded**: measured under 2 ms per walk on this corpus's largest feature directory (read-spec on 022, 81 entries, 5.63 ms best of 30 against 3.44 ms for --version), so the repeat costs nothing a caller sees
- perf: write-review loads [paths] three times per call, once itself and once each through compute-review-scope and its analyze-freshness read — `runtime/src/primitives/write_review.rs` — **discarded**: pre-existing composition this window did not add to: each composed reader loaded the config before the change too, and a malformed config prints one line per reader
- perf: dashboard and the --all prune walks now walk every feature directory — `runtime/src/primitives/dashboard.rs` — **discarded**: measured dashboard 0.23-0.25 s, prune-plan --all 0.04 s and prune-tasks --all 0.01 s on this corpus, the walks' share at most about 6 ms; refusing the whole command is the scenario's stated edge case
- perf: a link inside a feature directory to an ancestor makes every call on that feature walk the whole project and refuse on any outward link anywhere in it — `runtime/src/primitives/mod.rs` — **discarded**: only a feature directory holding a link to an ancestor reaches it, no ductus artifact is authored as a link (067's sibling-symlink-trust-boundary), the visited set walks each directory once, and following inside links is what keeps a read through two links from passing unseen
- other: the error for an outward link prints the absolute path the link resolves to — `runtime/src/primitives/mod.rs` — **discarded**: the runtime answers the operator's own agent over stdio, so no caller sits across a trust boundary, and the resolved path is what the operator needs to find the link
- other: canonicalize-and-contain logic exists in both interpreter/payload.rs and primitives/mod.rs — `runtime/src/interpreter/payload.rs` — **discarded**: the two differ on purpose: the payload readers skip a missing or outside path and give None for a non-UTF-8 name, where the check refuses with an error and judges a missing path, so one helper would carry both behaviors
- other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs` — **discarded**: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold
- other: validate_write_code_resolution takes its resolver as a closure with one caller — `runtime/src/schema/extensions.rs` — **discarded**: the closure keeps filesystem access out of the schema module, which otherwise holds shapes and their checks
- other: fifteen primitives that can return InvalidPath list none in their docs — `runtime/src/primitives/read_spec.rs` — **discarded**: pre-existing: none listed it for the lexical screen they already ran, and the change gives them a new cause of that error, not a new error
- other: verify-published fetches the just-uploaded assets with no retry, unlike verify-installer — `.github/workflows/runtime-acquisition.yml` — **discarded**: all 40 of the most recent runtime-release runs concluded success, so the 404 window has not failed it, and it gates nothing: both halves of the release are out when it runs
- other: verify-installer ignores a Retry-After of 2^63 or more and reads a redirect hop's Retry-After when the final hop sends none — `.github/workflows/runtime-release.yml` — **discarded**: an out-of-range value falls back to the jittered wait, and GitHub's release-asset redirect sends no Retry-After, so neither changes a run
- other: the bootstrap's own curls in framework/bootstrap/ductus.md carry no timeout and no --proto or --tlsv1.2, though runtime-acquisition.yml calls itself their executable copy — `framework/bootstrap/ductus.md` — **discarded**: pre-existing and outside this window: the bootstrap's curls are steps the host agent runs, 017's review routed only the workflows' and the installer's to 022 task 6, and bounding them changes the shipped bootstrap and 048's acquisition plan, a decision of its own
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
- other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs` — **discarded**: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
- other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md` — **routed** to `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md` — **discarded**: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
- other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md` — **discarded**: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
- other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md` — **discarded**: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
- other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md` — **discarded**: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
- other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md` — **discarded**: a resolved question records the reasoning at the time; rewriting it records a fiction
- perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs` — **discarded**: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
- other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs` — **discarded**: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
- other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml` — **discarded**: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
- bug: a pinned pre-release would read as current once its stable release ships, since sort -V orders 1.0.0 before 1.0.0-rc.1 — `scripts/check-runtime-currency.sh` — **discarded**: No pin in this tree is a pre-release, and the values it is compared with are crates.io's newest stable version and published release tags, so no input reaches the misordering.
- other: DUCTUS_SHA256 pairs with a DUCTUS_VERSION by file rather than by step, a tab split collapses an empty version field, and cargo install's --version=X, --vers and name@X forms are not parsed — `scripts/check-runtime-currency.sh` — **discarded**: Each fails loud, as a hash mismatch or an unpinned report, and the tree has one install step and only the `--version X` form.
- other: an unknown check name exits 2, the code that also means a check could not run — `scripts/check-runtime-currency.sh` — **discarded**: A misspelt argument is a run that examined nothing, which is what exit 2 states, and the message names the cause.
- other: 022's plan states a provenance exception for crates (BE-DEPS-004) but none for the pip-installed ziglang — `specs/022-deterministic-runtime/plan.md` — **discarded**: 022's own analysis assesses BE-DEPS-004 against 022's plan, which names the ziglang install; it is outside 017's scope.
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
