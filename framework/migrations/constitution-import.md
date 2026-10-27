# constitution-import

**Introduced in:** ductus 0.60.0
**Summary:** Add `@.ductus/constitution.md` to `CLAUDE.md` when nothing there imports the constitution, so a Claude Code session starts with it.

## Background

`050-constitution`'s cut brought the constitution to one read, at most 50,000 bytes as shipped, before `{project}` is substituted, so the `CLAUDE.md` template imports it at startup as `@.ductus/constitution.md`, above `@AGENTS.md`. A session started from a handoff prompt then has the constitution without running a command. `CLAUDE.md` is installed once, with the `skip` strategy (`framework/bootstrap/ductus.md`, **Shared files with conflict handling**), so an existing adopter keeps whatever its file held. Three kinds of file lack the import. In one, an earlier form of `claude-md-import-syntax`, which `--ref=main` adopters could run before any release carried it, removed the `@import .ductus/constitution.md` line. Another was installed from a template that carried no constitution line. The third is one the adopter wrote before adopting, which the skip strategy kept. `claude-md-import-syntax` runs first in a chain and converts an existing constitution line rather than removing it, so this migration finds that line and writes nothing. It runs second because both entries share `introduced_in` and `constitution-import` sorts after `claude-md-import-syntax` on the `id` tie-break. [Spec 050](../../specs/050-constitution/spec.md)'s `the-constitution-fits-one-read` scenario governs.

## Procedure

1. **Resolve the file.** The file is `CLAUDE.md` at the project root. When it does not exist, stop silently and report nothing. Do not create one; the Shared Files manifest installs `CLAUDE.md`, not a migration.

2. **Find the constitution import.** Walk the file line by line, tracking fenced code blocks: a fence opens on a line whose text, after at most three spaces, begins with three or more backticks or tildes, and closes on a later line that, after at most three spaces, holds a run of the same character at least as long and nothing else but spaces or tabs; a fence never closed runs to the end of the file. The constitution is **imported** when a line outside a fence reads, after any leading whitespace, `@.ductus/constitution.md` or `@./.ductus/constitution.md`, with nothing after it but whitespace. A line outside a fence that begins `@import` and names `.ductus/constitution.md` counts too: `claude-md-import-syntax`, which runs first, converts it, or in a pinned file has already told the operator to, so adding a second line here would import the constitution twice.

3. **Idempotency check.** When the constitution is imported, the file is converged: report `constitution-import: CLAUDE.md already imports the constitution — left as is` and stop without writing.

4. **Skip a pinned file.** When `CLAUDE.md` is listed in `.ductus/config.toml` `[pinned] files`, write nothing and report one line. Pinning opts a file out of framework writes, and this is one:

   `warning: CLAUDE.md is pinned; add @.ductus/constitution.md above @AGENTS.md by hand for Claude Code to load the constitution at startup.`

5. **Add the line.** The new line is `@.ductus/constitution.md` followed by the file's line ending: the first one the file contains, or `\n` when it contains none. Insert it immediately before the first line outside a fence that reads `@AGENTS.md`, ignoring leading and trailing whitespace, with that line's leading whitespace, where the template puts it. With no such line, insert it immediately after the file's first line when that line is a `#` heading, first ending that line with the file's line ending if it has none, and otherwise as the first line. Preserve everything else byte for byte. Write atomically: a temporary file in the same directory, then a rename.

6. **Report** `constitution-import: added @.ductus/constitution.md to CLAUDE.md`, so the operator knows the constitution loads at the next session start.

## Verification

- `git diff CLAUDE.md` shows one added line, `@.ductus/constitution.md`, directly above `@AGENTS.md` when the file has that line, and nothing else changed but a line ending given to a first line that had none.
- The file's line endings are unchanged.
- A second run reports `already imports the constitution` and writes nothing.
- A Claude Code session started in the project with no file tools can answer a question only the constitution answers.
