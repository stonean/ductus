---
status: draft
dependencies: [020-code-review]
next-criterion: 10
---

# 078 — Review fix rounds are re-read within a bound

`/{project}:review` gains a fix-round loop: the changes made to address a review's findings are re-read by an independent agent before the record is written, and the loop's scope, its check of prescribed edits, and its length are bounded by what the pipeline derives, not by the fixing agent's diligence. It extends [020's review](../020-code-review/spec.md).

## Motivation

`/{project}:review` had no fix-round loop. The passes produced findings, each was fixed, deferred or waived, and `write-review` recorded the run; a fix made during the run was checked by nobody but the agent that made it. This repository ran the loop by hand under a rule in its own `AGENTS.md`: an independent agent re-read each fix round that changed code or a durable contract, and the loop stopped at a round with no MUST or SHOULD. Adopters had none of it.

Spec 076's review showed both sides. The second round's re-read found a MUST the earlier rounds had recorded as fixed. The loop also ran five fix rounds and four re-reads in one session, about 933k subagent tokens and 34 minutes of agent time, and most of that was avoidable:

- two rounds existed because the fixer's sweep of a changed rule missed sentences, while a loaded rule already asked for a sweep by meaning;
- one decision went to the owner because the fixer widened a re-read's brief past the round it re-read;
- one re-read verified a round that only applied edits the previous re-read had prescribed.

No instruction the agent had loaded prevented any of them, and §design-principles rules out a bound that depends on diligence. This spec bounds the loop with inputs the pipeline derives.

## Behavior

### The loop

- Once a review's findings and observations are dispositioned and before `write-review` runs, each **fix round** — the changes made to address findings since the previous re-read, or since the passes for the first — that changed code or a durable contract (a scenario or a data model) is re-read by an independent agent.
- A round that changed no code and no durable contract is not re-read, and the record's Summary names it.
- The re-read's brief states the severity bar. **MUST**: the round's code does what the spec or data model forbids, fails a behavior they require, or breaks one that held. **SHOULD**: a sentence the round made untrue or left contradicting another it edited, or a test it added that would pass with its behavior removed. **NOTE**: a contrived input, a style point, or anything outside the round.
- The loop stops at a re-read with no MUST and no SHOULD. Each NOTE takes a disposition as an observation does.

### The scope is derived

- A re-read's scope is the round's changed paths, derived from the repository. Its brief carries that list and the severity bar, and no scope the fixer wrote, so a brief cannot widen past its round.
- A finding outside the round's paths is a NOTE: it is dispositioned as an observation and never changes the round.

### A prescribed edit is checked by diff

- A re-read may prescribe an edit: the file, the text it replaces, and the replacement.
- A round that only applies prescribed edits is verified by comparing its diff with the prescriptions — every hunk prescribed, every prescription applied — instead of by another re-read. A round with any change the re-read did not prescribe is re-read.

### The budget

- After two re-reads that each found a MUST or SHOULD, the host stops and reports to the operator — the open findings, the rounds run, and the re-reads' cost — instead of starting another round. The operator decides whether the loop continues, or each open finding is deferred or waived.
- The count belongs to one `/{project}:review` run and starts at zero on each.

## Edge Cases

- A run whose findings were all deferred or waived has no fix round and no re-read.
- A re-read that does not finish — cut off, or its agent stopped — is not a re-read. It is reported as incomplete and never counts as clean.
- A round that applies prescribed edits and also changes anything else is re-read whole.

## Acceptance Criteria

- [ ] AC1: `framework/commands/review.md` runs a fix-round loop between dispositioning and `write-review`: each round that changed code or a durable contract is re-read by an independent agent whose brief states the severity bar, and the loop stops at a re-read with no MUST or SHOULD.
- [ ] AC2: A round that changed no code and no durable contract is not re-read, and the record's Summary names it.
- [ ] AC3: A re-read's brief carries the round's changed paths, derived from the repository, and the severity bar, and no scope the fixing agent wrote.
- [ ] AC4: A finding a re-read makes outside the round's paths is recorded as an observation with a disposition and changes nothing in the round.
- [ ] AC5: A round that only applies edits a re-read prescribed is verified by comparing its diff with the prescriptions, and a round with any unprescribed change is re-read.
- [ ] AC6: After two re-reads that each found a MUST or SHOULD, the host starts no further round without the operator's decision, and reports the open findings and the rounds run.
- [ ] AC7: A re-read that does not finish is reported as incomplete and never counted as clean.
- [ ] AC8: Every agent in the registry runs the loop, or the review procedure states what replaces the independent re-read on a host that cannot start one.
- [ ] AC9: This repository's `AGENTS.md` re-read rule points at the framework's statement instead of restating it.

## Applicable Rules

- `QUAL-CLAIM-001` — a re-read that did not finish must not read as one that found nothing

## Open Questions

- What replaces the independent re-read on a host that cannot start an independent agent: a fresh session, the operator, or no loop with the record saying so?
- Where does the derived scope come from: `compute-review-scope` with the round's base as `--since`, a new primitive, or the host's diff?
- What marks a round's base so its changed paths can be derived — a commit, the working tree at the previous re-read, or a recorded sha?
- How is a prescribed edit represented so the diff check is mechanical: a fixed format in the re-read's report the host parses, or a patch file?
- Does the record carry the loop's outcome (rounds, re-reads, a budget stop) as frontmatter fields, or only in the Summary?
- Is the loop on by default, or opt-in, given that it spends agent tokens the adopter pays for?
- Does the loop also cover `/{project}:analyze`'s fixes and `/{project}:implement`'s disposition fixes, or only `/{project}:review`'s?
