---
section: "The review record"
---

# Scope-results-have-a-bounded-size

## Context

`compute-review-scope` returns the review's `scope`, `modified-since` and `plan-affected` path lists, and `diff-cross-spec` returns `cross-spec-paths`. Each list grows with the history since the diff base, and no artifact of this spec commits to a maximum size or to pagination, which `BE-PAYLOAD-001` (`framework/rules/performance-backend.md`) requires of any result that returns a collection or a variable-size body. The 2026-10-27 analysis found it as a blocking finding, and it is not hypothetical. The measurement scenario `an-empty-scope-flag-does-not-decide-the-denominator` records it: on 2026-09-14, 8 of 54 specs' `compute-review-scope` payloads exceeded 100KB, the largest 159,303 bytes, while every other primitive's result sat under 25KB. An MCP call over the host's output cap returns an error with no result at all, which is why `framework/commands/review.md` step 1 tells a host to re-run the call through the runtime binary with its output redirected to a file. The rule's subject is the result's size, and the runtime is a local stdio process, but `AGENTS.md` records that this rule still fires there.

The routing was the owner's delegated disposition (2026-10-27): this spec owns these result shapes (`data-model.md`), so the bound is recorded here rather than closed with a caveat.

## Behavior

- **Every result of `compute-review-scope` and `diff-cross-spec` has a stated maximum size.** It is small enough that an MCP host receives the result rather than an output-cap error, and it is stated in `data-model.md` beside each shape.
- **A bound never drops what the caller needs.** A review must read every in-scope file, and `write-review` derives `scope` itself, so a result that would exceed the bound gives the caller a way to reach every entry. It never truncates a list silently, and it never looks like a short complete list (`QUAL-CLAIM-001`).
- **Both paths keep working.** A host on the MCP path and the `ductus exec` walker, which dispatches each step once with no per-step arguments, each obtain the whole scope.

## Edge Cases

- **A scope that fits.** It is returned whole in one result, as now.
- **A bound reached on the CLI.** The CLI has no output cap, and the same result shape still applies to it, so both surfaces read one contract.

## Open Questions

- Which mechanism bounds the two results: cursor pagination per `BE-PAGE` (which the `ductus exec` walker, dispatching each step once with no arguments, cannot follow without a change of its own), a result that returns counts and a path to a file holding the lists, or a narrower default scope — and what is the stated maximum?

## Resolved Questions

*None yet.*
