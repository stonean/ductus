---
section: "Follow-on scenarios"
---

# A-review-may-precede-its-commit

## Context

§implement-phase carried a bullet promoted from this repository's `AGENTS.md` on 2026-08-17 (survey W7): *Record a review against a commit that already contains what it reviewed* — two commits, always: the work, then the review against that new HEAD, then the review itself. Its reason was that the review records the HEAD it ran against, so the natural order (edit, review, commit everything together) recorded a pre-edit HEAD and every durable contract in that commit then read as changed since the review.

That reason no longer holds. 066's `the-committed-tree-horizon` moved review staleness from commits to content: the record's `reviewed-digest` is a digest of the spec's durable contracts as the review read them, compared against the same files now, and §text-first-artifacts' record table calls `reviewed-against` "Provenance, never the staleness basis". The analyze record works the same way over its own subjects. The bullet contradicted the document it lived in.

It also cost something. A process that only ever runs one order leaves the other untested. `compute-review-scope` answered from commits, so a review run before its work was committed was scoped away from every file the work touched. This repository reviewed only after committing, on a clean tree where `HEAD` and the working tree agree, so nothing exposed the defect until an adopter reported it on 0.58.0 (066's `review-scope-includes-uncommitted-work`, fixed in 0.59.4).

What the bullet got right is already stated elsewhere: §grounding's Rules list requires that a check reading git history be run after committing, because it must be asked about committed state.

## Behavior

§implement-phase drops the bullet. A review may run before or after its work is committed. Its scope is the working tree, its staleness is judged by a digest of the spec artifacts it read, and `reviewed-against` records provenance only. The natural order (edit, review, analyze, then commit) is supported, and so is committing first.

The surviving obligation stays where it is, stated once: §grounding's Rules list, on picking checks by blast radius, already says a check that reads git history runs after committing. Nothing is added to restate it.

Two other sentences assumed the commit-first order, and they are corrected with it. §grounding's rule on verifying what a commit contains named a review and an analysis among the records written against a commit; it now names claims that a commit holds a change, such as a completion report or a release, and says a review or analysis record may precede its commit. §implement-phase's bullet on the review window said to measure it only after the reopen's first commit, because before it there was no transition to derive a base from. That stopped being true in 0.59.4, which puts the base at `HEAD` while the move into `in-progress` is uncommitted, and the bullet now says so.

This repository's `AGENTS.md` mirror of the retired bullet is rewritten. It drops the commit-first instruction and keeps its own corollary, running the framework self-audit after committing and before tagging, as a pointer to that §grounding rule. `plan.md`'s §Classification row for W7 stays as the record of the 2026-08-17 pass, with a note that this scenario retired the rule.

## Edge Cases

- **Committing first is still allowed.** The rule is retired, not inverted. A project that commits before reviewing loses nothing.
- **A record with no `reviewed-digest`** cannot be judged stale by commit at the completion gate either; it reads as undeterminable and clears on its next review. The order of commits does not change that.
- **Analyze still follows review.** `review.md` is one of the analyze record's subjects, so an analysis recorded before the review it depends on is stale whatever has been committed. That ordering is the pipeline's, not a commit rule.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
