---
section: "Behavior"
---

# Help-answers-with-no-model-turn

## Context

`/{project}:help` prints a static guide verbatim and calls no tool, so on the model path its whole cost is the model retyping that guide. The cost round in `research.md`'s *Command cost (AC8)* measured it at 16.7 s to result, 2,204 output tokens and $0.270 for 6,146 characters of reply (R1 under *Each command's reduction*).

## Behavior

- In Claude Code, `/{project}:help` typed by the user is answered with no model turn, as `/{project}:status` is: the guide is shown and the typed command is not echoed back. `/ductus` and `/{project}:configure` install what answers it beside the status hook, under the same version gate.
- In Pi, the bridge registers `/{project}-help`, whose handler shows the guide as a displayed message with no turn, as `/{project}-status` does.
- The guide shown is the one the model path prints, with the project's name in each command.
- Auggie, OpenCode and Antigravity keep the model path for help, as they do for status.

## Edge Cases

- The answer cannot be produced, such as a missing runtime: it fails without blocking, so the host runs the command on the model path, as the status hook does.
- Arguments after the command are ignored, as the model path ignores them.
- The agent invoking `/{project}:help` itself takes the model path.
- On Pi, the guide spells each command as Pi runs it, `/{project}-<name>`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
