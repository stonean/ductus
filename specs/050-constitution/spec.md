---
status: done
dependencies: []
review:
  last-run: 2026-09-12T23:03:16Z
  reviewed-against: 263a3644be0d26872006a55f063a61e430cba067
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  reviewed-digest:
    scenarios/a-declared-cross-spec-impact-gates-done.md: 17265775c8d619c2cad70f6ce7785c9734aaa3d08cf287785a6a91fde68ac981
    scenarios/a-partial-read-is-not-a-read.md: 21a78c9fe7d1b8d76f6f6f35097fce17e1ca1fe1ae3723bcb7ec007b7c9aeb56
    scenarios/a-retired-feature-leaves-no-spec.md: eadf56734b7018bdf20fc4c6b03d274f46c36bba6ba65ec43975a099eaaa98bb
    scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
    scenarios/findings-route-by-scope.md: f18d999fbe4045c1bd2a894e108e243bf3f578a220a1ebc0bcafb78e1e486bb7
    scenarios/governance-is-multi-source.md: ae59aca7a049317806297839a73cf335eeb3764db97999a7ddc0b12103e6ffeb
  blocking: false
next-criterion: 19
analyze:
  last-run: 2026-09-12T23:07:55Z
  analyzed-against: c6d53d752c9a01928837a4556c1ff7a330bd7eb9
  hard-fail: 0
  blocking-findings: 0
  advisory: 0
  unexamined: 0
  analyzed-digest:
    plan.md: cc73c3c7af6784205158b9b957264e5fbfaafa5ac4708d0a9ba097fcb52028af
    review.md: ceb15927330098d72fca82ccb56ffb44eaba9ed1da846e5565ed99db8f61cc81
    scenarios/a-declared-cross-spec-impact-gates-done.md: 17265775c8d619c2cad70f6ce7785c9734aaa3d08cf287785a6a91fde68ac981
    scenarios/a-partial-read-is-not-a-read.md: 21a78c9fe7d1b8d76f6f6f35097fce17e1ca1fe1ae3723bcb7ec007b7c9aeb56
    scenarios/a-retired-feature-leaves-no-spec.md: eadf56734b7018bdf20fc4c6b03d274f46c36bba6ba65ec43975a099eaaa98bb
    scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
    scenarios/findings-route-by-scope.md: f18d999fbe4045c1bd2a894e108e243bf3f578a220a1ebc0bcafb78e1e486bb7
    scenarios/governance-is-multi-source.md: ae59aca7a049317806297839a73cf335eeb3764db97999a7ddc0b12103e6ffeb
    spec.md: 1856cd934cd06e2493df46979016d0aa0305763bf0e7522c53464415f51defd7
    tasks.md: b00827c39400a81dc269f10da971439f4037bbbfc14d657a65d458ee15514d04
  blocking: false
---

# 050 — Constitution

The constitution is the one artifact every adopter receives and every command
reads, and until now no spec owned it. This spec is its home: what belongs in
it, how it is structured, and what reaches adopters through it.

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
material: it included both `AGENTS.md` §Design Principles entries, the fact that a markdown
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
- **shared** — true across one organization's projects and no one else's. Its
  canonical text lives in a constitution document that organization owns and
  registers under `.ductus/config.toml` `[constitutions.*]`
  (`055-shared-constitution`), which `/{project}:target` loads alongside the
  shipped `.ductus/constitution.md`. Not promoted to
  `framework/constitution.md`, and not left as a per-repository `AGENTS.md`
  entry.
- **project-only** — true because of something particular to this repository:
  its trunk-based flow, the retired project name, the `runtime/` release loop,
  the agent registry, the cargo and rustup gotchas, the primitive wiring sites.
  Stays in `AGENTS.md`, unchanged.
- **borderline** — universal in substance but stated in terms of machinery an
  adopter may not have. Resolved by the **reword test** rather than a vote per
  entry: promoted if and only if the rule can be restated without naming
  repo-only machinery and without losing what makes it actionable. See
  §Resolved Questions.

The first three are one question about the rule's **population**, which is what
the original two were testing without having to say so:

| True for | Tier | Canonical home |
| --- | --- | --- |
| every ductus project | universal | `framework/constitution.md` |
| more than one project, but not all | shared | that organization's registered constitution |
| exactly this repository | project-only | `AGENTS.md` |

The shared tier was not in the original partition, which was exhaustive only
while a project could receive exactly one governing document.
`055-shared-constitution` ended that condition. A rule true across one
organization's repositories then fits neither original tier: it is not
universal — promoting it pushes one organization's convention onto every
adopter — and it is not project-only either, since a rule retyped once per
repository is the drift §drift-prevention exists to prevent, one tier above the
one this spec addressed.

§Promotion mechanism carries over unchanged, and is what makes the tier worth
having: one normative statement, in the shared constitution, with `AGENTS.md`
keeping a line that points at it and states nothing of its own. Without that a
shared rule is a copy per repository, which is the state the tier exists to end.

The reword test carries over too, with its scope narrowed to match the tier. A
universal rule must be statable without naming machinery only this repository
has; a shared rule must be statable without naming machinery only *one project
in the organization* has, or it is project-only wearing a shared label.

One bound belongs with the rule rather than being discovered later: a shared
rule may add to the framework's rules and tighten them, never loosen them. The
framework constitution is a floor, and §governance-precedence is where that
order is stated — together with the fact that nothing enforces it.

Adopter-neutral wording is what separates **universal** from **borderline**. A
rule may cite a runtime primitive, because every adopter has the runtime. A
rule that cites `scripts/audit/` cannot be promoted as written, because that
directory is this repository's own and never ships. The same test, applied to a
narrower population, is what separates **shared** from **project-only**.

This repository registers no `[constitutions.*]` entry — `.ductus/config.toml`
carries `[host]`, `[review]` and `[runtime]` and no constitutions table, and
`resolve-constitutions` reports `examined: 0` against it — so naming the tier
adds a destination without reclassifying anything: every `AGENTS.md` entry keeps
the classification it already carries, and no project-only entry is touched.

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

- [x] AC1: Every entry in `AGENTS.md` **as the 2026-08-17 survey found it** carries exactly one classification — universal, borderline, or project-only — and the reason it was assigned, so the promotion pass is auditable rather than asserted. The scope is that corpus, which is what `plan.md` §Classification records (54 entries, cited by section and position *as of this pass*). It is deliberately **not** a standing requirement that every later entry be classified: the classification existed to decide what to promote, that migration is complete, and a rule requiring a human pass on every future entry with nothing to enforce it is the diligence dependency §design-principles rejects — `AGENTS.md` has since grown to 89 entries, which is what a standing reading of this criterion would already have made false. Where a *new* learning belongs is settled by §drift-prevention's **Shared knowledge stays in git** and the §Classification tiers above, at the moment it is written
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
- [x] AC16: §spec-lifecycle states that a retired feature's spec is deleted rather than left at `done`, naming the three cases (consolidate when content survives elsewhere, delete outright when nothing does, ordinary body edit for partial retirement) and why inbound pointers are re-pointed before removal. `AGENTS.md` carries the contributor-side mirror as a pointer rather than a second copy, and the scenario records that nothing enforces the rule yet.
- [x] AC17: §Classification names a third destination, **shared** — true across one organization's projects and no one else's, with its canonical text in a constitution that organization registers under `[constitutions.*]` — and states the three tiers as one question about the rule's population. §Promotion mechanism and the reword test carry over with the test's scope narrowed to match the tier, the framework-as-floor bound is stated with a pointer to §governance-precedence rather than a second copy, and no existing `AGENTS.md` entry is reclassified.
- [x] AC18: §cross-spec-impact separates what the framework enforces from what it cannot: a **declared** impact is recorded in `cross-spec-impact:` frontmatter and gates `done`, discharge is the affected spec's reciprocal back-link rather than the key's removal, and nothing detects an **undeclared** impact — stated as the author's and reviewer's judgment rather than implied to be covered. The acceptance-criterion sentence that previously stood as the enforcement story is replaced rather than kept alongside, and the frontmatter schema in §text-first-artifacts carries the new key.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **What decision rule resolves the borderline group?** The **reword test**, applied per entry: a borderline entry is promoted if and only if it can be restated without naming machinery that exists only in this repository *and* without losing what makes it actionable. Applying it to each borderline entry is part of this spec's work, and the per-entry outcome plus its reason is exactly what AC1's audit trail records — so the property becomes a rule by being tested against the corpus rather than asserted over it. Resolved 2026-08-17.
- **Do promoted rules join existing constitution sections or get their own?** **Bullets under an existing section by default; a new section only when the rule stands alone**, as §recommendations does. AC2 asks that the canonical text sit "under a named section with a stable anchor", which a bullet under an existing named section satisfies. Defaulting to bullets also keeps AC10 cheap: the constitution's largest sections already run to roughly 69 lines, so promoting about a dozen entries as sections would roughly double its top-level surface and raise one-paragraph rules to section rank. Resolved 2026-08-17.
- **Does a constitution-only change need a version bump?** **No — and the repo-root `version` file must not move for one.** That file is the *runtime acquisition pin*: `/ductus` reads it to decide which release to fetch, and `/ductus:audit` Family 20 requires it to equal `runtime/Cargo.toml` and the newest `runtime/CHANGELOG.md` heading. Bumping it alone therefore fails Family 20, and bumping all three declares a runtime release that must be tagged `ductus-v<version>` or every adopter's acquisition points at assets that do not exist. The constitution reaches adopters by a different route entirely — the **Shared Files** manifest row copying `framework/constitution.md` to `.ductus/constitution.md` — so they receive it on their next `/ductus` because the archive tracks `main`. Settled from those sources rather than by preference. Resolved 2026-08-17.
- **Should the classification live in this spec's body, or as a per-entry marker in `AGENTS.md`?** **In this spec's body.** A per-entry marker adds authored state to every future `AGENTS.md` entry, and §Design Principles treats "requires an author to remember to fill it in" as a hard filter on a new input rather than a tiebreaker — the same principle the question raises against itself. Resolved 2026-08-17.
- **Does promoting a rule retire the corresponding `AGENTS.md` entry?** **No — the mirror stays, in every section including §Gotchas.** §Promotion mechanism above already fixes the shape: one normative statement in the constitution, one `AGENTS.md` line pointing at it and stating nothing of its own. Contributors read `AGENTS.md` as the index of how to work in this repository, so a rule that silently left it reads as a rule that was dropped. §Gotchas is not a separate register in this respect and takes the same pointer treatment as §Workflow. Resolved 2026-08-17.
- **Is there constitution work beyond this promotion that should land here?** **No — scope stays as drafted:** the promotion plus the criterion-verification rule. Section reordering, splitting an overlong section, and retiring a principle no spec relies on are real work but a different subject; folding them in would make AC10 materially harder to hold and would mix two unrelated review surfaces in one spec. They get their own back-edge when a concrete complaint drives them. Resolved 2026-08-17.
