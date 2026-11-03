---
section: "Acceptance Criteria"
---

# Review-reports-a-defect-under-its-owner

## Context

Tasks 21 to 30 of [008 — Security Rules](../../008-security-rules/spec.md) gave each defect two rules overlap on one owner, named in the yielding rule's Verification by a **Not flagged:** clause, which `/{project}:analyze` applies, conditioned on the owner's file being loaded where the owner sits on another surface (008's `data-model.md` §Verification phrasing, items 5 and 6). `/{project}:review`'s security pass checks code against each loaded rule's Statement and applies no such clause (`framework/reference/review.md` §Security pass; `security-frontend.md`'s header), its de-duplication keys on rule ID, file and line range (AC13), and a waiver names one rule ID. So code that breaks two overlapping Statements is reported under both IDs, counts twice toward blocking and needs two waivers: a backend `Set-Cookie: session=…` with no `HttpOnly`, in a project loading both security files, is `BE-AUTHN-014`'s alone at analyze and `BE-AUTHN-014`'s and `FE-STORAGE-002`'s at review.

Counted at the end of the 2026-11-02 re-read round over the Verification paragraphs of `security-backend.md` and `security-frontend.md`, one per distinct rule ID named after **Not flagged:** other than the rule's own, a lower bound since some yields are written without that marker: 48 of their 136 rules carry such a clause, 37 of them naming 82 owners, 39 in the same file, 25 in the other security file, all but `BE-API-001`'s two scope pointers to `FE-CSP-007` and `FE-CSP-009` under a load condition, and 18 in files no pass applies today (`api-backend.md` 9, `configuration-cross.md` 8, `observability-backend.md` 1; scenario [every-loaded-rule-file-has-a-pass](every-loaded-rule-file-has-a-pass.md)). Routed from 008 task 36 by the owner, 2026-11-02.

## Behavior

- A finding a pass makes against a loaded rule is reported under the owner a loaded rule's Verification names for the case the code shows, as `/{project}:analyze` names it, and not under the rule that yields it.
- A load condition in that clause ("where `security-backend.md` is also loaded") is judged against the rule files this review loaded, so a frontend-only review keeps the frontend rule's finding.
- A finding stays with the yielding rule when this review did not load the owner's file, a file the project excludes from review included, or no pass applies it, so routing never leaves a defect review reports today reported by no rule.
- One defect is then one finding, at the owner's tier, so a defect a MUST rule yields to a SHOULD owner becomes advisory, and the reverse; it counts once toward blocking, needs one waiver, and AC13's de-duplication applies to it.
- `framework/reference/review.md` and `security-frontend.md`'s header say review applies the routing, the generated command copies are regenerated, and the runtime's `performReview` extension carries what the pass needs to apply it.

## Edge Cases

- A **Not flagged:** case that only a spec can show, such as a spec that only changes a client's registered redirect URIs, routes nothing at review.
- Once scenario every-loaded-rule-file-has-a-pass gives `api-backend.md`, `configuration-cross.md` or `observability-backend.md` a pass, the yields to their rules route too, under the third Behavior item.
- A waiver recorded under the yielding rule's ID before this lands expires once the finding routes to its owner, since that rule no longer fires there (`framework/reference/review.md` §Per-run waiver processing); the finding counts under the owner's ID, where a waiver already recorded under that ID applies, and otherwise surfaces to be fixed, waived, or deferred.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
