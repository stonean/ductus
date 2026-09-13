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

- [ ] Implement the behavior described in `scenarios/a-retired-filename-leaves-a-decision-record.md`
- [x] Resolve the scenario's open question first — whether the corpus rewrite is a §spec-lifecycle case (a) mechanical edit or takes the back-edge on each spec it touches; the answer sets the cost and must not be decided during the pass
- [x] Amend §drift-prevention to distinguish a retired identifier from a retired filename a compatibility path still reads, naming the exempt sites as a category (resolution ladder / fallback tier, migration procedure, the prose and shell spelling either) rather than as this repository's paths
- [x] State the corpus end state: the current name wherever a spec states current behavior, the retired name only in the few references recording the decision to change it
- [x] State that a ticked acceptance criterion is annotated rather than rewritten, citing 017 and 027 as the existing practice
- [x] Repoint the `AGENTS.md` §Gotchas entry on `.govern.toml` at the new constitution text so it states no normative content of its own, per AC3's promoted-entry pattern
- [ ] Audit the 317 `.govern.toml` occurrences under `specs/` per-hit: correct present-tense prose to `.ductus/config.toml`, annotate ticked criteria, and leave compatibility-path and decision-record occurrences as written
- [x] Leave every occurrence outside `specs/` untouched — 144 occurrences at the 2026-09-13 baseline (the earlier "131" counted matching *lines*, not occurrences): the `runtime/src/` resolution ladder, the `framework/migrations/` procedures that must name the file each migration migrates, and the `framework/commands|bootstrap`, `scripts/` and generated `.claude/` copies spelling the same ladder. `AGENTS.md` is the one deliberate exception, rewritten by the subtask above

- **Done when**: §drift-prevention carries the retired-filename rule with its exempt-site categories and the annotate-not-rewrite disposition; `AGENTS.md` mirrors it by reference; every `.govern.toml` occurrence under `specs/` has been judged individually, with the survivors limited to the compatibility-path and decision-record cases; and the 131 occurrences outside `specs/` are unchanged.

## 16. Correct 016's dead README section pointer

- [ ] `specs/016-cross-cutting-rules/spec.md:73` cites *the README's "Pinning files with …" section*; the README was restructured and has no such section under any name, so the pointer was already dead before the filename pass renamed the file it mentions
- [ ] Repoint it at what the README actually documents — `[pinned]` under §Configuration — or drop the parenthetical and name the key directly
- [ ] This is a **factual correction**, not a substitution: it rewords the line, so §spec-lifecycle's back-edge applies and 016 reopens `done → in-progress` for it. Do not bundle it with the filename sweep, which is exempt precisely because it rewords nothing
- [ ] `check-corpus-links` cannot catch this class — a prose reference to a section title is not a link — so note whether the corpus carries more of them before closing

- **Done when**: 016's spec body no longer points at a README section that does not exist, the correction is taken through the back-edge rather than folded into the exempt sweep, and the run says whether other prose section-title pointers were checked.

## 17. Apply the retired-filename rule to the three sibling names

- [ ] Task 15 was scoped to `.govern.toml` alone, but the rule it states governs every retired filename a compatibility path still reads. Measured 2026-09-13, three siblings carry comparable residue: `.govern.session.toml` (167 occurrences, 92 under `specs/`), `.govern/` (171, 40 under `specs/`), `.governance.toml` (36, 27 under `specs/`)
- [ ] Judge them by the same test — the resolution ladder's legacy tiers, the migration procedures that must name both sides, and the decision records stay; present-tense prose naming them as current is residue and takes the current name (`.ductus/session.toml`, `.ductus/`, and — for `.governance.toml` — whichever tier the ladder actually resolves)
- [ ] `.governance.toml` needs care: `framework/migrations/governance-config-rename.md` renames it **to** `.govern.toml`, so both sides of that procedure are retired today and both must survive
- [ ] Keep the prose substitutions free of any reworded line so the sweep exemption holds, exactly as task 15's mechanical half did — the annotations are what reopen a spec

- **Done when**: Every `.govern.session.toml`, `.govern/` and `.governance.toml` occurrence under `specs/` has been judged individually against §drift-prevention's retired-filename rule, the compatibility-path and decision-record occurrences are named as deliberate survivors with a count, and nothing outside `specs/` moved.
