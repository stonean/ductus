---
section: "Link and reference integrity"
---

# A-revision-qualified-path-is-not-a-working-tree-reference

## Context

`check-orphaned-references` finds a managed path in a referrer by scanning for a managed-root prefix at the start of a token and taking the run of path characters that follows. A prefix preceded by a path character or `@` does not start a token, so `x.ductus/y` is never read as a reference to `.ductus/y`. A colon was not in that set.

Git names a file at a revision as `<rev>:<path>` — `git show abc1234:specs/002-auth/spec.md` — and a file in the index as `:<path>`. An adopter that retired a spec directory and kept pointers to its last text wrote exactly that form, in a code span, in its `AGENTS.md`, `README.md` and `specs/system.md`. The check matched the `specs/` root after the colon, took the remainder as a working-tree path, found it absent, and reported three orphaned references. Observed on a live adopter running 0.58.0, and reproduced unchanged on 0.59.0.

None of the three is an orphan. The path resolves through git's object store at the named revision; no migration can move it, and it makes no claim about the working tree.

## Behavior

**A git object name is not a working-tree reference.** A managed-root prefix glued to a preceding `:` is not a candidate when the token before the colon is empty — the `:<path>` index form — or shaped like a revision: `abc1234`, `HEAD~2`, `origin/main`, `v0.58.0`. The token runs back from the colon to whitespace or one of `` ` ``, `'`, `"`, `=`, `(`, `[`, `,`, `;`. Such a name is excluded the way `x.ductus/y` is — not reported under `skipped`, since nothing recognized it as a working-tree reference — and `matched-prefixes` is unchanged.

**A colon alone does not decide it.** A colon-joined path list names working-tree directories: `PATH="$PATH:.ductus/bin"` in a hook is a reference a migration can orphan, and excluding every glued colon would hide it, which is the failure this check exists to prevent. So when the token before the colon contains `$` or `%` (a variable) or starts with `/` or `.` (a path; a refname cannot start with either), the entry after it is checked like any other reference. So is the target of a markdown link reference definition written without a space, `[old]:specs/x/spec.md`, whose token ends in `]`: CommonMark allows the form, and a label's target is a working-tree path.

**The revision is read by shape, never verified.** No `git cat-file`, no git dependency. Whether the object exists at that revision is a claim about history, outside this check's subject either way, and verifying it would make the result depend on clone depth, which a shallow CI checkout answers differently from a developer's.

**The same path written bare is still checked.** A file citing `abc1234:specs/x/spec.md` on one line and `specs/x/spec.md` on another yields exactly one finding — the bare one — when `specs/x/spec.md` does not exist.

## Edge Cases

- **A colon followed by a space** — `See: specs/x/spec.md` — leaves the prefix at a token start, and the reference is checked as before. Only a glued colon can exclude.
- **A list entry after a bare word** — `PATH=bin:.ductus/x` — is excluded. `bin` is a valid branch name and nothing on the line says otherwise, so the shape cannot tell the two apart; the reading that skips is chosen because the revision form is the one observed in adopter prose.
- **Prose with no space after the colon** — `see:specs/x/spec.md` — reads `see` as a revision and is excluded. It was a finding before; it is now the same silent skip as the bare-word list entry, and for the same reason.
- **A URL** — `https://host/owner/repo/blob/main/specs/x.md` — was already excluded: the prefix follows `/`, a path character.
- **A revision-qualified path under another managed root** — `abc1234:.ductus/constitution.md` — is excluded the same way; the boundary is tested per prefix, not per root.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
