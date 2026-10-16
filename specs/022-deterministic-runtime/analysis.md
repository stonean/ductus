---
spec: 022-deterministic-runtime
last-run: 2026-10-16T01:30:16Z
analyzed-against: ad7be704f96ce4a62f303fbc3d989bedef1ec089
hard-fail: 0
blocking-findings: 0
advisory: 10
unexamined: 2
analyzed-digest:
  data-model.md: 246e101e4bd67cee52a37df1fdd2ec0448a69fe31335f79913b2039059a659ab
  plan.md: fcb26e5ac43fa550692dcba54e9c95037c49a39b08906a21ffd819ce887b48f8
  review.md: 9ebc296a632d769b4bf29060a39e589d1a4e7f0e5e1d99862275e0049c469b0d
  scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md: d092f9d4f59c67c575cc59039a6e0fdf162546969de2820e747b609d7a3d43c7
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
  scenarios/config-resolution-single-probe.md: fc602eb6f2d64f6d1956da411b809af3a53a0c91b26dd198f6b1af61e3347ab4
  scenarios/config-sourced-paths-and-the-traversal-boundary.md: 5ce4f8c45bdfd56606b9a977431e06683f4f38904742f96c358e6b6c5828b30e
  scenarios/constitution-excerpts-as-skill-resources.md: c093f8288e03f8df28666cfd93ce057abba77a7badbcaac16413aacb8da7f746
  scenarios/coverage-expansion-primitives.md: 4626b19eef4cff4f9ce01047b2afa6746573ec289da039cc969b8d8d37ebcd3f
  scenarios/coverage-residue-cleanup.md: 04dc18f12694ed8b02821b50498c7e8ff8123993de72a27ae0d7c8722e024637
  scenarios/criterion-label-assignment.md: 4a1a3acc5d5ddcd44a07582988e06a1b97b2339c71482ccd24765fefb33207a0
  scenarios/dashboard-primitive.md: 9b513d55debc46091babff8120b3395f0eca3555e2556b076a92555c6f28b303
  scenarios/derive-boundary-uncommitted-spec-dir.md: ca28aaef13416801e00bc40d9d58da09c7a513b237bb659597e1efc91a0aaef5
  scenarios/done-when-authoring-forms.md: 080f8717135b67c8400e072dae02f172e40cb0b6ad596c3e227d6cc5f90d0191
  scenarios/exec-analyze-derives-its-list-seeds.md: 9fda65d4cd7a5ec0fab7e07ba9d5d25e794944c967cd09e7b3547f8e1d560726
  scenarios/exec-clarify-asks-each-open-question.md: 4851dc543c6dd80603aa86c776543017b9ab1a7adac469e1b49ec311a9877cab
  scenarios/extension-request-hygiene.md: 1b0c77335af3a2caf82e01896207d60261cd0071a8bc8da3162b9785f7c16e48
  scenarios/fetch-archive-dns-rebinding.md: 39482bed2a531d36dce770c9c20ae3848ba254da0d02ee51d5c6feabe7967031
  scenarios/fetch-archive-names-its-timeout.md: e714dd3679dc72aa9bc27a933ba4e32505a9c4290f09b9b0eb48812f7459317a
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
  scenarios/review-exec-wiring.md: de565bee22e478ebf57bd9fdf7ddc8ddd79374fa6caf659e8f669d76c398262d
  scenarios/review-runtime-acceleration.md: 91bf90e87e1b4aebeda541201bc3f448ea293c34fd24dcc5371a7d50b20a813d
  scenarios/review-scope-parse-fidelity.md: 89747b789e8c2f6d1b550cd662ddfe04a4a42727edba6daa39e6b700a6d2eee1
  scenarios/runtime-primitive-structural-bugs.md: 7fb973b4c5ef5c0268df24996c585f9bf70465f4f7e36ad237d129894f5fd366
  scenarios/runtime-review-019-hardening.md: 9b759c9ce4fc57702e28d764179dcae7b7df9d04dc99ac166b3c0726cf3f9449
  scenarios/scaffolding-primitives.md: faa5098bd2434ec823611879ede2d1f9ff56276650c29bfddfe40512d88ca96f
  scenarios/scenario-open-question-signal.md: bd9e127f09f6aa34417ee912f71a05f0205607b28c799944017a196943b7e129
  scenarios/scenario-question-parser-fix.md: bdcaf5f9bb7d0ff795d6e3a6a5045a79afc63b55d8ce4b3331f807c337bf2ada
  scenarios/severity-is-a-closed-set-not-a-string.md: 8b6ca46ef55c4f6b12420786ebae219f5aa4f2a2878c85d042098f5674013d2e
  scenarios/skipscanner-inline-code-exemption.md: ab402c94a437b19b1b9bdd5a58ad3303d6bb67e9822041520522e3a066c43ab5
  scenarios/spec-side-parser-hardening.md: 91a804882033f369ff98f36157a38896d29eb7d90e8ae4c78e86d45ac35aca3d
  scenarios/specify-routes-before-scaffolding.md: a67c6a1d0ad9322b73f33581169f3d63c8d344e088d30e55bdf292a2748068ad
  scenarios/the-cli-surface-nothing-pins.md: f63de14533aeb6b2c77f33421af289633e0fded63e795f219b82bfa30f071c8c
  scenarios/the-constitutions-registry-validates-its-values.md: 91d822062ae0e0d10a9796c7072a7cd03186283ba2aa7657b12a696f705bd2c2
  scenarios/the-pi-command-candidate.md: 38e81f6ca1732254fee5cce65eb829a0026c30bdb846ff0c3a80b7a308ba94bb
  scenarios/unchecked-done-when-clause-tally.md: 2fa92d88a135df29d4097e5fac7ba2a42583303237cb81bcb8a8df3a7e157ade
  scenarios/unreadable-scenario-is-reported.md: 28c763fee22e177c50586f69989911128438c23964e1fa4831f628ae366792ae
  scenarios/waiver-processing-order.md: 0adb750e651d94ac8bcf06dd0dfbba84cd937c09817035e913ec08b77687f8e8
  scenarios/write-boundary-path-normalization.md: aea2703ee38d7a2ab5806444dd12f99370bfee540bafd8935c9f20d43bd86dee
  scenarios/write-session-primitive.md: 006711337bb99bcb386c0f16e4659eb85e7cf7bb14d4211113550c8c27570bc8
  scenarios/writecode-boundary-derivation.md: 6c9f4071e87d008c30a3741740cbe5c294f0faa9e41c4a823c3898f52b7ce077
  scenarios/writecode-payload-bundling.md: 5c343929c3a42ac4406a02c173b6979231127b9583aabae171fee1f747d5084b
  scenarios/writecode-payload-canonicalize-paths.md: 61fc0c5dffd7d22ffd6e413e1c4f98adc1869f3fef7357340021d49b964a4a7c
  spec.md: 7928bbfe83ff3a80421ec03e789b272d9894e5c488a25f190941a1c3a2a6e610
  tasks.md: 034d55915c65b29284ba384b8e6a8c81bc37a3f754030cde10a905982597eb07
unexamined-by-reason:
  not-a-live-claim: 2
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 10
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-METRIC-001: the MCP tool handlers commit to no rate, error or duration metrics"
    outcome: discarded
    reason: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 062's analysis)
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says a single-feature /ductus:analyze takes 2-4 minutes of LLM wall-clock time today
    outcome: discarded
    reason: a motivation measured when the spec was written, describing the problem the runtime was built to solve rather than current behavior
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says 11 of 13 analyze check sections are mechanical
    outcome: discarded
    reason: a motivation measured when the spec was written; analyze's sections have since been restructured, and the claim records the case for the runtime, not current behavior
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md and plan.md call rmcp the reference MCP SDK
    outcome: discarded
    reason: the rationale for a 2026-05 dependency choice, recorded in a Resolved Question and the plan's dependency list
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says POSIX rename is atomic and Windows rename is weaker
    outcome: discarded
    reason: a property of the platforms the atomic-write design rests on, stated as rationale; the CI matrix exercises the write on all three
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says certain markdown extensions are off by default in pulldown-cmark
    outcome: discarded
    reason: rationale for a rejected alternative in Trade-offs; no current behavior rests on it
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says no primitive but fetch-archive makes an HTTP request of its own
    outcome: discarded
    reason: a scope statement in the Trade-offs entry recording the reqwest dependency's adoption; runtime/Cargo.toml is cited, and the plan's fetch-archive transport section names the path
    decided-at: 2026-10-15T16:54:50Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says tokio is required by rmcp
    outcome: discarded
    reason: rationale for a dependency choice, the same class as the stored rmcp discard
    decided-at: 2026-10-15T16:54:50Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says pulldown-cmark provides byte offsets
    outcome: discarded
    reason: rationale for the parser choice, the same class as the stored pulldown-cmark discard
    decided-at: 2026-10-15T16:54:50Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md and plan.md say analyze runs gen-help-tables through run-generator and the pre-commit hook calls the generators directly
    outcome: discarded
    reason: describes the wiring the Bash script relationships section exists to record, naming the command and the hook it describes
    decided-at: 2026-10-15T16:54:50Z
    decided-by: andy@stone.dev
---

# Analysis — 022-deterministic-runtime

## Summary

0 hard-fail, 0 blocking, 10 advisory; not blocking. 2 unexamined target(s). Dispositions: 0 fixed, 0 routed, 10 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-METRIC-001: the MCP tool handlers commit to no rate, error or duration metrics — `specs/022-deterministic-runtime/plan.md` — **discarded**: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 062's analysis)
- grounding — spec.md says a single-feature /ductus:analyze takes 2-4 minutes of LLM wall-clock time today — `specs/022-deterministic-runtime/spec.md` — **discarded**: a motivation measured when the spec was written, describing the problem the runtime was built to solve rather than current behavior
- grounding — spec.md says 11 of 13 analyze check sections are mechanical — `specs/022-deterministic-runtime/spec.md` — **discarded**: a motivation measured when the spec was written; analyze's sections have since been restructured, and the claim records the case for the runtime, not current behavior
- grounding — spec.md and plan.md call rmcp the reference MCP SDK — `specs/022-deterministic-runtime/spec.md` — **discarded**: the rationale for a 2026-05 dependency choice, recorded in a Resolved Question and the plan's dependency list
- grounding — plan.md says POSIX rename is atomic and Windows rename is weaker — `specs/022-deterministic-runtime/plan.md` — **discarded**: a property of the platforms the atomic-write design rests on, stated as rationale; the CI matrix exercises the write on all three
- grounding — plan.md says certain markdown extensions are off by default in pulldown-cmark — `specs/022-deterministic-runtime/plan.md` — **discarded**: rationale for a rejected alternative in Trade-offs; no current behavior rests on it
- grounding — plan.md says no primitive but fetch-archive makes an HTTP request of its own — `specs/022-deterministic-runtime/plan.md` — **discarded**: a scope statement in the Trade-offs entry recording the reqwest dependency's adoption; runtime/Cargo.toml is cited, and the plan's fetch-archive transport section names the path
- grounding — plan.md says tokio is required by rmcp — `specs/022-deterministic-runtime/plan.md` — **discarded**: rationale for a dependency choice, the same class as the stored rmcp discard
- grounding — plan.md says pulldown-cmark provides byte offsets — `specs/022-deterministic-runtime/plan.md` — **discarded**: rationale for the parser choice, the same class as the stored pulldown-cmark discard
- grounding — spec.md and plan.md say analyze runs gen-help-tables through run-generator and the pre-commit hook calls the generators directly — `specs/022-deterministic-runtime/spec.md` — **discarded**: describes the wiring the Bash script relationships section exists to record, naming the command and the hook it describes

## Unexamined targets

- not-a-live-claim: 2

## Fixed in this run

*None.*
