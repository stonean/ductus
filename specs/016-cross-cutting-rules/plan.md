---
title: "016-cross-cutting-rules — plan"
---

# 016 — Cross-Cutting Rules Plan

Implements [016 — Cross-Cutting Rules](spec.md).

## Overview

016 promotes rules to a first-class artifact tier without changing rule semantics, validate enforcement logic, or the rule-file format. The work is concentrated in five files: the constitution gains a §rules section and a fourth decision-tree route; the spec template gains an optional "Applicable Rules" prompt; `analyze.md` and `groom.md` are reframed from security-specific to general; and 008's spec gets a top-of-file signpost. After source edits, the generated `.claude/commands/ductus/*.md` mirrors are regenerated.

The governing constraint is **canonical-source discipline**: the constitution carries the *concept* of rules (definition, lifecycle, promotion checklist), and 008's `data-model.md` remains the *canonical schema*. The constitution back-links rather than restating, except for a short summary that lets a constitution reader understand the artifact without context-switching.

## Technical Decisions

### Section placement: §rules sits between §scenarios and §brownfield-inbox

The constitution today walks the artifact tiers in this order: §spec-phase / §spec-requirements / §spec-lifecycle, then §bug-handling and §scenarios, then §brownfield-inbox / §brownfield-process. §rules belongs alongside §scenarios as a sibling artifact-tier definition, immediately after §scenario-promotion and before §brownfield-inbox. This co-locates all three artifact tiers (specs, scenarios, rules) and reads naturally in the flow.

The HTML anchor is `<!-- §rules -->`, matching the existing convention. Naming it in §drift-prevention's canonical-sources table is part of this work; the table's row for this document now lists it as the rules tier.

### Bug decision tree extension: insert as new step 1, push existing 1–3 to 2–4

Before 016, §bug-handling's decision tree started with "1. No spec exists for the behavior." For cross-cutting concerns (perf budget, observability commitment, etc.), a missing spec isn't the right diagnosis — the right diagnosis is "no rule covers this." If §rules sits after §scenarios in the constitution, the decision tree should evaluate the rule question first (broadest-scope check) before falling back to spec-level questions.

Decision-tree order after this change:

1. **No rule covers this cross-cutting concern** — promote to a rule (new or amended), then fix the code.
2. **No spec exists for the behavior** — write the spec first, then fix the code.
3. **Spec exists but is ambiguous or incomplete** — fix the spec, then fix the implementation.
4. **Spec is clear but implementation is wrong** — add a scenario capturing the correct behavior, then fix the code.

The intro paragraph is rewritten to mention the three-tier framing (rule / spec / scenario) before listing the tree.

### §rules section content (concept summary + back-link to 008)

The §rules section carries:

1. **Definition** — a rule is an enforceable, citable requirement that applies across multiple features. Rule files ship under `specs/rules/{rule-set}.md`. Specs cite rules by ID.
2. **Conceptual format summary** — every rule has an ID (e.g., `BE-AUTHN-001`), a Statement (RFC 2119 keywords of one tier: MUST / MUST NOT, SHOULD / SHOULD NOT, or the tierless MAY), and a Verification step. The full schema is canonically declared in [`specs/008-security-rules/data-model.md`](../../specs/008-security-rules/data-model.md).
3. **When to write a rule** — the four-indicator promotion checklist (cross-cutting / citable / governance-recognized category / generalizable wording), a rule being justified when "all four" hold.
4. **When NOT to write a rule** — situational → scenario; feature-wide → acceptance criterion.
5. **Lifecycle** — IDs are permanent; rules are deprecated with a removal target version, not deleted. Cross-reference 008's data-model for the full ID-stability invariants.
6. **Relationship to specs and scenarios** — selected by scope: a situational concern is a scenario, a feature-wide one an acceptance criterion, a covered one a citation, the selection the three-tier table in 016's spec body states.

The constitution names 008's data model by its path in a code span, `specs/008-security-rules/data-model.md`, rather than by a markdown link, so an adopter whose tree lacks 008's spec directory carries no broken link.

### Validate generalization: the section is "Rules", and discovery is 024's

The section once named "Security rules" is "Rules", now `## Rules (blocking and advisory)` in `framework/reference/analyze.md` since 071, and its loading prose covers every rule file rather than the two security files. Which files load is 024's rule loader, `discover-rule-files`, which selects by filename suffix, so no list of rule files is kept by hand. Validate's per-rule integrity check, reference check, severity behavior, and brownfield-audit hook are unchanged.

The references to the rule-file schema location (currently "canonically declared in `specs/008-security-rules/data-model.md`") remain — that's still where the schema lives.

### Spec template: "Applicable Rules" goes between Acceptance Criteria and Open Questions

The current `framework/templates/spec/spec.md` has the body order: `{Section}` placeholder, `## Acceptance Criteria`, `## Open Questions`. The new `## Applicable Rules` section sits between `## Acceptance Criteria` and `## Open Questions` — close enough to the AC section to read together (rules constrain the same surface as ACs), but not interrupting the open-questions / clarify flow.

The section is comment-prompt only. The body is an HTML comment listing example IDs and explaining when to use it. Authors who skip the section delete the comment along with the section header; the markdownlint config allows but does not require the heading. Verified that `## Applicable Rules` headings would not collide with any existing required-heading check in `analyze.md` — the section is purely additive.

### Groom decision-tree update: add rule-promotion step before existing steps

`/{project}:groom`'s decision tree, `framework/reference/groom.md` §Groom decision tree since 071, walks four steps that mirror §bug-handling, rule promotion first: Step 1 asks whether the item is a cross-cutting concern with no covering rule, applies the four-indicator promotion checklist (§rules), and recommends amending the covering rule file, or, where none covers the domain, notes that a new rule file is its own spec. Groom still creates scenarios and appends tasks; rule promotion is *user-action*, not auto-applied (matching scenario promotion's user-decision discipline).

### 008 signpost: top-of-file note, body untouched

008's spec is `done`. Under the constitution's frozen-archaeology rule, in force when this plan was written, the body is not rewritten. (023's `living-specs` scenario later removed that rule; the signpost decision stands on its own merits.) The signpost is a quoted note inserted between the frontmatter and the H1, of the form:

> **Signpost:** 008 defines the *security instance* of the general rules tier later formalized in [016 — Cross-Cutting Rules](../016-cross-cutting-rules/spec.md). The rule-file format, ID conventions, and validate enforcement defined here remain authoritative for security rules and serve as the canonical reference for any future rule file. See [§rules](../../framework/constitution.md) in the constitution for the general framing.

This pattern (signpost note for evolved framing) is consistent with how 006 already handles renamed commands ("`/ductus:scenario` is now `/ductus:amend`," etc.) — a top-of-file note rather than body rewrites.

### Generated Claude commands: regenerate via scripts/gen-claude-commands.sh

`.claude/commands/ductus/*.md` is generated from `framework/commands/`, as `AGENTS.md` says. After editing `analyze.md` and `groom.md` sources, the generator must run. The generator substitutes `{project}` → `ductus` and `{cli-config-dir}` → `.claude`. No manual edits to the generated files.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | Add `<!-- §rules -->` section between §scenario-promotion and §brownfield-inbox; update §bug-handling intro and decision tree (4 steps); add row to canonical-sources table |
| `framework/commands/analyze.md` | Modify | Rename "Security rules" section heading to "Rules"; generalize its loading prose to every rule file |
| `framework/commands/groom.md` | Modify | Insert new Step 1 (rule-promotion check) into decision-tree walk; renumber existing 1–3 to 2–4 |
| `framework/templates/spec/spec.md` | Modify | Insert optional `## Applicable Rules` comment-prompt section between Acceptance Criteria and Open Questions |
| `specs/008-security-rules/spec.md` | Modify | Add top-of-file signpost noting 008 is the security instance of the general rules tier defined in 016; body untouched |
| `.claude/commands/ductus/analyze.md` | Generate | Regenerate from `framework/commands/analyze.md` via `scripts/gen-claude-commands.sh` |
| `.claude/commands/ductus/groom.md` | Generate | Regenerate from `framework/commands/groom.md` via `scripts/gen-claude-commands.sh` |
| `specs/016-cross-cutting-rules/plan.md` | Create | This file |
| `specs/016-cross-cutting-rules/tasks.md` | Create | Task breakdown |

No source code, no tests, no migrations. All artifacts are markdown.

## Trade-offs

- **§rules duplicates a small amount of 008's content.** The conceptual format summary in §rules names ID, Statement, Verification, and RFC 2119 — the same fields described more fully in 008's data-model. This duplication is bounded and intentional (the hybrid resolution to Q2). The constitution back-links for the full schema.
- **008's body remains security-framed.** Its motivation paragraph still reads "Security rules belong at the governance level because they are cross-cutting." The signpost makes the general framing discoverable without rewriting a `done` spec's body, but readers who land on 008 and don't notice the signpost may carry forward the security-only mental model. Acceptable — the constitution and 016 are now the canonical entry points.
- **"Applicable Rules" stays optional.** Authors can skip it; a citation must earn its place, since `/{project}:analyze` step 14 reports a cited rule whose trigger does not fire against the spec (scenario `applicable-rules-consistency-check`).
- **Adding §rules to canonical-sources is a small drift surface of its own.** The canonical-sources table grows by one row; future readers must remember to update it when the artifact location changes. No mitigation beyond the existing discipline of editing the table when canonical sources move.

## Open Questions Resolved

- **Naming** — keep "rule" throughout. No new vocabulary introduced.
- **Format definition home** — hybrid: constitution carries the concept summary; 008's data-model is the canonical schema and is back-linked.
- **"Applicable Rules" slot** — optional, comment-prompt section in the spec template.
- **Stronger validate consistency check** — deferred. Captured in `specs/inbox.md`; not in 016's scope.
- **Promotion threshold** — four-indicator checklist (cross-cutting, citable, governance-recognized category, generalizable wording), codified in the §rules section.
