# 069 — Accessibility rules (frontend) Plan

Implements [069 — Accessibility rules (frontend)](spec.md).

## Overview

069 takes ownership of a rule file that already ships, so most of its criteria describe `framework/rules/accessibility-frontend.md` as it stands and are verified against the tree, not built. Its edits to the file are the two 2026-11-08 decisions in Resolved Questions. The first is AC14, `FE-ARIA-001`'s WCAG basis. The second is AC13: the header's conformance disclaimer, plus the seven uncited Level A and AA criteria, which go into four rules, three as clauses of existing rules and one new rule. AC13 is then checked over all 55 criteria. AC4's re-fetch of every Source follows, so the claims the edits add are checked too. A last walk verifies AC1–AC3, AC5–AC12 and the spec's own counts. The rule file is the only deliverable that changes. The `FE` surface and its ID format already exist, so `scripts/lint-rule-ids.sh` is unchanged, and AC12 rules out a data model.

## Technical Decisions

### `FE-ARIA-001` rests on §4.1.2 alone (AC14)

The Statement drops its MUST NOT on restating an element's implicit role (`<button role="button">`) and the styled-list MAY that qualifies it. The Verification drops its flag of a restated role and that flag's styled-list exception. The Source adds WCAG 2.2 §4.1.2 (Name, Role, Value) for the MUST, and drops *ARIA in HTML* §3.2, which supported only the removed clause. *ARIA in HTML* §1 and §3.1 stay for the allowed-role and override clauses. Severity posture already names `FE-ARIA-001` "through §4.1.2" and lists it under no exception (spec.md, *Severity posture*), so the spec needs no change for it. The rule's other clauses, quoted at `framework/rules/accessibility-frontend.md` in its `### FE-ARIA-001` section, are unchanged. The only other artifacts that name `FE-ARIA-001` are 008's ticked task 20 bullets, which concern its `<div>`-with-role overlap with `FE-SEMHTML-001`. That clause stays, so no other artifact changes.

### The header disclaims a conformance claim (AC13)

The header's first paragraph ends with one sentence. It says meeting these rules is not a WCAG conformance claim, since WCAG 2.2 §5.2.2 (Full pages) makes conformance a property of full web pages, which no design-time commitment establishes. The paragraph stays one paragraph.

### Each of the seven criteria lands in the category its guideline sits beside (AC13)

The Resolved Question says each "gains a rule or a clause of a rule its category holds", so no category is added and AC1's eleven stand. Each placement follows the rule the file already has for the criterion's nearest neighbour, by the WCAG copy fetched 2026-11-08:

| Criterion | Level | Lands in | Because |
| --- | --- | --- | --- |
| 1.3.3 Sensory Characteristics | A | a clause of `FE-CONTRAST-002` | that rule is §1.4.1 Use of Color, and 1.3.3's text lists color among the sensory characteristics, its Note sending color to Guideline 1.4 |
| 1.4.5 Images of Text | AA | a clause of `FE-A11YMEDIA-001` | text alternatives for images; its Customizable and Essential exceptions go into the clause as MAY, with logotypes noted as essential |
| 2.5.4 Motion Actuation | A | a clause of `FE-POINTER-002` | that rule holds §2.5.1 and §2.5.7, Guideline 2.5 *Input Modalities*, and gives an alternative to complex input, as 2.5.4 does; its two exceptions become MAY |
| 2.4.5 Multiple Ways, 3.2.3 Consistent Navigation, 3.2.4 Consistent Identification, 3.2.6 Consistent Help | AA, AA, AA, A | a new rule, `FE-SEMHTML-004` | each concerns a set of web pages, the page structure `SEMHTML` holds, beside `FE-SEMHTML-002`'s §2.4.1 and §2.4.6 |

Each is MUST, being Level A or AA (spec.md, *Severity posture*). Each Verification triggers on a spec or plan that introduces the surface concerned: instructions, images, motion input, or a set of pages that shares navigation, help or components. The scope table's `SEMHTML`, `CONTRAST`, `A11YMEDIA` and `POINTER` rows gain the concern, and Severity posture's count moves from 39 rules, 35 MUST and 4 SHOULD, to 40, 36 and 4.

### AC13 is a completeness claim over all 55 criteria

The Resolved Question counts 55 Level A and AA criteria in WCAG 2.2: 31 at A, 24 at AA, §4.1.1 being obsolete. Task 3 lists every one from the fetched WCAG copy and checks it by number against the rules' `§` citations. AC13 ticks only when each is cited, or is recorded with why a design-time commitment cannot address it. The list goes in the landing commit's message.

### The file stays one read

The file is 48,596 bytes, with 39 rules averaging 1,192 bytes (`wc -c`, and its `### FE-` sections, at `849a9a26`). The decisions above add one rule, three clauses and the disclaimer, and remove one clause, about 1,700 bytes net, which would put it near 50,300. That is past the 50,000-byte read size `[artifacts] read-size-bytes` defaults to, though under the 51,200 bytes OpenCode and Pi read at once. No check holds a rule file to either. The plan keeps it at or under 50,000. The overage is paid by tightening wording, the header's first paragraph first, which 036's and 038's review records waive as long, without dropping a claim. Each task's commit states the size it measured.

### Every Source is re-fetched after the edits (AC4)

Task 4 checks every claim against copies fetched during implementation, by 068's task 6 method (`a8f419f7`). The method is `curl` with `scripts/check-runtime-currency.sh`'s User-Agent, or WebFetch for verbatim quotes where `curl` is refused. Read-only agents check rule groups against the copies, and every quote behind an edit is confirmed present before the edit. A claim no copy confirms is rewritten or removed, never kept on recall. The landing commit names each document, its URL, the date and the method. The clarify session's WCAG 2.2 and *ARIA in HTML* copies are reused, and the Severity posture and regime claims in the spec are checked with the header's. AC5's titles and levels are checked against the WCAG copy in the same pass.

### The spec's own stale passages are corrected

Enforcement still says "(see Open Questions)" for which review pass applies the file. The question was deferred to 020's scenario `every-loaded-rule-file-has-a-pass` and is recorded under Resolved Questions, so the pointer moves there. Severity posture's counts follow the new rule, as above.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/rules/accessibility-frontend.md` | Modify | `FE-ARIA-001` on §4.1.2; the conformance disclaimer; the 1.3.3, 1.4.5 and 2.5.4 clauses and `FE-SEMHTML-004`; Source corrections from the re-fetch |
| `specs/069-accessibility-frontend-rules/spec.md` | Modify | The scope table's four rows, Severity posture's counts, Enforcement's pointer, and the ticked criteria |

A change under `framework/rules/` runs `runtime.yml`, whose corpus test reads every shipped Statement, and `framework-checks.yml`, whose self-audit resolves every `§` reference in a shipped file. So each task runs `scripts/audit/run-all.sh` before it is pushed.

## Trade-offs

- **Clauses before rules.** Three criteria join a rule whose subject they share, and four consistency criteria share one new rule, rather than seven new rules. That keeps the file near one read, at the cost of rules that each cite several criteria, as `FE-A11YFORM-006` and `FE-POINTER-002` already do.
- **2.5.4 in `POINTER`, not `MOTION`.** `MOTION` holds animation and flashing (§2.2.2, §2.3.x), while 2.5.4 is input by device motion, a sibling of §2.5.1 in Guideline 2.5. The `POINTER` row's concern therefore names input modality as well as pointers.
- **3.2.4 in `SEMHTML`, not `ARIA`.** Consistent identification could sit with accessible names (`FE-ARIA-002`), but it is a property of a set of pages, which it shares with 2.4.5, 3.2.3 and 3.2.6.
- **No data model** (AC12): one would govern ahead of the header.
- **Known limitation.** AC4 checks each claim against its document as fetched on one date, and no check watches rule Sources afterwards. AC13 ties a rule to each criterion; whether a rule is a complete reading of its criterion is the review's and AC4's to judge. Meeting the file is not conformance, as the header will say.

## Cross-spec impact

The spec body links 008, 024, 025, 033 and 050, and its Resolved Questions name 020's scenario `every-loaded-rule-file-has-a-pass`. None needs an edit. 008's task bullets concern a clause of `FE-ARIA-001` that stays. 070's spec describes `FE-A11YMEDIA-001` as requiring `alt` on every `<img>`, which stays true. The deferral to 020 is recorded there already. So `cross-spec-impact:` stays empty.
