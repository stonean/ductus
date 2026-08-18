# 050 — Constitution Plan

Implements [050 — Constitution](spec.md).

## Overview

Three pieces of work over two files. Classify every rule-bearing `AGENTS.md`
entry, move the universal ones into `framework/constitution.md` leaving pointers
behind, and add two rules that were learned here but belong to every adopter —
criterion verification and the mechanical-edit test.

The work is almost entirely prose, and the risk is not that it breaks but that
it drifts: two copies of a rule, a promoted rule that still names this
repository's machinery, or a moved anchor another artifact cites. The plan is
shaped around making each of those checkable rather than careful.

## Technical Decisions

### The classification is written down before anything moves

The survey behind this spec was a judgment call recorded as three counts. AC1
asks for something stronger: every entry carries a classification *and* the
reason. So the first task produces the full table — one row per rule-bearing
entry, with its verdict and a one-line reason — and nothing is promoted until
that table exists.

The table lives in this plan rather than in `AGENTS.md`, per the clarify walk's
fourth resolution: a per-entry marker in `AGENTS.md` would add authored state to
every future entry, which §Design Principles rejects as a design that depends on
author diligence.

Scope is the four rule-bearing sections — `Workflow`, `Gotchas`, `Boundaries`,
`Design Principles`. `Project Structure` and `Tech Stack` are descriptive prose
about this repository, not rules, and are out of scope. As of this plan those
sections hold 34, 16, 2 and 2 entries respectively; the pass runs against the
file as it stands at implementation time, not against those numbers (spec
§State at hand-off).

### The reword test is applied, and its output recorded

The borderline group is decided by the test the clarify walk chose: an entry is
promoted if and only if it can be restated without naming machinery that exists
only here, *and* without losing what makes it actionable. Both halves matter —
dropping the second turns the test into "can it be made vague enough to ship".

The recorded reason is what makes it auditable: for a promoted borderline entry,
the reason names the machinery removed and asserts the rule still bites; for a
rejected one, it names what was lost. `scripts/audit/`, `runtime/`, the agent
registry, the `ductus-v*` release loop, the retired project name and the cargo
and rustup gotchas are all repo-only surfaces — an entry resting on one of them
fails the first half unless the rule survives its removal.

### Promotions land as bullets, in the section that already owns the subject

Per the clarify walk's second resolution. Each promoted rule is a bullet under
the existing section whose subject it shares — a spec-lifecycle rule under
§spec-lifecycle, a frontmatter rule under §text-first-artifacts, an
artifact-integrity rule under §drift-prevention — and a new section only when
the rule genuinely stands alone, as §recommendations did.

This is also what keeps AC10 cheap. The constitution carries 40-odd anchors of
the form `<!-- §name -->` that other artifacts cite by name; appending a bullet
inside a section moves none of them, while promoting a dozen rules to sections
would roughly double the top-level surface and put one-paragraph rules at
section rank.

### The mirror points and does not restate

A promoted entry's `AGENTS.md` line is rewritten to name the constitution
section and say nothing normative of its own — the shape §recommendations
already uses ("The rule is §recommendations … this entry is the contributor-side
mirror, not a second copy"). Two copies of a rule is precisely the drift
§drift-prevention exists to prevent, and a mirror that restates is two copies.

The check for this is mechanical and worth stating as one: grepping a promoted
rule's distinctive phrasing must find exactly one normative statement and one
pointer (AC3).

### Two rules are authored rather than promoted

**Criterion verification** (AC6) is not in `AGENTS.md` at all — it was learned
after the survey. It is universal for a structural reason: `check-artifacts`'
`criterion-path-existence` family examines specs at `done` only, so a spec that
sits in progress indefinitely never has its criteria examined, and the family
proves only that a path *resolves* — a criterion whose paths all exist while its
claim about their contents is false is invisible to it. Both halves were
observed here on 2026-08-17. Adopters run the same pipeline and inherit the same
blind spot.

**The mechanical-edit test** (AC13) settles what §spec-lifecycle's three
enumerated cases leave open: whether an edit that changes no claim is mechanical.
Today the list reads as closed, so a pure typo repair in a `done` spec's body is
argued to reopen it — which is the disproportion the held `045` chore in
`specs/inbox.md` is waiting on. The section gains a stated test rather than a
fourth enumerated case, so the next new case is decidable without another
constitution change.

### The version pin does not move

AC12, and it is a constraint rather than a deliverable. The repo-root `version`
file is the runtime acquisition pin: `/ductus` reads it to choose which release
to fetch, and `/ductus:audit` Family 20 requires it to equal
`runtime/Cargo.toml` and the newest `runtime/CHANGELOG.md` heading
(`scripts/audit/version-agreement.sh`). Bumping it alone fails that family;
bumping all three declares a runtime release that must be tagged `ductus-v*` or
every adopter fetches assets that do not exist. The constitution reaches
adopters by the **Shared Files** manifest row copying
`framework/constitution.md` to `.ductus/constitution.md`
(`framework/bootstrap/ductus.md:714`), which tracks `main` — so no bump, and no
tag.

### Verification is the audit, not a reading

AC7 through AC10 are all checkable without judgment, and the plan leans on that
rather than on care: `/ductus:audit` Family 1 catches a promotion that
contradicts another document's claim, the manifest row proves adopters receive
the file, `markdownlint` covers AC9, and the anchor set is a grep. The
substantive risk the checks do *not* cover is AC5 — whether a promoted rule
truly holds for an adopter — which is why the reword test's recorded reason
carries that weight.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | Receives every promoted rule as a bullet under the section owning its subject, plus the criterion-verification rule, the mechanical-edit test in §spec-lifecycle, and a §canonical-sources row naming this spec |
| `AGENTS.md` | Modify | Each promoted entry rewritten to a pointer; project-only entries untouched |
| `specs/050-constitution/plan.md` | Modify | Carries the classification table — the audit trail AC1 requires |
| `specs/inbox.md` | Modify | The held `045` chore is resolved against the new mechanical-edit test and removed |
| `specs/045-decision-state-drift-detection/spec.md` | Modify | The one-word sweep-residue repair the chore describes, once the test licenses it |

## Trade-offs

**The classification table lives in a plan, which is a design record.** Plans are
not durable contracts, so nothing gates on this table staying accurate as
`AGENTS.md` grows. The alternative — a per-entry marker in `AGENTS.md` — was
rejected in the clarify walk because it depends on every future author
remembering to classify. Accepted limitation: the table is a snapshot of one
pass, and a later promotion round re-derives rather than amends it.

**Promoted rules lose their war stories.** `AGENTS.md` entries carry the failure
that produced them ("surfaced 2026-08-17 when…"), which is much of what makes
them stick. An adopter-neutral restatement cannot cite this repository's
incidents, so the constitution gets the rule and the mirror keeps the story.
Considered and rejected: shipping the incident text too, which would put
this project's history into every adopter's constitution.

**The reword test is a judgment, applied by one reader.** It is more auditable
than a vote — the reason is recorded per entry and can be disagreed with
specifically — but it is not mechanical, and two readers could classify a
borderline entry differently. Accepted: the alternative was deferring the whole
borderline group, which costs adopters ~10 substantively universal rules.

**Nothing verifies AC5 mechanically.** No check proves a promoted rule holds for
an adopter; a rule can pass every gate here and still read as advice about
somebody else's repository. The reword test's recorded reason is the only
defence, and it is a written argument rather than a check.

**Bullets are less citable than sections.** A rule promoted as a bullet has no
anchor of its own, so another artifact can cite only its containing section.
Accepted per the clarify walk: the alternative doubles the constitution's
section surface, and a rule that later proves worth citing directly can be
promoted to a section without moving any existing anchor.
