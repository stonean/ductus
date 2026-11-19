---
section: "Design Decisions"
---

# Colon-form-commands-run-on-pi

## Context

Shipped text names each command in the `claude-style` colon form, `/{project}:<name>`: about 844 times across `framework/commands/`, `framework/reference/`, the constitution, `framework/rules/`, `framework/templates/`, `framework/bootstrap/` and `framework/migrations/` (counted at `7d590049`). 491 of them are in files `/ductus` scaffolds once per project, whatever agents it selects. The runtime prints the same form in its guidance.

Pi names an installed command by its prompt template's filename, `/{project}-<name>` (D1). So on Pi a colon-form reference names no command, and typed, it reaches the model as plain text. The bridge respells the form only in its own two commands' output (`piSpelling`, spec 080).

071's Resolved Questions left the fix as its own change. The owner chose on 2026-11-19 to have the bridge register colon-form aliases, built before the 0.60.0 release.

Pi 1.5.2's source supports this:

- `registerCommand` (`core/extensions/loader.js`) accepts any non-empty name.
- A typed command's name is parsed up to its first space (`_tryExecuteExtensionCommand`, `core/agent-session.js`).
- `sendUserMessage` with `expandPromptTemplates: true` dispatches extension commands and expands prompt templates as typed input does. The option is in the extension types of both 1.5.0 and 1.5.2.

## Behavior

- Once the bridge has learned the namespace at load, it registers `/{project}:<name>` for every prompt template `/ductus` installed as `.pi/prompts/{project}-<name>.md`. Each colon-form reference then names a command on Pi.
- An alias runs as typing `/{project}-<name>` with the same arguments does. Its handler submits that text through `pi.sendUserMessage` with `expandPromptTemplates: true`. Pi's own template expansion and argument substitution apply, and the bridge holds no copy of either.
- `/{project}:status` and `/{project}:help` run the same handlers as `/{project}-status` and `/{project}-help`. Each shows the view or the guide with no model turn.
- The hyphen form keeps working unchanged. No installed file is renamed, and the enforcement glob for `.pi/prompts` is unchanged.
- A load that registers no command registers no alias either: the runtime cannot start, or the call naming the namespace fails. The notice the load shows is unchanged.

## Edge Cases

- A template installed after the bridge loaded, such as by a `/ductus` run mid-session, gets its alias at the next session start, as Pi's own template discovery does at load.
- A `{project}-*.md` name whose `<name>` falls outside a command name's grammar (`[a-z][a-z0-9-]*`) gets no alias.
- A project's own prompt template named `{project}:<name>.md` is shadowed by the alias, since Pi dispatches an extension command before it expands a template.
- Pi's command completion lists both spellings of each command.
- A pi older than 1.5.0 is not examined.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
