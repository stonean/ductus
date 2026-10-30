---
section: "Motivation"
---

# An-assessment-sees-the-loaded-rule-files

## Context

`specs/008-security-rules/data-model.md` §Verification phrasing item 6 lets a rule yield an overlapping defect to an owner on another surface only where the owner's file is also loaded — `security-frontend.md` `FE-STORAGE-002` and `FE-CSRF-002` name `security-backend.md` `BE-AUTHN-014` as the owner of a session cookie's attributes "where `security-backend.md` is also loaded" — so a frontend-only project keeps the frontend rules and a full-stack one reports the defect once. An `assessSpecQuality` request carries `spec-path`, `spec-content` and the one `rule` (`runtime/src/interpreter/payload.rs`, `build_assess_spec_quality_request`; `specs/022-deterministic-runtime/data-model.md` §`assessSpecQuality`), so on the `ductus exec` path the host cannot tell whether the condition holds, and the same rule reads differently there than on the markdown path, where the host knows which files it loaded. Recorded by `008-security-rules` task 30, 2026-10-29.

## Behavior

- Every `assessSpecQuality` request carries the basenames of the rule files the walk loaded, in the walk's `rule-files` order, under one named field documented in `specs/022-deterministic-runtime/data-model.md` §`assessSpecQuality`.
- `framework/commands/analyze.md` steps 11 and 12 say that a Verification's "where `{file}` is also loaded" yield is applied from that list, and treated as false when a request carries none, and the markdown-only path applies it from the rule files the host loaded, so both paths report a load-conditioned case under the same ID.

## Edge Cases

- **A condition naming a file the run did not load, or no rule file at all.** The condition is false, so the yielding rule keeps the case; a misspelt file name can never silence a rule.
- **A rule file the walk lists but could not load.** A file that is unreadable, or that the analyze rule-integrity check treats as unloadable, is not in the list, so the list names exactly the files whose rules the walk assessed, and no rule yields to an owner nobody assessed.
- **A request that carries no list.** A host answering a request from a runtime that predates this field treats every load condition as false, as 008's data-model item 6 states, so the yielding rule keeps the case.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
