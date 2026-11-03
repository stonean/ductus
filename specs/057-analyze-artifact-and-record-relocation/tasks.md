# 057 — Analyze artifact and record relocation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Task 1 is first by constitutional rule, not by preference: §runtime-boundary
principle 4 requires a schema change to ship through the constitution with the
runtime following. Tasks 2-8 are the runtime; 9-12 the migration and the gates
around it; 13-14 the sweeps; 15-16 verification and the declared obligations.

## 1. Declare both record schemas in the constitution

- [x] Add a record-schema subsection to `framework/constitution.md`'s **Frontmatter Schema** naming the owning artifact for each record and the fields it carries
- [x] Leave the spec-file table at its existing five fields — the records were never in it
- [x] State that `spec.md` carries neither block, so a residual one is a defect rather than an open-schema extra

- **Done when**: the constitution declares both records at their owning artifacts, and no part of the declaration is sourced from a runtime type.

## 2. Add the merged record type and the record loaders

- [x] Extend `ReviewBlock` with the report-only scalars it must now carry: `spec`, `diff-base`, `captured-issues`, `skipped-passes`
- [x] Leave `waivers` untyped and open-schema — three raw parsers own it with `#[serde(flatten)]` extras, and `schema/primitives.rs` carries no `serde_norway::Value` for the `JsonSchema` derive to choke on
- [x] Add loaders that read each record from its owning artifact, returning **absent**, **unreadable**, and **present** as three distinct states
- [x] Leave `Frontmatter`'s `review` and `analyze` fields in place — this task is additive and nothing reads the loaders yet

- **Done when**: `cargo test` passes and the loaders return each of the three states against a fixture that exercises all three.

## 3. `write-review` writes the whole record into `review.md`

- [x] Render the merged record into `review.md` frontmatter: rename `reviewed-at` to `last-run`, and add `blocking`, `reviewed-digest`, `reviewed-unreadable`, and `waivers`
- [x] Leave the `spec.md` block write in place for now — retired in task 9, so the tree stays green in between

- **Done when**: `cargo test` passes and a review run produces a `review.md` whose frontmatter deserializes into the merged record with no field dropped.

## 4. `write-analysis` writes `analysis.md`

- [x] Render `analysis.md`: merged record frontmatter plus the fixed skeleton — Summary, hard failures, blocking findings, advisory findings, unexamined targets
- [x] Write it on every run, including a clean run and an empty scope
- [x] Emit no `- [ ]` item in any section
- [x] Leave the `spec.md` block write in place for now

- **Done when**: `cargo test` passes, and a clean run and an empty-scope run each produce an `analysis.md` containing zero checkbox items.

## 5. Move the waiver parsers to `review.md`

- [x] Repoint the raw waiver parsers in `process_waivers.rs`, `write_review.rs`, and `invalidate_review.rs` at `review.md`
- [x] Preserve the `#[serde(flatten)]` extras, so an organization's custom waiver fields survive the move

- **Done when**: `cargo test` passes and a waiver carrying org-custom fields round-trips through `review.md` byte-identical.

## 6. Migrate the gate and the artifact checks to the loaders

- [x] Replace the `frontmatter.review` / `frontmatter.analyze` reads in `check_review_gate.rs` (lines 134, 183) and `check_artifacts.rs` (583, 626) with the task-2 loaders
- [x] Report an absent artifact as never-run and an unparseable one as undeterminable — never collapse the second into the first
- [x] Leave every downstream branch untouched

- **Done when**: a differential run of the gate over the pre- and post-relocation corpus produces identical verdicts for every spec, and the absent and unreadable cases are distinguishable in the output.

## 7. Move the digest exclusions

- [x] Excise `analysis.md`'s own record from the analyze digest; digest `spec.md` whole and delete the `analyze:`-block surgery path
- [x] Leave the review digest subject set unchanged, and annotate in the code that the `spec.md` exclusion's stated rationale lapses once `write-review` stops touching it

- **Done when**: an analyze run immediately following itself reports current rather than stale, and the review digest's subject set is unchanged from before the relocation.

## 8. Validate each record at its owner

- [x] Point `validate_frontmatter.rs` at the owning artifact for each record
- [x] Report a `review:` or `analyze:` block still present in `spec.md` at the **Blocking** tier the constitution now declares for it
- [x] Report a `review.md` or `analysis.md` that exists but carries no parseable record as a hard fail

- **Done when**: a spec carrying a residual block is reported as Blocking, and a correctly migrated spec validates clean.

## 9. Retire the `spec.md` writes and drop the `Frontmatter` fields

- [x] Remove the `spec.md` block writes from `write_review.rs`, `write_analysis.rs`, and `invalidate_review.rs`
- [x] Remove `review` and `analyze` from `Frontmatter`, and fix the fixture at `primitives.rs:4421`
- [x] Point `check_review_agreement.rs` at its own empty-subject guidance path rather than letting it compare a record to itself — it is deleted in task 14, and a self-comparison in the interim is the false green its header warns about

- **Done when**: `cargo test` passes and no code path reads or writes either record in `spec.md` frontmatter.

## 10. Follow the records through the remaining readers

- [x] Update `read_spec.rs` and `dashboard.rs`
- [x] Confirm `/{project}:status` renders the same review and analyze state as before

- **Done when**: the status view and dashboard output match their pre-relocation values for the migrated corpus.

## 11. Add the `relocate-audit-records` primitive

- [x] Read a spec's frontmatter, write each record to its owning artifact, remove both blocks from `spec.md`
- [x] Invent no record for a spec that carries none
- [x] Make a spec with no blocks left a no-op
- [x] Leave body prose untouched, and halt if a run finds itself changing it

- **Done when**: running the primitive twice over a spec leaves the second run writing nothing, and a partially migrated corpus converges without duplicating or dropping a record.

## 12. Register the migration

- [x] Add one `[[migrations]]` entry to `framework/migrations.toml` with `introduced_in` set to the release that first carries the primitive
- [x] Write `framework/migrations/audit-record-relocate.md`, invoking the primitive per spec and skipping pinned files with one line naming each
- [x] State the §spec-lifecycle case (c) rule: the sweep is a mechanical edit, so a `done` spec stays `done`
- [x] Detect an adopter CI file still carrying the block-presence check and tell them to re-copy the template

- **Done when**: 027's audit family passes against the entry, and a `done` spec swept by the procedure is still `done`.

<!-- Tasks 1-12 are complete. Before task 13, read plan.md's
     **Implementation notes** — it records the write-boundary grant these tasks
     need, the release-binary/session-restart loop that task 15's sweep depends
     on, and the parity tests that guard primitive registration. -->

## 13. Replace the CI template's vacuous predicate

- [x] Read the record from `review.md` and `analysis.md`
- [x] Bound the exempt set with a committed high-water mark, in the form `scripts/audit/analyze-record-backlog.sh` already uses
- [x] Remove the block-presence skip

- **Done when**: no predicate in the step is satisfiable by every spec in a migrated corpus, and the step fails when the exempt set grows.

## 14. Retire the reconciliation check, reconcile the baseline

- [x] Delete `check_review_agreement.rs`, its schema types, and `scripts/audit/review-block-agreement.sh`
- [x] Point `scripts/audit/analyze-record-backlog.sh` at the relocated record and reconcile `analyze-record-baseline.txt`
- [x] Confirm the never-reviewed and never-analyzed cases remain distinguishable through task 6

- **Done when**: `/{project}:audit` passes with Family 31 removed and Family 37 counting the same population it counted before.

## 15. Ship the template and sweep this repo's corpus

- [x] Remove both blocks from `framework/templates/spec/spec.md`
- [x] Run the primitive across all spec directories under the spec root
- [x] Run the full markdown lint over the result

- **Done when**: a newly created spec opens with `status` and `dependencies` alone, no `spec.md` in the corpus carries either block, and the lint is clean.

## 16. Sweep the documentation

- [x] Update `framework/commands/review.md`, `analyze.md`, `implement.md`, `audit.md`, and `consolidate.md` — runtime steps **and** markdown-only references
- [x] Update `docs/analyze.md` field-by-field, including the `analysis.md` skeleton
- [x] Update `docs/slash-commands.md`, `docs/shared-constitution.md`, `README.md` (the `/analyze` entry states the old location outright), and `AGENTS.md`
- [x] Regenerate the per-agent command mirrors

- **Done when**: the command-parity audit passes and every mirror matches its source.

## 17. Verify the sweep mechanically

- [x] Search the repository for the pre-relocation shape — a `review:` or `analyze:` block addressed as `spec.md` frontmatter
- [x] Confirm the only surviving hits are this spec's own Motivation and Resolved Questions. **Run 2026-09-16 at `971c69a3`+, and the confirmation is a classification with a recorded allow-list rather than a zero-hit grep** — the plan says so, and the sweep bears it out. The location classifier returns **118** lines across the tracked corpus, every one classified: **17** in this spec's own artifacts and **8** in `runtime/CHANGELOG.md` (a shipped release note records what was true then); **2** in the migration entry and procedure, whose subject *is* the move; **3** in the constitution's declaration that a residual block is a violation plus `validate_frontmatter.rs` implementing it; and **88** past-tense accounts, spent `tasks.md` entries, and ticked criteria annotated in place in the 009-AC18 / 046-AC22 style. Three live claims were found in this pass and fixed before it closed: `framework/commands/review.md`'s two-spellings paragraph and its `review.blocking` accessor, `write_review.rs`'s module doc still saying it "updates the spec `review:` block", and `WriteReviewResult::analyze_freshness`'s doc saying the primitive had just written that block
- [x] **Two classes of hit no location classifier can reach, found by reading instead.** A sentence naming the record but no file — *"ordered after every `review:` check"*, *"an `analyze:` frontmatter block"* — carries no `spec.md`, `spec frontmatter` or `the spec` within the window, so it is invisible by construction; that class hid 047's `analyze-run-durability` Behavior section and most of 022's 41 corrections. And a reference to the **retired family** (`Family 31`, `check-review-agreement`) is not a location claim at all, so it needed its own sweep. Running all three patterns and reading the files is what closed them. An enumeration is a claim ([§drift-prevention](../../framework/constitution.md#drift-prevention)), and a pattern that cannot fire on a whole class is `QUAL-CLAIM-001` wearing a regex
- [x] **The confirmation must classify every hit individually — a per-file verdict is how two live claims survived.** Re-running the classifier on 2026-09-16 reported four hits in `framework/constitution.md`; two (537, 586) are the new declaration and the Blocking severity tier, which is the allow-listed family, and on that basis the file was waved through whole. The other two were the pre-relocation shape in the document that **ships to every adopter and is the canonical source the other copies point at**: §spec-lifecycle line 182 said `/review` "updates the `review:` frontmatter block", and §implement-phase line 282 said "the spec's frontmatter `review:` block records the result". Task 16's sweep had missed both. Corrected 2026-09-16, with `framework/commands/audit.md` and `scripts/audit/README.md`, which both described Family 19's predicate as reading `review.reviewed-against` after task 20 re-pointed it at `review.md`. This is the counting lesson AGENTS.md already carries, one tier up: read each finding's line and confirm what it lands on, never accept a file into an allow-list because some of its hits belong there
- [x] **Record the classifier's boundary, since it decides what the confirmation is even asked about.** A bare dotted accessor with no `spec.md` / "spec frontmatter" / "the spec" within ~140 characters is **not** the pre-relocation shape by the plan's own definition — it names the record's field. That leaves `framework/commands/review.md`'s four `review.blocking` mentions, `implement.md`'s `analyze.last-run`, `check_artifacts.rs`'s emitted drift messages and `process_waivers.rs`'s `review.waivers[N]` malformed-waiver message correct as written. The boundary is stated here rather than left implicit, because the same grep reports them all and a later reader deciding case by case will not reach the same line twice

- **Done when**: the search returns no hit outside this spec, so the sweep is verified rather than asserted.

## 18. Discharge the declared cross-spec obligations

- [x] Record in `047-analyze-findings-durability` that its resolved question is superseded on the record-location half, linking back here
- [x] Record in `020-code-review` the change to its CI-gate mechanism, linking back here
- [x] Correct 020's `## Frontmatter schema` section in the same reopen — it presents the `review:` block as *spec* frontmatter in a fenced YAML example, which the relocation makes false; the reopen is already being spent, and task 17's sweep reports this hit

- **Done when**: both specs link back to 057, clearing the `cross-spec-impact` entries that would otherwise block `done`.

## 19. Discharge the 026 obligation task 14 created

- [x] Reopen `026-framework-self-audit` — task 14 deleted Family 31, which its AC22 specifies and its `scenarios/family-31-review-block-agreement.md` is the durable contract for
- [x] Delete that scenario: it is obsolete, and §scenarios says an obsolete scenario is deleted rather than marked with a status
- [x] Annotate AC22 as superseded, naming 057 and the reason — the family's subject was the intersection of two records of one fact, and there is one record now
- [x] Re-run `/{project}:review` and `/{project}:analyze` over 026: deleting a scenario changes a durable contract, so the recorded `reviewed-digest` goes stale and `check-review-gate` will answer `review-stale`
- [x] Return 026 to `done`, and confirm it links back to 057 so the `cross-spec-impact:` entry is discharged

- **Done when**: 026 carries no scenario or ticked criterion asserting Family 31 exists, its review and analysis are current, it is back at `done`, and the `026-framework-self-audit` entry in this spec's `cross-spec-impact:` is discharged by the reciprocal back-link.

## 20. Re-point the two remaining corpus readers at the relocated record

- [x] `scripts/audit/review-freshness.sh` (Family 19) reads `review.reviewed-against` from the `spec.md` block and grandfathers a spec by the block's absence — after the sweep that is every spec, so the family goes vacuous exactly as the CI template did
- [x] `runtime/tests/mechanical_sweep_parity.rs` builds its subject from the same block; with the block gone its subject is empty and its vacuity guard fires, which is the guard working
- [x] Rewrite both to read `review.md`, pinning the new contract rather than relaxing the guard
- [x] Surveyed for others: `marker-list-parity.sh` and `template-alignment.sh` match on `analyze.md` the command file, not the record, and the pre-commit hooks read neither — so these two are the whole set

- **Done when**: Family 19 resolves a `reviewed-against` for every done spec and reports its grandfathered count from the absence of `review.md`, and `mechanical_sweep_parity` builds a non-empty subject and passes.

## 21. Reconcile the version pin and cut the release

- [x] Audit Family 20 has been red on `main` since `783602c5`: tasks 11-12 bumped `runtime/Cargo.toml` and `runtime/CHANGELOG.md` to 0.50.0 and left the repo-root `version` file at 0.49.8. It is red in the **safe** direction — `/{project}` reads the root file, 0.49.8 is published, so no adopter is halted — but the audit is a hard release gate and will block the tag
- [x] Bump the root `version` file to 0.50.0 **and** push `ductus-v0.50.0` in the same sitting: a bumped pin with no tag sends every adopter after assets that do not exist and aborts their run. Done 2026-09-16 — commit `4cb56c05` and the tag at the same sha, pushed minutes apart
- [x] Do this only after 057, and the specs it reopens (026, 020, 047), are back at `done` — a release is cut from completed work. All four, plus 022, were `done` before the bump commit; `check-review-gate` reported every `cross-spec-impact:` entry discharged first
- [x] Run `scripts/audit/run-all.sh` after committing and before tagging, and confirm Family 20 is green; then read every workflow run for that sha, not only the one whose name matches. The audit exited **0** with Family 20 green. All four workflows for `4cb56c05` were read, not just the matching one: `framework-checks`, `generators` and `runtime-release` success, `runtime` confirmed after. `runtime-release`'s twenty jobs were read individually rather than off the top-level conclusion — `Publish to crates.io`, `Publish release assets (complete set only)` and all five `Verify the published release is acquirable` legs green — and the release carries all twelve assets (five targets × tarball + sha256, plus the SBOM and its digest)

- **Done when**: All three version sites read 0.50.0, `scripts/audit/run-all.sh` is green including Family 20, and `ductus-v0.50.0` is pushed and its release workflow has published.

## 22. Sweep the runtime's own user-facing surfaces and doc comments

- [x] Task 17's mechanical search found the relocation's stale claims outside the artifacts task 16 named — which is the case AGENTS.md warns about: a superseded rationale hides best in the diff that falsified it
- [x] MCP tool descriptions in `runtime/src/mcp/server.rs` for `write-review`, `write-analysis` and `invalidate-review`, and the matching clap help in `runtime/src/main.rs` — these render to an operator, so a wrong location here is the most visible of all
- [x] Doc comments in `write_review.rs`, `write_analysis.rs`, `check_review_gate.rs`, `check_artifacts.rs`, `process_waivers.rs` and `schema/primitives.rs` that still address the record as a `spec.md` block
- [x] The three `.github/workflows/*.yml` comments explaining why `fetch-depth: 0` is needed, which name `review.reviewed-against` in the spec
- [x] `runtime/tests/parity/review/expected.txt`, whose strict-fields contract names a `review-block` in the spec
- [x] Classify every hit by tense: correct a claim about where the record lives now, keep a past-tense account of the two-homes condition
- [x] **Residue found 2026-09-16 re-running the classifier for task 17's confirmation — this task's own Done-when was not met; all nine are corrected as of 2026-09-16.** The first pass swept the three primitives it named and missed nine live claims in surfaces it did not enumerate. Two render to an operator: `mcp/server.rs:794` (`check-review-gate`'s tool description, "then the spec frontmatter review: block") and `main.rs:138` (the matching clap help). Five are doc comments on live behavior: `check_review_gate.rs:260` ("Gate checks 8, 9 and 10 — the spec's `analyze:` block"), `schema/primitives.rs:564` and `:739` (result fields described as the spec whose block "was updated"/"was written"), and `:3167` / `:3194` (the two gate-reason variants). Two are stale because task 9 left `write-review` with **one** write, not two: `write_review.rs:138` and the `malformed_spec_frontmatter_halts_before_any_write` comment at `:1888` both reason about a halt landing "between the two writes". Two more are stale mechanism inside a correct conclusion: `write_review.rs:1199` and `check_review_gate.rs:1623` say recording a review rewrites the spec's `review:` block — it rewrites `review.md`, which is still an analyze subject, so the conclusion stands and the mechanism does not. Correct all nine; leave the allow-listed past-tense accounts (`analyze_subjects.rs` 11/107/405, `check_artifacts.rs:679`, `check_review_gate.rs:944`, `write_review.rs:487`/`:1158`, `validate_frontmatter.rs:218`, `relocate_audit_records.rs:421`, `server.rs:547`, `review-freshness.sh:7`) alone

- **Done when**: No user-facing runtime surface — tool description, CLI help, or emitted message — places either record in `spec.md` frontmatter, and every doc comment that does is either corrected or demonstrably past-tense.

## 23. Discharge the 022 obligation the relocation created

- [x] Task 17's search found 10 lines across 6 of `022-deterministic-runtime`'s **durable contracts** that place the records in `spec.md` frontmatter — `data-model.md` at 750, 752, 759, 787 and 791 (the canonical registry of the gate's check order and the two digest subject sets), and one line each in the scenarios `implement-completion-gate`, `primitive-robustness-hardening`, `review-base-includes-the-transition-commit`, `review-gate-unexaminable-contracts` and `review-runtime-acceleration`
- [x] Nothing in 022's `spec.md`, `plan.md` or `tasks.md` is affected — the exposure is entirely in the digested set, which is what prices it
- [x] 022 is the durable home for runtime rules (AGENTS.md §Workflow), so these are 057's to correct: reopen 022, fix the claims by tense, and link back here. **The estimate above was low by a factor of four.** Sweeping on three patterns rather than the location classifier alone returned 69 candidates across 23 files, of which 41 were real across 17 durable contracts — 16 in `data-model.md`. The classifier could not see most of them by construction: it keys a record token to `spec.md` / "spec frontmatter" / "the spec", and `ordered after every \`review:\` check` names no file. Four scenarios needed more than a re-point because their closing argument rested on Family 31 or on an asymmetry 057 removed
- [x] Price it before starting: every hit is a durable contract, so this costs a reopen **plus a full five-pass re-review**, and AGENTS.md records that a full 022 re-review is the largest single unit in the corpus and has never been spent
- [x] Declare `022-deterministic-runtime` in this spec's `cross-spec-impact:` so the pre-done gate holds 057 until 022 links back

- **Done when**: No durable contract under 022 places either record in `spec.md` frontmatter, 022's review and analysis are current, it is back at `done` with a back-link to 057, and the `cross-spec-impact:` entry is discharged.

## 24. AC7 is not met: `analysis.md` stales its own record

Found 2026-09-16 recording the task-18 discharges' analyze runs. The
completion gate answered `analyze-stale` naming `analysis.md` itself,
immediately after `write-analysis` wrote it.

- [x] `subject_digest` excises `analysis.md`'s **frontmatter** and digests its **body**, which the same `write-analysis` call rewrites — so any run whose Summary or counts differ from the previous one stales itself, and the operator has to run `/{project}:analyze` twice to converge. Verified both ways on 020: run 1 → `blocked: analysis is stale — 1 artifact(s) changed since it ran: specs/020-code-review/analysis.md`; run 2 → `passed: true`, with the file byte-identical between them
- [x] AC7 says the run "does not report itself stale" without qualification, and the doc comment on `subject_digest` states the same intent — "a digest covering it can never match, and every analysis would report itself stale the instant it was recorded". Only the frontmatter half was implemented
- [x] Fix by excluding `analysis.md` from the analyze subject set **entirely**, not by excising more of it. It is `/{project}:analyze`'s own output, exactly as `review.md` is `/{project}:review`'s and is excluded wholesale from the *review* digest for the identical stated reason. Nothing reads its body: the one family that reads the file — `analyze-state-drift` — reads the record, which is already excised
- [x] Record the rule where it lives: 047's `analyze-record-freshness` carries the subject set and the exclusion's history; 022's `data-model.md` carries the canonical registry. Both reopened and corrected 2026-09-16
- [x] Re-record the analyses of every spec whose digest carries an `analysis.md` key after the fix lands. Measured 2026-09-16 as exactly four — the records written since the relocation. 047 and 022 were re-recorded in this pass; **020 and 026 were not**, and that is deliberate rather than overlooked: both are `done`, `check-review-gate` returns early on a `done` spec and no audit family reads analyze freshness, so their records join the latent set task 25 measures at 52 of 52 and self-heal on the next reopen. Re-opening two closed specs to rewrite a field nothing reads would cost two reviews and two analyses to change no verdict

- **Done when**: two consecutive `/{project}:analyze` runs over a spec whose findings changed leave the first one current, no recorded digest carries an `analysis.md` key, and both durable contracts state the exclusion and its reason.

## 25. Price the migration's effect on every recorded analyze digest

Measured 2026-09-16 over the whole corpus, by recomputing each done spec's
subject digest and comparing it to the record: **52 of 52 done specs'
analyze records are stale**, each on the same three paths — `analysis.md`,
`review.md` and `spec.md`.

- [x] Confirm the cause — **and the first measurement got the attribution wrong**. Re-measured 2026-09-16 with the causes separated: all 54 mismatch on `spec.md`, and that is the completing `set-status` flip alone (proven by re-hashing with `status: in-progress` restored, which reproduces the recorded digest exactly) — pre-existing, by design, and nothing to do with this spec. **50** additionally mismatch on `review.md`, and that is the migration's doing. The original bullet's reasoning follows, corrected: `analysis.md` did not exist when those records were written so no key was recorded; the sweep rewrote every `review.md` to carry the merged record; and it removed both blocks from every `spec.md`, whose old digest had been taken with the `analyze:` block excised and the `review:` block **included**
- [x] Decide and record the disposition. The staleness is truthful — the artifacts did change — and it is latent rather than live: `check-review-gate` returns early on a spec already at `done`, and no audit family checks analyze freshness, so nothing reports it and nothing is blocked. It self-heals on the next analyze run of any spec that is reopened, which is what the task-18 discharges just demonstrated on 020 and 047
- [x] State it as a limitation in `plan.md` and in `framework/migrations/audit-record-relocate.md`, so an adopter who reopens a migrated spec meets a priced expectation rather than a surprise. Backfilling the digests is **not** the answer — it would assert a run that nothing on disk substantiates, which is the reasoning 047's AC11 already used to reject backfilling the record itself
- [x] Check this against AC10's claim that the migration "leaves `spec.md` with no residual block" — that criterion is met; this is a consequence the spec never claimed either way, which is why it needs stating rather than fixing

- **Done when**: the disposition is recorded in both artifacts with the measurement that priced it, and no criterion asserts a freshness property the migration does not leave true.

## 26. Close the 076 reopen

The owner reopened this spec `done → in-progress` on 2026-11-03 (`53a87a87`): 076's sweep (`787093ed`) swapped `routed`/`discarded` for `deferred`/`waived` in `data-model.md`, and the token appears in that one file only, so the mechanical-rename rule did not cover it and the self-audit's Family 19 reported the review stale. The swap landed before the reopen, so the default window holds none of it: review with `--since=1405eacc`, reading only what changed since 076's review plus this spec's own artifacts, then analyze and propose `done`. Until it is `done`, Family 7 reports the 057/020 and 058/057 pairs.

- [ ] Review with `--since=1405eacc`, analyze, and close `done` with the owner's approval

- **Done when**: `review.md` and `analysis.md` are current, `check-review-gate` passes, Family 19 no longer reports 057, and the owner approved the transition.
