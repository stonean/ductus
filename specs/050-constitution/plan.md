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

## Classification

Every rule-bearing entry, one verdict and one reason each (AC1). Entries are
cited by section and position as of this pass. **23 to promote, 3 already
promoted, 28 project-only — 54 total.**

`R` marks a verdict reached through the reword test rather than directly.

Two entries were reclassified after checking the constitution rather than the
survey: **W14 and W15 are already promoted.** They are cases (b) and (c) of
§spec-lifecycle's mechanical-edit rule, and both `AGENTS.md` entries already
cite the section by anchor — they were in the target shape before this spec
started. The survey counted them as pending because it read `AGENTS.md` alone.

That check also surfaced a defect this promotion fixes rather than causes.
§spec-lifecycle's back-edge paragraph cites "the **Design Principles** rule:
never depend on human diligence" — a rule that lives in `AGENTS.md`, which the
**Shared Files** manifest never ships. So the one artifact every adopter
receives cites a rule none of them can read. Promoting DP2 resolves the
dangling reference; DP1's substance appears at §recommendations as a supporting
clause rather than as a principle in its own right.

### Promoted — universal

| Entry | Reason |
| --- | --- |
| W4 — SHOULD gates `done` | Every adopter runs the review command and inherits its blocking semantics; a spec sitting at `done` with an unaddressed finding is indistinguishable from unfinished work anywhere |
| W6 — superseded criterion recorded on the criterion | Specs, criteria and `review.md` are all adopter artifacts, and `write-review` regenerates the Summary wholesale for them too |
| W7 — `reviewed-against` already contains what it reviewed | **R** — drops the Family 19 citation; the ordering rule (commit, review, commit the review) stands on its own and adopters' freshness check reads the same field |
| W10 — the project config is a shared database | Adopters add config sections per spec; the anti-pattern of reopening the config's originating spec is the same |
| W11 — no dead references in live artifacts | **R** — the artifact list is restated generically (specs, commands, rules, docs, README) rather than naming this repo's directories |
| W12 — a behavior change needs a prose-claim sweep | Identifier greps miss stale claims in any project; the failure mode is the wording, not the paths |
| W16 — never hand-write an `AC{n}` label | The counter is adopter frontmatter and the collision risk is identical |
| W18 — use the file-writing tool, not shell redirects, for the session file | **R** — stated as the session file rather than a permission-entry anecdote; adopters carry the same per-path grants |
| W21 — reopen a `done` spec via `set-status` for on-disk edits | The back-edge and the refinement loop are both adopter-facing |
| W22 — a new rule goes to its surface's home spec | Adopters own rule files and hit the same spec-proliferation pressure |
| W24 — syncing a canonical table on a `done` spec is mechanical | Follows from the canonical-sources map, which adopters receive |
| W27 — a CI check that reads history needs full history | **R** — the shipped adopter CI template runs exactly such a check, so this is adopter-facing already |
| W28 — a test that reads history changes its workflow too | **R** — same family as W27; stated as history/inputs rather than this repo's workflow filenames |
| W29 — a chore route means fix it, not park it | The groom command and its five routes ship unchanged |
| W30 — restoring a spec directory reverts uncommitted pipeline state | Status flips and ticked criteria are tracked-file writes in every adopter repo |
| W31 — renaming a repo orphans path-keyed contributor state | **R** — generalised from this rename to any; no migration can reach state keyed to a path outside the repo |
| W32 — read a declarative entry before reporting it misbehaved | Adopters run the migration registry and read its gating fields; this is §grounding applied to a data row |
| G5 — never `git add -A` | The untracked-draft hazard is created by the pipeline itself, which adopters run |
| G11 — a body link creates a `dependencies:` edge | The dependency generator ships; citing versus depending is an adopter distinction |
| G12 — `create-scenario` appends its own question scaffolding | An adopter-facing primitive with adopter-facing output |
| B2 — never edit an installed command file directly | **R** — stated as "the installer overwrites it; pin it instead" rather than naming this repo's generator, which is what makes it matter to an adopter |
| DP1 — a check that cannot run must not look like one that passed | Already `QUAL-CLAIM-001` in a shipped rule file; the design-time statement belongs beside it |
| DP2 — never design features that depend on human diligence | The hardest constraint on any pipeline artifact an adopter authors |

### Already promoted

| Entry | Reason |
| --- | --- |
| W5 — work a recommendation out before presenting it | Promoted 2026-08-17 as §recommendations; the entry is already the pointer-shaped mirror this spec generalises |
| W15 — criterion-label assignment is mechanical | Adopters receive the labelling pass and the same back-edge question |
| W14 — cross-service reference edits are mechanical | Cross-service references ship to adopters; the non-reopening rule is part of that contract |

### Project-only

| Entry | Reason |
| --- | --- |
| W1 — commit directly to `main` | This repo's trunk-based flow; adopters choose their own branching |
| W2 — never recreate the retired repository name | Concerns this project's distribution redirect, not an adopter's pipeline |
| W3 — a `runtime/` change ships via a tag | Adopters have no `runtime/` and cut no release |
| W8 — never record another project's name here | Arises from using outside projects to test this framework |
| W9 — read a command's source before describing it | Loses nothing an adopter needs that §grounding does not already state |
| W13 — cover every agent in the registry | The registry is the framework's own enumeration |
| W17 — run the installer per its spec, no ad-hoc prompts | Canonical statement already ships inside the installer itself; this is its mirror |
| W19 — repo-relative paths in tool calls | Agent tool hygiene rather than pipeline governance; genericised it stops biting |
| W20 — never use frozen-archaeology phrasing | The substance (specs are living documents) is already §spec-lifecycle; what remains is a phrasing ban local to this repo |
| W23 — route runtime work to spec 022 | Names this repository's own spec |
| W25 — backtick a primitive only when the walker can bind its arguments | Concerns authoring framework command sources |
| W26 — do not build twice and read the second output | A cargo-specific shell trap in this repo's toolchain |
| W33 — a real adopter run is the only test of composition | About testing this framework against adopters |
| W34 — a run summary is evidence only about what it wrote | Restates §grounding's prefer-the-source rule for a narrow case; the general rule already ships |
| G1 — run the linter via `npx` | Tooling preference, not a rule |
| G2 — the command generator's substitutions | Framework build step |
| G3 — a command edit is invisible until the generator re-runs | Framework build step |
| G4 — `init.md` is the generator's one exception | Framework build step |
| G6 — a new adopter-facing generator wires into three sites | Framework authoring |
| G7 — `write_atomic_bytes` discards file mode | Runtime internals |
| G8 — the toolchain pins clippy and rustfmt | Runtime build |
| G9 — a new primitive wires into six sites | Runtime authoring |
| G10 — `audit:ignore-promotion` does not change the parser | Framework command authoring |
| G13 — a version bump needs one unlocked build | Runtime release |
| G14 — gitignored adopter state survives a reset | About testing adopters, not being one |
| G15 — the dogfooded copy is not the shipped copy | Structurally impossible for an adopter, who has only the shipped copy |
| G16 — `path` is a reserved array in zsh | A shell trap with no pipeline connection |
| B1 — no host-level enforcement of the deterministic path | A framework design non-goal |

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
