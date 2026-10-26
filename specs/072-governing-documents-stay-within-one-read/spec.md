---
status: done
dependencies: [063-oversized-artifacts-warn-with-a-fix]
cross-spec-impact: [050-constitution]
next-criterion: 12
---

# 072 — Governing documents stay within one read

The documents that govern how an agent works in this repository are bounded. `AGENTS.md` is small enough to be read whole in one call and `CLAUDE.md` actually loads it; the constitution can no longer grow, and its cut to one read is owed by `050-constitution`. Each has an exit: a lesson added past a ceiling retires another in the same change, so the rulebook cannot grow without bound.

## Motivation

Development had slowed, and the cause was compounding process cost rather than any slow component. Every lesson or finding became a permanent rule, check or re-review obligation, and nothing was retired, so each task had to pass a larger review than the last one, and passing it produced new findings. The measurements, taken 2026-10-26 from `git log` and the files on disk:

- **The rulebook outgrew a single read.** `AGENTS.md` grew from 14,155 bytes (2026-05-31) to 283,875; the constitution from 40,592 to 119,262. The read-size threshold `check-artifact-size` applies to spec artifacts, 50,000 bytes ([063](../063-oversized-artifacts-warn-with-a-fix/spec.md)), was the smallest size recorded past a host's read cap, so `AGENTS.md` took six reads and the constitution three. `AGENTS.md` grew a further 44 KB in the eleven days after `050-constitution` promoted its universal entries, because each round of work added entries — `0ea710b4`, *two gotchas from the body-path round*, is one instance.
- **Entries that claimed to be pointers were copies.** 56 of `AGENTS.md`'s 164 entries described themselves as *the contributor-side mirror rather than a second copy* of a constitution rule, and together held 81,563 bytes: a restatement of the rule plus the account of the session that taught it. §drift-prevention says a reference is a pointer, never a copy, that every reader pays for a copy in full in every session, and that history is git's job.
- **The guide was not loaded.** `CLAUDE.md` wrote its imports as `@import framework/constitution.md` and `@import AGENTS.md`, a form Claude Code does not resolve (its syntax is `@path`, per [its memory documentation](https://code.claude.com/docs/en/memory)), so a session started with `CLAUDE.md`'s 3 KB and none of the guide. A `claude -p` run with no file tools could not answer from `AGENTS.md` before the fix, and could after it (commit `4eebe91a`). `CLAUDE.md` also described `AGENTS.md` as *a 90KB file* when it was three times that. A rule the agent never read was broken, a review caught it, and a new entry recorded the lesson in the same unread file.
- **Effort moved from the product to the process.** In October, commits classified by subject prefix came to 352 pipeline bookkeeping (review, analyze, status, amend, tasks and the other pipeline verbs), 120 fixes and 44 build commits. Review and analyze commits per month from May to October were 12, 5, 14, 62, 286 and 151. Of the specs created in October, four reached `done` (063 to 066), none after 2026-10-16.

Together these violated four of the constitution's guiding principles — **Fast** (short time to market, rapid updates and fixes), **Learnable** (fast onboarding through clear patterns and documentation), **Cost-conscious** (cost across building) and **Serviceable** (solutions serve identified needs, not themselves) — and the design principle that no pipeline feature depends on human diligence. `CLAUDE.md` named that last violation itself: *a rule that depends on an agent reading a 90KB file in full is a diligence dependency.*

## What the contributor guide holds

An `AGENTS.md` entry states one rule, once, in the fewest words that let a contributor apply it. Four kinds of content leave it:

- **A copy of a constitution rule.** When the rule's canonical text is in `framework/constitution.md`, the entry is one line pointing at its `§anchor` and states nothing of its own. This is the shape `050-constitution`'s Promotion mechanism already requires, and its AC3 already claims. The 56 mirrors had drifted from that shape, and this restores it. Anything this repository adds to such a rule is an entry of its own.
- **The story of the lesson.** How a lesson was learned — the session, the task, the incident, the wrong turn — is history; the commit that added the entry records it. An owner decision keeps the decision and, at most, one clause naming who confirmed it and when, because that clause is what marks a standing rule rather than a preference.
- **A lesson the repository already enforces.** When a test, hook, audit family or CI check reports the failure an entry warns about, the report is the documentation. If the report does not name the fix, the fix is added to its message, or beside the enforcement site, and the entry goes. A report raised from `runtime/src/` without its fix is the exception: adding the fix there is a runtime change, so its entry stays, trimmed.
- **A lesson that is no longer true.** An entry about a file, flag or behavior the repository no longer has is deleted.

What remains is what an agent needs before it acts and no check would tell it: workflow rules, boundaries, and gotchas with no enforcement point.

## Loading the guide

`CLAUDE.md` imports `AGENTS.md` in the form Claude Code resolves, so a session starts with the guide in context instead of depending on the agent to read it. The import lands only after `AGENTS.md` is under its ceiling; fixing it first would load about 70,000 tokens into every session. Once the guide loads, `CLAUDE.md` copies none of its entries, because the reason for the copy was that the guide was not read. `CLAUDE.md` states no size or count of another file, since a count is the first thing to go stale.

## The exit

A size ceiling, checked mechanically, is what keeps the guide small after this spec closes. A commit that takes `AGENTS.md` past it fails, and the failure names the file, its size, the ceiling and the fix, so a contributor adding a lesson retires, merges or relocates another in the same change. Nobody has to remember the budget, which is what the design principle against human diligence requires.

The constitution gets the same check with a ceiling of its own, no higher than its size in the commit that adds the check, so from then on it can shrink but not grow. Cutting it to one read is `050-constitution`'s work, because that spec owns what the constitution holds; its scenario lowers the ceiling to the read-size threshold when the cut lands. This spec declares the obligation in `cross-spec-impact`, discharged by that scenario's link back here.

## Edge Cases

- **At the ceiling.** A file exactly at its ceiling passes; the ceiling is a maximum.
- **A missing or unreadable file** fails with a message of its own, never as a pass. A check that cannot run must never be indistinguishable from one that passed.
- **What is measured.** The hook measures each file as staged, so an unstaged edit neither fails nor passes a commit; CI measures the checkout. `--no-verify` skips the hook, which is why CI runs the check too.
- **Order of landing.** The check lands once `AGENTS.md` is already under its ceiling. Landing it first would fail every commit until the cut finished.
- **Entries added meanwhile.** Other sessions keep adding entries while the guide is cut; 25 `agents:` commits landed in October. Every entry present when the check lands is held to the entry rules and recorded in the plan, not only those present when the work began.
- **A lesson with nowhere to go.** When a new lesson would take the guide past its ceiling and nothing can be retired, the contributor finds its enforcement point or folds it into an existing entry. Raising a ceiling is an owner decision, recorded in this spec through its back-edge, never a choice made at commit time.
- **The constitution is not imported.** At its capped size it is still more than one read, so `CLAUDE.md` does not import it. The commands keep reading it as they do today, and that can change once `050-constitution`'s cut lands.

## Out of Scope

- **Which specs are worked next.** The rule packs `068`, `069` and `070` stayed in `draft` from 2026-10-19 while pipeline machinery was worked; ordering the work is the operator's decision, and this spec removes overhead rather than choosing priorities.
- **Reopens caused by plans going stale.** The two sweeps of 2026-10-16 reopened 34 `done` specs. Each of the first sweep's 18 reopens was a plan edit correcting a stale claim (the `amend(NNN): the plan …` commits of that day, ending at `8191e695`). The pure path re-points in the same sweep (`4afe66c6`) were already mechanical and reopened nothing, so classifying a dead-link repair as mechanical would have prevented none of the 18. The cost comes from §plan-phase's rule that a plan records the design as it stands, which every adopter receives. Changing it is a lifecycle decision that needs its own spec.
- **Tasks that exist to disposition other tasks' findings.** Spec 022 reopened with 22 tasks, seven of which dispositioned findings from other tasks. That loop belongs to §brownfield-inbox's Finding dispositions, not to the size of the governing documents.
- **The owner decisions queued on `022-deterministic-runtime`.** Its `Decide:` tasks wait on the operator, not on process overhead.

## Acceptance Criteria

- [x] AC1: `AGENTS.md` is at most 50,000 bytes on disk, the read-size threshold `check-artifact-size` applies to spec artifacts.
- [x] AC2: No `AGENTS.md` entry restates a rule whose canonical text is in `framework/constitution.md`; each such entry is one line pointing at its `§anchor` and stating nothing of its own, as `050-constitution`'s Promotion mechanism requires.
- [x] AC3: No `AGENTS.md` entry narrates how its lesson was learned; an owner decision keeps the decision and at most one clause naming who confirmed it and when.
- [x] AC4: No `AGENTS.md` entry warns about a failure that a test, hook, audit family or CI check in this repository already reports, except where the report comes from `runtime/src/` and does not name the fix, since adding it there is a runtime change; where a report elsewhere did not name the fix, its message, or a comment at the enforcement site, now does.
- [x] AC5: Every `AGENTS.md` entry removed or rewritten has a row in this spec's `plan.md` naming its disposition — pointer, trimmed, moved to an enforcement site (with the destination), or deleted (with the reason) — so no rule leaves the guide without a recorded decision.
- [x] AC6: Every import in `CLAUDE.md` uses Claude Code's `@path` form, names a file that exists, and names only a file of at most 50,000 bytes, so the constitution is not imported while it exceeds that.
- [x] AC7: A non-interactive Claude Code run (`claude -p`) started at the repository root answers a question that only `AGENTS.md`'s text answers, and its tool-use output shows no file read.
- [x] AC8: `CLAUDE.md` states no size or count of another file and restates no `AGENTS.md` entry.
- [x] AC9: A check run by `.githooks/pre-commit` (against the staged file) and by CI (against the checkout) fails when `AGENTS.md` exceeds its ceiling, and its message names the file, its size, the ceiling and the fix. It is shown to fail by taking the file past the ceiling, and to pass once the file is back under.
- [x] AC10: The check in AC9 also covers `framework/constitution.md`, with a ceiling no higher than the constitution's size in the commit that adds the check, the same message, and the same proof that it fails.
- [x] AC11: The check reports a missing or unreadable governed file as a failure with its own message, distinct from an over-ceiling one, and never as a pass.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Does the constitution come under the same ceiling in this spec?** Resolved 2026-10-26 by the owner: it is capped here, not cut. The check covers `framework/constitution.md` with a ceiling no higher than its size in the commit that adds the check, and the cut to one read is routed to `050-constitution` through `/ductus:amend` as a scenario, which reopens that spec. Reaching one read means removing 69,262 bytes (58%), against the 22% of 050's last cut (`979d77a2`), and the sections that mix command mechanics with policy measured about 48 KB (eleven line ranges of `framework/constitution.md` at `5cb01fdb` — the Grounding rules, lifecycle operational rules, implementation requirements, scenario primitive notes, inbox rules and finding dispositions, text-first gotchas, audit-record schema, host integration, cross-document references, concurrent features and reporting rules — summed with `sed -n | wc -c`), so moving mechanics alone would not get there; the cut is planned against its own measurement in 050. Capping here stops the growth, changes no constitution content and so needs no release.
