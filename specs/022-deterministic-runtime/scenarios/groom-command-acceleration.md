---
section: "Follow-on scenarios"
---

# Groom-command-acceleration

## Context

The spec's third deferred follow-on, never shipped: `/ductus:groom` remains legacy prose with zero primitive references, and — worse than mere non-acceleration — its prose re-specifies long-hand, in its own words, four writes that already exist as primitives and that `/ductus:amend` routes through them: scenario creation from template (`create-scenario`), task append (`append-task`), the done→in-progress reopen with concurrent-edit guard (`set-status`), and the session-target write with cli-config-dir preservation (`write-session`). Two divergent specifications of the same writes is exactly the drift §drift-prevention exists to prevent. The routing decision per inbox item is the semantic core and maps to the `routeInboxItem` extension point (typed builder ships in extension-request-hygiene).

## Behavior

`/ductus:groom`'s Instructions are rewritten to the parseable conventions: the inbox walk reads `specs/inbox.md`, each item's routing decision is the `routeInboxItem` extension seam (spec / scenario / rule / chore / discard, per the decision tree; a chore is done in the pass and then removed), and the mechanical consequences invoke the same primitives amend uses — `create-scenario` + `append-task` for a scenario route, `set-status` for a done-spec reopen, `write-session` where the target changes — plus the item's removal from the inbox and the completion-count summary. The file parses cleanly, leaves `legacy-prose-commands.txt`, and the markdown-only reference names the same operations as fallback prose (one contract, two paths).

## Edge Cases

- A chore item is done in the pass and then removed, as the constitution's inbox rules require; it is left in place only when the fix is blocked or turns out not to be mechanical, and groom says which.
- An item routing to a spec that does not exist directs the user to `/ductus:specify` and moves on, as today.
- Inbox item removal is the `remove-inbox-item` primitive (`coverage-expansion-primitives`), which groom step 8 invokes; on the markdown-only path the host removes the line with `Edit`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
