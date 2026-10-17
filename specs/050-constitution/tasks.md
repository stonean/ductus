# 050 — Constitution Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Classify every rule-bearing `AGENTS.md` entry

Scope is `Workflow`, `Gotchas`, `Boundaries`, `Design Principles`. `Project
Structure` and `Tech Stack` describe this repository rather than stating rules
and are out of scope. Run against the file as it stands, not the survey's counts.

- [x] Enumerate every entry in the four rule-bearing sections
- [x] Assign each exactly one verdict — universal, borderline, project-only
- [x] Apply the reword test to each borderline entry: promoted iff it restates without repo-only machinery **and** without losing what makes it actionable
- [x] Record one reason per entry — for a promoted borderline, name the machinery removed and assert the rule still bites; for a rejected one, name what was lost
- [x] Write the table into `plan.md` as the AC1 audit trail

- **Done when**: every entry in the four sections has exactly one verdict and a reason in `plan.md`, and the counts of each verdict are stated.

## 2. Promote the universal rules into the constitution

- [x] For each promoted entry, choose the existing section whose subject it shares; open a new section only where the rule genuinely stands alone
- [x] Write the canonical normative text as a bullet in that section, worded so it holds for an adopter — citing the constitution, the pipeline commands, the artifacts, or runtime primitives, never a path that exists only here
- [x] Confirm no existing `<!-- §anchor -->` is renamed, removed, or displaced

- **Done when**: every entry classified universal has its canonical text in `framework/constitution.md`, and the anchor set before and after the change is identical.

## 3. Rewrite the promoted entries in `AGENTS.md` as pointers

- [x] Replace each promoted entry's normative text with a line naming its constitution section and stating nothing the constitution does not
- [x] Leave every project-only entry byte-identical
- [x] Grep each promoted rule's distinctive phrasing and confirm it finds one normative statement and one pointer

- **Done when**: no promoted rule is stated twice, every project-only entry is unchanged, and AC3's grep holds for each promotion.

## 4. Add the criterion-verification rule

- [x] Write the rule into the constitution: a spec's ticked acceptance criteria are verified against the tree before it closes, and a ticked criterion is a claim to be re-earned rather than a fact already banked
- [x] State both gaps that make it necessary — that `check-artifacts`' `criterion-path-existence` family examines `done` specs only, and that a false claim whose paths all resolve is not detectable by it

- **Done when**: the constitution states the rule and both gaps, in adopter-neutral terms.

## 5. Add the mechanical-edit test to §spec-lifecycle

- [x] State whether an edit that changes no claim — a typo or sweep-residue repair in a `done` spec's body — is a mechanical edit
- [x] Phrase it as a test the next case can be decided against, not a fourth enumerated case
- [x] Confirm the three existing enumerated cases still read correctly beside it

- **Done when**: §spec-lifecycle carries the test, and the held `045` chore is decidable against it without further judgment.

## 6. Record the ownership split in §canonical-sources

- [x] Add a row naming this spec as the canonical home for constitution-content work, distinct from the rule that a spec changing behavior still amends the principle its change contradicts

- **Done when**: §canonical-sources carries the row, so a reader finds the ownership split where they already look.

## 7. Resolve the held `045` chore

- [x] Apply the new test to the chore and record the outcome
- [x] If licensed as mechanical, repair the one-word sweep residue in `045`'s spec body and confirm the spec stays `done`
- [x] Remove the item from `specs/inbox.md`

- **Done when**: the chore is resolved or explicitly re-parked with the test's reasoning, and `specs/inbox.md` reflects it.

## 8. Verify

- [x] `scripts/audit/run-all.sh` clean, Family 1 included — run **after** committing, since history-reading families cannot see uncommitted work
- [x] `npx markdownlint-cli2` clean over `framework/constitution.md` and `AGENTS.md`
- [x] The **Shared Files** manifest row copying `framework/constitution.md` to `.ductus/constitution.md` is intact, so adopters receive every promoted rule
- [x] `version`, `runtime/Cargo.toml` and `runtime/CHANGELOG.md` untouched, and Family 20 clean

- **Done when**: every check above passes on a committed tree and no criterion is ticked that the tree does not support.

## 9. Completion-claim filter in §design-principles

- [x] Implement the behavior described in `scenarios/completion-claims-carry-no-caveats.md`

- **Done when**: `framework/constitution.md` §design-principles carries the filter stating that incomplete work must never be indistinguishable from complete work, naming the three dispositions and the measurement rule; §implement-phase's SHOULD bullet references it rather than restating it; the section preamble no longer hardcodes a bullet count. Family 6 (SSOT invariants) stays green, confirming the rule is stated once.

## 10. Findings route by scope in §brownfield-inbox

- [x] Implement the behavior described in `scenarios/findings-route-by-scope.md`

- **Done when**: `framework/constitution.md` §brownfield-inbox's Automatic issue capture carries the scope-routing bullet naming all three tiers, states that `tasks.md` is not a second capture queue and that a chore with no feature home stays an inbox item, and its closing sentence names both destinations. The `AGENTS.md` entry is a pointer carrying no normative text of its own (§Promotion mechanism, AC3). `npx markdownlint-cli2` and `scripts/audit/run-all.sh` clean, Family 1 and Family 6 included, on a committed tree.

## 11. Sweep the prose claims the scope-routing rule falsifies

- [x] `framework/commands/implement.md` step 5 (*Capture incidental issues*) states the three tiers — today it sends every issue outside the current task's scope to `specs/inbox.md`, which is wrong for the whole middle tier
- [x] `framework/templates/project/inbox.md`'s Rules block names the scope test, so an adopter reading only the header does not route a spec-scoped finding to the inbox
- [x] This repo's `specs/inbox.md` header is kept in sync with that template
- [x] Regenerate `.claude/commands/ductus/` and run the full CI surface — `framework/commands/*.md` is runtime-adjacent, so `cargo test --release --locked` is part of the gate, not the markdown checks alone

- **Done when**: An agent implementing a task routes a finding by scope wherever it reads the procedure — `implement.md`, the shipped inbox template, and this repo's inbox agree with §brownfield-inbox rather than predating it. `npx markdownlint-cli2`, `scripts/audit/run-all.sh`, and `cargo test --release --locked` clean on a committed tree; if `implement-basic.jsonl` shifts, it is re-blessed filtered to that one golden with the diff confirmed to be the two sha fields only.

## 12. §grounding — a partial read is not a read

- [x] Implement the behavior described in `scenarios/a-partial-read-is-not-a-read.md`
- [x] Add the subsection to `framework/constitution.md` §grounding, after **Sources, in order of authority**, stating that a tool which elides content has not delivered the source
- [x] State the two dispositions — read the remainder, or name what was not examined where a later reader meets the claim — and that prose beside a completion claim is not a third
- [x] Tie it to `QUAL-CLAIM-001` explicitly, so a reviewer can cite the binding rather than argue the analogy
- [x] State the limit honestly: nothing intercepts a tool choice, so this is a governed requirement cited by `/{project}:review` and `/{project}:analyze`, not a gate
- [x] Add a second `CLAUDE.md` §Non-negotiables entry pointing at the new §grounding rule — one line, no restatement, since the constitution is its home and a second copy would drift (the trunk-based entry stays project-specific and does **not** move into the constitution, which ships to adopters who may not be trunk-based)
- [x] Verify no other constitution section already states a competing rule about partial reads, and that the SSOT invariants family stays clean

- **Done when**: `framework/constitution.md` §grounding carries the subsection, it ships to adopters unchanged, `CLAUDE.md` points at it rather than restating it, and `/{project}:audit` reports zero findings.

### A retired feature leaves no spec

Implements `scenarios/a-retired-feature-leaves-no-spec.md`. The constitution stated the rule for obsolete scenarios and never for obsolete specs, so 053's correct deletion happened by one spec's acceptance criterion rather than by a standing rule.

- [x] Implement the behavior described in `scenarios/a-retired-feature-leaves-no-spec.md`
- [x] State the rule in §spec-lifecycle as a fourth operational rule, cross-referencing the scenario-level form §scenarios already carries
- [x] Name the three cases explicitly — consolidate when content survives elsewhere, delete outright when nothing does, ordinary body edit when only part of a feature is retired
- [x] Say why pointer-first matters: a deleted spec with live inbound references trades one durability problem for a worse one
- [x] Add the contributor-side mirror to `AGENTS.md` as a pointer, not a second copy, recording that the summary that prompted it was wrong while the principle was right
- [x] State plainly that nothing enforces this yet, and what a check would have to reckon with — a rule whose enforcement is "someone remembers" is a diligence dependency and must be named as one

- **Done when**: §spec-lifecycle carries the rule with its three cases, `AGENTS.md` mirrors it by reference, and the scenario records both the 053 precedent and the unenforced status honestly.

## 13. Name the shared tier in §Classification

- [x] Implement the behavior described in `scenarios/governance-is-multi-source.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 14. Amend §cross-spec-impact to separate the enforceable half from the judgment half

- [x] Implement the behavior described in `scenarios/a-declared-cross-spec-impact-gates-done.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 15. State the retired-filename rule in §drift-prevention and clear the `.govern.toml` residue

- [x] Implement the behavior described in `scenarios/a-retired-filename-leaves-a-decision-record.md`
- [x] Resolve the scenario's open question first — whether the corpus rewrite is a §spec-lifecycle case (a) mechanical edit or takes the back-edge on each spec it touches; the answer sets the cost and must not be decided during the pass
- [x] Amend §drift-prevention to distinguish a retired identifier from a retired filename a compatibility path still reads, naming the exempt sites as a category (resolution ladder / fallback tier, migration procedure, the prose and shell spelling either) rather than as this repository's paths
- [x] State the corpus end state: the current name wherever a spec states current behavior, the retired name only in the few references recording the decision to change it
- [x] State that a ticked acceptance criterion is swept like any other reference, because only the filename moved and the requirement it states is unchanged; reserve annotation for a criterion whose *behavior* was superseded, and remove the annotations 017 and 027 carried from before this was settled
- [x] Repoint the `AGENTS.md` §Gotchas entry on `.govern.toml` at the new constitution text so it states no normative content of its own, per AC3's promoted-entry pattern
- [x] Audit every retired-filename occurrence under `specs/` per-hit: substitute the current name wherever an artifact states current behaviour — acceptance criteria included — and leave the compatibility-path, migration, historical-root, dated-observation and decision-record occurrences as written
- [x] Leave every occurrence outside `specs/` untouched — 144 occurrences at the 2026-09-13 baseline (the earlier "131" counted matching *lines*, not occurrences): the `runtime/src/` resolution ladder, the `framework/migrations/` procedures that must name the file each migration migrates, and the `framework/commands|bootstrap`, `scripts/` and generated `.claude/` copies spelling the same ladder. `AGENTS.md` is the one deliberate exception, rewritten by the subtask above

- **Done when**: §drift-prevention carries the retired-filename rule with its exempt-site categories and the sweep-don't-annotate disposition; `AGENTS.md` mirrors it by reference; every retired-filename occurrence under `specs/` has been judged individually, with the survivors limited to the compatibility-path, migration, historical-root, dated-observation and decision-record cases; and every occurrence outside `specs/` is unchanged.

## 16. Correct 016's dead README section pointer

- [x] `specs/016-cross-cutting-rules/spec.md:73` cites *the README's "Pinning files with …" section*; the README was restructured and has no such section under any name, so the pointer was already dead before the filename pass renamed the file it mentions
- [x] Repoint it at what the README actually documents — `[pinned]` under §Configuration — or drop the parenthetical and name the key directly
- [x] This is a **factual correction**, not a substitution: it rewords the line, so §spec-lifecycle's back-edge applies and 016 reopens `done → in-progress` for it. Do not bundle it with the filename sweep, which is exempt precisely because it rewords nothing
- [x] `check-corpus-links` cannot catch this class — a prose reference to a section title is not a link — so note whether the corpus carries more of them before closing

- **Done when**: 016's spec body no longer points at a README section that does not exist, the correction is taken through the back-edge rather than folded into the exempt sweep, and the run says whether other prose section-title pointers were checked.

## 17. Apply the retired-filename rule to the three sibling names

- [x] Task 15 was scoped to `.govern.toml` alone, but the rule it states governs every retired filename a compatibility path still reads. Measured 2026-09-13, three siblings carry comparable residue: `.govern.session.toml` (167 occurrences, 92 under `specs/`), `.govern/` (171, 40 under `specs/`), `.governance.toml` (36, 27 under `specs/`)
- [x] Judge them by the same test — the resolution ladder's legacy tiers, the migration procedures that must name both sides, and the decision records stay; present-tense prose naming them as current is residue and takes the current name (`.ductus/session.toml`, `.ductus/`, and — for `.governance.toml` — whichever tier the ladder actually resolves)
- [x] `.governance.toml` needs care: `framework/migrations/governance-config-rename.md` renames it **to** `.govern.toml`, so both sides of that procedure are retired today and both must survive
- [x] Keep the prose substitutions free of any reworded line so the sweep exemption holds, exactly as task 15's mechanical half did — the annotations are what reopen a spec

- **Done when**: Every `.govern.session.toml`, `.govern/` and `.governance.toml` occurrence under `specs/` has been judged individually against §drift-prevention's retired-filename rule, the compatibility-path and decision-record occurrences are named as deliberate survivors with a count, and nothing outside `specs/` moved.

## 18. A canonical source is pointed at, not copied

- [x] Implement the behavior described in `scenarios/a-canonical-source-is-pointed-at-not-copied.md`
- [x] State under §drift-prevention's *Canonical sources* that referencing means a pointer and that a reproduction is never one — the existing MUST says "reference rather than restate", which a verbatim copy satisfies on its face
- [x] Give both reasons, since the rule is read as a style preference without them: nothing can detect an embedded copy (a fenced snapshot is unreachable by every link check, anchor resolver and audit family this project has), and every reader pays for it in every session while a pointer costs one line
- [x] Name the disposition: git history already holds what a copy was preserving, and replacing a copy with a pointer changes what the document asserts, so it takes the back-edge rather than the mechanical-sweep exemption
- [x] Land it in the constitution rather than AGENTS.md — an adopter's corpus accumulates embedded copies for the same reasons, and a contributor-side entry does not propagate; reduce the AGENTS.md entry to a mirror that references the section rather than restating it

- **Done when**: `framework/constitution.md` §drift-prevention states the pointer-not-copy rule with both the detectability and the per-session cost reasons, the scenario records the 020 instance that exposed it and names what nothing enforces, and `AGENTS.md`'s entry is a contributor-side mirror rather than a second copy of the rule. 020's own §Embedded artifacts is **not** remediated here — that is its own back-edge, tracked in `specs/inbox.md`.

## 19. Route knowledge by population rather than by kind

- [x] Implement the behavior described in `scenarios/knowledge-routes-by-population-not-by-kind.md`
- [x] State the population question in `framework/constitution.md` §drift-prevention — two ordered questions, the three tiers on the project-learning bullet, and why routing by kind conceals itself
- [x] Rewrite the `AGENTS.md` header (line 5) to ask who the learning is true for before naming a section, and correct its section list to this repository's actual sections
- [x] Rewrite `CLAUDE.md` §Auto-Memory Routing as two ordered questions rather than one repository-scoped question
- [x] Apply the same two edits to the shipped templates `framework/templates/project/agents.md` and `framework/templates/project/claude-md.md`, worded for an adopter
- [x] Record in `specs/inbox.md` that the flow-fix half is discharged and the bulk promotion pass is not

- **Done when**: §drift-prevention's *Shared knowledge stays in git* states the two questions in order and routes a project learning by the three §Classification tiers; all four agent-facing surfaces point at it rather than restating it; `npx markdownlint-cli2` and `/ductus:audit` are clean, Family 25 included, with the `AGENTS.md` header still a single balanced line.

## 20. Re-key §Classification so the coverage check is re-derivable

- [x] Derive the survey-era positional-ID → lead-phrase mapping from `AGENTS.md` at the classification pass's own commit, and verify it reproduces the recorded 54
- [x] Re-key all 54 `plan.md` §Classification rows to the `AGENTS.md` lead phrase, keeping the positional ID in parentheses so the section's prose references still resolve
- [x] State in the section preamble why the positional ID cannot stay the key, and state the lead phrase's own limit — a reworded entry stops matching
- [x] Record the measured coverage (*unclassified = rule-bearing − (table-keyed ∪ constitution-citing)*) with the in-scope denominator it needs, as a notice and never a gate

- **Done when**: `plan.md` §Classification is keyed by lead phrase, the coverage figure is re-derivable from `AGENTS.md` on disk rather than from git archaeology, and the citing-side denominator is stated so a bare `grep -c` over the file is not mistaken for it. The standing-notice *rendering* is not done here — it is runtime work on the `inbox-standing` precedent and carries a release.

## 21. Classify the second-round entries

- [x] Judge each of the 44 entries the coverage check reports as unclassified against the reword test, one verdict and one reason each
- [x] Record them in `plan.md` §Classification as a second round, keyed by lead phrase like the first
- [x] Note that promotion is not performed by this task — the classification is the worklist, as it was in the first round

- **Done when**: every entry the coverage check reported is classified with its reason, the unclassified count is discharged to zero, and `plan.md` records that this is a second deliberate round rather than the standing requirement AC1 rejects. The promotions themselves are the next task and are deliberately not started here.

## 22. A recorded measurement states its method and its units

- [x] Implement the behavior described in `scenarios/a-measurement-states-its-method-and-units.md`
- [x] State the rule in `framework/constitution.md` §grounding's **Rules** list, immediately after *Cite what you consulted* — it extends that rule, so a bullet rather than a section, which is §Resolved Questions' default and keeps AC10's anchor set unchanged
- [x] Name the failure it prevents: without method and units a later reader cannot separate a **decayed** figure from one taken by a **different method**, and the reflex to trust the newer figure is wrong precisely when the older measurement was better informed
- [x] Give the disposition on a disagreement — re-derive at the recorded figure's own commit first, then diff the field across its own history, because a value can have been correct and then overwritten with a worse one
- [x] Reduce only the `AGENTS.md` entry that states the rule *generally* to a pointer; leave the six situational corollaries in place, since each sits inside a gotcha whose subject is something else and carries no restatement of the canonical text
- [x] Do **not** discharge this by adding another `AGENTS.md` entry, and do **not** fold it into the second promotion round's count in `plan.md` §Classification — it is a new rule, not a promotion

- **Done when**: `framework/constitution.md` §grounding states the rule as a bullet under **Rules**, the `AGENTS.md` entry that stated it generally points at that bullet rather than restating it, and the six situational corollaries are left where they are.

## 23. Promote the second round

- [x] Write each promoted entry's canonical text as a bullet in the existing section whose subject it shares — never a new section, so AC10's anchor set is unchanged and §Resolved Questions' bullets-not-sections rule holds
- [x] Word each promotion for an adopter: cite the constitution, the pipeline commands, the artifacts, or runtime primitives, and never a path, script, workflow, or release loop that exists only here
- [x] Rewrite each promoted entry's `AGENTS.md` line to name its constitution section and state nothing the constitution does not, keeping the bolded lead phrase byte-identical so `plan.md`'s table keys still resolve
- [x] Leave every entry in the project-only and already-promoted tables byte-identical (AC4)
- [x] Route the one entry that is universal but governs **code** to `framework/rules/quality-cross.md` with a permanent ID, through the owning spec's back-edge per §rules — not to the constitution
- [x] Verify: the anchor set is unchanged, `npx markdownlint-cli2` passes, each promoted rule's distinctive phrasing finds one normative statement and one pointer, and `plan.md`'s second-round table keys still match 1:1

- **Done when**: every entry in `plan.md` §Classification's `#### Second round — promote` table has its canonical normative text as a bullet in `framework/constitution.md` under an existing section, each such entry's `AGENTS.md` line is a pointer stating nothing normative of its own, the anchor set before and after is identical, no project-only entry is altered, and the one entry routed to a rule file is registered there rather than here.

## 24. Report outcomes, not edits

- [x] Implement the behavior described in `scenarios/report-outcomes-not-edits.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 25. A status commit holds only the transition

- [x] Implement the behavior described in `scenarios/a-status-commit-holds-only-the-transition.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 26. Re-promote W6, withdrawn by 054, and point the in-substance entry at §numbering

- [x] Add W6's residue to `framework/constitution.md` §implement-phase as a bullet: `/{project}:review` regenerates the report body of `review.md` on every run and only its waiver and decision lists carry forward, so anything that must survive belongs on the artifact it describes. Word it for the lists 058 made persistent, which the `AGENTS.md` text predates
- [x] Rewrite the `AGENTS.md` entry *Never record anything durable in `review.md`* as a pointer to §implement-phase, keeping its bolded lead phrase byte-identical so `plan.md`'s table key still resolves
- [x] Add a §numbering pointer to the `AGENTS.md` entry *A feature directory is not necessarily `NNN-slug`*, naming the constitution as home of the universal half and the entry as this repository's Rust half
- [x] Record on `plan.md` §Classification's W6 row that 054 withdrew the first promotion and this task re-promoted the residue
- [x] Verify: every row in the promoted, already-promoted, and in-substance tables resolves to an `AGENTS.md` entry citing the constitution or its rule ID; markdownlint, the `scripts/lint-*.sh` set, `scripts/audit/run-all.sh` and `cargo test --release --locked` are clean

- **Done when**: W6 has canonical normative text in `framework/constitution.md` and its `AGENTS.md` line is a pointer, so AC2 and AC3 hold for every promoted entry; the in-substance feature-directory entry points at §numbering; and the full local surface is green.

## 27. State the rule-file destination in the spec's §Classification

- [x] Add a paragraph to `spec.md` §Classification naming the destination the tier table cannot express: a universal entry that governs code rather than the pipeline routes to a rule file with a permanent ID through the owning spec's back-edge, per §rules, as `QUAL-DELEG-001` did through `036-quality-cross-rules`
- [x] Cite 036 as a backticked slug rather than a link, so no `dependencies:` edge is added

- **Done when**: The spec's §Classification states the fourth destination alongside the three population tiers, so AC22 holds as written.

## 28. Third classification round — the 9 entries the promotion-coverage notice reports unclassified

- [x] `plan.md` §Classification: add *Third round — classified 2026-09-28* with the 9 rows keyed by lead phrase — 7 project-only, `append-task`'s no-write shapes promoted, and *a test must fail when the behavior it names is removed* routed to a rule file — each with its reason (operator decision, 2026-09-28)
- [x] `framework/constitution.md` §scenarios: promote the `append-task` entry, reworded by the reword test, beside the scenario-creation paragraph; the anchor set stays byte-identical
- [x] `AGENTS.md`: the `append-task` entry becomes a pointer to §scenarios and the test entry a pointer to `QUAL-TEST-001`, each keeping its story and its lead phrase byte-identical so the table keys still resolve
- [x] Verify: the promotion-coverage family reports 0 unclassified and 0 unmatched keys; the new rule's distinctive phrasing finds one normative statement and pointers only (AC3); lint, generators, audit and `cargo test`

- **Done when**: the promotion-coverage notice reports 0 unclassified, the promoted text has one canonical home, and the local gate passes

## 29. A project command is run, not reproduced

- [x] Implement the behavior described in `scenarios/a-project-command-is-run-not-reproduced.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 30. A review may precede its commit

- [x] Implement the behavior described in `scenarios/a-review-may-precede-its-commit.md`

- **Done when**: §implement-phase no longer carries the commit-first review bullet and no remaining constitution text requires a review to follow its commit; the AGENTS.md mirror drops the commit-first instruction and points its audit-after-commit corollary at §grounding's Rules list; plan.md's W7 classification row notes the retirement; markdownlint and the framework self-audit pass.

## 31. Fourth classification round — the 11 entries the promotion-coverage notice reports unclassified

- [x] `plan.md` §Classification: add *Fourth round — classified 2026-10-15* with the 11 rows keyed by lead phrase — 9 project-only, and promoted *a scenario recorded after its fix is reviewed with `--since`* and *read what a push carries before pushing* — each with its reason (operator decision, 2026-10-15)
- [x] `framework/constitution.md`: promote the `--since` entry into §implement-phase's review-window bullet and the push entry into §concurrent-features' *What stays shared* paragraph, each reworded by the reword test; the anchor set stays byte-identical
- [x] `AGENTS.md`: both promoted entries become pointers, each keeping its story and its lead phrase byte-identical so the table keys still resolve
- [x] Verify: the promotion-coverage family reports 0 unclassified and 0 unmatched keys; each promoted rule's distinctive phrasing finds one normative statement and pointers only (AC3); lint, generators, audit and `cargo test`

- **Done when**: the promotion-coverage notice reports 0 unclassified, the promoted text has one canonical home, and the local gate passes

## 32. Reports end with the remaining work

- [x] Implement the behavior described in `scenarios/reports-end-with-the-remaining-work.md`
- [x] `framework/constitution.md` §pipeline-boundaries: add the bullet after *Report outcomes, not edits* — the closing line is *No further work identified* or a must/should/advisory list, with the reason and the dispositions boundary
- [x] Verify: lint, the shipped-anchors and promotion-coverage families, the full audit and `cargo test`

- **Done when**: the bullet ships in the constitution, the scenario records why, and the local gate passes

## 33. The constitution states rules, not stories

- [x] Implement the behavior described in `scenarios/the-constitution-states-rules-not-stories.md`
- [x] `framework/constitution.md`: tighten every prose section to rule, reason and boundary, keeping markers, headings, tables, the diagram and bold leads byte-identical, and the passages 021 quotes verbatim
- [x] Verify: structure comparison against the base, lint, anchors and links, the audit and `cargo test`

- **Done when**: the constitution is smaller with every rule kept, the scenario records the shape, and the local gate passes
