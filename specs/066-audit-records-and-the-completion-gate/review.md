---
spec: 066-audit-records-and-the-completion-gate
last-run: 2026-11-19T00:38:50Z
reviewed-against: efd3d488116e0752e8e7fb94f5c712c42f6263c5
diff-base: 78fd056ade2638e31f158626cd4df79fde08ad11
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 23
scope: 64
unread:
  - .claude/commands/ductus/amend.md
  - .claude/commands/ductus/implement.md
  - .claude/commands/ductus/review.md
  - .ductus/reference/audit.md
  - .ductus/reference/implement.md
  - .ductus/reference/review.md
  - .github/workflows/runtime-acquisition.yml
  - .github/workflows/runtime-release.yml
  - .pi/prompts/ductus-amend.md
  - .pi/prompts/ductus-implement.md
  - .pi/prompts/ductus-review.md
  - AGENTS.md
  - framework/bootstrap/ductus.md
  - framework/bootstrap/govern.md
  - framework/commands/amend.md
  - framework/constitution.md
  - framework/reference/audit.md
  - runtime/CHANGELOG.md
  - runtime/src/interpreter/mod.rs
  - runtime/src/interpreter/payload.rs
  - runtime/src/main.rs
  - runtime/src/primitives/check_review_gate.rs
  - runtime/src/primitives/compute_review_scope.rs
  - runtime/src/primitives/line_citations.rs
  - runtime/src/primitives/mod.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/extensions.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/mcp.rs
  - scripts/audit/README.md
  - scripts/audit/done-spec-criteria.sh
  - scripts/mutation/line-citation-strip-066.py
  - scripts/mutation/reopen-measure.py
  - scripts/mutation/scope-attribution.py
  - specs/000-slash-commands/spec.md
  - specs/029-bootstrap-runtime-autowire/review.md
  - specs/029-bootstrap-runtime-autowire/spec.md
  - specs/048-govern-acquired-runtime/data-model.md
  - specs/048-govern-acquired-runtime/plan.md
  - specs/048-govern-acquired-runtime/spec.md
excluded: 26
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: b4209d91ef5401225341a84cb8ce4b75063516a20781258c2f2df09b1a05450c
  scenarios/a-done-spec-has-no-transition-to-gate.md: adaf8b75ee0e15ca259cb8a0156e824af99a28c28aab876466798daa4e219e3a
  scenarios/a-done-specs-plan-edit-leaves-its-analysis-current.md: 3d63a66653413cb90127966cf092b984c71d44dad6d9dda77eba08131f6737e6
  scenarios/a-multi-number-renumber-is-one-rewrite.md: 918149e6f0137d420222d80b45c11ff0b415ed2af0349ea16da23457cf2bd589
  scenarios/a-renamed-file-contributes-its-rewrites.md: b578c5093853df3201d5d4b451b21c55943afb4840a47cb6d45f263a96e2a9fe
  scenarios/a-review-states-what-it-read.md: a75902f6c548b664d670a03ef4642b8cb4ff20fbdccfa39468666fa5d9787490
  scenarios/a-supplied-summary-carries-the-loops-account.md: 80f366f0462d15212f4c5ceb7c097095fd5c5cd87ee68f769056c3dc1c803882
  scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md: 39b59f38bb7b419b250d16d6ff6f3fec473959a3f9293706a6fcab931f6f232d
  scenarios/checkbox-state-is-outside-the-analysis-digest.md: ca9c4875ae8dec48c0c79edc74f3af3d4ee405601c6a8b1dee3c9a68f748834d
  scenarios/criteria-are-verified-in-review.md: 46195c281734fabf5056a32817a48667b7d6465b7a2f3f8119f16c238637c8c7
  scenarios/freshness-results-have-a-bounded-size.md: 0365689904e4ca8d3a11c14ce4411acbe95373560bf6ffa390a17afda8c832e0
  scenarios/only-gating-findings-hold-done.md: 27a6385fb7da6ccd6bf74303f92559b00998fe170248408e1df76bdf3a0b0f29
  scenarios/process-decisions-results-have-a-bounded-size.md: ef377a26ae73760cc642e3c8ca364ee8fb1d680b97f07d2bc18395de3c314013
  scenarios/process-waivers-file-lists.md: 3fb3b9e54f414691d28d984ba76cd48c7e1de4fe2ee67259a44ea8559da447dc
  scenarios/process-waivers-notices-flatten-stored-text.md: a0fe96d2f894add12b9ed470dfdc011ce1383b1528871c8d68ec64bb85d64c03
  scenarios/review-base-includes-the-transition-commit.md: 10d6d341e7338ac19fc9f35c9d2f79960ebc73aca7bf5e702adad17924cd8518
  scenarios/review-scope-includes-uncommitted-work.md: 9864ae8c8bb85d0ab1606b222b983f6fe449f89b21580af7b0a1c4e644e45bba
  scenarios/review-scope-plan-affected-table-format.md: 501c1d9737e6d2b928318c02e1d13164817ba7c1054854cd599f14707caba767
  scenarios/review-scope-reads-every-cell-path-and-sets-deleted-files-apart.md: e33e76432da85f5cc661041515f0f3aae300ad9974065e5eadba696cc820748c
  scenarios/review-scope-union.md: 50722645d52601a9b8f0c799272f58923a7250623abc097ad38bb5a4ec62181e
  scenarios/review-staleness-on-done-specs.md: e0c14689fc666e2658b2cf7f9891b0f00d717aee8bd3c871717d54ed7ff6900c
  scenarios/scope-results-have-a-bounded-size.md: 99120b144942e06b365e220c30085592a02c477c548e6336f8c23400e1ee4c9d
  scenarios/the-committed-tree-horizon.md: 046b6a8dcef5f85bdb9083308311729911f5afa49f0351c0505bc558b5adb96b
  scenarios/the-cross-spec-impact-gate.md: f94b4a47110ad23eab638e824f87b4a50b926e3df6ae70fe18d0261a8377cb35
  scenarios/the-fix-round-loop-is-recorded-and-gated.md: 93e7c7087a46831527685c57c1385cc5b5f1d1c45a4bcdae5b94d49559c15024
  scenarios/write-analysis-and-the-second-gate.md: 0bf446e34a55a8da10f04327c86d0d872e081130545b21158f15e3f86444741c
  scenarios/write-review-known-field-quoting.md: 914954823ee3da52dccbb25749851013d5dd3be442dcc85b09661c27c7bea2a9
criteria-judged: 6
criteria-met: 6
criteria-verdicts:
  - label: AC1
    digest: 9e0e58b8b19eaa4f663946642909bbfc3fb0ea98da7d377348c4395302a42e96
  - label: AC2
    digest: 15e0d6111aba62f9bcc84078d271215849a876e276afe5800b61a65f6a9b8575
  - label: AC3
    digest: 2e3b9b81f7b28d7d1bb9a2c2737ddd344c20bb4d94be221a6db94cb8f44a3780
  - label: AC4
    digest: e9c519e1f2a474c3e09a61a1f92e0417efb822600a7879dca7a9f5cf91525531
  - label: AC5
    digest: 288faa8b2d4022c825de9c8c0047ae4ab2e6a8895cce3afe76089cd74d5b0c50
  - label: AC6
    digest: 010fec4de3c948b457a9c6ce91153ca3e200783fe86ab5f7b66b154a9cc07016
blocking: false
dispositions:
  fixed: 25
  deferred: 0
  waived: 23
  undispositioned: 0
decisions:
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: waived
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: waived
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the implement reference's \"non-blocking\" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md`"
    outcome: waived
    reason: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md`"
    outcome: waived
    reason: "pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: waived
    reason: the Context records the state when the scenario was found, and the close is loose, not false
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md`"
    outcome: waived
    reason: "contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true"
    decided-at: 2026-10-27T06:47:44Z
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
  - key: other — fdc6d3c1 edited 058's data-model.md (the process-decisions result rows) while 058 is `done`; a data-model edit reopens a done spec under constitution §spec-lifecycle (classify-spec-edit reason data-model), yet 058 stays `done` and 066's spec.md declares no cross-spec-impact for it. The fix lands in 058's directory (status/back-edge), not 066's. — `specs/058-findings-route-at-discovery/data-model.md`
    outcome: waived
    reason: the edit mirrors a canonical shape 066 owns and describes it correctly; reopening 058 is a spec-lifecycle decision in another spec's directory the parent owns, and nothing in 066's code or records breaks
    decided-at: 2026-11-15T16:33:10Z
    decided-by: rule
  - key: "other — invalidate-review's SCALARS list drops the run's fix-round fields but leaves `open-findings` and `unread` (079/081 fields) in a record it resets to `last-run: null` (verified on a fixture built under the scratchpad: both survive). compute-review-scope reads `open_findings` from the HEAD record without requiring last-run, so an invalidated record can still name paths owing a review; `unread` is credited only when last-run is set, so it is inert. — `runtime/src/primitives/invalidate_review.rs:112-137`"
    outcome: waived
    reason: open-findings left on an invalidated record keep the paths the invalidated run left unresolved in the next review's scope, which is conservative; no gate reads either field when last-run is null, and the data model names only the six fix-round fields
    decided-at: 2026-11-15T16:33:10Z
    decided-by: rule
  - key: perf — process-decisions, check-review-gate and write-review build their `lists` JSON value (a clone of every list) on each call though bounded_lists::place writes it only past the read size — `runtime/src/primitives/process_decisions.rs:60-67`
    outcome: waived
    reason: the lists are bounded by a spec's decisions, violations and stale paths, a small constant against the file reads each call already makes; no measurement shows a cost, and a lazy builder would add a closure parameter to a shared helper
    decided-at: 2026-11-15T16:33:10Z
    decided-by: rule
  - key: other — `framework/commands/review.md` step 1 (94e5b151) says a zero scope with `deleted-count` above zero is empty 'because every path it would hold was deleted', but `deleted` holds every window deletion whoever's commit made it and `excluded` can also empty the scope, so the sentence and the Summary naming each `deleted` path can mislabel another spec's deletions — `framework/commands/review.md:119`
    outcome: waived
    reason: the scenario's edge case scopes the claim to the spec's own code deletions with review-owed true; the report over-names rather than hides a path, and no gate reads it. The wording fix reopens review.md's generated mirrors and 066 for a low-confidence imprecision
    decided-at: 2026-11-15T16:33:10Z
    decided-by: rule
  - key: convention — AGENTS.md 2604a6f9's two new bullets (a docs/ commit naming no spec; a subagent's refused Write) are true for every project running the pipeline, so §drift-prevention routes them to the constitution, and AGENTS.md sits 233 bytes under its 50,000-byte ceiling — `AGENTS.md:86`
    outcome: waived
    reason: AGENTS.md already carries sibling per-incident bullets of the same kind (the cut-off hand-back and killed-background-agent bullets) and the owner chose this home; the pre-commit hook enforces the ceiling
    decided-at: 2026-11-15T16:33:10Z
    decided-by: rule
  - key: perf — SweepIndex::explained_by re-sorts the repo-wide pair set on every call, once per file-local pair per path, instead of once at build — `runtime/src/primitives/mechanical_sweep.rs:237`
    outcome: waived
    reason: the repo-wide set is small and the window is capped at RENAME_LIMIT paths; no measured cost, and moving the sort changes no verdict
    decided-at: 2026-11-15T16:33:10Z
    decided-by: rule
  - key: "other — the new gate tests prove each of the three containments but not three neighbours: no test holds two scenarios (one linked outside, one linking back inside) to show the verdict does not depend on listing order, which the commit and plan state as the reason every scenario is contained before any is read; none shows a symbolic link that stays inside the project is still read and discharges; none asserts the outside-project guidance clause. A mutant that interleaves containment with the read, or refuses every link, survives — `runtime/src/primitives/check_review_gate.rs:1648-1760`"
    outcome: waived
    reason: each behavior the four tests and four mutants name is killed by reading (M1 to M4 each remove one containment and a named test asserts the state, the detail and the message); the order claim holds by construction of the separate loop, and a test of it would depend on read_dir order the OS controls, so it would fail only some of the time under its mutant. QUAL-TEST-001 judges a test that cannot fail, and none of the five does
    decided-at: 2026-11-15T23:51:40Z
    decided-by: rule
  - key: other — classify_cross_spec_entry's outside closure maps every error validate_contained returns to the outside-project state, including a symbolic-link loop, a lexical refusal of the spec root and an Io error resolving the project root itself, each reported as reads outside the project with the error text as detail — `runtime/src/primitives/check_review_gate.rs:1149-1158`
    outcome: waived
    reason: each such state holds done the same, nothing was read, and the detail carries the true reason; the spec root is validated before the gate runs and the project root exists, so the Io arm is unreachable in practice. A fourth state would add a response value with no distinct remedy
    decided-at: 2026-11-15T23:51:40Z
    decided-by: rule
  - key: other — the gate could have contained the affected spec in one call with validate_feature_path, which walks the directory and refuses on any outward link under it; the commit contains three path kinds by hand instead — `runtime/src/primitives/check_review_gate.rs:1150-1207`
    outcome: waived
    reason: "validate_feature_path refuses the whole call on a link in any file under the directory, including files the gate never reads (research.md, a data model), and returns an error rather than a per-entry state; using it would widen what the gate refuses and lose the outside-project result the spec requires (QUAL-DELEG-001's shape: the shared check enforces more than the local one needed)"
    decided-at: 2026-11-15T23:51:40Z
    decided-by: rule
  - key: perf — each validate_contained call re-resolves the canonicalized project root, so one entry costs two calls plus one per scenario, each a resolve of the root and a symlink_metadata walk of the path — `runtime/src/primitives/check_review_gate.rs:1171-1205`
    outcome: waived
    reason: an affected spec holds tens of scenarios and a spec declares a handful of entries, so the count is a small constant against the file reads the gate already makes for each; no measurement shows a cost and a shared canonical-base parameter would add a second containment entry point to mod.rs
    decided-at: 2026-11-15T23:51:40Z
    decided-by: rule
  - key: other — framework/reference/analyze.md and docs/analyze.md describe the cross-spec-impact-drift family as an entry no back-link discharges and do not name the new link-out message; the state inherited from the gate is a subset of that wording — `framework/reference/analyze.md:343`
    outcome: waived
    reason: the sentence stays true, since an entry read through a link out is one no back-link discharges, and the compiled CHECK_ARTIFACTS description, which is the canonical source for a family's behavior, carries the new message; the fix would land in framework and docs files outside 066's directory for a wording that is not false
    decided-at: 2026-11-15T23:51:40Z
    decided-by: rule
  - key: other — after the guard, write_review.rs and write_analysis.rs no longer call write_atomic, so dropping their two EXEMPT entries is correct and forced (no_exemption_outlives_its_writer would fail otherwise), but the same migration takes every guarded writer out of every_text_writer_accounts_for_line_endings' subject, whose module doc still says write_atomic_bytes callers copy bytes; prune_plan.rs, prune_tasks.rs, invalidate_review.rs and relocate_audit_records.rs already contain no write_atomic( call in their non-test code and so are not examined at all, and a writer of an authored file that moves onto write_if_unchanged without one leaves the lint green and unexamined. — `runtime/tests/line_ending_discipline.rs:37`
    outcome: waived
    reason: the lint and its module doc belong to 051's rewrites-preserve-line-endings scenario, and the class migration that widens the blind spot is 022 task 38's (which adds a test holding raw writes to an allowlist); nothing in 066's two writers is affected, since both render their record whole and carry no authored text to preserve. The fix would land in specs/051-branch-scoped-spec-numbering and specs/022-deterministic-runtime, outside 066.
    decided-at: 2026-11-16T13:45:39Z
    decided-by: rule
  - key: other — review.md step 12 invokes write-review a second time with step 11's arguments, after the whole fix-round loop, which is the longer window in which another writer can change review.md; the changed-since-read handling sentence sits only in step 11, so step 12 meets a refusal with its handling path stated one step earlier. — `framework/commands/review.md:137`
    outcome: waived
    reason: step 12's write is defined as step 11's arguments but for loop-open, and step 11's sentence is scoped to 'a write' of write-review, not to its first call; the operator reads one handling path for both.
    decided-at: 2026-11-16T13:45:39Z
    decided-by: rule
  - key: other — the handling path both commands state re-runs only the write call from the same arguments, so the arguments derived from the earlier read of the record (review step 8's applied and expired waivers and step 9's expired-decisions; analyze step 18's expired-decisions) are reused after another writer changed the record, and a waiver or decision that writer stored is then neither classified nor expired until the next run; a new-waivers item that writer already stored is refused as InvalidArgument rather than ChangedSinceRead. — `framework/commands/analyze.md:93`
    outcome: waived
    reason: BE-TXN-004 accepts reporting the conflict to the caller as a handling path and the sentence says to report it first; 066's plan chooses the re-run from the run's argument files and rejects plumbing the snapshot text into readers, and the next run re-derives every classification from the record as it then stands.
    decided-at: 2026-11-16T13:45:39Z
    decided-by: rule
  - key: "other — plan.md Affected Files omits files f65ef759 changed for task 25: docs/analyze.md, framework/reference/implement.md, runtime/src/primitives/mod.rs, scripts/mutation/reopen-measure.py and 079's spec.md — `specs/066-audit-records-and-the-completion-gate/plan.md`"
    outcome: waived
    reason: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims and is the owner's plan decision, not a mechanical edit; every one of these files is already in the window through the commit that changed it
    decided-at: 2026-11-18T01:05:21Z
    decided-by: rule
  - key: other — the data model's compute-review-scope Size paragraph says performReview's request names every in-scope file whole, but the exec walker's load_scope_files (runtime/src/interpreter/payload.rs) silently skips a scope file that is missing, unreadable or not UTF-8, naming only a path leaving the project in unexaminable — `specs/066-audit-records-and-the-completion-gate/data-model.md`
    outcome: waived
    reason: the code is 022 and 080's exec walker, outside 066's primitives; the sentence cites 022's data model for the claim, a correction lands in 022's directory, and nothing in 066's code depends on it
    decided-at: 2026-11-18T01:05:21Z
    decided-by: rule
  - key: convention — the constitution's §spec-lifecycle case (c), a citation's line number removed, does not say a line number set apart in its own code span is outside it, though 067's check reports one and removing it deletes a span, which reopens a done spec — `framework/constitution.md`
    outcome: waived
    reason: the plan's definition and CLASSIFY_SPEC_EDIT's compiled description state the exclusion, the case is a precision gap rather than a contradiction, and the constitution has one byte of headroom under its 50,000-byte ceiling, so a clarifying rewrite owes a cut elsewhere in a document 050 owns
    decided-at: 2026-11-18T01:05:21Z
    decided-by: rule
  - key: "other: nothing ties the reasons check-artifacts, resolve-anchor, check-rule-ids and the exec walker's tally emit to UNEXAMINED_REASONS; the test holds the list equal to the data model's table only, so a skip reason added to a check and not to the list and table makes every write-analysis that records it fail with a refusal naming the reason (loud, never a silent record). Every reason those sources emit at HEAD is in the list (checked by reading each emitter) — `runtime/src/primitives/write_analysis.rs`"
    outcome: waived
    reason: the refusal is loud and names the reason, so the gap cannot pass as a clean record; docs/developing-ductus.md's closed-set procedure greps a sibling reason such as root-absent across runtime/src and so reaches UNEXAMINED_REASONS; a test enumerating every emitter would copy the checks' reason sets, which this review found equal to the list at HEAD
    decided-at: 2026-11-18T02:04:08Z
    decided-by: rule
  - key: other — a run judging only some criteria records criteria-judged and criteria-met over those alone, and its generated Summary reads 'N of N', with no count of the criteria it left unjudged — `runtime/src/primitives/write_review.rs`
    outcome: waived
    reason: the data model defines criteria-judged as the criteria the run judged and promises no unjudged count; compute-review-scope names each unjudged criterion in criteria-owing and reads review-owed true, so the gate holds done, and the command names a criterion it could not judge in the Summary
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: other — 66203361 (task 27b) also carries 029 task 17's edit to framework/bootstrap/ductus.md and govern.md, dropping the 'after a fallback to tar' clause, which its message does not name — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: another session's staged hunk that rode along through the shared index; docs/developing-ductus.md §Running a second session records the incident, history on main stays as written, and the section reads consistently with 029 task 17
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: bug — criterion_verdicts keeps verdicts by text digest, so two criteria whose text is the same, labels set aside, share one met verdict — `runtime/src/primitives/criterion_verdicts.rs`
    outcome: waived
    reason: "contrived: two criteria with the same text state one requirement, so a verdict judging it holds for both, and the scenario keys a verdict to the criterion's text"
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: other — criterion_verdicts::owing_in reads a spec.md that is not a file as owing nothing, while an unreadable one fails — `runtime/src/primitives/criterion_verdicts.rs`
    outcome: waived
    reason: "unreachable from its callers: compute-review-scope and check-review-gate read the spec before it, so a missing or non-file spec.md fails there first"
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: other — check_review_gate's unticked_criterion_block enumerates and names criteria as criterion_verdicts::owing does, rather than through one shared helper — `runtime/src/primitives/check_review_gate.rs`
    outcome: waived
    reason: the naming (criterion_verdicts::name) and the checkbox parse are already shared; what repeats is a loop of a few lines, and the two select different criteria, unticked against unjudged
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: other — on the exec path mark_each_verdict returns the first mark-criterion error, a changed-since-read refusal included, so the walk halts with earlier verdicts ticked and no record written, where the host path reports a refusal beside its verdict and goes on — `runtime/src/interpreter/mod.rs`
    outcome: waived
    reason: the walker halts on any primitive error by design; the refused criterion is left as it was and no record is written, so the review stays owed and compute-review-scope names every criterion not judged met, the scenario's outcome
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: other — docs/developing-ductus.md §Sizing a review's parts calls the nine loaded rule files backend and gives 285,666 bytes, against 285,795 at HEAD with three of them cross files — `docs/developing-ductus.md`
    outcome: waived
    reason: a dated measurement of 2026-11-16 and 2026-11-17, outside efd3d488's hunks, and the drift changes no sizing rule the paragraph states
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: other — plan.md's Affected Files omits files task 27's commits changed, among them criterion_verdicts.rs, invalidate_review.rs, main.rs, mod.rs, tests/mcp.rs, tests/review_command.rs, the fixtures, framework/reference/audit.md, docs/slash-commands.md and four mutation lists — `specs/066-audit-records-and-the-completion-gate/plan.md`
    outcome: waived
    reason: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims, the owner's plan decision rather than a mechanical edit; each is already in the window through the commit naming 066 that changed it
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: other — 000's criterion-route-after-draft Resolved Question says reopening routes a criterion into implement's criterion-verification step — `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
    outcome: waived
    reason: a Resolved Question dated 2026-08-17 recording its reasoning as of then, quoting the constitution as it read, exemption (c) included; the decision it records, reopening, still holds and now routes the criterion to review's criteria step, and 9f5a0fcb swept the scenario's Context and Behavior, which state current behavior
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
---

# Review — 066-audit-records-and-the-completion-gate

## Summary

066's review of task 27, the criteria step's first run: the window from 78fd056a (base-source reopen) to efd3d488, whose 066 commits are cbffcddb to 24d9bbea, ce10acac and efd3d488. Five independent agents divided it by behavior. Three ran all five passes, over the record (27a), what owes a review and the gate (27b, 27c), and the commands, exec walker and sweep (27d, 27e), each reading the nine loaded backend and cross rule files whole (285,795 bytes; slice B read seven of them on a second request). One judged AC1 to AC6 against their code wherever it lies, running their tests and driving the release binary on fixtures. One checked 066's data model, scenario, plan, spec and tasks whole against the code. 23 of the 64 scope entries were read whole and 41 are unread: 17 at every 066 hunk with each item or section enclosing one (write_review.rs, compute_review_scope.rs, check_review_gate.rs, interpreter/mod.rs, payload.rs, extensions.rs, main.rs, primitives/mod.rs, schema/primitives.rs, descriptions.rs, tests/mcp.rs, reopen-measure.py, scope-attribution.py, the constitution, AGENTS.md, the audit reference and framework/bootstrap/ductus.md); 10 mirrors, the nine generated copies of the review, implement, amend and audit sources, which gen-claude-commands.sh --check reports in sync after this run's edits, and framework/bootstrap/govern.md, byte-identical to ductus.md by cmp; 7 uncommitted entries of another session's 029 and 048 work (two workflows, 029's spec and record, 048's spec, plan and data model), not read; and 7 of this run's own fixes that joined the scope (amend.md, runtime/CHANGELOG.md, line_citations.rs, whose text before the fix the criteria agent read whole, both scripts/audit files, line-citation-strip-066.py and 000's spec.md), which the fix round re-reads. Every pass ran. One finding, QUAL-TEST-001 (should, low confidence): an_unreadable_spec_is_an_error_not_a_spec_owing_nothing skips its assertion when the process can read a mode-000 file, as under root. The criteria step judged all six criteria met, AC6 only after a fix: line_citations took a range anchor's second L as optional, so stripping #L12-20 read mechanical, which AC6 and the seven other sources naming #LN-LM do not allow; the owner chose to require the L (2026-11-18), and on a fixture stripping #L12-20 now reopens a done spec. 48 observations. 25 were fixed in the run, among them the broken contract that failed CI run 44423820535 on every platform (review-basic.jsonl was blessed before ce10acac edited review.md, moving its fixture's diff-base; re-blessed, only diff-base changing), tests and mutants M22 to M27 for five task-27 behaviors no mutant covered, the gate's guidance offering a waiver for a not-met criterion (M28), changelog entries for tasks 26 and 27, and stale claims of an implement-side criteria check in amend.md, the audit script and its README, the review reference, 000's spec and 066's plan, spec and scenario. 9 were waived by the rule with their reasons, and 14 matched stored decisions and were carried, each issue still standing at HEAD. 16 stored decisions were retained, their files unread by this run. After the fixes, cargo fmt --check, clippy -D warnings and the whole cargo test --release --locked (3,623 passed, 0 failed) ran from a worktree of HEAD holding this run's diff alone, and all 28 mutants of criteria-in-review-066.py and all 23 of line-citation-strip-066.py were killed. Fix round 1's independent re-read read every path its brief names, another session's 029 record and the golden's changed lines whole on a second request, and found nothing; round 2 was empty.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

6 of 6 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-TEST-001 — an_unreadable_spec_is_an_error_not_a_spec_owing_nothing branches its assertion away when the process can read a mode-000 file

- **File**: `runtime/src/primitives/criterion_verdicts.rs:136-154`
- **Rule**: A test SHOULD fail when the behavior it names is removed or broken. A test that passes whether or not the code under test behaves as named SHOULD NOT be added or counted as coverage.
- **Finding**: The test sets mode 0o000 and asserts owing_in errors only inside `if std::fs::read(&spec).is_err()`; under a root user (or a file system ignoring modes) the assertion is skipped and the test passes with the unreadable-is-an-error behavior removed (mutant 'an unreadable spec owes nothing' is killed as uid 501 but could not be in a root run). The comment records the branch; the remaining assertion (absent file owes nothing) holds in either case. Low confidence: the suite's known runs are non-root.
- **Auto-fixable**: no
- **Suggested fix**: Make the unreadable file independent of the user, for example a directory named spec.md (read_text then fails for every user), or assert the not-root condition and skip loudly instead of passing.

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: write_analysis.rs run() derives blocking and refuses unparseable spec frontmatter (driven: YAML parse error, no analysis.md written); check_review_gate.rs run_with_lint orders review checks before analyze_half_block (NotAnalyzed, AnalyzeFindings); cargo test --lib check_review_gate (95 pass) and write_analysis (100 pass), incl. blocking_is_set_by_either_gating_tier, malformed_frontmatter_is_never_given_a_clean_record, a_reviewed_but_unanalyzed_spec_is_blocked, a_missing_review_is_reported_before_a_missing_analysis
- AC2 — met: write_analysis.rs UNEXAMINED_REASONS + refuse_unknown_reasons, by_reason fold with unexamined = sum, omitted when empty (render), schema/primitives.rs parse_reason_count rejects malformed pairs; driven on CLI (bad pair, bad count, bogus reason refused with nothing written; 2+3 gave unexamined 5 returned; clean run writes no map); tests a_reason_outside_the_closed_set_is_refused, the_reason_list_is_the_data_models_table, a_bare_total_survives_a_derived_reason_beside_it
- AC3 — met: write_review.rs render_contracts always writes reviewed-digest (empty map included) over is_review_contract; analyze_subjects.rs freshness_of/exempt_renames keep reviewed-against for the sweep exemption only and keep candidates when trees are unavailable; no digest is Undeterminable; compute_review_scope.rs contracts_against_digest uses it; the gate (check_review_gate.rs) neither compares it nor names an unjudged review in passing_notices (superseded clause holds); tests a_spec_with_no_durable_contracts_records_an_empty_digest, an_empty_contract_digest_is_current_but_an_absent_one_is_not, a_stale_digest_puts_its_contract_in_scope_and_owes_no_review, a_review_record_with_no_digest_raises_no_review_notice
- AC4 — met: check_review_gate.rs already_done_block (status == "done" exactly) runs before markdown_lint_block, returns AlreadyDone with passed false; driven: done gave blocked-by already-done, status wibble fell through to not-reviewed; tests a_done_spec_short_circuits_without_claiming_to_have_passed, a_done_spec_is_not_held_against_a_later_lint_violation, an_unrecognized_status_is_not_treated_as_done
- AC5 — met: check_review_gate.rs cross_spec_impact_states/classify_cross_spec_entry/links_back/cross_spec_impact_block, ordered after scenario_question_block and before load_review_record, states carried onto the pass; driven: discharged, undischarged, target-missing, self-reference and a symlinked spec.md leaving the project reported outside-project with a block; tests a_blockquoted_signpost_discharges_the_obligation, a_spec_md_linked_outside_the_project_is_reported_and_never_read, a_scenario_name_that_is_not_utf8_has_no_contained_spelling, an_undischarged_impact_outranks_a_missing_review
- AC6 — met: line_citations.rs find/strips_to (:N, :N-M with a hyphen or an en dash, :N:C, #LN, #LN-LM, the range's second L required since this run's fix, so stripping #L12-20 is no strip: a_range_anchor_needs_its_second_l) is the one rule mechanical_sweep.rs, classify_spec_edit.rs and analyze_subjects.rs freshness_of use for both records; a token needs an extension, a listed one with no slash, so a host:port is no citation; constitution.md §spec-lifecycle (c) states it; tests a_strip_only_commit_leaves_the_records_current, a_citation_stripped_from_a_criterion_is_a_correction, a_strip_beside_another_change_is_judged_on_that_change, line_citations (12 pass), mechanical_sweep_parity (5 pass); driven on a fixture with the fixed build: stripping #L12-20 reopens a done spec, and #L12-L20, #L12 and :12–20 strips read mechanical

## Observations

- convention — WRITE_REVIEW said an empty scope's report is blocking:false, though a not-met criterion blocks it (an_empty_scope_reads_blocking_from_a_not_met_criterion) — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- convention — WRITE_REVIEW's list of the CLI's --{name}-json arguments left out criteria, which task 27a added as --criteria-json — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- other — three behaviors task 27a added had no test its mutant turned red: the criteria section's place after the deferred findings, the quoted #N name of an unlabelled not-met criterion (bare, a YAML comment), and the criteria sentence on a supplied Summary; two tests and mutants M22 to M24 added — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- other — a run judging only some criteria records criteria-judged and criteria-met over those alone, and its generated Summary reads 'N of N', with no count of the criteria it left unjudged — `runtime/src/primitives/write_review.rs` — **waived**: the data model defines criteria-judged as the criteria the run judged and promises no unjudged count; compute-review-scope names each unjudged criterion in criteria-owing and reads review-owed true, so the gate holds done, and the command names a criterion it could not judge in the Summary (decided by the rule)
- other — task 27's breaking changes, which the plan's BE-COMPAT-001 paragraph classifies, had no runtime/CHANGELOG.md [Unreleased] entry, nor had task 26's; both added — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- other — criteria-owing's file form had no test, since the only file-form test's spec holds no criterion, so leaving the list out of the lists file, or inline, survived; a test and mutants M25 and M26 added — `runtime/src/primitives/compute_review_scope.rs` — **fixed** (decided by the rule)
- other — review_owed_block's arm naming both the changed paths and the owing criteria had no test; a test and mutant M27 added — `runtime/src/primitives/check_review_gate.rs` — **fixed** (decided by the rule)
- bug — must_violations_block's guidance for a record blocking only through a not-met criterion offered a waiver and a deferral, then said neither releases a criterion; it now matches on the three cases its message does, with mutant M28 — `runtime/src/primitives/check_review_gate.rs` — **fixed** (decided by the rule)
- convention — must_violations_block carried two 'Gate check 5:' doc comments, the second appended by e0de7982; merged into one — `runtime/src/primitives/check_review_gate.rs` — **fixed** (decided by the rule)
- other — 66203361 (task 27b) also carries 029 task 17's edit to framework/bootstrap/ductus.md and govern.md, dropping the 'after a fallback to tar' clause, which its message does not name — `framework/bootstrap/ductus.md` — **waived**: another session's staged hunk that rode along through the shared index; docs/developing-ductus.md §Running a second session records the incident, history on main stays as written, and the section reads consistently with 029 task 17 (decided by the rule)
- bug — criterion_verdicts keeps verdicts by text digest, so two criteria whose text is the same, labels set aside, share one met verdict — `runtime/src/primitives/criterion_verdicts.rs` — **waived**: contrived: two criteria with the same text state one requirement, so a verdict judging it holds for both, and the scenario keys a verdict to the criterion's text (decided by the rule)
- other — criterion_verdicts::owing_in reads a spec.md that is not a file as owing nothing, while an unreadable one fails — `runtime/src/primitives/criterion_verdicts.rs` — **waived**: unreachable from its callers: compute-review-scope and check-review-gate read the spec before it, so a missing or non-file spec.md fails there first (decided by the rule)
- other — check_review_gate's unticked_criterion_block enumerates and names criteria as criterion_verdicts::owing does, rather than through one shared helper — `runtime/src/primitives/check_review_gate.rs` — **waived**: the naming (criterion_verdicts::name) and the checkbox parse are already shared; what repeats is a loop of a few lines, and the two select different criteria, unticked against unjudged (decided by the rule)
- bug — at HEAD cargo test --test parity failed review_basic_stream_matches_golden, on all three CI platforms of run 44423820535: ce10acac edited framework/commands/review.md after e0de7982 blessed runtime/tests/golden/review-basic.jsonl, moving the fixture repo's diff-base sha; the golden is re-blessed — `runtime/tests/golden/review-basic.jsonl` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — framework/commands/amend.md's criterion route still said implement's completion gate verifies a criterion, and its report that implement verifies a reopened spec; both now name review — `framework/commands/amend.md` — **fixed** (decided by the rule)
- convention — scripts/audit/done-spec-criteria.sh's header and scripts/audit/README.md's Family 27 entry still said implement.md marks each verified criterion; 9f5a0fcb swept only framework/reference/audit.md — `scripts/audit/done-spec-criteria.sh` — **fixed** (decided by the rule)
- convention — framework/reference/review.md's §Resolve target and scope item 3 and §Record the run said an empty scope records blocking: false and blocking is MUST-only, though a not-met criterion blocks — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention — framework/reference/review.md's report template left out the ## Acceptance criteria section write-review renders after the deferred findings — `framework/reference/review.md` — **fixed** (decided by the rule)
- other — the criteria step did not say a verdict's note is one line, which write-review otherwise refuses before writing — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention — a payload.rs test comment still named the completion gate's read step as the source of the verifyCriteria request's criteria — `runtime/src/interpreter/payload.rs` — **fixed** (decided by the rule)
- other — on the exec path mark_each_verdict returns the first mark-criterion error, a changed-since-read refusal included, so the walk halts with earlier verdicts ticked and no record written, where the host path reports a refusal beside its verdict and goes on — `runtime/src/interpreter/mod.rs` — **waived**: the walker halts on any primitive error by design; the refused criterion is left as it was and no record is written, so the review stays owed and compute-review-scope names every criterion not judged met, the scenario's outcome (decided by the rule)
- other — docs/developing-ductus.md §Sizing a review's parts calls the nine loaded rule files backend and gives 285,666 bytes, against 285,795 at HEAD with three of them cross files — `docs/developing-ductus.md` — **waived**: a dated measurement of 2026-11-16 and 2026-11-17, outside efd3d488's hunks, and the drift changes no sizing rule the paragraph states (decided by the rule)
- convention — a write_review.rs test doc comment said the gate reads an empty reviewed-digest as current, though the gate no longer compares the digest (AC3's superseded clause) — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- bug — line_citations' anchor_suffix_end took a range anchor's second L as optional, so stripping #L12-20 read mechanical, which AC6 and the seven other sources naming only #LN-LM do not allow; the L is now required (owner decision, 2026-11-18), with a test and mutant M23 of line-citation-strip-066.py — `runtime/src/primitives/line_citations.rs` — **fixed**
- other — plan.md's Affected Files omits files task 27's commits changed, among them criterion_verdicts.rs, invalidate_review.rs, main.rs, mod.rs, tests/mcp.rs, tests/review_command.rs, the fixtures, framework/reference/audit.md, docs/slash-commands.md and four mutation lists — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims, the owner's plan decision rather than a mechanical edit; each is already in the window through the commit naming 066 that changed it (decided by the rule)
- other — plan.md said the exec walker's mark-criterion binding already read verifyCriteria's verdicts that way, though 27d added mark_each_verdict and write-review's criteria binding — `specs/066-audit-records-and-the-completion-gate/plan.md` — **fixed** (decided by the rule)
- convention — plan.md's Affected Files row for runtime/src/interpreter/mod.rs described a docs change, where the change is the verdict fan-out and the criteria binding — `specs/066-audit-records-and-the-completion-gate/plan.md` — **fixed** (decided by the rule)
- other — plan.md stated '24 live claims' of five passes with no method, where a grep at HEAD returns 27 lines; the count is dropped — `specs/066-audit-records-and-the-completion-gate/plan.md` — **fixed** (decided by the rule)
- convention — spec.md's The completion gate section listed the gate's checks without the unchecked-task and unticked-criterion checks — `specs/066-audit-records-and-the-completion-gate/spec.md` — **fixed** (decided by the rule)
- convention — spec.md's The review record section did not name the criteria step's four record fields — `specs/066-audit-records-and-the-completion-gate/spec.md` — **fixed** (decided by the rule)
- convention — ComputeReviewScopeResult's lists_file doc comment named five lists, leaving out criteria-owing — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention — the criteria-are-verified-in-review scenario cited 'this spec's task 9', pruned in a70ff74c, beside the scenario that already carries the point — `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md` — **fixed** (decided by the rule)
- convention — 000's spec.md command list still said implement verifies acceptance criteria — `specs/000-slash-commands/spec.md` — **fixed** (decided by the rule)
- other — 000's criterion-route-after-draft Resolved Question says reopening routes a criterion into implement's criterion-verification step — `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — **waived**: a Resolved Question dated 2026-08-17 recording its reasoning as of then, quoting the constitution as it read, exemption (c) included; the decision it records, reopening, still holds and now routes the criterion to review's criteria step, and 9f5a0fcb swept the scenario's Context and Behavior, which state current behavior (decided by the rule)
- other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md` — **waived**: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
- other: the implement reference's "non-blocking" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md` — **waived**: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
- edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md` — **waived**: contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true
- other — invalidate-review's SCALARS list drops the run's fix-round fields but leaves `open-findings` and `unread` (079/081 fields) in a record it resets to `last-run: null` (verified on a fixture built under the scratchpad: both survive). compute-review-scope reads `open_findings` from the HEAD record without requiring last-run, so an invalidated record can still name paths owing a review; `unread` is credited only when last-run is set, so it is inert. — `runtime/src/primitives/invalidate_review.rs:112-137` — **waived**: open-findings left on an invalidated record keep the paths the invalidated run left unresolved in the next review's scope, which is conservative; no gate reads either field when last-run is null, and the data model names only the six fix-round fields (decided by the rule)
- other — `framework/commands/review.md` step 1 (94e5b151) says a zero scope with `deleted-count` above zero is empty 'because every path it would hold was deleted', but `deleted` holds every window deletion whoever's commit made it and `excluded` can also empty the scope, so the sentence and the Summary naming each `deleted` path can mislabel another spec's deletions — `framework/commands/review.md:119` — **waived**: the scenario's edge case scopes the claim to the spec's own code deletions with review-owed true; the report over-names rather than hides a path, and no gate reads it. The wording fix reopens review.md's generated mirrors and 066 for a low-confidence imprecision (decided by the rule)
- convention — AGENTS.md 2604a6f9's two new bullets (a docs/ commit naming no spec; a subagent's refused Write) are true for every project running the pipeline, so §drift-prevention routes them to the constitution, and AGENTS.md sits 233 bytes under its 50,000-byte ceiling — `AGENTS.md:86` — **waived**: AGENTS.md already carries sibling per-incident bullets of the same kind (the cut-off hand-back and killed-background-agent bullets) and the owner chose this home; the pre-commit hook enforces the ceiling (decided by the rule)
- other — the new gate tests prove each of the three containments but not three neighbours: no test holds two scenarios (one linked outside, one linking back inside) to show the verdict does not depend on listing order, which the commit and plan state as the reason every scenario is contained before any is read; none shows a symbolic link that stays inside the project is still read and discharges; none asserts the outside-project guidance clause. A mutant that interleaves containment with the read, or refuses every link, survives — `runtime/src/primitives/check_review_gate.rs:1648-1760` — **waived**: each behavior the four tests and four mutants name is killed by reading (M1 to M4 each remove one containment and a named test asserts the state, the detail and the message); the order claim holds by construction of the separate loop, and a test of it would depend on read_dir order the OS controls, so it would fail only some of the time under its mutant. QUAL-TEST-001 judges a test that cannot fail, and none of the five does (decided by the rule)
- other — classify_cross_spec_entry's outside closure maps every error validate_contained returns to the outside-project state, including a symbolic-link loop, a lexical refusal of the spec root and an Io error resolving the project root itself, each reported as reads outside the project with the error text as detail — `runtime/src/primitives/check_review_gate.rs:1149-1158` — **waived**: each such state holds done the same, nothing was read, and the detail carries the true reason; the spec root is validated before the gate runs and the project root exists, so the Io arm is unreachable in practice. A fourth state would add a response value with no distinct remedy (decided by the rule)
- other — the gate could have contained the affected spec in one call with validate_feature_path, which walks the directory and refuses on any outward link under it; the commit contains three path kinds by hand instead — `runtime/src/primitives/check_review_gate.rs:1150-1207` — **waived**: validate_feature_path refuses the whole call on a link in any file under the directory, including files the gate never reads (research.md, a data model), and returns an error rather than a per-entry state; using it would widen what the gate refuses and lose the outside-project result the spec requires (QUAL-DELEG-001's shape: the shared check enforces more than the local one needed) (decided by the rule)
- perf — each validate_contained call re-resolves the canonicalized project root, so one entry costs two calls plus one per scenario, each a resolve of the root and a symlink_metadata walk of the path — `runtime/src/primitives/check_review_gate.rs:1171-1205` — **waived**: an affected spec holds tens of scenarios and a spec declares a handful of entries, so the count is a small constant against the file reads the gate already makes for each; no measurement shows a cost and a shared canonical-base parameter would add a second containment entry point to mod.rs (decided by the rule)
- other — review.md step 12 invokes write-review a second time with step 11's arguments, after the whole fix-round loop, which is the longer window in which another writer can change review.md; the changed-since-read handling sentence sits only in step 11, so step 12 meets a refusal with its handling path stated one step earlier. — `framework/commands/review.md:137` — **waived**: step 12's write is defined as step 11's arguments but for loop-open, and step 11's sentence is scoped to 'a write' of write-review, not to its first call; the operator reads one handling path for both. (decided by the rule)
- other — plan.md Affected Files omits files f65ef759 changed for task 25: docs/analyze.md, framework/reference/implement.md, runtime/src/primitives/mod.rs, scripts/mutation/reopen-measure.py and 079's spec.md — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims and is the owner's plan decision, not a mechanical edit; every one of these files is already in the window through the commit that changed it (decided by the rule)
- other — the data model's compute-review-scope Size paragraph says performReview's request names every in-scope file whole, but the exec walker's load_scope_files (runtime/src/interpreter/payload.rs) silently skips a scope file that is missing, unreadable or not UTF-8, naming only a path leaving the project in unexaminable — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **waived**: the code is 022 and 080's exec walker, outside 066's primitives; the sentence cites 022's data model for the claim, a correction lands in 022's directory, and nothing in 066's code depends on it (decided by the rule)
- convention — the constitution's §spec-lifecycle case (c), a citation's line number removed, does not say a line number set apart in its own code span is outside it, though 067's check reports one and removing it deletes a span, which reopens a done spec — `framework/constitution.md` — **waived**: the plan's definition and CLASSIFY_SPEC_EDIT's compiled description state the exclusion, the case is a precision gap rather than a contradiction, and the constitution has one byte of headroom under its 50,000-byte ceiling, so a clarifying rewrite owes a cut elsewhere in a document 050 owns (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/bootstrap/ductus-procedure.md` — 027-bootstrap-migration-registry
- `scripts/audit/migration-coverage.sh` — 027-bootstrap-migration-registry
- `scripts/tests/test-migration-coverage.sh` — 027-bootstrap-migration-registry
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule
- `specs/000-slash-commands/scenarios/validation-gates.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule
- `specs/007-govern-workflow/plan.md` — 007-govern-workflow
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/020-code-review/data-model.md` — 020-code-review
- `specs/020-code-review/spec.md` — 020-code-review
- `specs/022-deterministic-runtime/data-model.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule
- `specs/026-framework-self-audit/analysis.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-10-migration-coverage.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit
- `specs/027-bootstrap-migration-registry/plan.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/spec.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/056-bootstrap-archive-boundary-split/analysis.md` — 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/spec.md` — 056-bootstrap-archive-boundary-split
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule
- `specs/079-review-and-analysis-work-converges/spec.md` — 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/spec.md` — 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed

## Unexamined governance

*None.*
