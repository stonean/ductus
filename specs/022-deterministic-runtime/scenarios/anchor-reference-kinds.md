---
section: "Follow-on scenarios"
---

# Anchor-reference-kinds

## Context

`resolve-anchor` treated every `§X` in a file as a claim about the markers file. It is not.

`§` is this corpus's notation for **a section**, not for *a constitution section*. A spec writes all of these in the same paragraph:

| written | means | resolvable here? |
| --- | --- | --- |
| `§grounding` | a constitution section | yes — that is the check |
| `` `AGENTS.md` §Workflow `` | a section of another document | no — those markers are not in hand |
| `[review.md](../../framework/commands/review.md) §Behavior` | same, named by link | no |
| `per §Design above` | a heading in *this* file | yes, against this file |
| `spec 022 §Versioning` | another document, named only in prose | no, and not excludable either |

Resolving all of them against one file reported **112 unresolved anchors** across the spec corpus. `/{project}:analyze` surfaces them as advisory findings, so every spec carried a handful of them permanently.

**The cost was not noise for its own sake.** Spec [023](../../023-govern-refinement/spec.md) deleted `§lightweight-track` from the constitution — its AC2 asserts the section is gone, and its own task said *"verify the anchor `§lightweight-track` is no longer referenced anywhere."* [010](../../010-agent-autonomy/spec.md)'s spec body still cited it. `resolve-anchor` had been reporting that correctly the whole time, sitting among 111 lines that were not defects, and it survived four specs' worth of history until someone read the list by hand. A signal that is 99% noise is not a signal.

## Behavior

Each reference is classified into one of three kinds, and the classification is what makes an unresolved one worth reading.

- **`qualified`** — the reference's **line** names a markdown document other than the markers file: a backticked path, a markdown link target, or a bare `*.md` token. Excluded by construction and **counted**.

  *Line-scoped, not immediately-preceding.* The dominant real shape is a table row or clause that names the file once and then cites several of its sections — `` | `review.md` | edit — §Behavior step 5 and §Load | `` — where an immediately-preceding rule catches the first reference and misses the rest. Measured: immediately-preceding covered 34 of 112; line-scoped covers 136 of 311 references.

  *"Other than the markers file" is load-bearing*, and it was not in the first draft. A line citing the constitution itself — `[§known](constitution.md#known)`, or `` `framework/constitution.md` §grounding `` — names a document, but the document it names is precisely the one whose markers are in hand. Excluding it would drop the single kind of reference this primitive exists to check. An existing test caught that on the first draft, which is the argument for having had one.

- **`intra-document`** — the anchor names a heading in the citing file. Resolved against that file's own headings, longest first, so `§Hook Installation` is not satisfied by a shorter `§Hook` elsewhere in the file.

- **`markers`** — everything else: a genuine claim about the markers file, and **the only kind that can come back unresolved**.

`qualified` and `intra-document` counts ship in the result. An exclusion nobody counts is indistinguishable from a scan that found nothing — `QUAL-CLAIM-001` — and here the exclusion is large enough (136 of 311) that its silence would be the whole story.

Ordering: a line naming another document is `qualified` **even when** the anchor happens to match a local heading. The author named the document they meant, and preferring a coincidental local match over an explicit filename is how a rule starts inventing answers.

## What stays reported, deliberately

112 → 34. The residue is prose that names another document without naming its file — `spec 022 §Versioning`, `the bootstrap's §Derived values`, `both bootstrap twins' §Placeholder Substitution`. These can be neither verified nor excluded, and reporting them is the honest answer.

Three of the 34 are historical claims in `done` specs — `Delete §lightweight-track`, `§supersession-annotations` removed entirely, `§three-cycles` swept by 023 and since gone. They are correct as written and still reported: a reader following any of them finds nothing, which is what the check is for. The `not-a-live-claim` co-occurrence exemption that `criterion-path-existence` applies to *paths* would fit here, and is deliberately not built for three instances — two of which are arguably better reported than suppressed.

**The rule is exact, not a heuristic.** It excludes only references whose line demonstrably names another document. That is why `010`'s real dangling anchor — on a line naming no document — survives it, which is asserted as a unit test rather than left as a hope.

## Edge Cases

- **A `§` inside a marker comment** (`<!-- §foo -->`) is still not a reference; that exclusion predates this change and is unaffected.
- **A `§` inside an inline code span** is notation being *described*, not cited. The constitution defines what an anchor reference looks like by writing one — `anchor reference (§anchor)` — and reporting that as a dangling reference manufactures a finding out of prose. Same class as the marker-comment exclusion, and the same rule `check-corpus-links` already applies to link targets in code spans; it reuses the shared `inline_code_spans` helper rather than a second scanner. Found by running this primitive against the constitution while reviewing 050.
- **No `markers-path`** (the constitution self-consistency scan): the markers-identifier set is empty, so any `.md` token on a line qualifies it. Correct for that mode — the file's own markers are the subject, and a line naming another document is not about them.
- **A spec citing the constitution by basename from a deeper directory** (`../../framework/constitution.md`): both the full path and the basename count as naming the markers file, because a scenario one tier down writes a relative path whose basename is all that reliably matches.
- **A heading and a marker sharing a name.** `qualified` is checked first, then `intra-document`, then `markers`; the order is stated because all three can be true of one line.

## Resolved Questions

- **Why not resolve qualified references properly, by loading the named document's markers?** Because it changes the primitive from "one file against one marker set" into a link resolver with a file-loading policy, an error surface for unreadable targets, and a question about what counts as a document root. `check-corpus-links` already owns cross-document resolution. This primitive's honest scope is the one marker set it was given, and the fix is to stop pretending references to other documents are within it.
- **Why report the residue rather than widening the rule until it is empty?** Because the widening would be heuristic — matching `the bootstrap's` or `spec 022` means guessing which document prose meant — and a rule that fires falsely on a correct reference is worse than the silence it replaces, the standard [045](../../045-decision-state-drift-detection/spec.md) set. 34 classified findings that a maintainer can read beats 112 undifferentiated ones, and beats 0 bought by guessing.
