# claude-md-import-syntax

**Introduced in:** ductus 0.60.0
**Summary:** Rewrite each `@import <path>` line in `CLAUDE.md` to the `@<path>` form Claude Code resolves, and remove the line importing `.ductus/constitution.md`, which exceeds one read.

## Background

Through ductus 0.59 the `CLAUDE.md` template wrote its imports in the `@import <path>` form — since 0.28, `@import .ductus/constitution.md` and `@import AGENTS.md` — and the bootstrap rendered each registered shared constitution into the `ductus:constitutions` block as `@import {document-path}`. Claude Code imports a file only as `@path`: a path ends at the first unescaped space, so `@import AGENTS.md` names a file called `import`, and neither line loaded anything. A `claude-style` session therefore started without the project's `AGENTS.md`. The pipeline commands still read the constitution themselves, so nothing else broke.

`CLAUDE.md` is installed once, with the `skip` strategy (`framework/bootstrap/ductus.md`, **Shared files with conflict handling**), and nothing rewrites it afterwards, so an existing adopter keeps the broken lines until this migration converts them. The constitution line is removed rather than converted. Converting it would begin loading a document larger than one read into every session, and the template no longer carries it. [Spec 002](../../specs/002-project-scaffolding/spec.md)'s `claude-md-template-imports-load` scenario governs.

## Procedure

1. **Resolve the file.** The file is `CLAUDE.md` at the project root. When it does not exist, stop silently and report nothing. Do not create one; the Shared Files manifest installs `CLAUDE.md`, not a migration.

2. **Find the import lines.** Walk the file line by line, tracking fenced code blocks: a fence opens on a line whose text, after at most three spaces, begins with three or more backticks or tildes, and closes on a later line holding only a run of the same character at least as long, so every line between is code. An import line is a line outside a fence whose text, after any leading whitespace, begins with `@import` followed by at least one space or tab and then a path: the rest of the line with trailing whitespace removed. When quotes (`"` or `'`) wrap the whole path, the path is what they wrap. A line that begins with a backtick holds a code span, not an import, and does not match, so an `@import` written as text stays. Lines inside the `ductus:constitutions` managed block match like any other. The next `/ductus` run re-renders that block in the new form anyway, and converting it here leaves the file correct in between.

3. **Idempotency check.** When no line matches, the file is converged: report `claude-md-import-syntax: no @import line outside code in CLAUDE.md — left as is` and stop without writing. This covers a file already fixed by hand: a line reading `@AGENTS.md`, or any other `@path` line, is never touched.

4. **Skip a pinned file.** When `CLAUDE.md` is listed in `.ductus/config.toml` `[pinned] files`, write nothing and report one line. Pinning opts a file out of framework writes, and this is one:

   `warning: CLAUDE.md is pinned; leaving its @import lines — delete the .ductus/constitution.md one and rewrite each other as @<path> by hand for Claude Code to load it.`

5. **Rewrite each import line in place.**

   - A line whose path is `.ductus/constitution.md` (with or without a leading `./`) is **removed**, together with its line ending. The pipeline commands read the constitution when they need it, and `/{project}:target` loads it once per session. A line an adopter wrote as `@.ductus/constitution.md` matched nothing in step 2 and stays: importing the constitution is that adopter's choice.
   - Every other import line becomes `@` followed by its path, keeping the line's leading whitespace, with a backslash put before each space in the path that does not already follow one. Claude Code imports neither a quoted path nor one with an unescaped space.
   - A shared constitution that an adopter also imports by hand, outside the managed block, is rewritten like any other line, not removed. The adopter added that line. Removing it would drop the import if the registration were later withdrawn.

   Preserve everything else byte for byte: each rewritten line keeps its own line ending, and nothing else in the file is normalized. Write atomically: a temporary file in the same directory, then a rename.

6. **Report** one line naming what changed: `claude-md-import-syntax: rewrote {n} import line(s) to @path`, where `{n}` counts the rewritten lines, and, when step 5 removed one, `; removed the .ductus/constitution.md import`. Then name each rewritten path, so the operator can confirm what loads at the next session start.

## Verification

- `git diff CLAUDE.md` shows only the import lines changing. Each `@import <path>` becomes `@<path>`, the `.ductus/constitution.md` import is gone, and nothing outside those lines moved.
- No line outside a fenced code block still begins `@import`, and an `@import` inside a fence or a code span is unchanged.
- The file's line endings are unchanged.
- A second run reports `no @import line outside code in CLAUDE.md` and writes nothing.
- A Claude Code session started in the project with no file tools can answer a question only `AGENTS.md` answers.
