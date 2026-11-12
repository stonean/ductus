---
section: "Acceptance Criteria"
---

# An-assessment-sees-the-rule-it-asks-about

## Context

An `assessSpecQuality` request carries a rule's ID, Verification and tier beside the spec and the loaded rule files, and not the rule's Statement (022's `data-model.md` §`assessSpecQuality`; `runtime/src/interpreter/payload.rs`, `build_assess_spec_quality_request`), and the Verification it carries ends at the paragraph's first blank line, keeping only a pipe table directly after it, as spec 080 had it keep BE-API-001's (`runtime/src/primitives/rule_sections.rs`, `OpenSection::read`), though `RuleSection` already parses each Statement. Counted 2026-11-02 over the 304 `**Verification:**` lines of `framework/rules/*.md`, 64 point at text only the Statement holds: the lines matching `the Statement|either carve-out`, `all (two|three|four|five|six|seven|eight) (requirements|properties|attributes|conditions|controls|items|checks)`, `the (two|…|eight) carve-outs` or `the carve-outs?`, a lower bound, since a pointer worded another way is not matched. `BE-API-009` commits to "the Statement's allowlist", and `SEC-AUTHN-002` to "all four requirements", "each requirement" since `d64d8ab6`. One more, `BE-API-001`, commits to "the headers below", a table set after its Verification's first blank line, the only rule with material between that line and its **Source:**, which reaches the request since spec 080. A markdown-only host reads the rule file and resolves the Statement's pointers; an exec-path host cannot, so the two paths can judge one spec differently, and a project rule with a paragraph or list after that line still loses it. By shape, 10 such rules load in a cross-only project, 26 in a frontend-only one, 49 in a backend-only one and 65 in a full-stack one. Routed from [008 — Security Rules](../../008-security-rules/spec.md) task 39 by the owner, 2026-11-02.

## Behavior

- Every `assessSpecQuality` request carries the rule's Statement beside its ID, Verification and tier.
- The Verification a request carries runs from `**Verification:**` to the rule's next `**Field:**` paragraph or heading, the blank lines and every block between them included, so a table or paragraph after a blank line, `BE-API-001`'s header table among them, reaches the host.
- 022's `data-model.md` §`assessSpecQuality` shows the field and the bound, `framework/commands/analyze.md` steps 11 and 12 say the host reads a Verification beside its Statement, and the golden that holds an `assessSpecQuality` request is regenerated.
- A test fails when the Statement, or the material after a Verification's first blank line, is dropped from the request.

## Edge Cases

- A table or list after a Verification's first blank line arrives with its line breaks, so its rows stay readable, while a wrapped paragraph is still joined with single spaces.
- A Statement that wraps across several `>` lines arrives whole, as `RuleSection` already joins it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Does an exec-path assessment see the rule's Statement?** Yes: the request carries it, and the Verification is read to the rule's next field or heading, rather than every Verification being made to stand alone or the difference recorded. This is the call recorded on [008 — Security Rules](../../008-security-rules/spec.md) task 39 (owner, 2026-11-02, option a), which its `cross-spec-impact:` entry for this spec routes here.
