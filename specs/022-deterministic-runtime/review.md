---
spec: 022-deterministic-runtime
last-run: 2026-11-28T04:19:12Z
reviewed-against: 24497bf73008c39e420d8be498c198b20832d1d1
diff-base: f62c003fd8af664c604d8631cfc8fb544be77f99
must-violations: 0
should-violations: 0
low-confidence: 3
examined: 39
scope: 45
unread:
  - runtime/CHANGELOG.md
  - runtime/src/main.rs
  - runtime/src/primitives/resolve_anchor.rs
  - runtime/src/primitives/invalidate_review.rs
  - runtime/tests/exec_subprocess.rs
  - specs/066-audit-records-and-the-completion-gate/data-model.md
excluded: 571
contract-breaks: 1
open-findings:
  - specs/022-deterministic-runtime/spec.md
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: fdd7b84f4faf320cb090b70d267b3595b85a11958406119a9fc49d7a4e7cc73b
  scenarios/a-closed-stdout-is-a-quiet-exit.md: bdbf9b74de48542168e7cac7bc0f5890eed478fde5957c1bd08738b569354957
  scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md: 00cb6c412c88bdb50b23684ae40437eef930fe0954d902ac8892deb5952bcd0e
  scenarios/a-done-when-clause-reads-to-its-end.md: b2d6b2ab8cbd208eec3ac811edc662c296792d51ec7ab1e0c49ec2c8acb045e9
  scenarios/a-path-argument-resolves-inside-its-base.md: 503e9dd4b66563bb317c4deaf4e9cee88275c3c5e3a71a50628704f208a4ac3d
  scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md: 5fec39041378aa1786a94cf59eb1676dd33b7f15b5371815ac92d1e8382c4c72
  scenarios/append-inbox-comment-aware-write.md: 5df79036e979479b03037efd26f74d513c2ef128e579e33e198e4cf8b885366a
  scenarios/append-primitive-marker-normalization.md: 069071366c67e94e3766405b43845d5915b2fdbff7b0bd9a4bcfb4fd78fb2f6c
  scenarios/append-task-honours-slug-with-explicit-body.md: 4e52f43a5f6463d13e9509b34ff017ffb72eec230e8636beb2bc39dace87bc86
  scenarios/apply-manifest-substitution-contract.md: d207e610fe23640c400a4e08205172c699555fde5852c9de1e4405cf1ca72a60
  scenarios/apply-manifest.md: 60f4987e9a73e25827e88e90078295dff5ed59863de94af820f1b510b4a24a61
  scenarios/archive-network-hardening.md: 127616981fb074c9e21ef54b4f58d74413f1fdc31af349529601b8dbc096a512
  scenarios/ask-consolidation.md: b403cbe76bfa7d3f490d92137ac7577413ff763edc7d84925cdd63c717cca577
  scenarios/check-stuck-tasks-md-advancement.md: aaabecc62d10bbdde0c532dadc61cde5fa6b88e3792680389b26eb8b4b95d5b7
  scenarios/clarify-command-acceleration.md: 5f2a753c0a3aeffc1883225f19d7a352053bd8ef6968adb2c381312cec907a31
  scenarios/cli-config-dir-per-contributor.md: adc1a63bfb7c786e70a81a303b64cc19c974a91035fcb288a59ef149dda1e268
  scenarios/commands-dir-parameterization.md: 9e170f99b9d80d6b3657af6f3f14b14dde362cab2b1b05733c78a33939b47422
  scenarios/config-readers-name-unknown-keys.md: 569aec4b2acbb964f737bb9499a4fecc7dfc1807aad5a4db0ad360d4da6e5fff
  scenarios/config-resolution-single-probe.md: a487eec0a7cce706acd911511115140073284ca5b35409ec24cf4f2546b38724
  scenarios/config-sourced-paths-and-the-traversal-boundary.md: 3ed48e5af4e6a0ca019f9f7ae961771a01e94e595c02ae7c614258b42d777873
  scenarios/constitution-excerpts-as-skill-resources.md: 5a6ae02f06a19969028216a89f7855ca020cbdc983fac62ccbedf95fd7400e2f
  scenarios/coverage-expansion-primitives.md: e7e8d84c235adbae463b4abed4042649ac898da6c0be568f6b58b72ba059fd61
  scenarios/coverage-residue-cleanup.md: 0c6315ac8f0f041f624b0a389bbfca855d9886bb06703cef4ee5ffd513770d17
  scenarios/criterion-label-assignment.md: 57e455a91d0b76c6a3a5bdea80288300c246d00d53425730be88896e89c25305
  scenarios/dashboard-primitive.md: df9303f3a1872924761448ba4dc99eed173e01f2f7b8d7a6f1dca4c2b6e85bb5
  scenarios/derive-boundary-uncommitted-spec-dir.md: 64edf042d36ea149576e46e44ddf9bd4bce4e6eea12c59a092a31781baec5c6b
  scenarios/done-when-authoring-forms.md: 8cf2c40315fc62905b424b74ae21f5396a3201af5f4e2427e8ec11a36092a695
  scenarios/every-mcp-argument-has-a-cli-form.md: 942f7dac34955cf0f4714e53045b295e2aa9d47667ec56fb0d174ae2ae1f8191
  scenarios/exec-analyze-derives-its-list-seeds.md: 9fda65d4cd7a5ec0fab7e07ba9d5d25e794944c967cd09e7b3547f8e1d560726
  scenarios/exec-clarify-asks-each-open-question.md: 74ce986d0d56e4b6fc7c59254c7f3c3caca4e0ca0075324cb0ed4b9b62ce56b5
  scenarios/extension-request-hygiene.md: 1b0c77335af3a2caf82e01896207d60261cd0071a8bc8da3162b9785f7c16e48
  scenarios/fetch-archive-bounds-the-whole-download.md: cb908fbcee4bdcb1f8701c2a678fd472716b4e8e2e61172200324af7abb05379
  scenarios/fetch-archive-dns-rebinding.md: e876d9e0d27d6757b024b9f9ff885f8cb0baa56d5886939cae487a591d362af8
  scenarios/fetch-archive-names-its-timeout.md: d45c28473624ee40fb8f6bdf93961bf28947f8d0e05413c7e713f2b307d911a2
  scenarios/framework-list-dedup.md: e01884c811f286768f80cf0a616088c569bb1961ab07db670b47e843f39d91fd
  scenarios/govern-bootstrap.md: 14e0f52d3a8b71adb7d2b85b0bda38a33f849088feb6503d03d4fd9acae85ca4
  scenarios/groom-command-acceleration.md: 77b77c0cbd40f49e0a203834693b3b06b479d50aa438f503a9425f3fe2c04ed0
  scenarios/host-protocol-conformance.md: 6e17ebe8c14b713320a249729de266c2126161ba61ba21416b558628bb13bf0d
  scenarios/implement-completion-gate.md: 426e520df42224929b0988b9dbee323783c4c1ecb181da33b513c10f6ec7c2b0
  scenarios/installed-files-are-not-group-writable.md: ca14be0b2cca81724fdb32c2757e67374c375b542eacf7d86baf175da2c30cbc
  scenarios/link-registration-is-an-extension-point.md: 846e90bb68e0b35b610015e8421788aebd8adadfbc332fc93579adae678c261d
  scenarios/lint-markdown-lints-only-what-it-is-given.md: 92efe81c7326ae4daa08f1f40596afe6221a30433a45ccc7998ab5fadff28ddf
  scenarios/lint-markdown-tool-resolution.md: 79a8b063c83b9de05358a3aa25eab94c9ce930737b73d29638c57f9b6ab8f6af
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
  scenarios/parser-walker-conventions.md: 9585e6bdfc470bd13b9ee644a73a04c51a48fcb5a407f70bf8cdc3d6b4f38414
  scenarios/primitive-robustness-hardening.md: 7cdf3dcf99c06332244540b196c97b13783184f7b9fba989108dd98f84524ae5
  scenarios/project-directory-resolution-chain.md: d43223e3f037960a24f5660a9944aae0fe02df5f1da10257477926a63cfb5a82
  scenarios/resolve-references-cli-exec-wiring.md: 3684cf9f6da7b3cd90643582ddd6470c752eedff28157f0bb7cf74304978a48b
  scenarios/review-exec-wiring.md: 9dbf0c621deaa58d40d16d09c289abdd5cb31ab5555f2036088e1d8f04d76515
  scenarios/review-runtime-acceleration.md: 1239d9960d46b1d5967f6b7c4d75348d7622dfb3f9af54b8f079701686b31984
  scenarios/review-scope-parse-fidelity.md: 14a6808a1ed5df42f0b6eb78e74c77fbe2a92d2cfd9fc1704ad786eb03a47739
  scenarios/routing-ranks-by-claimed-path.md: 2e07048d970bc9b42b73539965204aa5c629ddd9d26421871338ee4f54ecd13d
  scenarios/runtime-primitive-structural-bugs.md: 7fb973b4c5ef5c0268df24996c585f9bf70465f4f7e36ad237d129894f5fd366
  scenarios/runtime-review-019-hardening.md: b05ed1a9dc79172077bff4c692d93e53d3d9b0b9d432521ae94a7d2ddc6a3df5
  scenarios/scaffolding-primitives.md: 83f1b49366c4605cb08f89fd30d162b2920e4b1033368d12da158b5bb4d8a1cb
  scenarios/scenario-open-question-signal.md: da90da6b660351c0622308ebc7361dc4ceb7fc961967737c701b240d0f5b1a9a
  scenarios/scenario-question-parser-fix.md: bdcaf5f9bb7d0ff795d6e3a6a5045a79afc63b55d8ce4b3331f807c337bf2ada
  scenarios/severity-is-a-closed-set-not-a-string.md: d4ea4d23382e2968d72df879d89c7617ffe0b1a8f4049430a25cf70d5f1bd25f
  scenarios/skipscanner-inline-code-exemption.md: 20cf003798a71e271115f0425b78f4d61c52cd2e56e42311d52ccf3660133348
  scenarios/spec-side-parser-hardening.md: 668ff16a2ef33a9833a7d4c6d42713e67e3fb6e976e9fc300fcef9cf3f97c69d
  scenarios/specify-routes-before-scaffolding.md: 9f37f7640d2ae26e3a6b2e95999d40583605bb2dbfd5e8916d1b1ace2c164ad2
  scenarios/the-cli-surface-nothing-pins.md: 634128f508bb6cc3d94913216ce5aefa3571a93bae2e2d45ecb8a0accf22dcc6
  scenarios/the-constitutions-registry-validates-its-values.md: 161cf85126b357e1c7fe6e63be2de5d80e96842a26acb3ed299af55f4bd92acc
  scenarios/the-pi-command-candidate.md: 577baec752506c60c08e56bbff45ff33e8e21f06d1763d73b7fe9e3170de41d0
  scenarios/unchecked-done-when-clause-tally.md: a599283d598b98880175f236aa7f42455e88669e9b01f818d22d45fad7f5767a
  scenarios/unreadable-scenario-is-reported.md: 28c763fee22e177c50586f69989911128438c23964e1fa4831f628ae366792ae
  scenarios/waiver-processing-order.md: 0adb750e651d94ac8bcf06dd0dfbba84cd937c09817035e913ec08b77687f8e8
  scenarios/write-boundary-path-normalization.md: aea2703ee38d7a2ab5806444dd12f99370bfee540bafd8935c9f20d43bd86dee
  scenarios/write-session-primitive.md: 14b4018aef76a67eaccfb3cb9436a2a0420c9543bbe7392cb728ae4c382a8b22
  scenarios/writecode-boundary-derivation.md: 44bbce7e22a35dd58048811c3441f633e2e14f37e48dcdeac089c42276a639cd
  scenarios/writecode-payload-bundling.md: f7d9803245a4ce80944bce99df469356646005d213816a1cf9dda26a64537b41
  scenarios/writecode-payload-canonicalize-paths.md: 635d30397102d2ab90aed011a4499395c13b469d281126fc944b9930fc16522f
criteria-judged: 21
criteria-met: 21
criteria-verdicts:
  - label: AC1
    digest: 1ee3232ad9c69c65b5edc183cf2fcf7eb7fd5eeecb9bd26f8acedd15dd9e455b
  - label: AC2
    digest: dc5e3ed5f8bb422afe5242f38a349758840644f9bb708fd0101ea40ad5187ac3
  - label: AC3
    digest: 858f966a227944e88e0fb9b4f08054451f30b36bf90e8008901d0126307dc767
  - label: AC4
    digest: 76db9598cdb563dff7783041edf377328bb05b85d120ea49ed08e6b603967777
  - label: AC5
    digest: 2a1f7dfbc67a5a57b2586ab5e43c98b9e0930ce091fd82d7fd2a18fbf709ba41
  - label: AC6
    digest: 9e3c2c504d2d1462152b74ff980f0b4c14f7f919579eddb246e24449a79660d6
  - label: AC7
    digest: c965889cae419a54960626110ea33dbb274fae000a48709d0cd199cb8a665c67
  - label: AC8
    digest: 60c17c84076fa16092f1f8091d763df48e23455ea1ad3f92792cce5518283fd4
  - label: AC10
    digest: 9471e857f5c1d5249e1e7c78777a041347a83d27b1f30b65dcca394cab8ceb6f
  - label: AC11
    digest: 35a39b0dbb7be8cf95cbf730dc92164e015e8e30523194f89a68b97e986457cc
  - label: AC13
    digest: 5791f1a5a83f19ec43f7268a29bfadfaa96c4ac73c76e9d91a21795f4dc3f1d5
  - label: AC14
    digest: 9f99a327bbb0e1f519bede2c94089b90134a3d7478c0e2223cd2004168932826
  - label: AC15
    digest: e35e70b9c7e6cfe596471bf18019006d27de7b1c93c70ad1ae748ac833400d36
  - label: AC16
    digest: bebe9b75762e7b61e7054d6af166678bf7534341ca8b248f51a1d62e093a6f1d
  - label: AC17
    digest: b5acb2290f8524c572d8584288d58f7516a41a7e913e0a8437dd5f6aa3c70fdc
  - label: AC19
    digest: 6833ce49340db8e7f894886fd61c068b19cf35c4c555d669b283b316802ad4bd
  - label: AC20
    digest: 6b4def3ee454042d4cc5823ee4f586a8f707eab0ce4f20601a6f10efe1409298
  - label: AC21
    digest: 2b286129139c8160a6236d50593dbdc830c6b4121651830bea211cf95de75188
  - label: AC28
    digest: 52a1993c428c268c396213450071d0e0d80b4a36fd2f16e4950e0e7f72b1b9da
  - label: AC29
    digest: 3d2883cffe1d765b59fe77173a3131a877cb18b537a11b3340d6ff2f0c36df06
  - label: AC31
    digest: 0be735b38d4da9a57fb5e26cc0e413ab9f8205df60f8a956bc614eff1187107d
blocking: false
dispositions:
  fixed: 31
  deferred: 1
  waived: 85
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
  - key: "other: [rules] surfaces is validated only when a rule directory exists, while disabled entries are validated always — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: with no rule directory nothing is selected, so the unread surfaces value changes no result; when it is validated is 033's degenerate-surfaces contract, not this change
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
  - key: security — `is_internal_v6` in `runtime/src/primitives/fetch_archive.rs` folds IPv4-mapped and NAT64 addresses back to the IPv4 they carry but not IPv4-compatible, 6to4 or Teredo forms — `runtime/src/primitives/fetch_archive.rs`
    outcome: waived
    reason: No loaded rule names IPv4-compatible, 6to4 or Teredo forms, and every range BE-INPUT-007 and BE-INPUT-016 enumerate is screened, so widening the screen to them is a new requirement, not a chore.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: security — `load_scope_files` in `runtime/src/interpreter/payload.rs` sends each scope file's content in the `performReview` request with a containment check only; the secret guard runs only for `writeCode`'s plan-relevant files — `runtime/src/interpreter/payload.rs`
    outcome: waived
    reason: "No loaded rule or 022 contract states a secret guard on performReview's scope: the data model and scenario writecode-payload-canonicalize-paths scope the guard to writeCode, so adding one is a new requirement, not a chore."
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: other — `read_envelope` in `runtime/src/io.rs` reads each `ductus exec` stdin line with no length cap — `runtime/src/io.rs`
    outcome: waived
    reason: AGENTS.md names BE-INPUT-006 among the rules that never fire against the runtime, a local stdio process whose only peer is the host that spawned it.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: other — `every_json_flag_help_states_its_arguments_shape` in `runtime/src/main.rs` compares field names only, and `scripts/mutation/json-flag-help-022.py` has no mutant for type labels or three developer-note regex arms — `runtime/src/main.rs`
    outcome: waived
    reason: QUAL-TEST-001's Not flagged clause covers a test whose mutation list records how it was shown to fail, as M1 to M4 do for the named behaviors; mutants for the type labels are hardening no criterion asks for.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: other — `every_mcp_argument_has_a_cli_form` in `runtime/src/main.rs` starts an in-memory MCP server to read schemas that `tool_input_schemas` now returns directly — `runtime/src/main.rs`
    outcome: waived
    reason: The test proves the served tools/list response, which tool_input_schemas does not build (no reject_unknown_fields or flag_always_loaded), so replacing it would change which surface the test proves.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: other — the command-file lookup ladder is built twice, by `exec_candidates` in `runtime/src/main.rs` and `locate_command_file` in `runtime/src/interpreter/payload.rs` — `runtime/src/main.rs`
    outcome: waived
    reason: Both take their installed candidates from Host::command_file_candidates, so the layout set lives in one place; merging the two ladders is a refactor of two callsites, not a chore.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: other — scenario `writecode-payload-canonicalize-paths` Done-when names the retired `gvrn` crate at a `0.7.3` patch bump — `specs/022-deterministic-runtime/scenarios/writecode-payload-canonicalize-paths.md`
    outcome: waived
    reason: The sentence records which release shipped the change, under the crate's name then; rewriting it edits history, and the scenario's Behavior and tests state no version.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: other — scenario `the-pi-command-candidate` names `main::locate_exec_command` as a callsite, though `main::exec_candidates` is the function that calls `Host::command_file_candidates` — `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md`
    outcome: waived
    reason: locate_exec_command is the callsite that walks the list and takes the first path that exists, as the sentence says, building it through exec_candidates; the data model and plan name the same function.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: other — the `plan-basic` fixture's stand-in spec mentions a strict-fields parity check — `runtime/tests/fixtures/plan-basic/specs/005-plan/spec.md`
    outcome: waived
    reason: It is fixture data, the body of a stand-in spec, and describes no live artifact or behavior.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: other — `warn` in `runtime/src/io.rs` repeats the inline `let _ = writeln!` pattern of `runtime/src/main.rs` and `runtime/src/schema/config_keys.rs` — `runtime/src/io.rs`
    outcome: waived
    reason: Those sites lock stderr once across a loop of lines, which a one-line warn would not do.
    decided-at: 2026-11-19T15:52:41Z
    decided-by: rule
  - key: security — `await_llm_response` and `await_gate_response` in `runtime/src/interpreter/mod.rs` write a host-supplied request-id raw into a stderr warning — `runtime/src/interpreter/mod.rs`
    outcome: waived
    reason: the id comes from the operator's own agent host over a local stdio protocol and the line is a diagnostic, not a log store; the same class as the low-confidence finding recorded for `report_broken_links`
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: other — `merge-managed-block` sets 0644 by path after the rename, so a same-user process swapping the path for a symlink in that window would have the chmod follow it — `runtime/src/primitives/merge_managed_block.rs`
    outcome: waived
    reason: not exploitable beyond the operator's own account on a local tool, and `merge-managed-block` replacing an in-project symlink is a recorded owner decision
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: convention — the `CHECK_STEP_REFERENCES` description omits that a command source or reference that is, or runs through, a symbolic link leaving the project is a skip, and that the subject includes two bootstrap files — `runtime/src/schema/descriptions.rs`
    outcome: waived
    reason: no loaded rule requires a description to carry them and the code is correct; its exit clause, which was wrong, is fixed
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: other — the `DERIVE_DEPENDENCIES` and `DERIVE_REFERENCES` descriptions say a zero `updated-count` means examined-and-clean when `unwritable-count` and `refused-count` are also zero, and those two are absent at zero — `runtime/src/schema/descriptions.rs`
    outcome: waived
    reason: the same descriptions say absent at zero earlier, so a reader resolves it, and no check is made to pass that should not
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: design — `load_scope_files` in `runtime/src/interpreter/payload.rs` drops an in-project file it cannot read from `scope-files` without naming it in `unexaminable` — `runtime/src/interpreter/payload.rs`
    outcome: waived
    reason: the data model documents that an unreadable path is skipped; naming it is a design choice with its own request field and test, not a defect of this window
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: convention — `AGENTS.md`'s `eprintln!` gotcha still names `warn_unknown_keys` and not `ductus::io::warn` and the `print_stderr` lint — `AGENTS.md`
    outcome: waived
    reason: "`AGENTS.md` is 49,793 of 50,000 bytes and the owner's decision of 2026-11-19 is that it takes no edit"
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: other — `specs/022-deterministic-runtime/analysis.md` records digests that no longer equal the tree's, so AC18's recorded pass predates the current text — `specs/022-deterministic-runtime/analysis.md`
    outcome: waived
    reason: this run's analysis supersedes the record
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: other — AC17's no-install-nag claim holds, but every command carries a stop-and-restart halt message for a missing server — `framework/commands/status.md`
    outcome: waived
    reason: "a statement of fact: there is no install instruction and no runtime detection, and the criterion says the no-nag half holds"
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: other — AC8 says `ductus exec` completes the six commands in seconds, and no test asserts a time bound — `runtime/tests/parity.rs`
    outcome: waived
    reason: the criterion's second sentence leaves the extension points' own time to the host, and `parity.rs` states that no per-command bound is applied; measured here at 12 ms to 1.1 s
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: convention — the `prune-project` changelog entry is marked BREAKING while its body says the primitive is new in this release — `runtime/CHANGELOG.md`
    outcome: waived
    reason: marked by the owner's ruling that every bounded-list entry is BREAKING
    decided-at: 2026-11-20T01:29:41Z
    decided-by: rule
  - key: "other: the Resolved Questions split at `97412348` (34 modules in one plan, 42 in two or more, 18 in none) states no counting method, and a re-derivation from first-column spans gives 31, 47 and 16, while agreeing on 94 modules, 77 plans and `commit_attribution.rs` claimed by exactly 079 and 081 — `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`"
    outcome: waived
    reason: "Not mechanical: the split is the evidence recorded with the operator's 2026-11-23 decision, the method that produced it is not recorded, so correcting it needs the constraint behind the gap found first (§grounding), and the decision turns on the claimants of `commit_attribution.rs`, which both derivations agree on."
    decided-at: 2026-11-24T01:24:19Z
    decided-by: rule
  - key: "convention: only `/ductus:specify` and `/ductus:groom` call `derive-routing-candidates`, so for `/ductus:review`, `/ductus:analyze` and `/ductus:implement`, which route deferred findings, Step 2's rule that a name is no evidence of ownership is prose they walk, with no path claim derived for them — `framework/reference/groom.md`"
    outcome: waived
    reason: "Not this spec's contract: the scenario states the rule in the decision tree because those commands walk it, and giving them the primitive is new scope that no 022 requirement asks for."
    decided-at: 2026-11-24T01:24:19Z
    decided-by: rule
  - key: "other: `claim_rank` trims `/` from both sides, so a directory named without its slash (`src/api`) matches a plan's `src/api/` entry as an exact claim and ranks ahead of a glob, where `compute-review-scope` would not match the pair — `runtime/src/primitives/derive_routing_candidates.rs`"
    outcome: waived
    reason: "Not mechanical: whether a directory named exactly as a plan's directory entry is an exact or a directory claim is a ranking choice the scenario does not make, and either reading ranks it above every word match."
    decided-at: 2026-11-24T01:24:19Z
    decided-by: rule
  - key: "perf: `claim_rank` compiles an `entry_pattern` regex once per plan entry and named path, and `runtime_home` re-reads plans `derive_path_claims` already read when a runtime signal fires — `runtime/src/primitives/derive_routing_candidates.rs`"
    outcome: waived
    reason: "Not measured, and no performance rule file is loaded for review: the work is plans times the paths one description names, a small product, and nothing shows it costs a run."
    decided-at: 2026-11-24T01:24:19Z
    decided-by: rule
  - key: "convention: `section_label_lines` and `check-rule-ids`' heading-ID scan read fenced lines while `parse_rule_sections` and `other_headings` skip them, so a project's rule file with a fenced example heading would be read two ways — `runtime/src/primitives/rule_sections.rs`"
    outcome: waived
    reason: "Not mechanical: skipping a fenced ID changes which IDs a rule file defines to `check-rule-ids`, which the rule-ID grammar in 008's data model does not decide, and no rule file in this project holds a fenced heading."
    decided-at: 2026-11-24T01:24:19Z
    decided-by: rule
  - key: "perf: `collect_markdown`'s `MAX_WALK_DEPTH` bounds depth, not work, and `is_dir()` follows a symbolic link inside the base, so a link to an ancestor repeats each file in `examined` up to the depth cap — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: "Not mechanical: following or refusing a directory link inside the base is a traversal policy `check-corpus-links`' description does not state, and no spec root in this project holds a directory link."
    decided-at: 2026-11-24T01:24:19Z
    decided-by: rule
  - key: "perf: the `ReviewGateBlock` variant doc comments are 8,210 bytes of the 16,087-byte `check-review-gate` output schema `ductus mcp` advertises, and carry incident narrative and dates (`2026-09-05` crates.io release) that specs 047 and 066 already hold — `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: Observation about the pipeline's own machinery, made while reviewing framework work (AGENTS.md); no loaded rule governs doc-comment length, the schema is fetched lazily per tool, and the claims are accurate. Trimming is a design choice with its own spec, not a defect of the window.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "other: `ProjectFileSize` and `PartSize` repeat the same state/bytes/head-bytes/refused field group with the same doc text, and 19 result types repeat the `form` / `lists-file` / `*-count` triple with near-identical docs — `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: Reuse observation on the pipeline's own machinery with no loaded rule; the bounded-list shapes sit under flattened halves that must share no key (the tests `prune_tasks_unexamined_keys_survive_a_round_trip_in_each_shape` and `check_artifact_size_round_trip_in_the_file_form` pin that), so a shared embedded struct would change the wire shape or the round trips; a refactor is its own change.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "other: `write_new_atomic` repeats `write_atomic_bytes`'s create-parent, tempfile, write, sync_all sequence; only the final `persist` vs `persist_noclobber` differs — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: Pre-existing since task 33, no behavioural divergence found, and a shared helper would put a change inside the atomic-write core both guarantees (AC13, BE-RACE-003) rest on for a saving of about fifteen lines.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "security: `refuse_outward_link` prints the link's name (`shown`) and the resolved path raw into an `InvalidPath` reason, and `InvalidPath`/`InvalidSlug` print the caller's value raw, so a repository file or directory name holding a newline forges a second line in an operator-facing refusal; the sibling errors for committed-config values flatten or escape theirs (`InvalidConstitutionEntry` uses `escape_debug`, `read_json_file_in` uses `flatten_line`) — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: "Same trust boundary D17 waived: the runtime answers the operator's own agent over stdio, and a file name that reaches the message needs a link planted under a feature directory, which no ductus artifact is authored as. Fixing it (flatten `shown` and the echoed values) is cheap if the owner prefers consistency with `BE-INPUT-011`'s use elsewhere in this file."
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "security: `validate_no_traversal`'s documented config-sourced path boundary and `resolve_path` (accepts absolute and `..` paths) leave a path read from committed config (`[constitutions.*]`, `[services]`) with no canonicalization or containment, which `BE-INPUT-004`'s Verification counts as user input ('an operator-set environment variable or configuration value, and text committed to the repository that the code turns into a path') — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: "A deliberate, owner-decided exemption stated in the code (AC29, `validate_no_traversal`'s 'The config-sourced path boundary') and in 022's plan §Path arguments ('a path a project's committed config names is the converse'): a sibling checkout such as `../governance` is the layout the manual prescribes, and the runtime is a local process whose operator wrote the config. The offending reads are in `resolve_constitutions::classify` and `resolve_references::classify`, other parts' files; mod.rs only holds the helper and the statement. No `departure` entry records the waiver of the rule, so the host may want one."
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "perf: `compute` re-reads and re-parses the same inputs per call: the 167,462-byte `framework/bootstrap/ductus.md` is read and table-parsed twice (`check_criterion_path_existence` and `check_body_path_existence` each call `adopter_destinations`), each scenario file is read about five times (`read_spec`'s scan, `check_link_adjacent_drift`'s readability pass and scan, `check_body_path_existence`, `check_line_number_citations`), and `paths_in_history` walks every commit once per call with a `get_path` per candidate that never existed. — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: "measured, not a defect at this scale: `ductus check-artifacts` on a local clone of this repo at this review's head, `chore(022): taiki-e/install-action pinned at v2.90.11` (5,271 commits) takes 0.67 s wall (0.43 s user) for 022-deterministic-runtime, the spec with the most scenarios, and 0.18 s for 084; the history walks already stop once every wanted path is found and dedupe blobs; sharing the reads would add a cache type for a saving under a second"
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "other: a `done` spec whose `review.md` exists but does not parse yields `clean: true` with no finding and no `skipped` entry from `check-artifacts`, because `read_spec` collapses `RecordLoad::Unreadable` into `None` and `check_review_drift` then applies the no-record grandfather exemption. — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: deliberate and documented at `compute` (the comment above `let review_record`) and in `framework/reference/analyze.md` §Review state drift, which makes an unparseable record a hard fail owned by `validate-frontmatter`, which `/{project}:analyze` step 2 runs on every audit record; the same file is not silently clean in the analysis, only in this one primitive's result
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "convention: `resolve_sibling` and `lexical_in_project` in this file are two copies of the same lexical `.`/`..` walk, and `check_corpus_links::lexical_join` and `resolve_anchor::normalize_rel` are two more elsewhere. — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: the copies differ in what they must refuse (this file returns `None` for a rooted path and for a climb out of the project, `lexical_join` drops a root and clamps), so one helper would change one family's trust boundary; none was touched in the window
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "perf: `compute` runs `diff_since` twice per call (base to working tree, then HEAD to working tree), each walking untracked directories, and `Attributor::new` is built in `attribute_window` and again in `held_elsewhere`. — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: waived
    reason: Bounded by project size and dwarfed by the history walk last_review_commit and attribute_window already do; no measurement shows the second diff matters, and held_elsewhere returns before building its Attributor when no uncommitted path is a candidate.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "convention: two resolvers answer \"which file is the spec\" for extension requests: `resolve_spec_path` in `runtime/src/interpreter/payload.rs` (askClarifyQuestion, routeFold, verifyCriteria: .md path as given, else feature under the spec root, else path/spec.md) and `spec_file` in `runtime/src/interpreter/mod.rs` (assessSpecQuality: directory path gets /spec.md, no feature fallback) — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: The walker context carries a `feature` and a `path` that name the same spec in every walk (target and read-spec set both), so the two resolvers return the same file; unifying them would change assessSpecQuality's precedence and needs its own scenario, not a chore.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "perf: each request builder serializes its whole payload twice (`fits` in the builder, then again in `bound_lists`) and `typed_with_legacy_context` clones every file body through `serde_json::to_value`; `assessSpecQuality` repeats this, with a fresh read of the spec, once per loaded rule — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: Cost is bounded by one spec (tens of KB) times the loaded rules (a few hundred), a few tens of MB of in-memory serialization per analyze walk, which is small against the LLM round trips each request waits on; caching the measured size would add state across two functions for no measurable gain.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "convention: the exec walker's `error` envelope codes `unknown-primitive`, `primitive-args-mismatch` and `invalid-edit` (and `io-error` from `main.rs`) appear in no spec, data model, scenario or doc, while the other walker codes (`primitive-failure`, `unknown-extension`, `schema-mismatch`, `out-of-boundary-edit`, `lists-file-unreadable`) are named in the data model — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: The data model gives `code` as an open machine-readable string ('e.g. parse-error') and no criterion asks for a closed list of codes, so listing them is a new requirement, not a chore. `unknown-primitive` is unreachable from a parsed command file, since the parser rejects unrecognized primitive names and `dispatch_handles_every_registry_primitive` pins the dispatch arms.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "convention: `mod.rs` extracts a string list from a context key by hand in `write_boundary`, in the `derive-boundary` union in `merge_primitive_result`, and in `note_restored_lists`, while `payload.rs` has `string_array` and `merge_legacy_context` does the same for `RESTORED_LISTS` — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: Reuse pass, with no loaded rule behind it. Each site is a three-line `get/as_array/filter_map(as_str)` chain and the shared helper is private to `payload.rs`, so sharing it widens a module boundary for no behaviour change.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "perf: `Walker::run` clones the whole step list although `self.procedure` is a shared reference, and every primitive dispatch clones the full walker context twice (`dispatch_primitive` clones it, and `handle_primitive` clones it again after a confirmed gate or for `write-analysis`), including lists restored from list files — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: Efficiency pass, no loaded rule file. The copies are per step of one local walk (about 20 to 30 steps), the context holds only what bounded results restore, and the cost is milliseconds against the host's model turns (AC8 names the extension points as what the wall clock is made of). Removing them changes borrow structure with no behaviour change.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "convention: the test `the_review_walk_judges_every_criterion_and_writes_its_tick` builds the same procedure and the same context as the helper `criteria_walk`, in a local closure, rather than calling it — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: Test-only duplication of about 40 lines with no behaviour or claim at stake, and no loaded rule requires one helper; the first test also reads the stream (`verifyCriteria` request) that `criteria_walk` discards, so a merge needs the helper to return it.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "convention: `summary` cuts `COMPUTE_REVIEW_SCOPE` after `(spec 084, AC7)` and `CHECK_AUDIT_RECORDS` after its long first clause, so `ductus --help` lists an unclosed-parenthesis fragment for the first and a roughly 270-character line for the second — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: Display only; the full text is what the MCP tool and `<primitive> --help` carry. The cut is the module's documented first-sentence-or-colon rule, and the old COMPUTE_REVIEW_SCOPE was cut the same way (after `ended`, inside an open parenthesis). The pipeline's own machinery, per AGENTS.md.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "security: `runtime/rust-toolchain.toml` pins the toolchain by version number (1.100.1) rather than by digest or checksum the build verifies; rustup checks each component against the sha256 its channel manifest records but not a publisher signature — `runtime/rust-toolchain.toml`"
    outcome: waived
    reason: "A recorded, documented exception: 022's plan.md, section 'Dependency pinning, provenance, and upgrades' (BE-DEPS-004, 'cargo-audit and the Rust toolchain: an installer gap, documented') and its 'Pinned inputs' paragraph (SEC-DEPS-001) state the pin, rustup's manifest hash check and its limit, with a remediation. The file's own comment names the documented update process."
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "other: spec.md's extension-point inventory names eight identifiers and says the data model carries every extension point's schema, but `framework/commands/link.md` step 2 marks `<!-- llm:registerService -->`, which neither the spec nor `data-model.md` lists and `runtime/src/schema/extensions.rs` `validate_response` has no arm for; `ductus exec link` halts at once with code `unknown-extension` — `specs/022-deterministic-runtime/spec.md`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    task: "65"
    contract-break: true
    decided-at: 2026-11-28T04:19:12Z
    decided-by: andy@stone.dev
  - key: "convention: framework-checks.yml and generators.yml carry the same cache-key, mtime-reset and build steps and comment text, and the `toolchain: 1.100.1` input is repeated in 11 steps across five workflows (the 0d35d9be bump edited all 11) — `.github/workflows/generators.yml`"
    outcome: waived
    reason: Pipeline machinery; `scripts/check-runtime-currency.sh` toolchain check holds the 11 copies equal to runtime/rust-toolchain.toml, and a composite action would add a new local action to pin and review.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "security: every job's actions/checkout keeps the default persist-credentials: true, so the job's GITHUB_TOKEN stays in .git/config for the later steps (npm ci, cargo build, scripts); in release-assets that token carries contents: write — `.github/workflows/runtime-release.yml`"
    outcome: waived
    reason: "No loaded rule governs it. Every token but release-assets' is contents: read on a public repository, and release-assets' later steps run only the repo's own cp and bash steps and SHA-pinned actions, one of which (softprops/action-gh-release) uses that token by design; setting persist-credentials: false on 15 checkouts would not remove a reachable exposure here."
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "`framework/constitution.md` §runtime-host-integration and spec 022 AC2 call every tool a bare `<verb>-<noun>` name, but `dashboard` (an MCP tool and primitive) is a single noun; the convention is stated as universal. — `specs/022-deterministic-runtime/spec.md`"
    outcome: waived
    reason: The substance (a bare name that each host prefixes) holds for all 77 tools; `dashboard` is shipped and renaming it is a breaking change out of proportion. It is the pipeline's own machinery, so waived per AGENTS.md.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: AC11 says the workflow 'exercises every primitive against fixture inputs', but `.github/workflows/runtime.yml` has no step that does; the claim rests on every primitive module carrying unit tests under `cargo test` and on no test that asserts that every primitive is exercised. — `.github/workflows/runtime.yml`
    outcome: waived
    reason: Every primitive module has tests today (script-checked), so the claim holds; a guard test would be new work on the pipeline's own machinery.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "other: the predicate 'symlink_metadata answers NotFound or NotADirectory' now stands in three places: the new private `is_missing` in mod.rs, `deleted_paths` in `compute_review_scope.rs`, and the `symlink_metadata` match in `rewrite_spec_links.rs` — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: "Reuse, advisory. Sharing `is_missing` (as `pub(crate)`) is not mechanical here: `scripts/mutation/review-scope-cells-and-deletions.py` M4 and M10 quote `deleted_paths`' literal text and would need rewriting, and `rewrite_spec_links` returns an `Io` error for any other kind inside the same match. Three small copies of a two-kind predicate carry no divergence today."
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "tests: the CHANGELOG and `read_recorded_list`'s doc say a directory standing where a record belongs is an error, not an empty list, but `a_dangling_record_is_unreadable_not_absent` pins only the dangling link; the directory case rests on the same `is_missing` line — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: The dangling-link assertions kill a revert of `is_missing` in both functions, and a directory takes the same non-missing branch (`symlink_metadata` succeeds, `read_to_string` errors), so a second test would add no separating input.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "convention: `Walker::new` defaults `command_file` to `framework/commands/{command}.md` for every caller, which is a path the walk never read for any caller that does not call `with_command_file`: the 30 in-module tests, `tests/walker.rs` and `tests/specify_command.rs` (commands such as `test`, which have no such file), and a command `ductus exec` found under the installed config directory or `framework/bootstrap/` — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: The only caller whose envelopes a host reads is `run_exec`, and it always sets the real file. The doc comment on `with_command_file` states the default as a default ('the first place `ductus exec` looks'), not as the file read. Making the argument required would change the `Walker::new` signature at about 40 call sites for no host-visible effect.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "other: spec.md's 'Subprocess interpreter' bullets now carry `request-id` on the four request and response messages, but its `complete` and `error` bullets still omit `runtime-version` (and `error`'s `location`), which `ProtocolMessage` always emits — `specs/022-deterministic-runtime/spec.md`"
    outcome: waived
    reason: Outbound fields are ones a host reads and does not have to send, and a host that ignores them still works, unlike the `request-id` an inbound message must carry. The data model is the protocol's documentation and gives all of them.
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
  - key: "convention: the [Unreleased] Fixed entry 'ductus exec's error locations name the command file, repo-relative' changes the value of `location.file` on dispatch and result errors from a command name to a path and is not marked BREAKING, as the earlier parse-error entry is not — `runtime/CHANGELOG.md`"
    outcome: waived
    reason: "`ErrorLocation.file` is documented 'Repo-relative file path' and spec 022's data-model payload-path rule says the same, so the old value broke the contract and the change repairs it; the earlier parse-error entry set the same unmarked precedent. A consumer matching `location.file == \"review\"` would break, but no documented consumer does. The pipeline's own machinery, per AGENTS.md."
    decided-at: 2026-11-28T04:19:12Z
    decided-by: rule
---

# Review — 022-deterministic-runtime

## Summary

Review of 022's window from `f62c003f` to the head of three unpushed commits, `chore(022): Rust 1.100.0 -> 1.100.1`, `chore(022): the lockfile takes cargo update's 27 changes` and `chore(022): taiki-e/install-action pinned at v2.90.11` (`base-source: last-review`), run in a clean worktree of `origin/main` plus those commits. The scope is 45 entries: the window's 38, among them the 8 the last record left unread, read whole, and 7 files this run's fixes changed. Ten passes parts of 470 to 565 KB each ran all five passes over all five loaded rule files (231,962 bytes, each read whole; five parts were sent back to finish rule files they had read in part or not opened), and two criteria parts judged the criteria (the agent's recommendation, adopted by the operator, 2026-11-27). The nine parts whose slices the first fix batch's 85,000-byte diff touched re-ran their passes over it whole; the second batch, which the re-runs prompted (the `attribute_window` fix, the new tests, the BREAKING changelog entry and the later scenario, data-model and 066 edits), was read by the fix round's independent re-read alone. `examined` is 39 of the 45. Six are unread: `runtime/CHANGELOG.md`, whose released history from `## [0.64.0]` on stays unread as published notes (lines 1 to 488 were read whole); and `runtime/src/main.rs`, `runtime/src/primitives/resolve_anchor.rs`, `runtime/src/primitives/invalidate_review.rs`, `runtime/tests/exec_subprocess.rs` and `specs/066-audit-records-and-the-completion-gate/data-model.md`, which join the scope only through this run's fix hunks, read in the fix diff and re-read in the fix round. Four gating violations were fixed in the run, each on the operator's confirmation: `BE-INPUT-004` at `deleted_paths` in `runtime/src/primitives/compute_review_scope.rs`, where a path under a directory later replaced by a link out of the project was probed through the link, and at `attribute_window` in the same file, found over the fix diff; `CFG-CONST-001` at `default_anchor_kind` in `runtime/src/schema/primitives.rs`; and `QUAL-CLAIM-001` at `read_record` in `runtime/src/primitives/mod.rs`, where a dangling record read as never run. The host raised the first `BE-INPUT-004` and the `CFG-CONST-001` from the low confidence they were handed back at, the one driven on a fixture and the other shown by its two definitions in the tree. Each behaviour fix has a test shown to fail without it. Two findings the re-runs raised on the fixes themselves were fixed as part of those fixes: the link entry's own place in `deleted`, which the docs and the test now state, and the error-location fix's untested wiring through `run_exec` and `take_result`. Two broken contracts were fixed by the rule: `ductus exec`'s walker error locations named the command, not its file, and the interpreter tests' `feature_argument_cases` lacked `prune-review`. A third was deferred to 022's own task 65 and the scenario `link-registration-is-an-extension-point` by the operator's decision (2026-11-27), which reopened 022 (`done` to `in-progress`): `framework/commands/link.md` marks `registerService`, an extension point with no schema, so `ductus exec link` halts with `unknown-extension`. Three low-confidence findings stand. 21 of 22 criteria are judged met. AC18 has no verdict: its analysis record predates the current text, with 53 sections changed and all 304 rules to judge again, and the operator chose to run that analysis after task 65 (the agent's recommendation, adopted by the operator, 2026-11-27). Of the stored decisions, 59 matched an observation this run, 27 of them after the host checked each against its file once `process-decisions` first expired it, and 13 were dropped as on files 022 does not own. One observation on a file outside the scope, `runtime/src/primitives/inbox_standing.rs`, is named here and recorded nowhere. Not traced: most claims in `runtime/src/schema/descriptions.rs` and the `[Unreleased]` changelog about code outside this scope. The fix round's independent re-read first relied on `grep` for three helper bodies and a test-case count; asked which paths it had read, it read them whole before its clean result was taken. A second fix round re-read the edits `scripts/audit/run-all.sh` prompted after the first write: two § references in the new scenario that named no document, and four test literals that spelled a host's command path.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

13 of 244 commits in the window no spec claimed.

21 of 21 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-GROUND-001 — a documented closed set of skip reasons is a free String, bound by no type or test

- **File**: `runtime/src/schema/primitives.rs`
- **Site**: SkippedTarget
- **Rule**: Code whose correctness depends on a contract defined outside that code — a database schema, another service's API shape, a file or wire format, or a list, schema, or format defined elsewhere in the repository — SHOULD bind to that contract so that a wrong assumption fails loudly: a typed or generated binding, a schema or migration reference the build or a test checks, a startup or first-use validation, or a test that exercises the real shape.
- **Finding**: SkippedTarget.reason documents a closed set of eleven values and every emitter writes its own string literal (record_skip(&mut skipped, family, "...", path) in check_artifacts.rs, skip() in check_artifact_size.rs); nothing, neither a type nor a test, ties the emitted set to the documented one, so a typo or a twelfth reason compiles and ships. Siblings in the same file type their closed sets (OwedWorkSkipReason, AuditRecordSkipReason, FixKind); the same shape recurs unbound in StepReferenceFinding.kind (7 kinds), CommandFlagFinding.direction (2), CheckOrphanedReferencesResult.attribution (2) and RoutingCandidate.source.
- **Auto-fixable**: no

### LOW-CONFIDENCE: QUAL-CLAIM-001 — A record that carries no reviewed-digest (RecordFreshness::Undeterminable) yields an empty list of changed contracts, indistinguishable in the result from a record whose digest matched

- **File**: `runtime/src/primitives/compute_review_scope.rs`
- **Site**: contracts_against_digest
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *examined the subject and found nothing* from *could not examine the subject*. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — not a bare zero, empty collection, or success string.
- **Finding**: contracts_against_digest returns the Stale paths and an empty list for every other state, Undeterminable included. For a record naming a run but written before reviewed-digest existed, freshness_of returns Undeterminable (the record carries no reviewed-digest), so no contract joins the scope and compute-review-scope reports nothing that says what the record examined is unknown. The description states only 'each contract the HEAD record's digest says changed', so a digest-less record is silently the clean case.
- **Auto-fixable**: no

### LOW-CONFIDENCE: QUAL-GROUND-001 — descriptions restate the 50,000-byte default read size (44 lines) and the append-task lock bounds as literals, with no check binding them to their constants

- **File**: `runtime/src/schema/descriptions.rs`
- **Site**: (top level)
- **Rule**: Code whose correctness depends on a contract defined outside that code — a database schema, another service's API shape, a file or wire format, or a list, schema, or format defined elsewhere in the repository — SHOULD bind to that contract so that a wrong assumption fails loudly: a typed or generated binding, a schema or migration reference the build or a test checks, a startup or first-use validation, or a test that exercises the real shape.
- **Finding**: The descriptions say `[artifacts] read-size-bytes (50,000 by default)` 44 times, and `APPEND_TASK` says 'ten seconds, or 1,000 attempts'. The values are defined in `DEFAULT_READ_SIZE_BYTES` (`runtime/src/primitives/check_artifact_size.rs`), `SESSION_LOCK_TIMEOUT` and `LOCK_MAX_ATTEMPTS` (`runtime/src/session.rs`). No test or build step ties the prose to them, so a change to a constant leaves the descriptions, which are the MCP tool text and `--help`, stating the old figure.
- **Auto-fixable**: no

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: Driven: `ductus mcp` completed the MCP handshake (serverInfo ductus 0.64.0) and `ductus exec` ran `status`, `target`, `analyze`, `plan` and `specify` to a `complete` envelope on staged fixtures; one crate, `runtime/src/main.rs`.
- AC2 — met: Driven: `tools/list` over `ductus mcp` returned 77 bare-named tools; the CLI's 78 primitive subcommands, `mcp`, `exec`, `parse`, `hook` and `help` aside, differ from them only by `run-generator`, which is not served.
- AC3 — met: `runtime/README.md` §JSON-over-stdio protocol and §Stability, and `specs/022-deterministic-runtime/data-model.md` §JSON-over-stdio protocol, document the seven envelope types `ProtocolMessage` in `runtime/src/schema/protocol.rs` defines; `spec.md`'s summary bullets now carry `request-id` as the code does.
- AC4 — met: Driven: `ductus parse --check` exits 0 on `framework/commands/{status,target,analyze,implement,plan,specify}.md`, and `scripts/lint-procedure-parseability.sh` exits 0; clarify, review and groom were rewritten by scenarios on this spec, as the criterion says they ship.
- AC5 — met: `specs/022-deterministic-runtime/data-model.md` and `runtime/src/schema/extensions.rs` carry the `assessSpecQuality`, `writeCode` and `writeSpecBody` request and response schemas, and each marker stands in its command's Instructions step.
- AC6 — met: `.github/workflows/framework-checks.yml` step (f) runs `scripts/lint-procedure-parseability.sh`, exit 0 over every command file, with `amend`, `audit`, `help` and `ductus-procedure` on the legacy allowlist; `markdown-only-pipeline.yml` is absent, renamed by 048 as the criterion records.
- AC7 — met: Driven: `cargo test --test parity` passed 13 of 13, the six `*_basic_stream_matches_golden` tests against `runtime/tests/golden/` among them; the whole `cargo test --no-fail-fast` on the fixed tree passed 4,100 with none failed.
- AC8 — met: Driven: `ductus exec` with canned host responses on the parity fixtures completed in 0.10 s (status), 0.17 s (target), 0.54 s (analyze), 0.92 s (plan) and 0.53 s (specify), spawn included; implement's golden walk passed in the parity run.
- AC10 — met: `framework/runtime-tools.txt` holds the 77 served tool names, set-equal to `tools/list` and held so by `runtime/tests/mcp.rs` `lists_every_manifest_tool_and_canonical_set`, plus `# cli-only: run-generator`; `scripts/lint-tool-coverage.sh` exits 0.
- AC11 — met: `.github/workflows/runtime.yml` job `build-and-test` runs `cargo build --release --locked` then `cargo test --release --locked` with no `continue-on-error`, so a failing test fails it; every primitive module carries unit tests under it.
- AC13 — met: `mark-task`, `mark-criterion` and `set-status` write through `guarded::write_guarded` to `write_atomic_bytes` in `runtime/src/primitives/mod.rs`: a tempfile in the parent, `sync_all`, then an atomic rename.
- AC14 — met: Driven: `ductus --version` prints `ductus 0.64.0`; `ductus exec` on a command naming `read-spek` emitted a `parse-error` envelope carrying the runtime version, a location and the version-mismatch note, exit 2, and the fixed tree's `cargo test` passed.
- AC15 — met: `.github/workflows/runtime-release.yml` builds exactly `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`, each archive with a `.sha256` sidecar it verifies, and no Windows target; `README.md` has `## The runtime`.
- AC16 — met: `framework/bootstrap/ductus-procedure.md` §Post-Scaffolding Output points at the README's `The runtime` section; the optionality it once announced is gone, as the criterion says 048 made the runtime required.
- AC17 — met: No `framework/commands/*.md` file carries a runtime install, download or detection step; the only text about an absent runtime is each command's stop-and-restart note for a required runtime, which is no install nag.
- AC19 — met: Driven: `node_modules/.bin/markdownlint-cli2` with the project's `.markdownlint-cli2.jsonc` reports 0 issues over `framework/commands/*.md`, `specs/022-deterministic-runtime/**/*.md` and the rest of the 946-file corpus on the fixed tree.
- AC20 — met: Driven: `validate_substitution_key` in `runtime/src/primitives/apply_manifest.rs` refused `{project}` and an empty key before any write, with the destination left empty, while a key holding spaces and punctuation still substituted; `runtime/CHANGELOG.md` 0.43.0 holds the breaking entry.
- AC21 — met: Driven: `apply-manifest` reports per-entry `substitutions-applied`, absent when substitution never ran, with top-level `substitutions-applied` and `entries-substituted`; the bare-key contract stands in the module docs, the schema doc comment and `framework/bootstrap/ductus.md` §Placeholder Substitution and its `govern.md` twin, and `ductus-procedure.md` surfaces both counts.
- AC28 — met: `runtime/src/main.rs` `every_registry_primitive_has_a_clap_subcommand` compares the clap subcommands, less `NON_PRIMITIVE_SUBCOMMANDS` (`mcp`, `exec`, `parse`, `hook`, `help`, each with its reason), with `PRIMITIVE_REGISTRY` by set difference both ways; the unpinned-surface prose is gone.
- AC29 — met: `validate_no_traversal` in `runtime/src/primitives/mod.rs` states the config-sourced path boundary, what an originating writer owes, and that nothing enforces it; `resolve_constitutions::classify` and `resolve_references::classify` cite it and name what each owes.
- AC31 — met: Driven: `render-status` ends its summary with the `Inbox:` line before the target hint; `pipeline_view.rs` `inbox_line` renders the four states, and none gates.

## Observations

- security: `refuse_outward_link` prints the link's name (`shown`) and the resolved path raw into an `InvalidPath` reason, and `InvalidPath`/`InvalidSlug` print the caller's value raw, so a repository file or directory name holding a newline forges a second line in an operator-facing refusal; the sibling errors for committed-config values flatten or escape theirs (`InvalidConstitutionEntry` uses `escape_debug`, `read_json_file_in` uses `flatten_line`) — `runtime/src/primitives/mod.rs` — **waived**: Same trust boundary D17 waived: the runtime answers the operator's own agent over stdio, and a file name that reaches the message needs a link planted under a feature directory, which no ductus artifact is authored as. Fixing it (flatten `shown` and the echoed values) is cheap if the owner prefers consistency with `BE-INPUT-011`'s use elsewhere in this file. (decided by the rule)
- security: `validate_no_traversal`'s documented config-sourced path boundary and `resolve_path` (accepts absolute and `..` paths) leave a path read from committed config (`[constitutions.*]`, `[services]`) with no canonicalization or containment, which `BE-INPUT-004`'s Verification counts as user input ('an operator-set environment variable or configuration value, and text committed to the repository that the code turns into a path') — `runtime/src/primitives/mod.rs` — **waived**: A deliberate, owner-decided exemption stated in the code (AC29, `validate_no_traversal`'s 'The config-sourced path boundary') and in 022's plan §Path arguments ('a path a project's committed config names is the converse'): a sibling checkout such as `../governance` is the layout the manual prescribes, and the runtime is a local process whose operator wrote the config. The offending reads are in `resolve_constitutions::classify` and `resolve_references::classify`, other parts' files; mod.rs only holds the helper and the statement. No `departure` entry records the waiver of the rule, so the host may want one. (decided by the rule)
- security: `load_scope_files` in `runtime/src/interpreter/payload.rs` sends each scope file's content in the `performReview` request after a containment check only; `secret_pattern` and the gitignore guard run only for `writeCode`'s plan-relevant files (`load_plan_relevant_files`) — `runtime/src/interpreter/payload.rs` — **waived**: No loaded rule or 022 contract states a secret guard on performReview's scope: the data model and scenario writecode-payload-canonicalize-paths scope the guard to writeCode, so adding one is a new requirement, not a chore. (decided by the rule)
- security: `await_llm_response` and `await_gate_response` write a host-supplied `request-id` raw into a stderr warning, so a host-supplied id holding a newline or ANSI escape reaches the operator's terminal unescaped; `read_envelope`'s own warning prints the whole line with `{:?}`, which escapes it — `runtime/src/interpreter/mod.rs` — **waived**: the id comes from the operator's own agent host over a local stdio protocol and the line is a diagnostic, not a log store; the same class as the low-confidence finding recorded for `report_broken_links` (decided by the rule)
- security: `runtime/rust-toolchain.toml` pins the toolchain by version number (1.100.1) rather than by digest or checksum the build verifies; rustup checks each component against the sha256 its channel manifest records but not a publisher signature — `runtime/rust-toolchain.toml` — **waived**: A recorded, documented exception: 022's plan.md, section 'Dependency pinning, provenance, and upgrades' (BE-DEPS-004, 'cargo-audit and the Rust toolchain: an installer gap, documented') and its 'Pinned inputs' paragraph (SEC-DEPS-001) state the pin, rustup's manifest hash check and its limit, with a remediation. The file's own comment names the documented update process. (decided by the rule)
- security: every job's actions/checkout keeps the default persist-credentials: true, so the job's GITHUB_TOKEN stays in .git/config for the later steps (npm ci, cargo build, scripts); in release-assets that token carries contents: write — `.github/workflows/runtime-release.yml` — **waived**: No loaded rule governs it. Every token but release-assets' is contents: read on a public repository, and release-assets' later steps run only the repo's own cp and bash steps and SHA-pinned actions, one of which (softprops/action-gh-release) uses that token by design; setting persist-credentials: false on 15 checkouts would not remove a reachable exposure here. (decided by the rule)
- security — create-feature and create-plan-artifacts read their template through resolve_template, which probes framework/templates/spec/ and the spec root's templates directory with no containment, so a template that is a symbolic link leaving the project (a tracked link in a cloned repository, since the candidate names are fixed: spec.md, plan.md, tasks.md, data-model.md) is read and copied into the new artifact. The feature directory is contained and walked; the template location is not. — `runtime/src/primitives/mod.rs:4294` — **waived**: the read reaches only a file of one of four fixed names and its content lands in a file in the project the operator is already creating; resolve_template is in mod.rs, outside this part, and the plan's Path arguments names path arguments and committed-text paths, which a fixed template name is not (decided by the rule)
- security — the .gitignore layer of the writeCode secret guard answers 'not ignored' when libgit2 errors (is_gitignored unwrap_or(false)) and is skipped when the project sits in no readable work tree, so a check that could not run looks like one that passed; secret_pattern on the file's basename and its canonical target stays the floor — `runtime/src/interpreter/payload.rs:1964` — **waived**: the degradation is stated on is_gitignored and on load_plan_relevant_files (gitignore is an opt-in second layer, the secret-pattern check the floor) and writecode-payload-bundling requires no failure mode for the layer; the guard only refuses, so a skipped layer sends no more than the plan's own listed files inside the project (decided by the rule)
- security — ductus parse FILE reads any path it is given with no containment and prints the parsed procedure (the whole AST, content included) when the file is Procedure-shaped markdown; every primitive path argument goes through validate_contained. The subcommand is CLI-only (excluded from the MCP registry by NON_PRIMITIVE_SUBCOMMANDS) and is driven by scripts/lint-procedure-parseability.sh over framework/commands. — `runtime/src/main.rs:635-681` — **waived**: a local CLI debug and CI surface the MCP server does not expose, supplied by the operator or a script, and an agent that can run it can already read the same file with its shell; no host-to-runtime trust boundary lies between the path's supplier and its reader, and the plan's path-argument section scopes the check to primitives (decided by the rule)
- bug: `Frontmatter::folds_into`'s doc comment (rendered in `read-spec`'s MCP schema) says the pre-`done` gate blocks on a pending fold, but no check reads `folds-into` for the gate: a pending fold never holds `done`. — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- other: `AnalyzeBlock::unexamined_by_reason`'s doc enumerates the closed reason set but omits `constitution-unresolved` and `constitution-registry-unreadable`, which `write-analysis` derives and writes into the record's breakdown. — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- other: `DiscoverRuleFilesResult::notices`'s doc lists the notice kinds but omits the `rule file X is judged by analysis alone` line `leave_to_analysis` emits (after the disabled-rule-file notices, before the not-a-regular-file lines) on every unrestricted review selection. — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- other: `ProcessWaiversResult::expired`'s doc says only the last two conditions expire on a dimension-restricted run, but a departure waiver's anchor whose `## Applicable Rules` entry no longer decides it also expires whatever the run's restrictions. — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- other: the `ReviewGateBlock::AnalyzeStale` doc comment says spec 047's `analyze-record-freshness` scenario records a measurement (54 recorded specs, the naive rule flagged all 54, 1 of 54 with the exclusion), and that scenario holds no such figure; the only holder is the message of commit `054c888c`, with no method stated — `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention: `CreateFeatureArgs.fold_into` is documented as `Required with branch_id and meaningless without it`, and `create_feature::resolve_branch_scope` refuses it without `branch-id` with an InvalidArgument; the doc also names the argument `branch_id` where the flag is `--branch-id` — `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention: two tests assert the absence of keys no serialized type has (`inbox-additions`, `inbox-standing` on `DiffCrossSpecResult`; `deduped` on `AppendInboxResult`), which cannot fail, since re-adding a field breaks the struct literal first — `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- perf: the `ReviewGateBlock` variant doc comments are 8,210 bytes of the 16,087-byte `check-review-gate` output schema `ductus mcp` advertises, and carry incident narrative and dates (`2026-09-05` crates.io release) that specs 047 and 066 already hold — `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs` — **waived**: Observation about the pipeline's own machinery, made while reviewing framework work (AGENTS.md); no loaded rule governs doc-comment length, the schema is fetched lazily per tool, and the claims are accurate. Trimming is a design choice with its own spec, not a defect of the window. (decided by the rule)
- other: `ProjectFileSize` and `PartSize` repeat the same state/bytes/head-bytes/refused field group with the same doc text, and 19 result types repeat the `form` / `lists-file` / `*-count` triple with near-identical docs — `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs` — **waived**: Reuse observation on the pipeline's own machinery with no loaded rule; the bounded-list shapes sit under flattened halves that must share no key (the tests `prune_tasks_unexamined_keys_survive_a_round_trip_in_each_shape` and `check_artifact_size_round_trip_in_the_file_form` pin that), so a shared embedded struct would change the wire shape or the round trips; a refactor is its own change. (decided by the rule)
- other: `strip_inline_comments` (private, used once by `split_blocks_reading`) duplicates the shared `without_inline_comments` in the same file: same loop over `find_outside_code("<!--")`/`("-->")`, same result; a third, code-span-unaware copy sits in `check_artifacts.rs` — `runtime/src/primitives/mod.rs` — **fixed** (decided by the rule)
- other: `write_new_atomic` repeats `write_atomic_bytes`'s create-parent, tempfile, write, sync_all sequence; only the final `persist` vs `persist_noclobber` differs — `runtime/src/primitives/mod.rs` — **waived**: Pre-existing since task 33, no behavioural divergence found, and a shared helper would put a change inside the atomic-write core both guarantees (AC13, BE-RACE-003) rest on for a saving of about fifteen lines. (decided by the rule)
- perf: `compute` re-reads and re-parses the same inputs per call: the 167,462-byte `framework/bootstrap/ductus.md` is read and table-parsed twice (`check_criterion_path_existence` and `check_body_path_existence` each call `adopter_destinations`), each scenario file is read about five times (`read_spec`'s scan, `check_link_adjacent_drift`'s readability pass and scan, `check_body_path_existence`, `check_line_number_citations`), and `paths_in_history` walks every commit once per call with a `get_path` per candidate that never existed. — `runtime/src/primitives/check_artifacts.rs` — **waived**: measured, not a defect at this scale: `ductus check-artifacts` on a local clone of this repo at this review's head, `chore(022): taiki-e/install-action pinned at v2.90.11` (5,271 commits) takes 0.67 s wall (0.43 s user) for 022-deterministic-runtime, the spec with the most scenarios, and 0.18 s for 084; the history walks already stop once every wanted path is found and dedupe blobs; sharing the reads would add a cache type for a saving under a second (decided by the rule)
- other: a `done` spec whose `review.md` exists but does not parse yields `clean: true` with no finding and no `skipped` entry from `check-artifacts`, because `read_spec` collapses `RecordLoad::Unreadable` into `None` and `check_review_drift` then applies the no-record grandfather exemption. — `runtime/src/primitives/check_artifacts.rs` — **waived**: deliberate and documented at `compute` (the comment above `let review_record`) and in `framework/reference/analyze.md` §Review state drift, which makes an unparseable record a hard fail owned by `validate-frontmatter`, which `/{project}:analyze` step 2 runs on every audit record; the same file is not silently clean in the analysis, only in this one primitive's result (decided by the rule)
- convention: `resolve_sibling` and `lexical_in_project` in this file are two copies of the same lexical `.`/`..` walk, and `check_corpus_links::lexical_join` and `resolve_anchor::normalize_rel` are two more elsewhere. — `runtime/src/primitives/check_artifacts.rs` — **waived**: the copies differ in what they must refuse (this file returns `None` for a rooted path and for a climb out of the project, `lexical_join` drops a root and clamps), so one helper would change one family's trust boundary; none was touched in the window (decided by the rule)
- convention: the COMPUTE_REVIEW_SCOPE description says a staged branch-scoped spec's brought paths are dropped 'unless in a spec's directory or gone', but `FoldTest::brought` drops every path under the spec root (`!path.starts_with(specs_prefix)`), so a path like `specs/rules/x.md` or `specs/system.md` that a staging-claimed commit changed is dropped too; the doc comment on `brought` ('outside any spec's directory') states the same narrower thing. — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- perf: `compute` runs `diff_since` twice per call (base to working tree, then HEAD to working tree), each walking untracked directories, and `Attributor::new` is built in `attribute_window` and again in `held_elsewhere`. — `runtime/src/primitives/compute_review_scope.rs` — **waived**: Bounded by project size and dwarfed by the history walk last_review_commit and attribute_window already do; no measurement shows the second diff matters, and held_elsewhere returns before building its Attributor when no uncommitted path is a candidate. (decided by the rule)
- other: `load_scope_files` in `runtime/src/interpreter/payload.rs` drops an in-project file it cannot read from `scope-files` without naming it in `unexaminable`; only paths that leave the project are named — `runtime/src/interpreter/payload.rs` — **waived**: the data model documents that an unreadable path is skipped; naming it is a design choice with its own request field and test, not a defect of this window (decided by the rule)
- other: canonicalize-and-contain logic exists in both `runtime/src/interpreter/payload.rs` (`classify_contained`, `read_repo_file`) and `runtime/src/primitives/mod.rs` (`validate_contained`, `leaves_project`), and payload.rs calls both — `runtime/src/interpreter/payload.rs` — **waived**: the two differ on purpose: the payload readers skip a missing or outside path and give None for a non-UTF-8 name, where the check refuses with an error and judges a missing path, so one helper would carry both behaviors
- convention: the module doc of `runtime/src/interpreter/payload.rs` lists six typed request shapes but the file also builds `PerformReviewRequest` and `RouteFoldRequest`, so the enumeration is stale — `runtime/src/interpreter/payload.rs` — **fixed** (decided by the rule)
- convention: two resolvers answer "which file is the spec" for extension requests: `resolve_spec_path` in `runtime/src/interpreter/payload.rs` (askClarifyQuestion, routeFold, verifyCriteria: .md path as given, else feature under the spec root, else path/spec.md) and `spec_file` in `runtime/src/interpreter/mod.rs` (assessSpecQuality: directory path gets /spec.md, no feature fallback) — `runtime/src/interpreter/payload.rs` — **waived**: The walker context carries a `feature` and a `path` that name the same spec in every walk (target and read-spec set both), so the two resolvers return the same file; unifying them would change assessSpecQuality's precedence and needs its own scenario, not a chore. (decided by the rule)
- perf: each request builder serializes its whole payload twice (`fits` in the builder, then again in `bound_lists`) and `typed_with_legacy_context` clones every file body through `serde_json::to_value`; `assessSpecQuality` repeats this, with a fresh read of the spec, once per loaded rule — `runtime/src/interpreter/payload.rs` — **waived**: Cost is bounded by one spec (tens of KB) times the loaded rules (a few hundred), a few tens of MB of in-memory serialization per analyze walk, which is small against the LLM round trips each request waits on; caching the measured size would add state across two functions for no measurable gain. (decided by the rule)
- bug: the interpreter tests' `feature_argument_cases` table has no row for `prune-review`, a call site that takes a `feature` argument and hands it to `validate_feature_argument`, so no test holds that site to the outward-link check, against a scenario that says the table holds a row for every such site — `runtime/src/interpreter/mod.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- other: the 'Subprocess interpreter' bullets in 022's `spec.md` give the `llm-request`, `llm-response`, `gate-confirm` and `gate-response` shapes without `request-id`, which the walker and `ProtocolMessage` require and the data model documents — `specs/022-deterministic-runtime/spec.md` — **fixed** (decided by the rule)
- convention: the exec walker's `error` envelope codes `unknown-primitive`, `primitive-args-mismatch` and `invalid-edit` (and `io-error` from `main.rs`) appear in no spec, data model, scenario or doc, while the other walker codes (`primitive-failure`, `unknown-extension`, `schema-mismatch`, `out-of-boundary-edit`, `lists-file-unreadable`) are named in the data model — `runtime/src/interpreter/mod.rs` — **waived**: The data model gives `code` as an open machine-readable string ('e.g. parse-error') and no criterion asks for a closed list of codes, so listing them is a new requirement, not a chore. `unknown-primitive` is unreachable from a parsed command file, since the parser rejects unrecognized primitive names and `dispatch_handles_every_registry_primitive` pins the dispatch arms. (decided by the rule)
- convention: `mod.rs` extracts a string list from a context key by hand in `write_boundary`, in the `derive-boundary` union in `merge_primitive_result`, and in `note_restored_lists`, while `payload.rs` has `string_array` and `merge_legacy_context` does the same for `RESTORED_LISTS` — `runtime/src/interpreter/mod.rs` — **waived**: Reuse pass, with no loaded rule behind it. Each site is a three-line `get/as_array/filter_map(as_str)` chain and the shared helper is private to `payload.rs`, so sharing it widens a module boundary for no behaviour change. (decided by the rule)
- perf: `Walker::run` clones the whole step list although `self.procedure` is a shared reference, and every primitive dispatch clones the full walker context twice (`dispatch_primitive` clones it, and `handle_primitive` clones it again after a confirmed gate or for `write-analysis`), including lists restored from list files — `runtime/src/interpreter/mod.rs` — **waived**: Efficiency pass, no loaded rule file. The copies are per step of one local walk (about 20 to 30 steps), the context holds only what bounded results restore, and the cost is milliseconds against the host's model turns (AC8 names the extension points as what the wall clock is made of). Removing them changes borrow structure with no behaviour change. (decided by the rule)
- convention: the test `the_review_walk_judges_every_criterion_and_writes_its_tick` builds the same procedure and the same context as the helper `criteria_walk`, in a local closure, rather than calling it — `runtime/src/interpreter/mod.rs` — **waived**: Test-only duplication of about 40 lines with no behaviour or claim at stake, and no loaded rule requires one helper; the first test also reads the stream (`verifyCriteria` request) that `criteria_walk` discards, so a merge needs the helper to return it. (decided by the rule)
- convention: `summary` cuts `COMPUTE_REVIEW_SCOPE` after `(spec 084, AC7)` and `CHECK_AUDIT_RECORDS` after its long first clause, so `ductus --help` lists an unclosed-parenthesis fragment for the first and a roughly 270-character line for the second — `runtime/src/schema/descriptions.rs` — **waived**: Display only; the full text is what the MCP tool and `<primitive> --help` carry. The cut is the module's documented first-sentence-or-colon rule, and the old COMPUTE_REVIEW_SCOPE was cut the same way (after `ended`, inside an open parenthesis). The pipeline's own machinery, per AGENTS.md. (decided by the rule)
- convention: `ductus exec`'s walker errors set `ErrorLocation.file` to the command's name (`self.procedure.command`, for example `review`), not a repo-relative path, in `restore_lists` failures and in dispatch errors, while `ErrorLocation.file` is documented 'Repo-relative file path' and the [Unreleased] Fixed entry names only the parse-error envelope as repaired — `runtime/src/interpreter/mod.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- other: spec.md's extension-point inventory names eight identifiers and says the data model carries every extension point's schema, but `framework/commands/link.md` step 2 marks `<!-- llm:registerService -->`, which neither the spec nor `data-model.md` lists and `runtime/src/schema/extensions.rs` `validate_response` has no arm for; `ductus exec link` halts at once with code `unknown-extension` — `specs/022-deterministic-runtime/spec.md` (breaks the spec's contract) — **deferred** to `specs/022-deterministic-runtime/tasks.md` task 65
- convention: plan.md §Release/Archive entries cites 'the `zip` crate (9.8.1, `src/types.rs`)', but `runtime/Cargo.lock` locks zip 9.9.0 since `b67ee602`; the cited behavior (UTF-8 name read as UTF-8, otherwise CP437) holds unchanged in 9.9.0 — `specs/022-deterministic-runtime/plan.md` — **fixed** (decided by the rule)
- convention: plan.md §Agent reach says '`RETIRED_ALLOW` thirty-one', but the list in `runtime/src/primitives/merge_permissions.rs` holds 32 entries since spec 084 added `mcp__ductus__classify-spec-edit` (in the window, `ac21c388`) — `specs/022-deterministic-runtime/plan.md` — **fixed** (decided by the rule)
- perf: `.github/workflows/runtime.yml` keys both its `actions/cache` steps on `hashFiles('runtime/Cargo.lock')` alone, so a toolchain-only bump (`0d35d9be`, 1.100.0 to 1.100.1) hits the old key, restores a `target/` built by the older rustc, rebuilds it, and never saves; framework-checks.yml and generators.yml already hash `runtime/rust-toolchain.toml` into theirs — `.github/workflows/runtime.yml` — **fixed** (decided by the rule)
- convention: framework-checks.yml and generators.yml carry the same cache-key, mtime-reset and build steps and comment text, and the `toolchain: 1.100.1` input is repeated in 11 steps across five workflows (the 0d35d9be bump edited all 11) — `.github/workflows/generators.yml` — **waived**: Pipeline machinery; `scripts/check-runtime-currency.sh` toolchain check holds the 11 copies equal to runtime/rust-toolchain.toml, and a composite action would add a new local action to pin and review. (decided by the rule)
- bug: the `writeSpecBody` request example in the data model shows `"existing-content": null`, but the runtime never emits a null: `WriteSpecBodyRequest.existing_content` is skipped when `None`, and the same section's field-sourcing bullet says the key is "omitted when the file or section is absent or empty" — `specs/022-deterministic-runtime/data-model.md` — `specs/022-deterministic-runtime/data-model.md` — **fixed** (decided by the rule)
- bug: ask-consolidation states `append-task`'s result as `{ "appended": { "task-number": "N", "path": ... } }`, but `AppendTaskResult` is flat — `task-number`, `path`, `created` and a boolean `appended` — so the nested object does not exist — `specs/022-deterministic-runtime/scenarios/ask-consolidation.md` — `specs/022-deterministic-runtime/scenarios/ask-consolidation.md` — **fixed** (decided by the rule)
- bug: ask-consolidation says `create-scenario` resolves the scenario template at `framework/templates/spec/scenario.md` and frames `body` with it, but `create_scenario::render` builds the frame inline and reads no template file; the already-exists error also prints the absolute destination, not `{feature-path}/scenarios/{slug}.md` — `specs/022-deterministic-runtime/scenarios/ask-consolidation.md` — `specs/022-deterministic-runtime/scenarios/ask-consolidation.md` — **fixed** (decided by the rule)
- other: append-primitive-marker-normalization describes one marker grammar (`bullet_text`, a dash then a space) for every append primitive, but `append-question` also strips a `*`, `+`, `1.` or `1)` marker (`question_text` falls back to `read_spec::list_item`), which the scenario does not state — `specs/022-deterministic-runtime/scenarios/append-primitive-marker-normalization.md` — `specs/022-deterministic-runtime/scenarios/append-primitive-marker-normalization.md` — **fixed** (decided by the rule)
- other: parser-walker-conventions' last Edge Case says help.md, amend.md, log.md, clarify.md and groom.md remain allowlisted until their rewrites land, but `runtime/legacy-prose-commands.txt` now lists only amend.md and help.md as transitional (log, clarify and groom were rewritten and removed; audit.md and ductus-procedure.md are the host-run and reference-prose entries) — `specs/022-deterministic-runtime/scenarios/parser-walker-conventions.md` — `specs/022-deterministic-runtime/scenarios/parser-walker-conventions.md` — **fixed** (decided by the rule)
- other: resolve-references-cli-exec-wiring's last Edge Case says generating the five parallel name registries from one source is tracked in the inbox, but it shipped as `PRIMITIVE_REGISTRY` (parser `PRIMITIVE_NAMES` aliases it, MCP `tool_names` derives from it) and `specs/inbox.md` holds no such item — `specs/022-deterministic-runtime/scenarios/resolve-references-cli-exec-wiring.md` — `specs/022-deterministic-runtime/scenarios/resolve-references-cli-exec-wiring.md` — **fixed** (decided by the rule)
- other: `specs/022-deterministic-runtime/analysis.md` records digests that no longer equal the tree's (data-model.md, plan.md, spec.md, tasks.md and several scenarios differ from `analyzed-digest`), and its Advisory section names data-model.md at 65023 bytes against 65346 now, so AC18's recorded pass predates the current text — `specs/022-deterministic-runtime/analysis.md` — `specs/022-deterministic-runtime/analysis.md` — **waived**: this run's analysis supersedes the record (decided by the rule)
- other: the Resolved Questions split in routing-ranks-by-claimed-path (34 modules in one plan, 42 in two or more, 18 in none, over 94 modules and 77 plans) states no counting method — `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md` — `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md` — **waived**: Not mechanical: the split is the evidence recorded with the operator's 2026-11-23 decision, the method that produced it is not recorded, so correcting it needs the constraint behind the gap found first (§grounding), and the decision turns on the claimants of `commit_attribution.rs`, which both derivations agree on. (decided by the rule)
- `framework/constitution.md` §runtime-host-integration and spec 022 AC2 call every tool a bare `<verb>-<noun>` name, but `dashboard` (an MCP tool and primitive) is a single noun; the convention is stated as universal. — `specs/022-deterministic-runtime/spec.md` — **waived**: The substance (a bare name that each host prefixes) holds for all 77 tools; `dashboard` is shipped and renaming it is a breaking change out of proportion. It is the pipeline's own machinery, so waived per AGENTS.md. (decided by the rule)
- AC11 says the workflow 'exercises every primitive against fixture inputs', but `.github/workflows/runtime.yml` has no step that does; the claim rests on every primitive module carrying unit tests under `cargo test` and on no test that asserts that every primitive is exercised. — `.github/workflows/runtime.yml` — **waived**: Every primitive module has tests today (script-checked), so the claim holds; a guard test would be new work on the pipeline's own machinery. (decided by the rule)
- other: the analyze walker's seed derivation leaves paths unbound for a refused feature rather than raising InvalidPath — `runtime/src/interpreter/mod.rs` — **waived**: it opens nothing, and the walk's first step, read-spec on the same feature, raises the InvalidPath before any step reads paths
- other: the Windows \\?\ comparison has no symlink test, every containment test being cfg(unix) — `runtime/src/primitives/mod.rs` — **waived**: set-status's eight tests, none cfg(unix), run on windows-latest through validate_feature_argument with tempdir bases, so a base and candidate in different canonical forms would refuse every one of them
- portability: on Windows Git Bash's mktemp prints /tmp/... with no drive letter, which the staging check refuses as relative — `runtime/src/primitives/mod.rs` — **waived**: the owner's answer (a) to 022 task 9 accepted the Windows temp directory as an unmeasured risk whose failure is a named halt; the scenario and plan now say why that halt is likely
- perf: a glob with an empty literal prefix, or a path naming the project root, walks the whole tree, honouring no ignore file — `runtime/src/primitives/mod.rs` — **waived**: no shipped caller passes one, and the walk errs toward refusal, naming the link it met
- other: validate_in_staging repeats enforce_manifest::resolve_contained_dir's absolute-under-base shape — `runtime/src/primitives/mod.rs` — **waived**: the two contracts differ: enforce-manifest accepts a relative path and judges an absolute one against the project root as spelled, while a staging path must be absolute and may be spelled canonically, so one helper would change one of them
- other: 61ec6512's message counts eleven mutations for twelve new check sites — `runtime/src/primitives/mod.rs` — **waived**: commit history; the review record states what was mutated
- scope: the staging check accepts any existing subdirectory of a shared /tmp — `runtime/src/primitives/mod.rs` — **waived**: the owner's answer (a) bounds staging writes to the system temp directory, which this does, and /ductus passes a directory mktemp has just made
- other: the root-itself refusal compares a root's spelling and canonical form, so an inner root reached through a link into the outer root is accepted as a path — `runtime/src/primitives/mod.rs` — **waived**: it needs a TMPDIR the operator set to name the inner root through such a link, and a path accepted there is still bounded by the outer root
- tests: a darwin_user_temp_dir that followed TMPDIR would still pass the macOS unit test where it runs — `runtime/src/primitives/mod.rs` — **waived**: the integration test, which runs the binary with TMPDIR pointed outside the per-user directory, fails under that mutation
- tests: no test pins that staging_roots calls staging_roots_for — `runtime/src/primitives/mod.rs` — **waived**: a one-line delegation read once per process, which no test can re-run with another TMPDIR in-process; the macOS integration test drives the real roots end to end through the binary
- defect: the runtime's symlink resolver recurses once per component of a missing path, so a very long writeCode edit path can exhaust the stack — `runtime/src/primitives/mod.rs` — **waived**: pre-existing in task 4's writeCode resolution, and this run took the staging check off that resolver; the caller is the agent that spawned the runtime, so a crash ends its own session and crosses no trust boundary (AGENTS.md's backend-rule entry)
- edge: on macOS a root spelled through more than PATH_MAX bytes of links to . is accepted as a path inside it — `runtime/src/primitives/mod.rs` — **waived**: the OS cannot open such a path either, so nothing is reached through it
- compat: on Linux an MCP server whose TMPDIR differs from the /ductus shell's refuses the shell's {tempdir} — `runtime/src/primitives/mod.rs` — **waived**: the refusal names the path and the directories it was held to, and on the host measured the two environments agree
- behavior: a root not created yet and listed after a root that holds it, or a dangling link inside a root that would resolve to it, is accepted as a path inside the outer root — `runtime/src/primitives/mod.rs` — **waived**: each needs a staging root that does not exist or a link planted in one, which /ductus's own arguments never produce, and a path accepted that way is still contained by its root; the comment and plan now say exactly what is refused
- tests: no test pins that a staging path is matched by text before it is resolved — `runtime/src/primitives/mod.rs` — **waived**: on Unix the order changes only one message, and on Windows its effect is a network contact that does not happen, which a test could observe only by contacting a host
- perf: a link inside a feature directory to an ancestor makes every call on that feature walk the whole project and refuse on any outward link anywhere in it — `runtime/src/primitives/mod.rs` — **waived**: only a feature directory holding a link to an ancestor reaches it, no ductus artifact is authored as a link (067's sibling-symlink-trust-boundary), the visited set walks each directory once, and following inside links is what keeps a read through two links from passing unseen
- other: the error for an outward link prints the absolute path the link resolves to — `runtime/src/primitives/mod.rs` — **waived**: the runtime answers the operator's own agent over stdio, so no caller sits across a trust boundary, and the resolved path is what the operator needs to find the link
- other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs` — **waived**: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold
- other: verify-installer ignores a Retry-After of 2^63 or more and reads a redirect hop's Retry-After when the final hop sends none — `.github/workflows/runtime-release.yml` — **waived**: an out-of-range value falls back to the jittered wait, and GitHub's release-asset redirect sends no Retry-After, so neither changes a run
- other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml` — **waived**: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
- other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml` — **waived**: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
- other — c01b86e0 (rmcp 3.5.1 to 3.7.0, tokio 1.54.1, typed-path 0.12.3 to 0.13.0, 23 packages) records only that cargo build --release --locked is clean; the other lockfile commits in the window each record clippy and the test suite. The pre-commit hook runs both when runtime/ is staged, and every later bump in the window records them. — `runtime/Cargo.lock` — **waived**: A commit-message evidence gap on a lockfile-only change the hook gated; the lockfile at HEAD is exercised by every later gated commit, and no dependency pin policy (BE-DEPS-002) is broken. (decided by the rule)
- other — --package-lock-only judges the lockfile alone and passes against a package.json out of step with it, and runtime-currency.yml runs no npm ci — `.github/workflows/runtime-currency.yml` — **waived**: framework-checks.yml runs npm ci, which fails on a lockfile out of step with package.json, on every push and pull request, so such drift cannot reach main without failing that run (decided by the rule)
- convention — the `CHECK_STEP_REFERENCES` description omits that a command source or reference that is, or runs through, a symbolic link leaving the project is a skip, and that the subject includes two bootstrap files — `runtime/src/schema/descriptions.rs` — **waived**: no loaded rule requires a description to carry them and the code is correct; its exit clause, which was wrong, is fixed (decided by the rule)
- other — the `DERIVE_DEPENDENCIES` and `DERIVE_REFERENCES` descriptions say a zero `updated-count` means examined-and-clean when `unwritable-count` and `refused-count` are also zero, and those two are absent at zero — `runtime/src/schema/descriptions.rs` — **waived**: the same descriptions say absent at zero earlier, so a reader resolves it, and no check is made to pass that should not (decided by the rule)
- convention — the `prune-project` changelog entry is marked BREAKING while its body says the primitive is new in this release — `runtime/CHANGELOG.md` — **waived**: marked by the owner's ruling that every bounded-list entry is BREAKING (decided by the rule)
- other: `specs/066-audit-records-and-the-completion-gate/data-model.md` still defines `deleted` as the `modified-since` paths absent from the working tree (not found, or under a parent that is now a file), but `compute-review-scope` now also puts a path leaving the project through a link there, as the fix diff's `ComputeReviewScopeResult::deleted` doc, description and changelog say. — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **fixed** (decided by the rule)
- other: the predicate 'symlink_metadata answers NotFound or NotADirectory' now stands in three places: the new private `is_missing` in mod.rs, `deleted_paths` in `compute_review_scope.rs`, and the `symlink_metadata` match in `rewrite_spec_links.rs` — `runtime/src/primitives/mod.rs` — **waived**: Reuse, advisory. Sharing `is_missing` (as `pub(crate)`) is not mechanical here: `scripts/mutation/review-scope-cells-and-deletions.py` M4 and M10 quote `deleted_paths`' literal text and would need rewriting, and `rewrite_spec_links` returns an `Io` error for any other kind inside the same match. Three small copies of a two-kind predicate carry no divergence today. (decided by the rule)
- other: `invalidate_review.rs` still says an unreadable record 'is already in the state this produces, so there is nothing to invalidate', but a `review.md` that is a dangling link or a directory now fails earlier: `read_recorded_waivers(&feature_dir)?` returns an error through `read_recorded_list` where it returned an empty list before the fix — `runtime/src/primitives/invalidate_review.rs` — **fixed** (decided by the rule)
- tests: the CHANGELOG and `read_recorded_list`'s doc say a directory standing where a record belongs is an error, not an empty list, but `a_dangling_record_is_unreadable_not_absent` pins only the dangling link; the directory case rests on the same `is_missing` line — `runtime/src/primitives/mod.rs` — **waived**: The dangling-link assertions kill a revert of `is_missing` in both functions, and a directory takes the same non-missing branch (`symlink_metadata` succeeds, `read_to_string` errors), so a second test would add no separating input. (decided by the rule)
- bug: the CHANGELOG entry and the `deleted` field doc say a path under a replaced directory is now deleted, and omit that the link entry itself is also filed as deleted (see the finding at `deleted_paths`). — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- other: the new scenario `specs/022-deterministic-runtime/scenarios/link-registration-is-an-extension-point.md` and tasks.md task 65 name `runtime/src/schema/extensions.rs` and `validate_response` as what makes `registerService` an extension point `ductus exec link` serves, and omit `build_extension_request` in `runtime/src/interpreter/payload.rs`, which has no arm for it: the identifier falls to `PayloadError::UnknownExtension` (code `unknown-extension`) at the request build, before any response is validated, so a walk of `ductus exec link` would still halt at the step after the scenario's listed work is done — `specs/022-deterministic-runtime/scenarios/link-registration-is-an-extension-point.md` — **fixed** (decided by the rule)
- convention: `Walker::new` defaults `command_file` to `framework/commands/{command}.md` for every caller, which is a path the walk never read for any caller that does not call `with_command_file`: the 30 in-module tests, `tests/walker.rs` and `tests/specify_command.rs` (commands such as `test`, which have no such file), and a command `ductus exec` found under the installed config directory or `framework/bootstrap/` — `runtime/src/interpreter/mod.rs` — **waived**: The only caller whose envelopes a host reads is `run_exec`, and it always sets the real file. The doc comment on `with_command_file` states the default as a default ('the first place `ductus exec` looks'), not as the file read. Making the argument required would change the `Walker::new` signature at about 40 call sites for no host-visible effect. (decided by the rule)
- other: spec.md's 'Subprocess interpreter' bullets now carry `request-id` on the four request and response messages, but its `complete` and `error` bullets still omit `runtime-version` (and `error`'s `location`), which `ProtocolMessage` always emits — `specs/022-deterministic-runtime/spec.md` — **waived**: Outbound fields are ones a host reads and does not have to send, and a host that ignores them still works, unlike the `request-id` an inbound message must carry. The data model is the protocol's documentation and gives all of them. (decided by the rule)
- convention: the [Unreleased] Fixed entry 'An audit record that is a dangling link reads as unreadable, not never run' is not marked BREAKING, although a call that read such a record, or a directory standing in its place, as an empty list now fails, which this file marks BREAKING under `BE-COMPAT-001` elsewhere ('two new operational errors where the sweep used to write'; 'a call that recorded an unknown reason now fails') — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- convention: the [Unreleased] Fixed entry 'ductus exec's error locations name the command file, repo-relative' changes the value of `location.file` on dispatch and result errors from a command name to a path and is not marked BREAKING, as the earlier parse-error entry is not — `runtime/CHANGELOG.md` — **waived**: `ErrorLocation.file` is documented 'Repo-relative file path' and spec 022's data-model payload-path rule says the same, so the old value broke the contract and the change repairs it; the earlier parse-error entry set the same unmarked precedent. A consumer matching `location.file == "review"` would break, but no documented consumer does. The pipeline's own machinery, per AGENTS.md. (decided by the rule)
- other: the new scenario `link-registration-is-an-extension-point.md` has `registerService`'s response carry the local path and the runtime write the `[services.<alias>]` block, so under exec the path reaches config through a host response; AC29's boundary on `validate_no_traversal` (an operator's transcribed answer needs no check, a value the runtime or an LLM originates moves the table into the validated tier in the same change) applies to task 65's implementation — `specs/022-deterministic-runtime/scenarios/link-registration-is-an-extension-point.md` — **fixed** (decided by the rule)
- other: the new scenario `link-registration-is-an-extension-point` says the `[services.<alias>]` block "is then written additively" but does not say who writes it under `ductus exec`: `link.md` is host-driven with no runtime primitive, so a walk that reaches its end (the scenario's own done-when) completes with `.ductus/config.toml` unwritten, the shape `a-confirmed-gate-authorizes-the-writes-after-it` records as a defect when nothing says the host performs the write — `specs/022-deterministic-runtime/scenarios/link-registration-is-an-extension-point.md` — `specs/022-deterministic-runtime/scenarios/link-registration-is-an-extension-point.md` — **fixed** (decided by the rule)
- other: the new scenario's request "carries the fields `link.md` step 2 names that its arguments left unset", but `ductus exec` binds only `key=value` arguments and halts with `invalid-argument` on any other (`exec_overrides` in `runtime/src/main.rs`), so `link.md`'s positional form (`<alias> <repo> <path>`) and its `--list` form have no exec spelling, and the scenario names no context key (`alias`, `repo`, `path`, `description`) the request reads them from, nor says what `--list` does under exec — `specs/022-deterministic-runtime/scenarios/link-registration-is-an-extension-point.md` — `specs/022-deterministic-runtime/scenarios/link-registration-is-an-extension-point.md` — **fixed** (decided by the rule)
- other: the data model's `error` envelope example shows `location` as always present, but `ProtocolMessage::Error.location` is optional and only a parse error and an error at a primitive step carry it (an `unknown-extension` halt, which the new scenario names, carries none; the probe's envelope has no `location`) — `specs/022-deterministic-runtime/data-model.md` — `specs/022-deterministic-runtime/data-model.md` — **fixed** (decided by the rule)
- tests: the macOS integration test leaks its mktemp directory if building the archive or spawning the binary panics — `runtime/tests/exec_subprocess.rs` — **waived**: it leaks only on a panic before its assertions, one empty directory under the per-user temp directory, which macOS clears
- other — the CLI flag hydration is a table of 21 hydrate_field calls and tests prove only the shared helper and two primitives, so a call dropped or pointed at the wrong argument (applied against expired waivers, say) fails no test; every_mcp_argument_has_a_cli_form pins clap presence only — `runtime/src/main.rs:339` — **waived**: d3192610 records the hydration helper proved red with its assignment removed, and each of the 21 calls pairs a flag with its same-named field, so a swap shows in review as a name mismatch (decided by the rule)
- perf — each assessSpecQuality request reads and canonicalizes the spec once and runs the read-size check twice (fits in the builder, then again in bound_lists), each of which reads and parses the project config, so a 192-rule walk does about 400 config parses — `runtime/src/interpreter/payload.rs:1329` — **waived**: each parse is a sub-millisecond read of one small file and the walk is bounded at 192 requests, each of which waits on a host round trip that costs seconds (decided by the rule)
- other — the REWRITE_SPEC_LINKS description in descriptions.rs states every behavior a listed commit changed except the project-root prefixes (a leading ./ or / in a root markdown file, ./ in the hook, ../ or / in system.md, kept as written) that 38fc39f6 and 54715126 added; it says only spec-root paths named in the referrers. Every claim it does make was checked against the code and holds, including the five lists, counts and refused semantics; the parenthetical that a spec root that is itself a link is followed holds only inside the project, where a root resolving outside is refused as an invalid path (driven on a fixture) — `runtime/src/schema/descriptions.rs:94` — **waived**: an omission, not a false claim: the prefix forms are spec-root paths the description already names, 067's data model states the boundary, and the outside-the-project refusal is the primitive's documented InvalidPath error (decided by the rule)
- other — check-artifacts' agent_scaffold_destinations lost its project-scope filter and its home-relative filter in b1d319f1 (PROJECT_SCOPES and the starts_with tilde test): every registered agent's MCP target now counts as a path shipped to adopters. With Claude and Pi, both project-scoped, the result is unchanged, but a future host with a user-global target would make a criterion naming a home path skip as shipped instead of being examined. — `runtime/src/primitives/check_artifacts.rs:1620` — **waived**: the guard served hosts that are no longer registered; the two that remain are project-scoped, so the filter checked nothing the registry can now hold, and the test the_registry_arm_derives_each_agents_scaffold_paths pins the current set (decided by the rule)
- other — load_scope_files and load_rule_files skip a path that is missing, outside the project or not UTF-8 text, so performReview's scope-files can hold fewer files than the scope the pass was asked to cover with nothing on the request naming the omission — `runtime/src/interpreter/payload.rs:566` — **waived**: the omission is the documented posture of the builder ('the pass reviews what it can read') and review-runtime-acceleration states the request carries the in-scope files; the walker context's scope and selected lists travel in the legacy tail (or the lists file past the bound), so the host can compare them with scope-files, and compute-review-scope sets deleted files apart before the request is built (decided by the rule)
- other — routeFold's load_fold_source_scenarios gives a scenario whose section frontmatter cannot be read an empty section (read_scenario_section unwrap_or_default), the same unreadable-looks-empty shape unreadable-scenario-is-reported names for read-spec — `runtime/src/interpreter/payload.rs:1136` — **waived**: the scenario is still listed by slug, so the fold's host sees it exists, and the section is context the route decision does not key on (the scenario crosses over unchanged, as the builder's doc states); unreadable-scenario-is-reported governs the open-question collector, which this reader is not (decided by the rule)
- perf — build_perform_review_request runs load_scope_files and load_rule_files in each of the five passes, so the whole scope and all loaded rule files are read from disk five times per review although the loaded-rule-files list needs only names and rule-files only the pass's own files' contents — `runtime/src/interpreter/payload.rs:463` — **waived**: local file reads once per human-timescale review pass, against an LLM call that dominates; the performance-backend rules are design-time commitments for a deployed service and do not fire against a local stdio runtime (decided by the rule)
- other — MARK_CRITERION, SET_STATUS (and MARK_TASK) say the call is refused with changed-since-read when the file changed between the read and the write, but a flip or status change that changes nothing writes nothing (mark_criterion.rs and set_status.rs guard the write with previous != checked and to != from), so such a call succeeds whatever another writer did; only MARK_TASK states the no-op writes nothing. — `runtime/src/schema/descriptions.rs:70` — **waived**: a call that writes nothing has no write for the clause to refuse and leaves no lost update, since the file already holds the requested state when read; the clause is stated for the calls that write (decided by the rule)
- convention — about thirty descriptions repeat the same past-the-read-size sentence (form is file, lists-file names the JSON file under .ductus/results/ holding the moved lists whole) and ten repeat the changed-since-read sentence in near-identical wording; the copies already differ (DASHBOARD's variant above), and a macro or shared fragment would hold one. — `runtime/src/schema/descriptions.rs:26` — **waived**: each description is the one text an MCP host and the --help output show for its primitive (the file's own header), so it must stand alone; a concat-style fragment would change how every constant is built and none of the copies is wrong except the one named above (decided by the rule)
- other — a stdout reader that has gone is a quiet exit 141 for every result written through write_stdout_line, but ductus exec (protocol envelopes written by the interpreter and emit_exec_error) and gate-confirm still report it as exit 1 or 74 with a stderr message. Driven on a fixture: resolve-session exits 141 with empty stderr; exec of an unknown command exits 1 and prints 'failed to emit error envelope: Broken pipe'. No panic in either, and the scenario a-closed-stdout-is-a-quiet-exit scopes the fix to the result emitters in main.rs. — `runtime/src/main.rs:733-748` — **waived**: the scenario's Context names the result emitters in main.rs as the sites and its test covers three of them; exec's stdout is the walk's protocol channel, where a closed reader ends the walk with an error status distinct from a panic, which the scenario's second behavior requires, and nothing is written twice (decided by the rule)
- other — the 21 --{name}-json flags are each hydrated by one hydrate_field call in main.rs (checked: 21 flags in schema/primitives.rs, 21 calls, every flag paired with its own field), but only process-waivers' fired-json and write-review's fix-rounds-json are driven by a test; every_mcp_argument_has_a_cli_form checks that a flag exists, not that main.rs reads it, so dropping a hydrate_field call for any of the other 19 passes the suite and silently sends an empty list. The commit message records one mutation proof only. — `runtime/src/main.rs:1719-1814` — **waived**: scenario every-mcp-argument-has-a-cli-form requires one test that every MCP argument has a CLI form, which every_mcp_argument_has_a_cli_form meets, and the hydration test the commit adds proves its behavior for two flags; the other pairings were read and are correct today, and no loaded rule's Statement is violated by the gap (decided by the rule)
- perf — a performReview or writeCode request is serialized and measured twice per build, once by the body step and once by bound_lists, and each measure re-reads the config for the read size — `runtime/src/interpreter/payload.rs:507` — **waived**: one request per host round trip, each measured twice at the size of the request itself, against a model call that takes seconds (decided by the rule)
- other — the two CLI json-flag tests drive 2 of the 14 hydration arms task 24 added (fired-json and fix-rounds-json), so deleting or mis-wiring another arm leaves both green while the flag is ignored; the commit records a mutant for each test, not for each arm — `runtime/src/main.rs:1774` — **waived**: each omitted arm is one hydrate_field call whose flag and field names were read pairwise here and agree, and the commit records how each test was shown to fail (decided by the rule)
- other — runtime-release.yml installs cargo-zigbuild and cargo-cyclonedx by exact version with cargo install --locked, not by commit or hash, while ziglang carries a sha256; the comment above the zig step says both are pinned under SEC-DEPS-001 — `.github/workflows/runtime-release.yml:187` — **waived**: cargo checks a crate against the checksum in the registry index and a crates.io version cannot be overwritten, as 022's plan Release workflow section states; the pins change only through scripts/check-runtime-currency.sh. (decided by the rule)
- tests — after b1d319f1 ported the Auggie fixture to a Pi-shaped one, no exec subprocess test reaches the claude-style commands/<project>/ candidate through ductus exec; the candidate order is asserted in host.rs (command_file_candidates_cover_claude_style_then_pi) and the installed claude-style path through the MCP server in tests/mcp.rs — `runtime/tests/exec_subprocess.rs:446-500` — **waived**: locate_exec_command walks a candidate list and is agnostic to which layout produced a candidate, and that list is asserted at the unit level, so the end-to-end exec path loses no distinct branch. (decided by the rule)
- other — runtime.yml's build-and-test job runs npm ci on three images with no npm audit signatures step, where framework-checks.yml runs one after its npm ci; the installed markdownlint tree is a test input, shipped in no artifact — `.github/workflows/runtime.yml:173-175` — **waived**: BE-DEPS-004 asks for the verification before a dependency is built into or shipped with an artifact; the tree installed here is a test tool, ships in none, and framework-checks.yml verifies the same lockfile's signatures on every push and pull request. (decided by the rule)
- other — the scenario's process-waivers summary (expire when the file is gone or the rule no longer fires in a file the run's scope holds) and the review reference's Per-run waiver processing items 2 and 3 omit two carve-outs the primitive's description states: a plan Affected Files entry the scope holds counts as reading a file, and a scope that cannot be read retains a waiver even for a deleted file. — `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md:23` — **waived**: Both are summaries of a rule the primitive's compiled description states whole and canonically (PROCESS_WAIVERS); the omissions narrow neither the scenario's tests nor the reference's steps, and a restatement of the carve-outs would be a second copy of a canonical source. (decided by the rule)
- other — ask-consolidation's Behavior lines for create-scenario and append-task still say 'Atomic via tempfile-in-parent + persist rename'. create-scenario now persists with no-clobber (write_new_atomic) and append-task writes through the compare-before-persist guard; the Edge Cases bullets for both state the exact mechanism. — `specs/022-deterministic-runtime/scenarios/ask-consolidation.md:24` — **waived**: The line is true at the level it states (a tempfile in the parent, persisted by rename), and the two Edge Cases bullets the commits for tasks 33 and 35 rewrote carry the mechanism, so no reader acts on a wrong claim; a second statement of the mechanism would be a copy that can rot. (decided by the rule)
- other — the scenario slug ductus-bootstrap is cited as a spec 022 scenario in seven live places, but the scenario file is govern-bootstrap.md (title Ductus bootstrap); no file named ductus-bootstrap ever existed in git history. 022's plan.md line 338 links it correctly with the label ductus-bootstrap and the path govern-bootstrap.md. Citations outside this part's table: runtime/src/main.rs:866, runtime/src/primitives/fetch_archive.rs:5, scripts/lint-procedure-parseability.sh:78, specs/022-deterministic-runtime/plan.md:338, scenarios/apply-manifest.md:9, scenarios/runtime-primitive-structural-bugs.md:13, runtime/tests/fixtures/ductus-basic/framework/bootstrap/install.md:28. — `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md:1` — **waived**: Not mechanical: the repair is a choice between renaming the scenario file to ductus-bootstrap.md (one rename and one plan.md link; the digest key in analysis.md regenerates) and re-pointing seven prose citations in four directories, including a runtime/ comment batch. The scenario is reachable by its link in plan.md and its title, so no reader is sent to a missing file by a link; it holds no requirement. (decided by the rule)
- convention — the Exec arguments paragraph sits between the inbound messages' JSON-shape sentence and the gate-response paragraph that continues the same topic — `specs/022-deterministic-runtime/data-model.md:124` — **waived**: the move would change no claim, rule or behavior, which the Observations rules in framework/reference/review.md say is no observation (decided by the rule)
- other — `every_json_flag_help_states_its_arguments_shape` in `runtime/src/main.rs` compares field names only, and `scripts/mutation/json-flag-help-022.py` has no mutant for type labels or three developer-note regex arms — `runtime/src/main.rs` — **waived**: QUAL-TEST-001's Not flagged clause covers a test whose mutation list records how it was shown to fail, as M1 to M4 do for the named behaviors; mutants for the type labels are hardening no criterion asks for. (decided by the rule)
- other — `every_mcp_argument_has_a_cli_form` in `runtime/src/main.rs` starts an in-memory MCP server to read schemas that `tool_input_schemas` now returns directly — `runtime/src/main.rs` — **waived**: The test proves the served tools/list response, which tool_input_schemas does not build (no reject_unknown_fields or flag_always_loaded), so replacing it would change which surface the test proves. (decided by the rule)
- other — the command-file lookup ladder is built twice, by `exec_candidates` in `runtime/src/main.rs` and `locate_command_file` in `runtime/src/interpreter/payload.rs` — `runtime/src/main.rs` — **waived**: Both take their installed candidates from Host::command_file_candidates, so the layout set lives in one place; merging the two ladders is a refactor of two callsites, not a chore. (decided by the rule)

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
- `.claude/commands/ductus/plan.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/status.md`
- `.claude/commands/ductus/target.md`
- `.ductus/docs/agents/developing-ductus.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/prune.md`
- `.ductus/reference/review.md`
- `.githooks/pre-commit`
- `.githooks/pre-push`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-plan.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-status.md`
- `.pi/prompts/ductus-target.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `docs/runtime.md`
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
- `framework/commands/plan.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/status.md`
- `framework/commands/target.md`
- `framework/constitution.md`
- `framework/migrations.toml`
- `framework/migrations/audit-record-relocate.md`
- `framework/migrations/criterion-label-backfill.md`
- `framework/migrations/project-review-record.md`
- `framework/migrations/report-project-file-links-into-docs.md`
- `framework/migrations/section-digest-backfill.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/fold.md`
- `framework/reference/groom.md`
- `framework/reference/implement.md`
- `framework/reference/plan.md`
- `framework/reference/prune.md`
- `framework/reference/review.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `install.sh`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_question.rs`
- `runtime/src/primitives/backfill_section_digest.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_audit_records.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_prescriptions.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_rule_ids.rs`
- `runtime/src/primitives/check_stuck.rs`
- `runtime/src/primitives/check_unreviewed_code.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/criterion_verdicts.rs`
- `runtime/src/primitives/dashboard.rs`
- `runtime/src/primitives/decisions.rs`
- `runtime/src/primitives/derive_references.rs`
- `runtime/src/primitives/diff_cross_spec.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/line_citations.rs`
- `runtime/src/primitives/lint_markdown.rs`
- `runtime/src/primitives/measure_reopenings.rs`
- `runtime/src/primitives/mechanical_sweep.rs`
- `runtime/src/primitives/merge_permissions.rs`
- `runtime/src/primitives/merge_status_hook.rs`
- `runtime/src/primitives/owed_work.rs`
- `runtime/src/primitives/pipeline_view.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/project_parts.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_project.rs`
- `runtime/src/primitives/prune_project/moves.rs`
- `runtime/src/primitives/prune_review.rs`
- `runtime/src/primitives/read_re_read.rs`
- `runtime/src/primitives/read_tasks.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/review_subject.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/spec_ownership.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/moved_scope.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/src/primitives/write_review/seed.rs`
- `runtime/src/schema/config_keys.rs`
- `runtime/src/schema/paths.rs`
- `runtime/src/schema/registry.rs`
- `runtime/src/session.rs`
- `runtime/tests/check_artifact_size_cli.rs`
- `runtime/tests/check_audit_records_cli.rs`
- `runtime/tests/check_unreviewed_code_cli.rs`
- `runtime/tests/common/mod.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/fix_rounds.rs`
- `runtime/tests/fixtures/review-basic/specs/007-review/plan.md`
- `runtime/tests/fixtures/review-basic/specs/007-review/spec.md`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `runtime/tests/measure_reopenings.rs`
- `runtime/tests/mechanical_sweep_parity.rs`
- `runtime/tests/merge_status_hook.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/review_command.rs`
- `runtime/tests/size_hook.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/analyze_lib.py`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/combine.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/record.py`
- `scripts/analyze/scenario-brief.md`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/audit/README.md`
- `scripts/audit/adopter-shell-behavior.sh`
- `scripts/audit/cross-doc-consistency.sh`
- `scripts/audit/done-spec-criteria.sh`
- `scripts/audit/rename-sweep-residue.sh`
- `scripts/audit/run-all.sh`
- `scripts/audit/ssot-invariants.sh`
- `scripts/audit/sweep-target-manifest-parity.sh`
- `scripts/lint-governing-size.sh`
- `scripts/lint-tool-coverage.sh`
- `scripts/mutation/artifact-size-bounded.py`
- `scripts/mutation/audit-records-085.py`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/compare-and-set-041.py`
- `scripts/mutation/compare-and-set-066.py`
- `scripts/mutation/criteria-in-review-066.py`
- `scripts/mutation/criterion-sweep-066.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/first-review-scope-086.py`
- `scripts/mutation/line-citation-strip-066.py`
- `scripts/mutation/loop-inputs-screened.py`
- `scripts/mutation/never-owed-084.py`
- `scripts/mutation/outcomes-outside-the-scope-086.py`
- `scripts/mutation/owed-work-084.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/plan-edit-takes-no-back-edge.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/process-decisions-retained.py`
- `scripts/mutation/project-parts-087.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/prune-project-containment.py`
- `scripts/mutation/prune-project.py`
- `scripts/mutation/prune-review-086.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/resolve-constitutions-size.py`
- `scripts/mutation/review-022-task-42.py`
- `scripts/mutation/review-022-task-46.py`
- `scripts/mutation/review-drops-unowned-086.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/rule-judgments-088.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/mutation/run.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/spec-ownership-086.py`
- `scripts/mutation/status-count-084.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/suffixed-task-number.py`
- `scripts/mutation/unreviewed-code-084.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/stale-literals.sh`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-audit-path-containment.sh`
- `scripts/tests/test-audit-record-gate.sh`
- `scripts/tests/test-done-spec-owed-work.sh`
- `scripts/tests/test-lint-governing-size.sh`
- `scripts/tests/test-pre-commit-staging.sh`
- `scripts/tests/test-stale-literals.sh`
- `specs/000-slash-commands/plan.md`
- `specs/000-slash-commands/review.md`
- `specs/000-slash-commands/scenarios/command-autocomplete-summary.md`
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
- `specs/000-slash-commands/scenarios/scenario-without-task-visibility.md`
- `specs/000-slash-commands/scenarios/target-clear-flag.md`
- `specs/000-slash-commands/spec.md`
- `specs/002-project-scaffolding/analysis.md`
- `specs/002-project-scaffolding/plan.md`
- `specs/002-project-scaffolding/spec.md`
- `specs/003-bootstrap-automation/analysis.md`
- `specs/003-bootstrap-automation/plan.md`
- `specs/003-bootstrap-automation/review.md`
- `specs/003-bootstrap-automation/spec.md`
- `specs/003-bootstrap-automation/tasks.md`
- `specs/006-bug-workflow/spec.md`
- `specs/007-govern-workflow/plan.md`
- `specs/007-govern-workflow/spec.md`
- `specs/007-govern-workflow/tasks.md`
- `specs/008-security-rules/plan.md`
- `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md`
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/009-scenario-targeting/analysis.md`
- `specs/009-scenario-targeting/review.md`
- `specs/009-scenario-targeting/spec.md`
- `specs/010-agent-autonomy/plan.md`
- `specs/010-agent-autonomy/spec.md`
- `specs/011-brownfield-process/spec.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/scenarios/settings-template-bash-allowlist.md`
- `specs/012-multi-agent-govern/spec.md`
- `specs/012-multi-agent-govern/tasks.md`
- `specs/013-text-first-artifacts/plan.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/013-text-first-artifacts/spec.md`
- `specs/014-reclarify-backedge/plan.md`
- `specs/014-reclarify-backedge/review.md`
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md`
- `specs/014-reclarify-backedge/spec.md`
- `specs/015-tarball-fetch/spec.md`
- `specs/015-tarball-fetch/tasks.md`
- `specs/016-cross-cutting-rules/plan.md`
- `specs/016-cross-cutting-rules/review.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/data-model.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/scenarios/tracked-specs-not-worktree.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/plan.md`
- `specs/018-adopter-owned-pre-commit/review.md`
- `specs/018-adopter-owned-pre-commit/spec.md`
- `specs/019-config-decisions/analysis.md`
- `specs/019-config-decisions/review.md`
- `specs/019-config-decisions/spec.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/021-runtime-boundary/plan.md`
- `specs/021-runtime-boundary/review.md`
- `specs/021-runtime-boundary/spec.md`
- `specs/023-govern-refinement/plan.md`
- `specs/023-govern-refinement/review.md`
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md`
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md`
- `specs/023-govern-refinement/scenarios/configure-retires-formerly-canonical-entries.md`
- `specs/023-govern-refinement/scenarios/living-specs.md`
- `specs/023-govern-refinement/spec.md`
- `specs/023-govern-refinement/tasks.md`
- `specs/024-rule-loader/plan.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/025-rule-opt-out/review.md`
- `specs/025-rule-opt-out/scenarios/a-malformed-disabled-entry-fails-fast.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/026-framework-self-audit/scenarios/audit-ci-hard-gate.md`
- `specs/026-framework-self-audit/scenarios/family-18-marker-list-parity.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/026-framework-self-audit/scenarios/family-27-done-spec-unchecked-criteria.md`
- `specs/026-framework-self-audit/scenarios/family-28-audit-family-registry-parity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/027-bootstrap-migration-registry/analysis.md`
- `specs/027-bootstrap-migration-registry/plan.md`
- `specs/027-bootstrap-migration-registry/review.md`
- `specs/027-bootstrap-migration-registry/spec.md`
- `specs/027-bootstrap-migration-registry/tasks.md`
- `specs/029-bootstrap-runtime-autowire/plan.md`
- `specs/029-bootstrap-runtime-autowire/review.md`
- `specs/029-bootstrap-runtime-autowire/scenarios/archive-fetch-direct-codeload.md`
- `specs/029-bootstrap-runtime-autowire/scenarios/project-inputs-asked-once.md`
- `specs/029-bootstrap-runtime-autowire/scenarios/runtime-probe-parity-audit.md`
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md`
- `specs/029-bootstrap-runtime-autowire/spec.md`
- `specs/029-bootstrap-runtime-autowire/tasks.md`
- `specs/030-cross-service-references/plan.md`
- `specs/030-cross-service-references/spec.md`
- `specs/033-rule-surface-setting/review.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/035-groom-session-target/plan.md`
- `specs/035-groom-session-target/review.md`
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md`
- `specs/035-groom-session-target/scenarios/reopen-done-spec-on-scenario.md`
- `specs/036-quality-cross-rules/analysis.md`
- `specs/036-quality-cross-rules/review.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
- `specs/040-configurable-specs-dir/plan.md`
- `specs/040-configurable-specs-dir/scenarios/command-prose-resolves-spec-root.md`
- `specs/040-configurable-specs-dir/spec.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/review.md`
- `specs/041-task-pruning/scenarios/a-plan-edit-does-not-reopen-a-done-spec.md`
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md`
- `specs/041-task-pruning/scenarios/prune-all-walks-every-spec.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/041-task-pruning/tasks.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/analysis.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/plan.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/review.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md`
- `specs/043-workflows-sunset/review.md`
- `specs/043-workflows-sunset/spec.md`
- `specs/044-relocate-constitution-under-govern-directory/plan.md`
- `specs/045-decision-state-drift-detection/data-model.md`
- `specs/045-decision-state-drift-detection/plan.md`
- `specs/045-decision-state-drift-detection/review.md`
- `specs/045-decision-state-drift-detection/spec.md`
- `specs/046-scenario-open-question-visibility/plan.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/047-analyze-findings-durability/plan.md`
- `specs/047-analyze-findings-durability/review.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md`
- `specs/047-analyze-findings-durability/spec.md`
- `specs/048-govern-acquired-runtime/data-model.md`
- `specs/048-govern-acquired-runtime/review.md`
- `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md`
- `specs/048-govern-acquired-runtime/scenarios/pin-is-readable-when-acquisition-needs-it.md`
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
- `specs/050-constitution/scenarios/findings-route-by-scope.md`
- `specs/050-constitution/scenarios/knowledge-routes-by-population-not-by-kind.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-pending-fold-never-holds-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/fold-target-checked-before-the-rewrite.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/052-spec-supersession-and-consolidation/analysis.md`
- `specs/052-spec-supersession-and-consolidation/review.md`
- `specs/052-spec-supersession-and-consolidation/scenarios/stranded-session-after-removal.md`
- `specs/052-spec-supersession-and-consolidation/tasks.md`
- `specs/054-remove-supersession/plan.md`
- `specs/054-remove-supersession/review.md`
- `specs/054-remove-supersession/spec.md`
- `specs/055-shared-constitution/data-model.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/056-bootstrap-archive-boundary-split/analysis.md`
- `specs/056-bootstrap-archive-boundary-split/review.md`
- `specs/056-bootstrap-archive-boundary-split/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/data-model.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/review.md`
- `specs/058-findings-route-at-discovery/scenarios/findings-cost-in-proportion-to-risk.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/059-project-in-a-repository-subdirectory/review.md`
- `specs/059-project-in-a-repository-subdirectory/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/analysis.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/review.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/rule-judging-runs-from-repo-scripts.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/tasks.md`
- `specs/061-updates-track-the-latest-release-tag/plan.md`
- `specs/061-updates-track-the-latest-release-tag/review.md`
- `specs/062-concurrent-session-targets/plan.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/062-concurrent-session-targets/spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`
- `specs/064-pi-host-support/analysis.md`
- `specs/064-pi-host-support/scenarios/the-bridge-closes-its-server-at-shutdown.md`
- `specs/064-pi-host-support/spec.md`
- `specs/064-pi-host-support/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-multi-number-renumber-is-one-rewrite.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-renamed-file-contributes-its-rewrites.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-review-states-what-it-read.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/freshness-results-have-a-bounded-size.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-base-includes-the-transition-commit.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-committed-tree-horizon.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-cross-spec-impact-gate.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/write-analysis-and-the-second-gate.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md`
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/071-command-reference-split/plan.md`
- `specs/071-command-reference-split/review.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/072-governing-documents-stay-within-one-read/plan.md`
- `specs/072-governing-documents-stay-within-one-read/spec.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/073-decided-rule-departures-are-recorded/tasks.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/075-agents-md-holds-what-every-session-needs/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/spec.md`
- `specs/075-agents-md-holds-what-every-session-needs/tasks.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/analysis.md`
- `specs/080-deterministic-runtime-efficiency/data-model.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/research.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/082-rule-assessments-converge-across-runs/tasks.md`
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
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/analysis.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/data-model.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/plan.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/review.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/tasks.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/analysis.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/review.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/analysis.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/review.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`
- `specs/088-rule-judgments-cost-only-what-changed/analysis.md`
- `specs/088-rule-judgments-cost-only-what-changed/data-model.md`
- `specs/088-rule-judgments-cost-only-what-changed/plan.md`
- `specs/088-rule-judgments-cost-only-what-changed/review.md`
- `specs/088-rule-judgments-cost-only-what-changed/spec.md`
- `specs/088-rule-judgments-cost-only-what-changed/tasks.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/analysis.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/review.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/tasks.md`
- `specs/README.md`
- `specs/inbox.md`
- `specs/review.md`

## Unexamined governance

*None.*
