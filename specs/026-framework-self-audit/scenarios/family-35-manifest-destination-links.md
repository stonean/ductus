---
section: "Check Families"
---

# Family-35-manifest-destination-links

## Context

`/ductus` copies files out of this repository and into an adopter's. A copied markdown file carries its link text verbatim, but a relative link is a claim about a *neighbour* — and the copy has different neighbours than the original.

`framework/rules/quality-cross.md` cited a scenario as evidence for `QUAL-CLAIM-001`:

```text
[017's `generator-sync-claim-honesty`](../../specs/017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md)
```

Two directories up from `framework/rules/` is the repository root, so the link resolves here and always has. The manifest installs that file at `specs/rules/quality-cross.md`, where two directories up is the *adopter's* root — and the adopter has no spec 017. Ductus's `specs/` is this project's development record; it is not in the manifest and never ships. The link was dead the moment it landed.

**Nothing in the repository was looking at the tree where it breaks.** Family 26 walks this repo, where the link is correct. `check-corpus-links` walks the spec corpus of whatever repo it runs in, and in this one that means ductus's own specs. `check-orphaned-references` scopes to five adopter-owned referrers pointing into ductus-managed roots, so a framework file's outbound link is outside it by design. All three passed, and go on passing, because each resolves the link against the only tree in which it is correct.

It was reported from an adopter run, and the reporting adopter was blocked rather than merely inconvenienced. The same release that shipped the dangling link added `check-corpus-links` to the adopter pre-commit hook, corpus-wide rather than staged-scoped — deliberately, since the commit that breaks a link is usually the one deleting its target and the dangling referrers are not in the commit at all. The rule file installs *inside* the spec root that step scans. So one release gave that adopter a broken link and a hook that refused **every** commit while it dangled, spec-related or not, until they hand-edited a framework-managed file that the next `/ductus` would overwrite.

The sweep that followed found 33 more of the same class across `framework/commands/`, `framework/bootstrap/`, and the constitution — dead for an adopter who clicks them, silent because they land outside the spec root. Two of those were found by this family's own first run, after a hand-grep for `](../` had already declared the sweep complete: `framework/constitution.md`'s `commands/review.md` and `framework/commands/amend.md`'s `../../framework/constitution.md#drift-prevention`, neither of which begins with `../` in the shape the grep looked for. A hand sweep that misses two instances on its first pass is precisely the diligence dependency [§design-principles](../../../framework/constitution.md#design-principles) rejects.

## Behavior

A `/{project}:audit` family resolves each shipped file's relative links against the path that file occupies **in the adopter's tree**, derived from the installer's own manifest rather than restated.

The subject is the manifest, and it is split in two because the two tables have different destination forms — not as a convenience.

- **35a — Shared Files.** Destinations are literal (`specs/rules/*.md`, `.ductus/constitution.md`, `.githooks/*`). Each source is copied to its destination in a throwaway tree and `check-corpus-links --scope repository` is run there.

  **The check is the primitive; this is the entry point** — the Family 30 shape, and here it is load-bearing rather than stylistic. A second link resolver written for the adopter tree would be a second implementation of the rule Family 26 already delegates, and those two would diverge exactly as Family 26's own python copy diverged from the primitive within a day of it shipping — the primitive resolving a root-absolute target against the repository root while the family still resolved it against the filesystem root. One resolver, a third subject, and the difference is a directory rather than a fork.

- **35b — Slash commands.** Destinations carry `{config_dir}` and `{project}` placeholders. These cannot go through 35a for two **independent** reasons, and either alone would be sufficient: the destination has no literal form without inventing an agent and a project name, and the primitive resolves the host's config dir out of its walk by construction — precisely so it never reports an adopter's generated command copies, whose links are broken by design. Building a tree the primitive is built to ignore would return a confident zero, which is the failure mode this suite exists to refuse.

  So 35b is lexical, and it is **exact rather than heuristic** because of what the destination directory contains: every ductus-authored file in `{config_dir}/commands/{project}/` is a sibling `.md` from this same manifest. A relative target holding a path separator therefore leaves that set *by definition* — `../constitution.md` and `commands/review.md` both do, and both shipped. A bare sibling (`review.md`) resolves and is not a finding. No resolution, no filesystem, no guessing.

The family reports and never repairs. Which repair is right depends on why the target is unreachable — a target that never ships wants an absolute URL, one that ships at a different depth wants prose — and that is a judgment about the target, not about the link.

## Edge Cases

- **A documented shape, not a pointer.** A template naming `specs/NNN-feature/spec.md` or `{config_dir}/commands/…` describes a form an adopter will create. Both halves drop targets containing `NNN`, `*`, or `{`, and both skip fenced blocks and inline code spans — the primitive's own filter on the 35a side, matched deliberately on the 35b side so the two halves cannot disagree about what a link is.
- **A manifest source that does not exist.** Reported as a manifest defect rather than skipped. The row is the claim; a row naming a missing file is wrong whether or not its links would have resolved.
- **No pair extracted from either table.** A finding. An empty manifest extraction and a manifest whose every link resolves exit identically otherwise, and the first is the one that happens when the table's shape changes.
- **The runtime is unreachable, or its result does not parse.** Each is a finding. 35a's whole guarantee is delegated, so a delegation that did not run must not be reported as a delegation that came back clean.
- **`specs/templates/` and the host config dir are excluded by the primitive.** Not silently: the excluded count is reported on stderr alongside the examined count, and 35b exists because one of those exclusions would otherwise hide the command files entirely.

## What this family does not assert

35a proves that a shipped file's links resolve against the **ductus-authored portion** of an adopter's tree — the manifest destinations, which is all this repository can know about. It does not prove that every link in an adopter's corpus resolves; that subject includes their own specs, and nothing here can see it. Both counts go to stderr so a clean exit reads as the first claim and never as the second.

This is the same discipline `QUAL-CLAIM-001` asks of shipped code, applied to the family that ships the rule.

## Resolved Questions

- **Why not extend Family 26 with a second scope instead of adding a family?** Family 26's subject is *this* tree, and its argument (`--scope repository`) selects how much of this tree. The adopter tree is not a scope of this one — it is a different tree that has to be constructed before anything can be resolved in it, from a manifest Family 26 does not read. Folding the construction into Family 26 would mean one family whose name describes one check and whose body performs two, with a shared examined-count that no longer means anything. The resolver is shared; the family is not.
- **Why not run 35a against a real adopter checkout?** Because there is not one, and a family that passes only on the maintainer's machine is worse than no family. The manifest is the contract, so a tree built from the manifest is the subject the contract describes — and it is reproducible in CI, which a borrowed checkout is not.
