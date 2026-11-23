---
section: "Versioning and Evolution"
---

# The-tag-check-reads-every-entry-above-the-newest-version

## Context

`scripts/check-release-tag.sh`, which the release workflow's `audit` job runs before anything is published, refuses a tag while `runtime/CHANGELOG.md`'s `## [Unreleased]` section holds an entry, a list item or a `###` subsection, so a release commit that leaves an entry there, a rule change's announcement among them, stops the release (this spec's task 43, `0e1e68e1`; 048's signpost states it). It finds the section by that one heading. 020's commit `8ef7d0e5` opened the section as `## Unreleased`, and the script counted nothing under it, so a release commit that left that entry in place would have passed the check and shipped the entry unannounced. The 0.63.0 release commit `dbfb04da` gave the section its version, so that release was not exposed. The fix joins this reopening, and the check reads every entry above the newest version heading, whatever the pending section is called (the agent's recommendations, adopted by the operator, 2026-11-23).

## Behavior

- `scripts/check-release-tag.sh` counts every list item and `###` subsection above `runtime/CHANGELOG.md`'s newest `## [X.Y.Z]` heading, and refuses the tag while any sits there, naming the count and that heading. A changelog whose only text above that heading is prose, the file's own introduction included, still passes.
- `scripts/tests/test-check-release-tag.sh` holds a case for a pending section headed `## Unreleased`, which fails with the new guard removed, beside its cases for `## [Unreleased]`, whose expected message follows the new wording.
- 048's signpost names the newest version heading rather than the `## [Unreleased]` spelling.

## Edge Cases

- **A changelog with no version heading** counts every list item and `###` subsection in it, since nothing in it has shipped.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
