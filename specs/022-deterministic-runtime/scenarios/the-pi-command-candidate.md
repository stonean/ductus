---
section: "Per-project file resolution"
---

# The-pi-command-candidate

The pi host support feature ([064](../../064-pi-host-support/spec.md), 064) adds an agent layout. Its command files are pi prompt templates: flat, non-recursive, installed at `.pi/prompts/{project}-{name}.md` — a second shape alongside the directory-shaped `claude-style` layout `Host::command_file_candidates` already resolves. This scenario records the pi candidate in the runtime's command-resolution contract.

## Behavior

`Host::command_file_candidates` (runtime/src/host.rs) returns the installed slash-command file's candidates in a fixed resolution order:

1. `{cli-config-dir}/commands/{project}/{command_name}.md` — `claude-style` (Claude Code), tried first,
2. `{cli-config-dir}/prompts/{project}-{command_name}.md` — `pi` (flat project-hyphenated prompt templates), **appended last**.

Each adopter installs into exactly one layout, selected by the agent's registry row; the `cli-config-dir` in the gitignored session file is the real selector, and the candidate order is belt-and-braces — trying both lets the runtime resolve any supported layout without knowing which agent wrote the file. The pre-existing candidate keeps its position first, so every pre-pi adopter resolves identically.

The runtime's two resolution callsites (`main::run_exec` and `interpreter::payload::locate_command_file`) MUST build their candidate list through this method rather than formatting the installed path inline, so the layout set lives in one place and the callsites cannot drift; both walk the list in order and take the first path that exists. The surrounding candidates are unchanged: `framework/commands/<name>.md` is tried first (the framework's own repo), the installed candidates next, and `framework/bootstrap/<name>.md` last (bootstrap procedures invoked before any framework files exist).

## Edge Cases

- **A repo carrying two layouts' files** (e.g. a contributor who adopted both claude and pi, both installed under their own config dirs, but one session file): only the `cli-config-dir`-selected candidates are ever returned — the list is built from the session's value, so the other layout's files are not candidates at all. The two candidates can coexist only if one `cli-config-dir` value spells both forms, which no supported layout does.
- **`.pi` session identity with no installed prompts** (fresh checkout, `/ductus` not yet run): both candidates miss, and resolution surfaces the existing "command file not found" error — the pi shape adds no new failure mode, it adds one more path to the same lookup.
- **A custom `[host] project`** (an adopter that named its own namespace): the pi candidate names `{project}` from the resolved host block exactly as the `claude-style` shape does — `.pi/prompts/acme-specify.md` for a project named `acme`, never the framework's own namespace.
- **`pi`-only `exec` against a pi-shaped install**: `ductus exec specify` in a repo whose session records `cli-config-dir = .pi` and whose `.pi/prompts/{project}-specify.md` exists resolves and walks that file — no pi-specific exec path exists or is needed; the candidate is the entire runtime surface of the feature.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
