---
spec: 066-audit-records-and-the-completion-gate
last-run: 2026-11-27T23:32:48Z
reviewed-against: 0d0eaf2745c9a87f8b3d7087356e3d3d4c570013
diff-base: fb4f198d1f9bdd7f88f3e769a2561ac686eb07bd
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 24
scope: 25
unread:
  - runtime/CHANGELOG.md
excluded: 282
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 9e33d5b883a7edcceeaeaf2d6ae5039d69447c3cdd7b01f910213273991aa7d6
  scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md: 0816f5c6cd5c4a4d38da12d52352935b49c883592ecc524df8d5310fa7c50bfe
  scenarios/a-done-spec-has-no-transition-to-gate.md: adaf8b75ee0e15ca259cb8a0156e824af99a28c28aab876466798daa4e219e3a
  scenarios/a-done-specs-plan-edit-leaves-its-analysis-current.md: 3d63a66653413cb90127966cf092b984c71d44dad6d9dda77eba08131f6737e6
  scenarios/a-multi-number-renumber-is-one-rewrite.md: 918149e6f0137d420222d80b45c11ff0b415ed2af0349ea16da23457cf2bd589
  scenarios/a-renamed-file-contributes-its-rewrites.md: 1cef0dd96a49ce4d53eba5cbdf352f72589a8ce9945e3472a66c96ac7a2a0340
  scenarios/a-review-states-what-it-read.md: a75902f6c548b664d670a03ef4642b8cb4ff20fbdccfa39468666fa5d9787490
  scenarios/a-supplied-summary-carries-the-loops-account.md: 80f366f0462d15212f4c5ceb7c097095fd5c5cd87ee68f769056c3dc1c803882
  scenarios/a-write-result-states-what-it-wrote.md: 71d4b19f014d1bc19335c9ec29af30fa4e782d57538db9167962cd7064f649de
  scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md: 89660ea69af57108cf4bdc725d23d77ca65f01dc39270839a04a017a4bfa1ae2
  scenarios/checkbox-state-is-outside-the-analysis-digest.md: ca9c4875ae8dec48c0c79edc74f3af3d4ee405601c6a8b1dee3c9a68f748834d
  scenarios/criteria-are-verified-in-review.md: 16aeca88696498509a0bc1dca3580fa9f6bf7f1d5054905ea61adaa1de9b3dd1
  scenarios/freshness-results-have-a-bounded-size.md: 0365689904e4ca8d3a11c14ce4411acbe95373560bf6ffa390a17afda8c832e0
  scenarios/only-gating-findings-hold-done.md: 27a6385fb7da6ccd6bf74303f92559b00998fe170248408e1df76bdf3a0b0f29
  scenarios/process-decisions-results-have-a-bounded-size.md: ef377a26ae73760cc642e3c8ca364ee8fb1d680b97f07d2bc18395de3c314013
  scenarios/process-waivers-file-lists.md: ae9414857f88794608b39b9abea16e76faa90a4ab21e5f325da80c27875719ae
  scenarios/process-waivers-notices-flatten-stored-text.md: a0fe96d2f894add12b9ed470dfdc011ce1383b1528871c8d68ec64bb85d64c03
  scenarios/record-paths-are-quoted-only-when-they-need-it.md: 729b1dccb47871d2a8f4fe1dc7e0cbe1449d92eb8c28004a1490859e88ff491d
  scenarios/review-base-includes-the-transition-commit.md: afeafa7f9bfe90edb08ece0420abf49620441b370abeb5dcbd487ea299a99948
  scenarios/review-scope-includes-uncommitted-work.md: 0e24e90807ac2080c3ec6f2f796fe779c5e1e64c15b6a9818645d681e0f4917c
  scenarios/review-scope-plan-affected-table-format.md: 501c1d9737e6d2b928318c02e1d13164817ba7c1054854cd599f14707caba767
  scenarios/review-scope-reads-every-cell-path-and-sets-deleted-files-apart.md: e33e76432da85f5cc661041515f0f3aae300ad9974065e5eadba696cc820748c
  scenarios/review-scope-union.md: 50722645d52601a9b8f0c799272f58923a7250623abc097ad38bb5a4ec62181e
  scenarios/review-staleness-on-done-specs.md: 2f8e0938b6f8528b1fb5c34fe0952e8555cb06f9dfd27e7d7508d94bc08387c5
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
    digest: 65b1e405e109365fe86f950075def52a2d33acecbba9f98d64d8028e873aaed3
  - label: AC4
    digest: e9c519e1f2a474c3e09a61a1f92e0417efb822600a7879dca7a9f5cf91525531
  - label: AC5
    digest: 288faa8b2d4022c825de9c8c0047ae4ab2e6a8895cce3afe76089cd74d5b0c50
  - label: AC6
    digest: 88b2090833a6ba966c51f8bae38af79176ec376bd1440394ca2c91da3885fa12
blocking: false
dispositions:
  fixed: 8
  deferred: 0
  waived: 21
  undispositioned: 0
decisions:
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: waived
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the implement reference's \"non-blocking\" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md`"
    outcome: waived
    reason: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md`"
    outcome: waived
    reason: "contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: claude-md-import-syntax's shared-import bullet contrasts its rewrite with a removal the procedure no longer performs — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: the bullet still guards against de-duplicating an import the adopter added by hand, which removal would drop if the registration were withdrawn; its sense does not depend on the constitution line
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: a CLAUDE.md symlinked to AGENTS.md becomes a copy when constitution-import writes it by temporary file and rename — `framework/migrations/constitution-import.md`"
    outcome: waived
    reason: every bootstrap write of CLAUDE.md already does this, merge-managed-block's managed-block render on every run included, so the migration adds no new case; a symlinked CLAUDE.md is not a layout the framework supports
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: constitution-import accepts a four-space-indented line, misses a match behind a leading byte-order mark, and reports the @import form on a file unpinned after claude-md-import-syntax was skipped — `framework/migrations/constitution-import.md`"
    outcome: waived
    reason: "contrived inputs: claude-md-import-syntax treats indentation the same way, the template writes no byte-order mark, and an operator who unpins a file has the earlier migration's pinned warning in hand"
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
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
    reason: the refusal is loud and names the reason, so the gap cannot pass as a clean record; .ductus/docs/agents/developing-ductus.md's closed-set procedure greps a sibling reason such as root-absent across runtime/src and so reaches UNEXAMINED_REASONS; a test enumerating every emitter would copy the checks' reason sets, which this review found equal to the list at HEAD
    decided-at: 2026-11-18T02:04:08Z
    decided-by: rule
  - key: other — a run judging only some criteria records criteria-judged and criteria-met over those alone, and its generated Summary reads 'N of N', with no count of the criteria it left unjudged — `runtime/src/primitives/write_review.rs`
    outcome: waived
    reason: the data model defines criteria-judged as the criteria the run judged and promises no unjudged count; compute-review-scope names each unjudged criterion in criteria-owing and reads review-owed true, so the gate holds done, and the command names a criterion it could not judge in the Summary
    decided-at: 2026-11-19T00:38:50Z
    decided-by: rule
  - key: other — 66203361 (task 27b) also carries 029 task 17's edit to framework/bootstrap/ductus.md and govern.md, dropping the 'after a fallback to tar' clause, which its message does not name — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: another session's staged hunk that rode along through the shared index; .ductus/docs/agents/developing-ductus.md §Running a second session records the incident, history on main stays as written, and the section reads consistently with 029 task 17
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
  - key: other — plan.md's Affected Files omits files task 27's commits changed, among them criterion_verdicts.rs, invalidate_review.rs, main.rs, mod.rs, tests/mcp.rs, tests/review_command.rs, the fixtures, framework/reference/audit.md, docs/slash-commands.md and four mutation lists — `specs/066-audit-records-and-the-completion-gate/plan.md`
    outcome: waived
    reason: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims, the owner's plan decision rather than a mechanical edit; each is already in the window through the commit naming 066 that changed it
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
  - key: "`render_rule_fields` in `runtime/src/primitives/write_analysis.rs` flattens a rule file name through `single_line` before `yaml_string` for `rule-digest` keys, a map the scenario does not name — `runtime/src/primitives/write_analysis.rs`"
    outcome: waived
    reason: rule-digest keys are rule file names, a map the scenario does not name, and flattening them predates this change and loses nothing a path needs
    decided-at: 2026-11-19T22:36:00Z
    decided-by: rule
  - key: "convention: plan.md's Affected Files table omits files tasks 29 and 30 changed (runtime/CHANGELOG.md, runtime/src/primitives/write_review/departure_waivers.rs, runtime/tests/fix_rounds.rs, runtime/tests/parity.rs, runtime/tests/walker.rs) and gives framework/reference/review.md, descriptions.rs and primitives.rs purposes naming other tasks only — `specs/066-audit-records-and-the-completion-gate/plan.md`"
    outcome: waived
    reason: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims, the owner's plan decision rather than a mechanical edit; each is already in the window through c6aa33df, whose subject names 066
    decided-at: 2026-11-22T05:44:34Z
    decided-by: rule
  - key: "other: no step of framework/commands/analyze.md or framework/commands/review.md tells the host to compare write-analysis's hard-fail, blocking-findings and advisory or write-review's excluded, unread-count and decisions with what it passed, so the discrepancy those fields were added to expose (a dropped decision, a net count) surfaces only when a host happens to look — `framework/commands/review.md`"
    outcome: waived
    reason: the scenario adds the fields so a host can reconcile a record without reading it back and states no obligation to compare them; a step requiring the comparison is a new requirement about the pipeline's own machinery, not a mechanical fix
    decided-at: 2026-11-22T05:44:34Z
    decided-by: rule
  - key: "reuse: `cut` in moved_scope.rs duplicates `truncate(text, budget)` in resolve_constitutions.rs (count characters, append an ellipsis when cut); the two differ only in `truncate` trimming trailing whitespace before the ellipsis, a property QUAL-DELEG-001 asks be enumerated before one replaces the other — `runtime/src/primitives/write_review/moved_scope.rs`"
    outcome: waived
    reason: truncate in resolve_constitutions.rs also trims trailing whitespace before its ellipsis, so one shared helper would change one caller's output (QUAL-DELEG-001), and each is a few lines private to its module
    decided-at: 2026-11-22T06:48:24Z
    decided-by: rule
  - key: "other: AC2's rejection of a malformed `--unexamined-reason` pair holds in code, driven by hand, but no test calls `parse_reason_count` in `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: "not mechanical: a unit test is new authoring, and no contract breaks — the CLI refuses a pair with no `=`, a non-integer, a negative count and an empty reason, driven by hand in this review"
    decided-at: 2026-11-26T02:01:05Z
    decided-by: rule
  - key: "convention: `criterion.label.is_some()` in `respelled_since_the_run`'s candidate filter repeats the later `label.as_deref()?` — `runtime/src/primitives/criterion_verdicts.rs`"
    outcome: waived
    reason: it keeps an unlabelled criterion out of the candidates, whose emptiness skips sweep_since_the_run's history walk and sweep-index build, so it saves that walk and changes no result
    decided-at: 2026-11-27T23:11:21Z
    decided-by: rule
  - key: "convention: CHECK_REVIEW_GATE says the gate names each criterion no review judged met with its current text, while a criterion a mechanical sweep alone changed keeps its verdict — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: §drift-prevention reads a mechanical sweep as no change, so a respelled criterion keeps its current text in that rule's sense, and COMPUTE_REVIEW_SCOPE, whose review-owed the gate reads, states the exception
    decided-at: 2026-11-27T23:11:21Z
    decided-by: rule
  - key: "other: no test drives PROCESS_DECISIONS's new leaving-the-project clause through `process_decisions::run` — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: process-decisions reaches a stored key's file only through RunScope::why_unread, whose leaving-the-project cases a_stored_path_leaving_the_project_is_never_probed pins, so removing the containment fails that test
    decided-at: 2026-11-27T23:11:21Z
    decided-by: rule
  - key: "convention: the Claude copy's ToolSearch line omits check-unreviewed-code, the first call of a project review — `.claude/commands/ductus/review.md`"
    outcome: waived
    reason: a project run costs one more ToolSearch call; the line is derived from the numbered steps by tool_loading::step_tools, the generator's code rather than 066's
    decided-at: 2026-11-27T23:11:21Z
    decided-by: rule
  - key: "other: analysis.md's `analyzed-against` names no object in the repository, so a changed rule file's copy cannot be read at it — `specs/066-audit-records-and-the-completion-gate/analysis.md`"
    outcome: waived
    reason: the analysis that follows this review rewrites the record, and a carried verdict whose rule-file copy cannot be read is judged afresh, which costs a judge's read and loses no verdict
    decided-at: 2026-11-27T23:11:21Z
    decided-by: rule
  - key: "other: `mechanical_sweep.rs`'s module and function docs explain the rule through the retired Family 19 — `runtime/src/primitives/mechanical_sweep.rs`"
    outcome: waived
    reason: each comment narrates that history accurately beside the rule as it holds now, and code comments are outside 050's AC27 sweep of spec bodies, scenarios, plans and data models
    decided-at: 2026-11-27T23:11:21Z
    decided-by: rule
  - key: "other: no mutation list removes the new leaves_project guard in still_on_disk or Pass::place, so run.py cannot re-prove their tests — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: waived
    reason: each guard's test is proven by hand to fail without it, as the commit records, and QUAL-TEST-001 asks for that failure, not for a mutation entry
    decided-at: 2026-11-27T23:32:11Z
    decided-by: rule
  - key: "other: Pass::place probes an anchor's file with exists(), which reads a dangling link as gone, while still_on_disk reads one as on disk — `runtime/src/primitives/process_waivers.rs`"
    outcome: waived
    reason: the two disagree only when a fired finding names a dangling link, which no pass can read, so no reachable input reaches the difference
    decided-at: 2026-11-27T23:32:11Z
    decided-by: rule
---

# Review — 066-audit-records-and-the-completion-gate

## Summary

Review of 066's window fb4f198d..0d0eaf27 (base-source last-review), from a clean worktree at 0d0eaf27 holding this run's fixes uncommitted. The scope is 25 entries, the record itself aside: the 18 the window resolved, among them the five the last review left unread, and seven that joined through this run's fixes (`framework/commands/review.md`, `runtime/src/primitives/process_waivers.rs`, `runtime/tests/golden/review-basic.jsonl`, `scripts/mutation/prune-review-086.py`, whose M66 anchor moved with `still_on_disk`, and three scenarios). `docs/developing-ductus.md` is deleted (087 moved it), so no pass could read it. Five passes parts split by behaviour each ran all five passes over all five loaded rule files (231,962 bytes; `discover-rule-files` left out seven whose headers give them to analysis alone), and one criteria part judged every criterion. Each hand-back was checked against `wc -c`; part Pa read every rule's Statement and Verification but not its `**Source:**` line (10,802, 2,487, 2,206 and 4,965 bytes of `security-backend.md`, `configuration-cross.md`, `security-cross.md` and `security-frontend.md`). Examined 24 of 25: `runtime/CHANGELOG.md` is unread, its [Unreleased] section read whole, which holds every hunk of the window, and lines 1915 to 1945 for a stored decision, its released history from line 473 not. One MUST was found and fixed in the run, so the record carries no violation: `BE-INPUT-004` at `still_on_disk` in `runtime/src/primitives/compute_review_scope.rs`, which probed a path drawn from a record's committed text without containing it; sweeping the class found a second instance in `process_waivers.rs`'s `Pass::place`, fixed by the operator's decision (2026-11-27). Each now contains the path first, as 022's plan §Path arguments requires, and each new test is proven to fail without its guard. All six criteria are judged met, AC3 and AC6 on the text 0d0eaf27 gave them. Of 29 observations, 8 are fixed, 8 waived by the rule and 13 matched to stored decisions, four of those re-fired on `framework/commands/review.md` once a fix put it in scope; 10 stored decisions on files that do not belong to 066 are dropped and 24 retained. The run also rewrote three scenarios to state only what holds now, 050's AC27, which the operator assigned to this lane (2026-11-27). Unchecked: the data model's measured byte figures in its Size paragraphs, which no part re-derived. Outside the scope and recorded nowhere: a low-confidence `QUAL-TEST-001` finding that no test in `runtime/tests/mechanical_sweep_parity.rs` pins `RENAME_THRESHOLD`'s and `RENAME_LIMIT`'s values. Two stored rule waivers keep an outcome that still holds beside a reason whose premise does not: `criterion_verdicts::owing_in`'s (`compute-review-scope` does not read `spec.md` first) and the constitution's (its headroom is no longer one byte).

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

8 of 115 commits in the window no spec claimed.

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

- AC1 — met: write_analysis::run_between derives blocking from gating_counts (no blocking arg) and refuses unparseable spec.md frontmatter (test malformed_frontmatter_is_never_given_a_clean_record); check_review_gate::run_with_lint runs review_half_block before analyze_half_block, NotAnalyzed/AnalyzeFindings come from analyze_gate_block (tests a_reviewed_but_unanalyzed_spec_is_blocked, a_missing_review_is_reported_before_a_missing_analysis, what_only_a_review_clears_outranks_a_stale_analysis pass).
- AC2 — met: write_analysis UNEXAMINED_REASONS closed set enforced by refuse_unknown_reasons, unexamined = by_reason.values().sum(), map omitted when empty, schema parse_reason_count rejects a malformed pair, result.unexamined returns the total; tests a_reason_outside_the_closed_set_is_refused, the_reason_list_is_the_data_models_table, a_bare_total_survives_a_derived_reason_beside_it pass.
- AC3 — met: write_review::render_contracts always writes reviewed-digest ({} when empty); compute_review_scope::contracts_against_digest -> review_freshness -> freshness_of/exempt_renames (every candidate kept when no tree), hold_in_scope, is_project_artifact; check_review_gate compares no digest; reviewed-against guard read whole: write_review::resolve_scope calls moved_scope::refuse_a_moved_scope(&args.reviewed_against, repo, &entries) for a spec (and resolve_project_scope for the project record), which refuses a sha naming no commit, a HEAD not descending from it, and a commit after it changing a scope path, writes nothing, bounds the refusal at 3 paths x 3 commits; an empty diff-base skips it (resolve_scope early return) as the scenario says; held-record window start read in compute_review_scope::recorded_run (HeldRun.against); 12 moved_scope tests pass incl. write_review_refuses_a_write_whose_scope_a_later_commit_moved. Verdict confirmed.
- AC4 — met: check_review_gate::already_done_block runs first (before markdown_lint_block), ReviewGateBlock::AlreadyDone with passed false, only the exact status done matches; tests a_done_spec_short_circuits_without_claiming_to_have_passed, a_done_spec_is_not_held_against_a_later_lint_violation, an_unrecognized_status_is_not_treated_as_done pass; reference/implement.md halts on not gated.
- AC5 — met: cross_spec_impact_block runs after scenario_question_block and before the review-record load; classify_cross_spec_entry + links_back (pointer scope, blockquotes discharge); CrossSpecImpactState has exactly the five states; outside-project covers dir/spec.md/scenarios/scenario links and non-UTF-8 names (tests *_linked_outside_the_project_is_reported_and_never_read pass); check never writes. Clause 'reports each entry on a pass as well as a block': a pass carries cross_spec_impact: cross_spec (run_with_lint) and the check's own block carries entries.to_vec() (cross_spec_impact_block, test an_entry_naming_no_spec_directory_is_a_finding_not_a_pass), but a later check's block (review_owed_block, fix_round_block, disposition_block, analyze blocks) sets cross_spec_impact: vec![] so discharged entries are not reported there. Read as the criterion's own subject, 'a block' is this check's block, which the code meets, so met; read as any block it fails, never-true since delivery in 11fe1928 (later checks were already empty then).
- AC6 — met: line_citations::find/strips_to cover :N, :N-M, :N:C, #LN, #LN-LM after a path with an extension (EXTENSIONS when no slash, hosts/ports excluded); mechanical_sweep::pair_run applies it; analyze_subjects::exempt_renames serves freshness_of and review_freshness (called from contracts_against_digest); other changes judged as before; tests a_strip_only_commit_leaves_the_records_current, a_strip_beside_another_change_is_judged_on_that_change, hosts_ports_and_other_colons_are_not_citations pass.

## Observations

- perf: SweepIndex::explained_by still re-sorts the repo-wide pair set on every call; task 32's respells now calls it per pair of each swept criterion too — `runtime/src/primitives/mechanical_sweep.rs` — **waived**: the repo-wide set is small and the window is capped at RENAME_LIMIT paths; no measured cost, and moving the sort changes no verdict (decided by the rule)
- other: plan.md's Affected Files still omits files f65ef759 changed for task 25 — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims and is the owner's plan decision, not a mechanical edit; every one of these files is already in the window through the commit that changed it (decided by the rule)
- other: the data model's compute-review-scope Size paragraph still says performReview's request names every in-scope file whole, while the exec walker's load_scope_files skips a missing or unreadable one — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **waived**: the code is 022 and 080's exec walker, outside 066's primitives; the sentence cites 022's data model for the claim, a correction lands in 022's directory, and nothing in 066's code depends on it (decided by the rule)
- convention: the constitution's mechanical-sweep definition, now in §drift-prevention's dead-references bullet, still does not say a line number set apart in its own code span is outside it — `framework/constitution.md` — **waived**: the plan's definition and CLASSIFY_SPEC_EDIT's compiled description state the exclusion, the case is a precision gap rather than a contradiction, and the constitution has one byte of headroom under its 50,000-byte ceiling, so a clarifying rewrite owes a cut elsewhere in a document 050 owns (decided by the rule)
- bug: criterion_verdicts still keys met verdicts by text digest, so two criteria with the same text, labels set aside, share one verdict — `runtime/src/primitives/criterion_verdicts.rs` — **waived**: contrived: two criteria with the same text state one requirement, so a verdict judging it holds for both, and the scenario keys a verdict to the criterion's text (decided by the rule)
- other: criterion_verdicts::owing_in still reads a spec.md that is not a file as owing nothing; a feature the pipeline creates always holds one, since create-feature writes it — `runtime/src/primitives/criterion_verdicts.rs` — **waived**: unreachable from its callers: compute-review-scope and check-review-gate read the spec before it, so a missing or non-file spec.md fails there first (decided by the rule)
- other: plan.md's Affected Files still omits files task 27's commits changed — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims, the owner's plan decision rather than a mechanical edit; each is already in the window through the commit naming 066 that changed it (decided by the rule)
- other: the task 22 CHANGELOG entry's BREAKING sentence still omits the inline notices change the entry's earlier sentence states — `runtime/CHANGELOG.md` — **waived**: The changed content of `notices` is stated two sentences earlier in the same entry, and the entry is already labelled BREAKING; repeating it adds no claim. (decided by the rule)
- convention: plan.md's Affected Files table still omits files tasks 29 and 30 changed — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: compute-review-scope reads Affected Files to attribute paths in commits several specs claim, so adding a path changes what 066 claims, the owner's plan decision rather than a mechanical edit; each is already in the window through c6aa33df, whose subject names 066 (decided by the rule)
- other: the 066 task 32 tick guard in `criterion_verdicts::respelled_since_the_run` had no test or mutation, so an unticked criterion a sweep alone changed could lose its owed review unseen; `an_unticked_swept_criterion_still_owes` and mutation M6 now pin it — `runtime/src/primitives/criterion_verdicts.rs` — **fixed** (decided by the rule)
- bug: spec.md's completion-gate paragraph put the analyze record ahead of the review checks and never named review-owed, while `check_review_gate.rs` runs every review check, review-owed included, before the analyze checks; reordered — `specs/066-audit-records-and-the-completion-gate/spec.md` — **fixed** (decided by the rule)
- other: task 31 named `git diff --name-only {reviewed-against} HEAD` as the moved-scope refusal's command, where `moved_scope.rs` and the plan give `git log --relative --name-only --format= {reviewed-against}..HEAD`; corrected — `specs/066-audit-records-and-the-completion-gate/tasks.md` — **fixed** (decided by the rule)
- other: the review-base scenario said a first run's base is the transition commit's parent, which a `done` spec's first run, based at its done commit, does not take; qualified to a spec short of `done` — `specs/066-audit-records-and-the-completion-gate/scenarios/review-base-includes-the-transition-commit.md` — **fixed** (decided by the rule)
- other: step 11 listed the scope among `write-review`'s arguments, though it takes none and derives the scope itself; dropped from `framework/commands/review.md` and the copies regenerated — `.claude/commands/ductus/review.md` — **fixed** (decided by the rule)
- other: the data model's `diff-cross-spec` result named no `guidance`, present when the window cannot be read, no commit touching the spec directory or the history cut short; named — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **fixed** (decided by the rule)
- other: the data model's `check-review-gate` paragraph left out that the cross-spec list is empty when a later check blocks; it now says populated on a pass and on its own block, empty on any other check's block — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **fixed** (decided by the rule)
- other: three scenarios this run rewrote for 050's AC27 carried a false owing claim, a figure with no method, a dangling antecedent and a present-tense claim about step 1; each corrected — `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md` — **fixed** (decided by the rule)
- convention: `criterion.label.is_some()` in `respelled_since_the_run`'s candidate filter repeats the later `label.as_deref()?` — `runtime/src/primitives/criterion_verdicts.rs` — **waived**: it keeps an unlabelled criterion out of the candidates, whose emptiness skips sweep_since_the_run's history walk and sweep-index build, so it saves that walk and changes no result (decided by the rule)
- convention: CHECK_REVIEW_GATE says the gate names each criterion no review judged met with its current text, while a criterion a mechanical sweep alone changed keeps its verdict — `runtime/src/schema/descriptions.rs` — **waived**: §drift-prevention reads a mechanical sweep as no change, so a respelled criterion keeps its current text in that rule's sense, and COMPUTE_REVIEW_SCOPE, whose review-owed the gate reads, states the exception (decided by the rule)
- other: no test drives PROCESS_DECISIONS's new leaving-the-project clause through `process_decisions::run` — `runtime/src/schema/descriptions.rs` — **waived**: process-decisions reaches a stored key's file only through RunScope::why_unread, whose leaving-the-project cases a_stored_path_leaving_the_project_is_never_probed pins, so removing the containment fails that test (decided by the rule)
- convention: the Claude copy's ToolSearch line omits check-unreviewed-code, the first call of a project review — `.claude/commands/ductus/review.md` — **waived**: a project run costs one more ToolSearch call; the line is derived from the numbered steps by tool_loading::step_tools, the generator's code rather than 066's (decided by the rule)
- other: analysis.md's `analyzed-against` names no object in the repository, so a changed rule file's copy cannot be read at it — `specs/066-audit-records-and-the-completion-gate/analysis.md` — **waived**: the analysis that follows this review rewrites the record, and a carried verdict whose rule-file copy cannot be read is judged afresh, which costs a judge's read and loses no verdict (decided by the rule)
- other: `mechanical_sweep.rs`'s module and function docs explain the rule through the retired Family 19 — `runtime/src/primitives/mechanical_sweep.rs` — **waived**: each comment narrates that history accurately beside the rule as it holds now, and code comments are outside 050's AC27 sweep of spec bodies, scenarios, plans and data models (decided by the rule)
- other: the review command's description still says it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md` — **waived**: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
- other: step 1 still says a zero scope with deletions is empty because every path it would hold was deleted, though `deleted` holds every window deletion whoever's commit made it — `framework/commands/review.md` — **waived**: the scenario's edge case scopes the claim to the spec's own code deletions with review-owed true; the report over-names rather than hides a path, and no gate reads it. The wording fix reopens review.md's generated mirrors and 066 for a low-confidence imprecision (decided by the rule)
- other: step 12's last write still takes step 11's arguments, while only step 11 states how to handle a changed-since-read refusal — `framework/commands/review.md` — **waived**: step 12's write is defined as step 11's arguments but for loop-open, and step 11's sentence is scoped to 'a write' of write-review, not to its first call; the operator reads one handling path for both. (decided by the rule)
- other: still no step tells the host to compare write-review's and write-analysis's returned counts with what it passed — `framework/commands/review.md` — **waived**: the scenario adds the fields so a host can reconcile a record without reading it back and states no obligation to compare them; a step requiring the comparison is a new requirement about the pipeline's own machinery, not a mechanical fix (decided by the rule)
- other: no mutation list removes the new leaves_project guard in still_on_disk or Pass::place, so run.py cannot re-prove their tests — `runtime/src/primitives/compute_review_scope.rs` — **waived**: each guard's test is proven by hand to fail without it, as the commit records, and QUAL-TEST-001 asks for that failure, not for a mutation entry (decided by the rule)
- other: Pass::place probes an anchor's file with exists(), which reads a dangling link as gone, while still_on_disk reads one as on disk — `runtime/src/primitives/process_waivers.rs` — **waived**: the two disagree only when a fired finding names a dangling link, which no pass can read, so no reachable input reaches the difference (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/plan.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/target.md`
- `.ductus/docs/agents/developing-ductus.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/prune.md`
- `.githooks/pre-commit`
- `.github/workflows/framework-checks.yml`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-plan.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-target.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `docs/runtime.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/commands/analyze.md`
- `framework/commands/help.md`
- `framework/commands/plan.md`
- `framework/commands/prune.md`
- `framework/commands/target.md`
- `framework/migrations.toml`
- `framework/migrations/report-project-file-links-into-docs.md`
- `framework/migrations/section-digest-backfill.md`
- `framework/reference/analyze.md`
- `framework/reference/implement.md`
- `framework/reference/plan.md`
- `framework/reference/prune.md`
- `framework/reference/review.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `install.sh`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/backfill_section_digest.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_audit_records.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_rule_ids.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/project_parts.rs`
- `runtime/src/primitives/prune_project.rs`
- `runtime/src/primitives/prune_project/moves.rs`
- `runtime/src/primitives/prune_review.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/spec_ownership.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/src/schema/paths.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/src/schema/registry.rs`
- `runtime/src/session.rs`
- `runtime/tests/check_artifact_size_cli.rs`
- `runtime/tests/check_audit_records_cli.rs`
- `runtime/tests/common/mod.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/fixtures/review-basic/specs/007-review/plan.md`
- `runtime/tests/fixtures/review-basic/specs/007-review/spec.md`
- `runtime/tests/mcp.rs`
- `runtime/tests/mechanical_sweep_parity.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/analyze_lib.py`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/combine.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/record.py`
- `scripts/analyze/scenario-brief.md`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/lint-governing-size.sh`
- `scripts/mutation/audit-records-085.py`
- `scripts/mutation/compare-and-set-041.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/first-review-scope-086.py`
- `scripts/mutation/outcomes-outside-the-scope-086.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/process-decisions-retained.py`
- `scripts/mutation/project-parts-087.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/prune-project-containment.py`
- `scripts/mutation/prune-project.py`
- `scripts/mutation/resolve-constitutions-size.py`
- `scripts/mutation/review-drops-unowned-086.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/rule-judgments-088.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/mutation/run.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/spec-ownership-086.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/stale-literals.sh`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-audit-record-gate.sh`
- `scripts/tests/test-lint-governing-size.sh`
- `scripts/tests/test-pre-commit-staging.sh`
- `scripts/tests/test-stale-literals.sh`
- `specs/000-slash-commands/review.md`
- `specs/002-project-scaffolding/analysis.md`
- `specs/003-bootstrap-automation/analysis.md`
- `specs/003-bootstrap-automation/review.md`
- `specs/003-bootstrap-automation/tasks.md`
- `specs/009-scenario-targeting/analysis.md`
- `specs/009-scenario-targeting/review.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/tasks.md`
- `specs/014-reclarify-backedge/review.md`
- `specs/016-cross-cutting-rules/review.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/review.md`
- `specs/019-config-decisions/analysis.md`
- `specs/019-config-decisions/review.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/spec.md`
- `specs/021-runtime-boundary/review.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md`
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md`
- `specs/022-deterministic-runtime/scenarios/parser-walker-conventions.md`
- `specs/022-deterministic-runtime/scenarios/resolve-references-cli-exec-wiring.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/scaffolding-primitives.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/023-govern-refinement/review.md`
- `specs/024-rule-loader/review.md`
- `specs/025-rule-opt-out/review.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/027-bootstrap-migration-registry/analysis.md`
- `specs/027-bootstrap-migration-registry/review.md`
- `specs/027-bootstrap-migration-registry/tasks.md`
- `specs/029-bootstrap-runtime-autowire/review.md`
- `specs/029-bootstrap-runtime-autowire/spec.md`
- `specs/033-rule-surface-setting/review.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/035-groom-session-target/review.md`
- `specs/036-quality-cross-rules/analysis.md`
- `specs/036-quality-cross-rules/review.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/review.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/analysis.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/review.md`
- `specs/043-workflows-sunset/review.md`
- `specs/045-decision-state-drift-detection/plan.md`
- `specs/045-decision-state-drift-detection/review.md`
- `specs/047-analyze-findings-durability/review.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/049-rename-govern-to-ductus/review.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/findings-route-by-scope.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/052-spec-supersession-and-consolidation/analysis.md`
- `specs/052-spec-supersession-and-consolidation/review.md`
- `specs/054-remove-supersession/review.md`
- `specs/055-shared-constitution/data-model.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/056-bootstrap-archive-boundary-split/analysis.md`
- `specs/056-bootstrap-archive-boundary-split/review.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/058-findings-route-at-discovery/review.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/059-project-in-a-repository-subdirectory/review.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/analysis.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/review.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/rule-judging-runs-from-repo-scripts.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/tasks.md`
- `specs/061-updates-track-the-latest-release-tag/review.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/064-pi-host-support/analysis.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/review.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/071-command-reference-split/plan.md`
- `specs/071-command-reference-split/review.md`
- `specs/071-command-reference-split/spec.md`
- `specs/072-governing-documents-stay-within-one-read/spec.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/075-agents-md-holds-what-every-session-needs/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/080-deterministic-runtime-efficiency/research.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/review.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/analysis.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/data-model.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/plan.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/review.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/tasks.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/analysis.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/review.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/analysis.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/review.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`
- `specs/088-rule-judgments-cost-only-what-changed/analysis.md`
- `specs/088-rule-judgments-cost-only-what-changed/data-model.md`
- `specs/088-rule-judgments-cost-only-what-changed/plan.md`
- `specs/088-rule-judgments-cost-only-what-changed/review.md`
- `specs/088-rule-judgments-cost-only-what-changed/spec.md`
- `specs/088-rule-judgments-cost-only-what-changed/tasks.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/analysis.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/review.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/tasks.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
