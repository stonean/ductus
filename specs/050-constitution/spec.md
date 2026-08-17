---
status: draft
dependencies: []
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 12
---

# 050 — Constitution

The constitution is the one artifact every adopter receives and every command
reads, and until now no spec owned it. This spec is its home: what belongs in
it, how it is structured, and what reaches adopters through it.

## State at hand-off (2026-08-17)

**Draft, and deliberately parked at the operator's judgment.** Created
2026-08-17 by `/{project}:groom`, which routed two inbox items here: the survey
of `AGENTS.md` entries that are true for any ductus project but ship to nobody,
and a criterion-verification rule that belongs in the constitution for the same
reason. Both are recorded below; the inbox items were removed as migrated.

The six Open Questions are real rather than scaffolding, and none should be
answered by an agent acting alone — every answer ships to every adopter, and the
last of them asks what else belongs in this spec's scope, which is a scoping
decision rather than a research task. `/{project}:clarify` is the vehicle.

One question already has a caller waiting. `specs/inbox.md` carries a one-word
sweep artifact in `045`'s **spec body** that is held here on purpose: repairing
it would trigger the `done → in-progress` back-edge to fix a word that changes
no claim, states no requirement, and alters no behaviour. §spec-lifecycle
enumerates three mechanical-edit cases and a pure typo repair is none of them,
so read strictly it reopens a done spec. Whether an edit that changes no claim
counts as mechanical is a constitution question, and settling it here also
settles that chore. The matching instance in `045`'s `plan.md` was already
repaired, since a design record is not a durable contract.

No implementation work has started and none should until the questions resolve.

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
  adopter may not have. Resolving this group needs a decision rule rather than
  a vote per entry; see Open Questions.

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

- [ ] AC1: Every entry in `AGENTS.md` carries exactly one classification — universal, borderline, or project-only — and the reason it was assigned, so the classification is auditable rather than asserted
- [ ] AC2: Each entry classified universal has its canonical normative text in `framework/constitution.md` under a named section with a stable anchor
- [ ] AC3: Each promoted entry's `AGENTS.md` line points at its constitution section and states no normative content of its own, so grepping a promoted rule's distinctive phrasing finds one statement and one pointer
- [ ] AC4: No entry classified project-only is altered by this spec
- [ ] AC5: Every promoted rule is worded so it holds for an adopter: it cites the constitution, the pipeline commands, the artifacts, or runtime primitives, and never a path that exists only in this repository
- [ ] AC6: The constitution states that a spec's ticked acceptance criteria are verified against the tree before it closes, naming both gaps that make the check necessary — that `check-artifacts` examines `done` specs only, and that a false claim whose paths resolve is not detectable
- [ ] AC7: An adopter receives every promoted rule: the sections promoted here are present in the constitution the **Shared Files** manifest copies to `.ductus/constitution.md`
- [ ] AC8: `/ductus:audit` passes with the promoted sections in place, Family 1 (cross-doc claim consistency) included, so no promotion contradicts a claim another document makes
- [ ] AC9: `npx markdownlint-cli2` passes over `framework/constitution.md` and `AGENTS.md`
- [ ] AC10: The constitution's existing section order and anchor set remain resolvable — no promotion renames or displaces an anchor another artifact cites
- [ ] AC11: The constitution's canonical-sources map names this spec as the home for constitution-content work, so the ownership split above is recorded where a reader already looks for it rather than only here

## Open Questions

- What decision rule resolves the borderline group? The distinguishing property is whether a rule can be restated adopter-neutrally without losing what makes it actionable — a rule that survives rewording is universal, one that does not is project-only. That needs applying to the ~10 entries before it can be called a rule rather than a heuristic.
- Do promoted rules join existing constitution sections or get their own? §recommendations became its own section, which suits a rule that stands alone; several candidates here are narrower and may read better as bullets under an existing section than as sections of their own.
- Does a constitution-only change need a version bump? The constitution travels in the archive fetch rather than the runtime binary, so no `ductus-v*` tag is implied — but adopters pull it on their next `/ductus`, and it is worth stating explicitly whether the repo-root `version` pin is expected to move for a framework-only change.
- Should the classification live in this spec's body, or in `AGENTS.md` itself as a per-entry marker? A marker in `AGENTS.md` would keep the classification next to the entry and survive future additions, but it adds authored state to every future entry — which the second §Design Principles entry warns against.
- Does promoting a rule retire the corresponding `AGENTS.md` §Gotchas entry when the rule and the gotcha overlap, or are those separate registers that both stay?
- Is there constitution work beyond this promotion that should land here — section reordering, splitting an overlong section, or retiring a principle no spec relies on? Worth deciding before this spec is planned, so its scope is settled rather than discovered.
