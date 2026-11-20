---
next-task: 7
---

# 069 — Accessibility rules (frontend) Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 6. The owner's rule decision of 2026-11-19 for `framework/rules/accessibility-frontend.md`

- [x] Re-read `FE-KBD-004`, `FE-A11YFORM-003`, `FE-A11YFORM-006`, `FE-LAYOUT-002` and `FE-MOTION-002` whole against the pre-tag audit's reading that each Verification flags a case its Statement does not cover (a fixed or sticky element that obscures nothing; a recipient's name or address field with autocomplete off; a URL-only filter update on change; a clipped fixed-height container whose text is revealable; a sub-333 ms cycle under the area threshold), and give each a verdict
- [x] Each rule whose reading holds gains the least Not-flagged clause for the harmless case, with any WCAG wording it relies on fetched from w3.org first; each whose reading does not hold is left unchanged and taken back to the owner with the sentence that shows it
- [x] Left as it is: `FE-A11YTEST-001` stays a MUST

- **Done when**: each item is stated in the rule file as decided, the spec records the requirement (scenario `verifications-flag-only-what-the-statement-covers`, and a Resolved Questions entry per decision with its reason) and corrects any criterion the change makes false, any external claim the new text makes is fetched and not recalled, bytes before and after are measured per rule and per file, `runtime/CHANGELOG.md` `[Unreleased]` gains an entry per changed rule, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change
