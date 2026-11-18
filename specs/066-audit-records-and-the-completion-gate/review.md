---
spec: 066-audit-records-and-the-completion-gate
last-run: 2026-11-18T02:04:08Z
reviewed-against: 680e47391e97f24f357cf058aba5ed0b4de3a271
diff-base: ed857b831303718a4e1f26ef890fe40a4c435ee9
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 11
unread:
  - runtime/src/primitives/write_analysis.rs
  - runtime/src/schema/descriptions.rs
  - scripts/mutation/reopen-measure.py
excluded: 0
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 0dfad555e49515b76f6cafc1f2b3b9f1bb6d17cb6ee2c78bf6d8d606812fc38e
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
  waived: 4
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
---

# Review — 066-audit-records-and-the-completion-gate

## Summary

066's follow-up review of task 26 (680e4739), the window since the record committed at ed857b83, which 066's pre-done check opened: line_citations::strips_to, read by pair_run, same_items and unticks_a_criterion so a partial strip is mechanical (AC6), and write-analysis's refusal of an unexamined reason outside UNEXAMINED_REASONS, held equal to the data model's table (AC2), both owner decisions of 2026-11-18. One independent agent ran all five passes over the window, reading all nine loaded backend and cross rule files whole (285,795 bytes) and 066's spec.md beside the scope. 8 of the 11 scope entries were read whole. Three are unread: runtime/src/primitives/write_analysis.rs, read at every hunk of the window and the whole of each item enclosing one (UNEXAMINED_REASONS, refuse_unknown_reasons, run_between through the reason fold, the errors doc and the three changed tests); runtime/src/schema/descriptions.rs, read at CLASSIFY_SPEC_EDIT and WRITE_ANALYSIS whole; and scripts/mutation/reopen-measure.py, read at its M116 entry. The agent checked that every reason check-artifacts, resolve-anchor, check-rule-ids and the exec walker's tally supply is in the list, that strips_to accepts no renumbered, moved or added citation and no other change, its worst case (2^MAX_ONE_BY_ONE alignments per text, pruned by each prefix match), and that an untick beside a partial strip still reopens. One finding, QUAL-TEST-001 (should, low confidence): the plan stated the bound as a figure the test does not hold; the plan and CLASSIFY_SPEC_EDIT now point at MAX_ONE_BY_ONE rather than copying it, so the finding no longer fires. Seven observations: three chores fixed (CLASSIFY_SPEC_EDIT names the bound; refuse_unknown_reasons' doc comment and the plan say the refusal precedes reading the spec and its records, not every read, since the configuration is loaded first); three stored decisions matched and carried; and one waived by the rule with its reason (nothing ties the emitters' reasons to the list but the loud refusal and the closed-set procedure in docs/developing-ductus.md). One stored decision expired: round 1's account of its first, shallow re-read, a record of that run's event.

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

- perf — SweepIndex::explained_by re-sorts the repo-wide pair set on every call, once per file-local pair per path, instead of once at build — `runtime/src/primitives/mechanical_sweep.rs:237` — `runtime/src/primitives/mechanical_sweep.rs` — **waived**: the repo-wide set is small and the window is capped at RENAME_LIMIT paths; no measured cost, and moving the sort changes no verdict (decided by the rule)
- other — plan.md Affected Files omits files f65ef759 changed for task 25: docs/analyze.md, framework/reference/implement.md, runtime/src/primitives/mod.rs, scripts/mutation/reopen-measure.py and 079's spec.md — `specs/066-audit-records-and-the-completion-gate/plan.md` — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims and is the owner's plan decision, not a mechanical edit; every one of these files is already in the window through the commit that changed it (decided by the rule)
- other — the data model's compute-review-scope Size paragraph says performReview's request names every in-scope file whole, but the exec walker's load_scope_files (runtime/src/interpreter/payload.rs) silently skips a scope file that is missing, unreadable or not UTF-8, naming only a path leaving the project in unexaminable — `specs/066-audit-records-and-the-completion-gate/data-model.md` — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **waived**: the code is 022 and 080's exec walker, outside 066's primitives; the sentence cites 022's data model for the claim, a correction lands in 022's directory, and nothing in 066's code depends on it (decided by the rule)
- convention: CLASSIFY_SPEC_EDIT says a strip's every change removes a suffix and each citation on a line is kept or stripped on its own, but strips_to tries each one by one only up to MAX_ONE_BY_ONE (twelve) citations in the text it is given, a line in pair_run and a whole item in same_items, and past that counts only a strip of every one, so a partial strip beside thirteen citations reads as a change; the plan states the bound and the description, which must state a primitive's rules completely, does not — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- convention: refuse_unknown_reasons' doc comment says it refuses before any read or write, but run_between loads the project configuration (Paths::load) and stats the feature directory first; it runs before the spec and the record are read or written — `runtime/src/primitives/write_analysis.rs` — **fixed** (decided by the rule)
- convention: 066's plan says write-analysis refuses an unknown reason before it reads or writes anything, but the refusal follows the configuration load and the feature-directory check; it precedes any read or write of the spec or its records — `specs/066-audit-records-and-the-completion-gate/plan.md` — **fixed** (decided by the rule)
- other: nothing ties the reasons check-artifacts, resolve-anchor, check-rule-ids and the exec walker's tally emit to UNEXAMINED_REASONS; the test holds the list equal to the data model's table only, so a skip reason added to a check and not to the list and table makes every write-analysis that records it fail with a refusal naming the reason (loud, never a silent record). Every reason those sources emit at HEAD is in the list (checked by reading each emitter) — `runtime/src/primitives/write_analysis.rs` — **waived**: the refusal is loud and names the reason, so the gap cannot pass as a clean record; docs/developing-ductus.md's closed-set procedure greps a sibling reason such as root-absent across runtime/src and so reaches UNEXAMINED_REASONS; a test enumerating every emitter would copy the checks' reason sets, which this review found equal to the list at HEAD (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

*None.*

## Unexamined governance

*None.*
