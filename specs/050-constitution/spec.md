---
status: in-progress
dependencies: []
review:
  last-run: 2026-08-25T18:36:17Z
  reviewed-against: 09d954565dea9f7787f04d0097b4017f54b99a8e
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 16
---

# 050 — Constitution

The constitution is the one artifact every adopter receives and every command
reads, and until now no spec owned it. This spec is its home: what belongs in
it, how it is structured, and what reaches adopters through it.

## State at hand-off (2026-08-17)

**Clarified 2026-08-17, after being parked at the operator's judgment.** Created
2026-08-17 by `/{project}:groom`, which routed two inbox items here: the survey
of `AGENTS.md` entries that are true for any ductus project but ship to nobody,
and a criterion-verification rule that belongs in the constitution for the same
reason. Both are recorded below; the inbox items were removed as migrated.

The six Open Questions were real rather than scaffolding, and none was answered
by an agent acting alone — every answer ships to every adopter, and the last of
them asked what else belonged in this spec's scope, which is a scoping decision
rather than a research task. They were walked through `/{project}:clarify` on
2026-08-17 and are recorded under §Resolved Questions: three were operator
decisions, and three turned out to be settled by existing sources rather than by
preference — the version-pin question by Family 20 and the **Shared Files**
manifest, the classification-location question by §Design Principles, and the
retire-the-mirror question by this spec's own §Promotion mechanism.

One question already has a caller waiting. `specs/inbox.md` carries a one-word
sweep artifact in `045`'s **spec body** that is held here on purpose: repairing
it would trigger the `done → in-progress` back-edge to fix a word that changes
no claim, states no requirement, and alters no behaviour. §spec-lifecycle
enumerates three mechanical-edit cases and a pure typo repair is none of them,
so read strictly it reopens a done spec. Whether an edit that changes no claim
counts as mechanical is a constitution question, and settling it here also
settles that chore. The matching instance in `045`'s `plan.md` was already
repaired, since a design record is not a durable contract.

No implementation work has started. With the questions resolved, the spec is
ready for `/{project}:plan`.

**The survey is stale, and the classification pass must not inherit its count.**
It was taken on 2026-08-17 against 56 entries. The rule-bearing sections now
hold 54 — `Workflow` 34, `Gotchas` 16, `Boundaries` 2, `Design Principles` 2 —
and two of those postdate the survey entirely, both learned from the same
adopter runs that produced this spec's neighbours. AC1 asks that *every* entry
carry a classification, so the pass is run against the file as it stands when
the work happens, not against the survey's numbers.

## Ownership

The constitution had no owning spec. Specs amended it constantly — `021` set
the runtime boundary, `048` replaced principle 3 and the opt-in invariant, `044`
moved the file, `013` established text-first artifacts — but each of those owns
a *behavior* that happened to require an amendment. None owns the document.

That split stays, and this spec does not disturb it: **a spec that changes
behavior still amends the principle its change contradicts**, in the same
change, because an amendment separated from the behavior it licenses is the
"constitution violation, not a feature" the constitution itself names.

What lands here is work whose *subject is the constitution* — deciding what
belongs in it, what does not, how its sections are organized, and which rules
adopters should receive. That work had nowhere to go, which is why a survey of
promotable rules sat in the inbox rather than in a spec.

This is the same split `022-deterministic-runtime` already uses for the
runtime: the requiring spec keeps its requirement and its criteria, while the
artifact's own rules accumulate in the artifact's home.

## Current work: promoting universal rules

`AGENTS.md` was contributor-only. The **Shared Files** manifest shipped
`framework/constitution.md` to `.ductus/constitution.md`, along with the rule
files, the hooks and the templates — but never this repository's `AGENTS.md`.
So a rule learned here reached no adopter, however universal it was, and the
only way an adopter could benefit was to independently make the same mistake.

The file had accumulated 56 entries by 2026-08-17. A survey that day put
roughly 12 of them as strongly universal, about 10 as borderline, and about 24
as genuinely specific to this project. The universal group was not marginal
material: it included both §Design Principles entries, the fact that a markdown
link in a spec body creates a `dependencies:` edge, that
`git checkout -- specs/{feature}/` silently reverts uncommitted pipeline state,
that `git add -A` sweeps untracked drafts into a commit, that an `AC{n}` label
must never be hand-written, that `create-scenario` appends its own question
scaffolding, that a new rule belongs on its surface's home spec rather than in a
new spec, that a done spec is re-opened via `set-status` when only on-disk edits
need reflecting, that a behavior change needs a prose-claim sweep an identifier
sweep will not catch, that a history-reading CI check needs full history, that
the project config is a shared database rather than one spec's schema, and that
a superseded acceptance criterion is recorded on the criterion itself.

Every one of those describes the pipeline every adopter runs, not this
repository's particular shape. Each was also learned the same way — by the
failure it now prevents — so the asymmetry was that adopters paid for the
lesson twice.

§recommendations was promoted on 2026-08-17 as the first instance, and it
established the shape: the canonical text in the constitution, a short
contributor-side mirror in `AGENTS.md` pointing at it rather than restating it.
What that instance did not settle was the rest, and it could not be settled by
one more edit — the constitution is a governed artifact that ships to every
adopter, so promoting a dozen entries is a spec's worth of work rather than a
sweep.

## Promotion mechanism

A promoted rule has exactly one normative statement. The constitution carries
it; `AGENTS.md` keeps a single line naming the section and saying nothing the
constitution does not already say. Two copies of a rule is the drift
§drift-prevention exists to prevent, and a mirror that restates rather than
points is two copies.

The mirror stays rather than being deleted, because contributors here read
`AGENTS.md` as the index of how to work in this repository, and a rule that
silently moved out of it reads as a rule that was dropped.

## Classification

Each entry is classified exactly once, with the reason recorded:

- **universal** — true for any project running the ductus pipeline. Promoted.
- **project-only** — true because of something particular to this repository:
  its trunk-based flow, the retired project name, the `runtime/` release loop,
  the agent registry, the cargo and rustup gotchas, the primitive wiring sites.
  Stays in `AGENTS.md`, unchanged.
- **borderline** — universal in substance but stated in terms of machinery an
  adopter may not have. Resolved by the **reword test** rather than a vote per
  entry: promoted if and only if the rule can be restated without naming
  repo-only machinery and without losing what makes it actionable. See
  §Resolved Questions.

Adopter-neutral wording is what separates the first group from the third. A
rule may cite a runtime primitive, because every adopter has the runtime. A
rule that cites `scripts/audit/` cannot be promoted as written, because that
directory is this repository's own and never ships.

## Criterion verification

One rule joins this promotion that was not in the 2026-08-17 survey, because it
was learned after it, and it is universal for a structural reason.

A ticked acceptance criterion is a completed claim, and nothing verifies it.
`check-artifacts`' criterion-path-existence family examines specs at `done`
only — correctly, since a criterion on a spec still in progress may name a path
not yet created — so a spec that sits in progress indefinitely never has its
criteria examined at all. The family also only proves a path *resolves*; a
criterion whose paths all exist while its claim about their contents is false is
invisible to it, and no deterministic check can close that half.

Both halves were observed here on 2026-08-17. Closing
`022-deterministic-runtime` surfaced five criteria that described guarantees a
later spec had retired, two of which fired the moment the status flipped and
three of which no check flags at all. Reviewing `048-govern-acquired-runtime`
found a criterion ticked and false, and a second that would have fired the
instant the spec closed. Every one was found by reading the criteria against the
tree by hand.

Adopters run the same pipeline, tick the same criteria, and get the same
`done`-only scoping, so they inherit the same blind spot. The rule is therefore
a constitution rule: verify a spec's ticked criteria against the tree before
closing it, and treat a ticked criterion as a claim to be re-earned rather than
a fact already banked.

## Acceptance Criteria

- [x] AC1: Every entry in `AGENTS.md` carries exactly one classification — universal, borderline, or project-only — and the reason it was assigned, so the classification is auditable rather than asserted
- [x] AC2: Each entry classified universal has its canonical normative text in `framework/constitution.md` under a named section with a stable anchor
- [x] AC3: Each promoted entry's `AGENTS.md` line points at its constitution section and states no normative content of its own, so grepping a promoted rule's distinctive phrasing finds one statement and one pointer
- [x] AC4: No entry classified project-only is altered by this spec
- [x] AC5: Every promoted rule is worded so it holds for an adopter: it cites the constitution, the pipeline commands, the artifacts, or runtime primitives, and never a path that exists only in this repository
- [x] AC6: The constitution states that a spec's ticked acceptance criteria are verified against the tree before it closes, naming both gaps that make the check necessary — that `check-artifacts` examines `done` specs only, and that a false claim whose paths resolve is not detectable
- [x] AC7: An adopter receives every promoted rule: the sections promoted here are present in the constitution the **Shared Files** manifest copies to `.ductus/constitution.md`
- [x] AC8: `/ductus:audit` passes with the promoted sections in place, Family 1 (cross-doc claim consistency) included, so no promotion contradicts a claim another document makes
- [x] AC9: `npx markdownlint-cli2` passes over `framework/constitution.md` and `AGENTS.md`
- [x] AC10: The constitution's existing section order and anchor set remain resolvable — no promotion renames or displaces an anchor another artifact cites
- [x] AC11: The constitution's canonical-sources map names this spec as the home for constitution-content work, so the ownership split above is recorded where a reader already looks for it rather than only here
- [x] AC12: The repo-root `version` pin, `runtime/Cargo.toml`, and `runtime/CHANGELOG.md` are untouched by this spec, and `/ductus:audit` Family 20 stays clean — a constitution-only change never moves the runtime acquisition pin
- [x] AC13: §spec-lifecycle states whether an edit that changes no claim — a typo or sweep-residue repair in a `done` spec's body — is a mechanical edit, so its enumerated cases read as a rule with a stated test rather than a closed list a fourth case must be argued into. The `045` chore held in `specs/inbox.md` is resolvable against that statement without further judgment
- [x] AC14: **Completion-claim filter.** §design-principles carries a hard filter stating that work which is not complete must never be indistinguishable from work that is, naming the three dispositions for known residue — fix it, record it where the pipeline resurfaces it with the status following that record, or record an out-of-scope decision with its reason — and requiring that residue knowable only by measurement be measured rather than caveated. §implement-phase's outstanding-SHOULD rule references the filter as its most frequent instance rather than restating it, so the rule is stated once and Family 6 stays green.
- [x] AC15: **Findings route by scope.** §brownfield-inbox's Automatic issue capture states that scope decides a finding's destination, naming three tiers — inside the current task, fixed in the task; inside the current spec but outside the task, a new task on that spec's `tasks.md`; outside the spec, the inbox — and states the two things the rule does not license: `tasks.md` does not become a second capture queue or a durable record, and a chore with no feature home stays an inbox item however close to the current work it surfaced. The section's closing sentence names both destinations rather than only the inbox, and the `AGENTS.md` mirror points at the section without restating it.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **What decision rule resolves the borderline group?** The **reword test**, applied per entry: a borderline entry is promoted if and only if it can be restated without naming machinery that exists only in this repository *and* without losing what makes it actionable. Applying it to each borderline entry is part of this spec's work, and the per-entry outcome plus its reason is exactly what AC1's audit trail records — so the property becomes a rule by being tested against the corpus rather than asserted over it. Resolved 2026-08-17.
- **Do promoted rules join existing constitution sections or get their own?** **Bullets under an existing section by default; a new section only when the rule stands alone**, as §recommendations does. AC2 asks that the canonical text sit "under a named section with a stable anchor", which a bullet under an existing named section satisfies. Defaulting to bullets also keeps AC10 cheap: the constitution's largest sections already run to roughly 69 lines, so promoting about a dozen entries as sections would roughly double its top-level surface and raise one-paragraph rules to section rank. Resolved 2026-08-17.
- **Does a constitution-only change need a version bump?** **No — and the repo-root `version` file must not move for one.** That file is the *runtime acquisition pin*: `/ductus` reads it to decide which release to fetch, and `/ductus:audit` Family 20 requires it to equal `runtime/Cargo.toml` and the newest `runtime/CHANGELOG.md` heading. Bumping it alone therefore fails Family 20, and bumping all three declares a runtime release that must be tagged `ductus-v<version>` or every adopter's acquisition points at assets that do not exist. The constitution reaches adopters by a different route entirely — the **Shared Files** manifest row copying `framework/constitution.md` to `.ductus/constitution.md` — so they receive it on their next `/ductus` because the archive tracks `main`. Settled from those sources rather than by preference. Resolved 2026-08-17.
- **Should the classification live in this spec's body, or as a per-entry marker in `AGENTS.md`?** **In this spec's body.** A per-entry marker adds authored state to every future `AGENTS.md` entry, and §Design Principles treats "requires an author to remember to fill it in" as a hard filter on a new input rather than a tiebreaker — the same principle the question raises against itself. Resolved 2026-08-17.
- **Does promoting a rule retire the corresponding `AGENTS.md` entry?** **No — the mirror stays, in every section including §Gotchas.** §Promotion mechanism above already fixes the shape: one normative statement in the constitution, one `AGENTS.md` line pointing at it and stating nothing of its own. Contributors read `AGENTS.md` as the index of how to work in this repository, so a rule that silently left it reads as a rule that was dropped. §Gotchas is not a separate register in this respect and takes the same pointer treatment as §Workflow. Resolved 2026-08-17.
- **Is there constitution work beyond this promotion that should land here?** **No — scope stays as drafted:** the promotion plus the criterion-verification rule. Section reordering, splitting an overlong section, and retiring a principle no spec relies on are real work but a different subject; folding them in would make AC10 materially harder to hold and would mix two unrelated review surfaces in one spec. They get their own back-edge when a concrete complaint drives them. Resolved 2026-08-17.
