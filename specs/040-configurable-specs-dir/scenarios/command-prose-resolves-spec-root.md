---
section: "Resolution behavior"
---

# Command-prose-resolves-spec-root

## Context

AC7 says no pipeline command reads or writes a hardcoded `specs/` path. It is
checked, and it is not satisfied: fourteen host-acted sites across six command
files still name the literal.

The confirmed ones, each verified against the file:

- `log.md:33` passes a literal `specs/inbox.md` to `lint-markdown`, after
  `append-inbox` wrote `{specs-root}/inbox.md` — so the lint targets a file the
  append did not touch.
- `log.md:42` creates a stray `specs/inbox.md` on the markdown-only path.
- `groom.md:30` checks for and reads a hardcoded `specs/inbox.md`, and **no
  primitive owns that read**, so both paths carry the literal and `/groom`
  reports "No inbox file found" under a renamed root.
- `review.md:382` diffs `git diff <diff-base>..HEAD -- specs/inbox.md`.
- `amend.md:60` runs `git status --porcelain -- specs/{feature}/…`.
- `implement.md:105` runs `git log --oneline -- specs/{feature}/tasks.md`, and
  `implement.md:97`, `:98`, `:133`, `:136` read or diff literals alongside it.

The root cause is a misapplied boundary rather than an oversight per site.
AC11 says "no other prose is parameterized", and that was read as covering
command bodies wholesale. But the line AC7 draws is not *constitution versus
command body* — it is **illustrative versus acted-on**. A path a primitive
resolves is fine: the primitive reads `[paths] specs-root` itself and the prose
merely describes what it does. A literal the **host** hands to a shell command,
a file tool, or a primitive argument is the defect, because nothing between the
prose and the filesystem resolves it.

The fix pattern already exists in the corpus and was simply never propagated.
`specify.md:60` carries a one-line note:

> **Spec-root resolution.** Every `specs/…` path below is written under the
> configured `[paths] specs-root` (default `specs`; spec 040).

It is the only command file that has it, which is why `specify.md:119`'s bare
`Create specs/{NNN-feature-name}/` is already correct while the same shape
elsewhere is not.

Adopter-reported. A project with a non-default spec root hits this on `/log`,
`/groom`, and `/review`.

## Behavior

Every command file that **acts on** a spec-root path resolves it, by one of two
means, chosen per site rather than applied uniformly:

- **The blanket note**, where the command's own prose is the instruction the
  host follows: the `specify.md:60` note verbatim, placed once near the top of
  the Instructions section. This covers the markdown-only path, which is where
  most of these sites live.
- **An explicit resolution step**, where a literal is passed as an *argument* —
  to `lint-markdown`, to `git`, or to a file tool. A note cannot change what
  string is passed, so these are rewritten to name the resolved root
  (`{specs-root}/inbox.md`) rather than the default.

Illustrative prose keeps the literal `specs/`, per AC11. The triage rule is the
one stated above and is recorded in the spec so a later reader applies the same
boundary: acted-on resolves, illustrative does not.

`groom.md`'s inbox read is called out separately because it is the only site
where **no primitive owns the operation on either path**. Resolving it in prose
is correct for the markdown-only path; whether the read should become a
primitive is left to a follow-up rather than widened into this scenario.

The generated command copies under the installed commands directory are
re-rendered so the shipped artifacts match their sources.

## Edge Cases

- **A note is not a substitute for a literal argument.** A command carrying the
  blanket note can still be wrong if a step passes `specs/inbox.md` to a
  primitive: the note tells the host how to read paths, not what string to send.
  Both sites in `log.md` are this shape, which is why the fix is two-part.
- **`{feature}` placeholders are not the defect.** `specs/{feature}/tasks.md` is
  wrong only in its `specs/` segment; the brace placeholder is already
  resolved by the caller and must be left alone.
- **Primitive-described paths are left unchanged.** Rewriting prose that merely
  describes where `create-scenario` writes would make the description disagree
  with the primitive, trading a real defect for a documentation one.
- **AC7 stays checked only if the sweep is complete.** A partial fix leaves the
  criterion asserting something untrue, which is the state this scenario exists
  to end; the acceptance is the full triage, not the four headline sites.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
