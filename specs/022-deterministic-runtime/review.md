---
spec: 022-deterministic-runtime
last-run: 2026-10-21T17:25:03Z
reviewed-against: 9e7b7943280981b3479dc205d6271cba83a119d6
diff-base: 37c8219c8730fd23b5fbac7629c11fd2d6bc5b3b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 114
scope: 242
skipped-passes: []
reviewed-digest:
  data-model.md: ec5962dd1273f2c4171a779ee56a24b0b25267d7beb637dc2ad8ee17d359bb4f
  scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md: d092f9d4f59c67c575cc59039a6e0fdf162546969de2820e747b609d7a3d43c7
  scenarios/a-path-argument-resolves-inside-its-base.md: 410e54c9c33a44c87a3c60033e03ae05b473ea90fa154c9d8ab1c5891a409c9f
  scenarios/append-inbox-comment-aware-write.md: c26a16c3be1a7e1c52eca2254061d545ec62b3b7d7c865affd1fa679d6c0ab6b
  scenarios/append-primitive-marker-normalization.md: 26c5064dcdf3f697885faac80332207d079bfd7db2d14c0769254da21e5e4246
  scenarios/append-task-honours-slug-with-explicit-body.md: d831ac5fada945c565042a7216a7d7dd0fa7477efb635597901a5720926cf726
  scenarios/apply-manifest-substitution-contract.md: dfb59f607b4ac27dec233dca8c71742dbc2f139c98794829fad2cdfb8598d733
  scenarios/apply-manifest.md: 738f8063cb42638b837510e00acac5a428ce5468884088b322759ed4bca08b0d
  scenarios/archive-network-hardening.md: 127616981fb074c9e21ef54b4f58d74413f1fdc31af349529601b8dbc096a512
  scenarios/ask-consolidation.md: 164935534cd47ba533aa7faf01ea0bc64c8906a0fc9a95d88728b36ef3bb94c7
  scenarios/check-stuck-tasks-md-advancement.md: 79c2ce08ab177bfb3120fe753082b51bd21e4ee366b9aa1d8dc31e9dc110f28a
  scenarios/clarify-command-acceleration.md: 0aab9170591c9030fe595b682073b2ad22d9db7ca34280e2f044eb50fdc99b1e
  scenarios/cli-config-dir-per-contributor.md: 89380792bcc3750de4070627d44ede6385f29b0e50f2a4e94b54b8cb18b6f51f
  scenarios/commands-dir-parameterization.md: 46f0a96502a38df49d6089ff1b94bfc4b858dfb0f5bd71129d54736966548f5e
  scenarios/config-readers-name-unknown-keys.md: 72bd21a792e7f31e29d73303f1e3947df41ae4c5496368eae31d8b9f1fa869c6
  scenarios/config-resolution-single-probe.md: fc602eb6f2d64f6d1956da411b809af3a53a0c91b26dd198f6b1af61e3347ab4
  scenarios/config-sourced-paths-and-the-traversal-boundary.md: 3ed48e5af4e6a0ca019f9f7ae961771a01e94e595c02ae7c614258b42d777873
  scenarios/constitution-excerpts-as-skill-resources.md: c093f8288e03f8df28666cfd93ce057abba77a7badbcaac16413aacb8da7f746
  scenarios/coverage-expansion-primitives.md: 4626b19eef4cff4f9ce01047b2afa6746573ec289da039cc969b8d8d37ebcd3f
  scenarios/coverage-residue-cleanup.md: 04dc18f12694ed8b02821b50498c7e8ff8123993de72a27ae0d7c8722e024637
  scenarios/criterion-label-assignment.md: 4a1a3acc5d5ddcd44a07582988e06a1b97b2339c71482ccd24765fefb33207a0
  scenarios/dashboard-primitive.md: eed2bcc78c3ec0be96c09cfedd63a27f227f4f9d92bb18af1abab4755a07b7b3
  scenarios/derive-boundary-uncommitted-spec-dir.md: ca28aaef13416801e00bc40d9d58da09c7a513b237bb659597e1efc91a0aaef5
  scenarios/done-when-authoring-forms.md: 080f8717135b67c8400e072dae02f172e40cb0b6ad596c3e227d6cc5f90d0191
  scenarios/exec-analyze-derives-its-list-seeds.md: 9fda65d4cd7a5ec0fab7e07ba9d5d25e794944c967cd09e7b3547f8e1d560726
  scenarios/exec-clarify-asks-each-open-question.md: 4851dc543c6dd80603aa86c776543017b9ab1a7adac469e1b49ec311a9877cab
  scenarios/extension-request-hygiene.md: 1b0c77335af3a2caf82e01896207d60261cd0071a8bc8da3162b9785f7c16e48
  scenarios/fetch-archive-bounds-the-whole-download.md: cb908fbcee4bdcb1f8701c2a678fd472716b4e8e2e61172200324af7abb05379
  scenarios/fetch-archive-dns-rebinding.md: 95e7165ec8ffed49e017f8745e9e909111114ce377aadc5f7073723f2c2ba2d2
  scenarios/fetch-archive-names-its-timeout.md: d45c28473624ee40fb8f6bdf93961bf28947f8d0e05413c7e713f2b307d911a2
  scenarios/framework-list-dedup.md: 9aef03cab7d2494a662058d1091a65e38e26637b10ba338d94eedbd15f9f7f4b
  scenarios/govern-bootstrap.md: 81bc21e03aa1d2c48f63c6f710f1f3303294ffb580a14fe3964e3a69d72f5da2
  scenarios/groom-command-acceleration.md: 636a9fdaa8522c5ef0a1c81b583fe538869645a6867127de43401ec5c58222a4
  scenarios/host-protocol-conformance.md: 13c3a2d89eedf4f79e66c9d4e8e97a9b7361e6fd1fa340a76c46730c28e4d542
  scenarios/implement-completion-gate.md: 6646aaa0aef1d0afbd90c8cf62c4ed7bb4ce07197f805408a6ba9b94fdea57cf
  scenarios/lint-markdown-tool-resolution.md: 06ec661f04cfabf19e7950adfb86d3db440ba48cd4ec8095c34897e0a96d1986
  scenarios/mark-task-backtick-headings.md: a405d3e4b08ef15de8037541c05a93f70a5553f4a72407b917aed4e50ba20507
  scenarios/mark-task-untick-symmetry.md: 0b04c0e1f2091244701f75c435884cb0857a2be728eac41f84fbe41c8df92fb0
  scenarios/mcp-arg-unknown-field-strictness.md: 57c17fb05d72b8f47f64140fc0d8ba6df0c0b8f757282e41d1036aa222309a1b
  scenarios/merge-managed-block-multi-subsection-end.md: b8808a23e83bff0c0b9f4bf2428eb1143a4b2e5e3667b068637d96f05b20824d
  scenarios/merge-managed-block-renamed-subsection.md: c03abb5ae0097d3b15c8a74ef472cdf77d0946ffd2a1625c79351cae300e9a84
  scenarios/merge-managed-block-subsection-insertion.md: 8073edff4f2cf4c93f443d37b927892d108b87fb6123779b3e9d8fa047f02d21
  scenarios/merge-managed-block-trailing-append.md: 6f18642416470c96b6f582a0305d62137a9ae2ec6f1ec054eccac421d8e68a4f
  scenarios/numbered-heading-grammar-single-source.md: 359d35403e9b3f853e4a7cbcca71f3fc032ad556406bae6a584272c6bcd97862
  scenarios/open-questions-are-any-list-item.md: e91d7466be00bcd396959c274dfce64c1d94d0e913b761b7e991da0bf611d6d7
  scenarios/opencode-command-resolution.md: cb9777b7f6da409d5c89fc6411cd4910a830d9860d68ce5cd6dca632661771f0
  scenarios/parser-nested-list-continuation.md: 60eb9129494668bb18045e94e9c6b208353a9a1195ef384cf2aeb6d2482ce139
  scenarios/parser-walker-conventions.md: b53b0ffcadc43ec54c18f6b473b84021cddef418bf6481227b87f06a6acf08cb
  scenarios/primitive-robustness-hardening.md: 7cdf3dcf99c06332244540b196c97b13783184f7b9fba989108dd98f84524ae5
  scenarios/project-directory-resolution-chain.md: d43223e3f037960a24f5660a9944aae0fe02df5f1da10257477926a63cfb5a82
  scenarios/resolve-references-cli-exec-wiring.md: 3ddff8ea493af57fdacb9fab644d33ae95e766cfe7b7d5b11a27d7f083ce5c59
  scenarios/review-exec-wiring.md: 9dbf0c621deaa58d40d16d09c289abdd5cb31ab5555f2036088e1d8f04d76515
  scenarios/review-runtime-acceleration.md: 11cc6eb0a77f963a0eb268276057d06e66e624bdbbea562a0e32b9c7d958ff91
  scenarios/review-scope-parse-fidelity.md: 89747b789e8c2f6d1b550cd662ddfe04a4a42727edba6daa39e6b700a6d2eee1
  scenarios/runtime-primitive-structural-bugs.md: 7fb973b4c5ef5c0268df24996c585f9bf70465f4f7e36ad237d129894f5fd366
  scenarios/runtime-review-019-hardening.md: 9b759c9ce4fc57702e28d764179dcae7b7df9d04dc99ac166b3c0726cf3f9449
  scenarios/scaffolding-primitives.md: faa5098bd2434ec823611879ede2d1f9ff56276650c29bfddfe40512d88ca96f
  scenarios/scenario-open-question-signal.md: bd9e127f09f6aa34417ee912f71a05f0205607b28c799944017a196943b7e129
  scenarios/scenario-question-parser-fix.md: bdcaf5f9bb7d0ff795d6e3a6a5045a79afc63b55d8ce4b3331f807c337bf2ada
  scenarios/severity-is-a-closed-set-not-a-string.md: 8b6ca46ef55c4f6b12420786ebae219f5aa4f2a2878c85d042098f5674013d2e
  scenarios/skipscanner-inline-code-exemption.md: ab402c94a437b19b1b9bdd5a58ad3303d6bb67e9822041520522e3a066c43ab5
  scenarios/spec-side-parser-hardening.md: 91a804882033f369ff98f36157a38896d29eb7d90e8ae4c78e86d45ac35aca3d
  scenarios/specify-routes-before-scaffolding.md: afa3ced4b110e3b7d2f2e96e86cfd719487fce6983e948cbfea4150f4670ea33
  scenarios/the-cli-surface-nothing-pins.md: f63de14533aeb6b2c77f33421af289633e0fded63e795f219b82bfa30f071c8c
  scenarios/the-constitutions-registry-validates-its-values.md: ca81f45e1bf0e7e8cb1ac99736115e5df5eb16afe9c8fa7debf10853e668c975
  scenarios/the-pi-command-candidate.md: 38e81f6ca1732254fee5cce65eb829a0026c30bdb846ff0c3a80b7a308ba94bb
  scenarios/unchecked-done-when-clause-tally.md: 2fa92d88a135df29d4097e5fac7ba2a42583303237cb81bcb8a8df3a7e157ade
  scenarios/unreadable-scenario-is-reported.md: 28c763fee22e177c50586f69989911128438c23964e1fa4831f628ae366792ae
  scenarios/waiver-processing-order.md: 0adb750e651d94ac8bcf06dd0dfbba84cd937c09817035e913ec08b77687f8e8
  scenarios/write-boundary-path-normalization.md: aea2703ee38d7a2ab5806444dd12f99370bfee540bafd8935c9f20d43bd86dee
  scenarios/write-session-primitive.md: 006711337bb99bcb386c0f16e4659eb85e7cf7bb14d4211113550c8c27570bc8
  scenarios/writecode-boundary-derivation.md: 6c9f4071e87d008c30a3741740cbe5c294f0faa9e41c4a823c3898f52b7ce077
  scenarios/writecode-payload-bundling.md: 5c343929c3a42ac4406a02c173b6979231127b9583aabae171fee1f747d5084b
  scenarios/writecode-payload-canonicalize-paths.md: 61fc0c5dffd7d22ffd6e413e1c4f98adc1869f3fef7357340021d49b964a4a7c
blocking: false
dispositions:
  fixed: 54
  routed: 2
  discarded: 87
  undispositioned: 0
decisions:
  - key: "reliability: dropping a timed-out client joins its runtime thread, so a slow connect-time lookup can hold a fetch past FETCH_TIMEOUT — `runtime/src/primitives/fetch_archive.rs`"
    outcome: discarded
    reason: the GaiResolver it replaced behaves the same way, and in production the lookup is getaddrinfo, which the operating system resolver's own timeouts bound
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "reliability: the initial and redirect-screen lookups run outside the client's timeout — `runtime/src/primitives/fetch_archive.rs`"
    outcome: discarded
    reason: unchanged by this fix and bounded the same way, by the operating system resolver's timeouts; FETCH_DEADLINE bounds the download itself
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "security: a redirect hop is looked up twice rather than connected to the address its screen resolved — `runtime/src/primitives/fetch_archive.rs`"
    outcome: discarded
    reason: 022's scenario allows an equivalent connect-time re-check, both lookups are screened, and the owner prescribed this resolver and its staged-rebind test
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: two proxy tests each define the same four-line internal-address lookup — `runtime/src/primitives/fetch_archive.rs`"
    outcome: discarded
    reason: test fixtures local to each test; sharing one would couple the tests for no gain
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 025's ticked task 2 lists a malformed warning among the step-5 outcomes that task 10 replaced — `specs/025-rule-opt-out/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: discarded
    reason: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: routed
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: routed
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: discarded
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: discarded
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T04:21:02Z
    decided-by: andy@stone.dev
  - key: "perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs`"
    outcome: discarded
    reason: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
    decided-at: 2026-10-20T13:09:28Z
    decided-by: andy@stone.dev
  - key: "other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs`"
    outcome: discarded
    reason: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
    decided-at: 2026-10-20T13:09:28Z
    decided-by: andy@stone.dev
  - key: "other: the analyze walker's seed derivation leaves paths unbound for a refused feature rather than raising InvalidPath — `runtime/src/interpreter/mod.rs`"
    outcome: discarded
    reason: it opens nothing, and the walk's first step, read-spec on the same feature, raises the InvalidPath before any step reads paths
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the Windows \\\\?\\ comparison has no symlink test, every containment test being cfg(unix) — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: set-status's eight tests, none cfg(unix), run on windows-latest through validate_feature_argument with tempdir bases, so a base and candidate in different canonical forms would refuse every one of them
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "bug: a writeCode delete of a link inside the boundary is judged by where the link points, so deleting a runtime/ link into framework/ is refused — `runtime/src/schema/extensions.rs`"
    outcome: discarded
    reason: "it errs safe: the boundary refuses an edit it could admit and never admits one it should refuse, and this repository tracks no symlinks"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: retire-feature runs the lexical screen twice — `runtime/src/primitives/retire_feature.rs`"
    outcome: discarded
    reason: "deliberate: the first screen runs ahead of refusal 1, so a traversing name is refused before the sequential-directory check, and the repeat is one scan of a short string"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: 061's plan says the 048 and 015 fetches keep their own terms, while neither plan states any — `specs/061-updates-track-the-latest-release-tag/plan.md`"
    outcome: discarded
    reason: after the owner's answer (a) to 022 task 8 both carry their own terms, the bootstrap's Derived paths bounds and fetch-archive's constants, which is what the sentence says; the plans' silence is the residue that answer accepted
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: 063's review record calls the unbounded bootstrap curls 048's surface while 061's tasks attribute five of them to 061 — `specs/063-oversized-artifacts-warn-with-a-fix/review.md`"
    outcome: discarded
    reason: a generated record, true of the window it described; review.md is regenerated whole and holds nothing durable
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: this repository's .pi/prompts/ductus-configure.md names .pi/prompts/ductus.md, which this repository does not track — `.pi/prompts/ductus-configure.md`"
    outcome: discarded
    reason: the dogfooded .pi copies keep the generator honest; this repository is not a Pi adopter, and an adopter's copy has .pi/prompts/ductus.md beside it, which install.sh writes
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: pi's configure step restates /ductus's latest-release fetch without its grammar check, floor halts or DUCTUS_REPO notice — `framework/bootstrap/configure/pi.md`"
    outcome: discarded
    reason: configure needs only the ref to fetch the bridge at; the halts guard /ductus's own writes, a bad ref fails the bridge fetch with curl's error, and the bounds are named rather than copied
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "defect: extract-archive joins entry names lexically under dest and File::create follows a link already there — `runtime/src/primitives/extract_archive.rs`"
    outcome: discarded
    reason: pre-existing, and /ductus extracts into a directory mktemp has just made, so nothing is under dest before the extraction writes it
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "portability: on Windows Git Bash's mktemp prints /tmp/... with no drive letter, which the staging check refuses as relative — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: the owner's answer (a) to 022 task 9 accepted the Windows temp directory as an unmeasured risk whose failure is a named halt; the scenario and plan now say why that halt is likely
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: a glob with an empty literal prefix, or a path naming the project root, walks the whole tree, honouring no ignore file — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: no shipped caller passes one, and the walk errs toward refusal, naming the link it met
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: validate_in_staging repeats enforce_manifest::resolve_contained_dir's absolute-under-base shape — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: "the two contracts differ: enforce-manifest accepts a relative path and judges an absolute one against the project root as spelled, while a staging path must be absolute and may be spelled canonically, so one helper would change one of them"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: 61ec6512's message counts eleven mutations for twelve new check sites — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: commit history; the review record states what was mutated
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "stale: 015's spec still says the bootstrap's mktemp -d -t lands under $TMPDIR or /tmp — `specs/015-tarball-fetch/spec.md`"
    outcome: discarded
    reason: a narrative sentence in a done spec's procedure, not a criterion; correcting it reopens 015 for an analysis, and the bootstrap adopters run now says what mktemp -t does on each platform
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "scope: the staging check accepts any existing subdirectory of a shared /tmp — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: the owner's answer (a) bounds staging writes to the system temp directory, which this does, and /ductus passes a directory mktemp has just made
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: the macOS integration test leaks its mktemp directory if building the archive or spawning the binary panics — `runtime/tests/exec_subprocess.rs`"
    outcome: discarded
    reason: it leaks only on a panic before its assertions, one empty directory under the per-user temp directory, which macOS clears
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the root-itself refusal compares a root's spelling and canonical form, so an inner root reached through a link into the outer root is accepted as a path — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: it needs a TMPDIR the operator set to name the inner root through such a link, and a path accepted there is still bounded by the outer root
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: a darwin_user_temp_dir that followed TMPDIR would still pass the macOS unit test where it runs — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: the integration test, which runs the binary with TMPDIR pointed outside the per-user directory, fails under that mutation
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: no test pins that staging_roots calls staging_roots_for — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: a one-line delegation read once per process, which no test can re-run with another TMPDIR in-process; the macOS integration test drives the real roots end to end through the binary
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "defect: the runtime's symlink resolver recurses once per component of a missing path, so a very long writeCode edit path can exhaust the stack — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: pre-existing in task 4's writeCode resolution, and this run took the staging check off that resolver; the caller is the agent that spawned the runtime, so a crash ends its own session and crosses no trust boundary (AGENTS.md's backend-rule entry)
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: no test feeds the staging check a candidate whose canonicalization fails and is then accepted — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: the staging test's not-yet-created candidates (run/a.tar.gz under a root) are exactly that case, and a refusal on a failed canonicalization would fail them
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "edge: on macOS a root spelled through more than PATH_MAX bytes of links to . is accepted as a path inside it — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: the OS cannot open such a path either, so nothing is reached through it
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "compat: on Linux an MCP server whose TMPDIR differs from the /ductus shell's refuses the shell's {tempdir} — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: the refusal names the path and the directories it was held to, and on the host measured the two environments agree
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "behavior: a root not created yet and listed after a root that holds it, or a dangling link inside a root that would resolve to it, is accepted as a path inside the outer root — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: each needs a staging root that does not exist or a link planted in one, which /ductus's own arguments never produce, and a path accepted that way is still contained by its root; the comment and plan now say exactly what is refused
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "tests: no test pins that a staging path is matched by text before it is resolved — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: on Unix the order changes only one message, and on Windows its effect is a network contact that does not happen, which a test could observe only by contacting a host
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: write-review checks its feature and compute-review-scope checks it again inside it, and the writeCode payload walks the feature directory once per request — `runtime/src/primitives/write_review.rs`"
    outcome: discarded
    reason: measured under 2 ms per walk on this corpus's largest feature directory (read-spec on 022, 81 entries, 5.63 ms best of 30 against 3.44 ms for --version), so the repeat costs nothing a caller sees
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: write-review loads [paths] three times per call, once itself and once each through compute-review-scope and its analyze-freshness read — `runtime/src/primitives/write_review.rs`"
    outcome: discarded
    reason: "pre-existing composition this window did not add to: each composed reader loaded the config before the change too, and a malformed config prints one line per reader"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: dashboard and the --all prune walks now walk every feature directory — `runtime/src/primitives/dashboard.rs`"
    outcome: discarded
    reason: measured dashboard 0.23-0.25 s, prune-plan --all 0.04 s and prune-tasks --all 0.01 s on this corpus, the walks' share at most about 6 ms; refusing the whole command is the scenario's stated edge case
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "perf: a link inside a feature directory to an ancestor makes every call on that feature walk the whole project and refuse on any outward link anywhere in it — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: only a feature directory holding a link to an ancestor reaches it, no ductus artifact is authored as a link (067's sibling-symlink-trust-boundary), the visited set walks each directory once, and following inside links is what keeps a read through two links from passing unseen
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the error for an outward link prints the absolute path the link resolves to — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: the runtime answers the operator's own agent over stdio, so no caller sits across a trust boundary, and the resolved path is what the operator needs to find the link
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: canonicalize-and-contain logic exists in both interpreter/payload.rs and primitives/mod.rs — `runtime/src/interpreter/payload.rs`"
    outcome: discarded
    reason: "the two differ on purpose: the payload readers skip a missing or outside path and give None for a non-UTF-8 name, where the check refuses with an error and judges a missing path, so one helper would carry both behaviors"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs`"
    outcome: discarded
    reason: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: validate_write_code_resolution takes its resolver as a closure with one caller — `runtime/src/schema/extensions.rs`"
    outcome: discarded
    reason: the closure keeps filesystem access out of the schema module, which otherwise holds shapes and their checks
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: fifteen primitives that can return InvalidPath list none in their docs — `runtime/src/primitives/read_spec.rs`"
    outcome: discarded
    reason: "pre-existing: none listed it for the lexical screen they already ran, and the change gives them a new cause of that error, not a new error"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: verify-published fetches the just-uploaded assets with no retry, unlike verify-installer — `.github/workflows/runtime-acquisition.yml`"
    outcome: discarded
    reason: "all 40 of the most recent runtime-release runs concluded success, so the 404 window has not failed it, and it gates nothing: both halves of the release are out when it runs"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: verify-installer ignores a Retry-After of 2^63 or more and reads a redirect hop's Retry-After when the final hop sends none — `.github/workflows/runtime-release.yml`"
    outcome: discarded
    reason: an out-of-range value falls back to the jittered wait, and GitHub's release-asset redirect sends no Retry-After, so neither changes a run
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-001 does not say whether an empty value on an optional value with a default is absent or malformed — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: predates the widening for environment variables and is unchanged by it; which way an empty optional value falls is a new decision, and CFG-ENV-008 already fails a present malformed value
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: \"at startup\" in CFG-ENV-003, CFG-ENV-005 and CFG-ENV-008 does not cover a configuration file a running process reads again — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: a value first read later is validated when it is read, the reading 063 applies on every call, and no spec here reloads a file mid-process
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-008's error names the setting and the value but not the file, which CFG-ENV-003 and CFG-ENV-005 now name for a file key — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: naming the file is a new requirement on a rule today's answers did not widen; the setting and the value already locate the defect
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005's error names the offending value with no redaction exception like CFG-ENV-008's — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: CFG-ENV-006's MUST redaction at the config-loading layer still applies, so both rules are met together
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-CONST-004 places a value's range bounds alongside its default constant, which a required value has none of — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: predates the widening for required environment variables; where a required value's bounds live is a new decision, and the bounds must still be named constants
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 names CFG-ENV-003 and CFG-ENV-008 as owning configuration, not CFG-ENV-005 — `framework/rules/quality-cross.md`"
    outcome: discarded
    reason: QUAL-GROUND-001 cedes all configuration to the CFG rules, so the omission changes no finding; the two it names are the presence and type rules its scope decision cited
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-011's Source pairs IEC 80000-13 with a NIST page that credits the binary prefixes to IEC 60027-2 — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: BIPM's VIM attributes the binary prefixes to IEC 80000-13 and NIST to its predecessor IEC 60027-2; together the two citations are accurate
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "security: FE-STORAGE-003 does not repeat BE-API-013's WebSocket-handshake exclusion — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: "its MAY requires a landing page that sets Referrer-Policy: no-referrer, which a WebSocket handshake has no page to do, so a handshake ticket already falls under its MUST NOT; the owner's answer placed the narrowing in BE-API-013 and BE-API-015"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015's handshake-URL MUST NOT also bars a CSRF token there, which the OWASP WebSocket Security Cheat Sheet suggests sending in the handshake — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the owner's 2026-10-20 answer keeps every token out of the handshake URL, following the same cheat sheet's advice against tokens in URL query strings; (a)'s Origin allowlist is the handshake's cross-site defense, and a CSRF token can still be checked in the first message
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the pre-signed-URL MAY covers object storage only, so another signed-URL scheme, a CDN's or an application's own HMAC-signed link, falls under BE-API-013 and FE-STORAGE-003 — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the owner's 2026-10-20 answer scoped the MAY to a pre-signed URL for one object and one method; whether other signed-URL schemes fit that scope is a new decision
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-007's Verification asks for legitimate-recovery paths during lockout, which its Statement does not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-001's Statement requires frame protection on all HTTP responses while its Verification table sets frame-ancestors on HTML responses only — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: predates this window; OWASP's HTTP Headers Cheat Sheet, which BE-API-001 cites, says X-Frame-Options gives no security on a redirect or a JSON API response, while its REST Security Cheat Sheet recommends frame-ancestors 'none' on API responses, so aligning the two decides what the rule requires
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: BE-API-013's handshake sentence says BE-API-015 keeps every token out of the handshake URL, while BE-API-015's Statement covers only endpoints serving non-public data or accepting state-changing messages — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence is the narrowing the owner's 2026-10-20 answer placed in both rules and matches BE-API-015's unscoped Verification; a token in a handshake URL authenticates the connection, so its endpoint is not public in practice; scoping it made the two rules disagree and was reverted
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-018's newer-token invalidation would cancel a code issued by one method when a code by another is issued in the same recovery — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: a recovery that verifies the first code before issuing the second complies, since the first is already used; NIST SP 800-63B-4 sets no invalidation across codes and nothing in BE-AUTHN-018 forbids that order
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: 008's ticked task rows 13, 14 and 15 describe FE-STORAGE-003, FE-DEPS-004 and BE-AUTHN-022 as they read before later edits — `specs/008-security-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: CFG-ENV-005, widened to every source, does not say whether a value read from several sources is range-checked as resolved or at each source — `framework/rules/configuration-cross.md`"
    outcome: discarded
    reason: the value validated at startup is the one CFG-ENV-007's resolution yields, the reading CFG-ENV-003 states for presence, and no spec here reads a ranged value from two sources
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml`"
    outcome: discarded
    reason: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "bug: a pinned pre-release would read as current once its stable release ships, since sort -V orders 1.0.0 before 1.0.0-rc.1 — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: No pin in this tree is a pre-release, and the values it is compared with are crates.io's newest stable version and published release tags, so no input reaches the misordering.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: DUCTUS_SHA256 pairs with a DUCTUS_VERSION by file rather than by step, a tab split collapses an empty version field, and cargo install's --version=X, --vers and name@X forms are not parsed — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: Each fails loud, as a hash mismatch or an unpinned report, and the tree has one install step and only the `--version X` form.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: an unknown check name exits 2, the code that also means a check could not run — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: A misspelt argument is a run that examined nothing, which is what exit 2 states, and the message names the cause.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the template's version check is close to redundant, since the sha256 pin already fixes the bytes — `framework/templates/ci/adopter-generators.yml`"
    outcome: discarded
    reason: Scenario the-adopter-ci-template-can-run requires it, and it catches a mispackaged release the hash cannot.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: the pins check's .yaml globs match no tracked file — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: A guard, so a workflow added as .yaml is not skipped silently.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: pass() and fail() are copied across the four scripts/tests/*.sh — `scripts/tests/test-check-runtime-currency.sh`"
    outcome: discarded
    reason: There is no shared test library to reuse, and adding one for two lines is not this change's work.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml`"
    outcome: discarded
    reason: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: \"shipped\" still describes the CI template in 017's AC24, plan.md's Affected Files row and 022's task 3 text — `specs/017-derive-dont-ask/spec.md`"
    outcome: discarded
    reason: AC24 calls it a shipped example, the Affected Files row says adopters copy it, and 022's task 3 text is a spent task; none claims it is installed.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
  - key: "other: cargo's own requests in checks 1, 2 and 4 keep cargo's defaults rather than a timeout the script names — `scripts/check-runtime-currency.sh`"
    outcome: discarded
    reason: Pre-existing checks, unchanged in this window; runtime-currency.yml's timeout-minutes bounds the whole CI run.
    decided-at: 2026-10-21T17:25:03Z
    decided-by: andy@stone.dev
---

# Review — 022-deterministic-runtime

## Summary

Re-review of 022 over its window, after tasks 4, 6 and 7 and the fixes this review made, which added tasks 8 and 9; the owner answered both (a). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking.

**Base.** `37c8219c`. The shared-window chain (022's record of `683e9204`, then 008's, 017's two and 063's of `a680b2c1`) read everything in this scope that changed through `a680b2c1`: each link's scope held every file changed since its base, an ancestor of the link before it, and each read what changed since that link. The one change no link read, 008's two-line plan pointer from its analysis commit, was read here.

**Read** (`examined: 114` of 242 scope entries). 022's `plan.md`, `tasks.md` and both changed scenarios in full; every in-scope file changed since `a680b2c1` except the records; the runtime code against the two changed scenarios, every containment call site and its tests; `runtime.yml`, `runtime-release.yml` and `framework-checks.yml`; all eleven rule files in full, split across three passes at `384249de`; and every file this review changed. The `runtime/src/` and `runtime/tests/` directory entries count as read for the files those passes covered, not for every file under them. **Not read** (128): the generated command mirrors and their `.pi` copies (`gen-claude-commands.sh --check` in sync); `govern.md` (`cmp` identical to `ductus.md`); 022's `spec.md`, `data-model.md`, `analysis.md` and five scenarios, unchanged since `683e9204`; the 061 and 063 `review.md` and `analysis.md`, read only for their decisions; and the other specs' artifacts, docs, command sources and scripts in scope, unchanged since `a680b2c1` and read by the chain.

**Findings: none remain.** Fixed in the run: BE-DEPS-004 (the plan now states provenance per installer, and `framework-checks.yml` runs `npm audit signatures`); BE-TIMEOUT-001 and BE-DATA-001 at the bootstrap's runtime downloads and per-language `.gitignore` fetch (task 8); BE-INPUT-004 at eight primitives no path check covered (task 9), and within that fix lint-markdown's `:`, directory and backslash-escape forms; and QUAL-TEST-001 five times. Six independent re-reads followed the fix rounds (`e2f4e1b2` to `61ec6512`, then `4ed8340d`, `fe4b949b`, `cdb679c9`, `478190a9` and `7767773f`); the last two found no rule finding, and `9e7b7943`, a prose-only correction from the last one's notes, was not re-read. 36 mutations each fail the test naming their behavior.

**Observations: 143.** 54 fixed; 25 discarded with their reasons; 64 carried byte-identically from 063's record (47) and 017's at `742db59b` (17), 62 discarded and 2 routed (033's task 10, 020's `every-loaded-rule-file-has-a-pass`), of which 27 match decisions already stored here. Two stored discards are recorded fixed because their work landed: the pip-installed ziglang's provenance and the bootstrap's unbounded curls. Five stored routes expire: the owner answered the four routed to 008's `independent-review-findings-await-the-owner` and the one routed to 017's `required-config-values-from-any-source`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 022's plan states a provenance exception for crates (BE-DEPS-004) but none for the pip-installed ziglang — `specs/022-deterministic-runtime/plan.md` — **fixed**
- other: the bootstrap's own curls in framework/bootstrap/ductus.md carry no timeout and no --proto or --tlsv1.2, though runtime-acquisition.yml calls itself their executable copy — `framework/bootstrap/ductus.md` — **fixed**
- other: the config-path scenario said any programmatic writer moves a path into the validated tier, while the code exempts a writer that transcribes the operator's answer, which is what makes /link's [services] writes trusted — `specs/022-deterministic-runtime/scenarios/config-sourced-paths-and-the-traversal-boundary.md` — **fixed**
- convention: the hand-edit citation for [constitutions.*] pointed at framework/bootstrap/ductus.md, which only shows a commented example; docs/shared-constitution.md states it — `runtime/src/primitives/resolve_constitutions.rs` — **fixed**
- other: the path scenario called the writeCode boundary the same check as validate_contained, but it matches where the whole path lands and admits a path that leaves and returns — `specs/022-deterministic-runtime/scenarios/a-path-argument-resolves-inside-its-base.md` — **fixed**
- bug: prune-tasks --all --apply wrote earlier features before a later one failed the check, though the scenario says such a walk is refused as a whole — `runtime/src/primitives/prune_tasks.rs` — **fixed**
- other: the 40-link budget was described as the kernel's count across a lookup, but it counts only the links the resolver follows itself — `runtime/src/primitives/mod.rs` — **fixed**
- other: enforce-manifest's absolute-directory branch, which deletes files, had no test of its containment check — `runtime/src/primitives/enforce_manifest.rs` — **fixed**
- other: no test refused prune-plan's or prune-tasks' all walk or the dashboard through an outward link — `runtime/src/primitives/prune_plan.rs` — **fixed**
- bug: the writeCode boundary reported a loop or an unresolvable link as reaching outside the project — `runtime/src/schema/extensions.rs` — **fixed**
- convention: resolve_path's doc listed generator scripts among its absolute-accepting callers, though run-generator checks its script first — `runtime/src/primitives/mod.rs` — **fixed**
- convention: collect_entries' doc said symlinks are followed for is_file(), but neither walk follows one — `runtime/src/primitives/enforce_manifest.rs` — **fixed**
- convention: pi's configure step confined its reading of .pi/prompts/ to the trust check, while its fetches need the bootstrap's Derived paths bounds — `framework/bootstrap/configure/pi.md` — **fixed**
- convention: the generator substituted placeholders inside pi configure's sentence about placeholders — `.pi/prompts/ductus-configure.md` — **fixed**
- other: no installer test covered --ref=latest below the floor, so dropping its branch passed every case — `scripts/tests/test-install.sh` — **fixed**
- other: no installer test checked the --proto =https and --tlsv1.2 the installer passes — `scripts/tests/test-install.sh` — **fixed**
- bug: the latest-release halt had no wording for a request that failed or timed out — `framework/bootstrap/ductus.md` — **fixed**
- other: Source resolution's list of the values it settles left out its two bounds and {tag origin} — `framework/bootstrap/ductus.md` — **fixed**
- other: the bootstrap's 404 halt named the tag but not the setting a recorded tag came from, unlike the floor halts — `framework/bootstrap/ductus.md` — **fixed**
- other: 022's plan contradicted the tree on stdin handling, the parseability allowlist and glob, anyhow, interpreter::State, target's parity file, the Windows target, and the Affected Files table — `specs/022-deterministic-runtime/plan.md` — **fixed**
- other: the plan said every CI build runs --locked, but check-zero.sh and lint-procedure-parseability.sh built without it — `scripts/audit/check-zero.sh` — **fixed**
- other: 022 task 6's state note said full jitter after 404db2ac made it equal jitter — `specs/022-deterministic-runtime/tasks.md` — **fixed**
- defect: on macOS mktemp -d -t ignores TMPDIR, so the staging check refused /ductus's own {tempdir} whenever TMPDIR pointed elsewhere — `runtime/src/primitives/mod.rs` — **fixed**
- claim: task 9, the scenario and the question put to the owner counted nine primitives where they named eight — `specs/022-deterministic-runtime/scenarios/a-path-argument-resolves-inside-its-base.md` — **fixed**
- claim: the resolved question said (a) leaves a supplied path only the temp directory outside the project, though apply-manifest's roots stay absolute-capable — `specs/022-deterministic-runtime/scenarios/a-path-argument-resolves-inside-its-base.md` — **fixed**
- claim: the scenario's context described the eight unchecked primitives in the present tense after task 9 checked them — `specs/022-deterministic-runtime/scenarios/a-path-argument-resolves-inside-its-base.md` — **fixed**
- other: the staging check admitted the temp directory itself, on Linux the shared /tmp — `runtime/src/primitives/mod.rs` — **fixed**
- wording: the npm signature step's comment named the publisher's signature where npm checks the registry's — `.github/workflows/framework-checks.yml` — **fixed**
- diagnostics: check-zero's --locked failure gave no hint that a stale Cargo.lock is a cause — `scripts/audit/check-zero.sh` — **fixed**
- defect: a TMPDIR set but empty made temp_dir() an empty root that matched every staging path and refused it — `runtime/src/primitives/mod.rs` — **fixed**
- defect: the first staging root to match decided, so an unresolvable first root failed a path under the second and a root inside another was accepted as a path — `runtime/src/primitives/mod.rs` — **fixed**
- claim: the glob test's doc said a path with no glob character is not walked while its body asserted that directories are — `runtime/src/primitives/mod.rs` — **fixed**
- claim: task 9 still quoted option (c) as all nine and stated the temp_dir() premise uncorrected — `specs/022-deterministic-runtime/tasks.md` — **fixed**
- claim: the backslash rule's reason said markdownlint-cli2 reads it as /, though it keeps a backslash before a glob character as an escape — `runtime/src/primitives/mod.rs` — **fixed**
- claim: the doc said markdownlint-cli2 reads a lone . as the directory's Markdown files without saying that holds only as the sole argument — `runtime/src/primitives/mod.rs` — **fixed**
- claim: the plan listed the macOS integration test among the interpreter tests that hold each site to the check by name — `specs/022-deterministic-runtime/plan.md` — **fixed**
- stale: lint-markdown's own docs named neither the refused forms nor the directory walk — `runtime/src/primitives/lint_markdown.rs` — **fixed**
- claim: the bootstrap defined {tag origin} for the floor halts alone after the 404 halt began naming it — `framework/bootstrap/ductus.md` — **fixed**
- portability: the scenario's Windows case did not separate MCP, where a Git Bash path arrives unconverted, from the CLI under Git Bash, where MSYS converts it — `specs/022-deterministic-runtime/scenarios/a-path-argument-resolves-inside-its-base.md` — **fixed**
- wording: the npm signature step's comment read as failing on a missing attestation, though only a missing registry signature fails — `.github/workflows/framework-checks.yml` — **fixed**
- defect: with TMPDIR set but empty, Linux had no staging root at all, since temp_dir() returns an empty path where GNU mktemp -t uses /tmp — `runtime/src/primitives/mod.rs` — **fixed**
- claim: the scenario and a test comment said a backslash stays an escape before a glob character, where the set is $()*+?[]^ — `specs/022-deterministic-runtime/scenarios/a-path-argument-resolves-inside-its-base.md` — **fixed**
- claim: the plan and the scenario disagreed on the Windows MCP and CLI cases, and the scenario equated State A with MCP — `specs/022-deterministic-runtime/plan.md` — **fixed**
- claim: the doc said the only argument where markdownlint-cli2 substitutes a lone . as the only path argument — `runtime/src/primitives/mod.rs` — **fixed**
- tests: the staging-roots test took temp_dir() as its input, so an empty TMPDIR would fail it falsely, and nothing pinned the /tmp root — `runtime/src/primitives/mod.rs` — **fixed**
- tests: the macOS tests ran whichever mktemp was first on PATH, so a GNU mktemp would decide them — `runtime/tests/exec_subprocess.rs` — **fixed**
- wording: the plan's sentence on the macOS integration test read as if TMPDIR pointed at the mktemp directory — `specs/022-deterministic-runtime/plan.md` — **fixed**
- defect: /tmp was a staging root on all of Unix, which on macOS accepted the shared /private/tmp and reopened the root-itself refusal through /tmp's alias — `runtime/src/primitives/mod.rs` — **fixed**
- claim: the scenario listed three staging roots and called them either, and task 9's premise note left out Linux with an empty TMPDIR — `specs/022-deterministic-runtime/scenarios/a-path-argument-resolves-inside-its-base.md` — **fixed**
- wording: the roots test's doc left out its /tmp assertion, a test comment sat apart from the case it described, and the darwin test's doc named a function it does not call — `runtime/src/primitives/mod.rs` — **fixed**
- security: the staging check resolved the candidate before matching it to a root, so on Windows a UNC argument outside every root would be opened before it was refused — `runtime/src/primitives/mod.rs` — **fixed**
- claim: the doc said macOS mktemp -t ignores TMPDIR altogether, and that any non-absolute temp_dir stands for /tmp, where GNU mktemp uses /tmp only for an empty TMPDIR and Apple's never — `runtime/src/primitives/mod.rs` — **fixed**
- claim: the plan dropped what an empty or relative TMPDIR contributes to the staging roots — `specs/022-deterministic-runtime/plan.md` — **fixed**
- claim: the staging check's comment and the plan said the root itself is refused however it is spelled, and the plan still said macOS mktemp -t ignores TMPDIR — `specs/022-deterministic-runtime/plan.md` — **fixed**
- other: the analyze walker's seed derivation leaves paths unbound for a refused feature rather than raising InvalidPath — `runtime/src/interpreter/mod.rs` — **discarded**: it opens nothing, and the walk's first step, read-spec on the same feature, raises the InvalidPath before any step reads paths
- other: the Windows \\?\ comparison has no symlink test, every containment test being cfg(unix) — `runtime/src/primitives/mod.rs` — **discarded**: set-status's eight tests, none cfg(unix), run on windows-latest through validate_feature_argument with tempdir bases, so a base and candidate in different canonical forms would refuse every one of them
- bug: a writeCode delete of a link inside the boundary is judged by where the link points, so deleting a runtime/ link into framework/ is refused — `runtime/src/schema/extensions.rs` — **discarded**: it errs safe: the boundary refuses an edit it could admit and never admits one it should refuse, and this repository tracks no symlinks
- other: retire-feature runs the lexical screen twice — `runtime/src/primitives/retire_feature.rs` — **discarded**: deliberate: the first screen runs ahead of refusal 1, so a traversing name is refused before the sequential-directory check, and the repeat is one scan of a short string
- other: 061's plan says the 048 and 015 fetches keep their own terms, while neither plan states any — `specs/061-updates-track-the-latest-release-tag/plan.md` — **discarded**: after the owner's answer (a) to 022 task 8 both carry their own terms, the bootstrap's Derived paths bounds and fetch-archive's constants, which is what the sentence says; the plans' silence is the residue that answer accepted
- other: 063's review record calls the unbounded bootstrap curls 048's surface while 061's tasks attribute five of them to 061 — `specs/063-oversized-artifacts-warn-with-a-fix/review.md` — **discarded**: a generated record, true of the window it described; review.md is regenerated whole and holds nothing durable
- other: this repository's .pi/prompts/ductus-configure.md names .pi/prompts/ductus.md, which this repository does not track — `.pi/prompts/ductus-configure.md` — **discarded**: the dogfooded .pi copies keep the generator honest; this repository is not a Pi adopter, and an adopter's copy has .pi/prompts/ductus.md beside it, which install.sh writes
- other: pi's configure step restates /ductus's latest-release fetch without its grammar check, floor halts or DUCTUS_REPO notice — `framework/bootstrap/configure/pi.md` — **discarded**: configure needs only the ref to fetch the bridge at; the halts guard /ductus's own writes, a bad ref fails the bridge fetch with curl's error, and the bounds are named rather than copied
- defect: extract-archive joins entry names lexically under dest and File::create follows a link already there — `runtime/src/primitives/extract_archive.rs` — **discarded**: pre-existing, and /ductus extracts into a directory mktemp has just made, so nothing is under dest before the extraction writes it
- portability: on Windows Git Bash's mktemp prints /tmp/... with no drive letter, which the staging check refuses as relative — `runtime/src/primitives/mod.rs` — **discarded**: the owner's answer (a) to 022 task 9 accepted the Windows temp directory as an unmeasured risk whose failure is a named halt; the scenario and plan now say why that halt is likely
- perf: a glob with an empty literal prefix, or a path naming the project root, walks the whole tree, honouring no ignore file — `runtime/src/primitives/mod.rs` — **discarded**: no shipped caller passes one, and the walk errs toward refusal, naming the link it met
- other: validate_in_staging repeats enforce_manifest::resolve_contained_dir's absolute-under-base shape — `runtime/src/primitives/mod.rs` — **discarded**: the two contracts differ: enforce-manifest accepts a relative path and judges an absolute one against the project root as spelled, while a staging path must be absolute and may be spelled canonically, so one helper would change one of them
- other: 61ec6512's message counts eleven mutations for twelve new check sites — `runtime/src/primitives/mod.rs` — **discarded**: commit history; the review record states what was mutated
- stale: 015's spec still says the bootstrap's mktemp -d -t lands under $TMPDIR or /tmp — `specs/015-tarball-fetch/spec.md` — **discarded**: a narrative sentence in a done spec's procedure, not a criterion; correcting it reopens 015 for an analysis, and the bootstrap adopters run now says what mktemp -t does on each platform
- scope: the staging check accepts any existing subdirectory of a shared /tmp — `runtime/src/primitives/mod.rs` — **discarded**: the owner's answer (a) bounds staging writes to the system temp directory, which this does, and /ductus passes a directory mktemp has just made
- tests: the macOS integration test leaks its mktemp directory if building the archive or spawning the binary panics — `runtime/tests/exec_subprocess.rs` — **discarded**: it leaks only on a panic before its assertions, one empty directory under the per-user temp directory, which macOS clears
- other: the root-itself refusal compares a root's spelling and canonical form, so an inner root reached through a link into the outer root is accepted as a path — `runtime/src/primitives/mod.rs` — **discarded**: it needs a TMPDIR the operator set to name the inner root through such a link, and a path accepted there is still bounded by the outer root
- tests: a darwin_user_temp_dir that followed TMPDIR would still pass the macOS unit test where it runs — `runtime/src/primitives/mod.rs` — **discarded**: the integration test, which runs the binary with TMPDIR pointed outside the per-user directory, fails under that mutation
- tests: no test pins that staging_roots calls staging_roots_for — `runtime/src/primitives/mod.rs` — **discarded**: a one-line delegation read once per process, which no test can re-run with another TMPDIR in-process; the macOS integration test drives the real roots end to end through the binary
- defect: the runtime's symlink resolver recurses once per component of a missing path, so a very long writeCode edit path can exhaust the stack — `runtime/src/primitives/mod.rs` — **discarded**: pre-existing in task 4's writeCode resolution, and this run took the staging check off that resolver; the caller is the agent that spawned the runtime, so a crash ends its own session and crosses no trust boundary (AGENTS.md's backend-rule entry)
- tests: no test feeds the staging check a candidate whose canonicalization fails and is then accepted — `runtime/src/primitives/mod.rs` — **discarded**: the staging test's not-yet-created candidates (run/a.tar.gz under a root) are exactly that case, and a refusal on a failed canonicalization would fail them
- edge: on macOS a root spelled through more than PATH_MAX bytes of links to . is accepted as a path inside it — `runtime/src/primitives/mod.rs` — **discarded**: the OS cannot open such a path either, so nothing is reached through it
- compat: on Linux an MCP server whose TMPDIR differs from the /ductus shell's refuses the shell's {tempdir} — `runtime/src/primitives/mod.rs` — **discarded**: the refusal names the path and the directories it was held to, and on the host measured the two environments agree
- behavior: a root not created yet and listed after a root that holds it, or a dangling link inside a root that would resolve to it, is accepted as a path inside the outer root — `runtime/src/primitives/mod.rs` — **discarded**: each needs a staging root that does not exist or a link planted in one, which /ductus's own arguments never produce, and a path accepted that way is still contained by its root; the comment and plan now say exactly what is refused
- tests: no test pins that a staging path is matched by text before it is resolved — `runtime/src/primitives/mod.rs` — **discarded**: on Unix the order changes only one message, and on Windows its effect is a network contact that does not happen, which a test could observe only by contacting a host
- other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs` — **discarded**: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
- perf: readers parse the same config text twice, once as a table for the key names and once as their typed struct — `runtime/src/schema/config_keys.rs` — **discarded**: the files are a few hundred bytes, and parsing the typed struct from the text keeps a wrong-typed value's error located by its line, which a conversion from the parsed table loses
- other: a misspelled section name such as [artifact] is never examined, so its settings take their defaults without a word — `runtime/src/schema/config_keys.rs` — **discarded**: the owner's 2026-10-20 decision scopes the check to owned sections; flagging near-miss section names would be a new requirement
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
- other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md` — **routed** to `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md` — **discarded**: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
- other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md` — **discarded**: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
- other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md` — **discarded**: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
- other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md` — **discarded**: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
- other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md` — **discarded**: a resolved question records the reasoning at the time; rewriting it records a fiction
- other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml` — **discarded**: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
- bug: a pinned pre-release would read as current once its stable release ships, since sort -V orders 1.0.0 before 1.0.0-rc.1 — `scripts/check-runtime-currency.sh` — **discarded**: No pin in this tree is a pre-release, and the values it is compared with are crates.io's newest stable version and published release tags, so no input reaches the misordering.
- other: DUCTUS_SHA256 pairs with a DUCTUS_VERSION by file rather than by step, a tab split collapses an empty version field, and cargo install's --version=X, --vers and name@X forms are not parsed — `scripts/check-runtime-currency.sh` — **discarded**: Each fails loud, as a hash mismatch or an unpinned report, and the tree has one install step and only the `--version X` form.
- other: an unknown check name exits 2, the code that also means a check could not run — `scripts/check-runtime-currency.sh` — **discarded**: A misspelt argument is a run that examined nothing, which is what exit 2 states, and the message names the cause.
- other: the template's version check is close to redundant, since the sha256 pin already fixes the bytes — `framework/templates/ci/adopter-generators.yml` — **discarded**: Scenario the-adopter-ci-template-can-run requires it, and it catches a mispackaged release the hash cannot.
- other: the pins check's .yaml globs match no tracked file — `scripts/check-runtime-currency.sh` — **discarded**: A guard, so a workflow added as .yaml is not skipped silently.
- other: pass() and fail() are copied across the four scripts/tests/*.sh — `scripts/tests/test-check-runtime-currency.sh` — **discarded**: There is no shared test library to reuse, and adding one for two lines is not this change's work.
- other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml` — **discarded**: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
- other: "shipped" still describes the CI template in 017's AC24, plan.md's Affected Files row and 022's task 3 text — `specs/017-derive-dont-ask/spec.md` — **discarded**: AC24 calls it a shipped example, the Affected Files row says adopters copy it, and 022's task 3 text is a spent task; none claims it is installed.
- other: cargo's own requests in checks 1, 2 and 4 keep cargo's defaults rather than a timeout the script names — `scripts/check-runtime-currency.sh` — **discarded**: Pre-existing checks, unchanged in this window; runtime-currency.yml's timeout-minutes bounds the whole CI run.
- reliability: dropping a timed-out client joins its runtime thread, so a slow connect-time lookup can hold a fetch past FETCH_TIMEOUT — `runtime/src/primitives/fetch_archive.rs` — **discarded**: the GaiResolver it replaced behaves the same way, and in production the lookup is getaddrinfo, which the operating system resolver's own timeouts bound
- reliability: the initial and redirect-screen lookups run outside the client's timeout — `runtime/src/primitives/fetch_archive.rs` — **discarded**: unchanged by this fix and bounded the same way, by the operating system resolver's timeouts; FETCH_DEADLINE bounds the download itself
- security: a redirect hop is looked up twice rather than connected to the address its screen resolved — `runtime/src/primitives/fetch_archive.rs` — **discarded**: 022's scenario allows an equivalent connect-time re-check, both lookups are screened, and the owner prescribed this resolver and its staged-rebind test
- other: two proxy tests each define the same four-line internal-address lookup — `runtime/src/primitives/fetch_archive.rs` — **discarded**: test fixtures local to each test; sharing one would couple the tests for no gain
- other: 025's ticked task 2 lists a malformed warning among the step-5 outcomes that task 10 replaced — `specs/025-rule-opt-out/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md` — **routed** to `specs/033-rule-surface-setting/tasks.md`
- other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md` — **discarded**: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
- other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md` — **discarded**: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
- other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md` — **discarded**: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
- other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md` — **discarded**: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
- other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md` — **discarded**: both splits are recorded as intended in 068; low
- other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md` — **discarded**: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
- other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md` — **discarded**: the repeated sentences are accurate and not contradictory; each rule stands alone
- other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md` — **discarded**: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
- other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction

## Skipped passes

*None.*

## Unexamined governance

*None.*
