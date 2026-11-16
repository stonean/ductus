---
spec: 066-audit-records-and-the-completion-gate
last-run: 2026-11-16T13:45:39Z
reviewed-against: c1ffa247f3039322c8663fa2f296322fd0477134
diff-base: 9179ec97aef0e8d0cdfadf11c5fbdd7a5323297d
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 16
unread:
  - .claude/commands/ductus/analyze.md
  - .claude/commands/ductus/review.md
  - .pi/prompts/ductus-analyze.md
  - .pi/prompts/ductus-review.md
  - framework/commands/analyze.md
  - framework/commands/review.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/write_analysis.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/descriptions.rs
excluded: 70
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 3f4ad82314fc938bc24fc39aa9faad7e788c76a12e2c45bf6a2906f2fa9982ab
  scenarios/a-done-spec-has-no-transition-to-gate.md: adaf8b75ee0e15ca259cb8a0156e824af99a28c28aab876466798daa4e219e3a
  scenarios/a-done-specs-plan-edit-leaves-its-analysis-current.md: 3d63a66653413cb90127966cf092b984c71d44dad6d9dda77eba08131f6737e6
  scenarios/a-multi-number-renumber-is-one-rewrite.md: 918149e6f0137d420222d80b45c11ff0b415ed2af0349ea16da23457cf2bd589
  scenarios/a-renamed-file-contributes-its-rewrites.md: b578c5093853df3201d5d4b451b21c55943afb4840a47cb6d45f263a96e2a9fe
  scenarios/a-review-states-what-it-read.md: a75902f6c548b664d670a03ef4642b8cb4ff20fbdccfa39468666fa5d9787490
  scenarios/a-supplied-summary-carries-the-loops-account.md: 80f366f0462d15212f4c5ceb7c097095fd5c5cd87ee68f769056c3dc1c803882
  scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md: 39b59f38bb7b419b250d16d6ff6f3fec473959a3f9293706a6fcab931f6f232d
  scenarios/checkbox-state-is-outside-the-analysis-digest.md: ca9c4875ae8dec48c0c79edc74f3af3d4ee405601c6a8b1dee3c9a68f748834d
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
blocking: false
dispositions:
  fixed: 3
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
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: waived
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
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
  - key: other — 02bda11a's check-review-freshness primitive, review-freshness.sh and Family 19 were retired by 33688944 (081); 066's live artifacts still describe them as current in places (scenarios a-renamed-file..., review-staleness-on-done-specs Context/Resolved Questions carry retirement notes, mechanical_sweep.rs docs still say Family 19 asks it at release) — `runtime/src/primitives/mechanical_sweep.rs:43`
    outcome: waived
    reason: the scenarios carry explicit 'Retired by spec 081' notes, and the module doc comment is 081's sweep to finish; no 066 hunk of this window is wrong at HEAD, since the retired code is deleted
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
  - key: other — the data-model Size paragraph 19234246 changed (compute-review-scope) states that performReview's request names every in-scope file whole, by content, by path alone, or in the lists file; that is 022 tasks 36 and 37's exec behavior, outside 066's primitives, and was not checked against the exec walker's code in this review — `specs/066-audit-records-and-the-completion-gate/data-model.md:51`
    outcome: waived
    reason: the sentence cites 022's data model for the claim and was written in the commit that delivered it; any correction lands in 022's directory, which is outside this review's files, and nothing in 066's code depends on it
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
---

# Review — 066-audit-records-and-the-completion-gate

## Summary

066's review of task 24 (`91771eec`), which puts `write-review` and `write-analysis` on the compare-and-set guard. One agent ran all five passes and read all nine loaded backend and cross rule files whole. It read 6 of the 16 scope entries whole: the review golden, the line-ending lint, the guard's mutation list, and 066's data model, plan and tasks; and 066's spec and all 25 scenarios, outside the scope. The four generated mirrors were compared with their sources by script and are counted unread; `write_review.rs`, `write_analysis.rs`, `descriptions.rs`, the CHANGELOG and the two command sources were read at the window's hunks with `run`, `run_between`, the touched steps and both descriptions whole. One finding, `QUAL-TEST-001` (should, low confidence): no test failed when the snapshot was taken after the helpers' first read; the fix round gives each `run_between` a hook at a fixed point just after the snapshot, with a test per primitive and mutants M7 and M8 (a second snapshot just before the write) that only those tests kill, so it no longer fires. Six observations: three chores fixed (the data model says each primitive takes a snapshot before any helper reads the record, not that it reads it once; `write_analysis.rs`' comment names the guarded write's refusal; the plan's Affected Files names the other files task 24 changed), and three waived by the rule with their reasons. The fix round was made in a worktree, its whole mutation list run there (8 of 8 killed) and the whole `cargo test` passed, then landed in the main tree. An independent agent re-read the round against the whole contract: spec, data model and all 25 scenarios, 141,634 bytes, read whole; clean, with `plan.md` exempt as an analysis subject, and the loop's closing round was empty. Of the four stored decisions the reviewer re-judged, three stand and are carried; the fourth, keyed on `process_decisions.rs` outside the scope, is retained, since the review checked only its `write-review` clause.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

## Observations

- other — 066's data model says write-review reads review.md once, and write-analysis reads analysis.md once, before any helper reads the lists it keeps, but each primitive reads the record several times: the snapshot, then each helper (load_review_record, read_recorded_waivers, read_decisions; rule_verdicts::standing and decisions::read_decisions for analysis), then the re-read under the lock. The sentence contradicts itself, since helpers read after the once. — `specs/066-audit-records-and-the-completion-gate/data-model.md:136` — **fixed** (decided by the rule)
- convention — write_analysis.rs's comment before render_analysis says the record is written on every run it is not refused and lists the refusals as the ones above (an unreadable spec or prior record, an unlistable rule directory); the guard adds a refusal after them, ChangedSinceRead, so the list is stale. The Errors doc and descriptions.rs were updated, the comment was not. — `runtime/src/primitives/write_analysis.rs:268` — **fixed** (decided by the rule)
- other — 066's plan Affected Files table lists descriptions.rs and the mutation list as task 24's files but not the rest of what the commit changed: framework/commands/review.md (step 11), framework/commands/analyze.md (step 21) and runtime/tests/line_ending_discipline.rs, so the plan's table understates the surface a later window's plan-affected scope draws on. — `specs/066-audit-records-and-the-completion-gate/plan.md:68` — **fixed** (decided by the rule)
- other — after the guard, write_review.rs and write_analysis.rs no longer call write_atomic, so dropping their two EXEMPT entries is correct and forced (no_exemption_outlives_its_writer would fail otherwise), but the same migration takes every guarded writer out of every_text_writer_accounts_for_line_endings' subject, whose module doc still says write_atomic_bytes callers copy bytes; prune_plan.rs, prune_tasks.rs, invalidate_review.rs and relocate_audit_records.rs already contain no write_atomic( call in their non-test code and so are not examined at all, and a writer of an authored file that moves onto write_if_unchanged without one leaves the lint green and unexamined. — `runtime/tests/line_ending_discipline.rs:37` — **waived**: the lint and its module doc belong to 051's rewrites-preserve-line-endings scenario, and the class migration that widens the blind spot is 022 task 38's (which adds a test holding raw writes to an allowlist); nothing in 066's two writers is affected, since both render their record whole and carry no authored text to preserve. The fix would land in specs/051-branch-scoped-spec-numbering and specs/022-deterministic-runtime, outside 066. (decided by the rule)
- other — review.md step 12 invokes write-review a second time with step 11's arguments, after the whole fix-round loop, which is the longer window in which another writer can change review.md; the changed-since-read handling sentence sits only in step 11, so step 12 meets a refusal with its handling path stated one step earlier. — `framework/commands/review.md:137` — **waived**: step 12's write is defined as step 11's arguments but for loop-open, and step 11's sentence is scoped to 'a write' of write-review, not to its first call; the operator reads one handling path for both. (decided by the rule)
- other — the handling path both commands state re-runs only the write call from the same arguments, so the arguments derived from the earlier read of the record (review step 8's applied and expired waivers and step 9's expired-decisions; analyze step 18's expired-decisions) are reused after another writer changed the record, and a waiver or decision that writer stored is then neither classified nor expired until the next run; a new-waivers item that writer already stored is refused as InvalidArgument rather than ChangedSinceRead. — `framework/commands/analyze.md:93` — **waived**: BE-TXN-004 accepts reporting the conflict to the caller as a handling path and the sentence says to report it first; 066's plan chooses the re-run from the run's argument files and rejects plumbing the snapshot text into readers, and the next run re-derives every classification from the record as it then stands. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/consolidate.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.claude/commands/ductus/fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.claude/commands/ductus/prune.md` — 022-deterministic-runtime, 041-task-pruning
- `.ductus/reference/fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.pi/prompts/ductus-consolidate.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.pi/prompts/ductus-fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.pi/prompts/ductus-prune.md` — 022-deterministic-runtime, 041-task-pruning
- `AGENTS.md` — 041-task-pruning, 050-constitution
- `framework/commands/consolidate.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `framework/commands/fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `framework/commands/prune.md` — 022-deterministic-runtime, 041-task-pruning
- `framework/reference/fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `runtime/src/main.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/bounded_lists.rs` — 022-deterministic-runtime
- `runtime/src/primitives/derive_dependencies.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_references.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/invalidate_review.rs` — 051-branch-scoped-spec-numbering
- `runtime/src/primitives/mod.rs` — 022-deterministic-runtime, 041-task-pruning, 057-analyze-artifact-and-record-relocation
- `runtime/src/primitives/prune_plan.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/prune_project.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/prune_tasks.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/relocate_audit_records.rs` — 057-analyze-artifact-and-record-relocation
- `runtime/src/primitives/rewrite_spec_links.rs` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `runtime/src/schema/paths.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/schema/primitives.rs` — 022-deterministic-runtime, 041-task-pruning, 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 067-analyze-and-audit-checks
- `runtime/src/session.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/tests/mcp.rs` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 067-analyze-and-audit-checks
- `scripts/mutation/compare-and-set-041.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/compare-and-set-051.py` — 051-branch-scoped-spec-numbering
- `scripts/mutation/compare-and-set-057.py` — 057-analyze-artifact-and-record-relocation
- `scripts/mutation/compare-and-set-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/containment-and-bounds-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/owner-reviews-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/prune-project-containment.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/prune-project.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/rewrite-spec-links.py` — 051-branch-scoped-spec-numbering
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 041-task-pruning, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 041-task-pruning, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/041-task-pruning/analysis.md` — 041-task-pruning
- `specs/041-task-pruning/data-model.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/plan.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/review.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/spec.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/tasks.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/050-constitution/analysis.md` — 050-constitution
- `specs/050-constitution/plan.md` — 050-constitution
- `specs/050-constitution/review.md` — 050-constitution
- `specs/050-constitution/scenarios/the-constitution-fits-one-read.md` — 050-constitution
- `specs/050-constitution/spec.md` — 050-constitution
- `specs/051-branch-scoped-spec-numbering/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/051-branch-scoped-spec-numbering/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/051-branch-scoped-spec-numbering/review.md` — 051-branch-scoped-spec-numbering
- `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md` — 051-branch-scoped-spec-numbering
- `specs/051-branch-scoped-spec-numbering/spec.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/051-branch-scoped-spec-numbering/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/052-spec-supersession-and-consolidation/spec.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `specs/057-analyze-artifact-and-record-relocation/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/057-analyze-artifact-and-record-relocation/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/057-analyze-artifact-and-record-relocation/review.md` — 057-analyze-artifact-and-record-relocation
- `specs/057-analyze-artifact-and-record-relocation/spec.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/057-analyze-artifact-and-record-relocation/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/analysis.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/review.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-results-are-bounded.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/spec.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
