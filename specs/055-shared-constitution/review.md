---
spec: 055-shared-constitution
last-run: 2026-11-22T00:34:39Z
reviewed-against: 8b3e9dd64575cc67f7053981151f75a11982084a
diff-base: 53fde26de34ba94a7347bf0e9c73240749643a86
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 8
unread:
  - framework/bootstrap/govern.md
excluded: 99
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: a8134b0d6cb8dff8f7cab2925bdf0843b17dfc794a5631f8f6c02ae9e7380c99
  scenarios/a-registered-source-is-named-with-its-description.md: f04ffbef1f2349e9f23d8446c20490833b592ae5edb7dad11ed94bbe9c2166f3
  scenarios/target-reports-each-constitutions-size.md: 7661b1fccbf76a02af4467a2a40ebf845cfc302b6104f84b831ff25238b6bd3c
criteria-judged: 13
criteria-met: 13
criteria-verdicts:
  - label: AC1
    digest: 32db87cecf366ad03554dbf44a257e421dd24ca457647b4c27dfc8b4cdf88536
  - label: AC2
    digest: ea8f6c175bb7fc219b5151a91dc3889842f8e611a1d6b8902c0c6bd43b1aa616
  - label: AC3
    digest: 889e65f42c77f718d5a6cf2e66bec927890fced9324d7f5f774fcbd58b34b785
  - label: AC4
    digest: 965dc5f8f2a8b0d478265189c04de3845062806e2dedb8395610f3a5223f0d09
  - label: AC5
    digest: fb5b6fc362b3e8d4bb72ff6da5c5e0c6e42e5c3a2587ce6de463d48a7f420a13
  - label: AC6
    digest: 19aec552376e755e19d8f226fb6f967a2e50ceb3af1b8d6a02a50852207ab458
  - label: AC7
    digest: 26cd075b6fa7ce587ba6f10b2304dd08d0ddc6a96d3e0c4df22e1dc53e939e2d
  - label: AC8
    digest: 61143ca6a7cbf03935d9bc8a56e2bb6cf5d9fd899c5b4cd48bf00c3c015ba15a
  - label: AC9
    digest: b6fea22a95e2fd49a5992bca037618f4735e3a83cfe1578ef4908cbe1d941098
  - label: AC10
    digest: abc24a563983f18ea4313f801fabc3c11fc8993d9017b46210f83eb1dc2b36c6
  - label: AC11
    digest: 6bd8edfc25beeebc5f8e4c9c910828b530ba1039743eaaf0f927af9737bf3c31
  - label: AC12
    digest: 6185cfcb14e50312b98d926c7a6142ded26883d8a75d880f66daaede94b30853
  - label: AC13
    digest: b9dea0cd41d2542bf91aaf9a50182773f6cd278beb29e593e1f31bc136c08afb
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "other: the module doc keeps 'a dangling @import yields a constitution that is simply not loaded' — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: it recounts the 048 AC10 incident, which was an @import, and 027's migration-chain-reference-integrity scenario keeps the same sentence as its record
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a BOM, or punctuation such as ( or \" before the `@`, keeps an @path import from being read as a reference — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: "contrived: the template and the bootstrap write each import at the start of a line, and 067's an-at-path-import-is-a-reference records both cases as edge cases"
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: --ref=main adopters get the migration before the @path orphan-check fix, which ships only in the runtime — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: a runtime change reaches adopters only through a ductus-v tag, and the next release is the owner's to cut; the gap closes there
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a legacy root `@import constitution.md` becomes `@constitution.md`, an orphan no check reports — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: the root path is not a managed root, the line was equally silent before, and `@import constitution.md` never loaded anything
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: 040's spec-root-rule-stated-once rests a rejected option on the CLAUDE.md premise 017 AC16 states — `specs/040-configurable-specs-dir/scenarios/spec-root-rule-stated-once.md`"
    outcome: deferred
    target: specs/050-constitution/tasks.md
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "security: the migration turns @import lines that loaded nothing into live imports — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: step 6 names every rewritten path for the operator, Claude Code asks before loading an import from outside the working directory, and no loaded security rule fires on a local text rewrite
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: the migration's code-span sentence repeats what 'begins with @import' already implies — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: "harmless: it names the edge case the scenario lists"
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: the adopter runs met pre-existing bootstrap behavior: v0.28.0 cannot be walked at its tag, the Frontmatter Migration's clean-tree precheck refuses after a migration writes under specs/, AGENTS.md placeholders an old release left literal are never repaired, and v0.59.11's dead old-form constitutions region survives — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: observations about the bootstrap's own machinery made while reviewing this change, none caused by it and none blocking it; the dead region is already an inbox item
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: length_on_disk repeats check_artifact_size's open-metadata-len chain — `runtime/src/primitives/resolve_constitutions.rs`"
    outcome: waived
    reason: "A two-file refactor with no behavior change: the repeated chain is three standard-library calls, and lifting it changes check-artifact-size's code for no reader."
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "bug: duplicate_paths compares raw path strings, so gov and ./gov are not grouped — `runtime/src/schema/constitutions.rs`"
    outcome: waived
    reason: "The contract, the spec's Edge Case and the data model, groups entries naming the same path as written, and gov and ./gov are different values, each measured; the DuplicateConstitutionPath doc comment that said \"checkout\" is fixed in this run."
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description.
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "other: 'truncated to 100 characters' keeps 100 characters and appends an ellipsis (fix round 1's NOTE) — `specs/055-shared-constitution/data-model.md`"
    outcome: waived
    reason: 100 is the characters kept and the ellipsis marks the cut, as the tests assert; no reader is misled about the bound.
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "other: the result's bound rests on the registry's own size rather than a fixed maximum (fix round 1's NOTE) — `specs/055-shared-constitution/data-model.md`"
    outcome: waived
    reason: "The bound the owner chose (2026-11-09): descriptions capped, the result bounded by the project's own operator-written registry."
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "convention: docs/shared-constitution.md's authoring guideline 'Anything that loosens a framework rule' leaves out the exception (fix round 1's NOTE) — `docs/shared-constitution.md`"
    outcome: waived
    reason: An authoring guideline for shared sources; the exception is a framework rule's own opt-in, which a shared author meets in that rule's text.
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "convention: the mutation list's docstring says one line per source, which fits M15 but not M16 (fix round 3's NOTE) — `scripts/mutation/resolve-constitutions-size.py`"
    outcome: waived
    reason: Each mutant's own description states what it guards; fixing the docstring's wording would start a fourth re-read round for no reader's gain.
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "other: no test pins the exec walker's halt on a rejected `[paths] specs-root`, which Walker::new reaches through derive_analyze_seeds since Host::load does not validate it — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: The walker path is 022's task 12 and outside 055's contract; the halt was driven on a fixture here (exit 1, config-unreadable, naming the file), and a review creates work only for a gating finding.
    decided-at: 2026-11-10T18:58:49Z
    decided-by: rule
  - key: "perf: write-review loads [paths] four times per call, and 022's task 12 made own_record_path and analyze_freshness_of fallible to carry the repeat, so a config edited mid-call can fail the call after review.md is written — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: Each load predates this window, which made them fallible as 022's task 12 requires; passing the loaded root down is a refactor no caller sees, and the post-write error, reachable only by a config edited mid-call, is the one analyze_freshness_of's doc comment states.
    decided-at: 2026-11-10T18:58:49Z
    decided-by: rule
  - key: "other: under ductus exec a malformed shared session default halts as config-unreadable, since Host::load reads it first, so main.rs's session-unreadable branch is reached only by the process's own record — `runtime/src/main.rs`"
    outcome: waived
    reason: The branch is still reached by a malformed own record, the exec test pins the code the default halts with, and either code labels a halt that names the file.
    decided-at: 2026-11-10T18:58:49Z
    decided-by: rule
  - key: "other: resolve_constitutions, check_artifact_size and discover_rule_files probe the config with is_file or exists, so a directory named config.toml reads as absent to them while Paths::load halts with an I/O error — `runtime/src/primitives/resolve_constitutions.rs`"
    outcome: waived
    reason: "Contrived input: every writer and the exec walk reach Paths::load first, which halts naming the file, and unifying the readers changes no behavior a caller sees."
    decided-at: 2026-11-10T18:58:49Z
    decided-by: rule
---

# Review — 055-shared-constitution

## Summary

Five-pass review of 055's reopening (diff-base 53fde26d, base-source reopen) at HEAD 8b3e9dd6. Its code commit is 2983d2a3 (task 19, AC13: `/ductus` maintains `AGENTS.md`'s constitutions region for every layout), with the spec-only 65052153 and f84932e8. Passes (part P2, one reviewer for 078 and 055) read those commits' hunks in each scope file, and the files whole where a behavior needed context, against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding and three observations, each fixed in this run. `docs/shared-constitution.md`'s step 4 now says `/ductus` writes a link block in `AGENTS.md` under every agent and an import block in `CLAUDE.md` under Claude Code; the plan's Overview states AC13's two blocks, and its Affected Files gain the guide and the migration; and `framework/bootstrap/ductus.md` writes the `AGENTS.md` line with its path in angle brackets, so a space in the path leaves the link whole. P2 re-ran its passes over the fix, which then left AC13, task 19, the guide's How it loads and the plan stating the old form; they now state it as the bootstrap writes it, AC13 included (055 is in-progress, so no back-edge), and a second re-run reported nothing. Five stored waivers still apply. The criteria step (part C3) judged every criterion individually: 13 of 13 met, `resolve-constitutions` and `merge-managed-block` driven with the release binary on fixtures; AC13 rests on a `claude-style` scratch adoption that 2983d2a3 records and, for `pi`, a fixture the owner accepted on 2026-11-21. Examined 7 of 8; unread: `framework/bootstrap/govern.md`, confirmed byte-identical to `ductus.md` by `cmp`.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

13 of 13 acceptance criteria judged met.

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

- AC1 — met: Driven: resolve-constitutions on no config and on an empty [constitutions] table both return loaded [], skipped [], examined 0; `write_review.rs` `render_unexamined_governance` returns *None.* (test `unexamined_governance_is_none_when_no_constitution_is_registered`); the as-written divergence (empty imports block, always-rendered section) is annotated in the criterion and true.
- AC2 — met: `framework/commands/target.md` step 4 loads `.ductus/constitution.md` and every resolve-constitutions loaded document once per session, so each command inherits them with no per-command step; claim is procedure prose, not drivable by the binary.
- AC3 — met: `framework/bootstrap/ductus.md` Shared Files table keeps `framework/constitution.md` -> `.ductus/constitution.md` under update; the Shared constitution imports step writes only managed blocks in `CLAUDE.md/AGENTS.md` and names no [pinned] requirement.
- AC4 — met: `target.md` step 4 reports loaded sources by alias with `bytes/threshold` and description; documents load whole and separately, so each rule is attributable to its document; resolve-constitutions loaded records carry alias, repo, path, document (driven on a 5-entry fixture).
- AC5 — met: Driven: a missing checkout returns skipped outcome not-checked-out and a checkout with no `constitution.md` returns no-constitution-document, each by alias; `write_review.rs` renders each under ## Unexamined governance and `write_analysis.rs` counts constitution-unresolved; tests in `resolve_constitutions.rs` (`missing_checkout_is_not_checked_out`, `checkout_without_document_is_its_own_outcome`) and `write_review.rs` (`unexamined_governance_names_a_source_that_could_not_be_read`).
- AC6 — met: Driven: repo 'nope', blank path, alias 'a b' and a missing path field each exit 1 with a message naming the file, alias and field (`schema/constitutions.rs` `from_toml_str` validates inside the constructor; tests `a_repo_that_is_not_url_shaped_halts_instead_of_resolving_loaded`, `an_empty_path_halts_instead_of_blaming_the_checkout`).
- AC7 — met: Driven: aliases zeta, alpha, dup, gone, wrong came back loaded in alias order (alpha, dup, zeta) regardless of file order; `schema/constitutions.rs` uses a BTreeMap; tests `iteration_is_alias_order_not_document_order` and `multiple_entries_load_in_alias_order`.
- AC8 — met: `write_review.rs` `render_unexamined_governance` names each skipped source with its reason and, on an unreadable registry, reports that none was loaded; `write_analysis.rs` records constitution-unresolved and constitution-registry-unreadable in unexamined-by-reason (documented in `framework/reference/analyze.md` Unexamined targets); tests `unexamined_governance_`* and the `write_analysis` constitution-unresolved test; with none registered the report reads *None.*.
- AC9 — met: Driven merge-managed-block with marker ductus:constitutions: on a template `AGENTS.md` only the region between the markers changed (diff showed one added line), and a markerless file got the block appended with its text above preserved byte-for-byte; `ductus.md` Shared constitution imports never targets `.ductus/constitution.md`; the as-written divergence is annotated and true.
- AC10 — met: `framework/constitution.md` Governance sources states the floor, project > shared > framework and that nothing enforces it, anchored at the governance-precedence marker; the only other occurrences of the order are pointers (`ductus.md` Project Configuration, `target.md` step 4, `docs/shared-constitution.md`, 050 spec), and grep for 'project > shared' finds the constitution and spec 055 alone.
- AC11 — met: Driven: five registered entries were all examined and the three that resolve were all loaded, two on one path as duplicate-paths [alpha, dup]; tests `multiple_entries_load_in_alias_order` and `two_entries_sharing_a_path_are_reported_as_duplicates`.
- AC12 — met: Driven: zeta's description 'Z   rules' came back as 'Z rules' (whitespace collapsed); `resolve_constitutions.rs` `normalize_description/truncate` (100 characters) and `a_skipped_entry_still_carries_its_description`, `an_embedded_newline_is_collapsed_before_it_reaches_a_report`; `write_review.rs` adds the description to each Unexamined governance bullet; an entry without one carries none. Did not drive a >100-character value or a skipped entry's description with the binary; the unit tests cover them.
- AC13 — met: `framework/bootstrap/ductus.md` Shared constitution imports steps 2-3 target the native rules file and `AGENTS.md` for every layout, with '- See [{alias}]({document-path})' links, appended with markers when `AGENTS.md` has none (reproduced with merge-managed-block on a markerless file and on the template); `govern.md` is byte-identical (diff -q); the template comment holds. The recorded evidence in commit 2983d2a3 is a real claude-style scratch adoption; the pi half rests on the pi-layout fixture and the owner's acceptance (2026-11-21), and I ran no full adoption of either layout.

## Observations

- convention: `docs/shared-constitution.md` step 4 of Setting one up says /ductus 'installs or refreshes the managed import block in the project's native rules file', which AC13 made incomplete: `AGENTS.md` carries the block under every layout, and the sibling §How it loads was corrected in 2983d2a3 while this sentence was not. — `docs/shared-constitution.md` — **fixed** (decided by the rule)
- convention: `plan.md`'s Overview still describes the block as one 'layout-derived managed block in the adopter's native rules file' imported under claude-style and linked under the `AGENTS.md` layouts, and its Affected Files give `agents.md` the purpose 'Managed-block marker for `AGENTS.md` layouts' and omit `docs/shared-constitution.md` and the migration's Background that task 19 edited; only Technical Decisions states AC13. — `specs/055-shared-constitution/plan.md` — **fixed** (decided by the rule)
- bug (low confidence): the `AGENTS.md` line `- See [{alias}]({document-path})` carries the path unescaped, while the `CLAUDE.md` line escapes each space; a registered path holding a space makes an invalid link destination, and AC13 now writes that line into every claude-style project's `AGENTS.md`. The step's own space-escaping branch shows such a path is expected. — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- convention: the angle-bracket link the fix gives the bootstrap left AC13, task 19's item, the guide's How it loads and the plan stating the link without the brackets — `specs/055-shared-constitution/spec.md` — **fixed** (decided by the rule)
- other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md` — **waived**: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
- convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md` — **waived**: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description. (decided by the rule)
- other: 'truncated to 100 characters' keeps 100 characters and appends an ellipsis (fix round 1's NOTE) — `specs/055-shared-constitution/data-model.md` — **waived**: 100 is the characters kept and the ellipsis marks the cut, as the tests assert; no reader is misled about the bound. (decided by the rule)
- other: the result's bound rests on the registry's own size rather than a fixed maximum (fix round 1's NOTE) — `specs/055-shared-constitution/data-model.md` — **waived**: The bound the owner chose (2026-11-09): descriptions capped, the result bounded by the project's own operator-written registry. (decided by the rule)
- convention: docs/shared-constitution.md's authoring guideline 'Anything that loosens a framework rule' leaves out the exception (fix round 1's NOTE) — `docs/shared-constitution.md` — **waived**: An authoring guideline for shared sources; the exception is a framework rule's own opt-in, which a shared author meets in that rule's text. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-review.md`
- `docs/analyze.md`
- `docs/developing-ductus.md`
- `docs/rules.md`
- `framework/commands/analyze.md`
- `framework/commands/review.md`
- `framework/constitution.md`
- `framework/reference/analyze.md`
- `framework/reference/review.md`
- `runtime/CHANGELOG.md`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/judge-brief.md`
- `scripts/analyze/record.py`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/criterion-ticks-outside-the-digest.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/reachable-trigger-bar-078.py`
- `scripts/mutation/rule-files-left-to-analysis-020.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/050-constitution/spec.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/082-rule-assessments-converge-across-runs/analysis.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/review.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/082-rule-assessments-converge-across-runs/tasks.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
