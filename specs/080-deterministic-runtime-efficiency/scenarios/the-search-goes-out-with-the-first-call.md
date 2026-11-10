---
section: "Loading tools"
---

# The-search-goes-out-with-the-first-call

## Context

The batched line in each Claude Code copy asked for one `ToolSearch` call before step 1 (AC29). In the cost round, where a command's first call was a deferred tool, the search went out alone in 4 of 5 runs: status's model path, log, specify and configure. status, log and specify each paid a model step of 1.51 to 1.70 s before the call, and configure's step went mostly to composing its call. fold sent its search and its deferred first call, `check-unfolded-specs`, in one message, and the call returned its result (`research.md`, R7).

## Behavior

- A command whose first `ductus` call is a deferred tool has its line ask for that call in the same message as the search that loads it, so the search need cost no model step of its own. Whether the model sends it there is the model's choice, not the host's (`research.md`, R7 and *One search, sent with the first call*; owner, 2026-11-09).
- The line stays generated from the command's files, and only Claude Code's copies carry it, as AC29 requires.

## Edge Cases

- A first call whose arguments come from an earlier result cannot go out with the search; the search then goes out alone, as now.
- A host release that refuses a call in the search's own message fails that call, and the model makes it again after the search, so the command still runs, at the cost of the step this scenario removes.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
