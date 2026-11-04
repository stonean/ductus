---
section: "The review record"
---

# Scope-results-have-a-bounded-size

## Context

`compute-review-scope` returns the review's `scope`, `modified-since` and `plan-affected` path lists, and `diff-cross-spec` returns `cross-spec-paths`. Each list grows with the history since the diff base, and no artifact of this spec commits to a maximum size or to pagination, which `BE-PAYLOAD-001` (`framework/rules/performance-backend.md`) requires of any result that returns a collection or a variable-size body. The 2026-10-27 analysis found it as a blocking finding, and it is not hypothetical. The measurement scenario `an-empty-scope-flag-does-not-decide-the-denominator` records it: on 2026-09-14, 8 of 54 specs' `compute-review-scope` payloads exceeded 100KB, the largest 159,303 bytes, while every other primitive's result sat under 25KB. An MCP call over the host's output cap returns an error with no result at all, which is why `framework/commands/review.md` step 1 tells a host to re-run the call through the runtime binary with its output redirected to a file. The rule's subject is the result's size, and the runtime is a local stdio process, but `AGENTS.md` records that this rule still fires there.

The routing was the owner's delegated disposition (2026-10-27): this spec owns these result shapes (`data-model.md`), so the bound is recorded here rather than closed with a caveat.

## Behavior

- **Every result of `compute-review-scope` and `diff-cross-spec` has a stated maximum size: the `[artifacts] read-size-bytes` setting, 50,000 bytes by default, of serialized result.** It is the size spec 063 already treats as what a host reads in one call, so one setting tunes it for a host with another cap, and it is stated in `data-model.md` beside each shape.
- **Up to the bound, the lists are returned inline, as now. Past it, the result carries each list's count and the path of a file that holds the lists whole**, and no list inline. The file is derived state, gitignored and written whole by the call that names it, and the result says which form it took, so a counts-only result never reads as an empty scope.
- **A bound never drops what the caller needs.** A review must read every in-scope file, and `write-review` derives `scope` itself, so a result that would exceed the bound gives the caller a way to reach every entry. It never truncates a list silently, and it never looks like a short complete list (`QUAL-CLAIM-001`).
- **Both paths keep working.** A host on the MCP path and the `ductus exec` walker, which dispatches each step once with no per-step arguments, each obtain the whole scope.

## Edge Cases

- **A scope that fits.** It is returned whole in one result, as now. A result exactly at the bound fits.
- **A bound reached on the CLI.** The CLI has no output cap, and the same result shape still applies to it, so both surfaces read one contract.
- **A list file larger than one read.** The host reads it in parts, by line offset, until every entry is read (§grounding, a partial read is not a read).
- **Two calls in one tree.** Each result names the file its own call wrote, so a concurrent review of the same feature never reads another call's lists.
- **A file that cannot be written.** The call fails; it never returns the lists truncated or a counts-only result with no file behind it.
- **A malformed `[artifacts] read-size-bytes`.** The call halts naming the setting, the value and the file, as every reader of the setting does.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which mechanism bounds the two results, and what is the stated maximum?** The lists are returned inline up to `[artifacts] read-size-bytes` (50,000 bytes by default) of serialized result; past it, the result carries their counts and the path of a file holding them whole (owner, 2026-11-03). It is the one mechanism that keeps one snapshot, every entry reachable, and both the MCP path and `ductus exec` working unchanged: the walker carries the path in its context like any other result key. Measured 2026-11-03 at `53d3d720`, the release binary's CLI with default arguments over all 76 spec directories, bytes of JSON output, no call failing: `compute-review-scope` reached 113,042 bytes (049, 1,056 scope entries), median 46,258, over 50,000 for 34 specs; `diff-cross-spec` reached 43,352, over 50,000 for none. Rejected: cursor pagination per `BE-PAGE`, which needs a continuation loop the walker lacks, re-computes each page from a tree that can change between them, and took six calls for 049 at 200 entries a page; and a narrower default scope, which states no maximum, since a long-lived spec's scope still grows past any size. Implementing it — 066 task 7 — corrects what describes the unbounded result: `specs/022-deterministic-runtime/plan.md`'s "bounded by the project … rather than paginated" and its re-run-through-the-binary workaround, and `framework/commands/review.md` step 1's.
