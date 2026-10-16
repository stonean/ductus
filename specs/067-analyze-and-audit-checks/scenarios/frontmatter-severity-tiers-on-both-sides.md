---
section: "Frontmatter, dependency, anchor and rule checks"
---

# Frontmatter-severity-tiers-on-both-sides

## Context

`validate-frontmatter` constructs fifteen findings. At `ductus-v0.50.0` thirteen carry `severity: "blocking"` and two carry `"hard-fail"` — the pair spec 057 task 8 added for a record artifact that exists but holds no readable record. The canonical assignment is the constitution's [§text-first-artifacts](../../../framework/constitution.md#text-first-artifacts) Validation Severity subsection, and the primitive disagrees with it in **eight** places, all of them Hard fail there and `blocking` here: frontmatter not valid YAML, frontmatter not a mapping, `status` out of set / non-string / missing, and a `dependencies` entry non-string / `dependencies` not a list / missing.

Five of the remaining `blocking` findings are correct: the `folds-into` shape pair, the `cross-spec-impact` shape pair, and a residual `review:`/`analyze:` block, which the constitution classifies Blocking *explicitly* and calls out as not a hard fail. The two `hard-fail` findings are correct. So the defect is not that the primitive lacks a vocabulary — `hard-fail` already exists and is already emitted — but that eight sites were never moved onto it. The code already knows: the comment introducing the `status`/`dependencies` checks states that absence is hard-fail per the constitution, directly above three pushes that emit `blocking`.

**The consumer is the other half of the same defect.** `framework/commands/analyze.md` step 2 states that `validate-frontmatter` emits each finding with `severity: blocking` and that the host therefore renders frontmatter findings in the report's hard-fail tier. The first clause stopped being true when 057 added the two `hard-fail` sites. The second is a rendering rule, and it mis-tiers in the *opposite* direction from the primitive: it flattens the residual-record-block finding up into Hard fail, when the constitution says Blocking precisely because the spec file itself parses and the defect is an inconsistent artifact set rather than a malformed spec. A fix applied only to the primitive would be erased at render, because the host is instructed to ignore what the finding says its severity is.

This is adjacent to [`spec-side-parser-hardening`](../../022-deterministic-runtime/scenarios/spec-side-parser-hardening.md), which made a missing `status`/`dependencies` produce a finding *at all*. That scenario settled whether the finding exists; this one settles which tier it carries, on both sides of the boundary.

## Behavior

**The primitive names the tier; the consumer renders the tier it was named.** Neither side holds a fixed tier for the frontmatter family.

- Each finding `validate-frontmatter` constructs carries the severity the constitution's Validation Severity subsection assigns to *that condition*. The eight Hard fail conditions above emit `hard-fail`; the five Blocking conditions continue to emit `blocking`; the two record-artifact conditions continue to emit `hard-fail`.
- `framework/commands/analyze.md` step 2 drops both halves of its stale sentence. In place of "emits each finding with `severity: blocking`" and the fixed hard-fail rendering, it says that the primitive assigns each finding the constitution's tier for its condition and that the host renders each finding in **the tier the finding names**, pointing at the canonical subsection rather than restating the assignment ([§drift-prevention](../../../framework/constitution.md#drift-prevention), canonical sources: a reference is a pointer, never a copy).
- 067's `data-model.md` registry entry for the primitive gains the same pointer, so the result shape's `severity` field is documented as a tier drawn from the constitution rather than as an opaque string whose only instance in the example is `blocking`.

The three tests whose names and assertions pin the old contract (`missing_status_is_blocking`, `missing_dependencies_is_blocking`, and the both-missing case asserting every finding is `blocking`) are **rewritten to pin the new one** — renamed and re-asserted against `hard-fail` — rather than deleted or loosened. The four tests asserting `blocking` for the `folds-into`, `cross-spec-impact` and residual-block findings, and the one asserting `hard-fail` for an unreadable record artifact, are already correct and stay.

## Edge Cases

- **No code branches on the severity string.** The only other `hard_fail` in `runtime/src` is `AnalyzeBlock`'s count field, which is unrelated. The change is therefore observable only in the rendered report and in tests that assert the literal — which is exactly why it drifted unnoticed and why the consumer half must move in the same change.
- **`analyze.md`'s Severity tiers section is not the defect.** It already lists the four tiers correctly; only step 2's flattening rule is wrong. Editing the tier list would be a second, unrelated change.
- **The markdown-only path reads the same prose.** Step 2's sentence is what the host follows when no runtime is registered, so correcting it fixes both paths at once — the two-paths guarantee holds only because the corrected rule is *per finding* and needs no knowledge of the emitter.
- **An eight-site tier raise makes previously-blocking findings hard-fail.** Both tiers already prevent pipeline advancement, so no spec that passed before now fails; what changes is which section of the report the finding lands in and how the run's tier counts split. A run over a corpus with a malformed spec will show a `hard-fail` count where it previously showed `blocking-findings`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
