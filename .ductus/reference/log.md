# log — markdown-only reference

**Where a named section lives.** This file is read alongside its command, `log.md`, which is already loaded whenever this one is. A section named below that has no heading in this file is in the command file, along with the numbered Instructions steps this file cites. A bare `§name` is a heading in this file or a constitution marker; a `§name` with another file's path on the same line is a heading in that file.

With no ductus runtime registered, the host performs the same append with its own file tools (Read, Edit, Write) — no shell-pipeline substitution (§runtime-host-integration):

1. If `specs/inbox.md` does not exist, create it with a minimal heading (`# Inbox`) followed by a blank line.
2. Append the item to the inbox list as a new checkbox bullet:

   ```markdown
   - [ ] {item text}
   ```

3. Run `npx markdownlint-cli2` on the modified file.

Either path ends with the same report: the line that was added, the new total item count in the inbox, and the `/ductus:groom` pointer — and stops there.
