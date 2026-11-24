---
status: draft
dependencies: [066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 084-reopen-for-owed-code-review-every-code-change]
cross-spec-impact: [066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 084-reopen-for-owed-code-review-every-code-change]
next-criterion: 5
---

# 086 — Review records hold only what their scope judged

A spec's `review.md` records the outcomes of its own review's scope, and a later run drops what that scope will not judge again.

## Motivation

- An adopter's `review.md` reached about 518 KB. Its frontmatter, copied without its body, was 140,562 bytes, of which `decisions:` was 129,546 (92%): 206 entries decided on 2026-11-23 and 2026-11-24, 150 of them a person's deferrals to about 20 other specs' `spec.md` or `tasks.md` and 56 waivers, 55 by the rule; 203 were anchored to a file, 154 files in all. Method: each copied line's UTF-8 length plus its newline, summed by top-level frontmatter key; entries counted as list items under `decisions:`. The body was not copied, so its findings and its 741 excluded paths were not examined.
- The owner's account (2026-11-24): `/{project}:review --all` ran with that spec as the session target and recorded every spec's outcomes in its record. `framework/commands/review.md` runs `--all` once per feature, each into its own record, but nothing enforced it: `write-review` refused an `unread` entry the scope did not hold and accepted a finding or observation on any file (`departure_waiver_entries`' doc: such a finding "stays counted"). The copy alone could not confirm the account, since code naming no spec sat in every open spec's scope under spec 081 and could have put those files there too.
- No re-run cleared them. `process-decisions` expired a decision whose file was deleted (`RunScope::why_unread`) and retained one whose file existed but which the run's scope did not hold, since absence proved nothing ([079](../079-review-and-analysis-work-converges/spec.md)). A decision on a file that never belonged to the spec was therefore kept forever.
- [084](../084-reopen-for-owed-code-review-every-code-change/spec.md)'s AC6 moves code no spec claims out of every open spec's scope into `{specs-root}/review.md`, which would make each decision already stored on such code in a feature's record permanent there, while the project record starts without them and asks again.

## Behavior

- `review.md`, like `tasks.md`, is not a durable artifact (owner, 2026-11-24; §implement-phase: nothing durable goes in a review record's body). A deferral's durable home is the task, scenario or criterion it wrote, so a stored decision only spares a later run from asking again, and dropping one costs at most that question.
- A review records only outcomes on files its own scope holds, so a collapsed `--all` run cannot write one spec's outcomes into another's record.
- A decision or waiver anchored to a file the spec does not claim is not carried past the spec's next unrestricted review. One on a file the spec claims but that is unchanged since its last review is carried, since since spec 081 a review's scope is only what changed, and dropping it would ask again on every incremental review.
- A deleted file's decision expires as it does today.

## Acceptance Criteria

- [ ] AC1: `write-review` refuses, writing nothing, a finding or an observation on a file the run's scope does not hold, naming the file, and a test proves the refusal and fails with it removed.
- [ ] AC2: After an unrestricted `/{project}:review`, the spec's `review.md` carries no decision or waiver anchored to a file the spec does not claim, and still carries one anchored to a file the spec claims that is unchanged since its last review; a fixture proves both directions.
- [ ] AC3: A decision on code no spec claims, stored in a feature's `review.md` before 084's scope change, is not left there permanently once that change lands, and a fixture proves it.
- [ ] AC4: A decision anchored to a deleted file still expires on an unrestricted run, and a test proves it.

## Applicable Rules

- `QUAL-CLAIM-001` — a decision dropped because its file does not belong to the spec is reported as dropped, never as a finding that stopped firing.
- `QUAL-TEST-001` — each fixture fails with its behavior removed.

## Open Questions

- What separates a file that does not belong to a spec from one unchanged since its last review? Candidates: the run's own `excluded` set (paths changed in the window only by commits another spec, or after 084 no spec, claims); the spec plan's Affected Files; or every commit in history that claims the spec. A file neither in the scope nor in `excluded`, unchanged in the window and never the spec's, is what a collapsed `--all` run leaves, and the first candidate does not catch it.
- Should `/{project}:review` carry `decisions:` and `waivers:` across runs at all, given the record is not durable? §implement-phase keeps them so a later run does not ask again; without them every run a finding fires on asks again.
- Should `/{project}:prune` reduce `review.md` as it reduces `tasks.md` (owner, 2026-11-24)? Prune's artifact table classes `review.md` as regenerated and never written, so this is cross-spec impact on spec 041.
- Does the criteria step, which reads the code a criterion's claim names wherever it lies, produce findings or observations on files outside the scope that the refusal must let through?
- Do decisions on code that 084's AC6 takes out of a feature's scope move to `{specs-root}/review.md`, or are they dropped, and does the project record's first review then ask again?
- Where does the list of a spec's files that a review with no recorded run reads live, and what keeps it current (owner, 2026-11-24)? A first review's scope adds every Affected Files entry (`compute_review_scope.rs`, the branch where `HEAD` holds no recorded run), and with an empty window, as for a spec never recorded `in-progress`, those entries are the whole scope; yet nothing keeps the section current after `/{project}:plan`: §plan-phase holds the template's `##` sections, Affected Files among them, to the design as it currently stands, while `framework/reference/plan.md` calls the section a planning aid of expected files and `framework/commands/implement.md` calls it not authoritative, and `/{project}:analyze` checks only that it names a path and that a named path that existed still resolves. Attribution also reads it, splitting a commit several specs claim by each one's plan (`attribute_window`). Candidates, each needing a check that compares the files of the commits attributed to the spec with the list: (A) a durable `files.md` at a fixed path in each spec directory, the owner's proposal, which adds an artifact type and its template, backfills the 78 of 80 spec directories here holding a `plan.md` and every adopter's through a migration, and touches `/{project}:plan`, `/{project}:implement`, `/{project}:review` and `compute-review-scope`, attribution, `/{project}:fold`, `/{project}:consolidate`, `/{project}:prune`'s table and `/{project}:analyze`'s path family, with about 150 of the constitution's 481 bytes of headroom (49,519 of 50,000 by `wc -c`); (B) Affected Files kept in `plan.md` and enforced, which changes the plan reference's and `/{project}:implement`'s wording and adds the check, with no artifact, backfill or constitution bytes; (C) no stored list, the review deriving it from history, which changes `compute-review-scope` alone while attribution still reads the plan's list. The agent recommends B: it gives A's guarantee without a new artifact, and A's one gain is keeping file ownership apart from the design decisions. Counts taken at `b6d5dd81`.

## Resolved Questions

- **One new spec for both halves**, the out-of-scope outcomes and the retained decisions, rather than an open question on 084, the inbox, or a scenario under [066](../066-audit-records-and-the-completion-gate/spec.md) or 079: the agent's recommendation, adopted by the operator, 2026-11-24.
