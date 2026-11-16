---
spec: 067-analyze-and-audit-checks
last-run: 2026-11-16T03:06:25Z
reviewed-against: 6a8d8de7a1f574a06720bc4d9a7371e5bb08d401
diff-base: 92e238b8878c00258fa6f7fd585e73e83214de17
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 16
unread:
  - runtime/CHANGELOG.md
  - runtime/src/main.rs
  - runtime/src/primitives/derive_dependencies.rs
  - runtime/src/primitives/derive_references.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/mcp.rs
  - scripts/mutation/owner-reviews-067.py
excluded: 72
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 3a6eec636674aeadf9392c22833cb8389d12d7a6cf5f350af09b853564955671
  scenarios/a-record-cites-a-removed-rule.md: 23bf46c125b0bb16377fcd9958126aa15633ac01e77db6219a4dd3d7303d1627
  scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md: 73b4360b8740a08f023125fdae8fe14b859bc2ac7e60a8a39642ea51e3d08d1a
  scenarios/a-scenario-file-is-checked-against-the-scenario-schema.md: 596fd2546524b3b353e1c077e85e496a3be0d52b44fec54480808571a36c6569
  scenarios/adopter-corpus-link-integrity.md: 5d5aca644cf0573708948c8030ddd1fb161ff263ca1c67af46fb38da30a9b35e
  scenarios/adopter-generator-promotion.md: 1d1829578fb2dbbd96329c1b50ddd9372b8c4aa96bbf7ca25e6ad0e6c4bcef91
  scenarios/an-at-path-import-is-a-reference.md: 38631023408ed48edd5072f1c61394eb5ce8d70ee6d340cf2c79403ba55517a2
  scenarios/analyze-artifact-checks.md: 529dd14e7258557d249aeea8379c45a3f26d6b034a939036f327df572bdf8c46
  scenarios/anchor-reference-kinds.md: c91dc7e263b77ff21f56a8866dcda145172facae7d471c094f96dabff8405795
  scenarios/argument-hint-needs-a-token.md: 130b949e9ca72ab34b48e43d2179fbed5d21ef111b40213180c72ae41bc28ae7
  scenarios/block-element-scanner.md: 0b96c1dc41bd91f898e535c17c8964ef9da5ccf39411d888d8e244c82afe9c38
  scenarios/body-path-existence-family.md: f6150c2ef9a8ae7602a0127d074a311de792f1044d063d1e6d8ee4a6085496e6
  scenarios/check-artifacts-skipped-targets.md: f8a8465285abab1ccf0c222164f63ff918b4b5663901ecf6b5d07e4d5ad5ee2b
  scenarios/criterion-adopter-scope-destinations.md: 43230ed75b6a8962c2db0d8024e0bfb9a9e72b8b372e6cfce945aa15c64e8e2e
  scenarios/criterion-negated-creation-phrasing.md: ca25bb4673581651b0ad883dc160b79e460c8007ec052dac7de3cb306075f802
  scenarios/criterion-non-assertion-phrasings.md: 5626be63a44cf409f82e3f5def18223129fe3dbde20c68d22eb9fe96b7032536
  scenarios/criterion-path-existence-family.md: 3b7a9b04d1dfed381bfb6dbf11d962d12423df113e91fc1eef0c90ac0a80597a
  scenarios/deprecation-labels-are-read-by-the-runtime.md: 7a1de6ee12d3a7668f702453cc8ad4149f2bfdc5e8804a1e01b8746607994c1f
  scenarios/derive-references-results-are-bounded.md: c2db6bbd00c15efdf9ad2ec6059e6c5b69e30d10a1959933e12fb0c0872281bd
  scenarios/derive-references-unstaged-drift-is-reported.md: 163ae170a657e8937515851f4f7ddf4e6e577c59944005d6f44c4431e1517c89
  scenarios/derive-unparseable-frontmatter-is-reported.md: db33d516db986ee67600e1c6bc9f4e4d8b1370bd2a2d0e92a23f96d8e3628368
  scenarios/disposition-drift-is-advisory.md: 567cae5608f75ddc2f1b14d3db198958003022ddaacbb6d997bbf17c7a122436
  scenarios/every-agents-generated-copies-are-excluded.md: 1c8123ced029eea21a1e14606164a1f0cf26db8c00b9cfc5f7fb4c9e734f3fa6
  scenarios/frontmatter-severity-tiers-on-both-sides.md: 8451df2cbe9705e75bf6d1ce36d548810e258be85d5fd4108a9a5cc7c56a8912
  scenarios/generated-reference-copies-are-excluded.md: 4fdeb0ad57aa757a6ed8297c7bbfd1d7ba620a0b89bf6fc520eedc8ef6f71018
  scenarios/link-adjacent-drift-family.md: 41814ebc48e3744fdb053d09ad024bb9da2bc998d3a4767d3b6a2338b4fd7b44
  scenarios/orphan-check-adopter-authored-references.md: f489b8688ad92e92846bc0c5d74d68239c7478a47aedd954aaac38dbd09c7340
  scenarios/orphaned-reference-check.md: f788fea631a7aadb2782fa33aa9f6f54db2b67a64c10f0abfca3a4282a454807
  scenarios/orphaned-reference-historical-roots.md: d77aba08251b5a3acf157ec6180088f8f77e307e8675fce1e5f20e9402ca5b17
  scenarios/per-agent-scaffold-paths-ship-to-adopter.md: 136825c7b3f5131194dbc009e80a1728573b511c0c9fb71cb27e4f5f5314ddef
  scenarios/pointers-into-shipped-files-are-judged.md: 2ebbb6da986b1762caa108b52b3146b13283d2859361154f549c47dfe6d80d0c
  scenarios/scenario-consistency-reads-tasks-history.md: 333b581457ce1c94ef2b00f8bad3afbe22e109ef6cde1231b7e5bdc414cef33b
  scenarios/shared-constitution-references-are-judged.md: 5a930ca02dca49ac14b929ed058f8f701311db1f8123127766689bc928a8f732
  scenarios/sibling-link-grammar-is-the-shared-one.md: 2b4a939505c289b47181ab8c1415318f084fde77bc7f14b4f45af527d4f6570f
  scenarios/sibling-symlink-trust-boundary.md: 27d33965efbc41edd42671597d39605ec4826fce87520b69641c11c39088c0bb
  scenarios/spec-integrity-and-plan-consistency-are-run.md: d6f36ae1fdf21bd78aa8aadbdbcc44ec6a4a8d1e8649583085e522bbdc916893
  scenarios/status-gated-families-are-not-silent.md: 1833246fc09413e940f018c8201286f35ca0ae386a567e7c52c8eac53c7d4059
  scenarios/the-promotion-coverage-line.md: 0d91757bfa43a8311441fd9d1319714c3756e192f0cb6c2dc22bfb35b245d48f
  scenarios/traverse-deps-cycle-check.md: 1561f174e00dc527d05a0f585b4b893807836a2f78df4b111eb258b89abf4a60
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 5
  undispositioned: 0
waivers:
  - rule: QUAL-CLAIM-001
    file: runtime/src/primitives/check_orphaned_references.rs
    reason: "an unparseable framework/migrations.toml reading as watermark attribution is the decided behavior: 067's review round 1 (2be18da6, 2026-10-26) documented it rather than add a skipped entry, and the compiled description states that the two cases read alike; the owner kept that choice on 2026-11-15"
    waived-at: 2026-11-15T22:47:15Z
    waived-by: andy@stone.dev
decisions:
  - key: "other — lexical_in_project (new in task 27), resolve_sibling and check_corpus_links' lexical_join each walk path components with the same Normal/ParentDir/CurDir arms. The three differ where it matters: resolve_sibling and lexical_in_project refuse a rooted path and test containment against different bases, lexical_join drops a root component and tests nothing, so a shared helper would have to take both policies. — `runtime/src/primitives/check_artifacts.rs:2587`"
    outcome: waived
    reason: The three walkers take different policies for a rooted path and for the containment base, so one helper would carry two flags to save about ten lines each; the divergence is the point of each.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: "bug — with an outward link in the feature directory, a done spec reads clean: true with a non-empty skipped (family outward-link, reason target-unparseable) for a scenario carrying unresolved open questions, a review.md with blocking: true, a tasks.md with unchecked boxes or a spec.md, where AC3 makes the spec's own unreadable artifact a blocking finding at done because an unreadable scenario with unresolved questions would otherwise pass the gate built to catch it. The tests a_scenario_linked_outside_the_project_is_skipped_while_the_run_continues and a_spec_linked_outside_the_project_ends_the_run_with_the_skip_alone pin the skip, and the plan records the choice, so this is a design tension between AC3 and the sibling-symlink-trust-boundary scenario rather than an unnoticed gap. check-review-gate reads scenario questions through collect_scenario_open_questions with no guard, so the completion gate does not depend on this result. — `runtime/src/primitives/check_artifacts.rs:298`"
    outcome: waived
    reason: "The plan and the sibling-symlink-trust-boundary scenario decide that a link out is recorded target-unparseable and never opened, with clean: true plus a non-empty skipped meaning partially examined (QUAL-CLAIM-001), and artifact-unreadable names a file the run tried and failed to read, not one it declines to open; the pre-done gate reads the scenario questions itself."
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: "other — 101 of the 116 references Family 39 counts as not judgeable by construction (run on this tree at HEAD: 544 files, 2400 references, exit 0) name a bare basename that exists elsewhere in the tracked tree but not at the citing directory or the project root, for example ductus-procedure.md section Pre-run Migrations in framework/migrations/ductus-rename.md line 16, ductus.md section Derived values in framework/reference/analyze.md line 329, spec.md and data-model.md in many scenarios. The primitive resolves a document only from the citing file's directory and then the project root (stated in its module docs and scenario anchor-reference-kinds), so these are excluded as 'absent' though the document exists; a renamed section behind such a pointer still fails no gate, contrary to the commit's aim that a renamed section fail CI instead of going unseen. The unexamined count is reported on stderr and recorded under anchor-target-unjudgeable, so nothing reads as passed. — `runtime/src/primitives/resolve_anchor.rs:568-644`"
    outcome: waived
    reason: The resolution bases (citing directory, then project root) are the decided design of scenarios anchor-reference-kinds and shared-constitution-references-are-judged, and the excluded references are counted and recorded unexamined, not reported clean. Resolving a bare basename to a unique tracked file would be a new requirement for the owner to decide (a basename can match several files), not a defect of this change.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: other — check-rule-ids tries the retired-rules arm before the shipped-index arm, so a rule a spec lists that the framework still ships but the project never installed (FE-XSS-001 with only a backend file loaded, run on a fixture) is reported retired, not not-installed, and step 5 then records no rule-not-installed unexamined target for it — `runtime/src/primitives/check_rule_ids.rs:91-114`
    outcome: waived
    reason: The scenario passes a listed ID that no loaded rule file defines, which is what the arm does, and a spec lists only a rule it records the retirement of; an author listing a rule the framework still ships has made an authoring error the list's owner can read from the retired state, and changing the precedence is a behavior decision for the scenario, not a mechanical fix.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: "other — Family 40's finding for a due rule tells the remover to re-point or classify each citing file and does not name retired-rules: as the place a record of the retirement is classified, though scenario a-record-cites-a-removed-rule says the remover records it there — `scripts/audit/deprecation-removal.sh:262-263`"
    outcome: waived
    reason: The message is the one 026's scenario family-40-deprecated-rules-are-removed-on-time states and test-deprecation-removal.sh asserts, and it names the classification the remover makes; the observation concerns the pipeline's own audit machinery, not the code under review breaking its contract.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: perf — section_label_lines counts the newlines before the heading (first = content[..heading_start].matches newline count) on every call, a cost proportional to the file prefix per rule heading, and check-rule-ids uses only whether the result is empty, so it pays for the line index it never reads (about 100 headings over the 121 KB security-backend file, a few megabytes of scanning per run) — `runtime/src/primitives/rule_sections.rs:214-218`
    outcome: waived
    reason: Measured by reading, the added scan is a few megabytes of newline counting per check-rule-ids run over the largest rule file, far below the cost of the file reads around it, and one shared reader is what keeps check-rule-ids and read-deprecation-labels from disagreeing.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: convention — the test the_primitive_labels_what_check_rule_ids_calls_deprecated is named for agreement between the primitive and check-rule-ids but asserts only the primitive's own output for a blockquoted label, never calling check-rule-ids; the agreement holds by construction (one shared section_label_lines), and the commit message records that the primitive's tests fail when the shared reader's label token is mutated — `runtime/src/primitives/read_deprecation_labels.rs:328-345`
    outcome: waived
    reason: "QUAL-TEST-001's first Not-flagged clause covers it: the commit message records the mutation (the shared reader's label token) that turns the primitive's tests red, and the property the name claims is held by one shared function rather than by two copies a test could compare."
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: "other — framework/reference/analyze.md is 62,697 bytes against the 50,000-byte read size: 72fd2ce6 cut it to 49,777, the next two commits (0a7787b4 of 041, 12ea0e44 of 063) left it at 49,917 and 50,479, and later commits including this group's f3ce20be (+1,596), a54ef591 (+1,017), 0cc0cc1e (+220), 8f3b4407 (+147) and a42867ae (+18) took it to 62,697; 079's ticked AC18 says it fits one read at that spec's close and no longer holds, and nothing (scripts/lint-governing-size.sh holds only AGENTS.md and the constitution) enforces it — `framework/reference/analyze.md`"
    outcome: waived
    reason: The claim belongs to 079's AC18 and 067's contract states none about the reference's size; the reference is read through read-section by heading, which the command's Instructions require, so a host never reads it whole, and the framework has no size gate for a reference file to hold it to, so this is an observation about the pipeline's own machinery
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: other — an unreadable plan.md on a done spec yields three blocking findings for one file, from link-adjacent-drift, body-path-existence and plan-consistency (reproduced on a fixture), so blocking-findings counts one defect three times — `runtime/src/primitives/check_artifacts.rs:932-958`
    outcome: waived
    reason: Each family records an artifact it could not read through record_unreadable_artifact by design (scenario unreadable-scenario-is-reported), so a family's silence never reads as examined, and the findings are one defect named by three families; collapsing them would make a family's result depend on another's
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: "other — lists_a_file counts any word holding a slash as a listed file, so a placeholder such as N/A or tbd/none under Affected Files passes plan-consistency (reproduced: the section reading N/A or tbd/none raised no finding) — `runtime/src/primitives/check_artifacts.rs:609`"
    outcome: waived
    reason: "The plan decision and 8f3b4407 state the rule on purpose: the section is a planning aid the template calls non-exhaustive, so the blocking check catches only a section left empty or holding its scaffold and never judges a spelling or its content"
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: convention — the plan template's Affected Files comment calls the section a planning aid that need not be exhaustive, but plan-consistency now blocks a plan whose section lists nothing outside comments, and the template says nothing of that block; the constitution's template-rule alignment names template and check as one change, and f3ce20be touched no template — `framework/templates/spec/plan.md`
    outcome: waived
    reason: The scaffold fails the two content checks on purpose, as 8f3b4407's scaffold test requires, while the template's Implements link passes the references-its-spec check by default; an author fills the section at /plan, whose readiness gate and step 8's finding name what is missing, so the alignment the constitution asks for is carried by the check's own message
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: other — 045's data-model row for outside-project still says a checked path's lexical resolution leaves the project; since 7f40a52e a path through a symbolic link that resolves outside it is recorded the same way — `specs/045-decision-state-drift-detection/data-model.md:88`
    outcome: waived
    reason: the row states the lexical half, which still holds, and links 067's check-artifacts-skipped-targets scenario, whose outside-project row (updated by 7f40a52e) states both halves; the closed set's wording is 067's data model's, which names the link case. The fix would land in 045's directory.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: other — 051's AC15 annotation says the dogfooded hook and the frontmatter lint are held by nothing and that 067's task 34 adds the check; Family 41 now holds both, so the annotation describes the prior state — `specs/051-branch-scoped-spec-numbering/spec.md:121`
    outcome: waived
    reason: the annotation is 051's record of the tree on 2026-11-14 and names task 34 as the owed check; 051's task 36 owns its wording before 051's analysis, and the fix would land in 051's directory.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: other — the adopter-corpus-link-integrity scenario's first Behavior bullet says resolution is lexical and never canonicalized, so the result does not depend on symlinks; judged against the new link check it still holds (the target is joined textually and never canonicalized, the base is resolved only to compare, and a link out is refused, which keeps the answer independent of what lies outside the checkout), but the Behavior and Edge Cases say nothing of a target that exists beyond the base being reported broken or of a file under a link out being skipped; the decision is stated in 067's data model, the compiled description and the changelog, as task 33 asked — `specs/067-analyze-and-audit-checks/scenarios/adopter-corpus-link-integrity.md:21-25`
    outcome: waived
    reason: task 33 names the data model and the two descriptions as the places that state the decision, and all three do; the scenario's requirements (report every relative link that resolves to nothing, lexically, with spans stripped and exclusions counted) still hold for every target inside the base.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: "other — task 35 (and task 33's second bullet at line 29) cite 026's analysis.md as recording the owner's 2026-11-15 deferral, but 026's committed analysis.md at HEAD (last commit 25e4234e, 2026-11-13) records deferred: 0 and names neither 067 nor either finding — `specs/067-analyze-and-audit-checks/tasks.md:47`"
    outcome: waived
    reason: the authority for each deferral is the owner's decision, which the task commits b63f0cf3 and a15b6cf6 record; the record the sentence cites is 026's own analysis run to write.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: other — compute, bound and Labels in read_deprecation_labels.rs, and bound in resolve_anchor.rs, are pub but called only within their own module and its tests (run is the one entry); the check_corpus_links shape they copy is pub because main.rs calls its bound — `runtime/src/primitives/read_deprecation_labels.rs:44`
    outcome: waived
    reason: each is called, so none is dead code, and run's doc comment links to them, which a private item would turn into a private intra-doc link; the visibility mirrors the sibling primitives' compute/bound split and widens nothing a caller can misuse.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: perf — leaves_project_by_link resolves the project root through canonical_base once per candidate path, where check-corpus-links' Containment resolves its base once per scan; each call is a handful of lstat and readlink calls on the root's components — `runtime/src/primitives/check_artifacts.rs:2610`
    outcome: waived
    reason: the cost is a few system calls per backticked path in one feature's criteria and body, no result changes, and hoisting the base is a signature change through three call sites, not a mechanical edit.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: other — Containment::judge counts an unresolvable base (canonical None) as leaving, so every link in the scan would be reported broken with guidance saying the target leaves the project, when the cause is the base; the failure is loud and the base is the project root the run already opened — `runtime/src/primitives/check_corpus_links.rs:574-580`
    outcome: waived
    reason: fail-closed and visible (every link is a finding, exit non-zero), the base is a directory the process is running in so the state needs the root removed mid-run, and a distinct guidance is a new state, not a mechanical edit.
    decided-at: 2026-11-15T22:47:15Z
    decided-by: rule
  - key: rule BE-INPUT-004 at runtime/src/primitives/check_review_gate.rs
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "23"
    decided-at: 2026-11-15T22:47:15Z
    decided-by: andy@stone.dev
  - key: "other — the two call sites that print each refused spec to stderr (report_refused in the DeriveDependencies and DeriveReferences arms of main) are covered by no test: refused_lines (the text) and the two gate functions are tested, but removing either report_refused call, or handing it a different list, fails nothing. The commit message and compare-and-set-067.py say no subprocess can induce a refusal, which needs a change between a read and its write; the gap is recorded, not hidden. — `runtime/src/main.rs:1198`"
    outcome: waived
    reason: QUAL-TEST-001 governs a test that passes whether or not the named behavior holds, and its Not-flagged clause covers a behavior whose commit message or comment records how it was shown to fail; the stderr text and the exit gate each have a test that fails when their behavior is removed, and the call-site gap is stated in compare-and-set-067.py's header with the reason no process-level test can reach it.
    decided-at: 2026-11-16T03:06:01Z
    decided-by: rule
  - key: "perf — a spec the derivations write is judged for containment three times: leaves_project (the soft pre-check that names it unparseable), ReadSnapshot::read, and write_within's re-check under the lock, each walking the path's components and resolving the project root's links. The walks cost a handful of lstat and readlink calls per spec and one per drifted spec. — `runtime/src/primitives/derive_dependencies.rs:237`"
    outcome: waived
    reason: The pre-check is what turns an outward link into a named unparseable entry instead of an aborted run, and the two checks inside the guard are what BE-INPUT-004 requires of a re-read; the added cost is a few system calls per spec and bounded by the corpus, with no result changed.
    decided-at: 2026-11-16T03:06:01Z
    decided-by: rule
  - key: other — the shipped hook's header (framework/bootstrap/hooks/ductus-pre-commit lines 11-22) describes the two derivations' non-zero exits as a dependency cycle and an unwritable spec and does not name the new refused exit. Each statement remains true, and a refused spec is named on stderr by its own line saying to run again, which is what a stopped commit shows. — `framework/bootstrap/hooks/ductus-pre-commit:11-22`
    outcome: waived
    reason: The header lists the exits it names and asserts no exhaustive set, so no claim in it is false; the handling path BE-TXN-004 asks for is stated on the line the operator reads when the hook stops, and task 36's text records that the shipped hook runs each derivation under set -euo pipefail and so stops without a change.
    decided-at: 2026-11-16T03:06:01Z
    decided-by: rule
  - key: other — compute_between puts a test seam (a boxed FnMut called before each write) into the production signature of two primitives, and the match block that turns ChangedSinceRead into a refused entry is written out twice, once per primitive, beside two otherwise parallel compute loops. — `runtime/src/primitives/derive_references.rs:243`
    outcome: waived
    reason: The seam is the pattern session.rs's lock_waiting and mod.rs's Stage hook already use to put a change between two steps, production passes a no-op closure, and the two primitives already keep separate parallel loops by design (derive-dependencies writes [] for an empty index, derive-references removes the key), so a shared helper would cross that deliberate asymmetry for about ten lines.
    decided-at: 2026-11-16T03:06:01Z
    decided-by: rule
  - key: other — the fix round pins the pairing sentence's new wording ('only when `unparseable`, `unwritable` and `refused` are also empty') in the description-phrase table for DERIVE_REFERENCES only; the same sentence in DERIVE_DEPENDENCIES has no pinned entry (a NOTE of the round's re-read) — `runtime/tests/mcp.rs:466-469`
    outcome: waived
    reason: "A coverage gap in the description-phrase table, not a chore: the round rewrote the one pin that already existed for this sentence and added none, the table samples phrases of each description rather than pinning every sentence, and the DERIVE_DEPENDENCIES copy was read whole by the review and by the round's re-read."
    decided-at: 2026-11-16T03:06:01Z
    decided-by: rule
---

# Review — 067-analyze-and-audit-checks

## Summary

067's review of task 36, the window from `92e238b8`, 067's last review, to HEAD: task 36 (`2a0aa755`), 067's hunk of `efc517e9`, `935a91a2`'s Resolved Question, and the task-text and citation commits `b88a8404`, `fb319f2d` and `61272c59`. One agent ran all five passes and read all nine loaded backend and cross rule files whole. It read 8 of the 16 scope entries whole: 067's spec, plan, data model and tasks, the bounded-results scenario, and two mutation lists, and the unparseable-frontmatter scenario, which the fix round brought into the scope and its re-read read whole at its final content; and the other 36 scenarios, outside the scope. The rest were read at the window's hunks with every touched function, type and test whole: `compute`, `compute_between` and `bound` in both derivations, the two CLI gates and `refused_lines` in `main.rs`, both result structs, both descriptions, the CHANGELOG entry and the re-anchored mutants. No rule finding and no broken contract. Two chores fixed (`326691cd`): the derivations' pairing contract (an empty `updated` is examined-and-clean only when the lists naming what was not are empty) now names `refused` at its six sites, the scenario among them, and the CHANGELOG no longer reads as if the `refused` list were absent from the lists file. The fix round was re-read by an independent agent that read the whole contract, spec, data model and all 38 scenarios, 227,890 bytes: clean, with one NOTE, the DERIVE_DEPENDENCIES copy of the pairing sentence unpinned in `runtime/tests/mcp.rs`, and the loop's closing round was empty. An earlier re-read of the same round, which read a few KB of the contract, was discarded and the round re-briefed from its base. The rule waived five observations with their reasons, the NOTE among them. The one stored decision on a file the run read stands and is carried.

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

- other — the pairing contract 'an empty updated means examined-and-clean only when unparseable and unwritable are also empty' was not extended to refused, though 067's data model says a refused spec is never examined-and-clean. The sentence stands in the DERIVE_DEPENDENCIES and DERIVE_REFERENCES descriptions, the unparseable doc comment on both result types, the scenario derive-unparseable-frontmatter-is-reported (its Behavior), and a runtime/tests/mcp.rs assertion pins the phrase for DERIVE_REFERENCES. A writing run whose only drifted spec is refused returns an empty updated beside empty unparseable and unwritable, so a reader applying the stated pairing alone reads it clean; the exit status and the refused list do say otherwise. — `runtime/src/schema/descriptions.rs:56-57` — **fixed** (decided by the rule)
- convention — the CHANGELOG entry for task 36 says the refused list 'is absent in the file form and in the lists file whole', which reads as absent from the lists file too; the code and data model put it in the lists file whole (a refused spec moved to the file is asserted there by the new bound tests). — `runtime/CHANGELOG.md:19-20` — **fixed** (decided by the rule)
- other — the two call sites that print each refused spec to stderr (report_refused in the DeriveDependencies and DeriveReferences arms of main) are covered by no test: refused_lines (the text) and the two gate functions are tested, but removing either report_refused call, or handing it a different list, fails nothing. The commit message and compare-and-set-067.py say no subprocess can induce a refusal, which needs a change between a read and its write; the gap is recorded, not hidden. — `runtime/src/main.rs:1198` — **waived**: QUAL-TEST-001 governs a test that passes whether or not the named behavior holds, and its Not-flagged clause covers a behavior whose commit message or comment records how it was shown to fail; the stderr text and the exit gate each have a test that fails when their behavior is removed, and the call-site gap is stated in compare-and-set-067.py's header with the reason no process-level test can reach it. (decided by the rule)
- perf — a spec the derivations write is judged for containment three times: leaves_project (the soft pre-check that names it unparseable), ReadSnapshot::read, and write_within's re-check under the lock, each walking the path's components and resolving the project root's links. The walks cost a handful of lstat and readlink calls per spec and one per drifted spec. — `runtime/src/primitives/derive_dependencies.rs:237` — **waived**: The pre-check is what turns an outward link into a named unparseable entry instead of an aborted run, and the two checks inside the guard are what BE-INPUT-004 requires of a re-read; the added cost is a few system calls per spec and bounded by the corpus, with no result changed. (decided by the rule)
- other — the shipped hook's header (framework/bootstrap/hooks/ductus-pre-commit lines 11-22) describes the two derivations' non-zero exits as a dependency cycle and an unwritable spec and does not name the new refused exit. Each statement remains true, and a refused spec is named on stderr by its own line saying to run again, which is what a stopped commit shows. — `framework/bootstrap/hooks/ductus-pre-commit:11-22` — **waived**: The header lists the exits it names and asserts no exhaustive set, so no claim in it is false; the handling path BE-TXN-004 asks for is stated on the line the operator reads when the hook stops, and task 36's text records that the shipped hook runs each derivation under set -euo pipefail and so stops without a change. (decided by the rule)
- other — compute_between puts a test seam (a boxed FnMut called before each write) into the production signature of two primitives, and the match block that turns ChangedSinceRead into a refused entry is written out twice, once per primitive, beside two otherwise parallel compute loops. — `runtime/src/primitives/derive_references.rs:243` — **waived**: The seam is the pattern session.rs's lock_waiting and mod.rs's Stage hook already use to put a change between two steps, production passes a no-op closure, and the two primitives already keep separate parallel loops by design (derive-dependencies writes [] for an empty index, derive-references removes the key), so a shared helper would cross that deliberate asymmetry for about ten lines. (decided by the rule)
- other — the fix round pins the pairing sentence's new wording ('only when `unparseable`, `unwritable` and `refused` are also empty') in the description-phrase table for DERIVE_REFERENCES only; the same sentence in DERIVE_DEPENDENCIES has no pinned entry (a NOTE of the round's re-read) — `runtime/tests/mcp.rs:466-469` — **waived**: A coverage gap in the description-phrase table, not a chore: the round rewrote the one pin that already existed for this sentence and added none, the table samples phrases of each description rather than pinning every sentence, and the DERIVE_DEPENDENCIES copy was read whole by the review and by the round's re-read. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/consolidate.md` — 051-branch-scoped-spec-numbering
- `.claude/commands/ductus/fold.md` — 051-branch-scoped-spec-numbering
- `.claude/commands/ductus/prune.md` — 022-deterministic-runtime, 041-task-pruning
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/fold.md` — 051-branch-scoped-spec-numbering
- `.pi/prompts/ductus-analyze.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-consolidate.md` — 051-branch-scoped-spec-numbering
- `.pi/prompts/ductus-fold.md` — 051-branch-scoped-spec-numbering
- `.pi/prompts/ductus-prune.md` — 022-deterministic-runtime, 041-task-pruning
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 041-task-pruning, 050-constitution
- `framework/commands/analyze.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/consolidate.md` — 051-branch-scoped-spec-numbering
- `framework/commands/fold.md` — 051-branch-scoped-spec-numbering
- `framework/commands/prune.md` — 022-deterministic-runtime, 041-task-pruning
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/reference/fold.md` — 051-branch-scoped-spec-numbering
- `runtime/src/primitives/bounded_lists.rs` — 022-deterministic-runtime
- `runtime/src/primitives/check_artifacts.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/check_review_gate.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/invalidate_review.rs` — 051-branch-scoped-spec-numbering
- `runtime/src/primitives/mod.rs` — 022-deterministic-runtime, 041-task-pruning, 057-analyze-artifact-and-record-relocation
- `runtime/src/primitives/prune_plan.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/prune_project.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/prune_tasks.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/relocate_audit_records.rs` — 057-analyze-artifact-and-record-relocation
- `runtime/src/primitives/rewrite_spec_links.rs` — 051-branch-scoped-spec-numbering
- `runtime/src/primitives/write_analysis.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/schema/paths.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/session.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/tests/golden/review-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/line_ending_discipline.rs` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/compare-and-set-041.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/compare-and-set-051.py` — 051-branch-scoped-spec-numbering
- `scripts/mutation/compare-and-set-057.py` — 057-analyze-artifact-and-record-relocation
- `scripts/mutation/compare-and-set-066.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/gate-cross-spec-containment.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/prune-project-containment.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/prune-project.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/reopen-measure.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/rewrite-spec-links.py` — 051-branch-scoped-spec-numbering
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 041-task-pruning, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 041-task-pruning, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
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
- `specs/051-branch-scoped-spec-numbering/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/051-branch-scoped-spec-numbering/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md` — 051-branch-scoped-spec-numbering
- `specs/051-branch-scoped-spec-numbering/spec.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/051-branch-scoped-spec-numbering/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/057-analyze-artifact-and-record-relocation/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/057-analyze-artifact-and-record-relocation/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/057-analyze-artifact-and-record-relocation/spec.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/057-analyze-artifact-and-record-relocation/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 080-deterministic-runtime-efficiency
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/review.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-cross-spec-impact-gate.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 057-analyze-artifact-and-record-relocation, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
