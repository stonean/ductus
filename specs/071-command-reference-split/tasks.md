---
next-task: 27
---

# 071 — Command reference split Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 25. Every reference section is reached by a step

- [x] Implement the behavior described in `scenarios/reference-sections-are-reached-by-a-step.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 26. The owner's decisions of 2026-11-19 on reference sections are recorded where this spec keeps decisions

- [x] A check is built so an unreached reference section fails the audit, owned by spec 026, since a requirement nothing checks rests on authors remembering it
- [x] A section is reached only from a numbered step under Instructions or from a section already reached, so a citation in a preamble, a note or Scope Boundaries reaches nothing; a section correctly cited from such a block carries the exemption marker with its reason
- [x] The dead sections are deleted, the five restating sections of the specify reference included, and a reference file stays as its opening paragraph when every section is gone

- **Done when**: the spec's Resolved Questions state each decision with its reason
