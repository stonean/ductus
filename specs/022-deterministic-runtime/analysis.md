---
spec: 022-deterministic-runtime
last-run: 2026-11-22T22:29:02Z
analyzed-against: 0f24da1dd26b4bad8683f5001990ffb96ee52da6
hard-fail: 0
blocking-findings: 0
advisory: 12
unexamined: 84
analyzed-digest:
  data-model.md: be68f8c9c29eef691e764398310095745827fca32cb0041f347d99234eae2515
  plan.md: 715a87e9002f05ed107aa649f35f451c06d29cc84df073c1cf0f114e7bb58601
  review.md: 97ceb087fc4c4d3584ecaf277dfbae1ee2697d45b358457680de2ad59f36362d
  scenarios/a-closed-stdout-is-a-quiet-exit.md: bdbf9b74de48542168e7cac7bc0f5890eed478fde5957c1bd08738b569354957
  scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md: f3c4072fc69ebfa829d92ae21c7043b7e3aa2a4be3db175b7df56204882e6fb4
  scenarios/a-done-when-clause-reads-to-its-end.md: b2d6b2ab8cbd208eec3ac811edc662c296792d51ec7ab1e0c49ec2c8acb045e9
  scenarios/a-path-argument-resolves-inside-its-base.md: 503e9dd4b66563bb317c4deaf4e9cee88275c3c5e3a71a50628704f208a4ac3d
  scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md: 5fec39041378aa1786a94cf59eb1676dd33b7f15b5371815ac92d1e8382c4c72
  scenarios/append-inbox-comment-aware-write.md: 5df79036e979479b03037efd26f74d513c2ef128e579e33e198e4cf8b885366a
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
  scenarios/constitution-excerpts-as-skill-resources.md: 5a6ae02f06a19969028216a89f7855ca020cbdc983fac62ccbedf95fd7400e2f
  scenarios/coverage-expansion-primitives.md: 8fbe190c64b21c9f0edf92d764bdf9138d4f122830eea60df726d526859578d6
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
  scenarios/govern-bootstrap.md: 5ffb4022ff53b32e9288a4f15fff7cd3441a2cea1bf42f07fbd080c264529089
  scenarios/groom-command-acceleration.md: dd6052b3d7ee1b1f8528913b82eabfbaf6debe4de51043824b2fcd5d040ed27e
  scenarios/host-protocol-conformance.md: 6e17ebe8c14b713320a249729de266c2126161ba61ba21416b558628bb13bf0d
  scenarios/implement-completion-gate.md: d094ebf57e1f453690a6dc01fd28282df377b3c4362daf90f95f76183d635292
  scenarios/installed-files-are-not-group-writable.md: ca14be0b2cca81724fdb32c2757e67374c375b542eacf7d86baf175da2c30cbc
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
  scenarios/review-runtime-acceleration.md: a9bd9f499b6aff34fd768a1dc91bb782b08432396643ed2c478b3fae8505d4a0
  scenarios/review-scope-parse-fidelity.md: f970155028f105cc7441f1597113df13592fc0e0b0fceee80b60787e420ef2ac
  scenarios/runtime-primitive-structural-bugs.md: 7fb973b4c5ef5c0268df24996c585f9bf70465f4f7e36ad237d129894f5fd366
  scenarios/runtime-review-019-hardening.md: dd72b20cb886ee804141895d169ebb90eb5c548df260fb02cb1643dd2a6f777a
  scenarios/scaffolding-primitives.md: c3495b99e2f440822fed57cb08ba0cd668e9093fd786ad37e616a8508417c699
  scenarios/scenario-open-question-signal.md: 92ec638cae624784afd957acecbd10ecb060ae3a2dcf4a864253c865ab50d6b0
  scenarios/scenario-question-parser-fix.md: bdcaf5f9bb7d0ff795d6e3a6a5045a79afc63b55d8ce4b3331f807c337bf2ada
  scenarios/severity-is-a-closed-set-not-a-string.md: d4ea4d23382e2968d72df879d89c7617ffe0b1a8f4049430a25cf70d5f1bd25f
  scenarios/skipscanner-inline-code-exemption.md: 20cf003798a71e271115f0425b78f4d61c52cd2e56e42311d52ccf3660133348
  scenarios/spec-side-parser-hardening.md: c32c2f69ce6d17a120ea927ceec6b87bf79350b46ae217c78023780778de71fc
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
  spec.md: 7384692858bfd8b7ccc6d56833d842713d6bb9321c0d6ecf0c72715ea784645a
  tasks.md: 4de4f2998f65d298bbe436bf3f730f634332cf2cae457012b590d534297f3018
unexamined-by-reason:
  anchor-target-unjudgeable: 3
  never-existed: 12
  not-a-live-claim: 10
  root-absent: 54
  ships-to-adopter: 5
rule-digest:
  accessibility-frontend.md: 4d7f5ac09d457c99774f5fc341222a464d2b461cb0467cd58a13dc31c928359b
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: f892282383d7377ded2e5fd1ee3e30421d0d6ad3f9bb1fd354ff0d9d114e807a
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#022 — Deterministic Runtime Data Model": 8bc35f991053c7d0
  "data-model.md#A refused write — the ten guarded read-modify-writes": 3b338ffa7ae89cb2
  "data-model.md#Command-file candidates — `Host::command_file_candidates`": 1e69948eb5713926
  "data-model.md#Envelope": 51c069403ccedae6
  "data-model.md#Extension-point schemas (initial release)": 78976ed790f8967e
  "data-model.md#Inbound (host → runtime) messages": 6696a15de1550f64
  "data-model.md#JSON-over-stdio protocol": ccbe6020bb7cc2ab
  "data-model.md#Notes": 6c1743351dbeb285
  "data-model.md#Outbound (runtime → host) messages": 58dc6303d76ba597
  "data-model.md#Per-project file resolution": 6840b672bb6ac93e
  "data-model.md#Primitive request/response schemas": 413cf54dac024302
  "data-model.md#Procedure AST": a4e252f4f5a10cd7
  "data-model.md#Request bodies past the read size": 7e7ab5d8def05714
  "data-model.md#Severity tiers — three closed sets, not one": ff4bc2433ce7d4d4
  "data-model.md#Versioning of these schemas": 0e86b067ee7ae612
  "data-model.md#`append-inbox` — append one bullet to the inbox": cff5cbfd2b050ecf
  "data-model.md#`append-question` — append one bullet to Open Questions": 0e75c5dbc3eca727
  "data-model.md#`askClarifyQuestion` (follow-on)": b7873e9477956cf7
  "data-model.md#`assessSpecQuality`": aef30c4d6af8a503
  "data-model.md#`check-stuck` — count tasks.md commits since `in-progress`": 088eaf5128092341
  "data-model.md#`create-feature` — scaffold the next feature directory": fbae8adaf83e3a4a
  "data-model.md#`create-plan-artifacts` — copy the plan/tasks/data-model templates into a feature directory": a33fb9640d111ced
  "data-model.md#`dashboard` — fields added since (addendum)": 0ae14e064c9003e1
  "data-model.md#`derive-boundary` — compute runtime write boundary": 26bcef370e2ae9ca
  "data-model.md#`derive-routing-candidates` — the homes proposed work could already have": b0730c0e6eacba1a
  "data-model.md#`discover-rule-files` — `notices` bounded (addendum)": ff770155bd7c282e
  "data-model.md#`gate-confirm` — surface a gate to the user through the host": de3ac5e81b1dec8d
  "data-model.md#`label-criteria` — assign stable `AC{n}` labels to acceptance criteria": d68afddc3647f38d
  "data-model.md#`lint-markdown` — wrap `npx markdownlint-cli2`": 27fe236269545fd1
  "data-model.md#`mark-criterion` — flip checkbox state on an acceptance criterion": 25cf57fae65c3477
  "data-model.md#`mark-task` — flip checkbox state on a task": 15ac90356db899c9
  "data-model.md#`performReview`": 2b0f5888a7b1c1b5
  "data-model.md#`read-spec` — parse spec frontmatter and body sections": af4f0f04f5e62892
  "data-model.md#`read-tasks` — parse tasks.md into structured task list": e8910f8d478cc7bb
  "data-model.md#`remove-inbox-item` — remove one bullet from the inbox": 849aec195149ffaa
  "data-model.md#`render-status` — the pipeline view (spec 080)": 20487741dd677a38
  "data-model.md#`resolve-feature` — resolve an identifier to a feature directory": 5d840aa4d23859b0
  "data-model.md#`routeFold` (follow-on)": e8ebd51ed5692d2f
  "data-model.md#`routeInboxItem` (follow-on)": 7829f930118dfd4a
  "data-model.md#`routeInboxItem` — `candidates` request field (addendum)": dc76cfdfb2de10bf
  "data-model.md#`run-generator` — invoke a bash generator in `--dry-run`": f9c03b7b2afd9144
  "data-model.md#`set-status` — update spec frontmatter status field": a4fac6779c319cf3
  "data-model.md#`verifyCriteria` (follow-on)": 8392cdf80358a6d9
  "data-model.md#`writeCode`": 056e7c13687ab099
  "data-model.md#`writeSpecBody`": c33c6d38a65e5cc7
  "plan.md": e3b0c44298fc1c14
  "plan.md#022 — Deterministic Runtime Plan": eaf90dc4c15758bf
  "plan.md#Affected Files": 9b537490896b0a1f
  "plan.md#Agent reach: what ductus installs grants only what its steps need": 9a602f60f168c16f
  "plan.md#Atomic writes: tempfile in same directory, then `std::fs::rename`": 8577c0ab1834554e
  "plan.md#Bootstrap completion-message pointer": c7312a0c4cf81f78
  "plan.md#CLI surface shape": 49a23bbb7e809a90
  "plan.md#Compare before persist": e90400bb41db5d6f
  "plan.md#Dependency pinning, provenance, and upgrades": b8c177c856bf4d5e
  "plan.md#Dependency selection": 12c048e9622289b4
  "plan.md#Error semantics and exit codes": 9a690754dca49028
  "plan.md#Fixture-based integration testing": ad94ac1ddb93d732
  "plan.md#Interpreter: synchronous walk + stdio JSON for asynchronous seams": 3493259fc2e17065
  "plan.md#JSON-over-stdio framing: line-delimited JSON, one message per line": f993c5d13a65c273
  "plan.md#Known limitations": c85bd7cc068fadc3
  "plan.md#MCP surface: one tool per primitive, no orchestration tools": 99a8db54f0dcb11d
  "plan.md#No data persistence outside session file + markdown": 307e5c96972a59a0
  "plan.md#Overview": 26f48b92d53a70d1
  "plan.md#Parseability check: a step in `framework-checks.yml`": 62549c162c539fa1
  "plan.md#Parser strategy: pulldown-cmark events + hand-walked structural recognizer": 43ab1287ca4d2c4c
  "plan.md#Path arguments: a lexical screen, then canonical containment": 66727474f4f98e6c
  "plan.md#Per-command parity testing: golden envelope streams": 3cccb90105b7d0a4
  "plan.md#Project layout: single Rust crate at `runtime/`": eaf04fbc65c41669
  "plan.md#README placement: root `README.md` Runtime section": 6199dc184bbef09e
  "plan.md#Release workflow": 8770fc7a8775c53e
  "plan.md#Slash command frontmatter: no runtime field": c916d17e9ff73933
  "plan.md#Slash command rewrites: order and rewrite shape": 2d6808ad57e3b19f
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#This repository's generator scripts stay bash": 824a0be212340212
  "plan.md#Trade-offs": 2674206fc8b69760
  "plan.md#Versioning: built-in from `Cargo.toml`": 91b7a8ec148418fc
  "plan.md#`fetch-archive` transport": ffe0b183d92ba060
  "plan.md#`framework/runtime-tools.txt` is the manifest": 68ce6f44dea15c85
  "plan.md#`runtime.yml` CI workflow": 07788a5abaa7f88d
  "scenarios/a-closed-stdout-is-a-quiet-exit.md": 6260d2e0860b5319
  "scenarios/a-closed-stdout-is-a-quiet-exit.md#A-closed-stdout-is-a-quiet-exit": 6e4fd7403bf7adfb
  "scenarios/a-closed-stdout-is-a-quiet-exit.md#Behavior": 119565c018064a66
  "scenarios/a-closed-stdout-is-a-quiet-exit.md#Context": 8fb1115d089344ca
  "scenarios/a-closed-stdout-is-a-quiet-exit.md#Edge Cases": 74c1f8a45c5f5ce4
  "scenarios/a-closed-stdout-is-a-quiet-exit.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-closed-stdout-is-a-quiet-exit.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md": 9c2b5be431a4c855
  "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md#A-confirmed-gate-authorizes-the-writes-after-it": b41820ce32d9e5d4
  "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md#Behavior": ff06032d6e59a153
  "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md#Context": cd7c38883827ca0e
  "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md#Edge Cases": 43b0f564f8f50f59
  "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/a-done-when-clause-reads-to-its-end.md": 6260d2e0860b5319
  "scenarios/a-done-when-clause-reads-to-its-end.md#A-done-when-clause-reads-to-its-end": ac3f15d892f6ecc6
  "scenarios/a-done-when-clause-reads-to-its-end.md#Behavior": d42476240b79f30c
  "scenarios/a-done-when-clause-reads-to-its-end.md#Context": 4ba8189f48db99ef
  "scenarios/a-done-when-clause-reads-to-its-end.md#Edge Cases": a69426ddc17479c7
  "scenarios/a-done-when-clause-reads-to-its-end.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-done-when-clause-reads-to-its-end.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/a-path-argument-resolves-inside-its-base.md": f4036d4aa3763d17
  "scenarios/a-path-argument-resolves-inside-its-base.md#A-path-argument-resolves-inside-its-base": b782a055b5d76160
  "scenarios/a-path-argument-resolves-inside-its-base.md#Behavior": 839c37235ef21168
  "scenarios/a-path-argument-resolves-inside-its-base.md#Context": 424261caeff067c2
  "scenarios/a-path-argument-resolves-inside-its-base.md#Edge Cases": e6c987b87a9b4e54
  "scenarios/a-path-argument-resolves-inside-its-base.md#Open Questions": 545eadd502d01a7c
  "scenarios/a-path-argument-resolves-inside-its-base.md#Resolved Questions": b09b5c74dbad4c2d
  "scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md": 6260d2e0860b5319
  "scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md#An-unsubstituted-placeholder-is-not-a-clean-run": 5af5339aaff36ab0
  "scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md#Behavior": af9dbb367325b410
  "scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md#Context": 5366ab259aa567ff
  "scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md#Edge Cases": 3ff90d8fe66b146a
  "scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md#Open Questions": 20edda8881516e4e
  "scenarios/an-unsubstituted-placeholder-is-not-a-clean-run.md#Resolved Questions": 67680ff65c8b2fd3
  "scenarios/append-inbox-comment-aware-write.md": f4036d4aa3763d17
  "scenarios/append-inbox-comment-aware-write.md#Append-inbox-comment-aware-write": 997d34dbe7ce99ae
  "scenarios/append-inbox-comment-aware-write.md#Behavior": c4eec986e9614e37
  "scenarios/append-inbox-comment-aware-write.md#Context": b602ca1bf5da1a38
  "scenarios/append-inbox-comment-aware-write.md#Edge Cases": 54daf06603352d04
  "scenarios/append-inbox-comment-aware-write.md#Open Questions": 9fc5979d590cdb97
  "scenarios/append-inbox-comment-aware-write.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/append-primitive-marker-normalization.md": f4036d4aa3763d17
  "scenarios/append-primitive-marker-normalization.md#Append-primitive-marker-normalization": bf12604562486056
  "scenarios/append-primitive-marker-normalization.md#Behavior": a364276d00bc9ff3
  "scenarios/append-primitive-marker-normalization.md#Context": 282c95fe67ae6920
  "scenarios/append-primitive-marker-normalization.md#Edge Cases": 9da1a490975e215c
  "scenarios/append-primitive-marker-normalization.md#Open Questions": 9fc5979d590cdb97
  "scenarios/append-primitive-marker-normalization.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/append-task-honours-slug-with-explicit-body.md": 6260d2e0860b5319
  "scenarios/append-task-honours-slug-with-explicit-body.md#Append-task-honours-slug-with-explicit-body": 9466c776efb44ba0
  "scenarios/append-task-honours-slug-with-explicit-body.md#Behavior": f06105be449aa279
  "scenarios/append-task-honours-slug-with-explicit-body.md#Context": 38125502b3008030
  "scenarios/append-task-honours-slug-with-explicit-body.md#Edge Cases": 61ea0a7f306f91ed
  "scenarios/append-task-honours-slug-with-explicit-body.md#Open Questions": 9fc5979d590cdb97
  "scenarios/append-task-honours-slug-with-explicit-body.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/apply-manifest-substitution-contract.md": f4036d4aa3763d17
  "scenarios/apply-manifest-substitution-contract.md#Apply-manifest — the substitution-key contract, and a count that was computed and thrown away": d3f2929a04e2b42b
  "scenarios/apply-manifest-substitution-contract.md#Behavior": d45c2ea7ece2cdc2
  "scenarios/apply-manifest-substitution-contract.md#Context": 1570b6b5784c20cf
  "scenarios/apply-manifest-substitution-contract.md#Edge Cases": a5555dd6309e61b9
  "scenarios/apply-manifest-substitution-contract.md#Resolved Questions": 0e9397a0a1265f44
  "scenarios/apply-manifest.md": f4036d4aa3763d17
  "scenarios/apply-manifest.md#Apply-manifest — strategy-aware bulk substitute + write for the bootstrap": cf5d025145bb098e
  "scenarios/apply-manifest.md#Behavior": 377d76de1e79ff8d
  "scenarios/apply-manifest.md#Context": 2edf6358f83ec2a0
  "scenarios/apply-manifest.md#Edge Cases": 7649fbf9f8751251
  "scenarios/apply-manifest.md#Open Questions": 9fc5979d590cdb97
  "scenarios/apply-manifest.md#Resolved Questions": b6d11b8f9b5403fe
  "scenarios/archive-network-hardening.md": f4036d4aa3763d17
  "scenarios/archive-network-hardening.md#Archive-network-hardening": ebc004a0b713252a
  "scenarios/archive-network-hardening.md#Behavior": 177cbcde0b85f2f0
  "scenarios/archive-network-hardening.md#Context": f5963cdac65cb5b8
  "scenarios/archive-network-hardening.md#Edge Cases": a48603ef7cd72c7e
  "scenarios/archive-network-hardening.md#Open Questions": 9fc5979d590cdb97
  "scenarios/archive-network-hardening.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/ask-consolidation.md": f4036d4aa3763d17
  "scenarios/ask-consolidation.md#Ask-consolidation — primitives for `/amend`'s scenario branch": 9e1c689e5c51369c
  "scenarios/ask-consolidation.md#Behavior": 0b851b04e20024fd
  "scenarios/ask-consolidation.md#Context": b8bc2c9b00c28d9f
  "scenarios/ask-consolidation.md#Edge Cases": e971840f9a0324d6
  "scenarios/ask-consolidation.md#Open Questions": 9fc5979d590cdb97
  "scenarios/ask-consolidation.md#Resolved Questions": 0a7ef7543c1efc33
  "scenarios/check-stuck-tasks-md-advancement.md": f4036d4aa3763d17
  "scenarios/check-stuck-tasks-md-advancement.md#Behavior": 07ed1cc99bbade09
  "scenarios/check-stuck-tasks-md-advancement.md#Check-stuck-tasks-md-advancement": dfebbab10bbbabe0
  "scenarios/check-stuck-tasks-md-advancement.md#Context": 0ed513b9bd5ef906
  "scenarios/check-stuck-tasks-md-advancement.md#Edge Cases": a5d3f078fb5d5fe3
  "scenarios/check-stuck-tasks-md-advancement.md#Open Questions": 20edda8881516e4e
  "scenarios/check-stuck-tasks-md-advancement.md#Resolved Questions": e52a37a4b9249d8d
  "scenarios/clarify-command-acceleration.md": f4036d4aa3763d17
  "scenarios/clarify-command-acceleration.md#Behavior": e3f0b05cd14d9b6f
  "scenarios/clarify-command-acceleration.md#Clarify-command-acceleration": d315b446e99040dd
  "scenarios/clarify-command-acceleration.md#Context": 02dfb7356bb8659b
  "scenarios/clarify-command-acceleration.md#Edge Cases": 63d8b38ed28fc627
  "scenarios/clarify-command-acceleration.md#Open Questions": 9fc5979d590cdb97
  "scenarios/clarify-command-acceleration.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/cli-config-dir-per-contributor.md": f4036d4aa3763d17
  "scenarios/cli-config-dir-per-contributor.md#Behavior": e6a17a157f4386cd
  "scenarios/cli-config-dir-per-contributor.md#Cli-config-dir-per-contributor": a4b23208be1f3576
  "scenarios/cli-config-dir-per-contributor.md#Context": 27529616072f4521
  "scenarios/cli-config-dir-per-contributor.md#Edge Cases": 82d48d28a6d96859
  "scenarios/cli-config-dir-per-contributor.md#Open Questions": 9fc5979d590cdb97
  "scenarios/cli-config-dir-per-contributor.md#Resolved Questions": 94cd5195f61d8631
  "scenarios/commands-dir-parameterization.md": f4036d4aa3763d17
  "scenarios/commands-dir-parameterization.md#Behavior": de349596ea2b74bd
  "scenarios/commands-dir-parameterization.md#Commands-dir parameterization": 15944a76ed14a806
  "scenarios/commands-dir-parameterization.md#Context": 8ddb491b02fd1d24
  "scenarios/commands-dir-parameterization.md#Design picks to evaluate": 8849b07989625418
  "scenarios/commands-dir-parameterization.md#Edge Cases": d811f40ba62642b3
  "scenarios/commands-dir-parameterization.md#Implementation shape (assuming Option 1)": 1f1906e9d89b4f68
  "scenarios/commands-dir-parameterization.md#Open Questions": 9d780cc6d46f6c54
  "scenarios/commands-dir-parameterization.md#Resolved Questions": 691e88d50578df7c
  "scenarios/commands-dir-parameterization.md#Scope": 469c8ca3638386ae
  "scenarios/config-readers-name-unknown-keys.md": f4036d4aa3763d17
  "scenarios/config-readers-name-unknown-keys.md#Behavior": 9199c5705dc3e548
  "scenarios/config-readers-name-unknown-keys.md#Config-readers-name-unknown-keys": 54496ee06f349e81
  "scenarios/config-readers-name-unknown-keys.md#Context": 35674617286756a5
  "scenarios/config-readers-name-unknown-keys.md#Edge Cases": bc457eecff08863b
  "scenarios/config-readers-name-unknown-keys.md#Open Questions": 9fc5979d590cdb97
  "scenarios/config-readers-name-unknown-keys.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/config-resolution-single-probe.md": f4036d4aa3763d17
  "scenarios/config-resolution-single-probe.md#Behavior": 527e58ab6e969b86
  "scenarios/config-resolution-single-probe.md#Config-resolution-single-probe": 74862aa7072e5a87
  "scenarios/config-resolution-single-probe.md#Context": ce877813bd385a30
  "scenarios/config-resolution-single-probe.md#Edge Cases": a136d6978152bc12
  "scenarios/config-resolution-single-probe.md#Open Questions": 9fc5979d590cdb97
  "scenarios/config-resolution-single-probe.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/config-sourced-paths-and-the-traversal-boundary.md": f4036d4aa3763d17
  "scenarios/config-sourced-paths-and-the-traversal-boundary.md#Behavior": e5d91cb910acc73b
  "scenarios/config-sourced-paths-and-the-traversal-boundary.md#Config-sourced-paths-and-the-traversal-boundary": 2d184a0323fd1bf3
  "scenarios/config-sourced-paths-and-the-traversal-boundary.md#Context": f2a248b368a6c787
  "scenarios/config-sourced-paths-and-the-traversal-boundary.md#Edge Cases": 7041dca84d0f3743
  "scenarios/config-sourced-paths-and-the-traversal-boundary.md#Open Questions": 9fc5979d590cdb97
  "scenarios/config-sourced-paths-and-the-traversal-boundary.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/constitution-excerpts-as-skill-resources.md": 037db3ce16126339
  "scenarios/constitution-excerpts-as-skill-resources.md#Behavior": 1a37b3e10739dd21
  "scenarios/constitution-excerpts-as-skill-resources.md#Constitution-excerpts-as-skill-resources": 3f241bfdc4d39319
  "scenarios/constitution-excerpts-as-skill-resources.md#Context": 182c103fd9e34334
  "scenarios/constitution-excerpts-as-skill-resources.md#Edge Cases": 41e1b980bc0eb35c
  "scenarios/constitution-excerpts-as-skill-resources.md#Open Questions": 20edda8881516e4e
  "scenarios/constitution-excerpts-as-skill-resources.md#Resolved Questions": b7367946e3300a88
  "scenarios/coverage-expansion-primitives.md": f4036d4aa3763d17
  "scenarios/coverage-expansion-primitives.md#Behavior": 18121fd838a2f29d
  "scenarios/coverage-expansion-primitives.md#Context": e56fc5b45fab65a9
  "scenarios/coverage-expansion-primitives.md#Coverage-expansion-primitives": 0666b59cd53e64aa
  "scenarios/coverage-expansion-primitives.md#Edge Cases": 4db8ecc498afa2df
  "scenarios/coverage-expansion-primitives.md#Open Questions": 20edda8881516e4e
  "scenarios/coverage-expansion-primitives.md#Resolved Questions": 4556512fc84cb801
  "scenarios/coverage-residue-cleanup.md": f4036d4aa3763d17
  "scenarios/coverage-residue-cleanup.md#Behavior": 7abfeed06a2b542e
  "scenarios/coverage-residue-cleanup.md#Context": c57385ea840f9be4
  "scenarios/coverage-residue-cleanup.md#Coverage-residue-cleanup": 754506991d9a472e
  "scenarios/coverage-residue-cleanup.md#Edge Cases": 42db7fb0764da9cf
  "scenarios/coverage-residue-cleanup.md#Open Questions": 20edda8881516e4e
  "scenarios/coverage-residue-cleanup.md#Resolved Questions": c1fcf521dad9df66
  "scenarios/criterion-label-assignment.md": f4036d4aa3763d17
  "scenarios/criterion-label-assignment.md#Behavior": 29ab98cdbf8ff7c1
  "scenarios/criterion-label-assignment.md#Context": b1a9c29c00f4555d
  "scenarios/criterion-label-assignment.md#Criterion-label-assignment": 61c2b2f85f51c3e0
  "scenarios/criterion-label-assignment.md#Edge Cases": 506cc382a864fd69
  "scenarios/criterion-label-assignment.md#Label-aware addressing": 2f2ab875389f327a
  "scenarios/criterion-label-assignment.md#Open Questions": 9fc5979d590cdb97
  "scenarios/criterion-label-assignment.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/criterion-label-assignment.md#The audit invariant": 22cb5fb6df781b25
  "scenarios/criterion-label-assignment.md#The labelling pass": 3fd0e09de970f100
  "scenarios/dashboard-primitive.md": f4036d4aa3763d17
  "scenarios/dashboard-primitive.md#Behavior": ed14287e7667ad46
  "scenarios/dashboard-primitive.md#Context": e6a56a50cd03a1a0
  "scenarios/dashboard-primitive.md#Dashboard-primitive": 6b05af106008cb1c
  "scenarios/dashboard-primitive.md#Edge Cases": beb037ffaca5928d
  "scenarios/dashboard-primitive.md#Open Questions": 20edda8881516e4e
  "scenarios/dashboard-primitive.md#Resolved Questions": 8c7414542b4995f0
  "scenarios/derive-boundary-uncommitted-spec-dir.md": f4036d4aa3763d17
  "scenarios/derive-boundary-uncommitted-spec-dir.md#Behavior": d0bf1a65a50d4803
  "scenarios/derive-boundary-uncommitted-spec-dir.md#Context": fed640615bb1172e
  "scenarios/derive-boundary-uncommitted-spec-dir.md#Derive-boundary-uncommitted-spec-dir": 98e25785e93164ff
  "scenarios/derive-boundary-uncommitted-spec-dir.md#Edge Cases": bce41114f2955963
  "scenarios/derive-boundary-uncommitted-spec-dir.md#Open Questions": 9fc5979d590cdb97
  "scenarios/derive-boundary-uncommitted-spec-dir.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/done-when-authoring-forms.md": f4036d4aa3763d17
  "scenarios/done-when-authoring-forms.md#Behavior": cc1c9a302bc181d7
  "scenarios/done-when-authoring-forms.md#Context": a651ee267034aa72
  "scenarios/done-when-authoring-forms.md#Done-when-authoring-forms": 4bb6a1e91b1adbed
  "scenarios/done-when-authoring-forms.md#Edge Cases": dcb9ce871a4473a3
  "scenarios/done-when-authoring-forms.md#Open Questions": 9fc5979d590cdb97
  "scenarios/done-when-authoring-forms.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/every-mcp-argument-has-a-cli-form.md": 6260d2e0860b5319
  "scenarios/every-mcp-argument-has-a-cli-form.md#Behavior": b82b9a159d873b24
  "scenarios/every-mcp-argument-has-a-cli-form.md#Context": cdf262127b72246d
  "scenarios/every-mcp-argument-has-a-cli-form.md#Edge Cases": a24696c37e42d133
  "scenarios/every-mcp-argument-has-a-cli-form.md#Every-mcp-argument-has-a-cli-form": deb29e66ffdf0da1
  "scenarios/every-mcp-argument-has-a-cli-form.md#Open Questions": 20edda8881516e4e
  "scenarios/every-mcp-argument-has-a-cli-form.md#Resolved Questions": 7a527e5701ee55c7
  "scenarios/exec-analyze-derives-its-list-seeds.md": f4036d4aa3763d17
  "scenarios/exec-analyze-derives-its-list-seeds.md#Behavior": aba1050461fc1851
  "scenarios/exec-analyze-derives-its-list-seeds.md#Context": 4bbcf43d70abac18
  "scenarios/exec-analyze-derives-its-list-seeds.md#Edge Cases": 77321a3cd2e4ef8e
  "scenarios/exec-analyze-derives-its-list-seeds.md#Exec-analyze-derives-its-list-seeds": d158a04db71ae960
  "scenarios/exec-analyze-derives-its-list-seeds.md#Open Questions": 9fc5979d590cdb97
  "scenarios/exec-analyze-derives-its-list-seeds.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/exec-clarify-asks-each-open-question.md": 037db3ce16126339
  "scenarios/exec-clarify-asks-each-open-question.md#Behavior": a644aa9c2d3b2363
  "scenarios/exec-clarify-asks-each-open-question.md#Context": af24058954269783
  "scenarios/exec-clarify-asks-each-open-question.md#Edge Cases": 3875ffa481c3342a
  "scenarios/exec-clarify-asks-each-open-question.md#Exec-clarify-asks-each-open-question": 2a41e2359f7f2e5f
  "scenarios/exec-clarify-asks-each-open-question.md#Open Questions": 9fc5979d590cdb97
  "scenarios/exec-clarify-asks-each-open-question.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/extension-request-hygiene.md": f4036d4aa3763d17
  "scenarios/extension-request-hygiene.md#Behavior": 5ee28a61dc0ebacd
  "scenarios/extension-request-hygiene.md#Context": 34c0fc2d61afe636
  "scenarios/extension-request-hygiene.md#Edge Cases": 565da531b782015f
  "scenarios/extension-request-hygiene.md#Extension-request-hygiene": 06d52e1597ea33ed
  "scenarios/extension-request-hygiene.md#Open Questions": 9fc5979d590cdb97
  "scenarios/extension-request-hygiene.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/fetch-archive-bounds-the-whole-download.md": f4036d4aa3763d17
  "scenarios/fetch-archive-bounds-the-whole-download.md#Behavior": 3074143d8ec91495
  "scenarios/fetch-archive-bounds-the-whole-download.md#Context": 698cadb9d49fdca2
  "scenarios/fetch-archive-bounds-the-whole-download.md#Edge Cases": a8ae7006b2fd436e
  "scenarios/fetch-archive-bounds-the-whole-download.md#Fetch-archive-bounds-the-whole-download": 46587eba2f5ccf5c
  "scenarios/fetch-archive-bounds-the-whole-download.md#Open Questions": 9fc5979d590cdb97
  "scenarios/fetch-archive-bounds-the-whole-download.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/fetch-archive-dns-rebinding.md": f4036d4aa3763d17
  "scenarios/fetch-archive-dns-rebinding.md#Behavior": e78b007cdd034182
  "scenarios/fetch-archive-dns-rebinding.md#Context": a742483968b0beb2
  "scenarios/fetch-archive-dns-rebinding.md#Edge Cases": 9bddc59af2483bf5
  "scenarios/fetch-archive-dns-rebinding.md#Fetch-archive-dns-rebinding": 141e86b3489c8e4e
  "scenarios/fetch-archive-dns-rebinding.md#Open Questions": 9fc5979d590cdb97
  "scenarios/fetch-archive-dns-rebinding.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/fetch-archive-names-its-timeout.md": f4036d4aa3763d17
  "scenarios/fetch-archive-names-its-timeout.md#Behavior": d88997713dc12c87
  "scenarios/fetch-archive-names-its-timeout.md#Context": bfee0bc9061d690e
  "scenarios/fetch-archive-names-its-timeout.md#Edge Cases": 42d9d7f6c11e95a2
  "scenarios/fetch-archive-names-its-timeout.md#Fetch-archive-names-its-timeout": 3e16bbcfd2f7b904
  "scenarios/fetch-archive-names-its-timeout.md#Open Questions": 9fc5979d590cdb97
  "scenarios/fetch-archive-names-its-timeout.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/framework-list-dedup.md": f4036d4aa3763d17
  "scenarios/framework-list-dedup.md#Behavior": 29ab98cdbf8ff7c1
  "scenarios/framework-list-dedup.md#Context": dee92d617ad90b8e
  "scenarios/framework-list-dedup.md#Edge Cases": 73db40dd21de8b24
  "scenarios/framework-list-dedup.md#Framework-list-dedup": 6ace62ec6b91b936
  "scenarios/framework-list-dedup.md#Open Questions": 9fc5979d590cdb97
  "scenarios/framework-list-dedup.md#Part A — `merge-permissions` (new primitive, JSON surface)": 5d79aed37b4cceb6
  "scenarios/framework-list-dedup.md#Part B — `merge-managed-block` cross-boundary dedup (existing primitive, line-prefix style)": 278ca0b89989a1cc
  "scenarios/framework-list-dedup.md#Resolved Questions": 7e3d3bb284ce16bc
  "scenarios/govern-bootstrap.md": f4036d4aa3763d17
  "scenarios/govern-bootstrap.md#Behavior": 84627372a63f69d9
  "scenarios/govern-bootstrap.md#Context": 9ed5d9dfd2f43d21
  "scenarios/govern-bootstrap.md#Ductus bootstrap": 81a306507fdec762
  "scenarios/govern-bootstrap.md#Edge Cases": a99720e4f1dea32f
  "scenarios/govern-bootstrap.md#Open Questions": 9fc5979d590cdb97
  "scenarios/govern-bootstrap.md#Resolved Questions": de223b10b9c179f9
  "scenarios/groom-command-acceleration.md": f4036d4aa3763d17
  "scenarios/groom-command-acceleration.md#Behavior": 119a992c5fde442b
  "scenarios/groom-command-acceleration.md#Context": b30e3627aa3dfbec
  "scenarios/groom-command-acceleration.md#Edge Cases": d547269465a1f078
  "scenarios/groom-command-acceleration.md#Groom-command-acceleration": a5a3f2e61b8ef398
  "scenarios/groom-command-acceleration.md#Open Questions": 9fc5979d590cdb97
  "scenarios/groom-command-acceleration.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/host-protocol-conformance.md": f4036d4aa3763d17
  "scenarios/host-protocol-conformance.md#Behavior": efe117e5c6ee35fa
  "scenarios/host-protocol-conformance.md#Context": d5420c3ceffdfa81
  "scenarios/host-protocol-conformance.md#Edge Cases": eaa29a859f3af9b5
  "scenarios/host-protocol-conformance.md#Host-protocol-conformance": 475273f7d5e5d3d3
  "scenarios/host-protocol-conformance.md#Open Questions": 9fc5979d590cdb97
  "scenarios/host-protocol-conformance.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/implement-completion-gate.md": f4036d4aa3763d17
  "scenarios/implement-completion-gate.md#Behavior": 017b76edece8524a
  "scenarios/implement-completion-gate.md#Context": bb47350b11c5f09a
  "scenarios/implement-completion-gate.md#Edge Cases": 88e015607f3bf5ca
  "scenarios/implement-completion-gate.md#Implement-completion-gate": 61ca97eacb61a88a
  "scenarios/implement-completion-gate.md#Open Questions": 9fc5979d590cdb97
  "scenarios/implement-completion-gate.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/installed-files-are-not-group-writable.md": f4036d4aa3763d17
  "scenarios/installed-files-are-not-group-writable.md#Behavior": e443599244097ab9
  "scenarios/installed-files-are-not-group-writable.md#Context": 5c4ea08989bf90be
  "scenarios/installed-files-are-not-group-writable.md#Edge Cases": 5be5aa4d6f91724b
  "scenarios/installed-files-are-not-group-writable.md#Installed-files-are-not-group-writable": b8cd56b0a97d4d90
  "scenarios/installed-files-are-not-group-writable.md#Open Questions": 9fc5979d590cdb97
  "scenarios/installed-files-are-not-group-writable.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/lint-markdown-lints-only-what-it-is-given.md": 6260d2e0860b5319
  "scenarios/lint-markdown-lints-only-what-it-is-given.md#Behavior": a4a06aced7357c98
  "scenarios/lint-markdown-lints-only-what-it-is-given.md#Context": c0a0ee2fd9acd249
  "scenarios/lint-markdown-lints-only-what-it-is-given.md#Edge Cases": ed3bf156306fe825
  "scenarios/lint-markdown-lints-only-what-it-is-given.md#Lint-markdown-lints-only-what-it-is-given": b51cbcba00ce1f9b
  "scenarios/lint-markdown-lints-only-what-it-is-given.md#Open Questions": 9fc5979d590cdb97
  "scenarios/lint-markdown-lints-only-what-it-is-given.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/lint-markdown-tool-resolution.md": f4036d4aa3763d17
  "scenarios/lint-markdown-tool-resolution.md#Behavior": 1a5b3f94da1a55dd
  "scenarios/lint-markdown-tool-resolution.md#Context": b9d98b29c99f4e6d
  "scenarios/lint-markdown-tool-resolution.md#Edge Cases": 4e210f5452c8ba05
  "scenarios/lint-markdown-tool-resolution.md#Lint-markdown-tool-resolution": a6def4b8c35af471
  "scenarios/lint-markdown-tool-resolution.md#Open Questions": 9fc5979d590cdb97
  "scenarios/lint-markdown-tool-resolution.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/mark-task-backtick-headings.md": f4036d4aa3763d17
  "scenarios/mark-task-backtick-headings.md#Behavior": 3c8f04c6a9dac22b
  "scenarios/mark-task-backtick-headings.md#Context": 0206fd69c10645c0
  "scenarios/mark-task-backtick-headings.md#Edge Cases": 4c166d79aad2b86c
  "scenarios/mark-task-backtick-headings.md#Mark-task-backtick-headings": 1e219997149acb62
  "scenarios/mark-task-backtick-headings.md#Open Questions": 9fc5979d590cdb97
  "scenarios/mark-task-backtick-headings.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/mark-task-untick-symmetry.md": f4036d4aa3763d17
  "scenarios/mark-task-untick-symmetry.md#Behavior": 24257a25e4be83ac
  "scenarios/mark-task-untick-symmetry.md#Context": c370cf5ad7f5334a
  "scenarios/mark-task-untick-symmetry.md#Edge Cases": 10c5680df2ab6bfb
  "scenarios/mark-task-untick-symmetry.md#Mark-task-untick-symmetry": 840dd10f305793f1
  "scenarios/mark-task-untick-symmetry.md#Open Questions": 9fc5979d590cdb97
  "scenarios/mark-task-untick-symmetry.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/mcp-arg-unknown-field-strictness.md": f4036d4aa3763d17
  "scenarios/mcp-arg-unknown-field-strictness.md#Behavior": 13963d2d3a9ad282
  "scenarios/mcp-arg-unknown-field-strictness.md#Context": e9a9db7e2cc930d8
  "scenarios/mcp-arg-unknown-field-strictness.md#Edge Cases": d0a5106ccb0d5458
  "scenarios/mcp-arg-unknown-field-strictness.md#Mcp-arg-unknown-field-strictness": fe981603a7bfe2c9
  "scenarios/mcp-arg-unknown-field-strictness.md#Open Questions": 4e6614fe0e100066
  "scenarios/mcp-arg-unknown-field-strictness.md#Resolved Questions": 0a19145706d12dd0
  "scenarios/merge-managed-block-multi-subsection-end.md": f4036d4aa3763d17
  "scenarios/merge-managed-block-multi-subsection-end.md#Behavior": ef0c3a453da389fd
  "scenarios/merge-managed-block-multi-subsection-end.md#Context": bb39d09dabf65884
  "scenarios/merge-managed-block-multi-subsection-end.md#Edge Cases": 5296f51f062be14b
  "scenarios/merge-managed-block-multi-subsection-end.md#Merge-managed-block-multi-subsection-end": b23d5f754f3f2cbc
  "scenarios/merge-managed-block-multi-subsection-end.md#Open Questions": 9fc5979d590cdb97
  "scenarios/merge-managed-block-multi-subsection-end.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/merge-managed-block-renamed-subsection.md": f4036d4aa3763d17
  "scenarios/merge-managed-block-renamed-subsection.md#Behavior": 332fbb8bf80b8409
  "scenarios/merge-managed-block-renamed-subsection.md#Context": ab248985423a6478
  "scenarios/merge-managed-block-renamed-subsection.md#Edge Cases": 035565b3c6686ee7
  "scenarios/merge-managed-block-renamed-subsection.md#Merge-managed-block-renamed-subsection": 96da96ed576ad41e
  "scenarios/merge-managed-block-renamed-subsection.md#Open Questions": 9fc5979d590cdb97
  "scenarios/merge-managed-block-renamed-subsection.md#Resolved Questions": 83b9fc3e8c3ce336
  "scenarios/merge-managed-block-subsection-insertion.md": f4036d4aa3763d17
  "scenarios/merge-managed-block-subsection-insertion.md#Behavior": fd263af69480ba3f
  "scenarios/merge-managed-block-subsection-insertion.md#Context": 4c8fc412b7673aad
  "scenarios/merge-managed-block-subsection-insertion.md#Edge Cases": f800e94d3af185cf
  "scenarios/merge-managed-block-subsection-insertion.md#Merge-managed-block-subsection-insertion": 5f488fb4024c1a7d
  "scenarios/merge-managed-block-subsection-insertion.md#Open Questions": 9fc5979d590cdb97
  "scenarios/merge-managed-block-subsection-insertion.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/merge-managed-block-trailing-append.md": f4036d4aa3763d17
  "scenarios/merge-managed-block-trailing-append.md#Behavior": cd4c2bb1242a82e3
  "scenarios/merge-managed-block-trailing-append.md#Context": 12a8fb5c43b62a13
  "scenarios/merge-managed-block-trailing-append.md#Edge Cases": 93c9870c31e74238
  "scenarios/merge-managed-block-trailing-append.md#Merge-managed-block-trailing-append": 2bf2168a01725d99
  "scenarios/merge-managed-block-trailing-append.md#Open Questions": 9fc5979d590cdb97
  "scenarios/merge-managed-block-trailing-append.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/numbered-heading-grammar-single-source.md": f4036d4aa3763d17
  "scenarios/numbered-heading-grammar-single-source.md#Behavior": 5d3d8ff94b9b1b34
  "scenarios/numbered-heading-grammar-single-source.md#Context": cfbb9b9c7ba0de45
  "scenarios/numbered-heading-grammar-single-source.md#Edge Cases": 5330259410fbf029
  "scenarios/numbered-heading-grammar-single-source.md#Numbered-heading-grammar-single-source": b26c83270c836b46
  "scenarios/numbered-heading-grammar-single-source.md#Open Questions": 9fc5979d590cdb97
  "scenarios/numbered-heading-grammar-single-source.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/open-questions-are-any-list-item.md": 6260d2e0860b5319
  "scenarios/open-questions-are-any-list-item.md#Behavior": 7de3393a445c5938
  "scenarios/open-questions-are-any-list-item.md#Context": 0e5ebd941961f4dc
  "scenarios/open-questions-are-any-list-item.md#Edge Cases": 40d864b98a642b17
  "scenarios/open-questions-are-any-list-item.md#Open Questions": 9fc5979d590cdb97
  "scenarios/open-questions-are-any-list-item.md#Open-questions-are-any-list-item": cc0ec512842df3e1
  "scenarios/open-questions-are-any-list-item.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/parser-nested-list-continuation.md": f4036d4aa3763d17
  "scenarios/parser-nested-list-continuation.md#Behavior": 1d2d399068dac1e0
  "scenarios/parser-nested-list-continuation.md#Context": 094572a7bdeee051
  "scenarios/parser-nested-list-continuation.md#Edge Cases": 0a6a9394a562a340
  "scenarios/parser-nested-list-continuation.md#Open Questions": 51567fc383af6a9c
  "scenarios/parser-nested-list-continuation.md#Parser-nested-list-continuation": c44ca5857c31c8f4
  "scenarios/parser-nested-list-continuation.md#Resolved Questions": ab3f3391b5a16689
  "scenarios/parser-walker-conventions.md": f4036d4aa3763d17
  "scenarios/parser-walker-conventions.md#Behavior": fd90f4fc59717264
  "scenarios/parser-walker-conventions.md#Context": 1c7d135c06db836d
  "scenarios/parser-walker-conventions.md#Edge Cases": e2159f70f74d6cf5
  "scenarios/parser-walker-conventions.md#Open Questions": 9fc5979d590cdb97
  "scenarios/parser-walker-conventions.md#Parser-walker-conventions": 63897e4efc6f205c
  "scenarios/parser-walker-conventions.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/primitive-robustness-hardening.md": f4036d4aa3763d17
  "scenarios/primitive-robustness-hardening.md#Behavior": d899afdc1b9bf950
  "scenarios/primitive-robustness-hardening.md#Context": cf8791223b3bcee8
  "scenarios/primitive-robustness-hardening.md#Edge Cases": 9ead5aa2b1934e9d
  "scenarios/primitive-robustness-hardening.md#Open Questions": 9fc5979d590cdb97
  "scenarios/primitive-robustness-hardening.md#Primitive-robustness-hardening": f4e9da42882e7813
  "scenarios/primitive-robustness-hardening.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/project-directory-resolution-chain.md": 6260d2e0860b5319
  "scenarios/project-directory-resolution-chain.md#Behavior": 65de3603e60d74e2
  "scenarios/project-directory-resolution-chain.md#Context": b673a29e58e28833
  "scenarios/project-directory-resolution-chain.md#Edge Cases": 16d2fdedf09f1426
  "scenarios/project-directory-resolution-chain.md#Open Questions": bcccfc1fb1fb2471
  "scenarios/project-directory-resolution-chain.md#Project-directory-resolution-chain": 71aa789a7b9edfd9
  "scenarios/resolve-references-cli-exec-wiring.md": f4036d4aa3763d17
  "scenarios/resolve-references-cli-exec-wiring.md#Behavior": 646a63f6de437bd2
  "scenarios/resolve-references-cli-exec-wiring.md#Context": 048bc0459520f08c
  "scenarios/resolve-references-cli-exec-wiring.md#Edge Cases": 74c68756835f4010
  "scenarios/resolve-references-cli-exec-wiring.md#Open Questions": 9fc5979d590cdb97
  "scenarios/resolve-references-cli-exec-wiring.md#Resolve-references-cli-exec-wiring": ca94d59a9530614d
  "scenarios/resolve-references-cli-exec-wiring.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/review-exec-wiring.md": f4036d4aa3763d17
  "scenarios/review-exec-wiring.md#Behavior": 46f7edbd96995853
  "scenarios/review-exec-wiring.md#Context": e61c472239813bb7
  "scenarios/review-exec-wiring.md#Edge Cases": d800bb06abc7c4c2
  "scenarios/review-exec-wiring.md#Open Questions": 9fc5979d590cdb97
  "scenarios/review-exec-wiring.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/review-exec-wiring.md#Review-exec-wiring — thread primitive results so `/ductus:review` runs under `ductus exec`": 388ede255f3216f5
  "scenarios/review-runtime-acceleration.md": f4036d4aa3763d17
  "scenarios/review-runtime-acceleration.md#Behavior": 29ab98cdbf8ff7c1
  "scenarios/review-runtime-acceleration.md#Content-ingestion convention (one payload, not multiple large params)": a3eab0b8bdfbb593
  "scenarios/review-runtime-acceleration.md#Context": 28af659b6895dd98
  "scenarios/review-runtime-acceleration.md#Edge Cases": 1abd2a0b0c06fcac
  "scenarios/review-runtime-acceleration.md#Extension point": 08d851a8c5a3f95b
  "scenarios/review-runtime-acceleration.md#Open Questions": 20edda8881516e4e
  "scenarios/review-runtime-acceleration.md#Primitives": 3c93a3ad2baa4fea
  "scenarios/review-runtime-acceleration.md#Prose-convention tightening (#3 / #4 — applies to every rewritten command)": 8f133da97315f286
  "scenarios/review-runtime-acceleration.md#Resolved Questions": 05cdacc1312ce36d
  "scenarios/review-runtime-acceleration.md#Review-runtime-acceleration — primitives + `performReview` for `/ductus:review`": 10b59c3c23b568fb
  "scenarios/review-runtime-acceleration.md#Tests (comprehensive — concrete and testable per the constitution; never depend on human diligence)": eda653f81f037017
  "scenarios/review-scope-parse-fidelity.md": f4036d4aa3763d17
  "scenarios/review-scope-parse-fidelity.md#Behavior": 363f218725788e31
  "scenarios/review-scope-parse-fidelity.md#Context": bafdd8f1cdf30308
  "scenarios/review-scope-parse-fidelity.md#Edge Cases": f019b681389dba3a
  "scenarios/review-scope-parse-fidelity.md#Open Questions": 9fc5979d590cdb97
  "scenarios/review-scope-parse-fidelity.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/review-scope-parse-fidelity.md#Review-scope-parse-fidelity": 55fe3d5d5db00dca
  "scenarios/runtime-primitive-structural-bugs.md": f4036d4aa3763d17
  "scenarios/runtime-primitive-structural-bugs.md#Behavior": 82c086389232989b
  "scenarios/runtime-primitive-structural-bugs.md#Bug 1: `append-task` default body uses `title` where it expects a slug": 297ea8ec216123bf
  "scenarios/runtime-primitive-structural-bugs.md#Bug 2: `append-task` numbering hardcoded to `## N.` top-level": 7e0edc410c9784eb
  "scenarios/runtime-primitive-structural-bugs.md#Bug 3: `read-tasks` returns empty when tasks.md is phased": 8956097f990fd211
  "scenarios/runtime-primitive-structural-bugs.md#Bug 4: `check-stuck` measures from the wrong baseline": f7fec4f9c9f82d49
  "scenarios/runtime-primitive-structural-bugs.md#Context": f87e8b31945aeb37
  "scenarios/runtime-primitive-structural-bugs.md#Edge Cases": 578f0067f45bfa85
  "scenarios/runtime-primitive-structural-bugs.md#Open Questions": 20edda8881516e4e
  "scenarios/runtime-primitive-structural-bugs.md#Resolved Questions": c62f81495878ff80
  "scenarios/runtime-primitive-structural-bugs.md#Runtime-primitive-structural-bugs": 41ced06691413ffa
  "scenarios/runtime-review-019-hardening.md": f4036d4aa3763d17
  "scenarios/runtime-review-019-hardening.md#Behavior": 6316c3ba286032ff
  "scenarios/runtime-review-019-hardening.md#Context": 1ff2cd57ec68b83c
  "scenarios/runtime-review-019-hardening.md#Edge Cases": b84e90618d1c0fc2
  "scenarios/runtime-review-019-hardening.md#Open Questions": 9fc5979d590cdb97
  "scenarios/runtime-review-019-hardening.md#Resolved Questions": 140717fec5f57ecc
  "scenarios/runtime-review-019-hardening.md#Runtime-review-019-hardening": e44b4facb747c4d7
  "scenarios/scaffolding-primitives.md": f4036d4aa3763d17
  "scenarios/scaffolding-primitives.md#Behavior": 1d3206b76d038438
  "scenarios/scaffolding-primitives.md#Context": ecdf902029489574
  "scenarios/scaffolding-primitives.md#Edge Cases": 1ee46fb4422e7ad0
  "scenarios/scaffolding-primitives.md#Open Questions": 9fc5979d590cdb97
  "scenarios/scaffolding-primitives.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/scaffolding-primitives.md#Scaffolding-primitives": 1560e589a58868f2
  "scenarios/scenario-open-question-signal.md": f4036d4aa3763d17
  "scenarios/scenario-open-question-signal.md#Behavior": 3d0e37512c9832ef
  "scenarios/scenario-open-question-signal.md#Context": 29b298a473df6563
  "scenarios/scenario-open-question-signal.md#Edge Cases": 3a7d7de2bf1a27b4
  "scenarios/scenario-open-question-signal.md#Open Questions": 20edda8881516e4e
  "scenarios/scenario-open-question-signal.md#Resolved Questions": fe1717cbcb01232b
  "scenarios/scenario-open-question-signal.md#Scenario-open-question-signal": b43f06f251de4575
  "scenarios/scenario-question-parser-fix.md": f4036d4aa3763d17
  "scenarios/scenario-question-parser-fix.md#Behavior": 29c9e4ff8a95953e
  "scenarios/scenario-question-parser-fix.md#Context": 83ff6b11d123c758
  "scenarios/scenario-question-parser-fix.md#Edge Cases": 30f6bc27437878dc
  "scenarios/scenario-question-parser-fix.md#Open Questions": f7af3fc1755659ec
  "scenarios/scenario-question-parser-fix.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/scenario-question-parser-fix.md#Scenario-question-parser-fix": 931f99521bbf61fe
  "scenarios/severity-is-a-closed-set-not-a-string.md": f4036d4aa3763d17
  "scenarios/severity-is-a-closed-set-not-a-string.md#Behavior": 2389bf9dcaa0ed6a
  "scenarios/severity-is-a-closed-set-not-a-string.md#Context": 4d1859255bc720fa
  "scenarios/severity-is-a-closed-set-not-a-string.md#Edge Cases": 12f16126d549ed73
  "scenarios/severity-is-a-closed-set-not-a-string.md#Open Questions": 9fc5979d590cdb97
  "scenarios/severity-is-a-closed-set-not-a-string.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/severity-is-a-closed-set-not-a-string.md#Severity-is-a-closed-set-not-a-string": d1b3fc040ac32b5d
  "scenarios/skipscanner-inline-code-exemption.md": f4036d4aa3763d17
  "scenarios/skipscanner-inline-code-exemption.md#Behavior": 6939965cb550def7
  "scenarios/skipscanner-inline-code-exemption.md#Context": c51380ac5288812b
  "scenarios/skipscanner-inline-code-exemption.md#Edge Cases": dcc867353f83c4a2
  "scenarios/skipscanner-inline-code-exemption.md#Open Questions": 9fc5979d590cdb97
  "scenarios/skipscanner-inline-code-exemption.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/skipscanner-inline-code-exemption.md#Skipscanner-inline-code-exemption": da57fbccfa088eec
  "scenarios/spec-side-parser-hardening.md": f4036d4aa3763d17
  "scenarios/spec-side-parser-hardening.md#Behavior": c380493beb8b2713
  "scenarios/spec-side-parser-hardening.md#Context": 7b58f34b8b6fb9bb
  "scenarios/spec-side-parser-hardening.md#Edge Cases": f4e92a9a3452f26e
  "scenarios/spec-side-parser-hardening.md#Open Questions": 9fc5979d590cdb97
  "scenarios/spec-side-parser-hardening.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/spec-side-parser-hardening.md#Spec-side-parser-hardening": 52c59e3892c00ce4
  "scenarios/specify-routes-before-scaffolding.md": f4036d4aa3763d17
  "scenarios/specify-routes-before-scaffolding.md#Behavior": da7a61e22a93a214
  "scenarios/specify-routes-before-scaffolding.md#Context": 0b47293c6bf33e98
  "scenarios/specify-routes-before-scaffolding.md#Edge Cases": dd606d9bc0b31f71
  "scenarios/specify-routes-before-scaffolding.md#Open Questions": 9fc5979d590cdb97
  "scenarios/specify-routes-before-scaffolding.md#Resolved Questions": d8b898030a8404d3
  "scenarios/specify-routes-before-scaffolding.md#Specify-routes-before-scaffolding": c6dbdd6c9b5d5b82
  "scenarios/the-cli-surface-nothing-pins.md": f4036d4aa3763d17
  "scenarios/the-cli-surface-nothing-pins.md#Behavior": 517f873b45f0e6c7
  "scenarios/the-cli-surface-nothing-pins.md#Context": 459eb4734578eab0
  "scenarios/the-cli-surface-nothing-pins.md#Edge Cases": cabb4c4332258463
  "scenarios/the-cli-surface-nothing-pins.md#Open Questions": 9fc5979d590cdb97
  "scenarios/the-cli-surface-nothing-pins.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/the-cli-surface-nothing-pins.md#The-cli-surface-nothing-pins": d100a862dd541f24
  "scenarios/the-constitutions-registry-validates-its-values.md": f4036d4aa3763d17
  "scenarios/the-constitutions-registry-validates-its-values.md#Behavior": 693daf159678adeb
  "scenarios/the-constitutions-registry-validates-its-values.md#Context": 108d67e1da1b2a67
  "scenarios/the-constitutions-registry-validates-its-values.md#Edge Cases": d3290fc59e16dbe8
  "scenarios/the-constitutions-registry-validates-its-values.md#Open Questions": 9fc5979d590cdb97
  "scenarios/the-constitutions-registry-validates-its-values.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/the-constitutions-registry-validates-its-values.md#The-constitutions-registry-validates-its-values": 83436a898f3e93d8
  "scenarios/the-pi-command-candidate.md": 24fa40bf71f310f7
  "scenarios/the-pi-command-candidate.md#Behavior": f1f6636df389d2b5
  "scenarios/the-pi-command-candidate.md#Edge Cases": 9a6ebb90d777122d
  "scenarios/the-pi-command-candidate.md#Open Questions": 9fc5979d590cdb97
  "scenarios/the-pi-command-candidate.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/the-pi-command-candidate.md#The-pi-command-candidate": 9053fa3687d87cbb
  "scenarios/unchecked-done-when-clause-tally.md": f4036d4aa3763d17
  "scenarios/unchecked-done-when-clause-tally.md#Behavior": d4557a50790a733e
  "scenarios/unchecked-done-when-clause-tally.md#Context": 4b29159d55568799
  "scenarios/unchecked-done-when-clause-tally.md#Edge Cases": 6020d38e0565a57f
  "scenarios/unchecked-done-when-clause-tally.md#Open Questions": 9fc5979d590cdb97
  "scenarios/unchecked-done-when-clause-tally.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/unchecked-done-when-clause-tally.md#Unchecked-done-when-clause-tally": 4836cfd0fc969655
  "scenarios/unreadable-scenario-is-reported.md": f4036d4aa3763d17
  "scenarios/unreadable-scenario-is-reported.md#Behavior": 5ffd0c602a574186
  "scenarios/unreadable-scenario-is-reported.md#Context": 46b8d80792d67693
  "scenarios/unreadable-scenario-is-reported.md#Edge Cases": 0feb6b6da1222b4e
  "scenarios/unreadable-scenario-is-reported.md#Open Questions": 9fc5979d590cdb97
  "scenarios/unreadable-scenario-is-reported.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/unreadable-scenario-is-reported.md#Unreadable-scenario-is-reported": 7f678e02d44b3ccc
  "scenarios/waiver-processing-order.md": f4036d4aa3763d17
  "scenarios/waiver-processing-order.md#Behavior": 25e01f73114a40c3
  "scenarios/waiver-processing-order.md#Context": cef9e15dc8c7abdc
  "scenarios/waiver-processing-order.md#Edge Cases": e8ad69633d43da41
  "scenarios/waiver-processing-order.md#Open Questions": 9fc5979d590cdb97
  "scenarios/waiver-processing-order.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/waiver-processing-order.md#Waiver-processing-order": 32a41e1a0fcd73bc
  "scenarios/write-boundary-path-normalization.md": f4036d4aa3763d17
  "scenarios/write-boundary-path-normalization.md#Behavior": baddd7d6e3ff022d
  "scenarios/write-boundary-path-normalization.md#Context": 4092c0c42ce10c20
  "scenarios/write-boundary-path-normalization.md#Edge Cases": 25b5ad1691e30b55
  "scenarios/write-boundary-path-normalization.md#Open Questions": 9fc5979d590cdb97
  "scenarios/write-boundary-path-normalization.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/write-boundary-path-normalization.md#Write-boundary-path-normalization": 18432baab9cb2c3b
  "scenarios/write-session-primitive.md": 6260d2e0860b5319
  "scenarios/write-session-primitive.md#Behavior": 100132e56aee4461
  "scenarios/write-session-primitive.md#Context": 650f29ef2c7f82e2
  "scenarios/write-session-primitive.md#Edge Cases": 83553e909e19ed52
  "scenarios/write-session-primitive.md#Open Questions": 9fc5979d590cdb97
  "scenarios/write-session-primitive.md#Resolved Questions": 37a991886b047c84
  "scenarios/write-session-primitive.md#Write-session-primitive": 1fe658d59a0f4f6c
  "scenarios/writecode-boundary-derivation.md": f4036d4aa3763d17
  "scenarios/writecode-boundary-derivation.md#Behavior": fc0bc95a7c8de563
  "scenarios/writecode-boundary-derivation.md#Context": 3480da3ca1b4ff4b
  "scenarios/writecode-boundary-derivation.md#Edge Cases": 8bb948a62e754441
  "scenarios/writecode-boundary-derivation.md#Open Questions": 20edda8881516e4e
  "scenarios/writecode-boundary-derivation.md#Resolved Questions": 285e75188306d789
  "scenarios/writecode-boundary-derivation.md#Writecode-boundary-derivation": 988d467e981b9317
  "scenarios/writecode-payload-bundling.md": 037db3ce16126339
  "scenarios/writecode-payload-bundling.md#Behavior": 958512858f56179d
  "scenarios/writecode-payload-bundling.md#Context": 6daa9a819cf40500
  "scenarios/writecode-payload-bundling.md#Edge Cases": 8333e101fcbe8f27
  "scenarios/writecode-payload-bundling.md#Open Questions": 9fc5979d590cdb97
  "scenarios/writecode-payload-bundling.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/writecode-payload-bundling.md#Writecode-payload-bundling": 261736dcfb964819
  "scenarios/writecode-payload-canonicalize-paths.md": 037db3ce16126339
  "scenarios/writecode-payload-canonicalize-paths.md#Behavior": 994de01bb7a1947e
  "scenarios/writecode-payload-canonicalize-paths.md#Context": 3793eafcfd54c175
  "scenarios/writecode-payload-canonicalize-paths.md#Done-when": bbfc7cd9a4eb96fc
  "scenarios/writecode-payload-canonicalize-paths.md#Edge Cases": 2d6b90b116f31864
  "scenarios/writecode-payload-canonicalize-paths.md#Open Questions": 9fc5979d590cdb97
  "scenarios/writecode-payload-canonicalize-paths.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/writecode-payload-canonicalize-paths.md#Writecode-payload-canonicalize-paths": bb1b2ff6b957301a
  "spec.md": a67065a1e6935f14
  "spec.md#022 — Deterministic Runtime": c52a480d46ea0470
  "spec.md#Acceptance Criteria": 46b262a35cf198fc
  "spec.md#Applicable Rules": 342f760d52e6dc6f
  "spec.md#Architecture": 2d87a83033453ad1
  "spec.md#Bash script relationships": ef70febae7dad974
  "spec.md#CI integration": 44343a53c0e9c29d
  "spec.md#Follow-on scenarios": 080847bfe94edae0
  "spec.md#Initial release — six commands": 40da364488900954
  "spec.md#Install policy": a6542933189dad83
  "spec.md#LLM extension points": 37eb6e7820bfc988
  "spec.md#MCP server": 8e523001d4a874ca
  "spec.md#Motivation": 2ad3ef0972b670ba
  "spec.md#Non-Goals": 0a83e5a344baf055
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Parseable slash command procedures": ae9d43b5af52aa43
  "spec.md#Per-rewrite checklist": ce1c9aaeb68d7613
  "spec.md#Resolved Questions": 1921a9072f102456
  "spec.md#Slash command rewiring": 27e47c5f99a0b0b8
  "spec.md#Subprocess interpreter": 44435834d84eeeea
  "spec.md#Surface choice by adopter": 1f379cdd9fdf941f
  "spec.md#Surfaces": 473807f7c4c2d2ac
  "spec.md#The interpreter": a4cc8522c0949f1b
  "spec.md#The primitive library": 4f8adb1342e30ffe
  "spec.md#Without the MCP server": 552d53da54a24914
rule-verdicts:
  BE-ASYNC-001: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-AUTHZ-004: {verdict: addressed, cite: "plan.md#Agent reach: what ductus installs grants only what its steps need"}
  BE-AUTHZ-007: {verdict: addressed, cite: "data-model.md#Primitive request/response schemas"}
  BE-COMPAT-001: {verdict: addressed, cite: "scenarios/review-runtime-acceleration.md#Primitives"}
  BE-COORD-002: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-DATA-001: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  BE-DEPS-001: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-002: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-003: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-DEPS-004: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-005: {verdict: addressed, cite: "plan.md#Dependency pinning, provenance, and upgrades"}
  BE-DEPS-007: {verdict: addressed, cite: "plan.md#`runtime.yml` CI workflow"}
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#Outbound (runtime → host) messages"}
  BE-ERR-004: {verdict: addressed, cite: "data-model.md#Outbound (runtime → host) messages"}
  BE-ERR-005: {verdict: addressed, cite: "plan.md#Agent reach: what ductus installs grants only what its steps need"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Extension-point schemas (initial release)"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  BE-INPUT-006: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
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
  BE-PAYLOAD-004: {verdict: finding, tier: should, at: "plan.md#`fetch-archive` transport", message: "fetch-archive's download (up to MAX_FETCH_BYTES, 256 MiB) and extract-archive's read (up to MAX_EXTRACT_BYTES, 2 GiB) are large bodies, but the plan never states they are streamed in bounded chunks and names no chunk or buffer size constant."}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-RACE-003: {verdict: addressed, cite: "data-model.md#`create-feature` — scaffold the next feature directory"}
  BE-REPR-001: {verdict: addressed, cite: "data-model.md#Notes"}
  BE-REPR-002: {verdict: finding, tier: should, at: "data-model.md#Outbound (runtime → host) messages", message: "The data model names no casing convention for enum values and mixes two: extension-point values are camelCase (writeCode, assessSpecQuality) while message types, severities, error codes and outcomes are lower-kebab-case."}
  BE-RETRY-001: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-RETRY-003: {verdict: addressed, cite: "plan.md#Release workflow"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#MCP surface: one tool per primitive, no orchestration tools"}
  BE-TIMEOUT-001: {verdict: waived, reason: "`ductus exec`'s waits for an `llm-response` or a `gate-response` are on the host that spawned the runtime and holds both of its pipes, and a gate waits on a person, so no bound fits either wait; the walk ends when the host closes stdin, which `read_envelope` reports as the end of input (owner decision, 2026-11-19)", decided-by: "andy@stone.dev", decided-at: "2026-11-19T17:20:03Z"}
  BE-TIMEOUT-002: {verdict: addressed, cite: "plan.md#`runtime.yml` CI workflow"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Compare before persist"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#Compare before persist"}
  CFG-CONST-001: {verdict: addressed, cite: "data-model.md#Per-project file resolution"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-003: {verdict: addressed, cite: "scenarios/the-constitutions-registry-validates-its-values.md#Behavior"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#Release workflow"}
  CFG-ENV-006: {verdict: addressed, cite: "plan.md#Dependency selection"}
  CFG-ENV-007: {verdict: addressed, cite: "data-model.md#Per-project file resolution"}
  CFG-ENV-008: {verdict: addressed, cite: "scenarios/review-runtime-acceleration.md#Edge Cases"}
  CFG-ENV-009: {verdict: addressed, cite: "plan.md#`fetch-archive` transport"}
  CFG-ENV-010: {verdict: addressed, cite: "plan.md#Versioning: built-in from `Cargo.toml`"}
  CFG-ENV-012: {verdict: addressed, cite: "scenarios/review-runtime-acceleration.md#Primitives"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "data-model.md#`writeCode`"}
  QUAL-DELEG-001: {verdict: addressed, cite: "scenarios/open-questions-are-any-list-item.md#Behavior"}
  QUAL-GROUND-001: {verdict: addressed, cite: "plan.md#`framework/runtime-tools.txt` is the manifest"}
  QUAL-STUB-001: {verdict: addressed, cite: "scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md#Behavior"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Path arguments: a lexical screen, then canonical containment"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#`runtime.yml` CI workflow"}
  SEC-LOG-001: {verdict: addressed, cite: "plan.md#Dependency selection"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 12
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
  - key: "rule-assessment — BE-PAYLOAD-004: fetch-archive's download (up to MAX_FETCH_BYTES, 256 MiB) and extract-archive's read (up to MAX_EXTRACT_BYTES, 2 GiB) are large bodies, but the plan never states they are streamed in bounded chunks and names no chunk or buffer size constant."
    outcome: waived
    reason: "SHOULD tier and advisory: the rule guards per-request memory in a service holding many bodies at once, while the runtime is one local process that fetches and extracts one release archive per bootstrap run, with MAX_FETCH_BYTES capping the download and MAX_EXTRACT_BYTES and MAX_EXTRACT_ENTRIES the extraction"
    decided-at: 2026-11-19T18:06:29Z
    decided-by: rule
  - key: artifact-size — `specs/022-deterministic-runtime/plan.md` is 109611 bytes, 3 read pages at the 50000-byte read size
    outcome: waived
    reason: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did
    decided-at: 2026-11-22T02:11:15Z
    decided-by: rule
  - key: artifact-size — `specs/022-deterministic-runtime/data-model.md` is 63645 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did
    decided-at: 2026-11-22T02:11:15Z
    decided-by: rule
  - key: "rule-assessment — BE-REPR-002: The data model names no casing convention for enum values and mixes two: extension-point values are camelCase (writeCode, assessSpecQuality) while message types, severities, error codes and outcomes are lower-kebab-case."
    outcome: waived
    reason: "SHOULD tier and advisory: the two casings are two closed sets with their own readers, extension-point names camelCase as the `<!-- llm:… -->` markers in command sources spell them (`assessSpecQuality`, `writeCode`) and message types, severities and codes lower-kebab-case; renaming either is a breaking change to the exec protocol for no reader, and `specs/system.md` declares no convention to follow"
    decided-at: 2026-11-22T02:11:15Z
    decided-by: rule
  - key: grounding — Motivation states that most of the analyze/implement time is spent on parsing, checkbox walking and similar non-semantic work, an unmeasured claim about current behavior.
    outcome: waived
    reason: a motivation stated when the spec was written, describing the problem the runtime was built to solve rather than current behavior, the class of the stored 2-4 minutes waiver beside it
    decided-at: 2026-11-22T22:29:02Z
    decided-by: rule
  - key: grounding — Surfaces, Subprocess interpreter, estimates host integration at a few hundred lines of code, stated unhedged with no source.
    outcome: waived
    reason: "advisory and no chore: a design-time estimate of a third-party host's integration effort, recorded where the surface is described; it states no current behavior, and grounding it would be a measurement of an integration this repository does not have, not an edit"
    decided-at: 2026-11-22T22:29:02Z
    decided-by: rule
---

# Analysis — 022-deterministic-runtime

## Summary

0 hard-fail, 0 blocking, 12 advisory; not blocking. 84 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 12 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Motivation states as fact that a /ductus:analyze invocation takes 2-4 minutes of LLM wall-clock time today, with no measurement, command or hedge cited. — `specs/022-deterministic-runtime/spec.md` — **waived**: a motivation measured when the spec was written, describing the problem the runtime was built to solve rather than current behavior
- grounding — Motivation states that 11 of 13 /ductus:analyze check sections are mechanical, a count about current command content with no citation to the command source. — `specs/022-deterministic-runtime/spec.md` — **waived**: a motivation measured when the spec was written; analyze's sections have since been restructured, and the claim records the case for the runtime, not current behavior
- grounding — Motivation states that most of the analyze/implement time is spent on parsing, checkbox walking and similar non-semantic work, an unmeasured claim about current behavior. — `specs/022-deterministic-runtime/spec.md` — **waived**: a motivation stated when the spec was written, describing the problem the runtime was built to solve rather than current behavior, the class of the stored 2-4 minutes waiver beside it (decided by the rule)
- grounding — Resolved Questions, Implementation language, states as fact that rmcp is the most mature MCP SDK and the reference implementation, and that Go has faster builds and easier cross-compilation, with no source for either external claim. — `specs/022-deterministic-runtime/spec.md` — **waived**: the rationale for a 2026-05 dependency choice, recorded in a Resolved Question and the plan's dependency list
- grounding — Surfaces, Subprocess interpreter, estimates host integration at a few hundred lines of code, stated unhedged with no source. — `specs/022-deterministic-runtime/spec.md` — **waived**: advisory and no chore: a design-time estimate of a third-party host's integration effort, recorded where the surface is described; it states no current behavior, and grounding it would be a measurement of an integration this repository does not have, not an edit (decided by the rule)
- grounding — Known limitations, pulldown-cmark edge cases, asserts that tables, footnotes and definition lists are off by default in pulldown-cmark, an external library behavior with no citation to its documentation. — `specs/022-deterministic-runtime/plan.md` — **waived**: rationale for a rejected alternative in Trade-offs; no current behavior rests on it
- scenario-consistency — scenario done-when-authoring-forms.md has no task in tasks.md and never had one in its history; append the task that implements it — `specs/022-deterministic-runtime/scenarios/done-when-authoring-forms.md` — **waived**: written after the fact for behavior shipped in the same commit (959d897d, fix(022): recognize all done-when authoring forms in read-tasks); implemented and tested, read_tasks.rs recognizes_all_done_when_authoring_forms (operator decision, 2026-10-16)
- scenario-consistency — scenario scenario-question-parser-fix.md has no task in tasks.md and never had one in its history; append the task that implements it — `specs/022-deterministic-runtime/scenarios/scenario-question-parser-fix.md` — **waived**: written after the fact to record shipped scenario-open-question work (f43818d1); implemented and tested, read_spec.rs parse_open_questions with commented_out_example_questions_are_not_counted, fenced_questions_are_not_counted and every_placeholder_is_skipped_even_as_a_bullet (operator decision, 2026-10-16)
- artifact-size — `specs/022-deterministic-runtime/plan.md` is 109611 bytes, 3 read pages at the 50000-byte read size — `specs/022-deterministic-runtime/plan.md` — **waived**: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did (decided by the rule)
- artifact-size — `specs/022-deterministic-runtime/data-model.md` is 63645 bytes, 2 read pages at the 50000-byte read size — `specs/022-deterministic-runtime/data-model.md` — **waived**: 022's plan and data model are the design record of the runtime core and its shared surfaces; prune-plan finds no section outside the design record, and no slice of 022 can reach done on its own to split off, so each reader reads them in parts by offset, as this run's judges did (decided by the rule)
- rule-assessment — BE-REPR-002: The data model names no casing convention for enum values and mixes two: extension-point values are camelCase (writeCode, assessSpecQuality) while message types, severities, error codes and outcomes are lower-kebab-case. — `specs/022-deterministic-runtime/data-model.md` — **waived**: SHOULD tier and advisory: the two casings are two closed sets with their own readers, extension-point names camelCase as the `<!-- llm:… -->` markers in command sources spell them (`assessSpecQuality`, `writeCode`) and message types, severities and codes lower-kebab-case; renaming either is a breaking change to the exec protocol for no reader, and `specs/system.md` declares no convention to follow (decided by the rule)
- rule-assessment — BE-PAYLOAD-004: fetch-archive's download (up to MAX_FETCH_BYTES, 256 MiB) and extract-archive's read (up to MAX_EXTRACT_BYTES, 2 GiB) are large bodies, but the plan never states they are streamed in bounded chunks and names no chunk or buffer size constant. — `specs/022-deterministic-runtime/plan.md` — **waived**: SHOULD tier and advisory: the rule guards per-request memory in a service holding many bodies at once, while the runtime is one local process that fetches and extracts one release archive per bootstrap run, with MAX_FETCH_BYTES capping the download and MAX_EXTRACT_BYTES and MAX_EXTRACT_ENTRIES the extraction (decided by the rule)

## Deferred to other specs

*None.*

## Waived rules

- BE-TIMEOUT-001 (MUST) — `ductus exec`'s waits for an `llm-response` or a `gate-response` are on the host that spawned the runtime and holds both of its pipes, and a gate waits on a person, so no bound fits either wait; the walk ends when the host closes stdin, which `read_envelope` reports as the end of input (owner decision, 2026-11-19) — decided by `andy@stone.dev` at `2026-11-19T17:20:03Z`

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 3
- never-existed: 12
- not-a-live-claim: 10
- root-absent: 54
- ships-to-adopter: 5

## Fixed in this run

- rule-assessment — BE-COMPAT-001: The changed Primitives text (and spec.md's 020 signposts) modifies existing primitives' contracts (discover-rule-files leaves files out of its selection; process-waivers/write-review anchor waivers by site, and an anchor recorded before sites expires) without classifying either change as additive or breaking. — `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md` — **fixed**
