---
section: "Follow-on scenarios"
---

# Review-observations-write-through

## Context

`/{project}:review`'s five passes bucket every finding into MUST, SHOULD, low-confidence or waived, and `write-review` renders exactly that set. A reviewer who notices something real that maps to **no loaded rule** has nowhere structured to put it. The command is explicit that inventing a rule is not the answer — *"Do not flag patterns that are not in the loaded rules"* — so the observation lands in the free-text Summary, which `write-review` regenerates wholesale on the next run. It is also per-spec, so a cross-cutting observation filed there is both erased and misfiled.

§brownfield-inbox's automatic issue capture is the intended route, and it works when it is used. But it depends on the agent *remembering* to call `append-inbox` at the moment of noticing, and nothing detects an uncaptured observation — the silent-degradation shape `AGENTS.md`'s second Design Principle forbids, in the command that enforces `QUAL-CLAIM-001` on everyone else.

Observed 2026-08-16. Reviewing 017 surfaced the done-spec review-staleness gap; it was recorded only in that spec's review Summary and a commit message. It survived to become 022 task 88 because the operator asked where it had been written down — the failure mode is that nobody asks.

Requiring spec: [017 — Derive, Don't Ask](../../017-derive-dont-ask/spec.md), whose subject is this exact principle. This scenario carries the runtime half per that spec's implementation-ownership split.

## Behavior

**`write-review` accepts an `observations` array.** Each entry is a finding that survived the passes but maps to no loaded rule — a defect, gap, or risk the reviewer judged real. Entries carry their own text and, optionally, a path; they are **not** rule findings and never enter the MUST / SHOULD / low-confidence counts, so `blocking` and the exit code are untouched.

**Recording is capture.** For each observation, `write-review` appends a bullet to `{specs-root}/inbox.md` as a side effect of the same call, dedup-guarded on a stable prefix so re-running a review over an unchanged repo appends nothing. This is the property the whole scenario exists for: the report and the inbox cannot diverge, because writing one *is* writing the other. No separate `append-inbox` call to forget.

**The report renders them in their own section.** An `## Observations` section, distinct from the four finding buckets and from Captured issues — the latter mirrors what the inbox *already* held over the review window, while this is what this run added. Empty renders `*None.*`, matching every other section.

**Suppression is impossible by construction.** There is no path that records an observation in the report without also recording it in the inbox. An observation supplied to a run whose scope is empty still captures: the reviewer's judgment is the input, not the diff.

## Edge Cases

- No observations: the section renders `*None.*`, the inbox is untouched, and the frontmatter is byte-identical to today's.
- The same observation on a re-run: the dedup prefix suppresses the append; the report still renders it, because the report describes this run.
- An observation whose text later becomes a rule finding: nothing special — the rule finding counts, the observation is the reviewer's to drop on the next run.
- Markdown-only path: the host writes both the section and the inbox bullet, in that order, with the same dedup rule — one contract, two paths.
- An observation containing a newline is rejected, as `append-inbox` already rejects one: structure injection into `inbox.md` is the reason that guard exists.
- The inbox append failing (unwritable file) fails the call rather than rendering the report without it — a report claiming an observation was captured when it was not is the defect this scenario removes, reintroduced one level down.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
