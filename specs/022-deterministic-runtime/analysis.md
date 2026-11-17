---
spec: 022-deterministic-runtime
last-run: 2026-11-17T20:06:33Z
analyzed-against: 991f1f9a265becb1cccd2ff8ea1b32258d15a192
hard-fail: 0
blocking-findings: 0
advisory: 19
unexamined: 67
analyzed-digest:
  data-model.md: 7ec795f721c05faeb74e015f56730c9ef5fcebe81d6bf5f1641e621b4ec176b8
  plan.md: 65ad6b42485e27485ddfc74213f42397f074ade133387c5bfdfef4219ba62d9f
  review.md: d77fe7d046ba64b6cff538325beac8ca6260026ff457d74e517377d4b6afa9b9
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
  spec.md: 9ff2245166d6c1a0630b8424b10ca93bcc36490f237206338ad23d55148334ee
  tasks.md: 3554845d81a365bbe9249c9b873e569168c0c451fd332e864170d68aefaceb62
unexamined-by-reason:
  never-existed: 15
  not-a-live-claim: 5
  root-absent: 42
  ships-to-adopter: 5
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
  BE-ASYNC-001: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-AUTHZ-004: {verdict: addressed, cite: "plan.md#Agent reach: what ductus installs grants only what its steps need"}
  BE-AUTHZ-007: {verdict: addressed, cite: "data-model.md#Primitive request/response schemas"}
  BE-COMPAT-001: {verdict: addressed, cite: "data-model.md#Versioning of these schemas"}
  BE-COND-001: {verdict: addressed, cite: "data-model.md#`set-status` — update spec frontmatter status field"}
  BE-COORD-002: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-DATA-001: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-DEPS-001: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-002: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-003: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-DEPS-004: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-005: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-007: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#Outbound (runtime → host) messages"}
  BE-ERR-004: {verdict: addressed, cite: "data-model.md#Outbound (runtime → host) messages"}
  BE-ERR-005: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Extension-point schemas (initial release)"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Severity tiers — three closed sets, not one"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  BE-INPUT-007: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#Dependency selection"}
  BE-INPUT-011: {verdict: addressed, cite: "plan.md#Dependency selection"}
  BE-INPUT-016: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-LOCK-002: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-LOG-002: {verdict: addressed, cite: "plan.md#Dependency selection"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-PAYLOAD-002: {verdict: addressed, cite: "data-model.md#`read-spec` — parse spec frontmatter and body sections"}
  BE-PAYLOAD-004: {verdict: finding, tier: should, at: "plan.md#`fetch-archive` transport", message: "fetch-archive reads a body up to MAX_FETCH_BYTES (256 MiB) whole into memory, and the plan states its size cap and deadline but neither a streamed body nor a named chunk size"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-REPR-001: {verdict: addressed, cite: "data-model.md#Notes"}
  BE-REPR-002: {verdict: addressed, cite: "spec.md#Resolved Questions"}
  BE-RETRY-001: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-RETRY-003: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#Project layout: single Rust crate at `runtime/`"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-TIMEOUT-002: {verdict: addressed, cite: "plan.md#`runtime.yml` CI workflow"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#Compare before persist"}
  CFG-CONST-001: {verdict: addressed, cite: "data-model.md#Per-project file resolution"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#Trade-offs"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-003: {verdict: addressed, cite: "scenarios/the-constitutions-registry-validates-its-values.md#Behavior"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#Release workflow"}
  CFG-ENV-006: {verdict: addressed, cite: "plan.md#Dependency selection"}
  CFG-ENV-007: {verdict: addressed, cite: "data-model.md#Per-project file resolution"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-009: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-012: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#CLI surface shape"}
  QUAL-DELEG-001: {verdict: addressed, cite: "scenarios/numbered-heading-grammar-single-source.md#Edge Cases"}
  QUAL-GROUND-001: {verdict: addressed, cite: "plan.md#`framework/runtime-tools.txt` is the manifest"}
  QUAL-STUB-001: {verdict: addressed, cite: "scenarios/review-exec-wiring.md#Behavior"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#`runtime.yml` CI workflow"}
  SEC-LOG-001: {verdict: addressed, cite: "plan.md#Dependency selection"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 19
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
  - key: grounding — plan.md says POSIX rename is atomic and Windows rename is weaker
    outcome: waived
    reason: a property of the platforms the atomic-write design rests on, stated as rationale; the CI matrix exercises the write on all three
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says certain markdown extensions are off by default in pulldown-cmark
    outcome: waived
    reason: rationale for a rejected alternative in Trade-offs; no current behavior rests on it
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says no primitive but fetch-archive makes an HTTP request of its own
    outcome: waived
    reason: a scope statement in the Trade-offs entry recording the reqwest dependency's adoption; runtime/Cargo.toml is cited, and the plan's fetch-archive transport section names the path
    decided-at: 2026-10-15T16:54:50Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says tokio is required by rmcp
    outcome: waived
    reason: rationale for a dependency choice, the same class as the stored rmcp discard
    decided-at: 2026-10-15T16:54:50Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says pulldown-cmark provides byte offsets
    outcome: waived
    reason: rationale for the parser choice, the same class as the stored pulldown-cmark discard
    decided-at: 2026-10-15T16:54:50Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md and plan.md say analyze runs gen-help-tables through run-generator and the pre-commit hook calls the generators directly
    outcome: waived
    reason: describes the wiring the Bash script relationships section exists to record, naming the command and the hook it describes
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
  - key: grounding — spec.md's Implementation language answer says Rust gives the smallest static binary and the fastest cold start and Go builds faster, uncited
    outcome: waived
    reason: rationale for the 2026-05 implementation-language decision, recorded in a Resolved Question, the same class as the stored rmcp waiver
    decided-at: 2026-11-17T19:34:20Z
    decided-by: rule
  - key: grounding — plan.md's Dependency selection says serde_norway reads a YAML tag as at most an enum variant or a Value::Tagged node and runs no code, uncited
    outcome: waived
    reason: rationale for a dependency choice, the same class as the stored tokio and pulldown-cmark waivers
    decided-at: 2026-11-17T19:34:20Z
    decided-by: rule
  - key: grounding — plan.md's runtime.yml section says actions/setup-node downloaded Node from a branch-read manifest with no checksum check, uncited
    outcome: waived
    reason: the observation that motivated pinning that install, stated as the pin's rationale, the same class as the stored POSIX-rename waiver
    decided-at: 2026-11-17T19:34:20Z
    decided-by: rule
  - key: grounding — plan.md's Agent reach says APFS finds a name under its case-converted and its composed and decomposed Unicode spellings, uncited
    outcome: waived
    reason: a property of the platform the name comparison rests on, stated as rationale, the same class as the stored POSIX-rename waiver; fold in runtime/src/primitives/host_config.rs and its tests hold the comparison
    decided-at: 2026-11-17T19:34:20Z
    decided-by: rule
  - key: grounding — plan.md's npm advisories bullet says the lint tooling globs only patterns this repository writes, uncited
    outcome: waived
    reason: the rationale for the listed exception, which scripts/npm-audit-allowed.json records as the entry's own reason, naming the globs and the staged paths it rests on; lint-markdown's parenthesis refusal, which the same sentence names, is the code
    decided-at: 2026-11-17T19:34:20Z
    decided-by: rule
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
  - key: "rule-assessment — BE-PAYLOAD-004: fetch-archive reads a body up to MAX_FETCH_BYTES (256 MiB) whole into memory, and the plan states its size cap and deadline but neither a streamed body nor a named chunk size"
    outcome: waived
    reason: the rule guards per-request memory in a service holding many bodies at once; the runtime is one local process fetching one release archive (about 6 MB) per bootstrap run, and MAX_FETCH_BYTES caps the one buffer it holds
    decided-at: 2026-11-17T19:34:20Z
    decided-by: rule
---

# Analysis — 022-deterministic-runtime

## Summary

0 hard-fail, 0 blocking, 19 advisory; not blocking. 67 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 19 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md says a single-feature /ductus:analyze takes 2-4 minutes of LLM wall-clock time today — `specs/022-deterministic-runtime/spec.md` — **waived**: a motivation measured when the spec was written, describing the problem the runtime was built to solve rather than current behavior
- grounding — spec.md says 11 of 13 analyze check sections are mechanical — `specs/022-deterministic-runtime/spec.md` — **waived**: a motivation measured when the spec was written; analyze's sections have since been restructured, and the claim records the case for the runtime, not current behavior
- grounding — spec.md and plan.md call rmcp the reference MCP SDK — `specs/022-deterministic-runtime/spec.md` — **waived**: the rationale for a 2026-05 dependency choice, recorded in a Resolved Question and the plan's dependency list
- grounding — plan.md says POSIX rename is atomic and Windows rename is weaker — `specs/022-deterministic-runtime/plan.md` — **waived**: a property of the platforms the atomic-write design rests on, stated as rationale; the CI matrix exercises the write on all three
- grounding — plan.md says certain markdown extensions are off by default in pulldown-cmark — `specs/022-deterministic-runtime/plan.md` — **waived**: rationale for a rejected alternative in Trade-offs; no current behavior rests on it
- grounding — plan.md says no primitive but fetch-archive makes an HTTP request of its own — `specs/022-deterministic-runtime/plan.md` — **waived**: a scope statement in the Trade-offs entry recording the reqwest dependency's adoption; runtime/Cargo.toml is cited, and the plan's fetch-archive transport section names the path
- grounding — plan.md says tokio is required by rmcp — `specs/022-deterministic-runtime/plan.md` — **waived**: rationale for a dependency choice, the same class as the stored rmcp discard
- grounding — plan.md says pulldown-cmark provides byte offsets — `specs/022-deterministic-runtime/plan.md` — **waived**: rationale for the parser choice, the same class as the stored pulldown-cmark discard
- grounding — spec.md and plan.md say analyze runs gen-help-tables through run-generator and the pre-commit hook calls the generators directly — `specs/022-deterministic-runtime/spec.md` — **waived**: describes the wiring the Bash script relationships section exists to record, naming the command and the hook it describes
- scenario-consistency — scenario done-when-authoring-forms.md has no task in tasks.md and never had one in its history; append the task that implements it — `specs/022-deterministic-runtime/scenarios/done-when-authoring-forms.md` — **waived**: written after the fact for behavior shipped in the same commit (959d897d, fix(022): recognize all done-when authoring forms in read-tasks); implemented and tested, read_tasks.rs recognizes_all_done_when_authoring_forms (operator decision, 2026-10-16)
- scenario-consistency — scenario scenario-question-parser-fix.md has no task in tasks.md and never had one in its history; append the task that implements it — `specs/022-deterministic-runtime/scenarios/scenario-question-parser-fix.md` — **waived**: written after the fact to record shipped scenario-open-question work (f43818d1); implemented and tested, read_spec.rs parse_open_questions with commented_out_example_questions_are_not_counted, fenced_questions_are_not_counted and every_placeholder_is_skipped_even_as_a_bullet (operator decision, 2026-10-16)
- grounding — spec.md's Implementation language answer says Rust gives the smallest static binary and the fastest cold start and Go builds faster, uncited — `specs/022-deterministic-runtime/spec.md` — **waived**: rationale for the 2026-05 implementation-language decision, recorded in a Resolved Question, the same class as the stored rmcp waiver (decided by the rule)
- grounding — plan.md's Dependency selection says serde_norway reads a YAML tag as at most an enum variant or a Value::Tagged node and runs no code, uncited — `specs/022-deterministic-runtime/plan.md` — **waived**: rationale for a dependency choice, the same class as the stored tokio and pulldown-cmark waivers (decided by the rule)
- grounding — plan.md's runtime.yml section says actions/setup-node downloaded Node from a branch-read manifest with no checksum check, uncited — `specs/022-deterministic-runtime/plan.md` — **waived**: the observation that motivated pinning that install, stated as the pin's rationale, the same class as the stored POSIX-rename waiver (decided by the rule)
- grounding — plan.md's Agent reach says APFS finds a name under its case-converted and its composed and decomposed Unicode spellings, uncited — `specs/022-deterministic-runtime/plan.md` — **waived**: a property of the platform the name comparison rests on, stated as rationale, the same class as the stored POSIX-rename waiver; fold in runtime/src/primitives/host_config.rs and its tests hold the comparison (decided by the rule)
- grounding — plan.md's npm advisories bullet says the lint tooling globs only patterns this repository writes, uncited — `specs/022-deterministic-runtime/plan.md` — **waived**: the rationale for the listed exception, which scripts/npm-audit-allowed.json records as the entry's own reason, naming the globs and the staged paths it rests on; lint-markdown's parenthesis refusal, which the same sentence names, is the code (decided by the rule)
- artifact-size — `specs/022-deterministic-runtime/plan.md` is 102424 bytes, 3 read pages at the 50000-byte read size — `specs/022-deterministic-runtime/plan.md` — **waived**: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did (decided by the rule)
- artifact-size — `specs/022-deterministic-runtime/data-model.md` is 62552 bytes, 2 read pages at the 50000-byte read size — `specs/022-deterministic-runtime/data-model.md` — **waived**: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did (decided by the rule)
- rule-assessment — BE-PAYLOAD-004: fetch-archive reads a body up to MAX_FETCH_BYTES (256 MiB) whole into memory, and the plan states its size cap and deadline but neither a streamed body nor a named chunk size — `specs/022-deterministic-runtime/plan.md` — **waived**: the rule guards per-request memory in a service holding many bodies at once; the runtime is one local process fetching one release archive (about 6 MB) per bootstrap run, and MAX_FETCH_BYTES caps the one buffer it holds (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 15
- not-a-live-claim: 5
- root-absent: 42
- ships-to-adopter: 5

## Fixed in this run

*None.*
