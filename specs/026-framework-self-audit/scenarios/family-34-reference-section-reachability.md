---
section: "Check Families"
---

# Family-34-reference-section-reachability

## Context

Spec 071 (command reference split) records the requirement in its scenario `reference-sections-are-reached-by-a-step`: an agent reads a reference section only when a step names it, so a `##` section no step names is never read. Nothing checked it, and 23 of 93 sections had gone unreached when it was found (measured at `b7b07500` over the 12 shipped references that have a command). Spec 080's `cited_tools` already walks the citations from a command into its reference sections to find the tools they invoke, so the reached set is a by-product of a walk the runtime makes, and `check-step-references` is where Family 34 reads command files.

## Behavior

- `check-step-references` and Family 34 extend their subject from the Instructions steps to the reference sections they reach: each `framework/reference/<command>.md` is judged against `framework/commands/<command>.md`, by the walk `tool_loading` makes for spec 080 started from the numbered steps under the command's `## Instructions` alone (each with its continuation lines, sub-steps and bullets; a citation in the preamble, a note after the steps or any other block reaches nothing), which also follows a markdown anchor link `(#slug)` inside a reached document to the heading it names in that document.
- A `##` section reached by no step is a finding of kind `unreached-section`, naming the reference, the section and its line. A heading inside a fenced block or an HTML comment is not a section.
- A kept section carries `<!-- audit:kept <reason> -->` on the first non-blank line beneath its heading and is reported in `reachability` as exempt with that reason, counted, never silent. A marker with no reason is an `empty-exemption` finding and exempts nothing; a marker on a section that is reached is a `stale-exemption` finding; a marker anywhere but on the first non-blank line beneath a `##` heading is a `misplaced-exemption` finding. The exemption cannot outlive its cause.
- Four sections carry the marker, for the readers outside the command that 071's scenario names: `Severity tiers` and `Record fields` in `framework/reference/analyze.md`, `Cross-spec context checklist` in `framework/reference/plan.md` and `Blocking semantics` in `framework/reference/review.md`. The last two are cited only from blocks that are no step, so they are exempt rather than reached; a marker on a section a step reaches is stale.
- A reference with no `##` section is reported as having none, a state of its own beside the examined ones, and a reference whose command or text cannot be read is a `skipped` entry with its reason: neither reads as a pass.
- The result states `sections-examined`, `sections-exempt`, `sections-unreached` and `references-without-sections` beside `reachability`, a per-reference list moved to the `lists-file` with the other lists past `[artifacts] read-size-bytes`. The CLI exits non-zero when `findings` is not empty or a subject could not be read.
- The check runs in a tree holding `framework/commands/` and `framework/reference/`; in an adopter's project it examines nothing and reports the skip that the Instructions subject already reports.
- Measured by `check-step-references` over this repository at the commit that added the step-start rule: 74 `##` sections outside fenced blocks and comments in 13 references, 4 exempt, 0 unreached, and 6 references with no section (`audit`, `consolidate`, `log`, `prune`, `specify`, `status`). A script counting from the numbered steps over the same tree before any marker found those four unreached.

## Edge Cases

- A section reached only through a section already reached, and one reached only by an anchor link, are reached.
- A section cited only from the Instructions preamble, a note after the steps or Scope Boundaries is unreached; one cited from a bullet inside a numbered step is reached.
- A `§` or a bold span naming a `###` subsection reaches that subsection and not its parent `##` section.
- A marker whose reason is empty on an unreached section yields both `empty-exemption` and `unreached-section`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
