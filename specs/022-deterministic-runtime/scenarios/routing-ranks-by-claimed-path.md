---
section: "Follow-on scenarios"
---

# Routing-ranks-by-claimed-path

## Context

`derive-routing-candidates` (scenario `specify-routes-before-scaffolding`) offers the homes proposed work could belong to from three sources. Only `runtime-work` reads a path, and only one under `runtime/` or a primitive's name, which an adopter's work does not carry; `rule-surface` and `spec-corpus` match words the description shares with a rule file's category or a spec's slug. In an adopter's project every candidate is a word match, so the nearest-named spec is what the gate offers.

Groom's Step 2 (`framework/reference/groom.md` §Groom decision tree) starts from those candidates, and `/{project}:review`, `/{project}:analyze` and `/{project}:implement` walk the same tree to route a deferred finding, so word matches steer where findings go.

Observed: an adopting project reported findings sent to the wrong spec on the evidence of a spec's name, or of a task already filed in it. In this repository, `/{project}:specify` for 083 returned 15 candidates, each on shared words ("cross", "project", "review"), none of them 079 or 081, the only two plans whose Affected Files name `runtime/src/primitives/commit_attribution.rs`, the code 083 changes.

## Behavior

**A path the work names ranks the plans that claim it first.** `derive-routing-candidates` gains a `path-claim` source. For each repo-relative path the description names, it offers every spec whose `plan.md` `## Affected Files` claims the path, read by `parse_affected_files` as `compute-review-scope` reads it: an exact path ranks ahead of a glob, a glob ahead of a directory, and specs within a rank order by number. Its candidates precede every other source's, and a spec it offers is not offered again by another.

**A path no plan claims is its own result.** A named path that no plan claims, exactly, by glob or by directory, is listed in the result as unclaimed, distinct from a description naming no path and from a source that could not be examined. An empty candidate list never stands for it.

**A claim is history, not ownership.** A path-claim candidate's reason says the spec's plan changed the path, never that the spec owns it. Which spec owns the code stays the routing judgment, and the decision the operator's, at the same gate.

**Groom's Step 2 names the evidence.** A spec's name, or a task already filed in it, is no evidence that the spec owns the code; a plan claiming the path is. Code no plan claims routes to a new spec through `/{project}:specify`'s procedure, or to an external work item, never to the nearest-named spec. The rule is stated in the decision tree, which review, analyze and implement already walk, not in the constitution.

**The sibling records follow.** 022's data model names the `path-claim` source among the primitive's sources in precedence order, and `specify-routes-before-scaffolding`'s 2026-10-16 resolution states this ranking as its exception.

## Edge Cases

- A path many plans claim (`runtime/src/primitives/check_artifacts.rs`): every claiming spec is offered in rank and number order, and the gate shows the count rather than picking one.
- A path claimed only by a sweep's glob or a directory (049's `runtime/src/**`, 022's `runtime/src/primitives/`): offered at that rank, below every exact claim.
- A path the description writes outside an inline code span: not read as one, since the description's paths are read by the acceptance-criterion path grammar, as a criterion's are (045's data model), so `and/or` names no path; a `:N` line citation after a path is stripped first, so a finding's `file:line` names its file.
- A path named only in a plan's prose, such as a "not changed" note: claims nothing, since only the section's table rows are read.
- A plan that cannot be read: named under `skipped`, and no path is reported unclaimed while one is, since an unread plan may claim it (`QUAL-CLAIM-001`).
- A description naming a primitive and no path: `runtime-work` offers its candidate as before.
- An adopter whose plans have no Affected Files table: every named path is unclaimed, and the result says so.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Which spec owns this?** 022, whose plan claims `runtime/src/primitives/` and whose data model holds the primitive's contract; every alternative (a new spec, 080, 017) also reopened 022 to correct that data model. The agent's recommendation, adopted by the operator, 2026-11-23.
- **Does `specify-routes-before-scaffolding`'s 2026-10-16 resolution stand?** In part. A plan's Affected Files still records history, not ownership, which is why a claim stays advisory. But at `97412348`, of the 94 modules under `runtime/src/primitives/`, the first-column paths of the 77 plans' `## Affected Files` tables name 34 in exactly one plan, 42 in two or more and 18 in none, and `commit_attribution.rs` in exactly 079 and 081; a word match offered neither. So path claims rank first. The agent's recommendation, adopted by the operator, 2026-11-23.
