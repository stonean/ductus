---
spec: 066-audit-records-and-the-completion-gate
last-run: 2026-11-19T01:12:39Z
reviewed-against: 14713cefea941f002828a2806c42e5fc4b0fcc6f
diff-base: d10c2c45063cd2a55fe0020691fe01dfbb7468d9
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 3
unread:
  - runtime/CHANGELOG.md
excluded: 12
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
---

# Review — 066-audit-records-and-the-completion-gate

## Summary

066's second review, of the window from d10c2c45 (base-source last-review, the first review's record) holding three uncommitted entries: runtime/CHANGELOG.md, whose four 066 entries for tasks 7, 14, 20 and 22 gain the BREAKING label and a sentence naming what is breaking against 0.59.11, and the plan's new BE-COMPAT-001 paragraph and grounding citations in the plan and spec, both made for 066's analysis. One agent that held the nine loaded backend and cross rule files read whole (285,795 bytes) ran all five passes over them, and read plan.md and spec.md whole and each CHANGELOG hunk with its whole entry; runtime/CHANGELOG.md is unread, read at its hunks only. It checked every claim the new text makes against the tree: each list named as inline at ductus-v0.59.11 against that tag's result structs, each commit and task named, the attributions to 080's task 5 and 073's task 14, and each symbol and test the citations name; the exec walker's reasons at 680e4739 were checked by the host in analyze_tally.rs. No finding. One observation, waived by the rule with its reason, and the two stored decisions on plan.md's Affected Files carried, each still standing. The criteria step judged AC1 to AC6 again, each met against the tree after d10c2c45, their text unchanged, the tests proving each re-run. Nothing was fixed, so no fix round ran.

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

- AC1 — met: write_analysis.rs run() derives blocking, refuses unparseable spec frontmatter; check_review_gate.rs run_with_lint puts every review check before analyze_half_block (NotAnalyzed, AnalyzeFindings); cargo test --lib check_review_gate (96) and write_analysis (100) pass; driven on the binary
- AC2 — met: write_analysis.rs UNEXAMINED_REASONS/refuse_unknown_reasons and the by_reason sum, schema/primitives.rs parse_reason_count; driven (2+3 gave unexamined 5, bogus reason and x=y refused); tests a_reason_outside_the_closed_set_is_refused, the_reason_list_is_the_data_models_table pass
- AC3 — met: write_review.rs render_contracts always writes reviewed-digest; analyze_subjects.rs freshness_of/exempt_renames, compute_review_scope.rs contracts_against_digest; the gate no longer compares it; tests an_empty_contract_digest_is_current_but_an_absent_one_is_not, a_stale_digest_puts_its_contract_in_scope_and_owes_no_review pass (write_review 156, compute_review_scope 134)
- AC4 — met: check_review_gate.rs already_done_block runs before markdown_lint_block on the exact value done; driven (done gave already-done); test a_done_spec_is_not_held_against_a_later_lint_violation passes
- AC5 — met: check_review_gate.rs cross_spec_impact_states/cross_spec_impact_block ordered after scenario_question_block and before the review record; driven (discharged/undischarged/target-missing/self-reference/outside-project); tests a_blockquoted_signpost_discharges_the_obligation, a_spec_md_linked_outside_the_project_is_reported_and_never_read pass; d10c2c45's must_violations_block change touches only the review half, which AC5 does not claim
- AC6 — met: line_citations.rs strips_to with anchor_suffix_end now requiring -L so only #LN and #LN-LM are strips, matching the criterion exactly, shared by mechanical_sweep.rs, classify_spec_edit.rs and analyze_subjects.rs; tests a_range_anchor_needs_its_second_l (line_citations 12 pass), a_strip_only_commit_leaves_the_records_current, mechanical_sweep_parity (5) pass

## Observations

- other — the task 22 CHANGELOG entry's new BREAKING sentence names `matched`, `expired`, `retained` and `notices` as absent inline past the read size, which holds (process_decisions::run sets each None when bounded_lists::place returns a file), but not that in the inline form `notices` now carries one summary line per reason where 0.59.11 carried one line per retained decision; the entry's earlier sentence states it, so the BREAKING label is already earned and nothing is wrong — `runtime/CHANGELOG.md` — **waived**: The changed content of `notices` is stated two sentences earlier in the same entry, and the entry is already labelled BREAKING; repeating it adds no claim. (decided by the rule)
- other — plan.md Affected Files omits files f65ef759 changed for task 25: docs/analyze.md, framework/reference/implement.md, runtime/src/primitives/mod.rs, scripts/mutation/reopen-measure.py and 079's spec.md — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims and is the owner's plan decision, not a mechanical edit; every one of these files is already in the window through the commit that changed it (decided by the rule)
- other — plan.md's Affected Files omits files task 27's commits changed, among them criterion_verdicts.rs, invalidate_review.rs, main.rs, mod.rs, tests/mcp.rs, tests/review_command.rs, the fixtures, framework/reference/audit.md, docs/slash-commands.md and four mutation lists — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims, the owner's plan decision rather than a mechanical edit; each is already in the window through the commit naming 066 that changed it (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.github/workflows/runtime-acquisition.yml` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime-release.yml` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `framework/bootstrap/ductus.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `framework/bootstrap/govern.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 029-bootstrap-runtime-autowire
- `specs/048-govern-acquired-runtime/data-model.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime

## Unexamined governance

*None.*
