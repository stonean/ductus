---
section: "Check Families"
---

# Family-34-reference-section-reachability

## Context

Spec 071 (command reference split) records the requirement in its scenario `reference-sections-are-reached-by-a-step`: an agent reads a reference section only when a step names it, so a `##` section no step names is never read. Nothing checked it, and 23 of 93 sections had gone unreached when it was found (measured at `b7b07500` over the 12 shipped references that have a command). Spec 080's `cited_tools` already walks the citations from a command into its reference sections to find the tools they invoke, so the reached set is a by-product of a walk the runtime makes, and `check-step-references` is where Family 34 reads command files.

## Behavior

- `check-step-references` and Family 34 extend their subject from the Instructions steps to the reference sections they reach: each `framework/reference/<command>.md` is judged against `framework/commands/<command>.md`, by the walk `tool_loading` makes for spec 080, which also follows a markdown anchor link `(#slug)` inside a reached document to the heading it names in that document.
- A `##` section reached by no step is a finding of kind `unreached-section`, naming the reference, the section and its line. A heading inside a fenced block or an HTML comment is not a section.
- A kept section carries `<!-- audit:kept <reason> -->` on the line directly beneath its heading and is reported in `reachability` as exempt with that reason, counted, never silent. A marker with no reason is an `empty-exemption` finding and exempts nothing; a marker on a section that is reached is a `stale-exemption` finding; a marker anywhere but directly beneath a `##` heading is a `misplaced-exemption` finding. The exemption cannot outlive its cause.
- `Severity tiers` and `Record fields` in `framework/reference/analyze.md` are the two sections carrying the marker, for the readers outside the command that 071's scenario names.
- A reference with no `##` section is reported as having none, a state of its own beside the examined ones, and a reference whose command or text cannot be read is a `skipped` entry with its reason: neither reads as a pass.
- The result states `sections-examined`, `sections-exempt`, `sections-unreached` and `references-without-sections` beside `reachability`, a per-reference list moved to the `lists-file` with the other lists past `[artifacts] read-size-bytes`. The CLI exits non-zero when `findings` is not empty or a subject could not be read.
- The check runs in a tree holding `framework/commands/` and `framework/reference/`; in an adopter's project it examines nothing and reports the skip that the Instructions subject already reports.
- Measured over this repository's tree by counting `##` headings outside fenced blocks in each `framework/reference/<command>.md` that has a command, under this walk: 74 sections, 2 of them the exempt pair, 0 unreached.

## Edge Cases

- A section reached only through a section already reached, and one reached only by an anchor link, are reached.
- A `§` or a bold span naming a `###` subsection reaches that subsection and not its parent `##` section.
- A marker whose reason is empty on an unreached section yields both `empty-exemption` and `unreached-section`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
