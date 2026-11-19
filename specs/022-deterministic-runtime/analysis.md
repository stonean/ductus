---
spec: 022-deterministic-runtime
last-run: 2026-11-19T17:30:43Z
analyzed-against: 4e9271179cdb782d52770fa94220aaaa828fe7fb
hard-fail: 0
blocking-findings: 0
advisory: 11
unexamined: 85
analyzed-digest:
  data-model.md: b18e13ef537f7a75101dfbe3f866554a1a881724627e42b31000403b5b9d9829
  plan.md: f44b8f37c0fd5b6bce0331dc1815cbac2a664ff5a37a09dff739b09447f2ce8e
  review.md: e97a192ec4477954d9cdbd7f0cad4cdc7268a316a1ca8ce3a26ec40eb4f19709
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
  scenarios/commands-dir-parameterization.md: 9e170f99b9d80d6b3657af6f3f14b14dde362cab2b1b05733c78a33939b47422
  scenarios/config-readers-name-unknown-keys.md: 569aec4b2acbb964f737bb9499a4fecc7dfc1807aad5a4db0ad360d4da6e5fff
  scenarios/config-resolution-single-probe.md: a487eec0a7cce706acd911511115140073284ca5b35409ec24cf4f2546b38724
  scenarios/config-sourced-paths-and-the-traversal-boundary.md: 3ed48e5af4e6a0ca019f9f7ae961771a01e94e595c02ae7c614258b42d777873
  scenarios/constitution-excerpts-as-skill-resources.md: b3a47b109ba64ff7d4ca132f66159e8de232aa9e659330e1968153dc63148b8b
  scenarios/coverage-expansion-primitives.md: 8fbe190c64b21c9f0edf92d764bdf9138d4f122830eea60df726d526859578d6
  scenarios/coverage-residue-cleanup.md: 0c6315ac8f0f041f624b0a389bbfca855d9886bb06703cef4ee5ffd513770d17
  scenarios/criterion-label-assignment.md: 57e455a91d0b76c6a3a5bdea80288300c246d00d53425730be88896e89c25305
  scenarios/dashboard-primitive.md: 70d8a91916780bab45ed495f1c7f197d1ebd83f27caba0b1a4a2ea9849434fca
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
  scenarios/govern-bootstrap.md: 5ffb4022ff53b32e9288a4f15fff7cd3441a2cea1bf42f07fbd080c264529089
  scenarios/groom-command-acceleration.md: dd6052b3d7ee1b1f8528913b82eabfbaf6debe4de51043824b2fcd5d040ed27e
  scenarios/host-protocol-conformance.md: 13c3a2d89eedf4f79e66c9d4e8e97a9b7361e6fd1fa340a76c46730c28e4d542
  scenarios/implement-completion-gate.md: d094ebf57e1f453690a6dc01fd28282df377b3c4362daf90f95f76183d635292
  scenarios/installed-files-are-not-group-writable.md: 3a29eb614d5bd804698622713e8d18e29d8b68f3030d342a4e1f1be30c276cc4
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
  scenarios/parser-walker-conventions.md: b53b0ffcadc43ec54c18f6b473b84021cddef418bf6481227b87f06a6acf08cb
  scenarios/primitive-robustness-hardening.md: 7cdf3dcf99c06332244540b196c97b13783184f7b9fba989108dd98f84524ae5
  scenarios/project-directory-resolution-chain.md: d43223e3f037960a24f5660a9944aae0fe02df5f1da10257477926a63cfb5a82
  scenarios/resolve-references-cli-exec-wiring.md: 51d572e34ffca5314a4a7af140296fbc1c34008a46a4fd7a65216eae522aa957
  scenarios/review-exec-wiring.md: 9dbf0c621deaa58d40d16d09c289abdd5cb31ab5555f2036088e1d8f04d76515
  scenarios/review-runtime-acceleration.md: 653e0021c5082d77cc4f827fbfcd309297f20a2d1eb277fdde752e677fc10084
  scenarios/review-scope-parse-fidelity.md: f970155028f105cc7441f1597113df13592fc0e0b0fceee80b60787e420ef2ac
  scenarios/runtime-primitive-structural-bugs.md: 7fb973b4c5ef5c0268df24996c585f9bf70465f4f7e36ad237d129894f5fd366
  scenarios/runtime-review-019-hardening.md: dd72b20cb886ee804141895d169ebb90eb5c548df260fb02cb1643dd2a6f777a
  scenarios/scaffolding-primitives.md: c3495b99e2f440822fed57cb08ba0cd668e9093fd786ad37e616a8508417c699
  scenarios/scenario-open-question-signal.md: 58fb1c80f5dacc4e4d2fb3619de1b626363283f71ed460910c10ac80f0ecfd65
  scenarios/scenario-question-parser-fix.md: bdcaf5f9bb7d0ff795d6e3a6a5045a79afc63b55d8ce4b3331f807c337bf2ada
  scenarios/severity-is-a-closed-set-not-a-string.md: d4ea4d23382e2968d72df879d89c7617ffe0b1a8f4049430a25cf70d5f1bd25f
  scenarios/skipscanner-inline-code-exemption.md: ab402c94a437b19b1b9bdd5a58ad3303d6bb67e9822041520522e3a066c43ab5
  scenarios/spec-side-parser-hardening.md: 91a804882033f369ff98f36157a38896d29eb7d90e8ae4c78e86d45ac35aca3d
  scenarios/specify-routes-before-scaffolding.md: afa3ced4b110e3b7d2f2e96e86cfd719487fce6983e948cbfea4150f4670ea33
  scenarios/the-cli-surface-nothing-pins.md: 634128f508bb6cc3d94913216ce5aefa3571a93bae2e2d45ecb8a0accf22dcc6
  scenarios/the-constitutions-registry-validates-its-values.md: 161cf85126b357e1c7fe6e63be2de5d80e96842a26acb3ed299af55f4bd92acc
  scenarios/the-pi-command-candidate.md: 577baec752506c60c08e56bbff45ff33e8e21f06d1763d73b7fe9e3170de41d0
  scenarios/unchecked-done-when-clause-tally.md: a599283d598b98880175f236aa7f42455e88669e9b01f818d22d45fad7f5767a
  scenarios/unreadable-scenario-is-reported.md: 28c763fee22e177c50586f69989911128438c23964e1fa4831f628ae366792ae
  scenarios/waiver-processing-order.md: 0adb750e651d94ac8bcf06dd0dfbba84cd937c09817035e913ec08b77687f8e8
  scenarios/write-boundary-path-normalization.md: aea2703ee38d7a2ab5806444dd12f99370bfee540bafd8935c9f20d43bd86dee
  scenarios/write-session-primitive.md: 56779f1f97b3622d04b44e61e3c05653afa8f3010014f0699b3f234a630f7db5
  scenarios/writecode-boundary-derivation.md: 44bbce7e22a35dd58048811c3441f633e2e14f37e48dcdeac089c42276a639cd
  scenarios/writecode-payload-bundling.md: f7d9803245a4ce80944bce99df469356646005d213816a1cf9dda26a64537b41
  scenarios/writecode-payload-canonicalize-paths.md: 635d30397102d2ab90aed011a4499395c13b469d281126fc944b9930fc16522f
  spec.md: 452d6947b1f74d83237d165628189914f87e1f887322cde5b9b7d8846b50d487
  tasks.md: 5923f99f40a76b942f41ba847eb9fb5643b5a93c72a9d9ce5cc777ee1005ea98
unexamined-by-reason:
  never-existed: 13
  not-a-live-claim: 12
  root-absent: 54
  ships-to-adopter: 6
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-ASYNC-001: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-AUTHZ-004: {verdict: addressed, cite: "plan.md#Agent reach: what ductus installs grants only what its steps need"}
  BE-AUTHZ-007: {verdict: addressed, cite: "data-model.md#Primitive request/response schemas"}
  BE-BULK-002: {verdict: addressed, cite: "data-model.md#`dashboard` — fields added since (addendum)"}
  BE-COMPAT-001: {verdict: addressed, cite: "data-model.md#Versioning of these schemas"}
  BE-COORD-002: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-DATA-001: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-DATA-003: {verdict: addressed, cite: "plan.md#Dependency selection"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-DEPS-001: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-002: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-003: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-DEPS-004: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-005: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-007: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#Outbound (runtime → host) messages"}
  BE-ERR-004: {verdict: addressed, cite: "data-model.md#JSON-over-stdio protocol"}
  BE-ERR-005: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Extension-point schemas (initial release)"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  BE-INPUT-006: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-INPUT-007: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#Dependency selection"}
  BE-INPUT-011: {verdict: addressed, cite: "plan.md#Dependency selection"}
  BE-INPUT-016: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-LOCK-002: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-LOG-002: {verdict: addressed, cite: "plan.md#Dependency selection"}
  BE-LOG-005: {verdict: addressed, cite: "data-model.md#Notes"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-PAYLOAD-002: {verdict: addressed, cite: "data-model.md#`read-spec` — parse spec frontmatter and body sections"}
  BE-PAYLOAD-004: {verdict: finding, tier: should, at: "plan.md#`fetch-archive` transport", message: "the plan's download is capped at 256 MiB by `MAX_FETCH_BYTES` but is never said to be streamed in bounded chunks, and no chunk or buffer constant is named"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-REPR-001: {verdict: addressed, cite: "data-model.md#Notes"}
  BE-RETRY-001: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-RETRY-002: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-RETRY-003: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-TIMEOUT-001: {verdict: waived, reason: "`ductus exec`'s waits for an `llm-response` or a `gate-response` are on the host that spawned the runtime and holds both of its pipes, and a gate waits on a person, so no bound fits either wait; the walk ends when the host closes stdin, which `read_envelope` reports as the end of input (owner decision, 2026-11-19)", decided-by: "andy@stone.dev", decided-at: "2026-11-19T17:20:03Z"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#Compare before persist"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Compare before persist"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#Release workflow"}
  CFG-ENV-006: {verdict: addressed, cite: "plan.md#Dependency selection"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-009: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-010: {verdict: addressed, cite: "plan.md#Versioning: built-in from `Cargo.toml`"}
  CFG-ENV-012: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "data-model.md#`writeCode`"}
  QUAL-DELEG-001: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  QUAL-STUB-001: {verdict: addressed, cite: "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md#Behavior"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#`runtime.yml` CI workflow"}
  SEC-LOG-001: {verdict: addressed, cite: "plan.md#Dependency selection"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 11
  undispositioned: 0
decisions:
  - key: grounding — spec.md says a single-feature /ductus:analyze takes 2-4 minutes of LLM wall-clock time today
    outcome: waived
    reason: a motivation measured when the spec was written, describing the problem the runtime was built to solve rather than current behavior
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says 11 of 13 analyze check sections are mechanical
    outcome: waived
    reason: a motivation measured when the spec was written; analyze's sections have since been restructured, and the claim records the case for the runtime, not current behavior
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md and plan.md call rmcp the reference MCP SDK
    outcome: waived
    reason: the rationale for a 2026-05 dependency choice, recorded in a Resolved Question and the plan's dependency list
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says certain markdown extensions are off by default in pulldown-cmark
    outcome: waived
    reason: rationale for a rejected alternative in Trade-offs; no current behavior rests on it
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says tokio is required by rmcp
    outcome: waived
    reason: rationale for a dependency choice, the same class as the stored rmcp discard
    decided-at: 2026-10-15T16:54:50Z
    decided-by: andy@stone.dev
  - key: scenario-consistency — scenario done-when-authoring-forms.md has no task in tasks.md and never had one in its history
    outcome: waived
    reason: "written after the fact for behavior shipped in the same commit (959d897d, fix(022): recognize all done-when authoring forms in read-tasks); implemented and tested, read_tasks.rs recognizes_all_done_when_authoring_forms (operator decision, 2026-10-16)"
    decided-at: 2026-10-16T02:59:11Z
    decided-by: andy@stone.dev
  - key: scenario-consistency — scenario scenario-question-parser-fix.md has no task in tasks.md and never had one in its history
    outcome: waived
    reason: written after the fact to record shipped scenario-open-question work (f43818d1); implemented and tested, read_spec.rs parse_open_questions with commented_out_example_questions_are_not_counted, fenced_questions_are_not_counted and every_placeholder_is_skipped_even_as_a_bullet (operator decision, 2026-10-16)
    decided-at: 2026-10-16T02:59:11Z
    decided-by: andy@stone.dev
  - key: artifact-size — `specs/022-deterministic-runtime/plan.md` is 102424 bytes, 3 read pages at the 50000-byte read size
    outcome: waived
    reason: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did
    decided-at: 2026-11-17T19:34:20Z
    decided-by: rule
  - key: artifact-size — `specs/022-deterministic-runtime/data-model.md` is 62552 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did
    decided-at: 2026-11-17T19:34:20Z
    decided-by: rule
  - key: grounding — Technical Decisions, Parser strategy, says pulldown-cmark 'handles the surface lexing (code spans, fenced blocks, lists, headings, HTML blocks) correctly' (and Dependency selection says it was chosen 'for correctness on edge cases'), with no source; only the Trade-offs entry hedges the library's reputation as assumed.
    outcome: waived
    reason: the rationale for the 2026-05 parser dependency choice, recorded in Parser strategy and hedged as assumed in Trade-offs; the same class as the stored rmcp and pulldown-cmark-defaults discards
    decided-at: 2026-11-19T17:30:43Z
    decided-by: rule
  - key: "rule-assessment — BE-PAYLOAD-004: the plan's download is capped at 256 MiB by `MAX_FETCH_BYTES` but is never said to be streamed in bounded chunks, and no chunk or buffer constant is named"
    outcome: waived
    reason: "SHOULD tier and advisory: the rule guards per-request memory in a service holding many bodies at once, while the runtime is one local process fetching one release archive per bootstrap run, and MAX_FETCH_BYTES caps the one body it holds"
    decided-at: 2026-11-19T17:30:43Z
    decided-by: rule
---

# Analysis — 022-deterministic-runtime

## Summary

0 hard-fail, 0 blocking, 11 advisory; not blocking. 85 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 11 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- scenario-consistency — scenario done-when-authoring-forms.md has no task in tasks.md and never had one in its history; append the task that implements it — `specs/022-deterministic-runtime/scenarios/done-when-authoring-forms.md` — **waived**: written after the fact for behavior shipped in the same commit (959d897d, fix(022): recognize all done-when authoring forms in read-tasks); implemented and tested, read_tasks.rs recognizes_all_done_when_authoring_forms (operator decision, 2026-10-16)
- scenario-consistency — scenario scenario-question-parser-fix.md has no task in tasks.md and never had one in its history; append the task that implements it — `specs/022-deterministic-runtime/scenarios/scenario-question-parser-fix.md` — **waived**: written after the fact to record shipped scenario-open-question work (f43818d1); implemented and tested, read_spec.rs parse_open_questions with commented_out_example_questions_are_not_counted, fenced_questions_are_not_counted and every_placeholder_is_skipped_even_as_a_bullet (operator decision, 2026-10-16)
- artifact-size — `specs/022-deterministic-runtime/plan.md` is 108463 bytes, 3 read pages at the 50000-byte read size — `specs/022-deterministic-runtime/plan.md` — **waived**: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did (decided by the rule)
- artifact-size — `specs/022-deterministic-runtime/data-model.md` is 63230 bytes, 2 read pages at the 50000-byte read size — `specs/022-deterministic-runtime/data-model.md` — **waived**: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did (decided by the rule)
- grounding — Motivation states as fact that a single-feature /ductus:analyze takes 2-4 minutes of LLM wall-clock time today and that /ductus:implement's per-task gate cycle is the same order, with no measurement, command or assumption marker in that section (the 'unmeasured here' hedge sits only in Surfaces > Subprocess interpreter). — `specs/022-deterministic-runtime/spec.md` — **waived**: a motivation measured when the spec was written, describing the problem the runtime was built to solve rather than current behavior
- grounding — Motivation asserts '11 of 13 check sections are mechanical' for /ductus:analyze, an enumeration about the existing command with no citation to the sections counted and no marker that it is an estimate. — `specs/022-deterministic-runtime/spec.md` — **waived**: a motivation measured when the spec was written; analyze's sections have since been restructured, and the claim records the case for the runtime, not current behavior
- grounding — Resolved Questions, Implementation language, states as fact that Anthropic's rmcp crate is 'the most mature MCP SDK and is the reference implementation', an external-system claim with no source or assumption marker (points 2 and 5 of the same entry are hedged or comparative; this one is not). — `specs/022-deterministic-runtime/spec.md` — **waived**: the rationale for a 2026-05 dependency choice, recorded in a Resolved Question and the plan's dependency list
- grounding — Technical Decisions, Dependency selection, says tokio is 'required by rmcp' as fact, a claim about an external crate's requirements with no citation to its manifest or docs. — `specs/022-deterministic-runtime/plan.md` — **waived**: rationale for a dependency choice, the same class as the stored rmcp discard
- grounding — Technical Decisions, Parser strategy, says pulldown-cmark 'handles the surface lexing (code spans, fenced blocks, lists, headings, HTML blocks) correctly' (and Dependency selection says it was chosen 'for correctness on edge cases'), with no source; only the Trade-offs entry hedges the library's reputation as assumed. — `specs/022-deterministic-runtime/plan.md` — **waived**: the rationale for the 2026-05 parser dependency choice, recorded in Parser strategy and hedged as assumed in Trade-offs; the same class as the stored rmcp and pulldown-cmark-defaults discards (decided by the rule)
- grounding — Known limitations, pulldown-cmark edge cases, states that tables, footnotes and definition lists are off by default and that the procedure parser opts into tables only, a claim about the library's defaults and the parser's options with no file-and-symbol or documentation citation. — `specs/022-deterministic-runtime/plan.md` — **waived**: rationale for a rejected alternative in Trade-offs; no current behavior rests on it
- rule-assessment — BE-PAYLOAD-004: the plan's download is capped at 256 MiB by `MAX_FETCH_BYTES` but is never said to be streamed in bounded chunks, and no chunk or buffer constant is named — `specs/022-deterministic-runtime/plan.md` — **waived**: SHOULD tier and advisory: the rule guards per-request memory in a service holding many bodies at once, while the runtime is one local process fetching one release archive per bootstrap run, and MAX_FETCH_BYTES caps the one body it holds (decided by the rule)

## Deferred to other specs

*None.*

## Waived rules

- BE-TIMEOUT-001 (MUST) — `ductus exec`'s waits for an `llm-response` or a `gate-response` are on the host that spawned the runtime and holds both of its pipes, and a gate waits on a person, so no bound fits either wait; the walk ends when the host closes stdin, which `read_envelope` reports as the end of input (owner decision, 2026-11-19) — decided by `andy@stone.dev` at `2026-11-19T17:20:03Z`

## Contradictions

*None.*

## Unexamined targets

- never-existed: 13
- not-a-live-claim: 12
- root-absent: 54
- ships-to-adopter: 6

## Fixed in this run

*None.*
