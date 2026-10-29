---
section: "Frontmatter, dependency, anchor and rule checks"
---

# Shared-constitution-references-are-judged

## Context

A project may register a shared constitution under `.ductus/config.toml` `[constitutions.<name>]` (path `../shared/constitution.md`), and its specs cite that document's sections in the qualified form, path and anchor on one line: ``(`../shared/constitution.md` §rule-with-three-parts)``. `/{project}:analyze` step 4 runs `resolve-anchor` with `markers-path` set to the framework constitution alone. Two adopter reports, verified 2026-10-26 at HEAD c0952038 with the 0.59.11 binary:

- **Markers matching by basename.** The markers file is recognized by substring on any path token containing its basename, so `../shared/constitution.md`, `team-constitution.md` and `constitution.md.bak` all read as the markers file. A shared-constitution anchor cited after such a path — a backticked `constitution.md`, a `../<owning-repo>/constitution.md` path, an in-tree registered path, a markdown link — comes back `kind: markers`, `resolved: false`. A test landed in 2be18da6 (`a_line_citing_a_nested_markers_file_by_a_relative_path_is_not_qualified`, mutation M31 in `scripts/mutation/owner-reviews-067.py`) locks in that matching.
- **An inconsistent split by anchor.** In one file, same form: `§two-part` → `qualified`, `resolved: true`; `§rule-with-three-parts` → `markers`, `resolved: false`; a second file's `§word-word-word` anchors the same. The reporter guessed anchor hyphen count (five data points); the basename matching above may be the cause instead. With `markers-path` set to the shared constitution, every one resolves.
- **Qualified means unread.** A missing anchor cited after any other document — existing, missing, or outside the tree — comes back `qualified`, `resolved: true`, because `resolve-anchor` never reads the named document. The primitive counts these (`qualified`), but step 4 reads only `unresolved` and records no unexamined reason, so at `/{project}:analyze` an unchecked reference is indistinguishable from a checked one. A non-markdown document comes back unresolved, a false alarm. A whole path-and-section citation inside one code span is not scanned. A registered constitution that cannot be read is already unexamined (`constitution-unresolved`, 055 AC8).
- **Consequences.** Each such cite yields an advisory `anchor-resolution` finding, discarded as a false positive, one per citing spec. The stored discard is keyed by family and message text, so after the cited section was renamed the real break matched the stored discard. Authors were told to put path and section in one code span so ductus skips them, which hides every rename.

## Behavior

- Whether a reference is qualified does not depend on its anchor's shape: two references in the same form on the same kind of line classify the same way.
- The markers file is recognized by its resolved path, not by a basename substring, so a registered shared constitution, or any other file sharing the basename, is not the markers file.
- A qualified reference that names a registered `[constitutions.*]` document is judged against that document's markers, so a missing anchor there is unresolved and a present one resolves.
- `/{project}:analyze` surfaces the references it did not check rather than dropping them, so a run's result distinguishes "every cited section exists" from "these citations were not examined".
- Step 4, its scope line and the installed-command anchor check take their marker sources from `resolve-constitutions`' `loaded` documents.

## Edge Cases

- Minimal fixture: a shared constitution with markers `§a-b` and `§a-b-c`, and one spec citing both in the qualified form. The test asserting both classify alike is proven to fail against the current resolver before the fix.
- `resolve-constitutions` returns a `..` sibling path, which `resolve-anchor` refuses today (`parent-directory component ('..') not permitted`).
- A registered constitution that cannot be read: stays `constitution-unresolved`, or the named document's references become unexamined.
- A stored discard of an `anchor-resolution` finding whose section was since renamed: falls away once registered documents are judged; what remains is 058's decision matching, with 063's primitive-decided `decision-key` as precedent.
- The constitution's Cross-document references bullet says nothing about shared constitutions or unchecked qualified references, and still says "longest-first", which AC2 says never decided a match.

## Open Questions

- Judging qualified references that name a registered `[constitutions.*]` document reverses in part `anchor-reference-kinds`' resolved question "Why not resolve qualified references properly" (already reversed once, by 071, for the Shared Files map only): is the registration the loading policy the same way the manifest is, and does the 2be18da6 basename-matching test (and mutation M31) change with it?
- How does `/{project}:analyze` surface qualified references it did not check — an unexamined reason, a count in the record, or a report line — and does an unreadable named document stay `qualified` (067's deliberate choice, test `a_mapped_source_that_cannot_be_read_stays_qualified`) or become unexamined?
- Is a path-and-section citation inside one code span brought into scope by extending 071's checked form (a bold section name plus `in` and the path) or a backticked path outside the span to registered constitutions, rather than by scanning code spans?

## Resolved Questions

*None yet.*
