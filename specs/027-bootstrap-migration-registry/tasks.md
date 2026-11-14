---
next-task: 16
---

# 027 — Bootstrap Migration Registry Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 14. Implement scenario: a-projects-own-constitution-is-not-relocated

- [ ] Implement the behavior described in `scenarios/a-projects-own-constitution-is-not-relocated.md`

Handoff (2026-10-29). After this task, re-review 027, since the scenario is a durable contract, then re-analyze: the record is at `c0952038`, last `unexamined` not-a-live-claim 4, root-absent 5, ships-to-adopter 1, and its four stored waivers still fire. Then verify all 30 ticked criteria against the tree before proposing done.

Implemented in `364c330b` (landed on `main` 2026-11-12): the box stays open for the done-when's real adopter-shaped bootstrap, in which a repository with its own root `constitution.md` keeps it and the run prints the declined line, and a legacy ductus copy still relocates.

- **Done when**: The scenario's open question is resolved, a repository with specs/ and its own constitution.md keeps it through a bootstrap run (verified with a real adopter-shaped bootstrap), and a legacy ductus copy still relocates.
