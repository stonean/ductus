---
section: "Check Families"
---

# Family-39-shipped-anchor-resolution

## Context

`resolve-anchor` classifies each `§` reference in a file and, given `markers-path`, resolves the ones that are claims about the constitution against its `<!-- §name -->` markers (022's `anchor-reference-kinds`). `/{project}:analyze` runs it over the targeted spec alone, and its Project-level consistency section asks the host to check the installed command files by hand. Nothing in this repository ran it over the framework files ductus ships.

An adopter ran it by hand over its installed `.claude/commands/ductus.md` against `.ductus/constitution.md`, because a decision the adopter had stored named the check, and got six unresolved anchors it could not fix: the file is framework-managed. At HEAD the same file carried seventeen, one of them (`§Verified`) added by the release two days before the report. Every family in this suite was green throughout. `f8db885a` and `bd14fefe` fixed all of them, and measured across every tracked `framework/**/*.md` — 64 files, 515 `§` references — none is unresolved. This family keeps it there.

## Behavior

A `/{project}:audit` family runs `resolve-anchor --markers-path framework/constitution.md` over every tracked markdown file under `framework/` and reports each unresolved reference at `file:line`, naming its anchor.

- **The subject is `framework/`, whole, enumerated from git.** Every file under it ships in the release archive a `/ductus` run extracts, and the installed ones — the bootstrap, the command sources, the rule files, the templates, the constitution itself — are a subset. Enumerating from `git ls-files` rather than the manifest keeps the archive-half files in scope, `ductus-procedure.md` and the migration bodies, which a host reads during a run without installing; they carried fourteen and six of the references that did not resolve. `govern.md` is included although Family 21 holds it byte-identical to `ductus.md`: pre-rename installs still fetch it, so it ships in its own right, and a defect in the pair reported twice is accurate.
- **The check is the primitive and this is the entry point**, the Family 30 shape. A second resolver here would drift from the one an adopter runs, and the report this family answers is about that one.
- **It runs here, not in the adopter.** An adopter-side finding is one the adopter cannot fix, which was the report's whole complaint; the repair is a ductus release, so the check belongs where the repair is made.
- **The repair is in the citing prose, never in the check.** Each finding names the three: spell a local heading as it reads, name another markdown document on the same line, or drop the `§` from a section of a document that is not markdown.

## Edge Cases

- **An unreachable runtime, an empty enumeration, and a file whose result could not be read or parsed** are each findings. The first two examined nothing and the third did not examine that file; none may exit like a clean run.
- **Every run reports its coverage on stderr** — files examined, `§` references found, and how many resolved as qualified, intra-document, and constitution markers — so a clean exit says what it looked at.
- **What it does not catch, stated rather than implied.** The primitive recognises another document only by a `.md` filename or link on the same line, so a section of a non-markdown document cannot carry `§` at all; a line naming any `.md` qualifies every `§` on it, a genuine constitution citation included; and a reference meant for another file resolves against a same-named heading in the citing file — `ductus.md`'s `§Inputs` beside `/ductus:review` did until `f8db885a` named `review.md` on that line. These are the primitive's classification rules (022's AC23), and the family inherits them.
- **No exemption marker.** A sanctioned way to silence one reference is a hiding place, the reason 046 refused a grandfather rule for scenario questions, and every reference the corpus carries today resolves without one.
- **Generated command copies are not a subject.** They sit outside `framework/`, and their repair is their source.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
