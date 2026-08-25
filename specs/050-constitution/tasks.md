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
