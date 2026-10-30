# inbox-guidance-refresh

**Introduced in:** ductus 0.53.0
**Summary:** Replace the guidance comment at the top of an existing inbox with the current template's, or remove it when the template carries none, as it has since 0.60.0. Every item is left byte-identical.

## Background

`specs/inbox.md` is installed once, with the `create` strategy (`framework/bootstrap/ductus.md`, **Shared Files**), and nothing rewrites it afterwards — so an adopter's inbox keeps the guidance comment it was first installed with. Before spec 058 that comment documented three item forms, two of them written by machines: findings `/review`, `/analyze`, and `/implement` captured automatically, and rule gaps the `/ductus` adoption audit wrote. After 058 no command writes to the inbox on its own. Every finding is fixed, routed, or discarded in the run that found it ([§brownfield-inbox](../constitution.md#brownfield-inbox), Finding dispositions), and the inbox holds only what a person logs with `/{project}:log`. A header still describing automatic capture tells its next reader the opposite of what the pipeline does.

Since 0.60.0 the template carries no guidance comment: its rules are the constitution's, and a copy of them drifts. Against that template the comment is removed rather than replaced.

This migration rewrites the guidance and nothing else. **Items are never touched** — including items captured automatically before this change, which stay until `/{project}:groom` walks them through the same five routes as a logged one. [Spec 058](../../specs/058-findings-route-at-discovery/spec.md) governs.

## Procedure

1. **Resolve the inbox.** Read `[paths] specs-root` from the active config file (default `specs`); the inbox is `{specs-root}/inbox.md`. When it does not exist, there is nothing to refresh: stop silently, reporting nothing. Do not create one — the Shared Files manifest installs the inbox, not a migration.

2. **Idempotency check.** Locate the guidance comment: the first HTML comment whose opening line begins `<!-- Rules:`, running to the first `-->` after it.

   A file with **no** `<!-- Rules:` comment is converged, and is left alone: its owner removed or replaced the guidance, or this migration removed it, and a migration must not reinstate what an adopter chose to drop. Report `inbox-guidance-refresh: no guidance comment in {specs-root}/inbox.md — left as is` and stop.

   When the current `framework/templates/project/inbox.md` from the acquired archive carries no such comment, no inbox comment is current: continue at step 3. Otherwise compare the inbox's span, `<!--` through `-->` inclusive, with the same span in the template, normalizing line endings to `\n` for the comparison only. When they are equal **and** the text between the `# Inbox` heading and the comment is not the pre-058 introduction quoted in step 5, the inbox is converged: report `inbox-guidance-refresh: guidance already current` and stop without writing.

3. **Skip a pinned inbox.** When `{specs-root}/inbox.md` is listed in `.ductus/config.toml` `[pinned] files`, write nothing and report one line — pinning opts a file out of framework writes, and this is one:

   `warning: {specs-root}/inbox.md is pinned; leaving its guidance comment — make it match framework/templates/project/inbox.md by hand, removing it when the template carries none, if you want it refreshed.`

4. **Replace the comment, and only the comment.** Substitute the template's comment for the span step 2 located. When the template carries none, remove the span instead. When the lines it occupies hold nothing else, delete them whole; then, when nothing but blank lines follows them, delete those blank lines and the run of blank lines immediately before the span too, so the file ends with the line ending after its last remaining non-blank line, and otherwise, when the line after them is blank and the line before them is blank or there is none, delete that following blank line, so the removal does not leave two blank lines where it stood. When its first or last line holds other text, delete only the span's characters and keep that text. Everything outside that span is preserved exactly: the heading, any text between the heading and the comment (except as step 5 allows), and every item below it — including items in the forms the new guidance no longer documents. A comment an adopter edited is replaced or removed as well: the comment is framework guidance, the migration cannot tell a customization from an older shipped version, and git history keeps the old text. **Preserve the file's line endings**: when the file uses `\r\n`, write the new comment's lines with `\r\n`, and never normalize the rest of the file. Write atomically — a temporary file in the same directory, then a rename.

5. **Refresh the introduction only when it is the shipped one.** The pre-058 template carried an introductory paragraph between the heading and the comment, describing issues "captured incidentally during work". When the text between the `# Inbox` heading and the comment is exactly that paragraph (line endings normalized for the comparison only), replace it with the current template's introduction — the text between its `# Inbox` heading and its guidance comment, or its end when it carries none — under step 4's line-ending rule and in the same atomic write. When it differs in any way — the adopter wrote their own, or removed it — leave it untouched: a customized introduction is the adopter's. The pre-058 paragraph, verbatim:

   ```text
   Capture queue for issues not yet assigned to a feature spec — both the
   brownfield-adoption backlog and issues captured incidentally during work.
   Items are migrated to their proper home by `/groom` (see the constitution,
   §brownfield-inbox).
   ```

6. **Report** one line naming what changed — `inbox-guidance-refresh: refreshed the guidance comment`, or `inbox-guidance-refresh: removed the guidance comment` when the template carries none, followed by `and replaced the introduction` when step 5 replaced it — and the item count, unchanged, so the operator can see that nothing below the header moved.

## Verification

- The inbox's items — every `-` list item outside an HTML comment, the grammar the inbox primitives use — are byte-identical before and after, in the same order.
- `git diff` over the inbox shows changes inside the guidance comment, or its removal with the blank lines step 4 names, and at most the shipped introduction, and nowhere else.
- The file's line endings are unchanged.
- A second run reports `guidance already current`, or `no guidance comment … left as is` after a removal, and writes nothing.
