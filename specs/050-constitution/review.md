---
spec: 050-constitution
last-run: 2026-11-23T23:22:45Z
reviewed-against: 4ace1d513115bf8f30422035b03eae640134fff7
diff-base: 124092e12dc3d49cca8c05c548bf3b4df6199260
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 27
unread:
  - .claude/commands/ductus/implement.md
  - .claude/commands/ductus/review.md
  - .ductus/reference/implement.md
  - .pi/prompts/ductus-implement.md
  - .pi/prompts/ductus-review.md
  - framework/commands/implement.md
  - framework/commands/review.md
  - framework/reference/implement.md
  - runtime/src/primitives/write_analysis.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/primitives/write_review/departure_waivers.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/golden/review-basic.jsonl
  - runtime/tests/mcp.rs
  - scripts/mutation/departure-waivers.py
  - scripts/mutation/departures.py
  - scripts/mutation/freshness-results-have-a-bounded-size.py
  - scripts/mutation/review-departures-bounded.py
  - scripts/mutation/waiver-notices-flatten.py
  - specs/073-decided-rule-departures-are-recorded/plan.md
  - specs/073-decided-rule-departures-are-recorded/tasks.md
  - specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md
excluded: 17
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
diff-checked: 0
not-re-read: 1
budget-stop: false
round-waivers:
  - round: 1
    reason: the round's 6 paths are other sessions' uncommitted work made after the run's base, none a 050 fix, and 050's one fix, the plan.md correction, is exempt; re-read declined (the agent's recommendation, adopted by the operator, 2026-11-23)
    waived-by: andy@stone.dev
    waived-at: 2026-11-23T23:22:45Z
reviewed-digest:
  scenarios/a-canonical-source-is-pointed-at-not-copied.md: 87a0dcce92143a717ff19e0ecc570e9262f7afaf42ac764fe43b197c6a7e48fa
  scenarios/a-decision-is-attributed-to-whoever-made-it.md: 0e91d8bb56e3c762d89879cd41d6aaa6cb26059d0f11e238031f938441983922
  scenarios/a-declared-cross-spec-impact-gates-done.md: 1f802a9ed0562b74fddc7984c9ff6d89d0d4c9a46d02138b874928381d03f92f
  scenarios/a-measurement-states-its-method-and-units.md: 4c250a480c11e9788058f1012b647daa14aba2036d4a3baa9d0c1ba3c5965b2a
  scenarios/a-partial-read-is-not-a-read.md: 078952ab7a67edeb949dd30d5f1bc0e0ef91904c467980f1e5547aab145e82f0
  scenarios/a-primitive-is-probed-in-a-local-clone.md: 92a3158c331cd59e69365df9faac2cbcdd7a693b29ecad636d9b6f4a7a85ca04
  scenarios/a-project-command-is-run-not-reproduced.md: 82eb4ccfa37346e02b05e32e802f5ec528a633801407a32e43ff678921329ce6
  scenarios/a-retired-feature-leaves-no-spec.md: bcaaa454287a16ca7494d06e7269110989191e4e84791e9f868a7bc6378e4a2e
  scenarios/a-retired-filename-leaves-a-decision-record.md: 721239305aed86b463cf047871ce4699a8018862115a51ee56c9ee8a24146685
  scenarios/a-review-may-precede-its-commit.md: b3b90fcf6f0f8d06c9db96e873c657370fda86e2a15e450d34bcf3735d259163
  scenarios/a-status-commit-holds-only-the-transition.md: 2f164ce212508e7f3b82508543157b868e8533fd15f977497d2ab569ec89f79e
  scenarios/adopter-practices-classified-for-promotion.md: 497849c654a3aca6253c10ead0eeea6c082fc7b90269809ed732b7be48739d05
  scenarios/agents-md-holds-only-what-every-session-needs.md: d98998ffcc8ddc321ef97b6e3b80fc2b6a523f82357d00b778a576b0790a8c68
  scenarios/an-answer-is-applied-as-chosen.md: ddccff2f13211a718842fd0954bc8e2e446eb99ad8fe6c60ebc9f87f2704f5fc
  scenarios/artifacts-are-as-short-as-their-required-content.md: 9cdb9568fd92e7ad24a1d213924733f31266809d9dabe97daf22fe8847044cc6
  scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
  scenarios/decisions-go-to-the-owner-one-at-a-time.md: 91dd288f71c59a081c23f8b380218cc89924859727e52885a88cf491c5465060
  scenarios/findings-route-by-scope.md: 482085721b560cc87fe3772936aca0668694e63c58c4d2093d70ae337ad1223c
  scenarios/governance-is-multi-source.md: 40078aa4ec14c45487493e4bfe67b026c95bb28377e2626e0ac4876f68cb183f
  scenarios/knowledge-routes-by-population-not-by-kind.md: 4de854f76c0fb8ff44b853543ece6d3b97f96994d4c62fa3b2358da7a477f689
  scenarios/memory-routing-lives-in-agents-md.md: c17eda6a37465999c1ceb21a224b1a5e85f3ea49a7112214f21178249783bd16
  scenarios/report-outcomes-not-edits.md: caf13342d7fe0a5af4108024cf424ab921030c896f0743307cf5111be2e163ab
  scenarios/reports-end-with-the-remaining-work.md: 816e498bb2ba5bd9bfa42682cad814955af57b5af656669754cdcec5c7dbdc3d
  scenarios/the-constitution-fits-one-read.md: 26c63cbf857a23a360fe4f887c62eccf2abcdf33763bce25fa973d4ae0605840
  scenarios/the-constitution-states-rules-not-stories.md: 691e5ea78b7012fd9f9a8114f20632d7852d5c319f54e3951b2ebaed65c013bc
criteria-judged: 26
criteria-met: 26
criteria-verdicts:
  - label: AC1
    digest: 4745828ccd0ef4a19caccefd50497a7057ce43d00240ff07b84ddd5f204cd310
  - label: AC2
    digest: 270b7a4738c6d1337c297103ac598521c2da97753cf36f422289cd991f58e054
  - label: AC3
    digest: e2def4e8216af33b376742b03768a8725b10ee16b4b13a8b1ba7f3881318ac3e
  - label: AC4
    digest: 916f1ae3f607800a7a98d49fbd4eb4123c4634c8aaebb4b54ae2302e7155ab67
  - label: AC5
    digest: 547df794d82df6286887f059d1d9991064d0b0ba72c8c3a7d6d7d7482e4ef29b
  - label: AC6
    digest: 1cec0037d2f7e8753dc3729250c50c0d916a83502fde66dc58b7744b42964f71
  - label: AC7
    digest: 2f1e6b34d426153b42887a76b617f52d90a55841b40070709994439440e34823
  - label: AC8
    digest: ebae7da98352fc8dd916987bd7fb8497e7749bfa2d3120b69e395326d02a0646
  - label: AC9
    digest: 0433b7dc2cc9fa5c9dbd05069c36e8d90f82fb9fa08736963fd482c248b09ef0
  - label: AC10
    digest: 76bfc83cd9e709906bced54b53329ad541d588823e57ab913f14f4c0e3a43a16
  - label: AC11
    digest: e8abae65ad56671875780436f3bea55c97c7e7d9f6a2fcf915b9d00161df07e6
  - label: AC12
    digest: 1665c49d2847b1e39392792258625ae2bc32f412e8e1a365e98192b632a4860e
  - label: AC13
    digest: 80db5ac1316960ebc136551e790fc004c3064685ab434ab27c975c15c7fdf25b
  - label: AC14
    digest: c1e394d1479c711e8766a08f968f79e75bf875aa6fd95e6127fbe3d0268340e2
  - label: AC15
    digest: be6d30ac2bfb1f317acbca8519facfa9e31a29bea1ac82df0cc3ef71dedccfb5
  - label: AC16
    digest: 4d868565653a589140f1360a3b123be9c1e7f53a0db5cf9995c485207d430770
  - label: AC17
    digest: f83ebe699106e76c280769375b118e7f74a97a120b27c7c775a51abe1c53a3ac
  - label: AC18
    digest: bed9f67e72edcd0419d3fc136d6aeaaf3da18bdf1602febdfc7bd4111b03820f
  - label: AC19
    digest: 0074680c4747fe72b5707b3633fcf274ecfd6d5c3d74b45271f6ea94f9712f0f
  - label: AC20
    digest: 8b09f8206179af7fcec7c26b9a0514b88afda47e4305a8cacce1f92ac9413644
  - label: AC21
    digest: 197b1aa843ba6eb37c03a583815e5f8a241cf0908f52f2bd043e1a642540778b
  - label: AC22
    digest: 55067ba914a858d00366ed2894e6e77fa3a1abfbb017ec85d2315997ae9cc85b
  - label: AC23
    digest: a50e9e37c058582c4b3b1bc86c0bbfb874a4febe9fd18e59d01c960098628a4e
  - label: AC24
    digest: 9bf3fb9aa805944d3ad70d4bc1b3e3bf6d8a1642b34e4d5780db9505ba36bf2f
  - label: AC25
    digest: 5411d8ff5ef51e289c3fc30321d2262a5fbeae6c8330a2e4ee156bec138495cc
  - label: AC26
    digest: 249ab6ada03fe79ebec75b7c39b281107cfd88c2135a867381991766bc999751
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: deferred
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: deferred
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: deferred
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: deferred
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: waived
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: waived
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: waived
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T00:57:57Z
    decided-by: andy@stone.dev
  - key: "convention: a-status-commit-holds-only-the-transition's Behavior reads as requiring the bullet to say a transition staged alone is lost — `specs/050-constitution/scenarios/a-status-commit-holds-only-the-transition.md`"
    outcome: waived
    reason: the Behavior requires the bullet to state its condition, that the spec file holds nothing but the transition, and its reason, that the pre-commit hook restages the file whole; the bullet states both, and the loss of a transition staged alone is the scenario's own account of the incident
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: claude-md-import-syntax's shared-import bullet contrasts its rewrite with a removal the procedure no longer performs — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: the bullet still guards against de-duplicating an import the adopter added by hand, which removal would drop if the registration were withdrawn; its sense does not depend on the constitution line
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 017's plan and 072's Affected Files still name the old import form — `specs/017-derive-dont-ask/plan.md`"
    outcome: waived
    reason: each records that spec's own edit at the time, a decision record, not a claim about the tree today
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: a CLAUDE.md symlinked to AGENTS.md becomes a copy when constitution-import writes it by temporary file and rename — `framework/migrations/constitution-import.md`"
    outcome: waived
    reason: every bootstrap write of CLAUDE.md already does this, merge-managed-block's managed-block render on every run included, so the migration adds no new case; a symlinked CLAUDE.md is not a layout the framework supports
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 022's constitution-excerpts-as-skill-resources says every CLAUDE.md carries the constitution import — `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`"
    outcome: waived
    reason: this change makes the premise true again, the template's third line being @.ductus/constitution.md; 022 is another session's and owes no edit
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: constitution-import accepts a four-space-indented line, misses a match behind a leading byte-order mark, and reports the @import form on a file unpinned after claude-md-import-syntax was skipped — `framework/migrations/constitution-import.md`"
    outcome: waived
    reason: "contrived inputs: claude-md-import-syntax treats indentation the same way, the template writes no byte-order mark, and an operator who unpins a file has the earlier migration's pinned warning in hand"
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "convention: a #fragment containing a dot is cut at the dot on a fold into a scenario — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: GitHub's heading slugs carry no dots, so such a fragment names no heading a link in this corpus can reach; a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: whether Auggie resolves CLAUDE.md's @AGENTS.md import, and so reads the routing block, is unverified — `framework/templates/project/claude-md.md`"
    outcome: waived
    reason: the rule's canonical statement is the constitution's drift-prevention section, which target step 4 loads whenever an import did not, so an Auggie session holds it either way
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "bug: a run the Frontmatter Migration precheck halts after generator-primitives leaves a pre-0.30 hook calling the scripts that migration deleted, so the commit the refusal asks for fails — `framework/bootstrap/ductus-procedure.md`"
    outcome: deferred
    target: specs/013-text-first-artifacts/scenarios/the-precheck-ignores-what-the-same-run-wrote.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: generator-primitives and audit-record-relocate state no report line for their no-op cases — `framework/migrations/generator-primitives.md`"
    outcome: waived
    reason: an observation about the migration machinery outside this window; the host's own wording reported each outcome truthfully
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fetch-archive sends no User-Agent — `runtime/src/primitives/fetch_archive.rs`"
    outcome: deferred
    target: specs/048-govern-acquired-runtime/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a .githooks linked into a host configuration directory inside the project is not judged by host_config::check_destination — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: a contrived layout; the write stays inside the project, and the referrer list is fixed to five files none of which is a host's configuration
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: the spec-root walk drops a directory it cannot list, so it is neither counted nor skipped — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: a directory the process cannot list inside a repository it writes is contrived, and the walk's filter predates this window
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a root-absolute /specs/ link is neither re-pointed nor reported — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: deferred
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an archive entry whose only mode bits are group or other write lands at the umask default — `runtime/src/primitives/extract_archive.rs`"
    outcome: waived
    reason: "a contrived entry: no archive the framework ships carries a file with neither owner read nor owner write"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: 062 task 23's checked item still gives the fixed-interval rationale for the lock poll — `specs/062-concurrent-session-targets/tasks.md`"
    outcome: waived
    reason: a ticked task records the decision as it was taken, task 28 records its reversal, and tasks.md is ephemeral work tracking that prune may drop
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: installed-file-modes M5's kill depends on ctime resolution finer than 50 ms — `scripts/mutation/installed-file-modes.py`"
    outcome: waived
    reason: disclosed in the list's docstring; APFS and ext4 stamp ctime in nanoseconds, and the test itself passes on any file system
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an @/specs/x import in CLAUDE.md is read as project-relative, where Claude Code reads an absolute path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: an absolute import of a top-level /specs directory is a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a kept-tails entry gives the reference from the spec root, without a prefix kept on the line — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: its path and line locate the reference, and the check reports the same span from the root
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a ./ prefix in the hook or a root markdown file assumes the project root is the work-tree root, which a project in a repository subdirectory breaks — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: waived
    reason: a ./ resolves as the bare paths in those files already do, the assumption every bare reference carries, not one task 30 adds
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: the Family 19 parity tests run git in their fixtures under a hook's inherited GIT_DIR, so a commit from a linked worktree fails four of them — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a leading / was read from the project root in a project below its repository's root, where a host resolves it from the repository's root — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the framework reads a root-absolute path from the project root, as check-corpus-links reads a root-absolute link (spec 059); reading it from the repository's root here alone split the fold's link and token passes, so 3fc5755a's gate was reverted
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root, /proj/specs/x, the project's own path spelled from the repository's root, is neither checked nor re-pointed — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: a project's README spelling its own spec from the repository's root is a contrived input, and the project-root reading names it nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root the hook runs from the repository's root, so its ./ and bare paths resolve there — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: waived
    reason: the same assumption every bare reference in the hook carries, which task 30 does not add; prefixed and bare agree
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: bare-rule cases outside task 30 — ~/\".ductus/x\", \"$X\"specs/x and ${X}specs/x in the hook, ${X:-./specs/x}, *specs/x*, and @/specs/x in CLAUDE.md — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the bare rule predates task 30 and prefixed and bare agree on each; every input is contrived
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: the leading-/ exception also covers (a)/specs/x, x~/specs/x and a quote after a letter — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: contrived text; requiring $( before ) would add a parser for no real input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: no agreement test puts a path at column 0 in system.md, so a mutant refusing a line-start path only there survives, in the check's tests and the rewrite's — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the scanner has no per-referrer branch, so a defect of that shape is contrived; the binary reports a bare, ../ and / path at column 0 in system.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: the subdirectory test asserts that a spec only the repository's root holds is reported, not that one only the project holds is not — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the realistic regression, reading existence from the repository's root, is killed by M46; skipping only a path present at both roots is contrived, and the binary does not report the project-only spec
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: two ticked items in 017's tasks.md name a rule's Rationale, which 70bb4af5 removed from the format — `specs/017-derive-dont-ask/tasks.md`"
    outcome: waived
    reason: tasks.md is ephemeral work tracking whose ticked items record what was built then (§tasks-phase); the format's canonical statement, 017's data-model.md, is current
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: other — principle 3 of §runtime-boundary still says commands assume determinism rather than specifying two executable paths to one result and that a quietly absent requirement leaves both paths alive, wording from the two-paths era that ad4fae5b's message says is gone from framework/ (its grep for two-paths and two paths does not match two executable paths) — `framework/constitution.md:387`
    outcome: waived
    reason: the sentence is still true of the commands, which each specify one executable path, and it is the stated reason acquisition halts the run; principle 3's wording is quoted by 021 and 048 and the plan keeps the five runtime principles byte-identical
    decided-at: 2026-11-16T01:05:27Z
    decided-by: rule
---

# Review — 050-constitution

## Summary

Reviewed 050's reopening for AC26 (owner decision, 2026-11-23) from its reopen, diff-base 124092e1, against 4ace1d51. The passes read bcc439f9's tree; 4ace1d51 (079's tasks 42 and 43) landed during the write and changed none of the five files they read. Of the window's 9 commits, 4 are 050's (52cba526, dd9ece1e, 7339c3ac, 7b782f43); 073's, 079's and 083's are excluded. The five passes, run in one subagent, read 5 of the 27 scope entries whole, each bytes read equal to its size: AGENTS.md (49,905), framework/constitution.md (49,990), specs/050-constitution/plan.md (66,896), spec.md (34,895) and tasks.md (6,287). They also read every loaded rule file whole: configuration-cross.md, quality-cross.md, security-backend.md, security-cross.md and security-frontend.md. The scope was 12 entries when resolved; other sessions' uncommitted work grew it to 27 while the run went on, and the scope was resolved again in the same command as this write. The other 22 entries are other sessions' uncommitted work, were not read, and every one is listed in unread. They include 073's departure-notices change and mutation lists, 073's plan.md and tasks.md, 083's spec.md, and an implement command change. The runtime/target/release/ductus binary that wrote this record, and that .ductus/bin/ductus links, was built from that uncommitted work; 050 has no Applicable Rules section, so the change does not reach this record. No finding. Two observations. In plan.md, a stale claim that §cost-levers' /cost example stays verbatim for 010's AC11 contradicted task 53's drop of it; it was fixed in the run as a correction, which takes no back-edge, and 010's AC11 still holds because §Cost levers says token budgets are the AI platform's. The principle 3 observation was matched to its stored waiver after a check of framework/constitution.md found it still stands. The criteria step ran in a second subagent, as on 050's last review: 26 of 26 met. It read plan.md, the constitution and AGENTS.md in the sections each criterion names, not whole. AC2 and AC3 were enumerated over 44 of 44 promote rows and AC4 over 51 project-only keys. The host re-checked four criteria. AC1: 50 rows of the 54-entry corpus remain; 072 removed three project-only rows and 079 the create-scenario row, and plan.md notes each removal. AC4: in this window, dd9ece1e touched only DP1's pointer and 40dea24c's post-survey bullet. AC24: check-artifacts parsed 77 of 77 specs and found 0 line-number-citations findings. AC26: the filter is the last bullet of §design-principles, and AGENTS.md carries no copy. AC26 is ticked.

1 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 1 not re-read. Round 1 was not re-read, waived: the round's 6 paths are other sessions' uncommitted work made after the run's base, none a 050 fix, and 050's one fix, the plan.md correction, is exempt; re-read declined (the agent's recommendation, adopted by the operator, 2026-11-23).

26 of 26 acceptance criteria judged met.

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

## Acceptance criteria

- AC1 — met: plan.md Classification tables: 22 promoted + 3 already promoted + 25 project-only rows = 50 of 54 (4 rows removed by 072 and 079, each noted beside its table); each row carries a reason; AGENTS.md 2026-08-17 corpus scope stated
- AC2 — met: Checked 44 of 44 promote rows (first-round 22 + second-round 21 + fourth-round 1): each AGENTS.md entry resolves and points at a framework/constitution.md# anchor, reference or rule file; shipped-anchors audit resolves all anchors; 079 supersession annotation holds
- AC3 — met: Same 44 of 44 AGENTS.md lines are pointers (none lacks a framework/constitution.md#, reference or rule pointer); the three 079-moved entries are absent from AGENTS.md as annotated
- AC4 — met: 49 of 51 project-only keys resolve to unaltered-looking AGENTS.md entries by lead phrase (the other 2 carry survey markers and were found by hand); 3ebafd8c appended the one sentence the criterion names; wording-level immutability since 08-17 not byte-diffed
- AC5 — met: framework/constitution.md cites no scripts/, .github/ or docs/-only repo path in promoted rules; its only repo-specific path is the canonical-sources row naming runtime/src/schema/descriptions.rs, which is a map row from 022, not a promoted rule
- AC6 — met: constitution Implement Phase bullet: ticked criteria verified by review; check examines in-progress and done specs and proves only a path resolves, not that its claim holds (as the superseded-in-part annotation states)
- AC7 — met: constitution ships via Shared Files manifest to .ductus/constitution.md; audit manifest-dest 35a/35b and archive-contents (78 of 78) pass in scripts/audit/run-all.sh (exit 0)
- AC8 — met: scripts/audit/run-all.sh exit 0 over the whole run (82 lines read); promotion-coverage 49 unclassified is a notice for post-survey entries, which AC1 excludes
- AC9 — met: npx markdownlint-cli2 on framework/constitution.md and AGENTS.md: 0 issues, exit 0 (config globs 903 files)
- AC10 — met: Anchor markers intact (principles, cost-levers, design-principles, grounding, governance-precedence, etc.); audit shipped-anchors examined 576 of 576 files, 2579 refs, no failure
- AC11 — met: framework/constitution.md Canonical sources table row 'Constitution content' names specs/050-constitution/spec.md
- AC12 — met: git diff HEAD on version, runtime/Cargo.toml, runtime/CHANGELOG.md empty; audit Family 20 passes; criterion's 061 annotation (releases may move the pin) is the only drift
- AC13 — met: constitution Spec lifecycle third back-edge and 'Any other edit there reopens' paragraph state the mechanical test (a)(b)(c) and the correction case, matching the 079 annotation
- AC14 — met: constitution Design principles: 'Work that is not complete MUST never be indistinguishable...' with fix/defer/waive and the measured-residue clause; Implement Phase SHOULD bullet says 'This is design-principles completion filter' instead of restating
- AC15 — met: Brownfield Inbox Finding dispositions: In implement, scope decides (task/spec/outside = one of three dispositions); tasks.md no capture queue; AGENTS.md 'A finding made during implementation' points at it; 058 and 079 annotations match
- AC16 — met: constitution Spec lifecycle: retired feature spec deleted, consolidate when content survives, spec goes when nothing survives, partial retirement is body edit, re-point before removal; AGENTS.md pointer; scenario a-retired-feature-leaves-no-spec states nothing enforces it
- AC17 — met: spec.md Classification states the shared tier and population table, reword test narrowed, floor bound points to governance-precedence (marker exists in constitution); config.toml has no [constitutions.*]; no reclassification
- AC18 — met: constitution Cross-Spec Impact 'What the framework enforces, and what it does not' states declared gates done, discharge by back-link, undeclared undetected; schema table carries cross-spec-impact; old acceptance-criterion enforcement sentence absent
- AC19 — met: constitution Cross-document references 'A retired filename is not a retired name' names ladder/migration/spelling categories, current name elsewhere, ticked criterion swept, annotation for superseded behavior, runtime computes reopens from the diff
- AC20 — met: constitution Shared knowledge stays in git states two questions and population routing; AGENTS.md header points at it; framework/templates/project/agents.md ships empty ductus:memory-routing markers; bootstrap/ductus.md writes the one-line block; both CLAUDE.md files hold only imports (template plus constitutions block)
- AC21 — met: constitution Grounding Rules: 'A recorded measurement states its method and its units' sits right after 'Cite what you consulted' and states every named element; AGENTS.md general entry is a pointer; only the git -E gotcha keeps a corollary per annotation
- AC22 — met: quality-cross.md has QUAL-DELEG-001; AGENTS.md entry points at it; spec.md Classification states the fourth destination; 036 in cross-spec-impact
- AC23 — met: constitution is 49990 bytes (<=50000) and scripts/lint-governing-size.sh exit 0; template and repo CLAUDE.md use @path imports; migrations claude-md-import-syntax and constitution-import exist and match the description
- AC24 — met: Grounding bullet states the rule; grep of spec/plan/tasks/data-model/research/scenario files finds no path:N citation; check-artifacts on 050 clean; reference/plan.md, reference/clarify.md, commands/plan.md point at it with no 'bare line number' text; AGENTS.md citation gotcha is a pointer
- AC25 — met: Guiding Principles holds only cost-levers and design-principles; no Technology or Business heading (03f78075 removed 18 principles); succinctness scenario deleted; 008 AC28 annotated superseded; remaining mentions in 008/010/072 are decision records
- AC26 — met: constitution Design principles final bullet 'Use what a task needs, and no more, however far below a limit' states the filter; grep finds no restatement in AGENTS.md; lint-governing-size passes at 49990

## Observations

- convention: plan.md contradicts itself on §cost-levers' `/cost` example: §Artifacts are as short as their required content keeps it verbatim for 010-agent-autonomy AC11, while §A task uses what it needs, and no more says task 53 dropped it; the constitution no longer carries it, and 010's AC11 needs only that §Cost levers point at platform tooling — `specs/050-constitution/plan.md` — **fixed** (decided by the rule)
- other — principle 3 of §runtime-boundary still says commands assume determinism rather than specifying two executable paths to one result and that a quietly absent requirement leaves both paths alive, wording from the two-paths era that ad4fae5b's message says is gone from framework/ (its grep for two-paths and two paths does not match two executable paths) — `framework/constitution.md:387` — **waived**: the sentence is still true of the commands, which each specify one executable path, and it is the stated reason acquisition halts the run; principle 3's wording is quoted by 021 and 048 and the plan keeps the five runtime principles byte-identical (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/review.md`
- `framework/reference/review.md`
- `runtime/CHANGELOG.md`
- `runtime/src/primitives/check_departures.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/departures.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/schema/descriptions.rs`
- `scripts/mutation/scope-attribution.py`
- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/tasks.md`

## Unexamined governance

*None.*
