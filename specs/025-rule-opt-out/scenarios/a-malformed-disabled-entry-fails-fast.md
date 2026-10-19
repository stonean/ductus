---
section: "Acceptance Criteria"
---

# A-malformed-disabled-entry-fails-fast

## Context

`configuration-cross.md` `CFG-ENV-008` (MUST, added 2026-10-19) requires a configuration value that is present but malformed to fail fast, naming the setting and the value, rather than being skipped or replaced by a default. This spec's AC4 skips a malformed `[[review.disabled-rule-files]]` entry — one missing `file` or `reason`, or whose `reason` is under the minimum length — with a warning and continues. The owner decided on 2026-10-19 that the rule stands and `ductus` changes to comply.

## Behavior

- A malformed `[[review.disabled-rule-files]]` entry halts `/{project}:review` before any rule file is selected, naming the entry's index and the field that failed, the way a degenerate `[rules] surfaces` value already halts it (`framework/commands/review.md` step 5). The entry is not skipped.
- The halt is a configuration error that stops the run, not a review finding, so `blocking: true` keeps meaning only that a MUST rule was violated — the reason this spec's Resolved Questions gave for keeping malformed entries out of the exit status still holds.
- AC4 and `framework/commands/review.md`'s malformed-entry handling state the fail-fast behavior, and the generated command copies are regenerated.

## Edge Cases

- A duplicate entry is well-formed; its existing "entry [N] ignored" warning is unchanged.
- Waivers are not configuration values, so the malformed-waiver handling in `review.md` is unchanged by this scenario.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
