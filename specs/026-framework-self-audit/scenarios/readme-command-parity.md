---
section: "Follow-on scenarios"
---

# Readme-command-parity

## Context

`/ductus:fold` shipped with 051 and never reached the README. Neither did the rest of that feature's adopter-facing surface: `/specify`'s row omits `--branch`, `--branch-id`, and `--fold-into` (while the `/review`, `/analyze`, `/prune`, and `/link` rows all document their flags), the branch-scoped `{identifier}.{n}-{slug}` numbering form appears nowhere, and `/status`'s row does not mention that it reports pending folds.

No existing family covers this, and the two nearest ones show why:

- **Family 16 (installer-command-parity)** pins the installer manifest in `framework/bootstrap/ductus.md` to the set of `framework/commands/*.md`, minus a maintainer-only exclusion list. Its own rationale records the same failure class in a different artifact — `prune.md` "was the first to bite," a command that existed in the framework and was dogfooded here yet never reached adopters.
- **Family 30 (command-flag-hint-parity)** checks a command's Flags table against that same command's `argument-hint:` frontmatter. Command-internal; it never looks outward at the README.

So the README is the one adopter-facing surface with no parity check behind it, and it drifts in exactly the way §drift-prevention's target list warns about: the sweep's own target list goes stale, and a grep aimed at the old surface passes cleanly while missing the files that moved.

A manual review corrects today's drift and guarantees nothing about tomorrow's — which is the diligence dependency [§design-principles](../../../framework/constitution.md#design-principles) forbids outright. The durable form is a check.

## Behavior

A new audit family asserts that every command shipped to adopters appears in the README's command tables, and fails the release gate when one does not.

- The shipped set is `framework/commands/*.md` **minus the maintainer-only exclusions**, which is the set Family 16 already computes — reuse it rather than restating it, since two copies of that list is how the two drift apart.
- `/audit`'s absence from the README is **correct** and must stay correct: it is maintainer-only, and adopters never invoke it. A family that reported it would be manufacturing a finding out of a deliberate omission.
- Findings render through `emit` and the script exits `"$drift"`, per the shell contract in `scripts/audit/README.md`.

## Edge Cases

- **A failed listing is a finding, not a clean pass.** If the command directory or the README cannot be read, say so — a corpus that could not be examined must never report as a README documenting everything (`QUAL-CLAIM-001`).
- **Presence is the assertion, not accuracy.** The family checks that a command is documented, not that its description is current; asserting the latter needs semantic judgment and belongs to a reviewer, not a shell family. State the bound rather than implying wider coverage.
- **Flag-level parity is a separate question.** Whether the README's rows must enumerate each command's flags — the `/specify` gap that prompted this — is worth deciding explicitly: it is a second assertion over the same artifacts, and folding it in silently would make one family's name describe two checks.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
