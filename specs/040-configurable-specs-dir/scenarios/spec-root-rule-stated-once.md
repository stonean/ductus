---
section: "Resolution behavior"
---

# Spec-root-rule-stated-once

## Context

[040](../spec.md)'s *Resolution behavior* section settled prose parameterization by stating the
configurability fact once at its canonical home and keeping `specs/` literal everywhere
else (AC11, and the *Prose parameterization* resolved question). The
[`command-prose-resolves-spec-root`](command-prose-resolves-spec-root.md) sweep then
propagated a **blanket note** into every command file that acts on a spec-root path,
because a note was the mechanism that made the markdown-only path substitute correctly.

The result is the state AC11 set out to avoid, reached by a different route. The same
rule is now restated verbatim in seven places:

- `framework/constitution.md:118` — the canonical statement.
- `framework/commands/specify.md:60`, `log.md:28`, `groom.md:27`, `review.md:114`,
  `amend.md:42`, `implement.md:94` — six near-identical ~55-word blockquotes.

(`framework/bootstrap/ductus.md:1129` also carries it, and is a different case — see
Edge Cases.)

That is what [§drift-prevention](../../../framework/constitution.md#drift-prevention)
forbids:

> For every kind of fact described in multiple places, one location is authoritative.
> Other documents that describe the fact MUST reference the canonical source rather than
> restate it.

The failure mode is not hypothetical — it is the one that produced the sibling scenario.
`specify.md` carried the note, the other five did not, and fourteen host-acted sites were
wrong across six files. The sibling's own diagnosis was that "the fix pattern already
exists in the corpus and was simply never propagated." Propagating it by hand to six
files does not remove that hazard; it multiplies the surface that must stay in sync. The
next command file added to the framework inherits the same silent-default bug unless its
author remembers to paste a 55-word blockquote into it.

Adopter-reported (issue #2, second half). The reporter's complaint was that the
constitution names `specs` and then explains it may be customized, and that the
explanation should be unnecessary. The token cost they cite is real but small — 83 words
in an 11,780-word constitution. The maintenance cost is the substantive one.

## Behavior

The spec-root substitution rule is stated **once**, in the constitution's §spec-phase
directory-layout block, and every command references it rather than restating it — the
discipline those same files already apply to §bug-handling, §scenarios, and every other
cross-cutting rule ("constitution loaded by `/{project}:target` — do not re-read").

Three changes:

- **The canonical statement becomes imperative.** `constitution.md:118` currently
  *describes* runtime behavior ("wherever a command or the runtime constructs a path
  under it, it resolves `[paths] specs-root`"). A description of what the runtime does is
  not an instruction to the host, which is precisely why six command files grew one. The
  canonical line carries the instruction the blockquotes carry: on the markdown-only
  path, substitute the configured name for the literal `specs/` wherever a command acts
  on a spec-root path.
- **The six command blockquotes are deleted.** Each command's existing `Reference:` line
  gains `§spec-phase` where it is not already cited, so the pointer is explicit rather
  than implied.
- **The canonical-sources table gains a row.** §drift-prevention instructs that a new
  kind of fact referenced from multiple documents is named there explicitly; spec-root
  resolution is one and was never registered. The row points at
  `framework/constitution.md` §spec-phase.

Illustrative prose is untouched. AC11's substantive holding — `specs/` stays literal, no
`{specs-root}` placeholders in human-read documents — is preserved exactly. This scenario
reduces the number of places the *caveat* is repeated, not the number of places the
default appears.

Full parameterization stays rejected, for the reason 040 already recorded and this
scenario does not reopen: `CLAUDE.md:3` imports `framework/constitution.md` directly and
this repo carries no installed `.ductus/constitution.md`, so `{specs-root}` placeholders
would permanently degrade ductus's own governance document to serve a key most adopters
never set.

The generated command copies under the installed commands directory are re-rendered so
the shipped artifacts match their sources.

## Edge Cases

- **The bootstrap keeps its own note.** `framework/bootstrap/ductus.md:1129` is not a
  redundant restatement: `/ductus` is what scaffolds the constitution, so it runs in
  sessions where no constitution is loaded and none may yet exist on disk. It cannot
  reference a canonical source that is not there. Its note stays — the one file where the
  duplication is load-bearing rather than drift.
- **A command reached without `/{project}:target`.** Every command's `Reference:` line
  already assumes the constitution was loaded by `/{project}:target`, and
  `/{project}:log` explicitly requires no session target, so a session can reach it with
  nothing loaded. That is a pre-existing property of the whole corpus rather than
  something deleting these notes introduces, and on the runtime path it is inert because
  `append-inbox` resolves the root itself. Widening it into a per-command constitution
  reload is out of scope: if the assumption is wrong it is wrong for §bug-handling too,
  and belongs in its own scenario.
- **Deleting a note is not deleting a resolution.** The blockquotes are host guidance,
  not resolution logic. Every site the sibling scenario rewrote to name the resolved root
  as a literal *argument* — `log.md`'s `lint-markdown` target, `groom.md:32`'s inbox read
  — is left exactly as it stands. This scenario removes duplicated guidance, never a
  resolved path.
- **Reducing restatement must not reintroduce silent defaults.** The acceptance is that a
  renamed-root project still completes the markdown-only path correctly with the six
  notes gone. If it cannot, the canonical statement is not carrying the instruction, and
  the fix is to strengthen that one line — not to paste the blockquote back into six
  files.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
