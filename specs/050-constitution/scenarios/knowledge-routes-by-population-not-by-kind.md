---
section: "Classification"
---

# Knowledge-routes-by-population-not-by-kind

## Context

This spec classified 54 rule-bearing `AGENTS.md` entries and promoted 23 of them into the constitution — 3 more were already in the promoted shape — and recorded the test that decided each one: §Classification's three tiers are *"one question about the rule's **population**"*. The promotion worked. What it did not do was change where the **next** rule lands, and the measurement is unambiguous about the consequence.

`AGENTS.md` held 54 rule-bearing entries at the 2026-08-17 survey. It held 92 on 2026-09-13, 106 on 2026-09-14, and 117 on 2026-09-15 — three measurements in three days, with **zero** entries promoted across that window. The queue refills faster than a promotion pass drains it, so a second bulk pass run against the same routing would reset a counter and nothing else.

The cause is not neglect, and it is not this spec's **AC1**: that criterion correctly refused a per-entry classification marker, because a field an author must remember to fill is the diligence dependency [§design-principles](../../../framework/constitution.md#design-principles) rejects. The cause is that **every write-time routing rule discriminated by *kind*, while promotion discriminates by *population*.** Three sites told an author where a learning goes and not one of them asked the universality question:

- `AGENTS.md`'s header said only *"append durable learnings to the matching section"* — a list of kinds.
- `CLAUDE.md`'s Auto-Memory Routing asked *"would this help any other contributor **to this project**?"* — a question whose scope stops at the repository boundary, so every answer above it is `AGENTS.md`.
- [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s *Shared knowledge stays in git* answered by kind outright: *"A **project learning** — a convention, a gotcha, a workflow rule, a boundary — goes in `AGENTS.md`."*

So a **universal** gotcha landed in `AGENTS.md` *because it was a gotcha*, never because anyone judged it project-only. The population test that would have caught it lived in this spec, which nobody reads while writing a one-line learning.

## Behavior

**Two questions decide where a learning lands, and they are asked in that order.** The first — *would this help a teammate?* — decides whether it is committed at all. The second — *who is it true for?* — decides **which committed artifact**, and it is a question about population, never about kind.

The canonical statement is [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s *Shared knowledge stays in git*, which now routes a project learning by the three tiers §Classification already names — every project running the pipeline, one organization's projects and no further, or this project alone — instead of sending all three to `AGENTS.md` because they are all "project learnings".

Agent-facing surfaces mirror it, and the pairing matters because one ships and one does not. Four did until `memory-routing-lives-in-agents-md` (2026-10-27) moved the routing out of both `CLAUDE.md` files and made an adopter's copy a block `/{project}` writes:

| Surface | Reaches |
| --- | --- |
| `AGENTS.md` header | this repository's contributors, and its Claude sessions through `@AGENTS.md` |
| the `ductus:memory-routing` block, which `framework/templates/project/agents.md` ships empty | every adopter's `AGENTS.md`, on every `/{project}` run |

The two templates are the half a three-site reading of this work misses. They carried the by-kind routing **byte-identical** to this repository's copies — `AGENTS.md:5` and `framework/templates/project/agents.md:5` were the same bytes — so every adoption seeded the same defect into a new repository, where it then produced the same backlog for the same reason, invisibly.

**Kind is not population, and routing by kind is the failure the ordering exists to prevent.** The mistake conceals itself: a universal rule filed under a gotchas heading *is* committed, so the first question is satisfied and nothing afterwards re-asks the second. Answering the first question is precisely what makes the second easy to skip, which is why they are stated as an ordered pair rather than as two independent tests.

**This adds no authored input**, which is what keeps it clear of [§design-principles](../../../framework/constitution.md#design-principles)' bar. An author already chooses a destination when writing a learning down; what changes is which question decides that destination, not how many questions there are. Nothing new must be remembered, no field can be left empty, and an author who ignores the rule produces a misfiled entry rather than a silently degraded feature — the same failure available before, no worse.

## Edge Cases

- **A rule that is universal in substance but stated in this repository's terms.** Unchanged: it is **borderline**, and the reword test in `spec.md` §Classification decides it. The population question asks who a rule is true *for*; the reword test asks whether it can be *said* without repo-only machinery. A rule can pass the first and fail the second, and then it stays until it is reworded.
- **A learning discovered while the population is genuinely unknown.** One instance is not a population. It goes to `AGENTS.md`, which is the honest answer rather than a hedge — the tier the evidence supports — and the standing-notice half of the inbox item leaves it in the unclassified set where a later pass re-asks. Guessing *universal* from a single instance is the error in the other direction and is more expensive, because it ships to every adopter.
- **An organization with no registered shared constitution.** The middle tier has no destination, so a rule true across that organization's projects has nowhere canonical to go. It stays in `AGENTS.md` per project until the organization registers one under `.ductus/config.toml` `[constitutions.*]`. This is the state this repository is in — `resolve-constitutions` reports `examined: 0` here — so the tier is a destination named in advance rather than one in use.
- **Both templates are `skip`-strategy manifest entries.** The installer writes `AGENTS.md` and `CLAUDE.md` only when they do not already exist, so a template edit reaches **new** adoptions only: an adopter's `AGENTS.md` is theirs, and overwriting it to fix a header would destroy every rule they had written in it. The routing therefore lives in the `ductus:memory-routing` block, which `/{project}` writes into every adopter's `AGENTS.md` on every run (`memory-routing-lives-in-agents-md`), and an existing adopter's header keeps whatever text it was seeded with.

## What this does not do

**It does not promote anything, and it does not make promotion happen.** The 117 entries standing at the time of this change are untouched — deliberately, because promoting opportunistically while fixing the routing would leave the unclassified set unmeasurable, which is the state the notice half exists to end. The bulk pass is its own unit of work and its own spec cycle.

**Nothing enforces the population question.** No check compares an `AGENTS.md` entry against the constitution to ask whether it should have been promoted; whether a rule is true for every adopter is semantic judgment, which [§runtime-boundary](../../../framework/constitution.md#runtime-boundary) principle 2 keeps off the runtime regardless. What this change does is make the correct question the one an author meets at the moment of writing, in both places where they might meet it. That is a better default, not a gate, and saying otherwise would be the defect [§grounding](../../../framework/constitution.md#grounding) names — a rule implying enforcement it does not have.

**It does not measure the existing backlog.** The standing-notice half of the routing item — rendering *unclassified = rule-bearing − (classified ∪ promoted)* — is unbuilt, and until it exists the count of unpromoted universal rules is derived by hand or not at all. The two halves are independent: this one stops new misrouting, that one makes not-having-promoted visible. Neither substitutes for the other, and shipping this one does not discharge that one.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Why change the routing rather than just run the promotion pass again?** Because the measurement says a pass alone resets a counter that refills. 54 rule-bearing entries at the 2026-08-17 survey, 92 on 2026-09-13, 106 on 2026-09-14, 117 on 2026-09-15, with zero promoted across that window — and the last two increments came from ordinary work that filed a universal rule in `AGENTS.md` because the write-time default said to. The routing fix is what makes the next pass's result hold. Resolved 2026-09-15.
- **Does this reopen AC1's rejected per-entry marker?** No, and the distinction is the whole reason this is shippable. AC1 refused a *new authored field* — something an author must remember to fill, which degrades silently when they do not. This changes the **question** attached to a choice the author already makes. There is no field, nothing can be left empty, and a misapplication produces a misfiled entry rather than a feature that quietly stops working. Resolved 2026-09-15.
- **Why four mirrors rather than one canonical statement?** There is one normative statement — the constitution's — and the other three are pointers in the register their reader is already in, which is §Promotion mechanism's shape rather than an exception to it. The routing question has to be met at the moment of writing, and an author writing a one-line learning is looking at `AGENTS.md` or at an agent's memory prompt, not at the constitution. A rule reachable only by reading the document nobody opens while writing is the diligence dependency in another costume. Resolved 2026-09-15. **Superseded in part by `memory-routing-lives-in-agents-md`** (2026-10-27): the two `CLAUDE.md` mirrors reached only `claude-style` agents, so the routing moved to `AGENTS.md`, the file every agent reads; the reasoning for a mirror in the author's register stands.
- **Why do the shipped templates take the same edit?** Because they are where the defect propagates. `AGENTS.md:5` and `framework/templates/project/agents.md:5` were byte-identical, so every adoption seeded the by-kind routing into a new repository, which then grew its own unpromotable backlog for the same reason and with nothing to notice. Fixing only this repository's copies would have fixed the instance and left the mechanism. The item this work came from enumerated three sites; the templates are the two it missed. Resolved 2026-09-15.
- **Does the middle tier do anything in this repository?** No, and it is named anyway. This repository registers no `[constitutions.*]` entry and `resolve-constitutions` reports `examined: 0`, so the shared tier has no destination here. It is stated because the routing question is asked by adopters too, and an organization running several projects on this pipeline is exactly who the tier exists for — omitting it would send their cross-project rules to `AGENTS.md`, one copy per repository, which is the drift one tier up. Resolved 2026-09-15.
