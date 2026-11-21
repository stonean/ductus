---
status: draft
next-task: 19
---

# 020 — `/ductus:review` Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 18. AC17: a review loads a rule file only when its header names review

- [ ] Update `plan.md`'s design record for AC17 first: the header test (the paragraph after the file's H1 names `/{project}:review`), where it runs (`discover-rule-files`' review selection, beside `[[review.disabled-rule-files]]`; `review_pass_for_rule_file`), and the notice naming each file left to analysis. At `d5fc51ec` seven shipped files name `/{project}:analyze` alone: `api-backend.md`, `concurrency-backend.md`, `observability-backend.md`, `performance-backend.md`, `reliability-backend.md`, `accessibility-frontend.md` and `performance-frontend.md`; `security-*.md`, `quality-cross.md` and `configuration-cross.md` keep their passes. The decision is the scenario `every-loaded-rule-file-has-a-pass`'s Resolved Question, marked Superseded in part (2026-11-21).
- [ ] Implement with a fixture rule file of each kind, proven to fail with the header test removed.
- [ ] Update `framework/reference/review.md` §Load rules and step 2 of `framework/commands/review.md`; read 022's data model §`performReview` and 024's loader for statements AC17 makes false, and add each spec it changes to `cross-spec-impact:`.

- **Done when**: A review of a project loading an analyze-only rule file leaves it out with the notice, the fixture proves both kinds, and the reference and command say so.
