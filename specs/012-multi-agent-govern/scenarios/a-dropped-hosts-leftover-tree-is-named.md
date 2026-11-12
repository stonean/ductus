---
section: "Agent Selection"
---

# A-dropped-hosts-leftover-tree-is-named

## Context

`/ductus` no longer supports Auggie, Antigravity and OpenCode (the agent-removal chore, [task 12](../tasks.md)), so no Agent Registry row names `.augment/`, `.agents/`, `.opencode/`, or a root `opencode.json` or `opencode.jsonc`. A project adopted before the removal still holds one of them. Auto-detection reads only registry rows' `config_dir`, so the tree is neither scaffolded nor mentioned, and its owner is left to wonder whether `/ductus` forgot it. The runtime refuses writes into such a tree, so nothing `/ductus` does can clean it up either.

## Behavior

- When the project holds any of `.augment/`, `.agents/`, `.opencode/`, `opencode.json` or `opencode.jsonc`, and no Agent Registry row names that path as its `config_dir`, `/ductus` prints one non-gating line in Agent Selection, beside auto-detection: the host is no longer supported, `/ductus` leaves the tree alone, and the operator removes it by hand.
- The line never prompts, never changes the selected agent set, and never stops the run.
- A path a registry row names is not reported, so a row added later for one of these hosts silences its line without an edit here.

## Edge Cases

- **Several of the five paths present** — one line apiece, or one line naming each.
- **None present** — nothing is printed.
- **Only a dropped host's tree and no registered agent's directory** — the notice still prints, and the first-run prompt proceeds as if no agent directory were detected.

## Open Questions

*None — all resolved.*
