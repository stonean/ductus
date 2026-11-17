---
spec: 022-deterministic-runtime
last-run: 2026-11-17T20:05:55Z
reviewed-against: 56ab4553473e02b5ed9f9c7b4547c68087e348e0
diff-base: 3e35c5341c0751e9aacb220347e90c39bc9fb88e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 4
unread:
  - specs/022-deterministic-runtime/analysis.md
  - specs/022-deterministic-runtime/tasks.md
excluded: 1
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 7ec795f721c05faeb74e015f56730c9ef5fcebe81d6bf5f1641e621b4ec176b8
  scenarios/a-closed-stdout-is-a-quiet-exit.md: ef3db93cc91fecfca53ab67300605cbdf3823c916fe6f1131933740434ad0ca5
  scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md: f3c4072fc69ebfa829d92ae21c7043b7e3aa2a4be3db175b7df56204882e6fb4
  scenarios/a-done-when-clause-reads-to-its-end.md: b2d6b2ab8cbd208eec3ac811edc662c296792d51ec7ab1e0c49ec2c8acb045e9
  scenarios/a-path-argument-resolves-inside-its-base.md: 503e9dd4b66563bb317c4deaf4e9cee88275c3c5e3a71a50628704f208a4ac3d
  scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md: 5fec39041378aa1786a94cf59eb1676dd33b7f15b5371815ac92d1e8382c4c72
  scenarios/append-inbox-comment-aware-write.md: c15499bda1cef7c531d2d135e20a5b265761841f1abb15db5f4225d286eb65b8
  scenarios/append-primitive-marker-normalization.md: 26c5064dcdf3f697885faac80332207d079bfd7db2d14c0769254da21e5e4246
  scenarios/append-task-honours-slug-with-explicit-body.md: 4e52f43a5f6463d13e9509b34ff017ffb72eec230e8636beb2bc39dace87bc86
  scenarios/apply-manifest-substitution-contract.md: d207e610fe23640c400a4e08205172c699555fde5852c9de1e4405cf1ca72a60
  scenarios/apply-manifest.md: 60f4987e9a73e25827e88e90078295dff5ed59863de94af820f1b510b4a24a61
  scenarios/archive-network-hardening.md: 127616981fb074c9e21ef54b4f58d74413f1fdc31af349529601b8dbc096a512
  scenarios/ask-consolidation.md: a406d0280d7fabf622c43e18b41268b3bcf1d43147325a033369fa1977767b4a
  scenarios/check-stuck-tasks-md-advancement.md: aaabecc62d10bbdde0c532dadc61cde5fa6b88e3792680389b26eb8b4b95d5b7
  scenarios/clarify-command-acceleration.md: 5f2a753c0a3aeffc1883225f19d7a352053bd8ef6968adb2c381312cec907a31
  scenarios/cli-config-dir-per-contributor.md: adc1a63bfb7c786e70a81a303b64cc19c974a91035fcb288a59ef149dda1e268
  scenarios/commands-dir-parameterization.md: f7cb45048600cd51dfe3c245ac61a689589486c28153999700581624ee6ac6bb
  scenarios/config-readers-name-unknown-keys.md: 569aec4b2acbb964f737bb9499a4fecc7dfc1807aad5a4db0ad360d4da6e5fff
  scenarios/config-resolution-single-probe.md: a487eec0a7cce706acd911511115140073284ca5b35409ec24cf4f2546b38724
  scenarios/config-sourced-paths-and-the-traversal-boundary.md: 3ed48e5af4e6a0ca019f9f7ae961771a01e94e595c02ae7c614258b42d777873
  scenarios/constitution-excerpts-as-skill-resources.md: 672e58e225993a12b8a332a642a918322967fe3c8a3679ffa06aa654788ddc69
  scenarios/coverage-expansion-primitives.md: 8fbe190c64b21c9f0edf92d764bdf9138d4f122830eea60df726d526859578d6
  scenarios/coverage-residue-cleanup.md: 0c6315ac8f0f041f624b0a389bbfca855d9886bb06703cef4ee5ffd513770d17
  scenarios/criterion-label-assignment.md: 57e455a91d0b76c6a3a5bdea80288300c246d00d53425730be88896e89c25305
  scenarios/dashboard-primitive.md: 70d8a91916780bab45ed495f1c7f197d1ebd83f27caba0b1a4a2ea9849434fca
  scenarios/derive-boundary-uncommitted-spec-dir.md: 64edf042d36ea149576e46e44ddf9bd4bce4e6eea12c59a092a31781baec5c6b
  scenarios/done-when-authoring-forms.md: 080f8717135b67c8400e072dae02f172e40cb0b6ad596c3e227d6cc5f90d0191
  scenarios/every-mcp-argument-has-a-cli-form.md: 409a34b5a138642d023dc7f0a0b864013f053c84b02620f9bcb16fda20cbc4ef
  scenarios/exec-analyze-derives-its-list-seeds.md: 9fda65d4cd7a5ec0fab7e07ba9d5d25e794944c967cd09e7b3547f8e1d560726
  scenarios/exec-clarify-asks-each-open-question.md: 4851dc543c6dd80603aa86c776543017b9ab1a7adac469e1b49ec311a9877cab
  scenarios/extension-request-hygiene.md: 1b0c77335af3a2caf82e01896207d60261cd0071a8bc8da3162b9785f7c16e48
  scenarios/fetch-archive-bounds-the-whole-download.md: cb908fbcee4bdcb1f8701c2a678fd472716b4e8e2e61172200324af7abb05379
  scenarios/fetch-archive-dns-rebinding.md: e876d9e0d27d6757b024b9f9ff885f8cb0baa56d5886939cae487a591d362af8
  scenarios/fetch-archive-names-its-timeout.md: d45c28473624ee40fb8f6bdf93961bf28947f8d0e05413c7e713f2b307d911a2
  scenarios/framework-list-dedup.md: e01884c811f286768f80cf0a616088c569bb1961ab07db670b47e843f39d91fd
  scenarios/govern-bootstrap.md: 5a52fb02c1be863ed6a9051fd004a9fc6f1dd802494f9405649bf34b95bc76c4
  scenarios/groom-command-acceleration.md: dd6052b3d7ee1b1f8528913b82eabfbaf6debe4de51043824b2fcd5d040ed27e
  scenarios/host-protocol-conformance.md: 13c3a2d89eedf4f79e66c9d4e8e97a9b7361e6fd1fa340a76c46730c28e4d542
  scenarios/implement-completion-gate.md: 0c466759ae195e7131a5a7838f7f6f0403d42baa6f98de596af9e32aa481285e
  scenarios/installed-files-are-not-group-writable.md: 3a29eb614d5bd804698622713e8d18e29d8b68f3030d342a4e1f1be30c276cc4
  scenarios/lint-markdown-lints-only-what-it-is-given.md: 92efe81c7326ae4daa08f1f40596afe6221a30433a45ccc7998ab5fadff28ddf
  scenarios/lint-markdown-tool-resolution.md: 06ec661f04cfabf19e7950adfb86d3db440ba48cd4ec8095c34897e0a96d1986
  scenarios/mark-task-backtick-headings.md: a405d3e4b08ef15de8037541c05a93f70a5553f4a72407b917aed4e50ba20507
  scenarios/mark-task-untick-symmetry.md: 0b04c0e1f2091244701f75c435884cb0857a2be728eac41f84fbe41c8df92fb0
  scenarios/mcp-arg-unknown-field-strictness.md: 57c17fb05d72b8f47f64140fc0d8ba6df0c0b8f757282e41d1036aa222309a1b
  scenarios/merge-managed-block-multi-subsection-end.md: b8808a23e83bff0c0b9f4bf2428eb1143a4b2e5e3667b068637d96f05b20824d
  scenarios/merge-managed-block-renamed-subsection.md: dc7fda75fa01616e225d4eb4589d650382cd373933247bc52b91d2eafe06aca0
  scenarios/merge-managed-block-subsection-insertion.md: 65dd7ccd8f3defeae5223c6d944fae823ed440dee241b090c41e31e4bfe7f457
  scenarios/merge-managed-block-trailing-append.md: 6f18642416470c96b6f582a0305d62137a9ae2ec6f1ec054eccac421d8e68a4f
  scenarios/numbered-heading-grammar-single-source.md: 359d35403e9b3f853e4a7cbcca71f3fc032ad556406bae6a584272c6bcd97862
  scenarios/open-questions-are-any-list-item.md: e91d7466be00bcd396959c274dfce64c1d94d0e913b761b7e991da0bf611d6d7
  scenarios/parser-nested-list-continuation.md: 60eb9129494668bb18045e94e9c6b208353a9a1195ef384cf2aeb6d2482ce139
  scenarios/parser-walker-conventions.md: b53b0ffcadc43ec54c18f6b473b84021cddef418bf6481227b87f06a6acf08cb
  scenarios/primitive-robustness-hardening.md: 7cdf3dcf99c06332244540b196c97b13783184f7b9fba989108dd98f84524ae5
  scenarios/project-directory-resolution-chain.md: d43223e3f037960a24f5660a9944aae0fe02df5f1da10257477926a63cfb5a82
  scenarios/resolve-references-cli-exec-wiring.md: 51d572e34ffca5314a4a7af140296fbc1c34008a46a4fd7a65216eae522aa957
  scenarios/review-exec-wiring.md: 9dbf0c621deaa58d40d16d09c289abdd5cb31ab5555f2036088e1d8f04d76515
  scenarios/review-runtime-acceleration.md: ffbd167fe98fcb8ccdde8f0c329ae395347258a98518a7d40c612300ea75a34e
  scenarios/review-scope-parse-fidelity.md: f970155028f105cc7441f1597113df13592fc0e0b0fceee80b60787e420ef2ac
  scenarios/runtime-primitive-structural-bugs.md: 7fb973b4c5ef5c0268df24996c585f9bf70465f4f7e36ad237d129894f5fd366
  scenarios/runtime-review-019-hardening.md: dd72b20cb886ee804141895d169ebb90eb5c548df260fb02cb1643dd2a6f777a
  scenarios/scaffolding-primitives.md: c3495b99e2f440822fed57cb08ba0cd668e9093fd786ad37e616a8508417c699
  scenarios/scenario-open-question-signal.md: b731886b9aa7affab0a231d595f4ed422cb9a566dc1c4c31faf03c33f0674227
  scenarios/scenario-question-parser-fix.md: bdcaf5f9bb7d0ff795d6e3a6a5045a79afc63b55d8ce4b3331f807c337bf2ada
  scenarios/severity-is-a-closed-set-not-a-string.md: 8b6ca46ef55c4f6b12420786ebae219f5aa4f2a2878c85d042098f5674013d2e
  scenarios/skipscanner-inline-code-exemption.md: ab402c94a437b19b1b9bdd5a58ad3303d6bb67e9822041520522e3a066c43ab5
  scenarios/spec-side-parser-hardening.md: 91a804882033f369ff98f36157a38896d29eb7d90e8ae4c78e86d45ac35aca3d
  scenarios/specify-routes-before-scaffolding.md: afa3ced4b110e3b7d2f2e96e86cfd719487fce6983e948cbfea4150f4670ea33
  scenarios/the-cli-surface-nothing-pins.md: 634128f508bb6cc3d94913216ce5aefa3571a93bae2e2d45ecb8a0accf22dcc6
  scenarios/the-constitutions-registry-validates-its-values.md: 161cf85126b357e1c7fe6e63be2de5d80e96842a26acb3ed299af55f4bd92acc
  scenarios/the-pi-command-candidate.md: 115a1720aa5a467e95d6525a6e5f0dd496898a5dba80a937992ea7b498c6500f
  scenarios/unchecked-done-when-clause-tally.md: 2fa92d88a135df29d4097e5fac7ba2a42583303237cb81bcb8a8df3a7e157ade
  scenarios/unreadable-scenario-is-reported.md: 28c763fee22e177c50586f69989911128438c23964e1fa4831f628ae366792ae
  scenarios/waiver-processing-order.md: 0adb750e651d94ac8bcf06dd0dfbba84cd937c09817035e913ec08b77687f8e8
  scenarios/write-boundary-path-normalization.md: aea2703ee38d7a2ab5806444dd12f99370bfee540bafd8935c9f20d43bd86dee
  scenarios/write-session-primitive.md: 56779f1f97b3622d04b44e61e3c05653afa8f3010014f0699b3f234a630f7db5
  scenarios/writecode-boundary-derivation.md: 44bbce7e22a35dd58048811c3441f633e2e14f37e48dcdeac089c42276a639cd
  scenarios/writecode-payload-bundling.md: 196c9d5ee60f7afe918bf0308e9276bcb8b224c80a2f80beacc44b474b39e195
  scenarios/writecode-payload-canonicalize-paths.md: 61fc0c5dffd7d22ffd6e413e1c4f98adc1869f3fef7357340021d49b964a4a7c
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: "reliability: dropping a timed-out client joins its runtime thread, so a slow connect-time lookup can hold a fetch past FETCH_TIMEOUT — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: the GaiResolver it replaced behaves the same way, and in production the lookup is getaddrinfo, which the operating system resolver's own timeouts bound
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "reliability: the initial and redirect-screen lookups run outside the client's timeout — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: unchanged by this fix and bounded the same way, by the operating system resolver's timeouts; FETCH_DEADLINE bounds the download itself
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "security: a redirect hop is looked up twice rather than connected to the address its screen resolved — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: 022's scenario allows an equivalent connect-time re-check, both lookups are screened, and the owner prescribed this resolver and its staged-rebind test
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: two proxy tests each define the same four-line internal-address lookup — `runtime/src/primitives/fetch_archive.rs`"
    outcome: waived
    reason: test fixtures local to each test; sharing one would couple the tests for no gain
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 025's ticked task 2 lists a malformed warning among the step-5 outcomes that task 10 replaced — `specs/025-rule-opt-out/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: waived
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs`"
    outcome: waived
    reason: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
    decided-at: 2026-10-20T13:09:28Z
    decided-by: andy@stone.dev
  - key: "other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs`"
    outcome: waived
    reason: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
    decided-at: 2026-10-20T13:09:28Z
    decided-by: andy@stone.dev
  - key: "other: the analyze walker's seed derivation leaves paths unbound for a refused feature rather than raising InvalidPath — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: it opens nothing, and the walk's first step, read-spec on the same feature, raises the InvalidPath before any step reads paths
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the Windows \\\\?\\ comparison has no symlink test, every containment test being cfg(unix) — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: set-status's eight tests, none cfg(unix), run on windows-latest through validate_feature_argument with tempdir bases, so a base and candidate in different canonical forms would refuse every one of them
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "bug: a writeCode delete of a link inside the boundary is judged by where the link points, so deleting a runtime/ link into framework/ is refused — `runtime/src/schema/extensions.rs`"
    outcome: waived
    reason: "it errs safe: the boundary refuses an edit it could admit and never admits one it should refuse, and this repository tracks no symlinks"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: retire-feature runs the lexical screen twice — `runtime/src/primitives/retire_feature.rs`"
    outcome: waived
    reason: "deliberate: the first screen runs ahead of refusal 1, so a traversing name is refused before the sequential-directory check, and the repeat is one scan of a short string"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: 061's plan says the 048 and 015 fetches keep their own terms, while neither plan states any — `specs/061-updates-track-the-latest-release-tag/plan.md`"
    outcome: waived
    reason: after the owner's answer (a) to 022 task 8 both carry their own terms, the bootstrap's Derived paths bounds and fetch-archive's constants, which is what the sentence says; the plans' silence is the residue that answer accepted
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: 063's review record calls the unbounded bootstrap curls 048's surface while 061's tasks attribute five of them to 061 — `specs/063-oversized-artifacts-warn-with-a-fix/review.md`"
    outcome: waived
    reason: a generated record, true of the window it described; review.md is regenerated whole and holds nothing durable
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: this repository's .pi/prompts/ductus-configure.md names .pi/prompts/ductus.md, which this repository does not track — `.pi/prompts/ductus-configure.md`"
    outcome: waived
    reason: the dogfooded .pi copies keep the generator honest; this repository is not a Pi adopter, and an adopter's copy has .pi/prompts/ductus.md beside it, which install.sh writes
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: pi's configure step restates /ductus's latest-release fetch without its grammar check, floor halts or DUCTUS_REPO notice — `framework/bootstrap/configure/pi.md`"
    outcome: waived
    reason: configure needs only the ref to fetch the bridge at; the halts guard /ductus's own writes, a bad ref fails the bridge fetch with curl's error, and the bounds are named rather than copied
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "portability: on Windows Git Bash's mktemp prints /tmp/... with no drive letter, which the staging check refuses as relative — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: the owner's answer (a) to 022 task 9 accepted the Windows temp directory as an unmeasured risk whose failure is a named halt; the scenario and plan now say why that halt is likely
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: a glob with an empty literal prefix, or a path naming the project root, walks the whole tree, honouring no ignore file — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: no shipped caller passes one, and the walk errs toward refusal, naming the link it met
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: validate_in_staging repeats enforce_manifest::resolve_contained_dir's absolute-under-base shape — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: "the two contracts differ: enforce-manifest accepts a relative path and judges an absolute one against the project root as spelled, while a staging path must be absolute and may be spelled canonically, so one helper would change one of them"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: 61ec6512's message counts eleven mutations for twelve new check sites — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: commit history; the review record states what was mutated
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "stale: 015's spec still says the bootstrap's mktemp -d -t lands under $TMPDIR or /tmp — `specs/015-tarball-fetch/spec.md`"
    outcome: waived
    reason: a narrative sentence in a done spec's procedure, not a criterion; correcting it reopens 015 for an analysis, and the bootstrap adopters run now says what mktemp -t does on each platform
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "scope: the staging check accepts any existing subdirectory of a shared /tmp — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: the owner's answer (a) bounds staging writes to the system temp directory, which this does, and /ductus passes a directory mktemp has just made
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: the macOS integration test leaks its mktemp directory if building the archive or spawning the binary panics — `runtime/tests/exec_subprocess.rs`"
    outcome: waived
    reason: it leaks only on a panic before its assertions, one empty directory under the per-user temp directory, which macOS clears
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the root-itself refusal compares a root's spelling and canonical form, so an inner root reached through a link into the outer root is accepted as a path — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: it needs a TMPDIR the operator set to name the inner root through such a link, and a path accepted there is still bounded by the outer root
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: a darwin_user_temp_dir that followed TMPDIR would still pass the macOS unit test where it runs — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: the integration test, which runs the binary with TMPDIR pointed outside the per-user directory, fails under that mutation
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: no test pins that staging_roots calls staging_roots_for — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: a one-line delegation read once per process, which no test can re-run with another TMPDIR in-process; the macOS integration test drives the real roots end to end through the binary
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "defect: the runtime's symlink resolver recurses once per component of a missing path, so a very long writeCode edit path can exhaust the stack — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: pre-existing in task 4's writeCode resolution, and this run took the staging check off that resolver; the caller is the agent that spawned the runtime, so a crash ends its own session and crosses no trust boundary (AGENTS.md's backend-rule entry)
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: no test feeds the staging check a candidate whose canonicalization fails and is then accepted — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: the staging test's not-yet-created candidates (run/a.tar.gz under a root) are exactly that case, and a refusal on a failed canonicalization would fail them
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "edge: on macOS a root spelled through more than PATH_MAX bytes of links to . is accepted as a path inside it — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: the OS cannot open such a path either, so nothing is reached through it
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "compat: on Linux an MCP server whose TMPDIR differs from the /ductus shell's refuses the shell's {tempdir} — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: the refusal names the path and the directories it was held to, and on the host measured the two environments agree
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "behavior: a root not created yet and listed after a root that holds it, or a dangling link inside a root that would resolve to it, is accepted as a path inside the outer root — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: each needs a staging root that does not exist or a link planted in one, which /ductus's own arguments never produce, and a path accepted that way is still contained by its root; the comment and plan now say exactly what is refused
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: no test pins that a staging path is matched by text before it is resolved — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: on Unix the order changes only one message, and on Windows its effect is a network contact that does not happen, which a test could observe only by contacting a host
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: write-review checks its feature and compute-review-scope checks it again inside it, and the writeCode payload walks the feature directory once per request — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: measured under 2 ms per walk on this corpus's largest feature directory (read-spec on 022, 81 entries, 5.63 ms best of 30 against 3.44 ms for --version), so the repeat costs nothing a caller sees
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: write-review loads [paths] three times per call, once itself and once each through compute-review-scope and its analyze-freshness read — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: "pre-existing composition this window did not add to: each composed reader loaded the config before the change too, and a malformed config prints one line per reader"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: dashboard and the --all prune walks now walk every feature directory — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: measured dashboard 0.23-0.25 s, prune-plan --all 0.04 s and prune-tasks --all 0.01 s on this corpus, the walks' share at most about 6 ms; refusing the whole command is the scenario's stated edge case
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: a link inside a feature directory to an ancestor makes every call on that feature walk the whole project and refuse on any outward link anywhere in it — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: only a feature directory holding a link to an ancestor reaches it, no ductus artifact is authored as a link (067's sibling-symlink-trust-boundary), the visited set walks each directory once, and following inside links is what keeps a read through two links from passing unseen
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the error for an outward link prints the absolute path the link resolves to — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: the runtime answers the operator's own agent over stdio, so no caller sits across a trust boundary, and the resolved path is what the operator needs to find the link
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: canonicalize-and-contain logic exists in both interpreter/payload.rs and primitives/mod.rs — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: "the two differ on purpose: the payload readers skip a missing or outside path and give None for a non-UTF-8 name, where the check refuses with an error and judges a missing path, so one helper would carry both behaviors"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: validate_write_code_resolution takes its resolver as a closure with one caller — `runtime/src/schema/extensions.rs`"
    outcome: waived
    reason: the closure keeps filesystem access out of the schema module, which otherwise holds shapes and their checks
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: fifteen primitives that can return InvalidPath list none in their docs — `runtime/src/primitives/read_spec.rs`"
    outcome: waived
    reason: "pre-existing: none listed it for the lexical screen they already ran, and the change gives them a new cause of that error, not a new error"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: verify-published fetches the just-uploaded assets with no retry, unlike verify-installer — `.github/workflows/runtime-acquisition.yml`"
    outcome: waived
    reason: "all 40 of the most recent runtime-release runs concluded success, so the 404 window has not failed it, and it gates nothing: both halves of the release are out when it runs"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: verify-installer ignores a Retry-After of 2^63 or more and reads a redirect hop's Retry-After when the final hop sends none — `.github/workflows/runtime-release.yml`"
    outcome: waived
    reason: an out-of-range value falls back to the jittered wait, and GitHub's release-asset redirect sends no Retry-After, so neither changes a run
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-001 does not say whether an empty value on an optional value with a default is absent or malformed — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: predates the widening for environment variables and is unchanged by it; which way an empty optional value falls is a new decision, and CFG-ENV-008 already fails a present malformed value
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: \"at startup\" in CFG-ENV-003, CFG-ENV-005 and CFG-ENV-008 does not cover a configuration file a running process reads again — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: a value first read later is validated when it is read, the reading 063 applies on every call, and no spec here reloads a file mid-process
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-008's error names the setting and the value but not the file, which CFG-ENV-003 and CFG-ENV-005 now name for a file key — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: naming the file is a new requirement on a rule today's answers did not widen; the setting and the value already locate the defect
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-CONST-004 places a value's range bounds alongside its default constant, which a required value has none of — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: predates the widening for required environment variables; where a required value's bounds live is a new decision, and the bounds must still be named constants
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 names CFG-ENV-003 and CFG-ENV-008 as owning configuration, not CFG-ENV-005 — `framework/rules/quality-cross.md`"
    outcome: waived
    reason: QUAL-GROUND-001 cedes all configuration to the CFG rules, so the omission changes no finding; the two it names are the presence and type rules its scope decision cited
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-011's Source pairs IEC 80000-13 with a NIST page that credits the binary prefixes to IEC 60027-2 — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: BIPM's VIM attributes the binary prefixes to IEC 80000-13 and NIST to its predecessor IEC 60027-2; together the two citations are accurate
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015's handshake-URL MUST NOT also bars a CSRF token there, which the OWASP WebSocket Security Cheat Sheet suggests sending in the handshake — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the owner's 2026-10-20 answer keeps every token out of the handshake URL, following the same cheat sheet's advice against tokens in URL query strings; (a)'s Origin allowlist is the handshake's cross-site defense, and a CSRF token can still be checked in the first message
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the pre-signed-URL MAY covers object storage only, so another signed-URL scheme, a CDN's or an application's own HMAC-signed link, falls under BE-API-013 and FE-STORAGE-003 — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the owner's 2026-10-20 answer scoped the MAY to a pre-signed URL for one object and one method; whether other signed-URL schemes fit that scope is a new decision
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-007's Verification asks for legitimate-recovery paths during lockout, which its Statement does not name — `framework/rules/security-backend.md`"
    outcome: waived
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-001's Statement requires frame protection on all HTTP responses while its Verification table sets frame-ancestors on HTML responses only — `framework/rules/security-backend.md`"
    outcome: waived
    reason: predates this window; OWASP's HTTP Headers Cheat Sheet, which BE-API-001 cites, says X-Frame-Options gives no security on a redirect or a JSON API response, while its REST Security Cheat Sheet recommends frame-ancestors 'none' on API responses, so aligning the two decides what the rule requires
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-018's newer-token invalidation would cancel a code issued by one method when a code by another is issued in the same recovery — `framework/rules/security-backend.md`"
    outcome: waived
    reason: a recovery that verifies the first code before issuing the second complies, since the first is already used; NIST SP 800-63B-4 sets no invalidation across codes and nothing in BE-AUTHN-018 forbids that order
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: 008's ticked task rows 13, 14 and 15 describe FE-STORAGE-003, FE-DEPS-004 and BE-AUTHN-022 as they read before later edits — `specs/008-security-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005, widened to every source, does not say whether a value read from several sources is range-checked as resolved or at each source — `framework/rules/configuration-cross.md`"
    outcome: waived
    reason: the value validated at startup is the one CFG-ENV-007's resolution yields, the reading CFG-ENV-003 states for presence, and no spec here reads a ranged value from two sources
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml`"
    outcome: waived
    reason: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "bug: a pinned pre-release would read as current once its stable release ships, since sort -V orders 1.0.0 before 1.0.0-rc.1 — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: No pin in this tree is a pre-release, and the values it is compared with are crates.io's newest stable version and published release tags, so no input reaches the misordering.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: DUCTUS_SHA256 pairs with a DUCTUS_VERSION by file rather than by step, a tab split collapses an empty version field, and cargo install's --version=X, --vers and name@X forms are not parsed — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: Each fails loud, as a hash mismatch or an unpinned report, and the tree has one install step and only the `--version X` form.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: an unknown check name exits 2, the code that also means a check could not run — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: A misspelt argument is a run that examined nothing, which is what exit 2 states, and the message names the cause.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the template's version check is close to redundant, since the sha256 pin already fixes the bytes — `framework/templates/ci/adopter-generators.yml`"
    outcome: waived
    reason: Scenario the-adopter-ci-template-can-run requires it, and it catches a mispackaged release the hash cannot.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the pins check's .yaml globs match no tracked file — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: A guard, so a workflow added as .yaml is not skipped silently.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: pass() and fail() are copied across the four scripts/tests/*.sh — `scripts/tests/test-check-runtime-currency.sh`"
    outcome: waived
    reason: There is no shared test library to reuse, and adding one for two lines is not this change's work.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml`"
    outcome: waived
    reason: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: \"shipped\" still describes the CI template in 017's AC24, plan.md's Affected Files row and 022's task 3 text — `specs/017-derive-dont-ask/spec.md`"
    outcome: waived
    reason: AC24 calls it a shipped example, the Affected Files row says adopters copy it, and 022's task 3 text is a spent task; none claims it is installed.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: cargo's own requests in checks 1, 2 and 4 keep cargo's defaults rather than a timeout the script names — `scripts/check-runtime-currency.sh`"
    outcome: waived
    reason: Pre-existing checks, unchanged in this window; runtime-currency.yml's timeout-minutes bounds the whole CI run.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: convention — check_no_protected decides which refusal to swallow by matching the refusal's message text (the words configuration directory followed by a backtick), so rewording refuse() would change what merge-permissions may write; a_config_dir_settings_file_stays_allowed and a_protected_path_is_refused hold both directions, so such a drift fails a test rather than passing. — `runtime/src/primitives/host_config.rs:375-390`
    outcome: waived
    reason: Both directions are pinned by tests that fail on a reworded refusal, and the coupling is stated in the function's comment; a typed refusal variant would be a design change to a shared surface rather than a mechanical chore.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — create-feature, given two different names under one branch-scoped identifier at the same moment, lets both create their directory with the same n, which the code comment states; the number allocation is a check-then-act that claim_feature_dir covers only for one name. The design and its record belong to spec 051, so a fix lands in specs/051-branch-scoped-spec-numbering. — `runtime/src/primitives/create_feature.rs:58-82`
    outcome: waived
    reason: The behavior is stated in the code and in 051's own plan, and it changes 051's contract, not 022's; it is another spec's to decide.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: convention — create-scenario writes through write_new_atomic, which leaves a new file at the tempfile's 0600 mode and re-applies none, where create-feature mirrors the template's mode; write_atomic gave the same mode before task 33, so the window did not change it, but a scenario lands more private than its siblings in a shared checkout. — `runtime/src/primitives/create_scenario.rs:47-52`
    outcome: waived
    reason: Unchanged by the window (the previous write_atomic gave the same mode), git records only the executable bit, and no 022 criterion states a mode for a scenario file.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — reapply_source_mode returns true both when it chmodded and when it left a symbolic-link destination alone, so a link destination whose mode was never re-applied is not named in modes-kept while its entry reads unchanged; the doc comment states the link is deliberately left alone because the bytes compared were its target's. — `runtime/src/primitives/apply_manifest.rs:484-509`
    outcome: waived
    reason: "The doc comment and the test an_unchanged_destination_that_is_a_link_is_not_chmodded_through state it as the design: the mode is a property of the file the link names, which the destination check did not judge."
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — raw_writes_allowlist counts only calls spelled write_atomic or write_atomic_bytes before the trailing test module, so a rewrite through std::fs::write, File::create, an aliased import or code after a test module is not seen; its doc comment names the first three limits, and line_ending_discipline counts guarded writers from the other side. — `runtime/src/primitives/guarded.rs:100-180`
    outcome: waived
    reason: The limits are stated in the module doc, a grep of non-test code found no std::fs::write or File::create rewriting a project file read first, and the check is a review aid, not a claim of proof.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — main.rs error paths use eprintln!, which panics when stderr is a pipe whose reader has gone, so a failing run can exit 101 instead of 1; the closed-stdout scenario scopes itself to stdout and names the eprintln sites as unchanged — `runtime/src/main.rs:455`
    outcome: waived
    reason: a-closed-stdout-is-a-quiet-exit scopes its behavior to the stdout result writers and states main.rs still writes errors with eprintln, and the run is already failing when these fire
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — the CLI flag hydration is a table of 21 hydrate_field calls and tests prove only the shared helper and two primitives, so a call dropped or pointed at the wrong argument (applied against expired waivers, say) fails no test; every_mcp_argument_has_a_cli_form pins clap presence only — `runtime/src/main.rs:339`
    outcome: waived
    reason: d3192610 records the hydration helper proved red with its assignment removed, and each of the 21 calls pairs a flag with its same-named field, so a swap shows in review as a name mismatch
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: perf — each assessSpecQuality request reads and canonicalizes the spec once and runs the read-size check twice (fits in the builder, then again in bound_lists), each of which reads and parses the project config, so a 192-rule walk does about 400 config parses — `runtime/src/interpreter/payload.rs:1329`
    outcome: waived
    reason: each parse is a sub-millisecond read of one small file and the walk is bounded at 192 requests, each of which waits on a host round trip that costs seconds
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — session-file tier resolution (newest_existing) probes with exists(), which follows links, so a dangling link at the newest tier reads as absent and the next older tier is chosen — `runtime/src/schema/paths.rs:158`
    outcome: waived
    reason: a link leaving the project is refused by session::contained before any open, and a dangling link at the newest tier is the contributor's own repair to make; no rule or contract of 022 names the case
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — scripts/mutation/session-file-containment.py records mutation M13 (the ductus exec seed in runtime/src/main.rs) as 'proved by hand with cargo test --bin ductus' because run.py runs only --lib and --test filters. The killing test exists (main.rs an_exec_seed_refuses_a_shared_default_linked_outside_the_project, line 1820) and the call it guards is at main.rs:775, but the proof cannot be re-run, so a later edit that drops that contained() call is caught by the test alone, with nothing showing the test fails without it. — `scripts/mutation/session-file-containment.py:1-12`
    outcome: waived
    reason: The gap is stated in the list's own header and the killing test is in the tree; extending run.py to bin tests is new harness work, an observation about the pipeline's own machinery made while reviewing framework work, which AGENTS.md Workflow waives with its reason.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — scripts/mutation/command-file-containment.py (M6-M10 are unix-only) and scripts/mutation/installed-file-modes.py (M7 and M8 macOS-only, M5 a 50 ms ctime wait) leave EXPECTED_SURVIVORS empty, so run.py exits non-zero on a platform where those link or chflags tests do not run; EXPECTED_SURVIVORS is defined as the survivors no portable test can kill, and these are platform-bound. Each list's docstring states the bound. — `scripts/mutation/command-file-containment.py:22-27`
    outcome: waived
    reason: The bound is stated in each docstring and the lists run on the maintainer's macOS tree, where all mutants are killed (10 of 10 per the commit); the harness's platform handling is the pipeline's own machinery.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — session-lock-backoff.py filters every mutant with the substring 'session::' and session-file-containment.py with 'session::tests::', which also match write_session and migrate_session_file tests, so a mutant counts as killed when any matching test fails, not necessarily the named killing tests the docstring lists. — `scripts/mutation/session-lock-backoff.py:30-90`
    outcome: waived
    reason: Every mutant here edits runtime/src/session.rs, whose own tests are the ones the docstring names; the broader substring only widens the net, and the list's purpose, proving some test fails, holds. Harness machinery.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — runtime/tests/line_ending_discipline.rs without_test_module cuts a file at its first '#[cfg(test)]' of any kind, so a '#[cfg(test)]' helper placed mid-file (bounded_lists.rs line 225, rule_verdicts.rs line 41 today) hides every writer call after it from the scan; no scanned file has a writer after such a helper now, and the corpus-size guards would not notice one. — `runtime/tests/line_ending_discipline.rs:126-131`
    outcome: waived
    reason: "Latent: checked 2026-11-16, no primitives file holds a text-writer call after a non-module '#[cfg(test)]' item. The file's header already states the lint is per file and not per function."
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — scripts/check-runtime-currency.sh accepts 'npm install -g name@1.2.3' as a pin (task: 347d985b), and the script's comment only requires an exact version; unlike cargo install, whose crate cargo verifies against the registry checksum, npm's exact version carries no checksum the build verifies, which SEC-DEPS-001 lists as the immutable-reference form for a downloaded tool. Its one user is windows-agent-session.yml (outside this manifest), installing @anthropic-ai/claude-code@2.1.325 for a measurement job. — `scripts/check-runtime-currency.sh:324-336`"
    outcome: waived
    reason: The subject is a check script reading a workflow another reviewer holds; an exact npm version is the immutable-by-registry form this script already accepts for cargo and pip, and whether the agent-session workflow's install meets SEC-DEPS-001 belongs to the review of that workflow.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: convention — a1683fd8 (task 21), 74f6361c and fbb62dfe, bca9a2fd and b1d319f1 edit migration procedures or registry entries an adopter may already have run (constitution-relocate, ductus-rename, govern-dir-consolidate, criterion-label-backfill, inbox-guidance-refresh, runtime-store-path), where AGENTS.md says to add a later entry instead; each commit states the already-run adopter is converged (task 21) or that a later entry (strip-inbox-guidance) covers it, and the edits serve adopters who have not yet run them — `framework/migrations.toml:1`
    outcome: waived
    reason: Observation about the pipeline's own machinery made reviewing framework work; the commits record why no new entry is needed (already-run adopters converge; strip-inbox-guidance covers inboxes that ran the old one), and none of the edits changes what an already-run adopter's tree holds.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — the bootstrap's curl downloads write to an unquoted -o {tempdir}/... operand; a TMPDIR containing a space would split it, though mktemp -d -t directories on macOS and Linux carry none — `framework/bootstrap/ductus.md:410`
    outcome: waived
    reason: Observation about the pipeline's own machinery; the URLs are quoted, the tempdir is a fresh mktemp directory the step creates, and the runtime path does not use these commands.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — AGENTS.md is 49,987 bytes against the 50,000 read size, so the pre-commit hook refuses any commit that grows it by more than 13 bytes. Not a violation; the gate already exists. — `AGENTS.md`
    outcome: waived
    reason: AGENTS.md is under its limit, and the hook's refusal of a commit that grows it past the limit is the mechanical gate; nothing is violated.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: perf — apply-manifest reads each update or create source twice (the placeholder pre-pass, then the walk or the host-destination check) when the substitutions map is non-empty — `runtime/src/primitives/apply_manifest.rs:158-162`
    outcome: waived
    reason: the an-unsubstituted-placeholder-is-not-a-clean-run scenario states the cost (one extra read per entry, a pre-write pass over every entry) and the owner chose it on 2026-11-12; sources are small local files
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — the REWRITE_SPEC_LINKS description in descriptions.rs states every behavior a listed commit changed except the project-root prefixes (a leading ./ or / in a root markdown file, ./ in the hook, ../ or / in system.md, kept as written) that 38fc39f6 and 54715126 added; it says only spec-root paths named in the referrers. Every claim it does make was checked against the code and holds, including the five lists, counts and refused semantics; the parenthetical that a spec root that is itself a link is followed holds only inside the project, where a root resolving outside is refused as an invalid path (driven on a fixture) — `runtime/src/schema/descriptions.rs:94`
    outcome: waived
    reason: "an omission, not a false claim: the prefix forms are spec-root paths the description already names, 067's data model states the boundary, and the outside-the-project refusal is the primitive's documented InvalidPath error"
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: perf — the sweep reads every markdown file under the spec root into memory before the first write and runs the per-line rewrite and the mention scan over all of them, though only files containing the retiring name can change; a contains check on the name would skip the per-line allocation for the rest — `runtime/src/primitives/rewrite_spec_links.rs:805-866`
    outcome: waived
    reason: the whole-corpus read before the first write is 051's scenario the-spec-root-is-read-before-the-first-write, and the per-file scan is linear in a corpus of hundreds of files run once per fold; no figure or spec states a time bound this misses
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — 92120f45's one-line change at this site (Paths::load now propagates a malformed config) has no test in this file; the halt was verified by driving the built binary on a fixture whose config.toml does not parse (exit 1, error naming the file), and the shared reader's halt is tested by a_document_that_does_not_parse_halts_naming_the_file in schema/paths.rs — `runtime/src/primitives/rewrite_spec_links.rs:684`
    outcome: waived
    reason: Paths::load returns a Result, so the call cannot use the layout without handling the error, and a fallback here would be a visible edit; the commit records its own tests and the M224 mutant for the shared change
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — append-inbox strips a caller-supplied leading marker from its text, but remove-inbox-item compares its item trimmed and unstripped, so text appended with a marker cannot be removed with the same argument (removed: false, no error); the round-trip test uses marker-free text. — `runtime/src/primitives/remove_inbox_item.rs:32`"
    outcome: waived
    reason: remove-inbox-item takes the bullet text as groom reads it from the inbox, which carries no marker; the scenario's normalization applies to the append primitives only, and a no-match is a stated clean outcome.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: bug — label-criteria computes max_label + 1 and counter += 1 on u32 values parsed from a spec, and the release profile sets no overflow-checks, so a hand-written label of 4294967295 wraps to 0 in release (and panics in debug). — `runtime/src/primitives/label_criteria.rs:94`
    outcome: waived
    reason: a criterion label near the u32 limit is hand-written nonsense in a committed spec, not a count any spec reaches; the wrap would assign label 0 rather than corrupt another label.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "security — create-feature and create-plan-artifacts read their template through resolve_template, which probes framework/templates/spec/ and the spec root's templates directory with no containment, so a template that is a symbolic link leaving the project (a tracked link in a cloned repository, since the candidate names are fixed: spec.md, plan.md, tasks.md, data-model.md) is read and copied into the new artifact. The feature directory is contained and walked; the template location is not. — `runtime/src/primitives/mod.rs:4294`"
    outcome: waived
    reason: the read reaches only a file of one of four fixed names and its content lands in a file in the project the operator is already creating; resolve_template is in mod.rs, outside this part, and the plan's Path arguments names path arguments and committed-text paths, which a fixed template name is not
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — check-artifacts' agent_scaffold_destinations lost its project-scope filter and its home-relative filter in b1d319f1 (PROJECT_SCOPES and the starts_with tilde test): every registered agent's MCP target now counts as a path shipped to adopters. With Claude and Pi, both project-scoped, the result is unchanged, but a future host with a user-global target would make a criterion naming a home path skip as shipped instead of being examined. — `runtime/src/primitives/check_artifacts.rs:1620`"
    outcome: waived
    reason: the guard served hosts that are no longer registered; the two that remain are project-scoped, so the filter checked nothing the registry can now hold, and the test the_registry_arm_derives_each_agents_scaffold_paths pins the current set
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — pipeline_view::config_items reads config.unknown_keys with unwrap_or_default, so a DashboardConfig in the dashboard file form (keys moved to the lists file, field None) would render no Unknown config keys line and nothing would say so; the code relies on the comment that the model is built only from the always-Some load_config result. — `runtime/src/primitives/pipeline_view.rs:586`
    outcome: waived
    reason: the only constructors are load_config (always Some) and tests; dashboard's file form clears the field on the result after the model is built, and the view renders from the model, so no reachable path passes None
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: perf — render_help::run loads the host twice, once inside guide and once for the project name, so the config and session files are each read and parsed twice per call. — `runtime/src/primitives/render_help.rs:36`
    outcome: waived
    reason: two reads of two small files per help call, unmeasured; guide is also called directly by the help hook (hook.rs), so passing a loaded host into it would change that call
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — load_scope_files and load_rule_files skip a path that is missing, outside the project or not UTF-8 text, so performReview's scope-files can hold fewer files than the scope the pass was asked to cover with nothing on the request naming the omission — `runtime/src/interpreter/payload.rs:566`
    outcome: waived
    reason: the omission is the documented posture of the builder ('the pass reviews what it can read') and review-runtime-acceleration states the request carries the in-scope files; the walker context's scope and selected lists travel in the legacy tail (or the lists file past the bound), so the host can compare them with scope-files, and compute-review-scope sets deleted files apart before the request is built
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: security — the .gitignore layer of the writeCode secret guard answers 'not ignored' when libgit2 errors (is_gitignored unwrap_or(false)) and is skipped when the project sits in no readable work tree, so a check that could not run looks like one that passed; secret_pattern on the file's basename and its canonical target stays the floor — `runtime/src/interpreter/payload.rs:1964`
    outcome: waived
    reason: the degradation is stated on is_gitignored and on load_plan_relevant_files (gitignore is an opt-in second layer, the secret-pattern check the floor) and writecode-payload-bundling requires no failure mode for the layer; the guard only refuses, so a skipped layer sends no more than the plan's own listed files inside the project
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — routeFold's load_fold_source_scenarios gives a scenario whose section frontmatter cannot be read an empty section (read_scenario_section unwrap_or_default), the same unreadable-looks-empty shape unreadable-scenario-is-reported names for read-spec — `runtime/src/interpreter/payload.rs:1136`
    outcome: waived
    reason: the scenario is still listed by slug, so the fold's host sees it exists, and the section is context the route decision does not key on (the scenario crosses over unchanged, as the builder's doc states); unreadable-scenario-is-reported governs the open-question collector, which this reader is not
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: perf — build_perform_review_request runs load_scope_files and load_rule_files in each of the five passes, so the whole scope and all loaded rule files are read from disk five times per review although the loaded-rule-files list needs only names and rule-files only the pass's own files' contents — `runtime/src/interpreter/payload.rs:463`
    outcome: waived
    reason: local file reads once per human-timescale review pass, against an LLM call that dominates; the performance-backend rules are design-time commitments for a deployed service and do not fire against a local stdio runtime
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — MARK_CRITERION, SET_STATUS (and MARK_TASK) say the call is refused with changed-since-read when the file changed between the read and the write, but a flip or status change that changes nothing writes nothing (mark_criterion.rs and set_status.rs guard the write with previous != checked and to != from), so such a call succeeds whatever another writer did; only MARK_TASK states the no-op writes nothing. — `runtime/src/schema/descriptions.rs:70`
    outcome: waived
    reason: a call that writes nothing has no write for the clause to refuse and leaves no lost update, since the file already holds the requested state when read; the clause is stated for the calls that write
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: convention — about thirty descriptions repeat the same past-the-read-size sentence (form is file, lists-file names the JSON file under .ductus/results/ holding the moved lists whole) and ten repeat the changed-since-read sentence in near-identical wording; the copies already differ (DASHBOARD's variant above), and a macro or shared fragment would hold one. — `runtime/src/schema/descriptions.rs:26`
    outcome: waived
    reason: each description is the one text an MCP host and the --help output show for its primitive (the file's own header), so it must stand alone; a concat-style fragment would change how every constant is built and none of the copies is wrong except the one named above
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — a stdout reader that has gone is a quiet exit 141 for every result written through write_stdout_line, but ductus exec (protocol envelopes written by the interpreter and emit_exec_error) and gate-confirm still report it as exit 1 or 74 with a stderr message. Driven on a fixture: resolve-session exits 141 with empty stderr; exec of an unknown command exits 1 and prints 'failed to emit error envelope: Broken pipe'. No panic in either, and the scenario a-closed-stdout-is-a-quiet-exit scopes the fix to the result emitters in main.rs. — `runtime/src/main.rs:733-748`"
    outcome: waived
    reason: the scenario's Context names the result emitters in main.rs as the sites and its test covers three of them; exec's stdout is the walk's protocol channel, where a closed reader ends the walk with an error status distinct from a panic, which the scenario's second behavior requires, and nothing is written twice
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — the 21 --{name}-json flags are each hydrated by one hydrate_field call in main.rs (checked: 21 flags in schema/primitives.rs, 21 calls, every flag paired with its own field), but only process-waivers' fired-json and write-review's fix-rounds-json are driven by a test; every_mcp_argument_has_a_cli_form checks that a flag exists, not that main.rs reads it, so dropping a hydrate_field call for any of the other 19 passes the suite and silently sends an empty list. The commit message records one mutation proof only. — `runtime/src/main.rs:1719-1814`"
    outcome: waived
    reason: scenario every-mcp-argument-has-a-cli-form requires one test that every MCP argument has a CLI form, which every_mcp_argument_has_a_cli_form meets, and the hydration test the commit adds proves its behavior for two flags; the other pairings were read and are correct today, and no loaded rule's Statement is violated by the gap
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: bug — ignore_everything writes the self-ignoring .gitignore of a lock directory under whichever lock the caller holds, and .ductus/locks now holds two lock files (append-task.lock and compare-and-set.lock) that do not exclude each other, so two first-use callers (append-task, and a mark-task or other guarded writer) can both find the file absent and rename onto it at once; the function's own comment says Windows refuses that rename with os error 5, so the loser fails. Unix lets the last rename win. Not run on Windows; read from lock_file_waiting and its two callers. — `runtime/src/session.rs:339-345`
    outcome: waived
    reason: needs a fresh tree, two different writers inside one instant and Windows; both writers write identical bytes and the failed writer's operation fails with an Io error naming the file and succeeds on the re-run the error calls for; no run has observed it
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: security — ductus parse FILE reads any path it is given with no containment and prints the parsed procedure (the whole AST, content included) when the file is Procedure-shaped markdown; every primitive path argument goes through validate_contained. The subcommand is CLI-only (excluded from the MCP registry by NON_PRIMITIVE_SUBCOMMANDS) and is driven by scripts/lint-procedure-parseability.sh over framework/commands. — `runtime/src/main.rs:635-681`
    outcome: waived
    reason: a local CLI debug and CI surface the MCP server does not expose, supplied by the operator or a script, and an agent that can run it can already read the same file with its shell; no host-to-runtime trust boundary lies between the path's supplier and its reader, and the plan's path-argument section scopes the check to primitives
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — the scenario commands-dir-parameterization says a project name that is not filesystem-safe is out of scope because the runtime can trust it, while Host::load now refuses a project or cli-config-dir that is not one path component (task 40, named in the plan section Path arguments) — `specs/022-deterministic-runtime/scenarios/commands-dir-parameterization.md:61`
    outcome: waived
    reason: the edge case concerns the characters the bootstrap emits for the project name, which stay lowercase ASCII, and the runtime's own one-component screen is stated where the plan section Path arguments now names it, so no reader is misled about what the runtime enforces
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — host.rs, paths.rs, session.rs, check_artifact_size.rs, discover_rule_files.rs, fix_rounds.rs, check_orphaned_references.rs and dashboard.rs each hand-roll the parse-then-PrimitiveError::Toml-naming-the-file step, with no shared helper — `runtime/src/host.rs:131`
    outcome: waived
    reason: each site names a different typed target and a different absent-file rule, so a shared helper would take the target type and the absence policy and save one map_err per site; a refactor across eight files that adds no requirement
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: perf — a performReview or writeCode request is serialized and measured twice per build, once by the body step and once by bound_lists, and each measure re-reads the config for the read size — `runtime/src/interpreter/payload.rs:507`
    outcome: waived
    reason: one request per host round trip, each measured twice at the size of the request itself, against a model call that takes seconds
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — main.rs reports its failure paths with over thirty eprintln! calls, which panic when the stderr reader has gone, where the stdout writes go through write_stdout_line (task 27) and warn_unknown_keys writes stderr without panicking — `runtime/src/main.rs:285`
    outcome: waived
    reason: each call is on a path that is already failing, so a closed stderr changes the exit status from 1 to a panic's 101 on a run that had failed anyway, and no result or write is lost
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — the two CLI json-flag tests drive 2 of the 14 hydration arms task 24 added (fired-json and fix-rounds-json), so deleting or mis-wiring another arm leaves both green while the flag is ignored; the commit records a mutant for each test, not for each arm — `runtime/src/main.rs:1774`
    outcome: waived
    reason: each omitted arm is one hydrate_field call whose flag and field names were read pairwise here and agree, and the commit records how each test was shown to fail
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — M7 to M18 (askClarifyQuestion and verifyCriteria mutants) list a_request_exactly_at_the_read_size among their killing tests, but that test builds only assessSpecQuality, so it cannot kill them; M1 to M37 also list a_malformed_read_size_halts_* tests that kill only the swallow mutants. The over-listed filters only add test runs and name no behavior that does not exist; the killing tests of each mutant are named too. — `scripts/mutation/exec-requests-bounded.py:30`
    outcome: waived
    reason: Harmless over-listing in a framework mutation list, the pipeline's own machinery; every mutant checked also lists a test that kills it, and removing a filter changes no verdict.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — c01b86e0 (rmcp 3.5.1 to 3.7.0, tokio 1.54.1, typed-path 0.12.3 to 0.13.0, 23 packages) records only that cargo build --release --locked is clean; the other lockfile commits in the window each record clippy and the test suite. The pre-commit hook runs both when runtime/ is staged, and every later bump in the window records them. — `runtime/Cargo.lock`
    outcome: waived
    reason: A commit-message evidence gap on a lockfile-only change the hook gated; the lockfile at HEAD is exercised by every later gated commit, and no dependency pin policy (BE-DEPS-002) is broken.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — runtime-release.yml installs cargo-zigbuild and cargo-cyclonedx by exact version with cargo install --locked, not by commit or hash, while ziglang carries a sha256; the comment above the zig step says both are pinned under SEC-DEPS-001 — `.github/workflows/runtime-release.yml:187`
    outcome: waived
    reason: cargo checks a crate against the checksum in the registry index and a crates.io version cannot be overwritten, as 022's plan Release workflow section states; the pins change only through scripts/check-runtime-currency.sh.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — install.sh copies the mktemp file to the installed bootstrap, which therefore lands at mode 0600 (verified on a fixture), while the settings seed beside it is written at 0644 — `install.sh:203`
    outcome: waived
    reason: git tracks only the executable bit and the file is read by its owner's agent; no behavior or contract depends on the mode.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: tests — after b1d319f1 ported the Auggie fixture to a Pi-shaped one, no exec subprocess test reaches the claude-style commands/<project>/ candidate through ductus exec; the candidate order is asserted in host.rs (command_file_candidates_cover_claude_style_then_pi) and the installed claude-style path through the MCP server in tests/mcp.rs — `runtime/tests/exec_subprocess.rs:446-500`
    outcome: waived
    reason: locate_exec_command walks a candidate list and is agnostic to which layout produced a candidate, and that list is asserted at the unit level, so the end-to-end exec path loses no distinct branch.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — scripts/audit/permission-entry-shape.sh counts the extracted entries of the allow and deny sections together (entry_count), so a deny section whose bullets stopped parsing leaves a clean result while allow still yields entries; its closing line reports the combined count, a count of what was examined, so the result is not a bare zero — `scripts/audit/permission-entry-shape.sh:205-208`
    outcome: waived
    reason: Observation about the pipeline's own audit machinery, made while reviewing framework work; the script reports the combined count it examined and locates both section headings, failing when either is absent, and the deny section's bullets are the same grammar as the allow section's.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — runtime.yml's build-and-test job runs npm ci on three images with no npm audit signatures step, where framework-checks.yml runs one after its npm ci; the installed markdownlint tree is a test input, shipped in no artifact — `.github/workflows/runtime.yml:173-175`
    outcome: waived
    reason: BE-DEPS-004 asks for the verification before a dependency is built into or shipped with an artifact; the tree installed here is a test tool, ships in none, and framework-checks.yml verifies the same lockfile's signatures on every push and pull request.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — the mutation lists root-prefixed-references.py and rewrite-spec-links.py filter by module path (check_orphaned_references:: and rewrite_spec_links::), so a kill shows that some test of the module fails, not that the test each list's docstring names does; their anchors all match exactly once (run.py --check: 46 and 54 mutations, 0 invalid), but the kills themselves were not re-run here (no cargo) — `scripts/mutation/root-prefixed-references.py:1-43`"
    outcome: waived
    reason: The lists' own docstrings say the named tests are among the killers and the module filter is the harness's documented form (run.py usage); each mutation's replacement was read against the code it anchors on and removes the behavior its name states.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — scripts/mutation/run.py never runs the unmutated filter first, so a red baseline (a pre-existing failing test, a flaky one) reads every mutant as KILLED; a compile error is distinguished (ERROR) but a failing baseline is not. The list's commits record green runs, so no current verdict is known wrong. — `scripts/mutation/run.py:72`
    outcome: waived
    reason: the mutation harness is the pipeline's own machinery, reviewed here as framework work; no verdict in this window is shown wrong by it and each landing commit records the lists run green
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "convention — line_ending_discipline.rs without_test_module cuts each primitive file at its first #[cfg(test)] marker, as the known raw-write allowlist test does for guarded.rs; a writer placed after a mid-file test item would go unexamined. Measured over runtime/src/primitives: no file has a write call only after its first cfg(test) marker, so nothing is hidden today (rule_verdicts.rs, bounded_lists.rs, guarded.rs, mod.rs and write_review.rs carry a marker before non-test code). — `runtime/tests/line_ending_discipline.rs:126-132`"
    outcome: waived
    reason: "latent only: no primitives file has a text write after its first test marker today, measured by scanning every file; same class as the known allowlist-test finding, which its owner is already disposing"
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: convention — FE-TIMING-001's Verification yields a session lacking an idle or absolute timeout to BE-AUTHN-005 (security-backend.md) with no 'where security-backend.md is also loaded' condition, which the 008 data-model's Verification phrasing item 6 asks of a frontend rule yielding to a backend owner. The case it names is one FE-TIMING-001 never flags (it flags a spec that introduces a limit and commits to none of turn-off, adjust or warn-and-extend), so no case is lost when security-backend.md is absent. — `framework/rules/accessibility-frontend.md:319`
    outcome: waived
    reason: "A rule-text edit on a rule surface 008 owns, in a rule file whose change the owner decides, and the clause routes nothing: the yielded case is not one this rule flags. Observation about the pipeline's own rule machinery made while reviewing framework work."
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — FE-CSRF-002 and FE-STORAGE-002 yield a wrong or unjustified SameSite value to BE-AUTHN-014 where security-backend.md is loaded, but BE-AUTHN-014's Verification flags 'SameSite=None without Secure and a justification', which can be read as flagging only when both are missing, so SameSite=None with Secure and no justification may be reported by neither rule when both files load. Low confidence: its trigger sentence also says MUST justify any SameSite=None. — `framework/rules/security-backend.md:119`"
    outcome: waived
    reason: The wording sits in security-backend.md, outside this part's listed commits, and its fix is an 008 rule-surface edit the owner decides; the trigger sentence of the same Verification already requires the justification, so the reading that leaves a gap is the less likely one.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — FE-STORAGE-003's Statement and Verification restate the OpenID hint, front-channel logout and public-key MAYs and the carve-out routing that BE-API-013 and SEC-DATA-001 also state, three copies of one set of carve-outs that can drift apart. The text arrived in commits after the two this part lists (the listed 74f6361c changed two Verification lines of FE-CSRF-002 and FE-STORAGE-002 only). — `framework/rules/security-frontend.md:141`
    outcome: waived
    reason: Owner's decisions of 008 tasks 30 and 31 placed each carve-out in the rule that owns the case, with the frontend and backend copies kept so a project loading one file keeps it; the commits that grew it are 008's, not 022's.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — FE-LOAD-007's closing sentence names BE-PAYLOAD-003 and BE-CACHE-004, owners in performance-backend.md, with no 'where performance-backend.md is also loaded' condition, which 008 data-model Verification phrasing item 6 asks of a yield from a frontend rule to a backend owner — `framework/rules/performance-frontend.md:129`
    outcome: waived
    reason: "the sentence marks scope, not a yield: FE-LOAD-007's flag set is the cache policy of build-emitted assets and the HTML that references them, which never includes an uncompressed asset or another server response, so with performance-backend.md unloaded no case goes unflagged; the Not-flagged exceptions that do yield (BE-API-001) carry the condition. The pipeline's own rule-file machinery, not 022's contract."
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — BE-API-001's Not-flagged clause says a missing Permissions-Policy or Cross-Origin-Opener-Policy on an HTML response is one this rule does not cover and security-frontend.md's FE-CSP-007 and FE-CSP-009 do, without a load condition, though item 6 of 008 data-model Verification phrasing asks one of a backend rule naming a frontend owner — `framework/rules/security-backend.md:497`
    outcome: waived
    reason: the clause states what BE-API-001 does not cover, so no case is yielded and none is lost when security-frontend.md is unloaded; the yields in the same clause (CSP, frame-ancestors, Referrer-Policy) carry the 'where security-frontend.md is also loaded' condition. The pipeline's own rule-file machinery, not 022's contract.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — QUAL-STUB-001's Verification yields a published API schema that can diverge from its implementation to api-backend.md BE-SCHEMA-002 with no load condition, though quality-cross.md is a cross file and the owner a backend file; item 6 of 008's data-model Verification phrasing (added in 74f6361c) asks such a yield to read where api-backend.md is also loaded, so a project without that file keeps the case. 74f6361c conditioned only BE-CACHE-004's yield and CFG-ENV-006's; QUAL-STUB-001's predates the item and was not swept. — `framework/rules/quality-cross.md:11`
    outcome: waived
    reason: A rule-file wording gap in a rule owned by spec 036 and the rule-format owned by 008, found while reviewing 022's window of rule-file edits; it is pipeline machinery, not 022's runtime, and 022's code does not break its contract. Waived per AGENTS.md Workflow (pipeline machinery observations are waived with reason). The fix would land in 036's rule text, another spec's directory.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — Instructions step 5 and 6 call merge-managed-block, whose description now says a changed-since-read refusal is cured by running it again, while the State A rule (line 340) says a primitive refusal halts the run and never falls back; a changed-since-read error names the file, so by line 340's own test it halts the run rather than re-running the one step. No window commit of this file introduced the tension (the guard is 022 task 38) — `framework/bootstrap/ductus.md:340`
    outcome: waived
    reason: Halting names the file and leaves the run restartable; /ductus is idempotent, so the operator's re-run is the description's run it again one level up, and nothing proceeds on unwritten state.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "security — the seeded Bash(.ductus/bin/ductus enforce-manifest *) deletes any file matching its glob inside the project with no prompt (checked on a fixture: a directory under the project lost its markdown file; a directory outside the project was refused for its parent-directory component); plan section Agent reach decides only run-another-command, write-outside-project and host-settings routes — `framework/bootstrap/ductus.md:64`"
    outcome: waived
    reason: Step 8 runs it, it is bounded to the project by the traversal check and the protected-destination screens, and a deletion inside a git work tree is recoverable; the plan's three categories do not include project-internal writes.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: security — Pointer materialization steps 3 and 4 put the Branch 1 [runtime] path into the ln -sf and cp command strings with no quoting or character rule, where {pin} and the ref get an explicit grammar for the same reason (later steps interpolate them); Branch 1 step 3 already executes that path with --version, and the seed grants neither command, so each prompts on Claude — `framework/bootstrap/ductus.md:461`
    outcome: waived
    reason: The committed path already names a binary this run executes (048's stated escape hatch for a project-supplied runtime), so a quoting rule adds no boundary the executing step does not already cross.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: security — the framework archive, whose files include .githooks/pre-commit that runs on every commit, is fetched with no digest check (fetch-archive returns verified false without a sidecar, and --ref=main names a moving ref), where the runtime binary is held to a missing-sidecar-is-a-failure rule — `framework/bootstrap/ductus.md:32`
    outcome: waived
    reason: "Recorded design of 015, 048 and 061: GitHub's source tarballs carry no sidecar, the ref is chosen by the operator, and the fetch is HTTPS-only with TLS 1.2 or later; no commit in this window changed it."
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: perf — framework/bootstrap/ductus.md is 159,893 bytes read into context on every /ductus invocation (line 845 says so), and the Permission Setup paragraph at line 163 alone is about 4.6K characters restating plan section Agent reach (the xargs disclosure, the curl and tar rule, the pointer-link prompt); the commits in this window grew that paragraph while removing three hosts' text elsewhere — `framework/bootstrap/ductus.md:163`
    outcome: waived
    reason: Plan section Agent reach places each disclosure in this section, so the paragraph is required content; the archive half already moves nine sections out of the file (spec 056) and the line-163 restatement is the disclosure a gate reads.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — analyze.md step 11 (4cc321b9) says a request carrying no loaded-rule-files list makes a load condition false, but a later bounded-request change (task 36/37) moves that list to the file named by request-lists-file once the request is past the read size, leaving only loaded-rule-files-count inline; the step text and data-model line 649 do not say to read it from there. Reachable only when the request is still past the read size after the spec body is dropped, since the list is a handful of basenames, so a default configuration never meets it. — `framework/commands/analyze.md:66`
    outcome: waived
    reason: An observation about the pipeline's own machinery made while reviewing framework work. The list is a few rule-file basenames and moves only when the whole request stays past the read size after its bodies are dropped, which the default 50,000-byte bound does not reach; the data model already tells a host that request-lists-file marks the moved-list form.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "bug — ductus-procedure.md migration step 2 (a1683fd8) replaced the atomic git mv with mv followed by git add -A for a tracked file, but the recovery branches cover only a failed rename. If mv succeeds and the staging command fails (index lock held, say), the branch text that says the destination does not exist and the adopter ends with both files is false: the inner file exists, the outer path is empty, and only the index is behind. — `framework/bootstrap/ductus-procedure.md:277`"
    outcome: waived
    reason: An observation about the pipeline's own machinery made while reviewing framework work. The worktree ends correct and complete (inner file in place, outer stub written by the create strategy) and git status shows the move as a delete plus an untracked file that the adopter's next add records; the missing line is a new message, not a mechanical edit.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: security — ductus-procedure.md Hook Installation case 5 and the manual snippet interpolate H and P (from git rev-parse --show-prefix, the project's directory path) into git config core.hooksPath {H} and ./{P}.githooks/ductus-pre-commit without quoting; a subdirectory project whose path holds a space or shell metacharacter would split or alter the command the agent runs (BE-INPUT-003's vector-argument requirement, applied as in 064's quoting fix for the release URL). At the work tree root H is the constant .githooks. — `framework/bootstrap/ductus-procedure.md:254`
    outcome: waived
    reason: BE-INPUT-003 does not fire against a markdown step an agent renders, and the path is the operator's own project directory, not caller input. Quoting only the non-empty-P case is a design choice, since the seed's exact rule for git config core.hooksPath .githooks matches the unquoted constant and a blanket quote would make every root project prompt; it is not a mechanical edit.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: reuse — amend.md step 3 of Confirm target and analyze.md step 6 each restate near-verbatim the derive-dependencies handling of unwritable-count and unparseable-count (name each spec from the list or the lists file, edit the key by hand, never read as in sync). — `framework/commands/amend.md:44`
    outcome: waived
    reason: An observation about the pipeline's own machinery made while reviewing framework work. Each command source is read alone as its own instruction set, so a shared reference section would add a lookup step to a safety-net step; the two copies are one primitive's contract and are checked against its description.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: convention — framework/migrations/ductus-rename.md (b1d319f1) no longer converts the auggie, antigravity and opencode registrations and permission entries, so an adopter on one of those hosts who has not yet run the migration gets no conversion. The owner dropped those hosts on 2026-11-11, and an adopter who already ran the entry is unaffected, so no new registry entry is owed. — `framework/migrations/ductus-rename.md:44-62`
    outcome: waived
    reason: host support was dropped by the owner's recorded decision (commit b1d319f1); the migration serves the supported hosts, and an adopter who ran it is already converged
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — fold.md steps 7 to 11 and consolidate.md step 4 and their resumption tables restate rewrite-spec-links, create-scenario and retire-feature behavior (refused-count protocol, kept-tails, the target-is-a-home refusal) that the primitives' descriptions already carry. The copy is accurate against the descriptions read today; it is a drift surface the constitution's pointer-not-copy rule discourages. — `framework/commands/fold.md:73`
    outcome: waived
    reason: an observation about the pipeline's own command text made while reviewing framework work; the restated claims were checked against the compiled descriptions and hold, and no contract of 022 is broken
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — a-closed-stdout-is-a-quiet-exit Context says in the present tense that the result emitters in main.rs write with println! and nothing handles a broken pipe; task 27 replaced them with write_stdout_line (exit 141) and closed_stdout.rs proves it, so Context now describes the defect as reproduced on 2026-10-26, not the tree — `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md:8`
    outcome: waived
    reason: The Context narrates the defect as reproduced and dated, and the Behavior section, which is the requirement, holds against write_stdout_line and runtime/tests/closed_stdout.rs; rewriting the narration adds no requirement.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — migrations constitution-relocate, govern-dir-consolidate, criterion-label-backfill and inbox-guidance-refresh were edited in place (a1683fd8, fbb62dfe, 74f6361c, bca9a2fd), which AGENTS Gotchas say reaches no adopter who already ran them; the first three change only how a move is spelled or how the label rule is worded, and an adopter who ran them is converged, and bca9a2fd's population is reached by the later strip-inbox-guidance entry (migrations.toml), as the gotcha asks — `framework/migrations/inbox-guidance-refresh.md`
    outcome: waived
    reason: Each edit changes text a re-run migration reads, not an outcome a finished run produced; the one outcome that differed, a comment left in an inbox that had run inbox-guidance-refresh, has its own entry ordered after it (strip-inbox-guidance), so the gotcha's remedy is in place.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — inbox-guidance-refresh step 4 and strip-inbox-guidance step 5 carry the same blank-line removal algorithm in prose, so a later fix to one can miss the other — `framework/migrations/inbox-guidance-refresh.md:28`
    outcome: waived
    reason: A migration is read alone by the host on an adopter whose last_applied sits anywhere in the registry, so each procedure must state its own steps; a shared section would make one migration depend on another's file, which the registry's per-entry procedure_file model does not allow.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — the scenario's process-waivers summary (expire when the file is gone or the rule no longer fires in a file the run's scope holds) and the review reference's Per-run waiver processing items 2 and 3 omit two carve-outs the primitive's description states: a plan Affected Files entry the scope holds counts as reading a file, and a scope that cannot be read retains a waiver even for a deleted file. — `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md:23`"
    outcome: waived
    reason: Both are summaries of a rule the primitive's compiled description states whole and canonically (PROCESS_WAIVERS); the omissions narrow neither the scenario's tests nor the reference's steps, and a restatement of the carve-outs would be a second copy of a canonical source.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — AGENTS.md is 49,987 bytes against the 50,000-byte read size, 13 bytes of headroom, so the next growth of the file is refused by the pre-commit hook. — `AGENTS.md`
    outcome: waived
    reason: "Inside the limit: check-artifact-size reports no oversized file, and the hook is the existing mechanism that stops growth past it; nothing in the file is wrong."
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — ask-consolidation's Behavior lines for create-scenario and append-task still say 'Atomic via tempfile-in-parent + persist rename'. create-scenario now persists with no-clobber (write_new_atomic) and append-task writes through the compare-before-persist guard; the Edge Cases bullets for both state the exact mechanism. — `specs/022-deterministic-runtime/scenarios/ask-consolidation.md:24`
    outcome: waived
    reason: The line is true at the level it states (a tempfile in the parent, persisted by rename), and the two Edge Cases bullets the commits for tasks 33 and 35 rewrote carry the mechanism, so no reader acts on a wrong claim; a second statement of the mechanism would be a copy that can rot.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — constitution-excerpts-as-skill-resources' Resolved Questions measurement names five anchors totalling 20,657 bytes for /ductus:implement's Reference line, while the line in framework/commands/implement.md now names six anchors plus the rule-file directory. — `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md:52`
    outcome: waived
    reason: The figure is a dated measurement (2026-08-30) recorded as the evidence for a decision, with its method (the five anchors and their byte total) stated beside it; a record of what was measured then is not a live claim, and the decision it supports does not turn on the count.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — AGENTS.md is 49,987 bytes against the 50,000-byte default read size (no [artifacts] read-size-bytes is set), 13 bytes of headroom, so the next addition to it must be offset by a cut or the pre-commit hook refuses the commit. All 77 links and every backticked repo path in it resolve, and its anchor references resolve (rules-1 is the second Rules heading). — `AGENTS.md`
    outcome: waived
    reason: AGENTS.md is within its read size and has no dead reference; the hook's refusal of a commit that grows it past the limit is the control, and the limit is the constitution's own bound, not a defect of this window's change.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: "other — the scenario slug ductus-bootstrap is cited as a spec 022 scenario in seven live places, but the scenario file is govern-bootstrap.md (title Ductus bootstrap); no file named ductus-bootstrap ever existed in git history. 022's plan.md line 338 links it correctly with the label ductus-bootstrap and the path govern-bootstrap.md. Citations outside this part's table: runtime/src/main.rs:866, runtime/src/primitives/fetch_archive.rs:5, scripts/lint-procedure-parseability.sh:78, specs/022-deterministic-runtime/plan.md:338, scenarios/apply-manifest.md:9, scenarios/runtime-primitive-structural-bugs.md:13, runtime/tests/fixtures/ductus-basic/framework/bootstrap/install.md:28. — `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md:1`"
    outcome: waived
    reason: "Not mechanical: the repair is a choice between renaming the scenario file to ductus-bootstrap.md (one rename and one plan.md link; the digest key in analysis.md regenerates) and re-pointing seven prose citations in four directories, including a runtime/ comment batch. The scenario is reachable by its link in plan.md and its title, so no reader is sent to a missing file by a link; it holds no requirement."
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: other — two Resolved Questions in this part say a real adopter bootstrap verifies or is needed for a change (an-unsubstituted-placeholder-is-not-a-clean-run line 32, every-mcp-argument-has-a-cli-form line 31), while AGENTS.md Workflow now says no real adopter run exists and the scratch-adopter bootstrap stands in (owner, 2026-11-16); 6464d1a9, in the window and naming 022, made that edit in 022's tasks.md and left the two scenarios. — `specs/022-deterministic-runtime/scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md:32`
    outcome: waived
    reason: Each is a decision record in the owner's 2026-11-12 words, and a sweep keeps the record of the decision it sweeps (AGENTS.md Gotchas); the decision itself, to verify through a bootstrap, still holds, and the 2026-11-16 stand-in is recorded in AGENTS.md Workflow and 022's tasks.md.
    decided-at: 2026-11-17T16:58:33Z
    decided-by: rule
  - key: convention — the Exec arguments paragraph sits between the inbound messages' JSON-shape sentence and the gate-response paragraph that continues the same topic — `specs/022-deterministic-runtime/data-model.md:124`
    outcome: waived
    reason: the move would change no claim, rule or behavior, which the Observations rules in framework/reference/review.md say is no observation
    decided-at: 2026-11-17T19:31:01Z
    decided-by: rule
  - key: other — no list under scripts/mutation/ records task 51's checker mutants — `scripts/check-npm-audit.py`
    outcome: waived
    reason: scripts/mutation/run.py applies Rust anchors and runs cargo test; the checker's mutants run through the test's CHECKER override, as e254a3f8 and this round's commit record
    decided-at: 2026-11-17T19:31:01Z
    decided-by: rule
  - key: other — --package-lock-only judges the lockfile alone and passes against a package.json out of step with it, and runtime-currency.yml runs no npm ci — `.github/workflows/runtime-currency.yml`
    outcome: waived
    reason: framework-checks.yml runs npm ci, which fails on a lockfile out of step with package.json, on every push and pull request, so such drift cannot reach main without failing that run
    decided-at: 2026-11-17T19:31:01Z
    decided-by: rule
  - key: convention — scripts/npm-audit-allowed.json's listed field is read by nothing — `scripts/npm-audit-allowed.json`
    outcome: waived
    reason: a note for the list's reader of when and under what decision the entry was listed, as a decision's decided-at is; the checker reads the four fields its contract names
    decided-at: 2026-11-17T19:31:01Z
    decided-by: rule
  - key: convention — parse_args is hand-rolled where argparse would do — `scripts/check-npm-audit.py:95`
    outcome: waived
    reason: the four-flag parser keeps the step's exit-2 nothing-was-judged message for an unknown argument, which argparse's own usage error would replace
    decided-at: 2026-11-17T19:31:01Z
    decided-by: rule
---

# Review — 022-deterministic-runtime

## Summary

The window since 3e35c534 holds task 52 (7d64a274), which extends the loopback proxy test to a [::1] origin and adds M7. One reviewer ran the five passes. It read runtime/src/primitives/fetch_archive.rs and scripts/mutation/fetch-archive-loopback-http.py whole, and the twelve rule files' Statements through discover-rule-files' result, as a projection of ID, tier and Statement. No Verification was needed. The two other scope entries, analysis.md and tasks.md, are artifacts read by their hunks and are named in unread. The reviewer ran the test and M7 by hand in a scratch copy on macOS, where M7 kills only through the [::1] origin. runtime.yml runs the test on Linux and Windows. No finding. Two observations were fixed in round 1: the [::1] bind names IPv6 when it cannot bind, and task 52's text names where the proxy-bypass claim lives. The four stored decisions on fetch_archive.rs were checked and still stand.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

## Observations

- security: a redirect hop is looked up twice rather than connected to the address its screen resolved — `runtime/src/primitives/fetch_archive.rs` — **waived**: 022's scenario allows an equivalent connect-time re-check, both lookups are screened, and the owner prescribed this resolver and its staged-rebind test
- other — the [::1] origin's bind in a_named_loopback_host_is_fetched_without_the_proxy is a bare unwrap, so a runner with no IPv6 loopback panics without saying why — `runtime/src/primitives/fetch_archive.rs:2189` — **fixed** (decided by the rule)
- convention — task 52's text credits specs/system.md and the plan with the loopback proxy-bypass claim, which docs/runtime.md and the changelog carry — `specs/022-deterministic-runtime/tasks.md:196` — **fixed** (decided by the rule)
- reliability: dropping a timed-out client joins its runtime thread, so a slow connect-time lookup can hold a fetch past FETCH_TIMEOUT — `runtime/src/primitives/fetch_archive.rs` — **waived**: the GaiResolver it replaced behaves the same way, and in production the lookup is getaddrinfo, which the operating system resolver's own timeouts bound
- reliability: the initial and redirect-screen lookups run outside the client's timeout — `runtime/src/primitives/fetch_archive.rs` — **waived**: unchanged by this fix and bounded the same way, by the operating system resolver's timeouts; FETCH_DEADLINE bounds the download itself
- other: two proxy tests each define the same four-line internal-address lookup — `runtime/src/primitives/fetch_archive.rs` — **waived**: test fixtures local to each test; sharing one would couple the tests for no gain

## Skipped passes

*None.*

## Excluded from scope

- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 060-exec-analyze-assesses-each-loaded-rule

## Unexamined governance

*None.*
