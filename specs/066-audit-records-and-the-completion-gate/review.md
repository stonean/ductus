---
spec: 066-audit-records-and-the-completion-gate
last-run: 2026-11-19T22:36:00Z
reviewed-against: b2421e14591b4065b58696417e4a74b02cab4be0
diff-base: bb47ec0f0e5c7b645514b06083169606882b12d8
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 7
unread: []
excluded: 144
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
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
  scenarios/record-paths-are-quoted-only-when-they-need-it.md: 729b1dccb47871d2a8f4fe1dc7e0cbe1449d92eb8c28004a1490859e88ff491d
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
  fixed: 0
  deferred: 0
  waived: 3
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
  - key: other — the task 22 CHANGELOG entry's new BREAKING sentence names `matched`, `expired`, `retained` and `notices` as absent inline past the read size, which holds (process_decisions::run sets each None when bounded_lists::place returns a file), but not that in the inline form `notices` now carries one summary line per reason where 0.59.11 carried one line per retained decision; the entry's earlier sentence states it, so the BREAKING label is already earned and nothing is wrong — `runtime/CHANGELOG.md`
    outcome: waived
    reason: The changed content of `notices` is stated two sentences earlier in the same entry, and the entry is already labelled BREAKING; repeating it adds no claim.
    decided-at: 2026-11-19T01:12:39Z
    decided-by: rule
  - key: the `yaml_string` doc comment says it is used for every string value in the waiver and decision lists and does not name the record paths it now quotes — `runtime/src/primitives/write_review.rs`
    outcome: waived
    reason: "the stale clause is incomplete, not false: ordinary paths stay unquoted as it says, and fixing a doc comment in `runtime/src/primitives/write_review.rs`, also in 079's window, costs a landing and a re-read round for no behavior"
    decided-at: 2026-11-19T22:36:00Z
    decided-by: rule
  - key: a private `yaml_quoted` in `runtime/src/primitives/create_scenario.rs` quotes like the shared one but lacks the line-hazard escape; pre-existing — `runtime/src/primitives/create_scenario.rs`
    outcome: waived
    reason: pre-existing and outside this change's files; a private quoting helper in another primitive is not what the scenario governs
    decided-at: 2026-11-19T22:36:00Z
    decided-by: rule
  - key: "`render_rule_fields` in `runtime/src/primitives/write_analysis.rs` flattens a rule file name through `single_line` before `yaml_string` for `rule-digest` keys, a map the scenario does not name — `runtime/src/primitives/write_analysis.rs`"
    outcome: waived
    reason: rule-digest keys are rule file names, a map the scenario does not name, and flattening them predates this change and loses nothing a path needs
    decided-at: 2026-11-19T22:36:00Z
    decided-by: rule
---

# Review — 066-audit-records-and-the-completion-gate

## Summary

066's third review, of the window from bb47ec0f (base-source reopen, the owner-approved reopen for the quoting scenario) holding seven entries: the CHANGELOG, write_analysis.rs, write_review.rs, the quoting mutation list, the scenario, spec.md and tasks.md. Three readers ran from one shared brief: A the security pass over the three security rule files (194,749 bytes, read whole), B the reuse, quality, efficiency and simplicity passes over the other nine rule files (213,777 bytes, read whole), AGENTS.md's Gotchas to Design Principles and specs/system.md, with a verdict on each stored decision naming an in-scope file, and C the criteria step. Each read the diff files whole (scope 19,540 bytes, CHANGELOG 16,272 bytes, B the CHANGELOG entries in the tree) and the two large Rust files at their hunks and the quoting functions, not whole. The 079 hunk in write_review.rs (the excluded section lists paths only) was read as code in the file. No finding. Three observations, waived by the rule with their reasons. The three stored decisions naming in-scope files (write_analysis.rs, write_review.rs, runtime/CHANGELOG.md) still hold and are carried. The criteria step judged AC1 to AC6 met against the tree and drove their tests (2969 lib tests under primitives pass); their ticks already matched, so no mark-criterion call changed a box. The mutation list scripts/mutation/record-path-quoting-066.py ran 5 mutants, 5 killed. Nothing was fixed, so no fix round ran.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

6 of 6 acceptance criteria judged met.

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

- AC1 — met: `write_analysis.rs` derives blocking and refuses unparseable frontmatter; `check_review_gate.rs` ReviewGateBlock NotAnalyzed/AnalyzeFindings; tests blocking_is_set_by_either_gating_tier, blocking_analyze_findings_block_and_name_both_tiers pass
- AC2 — met: `write_analysis.rs` folds unexamined_by_reason into the total over a closed set; `schema/primitives.rs` parse_reason_count; tests a_reason_outside_the_closed_set_is_refused, the_breakdown_is_written_and_sorted pass
- AC3 — met: `write_review.rs` render_contracts writes reviewed-digest; tests the_review_record_digests_its_durable_contracts, a_contract_change_since_the_review_holds_done_only_on_the_analysis pass; this window's quoting keeps ordinary keys byte-identical
- AC4 — met: `check_review_gate.rs` short-circuits on exactly done first; tests a_done_spec_short_circuits_without_claiming_to_have_passed, a_done_spec_is_not_held_against_a_later_lint_violation pass
- AC5 — met: `check_review_gate.rs` cross-spec-impact check; tests an_undischarged_impact_outranks_a_missing_review, a_blockquoted_signpost_discharges_the_obligation pass
- AC6 — met: `line_citations.rs` strips_to shared by `classify_spec_edit.rs` and `mechanical_sweep.rs`; tests a_citation_stripped_from_a_scenario_is_mechanical, a_strip_only_commit_leaves_the_records_current pass

## Observations

- the `yaml_string` doc comment says it is used for every string value in the waiver and decision lists and does not name the record paths it now quotes — `runtime/src/primitives/write_review.rs` — **waived**: the stale clause is incomplete, not false: ordinary paths stay unquoted as it says, and fixing a doc comment in `runtime/src/primitives/write_review.rs`, also in 079's window, costs a landing and a re-read round for no behavior (decided by the rule)
- a private `yaml_quoted` in `runtime/src/primitives/create_scenario.rs` quotes like the shared one but lacks the line-hazard escape; pre-existing — `runtime/src/primitives/create_scenario.rs` — **waived**: pre-existing and outside this change's files; a private quoting helper in another primitive is not what the scenario governs (decided by the rule)
- `render_rule_fields` in `runtime/src/primitives/write_analysis.rs` flattens a rule file name through `single_line` before `yaml_string` for `rule-digest` keys, a map the scenario does not name — `runtime/src/primitives/write_analysis.rs` — **waived**: rule-digest keys are rule file names, a map the scenario does not name, and flattening them predates this change and loses nothing a path needs (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/clarify.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/plan.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.claude/commands/ductus/target.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/consolidate.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/review.md`
- `.ductus/reference/specify.md`
- `.ductus/reference/status.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-clarify.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-plan.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `.pi/prompts/ductus-target.md`
- `framework/commands/analyze.md`
- `framework/commands/clarify.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/help.md`
- `framework/commands/implement.md`
- `framework/commands/plan.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/commands/target.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/consolidate.md`
- `framework/reference/fold.md`
- `framework/reference/plan.md`
- `framework/reference/review.md`
- `framework/reference/specify.md`
- `framework/reference/status.md`
- `runtime/Cargo.toml`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/io.rs`
- `runtime/src/main.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_promotion_coverage.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/classify_spec_edit.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_tasks.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/rewrite_spec_links.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/exec_stderr_gone.rs`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/plan-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `scripts/audit/README.md`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/installed-file-modes.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/prune-open-region.py`
- `scripts/mutation/prune-review-fixes-041.py`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/review-022-task-44.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/rewrite-spec-links.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/spec.md`
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md`
- `specs/022-deterministic-runtime/scenarios/append-inbox-comment-aware-write.md`
- `specs/022-deterministic-runtime/scenarios/host-protocol-conformance.md`
- `specs/022-deterministic-runtime/scenarios/installed-files-are-not-group-writable.md`
- `specs/022-deterministic-runtime/scenarios/skipscanner-inline-code-exemption.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/scenarios/an-open-fenced-block-or-comment-is-not-examined.md`
- `specs/041-task-pruning/spec.md`
- `specs/041-task-pruning/tasks.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-definition-and-a-title-follow-the-link-grammar.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/055-shared-constitution/plan.md`
- `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-plan-heading-is-matched-without-regard-to-case.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphan-check-skips-ignored-local-state.md`
- `specs/067-analyze-and-audit-checks/scenarios/promotion-coverage-header-without-a-key.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/a-step-gates-on-a-bounded-list-by-its-count.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
