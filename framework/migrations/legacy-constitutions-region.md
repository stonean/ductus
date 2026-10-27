# legacy-constitutions-region

**Introduced in:** ductus 0.60.0
**Summary:** Remove the `<!-- ductus:constitutions -->` … `<!-- /ductus:constitutions -->` comment region from `CLAUDE.md` and `AGENTS.md` when it is the empty region the template shipped, unchanged, since `/{project}` maintains a `BEGIN`/`END` block instead.

## Background

From ductus 0.47.0 through 0.59.11 both project templates carried a comment region, marked `<!-- ductus:constitutions -->` and `<!-- /ductus:constitutions -->`, whose own comment said `/{project}` maintains it. It never did. Shared Files' **Shared constitution imports** step calls `merge-managed-block` with the `html-comment` marker style, which reads and writes `<!-- BEGIN ductus:constitutions -->` and `<!-- END ductus:constitutions -->`. Finding no such markers, every run appended that block at the end of the file and left the template's region empty, with a comment that was untrue. Since `071-command-reference-split` task 20 (`b0b1f9d0`) the templates ship the `BEGIN`/`END` markers, which reaches new adoptions only: both files are installed once and never rewritten.

The adopter owns both files, so the migration removes only a region the framework wrote and nobody has edited: one that matches a shipped version, ignoring line endings and trailing whitespace. An edited region is left in place and reported. The `BEGIN`/`END` block is not touched; Shared Files writes it later in the same run when a file lacks it.

## Procedure

Apply steps 1–7 to `CLAUDE.md`, then to `AGENTS.md`, each at the project root. A step that says *stop* ends the walk for that file only.

1. **Resolve the file.** When it does not exist, stop silently and report nothing.

2. **Find the region.** Here and below, a line's **text** is the line without its ending, `\n` or `\r\n`, and a **blank** line is one whose text is empty or holds only spaces and tabs. Walk the file line by line, tracking fenced code blocks as `claude-md-import-syntax` step 2 defines them, applied to each line's text. The region starts at the first line outside a fence whose text, with leading and trailing spaces and tabs removed, is `<!-- ductus:constitutions -->`, and ends at the first later line outside a fence whose text, trimmed the same way, is `<!-- /ductus:constitutions -->`.

3. **Idempotency check.** When no line starts the region, the file is converged: report `legacy-constitutions-region: {file} has no unmaintained constitutions region — left as is` and stop without writing. When a line starts the region but no line ends it, report `legacy-constitutions-region: {file} opens a ductus:constitutions comment region that never closes — left as is; remove it by hand` and stop without writing.

4. **Skip a pinned file.** When the file is listed in `.ductus/config.toml` `[pinned] files`, report one line and stop without writing:

   `warning: {file} is pinned; remove its <!-- ductus:constitutions --> comment region by hand — /ductus never maintained it, and the BEGIN/END ductus:constitutions block it does maintain is separate.`

5. **Compare.** Normalize the region, start and end lines included, for comparison only: take each line's text, remove its trailing spaces and tabs, and join the lines with `\n`. Normalize each shipped version below the same way, taking the lines between its fence lines. The region **matches** when it equals either version.

6. **Leave an edited region.** When it matches neither, report one line and stop without writing:

   `legacy-constitutions-region: {file}'s <!-- ductus:constitutions --> comment region differs from every shipped version — left as is; /ductus never maintained it, so move anything you added there out of it and remove the region by hand.`

7. **Remove a matching region.** Delete its lines, from the start line through the end line. When the line after the end line is blank, and the line before the start line is blank or there is none, delete that following blank line too, so the region does not leave two blank lines where it stood. When the region ran to the end of the file, also delete the blank lines before it, so the file ends with the line ending after its last remaining non-blank line, or is empty when none remains. Preserve every other byte, line endings included. Write atomically, keeping the file's mode: a temporary file in the same directory, then a rename. Report `legacy-constitutions-region: removed the unmaintained ductus:constitutions comment region from {file} — the BEGIN/END block carries the imports`.

`{file}` is `CLAUDE.md` or `AGENTS.md`, as walked.

## Shipped versions

The region as the templates carried it, from `eb043505` (2026-09-11) to the last commit before `b0b1f9d0` (2026-10-26). Each is compared as step 5 describes, and either may match either file.

**Version 1** — `framework/templates/project/claude-md.md`:

````markdown
<!-- ductus:constitutions -->
<!-- /ductus maintains this region. Registered shared constitutions
     (.ductus/config.toml [constitutions.*]) are imported here, one line each.
     Empty when none are registered. Edits inside are overwritten; the rest of
     this file is preserved byte-for-byte. -->
<!-- /ductus:constitutions -->
````

**Version 2** — `framework/templates/project/agents.md`:

````markdown
<!-- ductus:constitutions -->
<!-- /ductus maintains this region. Registered shared constitutions
     (.ductus/config.toml [constitutions.*]) are listed here, one line each.
     Empty when none are registered. Edits inside are overwritten; the rest of
     this file is preserved byte-for-byte. -->
<!-- /ductus:constitutions -->
````

## Verification

- After a removal, `git diff` of the file shows only the region's lines removed, plus the one blank line or the trailing blank lines step 7 names; the `BEGIN`/`END` block is unchanged.
- A file whose region was edited, never closes, or that is pinned is unchanged, and the run reported the one line naming it.
- After a removal, a second run reports `has no unmaintained constitutions region` and writes nothing; after an edited, unclosed or pinned file's report, a second run repeats that report. No run writes to a converged file.
- The file's line-ending style is unchanged.
