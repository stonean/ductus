---
section: "Follow-on scenarios"
---

# Adopter-corpus-link-integrity

## Context

Three checks sit near this ground and none of them covers it.

- `check-orphaned-references` scopes to the five adopter-owned bootstrap referrers (`CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit`, and the spec root's `system.md`) pointing into ductus-managed roots. Spec-to-spec links are outside it by design.
- `/ductus:analyze` is bounded to one feature directory plus its declared dependencies, so it cannot see the corpus.
- `scripts/audit/broken-relative-links.sh` (Family 26) performs exactly the right check and is **maintainer-only** — adopters never invoke `/ductus:audit`.

So an adopter who deletes or renames a spec directory dangles every inbound pointer — sibling body links, scenario links one tier deeper, and the `dependencies:` edges derived from them — and nothing reports it. The failure is the silent kind this project pays most for: no error, no gate, and the damage surfaces only when a reader follows a pointer to nothing. Family 26 found 28 broken links in this repo when it was written, with a maintainer applying the no-dead-references rule by hand.

The pressure for it comes from adopters: a project inheriting the framework accumulates specs whose decisions later specs counter, and the reflex is to delete the stale ones. Deletion is not the problem — an unsupervised deletion is.

## Behavior

A runtime primitive reports every relative markdown link in the spec corpus whose target resolves to nothing, and the adopter pre-commit hook runs it, so a deletion fails at the commit that makes it rather than at a reader's next traversal.

- Resolution is **lexical**, against the citing file's own directory, never canonicalized — canonicalization would make the result depend on symlinks.
- **Inline code spans are stripped before matching.** This is load-bearing rather than tidy: docs that discuss linking quote link syntax constantly, and Family 26 reports 7 false positives without it, every one a doc correctly describing a link rather than making one.
- Generated command copies and adopter-facing templates are excluded by construction — their links resolve elsewhere by design — and each exclusion is **counted**, never silent.

## Edge Cases

- **A failed file listing is a finding, not a clean pass.** An unreadable corpus reported as "no broken links" is the exact shape `QUAL-CLAIM-001` forbids: a check that could not run must never be indistinguishable from one that passed.
- **Scheme-bearing targets (`https:`, `mailto:`) and bare fragments** are out of scope; a fragment on a relative target is stripped and the file part checked.
- **Family 26 is not necessarily retired by this.** The family covers this repository's own artifacts, including maintainer-only files the shipped primitive has no reason to know about. Whether the family then delegates to the primitive — the shape Family 30 already uses, where the check is a primitive and the script is the entry point — is a design question for the plan, not settled here.
- **A pattern is not a reference.** A candidate containing `*` or `NNN` is documentation naming a shape; testing it against the filesystem would manufacture findings out of prose.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
