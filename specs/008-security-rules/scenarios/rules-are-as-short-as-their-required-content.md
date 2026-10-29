---
section: "Rule Format"
---

# Rules-are-as-short-as-their-required-content

## Context

On 2026-10-28 the owner applied 050's `artifacts-are-as-short-as-their-required-content` to the shipped rule files, which that scenario leaves unswept. They held 297 rules in 509,421 bytes (`wc -c framework/rules/*.md` at `4e82686d`); by each paragraph's leading field marker, Rationale took 182,204, Verification 127,501, Statement 114,363, Source 53,651, and headers and headings 31,724.

## Behavior

- **A rule is an ID, a Statement and a Verification, with an optional Source and DEPRECATED label.** It carries no Rationale: the Statement stands without a stated reason (owner, 2026-10-28). The data model drops the field from its template, field table and integrity definition. A `**Rationale:**` paragraph in a project-authored or pinned rule file is ignored, never malformed.
- **A boundary moves to the Verification.** Where a Rationale named the sibling rule that owns an overlapping defect, the Verification's not-flagged list names it, rewritten to the clause a reviewer acts on. A citation that gave only background goes with the Rationale (owner, 2026-10-28).
- **A case only the Rationale stated is added to the Statement.** Where a Rationale stated an obligation, a flagged case or an exemption its Statement did not, the Statement gains it as a clause of the rule's own tier, or a MAY for an exemption, so the Verification checks only what the Statement says (owner, 2026-10-28). The cut found them in `BE-LRO-001`, `BE-RETRY-003`, `BE-BULK-002`, `BE-CACHE-002`, `BE-QUERY-004`, `BE-COORD-002`, `FE-LOAD-008`, `FE-ARIA-004` and `FE-PII-004`, and the second pass's review found more in `BE-API-006`, `BE-DEPS-002`, `BE-DEPS-006`, `BE-INPUT-003`, `FE-DEPS-004`, `BE-POOL-002`, `BE-TXN-004`, `BE-DRAIN-001`, `BE-RETRY-005` and `CFG-ENV-006`.
- **Every field is cut to its required content**, per constitution §text-first-artifacts (owner, 2026-10-28). Source keeps its citations and loses how the rule came about. A file header keeps its scope, ID prefix and categories, and points at its canonical sources for the severity mapping, ID permanence, and how a project selects or excludes the file. Statements and Verifications lose restatement, examples the rule does not need, and hedges. Every obligation, threshold, trigger, compliant case and boundary keeps its meaning.
- **No claim about an external standard is reworded from recall.** It is kept as written, deleted, or rewritten against the fetched source (data model, Source field).
- **The readers stop requiring Rationale:** `framework/reference/analyze.md`'s rule-file integrity check, `framework/bootstrap/ductus-procedure.md`'s adoption audit, and constitution §rules' summary.
- **Every artifact describing the old format is corrected through its own back-edge** (§drift-prevention, decision resolution). They are found by grepping `Rationale` across the live-artifact set, and each cut rule's ID across its owning spec and the specs it cites, then reading each hit by meaning: a sentence describing what a rule's Rationale, Source or header said need carry no `Rationale` token. The criteria among them: 008 AC3; AC1 of 034, 037, 038 and 039; 036 AC1, AC2 and AC8–AC11; 016's Non-goal naming 008's format; AC3 of 068, 069 and 070; 070 AC6; and 069's account of `FE-SEMHTML-002`.

## Edge Cases

- **Meaning is verified, not assumed.** Each file is compared before and after, rule by rule, by an agent that did not write the cut, for every obligation, threshold, trigger, compliant case and boundary; a loss is restored before the file lands.
- **A rule whose meaning cannot survive a cut keeps the words it needs.** A cut that drops required content is a defect, not a saving.
- **The tier discipline holds.** A cut Statement keeps at least one obligation keyword, all of one tier (`no_shipped_statement_mixes_tiers`).
- **Adopters receive the cut on their next `/ductus` run**, since rule files ship with the `update` strategy; a pinned copy keeps its Rationale and stays loadable.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Is a documented push channel a compliant alternative to `BE-LRO-001`'s status monitor?** Yes (owner, 2026-10-28): the Rationale said a push channel "can serve the same purpose" and the original flag never fired on one, so the Statement carries it as a MAY.
- **Does the first pass's residue get a second pass?** Yes (owner, 2026-10-28). The first pass's reviews listed three leftovers: reason clauses still in Statements, `**Source:**` citations that back no remaining sentence, and Verification sentences that restate their Statement. The second pass cuts them in every file, keeping a reason clause that defines a term (`FE-LOAD-007`'s "short") and a citation that backs a remaining claim, and each file is re-reviewed.
