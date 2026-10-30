# strip-inbox-guidance

**Introduced in:** ductus 0.60.0
**Summary:** Remove the guidance comment, whatever its text, and the template's former placeholder item, only as that exact line, from an existing inbox, since the template has carried neither since 0.60.0. Every other item is left byte-identical.

## Background

Until 0.60.0 the inbox template carried a `<!-- Rules: … -->` guidance comment and, below it, a placeholder item, `- [ ] {Brief description of the issue and any relevant context}`. Since then it carries neither ([spec 058](../../specs/058-findings-route-at-discovery/spec.md)): the comment restated the constitution, and a copy drifts; the placeholder sat outside the comment, so every inbox primitive counted it as an outstanding item.

`specs/inbox.md` is installed once, with the `create` strategy (`framework/bootstrap/ductus.md`, **Shared Files**), and nothing rewrites it afterwards. `inbox-guidance-refresh` (0.53.0) removes the comment from an inbox it reaches now, but an adopter who ran it before 0.60.0 received the comment the template then carried, and it never touches the placeholder. Nothing else would ever remove either.

The comment is framework guidance, so it is removed whatever its text, as `inbox-guidance-refresh` replaces or removes one. The placeholder is removed only as the exact line the template shipped: a line that differs in any way is an item someone wrote.

## Procedure

1. **Resolve the inbox.** Read `[paths] specs-root` from the active config file (default `specs`); the inbox is `{specs-root}/inbox.md`. When it does not exist, stop silently, reporting nothing. Do not create one — the Shared Files manifest installs the inbox, not a migration.

2. **Find what to remove.** Here and below, a line's **text** is the line without its ending, `\n` or `\r\n`, and a **blank** line is one whose text is empty or holds only spaces and tabs.

   - The **comment** is the first HTML comment whose opening line begins `<!-- Rules:`, running to the first `-->` after it, located as `inbox-guidance-refresh` step 2 locates it.
   - A **placeholder** is each line whose text is exactly `- [ ] {Brief description of the issue and any relevant context}` and that lies outside every fenced code block and every HTML comment, the grammar the inbox primitives use for an item.

3. **Idempotency check.** When the file has no comment and no placeholder, it is converged: report `strip-inbox-guidance: {specs-root}/inbox.md has no guidance comment and no placeholder item — left as is` and stop without writing.

4. **Skip a pinned inbox.** When `{specs-root}/inbox.md` is listed in `.ductus/config.toml` `[pinned] files`, write nothing and report one line — pinning opts a file out of framework writes, and this is one:

   `warning: {specs-root}/inbox.md is pinned; remove its <!-- Rules: --> guidance comment and its "- [ ] {Brief description of the issue and any relevant context}" placeholder item by hand if you want it to match framework/templates/project/inbox.md.`

5. **Remove the comment.** When step 2 found one, remove its span, `<!--` through `-->` inclusive. When the lines it occupies hold nothing else, delete them whole; then, when nothing but blank lines follows them, delete those blank lines and the run of blank lines immediately before the span too, so the file ends with the line ending after its last remaining non-blank line, and otherwise, when the line after them is blank and the line before them is blank or there is none, delete that following blank line, so the removal does not leave two blank lines where it stood. When its first or last line holds other text, delete only the span's characters and keep that text.

6. **Remove each placeholder**, in file order, each against the file as the previous removal left it. Delete its line. When nothing but blank lines follows it, delete those blank lines and the run of blank lines immediately before it too, so the file ends with the line ending after its last remaining non-blank line, or is empty when none remains. Otherwise, when the line after it is blank and the line before it is blank or there is none, delete that following blank line too.

7. **Write once.** Steps 5 and 6 produce one new file. Preserve every other byte, line endings included, and never normalize them. Write atomically, keeping the file's mode: a temporary file in the same directory, then a rename.

8. **Report** one line naming what changed and the item count before and after, so the operator can see that only the placeholder left: `strip-inbox-guidance: removed the guidance comment and {n} placeholder item(s) from {specs-root}/inbox.md — {before} items before, {after} after`, dropping `the guidance comment and` when step 2 found none, and `and {n} placeholder item(s)` when it found no placeholder. An item is a `-` list item outside every fenced code block and HTML comment.

## Verification

- Every item but a removed placeholder is byte-identical before and after, in the same order, and the item count drops by the number of placeholders removed.
- `git diff` over the inbox shows the comment's removal, each placeholder line's, and the blank lines steps 5 and 6 name, and nowhere else.
- The file's line endings are unchanged.
- An inbox the 0.59.11 template installed, still holding nothing but its placeholder, becomes byte-identical to the current `framework/templates/project/inbox.md` when its introduction is the current one.
- A pinned inbox is unchanged, and the run reported the one line naming it.
- A second run reports `has no guidance comment and no placeholder item` and writes nothing.
